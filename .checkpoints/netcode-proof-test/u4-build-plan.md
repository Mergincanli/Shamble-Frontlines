# U4 build plan, rev 2: the wire, the stand-in sim, the room, the client and the L1 runner

**rev 2, approved with conditions by the advisor (advisor-u4.md); conditions 1-11 and nits (a)-(f) applied** (each
edit marked "(the advisor's condition N)" or "(nit x)"; the approved text is aebe8ea's, sha256
27e0315857e51bd5667605d032b57a0ce75701d0ba6a08bd2b9319315e3b4dc2; the plan.md edits the verdict calls for are drafted
in u4-plan-md-edits.md, for the main session to make).

**Revision 2, 2026-11-09** (netcode-architect; branch claude/netcode-proof-test-plan-hcky5g, code as at f985226; the
folder's later commits are checkpoint autosaves only). Rev 1 (same file, same day) was reviewed by test-engineer
(u4-plan-review-test.md: T1-T23, REVISE) and engine-architect (u4-plan-review-arch.md: A1-A19, A25, A26, N1-N5,
REVISE). I checked every finding against the code myself (u4-plan-rev2-notes.md); §0.1 maps each one to where rev 2
meets it. Inputs as rev 1: the three scouts (u4-scout-arch.md F1-F48, u4-scout-marks.md F1-F32, u4-scout-netcode.md
F1-F124), the U3 plans, and my rev 1 notes (u4-build-plan-notes.md M1-M25). Next: the advisor. **No code before the
advisor's APPROVE.** "arch Fn", "marks Fn" and "net Fn" are the scouts' findings; "Mn" my rev 1 notes; "Tn" and "An"
the critics'.

## 0. In plain words
- **What U4 is.** The first unit that builds the game-shaped parts of the proof test: the message format (npt-proto),
  a stand-in simulation (npt-sim), the room that runs commands and writes snapshots (npt-room), the client endpoint that
  predicts and interpolates (npt-client), the transport-free half of the server (npt-server's library), and npt-lab's
  scenario runner that drives 48 headless clients through L1 on simulated time. Then the L1 halves of the 21 marks the
  U4 row names (plan.md:1223, quoted in §1), each shown first by its planted defects ("bites") failing.
- **What it proves, and what it does not.** L1 is logic only: counts, shares and hashes, never a time mark
  (ADR 0001:1466-1467). So U4 proves that the protocol carries each client's server view under P0 to P4 and B1 (RTT 0 to
  200 ms, jitter ±0 to ±40 ms, loss 0 to 5 %, 1 s total-loss bursts: npt-l1 lib.rs:114-138), across every counter wrap;
  that forged input changes nothing; that prediction is exact when commands arrive in time; that hits register as the
  shooter saw them. It proves no timing budget (ADR 0001:164-184).
- **Why it is split, now into twelve sub-units.** U4 starts six crates from doc-only stubs (sim/src/lib.rs:1-5, proto
  :1-4, room :1-5, client :1-6, server lib :1-5, bots lib :1-3; arch F9), changes npt-l1's stream model (C1'), and
  carries 52 planted defects and about 285 L1 runs (§9; nit a). The order (§5): **U4s** the seam types, the wire grid
  and every manifest change, alone and first; then in parallel **U4a** the wire, **U4b** the loop policy, **U4c** the link model,
  **U4d** the stand-in sim and **U4j** the judge and marks change; then **U4d2** the Wasm-in-Node hash and **U4e** the
  room and the codec path; then **U4r** the L1 runner and its judged runs; then **U4f** clock and commands, **U4g**
  prediction and interpolation, **U4h** lag compensation. Every sub-unit that moves marks builds, merges, and only then
  runs its judged set.
