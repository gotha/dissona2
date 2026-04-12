import re
from dataclasses import dataclass
from typing import List, Optional

import fitz
import structlog

from config import Settings

logger = structlog.get_logger()


@dataclass
class Chapter:
    title: Optional[str]
    text: str
    start_page: int
    end_page: int


class ChapterDetector:
    """Detects chapter boundaries in PDFs using multiple strategies."""
    
    def __init__(self, settings: Settings):
        self.settings = settings
        self.last_method = "unknown"
    
    async def detect(self, doc: fitz.Document, full_text: str) -> List[Chapter]:
        """Detect chapters using the best available method."""
        
        # Strategy 1: Try TOC
        chapters = self._detect_from_toc(doc)
        if chapters and len(chapters) > 1:
            self.last_method = "toc"
            return chapters
        
        # Strategy 2: Try heading patterns
        chapters = self._detect_from_headings(doc, full_text)
        if chapters and len(chapters) > 1:
            self.last_method = "headings"
            return chapters
        
        # Strategy 3: Try page-based patterns
        chapters = self._detect_from_patterns(full_text)
        if chapters and len(chapters) > 1:
            self.last_method = "patterns"
            return chapters
        
        # Strategy 4: Fall back to single chapter
        self.last_method = "single"
        return [Chapter(
            title="Full Document",
            text=full_text,
            start_page=0,
            end_page=len(doc) - 1,
        )]
    
    def _detect_from_toc(self, doc: fitz.Document) -> List[Chapter]:
        """Extract chapters from PDF TOC (table of contents)."""
        toc = doc.get_toc()
        if not toc:
            return []
        
        chapters = []
        for i, entry in enumerate(toc):
            level, title, page = entry
            
            # Only use top-level entries
            if level != 1:
                continue
            
            # Find end page
            end_page = len(doc) - 1
            for j in range(i + 1, len(toc)):
                if toc[j][0] == 1:
                    end_page = toc[j][2] - 1
                    break
            
            # Extract text for this chapter
            text = self._extract_pages(doc, page - 1, end_page)
            
            chapters.append(Chapter(
                title=title,
                text=text,
                start_page=page - 1,
                end_page=end_page,
            ))
        
        return chapters
    
    def _detect_from_headings(self, doc: fitz.Document, full_text: str) -> List[Chapter]:
        """Detect chapters from heading formatting."""
        # Look for consistent heading patterns in font sizes
        # TODO: Implement font-based detection
        return []
    
    def _detect_from_patterns(self, full_text: str) -> List[Chapter]:
        """Detect chapters from text patterns like 'Chapter 1' or 'PART I'."""
        patterns = [
            r"(?:^|\n)(?:Chapter|CHAPTER)\s+(\d+|[IVXLC]+)[:\.\s]+(.+?)(?=\n)",
            r"(?:^|\n)(?:Part|PART)\s+(\d+|[IVXLC]+)[:\.\s]+(.+?)(?=\n)",
            r"(?:^|\n)(\d+)\.\s+(.+?)(?=\n)",
        ]
        
        for pattern in patterns:
            matches = list(re.finditer(pattern, full_text))
            if len(matches) >= 2:
                return self._chapters_from_matches(matches, full_text)
        
        return []
    
    def _chapters_from_matches(self, matches, full_text: str) -> List[Chapter]:
        """Create chapters from regex matches."""
        chapters = []
        
        for i, match in enumerate(matches):
            title = match.group(0).strip()
            start = match.end()
            end = matches[i + 1].start() if i + 1 < len(matches) else len(full_text)
            
            chapters.append(Chapter(
                title=title,
                text=full_text[start:end].strip(),
                start_page=0,  # Unknown when using text patterns
                end_page=0,
            ))
        
        return chapters
    
    def _extract_pages(self, doc: fitz.Document, start: int, end: int) -> str:
        """Extract text from a range of pages."""
        text_parts = []
        for page_num in range(start, min(end + 1, len(doc))):
            text_parts.append(doc[page_num].get_text())
        return "\n\n".join(text_parts)
