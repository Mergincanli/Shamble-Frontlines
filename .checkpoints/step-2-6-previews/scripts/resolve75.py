# Resolves the merge of main b9aad01 (#77, the Guard fix chat's docs; #75, Builder V4's change-set 4b-1) into step 2.6's
# branch (157f888c), his words of 2026-11-27, 12:50 Amsterdam, item 2: DECISIONS, REPO_MAP, ROADMAP and STATUS. Both
# sides' changes kept; lines are moved, never retyped. Every block is one-sided by origin: where main replaced its own
# lines (merged rows, its Next items, its follow-ups and risks, its "Last updated"), main's text stands; lane 2's own
# lines (its "Last updated", #72's row, Next item 3, its follow-ups bullet and preview risk line, its REPO_MAP rows) stay.
# ROADMAP: main moved "Change-set 2 of the build plan, First page" (merged) to "Build history"; that move stands.
# DECISIONS by entry (main also changed "Queued"; this branch did not). Outside the blocks, one move: "Recent work" would
# hold 13 entries, so its oldest, "CI builds the WebAssembly..." (PR #64, merged), moves byte for byte to "Build
# history" after change-set 2 (OLD-LINES (a)). STATUS ends at main's 120 lines (lane 2 adds no net line).
# Usage: python3 -I resolve75.py <dir with base/ ours/ theirs/ copies> [--write], inside the worktree, with the merge
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
def in_base(name, lines):
    b = '\n'.join(base_of[name])
    return '\n'.join(lines) in b
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

# REPO_MAP: block 1: main re-worded rows lane 2 kept as the base had them; lane 2 added hosting/ after them.
# Block 2: each side added one report row at the same place: by name, main's (main-wall), then lane 2's (step-2-6).
def e2e_rows(ours, theirs):
    n = len(theirs)
    assert ours[:n] != theirs and in_base('REPO_MAP.md', ours[:n]), 'lane 2 changed a row main re-worded'
    assert [r.split('|')[1] for r in ours[:n]] == [r.split('|')[1] for r in theirs], (ours[:n], theirs)
    assert len(ours) == n + 1 and ours[n].startswith('| `hosting/` |'), ours[n:]
    return theirs + ours[n:]

def report_rows(ours, theirs):
    assert len(ours) == 1 and ours[0].startswith('| `docs/reports/2026-11-26-step-2-6-previews.md` |'), ours
    assert len(theirs) == 1 and theirs[0].startswith('| `docs/reports/2026-11-26-main-wall.md` |'), theirs
    return theirs + ours

p = 'docs/REPO_MAP.md'
out[p] = splice(read(p), [e2e_rows, report_rows])

# ROADMAP: main moved change-set 2 (merged) from the end of "Recent work" to "Build history"; lane 2 kept the base.
CS2 = '**Change-set 2 of the build plan, First page [2026-11-18 to 11-19, MERGED]** — cs2-first-page (foundation'

def cs2_insert(ours, theirs):
    assert ours == [] and theirs[0] == CS2 and theirs[-1] == '', (ours[:1], theirs[:1])
    return theirs

def cs2_removal(ours, theirs):
    assert theirs == [] and ours[0] == '' and ours[1] == CS2 and in_base('ROADMAP.md', ours), (ours[:2], theirs[:1])
    return theirs

p = 'docs/ROADMAP.md'
L = splice(read(p), [cs2_insert, cs2_removal])
assert L.count(CS2) == 1
# Main holds 12 "Recent work" entries; with lane 2's step 2.6 entry it would be 13 (the file's rule: 8 to 12). Its
# oldest, "CI builds the WebAssembly and runs the tests that need it" (PR #64, merged), moves byte for byte to "Build
# history" after change-set 2's entry (his OLD-LINES (a)), as in 6c812700, 90c939f4 and 851f004.
rw = L.index('## Recent work')
starts = [i for i in range(rw, len(L)) if L[i].startswith('**')]
print('  ROADMAP: Recent work entries before the move:', len(starts))
assert len(starts) == 13, len(starts)
s = starts[-1]
assert L[s].startswith('**CI builds the WebAssembly and runs the tests that need it [2026-11-19 to 11-24, MERGED]**'), L[s][:80]
e = s
while e < len(L) and L[e] != '':
    e += 1
