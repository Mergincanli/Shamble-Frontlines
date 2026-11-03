#!/usr/bin/env python3
"""Append one FIX block to the fixes file: BEFORE = lines start..end (1-based, inclusive) of the doc ON DISK
(the repository's docs, never the scratch tree), AFTER read from stdin. Checks BEFORE occurs exactly once.
Usage: add_block.py <id> <docs/path> <start> <end>  < after.txt"""
import sys, pathlib
REPO = pathlib.Path('/home/user/Shamble-Frontlines')
OUT = REPO / '.checkpoints/v2-decision-drafts/stop-test-engineer-fixes.md'
fid, path, s, e = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4])
lines = (REPO / path).read_text(encoding='utf8').split('\n')
before = '\n'.join(lines[s-1:e])
doc = (REPO / path).read_text(encoding='utf8')
n = doc.count(before)
if n != 1:
    sys.exit(f'BEFORE occurs {n} times in {path}: not written')
after = sys.stdin.read().rstrip('\n')
long_ = [l for l in after.split('\n') if len(l) > 120 and not l.startswith('|')]
if long_:
    sys.exit(f'AFTER has {len(long_)} prose line(s) over 120 chars: ' + ' / '.join(str(len(l)) for l in long_))
for l in after.split('\n'):
    if l.startswith('| PM-'):
        c = l.count(' | ') ; cells = len(l.strip().strip('|').split(' | '))
        if cells != 7: sys.exit(f'PM row has {cells} cells: {l[:40]}')
with OUT.open('a', encoding='utf8') as f:
    f.write(f'### FIX {fid}\nFILE: {path}\nBEFORE:\n{before}\nAFTER:\n{after}\nEND\n\n')
print(f'written FIX {fid}: {path}:{s}-{e} ({e-s+1} line(s) -> {len(after.splitlines())})')
