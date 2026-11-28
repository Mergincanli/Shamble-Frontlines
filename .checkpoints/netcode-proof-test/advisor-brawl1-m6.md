# Advisor: BRAWL-1 merge, M6 (review of pushed head 8f0939f; merge commit b30314a)

Started 2026-11-15. Agent file: origin/main:.claude/agents/advisor.md (read).

## Plan
1. Read the merge step of the round-1 ruling (advisor-brawl1-r1-full.md) and the round-2 amendments A1-A6
   (advisor-brawl1-r2-full.md).
2. Check the commit range: b30314a's parent, b30314a's file list, every later commit .checkpoints/-only.
3. M1: spikes/ at b30314a^ equals 352a245's; the patch file at 888ecbb hash c2561b6c...; b30314a's spikes/ diff
   equals the patch (hash of `git diff b30314a^ b30314a --binary -- spikes/`); the 10 files equal the builder
   worktree; the patch removed in b30314a.
4. M2: m2-clean-B/C logs: run lines vs the 2026-11-14 logs; df figures.
5. M3: m3.log header (HEAD, diff/untracked hashes) and every A3 figure it should show.
6. M4: the two gate logs; whether --no-smoke is admissible under the rulings (Chromium missing).
7. M5: push time, CI runs on b30314a (GitHub tools if available; gh CLI otherwise).
8. Since: the card and u4r-report.md's BRAWL-1 section vs my round-2 words.
9. Rule: verdict, findings, M7, for Mustafa.

## Checks (a line each as I go)
- C1 range: HEAD = origin branch = 8f0939fb. b30314a's parent f7dff4bf. Later commits 5b00f44, 8043264, 8f0939f touch only
  .checkpoints/ (git diff --name-only per commit, nothing else). b30314a: the 10 spikes files M + the patch file D.
  b30314a's message has no [skip ci]. spikes/netcode-proof tree ebbfeb40 at b30314a and at 8f0939f. PASS.
- C2 M1 base: spikes tree 63de1632 at b30314a^ and at 352a245 (netcode-proof abad0635 both). PASS.
- C3 M1 bytes: patch at 888ecbb and at b30314a^ both sha256 c2561b6c...; `git diff b30314a^ b30314a [--binary] -- spikes/`
  sha256 c2561b6c (identical to the reviewed patch). 10 files: sha256 equal at b30314a, in the builder worktree
  (HEAD 352a245, 10 M, git diff HEAD c2561b6c) and in the main folder. numstat 1139/127 = 1,266. PASS.
- C4 M2: m2-clean-B.log 15 run lines, m2-clean-C.log 6 run lines, each identical (diff of `run: ...`) to
  .gate/u4r-sets/clean-shared-{B,C}.log; every command exit 0; guards (test -f lock + both CACHEDIR.TAG, test ! -e each
  cleaned folder, ls -A) present. Headers HEAD b30314a, diff/untracked e3b0. df figures: 6,911,565,824 B = 6,591.4 MiB
  (B header 6591), 7,367,524,352 = 7,026.2 (C header 7026), 13,579,448,320 = 12,950.4 (M3 header 12950); B freed 434.8
  MiB, C freed 5,924.2 (A4 said debug 5,925). The df figures themselves live only on the card (no log). PASS.
  Slip: the card says "M2 DONE (08:38-08:40 UTC)"; the logs show 08:37:41-08:38:09 and M3 began 08:38:44. Low.
- C5 M3 (m3.log): lock exclusive 08:38:44; HEAD b30314a; diff e3b0 / untracked e3b0 (A2 PASS). cat /proc/uptime 227.41
  s (booted ~08:35, the restart). check.mjs all 11 pass; 538 / 616, 0 failed, 0 skips; self 77/77 (A3 PASS). df after
  check.mjs 8,342,093,824; uptime 08:53:06 (load 2.31) then det-u4 PASS 234.1 s, non-cargo sum 219.4+2.5+0.9+0.0 = 222.8
  < 240 (condition 8 PASS); df after det-u4 8,342,081,536 (A4 PASS, > 4 GiB). Regeneration line exact, 1f00d2d5, into a
  new folder .gate/brawl1-merge/m3 (created 08:57:11). Full test exit 0: its 9 lines diffed by me against
  final-fp-regen-tests.log's 13 (4 short-copy lines + 9 full): only the 4 short lines differ, the 9 identical. Every
  command exit 0.
