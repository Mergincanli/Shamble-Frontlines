# Re-plan mark table under NPT-TRIM (test-engineer, 2026-11-21; read only, no build)
Plan (block 0): list the 81 marks (marks.json, node one-liner); place each by plan.md §6.2/§7.2/§12.1 and the U4-U6
briefs; read judged results from the set reports; then blocks 1-6.
**Resume here:** blocks 1-6 all DONE (table complete through PM-0006-27). Nothing left but the hand-back.
Rule: NPT-TRIM, history.md:7561-7593 (KEEP :7569-7574; MOVE :7576-7581; the table and MOVED :7585).

## 1. The table (81 marks, marks.json order)
Key. Rev 3.3 column: unit; run (plan.md §6.2 line). U4 rows: U4r u4-build-plan.md:781-834, U4f :836-853, U4f2
:855-879, U4g :881-901, U4h :903-918; U5-U8 plan.md:1252-1255. Destinations: **CS6** Slice 1 change-set 6 Look, **CS7**
Lag, **CS8** Shoot, **CS9** Throw (docs/reports/2026-11-17-build-plan-slice-1.md:371-384, :386-397, :399-413, :415-419),
**S3** Slice 3 (transports, crate choice, lag compensation stay out until it: :895, :899), **S6** slice 6 on the real sim
(SPIKE_PLAN.md:633-635), **SB** the security baseline before the first online playtest (SPIKE_PLAN.md:631), **RP** the
real protocol and its dependency checks (history.md:7578). "Safari later" = BROWSER-SET (a) (:7581). ML parts are
judged only on the named laptop (R8; plan.md:717, :760-762); ML\* is rough, never judged. L1 = this container.

