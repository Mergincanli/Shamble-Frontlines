# Advisor: U4r review round 2 ruling (PR #40, netcode proof test)

**Slip (said first, as the rules ask):** one shell call ran `git fetch -q origin` (outside my read-only list). It only
updated the remote-tracking refs (origin/main still 4460ef5, as the card says); no working-tree, index or branch change.

## Plan
1. Read advisor.md from main (done), the round-2 results (u4r-review-r2-full.md), my round-1 ruling
   (advisor-u4r-r1-full.md), the fix-pass report (u4r-build.md, "## Fix pass, round 1: report"), the brief's merge and
   report sections, history.md's last entries (Mustafa 2026-11-14).
2. Per seat (netcode, security, test, rust, infra): check each round-2 finding against the snapshot code on
   .gate/u4r-snap2 (and the patch u4r.patch); confirm, correct or reject; final severity; ruling. A block per seat here.
3. Copy 6's two deviations (FP-2 plant (i) first run; end check 1, c6-end1.log): read the logs, rule.
4. Next steps in order for the main session (merge steps and checks, judged sets' order, disk); anything for Mustafa.

## Inputs read (13:3x UTC)
- advisor.md (main); u4r-review-r2-full.md whole (13 findings: INF-R2-1 Medium; N2-1, TE-R2-1, SEC-U4R-R2-1/-2,
  TE-R2-2/-3, RR4-L1/-L2, INF-R2-2/-3/-4/-5 Low); advisor-u4r-r1-full.md whole; u4r-build.md fix-pass report
  (:1955-2111) and FP-2/end-check blocks (:1656-1675, :1835-1870); u4r-copy6-prompt.md :34 ("If any outcome differs
  from what the ruling expects, stop and report: never re-run a failed suite into a pass (condition 5)"); brief
  sections 4 (end), 5, 6, 7 head, 12; history.md last entries (2026-11-14 11:07 SIZE-U4R (a), SIZE-CHECK, COMPACT,
  MAIN-52); card NOW block. U4r's merge conditions are NOT yet written (advisor-u4r-brief.md:441: "my merge conditions
  follow U4d2's M1 to M9 form") -> this ruling writes them (U4d2 r1 Block 9 / r2 Block 5 form read).

## State checked (read only)
- wip/u4r.patch sha256 21ca46be... (holds; committed at 450af1a); u4r-review-r2-full.md 1ea6fc44...; .gate/u4r-snap2
  HEAD 2fbec77 with the 26 files (13 M, 13 ??). `git diff --quiet 2fbec77 HEAD -- spikes/` exits 0 at a849eac; commits
  since 2fbec77 touch only .checkpoints/. Kit files equal origin/main (diff empty). Free 4,908,843,008 B (4,681 MiB).
  .gate/shared-target 8.2G, .gate/netcode-proof-test/target 6.8G, .gate/u4r 887M.
- End checks: c6-end2.log and c6-end3.log "check: all 11 steps pass", 531/609, self 77 of 77, wasm 43.1/48.7 s;
  c6-end1.log crashed after deps: "The value of "timeout" is out of range ... 296513.41192700004". det-u4: 235.0 s,
  non-cargo 220.3 + 2.5 + 0.9 = 223.7 s (round 1 187.8 + 2.3 + 0.8 = 190.9 s).
- Plants: fp1 (:2102, both cases), fp2i first (:1966 Kick count 2 vs 1), fp2i-b (:1993 ack_refused 5 / 15), ceil (76 of
  77, not ok 72): read in the logs.

## Netcode seat (N2-1)
- N2-1 = TE-R2-1, confirmed by reading: drop_slot (scenario.rs:408-422) pushes (slot,"replaced") only for a live old
  session; FP-1 already set it None (:496), so the 'no replacement' assert (:2209-2212) cannot fail; closed_checks
  (:2150-2164) reads room.client, owner, ended, never roster. Room::roster() room.rs:568, Roster::is_live roster.rs:80,
  Room::leave frees both (room.rs:690-695). The slot-freed claim is proven (plant fails by name); the rejoin claim is
  not. The assert's design was my ruling's (FP-1 TEST). Low. Follow-up: next npt-lab lane (add the roster check, drop or
  reword the no-op assert). Builder disclosed it (u4r-build.md:2089-2090).

## Security seat (SEC-U4R-R2-1, -2)
- SEC-U4R-R2-1 confirmed: rustAttributes needs `#\s*!?\s*\[` in the mask (check.mjs:339); `# $a` and `$h [..]` never
  read; pathProblems reads only attributes (:403-411). Probe log: both forms build, module text from .txt. Reach: the two
  deny crates (EXCEPTION_LEVELS :200); forbid elsewhere (E0453). No macro_rules in hash-wasm or tcpinfo; npt-hash-wasm's
  only dependency npt-sim has none either. My own U4d2 r2 ruling (advisor-u4d2-r2.md:115-118) said a macro_rules ban in
  the deny crates alone does not close the class (a dependency's exported macro carries the same tokens). So the
  follow-up's form must also cover npt-sim (the workspace dependency) or read the compiler's file list. Low; open
  security finding -> PR #40 needs-mustafa (it is anyway); in U4r's and PR #40's reports.
- SEC-U4R-R2-2 confirmed (check.mjs:1502-1507): `["profile"]` gives line 1; header regex reads the mask (string
  blanked), `quoted` wants `.`/`=` after the string, not `]`; fallback offset 0. Refusal still fires. Low; message only.

## Test seat (TE-R2-1, -2, -3)
- TE-R2-1: duplicate of N2-1 (merged).
- TE-R2-2 confirmed: fp3 test (:2420-2432) covers only --measured-s; the sum (:1376-1378), after-max (:1379-1381) and
  last-burst (:1443-1454) refusals have no case. Examples check: 1e18 x 30 > u64::MAX; 9.3e18 + 9.3e18 > 1.8447e19. Low.
- TE-R2-3 confirmed: T-3's sh child dies on SIGTERM, so spawnSync returns under the default signal too; killSignal
  SIGKILL (check.mjs:1910) is unproven. With `trap '' TERM` the default would wait the 30 s and the ms < 10,000 assert
  fails. Low.

## Rust seat (RR4-L1, -L2)
- RR4-L1 confirmed: ticks <= bytes/12 with bytes <= 64 MiB lets 5,592,405 ticks (hash.rs:319-331); encode reserves
  ticks x 1,900 (replay.rs:296), synthetic reserves ticks (:412); runner entry runs (w+m) x 30 ticks. Committed-manifest
  edit only; dev/CI tool. Low.
- RR4-L2 confirmed: due_ns `as u64` truncation (scenario.rs:131) above tick 553,402,322,211; burst_starts' plain
  `boundary + below(..)` (:1306-1307); release npt-lab has no overflow-checks (Cargo.toml:116-120 only sim, proto);
  bursts unbounded with every_s 0; :862 `as u32`. Operator input. Low.

## Infra seat (INF-R2-1 to -5)
- INF-R2-1 confirmed: c-u4r-runs.mjs sh() runs in ROOT (:202-206); tracked() = whole-repo `git status --porcelain
  --untracked-files=no`; unchanged() compares whole HEAD and that status (:316-321, used :349 and :366); c-u4r-r2.mjs the
  same (:209-211). The saver commits .checkpoints/ only when something there changed (checkpoint.mjs:157), but agents
  and the main session write there. A P0 set is about 691 + 1,151.7 s of runs (u4r-build.md:1464-1475) plus builds and
  the judge: more than 30 minutes, so "no commit for the whole set" would break CLAUDE.md's 30-minute checkpoint rule ->
  the interim rule works only for sets under about 25 minutes (r2, the pre-check, P2, P3, B1); the fix must land before
  the P0 set. Medium stays (fails closed; a workaround exists). Budget: below High -> follow-up, not a pre-merge fix.
- INF-R2-2 confirmed (c6-det-u4.log:25 vs det-u4.log:24). Low: report words + uptime first at the merge step.
- INF-R2-3 confirmed: mkdirSync(logs) :298, sha check :301, first floor :325/:359; r2 mkdir :180, floor :197. Low.
- INF-R2-4 confirmed: sh() has no timeout (:203); header says the lock may be held (:6-9). Low.
- INF-R2-5 confirmed: run() returns no signal (check.mjs:1921); step() writes "exit by a signal" (r2 :147). Low.

## Deviations (copy 6)
- FP-2 plant (i) first run: failed at the Kick count (2 vs 1) because the test drained c1 after joined()'s tick, not
  "right after the second join" as ruled; corrected to join_only + drain before the tick (scenario.rs:2045-2068), plant
  re-run fails at the ruled assert. No failed check re-run into a pass; the clean result is the same; test seat confirms
  r8 not weakened. Accepted.
- End check 1: wasm step crashed on a fractional spawnSync timeout from copy 6's own FP-6 item 11 edit (clock to
  performance.now). Not one of the no-retry kinds (mismatch, trap, SHA-256, stopped call): the call never started.
  Root-caused, fixed (Math.ceil, check.mjs:2357-2359, >= 1 ms while time is left), pinned by a self-test with its plant
  (76 of 77 without it), failed run kept whole, then two fresh whole runs. Infra seat: the right completion of its own
  INF-U4R-10 (3). Accepted. The prompt's "stop and report" line was not followed in either case; it stays as written
  for later copies.

## Rulings (summary)
- No Critical or High: no round 3. INF-R2-1 Medium -> follow-up U4r-F1 (own small change-set after the merge, before
  any c-u4r-runs.mjs set), with INF-R2-3 and INF-R2-4 riding it (same two files, no check.mjs change). Interim rule for
  r2's record run only. INF-R2-2: done at the merge (uptime first; report words). The check.mjs follow-up (check.mjs's
  next change, before PR #40 is marked ready at the latest): SEC-U4R-R2-1, SEC-U4R-R2-2, TE-R2-3, INF-R2-5 (with round
  1's INF-U4R-10 (2)). The next npt-lab lane: N2-1 (= TE-R2-1), TE-R2-2, RR4-L2, RR4-L1 (at the latest before U4h's
  carry-87 runner entry).
- Deviations: both accepted; stop line unchanged for later copies. The 73-of-76 re-pin of tests 21, 22, 24 was inside
  the brief's named expected failures (a carry 85 change's re-pin, recorded under TE2-5): not a deviation.
