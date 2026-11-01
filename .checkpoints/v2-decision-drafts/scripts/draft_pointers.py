#!/usr/bin/env python3
"""Check pointers into the drafts: in docs/adr/README.md, every "[ADR 000N](...) line L" in a row whose first cell
names an OQ id must land on (L..L+1 of) that id's line in the draft; prints the rows that are off and where the id is
now. Usage: draft_pointers.py (from the repository root)."""
import re, glob
drafts = {re.search(r'(000\d)', f).group(1): open(f, encoding='utf8').read().split('\n') for f in glob.glob('docs/adr/000[1-6]-*.md')}
R = open('docs/adr/README.md', encoding='utf8').read().split('\n')
ok = bad = 0
for i, l in enumerate(R, 1):
    for m in re.finditer(r'\[ADR (000\d)\]\([^)]*\)[^|;]*?\blines? (\d+)(?:[–-](\d+))?', l):
        d, a = m.group(1), int(m.group(2)); b = int(m.group(3) or a)
        first = l.split('|')[1] if l.startswith('|') else l
        ids = re.findall(r'OQ-\d{4}-[A-Z]*\d+', first)
        if not ids: continue
        if any(x in '\n'.join(drafts[d][a-1:b+1]) for x in ids): ok += 1; continue
        bad += 1
        now = [j+1 for j, t in enumerate(drafts[d]) if re.search(r'\*\*' + re.escape(ids[0]) + r'\b', t)]
        print(f'README:{i} {ids[0]} cited ADR {d} line {a}, now at {now[:2]}')
print(f'pointers checked {ok+bad}: ok {ok}, off {bad}')
