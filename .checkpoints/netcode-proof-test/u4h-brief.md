# U4h's brief, rev 2.1 (2026-11-21; rev 2 with the advisor's conditions met, advisor-u4h-brief.md:426-432)
**The unit:** lag compensation and aim, u4-build-plan.md:903-918 (plan rev 3.3; size M). It builds after U4g, or after
U4g2 if U4G-Q1 splits U4g, and after that unit's judged sets (one lane at a time: :951-952). Short by TEST-FIRST rule 3
(history.md:2264-2267; his message :2236-2282): what U4h builds, how it is tested, what is open; the rest is pointed
to. U4f's brief stays the reference for the start and stops (u4f-brief.md section 1, :288-355), the build rules
(section 4, :831-958), the pre-runs, judge calls, disk and bars (5.3-5.6, :997-1162), the end checks (section 6,
:1163-1183) and the return list (section 7, :1187-1225). **Three questions are his, none a stop now** (section 8):
U4H-Q4 (what -50's "honest twin" is), U4H-SIZE (split or not) and U4H-JUDGE (the judge's computations as their own
change), in the first day's message after this rev is checked and U4G-Q1 is answered (2026-11-22 at the earliest).

**Rev 1 to rev 2** (the advisor's check, REVISE, one High: advisor-u4h-brief.md, 361 lines; its list :332-343):
- Item 1, ADV-U4HB-1 (High): TC's own weapon (spread 0, a magazine and reload the shooter meets), and, mine beside it,
  TC's health (CO's 100 kills the sprinting target every 10 hits: world.rs:1694-1701): 1.2, 1.3, block 1, section 3.
- Item 2, ADV-U4HB-2: the shot id, press-edge firing, reruns and refused presses: blocks 6-7, section 3. Item 3, -3, -4,
  -7: Q2, Q5, Q6 folded: blocks 2, 6-8, 13, section 2. Item 4, -5: the wait cap, block 2, section 3. Item 5, -6: 1.4.
- Item 6, -8, -9: section 6 recomputed; section 8's three texts with its figures; J1-J3 a change of their own; block 9
  keeps the no-answer line. Item 7: one "Ruled" line each (section 8). Item 8, Lows: -10 the calibration's figure
  (section 2); -11 Q10; -12 section 9; -13 block 4's hand-off, block 7's frame wait, Q1's figure, the disk table's
  runs, 1.1's C2, section 9's after-close line, block 11's "differs". u4s-carries.md untouched: section 9 lists its lines.

