#!/usr/bin/env python3
"""mkfix.py <doc> <start> <end> <old_substr> <new_substr> [--rewrap indent] : print a FIX body (BEFORE copied from disk, whole lines)
with one substring replaced; checks BEFORE occurs once; optional rewrap of the paragraph lines at width 120."""
import sys, textwrap
doc, a, b, old, new = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4], sys.argv[5]
lines = open(doc, encoding='utf8').read().split('\n')
before = '\n'.join(lines[a-1:b])
full = open(doc, encoding='utf8').read()
assert full.count(before) == 1, 'BEFORE not unique'
assert before.count(old) == 1, f'old occurs {before.count(old)} times'
after = before.replace(old, new)
if '--rewrap' in sys.argv:
    ind = int(sys.argv[sys.argv.index('--rewrap')+1])
    first = lines[a-1]; lead = first[:len(first)-len(first.lstrip())]
    txt = ' '.join(l.strip() for l in after.split('\n'))
    after = '\n'.join(textwrap.wrap(txt, 120, initial_indent=lead, subsequent_indent=' '*ind, break_long_words=False, break_on_hyphens=False))
print('BEFORE:'); print(before); print('AFTER:'); print(after); print('END')
sys.stderr.write(f'lines {len(before.splitlines())} -> {len(after.splitlines())}\n')
