# Checks the merge of main b9aad01 (#77, #75) into step 2.6's branch (157f888c), on the four files both sides changed.
# Usage: python3 -I verify75.py <dir with base/ ours/ theirs/ copies> [<root of the result>] (inside the worktree; the
# copies made with `git show <rev>:<path>`, each git call alone). It runs no git and writes nothing. (Made from
# verify74.py with three lines changed: this header, the ROADMAP entries checked, and their order's message.)
# 1. Every line: result count == ours + theirs - base (a line both sides removed: gone once); no exception expected.
# 2. DECISIONS by entry: base + each side's new entries, byte for byte, each side's order, new ones newest first.
# 3. ROADMAP: "Recent work" holds 8 to 12 entries; change-set 2 (main's move) and CI-WASM (this merge's move) stand once
#    each, byte for byte (as the base had them), in "Build history", in that order.
# 4. STATUS: the result has main's line count plus lane 2's net change (0). No conflict marker in any of the four.
import re, sys
from collections import Counter

D = sys.argv[1]
ROOT = sys.argv[2] if len(sys.argv) > 2 else '.'
FILES = ['docs/DECISIONS.md', 'docs/REPO_MAP.md', 'docs/ROADMAP.md', 'docs/STATUS.md']
name = lambda p: p.split('/')[-1]
rd = lambda p: open(p if p.startswith(D) else f'{ROOT}/{p}', encoding='utf-8').read().split('\n')
side = lambda s, p: rd(f'{D}/{s}/{name(p)}')
fail = []

def check(cond, msg):
    print(('ok   ' if cond else 'FAIL ') + msg)
    if not cond:
        fail.append(msg)

for p in FILES:
    B, O, T = (Counter(side(s, p)) for s in ('base', 'ours', 'theirs'))
    Rl = rd(p)
    R = Counter(Rl)
    diff, both = {}, []
    for line in set(B) | set(O) | set(T) | set(R):
        want = O[line] + T[line] - B[line]
        if want < 0:
            both.append(line)
            want = 0
        if R[line] != want:
            diff[line] = R[line] - want
    for line in both:
        print(f'     {p}: removed by both sides, absent from the result: {line[:90]!r}' + ('' if R[line] == 0 else ' (STILL PRESENT)'))
    check(not diff, f'{p}: result = ours + theirs - base, line by line' +
          ('' if not diff else f'; differences: {[(l[:70], v) for l, v in diff.items()]}'))
    check(not any(l.startswith(('<<<<<<< ', '>>>>>>> ', '||||||| ')) or l == '=======' for l in Rl), f'{p}: no conflict marker')

def entries(lines):
    a = lines.index('## Decisions, newest first')
    b = lines.index('## Also recorded elsewhere (an index made by the split script; pointers, not his words)')
    ent = []
    for l in lines[a + 1:b]:
        if l.startswith('- **'):
            ent.append([l])
        elif ent:
            ent[-1].append(l)
        else:
            assert l == '', l[:60]
    return ['\n'.join(x) for x in ent]

p = 'docs/DECISIONS.md'
eB, eO, eT, eR = entries(side('base', p)), entries(side('ours', p)), entries(side('theirs', p)), entries(rd(p))
newO = [x for x in eO if x not in eB]
newT = [x for x in eT if x not in eB]
print(f'     DECISIONS entries: base {len(eB)}, ours {len(eO)} (+{len(newO)}), theirs {len(eT)} (+{len(newT)}), result {len(eR)}')
check(all(x in eO for x in eB) and all(x in eT for x in eB), 'DECISIONS: neither side removed or changed a base entry')
check(Counter(eR) == Counter(eB) + Counter(newO) + Counter(newT), "DECISIONS: result entries = base + ours' new + theirs' new, byte for byte")
def in_order(sub, full):
    it = iter(full)
    return all(x in it for x in sub)
check(in_order(eO, eR), "DECISIONS: ours' entries in ours' order")
check(in_order(eT, eR), "DECISIONS: theirs' entries in theirs' order")
TS = re.compile(r'^- \*\*Taken by Mustafa, (20\d\d-\d\d-\d\d), (\d\d:\d\d:\d\d) UTC')
bad, new = [], set(newO) | set(newT)
for i, x in enumerate(eR):
    if x not in new:
        continue
    for j in (i - 1, i + 1):
        if 0 <= j < len(eR):
            a, b = eR[min(i, j)], eR[max(i, j)]
            ta, tb = TS.match(a), TS.match(b)
            if ta and tb and ta.groups() < tb.groups():
                bad.append((a[:60], b[:60]))
check(not bad, 'DECISIONS: every new entry sits newest first against its neighbours' + (f'; {bad}' if bad else ''))

L = rd('docs/ROADMAP.md')
rw = L.index('## Recent work')
n = len([i for i in range(rw, len(L)) if L[i].startswith('**')])
check(8 <= n <= 12, f'ROADMAP: Recent work holds {n} entries (8 to 12)')
B = side('base', 'docs/ROADMAP.md')
def entry(lines, head):
    s = next(i for i, l in enumerate(lines) if l.startswith(head))
    e = s
    while e < len(lines) and lines[e] != '':
        e += 1
    return '\n'.join(lines[s:e])
whole = '\n'.join(L)
bh, cur = whole.index('## Build history'), whole.index('## Current state')
pos = []
for head in ('**Change-set 2 of the build plan, First page', '**CI builds the WebAssembly and runs the tests that need it'):
    t = entry(B, head)
    k = whole.find(t)
    check(whole.count(t) == 1 and bh < k < cur, f'ROADMAP: {head[2:40]}... once, byte for byte (as the base had it), inside Build history')
    pos.append(k)
check(pos == sorted(pos), 'ROADMAP: change-set 2, then CI-WASM')

S, SB, SO, ST = rd('docs/STATUS.md'), side('base', 'docs/STATUS.md'), side('ours', 'docs/STATUS.md'), side('theirs', 'docs/STATUS.md')
cnt = lambda x: len(x) - (1 if x[-1] == '' else 0)
check(cnt(S) == cnt(ST) + cnt(SO) - cnt(SB), f'STATUS: {cnt(S)} lines = main {cnt(ST)} + lane 2\'s net {cnt(SO) - cnt(SB)}')

print('PASS' if not fail else f'FAIL ({len(fail)})')
sys.exit(1 if fail else 0)