**Rev 2 to rev 2.1** (the advisor's rev-2 check, APPROVE WITH CONDITIONS, advisor-u4h-brief.md:364-483; the main
session's edits, no new check needed if met as written): ADV-U4HB-14 (Medium), TC's health of 30,000 cannot ride the
soldier record, so TC's weapon has damage 0, health stays 100 and the magazine covers the run (1.2, 1.3, block 1);
Q6 and Q7 as ruled on rev 2 (block 13, section 8 items 6 and 7, section 9's after-close line); ADV-U4HB-15 (the
judge's block), -16 (1.2), -17 (block 2's margin); the register's eight lines added (u4s-carries.md:618-659).

## 0. Where things are
- **`<base>`:** the spike branch's head at launch, after U4g's (or U4g2's) merge and its sets, filled by the main
  session in U4f's fill form (u4f-brief.md section 12, :1724); every count here is a delta on it. Every "there now"
  below was read at 33dabe8, whose spikes/ equals U4f2's base a289def (the advisor re-read it at f1a7f73: :22-23).
  U4f2, U4g and U4g2 open crates/room, client, bots, lab and sim before U4h, so those lines move and are re-read.
- **Read, never retyped** (stamps refilled at `<base>`): u4-build-plan.md, plan.md, u4s-carries.md (U4h's section
  :609-651), marks/marks.json, marks/SERIES.md, u4f-brief.md, u4g-brief.md (rev 2.1), advisor-u4h-brief.md,
  u4r-report.md, ADR 0001 (-50's row :1647), the catalogue's ClientSettings row (netcode-proof-messages.md:96).
- **One cite drift for the main session's next plan edit (Low):** u4-build-plan.md:125 and :152 cite plan.md:696 for
  -7's sizing; the line is now plan.md:724.

## 1. What U4h builds
### 1.1 What it needs from U4g and U4c, and what U4G-Q1 changes
- **From U4g:** `Interp`'s published interpolated movers and its render time (AimCast's set, the TC shooter's view test
  and the view time every client stamps: u4-build-plan.md:904-906, §4.8 :369-372); the ClientSettings send and the
  room's clamp (the announced delay the estimate reads: §4.9 :382-383); the delay's 2-to-5 adaptation, which sets the
  honest rewind the calibration measures. **An interface item U4g's report states** (advisor-u4h-brief.md:249): whether
  every `ClientEndpoint` sends ClientSettings, and what view time the brawl stamps (today its newest snapshot tick with
  fraction 0: bots/src/lib.rs:22-24). **Not needed:** `Predictor`, `CueLog` (the TC shooter stands at its spawn,
  scene.rs:262). **From U4c:** C2, an RTT sample only from the newest packet (u4-build-plan.md:145, :903), met at U4c's
  merge.
- **Under U4G-Q1 (a)** (advisor-u4g-brief.md:360-374, unanswered): U4h launches after U4g's merge and its sets.
  **Under (b):** all three items land in U4g2 (u4g-brief.md:86-93), so U4h launches after U4g2's merge and its sets.
  The answer moves U4h's launch, never its content. A later change to `Interp`'s delay rule re-runs the calibration
  before any set counts.

### 1.2 The lane
- **Open:** crates/sim (TC's weapon in scene.rs; the two pub items of 1.4; -51's site; one `with_*` scene builder
  by Q6); crates/room; crates/server (one accessor and its tests); crates/client (AimCast; the
  flight and -54's site); crates/proto (one pub item: the aim's widths, private at proto/src/input.rs:53-54);
  crates/bots; crates/lab (recordings.json and det-recordings.json included); ops/runs (new c-u4h-*.mjs; det-u4.mjs's
  print, INF-U4R-6); marks/SERIES.md (new rows, and the cells of Q2 and Q7). **crates/judge is not U4h's under U4H-JUDGE
  (b)**, the lean: J1-J3 are then a change of their own, sharing no file with U4h; under (a) it is U4h's (the judge's
  block, inside U4h between blocks 8 and 9: ADV-U4HB-16).
- **Shut:** l1, link, net, log, client-wasm, hash-wasm; marks.json, marks/README.md, every other existing SERIES.md row;
  ADR 0001 and the catalogue (a word that changes goes on the readings-PR list); check.mjs; Cargo.toml, Cargo.lock. A
  file outside the lane stops the build for the main session.

### 1.3 What is there and what is new (read at 33dabe8)
| Item | There now | New in U4h |
|---|---|---|
| TC | shooter, sprinting target, three plates (scene.rs:252-265), with CO's weapon (spread 5,000 µrad, 30 rounds, 2 s reload) and health 100 (:236-246, :263); the plates 30 m out (:338-342), the target about 20 m | TC's own weapon (spread 0, damage 0, a magazine covering the run; health stays 100); the suites; no TC run exists yet |
| The ring | `HitHistory::for_ms`, at most 220 ticks (one tick at most 4,536 B) (sim/src/history.rs:23-39); `HISTORY_MS` 2,000, private (room.rs:33, :550); the lab's copy (scenario.rs:78-80) | the default made readable, the TC sets' test span (Q2) |
| `rewind_at` | the integer half: the claim or `None`, counted (room/src/rewind.rs:13-33; room.rs:1359-1368) | the estimate, the tolerance, the cap (open, test, rule), each replacement and clip counted; the shot log |
| The RTT | `Decoder::rtt_ns` by `RttSource` (server/src/pump.rs:183-190, :339-344); `pump_report` has none (room.rs:877-885), called by the lab (scenario.rs:704) and npt-server's tests (common/mod.rs:290, loopback.rs:213) | the RTT through `pump_report` from those callers; one accessor for a Link session's ping estimate |
| The cast | the shot phase's f32 blend and cast, private (world.rs:1574-1610, `FBox::blend` :209-223); `World::ray_cast` on integer volumes (:1092-1102); the fire origin's private `CHEST_CM` (:82; lab copy scenario.rs:84-86) | one shared blend-and-cast and the fire-origin height, both pub (1.4) |
| Shots | `SimEvent::Shot { slot, seq }` and `Hit { shooter, seq, .. }` (seam.rs:466-471; world.rs:1618-1621, :1688-1693) | the shot id over them; press-edge firing |
| Throws | `trajectory_at` (sim/src/trajectory.rs:13); a throw press makes a grenade only (world.rs:1522-1545); TC spawns no stand-in | the client's landing compare; TC's stand-ins in `throws` (Q6) |
| Bots | brawl, forgers, lag switch, never-acker; "the target-course shooter is U4h's" (bots/src/lib.rs:10) | TC shooter, target sprint, edge stamper, pong delayer, plate shooter, thrower; the brawl's view stamp; the ES hit rule |
| Judge | four figures `planned` (judge/src/figures.rs:841-860); formulas SERIES.md:274, :289, :295, :299 | J1-J3, under U4H-JUDGE |

### 1.4 Public items, by crate (A19: each report lists them; u4-build-plan.md:102)
- **npt-sim:** the shot phase's blend-and-cast over two `HitVolumes` at a fraction, which the authority's shot phase and
  the client's AimCast both call (ADV-U4HB-6); the fire-origin height (`CHEST_CM` made pub, the lab's copy and
  `PLACEMENT_Z_BOUND_CM` giving way: EA-U4R-4, advisor-u4r-r1-full.md:182-186); the flags index and the dead bit
  (EA-B1-1's later half, u4r-report.md:1164-1165) if U4g's report does not list them; bites builds only, -51's list in
  `CHECK_BITES`' form; Q6's one `with_*` stand-in builder.
- **npt-room:** the default span made readable (carry 89); the cap setting (open, test with ticks, rule); `pump_report`
  gaining the RTT. **npt-server:** the ping-estimate accessor. **npt-client:** AimCast; the landing compare; `BITES`
  gains -54's id. **npt-proto:** the aim's widths. **npt-bots:** the new `Behaviour`s.

### 1.5 Blocks (each compiling with its tests passing before the next; at most about 400 added lines, a larger one split)
**The lag-compensation half** (U4h under either answer to U4H-SIZE):
1. **npt-sim:** TC's own weapon: spread 0 and damage 0 (the Hit is pushed before the damage is taken,
   world.rs:1687-1696, so every hit still registers and nobody dies, the cheats and the target included), its interval
   and reach as CO's, and a magazine that covers the run (AMMO holds up to 65,535: seam.rs:240), so no reload; TC
   keeps CO's health 100 (HEALTH holds only 0 to 1,023: seam.rs:224; ADV-U4HB-14); the weapon's values in every header
   (carry 60); the two pub items of 1.4; recordings.json's synthetic TC entry re-pinned only if its SHA-256 moves, with
   the reason.
