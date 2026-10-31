#!/usr/bin/env python3
"""Check the decision map's ownership TSV against the canonical research id list.
Usage: map_coverage.py research-ids.tsv decision-map-ids.tsv
Every unique id of research-ids.tsv (shared codes once) must appear exactly once in the map, with a valid owner."""
import sys, csv, collections
ids_file, map_file = sys.argv[1], sys.argv[2]
canon = []
with open(ids_file, encoding='utf8') as f:
    for row in csv.DictReader(f, delimiter='\t'):
        canon.append(row['id'])
canon_set = set(canon)
owners_ok = {'0001','0002','0003','0004','0005','0006','SPIKE_PLAN','DESIGN'}
seen = collections.Counter(); bad_owner = []; unknown = []; rows = 0
per_owner = collections.Counter()
with open(map_file, encoding='utf8') as f:
    rd = csv.DictReader(f, delimiter='\t')
    for row in rd:
        rows += 1
        i = (row.get('id') or '').strip(); o = (row.get('owner') or '').strip()
        if not i: continue
        seen[i] += 1
        if i not in canon_set: unknown.append(i)
        if o not in owners_ok: bad_owner.append((i, o))
        per_owner[o] += 1
missing = sorted(canon_set - set(seen))
dups = sorted(k for k, v in seen.items() if v > 1)
print(f'canonical unique ids: {len(canon_set)}; map rows: {rows}; ids in map: {len(seen)}')
print(f'missing from map: {len(missing)} {missing[:40]}')
print(f'listed more than once: {len(dups)} {dups[:40]}')
print(f'not in the canonical list: {len(unknown)} {unknown[:40]}')
print(f'bad owner values: {len(bad_owner)} {bad_owner[:20]}')
print('per owner:', dict(sorted(per_owner.items())))
sys.exit(1 if (missing or dups or unknown or bad_owner) else 0)
