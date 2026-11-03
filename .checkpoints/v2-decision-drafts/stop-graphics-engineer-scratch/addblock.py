#!/usr/bin/env python3
"""addblock.py <id> <docpath-rel> <start> <end> <after-file>
Copies lines start..end (1-based, inclusive) of the doc on disk as BEFORE, checks the text occurs exactly once,
checks AFTER lines <= 120 chars, appends a FIX block to the fixes file."""
import sys, pathlib
ROOT = pathlib.Path('/home/user/Shamble-Frontlines')
FIXES = ROOT / '.checkpoints/v2-decision-drafts/stop-graphics-engineer-fixes.md'
bid, rel, s, e, after = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4]), sys.argv[5]
doc = (ROOT / rel).read_text(encoding='utf-8')
lines = doc.split('\n')
before = '\n'.join(lines[s-1:e])
n = doc.count(before)
if n != 1:
    sys.exit(f'BEFORE occurs {n} times')
aft = pathlib.Path(after).read_text(encoding='utf-8').rstrip('\n')
for i, l in enumerate(aft.split('\n')):
    if len(l) > 120 and l not in lines:
        print(f'WARN long new line {i+1}: {len(l)}')
if '### FIX ' + bid + '\n' in FIXES.read_text(encoding='utf-8'):
    sys.exit('block id exists')
with FIXES.open('a', encoding='utf-8') as f:
    f.write(f'### FIX {bid}\nFILE: {rel}\nBEFORE:\n{before}\nAFTER:\n{aft}\nEND\n\n')
print('added', bid, f'{s}-{e}')
