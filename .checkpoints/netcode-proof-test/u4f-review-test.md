# U4f review panel, round 1, seat `test` (test-engineer), 2026-11-19

Tree: /home/user/Shamble-Frontlines/.claude/worktrees/agent-a0825a5a2491eb175 (HEAD 07289a4 + uncommitted U4f change). Read only.
(No Edit tool in this seat: the notes file is rewritten whole with Write at each save.)

## Plan
1. Mustafa's SIZE-U4F words (history.md:6896-6903) and advisor-u4f-stop.md :80-89 (the U4f2 list), :198-210 (S1-S7, N1).
2. u4f-brief.md 2.10 (tests, negative cases, minimums), 2.11 (bites/plants), 5.3, 5.6, 6; advisor-u4f-brief.md rulings + L1-L6; advisor-u4f-build1.md R1-R4/F1-F11.
3. The patch: numstat; confirm no marks.json / judge file touched; SERIES.md +20 lines, -0.
4. Each new test file + bites.rs: map brief 2.10's named tests to code, count U4f's negative cases; tests that cannot fail; timing-dependent tests.
5. marks.json bite ids vs the code's BITES lists (NA-11).
6. End-check logs A2/B: measured and bite counts vs minimums; hand logs.
7. SERIES.md's 20 new lines: the read.
8. Runs only to prove a finding (runner, one command per call, logs under .gate/u4f/review-test/).

## Findings log (one line each, as found)
- [14:1x] Bars: `git apply --numstat` on wip/u4f-build.patch (sha256 74ae26aa..., checked) lists 29 files; no marks.json, no judge file, no marks/README; SERIES.md 20 added / 0 removed. `git diff --quiet 5b29296b HEAD -- spikes/netcode-proof` exit 0 in the worktree (HEAD = base in spikes). His 13:35 rule: nothing to read; confirmed.
- [14:1x] Test inventory (scratchpad tests_list.py, read only, worktree vs HEAD): 96 `#[test]` fns added, 1 removed (P33's move: loopback.rs -> lab/tests/bite_homes.rs). 17 bites-only expected (14 unit proofs + f30 x2 + lab `f2_the_sims_check_bites_are_routed...`), so +78 measured, +95 bites: equals A2/B (to confirm the cfg gates).
- [14:1x] Minimums arithmetic: 2.10's 67 negative = 65 from the table's neg rows + U4F-Q6 (b) 1 + U4F-Q9 (a) 1. U4f2's share: f15 x6, f16 24, f22 2, f23 1, f33 13 = 46 measured/bites, 22 negative. So U4f's share = measured 73, bites 88, negative 45. Built 78 / 95 (meets); negative to be counted from the code below.
