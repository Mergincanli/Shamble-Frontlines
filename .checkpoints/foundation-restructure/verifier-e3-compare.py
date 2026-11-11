# Verifier (final, 2.1a): compare two E3 result files per id and column. Read-only.
# usage: python3 -I verifier-e3-compare.py <recorded.json> <new.json> [--ids a,b]
import json, sys
a = json.load(open(sys.argv[1]))
b = json.load(open(sys.argv[2]))
ids = None
if '--ids' in sys.argv:
    ids = sys.argv[sys.argv.index('--ids') + 1].split(',')
diffs = 0
keys = ids or sorted(set(a) | set(b))
print('ids recorded', len(a), 'new', len(b), 'compared', len(keys))
for k in keys:
    if k not in a or k not in b:
        print('MISSING', k, k in a, k in b); diffs += 1; continue
    ea, eb = a[k], b[k]
    for top in ('verdict', 'expected', 'flags', 'reportChecksInOld'):
        if top in ea and top in eb and ea[top] != eb[top]:
            print('DIFF', k, top, ea[top], '->', eb[top]); diffs += 1
    for col in ('old', 'new', 'newleg'):
        ca, cb = ea.get(col), eb.get(col)
        if ca is None and cb is None: continue
        if (ca is None) != (cb is None):
            print('DIFF', k, col, 'presence'); diffs += 1; continue
        for f in ('exit', 'failing', 'basePrefix', 'build'):
            if ca.get(f) != cb.get(f):
                print('DIFF', k, col, f, ca.get(f), '->', cb.get(f)); diffs += 1
        # PASS/FAIL word per check line
        la = {c: (v.split(' ', 1)[0]) for c, v in (ca.get('lines') or {}).items()}
        lb = {c: (v.split(' ', 1)[0]) for c, v in (cb.get('lines') or {}).items()}
        if la != lb:
            print('DIFF', k, col, 'line verdicts', la, '->', lb); diffs += 1
        fa = {c: v for c, v in (ca.get('lines') or {}).items() if c != '5a'}
        fb = {c: v for c, v in (cb.get('lines') or {}).items() if c != '5a'}
        if '--full' in sys.argv and fa != fb:
            print('DIFF', k, col, 'non-5a line text', [c for c in set(fa) | set(fb) if fa.get(c) != fb.get(c)]); diffs += 1
print('TOTAL DIFFS', diffs)
