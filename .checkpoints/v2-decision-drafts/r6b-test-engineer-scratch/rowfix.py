#!/usr/bin/env python3
"""rowfix.py <id> <docs path> <PM id> <json file of [[old,new],...]>: FIX block for one table row, BEFORE copied by
script from disk; each old must occur exactly once in the row; cell count must stay the same."""
import sys, json, pathlib
REPO = pathlib.Path('/home/user/Shamble-Frontlines')
OUT = REPO / '.checkpoints/v2-decision-drafts/r6b-test-engineer-fixes.md'
fid, path, pm, jf = sys.argv[1:5]
doc = (REPO / path).read_text(encoding='utf8')
rows = [l for l in doc.split('\n') if l.startswith(f'| {pm} |')]
assert len(rows) == 1, rows
row = rows[0]; new = row
for old, rep in json.loads(pathlib.Path(jf).read_text(encoding='utf8')):
    n = new.count(old)
    if n != 1: sys.exit(f'{old!r} occurs {n} times')
    new = new.replace(old, rep, 1)
cells = lambda r: len(r.strip().strip('|').split(' | '))
if cells(row) != cells(new): sys.exit(f'cell count {cells(row)} -> {cells(new)}')
assert doc.count(row) == 1
with OUT.open('a', encoding='utf8') as f:
    f.write(f'### FIX {fid}\nFILE: {path}\nBEFORE:\n{row}\nAFTER:\n{new}\nEND\n\n')
print('appended', fid, 'cells', cells(new))
