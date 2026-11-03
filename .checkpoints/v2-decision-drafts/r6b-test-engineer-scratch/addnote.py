#!/usr/bin/env python3
"""addnote.py <id> <docs path> <reason-file>: append a NOTE block."""
import sys, pathlib
OUT = pathlib.Path('/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/r6b-test-engineer-fixes.md')
fid, path, rf = sys.argv[1], sys.argv[2], sys.argv[3]
reason = pathlib.Path(rf).read_text(encoding='utf8').rstrip('\n')
with OUT.open('a', encoding='utf8') as f:
    f.write(f'### NOTE {fid}\nFILE: {path}\nno change: {reason}\nEND\n\n')
print('appended NOTE', fid)
