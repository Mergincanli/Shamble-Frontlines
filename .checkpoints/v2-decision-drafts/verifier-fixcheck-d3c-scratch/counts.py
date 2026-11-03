#!/usr/bin/env python3
"""Task 3 (fixcheck-d3c): counts at a ref, against README and the report. Per draft: OQ ids defined (each once,
numbered without gaps per prefix), open vs moved; PM rows (each once, 1..n without gaps); README's drafts table cells
and totals line; README's design questions (rows; against the decision map's DESIGN-owned ids); README's decisions
table (rows, duplicates; every OQ the drafts route to Mustafa or the Advisor present); ENGINE_OVERVIEW's line count;
SPIKE_PLAN's spikes. Usage: counts.py <ref>"""
import re, subprocess, sys, collections
REPO = '/home/user/Shamble-Frontlines'; ref = sys.argv[1] if len(sys.argv) > 1 else 'HEAD'
ADRS = {'0001': 'docs/adr/0001-multiplayer-netcode.md', '0002': 'docs/adr/0002-code-structure-build-toolchain.md',
        '0003': 'docs/adr/0003-rendering.md', '0004': 'docs/adr/0004-simulation-core.md',
        '0005': 'docs/adr/0005-content-abilities-assets.md', '0006': 'docs/adr/0006-infrastructure-operations.md'}
show = lambda p: subprocess.run(['git', '-C', REPO, 'show', f'{ref}:{p}'], capture_output=True, text=True).stdout
R = show('docs/adr/README.md')
tot_num = tot_open = tot_pm = 0; routed = {}
for n, d in ADRS.items():
    t = show(d); L = t.split('\n')
    heads = [(i, m.group(1), l) for i, l in enumerate(L, 1) for m in [re.match(r'^\s*- \*\*(OQ-%s-[A-Z]*\d+)[.*]' % n, l)] if m]
    ids = [h[1] for h in heads]; c = collections.Counter(ids)
    dup = [k for k, v in c.items() if v > 1]
    groups = collections.defaultdict(list)
    for k in ids:
        m = re.match(r'OQ-\d{4}-([A-Z]*)(\d+)$', k); groups[m.group(1)].append(int(m.group(2)))
    gaps = {g: [x for x in range(1, max(v) + 1) if x not in v] for g, v in groups.items()}
    moved = [h[1] for h in heads if re.search(r'\*\*\s*(Moved|Answered|Closed|Withdrawn)\b', h[2])]
    # each OQ's text: from its heading to the next heading or OQ
    for j, (i, k, l) in enumerate(heads):
        end = heads[j + 1][0] - 1 if j + 1 < len(heads) else min(len(L), i + 40)
        body = ' '.join(L[i - 1:end]); body = re.split(r'\n#', body)[0]
        m = re.search(r'\bDeciders?\*?:\*?\s*(.*?)(?:\s·\s|\s- \*?[A-Z]|\. \*?What decides|$)', body)
        routed[k] = (m.group(1)[:80] if m else None, k in moved)
    pms = [int(x) for x in re.findall(r'^\| *PM-%s-(\d+) *\|' % n, t, re.M)]
    pc = collections.Counter(pms); pdup = [k for k, v in pc.items() if v > 1]
    pgaps = [x for x in range(1, max(pms) + 1) if x not in pc] if pms else []
    tot_num += len(set(ids)); tot_open += len(set(ids)) - len(moved); tot_pm += len(set(pms))
    print(f'{n}: OQ numbered {len(set(ids))} (groups ' + ', '.join(f'{g or "-"}1..{max(v)}' for g, v in groups.items()) +
          f'), duplicates {dup}, gaps {{{", ".join(f"{g}: {v}" for g, v in gaps.items() if v)}}}, moved {moved}, open {len(set(ids)) - len(moved)}'
          f' | PM rows {len(pms)}, distinct {len(set(pms))}, max {max(pms) if pms else 0}, duplicates {pdup}, gaps {pgaps}')
print(f'TOTAL OQ numbered {tot_num}, open {tot_open}; PM {tot_pm}')
sp = show('docs/SPIKE_PLAN.md')
print('SPIKE_PLAN OQ ids defined:', re.findall(r'^- \*\*(OQ-SPIKE-\d+)\.', sp, re.M))
print('SPIKE_PLAN spike table numbered rows:', re.findall(r'^\| (\d+) \|', sp, re.M), '; not-a-spike rows:', len(re.findall(r'^\| – \|', sp, re.M)))
# README drafts table and totals
for l in R.split('\n'):
    if re.match(r'^\| \[000\d\]', l): print('README drafts row', l.split('|')[1].strip()[:6], '->', l.split('|')[-2].strip()[:120])
    if l.startswith('Total:'): print('README>', l)
# design questions
dq = R.split('## Design questions')[1].split('\n## ')[0]
rows = [l.split('|')[1].strip() for l in dq.split('\n') if re.match(r'^\| R[1-6]-', l)]
print(f'README design questions: {len(rows)} rows, distinct {len(set(rows))}')
mp = open(REPO + '/.checkpoints/v2-decision-drafts/engine-architect-decision-map-ids.tsv', encoding='utf-8').read().split('\n')[1:]
design = [x.split('\t')[0] for x in mp if x and x.split('\t')[1].startswith('DESIGN')]
print(f'  decision map ids owned by DESIGN: {len(design)}; in README not DESIGN: {sorted(set(rows) - set(design))}; DESIGN not in README: {sorted(set(design) - set(rows))}')
# decisions table
dec = R.split('## Decisions the drafts ask for')[1].split('\n## ')[0]
drows = [l for l in dec.split('\n') if l.startswith('| ') and not l.startswith('| Code or OQ') and not l.startswith('|---')]
first = [l.split('|')[1].strip() for l in drows]
oq_first = [o for f in first for o in re.findall(r'OQ-(?:\d{4}-[A-Z]*\d+|SPIKE-\d+)', f)]
oc = collections.Counter(oq_first)
print(f'README decisions table: {len(drows)} rows; OQ ids in first cells {len(oq_first)}, distinct {len(oc)}, twice {[k for k, v in oc.items() if v > 1]}')
codes_first = [f for f in first if not re.match(r'^OQ-', f)]
print(f'  rows whose first cell is a code (not a bare OQ id): {len(codes_first)}')
mustafa = {k: v for k, v in routed.items() if v[0] and re.search(r'Mustafa|Advisor', v[0]) and not v[1]}
missing = sorted(k for k in mustafa if k not in oc and k not in ' '.join(first))
anywhere = sorted(k for k in missing if k in dec)
print(f'  drafts\' open OQs with a Decider naming Mustafa or the Advisor: {len(mustafa)}; not in a first cell: {missing}; of those cited elsewhere in the table: {anywhere}')
nodecider = sorted(k for k, v in routed.items() if v[0] is None)
print(f'  OQs where the script found no "*Decider:*" field: {len(nodecider)} {nodecider[:60]}')
notrouted = sorted(k for k in oc if k in routed and routed[k][0] and not re.search(r'Mustafa|Advisor', routed[k][0]))
print(f'  README rows whose OQ names neither Mustafa nor the Advisor as Decider: {[(k, routed[k][0]) for k in notrouted]}')
eo = show('docs/ENGINE_OVERVIEW.md')
print('ENGINE_OVERVIEW lines:', eo.count('\n'))
