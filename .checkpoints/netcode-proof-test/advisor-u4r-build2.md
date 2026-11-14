# Advisor: U4r build, copy 2's points (ruling 2)

Started 2026-11-13 (after 21:51 UTC). Advisor agent file read from origin/main (335 lines).
Code read only in /home/user/Shamble-Frontlines/.gate/u4r-snap1 (2fbec77 + wip/u4r.patch).
Hashes checked: u4r-brief.md 59d570b8a73f... (matches); wip/u4r.patch 48fa2959799d... (matches).

## Plan
1. Read: u4r-build.md whole (569 lines; copy 2's part :284-569), my ruling 1 (advisor-u4r-build1-full.md and
   advisor-u4r-build1.md), my brief approval (advisor-u4r-brief.md: L1-L6), the brief's cited lines (7.2 :1193-1200,
   :1306-1311, :1358-1362, params :361-364), Mustafa's decisions in history.md as they bear on U4r.
2. Point 1 DEPTH: read input.rs (the measure, :28-29), the room's take, precheck.rs's depth histogram; decide L vs L - 1.
3. Point 2 LOGS: read precheck.rs result lines, the brief's records list; decide whether a per-leg log is required.
4. Point 3: (i) Cargo features of npt-server / npt-client and tests/loopback.rs; (ii) r3's test temp folder;
   (iii) log header commit naming vs uncommitted code; ruling 1's point 4 main-session step.
5. Point 4: conformance read of copy 2's Builder lines (notes :421-540) against the fixed copy's code.
6. Point 5: anything else in copy 2's part.
7. Write each point's block here as done; then return StructuredOutput.

## Blocks

### Read (done)
- u4r-build.md whole (569 lines); advisor-u4r-build1-full.md whole (169 lines) and advisor-u4r-build1.md; brief
  :330-420, :1135-1375; plan u4-build-plan.md :760-800; marks.json -30 (:6535-6580); SERIES.md :101, :240;
  room/src/input.rs :1-160; lab/src/precheck.rs whole (777 lines) in the snapshot.

### Point 1 DEPTH (done)
- Code: input.rs:28-29 doc "How many ticks ahead the newest buffered command was at the last take"; `take`
  (:120-150) removes the tick's command, then `depth = buffered.last().tick - tick` (:142-145). Pacing: precheck.rs
  :174-179 sends cmds with tick <= ran + lead at step 5 after tick `ran`; at take j the newest received was sent at
  instant j-1, stamped j-1+L, so depth L-1 by definition, both legs (Loopback: next instant is the next due tick;
  L1 P0: arrival next ns, before the take). Copy 2's figure {0: 48, 1: 5,568} per leg (5,616 = 48 x 117).
- precheck.rs:369-373 reads depth for every client in first_run after each tick, first takes included (the 48 zeros
  are the first takes, indistinguishable from a later 0 in a histogram). :568-569 prints "lead L
  (buffer_depth_ticks_count L, both legs)". r6's doc (:694-696) says "the room's measured depth the lead" but the test
  asserts no depth (:715-720 prints only): doc false and unasserted.
- marks.json -30 report "input buffer depth set for P0 (ticks)", series header.params.buffer_depth_ticks_count, stat
  max; SERIES.md:240 "-30's input buffer depth set for P0". The param is the SET value; under option B what is set is
  the pacing lead L. SERIES.md:101 `second.s.buffer_depth_count` is the room's measured series (-20's).
- Ruling: (a) with an exact comparison: header/param value L; the room's measure reported beside it; the expected
  relation is measured = L - 1 at each take after a client's first command run; report every take where measured is
  not L - 1, by (client, tick, measured), lossless (first-take depth per client on its client line, histogram over
  later takes, the deviations listed); r6 asserts it. (b) rejected: L - 1 is an observation, not what was set, and it
  would make -30's "depth set" read 1 at P0 while the replay sends 2 ahead. Words changed: the brief's 7.2 reading
  ("report any tick ... where the two differ") is the main session's brief text, approved by me; I amend it. Not
  Mustafa's. Severity Medium (as built the report's comparison would flag every tick, or say nothing; r6's doc false).

