# R7: applies the second skeptics' corrections, written as edit operations, to an area's final rows.
# Run from the repo root: python3 -I .checkpoints/r7-legacy-inventory/apply-ops.py <AREA>
# Reads final-<AREA>.json and fixes2-ops-<AREA>.json; writes after-<AREA>.json; prints every touched field before and
# after. An op is {"id", "field", "op": "set"|"replace"|"append", "find", "text"}:
#   set: the field becomes text (look_brief_pending: "true" or "false");
#   replace: find must occur exactly once in the field, and is replaced by text;
#   append: text is added at the end of the field, after one space.
# Any op that cannot apply exactly stops the script with an error; nothing is written then.
import json, sys

D = '.checkpoints/r7-legacy-inventory'
a = sys.argv[1]
FIELDS = ['system', 'what', 'where', 'evidence', 'state', 'v2_note', 'mark', 'reason', 'look_brief_pending']
rows = json.load(open(f'{D}/final-{a}.json'))
rows = rows['rows'] if isinstance(rows, dict) else rows
ops = json.load(open(f'{D}/fixes2-ops-{a}.json'))
ops = ops['ops'] if isinstance(ops, dict) else ops
by_id = {r['id']: r for r in rows}
before = {}
for i, o in enumerate(ops):
    r = by_id.get(o['id'])
    if r is None: sys.exit(f'op {i}: no row {o["id"]}')
    f = o['field']
    if f not in FIELDS: sys.exit(f'op {i}: bad field {f}')
    before.setdefault((o['id'], f), r[f])
    if f == 'look_brief_pending':
        if o['op'] != 'set' or o['text'] not in ('true', 'false'): sys.exit(f'op {i}: look_brief_pending takes set true/false')
        r[f] = o['text'] == 'true'; continue
    if f == 'state' and (o['op'] != 'set' or o['text'] not in ('C', 'M', 'D')): sys.exit(f'op {i}: state takes set C/M/D')
    if f == 'mark' and (o['op'] != 'set' or o['text'] not in ('port to TypeScript', 'rewrite in Rust', 'redesign', 'drop')):
        sys.exit(f'op {i}: mark takes set with one of the four marks')
    if o['op'] == 'set':
        r[f] = o['text']
    elif o['op'] == 'replace':
        n = r[f].count(o['find'])
        if n != 1: sys.exit(f'op {i} ({o["id"]}.{f}): find occurs {n} times, must be exactly once: {o["find"][:120]!r}')
        r[f] = r[f].replace(o['find'], o['text'])
    elif o['op'] == 'append':
        r[f] = r[f].rstrip() + ' ' + o['text'].strip()
    else:
        sys.exit(f'op {i}: bad op {o["op"]}')
    if not (isinstance(r[f], str) and r[f].strip()): sys.exit(f'op {i}: {o["id"]}.{f} would be empty')
json.dump(rows, open(f'{D}/after-{a}.json', 'w'), indent=1, ensure_ascii=False)
for (rid, f), old in before.items():
    print(f'=== {rid}.{f}\n--- before: {old}\n+++ after:  {by_id[rid][f]}\n')
print(f'{a}: {len(ops)} ops applied to {len(before)} fields of {len({k[0] for k in before})} rows -> after-{a}.json')
