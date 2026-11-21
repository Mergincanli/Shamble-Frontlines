# Verify R-PR placements: each reading's words appear verbatim (whitespace-collapsed), added lines <= 120 chars,
# table rows keep their pipe counts, and the number of table lines is unchanged.
import json, re, subprocess, sys
WT, WJ = sys.argv[1], sys.argv[2]
W = json.load(open(WJ, encoding='utf-8'))
files = ['docs/adr/0001-multiplayer-netcode.md', 'docs/adr/0004-simulation-core.md',
         'docs/adr/0006-infrastructure-operations.md', 'docs/SPIKE_PLAN.md', 'docs/adr/README.md']
txt = {f: open(f'{WT}/{f}', encoding='utf-8').read() for f in files}
old = {f: subprocess.run(['git', '-C', WT, 'show', f'af8393e:{f}'], capture_output=True, check=True).stdout.decode()
       for f in files}
col = {f: re.sub(r'\s+', ' ', t) for f, t in txt.items()}
for k, v in W.items():
    v = v.replace('(Mustafa, <date>)', '(Mustafa, 2026-11-06)')
    sents = [s for s in re.split(r'(?<=[.])\s+(?=[A-Z0-9\'n])', v)]
    where = {}
    for s in sents:
        core = s[:-1] if s.endswith('.') else s
        hits = [f.split('/')[-1] + ':' + str(col[f].count(core)) for f in files if core in col[f]]
        if not hits:
            # allow lowercase first letter (semicolon-list fit)
            core2 = core[0].lower() + core[1:]
            hits = ['(lc)' + f.split('/')[-1] + ':' + str(col[f].count(core2)) for f in files if core2 in col[f]]
        where[core[:50]] = hits
    print('==', k)
    for c, h in where.items():
        print('   ', 'MISSING' if not h else ' ', c, '->', h)
bad = 0
for f in files:
    o, n = old[f].split('\n'), txt[f].split('\n')
    ot = [l for l in o if l.startswith('|')]; nt = [l for l in n if l.startswith('|')]
    assert len(ot) == len(nt), (f, 'table line count')
    for a, b in zip(ot, nt):
        if a != b:
            assert a.count('|') == b.count('|'), (f, a[:40])
    oldset = set(o)
    for i, l in enumerate(n, 1):
        if l not in oldset and not l.startswith('|') and len(l) > 120:
            print('LONG', f, i, len(l)); bad += 1
print('tables ok; long added prose lines:', bad)
print('count of 2026-11-06 tags per file:', {f.split('/')[-1]: txt[f].count('2026-11-06') for f in files})