### Restart (2026-11-14 about 00:17 UTC)
- Resumed from this file after the last block marked done (Point 1). wip/u4r.patch now hashes 585a8639... (mtime
  2026-11-14 00:14:31): replaced after this review began (a later snapshot, not mine to judge here). The fixed copy
  .gate/u4r-snap1 is unchanged since 21:52:58 (its lab/src files' mtimes), HEAD 2fbec77: I read code there only.
  u4r-build.md is now 620 lines (copy 3 appends from :571); copy 2's part :284-569 unchanged.
- Point 1 refined at the restart: the deviations are listed capped (first 64 per leg, with the total), a deviation is
  reported and is never a VOID cause on its own; the 48 first takes are split out of the histogram (each on its
  client line). Reason for the cap: a broken pacing could give 432,000 entries in a committed runs.txt.

### Point 2 LOGS (done)
- Read: plan u4-build-plan.md :772-796 ("both legs' input-buffer depth (`buffer_depth_ticks`) recorded"; "the headers
  list no mark, so the judge never reads these runs; results/u4r-precheck-30/<commit>/runs.txt and the comparer's
  output are committed"); my U4 condition 2 (advisor-u4.md:90-102): "If the Loopback leg writes an npt-log header,
  README §1 gains the layer value `loopback`" (a header was optional by my own words); marks/README.md:36 (merged:
  layer loopback "an npt-lab test whose headers list no mark"); brief 7.2 :1197-1202, :1221, :1253 ("Log it per leg and
  per command"), :1308; 7.4 :1358-1365 (runs.txt's list; "The recordings' SHA-256s ... in runs.txt"); plan.md:630-631
  via brief :361-363 (a header holds the bite id, the features and the binary's SHA-256).
- Code: precheck.rs `main` takes no --commit and prints no build, no recording SHA-256 (it checks it, :531-550, prints
  only the name), no ring span; `run_leg` sums `rewinds_refused` per client at the end (:402-409), no per-command list;
  the comparer's hashes are `SimCore::state_hash` (:358-362), the same value logs.rs:466 writes as `h`.
- Ruling: no per-leg npt-log log at U4r; the result lines (runs.txt) are the record, as the plan and condition 2 say.
  The brief's "header" words in 7.2 read "the pair's result lines". So the lines must hold what a header would:
  `--commit` and a build line (machine::build, container), the recording line (name, bytes, SHA-256, generator,
  first_tick, span_ms, ticks), each leg's room configuration (layer, profile, cmd_timing, lead, tick base, seed, the
  ring), rewinds refused per command (first 64 per leg with the total), the depth per point 1. Severity Medium (the
  record misses header-held identity fields 7.4 and plan.md:630-631 ask for). Words changed: the brief's (main
  session's text, my approval); not Mustafa's.

### Point 3 (done)
- (i) server/Cargo.toml: `bites = ["npt-sim/bites", "npt-proto/bites", "npt-room/bites"]`, npt-client a dev-dependency;
  tests/loopback.rs:292-348 uses `npt_client::BITES` under cfg(feature = "bites"). Both files unchanged since 2fbec77
  (git diff --stat empty): pre-existing, U4e's files, outside U4r's npt-server lane (U4R-Q6 (a): pump.rs, schedule.rs,
  tests/schedule.rs). Low; card follow-up; report line.
- (ii) scenario.rs:1492-1501 `scratch` roots every lab test folder in std::env::temp_dir()/npt-lab-tests, removes only an
  old folder of its own pid, never after. /tmp is on /dev/vda (df: 5.2G free, the .gate disk); 93 folders, 126 MB now.
  Plan :762 says r3's bytes go "under .gate/". Mustafa's LEFTOVER-DELETE (history.md:404-406) and its applied line
  (:406-410: scratch in forms the approved methods can remove: a cargo target dir or a worktree). Ruling: root scratch in
  the test binary's cargo target dir (current_exe's folder: .gate/shared-target/...), removed when the test passes;
  leftovers in /tmp stay (no one-off delete). Low.
- (iii) whole-as-lost.log: `--commit 2fbec77...` on an uncommitted tree; header.build.commit names 2fbec77, so the
  logs' provenance is not the commit they name; the judge reads no commit (judge/src: only marks' drafts_commit). My
  ruling 1 point 4 main-session step would have judged them as the record: changed: the record is a bitten run re-made
  from the commit that holds U4r's code. Medium (the record would have cited a commit whose code differs).

### Point 4 conformance (done)
- Checked in the snapshot: point 9 (a) scenario.rs:1356-1363 + p9 test :2136-2160; (b) summary_line :1422-1470;
  point 3 :787-789, :872-874, test :2183-2204; point 4 logs.rs:302-347, LAB_BITES cfg(bites) :196-197, test :2209-2221;
  point 2 counters mirror.rs:254-307 and class_audit scenario.rs:1125-1129, pin test :2039-2134; point 5 p5
  :1981-2037, hash.rs check :365-395 + h5 :1386-1436; point 1 det-recordings.json (two entries as notes :225-230),
  h5 :1457-1470; wallclock site pump.rs:228-246.
- Conform, with two Low gaps: (a) p2's `None` branch prints "unpinned" and passes, so a later change can unpin the copy
  silently now that a placement exists; (b) the late-command bite test asserts `starts_with("result: VOID (L1
  late_count 1)")`, which a VOID by two causes also passes (printed line today is exact: c2-b10c-3.log). No departure
  from a Builder line beyond these.

### Point 5 (done)
- carry 83 (a): lab/src/lib.rs unchanged (mtime 20:49:48, not in git status); its struct doc still "the recording (or
  run) it replays"; dropped from copy 2's next list. Low.
- Bites::of `any` (scenario.rs:229-236) omits `b.pump`; the wallclock id passes only through b.schedule (npt_server::
  BITES holds it). Low.
- wip/u4r.patch replaced at 00:14 (585a8639): data for the main session.
- Copy 3's contract.rs design (beyond 2.7's list): outside this ruling (copy 3's part).
- Nothing for Mustafa: no word or decision of his changes; risk tier 1 (spike code, checkpoint text).