2. **npt-room, the estimate:** the pump's RTT + the announced delay (U4g's clamp; for a session that sent no
   ClientSettings, `Interp`'s starting 3 intervals, u4g-brief.md:88) + the input-buffer wait, capped room-side at
   **min(LEAD-BOUND's 400 ms, the session's RTT + 3 tick periods)**, from the pump's RTT, the tick period and
   LEAD-BOUND alone, never a client figure (carry 90: an honest controller aims to arrive one period after its upper
   one-way estimate, which lies inside the RTT, then rounds up a tick and holds its target: client/src/lib.rs:634-646)
   + half a tick; each capped wait counted (a new SERIES.md name). A session with no RTT yet: Q3. **The cap's margin**
   (ADV-U4HB-17): it holds an honest wait while the round trip's jitter is at most a third of it (true on P1 to P4
   and B1, P1's slack about 10 ms; P0's slack 0, so "capped" means above the cap); the most a stamp-ahead can buy is
   the cap less the honest wait, stated per profile for the security-auditor's read; the P0 pre-rows print capped
   waits too.
3. **npt-room, the rules:** the tolerance `TOLERANCE_NS` = 33,330,000 (data, ADR 0001:363), a claim within it of the
   estimate (inclusive) accepted, else replaced and counted; then the cap: `open` (the default) clips at the ring's
   reach (the span less the blend tick), `test` at its ticks, `rule` later, each clip counted; every run of a command
   judged at the tick it runs, reruns counted apart (carry 81); the per-shot log (SERIES.md:206-216) with
   `event.shot.registered` (:411) by shot id; -7's records; the sites -48/no-interp-delay, -48/no-buffer-wait,
   -50/no-tolerance-check (Q11).
