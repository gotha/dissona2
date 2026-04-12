import io
from typing import List, Optional

import boto3
import fitz  # PyMuPDF
import structlog
from prometheus_client import Counter, Histogram

from config import Settings
from chapter_detector import ChapterDetector

logger = structlog.get_logger()

# Metrics
JOBS_PROCESSED = Counter("pdf_jobs_processed_total", "Total PDF jobs processed", ["status"])
JOB_DURATION = Histogram("pdf_job_duration_seconds", "PDF job processing duration")
PAGES_PROCESSED = Counter("pdf_pages_processed_total", "Total PDF pages processed")


class PdfProcessor:
    def __init__(self, settings: Settings):
        self.settings = settings
        self.chapter_detector = ChapterDetector(settings)
        
        # Initialize S3 client
        self.s3 = boto3.client(
            "s3",
            endpoint_url=settings.s3_endpoint,
            aws_access_key_id=settings.s3_access_key,
            aws_secret_access_key=settings.s3_secret_key,
        )
    
    @JOB_DURATION.time()
    async def process(self, job: dict) -> dict:
        """Process a PDF parsing job."""
        job_id = job["job_id"]
        document_id = job["document_id"]
        file_path = job["file_path"]
        
        logger.info("Processing PDF", job_id=job_id, file_path=file_path)
        
        try:
            # Download PDF from S3
            pdf_data = self._download_pdf(file_path)
            
            # Extract text
            doc = fitz.open(stream=pdf_data, filetype="pdf")
            text, page_count = self._extract_text(doc)
            
            PAGES_PROCESSED.inc(page_count)
            
            # Detect chapters
            chapters = await self.chapter_detector.detect(doc, text)
            
            doc.close()
            
            # Build result
            result = {
                "document_id": document_id,
                "page_count": page_count,
                "chapters": [
                    {
                        "number": i + 1,
                        "title": ch.title,
                        "text": ch.text,
                        "word_count": len(ch.text.split()),
                    }
                    for i, ch in enumerate(chapters)
                ],
                "detection_method": self.chapter_detector.last_method,
            }
            
            JOBS_PROCESSED.labels(status="success").inc()
            logger.info("PDF processed", job_id=job_id, chapters=len(chapters))
            
            return result
            
        except Exception as e:
            JOBS_PROCESSED.labels(status="error").inc()
            logger.error("PDF processing failed", job_id=job_id, error=str(e))
            raise
    
    def _download_pdf(self, file_path: str) -> bytes:
        """Download PDF from S3."""
        bucket = self.settings.s3_bucket_uploads
        response = self.s3.get_object(Bucket=bucket, Key=file_path)
        return response["Body"].read()
    
    def _extract_text(self, doc: fitz.Document) -> tuple[str, int]:
        """Extract text from PDF, using OCR if needed."""
        all_text = []
        page_count = len(doc)
        
        for page_num, page in enumerate(doc):
            text = page.get_text()
            
            # If page has little text, try OCR
            if len(text.strip()) < 100 and self.settings.ocr_enabled:
                text = self._ocr_page(page)
            
            all_text.append(text)
        
        return "\n\n".join(all_text), page_count
    
    def _ocr_page(self, page: fitz.Page) -> str:
        """OCR a page using Tesseract."""
        try:
            import pytesseract
            from PIL import Image
            
            # Render page to image
            pix = page.get_pixmap(dpi=300)
            img = Image.frombytes("RGB", [pix.width, pix.height], pix.samples)
            
            # OCR
            text = pytesseract.image_to_string(img)
            return text
        except Exception as e:
            logger.warning("OCR failed", error=str(e))
            return ""
