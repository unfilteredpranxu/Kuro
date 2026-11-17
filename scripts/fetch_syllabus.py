#!/usr/bin/env python3
"""Acquire official source pages. Deliberately does not invent a curated topic dataset."""
import hashlib
import json
from pathlib import Path
import subprocess
import urllib.request

URL = 'https://jeeadv.ac.in/documents/jee-advanced-2026-syllabus.pdf'
ROOT = Path(__file__).resolve().parents[1] / 'data' / 'syllabus'

def main():
    ROOT.mkdir(parents=True, exist_ok=True)
    with urllib.request.urlopen(URL, timeout=60) as response:
        data = response.read(20_000_001)
    if not data.startswith(b'%PDF-') or len(data) > 20_000_000:
        raise ValueError('Response is not a supported-size PDF; nothing ingested')
    pdf = ROOT / 'official-2026.pdf'
    pdf.write_bytes(data)
    info = subprocess.check_output(['pdfinfo', str(pdf)], text=True)
    count = next(int(x.split(':', 1)[1].strip()) for x in info.splitlines() if x.startswith('Pages:'))
    pages = []
    for page in range(1, count + 1):
        text = subprocess.check_output(['pdftotext', '-layout', '-f', str(page), '-l', str(page), str(pdf), '-'], text=True)
        if not text.strip():
            raise ValueError(f'Page {page} needs manual/OCR inspection; no complete-source claim is possible')
        pages.append({'source_page': page, 'text': text})
    (ROOT / 'source-pages.json').write_text(json.dumps(pages, ensure_ascii=False, indent=2))
    (ROOT / 'source-manifest.json').write_text(json.dumps({'url': URL, 'baseline': 2026, 'sha256': hashlib.sha256(data).hexdigest(), 'pdf_pages': count, 'extracted_pages': len(pages), 'topic_audit': 'NOT_STARTED'}, indent=2))
    print(f'Extracted {len(pages)} of {count} pages. Manual section/topic audit still required.')

if __name__ == '__main__':
    main()
