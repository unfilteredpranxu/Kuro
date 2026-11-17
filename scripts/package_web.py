#!/usr/bin/env python3
"""Package only current, referenced release assets (not stale dx outputs)."""
from pathlib import Path
import hashlib
import json
import re
import zipfile

ROOT = Path(__file__).resolve().parents[1]
PUBLIC = ROOT / 'target/dx/kuro-web/release/web/public'
html = (PUBLIC / 'index.html').read_text()
js = re.search(r'assets/([^"<>]+\.js)', html).group(1)
wasm = {x for x in re.findall(r'[A-Za-z0-9_-]+\.wasm', (PUBLIC / 'assets' / js).read_text()) if (PUBLIC / 'assets' / x).is_file()}
if len(wasm) != 1:
    raise ValueError('Expected exactly one current WebAssembly payload')
files = [p for p in PUBLIC.rglob('*') if p.is_file() and ('assets' not in p.relative_to(PUBLIC).parts or p.name in {js, *wasm})]
manifest = {'build': '0.1.0', 'files': {str(p.relative_to(PUBLIC)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(files)}}
DIST = ROOT / 'dist'
DIST.mkdir(exist_ok=True)
archive = DIST / 'kuro-web-0.1.0.zip'
with zipfile.ZipFile(archive, 'w', zipfile.ZIP_DEFLATED) as z:
    for p in files:
        z.write(p, str(p.relative_to(PUBLIC)))
    z.write(ROOT / 'LICENSE', 'LICENSE.txt')
    z.writestr('KURO-MANIFEST.json', json.dumps(manifest, indent=2))
    z.writestr('RUN.txt', 'KURO 0.1 web foundation. Not production-complete.\nExtract and serve this directory with python3 -m http.server 4173\nOpen through HTTP(S), not file://. Serve at the origin root.\nStorage is specific to the browser origin; use backup/restore to move study data.\nNo streaming, native apps, PDF tools or audited official syllabus are included.\n')
with zipfile.ZipFile(archive) as z:
    assert z.testzip() is None
checksum = hashlib.sha256(archive.read_bytes()).hexdigest()
(DIST / 'SHA256SUMS').write_text(f'{checksum}  {archive.name}\n')
print(json.dumps({'artifact':str(archive), 'bytes':archive.stat().st_size, 'sha256':checksum, 'files':len(files)}, indent=2))
