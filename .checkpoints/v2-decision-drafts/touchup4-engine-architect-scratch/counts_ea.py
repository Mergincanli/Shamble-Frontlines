#!/usr/bin/env python3
"""Check the count statements in docs/adr/README.md, docs/SPIKE_PLAN.md and docs/ENGINE_OVERVIEW.md against their
sources. Usage: counts_ea.py <repo or scratch root> <real repo root (for STATUS, LEGACY_INVENTORY, inputs)>"""
import re, sys, glob
root, real = sys.argv[1], sys.argv[2]
rd = lambda p: open(p, encoding='utf8').read()
R = rd(root + '/docs/adr/README.md'); RL = R.split('\n')
drafts = {re.search(r'(000\d)', f).group(1): rd(f) for f in sorted(glob.glob(root + '/docs/adr/000[1-6]-*.md'))}
def table_after(lines, start_pat):
    i = next(k for k, l in enumerate(lines) if re.search(start_pat, l))
    j = i + 1
    while not lines[j].startswith('|'): j += 1
    rows = []
    while j < len(lines) and lines[j].startswith('|'): rows.append(lines[j]); j += 1
    return rows[2:]
res = []
def chk(what, stated, actual): res.append((what, stated, actual, 'OK' if str(stated) == str(actual) else 'DIFF'))
# A. per-draft OQs
tot_n = tot_open = 0
for d, t in drafts.items():
    L = t.split('\n'); ids = []; closed = []
    for i, l in enumerate(L):
        m = re.match(r'\s*- \*\*(OQ-\d{4}-[A-Z]*\d+)\b', l)
        if m and m.group(1) not in ids:
            ids.append(m.group(1)); body = ' '.join(L[i:i + 2])
            if re.search(r'\*\*\s*(Moved|Answered)\b|^\s*(Answered|Moved)\b', body.split('**', 2)[-1] + ' ' + (L[i+1] if i+1 < len(L) else '')):
                closed.append(m.group(1))
    row = next(l for l in RL if l.startswith('| [' + d + ']'))
    cell = row.rstrip('|').split('|')[-1].strip()
    n_st = int(re.match(r'(\d+)', cell).group(1)); o_m = re.search(r'(\d+) open', cell)
    chk(f'{d} numbered OQs (README drafts table)', n_st, len(ids))
    chk(f'{d} open OQs (README drafts table; stated = numbered when no "open" figure)', int(o_m.group(1)) if o_m else n_st, len(ids) - len(closed))
    tot_n += len(ids); tot_open += len(ids) - len(closed)
    res.append((f'{d} not open', '', ', '.join(closed), ''))
m = re.search(r'Total: (\d+) numbered open questions, (\d+) of them open', R.replace('\n', ' '))
chk('total numbered (README Total)', m.group(1), tot_n); chk('total open (README Total)', m.group(2), tot_open)
# B. PM rows
m = re.search(r'Pass marks: ([\d, and]+) rows \((\d+)\)', R.replace('\n', ' '))
st = [int(x) for x in re.findall(r'\d+', m.group(1))]
act = [len(set(re.findall(r'^\| (PM-' + d + r'-\d+) \|', t, re.M))) for d, t in drafts.items()]
chk('PM rows per draft', st, act); chk('PM total', m.group(2), sum(act))
# C. design questions
dq = table_after(RL, r'^## Design questions'); chk('design questions rows (README:50, LE-J "38")', 38, len(dq))
# D. carry-forwards
cf = table_after(RL, r"STATUS's five carry-forwards"); chk('carry-forward rows ("five")', 5, len(cf))
S = rd(real + '/docs/STATUS.md'); i = S.index('The five carry-forwards into the decision drafts')
chk('STATUS carry-forward items', 5, len(re.findall(r'^\s+\d+\. ', S[i:i + 4000].split('\n- **')[0], re.M)) or 'see by hand')
# E. XU ids
ids_tsv = rd(real + '/.checkpoints/v2-decision-drafts/inputs/research-ids.tsv')
chk('XU ids ("XU (30 ids)")', 30, len(set(re.findall(r'\bR\d-XU-[A-Z]*\d+\b', ids_tsv))))
# F. critic ranks
cr = table_after(RL, r'The 28 critic ranks'); chk('critic rank rows ("28")', 28, len(cr))
# G. LE-H
leh = next(l for l in RL if l.startswith('| LE-H'))
m = re.search(r'the (\d+) rows no draft had listed \(([^)]*)\)', leh)
n = 0
for tok in [x.strip() for x in m.group(2).split(',')]:
    r2 = re.match(r'([A-Z]+)(\d+)[–-]\1?(\d+)$', tok)
    n += (int(r2.group(3)) - int(r2.group(2)) + 1) if r2 else 1
chk('LE-H rows listed ("18")', m.group(1), n)
LI = rd(real + '/docs/LEGACY_INVENTORY.md')
chk('LE-H inventory rows look brief pending ("118")', re.search(r'(\d+) inventory rows', leh).group(1), re.search(r'Look brief pending on (\d+) rows', LI).group(1))
# H. old ADR entries
old = table_after(RL, r'The old ADR entries'); res.append(('old ADR entries table rows (text says "three entries replaced")', 3, len(old), 'see by hand'))
# I. SPIKE_PLAN
SP = rd(root + '/docs/SPIKE_PLAN.md'); SL = SP.split('\n')
st_rows = [l for l in SL if re.match(r'^\| (\d+|–) \| ', l)]
num = [l for l in st_rows if re.match(r'^\| \d+ ', l)]
chk('spike rows numbered', '1..10', f'1..{len(num)}' if [int(l.split("|")[1]) for l in num] == list(range(1, len(num) + 1)) else 'gap')
w1 = [l for l in num if l.split('|')[4].strip().startswith('wave 1')]
chk('wave 1 spikes in parallel ("five"; the 60 Hz run is inside 1)', 5, len([l for l in w1 if 'inside' not in l.split('|')[3]]))
pm4 = sorted(set(int(x) for x in re.findall(r'^\| PM-0004-(\d+) \|', drafts['0004'], re.M)))
sim = [x for x in pm4 if x <= 61 or x in (73, 74)]
res.append(('sim core spike marks ("about 60"): PM-0004-1..61, 73, 74', 'about 60', len(sim), 'OK' if 55 <= len(sim) <= 65 else 'DIFF'))
p19 = next(l for l in drafts['0004'].split('\n') if l.startswith('| PM-0004-19 |'))
res.append(('PM-0004-19 targets ("seven")', 'seven', 'seven' if re.search(r'\bseven\b', p19) else p19[:200], ''))
chk('six decision drafts (OVERVIEW, SPIKE_PLAN)', 6, len(drafts))
OV = rd(root + '/docs/ENGINE_OVERVIEW.md'); chk('ENGINE_OVERVIEW lines (report says 144)', 144, len(OV.rstrip('\n').split('\n')))
for r in res: print(' | '.join(str(x) for x in r))
