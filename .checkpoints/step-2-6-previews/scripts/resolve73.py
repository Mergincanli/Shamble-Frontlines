# Resolves the merge of main 4a62873 (#73, the Guard fix chat's guard-wrappers) into step 2.6's branch (7aadfc17):
# DECISIONS, REPO_MAP, ROADMAP and STATUS. Both sides' changes kept; lines are moved, never retyped. Where both sides
# changed the same line, it is always a lane 2 line (main's docs pass re-worded it to say #66 and #67 merged, which
# lane 2's current text already says): lane 2's text stands, by his RECORD-FILES (a) ("Each pull request writes only
# its own lines there ... It does not reword, re-wrap or move another lane's lines."). Those four lines of main's are
# named in the merge commit's message and in verify73.py's EXPECTED. One addition beyond the conflict blocks: ROADMAP's
# "Recent work" would hold 13 entries, so its oldest, change-set 1 (PR #58, merged), moves byte for byte to "Build
# history" after Part 15's entry (OLD-LINES (a)).
# Usage: python3 -I resolve73.py <dir with base/ ours/ theirs/ copies> [--write], inside the worktree, with the merge
# in progress (the copies made with `git show <rev>:<path>`, each git call alone). It runs no git and writes only docs/.
import re, sys

def read(p):
    return open(p, encoding='utf-8').read().split('\n')

def blocks(lines):
    out, i = [], 0
    while i < len(lines):
        if lines[i].startswith('<<<<<<< '):
            mid = next(j for j in range(i, len(lines)) if lines[j] == '=======')
            end = next(j for j in range(mid, len(lines)) if lines[j].startswith('>>>>>>> '))
            assert not any(l.startswith('||||||| ') for l in lines[i:end]), 'diff3 markers not expected'
            out.append((i, mid, end))
            i = end + 1
        else:
            i += 1
    return out

def splice(lines, resolutions):
    bs = blocks(lines)
    assert len(bs) == len(resolutions), (len(bs), len(resolutions))
    res, last = [], 0
    for (s, m, e), fn in zip(bs, resolutions):
        res += lines[last:s] + fn(lines[s + 1:m], lines[m + 1:e])
        last = e + 1
    return res + lines[last:]

D = sys.argv[1]
out = {}

# DECISIONS: by entry, from the side copies (as in resolve70.py). Main also changed "Queued" (the Guard fix chat's
# queue); this branch changed nothing outside "Decisions, newest first", so the head and tail come from the side that
# changed them.
TS = re.compile(r'^- \*\*Taken by Mustafa, (20\d\d-\d\d-\d\d), (\d\d:\d\d:\d\d) UTC')
HEAD_D, HEAD_E = '## Decisions, newest first', '## Also recorded elsewhere (an index made by the split script; pointers, not his words)'

def dec_split(lines):
    a, b = lines.index(HEAD_D), lines.index(HEAD_E)
    ent = []
    for l in lines[a + 1:b]:
        if l.startswith('- **'):
            ent.append([l])
        else:
            assert ent, ('the section starts with a line outside an entry', l[:80])
            ent[-1].append(l)
    return lines[:a + 1], ['\n'.join(x) for x in ent], lines[b:]

def one_side(b, o, t, what):
    if o == b:
        return t
    if t == b:
        return o
    raise AssertionError(f'both sides changed DECISIONS {what}')

