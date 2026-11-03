#!/usr/bin/env python3
"""Append one FIX block to the fixes file: BEFORE = whole lines start..end (1-based, inclusive) copied from the doc on
disk; AFTER read from a file. Checks BEFORE occurs exactly once and AFTER lines <= 120 chars (prose).
Usage: addblock.py <id> <docs path> <start> <end> <after-file>"""
import sys, pathlib
REPO = pathlib.Path('/home/user/Shamble-Frontlines')
OUT = REPO / '.checkpoints/v2-decision-drafts/r5-engine-architect-a-fixes.md'
fid, docp, s, e, af = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4]), sys.argv[5]
lines = (REPO / docp).read_text(encoding='utf8').split('\n')
before = '\n'.join(lines[s-1:e])
doc = (REPO / docp).read_text(encoding='utf8')
n = doc.count(before)
assert n == 1, f'BEFORE occurs {n} times'
after = pathlib.Path(af).read_text(encoding='utf8').rstrip('\n')
for i, l in enumerate(after.split('\n')):
    if not l.lstrip().startswith('|') and len(l) > 120:
        print(f'WARN after line {i+1} is {len(l)} chars'); sys.exit(1)
with OUT.open('a', encoding='utf8') as f:
    f.write(f'### FIX {fid}\nFILE: {docp}\nBEFORE:\n{before}\nAFTER:\n{after}\nEND\n\n')
print('appended', fid, f'{e-s+1} line(s) ->', len(after.split('\n')))
