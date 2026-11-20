# U4g's brief, rev 2.1 (rev 2 with the advisor's rev-2 conditions 1-4 applied; APPROVE WITH CONDITIONS, advisor-u4g-brief.md:350-358)
**The unit:** prediction, interpolation and events, u4-build-plan.md:881-900 (plan rev 3.3, with the "Moved in from U4r"
bullet :895-900). It builds after U4f2 merges and -21's runs are done (one lane at a time: :951-952). Short by
TEST-FIRST rule 3 (history.md:2264-2267; his message :2236-2282): what U4g builds, how it is tested, what is open; the
rest is pointed to. U4f's brief stays the reference for the start and stops (u4f-brief.md section 1, :288-355), the
build rules (section 4, :831-958), the pre-runs, judge calls, disk and bars (5.3-5.6, :997-1162), the end checks
(section 6, :1163-1183) and the return list (section 7, :1187-1222). **Two questions are his, neither a stop now:**
U4G-Q1 (split or not) goes in the next day's message, and the build starts with the prediction half either way;
U4G-Q2 (-58's two checks' wording) waits only for -58's judged set (section 8).

**Rev 1 to rev 2** (the advisor's check, REVISE: advisor-u4g-brief.md, 285 lines; its list :248-267):
- ADV-U4GB-1 (High), the B1 work moved in from U4r (the advisor's U4r build ruling 3, points 1 and 4): blocks 8 and 10,
  section 2 (-17's and -18's L1 B1 parts, -18's bite, the pre-runs), section 3's four new rows, section 4's first block.
- ADV-U4GB-2 (High) and -3: U4G-Q2 has no "not now", waits only for -58's judged set, with the drafting order and the
  wording pointed to (section 8).
- ADV-U4GB-4: section 6 recomputed with the new items and U4f's whole-scope 1.5; U4G-Q1 in :205-219's form; the blocks
  reordered, the prediction-and-cues half first; "stopped on him" and "before the build" gone.
- ADV-U4GB-5 (NC-U4R-4): block 5, section 3's -58 row, section 4. ADV-U4GB-6 and -7: by the U4G-Q9 and Q3 rulings.
- The rulings U4G-Q3 to Q9 and NC-U4R-4's route are folded where each pointed; section 8 keeps one "Ruled" line each.
- Lows: -8 in 1.2 and section 2; -9 in sections 3 and 5 and the blocks' heading; -10 in section 2's disk lines; -11 in
  section 5.
- The main session's item 7, re-read and cited: u4s-carries.md's carry 77 (b) amended (:384-386), carry 78 amended
  (:577-584), carries 91 (:585-588) and 92 (:589-592); carry 93 (:609-614) is U4h's, not U4g's; u4-build-plan.md's
  U4g bullet (:895-900), §9's -60 6 and totals 199 and 288 (:1031-1035), §11's heal-reading line (:1170-1173).

## 0. Where things are
- **`<base>`:** the spike branch's head at launch, after U4f2's merge and -21's set, filled by the main session in U4f's
  fill form (u4f-brief.md section 12, :1724) with its spike tree and its main-folder check.mjs counts, on which every
  count here is a delta. Every "there now" below was read at 391adda and 9d491fb (spike tree 2b7647ff…, U4f-F1's merge
  1faf894 plus results/) and is re-read at `<base>`: U4f2 opens crates/bots, lab, room and client
  (u4f2-brief.md:162-165), so lines there move.
- **Read, never retyped** (sha256's first 16 hex and lines at 9d491fb; refilled at `<base>`): advisor-u4g-brief.md
  (ea692318ec9519f7, 285); advisor-u4r-build3-full.md (78ebb5af7d48a035, 235); u4-build-plan.md (9186c15ba24fecaf,
  1,173); u4s-carries.md (ea1913c9256ba563, 735: U4g's section :557-604); plan.md (efba7e090a521b5a, 1,525);
  u4f-brief.md (135325a0bb524104, 2,040); u4f2-brief.md (a6d6e38ae23fe357, 257); advisor-u4f-stop.md
  (38821ebd2c870ca9, 247); u4f-report.md (5aeef4009e47e81b, 708); u4r-report.md (c5dee3fa561dddbe, 1,764);
  advisor-u4f-f1-r1.md (6683efe565b3cd16, 266); advisor-u4f-build1.md (da98295e0d0eae7c); u4g-brief-carry.md
  (c3abeccb004882f1, 14); advisor-brawl1-sets-p3-full.md (1bc5889c9b05443f, 212); c62h-bound-brief.md
  (7936074c15c5eb49, 663); history.md (1c763b7722efb76c, 7,253); marks/marks.json (c1935a5b996fb0aa, 12,630);
  marks/SERIES.md (7f11afbee31b1bd7, 823); docs/adr/0001-multiplayer-netcode.md (f2d3ac4df5d6bd48, 2,036).

## 1. What U4g builds
### 1.1 The lane
- **Open:** crates/client; crates/sim (carry 47's reset, -55's site, TE-F1-5 only if cheap); crates/room (carries 78,
  79 and 91, the recovery after an eviction, the ClientSettings clamp, -62/server-echo, -58's three command counters,
  -60's site); crates/lab; crates/bots; ops/runs (new files); marks/SERIES.md (new rows only).
- **Shut:** crates/proto (ClientSettings is coded: proto/src/msg.rs:124-126, :471), judge, log, server (its tests run as
  an end check, never edited), l1, link, net, client-wasm, hash-wasm; marks.json, marks/README.md, every existing
  SERIES.md row; ADR 0001 and the catalogue (a word the recovery's form changes goes on the readings-PR list, his go
  before R5); check.mjs; Cargo.toml and Cargo.lock. A file outside the lane stops the build for the main session.

### 1.2 What is there and what is new (read at 9d491fb)
| Item | There now | New in U4g |
|---|---|---|
| Prediction in the sim | `World::predict` (sim/src/world.rs:1047-1068) | the predicting World's reset (carry 47) |
| Own-state attributes | the owner-only record's move speed and fire interval (world.rs:1857-1873; seam.rs:165-172); `Room::script` (room/src/room.rs:794-806) | the client predicts with them; -60's suite scripts the changes |
| The client | `ClientEndpoint` with its rebuilds (client/src/lib.rs:222-290, :455) and ClockSync (clock.rs; :7 leaves the interpolation clock to U4g) | `Predictor`, `CueLog`, `Interp`, event release, the ClientSettings send, beside `StepOut` (lib.rs:203-219) |
| ClientSettings | coded in npt-proto; no room code reads it | the room's clamp (plan.md:515) and the per-client delay U4h reads (u4-build-plan.md:382-383) |
| Snapshot packing and recovery | own group, then events, then entities (room.rs:1437-1519); after an eviction the set is rebuilt from the defaults (room.rs:1446-1454) | carry 78's reorder and the recovery after an eviction (carry 78 amended) |
| The stale copy | taken only when the ring slot is overwritten (room/src/ack.rs:109-114), not when a tick becomes last_acked (:156-158) | carry 91 |
| The cosmetic count | every cosmetic event (room.rs:119, :1591) | per client, its own shots' (carry 79) |
| -41's and -58's bites | -41's two in npt-sim (world.rs:609-616; sim/src/lib.rs:47); -58's six built: proto quant.rs:39 (tick16), room lib.rs:41-43 (lt-ack, lt-cmd-seq, lt-press), client lib.rs:36-37 (lt-tick-offset, lt-event-seq); -18/no-fallback in the room (room.rs:1449-1450) | run judged here; only lt-tick-offset's replay half is new code |
| -58's setup | `--tick-start`, `--event-start`, `--cmd-start`, `ticks_below_wrap`, `counter_wrap` (lab/src/scenario.rs:1918-1921, :2063-2065; lab/src/logs.rs:339); five wrap names reported | the `wrap` suite, the press start, the sixth name `tick` (carry 92), the three command counters' writers |
| Series | every series these marks read has its row (SERIES.md:23-57, :83, :122, :150, :178-183, :456); none written | their writers; new rows only for new names (Q9's observer param) |

### 1.3 Blocks (each compiling with its tests passing before the next, at most about 400 added lines: a block estimated past that is split, the lab by suite; advisor-u4f-build1.md:174-177; ADV-U4GB-9)
**The prediction-and-cues half** (U4g under either answer to U4G-Q1):
1. **npt-sim:** carry 47's reset and its test, NC-F1-4 beside it; -55/vehicle-snap's site (Q5); TE-F1-5's two twins
   only if open and cheap (advisor-u4f-f1-r1.md:147-150).
2. **npt-client, `Predictor`** (u4-build-plan.md:882-884; plan.md:467): reset at the header's last-run sequence
   (lt-tick-offset's replay half: §4.14 :501), newer commands replayed through `World::predict`, the bitwise skip but
   in suite `cues`, the 100 ms fade, the 2 m snap, cleared on respawn or teleport; corrections above 1 mm traced by the
   client alone (Q6); carry 70's teleports apart; the 5 cm flag; the `own_change` events.
3. **npt-room, its half:** carry 79 with -62/server-echo; -58's three command counters; -60's site (Q5).
4. **npt-client, `CueLog`**, keyed by (entity, weapon, command sequence); -62/cues-from-replay's site (§4.14 :504).
5. **npt-lab and npt-bots, its half:** the suites `own-change` and `cues` on CO (the -60 soldier walks and fires
   throughout: Q3), `wrap` (the press start in carry 75's form; the lab's `tick` record when the room's tick crosses a
   multiple of 65,536: carry 92), `beside-vehicle` (SS, one client: Q3), -42's brawl settings; CO's course; the log
   writers and `params.collision_rule`; Q6's L1 cross-check print; -55's bite through the router (scenario.rs:240-261)
   as `CHECK_BITES` go; the contract cases (§4.12 :443-446).
6. **ops/runs, its half:** c-u4g-r2.mjs's rows (each suite short; -58's route-A rows: Q2; the P4 cap read: Q7) and
   c-u4g-runs.mjs's sets for -41, -42, -55, -58, -60 and -62 (c-u4f-runs.mjs's form, ops/runs/c-u4f-runs.mjs:44-52).
   **If his U4G-Q1 answer is not in when block 6 is done** (ADV-U4GB-14): the builder runs the end checks on the tree as
   it stands and returns (the size stop's form); the main session goes on by his answer.

**The interpolation, events and B1 half** (U4g2 under U4G-Q1 (b)):
7. **npt-client, `Interp`** (§4.8 :369-372; plan.md:468): its own clock, integer ns, 7 slots, the delay 3 intervals
   adapting 2 to 5 by one a second at most (§4.3 :262), extrapolation at most one interval then hold, placed respawns;
   the flags by Q9; the ClientSettings send; -45/buffer-one-short's site (§4.14 :497).
8. **npt-room, its half:** the ClientSettings clamp (plan.md:468, :515); carry 78's reorder and **the recovery after an
   eviction** (carry 78 amended, u4s-carries.md:577-584; its form U4g's, for example the relevant soldiers the baseline
   lacks first, with the reorder: advisor-u4r-build3-full.md:13); **ack.rs's fix** (carry 91: the stale copy taken when
   a tick becomes last_acked).
9. **npt-client, event release** by delayed time; -61/on-arrival's site (Q5).
10. **npt-lab and ops/runs, its half:** the suites `extrapolation` and `events`; the B1 rows (10 bursts in 600 s, as
    c-u4r-runs.mjs:25-28); the door-gap margin and "in a building" on every B1 run (Q8); the observer param (Q9);
    c-u4g-r2.mjs's short B1 suite, short bitten B1 run and B1 cap read; c-u4g-runs.mjs's sets for -45 (with -17's and
    -18's B1 parts), -18's bite and -61.
11. **The end checks** (section 3), after each half if split.

## 2. Marks moved and the runs (the main session's, after the merge, in the main folder)
| Mark (marks.json line) | L1 parts (`min`) | Suite, scene, length | Measured | Bites (site; run) | What it reads |
|---|---|---|---|---|---|
| -41 (:7719) | CO P0-P4 (3) | CO (1 client), 18,000 ticks | 30: 5 parts x 2 options x 3 seeds, each run must pass, runs.txt per option (:887-888) | server-speed-1pct, server-quant-differs (npt-sim, built); CO P0, seed 1, one run each | `tick.v.untraced_corrections_count` sum 0, count >= 18,000 |
| -42 (:7863) | SS brawl P2-P4 x A, B (3) | brawl, SS 48, 10 min, -20's settings less its press script (c-u4f-runs.mjs:52) | 18 | none at U4 (smoothing-off is R7's: §6 :975) | P3 share < 0.01, PASS unbitten; P2, P4 report only (:888-890) |
| -60 (:8071) | P2 (3) | own-change, CO, walking and firing, 100 changes per kind | 6: 3 per option (Q4) | own-state-missing (room); P2, seed 1 | `event.own_change.*` |
| -45 (:8598) | P2, P3, B1 (3) | extrapolation, SS 48 on the brawl's load, one option in the header; B1 with 10 bursts in 600 s | 9 | buffer-one-short: P2, sized from the clean P2 runs' largest delay + 2 - 1 (§4.14 :497) | slot 0's `block_extrapolated` share; any slot's `block_extrap_over_interval` (Q9) |
| -17, -18 L1 B1 (moved: ruling 3, point 1) | B1, `pre:PM-0001-2` (3 each) | served by -45's three B1 runs, which list -2, -17, -18 and -45 | 0 more | -18/no-fallback (room, built): B1, seed 1 | -17's checks, the two lost-tick checks among them (marks.json:3983-4004); -18's heal; PM-0001-2's share >= 0.99 |
| -61 (:8936) | P2 (3) | events, SS's fight, 48, 10 min | 3 | on-arrival (client); P2, seed 1 | the two misplaced counts |
| -62 (:10467) | P2 forced replays (3) | cues, CO, 1,000 scripted shots | 3 | cues-from-replay, server-echo; `cues`, seed 1 each | `own_cue.plays_count`; `own_cosmetic_sent_count` |
| -55 (:10694) | P2 beside a big vehicle (3) | beside-vehicle, SS with one client, contact ticks printed, 10 min | 3 | vehicle-snap (sim); P2, seed 1 | `snap_corr_over_5cm` share < 0.02 |
| -58 (:4148) | P0, P3 (3) | wrap, 48 clients, the tick 30 below 65,536, every counter a few steps below its wrap, 10 s | 6 | its six: P0, seed 1 each | ten checks, six wrap names; judged after U4g merges, and after his U4G-Q2 yes |
| -56 item (6) | none at U4 | from -42's runs | 0 | none | -42's share per option, P2-P4, in runs.txt and the report |

- **Counts:** 78 measured (-60's +3 by Q4) and 15 bites (-18/no-fallback added), 93 in all; the three B1 runs count
  once. The plan's §9 reads -60 6 and totals 199 and 288 (u4-build-plan.md:1031-1035).
- **The pre-runs** (c-u4g-r2.mjs, before the sets): each new suite short with its contract case; -58's route-A rows
  (Q2 step 2); short P4 brawl runs with the cap opened, before -42's set (Q7); on the tree with carry 78's reorder and
  the recovery, **the short B1 suite** (at most 18 flag-0 ticks per client per burst on average, ruling 3's target; "in
  a building: 0"; door-gap margins printed; the share printed beside it, 0.99 staying the judged runs' bar: ADV-U4GB-13)
  with the B1 cap read, and **the short bitten B1 run** under -18/no-fallback (no heal within 500 ms: carry 91; it also
  prints each client's flag-0 ticks per burst and the share, and if the bitten run would not hold pre:PM-0001-2 at the
  judged setting, it comes to the advisor with those figures before the B1 set: a setting for the bite run is the
  advisor's, a validity-rule change his: ADV-U4GB-12). A row other than
  its `expect` comes to the advisor before any set.
- **Judge calls:** two per set (u4f-brief.md 5.4, :1053-1079). The B1 set's calls name PM-0001-2, -17, -18 and -45:
  -17's and -18's B1 parts carry `pre:PM-0001-2`. On the other SS sets PM-0001-2 runs as the load report, never counted
  (u4f-report.md:691-694). -58's reorder row reads "not available" on L1 (ADV-U4F20-2, his, after-close list:
  u4f-report.md:644-647); no change in U4g.
- **Disk and time** (estimates; the runner projects at launch):

| Set | Runs | Wall a run | Logs a run (source) | The set, about |
|---|---|---|---|---|
| -42 | 18 | 320-390 s (results/u4r/*/runs.txt; results/u4f/96de6acf8223/pm-0001-20/runs.txt) | 62-210 MiB (-20 P3; U4r P2) | 1.6-2.0 h; 1.1-3.7 GiB |
| -45 with -17, -18 B1, and -18's bite | 11 | the same | 62-210 MiB; B1 about 274 MiB (821 MiB for 3: u4r-report.md:475) | 1.0-1.2 h; 1.5-2.5 GiB |
| -61 | 4 | the same | 62-210 MiB | 0.4 h; 0.25-0.8 GiB |
| -41, -60, -62, -55 (one client each) | 32, 7, 5, 4 | no one-client run yet | no figure | under 0.7 h; under 0.5 GiB (unmeasured) |
| -58 | 12 | well under a minute | up to about 25 MiB (-17's 35 s clean partner: 24,813,880 B, c-u4r-r2.mjs:89) | under 0.2 h; under 0.3 GiB |

  **In all** about 3.4 to 4.5 h of run wall and about 3 to 8 GiB of logs. D2 at launch against the 4,096 MiB floor; a
  set whose projection passes it waits for a pack of a closed set by c62h-bound-brief.md 8.1's procedure (:485) and
  KEPT-LOGS (history.md:6523-6546). **The judge's peak:** -42's 18 runs at 210 MiB give 302.2 + 2.3403 x 3,780 =
  9,149 MiB against the 10,240 MiB line; if the projection passes it, -42's call splits by option (no judge change).

## 3. How it is tested
| What | The test | Its plant, failing the test by name |
|---|---|---|
| carry 47 | reset from the authority's records at T, then N predicted commands equal the authority bit for bit | the reset leaves the press counters (NC-F1-4) |
| -41, the trace (Q6) | a CO replay at P2 with one late and one lost command: those traced, none untraced; the client's trace equals the room's records on L1; the skip and the replay give equal held state | every correction taken as traced; -41's two bites |
| lt-tick-offset (replay half) | across the wrap the replay starts after the rebuilt last-run | the plain `<` (§4.14 :501) |
| fade, snap, clear | a correction under 2 m spread over 100 ms; 2 m or more snapped; respawn and teleport clear | the fade off; the clear skipped |
| carry 70 | on ES's synthetic recording, teleport corrections apart, both reported | counted together |
| -42, -55 flag | `snap_corr_over_5cm` on a hand-made trace; -55's contact ticks above 0 | the threshold moved |
| -60 (Q3, Q5) | each change's one correction at its first snapshot; one fire-interval change makes its one correction | own-state-missing (pre-change values sent) |
| -62, carry 79 | a replayed shot plays once; 0 own cosmetic events sent or counted per client | cues-from-replay; server-echo |
| -58, NC-U4R-4 | the wrap suite's short run: 6 distinct wrap names, the three command counters 0, the press start on both sides | the `tick` record dropped; each of the six bites |
| -45 (Q9) | Interp on a fixed P2 trace: extrapolation only where the buffer runs dry, one interval at most, then hold; slot 0's flag and the any-slot flag | buffer-one-short |
| the clamp | an out-of-bounds delay clamped, the last kept; one interval a second at most | the clamp removed |
| -61 | remote events in the first block whose delayed time reaches them; own events in their tick's block | on-arrival |
| carry 78 and the recovery | a backlog larger than a snapshot's remainder still sends the nearest 8; after an evicted baseline under SS load, the first frames hold every relevant soldier | today's order |
| carry 91 | the ring takes the stale copy when a tick becomes last_acked (a ring unit test) | the copy taken only on overwrite |
| B1 runs (pre-runs) | the short B1 suite: at most 18 flag-0 ticks per client per burst on average, in a building 0, door-gap margins and the share printed (ADV-U4GB-13); the short bitten B1 run: no heal within 500 ms, each client's flag-0 ticks per burst and the share printed, to the advisor before the B1 set if it would not hold pre:PM-0001-2 at the judged setting (ADV-U4GB-12) | (the room's fix itself; never re-run into a pass) |
| the contract | every new suite's checks resolve, every name has its row | a renamed series |
- **Plants:** every new check and bite site fails by name under a plant, restored with the file's sha256 (u4f-brief.md
  :1202-1204). Counts as deltas on `<base>`'s; negative cases counted; the report states its own.
- **End checks:** u4f-brief.md section 6, items 1 to 5 (:1165-1182), **item 4 included** (`cargo test -p npt-server
  --all-features` compiling: U4g changes npt-room's and npt-client's public items and bite lists, which npt-server's
  tests and the lab's router read; ADV-U4GB-9).
- **After the merge:** the pre-runs, then the sets one per call; -58's last, after his U4G-Q2 yes. A judged run's
  overflow resync in a -17 or -18 run, a door entry in a B1 run, a slot above -45's bar, or a correction traced on one
  side only comes to the advisor before its set counts (Q6 to Q9).

## 4. The carries, by number (met by number in U4g's report)
- **The B1 work moved in from U4r** (the advisor's U4r build ruling 3; u4-build-plan.md:895-900):
  - **Point 1** (advisor-u4r-build3-full.md:13-15): the room's recovery after an eviction; target "on B1 with 10 bursts
    in 600 s, PM-0001-2's everyone-in-view share is at least 0.99, which allows at most 18 flag-0 ticks per client per
    burst on average", "shown first by a short B1 suite of U4g's build, then by -17's and -18's L1 B1 judged runs";
    "-18's bitten judged run moves to U4g". Carry 78 amended (u4s-carries.md:577-584). U4r's B1 verdicts for -17 and
    -18 were INVALID, never counted, and nothing of them is re-read (u4r-report.md:473; advisor-u4g-brief.md:165-171).
  - **Point 4** (:34-36): carry 91 (u4s-carries.md:585-588), riding U4g's npt-room work; carry 77 (b) amended (:384-386)
    counts the fast heals apart until 91 closes the hole.
  - **The heal reading** (plan §11, u4-build-plan.md:1170-1173; ruling 3's further finding, :69): U4g's report gives the
    recovered time per burst, the input to that readings-PR line; no check is added.
- **U4g's section of u4s-carries.md (:557-604):** 47 (:558-563), 70 (:564-568), 78 (:569-584), 91, 92 (:589-592, route
  ruled: the lab's `tick` record), 79 (:593-597); from U4f-F1's round (:598-604): TE-F1-5 (advisor-u4f-f1-r1.md:147-150),
  NC-F1-4 (:156-158), -58's press wrap by a lab start in carry 75's form, never an unchecked first command (:123-125;
  u8 counters, sim/src/seam.rs:33), and C-U6-19's line (:120-122): **U4g's runs hold no hostile client, so none throws
  or rejoins as one, and no client is restarted mid-run.** Carry 93 (:609-614) is U4h's.
- **Carry 24's U4g half** (u4s-carries.md:154-158): -45's and -62/cues-from-replay's ids in npt-client; -61/on-arrival
  there too; no id in two crates' lists.
- **C62HB-Q4 (b)** (his, RECS-1043: history.md:2204-2227; the advisor's routing :2477-2496; ADV-CBR-1 :6425-6429): -58
  gets -17's two lost-tick checks after his yes to the exact wording. U4G-Q2.
- **The event cap's margin, 1.43** (u4f-report.md:631-636; advisor-u4f-stop.md:132-134) and **EVQ-LOAD's line**
  (advisor-brawl1-sets-p3-full.md:156-158; carry 58, u4s-carries.md:269-273): about U4g's own B1 set. 1,600 stands
  unless R2's route moves it (advisor-u4f-stop.md:129-131). Ruled: Q7.
- **u4g-brief-carry.md** (history.md:3080-3081): how a B1 entry reads: it lowers -2's everyone-in-view share under
  0.99 for that run, so the run is INVALID for -17 and -18 by `pre:PM-0001-2`, recorded, never counted; -45's B1 part
  still counts it. Ruled: Q8.
- **C62H-BOUND's ten deferred Lows** (history.md:6421-6424) ride only where U4g opens the file cheaply
  (advisor-u4f-build1.md:213-217); the report says which.

## 5. The rules that stand
- Start and stops: u4f-brief.md section 1 (:288-355), read for U4g: the block-end size stop is the one his U4G-Q1 answer
  sets (past 6,000 whole; under (b) past 6,000 for U4g and past 3,200 for U4g2: ADV-U4GB-14), by
  advisor-u4f-build1.md:178-181's form; counts are deltas
  on `<base>`'s; the test step's stop is 1.5 times `<base>`'s measured test step (TE31-10, u4f-brief.md:326), written
  in the fill form at launch (1,207 s was 5b29296b's).
- U4f's build rules (section 4, :831-958) with 1.1's lane; the shared target and its lock; one builder in an isolated
  worktree, Opus 5.5 at xhigh; at most three agents at a time.
- **No pass mark, judge rule, marks.json text or existing SERIES.md row changes, and no run is dropped; -60's +3 as
  ruled (U4G-Q4).** A build that needs one of those stops, and the main session asks him (history.md:2262).
- **LEAD-BOUND (a), 400 ms, frozen** (history.md:6791-6806).
- Every agent brief carries, word for word: "Write files with the Write and Edit tools only (no sed, no shell
  redirect, no python writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit
  tool rewrites its notes file whole with Write."

## 6. Size (SIZE-CHECK, history.md:2048-2051: M is about 3,000 added lines, the stop past 6,000)
| Item (added lines in spikes/) | Prediction and cues | Interpolation, events, B1 |
|---|---|---|
| sim: carry 47 and its test; -55's site and proof; TE-F1-5 if cheap | 150-320 | 0 |
| client src: `Predictor` 300-450, `CueLog` 80-130, two sites, outputs 40-80 / `Interp` 250-400, release 70-120, the send, two sites, outputs | 440-700 | 380-640 |
| client tests | 300-480 | 200-320 |
| room: carry 79, server-echo, -58's counters, -60's site, tests / the clamp, carry 78, the recovery 50-120, ack.rs 20-50, tests | 230-420 | 230-480 |
| lab: four suites, CO's driver, press start, the `tick` record, writers, contract, tests, Q6's print / two suites, B1 rows, Q8's and Q9's prints, writers, contract, tests | 680-1,180 | 290-560 |
| bots: CO's course with fire, the cue shooter, the vehicle walker, the attribute script | 150-300 | 0 |
| ops: both scripts' rows and sets | 160-270 | 60-100 |
| SERIES.md new rows | 5-20 | 5-10 |
| **Each half** | **about 2,100 to 3,700** | **about 1,150 to 2,100** |
- **Whole: about 3,300 to 5,800** (midpoint about 4,550; the advisor's own rounded figure 3,350-5,850). U4f's whole
  scope ran about 1.5 times its estimate, its client and room about 2.0 (advisor-u4f-stop.md:56-57, :101-102;
  u4f-report.md:314-315): at 1.5 about 4,950 to 8,700 whole; the prediction half up to about 5,550 (under 6,000); the
  second half, planned about 1,600, up to about 3,150 (its stop past 3,200).
- Over M's 3,000 but under 6,000 is no stop by his words; U4G-Q1 is his because a split is a plan change
  (advisor-u4g-brief.md:188-203).

## 7. Reviewers and rounds
netcode-architect, test-engineer and rust-reviewer (u4-build-plan.md:901), each at xhigh, at most three at a time; at
most two rounds, a third only for a Critical or High; under (b), one panel per half. U4G-Q2's text gets test-engineer's
read before it goes to him (TEST-FIRST, "a reviewer's read on every change to marks.json or to the judge").

## 8. Questions
1. **U4G-Q1 (his; the next day's message, not a stop).** (a) Build it whole as planned: one review, one merge, the stop
   past 6,000. (b) Split now: U4g, prediction and own-shot cues (about 2,100 to 3,700; stop past 6,000), then U4g2,
   drawing other players, event timing and the server's recovery after a loss burst (about 1,150 to 2,100; planned
   about 1,600, stop past 3,200); no pass mark, judge rule or marks.json text changes, no run dropped. The cheaper
   option is (b): one more review and merge, and no mid-build stop waiting on him. The build starts with the
   prediction half either way. **Lean (b), the advisor's too.** Paste-ready, final: advisor-u4g-brief.md:360-374 (its
   rev 2 check, with section 6's figures).
2. **U4G-Q2 (his, later; only -58's judged set waits).** Yes to the exact wording of -58's two lost-tick checks: (a)
   yes, (b) change it. No "not now": he chose (b) "with U4g" (history.md:2216, :2204-2227). The making, in order
   (advisor-u4g-brief.md:223-232): the main session copies -17's two entries (marks.json:3983-4004) by script onto
   -58's P0 and P3 parts and drafts route B beside it (the figures reading the warm-up and measured ticks, never the
   after phase), weighing -17's P0 form for P0; c-u4g-r2.mjs's wrap rows show route A or not (P0 and P3, 48 clients,
   a 1 s warm-up, every slot admitted and resolved in it, every wrap at or after the measured start); test-engineer
   reads the text of the route that holds; the advisor checks; his yes recorded word for word with the text's sha256;
   the marks.json edit (and under route B the judge change, its own small change-set) committed before -58's set. The
   wording to him: advisor-u4g-brief.md:233-246. **Lean (a).**
3. **U4G-Q3, scenes. Ruled** (advisor-u4g-brief.md:137-147): -45 on SS on the brawl's load, its B1 runs serving -17 and
   -18; -55 on SS with one client, contact ticks printed; -60 and -62 on CO, -60's soldier firing throughout; -61 on SS.
4. **U4G-Q4, -60 under both options. Ruled** (:148-152): (b), 3 per option; §9 now reads 6.
5. **U4G-Q5, bite sites. Ruled** (:153-159): -60 in npt-room (pre-change values); -61 in npt-client's release; -55 in
   npt-sim in `CHECK_BITES`' form, armed in the authority's World only.
6. **U4G-Q6, the trace. Ruled** (:160-164): (a), the client alone, the room the L1 oracle, the disagreement printed.
7. **U4G-Q7, the cap. Ruled** (:165-171): (a) corrected: the P4 and B1 cap reads before their sets; 1,600 stands.
8. **U4G-Q8, the door gap. Ruled** (:172-177): (b), the per-burst margin printed always, with "in a building".
9. **U4G-Q9, -45's flags in a 48-client log. Ruled** (:178-184): slot 0 observes the share, any slot sets the bound,
   a header param names the observer (a new SERIES.md row), every slot's share printed; a per-slot series goes on
   rule 1's after-close list.
10. **NC-U4R-4's route. Ruled** (:185-186): the lab's `tick` record.

## What goes to Mustafa from this brief
- **U4G-Q1** in the next day's message: split U4g into U4g and U4g2, or build it whole. Lean (b). Not a stop: U4g's
  launch waits for U4f2's merge and -21's runs anyway, and the prediction half is the same work under either answer.
- **U4G-Q2** later, when its text is drafted, read and shown by the short rows: yes to the wording. Only -58's judged set
  waits for it.
- Ruled by the advisor and named in the day's message: U4G-Q3 to Q9 and NC-U4R-4's route. Nothing else: no money,
  nothing on main.
