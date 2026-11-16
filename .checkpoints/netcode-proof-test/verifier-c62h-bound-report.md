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
- Section 13: 7526ffe full SHA, 21:58:36 UTC, u4r-report.md +3 -2, 76680d3a → 777ee782: OK. u4r-report.md :990, :997
  and :1008 equal the_c_lines' three texts byte for byte (python3 -I compare).
- Section 1 commits: all eight full SHAs and ddd38ba equal git; times 11:01, 14:23, 15:36, 16:49 equal git.
- **WRONG 1 (section 1, draft :46):** commit 2 b423a7d "(11:54)"; git's author and committer date and the branch reflog
  say 12:18:58 UTC. (The build notes :243 carry the same wrong "11:54".)
- Section 1: diff hashes 0b41121e / f88bf7d5 / a1238e90 / 7b775057 and patch-ids 036447a4 / 654d080f / 77464093 /
  f1d1de9f recomputed: equal. git diff --quiet c62h-bound-build 561bf42 exit 0. SIZE-CHECK recomputed: steps 499 /
  322 / 99 / 47, lane 499 / 821 / 920 / 967, +904 -63, efe8401..561bf42 same; all 14 per-file counts equal; 212 / 92
  / 83 / 1.5x arithmetic OK; B-2 40 OK; netcode-architect's "about 6" and 33 of 47 in its JSON: OK; range 545-755 is
  the paste's :87 / :127 (the plan he said yes to; history.md:2172 holds "about 650"). OK.
- Section 6: merge.log 16:06:04-16:16:23 exit 1, 552/0, 429/1, test name at :51 (and :52), 1,577 lines; merge-2.log
  16:31:00-16:48:07 exit 0, free 5278, 622.2 s, 215.5 s, 200.8 + 2.5 + 0.9 = 204.2, 9.36 s, both hash lines, real
  1m55.935s: all equal. Baselines 113.1 / 121.1 / 123.8 equal the rust review. failed-steps/test.log gone: confirmed.
  check.mjs:2655-2657 is the FX-10 comment (removed when the step starts). GATE PASS in gate-merge.log. CI on ddd38ba
  (gh api, read only): six check runs, all success, the four run ids equal. OK.
- **WRONG 5 (section 6, draft :206-209, Low):** "check.mjs stopped at its test step"; merge.log shows check.mjs ran its
  later steps (deps, wasm, refs, marks, self, redact all PASS) and ended "check: 1 step(s) failed"; it was the
  main-run.sh call that stopped after check.mjs (det-u4, the build, the hash never ran).
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
- **WRONG 4 (section 3, draft :111, Low):** "Each was one temporary Edit"; CB-P6 was two edits (a prev_link vector and
  the changed room_tick argument; "Both edits restored": c62h-bound-build.md:79-81).
- Section 5 and 15: the read_line (marks :3845-4047, :3983-3993, :3994-4004, :4035-4038 spot-checked in marks.json at
  42b9646; judge ranges; 7 files 376 / 12), B-2 answers and every deferred Low's file:line equal the review JSONs.
- **WRONG 2 (card line refs, draft :238, :244, :438, :490, :495, :503, :505):** they match the card at 055d152 (21:59)
  but not the card now (67de11e, 4129 lines; already off at 3b826b4/b3fb754, 4128 lines): U3C-HELD-1 :3746-3759 is now
  :3760-3773; CHECK-NFF-1 :3760-3761 now :3774-3775; the deferred Lows :3762-3765 now :3776-3779; the 12:44-14:22
  restart :3198-3203 now :3212-3217; the 17:09-20:36 entry :3184-3197 now :3198-3211; slip 16 :3195-3197 now
  :3209-3211; slip 17 :3587-3588 now :3601-3602. Only :55-56 still holds. (Contents of each entry equal the draft.)
- Section 8: p0-rejudge.log 17:06:52-17:08:39, all exit 0, HEAD a87c7ec (spike tree = ddd38ba: exit 0), npt-judge
  d8b8c91e…, marks c1935a5b, peaks 3,920.3 (48.2 s) / 4,504.6 (52.8 s); the four verdict sha256s recomputed equal;
  rejudge-compare.log's per-part lines and SAME ANSWERS; cb37f56 holds the three files; advisor-u4r-sets-r2-full.md:10
  and :45 say what the draft says. OK.
- Section 9: preflight.log 20:44:46-20:45:43, 12 npt crates, 46.04 s, npt-lab 98c3967e…, S 5760 L 111 B 0 E1 0 E2 111
  N 0, resyncs 0; verdict.json -17 L1 P2 NOT-RUN "1 of 3 valid runs", all three checks 0 pass. OK.
