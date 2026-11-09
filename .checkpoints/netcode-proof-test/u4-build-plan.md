# U4 build plan, rev 1: the wire, the stand-in sim, the room, the client and the L1 runner

**Revision 1, 2026-11-09** (netcode-architect; branch claude/netcode-proof-test-plan-hcky5g, code as at f985226; the
folder's HEAD f2573f4 adds only checkpoint autosaves). Inputs: the three scouts, read in full
(u4-scout-arch.md F1-F48, u4-scout-marks.md F1-F32, u4-scout-netcode.md F1-F124), the approved U3 build plan
(u3-build-plan.md) and U3d plan rev 2 (u3d-plan.md) for the shape, and my own checks of the lines this plan leans on
(u4-build-plan-notes.md M1-M25). Next: the critics (test-engineer, engine-architect), the fold (rev 2), then the
advisor. **No code before the advisor's APPROVE.** References "arch Fn", "marks Fn" and "net Fn" are the scouts'
findings; "Mn" are my notes.

## 0. In plain words
- **What U4 is.** The first unit that builds the game-shaped parts of the proof test: the message format (npt-proto),
  a stand-in simulation (npt-sim), the room that runs commands and writes snapshots (npt-room), the client endpoint that
  predicts and interpolates (npt-client), the transport-free half of the server (npt-server's library), and npt-lab's
  scenario runner that drives 48 headless clients through L1 on simulated time. Then the L1 halves of the 21 marks the
  U4 row names (plan.md:1223, quoted in §1), each shown first by its planted defects ("bites") failing.
- **What it proves, and what it does not.** L1 is logic only: counts, shares and hashes, never a time mark
  (ADR 0001:1462-1463). So U4 proves that the protocol carries each client's server view under P0 to P4 and B1 (RTT 0 to
  200 ms, jitter ±0 to ±40 ms, loss 0 to 5 %, 1 s total-loss bursts: npt-l1 lib.rs:114-138), across every counter wrap;
  that forged input changes nothing; that prediction is exact when commands arrive in time; that hits register as the
  shooter saw them. It proves no timing budget: tick times, encode costs and bandwidth on real sockets are R2's and
  later (ADR 0001:164-184).
- **Why it is split.** U4 starts six crates from doc-only stubs (sim/src/lib.rs:1-5, proto :1-4, room :1-5, client
  :1-6, server lib :1-5; arch F9), changes npt-l1's stream model (C1'), and carries 52 planted defects (49 from
  marks.json, 2 from §4.5, 1 from the catalogue: net F101) and about 230 L1 runs (marks F31). That is too big to build,
  review or roll back as one change. **Nine sub-units** (§5): U4a the wire, U4b the loop policy, U4c the link model,
  U4d the stand-in sim, U4d2 the Wasm-in-Node hash, U4e the room and the codec marks, U4f clock and commands, U4g
  prediction and interpolation, U4h lag compensation. U4a, U4b, U4c and U4d do not depend on each other.
- **What needs a ruling first.** Seven questions change the structure (§10: U4-SEAM, U4-HIT, U4-WASM, U4-REPLAY,
  U4-SERVER, U4-LOSS, U4-CTL). U4a to U4c can start on the advisor's APPROVE of this plan whatever those say, except
  U4-LOSS for U4c's C3 part and U4-CTL for U4a's control types.
- **What it expects.** Every L1 part PASS, with every bite FAIL. A mark with L2, RS or ML parts stays NOT-RUN overall
  (marks F8, F35): U4's claim is "its L1 parts PASS and its bites FAIL", never "the mark passes". Two bites cannot fail
  on L1 at U4; they are carried by name (§6).

