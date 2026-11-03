#!/usr/bin/env python3
"""Append one FIX block to the fixes file. BEFORE = whole lines a..b (1-based, inclusive) copied from the doc on disk
(the repository's docs, never the scratch copy), checked to occur exactly once; AFTER read from a file.
Checks: every AFTER line that is new (not one of BEFORE's lines) and is prose is at most 120 characters; a table row
keeps the cell count of the BEFORE row it replaces (same position), and a new table row has the cells of its table.
Usage: addblock.py <fixes> <id> <docs path> <a> <b> <after file>
       addblock.py --note <fixes> <id> <text file>"""
import sys, re
if sys.argv[1] == '--note':
    fixes, fid, txt = sys.argv[2], sys.argv[3], sys.argv[4]
    body = open(txt, encoding='utf8').read().rstrip('\n')
    assert body.startswith('no change: '), 'a NOTE starts "no change: "'
    for l in body.split('\n'):
        if len(l) > 120: raise SystemExit(f'long line {len(l)}: {l[:60]}')
    with open(fixes, 'a', encoding='utf8') as f:
        f.write(f'\n### NOTE {fid}\n{body}\nEND\n')
    print('NOTE', fid); sys.exit(0)
fixes, fid, path, a, b, aft = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4]), int(sys.argv[5]), sys.argv[6]
doc = open('/home/user/Shamble-Frontlines/' + path, encoding='utf8').read()
L = doc.split('\n'); before = '\n'.join(L[a - 1:b])
assert doc.count(before) == 1, 'BEFORE not unique'
after = open(aft, encoding='utf8').read().rstrip('\n')
def cells(l):
    s = l.strip()
    if s.startswith('|'): s = s[1:]
    if s.endswith('|') and not s.endswith('\\|'): s = s[:-1]
    return len(re.split(r'(?<!\\)\|', s))
bl = before.split('\n'); al = after.split('\n')
for l in al:
    if l in bl: continue
    if not l.startswith('|') and len(l) > 120: raise SystemExit(f'long line {len(l)}: {l[:60]}')
brows = [x for x in bl if x.startswith('|')]; arows = [x for x in al if x.startswith('|')]
if brows or arows:
    if len(brows) != len(arows): raise SystemExit(f'table rows {len(brows)} -> {len(arows)}')
    for x, y in zip(brows, arows):
        if cells(x) != cells(y): raise SystemExit(f'cells {cells(x)} -> {cells(y)}: {y[:60]}')
with open(fixes, 'a', encoding='utf8') as f:
    f.write(f'\n### FIX {fid}\nFILE: {path}\nBEFORE:\n{before}\nAFTER:\n{after}\nEND\n')
print(fid, f'lines {a}-{b} ({b - a + 1}) -> {len(al)}')
