#!/usr/bin/env python3
"""Per draft: every numbered OQ definition ("- **OQ-...") and the first two lines of its body, flagging a status word
(answered, moved, closed, settled, resolved, withdrawn, decided). Usage: oq_status.py <docs dir>"""
import re, sys, glob, collections
d = sys.argv[1]
tot = collections.Counter()
for f in sorted(glob.glob(d + '/adr/000[1-6]-*.md')):
    L = open(f, encoding='utf8').read().split('\n')
    ids = []; flagged = []
    for i, l in enumerate(L):
        m = re.match(r'\s*- \*\*(OQ-\d{4}-[A-Z]*\d+)\b', l)
        if not m: continue
        oid = m.group(1)
        if oid in [x for x, _ in ids]: continue
        ids.append((oid, i + 1))
        body = ' '.join(L[i:i + 3])
        w = re.search(r'\b(Answered|Moved|moved to|Closed|Settled|Resolved|Withdrawn|Decided|Points to|points to)\b', body)
        if w: flagged.append((oid, i + 1, w.group(1)))
    print(f'{f.split("/")[-1][:4]}: numbered {len(ids)}; flagged {len(flagged)}: ' + '; '.join(f'{a} ({b}, "{c}")' for a, b, c in flagged))
    tot['n'] += len(ids); tot['f'] += len(flagged)
print('TOTAL numbered', tot['n'], 'flagged', tot['f'])
