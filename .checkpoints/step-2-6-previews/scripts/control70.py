# Planted defects for verify70.py: copies the four resolved files into <out>/c1..c3/docs and breaks one thing in each.
# Usage: python3 -I control70.py <out dir> (inside the worktree). Runs no git; writes only under <out>.
#   c1: main's STOP-4A entry (20:48:11) loses its last line (the line git moved out of the conflict block)
#   c2: main's entries of 17:44:41 and 16:59:52 change places (side order and newest first broken)
#   c3: STATUS keeps only lane 2's "Last updated" line (main's line dropped)
import os, sys
out = sys.argv[1]
FILES = ['docs/DECISIONS.md', 'docs/REPO_MAP.md', 'docs/ROADMAP.md', 'docs/STATUS.md']
src = {p: open(p, encoding='utf-8').read() for p in FILES}

def put(c, texts):
    os.makedirs(f'{out}/{c}/docs', exist_ok=True)
    for p in FILES:
        open(f'{out}/{c}/{p}', 'w', encoding='utf-8', newline='\n').write(texts.get(p, src[p]))

L = src['docs/DECISIONS.md'].split('\n')
s = next(i for i, l in enumerate(L) if l.startswith('- **Taken by Mustafa, 2026-11-25, 20:48:11 UTC'))
e = next(i for i in range(s + 1, len(L)) if L[i].startswith('- **'))
assert L[e - 1].startswith('> Unchanged:'), L[e - 1][:40]
put('c1', {'docs/DECISIONS.md': '\n'.join(L[:e - 1] + L[e:])})

def span(L, ts):
    s = next(i for i, l in enumerate(L) if l.startswith(f'- **Taken by Mustafa, 2026-11-25, {ts} UTC'))
    e = next(i for i in range(s + 1, len(L)) if L[i].startswith('- **'))
    return s, e
a0, a1 = span(L, '17:44:41')
b0, b1 = span(L, '16:59:52')
assert a1 <= b0
put('c2', {'docs/DECISIONS.md': '\n'.join(L[:a0] + L[b0:b1] + L[a1:b0] + L[a0:a1] + L[b1:])})

S = src['docs/STATUS.md'].split('\n')
k = next(i for i, l in enumerate(S) if l.startswith("_Last updated: 2026-11-25 (change-set 4a's docs pass"))
put('c3', {'docs/STATUS.md': '\n'.join(S[:k] + S[k + 1:])})
print('planted c1, c2, c3 under', out)
