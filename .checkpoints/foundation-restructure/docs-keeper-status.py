#!/usr/bin/env python3
"""docs-keeper, foundation 2.1a: STATUS.md and STATUS_ARCHIVE.md. Idempotent: starts from bb2d817's copies.
#49's STATUS row is moved into the archive's "In flight" by script (its cells copied, never retyped)."""
import subprocess

REPO = '/home/user/Shamble-Frontlines'
def base(path):
    return subprocess.run(['git', '-C', REPO, 'show', 'bb2d817:' + path], capture_output=True, check=True).stdout.decode('utf-8')

st = base('docs/STATUS.md')
ar = base('docs/STATUS_ARCHIVE.md')

def rep(text, a, b):
    assert text.count(a) == 1, a[:80]
    return text.replace(a, b)

# --- #49's row, by script ---
rows = [l for l in st.split('\n') if l.startswith('| #49 |')]
assert len(rows) == 1
row49 = rows[0]
cells = [c.strip() for c in row49.strip('|').split(' | ')]
assert len(cells) == 5 and cells[0] == '#49', cells
_, work, branch, state, waiting = cells

arch_row = ('| **' + work + '** (Mustafa\'s queue of 2026-11-10, items 2 and 3, with ACCOUNTS and his Models addition; docs and '
            'rules, no game code, symbols +0 −0) | merged: PR #49 (' + branch + '), bb2d817, 2026-11-11 09:03:10 UTC '
            '(the merge commit\'s date), by merge commit (parents 2d22b69 and 2e6e5d2), on Mustafa\'s go, given in the status-diet day '
            'session\'s chat (session_0173vVo6Vtm2C54toWvTt8sR) at about 09:02 UTC (his words, from the merge commit\'s '
            'message: `docs/DECISIONS.md`, 2026-11-11), after the advisor\'s APPROVE of 2e6e5d2 (PR comment 6779064873); '
            'CI green on main (push run 42991918886, on bb2d817); report `docs/reports/2026-11-10-status-diet.md` '
            '| done: ' + state + ' (its row in STATUS until the merge); then the advisor\'s APPROVE of the final head '
            '2e6e5d2 and his go; its merge recorded by foundation 2.1a (PR #50), as his go says ("The next change-set '
            'records it") | none |')
# the row moved: its cells appear byte for byte in the archive row
for c in (work, branch, state):
    assert c in arch_row
anchor = '|---|---|---|---|\n| **kit-v3-5, the guard\'s rename gap**'
ar = rep(ar, anchor, '|---|---|---|---|\n' + arch_row + '\n| **kit-v3-5, the guard\'s rename gap**')

# --- the kit-v2-1 Low, fixed in dd12aeb ---
low = ('- Deferred, low (kit-v2-1): `tools/gate.mjs:333` and `tools/README.md:34` still call `gate-globals.json` "reviewed '
       'data, not a gate tool", which contradicts kit-v2-1\'s category 3 (gate files need Mustafa\'s go). It errs on the '
       'safe side; fix the wording when gate.mjs is next touched')
ar = rep(ar, low + '\n', low + '. **Fixed** in dd12aeb (foundation 2.1a, PR #50; on main once #50 merges): both now call '
         'it a gate file whose change needs Mustafa\'s go [verified: `tools/gate.mjs:355`, `tools/README.md:35`]\n')

# --- STATUS ---
st = rep(st, row49 + '\n', '')
old47 = [l for l in st.split('\n') if l.startswith('| #47 |')]
assert len(old47) == 1
st = rep(st, old47[0], '| #50 | the foundation, 2.1a: the legacy gate and tools read both layouts (no file moves); replaces #47 '
         '(his BRANCH-47 (a), 2026-11-11) | `claude/pensive-heisenberg-aazidf` (his personal account) | draft: the docs '
         'pass done; the review round and the report next | the review round and the report, then the advisor\'s review '
         'of its ready head and his go (risk tier 2: gate files; decision docs); 2.1b next |')
st = rep(st, '_Last updated: 2026-11-10 (status-diet, PR #49; main 2d22b69)._',
         '_Last updated: 2026-11-11 (foundation 2.1a, PR #50, its docs pass; main bb2d817)._')
