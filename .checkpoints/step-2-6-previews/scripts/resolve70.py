# Resolves the merge of main 2483e9a7 (#70, lane 1's change-set 4a) into step 2.6's branch (dc00c79a): DECISIONS,
# REPO_MAP, ROADMAP and STATUS. Both sides' text kept; lane 1's lines byte for byte; lines are moved, never retyped.
# Two additions beyond the conflict blocks, each named in the merge commit's message:
# - STATUS: both "Last updated" lines are kept, which makes 121 lines; lane 2's own Waiting bullet (two lines) is
#   joined into one line, no word changed, to stay at 120 (STATUS-DIET).
# - ROADMAP: "Recent work" would hold 13 entries (the file's rule: 8 to 12), so its oldest, Part 15 (PR #57, merged),
#   moves byte for byte to "Build history", after change-set 0's entry (his OLD-LINES (a): lines about merged PRs).
# Usage: python3 -I resolve70.py <dir with base/ ours/ theirs/ copies> [--write], inside the worktree, with the merge
# in progress (the copies made with `git show <rev>:<path>`, each git call alone). It runs no git and writes only docs/.
import re, sys

def read(p):
    return open(p, encoding='utf-8').read().split('\n')

def blocks(lines):
    """(start, mid, end) indices of each conflict block (two-way markers only)."""
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
    """Replaces each block (in order) by its resolution list."""
    bs = blocks(lines)
    assert len(bs) == len(resolutions), (len(bs), len(resolutions))
    res, last = [], 0
    for (s, m, e), fn in zip(bs, resolutions):
        res += lines[last:s] + fn(lines[s + 1:m], lines[m + 1:e])
        last = e + 1
    return res + lines[last:]

out = {}

# DECISIONS: by entry, not by conflict block (git moved a last line both sides' entries share out of a block). Built
# from the two sides' own copies (argv[1]/ours, argv[1]/theirs, argv[1]/base, made with git show): the base's entries
# are anchors both sides keep in one order; between two anchors, each side's new entries go in newest first by the
# timestamp in their header. Both sides changed only the "Decisions, newest first" section (checked below).
TS = re.compile(r'^- \*\*Taken by Mustafa, (20\d\d-\d\d-\d\d), (\d\d:\d\d:\d\d) UTC')
HEAD_D, HEAD_E = '## Decisions, newest first', '## Also recorded elsewhere (an index made by the split script; pointers, not his words)'
D = sys.argv[1]

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

bh, bE, bt = dec_split(read(f'{D}/base/DECISIONS.md'))
oh, oE, ot = dec_split(read(f'{D}/ours/DECISIONS.md'))
th, tE, tt = dec_split(read(f'{D}/theirs/DECISIONS.md'))
assert bh == oh == th and bt == ot == tt, 'a side changed DECISIONS outside "Decisions, newest first"'
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
out['docs/DECISIONS.md'] = oh + '\n'.join(res).split('\n') + ot

# REPO_MAP: one block; each side added one row after the rules row. Main's e2e row first (its group), then hosting/.
def repo_rows(ours, theirs):
    assert len(ours) == 1 and ours[0].startswith('| `hosting/` |'), ours
    assert len(theirs) == 1 and theirs[0].startswith('| `packages/e2e/tests/replay/determinism.spec.ts` |'), theirs
    return theirs + ours

p = 'docs/REPO_MAP.md'
out[p] = splice(read(p), [repo_rows])

# ROADMAP: one block at the top of "Recent work"; each side added one entry. Main's entry first (as main has it, the
# precedent of 35adfbe), a blank line (git took the shared one out of the block), then lane 2's.
def recent(ours, theirs):
    assert ours[0].startswith('**Step 2.6, the preview links per pull request ['), ours[0][:60]
    assert theirs[0].startswith('**Change-set 4a of the build plan, The recording ['), theirs[0][:60]
    assert ours[-1] and theirs[-1], 'a side ends with a blank line'
    return theirs + [''] + ours