- C6 pins hashed by me: .gate/netcode-proof-test/det-u4/recordings/SS-P2-s1.rec 1ec25de7..., ES-P3-s2.rec 6abf7643...
  (mtimes 08:55-08:56, M3's det-u4); .gate/brawl1-merge/m3/rec/SS-P0-s4.rec 1f00d2d5.... Manifests a2faf3ea / 5ccf57f0
  at b30314a (and they hold the three pins). regenerate_all (hash.rs:461-494) checks every entry before keep. PASS.
- C7 M3 end free space: the round-1 M3 said "Free space is recorded at its start and end"; m3.log's footer has no free
  figure and the card gives none. Now (09:06:48, nothing running): 8,320,163,840 B free; shared target ~5.0 GiB
  (debug 4,611, release 250, wasm32 122, doc 15 MiB). Low (record gap; the figure is recoverable and far above floor).
- C8 M5 CI (GitHub tools): gate run 43697261383 on head_sha b30314a: success, jobs gate (132760462244) and smoke
  (132760462037, step "Smoke (check 5a)" success); toolchain 43697261438 on b30314a: success. Also on 8f0939f: gate
  43697365073 success, toolchain 43697365142 success. b30314a pushed alone: runs created 09:02:08 for b30314a only,
  then 09:03:22 for 8f0939f. PASS.
- C9 M4: m4-gate.log: checks 1-4, 5b-5d PASS, 5a FAIL (headless_shell-1194 missing; /opt/pw-browsers holds -1243
  only), GATE FAIL; m4-gate-no-smoke.log GATE PASS. My own run through main-run.sh
  (.gate/brawl1-merge/advisor-m6/gate.log, HEAD 83fb6ac, diff/untracked e3b0): --no-smoke exit 0 GATE PASS (symbols
  +0 -0); full gate exit 2, 5a only, same Chromium reason. CI's smoke job passed 5a on b30314a and 8f0939f. PASS.
- C10 A3 counts equal the copy's end: final-fp-check.log 538 / 616, 0/0, self 77, marks 81, redact 287/0; M3 the same.
  det-u4 hash time 219.4 s vs the copy's 210.0 and 211.1 (load 2.31 vs 1.26/1.42): in family; margin to 240 is 17.2 s.
- C11 A6: main-run.sh f0b7a3d0, u4r-run.sh 2f754d12 (sha256sum now). c-u4r-runs.mjs 99dd070e, unchanged since 493b3d7.
- C12 since: b30314a..8f0939f touches the card, brawl1_report_words.py and u4r-report.md only. Ruling file sha256
  64e361ab (now and at 8f0939f). u4r-report.md: +43 lines, 0 removed; its 36 quoted lines equal the ruling's :131-166
  (diff). PASS. Note: my round-2 words said "U4r's report (where its round 1 sits)"; round 1's review record sits in
  brawl1-build.md (:251 "Report", :620 "Fix pass, round 1: report"), and u4r-report.md holds only round 1's U4g words
  (:217-222). The new section starts at round 2 and cites ADV-BP-1..4 that it does not hold. Low (my wording; rides
  the merge-record write).
- C13 card at 8f0939f: M1-M5 lines checked against the logs above: all figures hold except M2's time range (C4). The
  restart record agrees with the logs (no clean before 08:37:41; 6,360 MiB target = 434.8 + 5,924.2 freed).
- C14 tree: after my run `git status --porcelain` shows only this notes file; spikes/ legacy/ tools/ clean.
- C15 PR #40: draft, open (head now 83fb6ac: saver commits of .checkpoints only). Out of scope: base main 4460ef5 not
  merged in (known, card :6).

## Ruling
- Verdict: APPROVE WITH FINDINGS on 8f0939fb (merge b30314aa). Three Lows, none triggers a push.
  - ADV-M6-L1: M2's record: card "08:38-08:40" vs logs 08:37:41-08:38:09 (M3 began 08:38:44); the pgrep result and the
    three df readings are on the card only. Fix: correct the range in the merge record; keep M7's df in a log.
  - ADV-M6-L2: M3's end free space not recorded (round-1 M3: "Free space is recorded at its start and end"). Fix: record
    my 09:06:48 reading (8,320,163,840 B; target ~5.0 GiB) and M7's first df as the post-M3 figures, labelled so.
  - ADV-M6-L3: u4r-report.md's BRAWL-1 section starts at round 2; round 1's record is in brawl1-build.md. Fix: when the
    merge record is added, add one line pointing to round 1's record (brawl1-build.md :251, :620; advisor-brawl1-r1-full.md).
- M7: may start as ruled, with M7-1..M7-4 (in the result).
- For Mustafa: nothing.
