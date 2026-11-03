#!/usr/bin/env python3
"""Append one FIX block to the fixes file: BEFORE = whole lines a..b (1-based, inclusive) copied from the doc on disk,
checked to occur exactly once; AFTER read from a file. Usage: addblock.py <fixes> <id> <docs path> <a> <b> <after file>"""
import sys
fixes, fid, path, a, b, aft = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4]), int(sys.argv[5]), sys.argv[6]
doc = open('/home/user/Shamble-Frontlines/' + path, encoding='utf8').read()
L = doc.split('\n'); before = '\n'.join(L[a - 1:b])
assert doc.count(before) == 1, 'BEFORE not unique'
after = open(aft, encoding='utf8').read().rstrip('\n')
for l in after.split('\n'):
    if len(l) > 120 and not l.startswith('|'): raise SystemExit(f'long line {len(l)}: {l[:60]}')
with open(fixes, 'a', encoding='utf8') as f:
    f.write(f'\n### FIX {fid}\nFILE: {path}\nBEFORE:\n{before}\nAFTER:\n{after}\nEND\n')
print(fid, f'lines {a}-{b} ({b - a + 1}) -> {len(after.split(chr(10)))}')
