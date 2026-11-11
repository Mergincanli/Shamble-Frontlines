#!/usr/bin/env python3
"""docs-keeper, foundation 2.1a: docs/ROADMAP.md. Idempotent: starts from bb2d817's copy."""
import subprocess
REPO = '/home/user/Shamble-Frontlines'
t = subprocess.run(['git', '-C', REPO, 'show', 'bb2d817:docs/ROADMAP.md'], capture_output=True, check=True).stdout.decode('utf-8')

def rep(a, b):
    global t
    assert t.count(a) == 1, a[:80]
    t = t.replace(a, b)

# 1. status-diet: merged.
rep('**STATUS-DIET with LANE-RULES [2026-11-10, PR #49: in review]**', '**STATUS-DIET with LANE-RULES [2026-11-10 to 11-11, MERGED]**')
rep('Report `docs/reports/2026-11-10-status-diet.md`. Next: the advisor\'s final-head review and his go.',
    'Report `docs/reports/2026-11-10-status-diet.md`. Merged: PR #49, bb2d817, 2026-11-11 09:03 UTC, by merge commit, on\n'
    'Mustafa\'s go after the advisor\'s APPROVE of 2e6e5d2; CI green on main (run 42991918886).')

# 2. 2.1a on top.
new = '''**The foundation, 2.1a: the legacy gate reads both layouts [2026-11-10 to 11-11, PR #50: in review]** —
foundation-restructure, the first of step 2.1's two pull requests (Mustafa's decision of 2026-11-10: the second
account plans and builds steps 2.1 and 2.2 now; his SPLIT (a)); gate files and tools only, no game file, no file
moved, symbols +0 −0 (dd12aeb). `tools/smoke.mjs` finds the build folder (`legacy/` when it holds a `shamble-*.js`
file, else the root; engine files in both is a setup error, exit 2); `tools/gate.mjs` reads the head there and each
base from `legacy/`, the root or `shamble-frontlines-repo/`; `tools/village.py` defaults to `legacy/` when present;
`gate-globals.json` is called a gate file. Proof: the mutation proof, 31 of 31 PASS (every planted fault fails the
same checks on the old gate, the edited gate at the root and the edited gate on `legacy/`; every control passes on all
three); base checks over root, CS-1 and `legacy/` bases; gate PASS and CI green. PR #50 replaces #47 (his BRANCH-47 (a)); its docs pass records #49's merge and his words of 2026-11-10
and 2026-11-11 (`docs/DECISIONS.md`). Next: the review round, the report, the advisor's review and his go.

'''
anchor = '*(Newest first, a few lines each. Older items fold into "Build history" above.)*\n\n'
rep(anchor, anchor + new)

# 3. Fold the two oldest Recent work entries into Build history.
k31 = t.index('**Kit v3.1 — routine guard [2026-10-27 to 11-04, MERGED]**')
dr = t.index('**v2 decision drafts [2026-10-31 to 11-03, MERGED]**')
assert k31 < dr
end = t.index('\n', dr) + 1
assert t[end:].strip() == '', repr(t[end:end + 80])
assert t[k31 - 2:k31] == '\n\n'
t = t[:k31 - 1]  # drop both entries (and the blank line before Kit v3.1)
if not t.endswith('\n'): t += '\n'
hist_anchor = '  planned in a fresh session (DRAFTS-SESSION (a)).\n'
rep(hist_anchor, hist_anchor +
    '- **v2 decision drafts (2026-10-31 to 11-03):** PR #33, e4d8b22, by squash (docs only): the six decision drafts\n'
    '  `docs/adr/0001`–`0006` (Proposed), `docs/ENGINE_OVERVIEW.md`, `docs/SPIKE_PLAN.md`, the register `docs/adr/README.md`,\n'
    '  GLOSSARY §8 and `docs/LOOK_V2.md`; signed off by Mustafa and the Advisor on 2026-11-03 as the basis for the spikes,\n'
    '  not yet accepted; reports/2026-11-01-v2-decision-drafts.md.\n'
    '- **Kit v3.1, the routine guard (2026-10-27 to 11-04):** PR #30, e2b5799, on Mustafa\'s "merge #30" (rules; no game\n'
    '  code): `.claude/hooks/` (`routine-guard.mjs`, `guard.mjs` and their tests) make the night shift and the PR Advisor\n'
    '  safe to run unattended (a limited shell, no web, no push to main, no `.claude/` edits; day sessions unchanged);\n'
    '  GUARD-FILE (a); reports/2026-10-27-kit-v3-1.md.\n')

# 4. Next / open: a dated pointer beside the old "before the foundation" order, and #44's state.
rep('the netcode proof test before the foundation); then phase 1',
    'the netcode proof test before the foundation; pointer, 2026-11-11: for foundation steps 2.1 and 2.2, overtaken by '
    'Mustafa\'s decision of 2026-11-10, `docs/DECISIONS.md`); then phase 1')
rep('the review policy (A4), in flight as PR #44,', 'the review policy (A4), merged as PR #44,')

open(REPO + '/docs/ROADMAP.md', 'w', encoding='utf-8').write(t)
rw = t[t.index('## Recent work'):]
print('Recent work entries:', sum(1 for l in rw.split('\n') if l.startswith('**') and '] —' in l or l.startswith('**') and ']** —' in l))
