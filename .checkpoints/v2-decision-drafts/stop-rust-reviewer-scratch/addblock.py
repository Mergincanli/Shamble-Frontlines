#!/usr/bin/env python3
"""Append one FIX block: addblock.py <fixes.md> <block id> <repo-relative doc> <first line> <last line> <after-file>
BEFORE = whole lines first..last of the doc on disk (the real repository file); checks it occurs exactly once."""
import sys, pathlib
fixes, bid, doc, a, b, after_f = sys.argv[1:7]
src = pathlib.Path('/home/user/Shamble-Frontlines/' + doc).read_text(encoding='utf8')
lines = src.split('\n')
before = '\n'.join(lines[int(a)-1:int(b)])
after = pathlib.Path(after_f).read_text(encoding='utf8').rstrip('\n')
assert src.count(before) == 1, f'BEFORE occurs {src.count(before)} times'
for l in after.split('\n'):
    if len(l) > 120 and not l.startswith('|'):
        sys.exit(f'line over 120: {len(l)}: {l[:80]}')
if before.startswith('|'):
    for x, y in zip(before.split('\n'), after.split('\n')):
        assert x.count('|') == y.count('|'), 'cell count differs'
with open(fixes, 'a', encoding='utf8') as f:
    f.write(f'### FIX {bid}\nFILE: {doc}\nBEFORE:\n{before}\nAFTER:\n{after}\nEND\n\n')
print('added', bid, f'{int(b)-int(a)+1} line(s) ->', len(after.split('\n')))
