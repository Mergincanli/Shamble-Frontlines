#!/usr/bin/env python3
"""Build FIX D3C-1 (netcode checklist step 7) and D3C-1b (steps 10 and 11) for docs/adr/0001-multiplayer-netcode.md:
BEFORE = the step's whole lines copied from disk; AFTER = the same words, greedy-wrapped at 120 characters with the
step's own indent. Usage: mk_step7.py <repo> <out>"""
import sys, pathlib, re
repo, out = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
L = (repo / 'docs/adr/0001-multiplayer-netcode.md').read_text(encoding='utf8').split('\n')
h = [i for i, l in enumerate(L) if l.startswith('#### The netcode checklist')]
assert len(h) == 1
def step(n):
    s = next(i for i in range(h[0], len(L)) if L[i].startswith(f'{n}. '))
    e = s + 1
    while e < len(L) and L[e].startswith(' ' * (len(str(n)) + 2)) and not re.match(r'\d+\. ', L[e]): e += 1
    return s, e
def wrap(lines, n):
    ind = ' ' * (len(str(n)) + 2)
    words = ' '.join(x.strip() for x in lines).split(' ')
    outl, cur = [], ''
    for w in words:
        pre = ind if outl else ''
        cand = (cur + ' ' + w) if cur else (pre + w)
        if cur and len(cand) > 120: outl.append(cur); cur = ind + w
        else: cur = cand
    outl.append(cur)
    assert ' '.join(' '.join(x.strip() for x in outl).split()) == ' '.join(' '.join(x.strip() for x in lines).split())
    return outl
blocks = []
for fid, steps in (('D3C-1', [7]), ('D3C-1b', [10, 11])):
    s0 = step(steps[0])[0]; e1 = step(steps[-1])[1]
    before = L[s0:e1]
    after = []
    for n in steps:
        s, e = step(n); after += wrap(L[s:e], n)
    bt, at = '\n'.join(before), '\n'.join(after)
    assert '\n'.join(L).count(bt) == 1
    print(fid, f'lines {s0+1}-{e1}', len(before), '->', len(after), 'max before', max(map(len, before)), 'max after', max(map(len, after)))
    blocks.append(f"### FIX {fid}\nFILE: docs/adr/0001-multiplayer-netcode.md\nBEFORE:\n{bt}\nAFTER:\n{at}\nEND\n")
out.write_text('\n'.join(blocks), encoding='utf8')