bh, bE, bt = dec_split(read(f'{D}/base/DECISIONS.md'))
oh, oE, ot = dec_split(read(f'{D}/ours/DECISIONS.md'))
th, tE, tt = dec_split(read(f'{D}/theirs/DECISIONS.md'))
head, tail = one_side(bh, oh, th, 'above "Decisions, newest first"'), one_side(bt, ot, tt, 'below it')
print('  DECISIONS: head from', 'theirs' if head is th and th != bh else 'ours/base', '; tail from', 'theirs' if tail is tt and tt != bt else 'ours/base')
base = set(bE)
assert all(x in oE for x in bE) and all(x in tE for x in bE), 'a side removed or changed a base entry'
res, i, j, picks = [], 0, 0, []
while i < len(oE) or j < len(tE):
    o = oE[i] if i < len(oE) else None
    t = tE[j] if j < len(tE) else None
    if o is not None and o == t:
        res.append(o); i += 1; j += 1
        continue
    o_new, t_new = o is not None and o not in base, t is not None and t not in base
    if o_new and t_new:
        ko, kt = TS.match(o).groups(), TS.match(t).groups()
        assert ko != kt, 'two new entries with one timestamp'
        take = 'ours' if ko > kt else 'theirs'
    elif o_new:
        take = 'ours'
    elif t_new:
        take = 'theirs'
    else:
        raise AssertionError('the two sides keep base entries in different orders')
    x = o if take == 'ours' else t
    picks.append((' '.join(TS.match(x).groups())[5:], take))
    res.append(x)
    if take == 'ours':
        i += 1
    else:
        j += 1
print('  DECISIONS new entries, in place:', picks)
out['docs/DECISIONS.md'] = head + '\n'.join(res).split('\n') + tail

# REPO_MAP: one block of report rows; by name: main's two (2026-11-25-guard-env-fix, 2026-11-26-guard-wrappers), then
# this branch's (2026-11-26-step-2-6-previews).
def report_rows(ours, theirs):
    assert len(ours) == 1 and ours[0].startswith('| `docs/reports/2026-11-26-step-2-6-previews.md` |'), ours
    assert len(theirs) == 2 and theirs[0].startswith('| `docs/reports/2026-11-25-guard-env-fix.md` |') \
        and theirs[1].startswith('| `docs/reports/2026-11-26-guard-wrappers.md` |'), theirs
    return theirs + ours

p = 'docs/REPO_MAP.md'
out[p] = splice(read(p), [report_rows])

# ROADMAP: block 1: this branch's step 2.6 entry and a blank line (added), then main's ci-after-3a header (main changed
# it; this branch kept the base's). Block 2 (lane 2's #67 entry): main re-worded its last line, this branch kept that
# line and added a two-line pointer after it: main's line, then the pointer (each side's change once).
def recent_1(ours, theirs):
    assert ours[0].startswith('**Step 2.6, the preview links per pull request [') and ours[-2] == '', ours[-2:]
    assert ours[-1].startswith('**The small CI pull request after change-set 3a [2026-11-24 to 11-25, waiting for his go]**')
    assert len(theirs) == 1 and theirs[0].startswith('**The small CI pull request after change-set 3a [2026-11-24 to 11-25, MERGED]**')
    return ours[:-1] + theirs

def entry_67(ours, theirs):
    assert len(ours) == 3 and ours[0] == "merge commit; waits for the advisor agent's approval of that head and his go."
    assert ours[1].startswith('_Pointer (2026-11-26): merged as PR #67')
    assert len(theirs) == 1 and theirs[0].startswith('merge commit. Merged: PR #67, c1da1bd')
    return theirs + ours[1:]

p = 'docs/ROADMAP.md'
L = splice(read(p), [recent_1, entry_67])
rw = L.index('## Recent work')
starts = [i for i in range(rw, len(L)) if L[i].startswith('**')]
print('  ROADMAP: Recent work entries before the move:', len(starts))
assert len(starts) == 13, len(starts)
s = starts[-1]
assert L[s].startswith('**Change-set 1 of the build plan, Types [2026-11-18, MERGED]**'), L[s][:80]
e = s
while e < len(L) and L[e] != '':
    e += 1
moved = L[s:e]
assert all(l == '' for l in L[e:]), 'something follows change-set 1'
assert L[s - 1] == '', 'no blank line before change-set 1'
L = L[:s - 1] + L[e:]
p15 = [i for i, l in enumerate(L) if l.startswith('**Part 15 of the build plan, the second lane, to main [2026-11-17 to 11-18, MERGED]**')]
assert len(p15) == 1, p15
j = p15[0]
assert j < L.index('## Current state (verified against the build, 2026-09-25)'), 'Part 15 is not in Build history'
while L[j] != '':
    j += 1
