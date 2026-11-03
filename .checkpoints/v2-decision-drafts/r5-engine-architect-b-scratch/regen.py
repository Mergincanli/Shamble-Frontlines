#!/usr/bin/env python3
"""Replace the block with id argv[1] in the fixes file, in place, by the last block of the file (which must carry the
same id): used to redo a block with mk.py (which appends) while keeping the file's order."""
import sys, re, pathlib
p = pathlib.Path('/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/r5-engine-architect-b-fixes.md')
t = p.read_text(encoding='utf8')
blocks = re.findall(r'(### (?:FIX|NOTE) (\S+)\n.*?\nEND\n\n)', t, re.S)
ids = [b[1] for b in blocks]
fid = sys.argv[1]
if ids.count(fid) != 2 or ids[-1] != fid:
    sys.exit(f'need the old block and a new last block with id {fid}: {ids.count(fid)} found, last {ids[-1]}')
new = blocks[-1][0]
old = blocks[ids.index(fid)][0]
t = t[:t.rfind(new)]
assert t.count(old) == 1
t = t.replace(old, new)
p.write_text(t, encoding='utf8'); print('replaced', fid)
