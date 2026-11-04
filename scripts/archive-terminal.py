#!/usr/bin/env python3
"""Archive ordinary package files only, without macOS AppleDouble sidecars."""
import sys,tarfile
from pathlib import Path
root=Path(sys.argv[1]).resolve();target=Path(str(root)+'.tar.gz')
if target.exists():raise SystemExit('occupied archive refused')
with tarfile.open(target,'w:gz',format=tarfile.PAX_FORMAT) as tar:
 for path in [root,*sorted(root.rglob('*'))]:
  if path.is_symlink() or not(path.is_dir() or path.is_file()):raise SystemExit('only ordinary package files/directories may be archived')
  if path.name.startswith('._'):raise SystemExit('unexpected AppleDouble file in package')
  tar.add(path,arcname=str(Path(root.name)/path.relative_to(root)),recursive=False)