moved = L[s:e]
assert all(l == '' for l in L[e:]), 'something follows the CI-WASM entry'
assert L[s - 1] == '', 'no blank line before the CI-WASM entry'
L = L[:s - 1] + L[e:]
j = L.index(CS2)
assert j < L.index('## Current state (verified against the build, 2026-09-25)'), 'change-set 2 is not in Build history'
while L[j] != '':
    j += 1
L = L[:j + 1] + moved + [''] + L[j + 1:]
print('  ROADMAP: moved', len(moved), 'lines (CI-WASM, PR #64) to Build history after change-set 2')
rw = L.index('## Recent work')
n = len([i for i in range(rw, len(L)) if L[i].startswith('**')])
print('  ROADMAP: Recent work entries:', n)
assert 8 <= n <= 12, n
out[p] = L

# STATUS: five blocks.
def last_updated(ours, theirs):
    assert len(ours) == 2 and in_base('STATUS.md', ours[:1]) and ours[0].startswith("_Last updated: 2026-11-26 (guard-wrappers' docs pass")
    assert ours[1].startswith("_Last updated: 2026-11-26 (step 2.6's docs pass"), ours[1][:50]
    assert len(theirs) == 1 and theirs[0].startswith("_Last updated: 2026-11-26 (MAIN-WALL's docs pass"), theirs
    return theirs + ours[1:]  # main replaced its own line; lane 2's stays

def in_flight(ours, theirs):
    assert len(ours) == 3 and in_base('STATUS.md', ours[:2]) and ours[2].startswith('| #72 |'), [o[:30] for o in ours]
    assert len(theirs) == 2 and theirs[0].startswith('| none yet | MAIN-WALL') and theirs[1].startswith('| #75 (draft) |'), [t[:30] for t in theirs]
    return theirs + ours[2:]  # main replaced its merged rows; lane 2's #72 row stays

def items(lines):
    starts = [k for k, l in enumerate(lines) if re.match(r'^\d\. ', l)]
    return {lines[k][0]: lines[k:(starts[q + 1] if q + 1 < len(starts) else len(lines))] for q, k in enumerate(starts)}

def next_items(ours, theirs):
    o, t = items(ours), items(theirs)
    assert sorted(o) == sorted(t) == ['2', '3', '4'], (sorted(o), sorted(t))
    assert in_base('STATUS.md', o['2']) and in_base('STATUS.md', o['4']), 'lane 2 changed item 2 or 4'
    assert in_base('STATUS.md', t['3']) and not in_base('STATUS.md', o['3']), 'main changed item 3, or lane 2 did not'
    return t['2'] + o['3'] + t['4']  # main's items 2 and 4, lane 2's item 3

def follow_ups(ours, theirs):
    k = next(q for q, l in enumerate(ours) if l.startswith('- **From step 2.6** (#72, lane 2;'))
    rest = ours[:k] + ours[k + 1:]
    assert in_base('STATUS.md', ours[:k]) and in_base('STATUS.md', ours[k + 1:]), 'lane 2 changed a line main changed'
    assert theirs[0].startswith('  STATUS (2026-11-24)"; #60, #61, #64 to #71, #73, #74'), theirs[0][:60]
    return theirs[:1] + [ours[k]] + theirs[1:]  # main's lines; lane 2's bullet after the merged-PR pointer, as before

def risks(ours, theirs):
    assert ours[-1].startswith('- **Preview hosting (lane 2):**') and not in_base('STATUS.md', ours[-1:]), ours[-1][:40]
    assert theirs[-1] == '- **Preview hosting (lane 2):** PHC-HEADER (the header mark; revised by #67, merged); PHC-PROBE-MAIL.'
    assert in_base('STATUS.md', ours[:-1]) and in_base('STATUS.md', theirs[-1:])
    return theirs[:-1] + [ours[-1]]  # main's v2 risk lines, lane 2's preview line

p = 'docs/STATUS.md'
L = splice(read(p), [last_updated, in_flight, next_items, follow_ups, risks])
n = len(L) - (1 if L[-1] == '' else 0)
print('  STATUS lines:', n)
assert n <= 120, n
out[p] = L

for p, lines in out.items():
    assert not any(l.startswith(('<<<<<<< ', '>>>>>>> ', '||||||| ')) or l == '=======' for l in lines), p
    if '--write' in sys.argv:
        open(p, 'w', encoding='utf-8', newline='\n').write('\n'.join(lines))
print('written' if '--write' in sys.argv else 'dry run')
