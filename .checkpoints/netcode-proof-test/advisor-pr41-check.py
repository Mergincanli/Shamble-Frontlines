# advisor's own check for PR #41: each sentence of the verdict's R-PR words present at head, absent at base.
import re, subprocess, sys
WT = '/home/user/Shamble-Frontlines/.gate/npt-readings'
V = '/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-verdict.md'
BASE, HEAD = 'af8393e9d40f3bd382e77f86e6df2612785c706c', '2ccc184b55f62430ece8f47afcfe8c058816f863'
FILES = {'0001': 'docs/adr/0001-multiplayer-netcode.md', '0004': 'docs/adr/0004-simulation-core.md',
         '0006': 'docs/adr/0006-infrastructure-operations.md', 'SP': 'docs/SPIKE_PLAN.md', 'RD': 'docs/adr/README.md'}
def show(rev, p):
    return subprocess.run(['git', 'show', f'{rev}:{p}'], cwd=WT, capture_output=True, text=True, check=True).stdout
norm = lambda s: re.sub(r'\s+', ' ', s)
head = {k: norm(show(HEAD, p)) for k, p in FILES.items()}
base = {k: norm(show(BASE, p)) for k, p in FILES.items()}
lines = open(V, encoding='utf-8').read().split('\n')
start = next(i for i, l in enumerate(lines) if l.startswith('## Exact words'))
end = next(i for i, l in enumerate(lines) if l.startswith('## Decisions the advisor'))
block = '\n'.join(lines[start + 1:end])
bullets = [norm(b) for b in re.split(r'\n- ', '\n' + block) if b.strip()]
bad = 0
for b in bullets:
    label = b.split(':**')[0].strip('* ')
    text = b[b.index('"') + 1: b.rindex('"')]
    text = text.replace('<date>', '2026-11-06')
    sents = re.split(r'(?<=[.])\s+(?=[A-Z0-9`])', text)
    print('==', label)
    for s in sents:
        s2 = s.rstrip('.')
        where = [k for k in head if s2 in head[k]]
        wasb = [k for k in base if s2 in base[k]]
        ok = bool(where) and not wasb
        if not ok: bad += 1
        print('  ', 'OK ' if ok else 'XX ', 'head:', ','.join(where) or '-', '| base:', ','.join(wasb) or '-', '|', s2[:70])
print('sentences not placed (or already at base):', bad)
tag = "(the advisor's reading, 2026-11-06)"
for k in head:
    print(k, 'tags head', head[k].count(tag), 'base', base[k].count(tag))