st = rep(st, '''- **Last merged:** kit-v3-5, F1, the guard's rename gap (PR #46, 2d22b69, 2026-11-10 16:05:52 UTC, by merge commit, on
  Mustafa's go after the advisor's APPROVE WITH NOTES of 0573db3, comment 6772168492): the PR Advisor merges only from
  an allow-list. Every merged change-set: `docs/STATUS_ARCHIVE.md`, "In flight"; `docs/ROADMAP.md`.''',
'''- **Last merged:** status-diet, STATUS-DIET with LANE-RULES (PR #49, bb2d817, 2026-11-11 09:03:10 UTC, by merge
  commit, on Mustafa's go after the advisor's APPROVE of 2e6e5d2, PR comment 6779064873): STATUS split into this
  dashboard, `docs/DECISIONS.md` and `docs/STATUS_ARCHIVE.md`; CLAUDE.md's "Lanes and accounts" and the advisor-tool
  lines. Every merged change-set: `docs/STATUS_ARCHIVE.md`, "In flight"; `docs/ROADMAP.md`.''')
st = rep(st, '''  `docs/DECISIONS.md`, "Queued"). His exceptions: a broken main, STATUS-DIET and LANE-RULES (this change-set); and by
  FREEZE-FDN it "does not stop the smallest edits the foundation needs to keep the kit true after its own moves"
  ("Each such edit still asks me and merges on my go").''',
'''  `docs/DECISIONS.md`, "Queued"). His exceptions: a broken main, STATUS-DIET and LANE-RULES (carried out by
  status-diet, #49, merged); and by FREEZE-FDN it "does not stop the smallest edits the foundation needs to keep the
  kit true after its own moves" ("Each such edit still asks me and merges on my go"). His two additions of
  2026-11-11 for the foundation chat: `docs/DECISIONS.md`.''')
st = rep(st, '''CI green on 2d22b69 (run
  42851159265).''', '''CI green on bb2d817 (run
  42991918886).''')
st = rep(st, '''1. In his order (ORDER (b), 2026-11-10): this change-set; then his unit walk of 2026-11-09 (a docs change-set: UNITS.md
   §0 and LOOP_V2), then the Slice 1 note of 2026-11-10, each in a fresh chat. His words: `docs/DECISIONS.md`,
   "Queued".''',
'''1. In his order (ORDER (b), 2026-11-10), after status-diet (merged, #49): his unit walk of 2026-11-09 (a docs
   change-set: UNITS.md §0 and LOOP_V2), then the Slice 1 note of 2026-11-10, each in a fresh chat. His words:
   `docs/DECISIONS.md`, "Queued".''')
old2 = '''2. v2: the netcode proof test (#40); the foundation after this change-set (2.1a, #47; then 2.1b; then 2.2, #48). Wave 1
   runs one heavy spike at a time (SPIKE_PLAN: the netcode proof test, the sim core spike and its 60 Hz run, the
   boundary spike, the look proof test once its models exist; the preview hosting check alongside any). Then phase 1
   and the playable slices (SLICE-ORDER, 2026-10-31: `docs/DECISIONS.md`).'''
new2 = '''2. v2: the netcode proof test (#40); the foundation in his order of 2026-11-11 (#49 merged; then 2.1a, #50 in #47's
   place; then 2.1b; then 2.2, #48). Wave 1 runs one heavy spike at a time (SPIKE_PLAN: the netcode proof test, the
   sim core spike and its 60 Hz run, the boundary spike, the look proof test once its models exist; the preview
   hosting check alongside any). _Pointer: his decision of 2026-11-10 lets the second account plan and build
   foundation steps 2.1 and 2.2 now (`docs/DECISIONS.md`, 2026-11-10)._ Then phase 1 and the playable slices
   (SLICE-ORDER, 2026-10-31: `docs/DECISIONS.md`).'''
st = rep(st, old2, new2)
st = rep(st, '''- His go for this change-set (risk tier 2: CLAUDE.md and docs-keeper.md; `docs/DECISIONS.md` is a decision doc), once
  the advisor approves its final head; then #47 (2.1a) on his go.''',
'''- His go for #50 (2.1a; risk tier 2: gate files; `docs/DECISIONS.md` and the decision docs' pointers), once the advisor
  approves its final head; no Brave check for 2.1a. Then, in his order of 2026-11-11, 2.1b and #48 (2.2).''')
st = rep(st, '''VISION_V2's line ranges in ADR 0005; and, from this change-set,''',
         '''VISION_V2's line ranges in ADR 0005; and, from status-diet,''')

open(REPO + '/docs/STATUS.md', 'w', encoding='utf-8').write(st)
open(REPO + '/docs/STATUS_ARCHIVE.md', 'w', encoding='utf-8').write(ar)
n = st.count('\n')
print('STATUS lines:', n)
assert n <= 120
for l in st.split('\n'):
    if not l.startswith('|') and len(l) > 120: print('long line:', len(l), l[:60])
