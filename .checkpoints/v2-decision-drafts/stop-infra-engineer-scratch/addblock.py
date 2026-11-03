#!/usr/bin/env python3
"""Append one FIX block to the fixes file. BEFORE = lines a..b (1-based, inclusive) of the doc ON DISK (docs/, never
edited), copied by script, whole lines, checked to occur exactly once. AFTER = the spec file's text, verbatim.
Checks: no AFTER prose line over 120 chars (table rows exempt), no AFTER line equal to END, table rows keep their
cell count when the BEFORE row count matches. Usage: addblock.py <id> <a> <b> <spec> [--dry]"""
import sys, pathlib
ROOT = pathlib.Path('/home/user/Shamble-Frontlines')
REL = 'docs/adr/0006-infrastructure-operations.md'
DOC = ROOT / REL
OUT = ROOT / '.checkpoints/v2-decision-drafts/stop-infra-engineer-fixes.md'
fid, a, b, spec = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
dry = '--dry' in sys.argv
text = DOC.read_text(encoding='utf8')
lines = text.split('\n')
before = '\n'.join(lines[a - 1:b])
n = text.count(before)
assert n == 1, f'BEFORE occurs {n} times'
after = pathlib.Path(spec).read_text(encoding='utf8').rstrip('\n')
al = after.split('\n')
bad = [(i + 1, len(l)) for i, l in enumerate(al) if len(l) > 120 and not l.lstrip().startswith('|')]
assert not bad, f'long lines {bad}'
assert 'END' not in [l.strip() for l in al] and not any(l.startswith('END') for l in al)
cells = lambda l: l.count('|') - l.count('\\|')
brows = [l for l in before.split('\n') if l.lstrip().startswith('|')]
arows = [l for l in al if l.lstrip().startswith('|')]
for x, y in zip(brows, arows):
    assert cells(x) == cells(y), f'cell count {cells(x)} -> {cells(y)}'
block = f'### FIX {fid}\nFILE: {REL}\nBEFORE:\n{before}\nAFTER:\n{after}\nEND\n\n'
if dry:
    print(block)
else:
    with OUT.open('a', encoding='utf8') as f:
        f.write(block)
    print(f'appended FIX {fid}: lines {a}-{b} ({b - a + 1}) -> {len(al)}')
