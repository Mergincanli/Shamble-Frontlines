# Planted defects for verify73.py: copies the four resolved files into <out>/c1..c4/docs and breaks one thing in each.
# Usage: python3 -I control73.py <out dir> (inside the worktree). Runs no git; writes only under <out>.
#   c1: main's entry of 14:09:28 (R4-END) loses its last line
#   c2: main's entries of 12:24:26 and 23:01:07 (the 25th) change places
#   c3: STATUS loses main's guard-wrappers "Last updated" line
#   c4: STATUS carries main's re-worded preview line in place of lane 2's (a named line swapped the other way)
import os, sys
out = sys.argv[1]
FILES = ['docs/DECISIONS.md', 'docs/REPO_MAP.md', 'docs/ROADMAP.md', 'docs/STATUS.md']
src = {p: open(p, encoding='utf-8').read() for p in FILES}

def put(c, texts):
    os.makedirs(f'{out}/{c}/docs', exist_ok=True)
    for p in FILES:
        open(f'{out}/{c}/{p}', 'w', encoding='utf-8', newline='\n').write(texts.get(p, src[p]))

L = src['docs/DECISIONS.md'].split('\n')
def span(day, ts):
    s = next(i for i, l in enumerate(L) if l.startswith(f'- **Taken by Mustafa, {day}, {ts} UTC'))
    e = next(i for i in range(s + 1, len(L)) if L[i].startswith('- **'))
    return s, e
s, e = span('2026-11-26', '14:09:28')
put('c1', {'docs/DECISIONS.md': '\n'.join(L[:e - 1] + L[e:])})
a0, a1 = span('2026-11-26', '12:24:26')
b0, b1 = span('2026-11-25', '23:01:07')
assert a1 <= b0
put('c2', {'docs/DECISIONS.md': '\n'.join(L[:a0] + L[b0:b1] + L[a1:b0] + L[a0:a1] + L[b1:])})
S = src['docs/STATUS.md'].split('\n')
k = next(i for i, l in enumerate(S) if l.startswith("_Last updated: 2026-11-26 (guard-wrappers' docs pass"))
put('c3', {'docs/STATUS.md': '\n'.join(S[:k] + S[k + 1:])})
k = next(i for i, l in enumerate(S) if l.startswith('- **Preview hosting (lane 2):**'))
S2 = S[:k] + ['- **Preview hosting (lane 2):** PHC-HEADER (the header mark; revised by #67, merged); PHC-PROBE-MAIL.'] + S[k + 1:]
put('c4', {'docs/STATUS.md': '\n'.join(S2)})
print('planted c1 to c4 under', out)
