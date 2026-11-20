# U4g's brief, draft rev 1 (for the advisor's check; <base> filled at launch)
**The unit:** prediction, interpolation and events, u4-build-plan.md:881-895 (plan rev 3.3). It builds after U4f2 merges
and -21's runs are done (the lanes run one at a time: u4-build-plan.md:945-946). Short by TEST-FIRST rule 3
(history.md:2264-2267; his whole message :2236-2282): it says what U4g builds, how it is tested and what is open, and
points to the rest. U4f's brief stays the reference for the builder's start and stops (u4f-brief.md section 1,
:288-355), the build rules (section 4, :831-958), the pre-runs, judge calls, disk and bars (5.3-5.6, :997-1162), the end
checks (section 6, :1163-1183) and the return list (section 7, :1187-1222); U4f2's brief (u4f2-brief.md) is the shape.
**Two questions are his before the build:** U4G-Q1 (the size is over 3,000) and U4G-Q2 (-58's two checks are
marks.json text). Section 8 and the last section.

## 0. Where things are
- **`<base>`:** the spike branch's head at launch, after U4f2's merge and -21's set, filled by the main session in U4f's
  fill form (u4f-brief.md section 12, :1724) with its spike tree and its main-folder check.mjs counts (measured, bites),
  on which every count here is a delta. Every "there now" below was read at 391adda (2026-11-20 04:31 UTC; spike tree
  2b7647ff…: U4f-F1's merge 1faf894 plus results/) and is re-read at `<base>`: U4f2 opens crates/bots, lab, room and
  client (u4f2-brief.md:162-165), so line numbers there move.
- **Read, never retyped** (sha256's first 16 hex and lines at 391adda; refilled at `<base>`): u4-build-plan.md
  (d3e3ddbbc04e0411, 1,163); plan.md (efba7e090a521b5a, 1,525); u4f-brief.md (135325a0bb524104, 2,040); u4f2-brief.md
  (a6d6e38ae23fe357, 257); u4s-carries.md (a0e5aee16e470c92, 710: U4g's section :554-585); advisor-u4f-stop.md
  (38821ebd2c870ca9, 247); u4f-report.md (5aeef4009e47e81b, 708); advisor-u4f-f1-r1.md (6683efe565b3cd16, 266);
  advisor-u4f-build1.md (da98295e0d0eae7c); u4g-brief-carry.md (c3abeccb004882f1, 14); advisor-brawl1-sets-p3-full.md
  (1bc5889c9b05443f, 212); u4r-report.md (c5dee3fa561dddbe, 1,764); history.md (1c763b7722efb76c, 7,253);
  spikes/netcode-proof/marks/marks.json (c1935a5b996fb0aa, 12,630); marks/SERIES.md (7f11afbee31b1bd7, 823);
  docs/adr/0001-multiplayer-netcode.md (f2d3ac4df5d6bd48, 2,036).

## 1. What U4g builds
### 1.1 The lane
- **Open:** crates/client (the new client code); crates/sim (carry 47's reset, -55's bite site by U4G-Q5, TE-F1-5 only
  if cheap); crates/room (the ClientSettings clamp, carries 78 and 79, -62/server-echo, -58's three command counters,
  -60's site if U4G-Q5 puts it there); crates/lab; crates/bots; ops/runs (new files); marks/SERIES.md (new rows only).
- **Shut:** crates/proto (ClientSettings is coded: proto/src/msg.rs:124-126, :471), judge, log, server, l1, link, net,
  client-wasm, hash-wasm; marks.json, marks/README.md and every existing SERIES.md row; check.mjs; Cargo.toml and
  Cargo.lock (no new dependency is expected). A file outside the lane stops the build for the main session.

### 1.2 What is there and what is new (read at 391adda)
| Item | There now | New in U4g |
|---|---|---|
| Prediction in the sim | `World::predict` runs one command through `step`'s phases and refreshes its soldier's two records (sim/src/world.rs:1047-1068) | the predicting World's reset (carry 47) |
| Own-state attributes | the owner-only record holds move speed and fire interval (world.rs:1857-1873; seam.rs:165-172); the room scripts attribute changes (room/src/room.rs:794-806) | the client predicts with them after a reset; -60's suite scripts 100 changes of each kind through `Room::script` |
| The client | `ClientEndpoint` with its rebuilds, the last-run sequence among them (client/src/lib.rs:222-290, :455), and ClockSync (client/src/clock.rs; :7 leaves the interpolation clock to U4g) | `Predictor`, `Interp`, `CueLog`, event release, the ClientSettings send; their per-tick outputs beside `StepOut` (lib.rs:203-219) |
| ClientSettings | coded in npt-proto; no code in crates/room reads it (grep) | the room's clamp (plan.md:515's npt-room column) and the per-client delay U4h's view-time estimate reads (u4-build-plan.md:382-383) |
| Snapshot packing | own group, then events, then entity changes (room.rs:1437-1519) | the nearest 8 enemies' changes before the events (carry 78) |
| The cosmetic count | `cosmetic_sent` counts every cosmetic event (room.rs:119, :1591) | per client, those sent for its own shots (carry 79) |
| -41's bites | both planted and unit-shown in npt-sim (world.rs:609-616; sim/src/lib.rs:47; u4-build-plan.md:672-673) | run judged here |
| -58's setup | `--tick-start`, `--event-start`, `--cmd-start`, `ticks_below_wrap`, `counter_wrap` (lab/src/scenario.rs:1918-1921, :2063-2065; lab/src/logs.rs:339) | the `wrap` suite, a press-counter start (section 4, F1's line), the writers of `tick.v.cmd_refused_count`, `.cmd_lost_count` and `.cmd_doubled_count` (SERIES.md:25-27 names npt-room) |
| Series | every series these marks read has its row (SERIES.md:23-57, :83, :122, :150, :178-183; `collision_rule` :456) | their writers: no crate writes them at 391adda (grep); new rows only for names beyond them |
| Bite ids | npt-client's six (lib.rs:34-41) | -45's and -62/cues-from-replay in npt-client (carry 24, u4s-carries.md:154-158), -61/on-arrival there too; -62/server-echo in npt-room; -60's and -55's by U4G-Q5 |

### 1.3 Blocks (each compiling with its tests passing before the next; about 400 added lines at most each, as U4f's build ruling's line 4, advisor-u4f-build1.md:174-177)
1. **npt-sim:** carry 47's reset and its test; NC-F1-4 named beside it; -55/vehicle-snap's site (U4G-Q5); TE-F1-5's two
   2-edge twins only if open and cheap (advisor-u4f-f1-r1.md:147-150), else left by number.
2. **npt-client, `Predictor`** (u4-build-plan.md:882-884; plan.md:467, E8): reset at the header's last-run sequence
   (-58/lt-tick-offset's U4g half: §4.14 :501), newer commands replayed through `World::predict`, the bitwise skip except
   in suite `cues`, the 100 ms fade, the 2 m snap, cleared on respawn or teleport; corrections above 1 mm counted, each
   traced or not to a late or lost command (U4G-Q6); carry 70's teleport corrections counted apart; the per-snapshot
   5 cm flag; the `own_change` events. Float code stays outside the sim's bans; the skip compare is bitwise (§8 risk 9,
   :1011-1013).
3. **npt-client, `Interp`** (§4.8 :369-372; plan.md:468, E9): its own monotonic, rate-slewed clock, integer ns at the
   API, 7 slots at 30 Hz, the delay from 3 intervals adapting between 2 and 5 by one interval a second at most (§4.3
   :262), extrapolation at most one interval then hold, placed respawns; the per-tick `block_extrapolated` and
   `block_extrap_over_interval`; a ClientSettings sent when the delay changes; -45/buffer-one-short's site (§4.14 :497).
4. **npt-room:** the ClientSettings clamp (bounds as plan.md:468 and :515 give them); carry 78; carry 79 with
   -62/server-echo; the three -58 command counters; -60's site if U4G-Q5 puts it here.
5. **npt-client, `CueLog` and event release** (u4-build-plan.md:885-886): cues keyed by (entity, weapon, command
   sequence); remote spatial events released into the first block whose delayed time reaches their tick, own,
   predicted and owner events into the block of their tick; the misplaced counts and `own_cue.plays_count`; the sites
   of -62/cues-from-replay (§4.14 :504) and -61/on-arrival.
6. **npt-lab and npt-bots:** the suites `own-change`, `extrapolation`, `events`, `cues` (the replay skip off, a replay
   on every snapshot: §4.14 :504), `beside-vehicle` and `wrap`; CO's course script (one soldier among the greybox's
   doors: sim/src/scene.rs:218-222; ADR 0001:1480-1481); the press start; the client- and room-log writers;
   `params.collision_rule` (-42's notes); each suite's contract case (§4.12 :443-446); runs.txt's line gains U4g's
   counts, per option where a mark pools both.
7. **ops/runs:** c-u4g-r2.mjs (short pre-runs in 5.3's form, u4f-brief.md:997-1052) and c-u4g-runs.mjs (one set per
   call, two judge calls, in c-u4f-runs.mjs's form: ops/runs/c-u4f-runs.mjs:44-52); SERIES.md's new rows.
8. **The end checks** (section 3).

## 2. Marks moved and the runs (the main session's, after the merge, in the main folder)
| Mark (marks.json line) | L1 parts (`min`) | Suite, scene, length | Measured runs | Bites (site; run) | What it reads |
|---|---|---|---|---|---|
| -41 (:7719) | CO P0-P4 (3) | CO (1 client), 18,000 ticks | 30: 5 parts x 2 options x 3 seeds; the parts pool both options, so each run must pass; runs.txt per option (:887-888) | server-speed-1pct, server-quant-differs (npt-sim, built: §4.14 :495-496); CO, seed 1, one run each | `tick.v.untraced_corrections_count` sum 0, count >= 18,000; report `tick.v.corrections_count` |
| -42 (:7863) | SS brawl P2, P3, P4 x A and B (3) | brawl, SS 48, 10 min, -20's settings less its press script (c-u4f-runs.mjs:52) | 18 | none at U4: smoothing-off is R7's, carried by name (§6 :969) | P3: `second.s.snap_corr_over_5cm` share < 0.01, PASS unbitten; P2 and P4 report only, never claimed (:888-890; §0 :52-54) |
| -60 (:8071) | P2 (3) | own-change, scene by U4G-Q3, 100 changes per kind (±10 %: the notes) | 3 (6 by U4G-Q4 (b)) | own-state-missing (U4G-Q5); P2, seed 1 | `event.own_change.*`: count >= 200, max <= 1, misplaced 0, later over 1 mm 0 |
| -45 (:8598) | P2, P3, B1 (3) | extrapolation, SS by U4G-Q3, 10 min | 9 | buffer-one-short (Interp's slot count): P2, its size from the clean P2 runs' largest logged delay + 2 - 1, so after them (§4.14 :497) | `tick.v.block_extrapolated` share < 0.01 (P2), < 0.03 (P3); `block_extrap_over_interval` sum 0 (B1) |
| -61 (:8936) | P2 (3) | events, SS's fight, 10 min (the notes) | 3 | on-arrival (U4G-Q5); P2, seed 1 | `tick.v.remote_event_misplaced_count`, `.own_event_misplaced_count` sum 0 |
| -62 (:10467) | P2, forced replays (3) | cues, scene by U4G-Q3, 1,000 scripted shots | 3 | cues-from-replay (CueLog), server-echo (room): §4.14 :504-505; `cues`, seed 1 each | `event.own_cue.plays_count` count >= 2,000, min >= 1, max <= 1; `second.v.own_cosmetic_sent_count` sum 0 |
| -55 (:10694) | P2 beside a moving big vehicle (3) | beside-vehicle, scene by U4G-Q3, 10 min | 3 | vehicle-snap (U4G-Q5); P2, seed 1 | `second.s.snap_corr_over_5cm` share < 0.02; readings R-11, R-12 |
| -58 (:4148) | P0, P3 with `reorder` (3) | wrap, 48 clients, the tick 30 below a multiple of 65,536 and every counter a few steps below its wrap, then 10 s | 6 | six (§4.14 :490, :499-503): tick16 (proto), lt-cmd-seq, lt-press, lt-ack (room), lt-tick-offset, lt-event-seq (client); P0, seed 1 each | ten checks: decoded hash, commands, presses, untraced corrections, setup, wraps >= 6, 300 ticks; judged after U4g merges (:892-894) |
| -56 item (6) (:10745) | none at U4 | from -42's 18 runs | 0 | none (the row names none) | -42's share per option at P2-P4, in runs.txt and the report for the record suite; no `record_item` at U4 |

- **Counts:** 75 measured (78 by U4G-Q4 (b)) and 14 bite runs, 89 in all: §9's count (:1025-1029) names the same 75.
- **Judge calls:** two per set (measured, bites; u4f-brief.md 5.4, :1053-1079). None of these marks carries
  `pre:PM-0001-2` (marks.json: no validity on -41, -42, -45, -55, -60, -61, -62; `reorder` on -58's P3 only), so
  PM-0001-2 is in no judge call; its checks run as the load report, never counted, on each SS set, as on -22's
  (u4f-report.md:691-694).
- **-58's reorder report row** reads "not available" on L1: ADV-U4F20-2 (Medium), his to change, on the after-close list
  (u4f-report.md:644-647); P3's `reorder` validity holds through the lab's `reorder` event. No change in U4g.
- **Disk and time** (estimates; the runner projects at launch; sets one per call, D2 before each, packing by KEPT-LOGS,
  history.md:6523-6546; u4f-brief.md 5.5, :1080-1126):

| Set | Runs | Wall a run (source) | Logs a run (source) | The set, about |
|---|---|---|---|---|
| -42 | 18 | 320-390 s (SS P2 and P3: results/u4r/*/runs.txt; -20: results/u4f/96de6acf8223/pm-0001-20/runs.txt) | 62-210 MiB (-20 P3: 371.8 MiB over 6; U4r P2: 628.2 MiB over 3) | 1.6-2.0 h; 1.1-3.7 GiB |
| -45 | 10 | the same (B1 322-324 s: U4r) | the same | 0.9-1.1 h; 0.6-2.1 GiB |
| -61, -55 (if on SS) | 4 each | the same | the same | 0.4 h each; up to 0.8 GiB each |
| -41, -60, -62 (if on CO) | 32, 4 (7), 5 | no CO run yet (1 client) | no figure | under 0.5 h and 0.3 GiB together (unmeasured) |
| -58 | 12 | about 12 s of sim each, well under a minute | about 3 MiB (-17's 35 s tick16 pre-run: 2,937,440 B, ops/runs/c-u4r-r2.mjs:88) | under 0.2 h; 0.1 GiB |

  About 3.7 to 4.6 h of run wall in all (36 SS runs, the rest small). U4g's client series add to every log: the
  first pre-run measures the size.
  **The judge's peak:** -42's 18 runs at 210 MiB give 3,780 MiB of logs, 302.2 + 2.3403 x 3,780 = 9,149 MiB against the
  10,240 MiB line (the fit as runs.txt prints it); if the projection passes the line, -42's measured call splits by
  option, a runner choice with no judge change.

## 3. How it is tested
| What | The test | Its plant, failing the test by name |
|---|---|---|
| carry 47 | reset from the authority's records at T, then N predicted commands equal the authority bit for bit (u4s-carries.md:555-560) | the reset leaves the press counters: a backwards step read as up to 255 edges (NC-F1-4) |
| -41, the trace | a CO replay at P2 with one late and one lost command: those corrections traced, none untraced; at P0 none at all; the skip and the replay give equal held state | the trace taking every correction as traced; the two built -41 bites (§4.14 :495-496) |
| -58/lt-tick-offset (U4g's half) | across the wrap the replay starts at the command after the rebuilt last-run | the plain `<` (§4.14 :501): the replay starts at the wrong command |
| fade, snap, clear | a correction under 2 m spread over 100 ms; one at 2 m or more snapped; respawn and teleport clear the Predictor | the fade off; the clear skipped |
| carry 70 | on ES's synthetic recording, teleport corrections counted apart, both reported (u4s-carries.md:561-565) | teleports counted with the others |
| -42, -55, the flag | `snap_corr_over_5cm` flags exactly the snapshots whose correction passes 5 cm, on a hand-made trace | the threshold moved |
| -60 | a short own-change case: each change's one correction at the first snapshot carrying it, none after | own-state-missing |
| -45 | Interp on a fixed P2 arrival trace: extrapolation only where the buffer runs dry, at most one interval, then hold | buffer-one-short (§4.14 :497's unit proof) |
| the clamp | the room clamps an out-of-bounds delay and keeps the last; the client changes it by one interval a second at most | the clamp removed |
| -61 | remote events in the first block whose delayed time reaches their tick; own events in their tick's block | on-arrival (the newest block) |
| -62, carry 79 | a replayed shot plays once; the room counts per client, and sends the shooter, 0 own cosmetic events | cues-from-replay; server-echo (§4.14 :504-505) |
| carry 78 | a backlog larger than one snapshot's remainder still sends the nearest 8's changes in every snapshot (u4s-carries.md:566-573) | today's order (events first) |
| -58 | the wrap suite's short run: at least 6 `counter_wrap` events, the three command counters 0, the press start given to client and room | each of its six bites (§4.14's unit proofs) |
| the contract | every new suite's checks resolve and every name has its row (§4.12 :443-446) | a renamed series |
- **Plants:** every new check and bite site fails by name under a plant, restored with the file's sha256 (u4f-brief.md
  section 7 item 4, :1202-1204). Counts as deltas on `<base>`'s; the negative cases counted; the report states its own.
- **End checks:** u4f-brief.md section 6, items 1 to 5 (:1165-1182) as they apply: check.mjs twice with all 11 steps,
  both clippy targets (npt-client and npt-room are linted for wasm32 too), fmt, each run-script command line by hand.
  Item 4 (npt-server's tests) applies only if U4g touches them, which its lane does not.
- **After the merge (the main session's):** c-u4g-r2.mjs whole: a short run of each new suite with its contract case,
  and the cap reads of U4G-Q7; a row that reads other than its `expect` comes to the advisor before the sets. Then the
  sets, one per call; -58's last.

## 4. The carries, by number (met by number in U4g's report)
- **U4g's section of u4s-carries.md (:554-585):** 47 (:555-560), 70 (:561-565), 78 (:566-573), 79 (:574-578); from
  U4f-F1's round (:579-585; advisor-u4f-f1-r1.md): TE-F1-5 by id (:147-150), NC-F1-4 beside 47 (:156-158), -58's press
  wrap "by pressing through the wrap, or by a start the lab gives both the client and the slot's references (carry 75's
  form for event numbers), never by an unchecked first command" (:123-125; u8 counters: sim/src/seam.rs:33; lean the
  lab start: from 0 a wrap needs 256 edges, about 8.5 s at one a tick, past -58's first 2 s), and C-U6-19's line
  (:120-122), stated here:
  **U4g's runs hold no hostile client, so none throws or rejoins as one, and no client is restarted mid-run.**
- **Carry 24's U4g half** (u4s-carries.md:154-158): -45's ids and -62/cues-from-replay in npt-client's `BITES`; no id
  in two crates' lists.
- **C62HB-Q4 (b): -58's two new lost-tick checks are U4g's.** His answer, as recommended (RECS-1043, history.md:2204-2227:
  the question at :2216); the advisor confirmed it with ADV-CBR-1 (:2477-2478; the text :6425-6429). The two checks are
  -17's figure checks `snap_lost_unexplained_count` and `snap_whole_despite_link_loss_count` (marks.json:3983-4004);
  both figures read measured ticks only (SERIES.md:291-292), while -58's wraps fall in its first seconds and its checks
  read the whole run. Adding them is marks.json text: U4G-Q2, his. u4f-brief.md:128 (C5), :1302 and :1333 hand both
  to U4g.
- **The event cap's margin, 1.43** (u4f-report.md:631-636: -20's 10-minute high-water 1,117 against 1,600), named for
  U4g by the stop ruling R2 (advisor-u4f-stop.md:132-134: "U4g, whose B1 judged runs are where the cap bites (P32)").
  1,600 stands unless R2's own route moves it (an honest overflow resync: :129-131). U4G-Q7.
- **EVQ-LOAD's U4g line** (advisor-brawl1-sets-p3-full.md:156-158; u4r-report.md:1156-1161, :1592-1593): "U4g's plan
  reads carry 58's per-burst high-water marks from a short B1 run before -18's judged B1 runs count"; carry 58
  (u4s-carries.md:269-273) reports any overflow resync in a judged -17 or -18 run. U4G-Q7.
- **u4g-brief-carry.md** (14 lines, word for word from advisor-brawl1-r1-full.md:279-288; history.md:3080-3081): B1's
  bursts can carry a soldier into a door gap; "U4g's plan says how a B1 building entry reads in -17, or measures how
  often it happens, before its judged B1 runs". U4G-Q8.
- **C62H-BOUND's ten deferred Lows** (history.md:6421-6424: takers include U4g) ride only where U4g opens the file and
  it is cheap (the line-15 rule, advisor-u4f-build1.md:213-217), else stay with their takers; the report says which.

## 5. The rules that stand
- The builder's start and stops are u4f-brief.md section 1's (:288-355), read for U4g: the block-end size stop past
  6,000 added lines (his SIZE-CHECK, history.md:2048-2051; advisor-u4f-build1.md:178-181: build nothing more, run the
  end checks on the tree as it stands, write the count and the estimate for the rest, return), or past the figure
  U4G-Q1 sets; the counts are deltas on `<base>`'s; the time rule and the test step's 1,207 s line
  (advisor-u4f-build1.md:240-242) stand unless the advisor re-sets the line at launch from `<base>`'s test step.
- U4f's build rules (u4f-brief.md section 4, :831-958) with U4g's lane (1.1); the shared target and its lock; one
  builder in an isolated worktree, Opus 5.5 at xhigh (netcode); at most three agents at a time.
- **No pass mark, judge rule, marks.json text, existing SERIES.md row or run count changes** in this brief or the
  build; a build that needs one stops, and the main session asks him (TEST-FIRST rule 2, history.md:2262).
- **LEAD-BOUND (a), 400 ms, frozen** (history.md:6791-6806: "It is never changed to fit a run's result").
- Every agent brief carries, word for word: "Write files with the Write and Edit tools only (no sed, no shell
  redirect, no python writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit
  tool rewrites its notes file whole with Write."

## 6. Size (his SIZE-CHECK, history.md:2048-2051: M is about 3,000 added lines; past 6,000 the build stops)
| Item (added lines in spikes/) | Where | Estimate |
|---|---|---|
| Carry 47's reset and test; -55's site and proof; TE-F1-5 if cheap | sim | 150-320 |
| `Predictor` 300-450, `Interp` 250-400, `CueLog` and release 150-250, the settings send, four bite sites, the outputs | client src | 820-1,340 |
| Their unit tests (section 3's rows) | client tests | 500-800 |
| The clamp, carries 78 and 79, server-echo, -58's counters, -60's site, their tests | room | 390-730 |
| Six suites and CO's driver, the press start, the log writers, the header param, contract cases, lab tests | lab | 930-1,590 |
| CO's course, the cue shooter, the vehicle walker, the attribute script | bots | 150-300 |
| c-u4g-r2.mjs and c-u4g-runs.mjs | ops | 220-370 |
| New rows only | SERIES.md | 10-30 |
| **Total** | | **about 3,200 to 5,500** (midpoint about 4,300) |
- **Plainly: this is over about 3,000, so it goes to him before the build** (U4G-Q1). At U4f's 1.2 overrun
  (advisor-u4f-stop.md:104-105; U4f closed at 6,425 against its brief's 5,150: u4f-report.md:704-706,
  u4f-brief.md:1221) the top reaches about 6,600, past his 6,000 stop.

## 7. Reviewers and rounds
netcode-architect, test-engineer and rust-reviewer (u4-build-plan.md:895), each at xhigh, at most three at a time. At
most two review rounds, a third only for a Critical or High. A marks.json change (U4G-Q2, if he says yes) gets a
reviewer's read (TEST-FIRST, "What stays exactly as it is").

## 8. Questions (U4G-Q1 and U4G-Q2 are his; the rest the advisor's)
1. **U4G-Q1 (his), the size.** About 3,200 to 5,500 added lines against M's 3,000. (a) Build it whole; the stop past
   6,000 stays. (b) Cut it in two, as U4f was: U4g, prediction and cues (blocks 1 and 2, block 4 less the clamp and
   carry 78, block 5's `CueLog`, and blocks 6 and 7 for -41, -42, -55, -58, -60, -62 and -56 (6); about 2,100 to
   3,600), then U4g2, interpolation and events (`Interp`, the clamp, release, carry 78; -45 and -61; about 1,100 to
   1,900), each with its own stop.
   (c) Build it whole with the stop raised. **Lean (b):** each half stays near M, and (a)'s top passes 6,000 at U4f's
   overrun.
2. **U4G-Q2 (his), -58's two lost-tick checks** (C62HB-Q4 (b) with ADV-CBR-1). (a) Two check entries copied from
   -17's (marks.json:3983-4004) onto -58's L1 P0 and P3 parts, exact text sent for his yes, and -58's runs set so every
   wrap falls in the measured window (a run setting: the warm-up ends before the first wrap). (b) A whole-run reading of
   the two figures: a judge change, his. (c) Not now: -58 runs without them, the gap named. **Lean (a);** the build
   writes no marks.json text either way, and -58's judged runs wait for his answer.
3. **U4G-Q3, the scenes the parts leave open** (header only, no selector). -45 SS (remote soldiers); -60 CO; -62 CO
   (100 ms interval, 30 rounds, 2 s reload: sim/src/scene.rs:219-250; 1,000 shots in about 3 minutes); -55 SS with
   slot 0 scripted beside a looping very large unit, or a new scene (sim data). **Lean as listed, -55 on SS** (no new
   scene).
4. **U4G-Q4, -60 under one option or both.** plan.md:847 picks OQ-0001-6's option using -60 among others; §9 counts -60
   at 3. (a) 3 runs, option B. (b) 3 per option, +3 short runs. **Lean (b);** his if the advisor reads +3 as a run-count
   change.
5. **U4G-Q5, the bite sites §4.14 does not give.** -60/own-state-missing: npt-room's own group sending the two attribute
   fields unchanged while a change lasts (id in npt-room). -61/on-arrival: npt-client's release (id there). -55/
   vehicle-snap: npt-sim's soldier-against-hull push-out, its id in a list apart from `World::BITES`, which npt-hash-wasm's
   replayer indexes (world.rs:944-954). **Lean as listed.**
6. **U4G-Q6, what traces a correction.** (a) The client alone, from the header's two acknowledgements (the same code
   ML runs; marks.json -41's notes: "the client counts corrections ... and logs each command's arrival"). (b) The lab
   joining the room's late and lost records. **Lean (a);** (b) would pass on L1 what ML cannot see.
7. **U4G-Q7, the cap at 1,600 in U4g's runs.** EVQ-LOAD's line and the 1.43 margin; -42's P4 parts (RTT 200 ms, 5 %,
   full view) were never measured, and -20's P3 maximum is 1,117. (a) c-u4g-r2.mjs reads the per-client and per-burst
   high-water marks on short P4 brawl and B1 runs with the cap opened (c-u4f-cap.mjs's method), set against 1,600,
   and every judged SS run prints its overflow resyncs; an honest one goes to the advisor by R2's route before the
   set counts. (b) U4f's 60 s B1 figures
   (bursts 615-727) meet EVQ-LOAD as they are. **Lean (a):** U4g changes packing and release. Whether U4r's -18 B1
   verdict needs a re-read is outside U4g: the advisor's.
8. **U4G-Q8, u4g-brief-carry.md's door gap.** U4g's judged B1 part is -45's, with no `pre:PM-0001-2`, so an entry voids
   nothing there. (a) The "in a building" line npt-lab already prints (results/u4f/96de6acf8223/pm-0001-20/runs.txt) on
   the B1 pre-run and every B1 run, with TE-B1-5's per-burst door-gap margin added only if it reads above 0. (b) That
   margin printed always. **Lean (a);** the bots' leash keeps reading what it reads now, not the Predictor.

## What goes to Mustafa from this brief
- **U4G-Q1, the size** (stopped on him): about 3,200 to 5,500 added lines, over M's 3,000, and about 6,600 at U4f's
  overrun. Lean: cut it into U4g (prediction and cues) and U4g2 (interpolation and events).
- **U4G-Q2, -58's two lost-tick checks:** marks.json text for his yes, with the run set so the wraps are measured.
- U4G-Q4 only if the advisor reads its +3 runs as a run-count change. Nothing else: no money, nothing on main.
