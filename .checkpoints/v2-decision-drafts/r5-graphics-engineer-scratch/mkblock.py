#!/usr/bin/env python3
"""Append one FIX block: BEFORE = whole lines start..end of the doc on disk (must occur exactly once),
AFTER = the text of an after-file. Usage: mkblock.py <id> <docs path> <start> <end> <after file>"""
import sys, pathlib
fid, rel, s, e, aft = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4]), sys.argv[5]
root = pathlib.Path('/home/user/Shamble-Frontlines')
lines = (root / rel).read_text(encoding='utf8').split('\n')
before = '\n'.join(lines[s - 1:e])
doc = (root / rel).read_text(encoding='utf8')
n = doc.count(before)
assert n == 1, f'BEFORE occurs {n} times'
after = pathlib.Path(aft).read_text(encoding='utf8').rstrip('\n')
for l in after.split('\n'):
    if not l.startswith('|') and len(l) > 120: sys.exit(f'line over 120 ({len(l)}): {l}')
out = root / '.checkpoints/v2-decision-drafts/r5-graphics-engineer-fixes.md'
with open(out, 'a', encoding='utf8') as f:
    f.write(f'### FIX {fid}\nFILE: {rel}\nBEFORE:\n{before}\nAFTER:\n{after}\nEND\n\n')
print('appended', fid, f'{e - s + 1} line(s) -> {len(after.splitlines())}')
