import uuid
from typing import List, Optional

import boto3
import fitz  # PyMuPDF
import psycopg2
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

    def _get_db_conn(self):
        return psycopg2.connect(self.settings.database_url)

    def _update_document_status(self, document_id: str, status: str, substatus: str = None):
        conn = self._get_db_conn()
        try:
            with conn.cursor() as cur:
                cur.execute(
                    "UPDATE documents SET status = %s, substatus = %s, updated_at = NOW() WHERE id = %s",
                    (status, substatus, document_id),
                )
            conn.commit()
        finally:
            conn.close()

    @JOB_DURATION.time()
    async def process(self, job: dict) -> dict:
        """Process a PDF parsing job."""
        job_id = job["job_id"]
        document_id = job["document_id"]
        project_id = job["project_id"]
        file_path = job["file_path"]

        logger.info("Processing PDF", job_id=job_id, file_path=file_path)

        try:
            self._update_document_status(document_id, "processing", "parsing")

            # Download PDF from S3
            pdf_data = self._download_pdf(file_path)

            # Extract text
            doc = fitz.open(stream=pdf_data, filetype="pdf")
            text, page_count = self._extract_text(doc)

            PAGES_PROCESSED.inc(page_count)

            # Extract metadata (Story 2.3)
            metadata = doc.metadata or {}
            pdf_title = metadata.get("title", "").strip()
            pdf_author = metadata.get("author", "").strip()

            self._update_document_status(document_id, "processing", "detecting_chapters")

            # Detect chapters
            chapters = await self.chapter_detector.detect(doc, text)

            doc.close()

            self._update_document_status(document_id, "processing", "saving")

            # Save results to database
            self._save_results(
                document_id=document_id,
                project_id=project_id,
                page_count=page_count,
                detection_method=self.chapter_detector.last_method,
                chapters=chapters,
                pdf_title=pdf_title,
                pdf_author=pdf_author,
            )

            JOBS_PROCESSED.labels(status="success").inc()
            logger.info("PDF processed", job_id=job_id, chapters=len(chapters))

            return {"document_id": document_id, "chapters": len(chapters)}

        except Exception as e:
            JOBS_PROCESSED.labels(status="error").inc()
            logger.error("PDF processing failed", job_id=job_id, error=str(e))
            self._mark_failed(document_id, project_id, str(e))
            raise

    def _save_results(self, document_id, project_id, page_count, detection_method, chapters, pdf_title, pdf_author):
        """Write processing results to the database."""
        conn = self._get_db_conn()
        try:
            with conn.cursor() as cur:
                # Update document
                cur.execute(
                    """UPDATE documents
                       SET status = 'processed', substatus = NULL, page_count = %s,
                           detection_method = %s, extracted_chapters_count = %s,
                           title = COALESCE(NULLIF(%s, ''), title),
                           author = COALESCE(NULLIF(%s, ''), author),
                           updated_at = NOW()
                       WHERE id = %s""",
                    (page_count, detection_method, len(chapters), pdf_title, pdf_author, document_id),
                )

                # Insert chapters
                for i, ch in enumerate(chapters):
                    chapter_id = str(uuid.uuid4())
                    word_count = len(ch.text.split())
                    cur.execute(
                        """INSERT INTO chapters (id, document_id, project_id, chapter_number, title, source_text, word_count, status)
                           VALUES (%s, %s, %s, %s, %s, %s, %s, 'ready')""",
                        (chapter_id, document_id, project_id, i + 1, ch.title or f"Chapter {i + 1}", ch.text, word_count),
                    )

                # Update project title from PDF metadata if available
                if pdf_title:
                    cur.execute(
                        "UPDATE projects SET title = %s, status = 'ready', updated_at = NOW() WHERE id = %s",
                        (pdf_title, project_id),
                    )
                else:
                    cur.execute(
                        "UPDATE projects SET status = 'ready', updated_at = NOW() WHERE id = %s",
                        (project_id,),
                    )

            conn.commit()
            logger.info("Results saved to database", document_id=document_id, chapters=len(chapters))
        finally:
            conn.close()

    def _mark_failed(self, document_id, project_id, error_message):
        """Mark document and project as failed."""
        try:
            conn = self._get_db_conn()
            with conn.cursor() as cur:
                cur.execute(
                    "UPDATE documents SET status = 'failed', error_message = %s, updated_at = NOW() WHERE id = %s",
                    (error_message, document_id),
                )
                cur.execute(
                    "UPDATE projects SET status = 'failed', updated_at = NOW() WHERE id = %s",
                    (project_id,),
                )
            conn.commit()
            conn.close()
        except Exception as e:
            logger.error("Failed to mark as failed", error=str(e))
    
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