4. **npt-server and the hand-off:** the lab (scenario.rs:704) and npt-server's tests (common/mod.rs:290,
   loopback.rs:213) pass each session's RTT to `pump_report`; npt-server's src gains only the ping-estimate accessor
   (-50's gap input; pump.rs has none: :339-344); U6's TickLoop carry names the same hand-off. End check item 4.
5. **npt-client, AimCast:** the shared blend-and-cast on the volumes of `Interp`'s two published snapshots
   (`World::hit_volumes_from`, world.rs:933) at the view time it sends (the quantized tick and fraction), from the
   fire-origin height; hull parts blend as the server's (N2-L3 (b)); a non-finite ray is a Miss (carry 45).
6. **npt-bots:** the TC shooter (a 60 Hz frame loop, seeded phase, input and view time each frame: plan.md:471),
   **firing by press edges only** (fire never held, so an option-B rerun, whose counters equal the last run's, fires
   nothing: u4-build-plan.md:247-252), its aim by Q8, its view test per press, and its frame-to-tick wait per press;
   the target's sprint (9 m/s, seeded turns); the edge stamper and the pong delayer by Q5; **the brawl stamps
   `Interp`'s render time** in place of its newest snapshot tick, so honest brawl shots sit inside the tolerance (Q2).
7. **npt-lab:** `--cap open|test <ticks>|rule` and the span, by Q2; the TC suites (calibration, test cap),
   `rewind-cheat` (Q4's two figures, Q5's settings, the 1-second gap, `--transport ws` with `RttSource::Pings`); the
   **shot id** (SERIES.md defines it: one per server shot, (shooter slot, firing command's sequence), unique in a run
   since a slot sends under 65,536 commands in one; a contract assert); the view test logged only for a press the room
   fired, a press the sim refused counted "not fired" (a new name, printed), never a gap; `frame_wait_ns` written into
   the room log's shot event by the lab from the shooter's frame record (the room sees no frame: SERIES.md:211); the
   calibration print; the header params; the contract cases (§4.12 :443-446), one holding a rerun and a refused press.
8. **npt-lab and ops, the recording:** TC B1 seed 3 at the default cap and span (`open`, 2,000 ms: Q2), its length set
   by Q10's measuring row first, then `scenario --record` into det-recordings.json (carry 87); det-u4.mjs prints its
   own non-cargo sum and "over 240 s: to the advisor" (INF-U4R-6).
9. **ops/runs:** c-u4h-r2.mjs (the pre-runs, the calibration, the cap computed) and c-u4h-runs.mjs (the sets), in
   c-u4f-runs.mjs's form. **If his U4H-SIZE answer is not in when block 9 is done:** the end checks run on the tree as
   it stands and the builder returns (the size stop's form); the main session goes on by his answer.

**The aim-and-throws half** (U4h2 under U4H-SIZE (b)):
10. **npt-sim and npt-proto:** -51/parallel-ray-10cm's site and its unit proof; the aim's widths pub.
11. **npt-client:** each landing through `trajectory_at` beside the server's; "differs" = not the same landing tick and
    a bit-equal point (both from `trajectory_at`); -54/step-1-60's site.
12. **npt-bots:** the plate shooter (ground, 2.5 m, 4 m; press edges), the thrower, the brawl's ES-only seeded hit rule
    (carry 93).
13. **npt-lab and ops:** the suites `aim-height` and `throws` (the stand-in spawn rate set by Q6's one `with_*`
    builder in `with_stand_ins`' form, scene.rs:328-331, used by `throws` alone, its values in the header, so TC's data
    stays as it is for -48, -50, -51 and the synthetic TC entry; the pre-run prints each kind's compared count, the
    stand-ins spawning inside the TC client's 64 m set), `--cover` on SS (Q9), their writers and contract cases, carry
    93's ES rows, the sets.
14. **The end checks** (section 3), after each half if split.

**The judge's change** (J1-J3; its own small change under U4H-JUDGE (b), the lean; inside U4h between blocks 8 and 9
under (a); not built under (c)): **J1** `view_hit_registered_share` and `false_hit_share` joined by shot id,
`rule_cap_set_by_rule` (fixtures only), `throw_desync_count`, each with its L2 Arm-rule half on fixtures only (T17's
route, u4-build-plan.md:435-436); **J2** `cross_arch_mismatch_count`'s shot view-test and throw-landing kinds
(SERIES.md:265), on fixtures; **J3** JDG-ONLY-1's fix with its interim rule's fixture (history.md:4611-4614). PASS,
FAIL and gap fixtures for each; the test-engineer's and rust-reviewer's read of every line, named in the report with
marks.json's sha256 before and after (history.md:2127). Only U4h's judged sets wait for it. The judge's change writes
no SERIES.md line: the shot id's row is U4h's (block 7), and J1's fixtures follow its wording, (shooter slot, firing
command's sequence), whichever merges first (ADV-U4HB-15).

## 2. Marks moved and the runs (the main session's, after the merge, in the main folder)
| Mark (marks.json) | L1 parts (`min`) | Suite, scene | Measured | Bites (site; run) | What it reads |
|---|---|---|---|---|---|
| calibration (no mark) | P3, P4 x A, B, seeds 101-103 (T14: :74, :910-912) | TC, cap `open`, 2,000 shots | 12 | none | per (profile, option) the larger of max `honest_rewind_ns` and max `rewind_ns` of honest shots; the test cap = ceil to ticks + 3 (Q1) |
| -48 (:9137) | P0-P3 x A, B (3), `params.cap` test | TC, 2,000 shots a run | 24 | no-interp-delay (P2, seed 1); no-buffer-wait (P3 B, seed 1: -20's buffer median 2 ticks, u4f-report.md:622; the shooter's depth printed, below 2 to the advisor: §4.14 :498) | registration >= 0.99 to P2, >= 0.95 at P3; false hits < 0.005; setup `clipped` 0 |
| -49 test cap (:9441) | P0-P4 x A, B (3) | -48's 24 runs list -49 too; P4 apart | 6 | cap-estimate-only waits for R3's rule cap (marks F24) | `over_cap` 0; parts PASS unbitten, the mark NOT-RUN on its bite |
| -50 (:9986) | P0, P1 (3); ws leg (1) | `rewind-cheat`, TC (Q5) | 7 | no-tolerance-check (P1, seed 1; alternating stamps: Q5) | both cheats' 1,000 shots, gains <= 33.33 ms (Q4's reading); the gap >= 145 ms each second; ws reported |
| -51 (:10111) | TC P0-P3 (3) | `aim-height`, 1,000 shots a height | 12 | parallel-ray-10cm (P0, seed 1) | each height's 5 cm share >= 0.999; aim <= 80 bits (44: input.rs:310) |
| -54 (:10626) | L1 (3) | `throws`, TC: grenades and stand-ins, 1,000 each (Q6) | 3 | step-1-60 (seed 1) | `throw_desync_count` 0 |
| -56 (6), (10) (:10745) | none at U4 | (6) from -48 and -49 (P2-P4 per option); (10) on SS (Q9) | 0 | none | registration and honest rewind per option; cover per variant |
| -7 (:2646) | none at U4 (L2, R2) | every header records the span | 0 | none | R2's run plan takes the test span (plan.md:724) |

- **Counts:** 52 measured, 5 bites, 12 calibration: 69, as §9 (:1031-1035). Under (b), U4h holds the calibration,
  -48, -49 and -50 (37 measured, 3 bites), U4h2 -51 and -54 (15, 2). L2, ML and rule-cap parts stay NOT-RUN.
  **Recorded beside them:** the TC B1 seed 3 recording (1); Q9's cover runs (6 on SS, or 3 beside carry 93's); carry
  93's ES measurement (6-9: P2, seeds 1-3, a 10-minute window, per candidate value) and its check that SS on U4h's
  build reads the same with and without the ES rule; carry 88's -30 pre-check re-run (6, results/u4h-precheck-30/,
  and SS P0 seed 4 re-recorded on U4h's build if a leg replaces: its old commands carry the brawl's old stamp); the
  pre-runs. About 85-95 runs.
