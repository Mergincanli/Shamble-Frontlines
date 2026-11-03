# Try word replacements in ADR 0004 part 7 (lines A..290), greedy re-wrap at 120 characters with a 3-space indent,
# and report whether the line count holds. Usage: fit.py <A> <candidates.py> [name]  (candidates: dict name -> [(old, new)])
import sys, re, difflib
DOC = '/home/user/Shamble-Frontlines/docs/adr/0004-simulation-core.md'
L = open(DOC, encoding='utf8').read().split('\n')
A, B = int(sys.argv[1]), 290
G = {}; exec(open(sys.argv[2], encoding="utf8").read(), G); C = G["CANDS"]
def tokens(text):
    # code spans with spaces stay whole (the original keeps `-C target-cpu=native` on one line)
    out, buf = [], None
    for w in text.split(' '):
        if buf is not None:
            buf += ' ' + w
            if w.count('`') % 2: out.append(buf); buf = None
        elif w.count('`') % 2: buf = w
        else: out.append(w)
    assert buf is None; return out
def wrap(text):
    lines, cur = [], ''
    for w in tokens(text):
        c = (cur + ' ' + w) if cur else w
        if len('   ' + c) <= 120: cur = c
        else: lines.append('   ' + cur); cur = w
    lines.append('   ' + cur); return lines
text = ' '.join(l[3:] for l in L[A-1:B])
for name, pairs in C.items():
    if len(sys.argv) > 3 and name != sys.argv[3]: continue
    t = text
    for o, n in pairs:
        assert t.count(o) == 1, (name, o); t = t.replace(o, n)
    out = wrap(t)
    risky = [l for l in out if re.match(r'   ([-+*>#|]|\d+[.)])( |$)', l)]
    print(f'{name}: {B-A+1} -> {len(out)} lines, max {max(map(len, out))}, last {len(out[-1])}, risky {len(risky)}, added {len(t)-len(text)}')
    if len(sys.argv) > 3:
        for l in out: print(f'{len(l):4d} {l}')
        print('word diff:', [d for d in difflib.ndiff(text.split(), t.split()) if d[0] in '+-'])
