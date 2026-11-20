# U4h's brief, rev 1 (2026-11-20; for the advisor's check before the build)
**The unit:** lag compensation and aim, u4-build-plan.md:903-918 (plan rev 3.3; size M). It builds after U4g, or after
U4g2 if U4G-Q1 splits U4g, and after that unit's judged sets (one lane at a time: :951-952). Short by TEST-FIRST rule 3
(history.md:2264-2267; his message :2236-2282): what U4h builds, how it is tested, what is open; the rest is pointed
to. U4f's brief stays the reference for the start and stops (u4f-brief.md section 1, :288-355), the build rules
(section 4, :831-958), the pre-runs, judge calls, disk and bars (5.3-5.6, :997-1162), the end checks (section 6,
:1163-1183) and the return list (section 7, :1187-1225). **Two questions are his, neither a stop now** (section 8):
U4H-SIZE (split or not) and U4H-JUDGE (npt-judge's four planned figures, a judge change under rule 2).

## 0. Where things are
- **`<base>`:** the spike branch's head at launch, after U4g's (or U4g2's) merge and its sets, filled by the main
  session in U4f's fill form (u4f-brief.md section 12, :1724) with its spike tree and its main-folder check.mjs counts;
  every count here is a delta on it. Every "there now" below was read at 33dabe8, whose spikes/ equals U4f2's base
  a289def (`git diff --stat a289def HEAD -- spikes/netcode-proof`: empty). U4f2, U4g and U4g2 open crates/room,
  client, bots and lab before U4h, so those lines move and are re-read at `<base>`.
- **Read, never retyped** (stamps refilled at `<base>`): u4-build-plan.md (§4.2 :223-241, §4.3 :263, §4.9 :374-389,
  §4.14 :498, §4.15 :527-532, §5 U4h :903-918, §9 :1031-1035); plan.md (E11 :470, E12 :471, the PM rows :724, :729,
  :742-746); u4s-carries.md (U4h's section :609-651); marks/marks.json (-7 :2646, -48 :9137, -49 :9441, -50 :9986, -51
  :10111, -54 :10626, -56 :10745); marks/SERIES.md; u4f-brief.md; u4g-brief.md (rev 2.1); advisor-u4g-brief.md;
  u4r-report.md; docs/adr/0001-multiplayer-netcode.md (PM-0001-50's row :1647).
- **One cite drift for the main session's next plan edit (Low):** u4-build-plan.md:125 and :152 cite plan.md:696 for
  -7's sizing; the plan has grown and the line is now plan.md:724.

## 1. What U4h builds
### 1.1 What it needs from U4g, and what U4G-Q1 changes
- **Needed:** `Interp`'s published interpolated movers and its render time (AimCast's set, the TC shooter's view test
  and the view time it stamps: u4-build-plan.md:904-906, §4.8 :369-372); the ClientSettings send and the room's clamp
  (the announced delay §4.9's estimate reads: :382-383; u4g-brief.md:58); the delay's 2-to-5 adaptation, which sets the
  honest rewind the calibration measures. **Not needed:** `Predictor` and `CueLog` (the TC shooter stands at its spawn,
  scene.rs:262, so its fire origin needs no prediction).
- **Under U4G-Q1 (a)** (advisor-u4g-brief.md:360-374, unanswered): U4h launches after U4g's merge and its sets.
  **Under (b):** `Interp`, the ClientSettings send and the clamp land in U4g2 (u4g-brief.md blocks 7-8, :86-93), so
  U4h launches after U4g2's merge and its sets; U4g's prediction half alone is not enough. Either way the calibration
  (section 2) runs on the final `Interp`: a later change to its delay rule re-runs the calibration before any set counts.

### 1.2 The lane
- **Open:** crates/room; crates/server (the pump hands the room the RTT; its tests); crates/client (AimCast; the
  flight's call and -54's site); crates/sim (-51's bite site only, in `CHECK_BITES`' form); crates/proto (at most one
  public item: the aim's widths, private today at proto/src/input.rs:53-54); crates/bots; crates/lab (its
  det-recordings.json included); crates/judge (**only after his U4H-JUDGE yes**); ops/runs (new c-u4h-*.mjs files;
  det-u4.mjs's print, INF-U4R-6); marks/SERIES.md (new rows; :86 only by U4H-Q7's ruling).
- **Shut:** l1, link, net, log, client-wasm, hash-wasm; marks.json, marks/README.md, every other existing SERIES.md row;
  ADR 0001 and the catalogue (a word that changes goes on the readings-PR list); check.mjs; Cargo.toml, Cargo.lock. A
  file outside the lane stops the build for the main session.

### 1.3 What is there and what is new (read at 33dabe8)
| Item | There now | New in U4h |
|---|---|---|
| The ring | `HitHistory::for_ms`, at most 220 ticks or 7,333 ms (sim/src/history.rs:23-39); the room's private `HISTORY_MS` 2,000, provisional (room.rs:33, :533-550); the lab's copy `ROOM_HISTORY_MS` (scenario.rs:78-80) | its span from the test cap (U4H-Q1, Q2); carries 28 and 89 |
| `rewind_at` | the integer half: the claimed view time or `None`, counted as `rewinds_refused` (room/src/rewind.rs:13-33; room.rs:1359-1368) | the estimate, the tolerance, the switchable cap, each replacement and clip counted; the per-shot log |
| The RTT | `Decoder::rtt_ns` by `RttSource` (server/src/pump.rs:183-190, :336-344); the room reads none (no `rtt` in room/src); `pump_report` carries bytes and the largest datagram only (room.rs:877; advisor-u4e-r1.md:662-664) | the pump's RTT into `pump_report` (A6, u4-build-plan.md:89) |
| The ray | `World::ray_cast(ray, movers)`, a non-finite ray a Miss (world.rs:1092-1102); the server's blend by box centre (world.rs:209-223) | AimCast on `Interp`'s set (carry 45; N2-L3 (b) below) |
| Throws | the closed-form `trajectory_at` (sim/src/trajectory.rs:13), two projectile types (world.rs:111-113), `PRESS_THROW` (seam.rs:68) | the client's flight and its landing compare |
| TC | the scene: shooter, sprinting target, three plates (scene.rs:252-265; lab scenario.rs:2047) | the suites; no TC run exists yet |
| Bots | the brawl, forgers, lag switch, never-acker; "the target-course shooter is U4h's" (bots/src/lib.rs:10) | the TC shooter, the target's sprint, the tolerance-edge stamper, the 150 ms pong delayer, the plate shooter, the thrower, the ES hit rule |
| Judge | the four figures `planned`, compute `None` (judge/src/figures.rs:841-860); formulas at SERIES.md:274, :289, :295, :299 | their computations and fixtures (U4H-JUDGE) |

### 1.4 Blocks (each compiling with its tests passing before the next; at most about 400 added lines, a larger one split)
**The lag-compensation half** (U4h under either answer to U4H-SIZE):
1. **npt-room, the estimate:** RTT (from `pump_report`) + the announced delay (U4g's clamp) + the input-buffer wait,
   capped at the honest lead (the controller's padded target for that client: u4f-report.md:622-623), never the bound
   (carry 90) + half a tick (§4.9 :382-383); a session with no RTT yet by U4H-Q3 (carry 88).
2. **npt-room, the rules:** accept within one snapshot interval of the estimate, else replace and count, then clip to
   the cap and count (§4.9 :379-381); every run of a command judged at the tick it runs, reruns counted apart (carry
   81); the cap switchable, test or rule (§4.9 :388); the ring's span from the test cap plus its margin (carries 28, 89;
   U4H-Q1, Q2); the per-shot log of its parts (SERIES.md:206-216) with `event.shot.registered` (:411); -7's history
   records (`header.params.history_ticks_count`, `event.hit_history.size_bytes`); the sites -48/no-interp-delay,
   -48/no-buffer-wait and -50/no-tolerance-check (U4H-Q11).
3. **npt-server:** the pump hands each session's RTT to `pump_report`; npt-server's tests follow (end check item 4).
4. **npt-client, AimCast:** `World::ray_cast` on `Interp`'s published set at the view time it sends (the quantized
   tick and fraction), hull parts drawn as the sim blends them (N2-L3 (b)); a non-finite ray is a Miss (carry 45).
5. **npt-bots:** the TC shooter (a page-like 60 Hz frame loop with a seeded phase, posting input and view time each
   frame: plan.md:471; its aim by U4H-Q8; its view test `event.view_test.hit`); the target's sprint (9 m/s, seeded
   turns); the tolerance-edge stamper; the 150 ms pong delayer, which reports the hold an honest client would have,
   never the 150 ms, so the room's ping RTT rises by about 150 ms (ADR 0001:1647), with `event.pong_delayer.hold_ns`.
6. **npt-lab:** the TC suites (calibration with the cap opened; test cap; `--cap`, the cap and span parameters by
   U4H-Q2), `rewind-cheat` (the gains by U4H-Q4, the 1-second gap, `--transport ws` with `RttSource::Pings`: U4H-Q5),
   the header params, the writers, the calibration print, the contract cases (§4.12 :443-446); C-U6-19's line holds by
   construction.
7. **npt-lab and ops, the recording:** TC B1 seed 3 by `scenario --record` with the TC shooter into det-recordings.json
   (carry 87); det-u4.mjs prints its own non-cargo sum and "over 240 s: to the advisor" (INF-U4R-6; U4H-Q10).
8. **npt-judge (after his yes):** `view_hit_registered_share` and `false_hit_share` (joined by shot id),
   `rule_cap_set_by_rule` (fixtures only at U4), each with PASS, FAIL and gap fixtures. Without his yes the block
   waits and the build goes on to block 9; only the judged sets wait for it.
9. **ops/runs:** c-u4h-r2.mjs (the pre-runs and the calibration, the test cap computed) and c-u4h-runs.mjs (the sets),
   in c-u4f-runs.mjs's form. **If his U4H-SIZE answer is not in when block 9 is done:** the end checks run on the tree
   as it stands and the builder returns (the size stop's form); the main session goes on by his answer.

**The aim-and-throws half** (U4h2 under U4H-SIZE (b)):
10. **npt-sim and npt-proto:** -51/parallel-ray-10cm's site (U4H-Q11) and its unit proof; the aim's widths read public.
11. **npt-client:** the throw's landing through `trajectory_at` beside the server's; -54/step-1-60's site.
12. **npt-bots:** the plate shooter (ground, 2.5 m, 4 m), the thrower (both projectile types), the brawl's ES-only
    seeded hit rule (carry 93).
13. **npt-lab, npt-judge and ops:** the suites `aim-height` and `throws`; `--cover` (U4H-Q9); their writers and
    contract cases; `throw_desync_count` with fixtures (after his yes); carry 93's ES rows; the sets.
14. **The end checks** (section 3), after each half if split.

## 2. Marks moved and the runs (the main session's, after the merge, in the main folder)
| Mark (marks.json) | L1 parts (`min`) | Suite, scene | Measured | Bites (site; run) | What it reads |
|---|---|---|---|---|---|
| calibration (no mark) | P3, P4 x A, B on seeds 101-103 (T14, :74, :910-912) | TC, the cap opened, 2,000 shots | 12 | none | the largest honest rewind per profile and option; the test cap computed |
| -48 (:9137) | P0-P3 x A, B (3), `params.cap` test | TC, 2,000 shots a run | 24 | no-interp-delay (P2, seed 1); no-buffer-wait (P3 B, seed 1: -20's buffer median 2 ticks, u4f-report.md:622; the shooter's own depth printed, below 2 to the advisor: §4.14 :498) | registration >= 0.99 to P2, >= 0.95 at P3; false hits < 0.005; setup `clipped` 0 |
| -49 test cap (:9441) | P0-P4 x A, B (3) | -48's 24 runs list -49 too; P4 apart | 6 | cap-estimate-only waits for R3's rule cap (marks F24) | `over_cap` 0; parts PASS unbitten, the mark NOT-RUN on its bite |
| -50 (:9986) | P0, P1 (3); the ws leg (1) | `rewind-cheat` (U4H-Q5) | 7 | no-tolerance-check (P1, seed 1; its setting by Q5) | both cheats' 1,000 shots, gains <= 33.33 ms; the gap >= 145 ms each second; ws reported |
| -51 (:10111) | TC P0-P3 (3) | `aim-height`, 1,000 shots a height | 12 | parallel-ray-10cm (P0, seed 1) | each height's 5 cm share >= 0.999; aim <= 80 bits (44 today: input.rs:310) |
| -54 (:10626) | L1 (3) | `throws`, 1,000 a type (U4H-Q6) | 3 | step-1-60 (seed 1) | `throw_desync_count` 0 |
| -56 (6), (10) (:10745) | none at U4 | from -48 and -49 (P2-P4 per option); (10) by U4H-Q9 | 0 (or 3 under Q9 (b)) | none | registration and honest rewind per option in runs.txt and the report |
| -7 (:2646) | none at U4 (L2, R2) | every U4h run records the span | 0 | none | `history_ticks_count` recorded for R2 |

- **Counts:** 52 measured, 5 bites and 12 calibration runs (§9 :1031-1035), the TC B1 seed 3 recording's run apart.
  Under (b), U4h holds the calibration, -48, -49 and -50 (37 measured, 3 bites), U4h2 -51 and -54 (15 measured, 2
  bites). L2, ML and rule-cap parts stay NOT-RUN until their run days.
- **Beside them:** carry 88's -30 pre-check re-run (six runs, results/u4h-precheck-30/; the main session's); carry 93's
  ES measurement (P2, seeds 1-3, a 10-minute window, per candidate value) and its SS figure-for-figure check.
- **The pre-runs** (c-u4h-r2.mjs, before the sets): each new suite short with its contract case; a short P3 B TC run
  printing the shooter's buffer depth; a short P0 pong-delayer run printing each second's gap; a short P1 edge-cheat
  run printing its gains; the calibration set, then the cap computed and written into the sets' arguments. A row other
  than its `expect` comes to the advisor before any set.
- **Judge calls:** two per set (u4f-brief.md 5.4, :1053-1079); no U4h mark carries `pre:PM-0001-2`, so JDG-ONLY-1's
  interim rule does not apply. A clip in a judged run's setup, a gain above 33.33 ms, or a judged run's registration
  below its bar comes to the advisor before its set counts; no run is re-run into a pass.
- **Disk and time** (estimates; the runner projects at launch):

| What | Runs | Wall a run | Logs a run | About |
|---|---|---|---|---|
| TC sets, calibration and bites (1-2 clients) | 69 | no TC run yet; 10-60 s (SS's 48 clients took 59-62 s for 3,900 ticks: results/u4f/b57221ed139d/pm-0001-47/runs.txt:17-19) | 2-10 MiB (unmeasured) | 0.3-1.2 h; 0.15-0.7 GiB |
| carry 93's ES runs (48 clients) | 6-9 | 320-390 s (-20's 10-minute runs) | 62-210 MiB | 0.6-1.0 h; 0.4-1.9 GiB |
| carry 88's pre-check pairs | 6 | under a minute | about 17 MB a folder (history.md:6528) | 0.1 h |

  **The build's disk:** about 10.1 GB free after a part-clean (history.md:7344-7353), the runner's floor 4,096 MiB,
  and a full check.mjs run fills the shared target by about 6.3-7.1 GiB (dev) and 0.26 GiB (release)
  (history.md:7336-7352), so every counted check.mjs run meets the floor: one part-clean before each (P27's second
  form, no lane running, under the lock: history.md:7330-7338), as U4f2's four. **The sets:** about 0.6 to 2.7 GiB of
  logs in all, run with the shared target cleaned; D2 at launch; a set past the floor waits for a pack of a closed set
  (c62h-bound-brief.md 8.1 :485; KEPT-LOGS). The judge's peak stays far under 10,240 MiB.

## 3. How it is tested
| What | The test | Its plant, failing the test by name |
|---|---|---|
| the estimate | its four terms from a hand-made session; a stamp-ahead under option B buys no buffer wait past the honest lead (carry 90) | the buffer-wait cap removed |
| the tolerance and cap | inside one interval accepted; beyond replaced and counted; over the cap clipped and counted; `over_cap` 0 | -48's two bites; -50's bite |
| reruns (carry 81) | a rerun judged at its own tick; fire held with the uplink starved buys no rewind | reruns judged at the claim's tick |
| no RTT yet (carry 88) | a shot before the first sample by U4H-Q3, counted | the claimed view time taken |
| the ring (carries 28, 89) | the span is the cap plus its margin, in the header; hash.rs's span check (:370-375) reads the room's own value | a span copy that differs |
| AimCast (carry 45, N2-L3 (b)) | on a fixed trace, the client's cast at its sent view time equals the server's blended cast at that Rewind, hull parts included; a non-finite ray a Miss | the cast at the unquantized fraction |
| the TC shooter | 60 Hz frames with a seeded phase; the frame-to-tick wait in [0, one tick); same seed, same commands | an unseeded phase |
| the pong delayer | the gap at least 145 ms each second on a short P0 run | the hold reported as 150 ms (the room then subtracts a tick) |
| the calibration | the printed maximum per (profile, option); the cap the script computes equals ceil + margin | a cap rounded down |
| the figures | PASS, FAIL and gap fixtures each; an unjoined shot id a gap; 0 view misses a gap | a join by tick, not shot id |
| -51 | each height's server hit point within 5 cm of the picked point | the bite |
| -54 | each type's client landing equals the server's | the bite |
| the recording | TC B1 seed 3 regenerates to its pinned SHA-256; det-u4.mjs's sum printed | a changed seed |
| the contract | every new suite's checks resolve, every name has its row | a renamed series |
- **Plants:** every new check and bite site fails by name under a plant, restored with the file's sha256 (u4f-brief.md
  :1202-1204). Counts as deltas on `<base>`'s; negative cases counted; the report states its own.
- **End checks:** u4f-brief.md section 6, items 1 to 5 (:1165-1182), item 4 included (npt-server and its tests change).
- **The judge's read:** every crates/judge line gets the test-engineer's and the rust-reviewer's read in U4h's round,
  named in the report in its own line with marks.json's sha256 before and after (his 13:35 rule, history.md:2127).

## 4. The carries, by number (met by number in U4h's report)
- **U4h's section of u4s-carries.md (:609-651):** C-U6-19's line (:610-611): **U4h's runs hold no throwing or
  rejoining hostile client and no client restarted mid-run** (-50's cheats only stamp view times or hold pongs; -54's
  thrower is honest). 93, the brawl's ES hit rule (:612-617); 28 (:618-619); 45 (:620-622); 81 (:623-628); 87 as
  amended (:629-635); 88 (:636-643; its rule by U4H-Q3); 89 (:644-647; by U4H-Q2); 90 (:648-651).
- **Found outside the register** (ADV-U4GB-1's lesson; one pointer line each goes into u4s-carries.md's U4h section at
  the main session's next carries edit):
  - **N2-L3 (b)** (u4d-round2-netcode.md:81-82; named for "the U4g and U4h briefs to take": advisor-u4d-close.md:305-306;
    u4d-report.md:64): "U4h must blend composed boxes or interpolate hull yaw wrap-aware". Met in block 4 and section 3.
  - **INF-U4R-6 with INF-R2-2's base** (u4r-report.md:154-157, :283; advisor-u4r-r1-full.md:289;
    advisor-u4r-r2-full.md:45): det-u4.mjs's own non-cargo sum and the 240 s line, wasmStep's default print unchanged;
    at U4r's merge 223.7 s of 240, and "Adding a runner entry ... will cross 240 s, so U4h's plan settles the route,
    RR-L4 weighed first, before recording". RR-L4 acts on the Wasm leg alone (advisor-u4r-build1-full.md:99). U4H-Q10.
  - **`pump_report`'s RTT** (advisor-u4e-r1.md:662-664): block 3.
- **Already taken elsewhere:** N2-1, EA-U4R-1 and -2 and RR4-L1 (BRAWL-1's Part 1: u4r-report.md:275-280); npt-server's
  tests compiling (U4f: advisor-u4f-fold-check.md:34; still end check item 4); the 1 MB bound (carry 43: U4d's, :710).
  RR-5's encode-path wins (advisor-u4e-r1.md:211-217) are not taken: U4h does not edit the encode path.

## 5. The rules that stand
- Start and stops: u4f-brief.md section 1 (:288-355), read for U4h: the block-end size stop is the one his U4H-SIZE
  answer sets (past 6,000 whole; under (b) past 6,000 for U4h and past 2,400 for U4h2), by advisor-u4f-build1.md:178-181's
  form; counts are deltas on `<base>`'s; the test step's stop is 1.5 times `<base>`'s measured test step (TE31-10,
  u4f-brief.md:326), written in the fill form at launch.
- U4f's build rules (section 4, :831-958) with 1.2's lane; the shared target and its lock; one builder in an isolated
  worktree, Opus 5.5 at xhigh; at most three agents at a time; never `cd` into its worktree.
- **No pass mark, judge rule, marks.json text or existing SERIES.md row changes, and no run is dropped.** The judge's
  four figures only on his U4H-JUDGE yes; a build that needs any other such change stops, and the main session asks him
  (history.md:2262). LEAD-BOUND (a), 400 ms, frozen (history.md:6791-6806).
- Every agent brief carries, word for word: "Write files with the Write and Edit tools only (no sed, no shell
  redirect, no python writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit
  tool rewrites its notes file whole with Write."

## 6. Size (SIZE-CHECK, history.md:2048-2051: M is about 3,000 added lines, the stop past 6,000)
| Item (added lines in spikes/) | Lag compensation | Aim, throws, cover, ES |
|---|---|---|
| room: the estimate, the rules, reruns, no-RTT, the span, the shot log, three sites, tests / the plates' error writer | 550-950 | 60-120 |
| server: the RTT hand-off and its tests | 40-120 | 0 |
| client: AimCast and its tests / the flight, -54's site, tests | 120-250 | 100-200 |
| sim and proto: -51's site and proof, the aim widths | 0 | 45-120 |
| bots: TC shooter, sprint, edge stamper, pong delayer, tests / plate shooter, thrower, ES hit rule, tests | 350-600 | 150-300 |
| lab: TC suites, calibration, rewind-cheat, ws, params, writers, recording, contract, tests / two suites, cover, writers, ES rows | 600-1,000 | 300-550 |
| judge: three figures and fixtures / `throw_desync_count` | 200-400 | 60-150 |
| ops: both scripts, det-u4's print / their rows | 170-300 | 60-120 |
| SERIES.md new rows | 5-20 | 5-15 |
| **Each half** | **about 2,050 to 3,650** | **about 800 to 1,600** |
- **Whole: about 2,800 to 5,200** (midpoint about 4,000): past M's 3,000 at its midpoint. U4f's whole scope ran about
  1.5 times its estimate, its client and room about 2.0 (advisor-u4f-stop.md:56-57, :101-102; u4f-report.md:314-315):
  at 1.5 about 4,200 to 7,800, so built whole it stands about even to pass 6,000 and stop mid-build. Split: U4h up to
  about 5,450 at 1.5 (under 6,000); U4h2, planned about 1,200, up to about 2,350 (its stop past 2,400). The room, the
  largest item, is where U4f ran 2.0. These are judgements on the code's shape, not counts. Over 3,000 but under 6,000
  is no stop by his words; U4H-SIZE is his because a split is a plan change (advisor-u4g-brief.md:188-193).

## 7. Reviewers and rounds
netcode-architect, security-auditor (the rewind's bounds, the cheats, the RTT source), test-engineer and rust-reviewer
(u4-build-plan.md:918), each at xhigh, in two batches (at most three agents at a time); at most two rounds, a third
only for a Critical or High; under (b), one panel per half. The judge's lines are read by test-engineer and
rust-reviewer inside the round (section 3).

## 8. Questions
**For the advisor** (each inside plan rev 3.3, undoable, no pass mark, no money, nothing on main; my lean last):
1. **U4H-Q1, the margin** (T14: the test cap is the calibration's largest honest rewind rounded up to ticks, plus a
   margin, as data; the ring holds the cap plus one tick for the blend). (a) 2 ticks; (b) 3 ticks (100 ms: one step of
   the delay's adaptation, one tick of frame wait, one tick of tail beyond seeds 101-103); (c) 20 % rounded up. The
   setup check `clipped` 0 can still fail under each. **Lean (b)**: about 9 KB more ring, far under -7's 1 MB.
2. **U4H-Q2, where the cap and span live.** (a) Run parameters set by c-u4h-r2.mjs from the calibration, recorded in
   every header (`rewind_cap_ticks_count`, `history_ticks_count`); `HISTORY_MS` stays the default for SS, ES and CO, made
   readable as carry 89 asks, and no existing runner entry is re-pinned; R2's run plan takes the value for -7. (b)
   Constants set by a main-session commit after the calibration (carry 89's form), re-pinning the runner entries, with
   its own read. **Lean (a)**: no code commit between the calibration and the sets.
3. **U4H-Q3, no RTT yet (carry 88).** (a) The shot runs at the present (no Rewind) and is counted apart, as a
   replacement in -30's setup condition (carry 40); the measured window holds none (the contract test shows it). (b)
   RFC 9002's initial 333 ms stands in. (c) The claim is taken, clipped to the cap. **Lean (a)**: it buys nothing and
   shows; (c) would give a fresh session unchecked rewind.
4. **U4H-Q4, -50's "honest twin shot at the same RTT"** (marks.json:10108). (a) The estimate's own rewind for that
   client at that tick (an honest client at the same RTT and announced delay), so the gain is the cheat's rewind minus
   it; (b) a paired honest slot's shot on a link of the same profile; (c) the same client's honest stamp from the same
   frame, run through the room's rule. Under (b) and (c) an honest view can lead the estimate by up to the half-tick
   frame wait plus jitter, so a correct room could read about 50 ms or more against the 33.33 ms bar. **Lean (a)**,
   with (c)'s figure printed beside it (reported). If you read (a) as changing the mark's meaning, it goes to him.
5. **U4H-Q5, -50's runs.** Scene: TC, slot 0 the tolerance-edge stamper firing at slot 1, slot 1 the pong delayer
   sprinting and firing at slot 0 (no scene change). The bite run: the stamper's shots alternate at the edge and two
   intervals beyond it (a correct room replaces the beyond-edge ones; under the bite they gain about 100 ms); the
   measured runs stamp at the edge only, as the ADR row says. The ws leg: P1 (the row names no profile), every session
   on `RttSource::Pings`, reported only. **Lean: all three as written.**
6. **U4H-Q6, -54's types:** both projectile types (stand-in and grenade, world.rs:112-113), 1,000 throws each. **Lean:
   yes**; -54 is conditional on live projectiles, which R-11 rules met.
7. **U4H-Q7, the pong gap's sign.** SERIES.md:86 reads "transport RTT minus its ping RTT"; ADR 0001:1647 and the
   check (min >= 145 ms) need the ping RTT minus the transport RTT, since the delayer inflates the ping RTT. (a) The
   writer logs ping minus transport, and :86's cell is corrected in U4h's lane with test-engineer's read (no check,
   number or marks.json text changes); (b) the cell stays, the reading noted, on rule 1's after-close list. **Lean (a).**
8. **U4H-Q8, the TC shooter's aim.** `false_hit_share` needs view misses: aimed at the target's centre, nearly every
   shot is a view hit and the figure has no denominator. (a) Centre only; (b) a seeded offset straddling the box edge;
   (c) half at the centre, half straddling the edge (about 500 near misses a run, so the bar allows 2). **Lean (c).**
9. **U4H-Q9, R-10's cover item (-56 (10)).** (a) The sim evaluates each rewound shot under both variants, the other's
   result in `reports()` (sim opens wider; -7's timed query must not include it); (b) `--cover` per run, three recorded
   TC P2 runs under "present" beside -48's under "rewound", each printing how many shots' rays met cover; if none
   does, the scene comes back to you. **Lean (b).**
10. **U4H-Q10, det-u4.mjs's 240 s line.** (a) Record TC B1 seed 3, then one det-u4.mjs run with `uptime` first; over
    240 s comes to you with the figure; (b) a shorter TC recording. RR-L4 cannot buy back the lab leg. **Lean (a)**: TC's
    few clients make the entry cheap to regenerate (estimate, unmeasured).
11. **U4H-Q11, the bite sites:** -48's two and -50's in npt-room's estimate and tolerance; -51 in npt-sim's shot phase
    in `CHECK_BITES`' form, apart from `World::BITES` so npt-hash-wasm's index does not move, armed in the authority's
    World only (as U4G-Q5's -55); -54 in npt-client's flight call (`npt_client::BITES`); no id in two crates' lists
    (carry 24). **Lean: as listed.**
12. **U4H-Q12, JDG-ONLY-1** (history.md:4611-4614; its home npt-judge's next change, as its own change-set: the card's
    Next steps). U4h's figures would be that next change. **Lean:** JDG-ONLY-1 goes first as its own small change-set,
    its text in the same day's message as U4H-JUDGE; U4h's judge block follows it.

**For Mustafa** (his under TEST-FIRST: rule 2's judge line and the plan's size stop; paste-ready):
```text
U4H-SIZE: the size of unit U4h (lag compensation and aim). It is planned as a medium unit (about 3,000 lines; your
stop at 6,000). Our estimate is about 2,800 to 5,200 lines. U4f, the last medium unit, came to about 1.5 times its
estimate in all, so U4h built whole stands about even to pass 6,000 and stop mid-build.
(a) Build it whole as planned: one review, one merge. The stop past 6,000 stays.
(b) Split it in two, each with its own review and merge: U4h, lag compensation (hits as the shooter saw them, the
    rewind cap, the two rewind cheats: about 2,050 to 3,650 lines; stop past 6,000), then U4h2, aiming at height,
    thrown projectiles, the cover record and the busy-scene hit rule (about 800 to 1,600 lines; planned about 1,200,
    stop past 2,400). No pass mark, judge rule or marks.json text changes, and no run is dropped.
The cheaper option is (b): one more review and merge, and no mid-build stop waiting on you. The build starts with the
lag-compensation half either way. Nothing waits for this until U4g is merged and its runs are done.
My lean: (b).
```
```text
U4H-JUDGE: the judge gains four computations. marks.json already names four figures the judge cannot compute yet:
the share of shots that hit in the shooter's view and that the server registered; the share of view misses the
server counted as hits (out of the view misses, as the series file defines it and -48's note calls a reading); whether
the later rule cap equals its rule (tested on made-up logs only now); and the thrown projectiles whose flight differs
from the server's. Their formulas are written (marks/SERIES.md lines 274, 289, 295 and 299). Building them changes the
judge's code, not marks.json, a pass mark or a number. Each gets pass, fail and gap test files, and the test-engineer
and rust-reviewer read every judge line.
(a) Yes: built in U4h, after the rest of its build.
(b) Yes, but as its own small change before U4h's runs.
(c) No: -48's hit checks and -54's check stay not-run at this stage.
My lean: (a). Only U4h's judged sets wait for it.
```
- Nothing else is his: no money, nothing on main, no run day (all runs are L1 in this container). If the advisor reads
  U4H-Q4 or Q7 as a mark's meaning, that text joins the same day's message.
