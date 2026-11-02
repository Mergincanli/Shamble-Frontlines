import re, sys
SNAP = sys.argv[1]   # snapshot dir (9a03999)
OUT = sys.argv[2]
P = {'docs/ENGINE_OVERVIEW.md': 'ENGINE_OVERVIEW.md', 'docs/SPIKE_PLAN.md': 'SPIKE_PLAN.md', 'docs/adr/README.md': 'README.md'}
txt = {k: open(f'{SNAP}/{v}', encoding='utf8').read() for k, v in P.items()}
lines = {k: t.split('\n') for k, t in txt.items()}
blocks = []
def fix(fid, path, a, b, after=None, subs=None):
    before = '\n'.join(lines[path][a-1:b])
    n = txt[path].count(before)
    assert n == 1, (fid, path, a, b, n)
    if subs is not None:
        after = before
        for old, new in subs:
            assert after.count(old) == 1, (fid, old, after.count(old))
            after = after.replace(old, new)
    blocks.append(f'### FIX {fid}\nFILE: {path}\nBEFORE:\n{before}\nAFTER:\n{after}\nEND\n')
def note(fid, body):
    blocks.append(f'### NOTE {fid}\n{body}\nEND\n')
exec(open(sys.argv[3], encoding='utf8').read())
open(OUT, 'w', encoding='utf8').write(open(sys.argv[4], encoding='utf8').read() + '\n' + '\n'.join(blocks))
print(len(blocks), 'blocks')
