# Advisor: review of the pushed head after U4r's merge into the spike branch (PR #40), 2026-11-14

Task: review merge commit 71ee13d and the branch head after it, against advisor-u4r-r2-full.md next steps 2-10 (M1-M9).

## Plan
1. Branch head: confirm the commits after 71ee13d touch only .checkpoints/.
2. Merge commit file list: spikes/ part equals the reviewed patch (450af1a's wip/u4r.patch, sha256 21ca46be...),
   file for file; no marks.json change; other paths .checkpoints/ only; wip/u4r.patch removed.
3. Logs (.gate/u4r-merge/: m5.log, gate.log, gate-no-smoke.log) against M5-M7's expectations: uptime first, six
   builds with main-folder provenance, check 11 steps, 531/609, 0 failed 0 skips, self 77/77, marks 81, redact 0,
   deps and wasm lines as c6-end2.log:173-174, det-u4 PASS with non-cargo sum <= 240 s, SS-P0-s4 sha256 6933b56a...
   equal to the manifest's, TE2-5's state.
4. u4r-build.md "## Merge into the spike branch": M1-M8 records against the logs.
5. u4r-report.md: each statement against its sources; step 9's list all present.
6. CI on 71ee13d (gate run 43603457356 gate+smoke, toolchain 43603457370), if GitHub is reachable.
7. Verdict, next steps, Mustafa.

## Checks (a line per check as I go)
- C1 head: origin branch = local HEAD = f0a1e60 (now two .checkpoints/-only commits after 71ee13d: 94dc9e3 and f0a1e60,
  both only .checkpoints/netcode-proof-test.md, mode 100644). Main still 4460ef5.
- C2 patch bytes: git show 450af1a:...wip/u4r.patch and 71ee13d^:... both sha256 21ca46be...bce. 26 diff chunks.
- C3 spikes/ = patch: per-file split of `git diff --no-renames 71ee13d^ 71ee13d -- spikes/` vs the patch: 26 files each,
  none only in one, 26 of 26 chunks byte-identical (only the file order differs); every patch index abbreviation is a
  prefix of the full blob ids. 13 A + 13 M, all 100644, no 120000/160000. No marks.json path.
- C4 other paths: .checkpoints/netcode-proof-test.md (M), u4r-build.md (M), u4r-report.md (M; drafted at 5e0eae1),
  wip/u4r.patch (D). Nothing else.
- C5 base: spikes/ at 71ee13d^ equals 2fbec77's (git diff --quiet exit 0); commits 2fbec77..71ee13d^ touch only
  .checkpoints/ (I used git diff-tree for this list: read-only, but outside my named commands; noted).
- C6 m5.log: lock exclusive; uptime first (load 0.27, 0.09, 0.10); HEAD 0eded3c + the applied patch (uncommitted, as
  M3-M5 run before M8); six builds, all provenance lines under /home/user/Shamble-Frontlines/spikes/netcode-proof/crates/;
  check 11/11; test 531 / 609, 0 failed, 0 skips; self 77/77; marks 81, ADR rows unchanged; redact 0 findings.
- C7 deps line (m5:176) = c6-end2.log:173 with the time stripped (exact); det-u4's deps line = check's deps line.
- C8 wasm line (m5:177) = c6-end2.log:174 except times and the native hash-map digests (a05095fd/070080d7 vs
  dd69689d/f2a899af). Native digests differ in every kept log (7 distinct pairs found), Wasm digest f5e8d0e8 stable:
  by design (the bite is the hash-map order). Catch points as expected (SS-s1 tick 1234; std-sin SS-s1 tick 2
  vehicle[8].pos.y). 43.5 s < 300 s.
- C9 det-u4: PASS, 18000 / 0 both pairs; non-cargo sum = npt-lab hash 223.8 + Wasm 2.5 + 0.9 + 0.0s = 227.2 s <= 240
  (round-2 base 223.7). Step total 240.3 s includes builds 11.6 + 1.4.
- C10 regen: sha256 6933b56a... printed "as the manifest's"; I hashed .gate/u4r-merge/regen/rec/SS-P0-s4.rec myself:
  6933b56a...bab3 = precheck-recordings.json at 71ee13d; 16950444 bytes; native-1.lines cmp-equal native-2.lines
  (9000 lines). Command = regen-precheck.log:18's but the folders (.gate/u4r-merge/regen vs .gate/u4r/regen). 130.1 s.
- C11 gate.log: full gate FAIL only at 5a (Chromium executable missing); gate-no-smoke.log PASS, smoke skipped.
- RESTART (after the spend limit's cut): head now 1284dad; commits after 71ee13d: 94dc9e3, f0a1e60, 9830132, e5bd633,
  1284dad, each touching only .checkpoints/ (netcode-proof-test.md or this notes file). Continuing at C12.
  Re-checked C2/C3 after restart: patch sha256 21ca46be...bce; 26 = 26 files, 26 bodies equal (index lines aside).
- C12 u4r-build.md "## Merge into the spike branch" (71ee13d, lines 2113-2137) against the logs: M3's card commit
  0eded3c (13:43:16 UTC) = m5.log's HEAD; M4: I compared each of the 26 files at 71ee13d with .gate/u4r-snap2 (26 of 26
  equal) and with the working tree (26 equal); M5's times 13:44:35-14:06:39, exit 0, free 4679, up 3:09, load 0.27 0.09
  0.10, 531/609/0/0, self 77, marks 81, redact 0, wasm 43.5 s, det-u4 18000/0 x2, non-cargo 227.2 (223.8+2.5+0.9),
  regen 16950444 bytes 6933b56a...bab3 "as the manifest's", legs equal 9000: all match m5.log:1-232. M6: no failure
  in the test step (0 failed both builds), so TE2-5 ends (TE-Q2): matches. M7: gate.log 5a FAIL (headless_shell-1194
  missing; /opt/pw-browsers holds chromium_headless_shell-1243: confirmed by ls), all else PASS; gate-no-smoke.log GATE
  PASS. M8: the commit's file list matches. M1's fingerprint not re-run (no script runs in my seat); M4's cmp done by
  me instead, which covers the same bytes.
- C13 report (u4r-report.md at 71ee13d, 220 lines) against its sources. Held: 8,362 / 6,906 / 1,456 / 106 (my numstat
  on 71ee13d^..71ee13d -- spikes/); round 1 six seats (rust REQUEST CHANGES, five APPROVE WITH FINDINGS), 50 = 10 M
  + 40 L (u4r-review-r1-full.md headings); round 2 five seats, 13 = 1 M + 12 L; every advisor-u4r-r2-full.md line ref
  (:7 :12-15 :17-24 :28 :32 :38-41 :45-48 :52-62 :66-104 :108-118 :122-125) points where it says; history.md :2044,
  :2048-2051, :2067-2074, :2082-2086 (his SIZE-U4R and SIZE-CHECK words, the main session's reading labelled as its
  own); u4r-build.md :1448-1486 (r5 rates, 5,240.9 s), :1867 (fingerprint), :1916-1923 (wasm figures), :1927-1929
  (124 folders), :2012 (531/609/77), :2075-2077; code refs scenario.rs:323, :855, precheck.rs:285, :322,
  c-u4r-runs.mjs:152, pump.rs:279, schedule.rs:252, check.mjs:2357: each as stated; m5.log figures as stated; deps
  line words ("no attribute holds a metavariable", npt-sim's features one set) in m5.log:176. Step 9's list: all
  present (copies 1-6; four build rulings; both rounds' counts; two deviations; M1-M7; carries by number; public items
  with EA-U4R-1's line and contract/precheck; timing with labels, INF-R2-2's base, the merge figure; SIZE-CHECK
  8,362 vs about 3,000 under SIZE-U4R (a); follow-ups: round 1's FP-9 item 4 list (12 entries) and this ruling's 13
  findings all homed; SEC-U4R-R2-1 and -2; judged runs pending in steps 11-17's order; Decisions needed; For Mustafa).
  Gaps (all Low): (a) report:95 "under 240 s, at load average 0.27": that uptime is the call's first command at
  13:44:36, det-u4 started 14:00:17 after check.mjs's 13.7 min, so the load at det-u4's start is not that, and the
  base it is compared with (1.32) is at det-u4's own start; report:88 calls the provenance counts "the six builds'"
  while they count the whole call (seven builds, the regeneration's npt-lab build included). (b) report:125-127 leaves
  carries 20 and 69 "not verified here, for the advisor's review". I checked both: 20 met (crates/lab/src names
  grid::Config only at mirror.rs:23/:152, a local Config::default() used by value; replay's re-decoder takes
  npt-client's own grid_with_bite, replay.rs:8-10; nothing hashes, records or compares a Config); 69 met
  (Recording::synthetic only in hash.rs's regeneration, hash.rs:384; no ES byte or bandwidth figure in crates/lab/src
  or the run scripts, whose "bytes" are disk sizes, c-u4r-r2.mjs:55-78). (c) round 1's FP-9 item 3 (EA-U4R-2 named as
  a deviation from U4-SEAM, u4-build-plan.md:162-163, with its sites) is only in u4r-build.md:1930-1934; the report
  lists the follow-up (:173) but not the deviation. Sites re-checked at 71ee13d: logs.rs:97, :472, :595,
  precheck.rs:296; Room::new at scenario.rs:751 and precheck.rs:301.
  Nits, not findings: the commit message says "1 Medium and 12 Low, all follow-ups" (INF-R2-2 was done at the merge;
  history is not rewritten for it); the card's "Last update" line (11:25) and its "Next: M9" line are stale.
- C14 CI on 71ee13d (GitHub MCP, read only): gate run 43603457356: job gate success, job smoke success (step "Smoke
  (check 5a)" success); toolchain run 43603457370: job toolchain success; all head_sha 71ee13d29dc6..., run_attempt 1.
  The later commits are .checkpoints/-only [skip ci]. No .github/ or gate file in the diff, so CI's green stands.
- C15 M9: card at 1284dad says merged at 71ee13d, 531/609/77, TE2-5 ends, follow-ups, SEC-U4R-R2-1 open; saver
  restarted (b02nlg0ph); copy 6's worktree .claude/worktrees/agent-a11e2354169f39acf kept; .gate/u4r logs kept.
- Verdict: APPROVE (three Lows, none triggers a push; they ride the report's next edit, after r2.txt is written).
