# R7: applies the second skeptics' corrections, written as edit operations, to an area's final rows.
# Run from the repo root: python3 -I .checkpoints/r7-legacy-inventory/apply-ops.py <AREA>
# Reads final-<AREA>.json, then fixes2-ops-, fixes3-ops- and fixes4-session-<AREA>.json if present, in that order; writes after-<AREA>.json; prints every touched field before and
# after. An op is {"id", "field", "op": "set"|"replace"|"append", "find", "text"}:
#   set: the field becomes text (look_brief_pending: "true" or "false");
#   replace: find must occur exactly once in the field, and is replaced by text;
#   append: text is added at the end of the field, after one space;
#   remove: field "row", text = the reason; the row leaves the area (a duplicate); listed in removed-<AREA>.json.
# Any op that cannot apply exactly stops the script with an error; nothing is written then.
import json, sys

D = '.checkpoints/r7-legacy-inventory'
a = sys.argv[1]
FIELDS = ['system', 'what', 'where', 'evidence', 'state', 'v2_note', 'mark', 'reason', 'look_brief_pending']
rows = json.load(open(f'{D}/final-{a}.json'))
rows = rows['rows'] if isinstance(rows, dict) else rows
ops = []
for name in (f'fixes2-ops-{a}.json', f'fixes3-ops-{a}.json', f'fixes4-session-{a}.json'):  # 3: final review; 4: session edits
    try: o = json.load(open(f'{D}/{name}'))
    except FileNotFoundError: continue
    ops += o['ops'] if isinstance(o, dict) else o
by_id = {r['id']: r for r in rows}
before = {}
removed = {}
for i, o in enumerate(ops):
    r = by_id.get(o['id'])
    if r is None: sys.exit(f'op {i}: no row {o["id"]}')
    if o['op'] == 'remove':
        if o['field'] != 'row' or not o['text'].strip(): sys.exit(f'op {i}: remove takes field "row" and a reason')
        removed[o['id']] = o['text']; continue
    if o['id'] in removed: sys.exit(f'op {i}: {o["id"]} was removed')
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
rows = [r for r in rows if r['id'] not in removed]
json.dump(rows, open(f'{D}/after-{a}.json', 'w'), indent=1, ensure_ascii=False)
json.dump(removed, open(f'{D}/removed-{a}.json', 'w'), indent=1, ensure_ascii=False)
for (rid, f), old in before.items():
    print(f'=== {rid}.{f}\n--- before: {old}\n+++ after:  {by_id[rid][f]}\n')
for k, why in removed.items(): print(f'=== {k} REMOVED: {why}')
print(f'{a}: {len(ops)} ops applied to {len(before)} fields of {len({k[0] for k in before})} rows -> after-{a}.json')
