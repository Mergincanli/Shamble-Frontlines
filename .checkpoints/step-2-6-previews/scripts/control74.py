# Planted defects for verify74.py: copies the four resolved files into <out>/c1..c3/docs and breaks one thing in each.
# Usage: python3 -I control74.py <dir with base/ copies> <out dir> (inside the worktree). Runs no git; writes only
# under <out>.
#   c1: main's newest DECISIONS entry (14:08:56) loses its last line
#   c2: REPO_MAP: main's re-worded toolchain.yml row replaced by the base's row (main's change lost)
#   c3: ROADMAP: the kit hold entry also left at the end of Recent work (moved twice: a copy, not a move)
import os, sys
D, out = sys.argv[1], sys.argv[2]
FILES = ['docs/DECISIONS.md', 'docs/REPO_MAP.md', 'docs/ROADMAP.md', 'docs/STATUS.md']
src = {p: open(p, encoding='utf-8').read() for p in FILES}

def put(c, texts):
    os.makedirs(f'{out}/{c}/docs', exist_ok=True)
    for p in FILES:
        open(f'{out}/{c}/{p}', 'w', encoding='utf-8', newline='\n').write(texts.get(p, src[p]))

L = src['docs/DECISIONS.md'].split('\n')
s = next(i for i, l in enumerate(L) if l.startswith('- **Taken by Mustafa, 2026-11-26, 14:08:56 UTC'))
e = next(i for i in range(s + 1, len(L)) if L[i].startswith('- **'))
put('c1', {'docs/DECISIONS.md': '\n'.join(L[:e - 1] + L[e:])})

R = src['docs/REPO_MAP.md'].split('\n')
B = open(f'{D}/base/REPO_MAP.md', encoding='utf-8').read().split('\n')
k = next(i for i, l in enumerate(R) if l.startswith('| `.github/workflows/toolchain.yml` |'))
b = next(l for l in B if l.startswith('| `.github/workflows/toolchain.yml` |'))
assert R[k] != b
put('c2', {'docs/REPO_MAP.md': '\n'.join(R[:k] + [b] + R[k + 1:])})

M = src['docs/ROADMAP.md'].split('\n')
s = next(i for i, l in enumerate(M) if l.startswith('**The kit hold and two CI edits [2026-11-18, MERGED]**'))
e = s
while M[e] != '':
    e += 1
tail = M[:]
while tail and tail[-1] == '':
    tail.pop()
put('c3', {'docs/ROADMAP.md': '\n'.join(tail + [''] + M[s:e]) + ('\n' if src['docs/ROADMAP.md'].endswith('\n') else '')})
print('planted c1, c2, c3 under', out)