- Section 10: pre-P2 20:47:14 HEAD 8ee453c tree 25c5b760; pre-P3 21:19:05 HEAD d66e23c tree 14d78602; four runner
  sha256s; FOUND c-u4r-runs line before P2; P2 log 1755.5 s / 1799.0 MiB (21:17:23 - 1755.5 s = 20:48:07); P3 1029.4 s
  / 995.8 MiB; every per-run S / L / B / E1 / E2 / N, incomplete, hashes, stop, fate log, resync and L/S % recomputed
  from runs.txt: equal; every verdict line and A / B / mismatches value in the three verdict files: equal; npt-lab
  98c3967e and npt-judge d8b8c91e in both runs.txt. OK.
- Note (not counted): "5,307 MiB before P3" (8.1) equals build.md:510 and the card, but no log holds it: pre-P3-c62h.log
  and the runner's start check both read 5306 MiB (5,563,871,232 B = 5,306.1 MiB). P3's end is 21:36:27 by its log
  (draft "about 21:20-21:37"); the pack log ends 09:16:47 (draft "09:10-09:16"). Within "about".
- Section 11: the ruling's s1 / s3 figures (27 resyncs, 25 slots, 76,910-77,844, 4 to 18 ticks, limit 60; slot 35 at
  76,803, 76,804-76,808; 228 = 201 + 27, 6 = 5 + 1), advisor-brawl1-sets-p3-full.md:18-22 and :20, the five Lows,
  P7's three items, card time 21:38-21:48: all equal. No "recorded as incomplete", no "not at all" in the draft.
- Section 12: paste :120-124 are the three stop rules; history.md:2175 adopts them. OK.
- Section 14: pack log 5883 / 6455 MiB, 217268267 from 816778012, 11 of 11; 6c8c7cf; runner sha256s at LANE_BASE and
  now; c-u4r-runs.mjs 99dd070e at 044fab1 and f2002890 at 42b9646; plan-dry.mjs 693bcf3f, 22 of 22; both P0 folders,
  fc411116c1f1, fd4612050cd6 and both c-u2 folders still unpacked under .gate/netcode-proof-test/runs/. OK.
- Sections 16-18 and Decisions: paste :101-106, design C62HB-Q4 (:170-175), advisor-c62h-review-full.md:90, rust
  review's scenario.rs:1051-1055 and Wasm line, the test-engineer's points, step4 JSON deviations: all equal. Not
  checked: "three seeds each, 48 clients" for -58's wrap parts (marks.json gives min 3; clients not found).
- **WRONG 3 (time zone, Low):** the key says "Times are UTC, 2026-11-16 unless named" (:9), but Mustafa's word times are
  Amsterdam, unnamed: "16:35" and "20:50" (:3-4), "his 13:35 rule" (:164) and "your 13:35 rule" (:522), "his 21:50
  words" (:499), "his 22:09 words" (:504), "at 22:09" (:537). history.md's headings give 15:35, 19:50, 12:35, 20:50
  and 21:09 UTC.
- Quotes: no quoted string in the draft is Mustafa's; his words appear only as paraphrase with history.md line refs. OK.
- Coverage, brief section 9: every item present (commits, size per step and total, B-2's four files, evidence not
  counted, counts and test names, CB-P1..P18, merge log, head check, built check, three shown-to-fail runs, P0
  re-judge, 8.4 lines, S/L/B and E1/E2/N, kept logs, runner sha256s, the read line, B-2 answers, (c), still
  unbounded, kept P0 set, Decisions / For Mustafa). Placement note: B-2's answers sit in section 5, not "beside the P0
  SAME ANSWERS line" (section 8).
- Coverage, order items 4-9: all carried (H3, H4, H6, C1-C8 with the full sha256s, the ten Lows with file:line, B-2,
  B-3, the re-judge hashes, deferred (ii), carry 58, ADV-CBP3-2 / -5, both caches, no pack, runner sha256s). The Lows'
  taker is one shared list ("the next npt-judge or npt-lab change, U4f, U4g or JDG-ONLY-1"), as the merge-stop
  ruling words it.
- Coverage, P1-P7: P1, P2 (section 13), P3, P4, P5, P7 carried. **MISSING: P6.** Only "P6's spike-tree check" in passing
  (:186); the report does not say its head gets the advisor's check, by full SHA, before it goes to Mustafa, nor give
  the check `git diff --quiet ddd38ba HEAD -- spikes/netcode-proof ':(exclude)spikes/netcode-proof/results'` at that
  head (exit 0 at 67de11e today).
- Note: For Mustafa item 4 (PR #54 "after its review round") is behind the card: the round is done (card :164-173) and
  NS-TRAILER / NS-FIRST wait for him there. Outside this lane; not counted.

## Verdict
FAIL: five wrong (one time, stale card refs, time-zone labels, CB-P6's edit count, "stopped at its test step"), one
missing (P6). About 340 claims compared.