- **What rev 2 changes, in short.** Four high findings, all fixed in text:
  - **-22 would have been INVALID by design** (T1): it keeps no stress-scene precondition; each mark's precondition is
    now decided in a per-mark table (§4.12).
  - **-30's pre-check could be judged as a -30 part** (T2): it now has its own suite, layer value and pre-check file.
    Superseded by the advisor's condition 2: no pre-check file and no `PRE-` id; a recorded npt-lab test whose headers
    list no mark, read by the advisor (§5 U4r).
  - **-17's hash was undefined when a snapshot splits** (T3, T4): a client logs a tick's hash only once every group of
    that tick decoded; a refused group logs a value that always mismatches; the hash covers the rebuilt tick, the event
    numbers and the client's own held values, so both -17 bites have a site.
  - **The rewind's data was unspecified** (A1): a shot crosses the seam as four integers, the ring of past hit volumes
    becomes an npt-sim type the room owns and lends, and a 5-minute recording is about 17 MB, not gigabytes (the
    advisor's F1 ruling).
- **What needs a ruling first.** Q1 to Q26 in §10. Four are new (Q23 the server snapping its own copy, Q24 the reading
  of -41's "run speed", Q25 the clamp's home, Q26 the CPU burn on L1); Q2, Q10, Q11, Q16 and Q22 changed. U4s needs Q1,
  Q2 and Q23; the rest of wave 1 can start once U4s merges.
- **What it expects.** Every L1 part PASS, with every bite FAIL. A mark with L2, RS or ML parts stays NOT-RUN overall
  (marks F8): U4's claim is "its L1 parts PASS and its bites FAIL", never "the mark passes". Stated plainly where a
  part passes without a bite at U4: -42's L1 P3 parts and -49's test-cap parts; -42's P2 and P4 parts have no checks at
  all and are recorded, never claimed as passing (§5 U4g, U4h).

## 0.1 The critics' findings, and where rev 2 meets each
"Agree" means I re-checked the evidence at the lines given and it holds.

| Id | Sev | My check | Where rev 2 meets it |
|---|---|---|---|
| T1 | HIGH | Agree: PM-0001-2's "SS runs" part checks `tick.v.snapshot_clients_count` min ≥ 48 and `soldiers_count` min ≥ 48 (marks.json PM-0001-2); `pre:` applies the matched part's checks (eval.rs:850-887; README:193-200); -22's drop leaves 47 | §4.12's per-mark table: -22 gains scene and clients only, no precondition; -17, -18, -20 and -21 gain it with the shot-band arithmetic (§4.13); marks with no U4 L1 part go to the R run plans; the contract test resolves PM-0001-2's checks on every SS suite. Q22 |
| T2 | HIGH | Agree: -30's C part selects no layer (marks.json -30; matched_parts eval.rs:921-930 drops only `role`); README:32 has no Loopback value; catalogue item 5 is a test, not a mark | §5 U4r: suite `precheck-30`, layers `L1` and `loopback` (README §1 gains the value in U4j), judged as `PRE-0001-30` from marks/prechecks.json, results in results/u4r-precheck-30/, never a -30 part, never U6's folder. Items 4 and 5 listed as tests. Superseded by the advisor's condition 2: no `PRE-` id and no prechecks.json; a recorded npt-lab test the judge never reads (§5 U4r). Q10 |
| T3 | HIGH | Agree: SERIES.md:266 pairs by (client, tick); a split tick with a lost group has no defined client hash | §4.6: whole-tick hash only on a complete tick; a refused group logs the sentinel; a lost group leaves the tick unlogged and counted (`snap_incomplete`); six figure fixtures (§5 U4j). Q11 |
| T4 | MED | Agree: -17 has only the figure and the S30 setup (marks.json -17); a rejecting client gives no pairs; the client decodes integers and rounds nothing | §4.6: the hash covers the rebuilt 32-bit tick and event numbers; a client hash with no server partner counts as a mismatch; the client hashes its held floats re-gridded, so client-rounding has a real site; §4.14 lists both sites |
| T5 | MED | Agree: PM-0004-21 asks for "its two orders logged" (ADR 0004:1142); a first-key outcome repeats with p ≈ 1/32 | §5 U4d: the bite folds the whole order, each process logs its order digest, equal digests mean "not exercised" |
| T6 | MED | Agree for -45, -48, -62, -58, -41; -21's not-older-than is already pinned in marks.json -21's notes | §4.14: the site, failing check, run setting and unit proof per bite; Q23, Q24 for -41 |
| T7 | MED | Agree: a voided datagram drops its ack too, so baselines, sizes, group counts and the server-to-client packet indexes differ | §4.10: the twin's slot repeats the previous ack; the forged and clamp suites run open-loop scripts; the fate test also compares group counts per tick. Q16 |
| T8 | MED | Agree: the catch-up cap equals the redundancy cap (ADR 0001:238-239), which one datagram cannot exceed | §4.10: the catch-up kind's extra datagrams stay inside the per-connection bucket and are mirrored in the twin |
| T9 | MED | Agree: the runner is "built as the server builds the sim" (ADR 0002:500-505) | §4.15, §5 U4d2: native legs on npt-lab's release build; a 5-minute budget for the step, measured; recordings regenerated from committed seeds and checked against committed SHA-256s |
| T10 | MED | Agree: L1 picks its bite per instance (l1/src/lib.rs:856-868) | §4.11: every bite per instance or per config value, never a static; a parallel-threads test |
| T11 | MED | Agree | §5: U4j apart, merged before U4c's L-c8 re-judge; builds apart from judged runs |
| T12 | MED | Agree: RFC 9002 §5.3, §6.1.2, §6.2.1, §6.2.2 (read 2026-11-09) | §5 U4c: kInitialRtt 333 ms (initial PTO 1,024 ms), kGranularity 1 ms; L-c9; three new bites |
| T13 | MED | Agree (marks.json -42, -49) | §0 and §5 U4g, U4h: those parts named as unbitten or check-less |
| T14 | LOW-MED | Agree | §5 U4h: calibrate on seeds 101-103, judge on seeds 1-3 |
| T15 | LOW-MED | Agree | §4.5: ring ages 0-31 pinned; `event.burst_eviction` per burst |
| T16 | LOW-MED | Agree | §5: a self-test bite for every check.mjs addition; §4.3's constants at 30 and 60 Hz |
| T17 | LOW-MED | Agree | §5 U4e's negative tests; §4.11's rule for paths only L2, RS or ML would run |
| T18 | LOW-MED | Agree: `roster` reads `admit` events (README:171) | §4.12: the contract test asserts the validity inputs |
| T19 | LOW | Agree | §5 U4a a2-a4 and U4s s2 reworded |
| T20 | LOW | Agree | §4.6: -57 pairs only fields the tick's snapshot carried and the client decoded |
| T21 | LOW | Partly: 49 mark bites hold (46 in the 21 marks plus PM-0004-67's 3); the run count was low | §9: about 270 L1 runs (about 285 with -3's whole re-run set: nit a); -41's per-option report is runs.txt's, not the judge's |
| T22 | LOW | Agree | §5 U4d2: PM-0004-21's cross-target vectors replace "no usize" |
| T23 | LOW-MED | Agree | §5 U4c L-c8: SHA-256 of (v1 tag + v2's datagram entries); acks' own stream and entries. Q15 |
| A1 | HIGH | Agree (plan.md:577 puts HitHistory in npt-room; hash-wasm depends on npt-sim only) | §4.2, §4.9, §4.15: integer `Rewind`; `HitHistory` in npt-sim, owned by the room, lent to `step`; one replay driver; recording fields and size. Q2, Q4 |
| A2 | MED | Agree (E5, plan.md:452) | §4.2: `portal_states()` and an area id per `EntityRecord` |
| A3 | MED | Problem agreed; fix differs | §4.2, §4.10: the clamp inside npt-sim's shot phase on integer cm; npt-room float-free (interest on integer cm; a grep). Q25 |
| A4 | MED | Agree (plan.md:485; ADR 0002:336-337) | §4.3: `npt_sim::LOOP_CATCHUP`; `Schedule` takes the cap; U4b depends on U4s only |
| A5 | MED | Agree (plan.md:544-546) | §4.3: every tick-counted value a const fn of TICK_HZ, tested at 30 and 60 Hz |
| A6 | MED | Agree, with the folded A6+ (ADR 0001:367-368; link lib.rs:79-81) | §4.9: the pump hands the room one RTT per client and runs the server's ping sender |
| A7 | MED | Agree (plan.md:505-506) | §4.5: the room owns the window; the pump reports its bytes sent |
| A8 | MED | Agree (check.mjs:903-942 reads lint-lowering flags only) | §5 U4d2: a build-flag check with self-test bites |
| A9 | MED | Agree | §5 Lanes: U4s takes every wave-1 manifest change; one check.mjs editor per wave; counts as deltas |
| A10 | MED | Agree | §5: U4j apart; U4e split into U4e and U4r; runs apart |
| A11 | LOW | Agree | §4.2: `World::ray_cast(ray, movers)` |
| A12 | MED | Agree | §4.15, §5 U4d2: one `Replayer`; bites through `begin`; like-for-like features |
| A13 | LOW | Agree (check.mjs:25, :135-137, :1110; tcpinfo's rule :94-102) | §5 U4d2: deny plus per-export allows; LEAF, WASM_CRATES; npt-sim only |
| A14 | LOW | Agree | §4.11: tokio's Instant in the bans; U6's reviewed allows |
| A15 | LOW | Agree | §5 U4e's fake-clock tests; AddrTable and the handshake cap to U6 (§6) |
| A16 | LOW | Agree | §5: dependencies restated per sub-unit |
| A17 | LOW | Agree (2.80 ms x 18,000 ticks = 50.4 s per run) | §4.13: burn and top-up off on L1, recorded. Q26 |
| A18 | LOW | Agree | §4.4: the mapping in npt-proto, with a round-trip test |
| A19 | LOW | Agree (check.mjs:1134-1135) | §7.2: each report lists its new pub items; engine-architect checks them |
| A25 | LOW | Agree | §4.11: Send asserts |
| A26 | LOW | Agree | §4.4: the reading stated |
| N1 | nit | Agree | §0, §1: ADR 0001:1466-1467 |
| N2 | nit | Agree | §4.3: `CMD_CATCHUP = Limits::REDUNDANCY_CAP`, const-asserted |
| N3 | nit | Agree | §0: the sixth stub is npt-bots' library |
| N4 | nit | Agree (plan.md:549 "pure") | §4.3: `due_ticks(now, &Schedule) -> Due` |
| N5 | nit | Deferred | §4.1: `RoomControl` stays unfeatured in npt-proto (types and a decoder npt-client never calls; unreferenced code is dropped at link time, which U7's Wasm size shows); a feature gate is Q7's fallback |

## 1. Requirements
| # | Requirement (quoted where short) | Source | Exists today | Where in U4 |
|---|---|---|---|---|
| RQ1 | "npt-sim, npt-proto, npt-room and npt-client; npt-lab's scenario runner; `due_ticks` with PM-0004-67's bites on a fake clock; the Node-hosted native-against-Wasm hash test (§4.5); then the L1 row groups with their bites: codec (-17, -57, -58); commands and hostile input (-20, -21, -22); prediction (-41, -42, -60); interpolation, clock and events (-45, -47, -61, -62); lag compensation (-48 to -51); bursts (-18); stand-ins (-54, -55); -30 on loopback" | plan.md:1223 | The crates are doc stubs (arch F9); npt-lab has `probe` only | All of §5 |
| RQ2 | Layout: sim "no I/O, clock, transport or bot logic"; room "tick-counted timers only; no I/O, async runtime or clock; builds for wasm32 and native"; client "no bindings, transport or clock"; lab the L1 runner, the re-decode and replay tool, the hash comparer | plan.md:376-380, :390, :399-400 | Manifest comments (M7) | U4s, U4d, U4e, U4r |
| RQ3 | Leaf crates reach no transport crate or runtime and pass the wasm32 check; "each crate's public items are those in §4.7; everything else is `pub(crate)`" | plan.md:420-422, :444 | check.mjs:25 (LEAF); the deps step | Every sub-unit; §4.1 lists each addition |
| RQ4 | Determinism: ADR 0004's float allow-list, libm only, no NaN in state; the hash-map and clock bans; no `+fma`, no `target-cpu=native`, no relaxed-simd, simd128 off; overflow-checks in sim and proto; one npt-sim feature set in the server and Wasm builds; ChaCha8Rng pinned; a canonical-serialization hash | plan.md:513-531 | sim/clippy.toml; Cargo.toml:103-107; no flag or features check (A8, M8) | U4s, U4d, U4d2 |
| RQ5 | "The check (from U4)": 3 recordings x 5 min = 27,000 ticks; natively twice in separate processes, once as Wasm in Node; first mismatching tick with a field diff; 0 mismatches; bites one std `sin` and one hash-map iteration | plan.md:532-537; ADR 0002:500-505 | Nothing; check.mjs:1156-1158 "no wasm test is planned" | U4d, U4d2 (U4r adds its recordings) |
| RQ6 | The seam: one trait, one implementation; `TICK_HZ` one constant; `LOOP_CATCHUP` = ⌊0.25 x TICK_HZ⌋, "no literal 7 anywhere"; `CMD_CATCHUP` = 8, separate; `due_ticks` pure, fake-clock tests; "the 60 Hz run's own change-set replaces npt-sim through this seam and changes no other crate"; every tick-counted timer derives from TICK_HZ | plan.md:539-552 | Nothing | U4s, U4b, U4d |
| RQ7 | Check homes: the room counts ticks, the server takes an injected `now`; the room gets per client RTT, loss, send-buffer space, max datagram size and unsent count; "On L1: npt-lab drives the npt-server library and npt-room on its simulated clock" | plan.md:495-511 | Nothing | U4e, U4f (Q5, Q6, Q13) |
| RQ8 | L1's step order per simulated instant; each client's host clock offset + rate x SimClock; fates per (seed, link, direction, packet index) | plan.md:597-607 | npt-l1 draws fates per flow (l1/src/lib.rs:9-12) | U4r |
| RQ9 | The catalogue's six U4 assertions | docs/design/netcode-proof-messages.md:240-253 | U1's doc audit (proto/tests/catalogue.rs:4-5) | U4a (1-3, 6), U4e (4), U4r (5) (Q7, Q8) |
| RQ10 | "Bites first: every automated mark's planted defect must FAIL before its pass counts" | plan.md:680-683 | U2's pattern | Every sub-unit; §4.14 |
| RQ11 | "the same seed gives the same packet-fate log and the same per-tick hashes" | plan.md:749-750 | -3 on L1 (U2) | U4c, U4r |
| RQ12 | -7's history sized from -48's test cap "fixed in U4 from L1's largest honest rewind at P3 and P4" plus the margin, recorded | plan.md:696 | Nothing | U4h |
| RQ13 | Mustafa, 2026-11-09: one shared CARGO_TARGET_DIR for every copy, with the touch and count safeguards | checkpoint :404-413 | Applied in U3d | §7 |
| RQ14 | Netcode evidence: "deterministic tests with injected latency, jitter and loss" | CLAUDE.md, change-set protocol item 4 | | §9 |

**The numbers U4 works to** (sources as rev 1; estimates marked):
- 48 soldiers in 3 x 16, each with a snapshot-receiving client; 15 big vehicles, 4 of at least 20 parts; 300 live
  projectiles; hitscan at 700 shots per second ± 5 %; one portal change per second (PM-0001-2, ADR 0001:1589).
- A 30 Hz tick, a snapshot every tick, one input per tick (ADR 0001:125-163).
- Per client 256 kbps = 1,066.7 B per tick, counted as UDP payload + 28 B per datagram (catalogue :262); a datagram cap
  of 1,000 B until -36 measures (plan.md:452); `Input` at most 166 B (catalogue :66); the per-connection Input bucket
  8 datagrams, refilled at 2 x TICK_HZ a second (catalogue, Input row).
- The hit history at most 1 MB in the stress scene (plan.md:459, E11).
- Estimates: a stress-scene snapshot of about 690 B per client per tick (net F112); about 350 B more of event resends at
  P2 (net F121); a hit-volume tick of about 3 KB (net F106); `Input` about 23 B whole, at most 19 B per repeat (net F116);
  a recording about 17 MB per 9,000 ticks (§4.15, my arithmetic; the advisor's F1 ruling).

## 2. Carried items U4 must meet
| Id | What (short) | Source | Where | Proof |
|---|---|---|---|---|
| C1' | L1's streams as quinn's QUIC (packets ≤ 1,200 B, own fates, the RFC 9002 loss rules, PTO doubling, in-order streams; three unit tests; the doc says L1 has no congestion window). Rev 2 adds RFC 9002's initial RTT and kGranularity (T12) | advisor-u3q.md:160-173; RFC 9002 §5.3, §6.1.2, §6.2.1, §6.2.2 | U4c | L-c1 to L-c9 |
| C2 | An RTT sample only from the newest packet of its link and direction; a stream message samples from its own packet; due before npt-room reads rtt_ns | advisor-u2.md:79-83 | U4c | L-c7 |
| C3 | `loss` on Link with npt-room, its first consumer | advisor-u2.md:84-89 | U6 if Q6 (b) | per the ruling |
| C4 | The judge: a measured run (or twin) whose build lists `bites` or names a bite, and a bite run whose build lacks it, are INVALID, with fixtures | advisor-u2.md:101-103 | U4j | fixtures |
| U6 note | `Box<dyn Link>` without `Send`; U4 adds no Send bound on Link (rev 2 asserts Send on Room, Decoder and ClientEndpoint, which hold no link) | advisor-u2.md:59-60 | all | §4.11 |
| -18 split | marks.json -18 per transport, checks (a)-(e), SERIES's `burst_heal` and `burst_return`, the `ws-practice` rule with fixtures | advisor-ws-docs-r2.md:375-377 | U4j (Q20) | fixtures |
| Sizes for fixes.rs | L1 B1 runs of the stress scene log each client's per-tick snapshot payload and datagram count, each input's payload, and Join, JoinAccepted and FullState sizes (fixes.rs:34, :36) | advisor-ws-docs-r2.md:383 | U4r logs them; the bound is its own small unit after U4r | series with SERIES.md rows |
| Scene selects | the selects of -14, -16 to -25 and -27, and `pre:PM-0001-2` | checkpoint :325-328 | U4j, per §4.12's table (Q22) | the marks step |
| -48 test cap, -7 size | from L1's largest honest rewind at P3 and P4, on calibration seeds (T14) | plan.md:696 | U4h | recorded |
| -56 items | (6) OQ-0001-6's options with -42's share, -48's registration, -49's honest rewind at P2 to P4; (10) R-10's cover variants | marks.json -56; plan.md:1382 | U4g, U4h | the record series |
| `hs` | per-client hash list on tick records | marks/README.md:147-148 | U4j | a schema test |
| Stale text | the checkpoint's Open findings still describe C1 | checkpoint :298-301 | the main session, before U4c's brief | text |
| Drift | proto/clippy.toml says the check script compares it with sim's; nothing does | proto/clippy.toml:1-2 | U4a | check.test.mjs bite |
| Features | one npt-sim feature set, server against Wasm, like for like (A12) | plan.md:522; ADR 0004:1184 | U4d2 | a deps-step line and its bite |
| Rules file | .claude/rules/netcode.md loads only for server/, shared/ and net/ paths, yet binds proto, room and client | net F98 | every brief names it (the glob change is a kit edit: Mustafa's, not asked) | brief text |
| C-U7-1 (new, carried out of U4) | U4d2 does not cover wasm-opt, while ADR 0002:500-505 checks "the exact file G6 ships": U7 reruns U4d2's check on npt-hash-wasm passed through the same wasm-opt step and flags as the shipped npt-client-wasm, or states why not | advisor-u4.md, condition 8 | U7, not U4 (the advisor's condition 8) | plan.md's U7 row and the checkpoint carry it (u4-plan-md-edits.md) |

## 3. Rulings the structure rests on (asked in §10; this plan is written to the recommendations)
- **U4-SEAM (Q1):** the trait `SimCore` and its plain-data types live in npt-sim; npt-room is generic over `S: SimCore`
  and depends on npt-sim (ADR 0002:186-189); npt-server and npt-lab name `npt_sim::World` once.
- **U4-HIT (Q2):** a lag-compensated shot crosses the seam as an integer `Rewind`; the ring of past hit volumes is an
  npt-sim type, owned by the room and lent to `step`; `rewind_at` (tolerance, cap) stays in npt-room. All float
  geometry stays in npt-sim.
- **U4-WASM (Q3):** a check-only crate over npt-sim with integer-only exports, run by Node (option B).
- **U4-REPLAY (Q4):** recordings hold each tick's `TickInput`; npt-sim replays them alone.
- **U4-SERVER (Q5):** npt-server's and npt-bots' U4 halves are transport-free and clock-injected; AddrTable and the
  handshake cap go to U6 with -25.
- **U4-LOSS (Q6):** `loss` and send-buffer space join Link at U6.
- **U4-CTL (Q7):** the room-wide control types and their decoder in npt-proto at U4.
- **U4-OWNSNAP (Q23):** the sim snaps each soldier's own state to the wire grid every tick; the grid lives in npt-sim
  and npt-proto depends on npt-sim (ADR 0002:188 allows sim types in the protocol crate).
- **U4-CLAMP (Q25):** the clamp beyond reach runs inside npt-sim's shot phase.

## 4. Design rules U4 builds to (each becomes a test in §5)
### 4.1 Public items: plan §4.7 plus these, each with its reason
| Crate | Added or changed | Why |
|---|---|---|
| npt-sim | `SimCore` (moved from npt-room) | U4-SEAM |
| npt-sim | `TICK_HZ`, `LOOP_CATCHUP`, the tick-constant functions and their derived constants (§4.3) | plan.md:544-546; A4, A5: the Worker reaches npt-sim, never npt-server |
| npt-sim | `grid` (the wire grid, each way, per field kind) | U4-OWNSNAP: the sim's own-copy snap and both codec ends need one function |
| npt-sim | seam data: `Command`, `Rewind`, `TickInput`, `EntityRecord`, `HitVolumes`, `HitHistory` (moved from npt-room), `PortalStates`, `SimEvent`; `InputReports` and `SimCore::reports` (the clamp reports and the refusal counters, outside the hash: the advisor's condition 1); the owner-only own-state kind (W1-S1); the records-to-volumes function (W1-U4D A-M1); `InputReports`' refusal records (W1-U4D N-M2); the plain-data types `Placement`, `AttributeChange`, `Attribute`, `ClampReport`, `VolumeBox`, `FieldDiff`, `Ray`, `RayHit`, `replay::Step`, `Weapon` (TUNE-1); `Scene`'s and `World`'s accessors (`World::new`, `scene`, `greybox`, `next_tick`, `dump`; `Scene::by_id`, `id`, `name`, `slots`, `team`, `stand_ins`, `with_stand_ins`, `weapon`, `health`); W1-PUB's other approved items: `StandIns::OFF` and `s1`, `HitVolumes::MAX_SOLDIERS`, `MAX_COVER` and `MAX_BYTES`, carry 17's button and press constants, `Greybox::dm_tri`, and, bites builds only, `World::BITES`, `World::with_bite` and `World::bite_order_digest` | §4.2; A1, A2; the advisor's condition 1; the advisor's W1-S1, W1-U4D and W1-PUB rulings; TUNE-1: `Weapon`, `Scene::weapon` and `Scene::health` are the facts each run's header records (U4r's npt-lab headers and U6's npt-server header: carry 60) |
| npt-sim | `Recording` (with `GENERATOR`, `encode`, `decode` and `synthetic`), `StateDump`, `replay::Replayer` and its methods | RQ5's field diff; U4-REPLAY; A12's one driver; W1-PUB: `synthetic` is in the library because U4d2 regenerates the recordings (§4.15) |
| npt-sim | `World::predict`, `World::load_movers`, `World::ray_cast`; `Scene::estimate`, `Scene::course`, `Scene::targets` beside `Scene::stress` | ADR 0004:166-178; ADR 0001:258-260; ADR 0001:1470-1482 |
| npt-sim | `EntityKind::ALL` and `EntityKind::layout`; `grid::Field::integer`; `full_state_ack_ticks_at`; `HitHistory::for_ms` in place of `HitHistory::new`; bites builds only: `grid::Config::floor_rounding`, `OWN_COPY_SNAP_BITES` and `own_copy_snap_with_bite`. Removed: bites-only `grid::Config::BITES` and `grid::Config::with_bite` (the advisor's U4s ruling, condition 1 (a)) | u4s-carries.md carry 30: arch F2 (one home for the per-kind field order; one type for a layout's fields); rust 1 (each derived constant is `<name>_at(TICK_HZ)`); netcode F6, A5; arch F1 |
| npt-proto | `RoomControl` (unfeatured; N5) | catalogue item 6 (:253); U5's fuzz target (catalogue :280) |
| npt-proto | `Wrap` (one compare per width, called with its counter's name) | -58's per-counter bites (§4.14) |
| npt-proto | `client_hash`; the `EntityRecord` ↔ wire mapping | -17 (§4.6); A18 |
| npt-proto | dependencies on npt-sim (grid, seam types) and npt-link (`Channel`) | U4-OWNSNAP, A18; catalogue item 2 (arch F37); leaf to leaf |
| npt-proto | `Message`, `Direction`, `DecodeError`, `EncodeError`; the message and control types' field types (`EndReason`, `ProfileId`, `StepResult`, `TokenKind`); `Limits`' constants and `TYPES`; `Quant`'s `encode` and `decode`; `Wrap`'s compares and `widen16`; `EntityDelta`; `Class` and the two `GROUPS` tables; the catalogue audit's name and variant helpers; bites builds only: `BITES`, `Quant::with_bite`, `Wrap::plain_lt` | the signatures and fields of plan §4.7's items; §5 U4a's layout table; catalogue items 2, 3 and 6 (the advisor's W1-PUB ruling) |
| npt-client | a dependency on npt-link, for `Channel`; bites builds only, `BITES` and `grid_with_bite` (the advisor's U4s ruling, condition 1 (a)) | as above; u4s-carries.md carry 30: arch F1 |
| npt-room | `Room<S: SimCore>`; `InputBuffer::CMD_CATCHUP`; bites builds only: `BITES` | U4-SEAM; RQ6; U4E-Q3 (the advisor's W1-PUB ruling) |
| npt-server | `Decoder::pump` (the I/O step over `&mut dyn Link`, injected `now`, the ping sender, per-client RTT and bytes sent); `Schedule`, `Due`; `Dropped`, `LoopCounters`, `next_due_ns`, `counters`, `tick_hz`, `catchup`; bites builds only: `Schedule::with_bite`, `BITES` | U4-SERVER; §4.3; A6, A7; the advisor's W1-PUB ruling |
| npt-l1 | `L1Net::next_event_ns` | C1' makes L1 event-driven |
| npt-bots (library) | `Behaviour` (the brawl, the open-loop scripts, the target-course shooter, the cheats, the never-acker), clock-injected | E12; U4-SERVER |
| npt-lab | subcommands `scenario` (with `--record`), `replay` and `hash` | plan.md:590-591 |
| npt-log | `TickRecord.hs` | marks/README.md:147-148 |
| npt-judge | nothing for -30's pre-check: the `PRE-` id, marks/prechecks.json and the `"kind": "precheck"` rule are dropped (the advisor's condition 2) | Q10 (T2); the advisor's condition 2 |
| a new crate | `npt-hash-wasm` (cdylib; integer-only exports) | U4-WASM (B) |
| not built at U4 | `BlockWriter` (U7); `TickLoop`, `PhaseTimers`, `ControlSocket`'s socket, `AddrTable`, the handshake cap (U6) | §6 |

### 4.2 The seam
- `SimCore`:
  - `step(&mut self, input: &TickInput, history: &HitHistory) -> &[SimEvent]`;
  - `state_hash(&self) -> u64`;
  - `entity_records(&self) -> &[EntityRecord]`: per entity its id, kind, area id, grid integers, and the pre-grid
    values as `u32` bits (for -57's log; the room copies bits and never computes with them);
  - `hit_volumes(&self) -> &HitVolumes`; `portal_states(&self) -> &PortalStates` (each portal open or closed: A2);
  - `reports(&self) -> &InputReports`: the tick's clamp reports and the running refusal counters per kind, outside
    `state_hash` and outside the hashed event list; the room reads them through this accessor alone and logs them
    (the advisor's condition 1);
  - spawn, teleport and scripted attribute changes go in through `TickInput`.
  - One implementation: `World`. The static graph is npt-sim's plain-data `Areas` and `Portals` (plan.md:567), so the
    room's `AreaFlood` re-floods when `portal_states` changes, with no float and no sim internals.
- **`Rewind`** rides with each command that has fire held or a fire edge: `tick_a: u32`, `fraction: u8` (the blend
  from tick_a toward tick_a + 1; U4a's field table fixes the bits, net F116 proposes 8), `cover: CoverVariant`
  (R-10's two variants). Integers only. A Rewind naming a tick the lent ring does not hold is refused and counted by
  the sim, in `reports()` and never in the hash (the advisor's condition 1); the debug assert that a correct room never
  sends one sits in the room's `rewind_at`, not in the sim, so that condition 1's unit test can feed the sim one (the
  advisor's condition 1).
- **`HitHistory`** is npt-sim's fixed ring of `HitVolumes` per tick (capacity set at construction from U4h's test cap,
  in ms, through §4.3's functions). The room owns its instance and pushes `world.hit_volumes()` after each `step`; the
  replay driver does the same, so a recording needs no volumes. The client's predicting World carries none.
- **The clamp** (Q25): inside the shot phase, on the aim point in integer cm, from the in-step fire origin: a point
  beyond the weapon's reach + 0.87 cm (one grid cell's half-diagonal) moves along its own ray to exactly the reach and
  is snapped to the grid; a clamp report `{ slot, seq, point }` in `reports()` records it, never a `SimEvent` and never
  in the hash (the advisor's condition 1). A point within reach + 0.87 cm is never clamped again, so a clamped point
  sent back is stable (the twin's rule, §4.10), for every fire origin below the bound World's doc states (TUNE-2).
- **Two entry points** over the same phase functions (ADR 0004:166-178): the authority `step` and `World::predict` (one
  slot and its vehicle, against the movers `World::load_movers` loaded from the last decoded snapshot, drawing from no
  random stream). Both end each tick with the own-copy snap through `grid` (Q23).
- `World::ray_cast(&self, ray, movers: &HitVolumes) -> RayHit`: the static world from self, the movers supplied (the
  client's interpolated set for AimCast, the blended rewound set for a server shot: ADR 0001:258-260, :358-362; A11).
- Pools have fixed capacity; a full pool refuses and counts. The tick's event list is state and hashed
  (ADR 0004:544-561).
- **Refused input leaves no trace in the hash (the advisor's condition 1).** ADR 0004:544-556 hashes the tick's event
  list, so the clamp reports and every refusal counter (the cooldown, ammunition and reload edges, Rewinds outside the
  ring, a full pool) stay outside `state_hash` and outside that list (the advisor's U4s ruling, condition 1 (b)): they
  live in `reports()`, and the room logs them. The sim's one piece of per-slot input bookkeeping is the press counters
  of the last command it ran for that slot: a press edge is a counter that differs from them, and a refused press's
  edge is consumed there and changes nothing hashed. They stay outside `state_hash` and `StateDump` (a twin's counters
  differ by design); they are a function of the command stream alone, and the predicting World restores them from the
  client's own command at the snapshot's last-run sequence (U4g). `reports()` also lists each refusal per command
  (`{ slot, seq, kind }`, beside the clamp reports), so a twin removes exactly the refused parts. D-10 runs past the
  next accepted press. (The advisor's W1-U4D ruling: it replaces condition 1's 'what the server accepted, not what
  the client claimed' for the press counters.)

### 4.3 The tick constant, the derived constants and the loop policy
- In npt-sim: `TICK_HZ = 30`; `LOOP_CATCHUP = TICK_HZ / 4` (7 at 30 Hz, 15 at 60 Hz); `const fn ticks_for_ms(ms)` =
  ⌈ms x TICK_HZ / 1,000⌉. Every tick-counted value is such a function of TICK_HZ, never a literal (A5):
  - the FullState acknowledgement bound: `full_state_ack_ticks_at(TICK_HZ)`, which is `ticks_for_ms(2_000)` (60 at
    30 Hz, 120 at 60 Hz); `TICK_HZ` is asserted in 1 to 1,000 (the advisor's U4s ruling, condition 1 (a));
  - the AckRing and the client's frame ring: `32 x TICK_HZ / 30` ticks (32, 64; 1.067 s);
  - the ClientSettings step: one interval per second, `TICK_HZ` ticks;
  - the HitHistory capacity: `ticks_for_ms(test cap + margin)` from U4h's data.
  A test evaluates each function at 30 and at 60 and pins the values; a check.mjs grep refuses those names set from a
  literal (U4s, with its self-test bite).
- `CMD_CATCHUP = npt_proto::Limits::REDUNDANCY_CAP` (8), with a const assert that it is no larger (ADR 0001:238-239; N2).
- npt-server: `Schedule::new(tick_hz, catchup)`; `due_ticks(now_ns, &Schedule) -> Due { to_run, dropped, next:
  Schedule }`, pure (N4): tick k is due at base + ⌊k x 10⁹ / TICK_HZ⌋ ns; at most `catchup` per wake; a longer stall
  drops the excess, counts it and rebases in `next`. npt-server and npt-lab pass `npt_sim::LOOP_CATCHUP`; the Worker's
  host (U7) reads the same constant through npt-client-wasm, which reaches npt-sim and never npt-server.

### 4.4 The wire
- Framing, channels and limits as the catalogue gives them (:61-78; rows :87-101). Every decoder checks the size first,
  returns an error and never panics (ADR 0001:446).
- **The grid** (`npt_sim::grid`): positions at 1 cm, yaw in 16 bits, pitch in 10 bits over ±90° (net F37, F55),
  velocity as U4a's table states; round to nearest with one stated tie rule, in integers after one scale multiply. The
  sim's own-copy snap, the encoder and the client all call it. Bites on it are chosen through a grid config value the
  caller passes, never a static (§4.11).
- **npt-proto's `Quant`** packs grid integers into their widths (position 16/16/12 bits: ±327.68 m, 0 to 40.95 m). The
  `EntityRecord` ↔ wire-field mapping lives here, one home for the room's encode and the client's decode, with a round
  trip test (A18).
- The `Input` field table is stated by U4a and read by the limit test (net F116's proposal: about 23 B whole; a repeat
  as change mask + tick delta + changed fields, never larger than the whole command; 5 + 23 + 7 x 19 = 161 B at the
  redundancy cap, under 166 B; the aim at 1 cm in 44 bits, under ADR 0001:226-227's 80).
- The snapshot header: the full 32-bit server tick; the baseline as its 16-bit age in ticks (0: coded against the
  per-type defaults); the newest command received and the last command run as 16-bit command sequences, each with a
  known flag (ADR 0001:228-230 acknowledges two commands; a sequence names the command, while the tick it ran at can
  differ from its stamp; net F112 proposed them; the advisor's W1-N2 ruling); the group index **and the group count**
  (so the client knows when a tick is complete: §4.6).
- `Wrap`: one wrap-aware compare per width, called with its counter's name; every counter takes an injectable start
  value (-58).
- **Reading (A26):** plan §4.4's server column lists "trailing bytes and unknown fields; the wrong channel". Those
  checks run in npt-server's `Decoder::pump`, which counts them; they are implemented as npt-proto decode errors
  (`TrailingBytes`, `WrongChannel`), and only the pump calls the decoder on client input. Their bite sites are in
  proto's decode.

### 4.5 Baselines, groups and the window (Q12, Q13)
- A snapshot that splits goes out as entity groups that each decode alone, all against one baseline B: the newest sent
  snapshot whose every group the client acknowledged (the input's ack carries the low 16 bits of the tick plus a group
  mask). A partly acknowledged tick is never a baseline.
- **The AckRing keeps ages 0 to 31** (age = this tick minus the snapshot's tick; 32 entries at 30 Hz), pinned by a unit
  test at both edges (age 31 kept, age 32 evicted: T15). A B older than that means coding against the per-type
  defaults. The client keeps its complete frames for the same ages, so every B the server can pick, the client holds.
- **Each burst logs `event.burst_eviction`** (1 if the last acknowledged baseline had left the ring at the first tick
  after the burst, else 0), so -18's bite shows whether it could bite.
- **The window (A7):** the room owns each client's 256 kbps window (plan.md:503's room column). The pump reports per
  client the bytes it sent since the last tick (pings, pongs: payload + 28 B each); the window counts them, and the
  encoder's budget is what remains. plan.md §4.4's list of per-tick inputs gains "bytes sent by the server side".
- The encoder exposes one tick's whole snapshot as a unit, for WsServer's gate at U6 (plan.md:1225).

### 4.6 The client-held hash (Q11)
- **What both sides hash** with one function, `client_hash`: the full 32-bit tick; the client's entity set for that
  tick in entity-id order, each entity's id, kind and grid integers; the own-state section; the event section's events
  as (32-bit event number, kind). The authority-only streams stay out (plan.md:531). The output is never 0.
- **The server side** hashes what the encoder intends the client to hold after decoding that tick: baseline B plus this
  tick's delta over the client's set (an entity skipped by the priority accumulator at B's value), the own-state
  section and the events as sent, with the server's own 32-bit tick and event numbers.
- **The client side** hashes its own held state for that tick: the frame it built, taken as the floats it holds for
  interpolation and prediction and turned back into grid integers through `grid`, with the 32-bit tick and event
  numbers as it rebuilt them. A correct client gives the same integers (an exhaustive round-trip test, U4s s2); a
  client that places an entity a cell off, rebuilds the wrong tick or numbers an event wrongly does not.
- **When the client logs:**
  - a tick whose every group decoded: its hash, under the tick it rebuilt;
  - a tick with a received group the client could not decode (no baseline, a bad header, a size or decode error): the
    sentinel 0, which always mismatches. In an honest run this never happens (the client holds every baseline the
    server can pick, and L1 neither corrupts nor duplicates: l1/src/lib.rs:42-43);
  - a tick with a group lost and none refused: nothing, and `tick.s.snap_incomplete` = 1 for that client.
- **The figure** (`decoded_hash_mismatch_count`, U4j) counts (client, tick) pairs whose hashes differ, plus every client
  hash at a (client, tick) the server never logged (a client that labels a frame with the wrong tick); server-only
  pairs are loss, not mismatches; 0 compared fails. `decoded_hash_compared_count` and the incomplete count are reported
  beside it.
- **-57's pairing (T20):** npt-lab pairs (client, tick, entity) only for fields that tick's snapshot carried and the
  client decoded, against the server's pre-grid value at that tick.

### 4.7 Events (Q14)
- The per-client event queue is bounded (cap as data). On overflow the room forces one resync (the FullState clears the
  client's unacknowledged events: catalogue Q8, :282). If that FullState is not acknowledged within
  `ticks_for_ms(2_000)` (60 ticks at 30 Hz), or the queue overflows again inside it, the room sends Kick(overflow) and
  drops the client (net F119). Event numbers are 16-bit on the wire, rebuilt to 32 bits by `Wrap`.
- The cap: the stress scene's measured rate of relevant gameplay events x (B1's 1 s + RTT + a margin), recorded. At
  about 350 hit events per second (net F121), a cap below about 400 would resync all 48 clients on every burst.

### 4.8 Clocks and time on the client (Q19)
- As rev 1: the command clock (server-time offset from the client's own pings, ADR 0001:239-242's filter) and the
  interpolation clock (monotonic, rate-slewed), kept apart (net F117); integer ns at npt-client's API (arch F39); Interp
  slots = the 5-interval maximum delay + 1 of extrapolation + 1 being written = 7 at 30 Hz.

### 4.9 Lag compensation
- `HitHistory` (§4.2) stores `HitVolumes` per tick: soldiers, vehicle parts, and R-10's cover items (doors, the wall
  stand-in, vehicle part transforms: net F105). Its capacity comes from the measured test cap plus a margin (U4h),
  recorded, at most 1 MB (10^6 B, as PM-0001-7 judges it) (proposed 64 ticks, about 192 KB at about 3 KB a tick:
  net F106).
- `rewind_at` (npt-room) turns a command's accepted view time into a `Rewind` (tick_a, fraction, cover), or replaces the
  view time by the estimate and counts it, or clips to the cap and counts it. The sim blends linearly, as the client's
  Interp drew it (net F107).
- **The tolerance** (ADR 0001:358-395): a view time is accepted within one snapshot interval of the server's estimate
  (RTT + announced interpolation delay + input-buffer wait + half a tick). **The RTT comes from the pump (A6):** for a
  `wt` or L1 session the link's `rtt_ns`, for a `ws` session the lowest recent sample of the server's own pings
  (ADR 0001:367-368). The pump knows each session's kind from the listener that accepted it, so the room never reads a
  run parameter. The pump runs the server's ping sender (a few a second: catalogue Ping row), which -50's WebSocket leg
  and its ≥ 145 ms gap check read.
- The cap is switchable: the test cap, then after R3 the rule cap (plan.md:818-820). Cover variants: rewound with the
  victim, and present; both recorded (plan.md:1382).

### 4.10 Hostile input (Q16)
- Every check in plan §4.4's table gets its home and a -21 counter; the clamp is "(in the sim)", as cooldown, ammo and
  reload already are (Q25).
- **The clamp twin:** the forged run's room logs each clamp report's point (read through `reports()`, never from the
  hashed event list: the advisor's condition 1); the twin run sends exactly those points at the same instants. Since a
  point within reach + 0.87 cm is never clamped again, the twin's shots use the same points, and a unit test shows
  clamp-then-grid is idempotent, for every fire origin below the bound World's doc states (TUNE-2).
- **The twin's command for a kind the sim checks** (the cooldown, ammunition and reload edges; the clamp, whose twin
  sends the clamped point) is the forged command with only the refused part removed, everything else equal (the
  advisor's condition 1). For a refused press the refused part is the press itself: the twin's counter for that button
  is one lower from the refused command on, every other field equal, and the twin removes exactly the presses the
  forged run's reports list (the advisor's W1-U4D ruling).
- **`event.clamp_kind.counted_count` counts refusals only, never clamp reports** (SERIES.md:144; marks.json -21's
  "clamp: never refused or counted"): npt-lab writes it from the refusal counters, and a clamp report adds nothing to
  it (the advisor's condition 1).
- **-21's twins are fate-aligned (marks F27, M22) and ack-aligned (T7):**
  - forged content rides only inside datagrams the honest script sends anyway, at the same instants, on the same flows;
  - a forgery that voids a whole datagram (trailing bytes, an unknown field) has, in its twin, a datagram in that slot
    with the header only, no commands, and an ack equal to the previous datagram's (no new information), so the
    server's acks, baselines, snapshot sizes and group counts match the forged run's;
  - every client of the forged and clamp suites runs an open-loop seeded script (commands a function of seed, slot and
    tick only), so nothing it decodes feeds back into what it sends;
  - a wrong-channel forgery on the control stream has, in its twin, a valid no-effect message in that slot (a
    ClientSettings repeating the current delay); a wrong-channel message of a known type within its limits is refused
    and counted, never kicked, so no forged kind closes the session (catalogue Input row :87 against Kick row :99);
  - **the catch-up kind (T8):** one datagram holds at most 8 commands, the cap itself, so this kind sends extra
    datagrams in one tick (16 new commands in two), inside the per-connection bucket (8 datagrams, refilled at 2 x
    TICK_HZ a second), so the catch-up check refuses the excess, not the bucket. The twin sends the same number of
    datagrams at the same instants, carrying repeats only;
  - a test shows the forged run's and its twin's fate logs equal on every flow, and their snapshot group counts equal on
    every tick.

### 4.11 Determinism and hygiene outside npt-sim (Q9)
- The HashMap and HashSet type bans join room's and client's clippy.toml; new clippy.toml files for npt-server and
  npt-lab, and for npt-bots (all three with the clock ban, naming `std::time::Instant`, `std::time::SystemTime`,
  `tokio::time::Instant` and their `now`: A14). npt-bots' file is crate-wide; U6's runtime half and its npt-net glue
  then carry reviewed per-item allows, listed in U6's plan.
- **npt-room is float-free (A3):** interest, priority and the window work on grid integers; a check.mjs grep refuses
  `f32` and `f64` in crates/room/src (U4s, with its self-test bite).
- **Bites per instance (T10):** every bite is chosen by a constructor argument or a config value the caller passes
  (`World::with_bite`, `Room::with_bite`, the grid config), never a static, as `L1Net::with_bite` does
  (l1/src/lib.rs:856-868). A test runs a bite instance and a clean instance on parallel threads in one process.
- **Send (A25):** compile-time asserts that `Room<World>`, `Decoder` and `ClientEndpoint` are `Send`, so U6 can move
  them to the tick thread; none holds a `Link`.
- **Paths no L1 run reaches (T17):** a U4-built path whose only marks are L2, RS or ML parts gets a named unit test in
  U4, or moves to the unit that first runs it (AddrTable and the handshake cap move to U6).
- The runner's own twin test (two processes, same seed and scenario: equal fate-log hashes, equal per-tick server and
  client hashes); bite: a HashMap-ordered client order (marks F29).

### 4.12 Logs, the contract test and the stress-scene precondition
- On L1 one npt-lab process writes one log per process value the checks select on (`room`, `client`, `lab`), each with
  its own header (marks/README.md:9; marks F28).
- **The contract test** (a short scenario per suite) shows that every check of every U4 L1 part resolves (no gap),
  every written name has its SERIES.md row, and **the validity inputs exist (T18):** one `admit` event per client (so
  the roster rule, README:171, is not vacuous), a footer per log, and `tick.v.late_count` wherever `late-zero` applies.
  It also resolves PM-0001-2's checks on every SS suite (T1).
- **Which marks gain the scene, the clients and `pre:PM-0001-2` (Q22, T1):**

| Mark | U4 L1 parts | Select gains | `pre:PM-0001-2` | Why |
|---|---|---|---|---|
| -17 | P0, P2, P3, B1 | scene SS, clients 48 | yes | the codec under the real load; shots hold (§4.13) |
| -18 | B1 | scene SS, clients 48 | yes | B1 under option A loses about 1.2 % of shots, inside the band (§4.13) |
| -20 | P3, A and B | scene SS, clients 48 | yes | redundancy repairs P3's losses |
| -21 | 8 suites | scene SS, clients 48 | yes | at most about 9 shots per second lost to refusals (§4.13) |
| -22 | never-ack | scene SS, clients 48 | **no** | the drop leaves 47 clients by design, which PM-0001-2's "a client on every soldier" refuses (the slot count of `everyone_in_view` changes too: figures.rs:26-36). README §4 (:193-197) puts the precondition on -22, so U4j writes this exception and its reason into README §4's sentence; -22's runs still log PM-0001-2's series (shots, projectiles, vehicles, teams, portals), and U4f's report states the load each run carried (the advisor's condition 4; a reading of ADR 0001:1589, for the readings PR: §10) |
| -14, -16, -19, -23 to -25, -27 | none at U4 | unchanged | unchanged | no U4 run feeds them; their scenes belong to the R run plans |

### 4.13 Scenes (data, recorded in each header)
- **SS, the stress scene,** with net F113's numbers: 48 soldiers in 3 x 16 in one cluster in the centre field; 15 big
  vehicles (the 4 of at least 20 parts in the fight); 300 live projectiles as trajectory records; 384 effect rows (8 per
  soldier) in the state and the hash; doors giving one portal change per second near the fight; one destructible-wall
  stand-in; the stand-in weapon's interval in integer sub-tick units for 700 shots per second with deaths counted;
  DM_Tri's extent (LEGACY_INVENTORY.md:153); the 4 very large units loop inside the fight and the other 11 elsewhere,
  and the stand-in projectiles start inside it; parts are recorded relative to their hull; effect rows record their
  end tick; a hit takes 1 of 1,000 health and SS and ES script no reloads, with a magazine that covers an S30 plus 5
  minutes at one soldier's rate (the advisor's W1-U4D ruling). A soldier counts in `soldiers_count` whether alive or
  awaiting respawn (one per slot), so deaths never drop it below 48.
- **The shot band under each precondition suite (arithmetic, before any run):** 700 / (48 x 30) = 0.486 shots per
  command-tick.
  - B1 under option A (nothing runs during a burst: OQ-0001-6 (a), ADR 0001:1156-1162): each 1 s burst loses about 22
    of its 30 ticks of commands (the redundancy cap recovers 8), about 0.73 s of shots; 10 bursts in 600 s give a mean of
    about 700 x (1 - 7.3 / 600) = 691.5 ≥ 665. Under option B the server repeats the last command on a miss, so about 700.
  - -21's forged suite: 11 forging clients, one kind each, 1,000 forged commands each; if every refusal costs that
    tick's shot, about 11 x 1,000 x 0.486 = 5,350 shots over 600 s, about 8.9 a second: a mean of about 691.
  - Any one client silent for a whole run: 700 x 47 / 48 = 685.4 ≥ 665. No suite can raise shots above 735: the
    command cap and the sim's cooldown bound each soldier's rate.
  U4r's contract test shows the band per suite on its short runs; a suite outside it goes to the advisor before its
  marks count.
- **S1's stand-ins on L1 (A17, Q26):** the 1 ms burn and the 2.80 ms top-up are off on L1 (no L1 mark reads time: ADR
  0001:1466-1467), recorded in `params.stand_ins`; the 384 effect rows stay, since they are state.
- ES, CO and TC as ADR 0001:1470-1482 gives them; TC's static targets are thin plates (at most 2 cm thick) on the
  ground, at 2.5 m and at 4 m (Q17; net F109).

### 4.14 Bites that need a site or a run setting (T4, T6, risk 3)
Each is planted where its code lives, shown to change state by a unit test in that sub-unit, and run judged in the
sub-unit that moves its mark.

| Bite | Its site | The check it fails | Run setting | Unit proof |
|---|---|---|---|---|
| -17/tick16, -58/tick16 | proto's header writer and the client's tick rebuild (the 16-bit value taken as the tick) | decoded hash: post-wrap frames logged under the wrong tick (an orphan pair) or as the sentinel | -17: `tick_start` = 63,736; -58: `ticks_below_wrap` = 30 | a frame at tick 65,537 hashes differently or is refused |
| -17/client-rounding | npt-client's call into `grid` (floor in place of nearest); the id lives in npt-client (`npt_client::BITES`, `grid_with_bite`), the grid holding only the defect (the advisor's U4s ruling, condition 1 (a)) | decoded hash | any -17 profile | the floor variant differs on about half of all position values |
| -18/no-fallback | the encoder's baseline choice, never falling back to the defaults | `burst_heal` (the fresh delta) | B1, ring ages 0-31 | a 1 s gap evicts B; `burst_eviction` = 1 |
| -21/not-older-than | the room's tick check (≥ in place of >) | fast clock's excess commands | the fast-clock suite, option A (marks.json -21 notes) | an equal-tick command runs |
| -21/no-catchup-cap | the room's per-tick command cap | forged: unrejected | the forged suite, option A, two datagrams in one tick | 16 commands run in one tick |
| -41/server-speed-1pct | the authority step's run speed x 1.01, not `predict`'s (Q24) | untraced corrections | CO, every profile | one moving tick differs |
| -41/server-quant-differs | the authority step's own-copy snap with another rounding than `grid`'s (Q23); the id lives at npt-sim's own-copy snap side (`OWN_COPY_SNAP_BITES`, `own_copy_snap_with_bite`), the grid holding only the defect (the advisor's U4s ruling, condition 1 (a)) | untraced corrections | CO | the snapped copy differs from the sent one |
| -45/buffer-one-short | Interp's slot count | L1 P2: extrapolated share < 1 % | P2; size = the clean P2 runs' logged largest delay in intervals + 2 - 1; the delay never clamped to the buffer | extrapolation on a fixed P2 arrival trace |
| -48/no-buffer-wait | the estimate without the input-buffer wait | view-hit share | where the buffer settles at 2 ticks or more (P3, option B, read from -20's logged depth; below 2, the advisor is asked before a fixed minimum is set) | honest view times replaced |
| -58/lt-cmd-seq | the room's sequence-forward compare | 0 commands refused | sequence 2 below its wrap | post-wrap commands refused |
| -58/lt-press | the room's press-edge compare | 0 presses lost or doubled | presses scripted through the wrap | an edge lost |
| -58/lt-tick-offset | the client's rebuild of the header's 16-bit 'last command run' sequence, where its replay starts (a plain `<` in place of its wrap-aware compare): U4e builds the rebuild, U4g's Predictor starts its replay there (the advisor's W1-N2 ruling; U4E-Q2) | 0 untraced corrections | command sequences crossing the wrap (as -58/lt-cmd-seq: a few steps below it) | U4e: the rebuilt sequence names the wrong command; U4g: the replay starts at the wrong command |
| -58/lt-event-seq | the client's event-number rebuild | decoded hash (event numbers are hashed: §4.6) | event numbers 3 below the wrap; SS's hits flow | a post-wrap event rebuilt wrong |
| -58/lt-ack | the room's rebuild of the 16-bit ack: a wrong rebuild names a snapshot never sent, and the check "the acknowledgement names a sent snapshot" refuses that datagram's commands | 0 commands refused | acks crossing the wrap | a post-wrap ack refused |
| -62/cues-from-replay | CueLog written from replays | plays at most once | suite `cues`: the replay skip off, a replay on every snapshot (as PM-0001-44: ADR 0001:1650) | a replayed shot plays twice |
| -62/server-echo | the room sending the owner its own cosmetic shot events | own cosmetic sent = 0 | `cues` | one event sent |
| -22/unbounded | the event queue's cap | headroom min = 0 | a full S30 of simulated time (marks.json -22) | headroom below 0 |
| PM-0004-21 hash-map | a bites-only HashMap of at least 32 keys whose whole iteration order folds into a state value | D-2 and U4d2: first mismatching tick = T | two processes; each logs its order digest | the two digests differ; equal digests mean "not exercised", never caught or missed |

### 4.15 Recordings (A1, A12, T9; Q4)
- **Fields:** a header (scene, seed, TICK_HZ, pool sizes, HitHistory's capacity (nit b)); then per tick a
  `TickInput`: per soldier one `Command` (slot 1 B, tick 4, sequence 2, buttons 2, press counters 4, move 2 (the advisor's U4s ruling, condition 1 (a)), aim in cm 12, view tick 4 and fraction 1) and its
  `Rewind` (present 1, tick_a 4, fraction 1, cover 1); spawns, teleports and scripted attribute changes when they occur.
  Fixed-width little-endian, no `usize` (plan.md:527-529).
- **Size:** 39 B per command (move is 2 B), so about 1,872 B a tick of commands for 48 soldiers before spawns and the
  tick's own header (the advisor's U4s ruling, condition 1 (a)); about 16.8 MB per 9,000 ticks and 50.5 MB for three,
  commands only (my arithmetic; U4d states the measured size). Fed to Wasm as 32-bit words: about 4.2 million calls
  each (the advisor's F1 ruling).
- **Both uses:** the sim-only replay steps npt-sim with each `TickInput` and the ring the driver rebuilds; -30's
  pre-check (a recorded npt-lab test, never judged: the advisor's condition 2) takes the SS P0 recording below (the
  advisor's condition 3) and re-encodes its commands as `Input` (stamped ticks, view times, sequences and press
  counters from the recording; the ack from what that leg's replay client decodes live) and lets the room compute its
  own Rewinds.
- **One driver:** `npt_sim::replay::Replayer` (feed words, step, per-tick hash and dump). npt-lab's `hash` subcommand
  runs it natively; npt-hash-wasm's exports wrap it; D-2 calls it in a test.
- **Sources:** U4d's three synthetic recordings (open-loop commands, Rewinds drawn inside the ring) from a seeded
  generator in npt-sim's library (`Recording::synthetic`), since U4d2's step regenerates them from the manifest,
  needing no room; U4r's three runner recordings (`scenario --record`: SS P2 seed 1, ES
  P3 seed 2, TC B1 seed 3), which hold the room's real Rewinds; and a fourth runner recording, **SS P0 seed 4**
  (`scenario --record`), the -30 pre-check's source (the advisor's condition 3). Replayed at P0, the other three's view
  times sit about one RTT older than P0's estimate, so the room would replace them on both legs, each from its own RTT;
  a recording made at P0 keeps them inside the tolerance. The determinism step keeps its three; the fourth serves the
  pre-check only.
- **Where they live:** regenerated at check time under .gate/ from a committed recordings manifest (scene, seed,
  script, generator version) with each file's SHA-256; a different hash fails the step and names the recording.
  Never committed as files.

## 5. The sub-units, in dependency order
Every sub-unit ends with: check.mjs green on both builds, its counts stated **as deltas on its base** (from U3d's
267 measured, 281 bites, self 43: checkpoint :620-621) and recomputed after the merge, both stated in its report;
clippy -D warnings, host and wasm32, both builds; fmt; Cargo.lock changed only by the dependencies named; the merge
into the main folder, one at a time; check.mjs and `node tools/gate.mjs --no-smoke` there, on its own cache; a commit,
a checkpoint save and a push. A "bite" is a planted defect compiled only into the `bites` build and chosen at run time
(plan.md:609-610), which must fail the named test or check. Each report lists its new pub items (§7.2).

### U4s, the seam types, the grid and the manifests (depends on Q1, Q2, Q23; size S-M; alone, before wave 1)
- **Builds:** crates/sim/src: `TICK_HZ`, `LOOP_CATCHUP`, `ticks_for_ms` and the derived constants (§4.3); `grid`
  (§4.4); the seam types and the `SimCore` trait (§4.2), `HitHistory` included; no `World`. Manifests, the only wave-1
  manifest change: npt-sim gains libm (exact pin), rand_chacha =0.10.0 and rand_core =0.10.1 (the workspace's pins);
  npt-proto gains npt-sim and npt-link; npt-client gains npt-link; npt-room gains npt-sim; the `bites` feature of every
  wave-1 crate (npt-sim, npt-proto, npt-server) and npt-lab's forwarding to them (the advisor's condition 10 (a)); for
  wave 2 also npt-room's and npt-client's, with npt-lab's forwarding, and npt-server's dev dependency on npt-client for
  U4e's Loopback tests, so no wave-2 lane but U4d2 edits crates/lab or Cargo.lock (condition 10 (b); Lanes); Cargo.lock
  by those only.
  check.mjs: the derived-constant grep (§4.3) and npt-room's float grep (§4.11), each with a self-test bite in
  check.test.mjs.
- **Tests:** s1 each derived constant at 30 and 60 Hz; s2 the grid: `from` then `to` returns every integer of every
  field width (exhaustive), and `to` errs by at most half a step plus one ulp of the scaled value over a sweep with the
  half-steps and range ends (T19); s3 the tie rule pinned; s4 `HitHistory` keeps exactly its capacity, oldest evicted;
  s5 the seam types are `Send`.
- **Bites:** grid floor-rounding (s2), the ring kept one short (s4).
- **Reviewers:** netcode-architect, engine-architect (the seam), rust-reviewer.

### U4a, the wire (npt-proto; depends on U4s; size S-M)
- **Builds:** the 14 messages and SessionControl's 4 variants (plan.md:569-575), `RoomControl` (Q7), framing per
  channel, `Limits`, `Quant` (packing over the grid), `Wrap`, the `Input` codec with U2's delta repeats, the Snapshot
  container (header with group index and count) and its field-group layout table, the `EntityRecord` ↔ wire mapping,
  `client_hash` (§4.6), `encode`, `decode`. proto/tests/catalogue.rs gains items 1, 2, 3 (the size half) and 6. The
  catalogue's §6 item 3 gains a note, in U4a's change-set, that its allocation half (PM-0001-34's counting allocator)
  moves to U5 (Q8; nit e); the class audit test re-runs on the changed doc.
  check.mjs gains the sim/proto clippy.toml comparison (§2, Drift). In wave 1, U4a is the only check.mjs editor.
- **Tests, and the bites that must fail them:**
  - a1 a round trip per type on fixed seeds, 10,000 cases each; the `EntityRecord` mapping round trip.
  - a2 totality: every proper prefix of every valid encoding (a cut at every byte) and 100,000 fixed-seed random
    buffers per type return an error, never a panic.
  - a3 limits: the size check runs first: a buffer one byte over its type's limit returns the size error whatever its
    content, even when it is also malformed (the allocation half is U5's: Q8).
  - a4 the packed widths at their range ends (bites pos-4cm, yaw-12bit, pitch-8bit: -57's sites, set through `Quant`'s
    own bite choice in npt-proto, as integer coarsening, not through the grid config; the ids in npt-proto's own list)
    (the advisor's U4s ruling, condition 1 (a)).
  - a5 `Wrap`: each width across its wrap, per counter name (bites: a plain `<` for one named counter; tick16).
  - a6 `Input`: whole command and repeat at their maxima; a repeat never larger; 8 commands within 166 B; trailing bytes
    give `TrailingBytes` (bite no-trailing).
  - a7 the catalogue's items 1, 2 and 6, each with a broken-copy bite; a wrong channel gives `WrongChannel` (bite
    no-channel).
  - a8 `client_hash`'s canonical vectors pinned, the 32-bit tick and the event numbers inside; it never returns 0.
  - a9 the clippy.toml comparison (self-test bite: one line changed in proto's copy).
- **Marks moved:** none judged.
- **Reviewers:** netcode-architect, security-auditor (decoders of network input), rust-reviewer, test-engineer.

### U4b, the loop policy (npt-server's library; depends on U4s; size S; the main session)
- **Builds:** crates/server/src/schedule.rs: `Schedule`, `Due`, `due_ticks` (§4.3); crates/server/clippy.toml with the
  std and tokio clock bans (§4.11). No check.mjs edit.
- **Tests on a fake clock:** as rev 1, at 30 Hz with catch-up 7 and at 60 Hz with 15: on schedule, one tick per period;
  a late wake under a period runs one; a 200 ms stall runs 6 at the next wake and is back on schedule; a 1 s stall runs
  7, drops 23 ticks' time, counts it and rebases; never more than the cap per wake; the counters. **Bites**
  (PM-0004-67's three, per instance): timer-period counting, an uncapped catch-up, an uncounted drop.
- **Marks moved:** none judged; PM-0004-67 is judged at R2 (plan.md:720).
- **Reviewers:** netcode-architect, rust-reviewer, test-engineer.

### U4c, the link model (npt-l1; depends on nothing; size M)
- **Builds:** C1' in npt-l1, event-driven (internal events processed in (time, fate index) order whenever a call's
  `now` passes them; `L1Net::next_event_ns`). One packet-number space per link and direction (datagrams are
  ack-eliciting: RFC 9221 §5.2), so datagram packets count as later packets. **RFC 9002's starting state (T12):**
  before the first sample, smoothed RTT 333 ms and variance 166.5 ms (§6.2.2, §5.3), so the first PTO is 333 +
  max(4 x 166.5, 1) + 25 = 1,024 ms; the first sample sets both (§5.3); the time threshold is max(9/8 x max(srtt,
  latest), 1 ms) (§6.1.2). Acknowledgements are immediate, delayed by the other direction's draw, lost inside bursts
  only (Q15); **acks draw from their own per-flow stream** and write their own fate-log entries (T23), so datagram
  fates do not move. C2's two sampling rules. The crate doc rewritten to C1' (a)-(g) plus the starting state; the old
  E15 sentence gone.
- **The fate log:** stream and ack entries one per packet send; datagram entries keep their 40 B form (:241-266); the
  tag moves to `npt-l1 fate log v2` (Q15).
- **Tests:**
  - L-c1 a single control-message loss under 30 Hz datagram traffic at P2: declared lost on the third later packet's
    ack or after the time threshold, resent, delivered within its computed window.
  - L-c2 a lost message on a quiet link at P2: resent at the PTO (about 155-160 ms: advisor-u3q.md:170).
  - L-c3 a 1 s burst at P2: the PTO doubles while acks are lost; the first resend after the burst at its backed-off PTO.
  - L-c4 a 3,000 B message in 3 packets, one lost: the message waits for that resend; in order across messages.
  - L-c5 the same sends polled every 1 ms and only at `next_event_ns` give equal fate logs and arrivals.
  - L-c6 at one instant, internal events come before application sends, by (time, fate index).
  - L-c7 C2: an overtaken or reordered packet gives no sample; a message held behind a resent one samples about one
    RTT, not RTT + PTO.
  - L-c8 -3's regression: its five L1 parts re-run on the new build and re-judged PASS (after U4j merges, so C4
    applies), its four bites failing as before (advisor-u2.md:20-22); and for P2, B1 and P4 seed 1, SHA-256 of (the v1
    tag + the v2 log's datagram entries) equals U2's committed 913db8e9, feabf2f0 and b8a04f2f (advisor-u2.md:31-32).
  - L-c9 a message lost before any RTT sample is resent at 1,024 ms.
  - **Bites** (unit, per instance): no PTO doubling (L-c3); acks delivered inside bursts (L-c3); every packet sampled
    (L-c7); stream samples from in-order delivery (L-c7); one loss draw per message (L-c4); **new:** separate
    packet-number spaces, so datagrams never count as later packets (L-c1 falls back to the PTO and misses its window);
    internal events applied only at polls (L-c5); application sends before internal events (L-c6).
- **Marks moved:** -3's L1 parts re-committed (results/u4c-pm-0001-3/<commit>/).
- **Reviewers:** netcode-architect, rust-reviewer, test-engineer.

### U4d, the stand-in sim (npt-sim; depends on U4s; size M-L)
- **Builds:** `World` implementing `SimCore`, with fixed pools; `Scene` (SS, ES, CO, TC: §4.13); movement with the
  own-copy snap; hitscan against the lent, blended history and the static world; the clamp (§4.2); `trajectory_at`
  (closed form, libm only); the greybox with areas, portals, doors at 1/s and the wall stand-in; vehicles and parts;
  S1's stand-ins as data (the burn and top-up switchable off: §4.13); `SimRng` (ChaCha8, one stream per purpose, the
  authority-only spread stream); the event list; the canonical hash (FNV-1a-64, spike-local); `StateDump`;
  `Recording`; `replay::Replayer`; the synthetic recording generator (`Recording::synthetic`, in npt-sim's library:
  §4.15; the advisor's W1-PUB ruling).
- **Tests:**
  - D-1 two Worlds on one recording agree every tick.
  - D-2 two processes (the test re-runs itself through `std::env::current_exe()`): equal per-tick hashes over the 3
    synthetic recordings of 9,000 ticks; a mismatch names the first tick and the field. A quick test; the judged check
    is U4d2's release-build step.
  - D-3 prediction equals authority for one slot, bit for bit, given exact movers, snap included; the predicting World's
    streams untouched.
  - D-4 every value that enters the sim from outside is finite by type: `Command`, `Rewind` and `TickInput`, its spawn,
    teleport and attribute fields included, hold integers only, so nothing is refused or counted; a debug and test
    assertion each tick that the World holds no non-finite value (every float read by `to_bits`, as the hash reads
    it), shown by a planted `0.0 / 0.0` in one phase that trips it at its tick (a scratch plant, §7.2); and an overflow
    test: recorded commands with `move_xy` at -128 and 127 and `aim` at `i32::MIN` and `i32::MAX` step in a debug
    build (overflow checks on) without a panic (carry 15). The words are the ruling's new D-4; carry 15 is
    u4s-carries.md's (the advisor's U4s ruling, condition 1 (b)).
  - D-5 a full pool refuses and counts.
  - D-6 the SS counters, asserting PM-0001-2's thresholds exactly, never "about" (the advisor's condition 5): on every
    tick 48 soldiers (alive or awaiting respawn), teams of at least 16, at least 15 big vehicles of which at least 4
    have 20 parts or more, at least 300 projectiles, and 384 effect rows; shots per second with a mean in 665-735 and
    portal changes per second with a mean in 0.95-1.05 (marks.json PM-0001-2's bands).
  - D-7 the hash and dump vectors pinned (their cross-target half is U4d2's).
  - D-8 the clamp: a point beyond reach + 0.87 cm lands at the reach on the grid; a point within it is untouched; a
    clamped point sent again is not moved; all three for every fire origin below the bound World's doc states (no
    scene places one above it); above it a reach point saturates in z and is not claimed to lie at the reach.
  - D-9 a `Rewind` blends tick_a and tick_a + 1 by its fraction against a reference; a Rewind outside the ring is
    refused and counted.
  - D-10 refused input leaves no trace (the advisor's condition 1): one unit test per kind the sim checks (the
    cooldown, ammunition and reload edges; the clamp; a Rewind outside the ring): the forged command and its twin (the
    forged command with only the refused part removed) give equal state hashes on that tick and on every tick after,
    run on past the soldier's next accepted press; `reports()` shows the refusal or the clamp report, and the hashed
    event list holds neither.
- **Bites:** the hash-map (§4.14's form: the whole order folded, each process logging its order digest; D-2 must report
  T as the first mismatching tick); the std `sin` site (its proof is U4d2's); -41's two sites (§4.14), unit-shown here
  and run judged in U4g. Each bite site carries a bites-only clippy allow (sim/clippy.toml denies them: marks F22).
- **Reviewers:** netcode-architect, rust-reviewer (determinism, floats), engine-architect (the seam), test-engineer.

### U4j, the judge and marks change (npt-judge, npt-log, marks/; depends on nothing; size M; the main session or the first free lane)
- **Builds:** npt-log `hs` and its schema test. npt-judge: C4 with fixtures; the figures
  `decoded_hash_mismatch_count` (§4.6's rules), `decoded_hash_compared_count`, `twin_hash_mismatch_count`, and, with
  fixtures only since no U4 run feeds them, `cross_arch_mismatch_count` (L2) and `machine_named` (-1's parts select
  RS, LG, ML, RD and MAC). No `PRE-` id, no marks/prechecks.json and no `"kind": "precheck"` rule, nor their tests:
  -30's pre-check is a recorded npt-lab test the judge never reads (the advisor's condition 2).
  marks.json: -18's split, `burst_heal` and `burst_return`, the `ws-practice` rule (Q20); §4.12's table (Q22).
  README: the `loopback` layer value in §1, one line, since the pre-check's Loopback leg writes an npt-log header
  (the comparer reads both legs' room logs), with no judge change (the advisor's condition 2); `hs`, stating how it
  writes a client that has no hash at a tick (`null`), the form the six decoded-hash fixtures use (nit c). README §4's
  `pre:PM-0001-2` sentence (:193-197) gains -22's exception and its reason: -22's designed drop leaves 47 clients,
  which PM-0001-2's "a client on every soldier" refuses, so -22 carries no precondition (the advisor's condition 4).
  README and SERIES fix the format of the run-structured bound's machine-readable line and of the WebSocket practice
  run's outcome record (Q20; nit f). SERIES.md: the rows rev 2 adds (`snap_incomplete`, `burst_eviction`, the clamp
  events, the precheck suite) and the figure rows' formulas.
- **Tests:** PASS, FAIL, gap and INVALID fixtures per figure and rule; for the decoded hash six cases (equal; differing;
  a lost group, not compared; a refused group, sentinel; a client pair with no server pair; 0 compared, FAIL); C4's
  four cases; `--check-marks` on marks.json (the second file is dropped: the advisor's condition 2); the drift test.
- **Marks moved:** none judged here; every later judged set reads this judge.
- **Reviewers:** test-engineer, netcode-architect, security-auditor (validity rules).

### U4d2, the Wasm-in-Node hash and the determinism step (depends on U4d and Q3; size S-M)
- **Builds (option B):** crates/hash-wasm, a cdylib depending on npt-sim only, its exports thin wrappers over
  `Replayer`: `begin(seed_lo, seed_hi, scene, bite, arm_tick)`, `feed(word)`, `step() -> status`, `hash_lo`,
  `hash_hi`, `dump_len`, `dump_word(i)`, each `#[unsafe(no_mangle)] extern "C"`, integers only; its `bites` feature
  forwards to npt-sim's. A Node script loads it with `WebAssembly.instantiate` and streams a recording in; npt-lab's
  `hash` subcommand, on the release build, runs the two native legs in two processes and `HashCompare` prints the first
  mismatching tick and the field diff. `HashCompare` takes two per-tick hash lists, so U4r's -30 pre-check reuses it
  on the two legs' per-tick server hashes (the advisor's condition 2).
- **check.mjs** (U4d2 is its only editor in its wave), each with a self-test bite in check.test.mjs:
  - a `wasm` step: the release build with §4.5's flags, the Node run (a script, so the runner refusal at
    check.mjs:1156-1158 stands, its note updated), the comparison; its wall time printed, budget 5 minutes;
  - the features comparison, like for like: npt-sim's resolved features under the server build against the hash-wasm
    build, measured builds together and bites builds together (A12);
  - **the build-flag check (A8):** `+fma`, `target-cpu=native`, `+simd128` and `+relaxed-simd` refused in the sources
    the deps step already reads (environment rustflags and cargo configs: check.mjs:903-942), and the effective
    target features read back with `rustc --print cfg` per target: the step fails on `fma` (x86-64) and on `simd128`
    or `relaxed-simd` (wasm32) in that read-back and never only reports, with a self-test bite (a read-back that lists
    one of them) (the advisor's condition 7 (a));
  - **hash-wasm's rules (A13):** its `[lints.rust]` denies unsafe_code, each listed export carries one allow (the
    pattern npt-tcpinfo has: check.mjs:135-137), and the scan refuses any other `unsafe`; it joins LEAF and
    WASM_CRATES (check.mjs:25, :1110); it depends on npt-sim only and nothing depends on it (as tcpinfo's rule,
    check.mjs:94-102). plan.md §4.1 gains its layout line.
  - **the third exception follows ADR 0002:264-265** ("an exception needs a written invariant, a test and an
    `#[allow]` with its reason"; the advisor's condition 7 (b), (c)): the written invariant, in the crate doc and beside
    each allow: no exported name collides with another symbol, and every export has integer signatures only; the
    test: the Node loader lists the module's exports and refuses any not on the list (its bite below); check.mjs's
    `UNSAFE_EXCEPTIONS` (check.mjs:135) gains `crates/hash-wasm/`; and before the code merges, plan.md §4.2's "two
    named exceptions" becomes three, hash-wasm named (the main session's plan.md edit, u4-plan-md-edits.md).
- **Tests:** 0 mismatching ticks over the 27,000 synthetic ticks, native twice and Wasm once; PM-0004-21's cross-target
  vectors: one committed byte string's digest and one committed state's bytes equal on x86-64 and wasm32 (T22).
- **Bites:** the std `sin` (input from a scan for inputs where glibc's `sinf` and the wasm32 builtin differ; if none
  differs, recorded as not catchable on these targets and a 1-ulp planted change stands in: ADR 0004:1142); the hash-map
  (caught, or recorded as not catchable on wasm32, where std documents no seeding: marks S-HashMap); an extra feature on
  a shared dependency (PM-0004-74); each flag; a stray `unsafe` block in hash-wasm; an export not on the loader's list
  (the advisor's condition 7 (b)).
- **Reviewers:** rust-reviewer (the exception, Wasm), engine-architect (the boundary), test-engineer, infra-engineer
  (the step's time in CI).

### U4e, the room and the codec path (depends on U4a, U4b, U4d; size M-L; its own brief)
- **Builds:**
  - npt-room: `Room<S>`; `Roster` and the slot token compare (lab passes tokens in; minting is U6's); `ClientSlot`;
    `AckRing` (§4.5); `Encoder` (the window's remainder, groups with index and count, the layout table, the class
    filter, the mirror hash of §4.6); `EventQueue` (§4.7); `InterestGrid` (integer cm), `AreaFlood` (from
    `portal_states`), `Priority`; `InputBuffer` option B with `buffer_depth_ticks`; `rewind_at` (the integer half; the
    tolerance and cap rules land in U4h); the room-side checks -58's counters pass through; Resync one in flight.
    room's and client's clippy.toml gain the hash-map type bans (§4.11; U4E-Q8).
  - npt-server's library: `Decoder::pump` (decode, socket checks, the ping sender and answers with their hold time,
    per-client RTT and bytes sent: §4.5, §4.9), `ConnLimits`, `Counters`, the rate buckets and the Join deadline on the
    injected now. No AddrTable and no handshake cap (U6).
  - npt-client: decoding into integer frames and held floats, the complete-tick rule and the sentinel (§4.6), acks with
    the group mask, a minimal `ClientEndpoint` (input, step, the next due time in ns).
- **Tests:** catalogue item 4 (the layout table equals the Field-group rows; the authority-only groups absent); a forced
  group split, each group decoding alone; a lost group leaves its tick unlogged and counted, a refused one logs the
  sentinel; the baseline rule and the ring's age edges; a 1 s gap evicting B; the window counting the pump's bytes;
  the mirror equal to the client's hash over Loopback at P0 for 1,000 ticks; the Send asserts; **negative cases (T17,
  A15):** each JoinRejected reason (bad token, bind mismatch, roster full, wrong version, wrong content hash); a
  datagram before Join dropped and counted; a control message before Join closing the session as malformed, counted,
  never INVALID; the Input bucket (8, refilled at 60 a second) and the Ping limit (10 a second) on a fake clock; the 2 s
  Join deadline on a fake clock; a second Resync before the FullState's ack dropped and counted; npt-client drops and
  counts a message on the wrong channel (catalogue item 2's "on both sides", plan E7; nit d).
- **-18's bite end to end (the advisor's condition 6):** §4.14's unit proof is the server half only. U4e's proof runs
  from the encoder to the client, through a 1 s gap in delivery: under no-fallback the client refuses every delta after
  the gap, so no fresh delta decodes within 500 ms; the clean build decodes one. If the bite's baseline turns out to be
  one the client still holds, the bite is not a defect on L1, and -18's L1 part comes to the advisor before it counts.
- **Bites (unit-shown):** no-fallback (-18, shown end to end as above), tick16, client-rounding (the client's call
  site), lt-ack, lt-tick-offset and lt-event-seq (§4.14), lt-cmd-seq and lt-press (the room's compares), server-echo
  (-62, the room's routing), owner-to-all (catalogue item 5's), no-trailing and no-channel through the pump.
- **Marks moved:** none judged (U4r runs them).
- **Reviewers:** netcode-architect, security-auditor (the decode path, roster and token compare), engine-architect
  (crate boundaries), rust-reviewer, test-engineer.

### U4r, the L1 runner and its judged runs (depends on U4c, U4d2, U4e, U4j; size M; its own brief)
- **Builds:** npt-lab: `SimClock`; `L1Scenario` (RQ8's order); the header params (scene, clients, profile, seed,
  `cmd_timing`, `tick_start`, `ticks_below_wrap`, suite, `stand_ins`); the `--bite` dispatch to each instance; the
  per-process logs with `admit` events and footers (§4.12); `scenario` (with `--record`) and `replay` (the x86-64
  re-decode, R-8: it re-decodes a run's logged client bytes natively and writes the clients' hashes; the advisor's
  condition 9); the four runner recordings (§4.15; the fourth, SS P0, for the -30 pre-check: the advisor's condition
  3); -57's pairing (§4.6). npt-bots' library: the brawl `Behaviour` and the open-loop scripts. Run
  scripts ops/runs/c-u4r-*.mjs, as U2's c-u2-pm-0001-3.mjs.
- **Tests:** the runner's twin test with its bite (§4.11); the contract test (§4.12), which also shows every PM-0001-2
  check passing, not only resolving, on the short runs of each precondition suite: -17's four profiles, -18's B1,
  -20's P3 and each of -21's eight suites with its open-loop scripts; a suite that fails comes to the advisor before its
  marks count, as §4.13 says for the shot band (the advisor's condition 5); `replay`'s U4 test (the advisor's
  condition 9): one short L1 run (SS P2) logs each client's received bytes under .gate/, and `replay`, re-decoding
  them, gives the clients' logged hashes at every (client, tick); bite: the re-decoder's grid config floored, which must
  give mismatches; the class audit, catalogue item 5, at SS P2 over 3 seeds with the owner-to-all bite; the runner's
  measured ticks per second, and the total wall time of the run set stated before it starts.
- **Then the judged runs** (after the merge, in the main folder, by `npt-judge --only`, 3 seeds per part):
  - -17 at L1 P0, P2, P3 and B1 (10 min, 48 clients, SS); bites tick16 (runs starting at `tick_start` = 63,736) and
    client-rounding.
  - -57 on -17's runs; bites pos-4cm, yaw-12bit, pitch-8bit.
  - -18 at L1 B1 (10 bursts, each offset 0 to 1 tick; the fresh-delta reading: advisor-u3q.md:180-186); bite
    no-fallback, with `burst_eviction` logged.
  - **-30's pre-check, a recorded npt-lab test, not a judged id (Q10; the advisor's conditions 2 and 3):**
    - suite `precheck-30`; one leg over Loopback (layer `loopback`), one over L1 P0 (layer `L1`); replay clients send
      the SS P0 recording's commands (§4.15) to the room on both legs, under option B with the input buffer deep enough
      for P0; both legs' input-buffer depth (`buffer_depth_ticks`) recorded (marks.json -30's notes; condition 3);
    - the comparer: npt-lab's `HashCompare` (built in U4d2) compares the two legs' per-tick server hashes and names the
      first tick that differs; `tick.v.late_count` must sum to 0 on both legs (condition 2);
    - the setup condition (condition 3): 0 view-time replacements and 0 clipped shots on both legs, each replacement
      and clip logged per shot; a leg with any, or with `late_count` above 0, makes the pair VOID, reported, never a
      pass or a fail;
    - why the L1 leg's RTT before its first sample cannot make the legs decide differently (condition 3): Loopback's
      `rtt_ns` is always Some(0) (link/src/lib.rs:203-205), while L1 starts at U4c's 333 ms. The RTT reaches the room
      only through the tolerance's accept-or-replace and the cap's clip (§4.9), and an accepted view time's Rewind
      depends on the view time alone, so the legs can differ through the RTT only by a replacement or a clip, which the
      setup condition makes VOID. In a correct build it does not arise: each replay client sends its first `Input` only
      after it decodes `JoinAccepted`, whose packet it acknowledges at once (U4c's immediate acks; at one instant
      internal events come before application sends: L-c6), so the server's L1 link holds its first sample (about 0 at
      P0) before the room reads that client's RTT for any command. The runner logs per client the instant of that first
      sample and the tick of its first command run; a client whose first command ran before its first sample makes the
      leg VOID;
    - the bites: wallclock (a wall-clock read in the pump for one leg's sessions, bites build only) must give a
      mismatch; late-command (one replayed command delayed past its tick) must make the pair VOID, never a pass;
    - the headers list no mark, so the judge never reads these runs; results/u4r-precheck-30/<commit>/runs.txt and the
      comparer's output are committed, and the advisor reads them in U4r's review, as for U3d's self-test
      (advisor-u3d.md, U3D-JUDGE); nothing goes into marks.json;
    - -30 stays NOT-RUN until U6: never reported as -30's part, never in U6's folder.
  - The sizes for fixes.rs (§2), as series with SERIES.md rows; the WebSocket framing they assume, stated (a 1 B channel
    tag, catalogue :73; a 2 B header under 126 B and 4 B up to 65,535 B, RFC 6455 §5.2; +4 B for the client's mask;
    TLS 1.3's +22 B, RFC 8446 §5.2).
- **Evidence:** results/u4r/<commit>/verdict.json and runs.txt (the commit, the binaries' SHA-256s, one line per run);
  the recordings' SHA-256s and U4d2's step green on them (in check.mjs if the step stays within 5 minutes, else
  through ops/runs/det-u4.mjs at every merge that touches sim, proto, room or hash-wasm, on the advisor's ok).
- **Reviewers:** netcode-architect, test-engineer, engine-architect, rust-reviewer.

### U4f, clock and commands (depends on U4r; size M; its own brief)
- **Builds:** `ClockSync` (§4.8); the client's tick-lead controller (net F104); `InputBuffer` option A; every other
  room-side check with its -21 counter; the server-side checks' -21 counters and judged runs on U4e's pump (U4e builds
  and unit-tests the checks: U4E-Q11); the cheats' `Behaviour`s (the 11 forged kinds per §4.10, the clamp points, the
  1.5x clock, the lag switch, the never-acker).
- **Marks moved** (build, merge, then the judged runs): -47 (1,000 connects at P2; bites newest-raw-sample and
  tick-quantised receive times); -20 (P3, both options; bite redundancy-1); -21's 8 parts with their twins (§4.10) and
  13 bites, each bite run in the suite and option §4.14 or marks.json's notes name; -22 (one never-acker among 48 for
  10 min, no precondition (README §4 states the exception: U4j); its runs still log PM-0001-2's series (shots,
  projectiles, vehicles, teams, portals), and U4f's report states the load each run carried (the advisor's condition
  4); bite unbounded over a full S30).
- **Reviewers:** netcode-architect, security-auditor (hostile input), test-engineer, rust-reviewer.

### U4g, prediction, interpolation and events (depends on U4f; size M; its own brief)
- **Builds:** `Predictor` (reset to the server's state for the last command run, newer commands replayed through
  `World::predict`, the replay skipped when the stored prediction matches bit for bit except in suite `cues`, a 100 ms
  fade, a 2 m snap, cleared on respawn or teleport); the own-state section's move-speed and fire-interval attributes;
  `Interp` (§4.8); the ClientSettings clamp; `CueLog`, keyed by (entity, weapon, command sequence); event release by
  delayed time.
- **Marks moved:** -41 (CO, P0-P4, both OQ-0001-6 options, 6 runs per part; its parts pool both options, so each run
  must pass, and runs.txt reports per option; bites §4.14); -42 (the P3 parts PASS **unbitten** at U4, smoothing-off
  being ML-only; the P2 and P4 parts have **no checks**, so they are recorded for -56 item (6), never claimed as
  passing); -60 (100 changes per kind; bite own-state-missing); -45's L1 parts (bite §4.14); -61 (bite release on
  arrival); -62 (§4.14); -55 (bite a vehicle collision that snaps); -56 item (6)'s -42 share per option; -58 (10 s at
  P0 and P3, every counter a few steps below its wrap, 3 seeds per part; its six bites, §4.14), judged after U4g
  merges: its untraced-corrections check and lt-tick-offset's failing check read the Predictor (the advisor's W1-N2
  ruling and U4E-Q2).
- **Reviewers:** netcode-architect, test-engineer, rust-reviewer.

### U4h, lag compensation and aim (depends on U4g and on U4c's C2; size M; its own brief)
- **Builds:** `HitHistory`'s sizing and `rewind_at`'s rules (§4.9); the view-time estimate with the pump's RTT; the
  switchable cap; the per-shot log of its parts; `AimCast` through `World::ray_cast` on the published interpolated
  movers; the TC shooter's `Behaviour` (a page-like 60 Hz frame loop), the tolerance-edge stamper and the 150 ms pong
  delayer; the figures `view_hit_registered_share`, `false_hit_share`, `rule_cap_set_by_rule` (fixtures only at U4) and
  `throw_desync_count`, with fixtures.
- **Order inside (T14):** measure the largest honest rewind at P3 and P4, both options, on **calibration seeds 101-103**;
  round it up to ticks and add the margin (data): the test cap; size HitHistory from it (recorded: -7). Then the judged
  test-cap runs on **seeds 1-3**, with the setup check `clipped == 0` able to fail.
- **Marks moved:** -48 (TC, P0-P3, A and B, 2,000 shots per profile; bites §4.14 and no-interp-delay); -49's test-cap
  parts only, PASS **unbitten** until R3's rule cap (its bite cap-estimate-only waits: marks F24); -50 (P0, P1, and the
  recorded WebSocket leg with the ping RTT; bite no tolerance check); -51 (thin targets, 1,000 shots per height and
  profile; bite a parallel ray plus a 10 cm fire-origin error); -54 (1,000 throws per kind; bite a 1/60 s client step).
  -56's items (6) and (10).
- **Reviewers:** netcode-architect, security-auditor (the rewind's bounds), test-engineer, rust-reviewer.

### Lanes (at most 3 agents at a time, in worktrees, merged one at a time)
- **Step 0:** U4s alone (the main session or one builder), merged before anything else. It makes every manifest and
  Cargo.lock change wave 1 needs, so no wave-1 lane touches Cargo.toml or Cargo.lock. It also adds every wave-1
  crate's `bites` feature and npt-lab's forwarding to them (the advisor's condition 10 (a)), and the wave-2 manifest
  lines U4e's crates need (npt-room's and npt-client's `bites` features with npt-lab's forwarding; npt-server's dev
  dependency on npt-client), so that in wave 2 only U4d2 edits crates/lab and Cargo.lock (condition 10 (b)).
- **Wave 1:** U4a, U4c and U4d as three agents; U4b and U4j in the main session (or the first free lane). File
  ownership: U4a crates/proto and check.mjs; U4c crates/l1; U4d crates/sim; U4b crates/server/src/schedule.rs and
  server/clippy.toml; U4j crates/judge, crates/log and marks/. No file is shared. U4c's L-c8 judged re-run waits for
  U4j's merge.
  - **Freeze the seam's shapes for wave 1.** Through wave 1, U4d keeps unchanged every existing field, variant and
    width of `Command`, `Rewind`, `CoverVariant`, `EntityKind` and `EntityRecord` (FIELDS 12), and the five grid
    fields as built at f080551. It may add items (World, carry 17's button constants, `TickInput`'s new fields),
    complete `layout()` within FIELDS (carry 12), and drop `InputReports::non_finite` (NF-D4). A change to an
    existing item waits for U4d's merge and goes to the main session, as carry 1 already says for the grid. Why: U4a
    builds its codec and field tables against these types at the same time, so a silent change breaks the second
    merge. The words are the ruling's condition 3; the carries are u4s-carries.md's
    (the advisor's U4s ruling, condition 1 (d)). **Lifted for one change (the advisor's W1-S1 ruling, 2026-11-10):**
    U4d's fix pass appends one owner-only kind to `EntityKind` (last in `ALL`), holding velocity, the ammunition, the
    fire timer, the reload left and the move-speed and fire-interval attributes, and takes velocity and the weapon
    state out of SOLDIER's layout. `EntityRecord::FIELDS`, `Command`, `Rewind`, `CoverVariant` and the five grid fields
    stay frozen. U4a's half (the kind in both GROUPS tables' own-state rows, the codec's refusal of a kind its section
    does not list, and its test) is the main session's, in U4d's merge commit, after U4a has merged.
- **Wave 2:** U4d2 (after U4d) and U4e (after U4a, U4b, U4d) in parallel: U4d2 owns check.mjs, Cargo.toml's members,
  Cargo.lock, crates/hash-wasm and npt-lab's `hash`; U4e owns crates/room, crates/server (beyond schedule.rs) and
  crates/client. In wave 2 only U4d2 edits crates/lab, its Cargo.toml included; U4e's tests live in its own crates
  (Loopback as a dev dependency: npt-server's tests, through its npt-link and the dev dependency on npt-client U4s
  adds), never in crates/lab (the advisor's condition 10 (b)).
- **Then** U4r (after U4c, U4d2, U4e, U4j) and its runs; then U4f, U4g and U4h in order (they share the room's and the
  client's files: never two at once).
- After each merge the next lane's base moves: its brief's counts are deltas, and the merge step recomputes the
  absolute counts on the merged tree.
- **Reviewers count toward the 3-agent limit** and run at effort xhigh, as netcode work (the advisor's condition
  10 (c)): a review round starts only when enough of the three slots are free.
- **The main session builds U4b and U4j only on committed state,** never with uncommitted edits while it merges a lane
  (the advisor's condition 10 (d)).

## 6. What U4 does not build
- **U5:** fuzzing every message type and the room-wide controls (-34); the counting allocator and the allocation half
  of catalogue item 3 (Q8); the audit of libm and every other U4 dependency (-35).
- **U6:** the tokio runtime, `TickLoop`'s thread, `PhaseTimers`, `ControlSocket`'s socket, the metrics, the panic catch,
  npt-server's binary (server/src/main.rs:3-6); WtServer, WsServer and their clients; -30's marked C part on both real
  transports; `+ Send` on L2 links; token minting; **AddrTable and the handshake cap** (their only mark is -25: A15);
  `loss`, the send-buffer space and rate adaptation (Q6); npt-bots' npt-net glue and npt-load, with the clock bans'
  reviewed allows (§4.11).
- **U7:** npt-client-wasm, wasm-bindgen, wasm-opt, web/, `BlockWriter`, `mode`, the hash check in Chromium (it loads
  U4d2's file); C-U7-1: U4d2's check rerun on npt-hash-wasm passed through the shipped wasm-opt step and flags, or why
  not (§2; the advisor's condition 8).
- **U8 and the R runs:** every L2, RS and ML part; PM-0004-67's judged run; -49's rule cap; -42's frame check and its
  bite; every time mark; the scene selects of the marks §4.12's table leaves unchanged.
- **Its own small unit after U4r:** fixes.rs's run-structured bound (advisor-ws-docs-r2.md:383); it writes exactly the
  machine-readable line's format U4j fixes and reads it back with the judge's parser (nit f).
- **Two bites that cannot fail at U4, carried by name:** PM-0001-42/smoothing-off (R7) and
  PM-0001-49/cap-estimate-only (after R3).

## 7. Build rules
### 7.1 Mustafa's shared-target rule (2026-11-09; checkpoint :404-413; as u3d-plan.md §7.1)
1. Every cargo call in each builder's worktree and each bite's scratch copy runs with
   `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target`; checked once after the first build.
2. Touch before each build: `find <copy>/spikes/netcode-proof/crates -type f -exec touch {} +`.
3. Each run's test count matches the brief's delta; `--list` names the new tests; a bite run shows its own test
   failing by name; a mismatch means another copy's artifact ran: touch, rebuild, never report that run.
4. After the last bite, touch and rebuild before any further claim.
5. Cargo's own lock serializes two copies' builds: expected.
6. The main folder keeps its own cache (.gate/netcode-proof-test/target): the merge checks, the gate and every judged
   L1 run happen there, with `CARGO_TARGET_DIR` unset.
7. No copy cleans the shared folder.
8. Every brief names the folder, items 2 and 3, and .claude/rules/netcode.md as binding on proto, room, client and
   server code.

### 7.2 Who builds, and how
- Each sub-unit: one builder (general-purpose, opus, effort xhigh) in an isolated worktree, or, for U4s, U4b and U4j,
  the main session or one builder in a free lane (the advisor's U4s ruling, condition 1 (c)); the bites planted by a
  scratch script under §7.1; merged by the main session, one at a time.
- Judged L1 runs: the main session, in the main folder, on release builds, one set at a time, in the background;
  raw logs under .gate/, verdicts and runs.txt committed under results/.
- **Public items (A19):** each report lists the pub items it adds, by crate, against §4.1; engine-architect checks them
  in review. check.mjs does not check narrow surfaces (check.mjs:1134-1135), so review is the check.
- One review round per sub-unit on its final diff, then a short report to the advisor; U4's report folds them.

## 8. Risks
1. **Size and drift.** Twelve sub-units, each with its own merge, review and report; U4e, U4r, U4f, U4g and U4h with
   short briefs the advisor checks (Q21).
2. **C1' moves L1 under U2's evidence.** L-c8 re-runs -3's set and compares the datagram entries by hash.
3. **Bites that cannot bite by construction.** §4.14 gives each its site, its failing check, its run setting and a unit
   proof; the hash-map bite's order digests show whether it was exercised.
4. **Run volume.** About 285 L1 runs (nit a), most 18,000 ticks with 48 clients; U4r measures ticks per second first and states
   the wall time before the full set. Evidence is never shrunk to save time.
5. **The stress scene may not fit the budget** (about 690 B plus up to 350 B of resends against 1,000 B per datagram and
   1,066 B per tick). U4e and U4r measure; a scene that cannot fit goes to the advisor before any codec mark counts.
6. **Determinism outside the sim.** §4.11's bans and the runner's twin test.
7. **Selectors that find no data** make a part NOT-RUN; a validity rule with no inputs passes vacuously. §4.12's
   contract test covers both.
8. **A third unsafe exception (Q3 B):** the attribute only, integer signatures, deny plus listed allows, a scan.
9. **Float code in the client** (Predictor, Interp, ClockSync) is not under the sim's bans. Only npt-sim's state is
   hashed; prediction runs npt-sim's functions; the skip compare is bitwise; the client-held hash re-grids the floats,
   so a float slip there shows as a mismatch.
10. **The own-copy snap (Q23)** changes the stand-in's movement to a 1 cm grid. It is a throwaway sim; the 60 Hz run and
    the real sim make their own call.
11. **Plan text moves** (plan §4.4's clamp and per-tick inputs, §4.7's HitHistory and SimCore lines, E7's units, the
    proto → sim edge): each lands in plan.md in the sub-unit that needs it, before its code merges.
12. **The `PRE-` id** is dropped (the advisor's condition 2): -30's pre-check is a recorded npt-lab test whose headers
    list no mark, so it can never reach a mark's verdict, and the judge gains no new concept.

## 9. Change type and evidence rule
- **Change type:** netcode (protocol, simulation, room, client, the L1 model), plus a judge change.
- **Evidence rule:** deterministic tests with injected latency, jitter and loss: L1's P0 to P4 and B1 on simulated time,
  fixed seeds, 3 seeds per part, each bite failing; verdicts by npt-judge; same-seed identity by the twin tests.
- **The run count (T21), by marks.json's `min` and §5:** about 196 measured L1 runs (-17 12, -58 6, -18 3, -20 6, -21
  24 plus 12 twins, -22 3, -41 30 with both options, -42 18, -60 3, -45 9, -47 3, -61 3, -62 3, -48 24, -49's P4 test
  cap 6 beyond -48's, -50 7, -51 12, -54 3, -55 3, the pre-check 6); 12 calibration runs (U4h); 44 bite runs (46 bites
  less the two carried); 35 for -3's re-run (U2's whole set: 15 measured runs, 15 twins, 4 bites and the clock-seed
  twin: checkpoint :269-271; nit a): about 285 in all. Each brief gives its exact list.
- **Gate:** check.mjs green on both builds with the stated counts; clippy host and wasm32 on both builds; fmt;
  `node tools/gate.mjs --no-smoke` (symbols +0 -0); Cargo.lock changed only by named dependencies; all under §7.1.

## 10. Questions for the advisor
Plain words first, then the options, then my recommendation. None is Mustafa's to decide before U4 builds. Where an
answer is a reading of an ADR row's words (Q11, Q16's clamp, Q17, Q22's -22 exception, Q24, and Q26, a reading of
ADR 0001:1589's stand-in sentence; Q10 is off the list: the advisor's condition 11), the words go into the readings PR
before R5 on his go (checkpoint :320-324). The advisor's wave-1 rulings add two readings, in their words: beside Q16's
clamp, "PM-0001-21's 'a run with only their valid parts' (ADR 0001:1611), for a refused press, is the same stream with
that press removed: the button's counter one lower from the refused command on (W1-U4D)"; and "PM-0001-58's 'the
snapshot's tick offsets' (ADR 0001:1607) read as the snapshot header's two command acknowledgements (16-bit
sequences); the remaining tick offsets, the baseline's age (at most 31: §4.5's ring) and the event age (8 bits), never
cross a wrap, so they get no bite (W1-N2)". Changed in rev 2: Q2, Q3, Q4, Q5, Q9, Q10, Q11, Q13, Q15, Q16, Q21, Q22;
new: Q23 to Q26.

- **U4-SEAM (Q1), where the sim trait lives.** (a) In npt-sim; npt-room generic over it, depending on npt-sim. (b) In
  npt-room, so npt-sim depends on npt-room, against ADR 0002:186-189. (c) Trait and implementation in npt-room: the
  60 Hz swap then edits the room (plan.md:552). **Recommend (a)** (arch F11, F19).
- **U4-HIT (Q2), how a lag-compensated shot crosses the seam, and who owns the volume ring.** (a) The room turns each
  firing command's view time into four integers (tick_a, fraction, cover variant, and their presence); the ring type
  `HitHistory` moves from npt-room to npt-sim, the room owns its instance and lends it to `step`, and the replay driver
  rebuilds it from `hit_volumes()`; `rewind_at` stays in npt-room. plan §4.7 moves one name. (b) The same, but the
  World owns the ring inside itself, outside the hash: a simpler seam, but every World, the client's predicting one
  too, carries up to 1 MB, and the sim sets a size the room's cap decides. (c) Rev 1's reading, two history entries by
  value: about 288 KB per tick in a recording (arch A1). **Recommend (a).**
- **U4-WASM (Q3), which Wasm file Node runs.** (A) npt-client-wasm and wasm-bindgen brought forward from U7. (B) A
  check-only crate over npt-sim with integer-only exports: a third named exception, for the attribute only, denied with
  a listed allow per export (npt-tcpinfo's pattern); U7's Chromium check loads the same file. (C) A wasm32-wasip1 binary
  under Node's WASI: not the browser's target. **Recommend (B)**, medium confidence; (A) if you want one Wasm file from
  the start.
- **U4-REPLAY (Q4), what the recordings hold.** (a) Each tick's `TickInput` (commands with their integer Rewinds;
  spawns, teleports, attribute changes): about 17 MB per 9,000 ticks (the advisor's F1 ruling), regenerated at check time from committed seeds and
  checked by committed SHA-256s (§4.15). (b) Whole datagram streams replayed through room and sim: the Wasm file then
  needs the room. **Recommend (a).**
- **U4-SERVER (Q5), npt-server's and npt-bots' U4 halves.** (a) U4 builds their transport-free, clock-injected halves;
  AddrTable and the handshake cap move to U6 with -25, their only mark (arch A15). (b) Put them provisionally in
  npt-room or npt-lab. **Recommend (a)**; `BlockWriter` moves to U7.
- **U4-LOSS (Q6), your C3.** No U4 mark reads a loss estimate (its consumer is rate adaptation, judged by -23 and -24
  on L2: ADR 0001:1613-1614; L1 has no rate limit: l1/src/lib.rs:42-43). (a) Build `loss` at U4 with a unit-tested
  pressure path. (b) `loss` and the send-buffer space join Link at U6 with their consumer. **Recommend (b).**
- **U4-CTL (Q7), the room-wide control types.** (a) The types and the decoder in npt-proto at U4, unfeatured: npt-client
  never calls them and unreferenced code is dropped at link time, which U7's Wasm size shows (arch N5); a `control`
  feature is the fallback. (b) Item 6 and its fuzz target move to U6. **Recommend (a).**
- **U4-ALLOC (Q8), catalogue item 3's "refused before anything is allocated".** (a) U4 proves the size-first refusal;
  the allocation count joins PM-0001-34's fuzz crate at U5. (b) A counting-allocator dependency in proto's tests. (c) A
  fourth unsafe exception. **Recommend (a).**
- **U4-BANS (Q9), determinism beyond npt-sim.** (a) The HashMap and HashSet type bans in room, client, server, lab and
  bots; the clock bans in server, lab and bots naming std's and tokio's `Instant` (U6's runtime half then carries
  reviewed per-item allows); npt-room float-free, by a check.mjs grep. (b) Bans in room and client only. **Recommend
  (a).**
- **U4-30 (Q10), "-30 on loopback" at U4, and how it is judged.** Both real transports are U6's. (a) A pre-check under
  its own suite `precheck-30`, on a Loopback leg (layer `loopback`, new in README) and an L1 P0 leg, judged as
  `PRE-0001-30` from marks/prechecks.json (the judge takes `PRE-` ids only from a file marked as pre-checks), its bites
  shown by the same twin figure and `late-zero` rule, results in results/u4r-precheck-30/: never a -30 part. (b) The
  same runs, judged only inside npt-judge's tests as fixtures (no second file). (c) A provisional part in marks.json's
  -30 (it stays in -30's verdict until removed). (d) All of -30 at U6. **Recommend (a).**
- **U4-MIRROR (Q11), what -17's two sides hash.** (a) §4.6: a whole-tick hash only on complete ticks; the sentinel for
  a refused group; the rebuilt 32-bit tick and event numbers inside; the client's held floats re-gridded; a client hash
  with no server partner counts as a mismatch. (b) One hash per group with a per-group mirror: more pairs compared under
  loss, but a per-group `hs` format and more judge code. (c) The literal world state of the client's set, which
  mismatches whenever an entity is skipped. **Recommend (a)**; SERIES.md:266 follows.
- **U4-BASELINE (Q12).** (a) One baseline per tick, the newest completely acknowledged, ages 0 to 31, the defaults
  beyond. (b) A baseline per group. **Recommend (a).**
- **U4-WINDOW (Q13).** (a) One 256 kbps window per client, owned by the room, counting the pump's pings and pongs, which
  the pump reports each tick (plan §4.4's per-tick inputs gain them). (b) The encoder alone at 1,066 B per tick, which
  breaks -12's window by design. **Recommend (a).**
- **U4-EVENTS (Q14).** (a) The queue cap from the stress scene's measured event rate, resync then Kick (the bound 2 s),
  third-party hits as resent gameplay events (ADR 0001:297-303 as I read it). (b) Third-party hits sent once as
  cosmetic events. **Recommend (a)** for the spike; (b) is a design change, for your reading.
- **U4-ACKS (Q15), C1' details.** (a) Acks immediate, lost inside bursts only, drawn from their own per-flow stream and
  logged; the fate-log tag moves to v2; L-c8 compares SHA-256(v1 tag + v2's datagram entries) with U2's hashes.
  (b) Acks also lost at the profile's rate outside bursts. (c) Keep the tag at v1. **Recommend (a).**
- **U4-TWIN (Q16), -21's twins.** (a) §4.10: fate- and ack-aligned twins (a voided datagram's slot repeats the previous
  ack), open-loop scripts in the forged and clamp suites, the catch-up kind's extra datagrams mirrored in the twin and
  inside the bucket, the clamp twin built from the forged run's clamp events; no forged kind closes the session.
  (b) Twins matched by command count only: fates shift. **Recommend (a).**
- **U4-TARGETS (Q17).** (a) TC's static targets are thin plates. (b) Solid targets, -51 reading the first surface the
  ray meets. **Recommend (a).**
- **U4-49 (Q18).** -49's rule-cap parts and its bite, and -42's bite, cannot run at U4. (a) Carry them by name; -49 and
  -42 stay NOT-RUN past U4, their U4 parts labelled unbitten or check-less. (b) Build cheat shooters into the test-cap
  parts now. **Recommend (a).**
- **U4-UNITS (Q19).** (a) npt-client's API in integer ns, E7's text changed. (b) Keep E7's ms. **Recommend (a).**
- **U4-18 (Q20).** (a) -18's split, `burst_heal` and `burst_return`, and `ws-practice` with its fixtures, all in U4j
  beside C4. (b) The split now, `ws-practice` with R3's run plan. **Recommend (a).**
- **U4-BRIEFS (Q21).** (a) This plan is the umbrella; U4s, U4a, U4b, U4c, U4d, U4j and U4d2 build from it; U4e, U4r,
  U4f, U4g and U4h each get a short brief (field tables, budgets, run lists with counts) that you check before its
  build. (b) Everything from this plan. **Recommend (a).**
- **U4-SCENES (Q22), your item from the session (checkpoint :325-328).** (a) §4.12's table: -17, -18, -20 and -21 gain
  scene SS, clients 48 and `pre:PM-0001-2`; -22 gains scene and clients only, since its designed drop leaves 47
  clients; the marks with no U4 L1 part stay for the R run plans. (b) Leave all of it to the R run plans. **Recommend
  (a).**
- **U4-OWNSNAP (Q23, new), how the own-state section stays exact.** ADR 0001:207-208 lets it go "unquantised, or the
  server quantises its own copy the same way each tick". -41's bite, "the server quantising its own copy differently
  from what it sends", has a site only in the second. (a) The sim snaps each soldier's own state to the wire grid at
  the end of every tick, in `step` and `predict` alike; the grid lives in npt-sim and npt-proto depends on npt-sim
  (ADR 0002:188 lets the protocol crate use sim types). (b) An unquantised own-state section: the bite then has no site,
  its run passes, and -41 comes out FAIL. **Recommend (a).**
- **U4-41RUN (Q24, new, a reading).** -41's other bite is "the server's run speed 1 % above the client's"
  (ADR 0001:1636). (a) The soldier's movement speed, 1 % higher in the authority step than in `predict`: prediction is
  wrong on every moving tick while every command is on time, so the untraced-corrections check fails. (b) The server's
  tick rate 1 % fast: the clock sync's slew follows it (ADR 0001:239-242) and any late command is traced, so the bite
  would likely be missed. **Recommend (a).**
- **U4-CLAMP (Q25, new), where the clamp beyond reach runs.** (a) Inside npt-sim's shot phase, on integer cm, from the
  fire origin the shot really uses; plan §4.4's room column marks it "(in the sim)", as it does cooldown, ammo and
  reload. (b) Arch A3's proposal: a pure npt-sim function the room calls before `step`, with a fire-origin read-out;
  that origin is the one before the tick's movement, up to one tick's movement from the shot's own. **Recommend (a).**
- **U4-L1BURN (Q26, new).** S1's 1 ms burn and 2.80 ms top-up cost 50.4 s of CPU per 18,000-tick run (arch A17), about
  3.8 h over U4's runs, and L1 judges no time mark (ADR 0001:1466-1467). (a) Off on L1, recorded in
  `params.stand_ins`; the 384 effect rows stay, since they are state. (b) On everywhere. **Recommend (a).**

## 11. For Mustafa
Nothing now. Later, your go on the readings PR before R5, which carries the words of the eight readings the advisor
ruled (the advisor's condition 11; its W1-U4D and W1-N2 rulings): what both sides of the client-held hash cover (Q11);
the clamp twin's points at the weapon's reach for fire origins inside the grid's range, as in every scene (Q16's
clamp; W1-U4D TUNE-2); PM-0001-21's 'a run with only their valid parts' (ADR 0001:1611), for a refused press, is the
same stream with that press removed: the button's counter one lower from the refused command on (W1-U4D); the
course's targets as thin plates (Q17); -22 running without the stress-scene precondition (Q22's -22 exception); -41's
"run speed" as the soldier's movement speed (Q24); S1's stand-in CPU burn off on L1, a reading of ADR 0001:1589's
stand-in sentence (Q26); and PM-0001-58's 'the snapshot's tick offsets' (ADR 0001:1607) read as the snapshot header's
two command acknowledgements (16-bit sequences); the remaining tick offsets, the baseline's age (at most 31: §4.5's
ring) and the event age (8 bits), never cross a wrap, so they get no bite (W1-N2). Q10 is off the list. A third
unsafe exception (Q3 B) is the advisor's call (plan.md:430-438).