| Mark | In plain words | Rev 3.3 | Judged so far | NPT-TRIM | FLAG |
|---|---|---|---|---|---|
| -1 | Rig named, measuring unit holds | U1, U6; every run (:722) | not run | KEEP: U6, U8; R1, R2, R3, R5. Parts RD (with -43) and MAC MOVED: S3, Safari later | |
| -2 | The load is the stress scene | U4, U6; every run (:722) | L1 load only: SS PASS on BRAWL-1 and U4f sets (advisor-brawl1-sets-p0.md:25; u4f-report.md:599); ES FAIL 50.05 vs 10.5 events/s (u4r-report.md:308) | KEEP: all kept runs; ES needs carry 93 (F10) | F10 |
| -3 | Network profiles are what they say | U2, U6; C, R1, every L2 block, R7's MP (:723) | L1 P1-P4, B1 PASS (results/c-u2-pm-0001-3/81c6d01e371b/verdict.json); L2, MP NOT-RUN | KEEP: R1, R2, R3 probes. Part "MP at P2" MOVED with R7: S3 | |
| -4 | Server tick budget holds at 30 Hz | U6; R2 (:724) | not run | KEEP: U6; R2 (STAND-IN until S6, marks.json notes) | |
| -5 | Its two shares hold, each alone | U6; R2 (:724) | not run | KEEP: U6; R2 (server share holds the rewind queries) | |
| -6 | Snapshot encoding fits | U6; R2 (:724) | not run | KEEP: U6; R2 | |
| -7 | Lag compensation stays cheap and small | U4h span, U6; R2 (:724) | not run | KEEP: U6 + block 4's piece; R2 | F9 |
| -8 | Interest picks under the cap, in time | U6; R2 (:724) | not run | KEEP: U6; R2 | |
| -9 | Enclosure test fits, re-flood included | U6; R2 (:724) | not run | KEEP: U6; R2 | |
| -10 | Sync cost grows with what clients see | U6; R2 N=24, 36 (:725) | not run | KEEP: U6; R2 (its N=36 is R2's scale step, not R5's search) | |
| -11 | Transport runtime fits beside the tick | U6; R2 (:724) | not run | KEEP: U6; R2 (one crate: -56 item 1 narrows, F2) | |
| -12 | Downstream fits per client | U6; R2 ES, SS (:724) | not run | KEEP: U6; R2. ES part needs carry 93 (F10); -54's E3b may re-open it (F7) | F10 |
| -13 | Upstream fits per client | U6; R2 P2, P4 (:724) | not run | KEEP: U6; R2 | |
| -14 | No datagram is ever too big | U6, U7; R1, R2, R4, R7 (:726) | not run | KEEP: R2 (S30), R1 browser sessions. MAC part MOVED: Safari later | |
| -15 | A room's memory stays bounded, flat | U6; R2 (:724) | not run | KEEP: U6; R2 | |
| -16 | A room starts fast | U6; every run, first R1 (:727) | not run | KEEP: R1, R2, R3, R5 | |
| -17 | Every decoded snapshot is the server's | U4r, U6; C + R3 (:728) | L1 P0, P2, P3 PASS (advisor-brawl1-sets-p0-full.md:226, :233; history.md:3266); L1 B1 INVALID, ruled (u4r-report.md:545) | KEEP: L1 B1 by U4's B1 row (block 3); R2 (every S30); R3 L2 | |
| -57 | Decoded values keep protocol precision | U4r; C (:729) | PASS, L1 P0/P2/P3/B1 (u4r-report.md:543) | KEEP: done (L1) | |
| -58 | Every counter survives its wrap | U4g; C (:729; u4-build-plan.md:891-894) | not run | lean KEEP: L1 by U4's -58 row (block 3) | F1 |
| -18 | A loss burst heals | U4r→U4g, U6; C + R3 (:728) | L1 B1 INVALID, ruled (u4r-report.md:545) | KEEP: L1 B1 by U4's B1 row; R3 L2 B1 wt, ws | |
| -19 | Mid-match join or resync gets world fast | U6; R3 LG + R7 ML (:730) | not run | KEEP: R3 LG joins (after C-U6-11/-13/-16/-19). ML Chrome part MOVED with R7: S3 | |
| -20 | Commands arrive in time, count once | U4f, U6; C + R3 (:728) | L1 A and B PASS 3/3, 46ca143 (u4f-report.md:654-655); L2 NOT-RUN | KEEP: R3 L2 P3, both options | F13 |
| -21 | Forged commands, lag switches change nothing | U4f2; C (:729) | not run (U4f2 building) | KEEP: U4f2; L1 (-21's set) | |
| -22 | A never-acker cannot grow the server | U4f; C (:729) | PASS 3/3 (u4f-report.md:685-689) | KEEP: done (L1) | |
| -23 | A throttled link degrades by priority | U6; R3 (:731) | not run | KEEP: U6; R3 | |
| -24 | A bottleneck never stacks stale snapshots | U6; R3 (:731) | not run | KEEP: U6; R3 | |
| -25 | Hostile input contained on both transports | U6, U7; R4 (:733) | not run | MOVED: SB (R4, history.md:7579) | |
| -26 | A connection's panic ends only it | U6; R3 (:732) | not run | KEEP: U6; R3 | |
| -59 | A tick-thread panic ends match, process | U6; R3 (:732) | not run | KEEP: U6; R3 | |
| -27 | Rooms share a VM fairly | U6; R5 D1-D3 (:734) | not run | KEEP: U6; R5 D1, D2, D3 | |
| -28 | Each WebTransport crate works in every browser | U6, U7; ML\* loopback, R4 MAC-r (:735) | not run | lean KEEP: carried crate, ML loopback after U7; other crate MOVED: S3 (:895); Safari later | F2 |
| -29 | Fallback takes over when QUIC blocked | U6, U7; R4 (:736) | not run | lean KEEP: R1 (+0.25 h); Safari later | F3 |
| -30 | Both transports give the same server | U6; R3 + C loopback (:732) | pre-check recorded, not judged; NOT-RUN until U6 (u4-build-plan.md:827) | KEEP: L1 by U6d B3 + U6j J1 (u6-brief.md:35-36, :57-58); R3 | |
| -31 | Fallback's stalls stay short | U6; R3 LG + R7 ML (:731, :737) | not run | KEEP: R3 LG. ML Chrome part MOVED with R7: S3 | |
| -32 | One build runs four browsers, both backends | U7; R7 (:737) | not run | MOVED: CS6 measures both backends (:381-382), browsers online S3; Safari later. R7's; U7 draws nothing | |
| -33 | Client's RTT agrees with the server's | U7; R7 (:737) | not run | MOVED: S3 (needs WebTransport's rtt(); R7's) | |
| -34 | No decoder panics or over-allocates | U5; C (:738) | not run (U5 stopped, built kept) | MOVED: RP (:7578) | |
| -35 | Transport stays swappable and current | U5, U6; C (:738) | not run | MOVED: RP (:7578) | |
| -36 | Datagram size measured per browser | U7; laptop before R1, R1 (:739) | not run | KEEP: ML loopback legs after U7; R1 EU path. MAC-b part MOVED: Safari later | |
| -37 | Datagrams arrive in time at P2 | U6; R3 (:731) | not run | KEEP: R3 | |
| -38 | One-shots survive loss; bulk spares control | U6; R3 (:731) | not run | KEEP: R3 | |
| -39 | The 60 Hz margin | outside (:747) | outside | MOVED (already outside): S6's 60 Hz run | |
| -40 | The research's other 60 Hz marks | outside (:747) | outside | MOVED (already outside): S6 | |
| -63 | Goal budget at 60 Hz rooms per VM | outside (:747) | outside | MOVED (already outside): S6 | |
| -41 | Prediction exact when commands arrive in time | U4g; C + R7 (:740) | not run | MOVED: CS7 (:395) | |
| -42 | Corrections stay small in a crowd | U4g; C + R7 (:740) | not run | MOVED: CS7 (48 scripted clients: :392) | |
| -60 | Server change to own state predicted next snapshot | U4g; C + R7 (:740) | not run | MOVED: CS7 (movement), CS8 (fire timing: :404) | |
| -43 | Own moves, shots feel same at any latency | U7; R7 ML, RD (:741) | not run | MOVED: CS7 (:253), CS8 for shots; RD part when RD is named | |
| -44 | The replay fits the minimum laptop | U7; R7 (:741) | not run | MOVED: CS7 (the replay's cost in the Worker: :395-396) | |
| -45 | Remote soldiers drawn without guessing | U4g2; C + R7 (:740) | not run | MOVED: CS7 (interpolation: :388) | |
| -46 | Delay adapts; respawns never slide | U7; R7 (:742) | not run | MOVED: CS7 (delay), CS8 (respawns: :403) | |
| -61 | Remote events play with their poses | U4g2; C + R7 (:740) | not run | MOVED: CS8 (events: :403) | |
| -47 | The client's clock is right | U4f; C (:729) | PASS 3/3, a4cac39 (u4f-report.md:564-573) | MOVED: CS7 (clock estimate: :388), his list (:7577); L1 PASS kept as record | |
| -48 | Hits register as the shooter saw them | U4h; C + R3 test cap (:743) | not run | MOVED: S3 (:899) | |
| -49 | Rewind cap holds; clipping is known | U4h; C + R3 + R7 (:743) | not run | MOVED: S3 (:899) | |
| -50 | A cheat cannot buy rewind | U4h; C (:729) | not run | MOVED: S3 (:899) | |
| -51 | Aiming at height hits the clicked point | U4h; C + R7 (:742) | not run | MOVED: CS8 (hitscan: :402), lagged parts S3 | |
| -52 | Hit feedback waits for the server | U7; R1, R4, R7 (:742) | not run | MOVED: CS8 (hit marker on confirmation: :405), browsers S3. R7's; U7 draws nothing | |
| -62 | Own shot cues play once on replay | U4g; C + R7 (:740) | not run | MOVED: CS8 (own weapon state predicted: :404) | |
| -53 | Mustafa's verdict at P2 | R7 (:744) | not run | MOVED: S3 (R7, :7577) | |
| -54 | Client projectile flight holds, or E3b | U4h2; C + R3 (:745) | not run | lean MOVED: CS9 (shared trajectory: :416) | F7 |
| -55 | Soldier beside moving vehicle barely corrected | U4g; C (:745) | not run | lean MOVED: where vehicles enter (SPIKE_PLAN.md:629) | F8 |
| -56 | The proof test's records are complete | all; every run (:746) | not run | KEEP: every kept run; items 5-10 and item 1's pick move | F11 |
| PM-0004-67 | The server loop keeps time | U4b, U6; R2 (:748) | not run | KEEP: R2 | |
| PM-0006-11 | Reference server exists, ran every row | U6, U8; every run (:749) | not run | KEEP: R1, R2, R3, R5 | |
| PM-0006-12 | Round trip to the test servers | U8; R1 ping (:750) | not run (person) | KEEP: R1 (MP and ML pings) | |
| PM-0006-13 | 8 rooms at N = 48 on RS | U6; R5 D1-D3 (:751) | not run | KEEP: R5 | |
| PM-0006-14 | Rooms per VM counted, replaces A-2 | U6; R5, R6 (:752) | not run (reported) | KEEP: R5 N=48. Parts N=36, N=18 MOVED: S6 (:7580) | |
| PM-0006-15 | Firewall admits both transports, nothing else | U8; R4 (:753) | not run | lean MOVED: SB with R4 | F6 |
| PM-0006-16 | Metrics path works with one rule | U6; R5 D1 (:754) | not run | KEEP: R5 (probe from ML: named laptop) | |
| PM-0006-17 | Egress per match-hour matches R2 | U6; R2 (:755) | not run | KEEP: R2 | |
| PM-0006-18 | The spike's own hosting bill | after the last run (:756) | not run (person) | KEEP: after R5 | |
| PM-0006-19 | Logging costs the tick almost nothing | U6; R2 (:755) | not run | KEEP: R2 | |
| PM-0006-20 | Logging allocates nothing on hot path | U6; R2 (:755) | not run | KEEP: R2 | |
| PM-0006-21 | Logs keep no secrets or addresses | U6; R5 soak (:754) | not run | KEEP: R5 D1 soak | |
| PM-0006-22 | Every first metric exists and moves | U6; R5 soak (:754) | not run | KEEP: R5 D1 soak | |
| PM-0006-23 | A stall is visible at once | U6; R5 soak (:754) | not run | KEEP: R5 D1 soak | |
| PM-0006-24 | Metrics small, no player or match id | U6; R5 soak (:754) | not run | KEEP: R5 D1 soak | |
| PM-0006-25 | Dev certificate works in every browser | U6, U7; R4 (:757) | not run | lean KEEP: R1 (+0.2 h); Safari later | F4 |
| PM-0006-26 | Certificate rotates without dropping anyone | U6; R4 (:757) | not run | lean MOVED: SB with R4 | F5 |
| PM-0006-27 | Out-of-rule certificate is refused | U6, U7; R4 (:757) | not run | lean MOVED: SB with R4 | F5 |

## 2. Counts, and the flags
- **Kept (settled by NPT-TRIM): 48** (-7, -12 and -56 among them, each with a detail flag). **Moved (settled): 24**
  (21 by his list or with R7, R4, U5; 3 already outside: -39, -40, -63). **Flagged (status open): 9** (-58, -28, -29,
  -54, -55, PM-0006-15, -25, -26, -27). 48 + 24 + 9 = 81. **By my leans: kept 52, moved 29.**
- **Kept with some parts moved** (part-level MOVED needed, block 6): -1, -3, -14, -19, -31, -36, PM-0006-14, -56; by
  leans also -28, -29, PM-0006-25, -58. **Already PASS here:** kept -57, -22 whole, -3/-17/-20 in part; moved -47 (L1).
- **Kept marks per run (by leans):** R1 9 (+2 by F3, F4): -1, -2, -3, -14, -16, -36, -56, PM-0006-11, -12. R2 23: -1 to
  -17, -56, PM-0004-67, PM-0006-11, -17, -19, -20. R3 18: -1, -2, -3, -16, -17, -18, -19, -20, -23, -24, -26, -59, -30,
  -31, -37, -38, -56, PM-0006-11. R5 13: -1, -2, -16, -27, -56, PM-0006-11, -13, -14, -16, -21 to -24; PM-0006-18 after.
  L1 here 9: -3, -17, -57, -18, -20, -22 (done or part done), -58, -21, -30's C part.
- **F1 -58:** one check (corrections above 1 mm untraced, -41's count) and the lt-tick-offset bite read the Predictor
  (u4-build-plan.md:892-894), which moves. Keep it with those two marked moved, or move it whole? Lean keep: R5's runs
  cross every 16-bit wrap (65,536 ticks = 36.4 min at 30 Hz; the soak 0.75 h, -27's blocks 1.2 h: plan.md:822). U4G-Q2
  (his: -17's two lost-tick checks on -58, u4g-brief.md:249-257) still comes before -58's set.
- **F2 -28:** with one WebTransport crate (:7571), the other crate's parts? Lean MOVED to S3's crate choice (:895).
- **F3 -29** and **F4 PM-0006-25:** R4 rows that are not hostile input. Lean keep both in R1 (+0.25 h, +0.2 h: plan.md:821):
  the fallback is half of his question (:7567), and every WebTransport browser session rests on the hash-pinned dev
  certificate (plan.md:991-996).
- **F5 PM-0006-26, -27** and **F6 PM-0006-15:** lean MOVED to SB with R4: no run's VM lives near 14 days (no rotation
  needed), the 15-day refusal is a browser-rule control, and his source-limited firewall (:7571) is not the shape -15 checks.
- **F7 -54, F8 -55:** U4h2's and U4g's marks, not named in his list. Lean MOVED: -54 to CS9 (a kind that still desyncs
  there rides in snapshots as E3b and -12 runs again: ADR 0001:1653); -55 where vehicles enter (ADR 0001:1654: conditional).
- **F9 -7:** its history is sized from -48's test cap (plan.md:724; marks.json -7 notes), which moves. Lean: R2 measures
  at the room's default span, 2,000 ms (room.rs:32-33, :550), which bounds any rule cap at or under it; -7's notes
  reworded (marks.json: his, with a reviewer's read).
- **F10 -2 ES, -12 ES:** carry 93's ES hit rule is to be set "on U4h's build with lag compensation", before any ES run of
  -12 counts (u4s-carries.md:612-617). Lean: set it on the kept build, which already rewinds every firing command (block
  4); the advisor re-reads its carry's words.
- **F11 -56:** its check counts 10 items (marks.json -56); items 5-10 (smoothness, OQ-0001-6 with -42/-48/-49, -43, -44,
  Brave on MP, cover variants) and item 1's pick ride moved marks. Lean: those items marked moved, the count to 4; his.
- **F12 R8 (no row):** NPT-TRIM is silent on the named-laptop run; kept browser marks' ML parts stay NOT-RUN until it
  (plan.md:760-762). Lean: name the laptop before R1 if he can, so R1's browser legs judge; else R8 later, no Safari.
- **F13 OQ-0001-6:** the pick's inputs (-41, -42, -60, -48) move (plan.md:846-848). Lean: the pick moves to S3 (CS7
  builds both: build plan :389-391); R2 and R5 run option B (the draft's buffer: :909), stated in headers; R3's -20 both.

## 3. U4's kept rows ("about the wire and the server"); sizes are added lines, my estimates unless cited
- **(a) U4f2** whole: FP-2, its review, the merge, -21's set (:7570).
- **(b) U4g2 block 8's server part:** carry 78's reorder and the room's recovery after an eviction (target: -2's
  everyone-in-view at least 0.99 on B1, 10 bursts in 600 s, at most 18 flag-0 ticks per client per burst:
  u4g-brief.md:175-178); carry 91, ack.rs taking the stale copy when a tick becomes last_acked, one ring test and a
  short bitten B1 run with no heal within 500 ms (u4s-carries.md, carry 91). Not the ClientSettings clamp (it feeds
  Interp and U4h's estimate: u4h-brief.md:40-43). Priced 230-480 with the clamp (u4g-brief.md:223: recovery 50-120,
  ack.rs 20-50, tests): about 180-430 without it, up to about 860 at U4f's room factor 2.0 (u4g-brief.md:230-231).
- **(c) The B1 rows:** -17's and -18's L1 B1 parts and -18's no-fallback bite rode -45's extrapolation runs
  (u4g-brief.md:108), which move. **Carrier now: U4r's own judged B1 set** (SS 48, 10 bursts every 60 s; headers list -2,
  -17, -57, -18; the bites build at seed 1: ops/runs/c-u4r-runs.mjs:25-31), re-run at (b)'s commit, every judge call
  listing -2 (history.md:4998). Add the door-gap and "in a building" print (Q8: u4g-brief.md:96), the short B1 suite and
  short bitten run first (:117-124). About 100-250 (from the second half's lab 290-560 and ops 60-100, which also held
  two suites and Q9: u4g-brief.md:224, :226).
- **(d) -58 (F1):** the `wrap` suite, the press start in carry 75's form, the lab's `tick` record (carry 92), -58's three
  command counters (block 3, already in U4G-EARLY's patch: netcode-proof-test.md:158-161), its rows and set
  (u4g-brief.md:76-78, :81-82, :112). About 200-450 (one of block 5's four suites in a 680-1,180 half: :224); unsure.
- **(e) Carry 93's ES hit rule (F10):** the brawl's ES-only seeded rule in npt-bots, valued at P2 by the step-4 form
  (6-9 runs), frozen with its measurement (u4s-carries.md:612-617; u4h-brief.md:141-142, :174-176). About 60-150 (U4h2's
  bots 150-300 also held the plate shooter and the thrower: u4h-brief.md:260).
- **Not U4's:** -30's judged C part (U6d B3 + U6j J1; the pre-check is done: u4-build-plan.md:803-827); -7's writers
  (block 4). **Moved:** U4g's prediction half, U4g2 blocks 7, 9 and its two suites, U4h whole, the clamp; -54, -55.
- **Sum (b)-(e): about 540-1,280, about 800-1,900 at 1.5x: one S unit.** None is an R1 prerequisite (R1 judges none of
  -17, -18, -58, -12), so it can run in a lane beside U6: (b)+(c) before R3, (e) before R2's ES rows, (d) before R5.

## 4. Lag compensation's server cost
- **Yes, kept marks need it:** -7 is that cost (rewound shot against 48 candidates p99 at most 20 µs at 700 shots/s;
  history at most 1 MB: ADR 0001:1595); -4/-5 judge the tick with "lag-compensation queries" inside the server share
  (marks.json -5 notes; SERIES.md:49); -12's ES rows wait on carry 93, set on a build that rewinds (F10).
- **The smallest piece is already built:** the room keeps a 2,000 ms hit-history ring (room.rs:32-33, :550; at most 220
  ticks of at most 4,536 B: u4h-brief.md:69), gives every firing command its Rewind at the claimed view time
  (room/src/rewind.rs:18-33; room.rs:1089, :1106, :1331-1340), and the sim's step casts against the rewound volumes
  (room.rs:1039; u4h-brief.md:72). The brawl stamps its newest snapshot tick, fraction 0 (u4h-brief.md:44-45), so SS's
  700 shots/s all rewind. **Missing is the measurement only:** `tick.s.rewind_ns` (npt-server: SERIES.md:69),
  `event.hit_history.size_bytes` (npt-room: SERIES.md:166), the rewind time moved out of the sim share for -5, and the
  span in the header (F9); neither name has a writer in crates/ today (grep). About 80-250 lines with tests and plants,
  unsure; the npt-server writer likely sits inside U6c's S9 (400-800: u6-brief.md:34), and timing per shot needs a
  clock-free seam into the shot phase like S1b's (u6-brief.md:32).
- **Not needed:** U4h's estimate, tolerance, cap, AimCast, TC suites and calibration (u4h-brief.md:96-147): per-shot
  integer comparisons on top of the cast (:96-111). Not measured; say so in the re-plan.

## 5. R1, R3 and R5 after the trim (plan.md:818, :820, :822)
- **R1:** keeps bring-up 0.9, certificate and host keys 0.25, headers 0.15, L2 probe 0.2 (ImpairSocket's self-test, 0
  added: plan.md:1255), calibration 0.1, -16 0.1, the smoke page 0.25 (five browsers less Safari), ping 0.1, -36's EU
  path 0.5, first bites 0.1; cuts Safari runner conditions 0.3: **2.65 h; 3.1 h with F3/F4**; slack to 4 kept. Unsized
  still: DelayStream's loaded self-test (C-U8-3) and S14's veth + netem check (plan.md:1255).
- **R3:** keeps bring-up 0.9, two L2 probes 0.4, 3 clean references 0.6, VS4 whole (3.85: -17, -18, -19, -20, -23, -24,
  -31, -37, -38, bites 0.85), VS5's -26 0.55, -30 0.5, -59 1.2, bites 0.85. **"The rewind rows" = "-48 and -49's
  test-cap runs under both options 1.0"** (plan.md:820) and the option pick after them (plan.md:846-848, F13); -54 0.3
  goes too by F7. **8.85 h**, less the moved marks' unsplit share of VS5's 0.85 h of bites (the run plan sizes it).
- **R5:** keeps bring-up 1.0; D1 -27 1.2, N=48 search 2.6, 8-room soak 0.75, idle 1.45; D2 -27 1.2, N=48 count and
  count+1 1.5; D3 -27 1.2, N=48 count and count+1 1.5; teardown 0.55; cuts the N=36 search 2.9 and its time-of-day row
  (plan.md:835). **12.95 h of blocks, but RS-hours stay about 16.25 (billed 17)**: D3 must still start at 20:00 (three
  starts 6 h apart, one evening: plan.md:828-836), so D2's gap becomes 3.3 h idle unless the run plan stops RS in it;
  LG-hours drop (the search's extra CAX21s go: plan.md:859). DENSITY-FIGURE still first (plan.md:852-853).
- **R2** stays whole (plan.md:819). With Safari later, -14's cap is never final here (plan.md:739, :851): R2-R5 run at
  1,000 B; a later Safari reading below it re-opens -8/-12/-14 in the real game.

## 6. The MOVED status (smallest form; marks.json and npt-judge, so a reviewer's read: :7585; netcode-proof-test.md:41-42)
- **marks.json:** an optional `"moved": {"to": ..., "why": ..., "by": "NPT-TRIM, 2026-11-21"}` on a mark, on a part
  (8 kept marks, 12 by leans), and only if F1 holds on one check and one bite of -58; `verdict` (pass, person, reported)
  stays, so the checks travel as data. -56's count change (F11) is a check value, his. -39/-40/-63: map `outside` to
  MOVED (one arm; today it prints NOT-RUN "outside this change-set": eval.rs:1626-1630).
- **npt-judge:** marks.rs: the `moved` struct with deny_unknown_fields and non-empty `to`/`why`; refused on an outside
  mark, at mark and part level both, or on a part name not in `parts` (beside marks.rs:842-849's rules). eval.rs:
  `VerdictKind::Moved`, "MOVED" (eval.rs:1054-1080); a mark-level moved returns early like Outside, serving runs counted,
  not judged; moved parts, checks and bites are skipped; in the headline order (eval.rs:1655-1684) MOVED sits after
  NOT-RUN and before REPORTED/STAND-IN/PASS, so a kept FAIL, INVALID or NOT-RUN still shows and nothing moved reads PASS.
  runs.txt and verdict.json print MOVED with `to`.
- **Tests and fixtures:** all-pass runs plus `moved` read MOVED; part moved with kept parts passing reads MOVED; with a
  kept part failing reads FAIL; a kept part unrun reads NOT-RUN; an unrun moved bite does not hold the mark at NOT-RUN;
  compile refusals (empty to/why, unknown field, outside, bad part name); a property test: every passing fixture with
  `moved` added anywhere is never PASS; real data: results/u4f/b57221ed139d/pm-0001-47/ re-judged reads MOVED, -57's
  folder still PASS. Plants: the early return removed, and MOVED ranked above FAIL, each failing a fixture by name.
  check.mjs's self count (78 of 78: netcode-proof-test.md:129) changes as moved marks' bites drop out: stated.
- **Size:** about 300-600 lines of code, tests and README (marks.rs 60-120, eval.rs 50-110, output 20-40, tests 150-300,
  marks/README.md 10-20) plus about 60-120 lines of marks.json data; my estimate (U4h's three-figure judge change was
  priced 400-900: u4h-brief.md:265). **Review:** test-engineer and rust-reviewer, one round (:7590), named in its report
  in its own line with marks.json's sha256 before and after. **When:** before R1's judge calls (R1 reports -1, -14, -36,
  which carry moved parts); it gates no build.
