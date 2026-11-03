#!/usr/bin/env python3
"""Append one FIX block: BEFORE = lines <start>..<end> (1-based, inclusive) copied from the doc on disk,
AFTER = the text of <afterfile>. Checks: BEFORE occurs exactly once; AFTER prose lines <= 120 chars;
table rows keep their cell count. Usage: mkblock.py <id> <start> <end> <afterfile> [--write]"""
import sys, pathlib
DOC = 'docs/adr/0006-infrastructure-operations.md'
root = pathlib.Path('/home/user/Shamble-Frontlines')
fixes = root / '.checkpoints/v2-decision-drafts/r5-infra-engineer-fixes.md'
fid, s, e, af = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
text = (root / DOC).read_text(encoding='utf8')
lines = text.split('\n')
before = '\n'.join(lines[s - 1:e])
after = pathlib.Path(af).read_text(encoding='utf8').rstrip('\n')
ok = True
n = text.count(before)
print(f'{fid}: BEFORE lines {s}-{e}, occurs {n} time(s)'); ok &= n == 1
for i, l in enumerate(after.split('\n')):
    if not l.lstrip().startswith('|') and len(l) > 120:
        print(f'  long AFTER line {i+1}: {len(l)}'); ok = False
bt = [l for l in before.split('\n') if l.lstrip().startswith('|')]
at = [l for l in after.split('\n') if l.lstrip().startswith('|')]
for b, a in zip(bt, at):
    if b.count('|') != a.count('|'):
        print('  cell count differs', b.count('|'), a.count('|')); ok = False
if len(bt) != len(at): print('  table row count differs', len(bt), len(at)); ok = False
for bad in ('\nAFTER:\n', '\nEND'):
    if bad in '\n' + after or bad in '\n' + before: print('  marker inside text:', repr(bad)); ok = False
print('  OK' if ok else '  NOT OK')
if ok and '--write' in sys.argv:
    with fixes.open('a', encoding='utf8') as f:
        f.write(f'### FIX {fid}\nFILE: {DOC}\nBEFORE:\n{before}\nAFTER:\n{after}\nEND\n\n')
    print('  appended to', fixes)