L = L[:j + 1] + moved + [''] + L[j + 1:]
print('  ROADMAP: moved', len(moved), 'lines (change-set 1) to Build history after Part 15')
out[p] = L

# STATUS: six blocks.
def last_updated(ours, theirs):
    assert len(ours) == 2 and ours[0].startswith('_Last updated: 2026-11-26 (step 2.6') \
        and ours[1].startswith("_Last updated: 2026-11-25 (change-set 4a's docs pass"), ours
    assert len(theirs) == 1 and theirs[0].startswith("_Last updated: 2026-11-26 (guard-wrappers' docs pass"), theirs
    return theirs + [ours[0]]  # main replaced its own line; lane 2's stays; newest first (main's, 14:36 UTC, above)

def in_flight(ours, theirs):
    assert len(ours) == 2 and ours[0].startswith('| #70 |') and ours[1].startswith('| #72 |'), ours
    assert len(theirs) == 1 and theirs[0].startswith('| none yet | guard-wrappers'), theirs
    return theirs + [ours[1]]  # main replaced #70's row (merged) with guard-wrappers'; lane 2's #72 row stays

def next_items(ours, theirs):
    assert len(ours) == 1 and ours[0].startswith('3. Lane 2 (Builder V2), his order of 2026-11-25: #66 (09faefa) and #67 (c1da1bd) merged; step 2.6 (#72)'), ours
    k = next(i for i, l in enumerate(theirs) if l.startswith('4. The Guard fix chat'))
    assert theirs[0].startswith('3. Lane 2 (Builder V2)') and k == 3, (theirs[0][:40], k)
    return ours + theirs[k:]  # lane 2's item 3 (RECORD-FILES), main's new item 4

def waiting(ours, theirs):
    assert len(ours) == 1 and ours[0].startswith("- Lane 2's step 2.6 (#72): his answers of 2026-11-26"), ours
    assert len(theirs) == 2 and theirs[0] == '- Lane 2, at his own timing (TR-END met): the test', theirs
    return ours  # lane 2's own bullet (RECORD-FILES); its text already holds the TR-END part

def follow_ups(ours, theirs):
    k = next(i for i, l in enumerate(ours) if l.startswith('- **From step 2.6** (#72, lane 2;'))
    g = next(i for i, l in enumerate(theirs) if l.startswith('- **From guard-env-fix (#69):**'))
    assert theirs[0].startswith("- **Merged pull requests' bullets** (moved by script)"), theirs[0][:50]
    return theirs[:g] + [ours[k]] + theirs[g:]  # main's lines; lane 2's bullet before guard-env-fix's, as on its side

def risks(ours, theirs):
    assert len(ours) == 1 and ours[0].startswith('- **Preview hosting (lane 2):** PHC-HEADER (the header mark; revised by #67, merged); PHC-PROBE-MAIL. Step 2.6'), ours
    assert len(theirs) == 1 and theirs[0] == '- **Preview hosting (lane 2):** PHC-HEADER (the header mark; revised by #67, merged); PHC-PROBE-MAIL.', theirs
    return ours  # lane 2's own line (RECORD-FILES); main's change ("merged") is in it already

p = 'docs/STATUS.md'
L = splice(read(p), [last_updated, in_flight, next_items, waiting, follow_ups, risks])
n = len(L) - (1 if L[-1] == '' else 0)
print('  STATUS lines:', n)
assert n <= 120, n
out[p] = L

for p, lines in out.items():
    assert not any(l.startswith(('<<<<<<< ', '>>>>>>> ', '||||||| ')) or l == '=======' for l in lines), p
    if '--write' in sys.argv:
        open(p, 'w', encoding='utf-8', newline='\n').write('\n'.join(lines))
print('written' if '--write' in sys.argv else 'dry run')
