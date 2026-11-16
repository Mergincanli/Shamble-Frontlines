# Verifier: C62H-BOUND report draft (c62h-bound-report-draft.md, 538 lines)

## Plan
- Read only. Compare every number, SHA, file:line, log path and verdict in the draft with its source.
- Sources: c62h-bound-build.md, the brief (section 9 items), advisor-c62h-bound-p3-full.json (order 4-9, P1-P7),
  advisor head / merge-stop JSONs, the three review JSONs, the card, u4r-report.md, history.md (line refs only),
  .gate/u4r/c62h-bound/*.log, .gate/u4r-sets/*.log, results/ files, git (plain read commands).
- Check coverage: brief section 9 items; order items 4-9; P1-P7; no Mustafa words quoted beyond line refs.
- One line per finding below, saved as I go. (No Edit tool here: this file is rewritten whole with Write.)

## Findings (one line each)
- Header: brief 663 lines, sha256 7936074c…857 equal; p3 JSON facf298e… and 71 lines equal; u4r-report.md :997, :1008
  (Bounded lines), B1 :1035 and :1167 equal. history.md refs :2143-2146 (C62H-BOUND (a)), :2172-2175 (YES), :2127
  (13:35 rule), :2146, :2172, :2174, :2175, :2187 all point at the right lines. OK.
- Hashes OK: head JSON a64259f5, merge-stop 55ad3561, reviews 3bf8888c / 5adfdf18 / 3f39ade6, built check 4e35e703,
  set check e9eb9b23…ea8 (equal at ddd38ba), texts v3 3f21c9b7, freeze 9cb8be17, paste cfac7502.
- Section 13: 7526ffe full SHA, 21:58:36 UTC, u4r-report.md +3 -2, 76680d3a → 777ee782: OK.
- Section 1 commits: all eight full SHAs and ddd38ba equal git; times 11:01, 14:23, 15:36, 16:49 equal git.
- **WRONG 1 (section 1, draft :46):** commit 2 b423a7d "(11:54)"; git's author and committer date and the branch reflog
  say 12:18:58 UTC. (The build notes :243 carry the same wrong "11:54".)
- Section 1: diff hashes 0b41121e / f88bf7d5 / a1238e90 / 7b775057 and patch-ids 036447a4 / 654d080f / 77464093 /
  f1d1de9f recomputed: equal. git diff --quiet c62h-bound-build 561bf42 exit 0. SIZE-CHECK recomputed: steps 499 /
  322 / 99 / 47, lane 499 / 821 / 920 / 967, +904 -63, efe8401..561bf42 same; all 14 per-file counts equal; 212 / 92
  / 83 / 1.5x arithmetic OK; B-2 40 OK; netcode-architect's "about 6" and 33 of 47 in its JSON: OK; range 545-755 is
  the paste's :87 (draft cites history.md:2172 for it, which holds only "about 650"; attribution nit, not counted).
- Section 6: merge.log 16:06:04-16:16:23 exit 1, 552/0, 429/1, test name at :51 (and :52), 1,577 lines; merge-2.log
  16:31:00-16:48:07 exit 0, free 5278, 622.2 s, 215.5 s, 200.8 + 2.5 + 0.9 = 204.2, 9.36 s, both hash lines, real
  1m55.935s: all equal. Baselines 113.1 / 121.1 / 123.8 equal the rust review. failed-steps/test.log gone: confirmed.
  check.mjs:2655-2657 is the FX-10 comment (removed when the step starts). GATE PASS in gate-merge.log. OK.
- Section 4: built-check logs 12 PASS 13 FAIL (negative), 25 / 0 on b423a7d, b01d3ac, 561bf42; three after-hashes and
  73 words equal; rejudge self-test 4 of 4; set-check dry / dry-2 / negative 12 PASS 29 FAIL; self-test PASS; P2 46
  PASS 0 FAIL, 34 lines; P3 32 PASS 0 FAIL, 19 lines. All equal.
- Head JSON (H1-H6) and merge-stop JSON (C1-C8, 202 of 632, 96 logs, script condition (iii)): the draft's readings
  equal them.
- Section 2: step logs' times (10:46:22-10:58:53, 11:40:16-11:52:06, 12:29:32-12:42:37, 14:34:29-14:49:56), exit 0,
  counts 545/625, 551/631, 552/632, 552/632; the 16 new test names in the runner's output; step3-judge-after.log :102
  and :228; the short P2 line (S 1920, L 29, B 0, E1 0 E2 29 N 0; bites N 16 and join 0 / 16): all equal.
- Section 3: plant logs' times (10:43:01-10:46:00; 11:36:39-11:38:56; 14:33:59), "1 failed" / "2 failed", sent 180 /
  202 / 120 vs 90, (28, 29), the three messages, engine.rs :1624 / :1677 / :1632 / :1610 / :1680, CB-P11 17 passed 5
  failed: all equal.
- Section 5 and 15: the read_line (marks :3845-4047, :3983-3993, :3994-4004, :4035-4038, judge ranges, 7 files 376 /
  12), B-2 answers and every deferred Low's file:line equal the three review JSONs.
- **WRONG 2 (card line refs, draft :238, :244, :438, :490, :495, :503, :505):** they match the card at 055d152 (21:59)
  but not the card now (67de11e, 4129 lines; already off at 3b826b4/b3fb754, 4128 lines): U3C-HELD-1 :3746-3759 is now
  :3760-3773; CHECK-NFF-1 :3760-3761 now :3774-3775; the deferred Lows :3762-3765 now :3776-3779; the 12:44-14:22
  restart :3198-3203 now :3212-3217; the 17:09-20:36 entry :3184-3197 now :3198-3211; slip 16 :3195-3197 now
  :3209-3211; slip 17 :3587-3588 now :3601-3602. Only :55-56 still holds.
