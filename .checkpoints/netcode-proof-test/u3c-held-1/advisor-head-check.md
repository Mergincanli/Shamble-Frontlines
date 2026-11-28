# Advisor head check: PR #40 head aab3712fca3af64b25169af1f4ee75c01a0411cd (U3C-HELD-1, plan step 9)

Role: advisor (main's .claude/agents/advisor.md, read via git show origin/main:...). Day-session review of a pushed head.
Read-only: no cargo, no check.mjs, no main-run.sh; read-only git.

## Plan
1. Read the plan, my plan review (C1-C10, HELD-Q1..Q12), build.md, the three reviews.
2. Check 1: 0ad9661 diff = plan/C1 (3 files, 74+/42-, test-only lines, hashes = build.md, message footer, no D-1/P1-P5).
3. Check 2: commits since the plan started touch only .checkpoints/ except 0ad9661 and 978202d.
4. Check 3: C5 / E6 pass bar written before the call; counted.log items (exit code + summary).
5. Check 4: C1-C10, rulings, L1 run 94 catch, E3/E4/E5 against the logs.
6. Check 5: CI on aab3712 (check runs / workflow runs).
7. Check 6: card and build.md say only what records show (cut record, step order).
8. Verdict, findings ADV-UH1H-n.

## Findings log (one line per finding as it comes)
- Check 1 HOLDS: 0ad9661 = 3 files (delay.rs, delay_tests.rs, SERIES.md), 74+/42-, modes 100644; parent 978202d. All 78
  "+" lines of the design notes' proposed diff (netcode-architect-design.md:141-302) occur verbatim in the fixed files
  (0 missing); SERIES.md old phrases 2 and 2 before, new 2 and 2 after, old 0 after. Every non-doc change: Life
  (#[cfg(test)] struct :376), held_now's #[cfg(test)] call, check_held_bound / check_held_falls (#[cfg(test)]),
  model_write's #[cfg(test)] call; delay_tests.rs is `#[cfg(test)] #[path = "delay_tests.rs"]` (delay.rs:1915-1916).
  Hashes at 0ad9661 = aab3712 = working tree = build.md's fixed: 3dbbccf7…, 8f8c0f9a…, 1dd67e2f…. Message ends with
  Co-Authored-By + Claude-Session; the only model name is in Co-Authored-By. D-1 and P1-P5: no commit carries them
  (only 978202d and 0ad9661 touch non-.checkpoints paths; 0ad9661's diff is the fix alone).
- Check 2 HOLDS: 47af81a..aab3712, 27 commits; non-.checkpoints paths only in 978202d (one file, A,
  spikes/netcode-proof/results/u4r/64f3307d5c7f/B1/raw-logs.sha256, 12 lines) and 0ad9661. No merge commit.
- CI (check 5) on aab3712: gate push 44084854535 success; toolchain push 44084854552 success; gate pull_request
  44084863595 success; toolchain pull_request 44084863752 success. Check runs (6): toolchain 134044527505, gate
  134044526083, smoke 134044525593, smoke 134044493463, gate 134044492975, toolchain 134044492260: all completed
  success. PR head = aab3712 (draft, mergeable clean). 978202d's four runs also success (11:56).
- My own gate at aab3712 (main folder, HEAD = the PR head; tools/, .github/, .claude/, CLAUDE.md, legacy/ equal
  origin/main): `node tools/gate.mjs --no-smoke` GATE PASS ([1]-[4], [5b]-[5d]; smoke is CI's, green twice).
- Check 3 (C5) HOLDS: transcript 077a208e…jsonl: the pass-bar Edit to build.md at 12:18:35.883Z (result 12:18:36.590Z),
  the counted Bash call at 12:18:37.320Z; that Edit's new_string is still verbatim in build.md (not edited after).
  counted.log (29 lines): header HEAD 0ad9661 (rev-parse --short; main-run.sh:39-43, diff/untracked scoped to
  spikes/netcode-proof), diff and untracked e3b0c442…; "check: all 11 steps pass"; "=== 12:34:46 UTC exit 0: node
  check.mjs"; "--- end 12:34:46 UTC, exit 0"; the task notice "completed (exit code 0)". Items: 552 / 632, 0 failed,
  0 skip; refs "1382 tracked files" = git ls-tree -r at 0ad9661 (1382); marks 81; self 78 of 78; redact 10 of 10, 306
  items, 0 findings; deps "(137 crate files and 1 cargo config file(s)"; wasm 27000 / 0 mismatching both legs. Free
  5,552 -> 5,514 MiB. One counted log only.
- L1 (C2): l1.log 188 runs exit 0 (each 169 passed, G-NC2 ok) + run 94 exit 101 = 189; header HEAD e85fdba, diff
  bdc5bba4 (D-1), BIN npt_net-ed8673eeeaf4db12, D-1 hashes b70ab4e5/2ccaccc3. Run 94: G-NC2 alone, delay_tests.rs:2302,
  left [1, 0]; D-1 line (l1.log:227, l1-1-94.log:188): excess 14326, waiting [1, 0], at writes [0, 0], first losses 28.
  The Confirmed line holds whole; no stop line.
- E3: e3a 600 of 600 "1 passed; 0 failed", e3b 600 of 600, e3c 20 of 20 "169 passed; 0 failed"; every header diff
  a7199f7a… = `git diff 978202d 0ad9661 -- spikes/netcode-proof | sha256sum` (the tested tree is the commit); fixed
  hashes in each; BIN ed8673 in all 16 header mentions (E3 + plants); "first failure: none"; exit 0.
- E4 (C3): P1-P5 each rebuilt on a stream from 0ad9661's delay.rs (sed 481d; 546 s/+ prefix//; 1597 moved after
  1615; 634-635d; 488d): hashes 091957eb, 0e65d301, dfbf8d2d, 7b41e90a, 09e3fb98 = the five log headers: each plant
  was exactly the planned edit; delay_tests.rs 8f8c0f9a in all five. Asserts and values as build.md:73-97; failing
  tests per run log: P1 2, P2 1 (G-NC2), P3 2, P4 2, P5 1 (stalled-client); no other test.
- E5: npt-lab 98c3967e… (= preflight.log's), header diff a7199f7a (all plants out). light.log fmt/clippy/doc PASS (C4).
  b1-redact 306 items, 0 findings.
- C6: packing opened: RECS-1043 answers C62HB-Q4 (b), the C62H-BOUND report's one decision
  (c62h-bound-report-draft.md:37), which meets my HELD-Q7 test ("he answers one of its questions"). 8.1 followed
  (transcript 11:54:44-11:56:59): pgrep exit 1; free 5,191,479,296 B; list; redact; commit of that one file; push;
  pack ("free before: 4950 MiB", 227,889,786 from 860,377,134, "read back: 11 of 11", removed, "free after: 5553
  MiB"). Bar 5,220 held, so P3 not packed; no R4 folder touched. Every call started above 4,096 (lowest 4,899, e3b).
  Note: the plan's section 6 still says "about 7 builds"; my C6 text carries the correction; no effect.
- C7: 482 tool calls from 08:30 to the end: none waited over 90 s for its result (no approval wait).
- C8: wf_88c44554-26a ran netcode-architect, test-engineer, rust-reviewer with effort 'xhigh' (the workflow's own
  script); review files hash 015aa739/834bf936/ac2cf470 as build.md; 0 C, 0 H, 1 M, 12 L; one fix pass, notes only.
- C9: plan diff ec70096..aab3712 = only the three C9 edits (card entry by name, saver sentence, the dated
  difference-set paragraph); its cites hold at 0ad9661 (:541-545, :635, :698-709, :829, :1530-1534, :1597,
  :1782-1785; extends only at :829 and :1597, drains at :1526 and :1771). The report half: no report yet (finding 1).
- C10 / order: no push between 978202d (11:56:14) and aab3712 (12:37:14) (origin reflog); E6 ended 12:34:46; gate
  12:35:40-12:35:43 PASS; save 12:36:04 pushed. One counted call. SIZE 116 lines. D-1/P1-P5 in no commit.
- Cut record: notice text and 11:57:32.558 / 11:59:56 / 11:59:59 / 12:00:02, git diff --cached at 11:57:32.256,
  queued resume 12:13:14.810, worker restart 12:13:15.983, taken in 12:13:37.941: all as the transcript shows.
- ADV-UH1H-1 Low: build.md:108-122 and card :313-315 say Lows are "carried into the report" and the deferred ones
  are "on the card's Open findings"; no U3C-HELD-1 report exists yet and the card's Open findings has no entry for
  them (UH1- only at :300, :313, :314). Fix at the close: the Open findings entry and the close report (C9's half).
- ADV-UH1H-2 Low: brief 8.1 asks the pack's free-before, free-after and read-back lines on the card; the card has
  free 4,950 -> 5,553 only; read-back and archive lines only in build.md:128-130 (and the transcript); the card's
  KEPT-LOGS record has entries for packs 1 and 2, none for this third pack. Fix: add it at the next card save.
- ADV-UH1H-3 Low: the plan's hash after C9 (d6801289…) is recorded nowhere (build.md:2, card :287 give a53b2803).
  Fix: note it beside the C9 line at the close.

## Verdict
APPROVE WITH FINDINGS on aab3712fca3af64b25169af1f4ee75c01a0411cd. C1-C8 and C10 met; C9 met in the plan, its report
half due at the close. CI green (4 runs, 6 check runs). Three Lows, no push for them; they ride the close save.
