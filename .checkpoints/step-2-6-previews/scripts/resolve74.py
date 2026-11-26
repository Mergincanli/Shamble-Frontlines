# Resolves the merge of main 3b949c1 (#74, the CI Builder's ci-preview-handover) into step 2.6's branch (5ab420d7), his
# MERGE-74 (a): DECISIONS, REPO_MAP, ROADMAP and STATUS. Both sides' changes kept; lines are moved, never retyped. Every
# block is one-sided by origin: main re-worded two REPO_MAP rows lane 2 kept as they were (main's rows, then lane 2's
# added rows); ROADMAP: lane 2 had moved change-set 1 (PR #58) to "Build history" in 90c939f4, and main changed only the
# lines next to it (it moved Part 15 the same way as lane 2, which git applied once), so the move stands; STATUS: each
# side added one follow-ups bullet at the same place (main's, then lane 2's). DECISIONS by entry, as in resolve70/73.
# Outside the blocks, one move: ROADMAP's "Recent work" would hold 13 entries (main holds 13 itself), so its oldest, "The
# kit hold and two CI edits" (merged), moves byte for byte to "Build history" after change-set 1 (OLD-LINES (a)).
# STATUS stays at main's own count (125: main is above 120 itself; lane 2's side adds no net line), named in the merge
# commit's message.
# Usage: python3 -I resolve74.py <dir with base/ ours/ theirs/ copies> [--write], inside the worktree, with the merge
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
base_of = {n: read(f'{D}/base/{n}') for n in ('REPO_MAP.md', 'ROADMAP.md', 'STATUS.md')}
out = {}

# DECISIONS: by entry, from the side copies.
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

# REPO_MAP: two blocks; in each, lane 2 kept a row as the base had it and added rows after it; main re-worded that row.
def rows(name):
    def fn(ours, theirs):
        assert len(theirs) == 1 and theirs[0].startswith(f'| `{name}` |'), theirs
        assert ours[0].startswith(f'| `{name}` |') and ours[0] in base_of['REPO_MAP.md'], ('lane 2 changed', name)
        assert ours[0] != theirs[0]
        return theirs + ours[1:]
    return fn

p = 'docs/REPO_MAP.md'
out[p] = splice(read(p), [rows('packages/e2e/tests/replay/determinism.spec.ts'), rows('.github/workflows/toolchain.yml')])

# ROADMAP: block 1: lane 2's insert of change-set 1 into Build history (kept); block 2: lane 2's removal of it from the
# end of Recent work (kept: main has it there only because it kept the base's lines).
CS1 = '**Change-set 1 of the build plan, Types [2026-11-18, MERGED]** — cs1-types (foundation step'

def cs1_insert(ours, theirs):
    assert ours[0] == CS1 and ours[-1] == '' and theirs == [], (ours[:1], theirs[:1])
    return ours

def cs1_removal(ours, theirs):
    assert ours == [] and theirs[0] == '' and theirs[1] == CS1, (ours[:1], theirs[:2])
    bl = base_of['ROADMAP.md']
    k = bl.index(CS1)
    assert theirs[1:] == bl[k:k + len(theirs) - 1], 'main changed the change-set 1 entry'
    return ours

p = 'docs/ROADMAP.md'
L = splice(read(p), [cs1_insert, cs1_removal])
assert L.count(CS1) == 1
# Main itself holds 13 "Recent work" entries; with lane 2's step 2.6 entry and without change-set 1 it stays 13 (the
# file's rule: 8 to 12). Its oldest, "The kit hold and two CI edits" (merged), moves byte for byte to "Build history"
# after change-set 1's entry (his OLD-LINES (a): lines about merged pull requests), as in 6c812700 and 90c939f4.
rw = L.index('## Recent work')
starts = [i for i in range(rw, len(L)) if L[i].startswith('**')]
print('  ROADMAP: Recent work entries before the move:', len(starts))
assert len(starts) == 13, len(starts)
s = starts[-1]
assert L[s].startswith('**The kit hold and two CI edits [2026-11-18, MERGED]**'), L[s][:80]
e = s
while e < len(L) and L[e] != '':
    e += 1
moved = L[s:e]
assert all(l == '' for l in L[e:]), 'something follows the kit hold entry'
assert L[s - 1] == '', 'no blank line before the kit hold entry'
L = L[:s - 1] + L[e:]
j = L.index(CS1)
assert j < L.index('## Current state (verified against the build, 2026-09-25)'), 'change-set 1 is not in Build history'
while L[j] != '':
    j += 1
L = L[:j + 1] + moved + [''] + L[j + 1:]
print('  ROADMAP: moved', len(moved), 'lines (the kit hold and two CI edits) to Build history after change-set 1')
rw = L.index('## Recent work')
n = len([i for i in range(rw, len(L)) if L[i].startswith('**')])
print('  ROADMAP: Recent work entries:', n)
assert 8 <= n <= 12, n
out[p] = L

# STATUS: one block: each side added one follow-ups bullet after "Merged pull requests' bullets".
def follow_ups(ours, theirs):
    assert len(ours) == 1 and ours[0].startswith('- **From step 2.6** (#72, lane 2;'), ours
    assert len(theirs) == 1 and theirs[0].startswith('- **From #60, #61, #64, change-set 3a (#65), guard-env-fix (#69) and ci-after-3a (#71)**'), theirs
    return theirs + ours

p = 'docs/STATUS.md'
L = splice(read(p), [follow_ups])
n = len(L) - (1 if L[-1] == '' else 0)
print('  STATUS lines:', n)
out[p] = L

for p, lines in out.items():
    assert not any(l.startswith(('<<<<<<< ', '>>>>>>> ', '||||||| ')) or l == '=======' for l in lines), p
    if '--write' in sys.argv:
        open(p, 'w', encoding='utf-8', newline='\n').write('\n'.join(lines))
print('written' if '--write' in sys.argv else 'dry run')