## 1. Requirements
| # | Requirement (quoted where short) | Source | Exists today | Where in U4 |
|---|---|---|---|---|
| RQ1 | "npt-sim, npt-proto, npt-room and npt-client; npt-lab's scenario runner; `due_ticks` with PM-0004-67's bites on a fake clock; the Node-hosted native-against-Wasm hash test (§4.5); then the L1 row groups with their bites: codec (-17, -57, -58); commands and hostile input (-20, -21, -22); prediction (-41, -42, -60); interpolation, clock and events (-45, -47, -61, -62); lag compensation (-48 to -51); bursts (-18); stand-ins (-54, -55); -30 on loopback" | plan.md:1223 | The crates are doc stubs (arch F9); npt-lab has `probe` only (lab/src/main.rs:1-3) | All of §5 |
| RQ2 | Layout: sim "no I/O, clock, transport or bot logic"; room "tick-counted timers only; no I/O, async runtime or clock; builds for wasm32 and native"; client "no bindings, transport or clock"; lab "the L1 scenario runner ..., the x86-64 re-decode and replay tool, the native-against-Wasm hash comparer" | plan.md:376-380, :390, :399-400 | The manifests carry these as comments (M7) | U4d, U4e-U4h |
| RQ3 | Leaf crates (link, l1, sim, proto, room, client, log) reach no tokio, quinn, wtransport, web-transport-quinn or tungstenite, and pass `cargo check --target wasm32-unknown-unknown`; "Narrow public surfaces: each crate's public items are those in §4.7; everything else is `pub(crate)`" | plan.md:420-422, :444 | check.mjs:24 (LEAF); the deps step | Every sub-unit; §4.1 lists each addition with its reason |
| RQ4 | Determinism: ADR 0004's float allow-list, libm only, no NaN in state; the hash-map and clock bans; no `+fma`, no `target-cpu=native`, no relaxed-simd, simd128 off; overflow-checks in sim and proto; "one feature set for npt-sim in the server and Wasm builds, compared with `cargo tree -e features`"; ChaCha8Rng pinned, seeded by the host; a canonical-serialization hash | plan.md:513-531 | sim/clippy.toml:9-58; Cargo.toml:103-107; no features comparison (M8) | U4d, U4d2 |
| RQ5 | "The check (from U4)": 3 recordings x 5 min = 27,000 ticks; natively twice in separate processes, once as Wasm in Node (Chromium is U7's); "the report names the first mismatching tick, with a field diff"; pass mark 0 mismatching ticks; bites one std `sin` and one hash-map iteration, "each caught at the first tick it changes" | plan.md:532-537; U7 row plan.md:1226 | Nothing; check.mjs:1156-1158 says "no wasm test is planned" | U4d (native), U4d2 (Wasm) |
| RQ6 | The seam: "one trait with one implementation": `step(commands)`, `state_hash`, the snapshot-field read-out, the hit-volume read-out, spawn and teleport; `TICK_HZ` one build-time constant; `LOOP_CATCHUP` = ⌊0.25 x TICK_HZ⌋, "no literal 7 anywhere"; `CMD_CATCHUP` = 8, separate; `due_ticks(now, schedule) -> (to_run, dropped)`, pure, fake-clock tests with PM-0004-67's bites; "The 60 Hz run's own change-set replaces npt-sim through this seam and changes no other crate" | plan.md:539-552 | Nothing | U4b, U4d, U4e (Q1 U4-SEAM) |
| RQ7 | Check homes: the room counts ticks, the server takes an injected `now` (the table); the room gets per client "the RTT, loss, send-buffer space, maximum datagram size and unsent count"; "On L1: npt-lab drives the npt-server library and npt-room on its simulated clock" | plan.md:495-511 | Nothing | U4e, U4f (Q5 U4-SERVER, Q6 U4-LOSS) |
| RQ8 | L1's step order per simulated instant: packets due by arrival, then fate index, never in the instant sent; the server's I/O step; the tick if due at base + ⌊k x 10⁹ / TICK_HZ⌋; client frames at 60 Hz with a seeded phase; the clients whose next due time has come, in slot order. Each client's host clock is offset + rate x SimClock (rate 1.5 for -21's cheat); fates per (seed, link, direction, packet index) | plan.md:597-607 | npt-l1 draws fates per flow (l1/src/lib.rs:9-12) | U4e (the runner) |
| RQ9 | The catalogue's six U4 assertions: the types both ways; channel and direction, a wrong channel refused and counted; Limits = the Size column, one byte over refused before allocation; the layout table = the Field-group rows; an L1 class audit at P2 with an owner-to-all bite; the control-socket messages decode at 1,024 B and never panic | docs/design/netcode-proof-messages.md:240-253 | U1's audit test checks the doc only (proto/tests/catalogue.rs:4-5) | U4a (items 1-3 and 6), U4e (4-5) (Q7, Q8) |
| RQ10 | "Bites first: every automated mark's planted defect must FAIL before its pass counts" | plan.md:680-683 | U2's pattern (advisor-u2.md:97-100) | Every sub-unit |
| RQ11 | "the same seed gives the same packet-fate log and the same per-tick hashes" | plan.md:749-750 | -3 on L1 (U2) | U4c, U4e (the runner's twin test) |
| RQ12 | -7's history is "sized for the largest cap the test uses (-48's test cap, fixed in U4 from L1's largest honest rewind at P3 and P4) plus the margin as data, recorded" | plan.md:696 | Nothing | U4h |
| RQ13 | Mustafa, 2026-11-09: "from U3d on, point all copies at one shared Cargo target folder (CARGO_TARGET_DIR), so each unit doesn't leave a 4–6 GB cache behind", with the touch and count safeguards | checkpoint :404-413 | Applied in U3d (u3d-plan.md §7.1) | §7 |
| RQ14 | Netcode evidence: "deterministic tests with injected latency, jitter and loss" | CLAUDE.md, the change-set protocol, item 4 | | §9 |

**The numbers U4 works to** (each from its source; U4 measures the ones marked as estimates):
- 48 soldiers in 3 x 16, each with a snapshot-receiving client; 15 big vehicles, 4 of at least 20 parts; 300 live
  projectiles; hitscan at 700 shots per second ± 5 %; one portal change per second (PM-0001-2, ADR 0001:1589).
- A 30 Hz tick, a snapshot every tick, one input per tick (ADR 0001:125-163, TICK-30).
- Per client 256 kbps = 1,066.7 B per tick, counted as UDP payload + 28 B per datagram (catalogue :262); a datagram cap
  of 1,000 B "until -36 measures" (plan.md:452); `Input` at most 166 B (catalogue :66).
- The hit history at most 1 MB in the stress scene (plan.md:459, E11).
- Estimates, not measurements: a stress-scene snapshot of about 690 B per client per tick (net F112); about 350 B more
  of event resends at P2 if every hit is a resent gameplay event (net F121); a hit-history tick of about 3 KB (net
  F106); `Input` at about 23 B whole and at most 19 B per repeat (net F116).

## 2. Carried items U4 must meet
| Id | What (short) | Source | Where | Proof |
|---|---|---|---|---|
| C1' | L1's streams as quinn's QUIC: packets of at most 1,200 B, each with its own fate; lost once a later packet is acked and 3 later packets are, or 9/8 x max(srtt, latest) has passed; else the PTO srtt + max(4 rttvar, 1 ms) + 25 ms, doubling, reset by an ack, counted from the last ack-eliciting send; each resend its own fate; in order per stream; three unit tests; the doc says L1 has no congestion window. Replaces C1 | advisor-u3q.md:160-173 | U4c | L-c1 to L-c8 (§5 U4c) |
| C2 | An RTT sample only when the newest packet of the link and direction is acked (RFC 9002 §5.1); a stream message's sample from its own packet, not its in-order delivery; tests; "due before npt-room reads rtt_ns" | advisor-u2.md:79-83 | U4c | L-c7 |
| C3 | `loss` on Link "with npt-room, its first consumer", each backend's own estimate; §4.4's send-buffer space settled | advisor-u2.md:84-89 | U4c or U6 (Q6 U4-LOSS) | per the ruling |
| C4 | The judge: a measured run (or its twin) whose build lists `bites` or names a bite, and a bite run whose build lacks it, are INVALID, with fixtures | advisor-u2.md:101-103 | U4e (with the judge change) | PASS and INVALID fixtures |
| U6 note | `Box<dyn Link>` without `Send` (L1 is Rc/RefCell); U4 adds no Send bound | advisor-u2.md:59-60 | all | review |
| -18 split | marks.json -18's parts per transport, its checks (a)-(e), its bites, SERIES's `burst_heal` (the fresh delta) and `burst_return`, and the `ws-practice` validity rule with fixtures | advisor-ws-docs-r2.md:375-377; checkpoint :540-541; marks.json still holds the old row (M10) | U4e (Q20 U4-18) | the marks step; judge fixtures |
| Sizes for fixes.rs | L1 B1 runs of the stress scene log each client's per-tick snapshot payload and datagram count, each input's payload, and the Join, JoinAccepted and FullState sizes, so the run-structured bound replaces SNAP = 1,050 and INPUT = 88 (crates/net/src/tcp_model/fixes.rs:34, :36) | advisor-ws-docs-r2.md:383 | U4e logs them; the bound is its own small unit after U4e | the series and their SERIES.md rows |
| Scene selects | -14, -16 to -25 and -27 add scene and clients to their selects, and `pre:PM-0001-2`, once the runner names each run's scene | checkpoint :325-328 | U4e (Q22 U4-SCENES) | the marks step |
| -48 test cap, -7 size | the test cap from L1's largest honest rewind at P3 and P4; -7's history size from it | plan.md:696 | U4h | recorded |
| -56 items | item (6): OQ-0001-6's two options, each with -42's correction share, -48's registration and -49's honest rewind at P2 to P4; item (10): R-10's cover item, both variants recorded on L1 | marks.json:10517; plan.md:1382 | U4g, U4h | the record series |
| `hs` | the tick record's per-client hash list: "npt-log adds it with its first writer" | marks/README.md:147-148; log/src/lib.rs:104-117 has none | U4e | a schema test |
| Stale text | the checkpoint's Open findings still describe C1, not C1' | checkpoint :298-301 (M11) | the main session, before U4c's brief | text |
| Drift | proto/clippy.toml says the check script compares it with sim's; nothing does | proto/clippy.toml:1-2; M8 | U4a (a check.mjs comparison, with a bite) | check.test.mjs |
| Features | one npt-sim feature set in the server and Wasm builds, by `cargo tree -e features`; bite: an extra feature on a shared dependency (PM-0004-74) | plan.md:522; ADR 0004:1184 | U4d2 | a deps-step line and its bite |
| Rules file | .claude/rules/netcode.md loads only for server/, shared/ and net/ paths (:2-5), yet its rules bind proto, room and client too | net F98; plan.md:416-417 | every brief names the file (changing the globs is a kit edit: Mustafa's, not asked) | the brief's text |

## 3. Rulings the structure rests on (asked in §10; this plan is written to the recommendations)
- **U4-SEAM:** the trait `SimCore` and its plain-data types live in npt-sim; npt-room is generic over `S: SimCore` and
  depends on npt-sim; npt-server and npt-lab name `npt_sim::World` once. Rust lets `impl SimCore for World` sit only in
  the trait's crate or the type's (arch F11). This keeps "sim depends on content only" (ADR 0002:186-189) and the 60 Hz
  swap inside npt-sim (plan.md:552). §4.7's npt-room line loses `SimCore`; npt-sim's gains it.
- **U4-HIT:** for each command with fire held or a fire edge, the room picks two history entries and a fraction
  (integers only) for that command's accepted view time and passes them with the command into `step`. npt-sim blends the
  volumes, casts from its own fire origin, and uses the rewound set only if the shot fires. All float geometry stays in
  npt-sim under its bans (arch F22, net F107, F123).
- **U4-WASM:** a small check-only crate over npt-sim with integer-only exports, run by Node's WebAssembly API (arch F26
  option B; M23).
- **U4-REPLAY:** recordings hold each tick's seam inputs, and the replay runs npt-sim alone (arch F27). -30's U4 leg
  replays the same recording's commands through the room (Q10).
- **U4-SERVER:** npt-server's U4 half is transport-free and runtime-free (arch F34); npt-bots' behaviours likewise
  (arch F38).
- **U4-LOSS:** `loss` and send-buffer space join Link at U6 with their consumer, rate adaptation (M13).
- **U4-CTL:** the room-wide control types and their decoder go in npt-proto at U4 (M3, M15).

## 4. Design rules U4 builds to (each becomes a test in §5)
### 4.1 Public items: §4.7 plus these, each with its reason
| Crate | Added or changed | Why |
|---|---|---|
| npt-sim | `SimCore` (moved from npt-room) | U4-SEAM |
| npt-sim | `TICK_HZ` | plan.md:544 asks for one constant and names no home; the sim owns dt (ADR 0004:185-187), so the 60 Hz swap edits only npt-sim |
| npt-sim | seam data: `Command`, `Rewind`, `TickInput` (one tick's commands, rewinds, spawns, teleports and scripted attribute changes), `EntityRecord` (the snapshot-field read-out), `HitVolumes` (the hit-volume read-out), `SimEvent` (the tick's events) | §4.6 names the read-outs but no types; plain data, no behaviour |
| npt-sim | `Recording` (the seam-level record codec, fixed-width little-endian), `StateDump` (the canonical state with its field table) | RQ5's first mismatching tick and field diff (plan.md:535); U4-REPLAY |
| npt-sim | the methods `World::predict`, `World::load_movers`, `World::ray_cast`; `Scene::estimate`, `Scene::course`, `Scene::targets` beside `Scene::stress` | ADR 0004:166-178's prediction entry; ADR 0001:258-260's one ray query for AimCast and the server; the ES, CO and TC scenes (ADR 0001:1470-1482) |
| npt-proto | `RoomControl` (the room-wide controls and their decoder) | catalogue item 6 (:253); U5's fuzz target needs it (catalogue :280, the answer to its Q6); plan.md:377 already homes "controls" in npt-proto |
| npt-proto | `Wrap` (one wrap-aware compare per counter width) | -58's six plain-`<` bites need one shared site per width (ADR 0001:1607) |
| npt-proto | `client_hash` | -17: the room and the client must call one function over one quantiser (arch F33; Q11) |
| npt-proto, npt-client | a dependency on npt-link, for `Channel` only | catalogue item 2 and the wrong-channel check use one enum (arch F37); leaf to leaf |
| npt-room | `Room<S: SimCore>`; `InputBuffer::CMD_CATCHUP` | U4-SEAM; RQ6 |
| npt-server | `Decoder::pump` (the I/O step over `&mut dyn Link` with an injected `now`); `Schedule::LOOP_CATCHUP` | U4-SERVER; RQ6 |
| npt-l1 | `L1Net::next_event_ns` | C1' makes L1 event-driven (net F115 iv) |
| npt-bots (library) | `Behaviour` (the honest brawl, the target-course shooter, the cheats, the never-acker), transport-free and clock-injected | E12 (plan.md:460); lab drives them on SimClock (U4-SERVER) |
| npt-lab | the subcommands `scenario` (with `--record`), `replay` and `hash` | plan.md:590-591 |
| npt-log | `TickRecord.hs` | marks/README.md:147-148 |
| a new crate | `npt-hash-wasm` (cdylib; its exports only, integers only) | U4-WASM (B) |
| not built at U4 | `BlockWriter` (U7: only the page reads the block, and no U4 row needs it: arch F45c); `TickLoop`, `PhaseTimers`, `ControlSocket`'s socket (U6) | §6 |

### 4.2 The seam
- `SimCore`: `step(&mut self, input: &TickInput) -> &[SimEvent]`; `state_hash(&self) -> u64`;
  `entity_records(&self) -> &[EntityRecord]`; `hit_volumes(&self) -> &HitVolumes`; spawn and teleport through
  `TickInput`. One implementation: `World`.
- Two sim entry points over the same phase functions (ADR 0004:166-178): the authority `step` (all phases) and
  `World::predict` (one slot and its vehicle: own movement, own weapon state and cosmetic shot events, against the movers
  `World::load_movers` loaded from the last decoded snapshot, drawing from no random stream).
- Pools have fixed capacity, set at scene start; a full pool refuses the new row and counts it (ADR 0004:131-152).
- The tick's event list is part of the state and of its hash (ADR 0004:544-561).

### 4.3 The tick constant and the loop policy
- `npt_sim::TICK_HZ = 30`. `Schedule::for_tick_hz(TICK_HZ)` derives the period and `LOOP_CATCHUP = TICK_HZ / 4` (7 at
  30 Hz, 15 at 60 Hz). A test derives 7 and 15; a check.mjs grep refuses `LOOP_CATCHUP` set from a literal.
- `due_ticks(now_ns, &mut Schedule) -> (to_run, dropped)`: tick k is due at base + ⌊k x 10⁹ / TICK_HZ⌋ ns; at most
  LOOP_CATCHUP run per wake; a longer stall drops the excess, counts it, and rebases. npt-lab calls the same function
  with SimClock's now (arch F35): one schedule, never two.

### 4.4 The wire
- Framing, channels and limits as the catalogue gives them (:61-78; rows :87-101). Every decoder checks the size
  first, returns an error and never panics (ADR 0001:446).
- Quantisation (`Quant`, one function each way, used by both ends): positions at 1 cm in 16/16/12 bits (±327.68 m; 0 to
  40.95 m), yaw in 16 bits, pitch in 10 bits over ±90° (net F37, F55); round to nearest with one stated tie rule,
  computed in integers after one scale multiply.
- The `Input` field table is stated by U4a and read by the limit test. net F116's proposal: a whole command of about
  23 B; a repeat as a change mask + a tick delta + the changed fields, never larger than the whole command; 5 + 23 +
  7 x 19 = 161 B at the redundancy cap, under 166 B. The aim point at 1 cm in 44 bits, under the 80-bit limit
  (ADR 0001:226-227).
- The snapshot header: the full 32-bit server tick, then 16-bit offsets (baseline, newest command received, last
  command run) and the group index (the catalogue's Snapshot row, :88).
- Wrap-aware compares per width through `Wrap`; every counter takes an injectable start value (-58).

### 4.5 Baselines, groups and the window (Q12 U4-BASELINE, Q13 U4-WINDOW)
- A snapshot that splits goes out as entity groups that each decode alone, all against one baseline B: the newest sent
  snapshot whose every group the client has acknowledged (the input's ack carries the low 16 bits of the tick plus a
  group mask). A partly acknowledged tick is never a baseline. A B older than the 32-entry ring (1.07 s) means coding
  against the per-type defaults (net F111). Events ride one group and are acknowledged with it.
- The per-client 256 kbps cap is one budget for every datagram the room side sends that client, pings and pongs
  included; the encoder's per-tick budget is what that budget leaves (net F110). Otherwise a 1,066 B snapshot every tick
  plus about 600 B per second of pings and pongs breaks PM-0001-12's 1 s window by design.
- The encoder exposes one tick's whole snapshot as a unit, for WsServer's gate at U6 (plan.md:1225, W18).

### 4.6 The client-held hash (Q11 U4-MIRROR)
- Per client per tick, the server side hashes what the encoder intends that client to hold after decoding: the baseline
  plus this tick's delta, over that client's entity set, quantised, with the own-state section as sent. The client
  hashes what it decoded. Both call `client_hash` (net F122). Hashing "the server's quantised state" literally would
  mismatch by design whenever the priority accumulator skips an entity, which it may do for up to 3 snapshots
  (ADR 0001:405-407). The authority-only streams stay out (plan.md:531).

### 4.7 Events (Q14 U4-EVENTS)
- The per-client event queue is bounded (its cap as data). On overflow the room forces one resync (the FullState clears
  the client's unacknowledged events: the catalogue's Q8, :282). If that FullState is not acknowledged within a bound
  (data; proposed 60 ticks), or the queue overflows again inside it, the room sends Kick(overflow) and drops the client
  (net F119). Event ids are 16-bit and wrap-aware.
- The cap is set from the stress scene's measured rate of relevant gameplay events x (B1's 1 s + RTT + a margin), and
  recorded. At about 350 hit events per second (net F121), a cap below about 400 would resync all 48 clients on every
  burst and change -18's heal path. Whether a hit on a third party is a resent gameplay event or a once-only cosmetic
  one is part of Q14.

### 4.8 Clocks and time on the client (Q19 U4-UNITS)
- Two clocks, kept apart (net F117). The command clock (the server-time offset from the client's own pings: a filtered
  estimator over the lowest-RTT samples; drift at most 1-2 ms per frame, errors over 100 ms halved, over 500 ms reset:
  ADR 0001:239-242) decides which tick a command is stamped for. The interpolation clock (the newest snapshot's tick
  plus the local clock since its arrival stamp, minus the adaptive delay) is monotonic and rate-slewed.
- npt-client's API takes integer nanoseconds (arch F39), not E7's milliseconds (plan.md:476-477); npt-client-wasm
  converts `performance.now()` once (U7).
- Interp buffer slots = the 5-interval maximum delay + 1 of extrapolation + 1 being written = 7 at 30 Hz; -45's bite is
  the P2-derived size minus one (net F118).

### 4.9 Lag compensation
- `HitHistory` stores the seam's `HitVolumes` per tick: soldiers, vehicle parts, and the R-10 cover items (doors, the
  wall stand-in, vehicle part transforms: net F105). The proposed ring is 64 ticks (about 192 KB at about 3 KB a tick:
  net F106), fixed by U4h from the measured test cap plus a margin, recorded, at most 1 MB.
