#!/usr/bin/env python3
"""Append one FIX block to the fixes file; BEFORE is copied from the doc on disk by line numbers.
usage: addblock.py <row id> <start> <end> <after-text-file>"""
import sys
ROOT = '/home/user/Shamble-Frontlines/'
DOC = 'docs/adr/0005-content-abilities-assets.md'
OUT = ROOT + '.checkpoints/v2-decision-drafts/stop-game-designer-fixes.md'
rid, s, e, after = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
lines = open(ROOT + DOC, encoding='utf-8').read().split('\n')
before = '\n'.join(lines[s-1:e])
text = open(ROOT + DOC, encoding='utf-8').read()
n = text.count(before)
assert n == 1, f'BEFORE occurs {n} times'
aft = open(after, encoding='utf-8').read().rstrip('\n')
for ln in aft.split('\n'):
    assert len(ln) <= 120, f'line over 120 ({len(ln)}): {ln}'
with open(OUT, 'a', encoding='utf-8') as f:
    f.write(f'### FIX {rid}\nFILE: {DOC}\nBEFORE:\n{before}\nAFTER:\n{aft}\nEND\n\n')
print('appended', rid, 'BEFORE lines', s, e)