- **The pre-runs** (c-u4h-r2.mjs, before the sets): each new suite short with its contract case; a short P3 B TC run
  printing the shooter's buffer depth; a short P0 pong-delayer run printing each second's gap; Q5's short P1 edge run
  and the bite's alternating setting run clean; **a short SS P2 brawl row printing the replaced and clipped shares**
  (Q2); Q10's TC runner-rate row; the calibration set, then the cap written into the sets' arguments. A row other than
  its `expect`, or a replaced share above 1 % on the brawl row, in carry 93's runs or in carry 88's legs, comes to the
  advisor before any of them counts.
- **Judge calls:** two per set (u4f-brief.md 5.4, :1053-1079); no U4h mark carries `pre:PM-0001-2`. A clip in a judged
  run's setup, a gain above 33.33 ms, a capped wait on an honest shot, fewer than 400 view misses in a -48 run, or a
  registration below its bar comes to the advisor before its set counts; no run is re-run into a pass.
- **Disk and time** (estimates; the runner projects at launch):

| What | Runs | Wall a run | Logs a run | About |
|---|---|---|---|---|
| TC sets, calibration, bites, recording (1-2 clients) | 70 | no TC run yet; 10-60 s (SS's 48 clients: 59-62 s for 3,900 ticks, results/u4f/b57221ed139d/pm-0001-47/runs.txt:17-19) | 2-10 MiB (unmeasured) | 0.3-1.2 h; 0.15-0.7 GiB |
| Q9's SS cover runs; the SS brawl row | 3-6; 1 | 250-390 s (-22's and -20's runs) | 62-210 MiB | 0.3-0.7 h; 0.4-1.3 GiB |
| carry 93's ES runs (48 clients) | 6-9 | 320-390 s | 62-210 MiB | 0.6-1.0 h; 0.4-1.9 GiB |
| carry 88's pre-check pairs | 6 | under a minute | about 17 MB a folder (history.md:6528) | 0.1 h |

  **The build's disk:** about 10.1 GB free after a part-clean (history.md:7344-7353), the runner's floor 4,096 MiB,
  and a full check.mjs run fills the shared target by about 6.3-7.1 GiB (dev) and 0.26 GiB (release)
  (history.md:7336-7352), so every counted check.mjs run meets the floor: one part-clean before each (P27's second
  form, no lane running, under the lock: history.md:7330-7338), as U4f2's four. **The sets:** about 1.0 to 4.0 GiB of
  logs in all, run with the shared target cleaned; D2 at launch; a set past the floor waits for a pack of a closed set
  (c62h-bound-brief.md 8.1 :485; KEPT-LOGS). The judge's peak stays far under 10,240 MiB.

## 3. How it is tested
| What | The test | Its plant, failing the test by name |
|---|---|---|
| TC's weapon (item 1) | a TC shot's server ray runs from the server's fire origin through the command's aim point exactly; the target lives through a run's shots | CO's spread restored |
| the estimate | its four terms from a hand-made session; the default 3 intervals for a session with no ClientSettings | the default omitted |
| the wait cap (item 4) | an honest option-B client at P4 never meets it (the calibration's P4 B runs print 0 capped); a stamp-ahead at LEAD-BOUND's 400 ms buys no wait past it (carry 90) | the cap removed |
| the tolerance and cap | accepted at exactly `TOLERANCE_NS`, replaced one view-time step (1/256 tick) past it; clipped and counted over each cap; `open` clips at the ring's reach; `over_cap` 0 | -48's two bites; -50's bite |
| reruns, presses (carry 81, item 2) | a rerun judged at its own tick fires nothing; fire held with the uplink starved buys no rewind; a refused press is "not fired", never a gap | reruns judged at the claim's tick |
| no RTT yet (carry 88) | a shot before the first sample runs at the present, counted; 0 in every TC run's whole log | the claim taken |
| the ring (carries 28, 89) | the default span readable; hash.rs's span check (:370-376) reads it; the TC span in the header | a span copy that differs |
| AimCast (carry 45, N2-L3 (b)) | on a fixed trace with a hull, the client's cast equals the server's: one shared function, the same volumes and quantized fraction; a non-finite ray a Miss | the cast at the unquantized fraction |
| the shot id (item 2) | unique per server shot in a run; the contract case's rerun and refused press | a join by tick |
| the TC shooter | seeded 60 Hz phase; frame-to-tick wait in [0, one tick); same seed, same commands | an unseeded phase |
| -50's cheats (Q5) | the stamper announces the clamp's top; the pong delayer's gap >= 145 ms each second at P0 | the hold reported as 150 ms |
| the calibration | the printed figure per (profile, option); the script's cap = ceil + 3 | a cap rounded down |
| -51, -54 | each height within 5 cm; each kind's landing equal (tick and bits) | their bites |
| the recording | TC B1 seed 3 regenerates to its pinned SHA-256; det-u4.mjs's sum printed | a changed seed |
| the contract | every new suite's checks resolve, every name has its row | a renamed series |
- **Plants:** every new check and bite site fails by name under a plant, restored with the file's sha256 (u4f-brief.md
  :1202-1204); each bite id in lab/tests/bite_homes.rs (Q11). Counts as deltas on `<base>`'s; negative cases counted.
- **End checks:** u4f-brief.md section 6, items 1 to 5 (:1165-1182), item 4 included (npt-server's src and tests change).
- **The wait cap and the tolerance** get the security-auditor's read in the round (ADV-U4HB-5).

## 4. The carries, by number (met by number in U4h's report)
- **U4h's section of u4s-carries.md (:609-651):** C-U6-19's line (:610-611): **U4h's runs hold no throwing or
  rejoining hostile client and no client restarted mid-run** (-50's cheats only stamp view times or hold pongs; -54's
  thrower is honest). 93 (:612-617); 28 (:618-619); 45 (:620-622); 81 (:623-628); 87 as amended (:629-635); 88
  (:636-643: Q3, the re-run above); 89 (:644-647: Q2's terms 2 and 4); 90 (:648-651: block 2).
- **Outside the register, eight items**, each met here (block 5, block 8, block 4, 1.4, Q7's edit, J2, J3, block 1);
  section 9 lists them for the main session's carries edit before launch.
- **Already taken elsewhere:** N2-1, EA-U4R-1 and -2, RR4-L1 (BRAWL-1's Part 1: u4r-report.md:275-280); T-6 (U4f2);
  npt-server's tests compiling (U4f; still end check item 4); the 1 MB bound (carry 43, U4d's). RR-5's encode-path wins
  (advisor-u4e-r1.md:211-217) are not taken: U4h does not edit the encode path.

## 5. The rules that stand
- Start and stops: u4f-brief.md section 1 (:288-355), read for U4h: the block-end size stop is the one his U4H-SIZE
  answer sets (past 6,000 whole; under (b) past 6,000 for U4h and past 2,300 for U4h2; the judge's change past 1,300),
  by advisor-u4f-build1.md:178-181's form; counts are deltas on `<base>`'s; the test step's stop is 1.5 times `<base>`'s
  measured test step (TE31-10, u4f-brief.md:326), written in the fill form at launch.
- U4f's build rules (section 4, :831-958) with 1.2's lane; the shared target and its lock; one builder in an isolated
  worktree, Opus 5.5 at xhigh; at most three agents at a time; never `cd` into its worktree.
- **No pass mark, judge rule, marks.json text or existing SERIES.md row changes beyond Q2's and Q7's cells, and no run
  is dropped.** The judge only by J1-J3 on his yes; -50's two gain rows (SERIES.md:153, :187) only by his U4H-Q4
  answer; a build that needs any other such change stops, and the main session asks him (history.md:2262). LEAD-BOUND
  (a), 400 ms, frozen (history.md:6791-6806).
- Every agent brief carries, word for word: "Write files with the Write and Edit tools only (no sed, no shell
  redirect, no python writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit
  tool rewrites its notes file whole with Write."

## 6. Size (SIZE-CHECK, history.md:2048-2051: M is about 3,000 added lines, the stop past 6,000)
| Item (added lines in spikes/) | Lag compensation | Aim, throws, cover, ES |
|---|---|---|
| room: estimate, wait cap 20-50, rules, open cap 30-70, shot id 20-50, reruns, no-RTT, span, shot log, sites, tests / plates' error writer | 620-1,120 | 60-120 |
| server: the accessor 10-30 and the tests' hand-off | 30-90 | 0 |
| client: AimCast on the shared cast, tests / landing compare, -54's site, tests | 120-250 | 100-200 |
| sim and proto: TC weapon 20-60, the pub items 30-80 / -51's site, aim widths, a scene builder 5-20 | 50-140 | 50-140 |
| bots: shooter, sprint, edges 10-30, cheats' forms 15-35, the brawl's stamp 15-40, tests / plate shooter, thrower, ES rule, tests | 390-705 | 150-300 |
| lab: suites, calibration, rewind-cheat, ws, params, shot id 20-50, prints 15-35, hand-off, re-pins, recording, contract, tests / two suites, cover on SS 15-40, writers, ES rows | 645-1,115 | 315-590 |
| ops: both scripts, det-u4's print, Q3/Q8 rows 20-40, Q10's row 10-20, the brawl row 10-20 / their rows, cover rows 10-30 | 210-380 | 70-150 |
| SERIES.md: new rows and the three cells | 15-50 | 5-15 |
| **Each half, without the judge** | **about 2,100 to 3,850** | **about 750 to 1,500** |
| **The judge's change (J1-J3):** rev 1's 260-550, the L2 halves and two kinds 100-250, JDG-ONLY-1 40-100 | **about 400 to 900** | |
- **Whole without the judge: about 2,850 to 5,350** (midpoint about 4,100). U4f ran about 1.5 whole and 2.0 on client
  and room (advisor-u4f-stop.md:56-57, :101-102; u4f-report.md:314-315); U4f2 about 1.35 (2,077 against about 1,550:
  history.md:7350-7351). At 1.5: about 4,250 to 8,050, about even to pass 6,000 built whole. Split: U4h up to about
  5,750 at 1.5 (the room at 2.0 and the rest at 1.5 could reach about 6,300 at the range's top: what the stop is for);
  U4h2, planned about 1,150, up to about 2,250 (stop past 2,300); the judge's change, planned about 650, stop past
  1,300. With the judge inside U4h its first half reaches about 7,100 at 1.5. These are judgements on the code's
  shape, not counts. U4H-SIZE is his because a split is a plan change (advisor-u4g-brief.md:188-193).

## 7. Reviewers and rounds
netcode-architect, security-auditor (the rewind's bounds, the wait cap, the cheats, the RTT source), test-engineer and
rust-reviewer (u4-build-plan.md:918), each at xhigh, in two batches (at most three agents at a time); at most two
rounds, a third only for a Critical or High; under (b), one panel per half. The judge's change: test-engineer and
rust-reviewer.

## 8. Questions
**Ruled by the advisor** (TEST-FIRST rule 2; recorded with the reason and named in the day's message that carries
U4H-SIZE; each folded above):
1. **U4H-Q1, the margin. Ruled** (advisor-u4h-brief.md:145-149): (b), 3 ticks on the calibration's larger figure
   (section 2); the ring holds the test cap (margin inside it, as T14 words it) plus the blend's tick: under 14 KB more
   than the figure; a judged `clipped` above 0 comes to the advisor.
2. **U4H-Q2, the cap and span. Ruled** (:150-163): (a) with six terms: `open` the default, clipping at the ring's reach,
   and SERIES.md:450's `cap` cell gaining `open`; `HISTORY_MS` 2,000 readable, the lab's copy giving way; TC B1 seed 3
   at the default; SS-P2-s1 and ES-P3-s2 re-pinned with the reason if their SHA-256 moves (under RR4-L1's cap); the
   default delay and the brawl's stamp stated (blocks 2, 6) with the brawl row; R2 takes the test span for -7.
3. **U4H-Q3, no RTT yet. Ruled** (:164-168): (a); the TC shooter and the cheats fire only in the measured window, the
   warm-up holding every first RTT sample; SS and ES early shots at the present, counted under a new name.
4. **U4H-Q4, -50's honest twin: his, text: advisor-u4h-brief.md** (:169-172, :276-291; below). The build makes both
   figures: the gain series by (a), the honest reference on the link's true RTT even on the ws leg; the real-twin
   figure printed beside it. Only -50's judged set waits.
5. **U4H-Q5, -50's runs. Ruled** (:173-181): as written, with five terms: the stamper announces the clamp's top;
   `TOLERANCE_NS` 33,330,000 with the boundary test; the short P1 edge run expects 0 replaced and gains within one
   view-time step (about 130 µs) of the edge (it may read the room's estimate through the lab: the strongest cheat);
   the bite's alternating setting run clean in the pre-runs; the delayer stamps as a client that believed its ping RTT.
6. **U4H-Q6, -54's kinds. Ruled** (:182-185, and on rev 2 :398-403): both, 1,000 each; the stand-in spawn rate set by
   one `with_*` builder in `with_stand_ins`' form (scene.rs:328-331), used by `throws` alone, its values in the header
   (block 13); a kind with fewer than 1,000 a run comes to the advisor.
7. **U4H-Q7, the pong gap's sign. Ruled** (:186-190, and on rev 2 :404-406): (a); the one SERIES.md edit corrects :86,
   :396's writer cell, :450's `cap` values and :211's writer cell for `frame_wait_ns` (npt-lab writing into the room
   log's shot event, L1), with test-engineer's read.
8. **U4H-Q8, the TC shooter's aim. Ruled** (:191-193): (c): half the presses at the target box's centre, half at a
   seeded offset uniform within ±2 half-widths of it across the line of fire (half of those miss: about 1,500 view hits
   and 500 view misses a 2,000-shot run), the band recorded in the header (a new name), never tuned after a run.
9. **U4H-Q9, the cover record. Ruled** (:194-198): (b) on SS: three SS P2 brawl runs under `--cover present` beside
   three under rewound (or carry 93's three frozen-value ES runs as the rewound half), each printing shots, rays meeting
   cover and hits per variant; recorded, never judged.
10. **U4H-Q10, det-u4's 240 s. Ruled** (:199-203): (a), settling first: the runner-rate row, then a length that projects
    at most about 230 s with a burst inside and under 36,000 ticks, then the recording, then one det-u4 run with
    `uptime` first; over 240 s to the advisor whole, never retried. RR-L4 acts on the Wasm leg, 2.5 s of 223.7.
11. **U4H-Q11, the bite sites. Ruled** (:204-206): as listed in rev 1; each id in lab/tests/bite_homes.rs.
12. **U4H-Q12, JDG-ONLY-1. Ruled** (:207-210): in the judge's change (J3), its own block and fixture; one read, one yes.

**For Mustafa** (his under TEST-FIRST; the advisor's texts, :274-323, with this rev's figures; all three in one day's
message after this rev is checked and U4G-Q1 is answered, 2026-11-22 at the earliest; none a stop):
```text
U4H-Q4: what "an honest twin" means in the cheat test PM-0001-50 (a cheat cannot buy rewind). A cheating test client
claims the largest view delay the server allows and stamps its shots at the very edge of what the server accepts. The
test passes if each such shot gets at most 33.33 ms more rewind than "an honest client at the same RTT"; marks.json's
note says the lab pairs each cheat shot with "an honest twin shot".
(a) The honest twin is the server's own estimate for an honest client at the same true RTT and the same announced
    delay; the gain is the cheat's rewind minus that. A server built as ADR 0001 says passes; the planted bug (no
    tolerance check) fails by about 100 ms.
(b) The honest twin is a real honest shot from the same moment. A real shot also carries the wait from its drawn frame
    to the next tick and the network jitter, which the server's estimate can only centre, so a server built exactly as
    ADR 0001 says reads up to about 50 ms (about 100 ms if the honest player announced a normal delay): -50 would then
    fail unless its bar changed, which would be a separate decision of yours.
Under (a), marks.json keeps its words, the series file's two lines for the gain say what is measured, the reading goes
on the readings list, and (b)'s figure is printed beside it in every run's report, never judged. Only -50's judged
runs wait for this.
My lean, and the advisor's: (a).
```
```text
U4H-SIZE: the size of unit U4h (hits as the shooter saw them, the rewind cap, the rewind cheats, aiming at height,
thrown projectiles). It is planned as a medium unit (about 3,000 lines; your stop at 6,000). Our estimate is about
2,850 to 5,350 lines, not counting the judge's new computations (U4H-JUDGE). U4f, the last medium unit, came to about
1.5 times its estimate in all, so U4h built whole stands about even to pass 6,000 and stop mid-build.
(a) Build it whole as planned: one review, one merge. The stop past 6,000 stays; if it is hit, you get three lines then.
(b) Split it in two, each with its own review and merge: U4h, lag compensation (hits as the shooter saw them, the
    rewind cap, the two rewind cheats: about 2,100 to 3,850 lines; stop past 6,000), then U4h2, aiming at height,
    thrown projectiles, the cover record and the busy-scene hit rule (about 750 to 1,500 lines; planned about 1,150,
    stop past 2,300). No pass mark, judge rule or marks.json text changes, and no run is dropped.
The cheaper option is (b): one more review and merge, and no mid-build stop waiting on you. The build starts with the
lag-compensation half either way, so nothing waits for this answer until that half is done.
My lean, and the advisor's: (b).
```
```text
U4H-JUDGE: the judge gains the computations U4h's tests need, and one fix. marks.json already names figures the judge
cannot compute yet: the share of shots that hit in the shooter's view and that the server registered; the share of
view misses the server counted as hits; whether the later rule cap equals its rule; and the thrown projectiles whose
flight differs from the server's. Their formulas are written in the series file (marks/SERIES.md), with the parts for
the later network runs, where a difference seen only on the Arm load machine is reported and decides nothing; those
parts are built now too, tested on made-up logs only. The same change fixes JDG-ONLY-1: the judge's option that
narrows a run to some marks can skip a precondition mark (today a rule of use covers it). This changes the judge's
code, not marks.json, a pass mark or a number. Each computation gets pass, fail and gap test files, and the
test-engineer and the rust-reviewer read every judge line.
(a) Yes, built inside U4h, after the rest of its build (its first half then reaches about 7,100 lines at U4f's 1.5,
    past your 6,000).
(b) Yes, as its own small change (the judge only, about 400 to 900 lines; stop past 1,300), reviewed and merged before
    U4h's judged runs. It shares no file with U4h, so it need not wait for U4h's build.
(c) No: the hit-registration and throw checks stay not-run at this stage, and JDG-ONLY-1's fix comes back to you
    alone.
My lean, and the advisor's: (b). Only U4h's judged runs wait for it.
```
- Nothing else is his: no money, nothing on main, no run day (all runs are L1 in this container).

## 9. For the main session before launch
- **The register: DONE 2026-11-21** (u4s-carries.md:618-659, entries 94-101, copied word for word from
  advisor-u4h-brief.md:441-482 by add_u4h_register.py; the list below is what they cover):
  1. N2-L3 (b), "U4h must blend composed boxes or interpolate hull yaw wrap-aware" (u4d-round2-netcode.md:81-82; routed
     by advisor-u4d-close.md:305-306); met by the shared blend-and-cast.
  2. INF-U4R-6 with INF-R2-2 (advisor-u4r-r1-full.md:289; advisor-u4r-r2-full.md:45, its base and route words); met by
     U4H-Q10 as ruled.
  3. `pump_report`'s RTT (advisor-u4e-r1.md:662-664), with block 4's site and U6's TickLoop line.
  4. EA-U4R-4 (advisor-u4r-r1-full.md:185) and EA-B1-1's later half (u4r-report.md:1164-1165), "the next lane that
     opens crates/sim": U4h's if U4g's report does not list them.
  5. SERIES.md's everyone_in_view writer cell, now :396 (U4r build ruling 3 (e), history.md:4976): U4h's SERIES.md edit.
  6. `cross_arch_mismatch_count`'s shot and throw kinds (SERIES.md:265, "U4h"): the judge's change (J2).
  7. JDG-ONLY-1 (history.md:4611-4614): the judge's change (J3; U4H-Q12).
  8. ADV-U4HB-1, TC's weapon: a carry line of its own, so U4h2 (if split) and R3's TC runs inherit it.
- **After the close (rule 1's list):** -48's report row "the run's largest measured rewind, rounded up" (marks.json:9413)
  predates T14's calibration; its wording is marks.json text, his. Beside it (Q7 on rev 2): -49's frame-wait report
  reads process `room` (marks.json:9961-9969), which on L2 no room can write: marks.json text, his.