p = 'docs/ROADMAP.md'
L = splice(read(p), [recent])
rw = L.index('## Recent work')
starts = [i for i in range(rw, len(L)) if L[i].startswith('**')]
print('  ROADMAP: Recent work entries before the move:', len(starts))
assert len(starts) == 13, len(starts)
s = starts[-1]
assert L[s].startswith('**Part 15 of the build plan, the second lane, to main [2026-11-17 to 11-18, MERGED]**'), L[s][:80]
e = s
while e < len(L) and L[e] != '':
    e += 1
moved = L[s:e]
assert all(l == '' for l in L[e:]), 'something follows Part 15'
assert L[s - 1] == '', 'no blank line before Part 15'
tail = L[e:]
L = L[:s - 1] + tail  # drop the entry and the blank line before it
cs0 = [i for i, l in enumerate(L) if l.startswith('**Change-set 0 of the build plan [2026-11-17, MERGED]**')]
assert len(cs0) == 1, cs0
j = cs0[0]
while L[j] != '':
    j += 1
assert L[j + 1].startswith('- **Kit: KEPT-LOGS (2026-11-15):**'), L[j + 1][:60]
L = L[:j + 1] + moved + [''] + L[j + 1:]  # after change-set 0's entry and its blank line
print('  ROADMAP: moved', len(moved), 'lines (Part 15) to Build history after change-set 0')
out[p] = L

# STATUS: four blocks.
def last_updated(ours, theirs):
    assert len(ours) == 1 and ours[0].startswith('_Last updated: 2026-11-26 (step 2.6'), ours
    assert len(theirs) == 1 and theirs[0].startswith("_Last updated: 2026-11-25 (change-set 4a's docs pass"), theirs
    return ours + theirs  # newest first

def in_flight(ours, theirs):
    assert len(ours) == 2 and ours[0].startswith('| none yet (title and text in the notes') and ours[1].startswith('| #72 |'), ours
    assert len(theirs) == 1 and theirs[0].startswith('| #70 |'), theirs
    return theirs + [ours[1]]  # main replaced ci-after-3a's row with #70's; lane 2 added #72's

def next_items(ours, theirs):
    k_o = next(i for i, l in enumerate(ours) if l.startswith('3. Lane 2'))
    k_t = next(i for i, l in enumerate(theirs) if l.startswith('3. Lane 2'))
    assert ours[0].startswith('2. v2:') and theirs[0].startswith('2. v2:')
    assert len(ours) == k_o + 1, 'lane 2 item 3 is one line'
    return theirs[:k_t] + ours[k_o:]  # main's item 2, lane 2's item 3

def risks(ours, theirs):
    assert ours[-1].startswith('- **Preview hosting (lane 2):**') and theirs[-1].startswith('- **Preview hosting (lane 2):**')
    return theirs[:-1] + [ours[-1]]  # main's v2 risk lines, lane 2's preview line

p = 'docs/STATUS.md'
L = splice(read(p), [last_updated, in_flight, next_items, risks])
k = [i for i, l in enumerate(L) if l.startswith("- Lane 2's step 2.6 (#72): his answers of 2026-11-26")]
assert len(k) == 1, k
k = k[0]
assert L[k + 1].startswith("  repository's hourly checker off before 2 December"), L[k + 1][:60]
L = L[:k] + [L[k] + ' ' + L[k + 1].lstrip(' ')] + L[k + 2:]
n = len(L) - (1 if L[-1] == '' else 0)
print('  STATUS lines:', n)
assert n <= 120, n
out[p] = L

for p, lines in out.items():
    assert not any(l.startswith(('<<<<<<< ', '>>>>>>> ', '||||||| ')) or l == '=======' for l in lines), p
    if '--write' in sys.argv:
        open(p, 'w', encoding='utf-8', newline='\n').write('\n'.join(lines))
print('written' if '--write' in sys.argv else 'dry run')