- Merge conditions M1-M9 written (U4d2's form). After the merge, in order: r2 record run (interim rule) -> carry 62 (h)
  marks.json commit and the two bite checks -> F1 -> D4 -> disk plan -> pre-check set -> judged sets P3, B1, P2, P0.
- /usr/bin/time is not installed (ls): ruling 1 point 3 (4)'s judge memory read needs another method (python3
  resource.getrusage(RUSAGE_CHILDREN).ru_maxrss: Linux keeps the reaped tree's maximum). RAM 15 GiB, 4 CPUs.
- Disk: free 4,681 MiB; kept logs still to come about 162 (r2) + 16 + 360 + 545 + 629 + 3,020 MiB = 4,732 MiB, plus
  the 4 GiB floor and builds: about 4.1 GiB short. Approved part-cleans (shared target 8.2G, main cache 6.8G) between
  lanes; else LEFTOVER-DELETE route to Mustafa.
- For Mustafa: nothing to decide, no merge of his, no Brave check; heads-ups only (disk; one Low security finding kept
  open as a follow-up, PR #40 needs his go anyway).

## Progress
- [x] inputs read
- [x] seat blocks (all five)
- [x] deviations
- [x] next steps (merge conditions, post-merge order, disk)
- [ ] StructuredOutput returned
- [ ] netcode block
- [ ] security block
- [ ] test block
- [ ] rust block
- [ ] infra block
- [ ] deviations
- [ ] next steps
