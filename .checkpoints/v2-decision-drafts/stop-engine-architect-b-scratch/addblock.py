#!/usr/bin/env python3
"""Append one FIX block (BEFORE = whole lines start..end copied from the doc; AFTER from stdin) or one NOTE block
(reason from stdin) to the fixes file. Usage: addblock.py fix <id> <docs path> <start> <end> [--src <copy>] < after
                                          addblock.py note <id> < reason
--src: take BEFORE from another copy (the pointer step: a scratch README with the earlier blocks applied)."""
import sys, pathlib
FIX = pathlib.Path('/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/stop-engine-architect-b-fixes.md')
ROOT = pathlib.Path('/home/user/Shamble-Frontlines')
mode, bid = sys.argv[1], sys.argv[2]
text = FIX.read_text(encoding='utf8')
if f'### FIX {bid}\n' in text or f'### NOTE {bid}\n' in text:
    sys.exit(f'block {bid} already in the file')
body = sys.stdin.read().rstrip('\n')
if mode == 'note':
    FIX.write_text(text + f'### NOTE {bid}\n{body}\nEND\n\n', encoding='utf8'); print('note', bid); sys.exit()
doc, a, b = sys.argv[3], int(sys.argv[4]), int(sys.argv[5])
src = pathlib.Path(sys.argv[sys.argv.index('--src') + 1]) if '--src' in sys.argv else ROOT / doc
lines = src.read_text(encoding='utf8').split('\n')
before = '\n'.join(lines[a - 1:b])
n = src.read_text(encoding='utf8').count(before)
if n != 1:
    sys.exit(f'{bid}: BEFORE occurs {n} times')
for i, l in enumerate(body.split('\n'), 1):
    if len(l) > 120 and not l.startswith('|'):
        print(f'WARN {bid}: AFTER line {i} has {len(l)} chars')
FIX.write_text(text + f'### FIX {bid}\nFILE: {doc}\nBEFORE:\n{before}\nAFTER:\n{body}\nEND\n\n', encoding='utf8')
print('fix', bid, f'{doc}:{a}-{b}', f'{b - a + 1} -> {len(body.splitlines())} lines')
