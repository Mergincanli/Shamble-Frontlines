#!/usr/bin/env python3
"""Count dispositions: every unique research id (inputs/research-ids.tsv) must have exactly one disposition row across
the six drafts' 'Appendix A' tables, docs/adr/README.md's register and docs/SPIKE_PLAN.md (plan rule 8, condition 6).
A disposition row is a table row inside those sections whose first cell names one or more ids (comma or slash separated,
backticks allowed). Also checks that each id is disposed where the decision map's id table says (owner), and flags the
rest. Usage: dispositions.py <research-ids.tsv> <decision-map-ids.tsv> <file>..."""
import sys, re, csv, collections, pathlib
ids = set(); 
with open(sys.argv[1], encoding='utf8') as f:
    for r in csv.DictReader(f, delimiter='\t'): ids.add(r['id'])
owner = {}
with open(sys.argv[2], encoding='utf8') as f:
    for r in csv.DictReader(f, delimiter='\t'): owner[r['id'].strip()] = r['owner'].strip()
idre = re.compile(r'\bR[1-6]-[A-Z0-9]+-(?:XU-OQ|XT-OQ|RR-OQ|REC|OQ)[0-9]+\b|\b(?:MODES-3|PREVIEW-WALL|BASELINE-WHEN|SEC-SPREAD|ABL-PREDICT|PREV-DELETE)\b')
seen = collections.defaultdict(list)
for fn in sys.argv[3:]:
    p = pathlib.Path(fn); text = p.read_text(encoding='utf8')
    name = p.name
    where = '0001' if name.startswith('0001') else '0002' if name.startswith('0002') else '0003' if name.startswith('0003') else '0004' if name.startswith('0004') else '0005' if name.startswith('0005') else '0006' if name.startswith('0006') else 'DESIGN' if name == 'README.md' else 'SPIKE_PLAN' if name == 'SPIKE_PLAN.md' else name
    # sections that hold dispositions
    if where in ('DESIGN',):
        m = re.search(r'^##+ .*(?:register|design questions|DESIGN).*$', text, re.M | re.I)
        sect = text[m.start():] if m else ''
    elif where == 'SPIKE_PLAN':
        m = re.search(r'^##+ .*dispositions.*$', text, re.M | re.I)
        sect = text[m.start():] if m else ''
    else:
        m = re.search(r'^## Appendix A.*$', text, re.M)
        sect = text[m.start():] if m else ''
        n = re.search(r'^## (?!Appendix A)', sect[3:], re.M)
        if n: sect = sect[:n.start()+3]
    for line in sect.split('\n'):
        if not line.startswith('|') or line.startswith('|---') or line.startswith('| ---'): continue
        cells = line.split('|')
        if len(cells) < 3: continue
        for i in idre.findall(cells[1]):
            seen[i].append(where)
missing = sorted(ids - set(seen))
dups = {k: v for k, v in seen.items() if len(v) > 1}
wrong_place = {k: (v, owner.get(k)) for k, v in seen.items() if owner.get(k) and owner.get(k) not in v}
unknown = sorted(set(seen) - ids)
print(f'ids: {len(ids)}; disposed: {len(set(seen) & ids)}; missing: {len(missing)}; more than once: {len(dups)}; not where the map puts them: {len(wrong_place)}; unknown ids: {len(unknown)}')
if missing: print('MISSING', ' '.join(missing[:200]))
for k, v in list(dups.items())[:60]: print('DUP', k, v)
for k, v in list(wrong_place.items())[:60]: print('PLACE', k, 'disposed in', v[0], 'map owner', v[1])
if unknown: print('UNKNOWN', unknown[:40])
by = collections.Counter(w for v in seen.values() for w in v)
print('rows per place:', dict(sorted(by.items())))
sys.exit(1 if (missing or dups) else 0)