- `rewind_at` picks the two entries around the accepted view time, and the fraction (U4a's field table gives the view
  time's fraction bits; net F116 proposes 8). The sim blends linearly, as the client's Interp drew it (net F107): a
  tick-step rewind errs by up to 30 cm at 9 m/s.
- A view time is accepted within one snapshot interval of the server's estimate (RTT + interpolation delay + input-buffer
  wait + half a tick); otherwise it is replaced by the estimate and counted (ADR 0001:358-395). The RTT source follows
  `params.transport`: `wt` reads `Link::rtt_ns`, `ws` the room's own lowest recent ping sample (ADR 0001:367-368;
  net F103).
- The cap is switchable: the test cap (above every honest rewind), then, after R3, the rule cap (plan.md:818-820).
- Cover variants for R-10: rewound with the victim, and present; both recorded (plan.md:1382).

### 4.10 Hostile input (Q16 U4-TWIN)
- Every check in plan §4.4's table gets its home and a -21 counter (the room in ticks; the server on the injected now).
- **The clamp is stable under the wire's rounding** (net F108). The room clamps only points beyond the reach + 0.87 cm
  (one quantum's half-diagonal), clamps along its own ray to exactly the reach, then snaps the result to the wire grid
  with `Quant`. The harness builds the clamp twin's point as `Quant(clamp(P))` with the room's own function, so the
  twin's sent point equals the forged run's clamped one. A unit test shows that quantising after the clamp is
  idempotent.
- **-21's twin is fate-aligned** (marks F27, M22). Forged content rides only inside packets the honest client sends
  anyway, at the same instants, on the same flows. A forgery that voids a whole datagram (trailing bytes, an unknown
  field) has, in its twin, a datagram in that slot carrying the header and acknowledgement only, with no commands. A
  wrong-channel forgery on the control stream has, in its twin, a message in that slot of a valid no-effect kind (a
  ClientSettings repeating the current delay). A wrong-channel message of a known type within its limits is refused and
  counted, never a Kick, so no forged kind closes the session (the catalogue's Input row, :87, against its Kick row,
  :99, which kicks on "type checks": Q16 settles the reading). A test shows the forged run's fate log and its twin's
  equal on every flow.

### 4.11 Determinism outside npt-sim (Q9 U4-BANS)
- The HashMap and HashSet type bans join room's and client's clippy.toml; new clippy.toml files for npt-server (with the
  clock ban) and npt-lab, and for npt-bots' behaviour code (with the clock ban) (net F28, F114; arch F36). The
  workspace's `iter_over_hash_type` deny (Cargo.toml) may cover only `for` loops (M21: not verified); the type ban
  settles it either way.
- The runner's own twin test: two processes, same seed and scenario, equal fate-log hashes and equal per-tick server and
  client hashes; bite: a HashMap-ordered client order (marks F29).

### 4.12 Logs per process
- On L1 one npt-lab process writes one log per process value the checks select on (`room`, `client`, `lab`), each with
  its own header (marks/README.md:9: "One file per process and run"; marks F28). A contract test on a short scenario
  shows that every check of every U4 L1 part resolves (no gap), and that every written name has its SERIES.md row (U3's
  pattern: net/src/l2/names.rs:139-151).

### 4.13 Scenes (data, recorded in each header)
- SS, the stress scene, with net F113's numbers: 48 soldiers in 3 x 16 in one cluster in the centre field; 15 big
  vehicles (the 4 of at least 20 parts in the fight, 11 moving elsewhere); 300 live projectiles as trajectory records;
  384 effect rows (8 per soldier) in the state and the hash; doors giving one portal change per second near the fight;
  one destructible-wall stand-in; the stand-in weapon's interval in integer sub-tick units so that 48 shooters give
  700 shots per second with deaths counted (about 48/700 s = 0.0686 s; the legacy 0.072 s gives 666.7 per second,
  inside the band by 1.7: net F113), measured by the scene's counters; DM_Tri's extent (LEGACY_INVENTORY.md:153),
  inside E3's range.
- ES, CO and TC as ADR 0001:1470-1482 gives them. TC's static targets are thin plates (at most 2 cm thick) on the
  ground, at 2.5 m and at 4 m (Q17 U4-TARGETS): a solid target can hide its own top from the chest-height fire origin,
  so -51 would measure the target's shape (net F109).

## 5. The sub-units, in dependency order
Every sub-unit ends with: check.mjs green on both builds, with its counts stated in its brief (from U3d's 267
measured, 281 bites, self 43: checkpoint :620-621); clippy -D warnings, host and wasm32, on both builds; fmt;
Cargo.lock changed only by the named dependency; the merge into the main folder, one at a time; check.mjs and
`node tools/gate.mjs --no-smoke` there, on its own cache; a commit, a checkpoint save and a push. A "bite" is a planted
defect compiled only into the `bites` build and chosen at run time (plan.md:609-610), which must fail the named test or
check.

### U4a, the wire (npt-proto; depends on nothing; size S-M)
- **Builds:** in crates/proto/src: the 14 messages and SessionControl's 4 variants (plan.md:569-575), `RoomControl`
  (Q7), framing per channel, `Limits`, `Quant`, `Wrap`, the `Input` codec with U2's delta repeats, the Snapshot
  container and its field-group layout table, `client_hash`, `encode`, `decode`. crates/proto/tests/catalogue.rs gains
  items 1, 2, 3 (the size half) and 6. check.mjs gains the sim/proto clippy.toml comparison (§2, Drift).
- **Tests, and the bites that must fail them:**
  - a1 a round trip per type on fixed seeds, 10,000 cases each.
  - a2 totality: every truncation of every valid encoding, and 100,000 fixed-seed random buffers per type, return an
    error, never a panic.
  - a3 limits: one byte over each type's limit is refused by the size check before any decoding (the allocation count:
    Q8 U4-ALLOC).
  - a4 quantiser bounds over a sweep that includes the half-steps and the range ends: position error at most 0.5 cm,
    yaw at most half of 360/65,536°, pitch at most half of 180/1,024° (bites pos-4cm, yaw-12bit, pitch-8bit: -57's
    sites).
  - a5 wrap: each width's compare across its wrap (bites: a plain `<` at each of -58's six sites; tick16, a 16-bit
    header tick, the site -17 and -58 share).
  - a6 `Input`: the whole command and a repeat at their stated maxima; a repeat never larger than the whole command; 8
    commands within 166 B; trailing bytes drop the datagram whole (bite no-trailing, -21's site).
  - a7 the catalogue's items 1, 2 and 6, each with a broken-copy bite (the catalogue's rule, :237-238); a
    wrong-channel message refused and counted (bite no-channel, -21's site).
  - a8 `client_hash`'s canonical vectors pinned; a client quantiser that rounds differently changes it (bite
    client-rounding, -17's site).
  - a9 the clippy.toml comparison (bite: one line changed in proto's copy).
- **Marks moved:** none judged; U4e and U4f run their marks against these bite sites.
- **Evidence:** the test list with its counts; each bite failing its test by name (in the build's notes).
- **Reviewers on the final diff:** netcode-architect, security-auditor (decoders of network input), rust-reviewer,
  test-engineer.

### U4b, the loop policy (npt-server's library; depends on nothing; size S)
- **Builds:** crates/server/src/schedule.rs: `Schedule`, `due_ticks`, `Schedule::LOOP_CATCHUP`; a
  crates/server/clippy.toml with the clock ban (Q9). npt-server's manifest already lists npt-net (server/Cargo.toml:24),
  so this compiles the transport stack too; `due_ticks` itself uses no tokio type.
- **Tests on a fake clock:** on schedule, one tick per period; a wake late by less than a period runs one; a 200 ms
  stall runs 6 at the next wake (at most 7) and is back on schedule; a 1 s stall runs 7, drops 23 ticks' time, counts it
  and rebases; never more than LOOP_CATCHUP per wake; the overrun and dropped counters; LOOP_CATCHUP derived as 7 at
  30 Hz and 15 at 60 Hz. **Bites** (PM-0004-67's three): timer-period counting (counting wakes, not elapsed time: the
  200 ms stall fails); an uncapped catch-up (30 in one wake: "at most 7" fails); an uncounted drop (dropped stays 0: the
  counter test fails).
- **Marks moved:** none judged; PM-0004-67 is judged at R2, VS3 (plan.md:720; marks F14).
- **Reviewers:** netcode-architect, rust-reviewer, test-engineer.

### U4c, the link model (npt-link, npt-l1; depends on nothing; size M)
- **Builds:** C1' in npt-l1 (today's model: l1/src/lib.rs:21-35, :159-162, send_stream :651-711), event-driven:
  internal events (ack arrivals, loss declarations, PTO expiries, resends) processed in (time, fate index) order whenever
  a call's `now` passes them; `L1Net::next_event_ns`. One packet-number space per link and direction for loss
  detection, so datagram packets count as later packets (RFC 9000 §12.3, not re-read here; DATAGRAM frames are
  ack-eliciting: RFC 9221 §5.2, read 2026-11-09). Fates stay drawn per (seed, link, direction, channel, packet index)
  (l1/src/lib.rs:9-12), so datagram fates do not move. Acknowledgements are immediate, cross the other direction with
  its drawn delay, and are lost inside a burst (Q15 U4-ACKS). C2's two sampling rules. The crate doc rewritten to C1'
  (a)-(g), and the old E15 sentence gone. C3 per Q6.
- **The fate log:** stream entries become one per packet send; datagram entries keep their 40 B form (:241-266). The
  tag moves to `npt-l1 fate log v2` (Q15); a test shows a datagram-only log's entries (the tag excluded) byte-equal to
  v1's.
- **Tests:**
  - L-c1 a single control-message loss under 30 Hz datagram traffic at P2: declared lost on the third later packet's
    ack or after 9/8 x max(srtt, latest), resent, delivered within its computed window.
  - L-c2 a lost message on a quiet link at P2: resent at the PTO (about 155-160 ms: advisor-u3q.md:170).
  - L-c3 a 1 s burst at P2: the PTO doubles while acks are lost; the first resend after the burst comes at its
    backed-off PTO; the recovery falls inside the bound computed from the formula.
  - L-c4 a 3,000 B message cut into 3 packets, one of them lost: the message waits for that packet's resend; delivery
    in order across messages.
  - L-c5 poll independence: the same sends, polled every 1 ms and polled only at `next_event_ns`, give equal fate logs
    and arrivals (net F115 ii).
  - L-c6 the tie rule: at one instant, internal events come before application sends, by (time, fate index).
  - L-c7 C2: an overtaken or reordered packet gives no sample; a message held behind a resent one samples about one
    RTT, not RTT + PTO.
  - L-c8 -3's regression: its five L1 parts re-run on the new build and re-judged PASS, its four bites failing as
    before (advisor-u2.md:20-22); the datagram entries equal to U2's committed logs at P2, B1 and P4 seed 1 (913db8e9,
    feabf2f0, b8a04f2f: advisor-u2.md:31-32).
  - **Bites** (unit): no PTO doubling (L-c3 fails); acks delivered inside bursts (L-c3); every packet sampled (L-c7);
    stream samples from in-order delivery (L-c7); one loss draw per message (L-c4).
- **Marks moved:** -3's L1 parts re-committed (results/u4c-pm-0001-3/<commit>/); this unblocks -18's L1 part and
  -50's RTT.
- **Reviewers:** netcode-architect, rust-reviewer, test-engineer.

### U4d, the stand-in sim (npt-sim; depends on U4-SEAM and U4-HIT; size M-L)
- **Builds:** in crates/sim/src: `World` with fixed pools; the seam (§4.2); `Scene` (SS, ES, CO and TC: §4.13);
  movement; hitscan against the supplied rewound volumes and the static world; `trajectory_at` (closed form, libm only);
  the greybox with areas, portals, doors at 1/s and the wall stand-in; vehicles and parts; S1's stand-ins as data (the
  2.80 ms top-up at 30 Hz as a counted loop, never timed here); `SimRng` (ChaCha8, one stream per purpose, our own
  sampling, the authority-only spread stream); the event list; the canonical hash (64 bits, spike-local: FNV-1a-64, no
  dependency; the TickRecord's 16 hex digits: log/src/lib.rs:114); `StateDump`; `Recording`. Cargo: libm (exact pin);
  rand_chacha =0.10.0 and rand_core =0.10.1 (the workspace's pins).
- **Tests:**
  - D-1 one build, one process: two Worlds on one recording agree every tick.
  - D-2 two native processes (the test re-runs itself as a child through `std::env::current_exe()`): equal per-tick
    hashes over 3 synthetic recordings of 9,000 ticks each (27,000 ticks); a mismatch names the first tick and the
    field.
  - D-3 prediction equals authority for one slot, bit for bit, given exact movers; the predicting World's streams are
    untouched.
  - D-4 NaN never enters: a non-finite command component is refused and counted; a debug assert runs each tick.
  - D-5 a full pool refuses and counts.
  - D-6 the SS counters per tick: 48 soldiers; 15 vehicles with at least 4 x 20 parts; about 300 projectiles; shots
    per second in 665-735 (PM-0001-2's ± 5 %); one portal change per second; 384 effect rows.
  - D-7 the hash and dump vectors pinned; the serialization holds no `usize`.
  - **Bites:** the hash-map iteration (a bites-only HashMap of at least 32 keys whose order decides an outcome, armed at
    a known tick T, in PM-0004-21's form: ADR 0004:1142): D-2's two processes must report T as the first mismatching
    tick. The std `sin` bite is caught native against Wasm, so its proof is U4d2's. Both bite sites carry a bites-only
    clippy allow (sim/clippy.toml denies both: marks F22).
- **Marks moved:** none judged; RQ5's native half; PM-0001-2's counters exist for U4e.
- **Reviewers:** netcode-architect, rust-reviewer (determinism, floats), engine-architect (the seam), test-engineer.

### U4d2, the Wasm-in-Node hash (depends on U4d and U4-WASM; size S)
- **Builds (option B):** crates/hash-wasm (a cdylib that depends on npt-sim only), with integer-only exports
  (`begin(seed_lo, seed_hi, scene)`, `feed(word)`, `step() -> status`, `hash_lo`, `hash_hi`, `dump_len`,
  `dump_word(i)`), each `#[unsafe(no_mangle)] extern "C"`, with no unsafe block and no pointer: the third named exception
  in plan §4.2 and in check.mjs's tables (the attribute only; the scan refuses any `unsafe` block in that crate). A Node
  script loads the file with `WebAssembly.instantiate`, streams a recording into it and collects the per-tick hashes;
  npt-lab's `hash` subcommand and `HashCompare` compare them with the native runs and print the first mismatching tick
  and the field diff (from both sides' dumps). check.mjs gains a `wasm` step (a release build with §4.5's flags; the
  Node run is a script, so the cargo-runner refusal at check.mjs:1156-1158 stands, with its note updated) and the
  `cargo tree -e features` comparison (§2, Features).
- **Not here:** wasm-opt. U7 adds wasm-bindgen and wasm-opt; if U7 changes the file, Node and Chromium both run the
  changed file there. Option A would instead bring npt-client-wasm and wasm-bindgen forward (Q3).
- **Tests:** 0 mismatching ticks over the 27,000 (U4d's recordings; U4e's three runner recordings join this step when
  U4e lands). **Bites:** the std `sin`, its input chosen by a scan for inputs where glibc's `sinf` and the wasm32
  builtin differ (a bites-only export returns the bits). If none differs, that is recorded as "not catchable on these
  targets" and PM-0004-21's 1-ulp planted change stands in (ADR 0004:1142; arch F48). Either is caught at its armed
  tick. The hash-map bite is caught here too, or recorded as not (std documents no seeding for wasm32: marks S-HashMap).
  The features bite: an extra feature on a shared dependency in the server build (PM-0004-74) fails the comparison.
- **Reviewers:** rust-reviewer (the exception, Wasm), engine-architect (the Wasm boundary), test-engineer.

### U4e, the room, the server's I/O half, the client's decoding and the L1 runner (depends on U4a, U4c, U4d; size L)
U4e gets a short build brief, checked by the advisor before its build (Q21): the exact encoder budget rule, the group
splitter, the run scripts and the run count.
- **Builds:**
  - npt-room: `Room<S>`; `Roster` and the slot token compare (lab passes the tokens in; minting is U6's: arch F44);
    `ClientSlot`; `AckRing` (32 entries); `Encoder` (the budget of §4.5, the groups, the layout table, the class filter,
    the per-client mirror hash of §4.6); `EventQueue` (§4.7); `InterestGrid`, `AreaFlood`, `Priority`; `InputBuffer`
    option B with `buffer_depth_ticks`; and the room-side checks that -58's counters pass through (the tick strictly
    newer, the sequence forward, the press-counter edges, the ack naming a sent snapshot). Option A and the other checks
    are U4f's.
  - npt-server's library: `Decoder` with `pump`, `ConnLimits`, `AddrTable` (fixed capacity), `Counters`, ping answers
    with their hold time, the rate buckets, the Join deadline on the injected now.
  - npt-client: decoding, acknowledging, `client_hash`; a minimal `ClientEndpoint` (input, step, the next due time in
    ns).
  - npt-bots' library: the honest brawl's `Behaviour`.
  - npt-lab: `SimClock`; `L1Scenario` (RQ8's order); `ReplayX86`; the `scenario`, `replay` and `hash` subcommands; the
    header params (scene, clients, profile, seed, `cmd_timing`, `tick_start`, suite); the `--bite` dispatch; the
    per-process logs (§4.12); the three §4.5 recordings (`scenario --record`: SS at P2 seed 1, ES at P3 seed 2, TC at
    B1 seed 3).
  - npt-log: `hs`. npt-judge: C4, with fixtures; the figures `decoded_hash_mismatch_count`,
    `decoded_hash_compared_count`, `twin_hash_mismatch_count`, `cross_arch_mismatch_count` and `machine_named`
    (figures.rs:175-224), each with PASS, FAIL and gap fixtures (plan.md:676); the -18 split and `ws-practice` (Q20);
    the scene selects (Q22).
  - Run scripts ops/runs/c-u4e-*.mjs, as U2's c-u2-pm-0001-3.mjs.
- **Tests:** the runner's twin test (§4.11); the log contract test (§4.12); the class audit (the catalogue's item 5) at
  SS P2 over 3 seeds, with the owner-to-all bite; item 4 (the layout table equals the Field-group rows; the
  authority-only groups absent); a forced group split, each group decoding alone; the baseline rule (a partly
  acknowledged tick is never a baseline; past 32 entries, the defaults); a 1 s burst at B1 evicting the last
  acknowledged baseline from the 32-entry ring (about 1.08 s old against 1.07 s: shown by arithmetic and a unit test,
  so -18's bite can bite: marks F26); the window counting pings and pongs.
- **Marks moved (L1 parts, 3 seeds each, judged by `npt-judge --only`):**
  - -17 at L1 P0, P2, P3 and B1 (10 min each, 48 clients: 864,000 hash pairs per run); bites tick16 (its runs start at
    `tick_start` = 65,536 - 1,800, so they cross the wrap: marks F26) and client-rounding.
  - -57 on -17's runs; bites pos-4cm, yaw-12bit, pitch-8bit.
  - -58 for 10 s at P0 and P3, every counter started a few steps below its wrap, at least 6 wraps and 300 ticks; the six
    plain-`<` bites and tick16.
  - -18 at L1 B1 (10 bursts, 48 clients, each burst offset 0 to 1 tick; the fresh-delta reading: advisor-u3q.md:180-186);
    bite no-fallback.
  - -30's U4 pre-check (Q10): replay clients send one recording's commands (re-encoded as `Input` with their stamped
    ticks and view times) to the room over Loopback and over L1 P0; identical server hashes every tick, the late-command
    counter 0; bites wallclock (a wall-clock read on one path's command handling) and late-command (INVALID). The view
    times must stay inside the tolerance on both legs (each replacement is logged per shot); a replacement on one leg
    only is the first cause to check, since the tolerance reads each leg's RTT (§4.9).
  - The catalogue's items 4 and 5.
  - The sizes for fixes.rs (§2), as series with SERIES.md rows; the WebSocket framing they assume, stated: a 1 B
    channel tag (catalogue :73); a 2 B header under 126 B and 4 B up to 65,535 B (RFC 6455 §5.2); +4 B for the client's
    mask; TLS 1.3's +22 B (RFC 8446 §5.2).
- **Evidence:** results/u4e/<commit>/verdict.json and runs.txt (the commit, the binaries' SHA-256s, one line per run);
  the runner's measured ticks per second, and the total wall time stated before the full set; the three recordings'
  SHA-256s, and U4d2's step green on them.
- **Reviewers:** netcode-architect, security-auditor (the server decode path, the roster and token compare),
  engine-architect (the crate boundaries, npt-server's half), rust-reviewer, test-engineer.

### U4f, clock and commands (depends on U4e; size M; its own brief)
- **Builds:** `ClockSync` (§4.8); the client's tick-lead controller (target lead = the one-way estimate + a jitter
  margin + the buffer target, the same for both options: net F104); `InputBuffer` option A; every other room-side check
  with its -21 counter (the catalogue's Input row, :87; plan §4.4); the server-side checks (size, trailing bytes, wrong
  channel, the rate bucket); the cheats' `Behaviour`s: the 11 forged kinds, the clamp points, the 1.5x clock, the lag
  switch, the never-acker.
- **Marks moved:** -47 (1,000 connects with random true offsets at P2, error at most 5 ms after 2 s; bites
  newest-raw-sample and tick-quantised receive times); -20 (P3, both options; bite redundancy-1); -21's 8 parts (forged,
  clamp, fast-clock and lag-switch, each under A and B; the 13 bites; the twins of §4.10 with their fate-alignment test);
  -22 (one never-acker among 48 for 10 min; bite unbounded, run over a full S30 of simulated time: marks F26).
- **Reviewers:** netcode-architect, security-auditor (hostile input), test-engineer, rust-reviewer.

### U4g, prediction, interpolation and events (depends on U4f; size M; its own brief)
- **Builds:** `Predictor` (reset to the server's state for the last command run, newer commands replayed through
  `World::predict`, the replay skipped when the stored prediction matches bit for bit, a 100 ms fade, a 2 m snap,
  cleared on respawn or teleport); the own-state section's move-speed and fire-interval attributes (-60); `Interp`
  (§4.8; extrapolation for at most one interval, then hold; placed respawns); the ClientSettings clamp (2 to 5
  intervals, at most one interval per second); `CueLog`, keyed by (entity, weapon, command sequence); event release by
  delayed time.
- **Marks moved:** -41 (CO, P0-P4; bites a server running 1 % fast, and the own copy quantised differently), run under
  both OQ-0001-6 options and reported per option (plan.md:819; marks F24); -42's L1 share (the P3 brawl; its bite
  smoothing-off cannot fail on L1: carried to R7); -60 (100 changes per kind; bite an own state without the changed
  value); -45's L1 part (bite one slot short); -61 (bite release on arrival); -62 (bites cues written from replay, and
  the server echoing a client's own shots); -55 (bite a vehicle collision that snaps); -56 item (6)'s -42 share per
  option.
- **Reviewers:** netcode-architect, test-engineer, rust-reviewer.

### U4h, lag compensation and aim (depends on U4g and on U4c's C2; size M; its own brief)
- **Builds:** `HitHistory` and `rewind_at` (§4.9); the view-time estimate and its tolerance; the switchable cap; the
  per-shot log of its parts (RTT, announced delay, buffer wait, blend fraction, frame-to-tick wait); `AimCast` through
  `World::ray_cast` on the published interpolated state; the TC shooter's `Behaviour` with a page-like 60 Hz frame
  loop, the tolerance-edge stamper and the 150 ms pong delayer; the judge figures `view_hit_registered_share`,
  `false_hit_share`, `rule_cap_set_by_rule` and `throw_desync_count`, with fixtures.
- **Order inside:** measure the largest honest rewind at P3 and P4; round it up to ticks and add the margin (data): the
  test cap. Size HitHistory from it (recorded: -7, plan.md:696). Then the test-cap runs, with the setup check
  `clipped == 0`.
- **Marks moved:** -48 (TC, P0-P3, under A and B, 2,000 shots per profile; bites a rewind without the interpolation
  delay, and an estimate without the buffer wait); -49's test-cap parts only (its rule-cap parts and its bite
  cap-estimate-only wait for R3's pick: marks F24); -50 (P0, P1, and the recorded WebSocket path with the ping-RTT
  source; bite no tolerance check); -51 (thin targets, 1,000 shots per height and profile, the aim at most 80 bits;
  bite a parallel ray plus a 10 cm fire-origin error); -54 (1,000 throws per kind; bite a 1/60 s client step). -56's
  items (6) and (10): both options and both cover variants recorded.
- **Reviewers:** netcode-architect, security-auditor (the rewind's bounds against cheats), test-engineer,
  rust-reviewer.

### Lanes (at most 3 agents at a time, in worktrees, merged one at a time)
- First: U4a, U4c and U4d in parallel. U4b is small: the main session, or the first free lane.
- Then U4d2; then U4e; then U4f, U4g and U4h in order (they share the room's and the client's files: never two at
  once).

## 6. What U4 does not build
- **U5:** fuzzing every message type and the room-wide controls (-34); the counting allocator, and with it the
  allocation half of the catalogue's item 3 (Q8); the audit of libm and of any other U4 dependency (-35, §4.8).
- **U6:** the tokio runtime, `TickLoop`'s thread, `PhaseTimers`, `ControlSocket`'s socket, the metrics, the panic
  catch, npt-server's binary (server/src/main.rs:3-6); WtServer, WsServer and their clients; -30's marked C part on both
  real transports (marks F11, net F100); `+ Send` on L2 links; token minting and the prefixed form; `loss`, the
  send-buffer space and rate adaptation under pressure (Q6; judged by -23 and -24 on L2); npt-bots' npt-net glue and
  npt-load.
- **U7:** npt-client-wasm, wasm-bindgen, wasm-opt, web/, `BlockWriter`, `mode`, the hash check in Chromium.
- **U8 and the R runs:** every L2, RS and ML part; PM-0004-67's judged run (R2, VS3); -49's rule cap (after R3); -42's
  frame check and its bite (R7); every time mark.
- **Its own small unit after U4e:** fixes.rs's run-structured bound (advisor-ws-docs-r2.md:383: "before R3's run
  plan").
- **Two bites that cannot fail at U4, carried by name:** PM-0001-42/smoothing-off (R7) and PM-0001-49/cap-estimate-only
  (after R3) (M24).

## 7. Build rules
### 7.1 Mustafa's shared-target rule (2026-11-09, verbatim at checkpoint :404-405; applied as at :406-413 and u3d-plan.md §7.1)
1. **One folder for every copy.** In each builder's worktree and each bite's scratch copy, every cargo call (test,
   clippy, fmt, check.mjs's steps, the wasm32 build) runs with
   `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target`. Checked once after the first build: the copy
   has no `.gate/netcode-proof-test/target` of its own, and the shared folder has the new artifacts.
2. **Touch before each build:** `find <copy>/spikes/netcode-proof/crates -type f -exec touch {} +` (mtime only),
   because cargo's fingerprint ignores a copy's location, so another copy's newer artifact can read as fresh.
3. **Check each run's test count.** check.mjs's summary must read the counts the sub-unit's brief states; `--list`
   must name the new tests; a bite run must show its own test failing by name. A count or name that does not match
   means another copy's artifact ran: touch, rebuild, and never report that run.
4. **Bites and clean builds alternate safely:** after the last bite, touch and rebuild before any further claim.
5. **Cargo's own lock** serializes two copies' builds ("Blocking waiting for file lock"): expected, not an error.
6. **The main folder keeps its own cache** (.gate/netcode-proof-test/target): the merge checks, the gate and every
   judged L1 run happen there, with `CARGO_TARGET_DIR` unset.
7. **No copy cleans the shared folder;** that is the main session's job.
8. **Every brief names the folder and the two steps** (items 2 and 3), and names .claude/rules/netcode.md as binding
   on proto, room, client and server code (§2, Rules file).

### 7.2 Who builds, and how
- Each sub-unit: one builder (general-purpose, opus, effort xhigh) in an isolated worktree, or the main session for
  U4b; the bites planted by a scratch script under §7.1; merged by the main session, one at a time.
- Judged L1 runs: the main session, in the main folder, on release builds, one evidence run set at a time; raw logs
  under .gate/, verdicts and runs.txt committed under results/ (never raw data: plan §4.1's results/ line).
- One review round per sub-unit on its final diff, then a short report to the advisor; U4's report folds them at the
  end.

## 8. Risks
1. **Size and drift.** U4 is about three times U3. Mitigation: nine sub-units, each with its own merge, review and
   report; U4e to U4h with short briefs the advisor checks (Q21).
2. **C1' moves L1 under U2's evidence.** Mitigation: L-c8 re-runs -3's L1 set and shows the datagram entries
   unchanged.
3. **Bites that cannot bite by construction:** the std `sin` bite if glibc and the Wasm builtin agree (arch F48); the
   hash-map bite if a process's iteration order changes no state; tick16 below the wrap; no-fallback if the ring
   outlives the burst; -22's bite on a short run. Mitigation: each bite's run is configured to exercise it (§5), and
   each is shown to change state, never assumed.
4. **Run volume.** About 230 L1 runs, most of them 18,000 ticks with 48 clients (marks F31); the runner's speed is
   unknown. Mitigation: U4e measures ticks per second on its first run and reports the total wall time before the full
   set; runs go in the background without ending the turn (the advisor's lesson, checkpoint :535-536). Evidence is
   never shrunk to save time.
5. **The stress scene may not fit the budget.** The estimates put a snapshot near 690 B, plus up to 350 B of event
   resends, against 1,000 B per datagram and 1,066 B per tick (net F112, F121). Mitigation: U4e measures; a scene that
   cannot fit by design goes to the advisor before any codec mark counts, never tuned silently.
6. **Determinism outside the sim:** hash ordering or a clock in lab, room, server or bots code would break same-seed
   identity. Mitigation: §4.11's bans, and the runner's twin test with its bite.
7. **Selectors that find no data** make a part NOT-RUN, not PASS. Mitigation: §4.12's contract test.
8. **A third unsafe exception (U4-WASM B)** widens a rule the advisor set. Mitigation: the attribute only, integer
   signatures, no unsafe block, and a scan that refuses any unsafe block in that crate.
9. **Float code in the client** (Predictor, Interp, ClockSync) is not under the sim's bans. Only npt-sim's state is
   hashed, prediction runs npt-sim's own functions, and the "skip when it matches" compare is bitwise.
10. **Plan text moves** (§4.7, §4.2, E7's units): each change lands in plan.md in the sub-unit that needs it, before
    its code merges, as U3's did.

## 9. Change type and evidence rule
- **Change type:** netcode (protocol, simulation, room, client, the L1 model), plus a judge change.
- **Evidence rule:** deterministic tests with injected latency, jitter and loss: L1's P0 to P4 and B1 profiles on
  simulated time, fixed seeds, 3 seeds per part, each bite failing; verdicts by npt-judge; same-seed identity by the twin
  tests.
- **Gate:** check.mjs green on both builds with the stated counts (from U4d2 on, with its `wasm` step); clippy host and
  wasm32 on both builds; fmt; `node tools/gate.mjs --no-smoke` (the legacy gate: symbols +0 -0); Cargo.lock changed
  only by named dependencies; all under §7.1.

## 10. Questions for the advisor
Plain words first, then the options, then my recommendation. None is Mustafa's to decide before U4 builds. Where an
answer is a reading of an ADR row's words (Q10, Q11, Q16's clamp, Q17), the words go into the readings PR before R5 on
his go, as Q15 (a) does (checkpoint :320-324).

- **U4-SEAM (Q1), where the sim trait lives.** Rust lets the trait's implementation sit only in the trait's crate or
  the type's. (a) The trait in npt-sim; npt-room generic over it, depending on npt-sim. (b) The trait in npt-room, with
  World implementing it in npt-sim: npt-sim then depends on npt-room, against ADR 0002:186-189, and the client's Worker
  pulls the room in. (c) Trait and implementation in npt-room, over npt-sim's concrete World: the seam is a name only,
  and the 60 Hz swap edits the room (plan.md:552). **Recommend (a)**; §4.7's two lines move (arch F11, F19).
- **U4-HIT (Q2), how a lag-compensated shot crosses the seam.** (a) The room passes, per firing command, two history
  entries and a fraction; the sim blends, casts, and decides whether the shot fires. (b) The room resolves hits itself:
  float geometry in the room, outside the sim's bans, and the room must know whether a shot fires. **Recommend (a)**
  (arch F22, net F107, F123).
- **U4-WASM (Q3), which Wasm file Node runs.** (A) npt-client-wasm and wasm-bindgen brought forward from U7, with a
  replay mode: keeps two unsafe exceptions and ADR 0002:503's "exact file" idea, but pulls U7's export design into U4.
  (B) A check-only crate over npt-sim with integer-only exports: a third named exception, for the attribute only (the
  unsafe_code lint catches `no_mangle`: rustc's lint listing, arch F25); U7's Chromium check loads the same file.
  (C) A wasm32-wasip1 binary under Node's WASI: a new target, not the browser's. **Recommend (B)**, medium confidence;
  (A) if you want one Wasm file from the start.
- **U4-REPLAY (Q4), what the recordings hold.** (a) Each tick's seam inputs; npt-sim alone replays them on every target.
  (b) Whole datagram streams, replayed through room and sim: the Wasm file then needs the room. **Recommend (a)**
  (arch F27).
- **U4-SERVER (Q5), npt-server's and npt-bots' U4 halves.** The U4 row names neither (plan.md:1223), yet `due_ticks` and
  the L1 I/O step live in npt-server (plan.md:510-511, :549, :578) and the cheats in npt-bots (plan.md:460). (a) U4
  builds their transport-free, clock-injected halves (§4.1's list); U6 the runtime, the sockets and the glue. (b) Put
  them provisionally in npt-room or npt-lab and move them at U6. **Recommend (a)**; with it, `BlockWriter` moves to U7
  (arch F45c).
- **U4-LOSS (Q6), your C3.** Plain words: the room's only use for a loss estimate is rate adaptation under pressure
  (ADR 0001:449-451), which -23 and -24 judge on L2 (ADR 0001:1613-1614); L1 has no rate limit (l1/src/lib.rs:42-43), so
  no U4 mark reads it. (a) Build `loss` at U4 as C3 says (L1 from its implicit acks, Loopback None), with the encoder's
  pressure path unit-tested only. (b) `loss` and the send-buffer space join Link at U6, with quinn's path stats and the
  rate adaptation, their real consumer; the room never reads the send-buffer space (WsServer applies fix 1 and returns
  BufferFull: plan.md:507-509), so §4.4's list loses it. **Recommend (b)**; it is your condition to amend.
- **U4-CTL (Q7), the room-wide control types.** The catalogue puts their decoding at U4 (:253), the plan puts the socket
  at U6 (plan.md:1225), and U5 fuzzes them (catalogue :280). (a) The types and the decoder in npt-proto at U4 (plan.md:377
  already says "controls"), the socket at U6. (b) Item 6 and its fuzz target move to U6. **Recommend (a)**.
- **U4-ALLOC (Q8), the catalogue item 3's "refused before anything is allocated".** A counting allocator implements
  `GlobalAlloc`, which is unsafe to implement: forbidden in this workspace (M14). (a) U4 proves the size-first refusal;
  the allocation count joins with PM-0001-34's fuzz crate at U5. (b) A counting-allocator dependency in proto's tests
  (a new dependency, audited at U5). (c) A fourth unsafe exception. **Recommend (a)**.
- **U4-BANS (Q9), determinism beyond npt-sim.** (a) The HashMap and HashSet type bans in room, client, server, lab and
  bots, and the clock ban in server and in bots' behaviours (§4.11). (b) Bans in room and client only, with BTreeMap by
  review elsewhere. **Recommend (a)**; plan §4.5's text (:517-518) follows.
- **U4-30 (Q10), "-30 on loopback" at U4.** Both real transports are U6's (plan.md:1225). (a) U4 builds the recording,
  the replay, the late-command validity and both bites, run as a pre-check on Loopback against L1 P0; the marked C part
  runs at U6. (b) All of -30 at U6. **Recommend (a)**.
- **U4-MIRROR (Q11), what -17's server side hashes.** (a) The per-client mirror (§4.6). (b) The literal world state of
  the client's set, which mismatches by design whenever an entity is skipped. **Recommend (a)**; SERIES.md's row
  follows.
- **U4-BASELINE (Q12).** (a) One baseline per tick, the newest completely acknowledged, the defaults past the ring
  (§4.5). (b) A baseline per group: more bytes saved, more state, harder to test. **Recommend (a)**.
- **U4-WINDOW (Q13).** (a) One 256 kbps budget per client for every room-side datagram, pings and pongs included.
  (b) The encoder alone at 1,066 B per tick, which breaks -12's window by design. **Recommend (a)**.
- **U4-EVENTS (Q14).** (a) The queue cap from the stress scene's measured event rate, the resync-then-Kick rule, and hits
  on third parties as resent gameplay events (ADR 0001:297-303 as I read it). (b) The same, but third-party hits sent
  once as cosmetic events, which shrinks the event section. **Recommend (a)** for the spike; (b) is a design change, for
  your reading.
- **U4-ACKS (Q15), C1' details.** (a) Acks immediate, lost inside bursts and never outside them (QUIC's cumulative ACK
  ranges repair one lost ack within a packet spacing: RFC 9000 §19.3, not re-read here); the fate-log tag moves to v2,
  with the datagram entries shown unchanged. (b) Acks also lost at the profile's rate outside bursts. (c) Keep the tag
  at v1. **Recommend (a)**.
- **U4-TWIN (Q16), -21's twins.** (a) §4.10: forged content inside packets the honest client sends anyway; a
  header-only twin for a voided datagram; a no-effect twin for a wrong-channel stream message; no forged kind closes the
  session (a wrong-channel message of a known type within its limits is refused and counted, never kicked); the
  quantise-stable clamp. (b) Twins matched by command count only: fates shift, and hashes differ for reasons that are
  not forging. **Recommend (a)**.
- **U4-TARGETS (Q17).** (a) TC's static targets are thin plates. (b) Solid targets, with -51 reading the first surface
  the server's ray meets. **Recommend (a)**.
- **U4-49 (Q18).** -49's rule-cap parts and its bite cap-estimate-only, and -42's bite smoothing-off, cannot run at U4
  (marks F24, F26). (a) Carry them by name (§6); -49 and -42 stay NOT-RUN past U4. (b) Build cheat shooters into the
  test-cap parts now. **Recommend (a)**.
- **U4-UNITS (Q19).** (a) npt-client's API in integer ns (§4.8), with E7's text changed. (b) Keep E7's ms. **Recommend
  (a)**.
- **U4-18 (Q20).** (a) marks.json -18's split, SERIES's burst_heal and burst_return, and the `ws-practice` rule with its
  fixtures, all in U4e's judge change beside C4. (b) The split now, `ws-practice` with R3's run plan. **Recommend (a)**:
  one judge change, one review, and the L1 B1 part judged against its final checks.
- **U4-BRIEFS (Q21).** (a) This plan is the umbrella; U4a to U4d2 build from it; U4e to U4h each get a short brief
  (field tables, budgets, run counts) that you check before its build. (b) Everything from this plan. **Recommend (a)**.
- **U4-SCENES (Q22), your item from the session (checkpoint :325-328).** (a) U4e adds scene and clients to the selects
  of -14, -16 to -25 and -27, and `pre:PM-0001-2`, once the runner writes the scene. (b) Leave it to the R run plans.
  **Recommend (a)**.

## 11. For Mustafa
Nothing now. Later, the readings PR before R5 carries the words of Q10, Q11, Q16's clamp and Q17, if the advisor rules
them as readings. A third unsafe exception (Q3 B) is the advisor's call: plan §4.2's exceptions come from his ruling of
2026-11-08 (plan.md:430-438).
