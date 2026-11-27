# Checks the merge of main 4a62873 into step 2.6's branch (7aadfc17), on the four files both sides changed.
# Usage: python3 -I verify73.py <dir with base/ ours/ theirs/ copies> [<root of the result>] (inside the worktree; the
# copies made with `git show <rev>:<path>`, each git call alone). It runs no git and writes nothing.
# 1. Every line: result count == ours + theirs - base (a line both sides removed: gone), except EXPECTED: four lane 2
#    lines that main's docs pass re-worded while this branch had rewritten them too (lane 2's text stands, RECORD-FILES).
# 2. DECISIONS by entry: base + each side's new entries, byte for byte, each side's order, new entries newest first.
# 3. ROADMAP: "Recent work" 8 to 12 entries; change-set 1 and Part 15 each once, byte for byte, in "Build history",
#    change-set 0, then Part 15, then change-set 1.
# 4. STATUS: at most 120 lines. No conflict marker in any of the four.
import re, sys
from collections import Counter

D = sys.argv[1]
ROOT = sys.argv[2] if len(sys.argv) > 2 else '.'
FILES = ['docs/DECISIONS.md', 'docs/REPO_MAP.md', 'docs/ROADMAP.md', 'docs/STATUS.md']
name = lambda p: p.split('/')[-1]
rd = lambda p: open(p if p.startswith(D) else f'{ROOT}/{p}', encoding='utf-8').read().split('\n')
fail = []

def check(cond, msg):
    print(('ok   ' if cond else 'FAIL ') + msg)
    if not cond:
        fail.append(msg)

T_STATUS = rd(f'{D}/theirs/STATUS.md')
i3 = next(i for i, l in enumerate(T_STATUS) if l.startswith('3. Lane 2 (Builder V2)'))
NOT_CARRIED = [T_STATUS[i3], T_STATUS[i3 + 1],
               '- Lane 2, at his own timing (TR-END met): the test',
               '- **Preview hosting (lane 2):** PHC-HEADER (the header mark; revised by #67, merged); PHC-PROBE-MAIL.']
assert all(x in T_STATUS for x in NOT_CARRIED)
EXPECTED = {'docs/STATUS.md': {x: -1 for x in NOT_CARRIED}}

for p in FILES:
    B, O, T = (Counter(rd(f'{D}/{s}/{name(p)}')) for s in ('base', 'ours', 'theirs'))
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
    exp = EXPECTED.get(p, {})
    check(diff == exp, f'{p}: result = ours + theirs - base, line by line' +
          (f' (except the {len(exp)} named lines of main not carried)' if exp else '') +
          ('' if diff == exp else f'; differences: {[(l[:70], v) for l, v in diff.items()]}'))
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

eB, eO, eT, eR = (entries(rd(x)) for x in (f'{D}/base/DECISIONS.md', f'{D}/ours/DECISIONS.md', f'{D}/theirs/DECISIONS.md', 'docs/DECISIONS.md'))
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
def entry_of(lines, lead):
    s = next(i for i, l in enumerate(lines) if l.startswith(lead))
    e = s
    while e < len(lines) and lines[e] != '':
        e += 1
    return '\n'.join(lines[s:e])
T = rd(f'{D}/theirs/ROADMAP.md')
cs1 = entry_of(T, '**Change-set 1 of the build plan, Types [2026-11-18, MERGED]**')
p15 = entry_of(T, '**Part 15 of the build plan, the second lane, to main')
whole = '\n'.join(L)
bh, cur = whole.index('## Build history'), whole.index('## Current state')
for nm, x in (('change-set 1', cs1), ('Part 15', p15)):
    check(whole.count(x) == 1 and bh < whole.index(x) < cur, f"ROADMAP: {nm}'s entry once, byte for byte, inside Build history")
cs0 = whole.index('**Change-set 0 of the build plan [2026-11-17, MERGED]**')
check(cs0 < whole.index(p15) < whole.index(cs1), 'ROADMAP: change-set 0, then Part 15, then change-set 1')

S = rd('docs/STATUS.md')
ns = len(S) - (1 if S[-1] == '' else 0)
check(ns <= 120, f'STATUS: {ns} lines (at most 120)')
print('PASS' if not fail else f'FAIL ({len(fail)})')
sys.exit(1 if fail else 0)
