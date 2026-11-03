#!/usr/bin/env python3
"""addblock.py <id> <docs path> <start> <end> <after-file>: append a FIX block whose BEFORE is lines start..end
(1-based, inclusive) of the doc on disk, copied by script; checks BEFORE occurs once and AFTER prose lines <= 120."""
import sys, pathlib
REPO = pathlib.Path('/home/user/Shamble-Frontlines')
OUT = REPO / '.checkpoints/v2-decision-drafts/r6b-test-engineer-fixes.md'
fid, path, s, e, af = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4]), sys.argv[5]
doc = (REPO / path).read_text(encoding='utf8')
lines = doc.split('\n')
before = '\n'.join(lines[s-1:e])
after = pathlib.Path(af).read_text(encoding='utf8').rstrip('\n')
n = doc.count(before)
if n != 1:
    sys.exit(f'BEFORE occurs {n} times')
bad = [l for l in after.split('\n') if not l.startswith('|') and len(l) > 120 and '](' not in l]
if bad:
    sys.exit('long prose lines:\n' + '\n'.join(f'{len(l)}: {l}' for l in bad))
for l in after.split('\n'):
    if l.startswith('| PM-'):
        c = l.count(' | ') ; print('cells', l.split(' | ')[0], len(l.strip('|').split(' | ')))
with OUT.open('a', encoding='utf8') as f:
    f.write(f'### FIX {fid}\nFILE: {path}\nBEFORE:\n{before}\nAFTER:\n{after}\nEND\n\n')
print('appended', fid, f'{e-s+1} line(s) -> {len(after.split(chr(10)))}')
