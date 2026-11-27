# Checks the merge of main 2483e9a7 into step 2.6's branch (dc00c79a), on the four files both sides changed.
# Usage: python3 -I verify70.py <dir with base/ ours/ theirs/ copies> (run inside the worktree; the copies were made
# with `git show <rev>:<path>`, each git call alone). It runs no git and writes nothing.
# 1. Every line: result count == ours + theirs - base (each side's adds kept as often as added, its removals gone,
#    nothing else new), except the lines named in EXPECTED (the STATUS join).
# 2. DECISIONS by entry (git can shift shared lines out of a block): the result's entries are the base's plus each
#    side's new ones, each byte for byte; each side's entries keep that side's order; new entries sit newest first.
# 3. ROADMAP: "Recent work" holds 8 to 12 entries; Part 15's entry stands once, byte for byte, in "Build history".
# 4. STATUS: at most 120 lines. No conflict marker in any of the four.
import re, sys
from collections import Counter

D = sys.argv[1]
ROOT = sys.argv[2] if len(sys.argv) > 2 else '.'  # where the result's docs/ is (a planted copy for the controls)
FILES = ['docs/DECISIONS.md', 'docs/REPO_MAP.md', 'docs/ROADMAP.md', 'docs/STATUS.md']
name = lambda p: p.split('/')[-1]
rd = lambda p: open(p if p.startswith(D) else f'{ROOT}/{p}', encoding='utf-8').read().split('\n')
fail = []

def check(cond, msg):
    print(('ok   ' if cond else 'FAIL ') + msg)
    if not cond:
        fail.append(msg)

ours_status = rd(f'{D}/ours/STATUS.md')
k = next(i for i, l in enumerate(ours_status) if l.startswith("- Lane 2's step 2.6 (#72)"))
JOIN_A, JOIN_B = ours_status[k], ours_status[k + 1]
JOINED = JOIN_A + ' ' + JOIN_B.lstrip(' ')
EXPECTED = {'docs/STATUS.md': {JOIN_A: -1, JOIN_B: -1, JOINED: +1}}  # result minus (ours + theirs - base)

for p in FILES:
    B, O, T, R = (Counter(rd(f'{D}/{s}/{name(p)}')) for s in ('base', 'ours', 'theirs')), None, None, None
    B, O, T = (Counter(rd(f'{D}/{s}/{name(p)}')) for s in ('base', 'ours', 'theirs'))
    Rl = rd(p)
    R = Counter(Rl)
    diff, both = {}, []
    for line in set(B) | set(O) | set(T) | set(R):
        want = O[line] + T[line] - B[line]
        if want < 0:  # both sides removed the same base line (each put its own line in its place): gone once
            both.append(line)
            want = 0
        if R[line] != want:
            diff[line] = R[line] - want
    for line in both:
        print(f'     {p}: removed by both sides, absent from the result: {line[:90]!r}' + ('' if R[line] == 0 else ' (STILL PRESENT)'))
    exp = EXPECTED.get(p, {})
    check(diff == exp, f'{p}: result = ours + theirs - base, line by line' +
          (f' (except the {len(exp)} lines of the named join)' if exp else '') +
          ('' if diff == exp else f'; differences: {[(l[:70], v) for l, v in diff.items()]}'))
    check(not any(l.startswith(('<<<<<<< ', '>>>>>>> ', '||||||| ')) or l == '=======' for l in Rl), f'{p}: no conflict marker')

# 2. DECISIONS by entry
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

eB, eO, eT, eR = (entries(rd(x)) for x in (f'{D}/base/DECISIONS.md', f'{D}/ours/DECISIONS.md', f'{D}/theirs/DECISIONS.md', 'docs/DECISIONS.md'))
newO = [x for x in eO if x not in eB]
newT = [x for x in eT if x not in eB]
print(f'     DECISIONS entries: base {len(eB)}, ours {len(eO)} (+{len(newO)}), theirs {len(eT)} (+{len(newT)}), result {len(eR)}')
check(all(x in eO for x in eB) and all(x in eT for x in eB), 'DECISIONS: neither side removed or changed a base entry')
check(Counter(eR) == Counter(eB) + Counter(newO) + Counter(newT), 'DECISIONS: result entries = base + ours\' new + theirs\' new, byte for byte')
def in_order(sub, full):
    it = iter(full)
    return all(x in it for x in sub)
check(in_order(eO, eR), 'DECISIONS: ours\' entries in ours\' order')
check(in_order(eT, eR), 'DECISIONS: theirs\' entries in theirs\' order')
TS = re.compile(r'^- \*\*Taken by Mustafa, (20\d\d-\d\d-\d\d), (\d\d:\d\d:\d\d) UTC')
bad = []
new = set(newO) | set(newT)
for i, x in enumerate(eR):
    if x not in new:
        continue
    for j in (i - 1, i + 1):
        if 0 <= j < len(eR):
            a, b = (eR[min(i, j)], eR[max(i, j)])
            ta, tb = TS.match(a), TS.match(b)
            if ta and tb and ta.groups() < tb.groups():
                bad.append((a[:60], b[:60]))
check(not bad, 'DECISIONS: every new entry sits newest first against its neighbours' + (f'; {bad}' if bad else ''))

# 3. ROADMAP
L = rd('docs/ROADMAP.md')
rw = L.index('## Recent work')
n = len([i for i in range(rw, len(L)) if L[i].startswith('**')])
check(8 <= n <= 12, f'ROADMAP: Recent work holds {n} entries (8 to 12)')
T = rd(f'{D}/theirs/ROADMAP.md')
s = next(i for i, l in enumerate(T) if l.startswith('**Part 15 of the build plan, the second lane, to main'))
e = s
while e < len(T) and T[e] != '':
    e += 1
part15 = '\n'.join(T[s:e])
whole = '\n'.join(L)
bh = whole.index('## Build history')
cur = whole.index('## Current state')
check(whole.count(part15) == 1 and bh < whole.index(part15) < cur, 'ROADMAP: Part 15\'s entry once, byte for byte, inside Build history')
cs0 = whole.index('**Change-set 0 of the build plan [2026-11-17, MERGED]**')
check(cs0 < whole.index(part15), 'ROADMAP: Part 15 after change-set 0\'s entry')

# 4. STATUS
S = rd('docs/STATUS.md')
ns = len(S) - (1 if S[-1] == '' else 0)
check(ns <= 120, f'STATUS: {ns} lines (at most 120)')

print('PASS' if not fail else f'FAIL ({len(fail)})')
sys.exit(1 if fail else 0)
