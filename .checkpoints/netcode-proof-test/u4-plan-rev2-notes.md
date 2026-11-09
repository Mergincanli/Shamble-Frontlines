# U4 build plan rev 2: netcode-architect notes

Task: read both critics (u4-plan-review-test.md, u4-plan-review-arch.md) in full, check each finding against the
code, rewrite u4-build-plan.md as rev 2 with a §0.1 table (finding -> where rev 2 meets it / deferral / why wrong),
keep and update the advisor questions. No code edits, no builds, no runs. Notes by the Write tool only (checkpoint
:294-295); my first append went in by a shell heredoc before I re-read that rule; this rewrite carries it unchanged.

## Plan
1. Read both reviews in full; list every finding id here.
2. Read plan rev 1 in full (645 lines) and its notes.
3. Check each finding against code / marks.json / README / SERIES / ADRs; a line per finding here.
4. Draft rev 2 text: changes per section; §0.1 table.
5. Write rev 2 to u4-build-plan.md (Write tool), DONE line here.

## Findings log
- step 1 done: both critics' notes read (test T1-T23 one-liners + T1-T4 in the task text; arch A1-A26, N1-N5
  one-liners + A1-A4 in the task text). Step 2 done: plan rev 1 read in full (646 lines) and its notes M1-M25.
- T1 CONFIRMED: marks.json PM-0001-2 checks `snapshot_clients_count` min >= 48 and `soldiers_count` min >= 48 on
  part "SS runs" {scene SS, clients 48}; README:193-200 pre rule; eval.rs:850-887 Rule::Pre applies the matched part's
  checks. -22's checks: headroom min == 0, event.ack_starved.dropped count >= 1 (marks.json -22). A kick drops
  a client -> 47 -> INVALID under pre. Fix: -22 gets no pre (its select is suite never-ack, no scene).
- T2 CONFIRMED: -30 part C select {suite both-transports, profile P0, machine.role C}, no layer (marks.json -30);
  matched_parts eval.rs:921-930 ignores only role. README:32 layer values L1, L2, real.
- T3 CONFIRMED: SERIES.md:266 compares (client, tick) pairs; a split tick with one group lost has no defined client
  hash. Fix chosen: whole-tick hash only on complete ticks; a received-but-refused group logs a sentinel.
- T4 CONFIRMED: -17 checks = figure ==0 + S30 setup (marks.json -17); notes say "fails, never passes, on 0 compared".
  Fix: client_hash covers the rebuilt 32-bit tick; refusal sentinel; client hashes its HELD presentation state
  re-quantised through Quant (so client-rounding has a real path), property test Quant(deQuant(q)) == q.
- T5 CONFIRMED: ADR 0004:1142 PM-0004-21 "the HashMap holding at least 32 keys whose order decides an outcome, with its
  two orders logged"; PM-0004-18 (same block) "each process logs its std HashMap seed and the two differ". Fix: the
  outcome folds the whole order (not the first key); each process logs an order digest; equal digests = not exercised.
- T6 CONFIRMED in part: -21's notes already pin not-older-than to fast-clock option A (marks.json -21 notes); -62 needs
  "replays forced as in PM-0001-44" (ADR 0001:1650) but rev 1's Predictor skips matching replays (plan :444-446);
  -48 no-buffer-wait bites only "with a buffer of 2 ticks or more" (marks.json -48 bite text); -45 "one slot too short
  for P2" needs P2's measured need, not 7-1; -41 server-speed-1pct: ADR 0001:1636 "the server's run speed 1 % above the
  client's" -> movement-speed reading (the clock reading is absorbed by clock sync and traced as late); -58 lt-ack has
  no failing check unless its site is named (the baseline path falls back to defaults and decodes fine).
- T7 CONFIRMED: a voided forged datagram loses its ack too; rev 1's twin keeps "the header and acknowledgement"
  (plan :215-216) -> different baselines -> different budgets/skips -> closed-loop bots diverge. Fix: twin slot repeats
  the previous ack; -21 clients run open-loop seeded scripts.
- T8 CONFIRMED: ADR 0001:238-239 catch-up cap "no larger than the redundancy cap" = 8 = max commands per Input; one
  datagram per tick cannot exceed it, so the forged kind needs extra datagrams; alignment then needs equal counts in
  the twin (no-effect extra datagrams) and the forged burst inside the per-connection bucket (catalogue Input row: 8
  datagrams, refilled 2 x TICK_HZ/s).
- T9 CONFIRMED: ADR 0002:500-505 "a devtools replay runner, built as the server builds the sim"; rev 1 D-2 is a cargo
  test (test profile); no wall-time budget for check.mjs; recordings' source unstated.
- T10 CONFIRMED: l1/src/lib.rs:856-868 `with_bite(seed, bite)` stores the bite in the instance (no global).
- T11 CONFIRMED (structure): U4e carried judge + marks + runs; -3's re-judge (L-c8) would precede C4.
- T12 CONFIRMED: advisor-u3q.md:160-173 C1' (a)-(g) name no initial RTT before the first sample and no kGranularity in
  the time threshold; test critic's S-RFC9002 summary: §6.2.2 kInitialRtt 333 ms, §5.3, §6.1.2 kGranularity 1 ms.
  Rev 1 L-c5/L-c6 and the one-space rule have no bites (plan :317-326).
- T13 CONFIRMED: -42 parts P2/P4 have no checks (marks.json -42: checks name P3 parts and ML only) -> vacuous PASS;
  -42 L1 and -49 test-cap parts carry no bite at U4.
- T14 CONFIRMED: rev 1 U4h measures the honest rewind and then judges on the same profiles; seeds unstated.
- T15 CONFIRMED (low): test critic H1: eviction certain iff the ring keeps ages 0-31.
- T16 CONFIRMED: check.test.mjs self 43 (checkpoint :620-621); rev 1 adds check.mjs steps with no self-test bites.
- T17/T18 CONFIRMED: README:171 roster rule reads `admit` events: none written -> vacuous.
- T19-T23: wording/accounting; recount: 46 mark bites in the 21 marks + PM-0004-67's 3 = 49 (rev 1 right); L1 runs:
  ~196 measured incl. 12 twins and -41 x2 options, + 12 calibration, + 44 bite runs, + -3's re-runs: ~270, not 230.
- A1 CONFIRMED: plan.md:577 HitHistory, rewind_at in npt-room; plan.md:567 Areas, Portals in npt-sim. Recording est.
  (mine): 41 B per command x 48 + 5 = ~1,973 B/tick -> ~17.8 MB per 9,000 ticks; ~4.4 M feed(word) calls.
- A2 CONFIRMED: E5 (plan.md:452) "areas and portals flood-numbered and re-flooded on change"; seam has no read-out.
- A3 CONFIRMED: room/clippy.toml:1-9 clock ban only; rev 1 §4.10 float clamp in the room. Fix: the clamp in npt-sim's
  shot phase (in-step fire origin, integer cm in and out), clamped points as SimEvents; interest on integer cm.
- A4 CONFIRMED: plan.md:485 host.ts "at most 7 ticks per wake (from the tick constant, §4.6)"; ADR 0002:336-337 the
  Worker's catch-up at most 7 per wake; server/Cargo.toml lists npt-net (-> tokio); client-wasm -> client only. Fix:
  npt_sim::LOOP_CATCHUP beside TICK_HZ; due_ticks takes the cap as a Schedule field (U4b independent).
- A5 CONFIRMED: plan.md:544-546 every tick-counted timer derives from TICK_HZ; rev 1 states 60 ticks, 64, 30 as literals.
- A6 CONFIRMED: ADR 0001:367-368 RTT = quinn rtt() or the lowest recent server ping sample; link lib.rs:80-81
  rtt_ns -> Option<u64>. Fix: the pump hands the room one RTT per client; server's ping sender in U4e (-50 needs it).
- A7 CONFIRMED: plan.md:505-506 lists RTT, loss, send-buffer space, max datagram size, unsent count: no bytes-sent.
- A8 CONFIRMED: check.mjs:903-942 refuses lint-lowering flags only (LINT_LOWERING); nothing on +fma/target-cpu/simd.
- A9 CONFIRMED: U4a a9 edits check.mjs; rev 1 U4b adds a check.mjs grep (plan :136); both wave 1.
- A10 CONFIRMED: judge depends on npt-log only (check.mjs:90-91 "npt-judge reaches"); can be its own sub-unit.
- A11, A12, A16, A18, A25, A26 CONFIRMED as stated (text gaps). A17 arithmetic right (2.80 ms x 18,000 = 50.4 s).
- A13 CONFIRMED: check.mjs:135-137 UNSAFE_EXCEPTIONS two entries, EXCEPTION_LEVELS; :1110 WASM_CRATES = LEAF + tcpinfo;
  :24 LEAF. tcpinfo precedent :94-102 (ships libc only).
- A19 CONFIRMED: check.mjs:1132-1135 "its narrow public surfaces are not checked by this command".
- A14 CONFIRMED: room/clippy.toml bans std Instant/SystemTime only; server -> npt-net -> tokio; bots -> npt-net.
- A15 CONFIRMED: catalogue Input row: per-connection bucket 8, refilled 2 x TICK_HZ/s; Join row: 2 s deadline; both
  "built in U4" per the arch critic's note (catalogue :87, :93). AddrTable and the handshake cap: -25 only (L2/RS).
- N1 CONFIRMED: L1 "logic marks only" sits at ADR 0001:1466-1467 (rev 1 cited :1462-1463). N2 CONFIRMED (ADR
  0001:238-239). N3: the sixth stub is npt-bots' lib (bots/src/lib.rs, 3 lines). N4 CONFIRMED (plan.md:549 "pure").
  N5: DEFER with reason (types only; linker drops unreferenced code; checked by U7's Wasm size).
- Shot-band arithmetic for T1 (mine): 700/s / (48 x 30) = 0.486 shots per command-tick. B1 under option A: each 1 s
  burst loses ~22 of 30 ticks of commands (8 recovered by the redundancy cap) -> ~0.73 s of shots lost per burst; 10
  bursts in 600 s -> mean ~ 700 x (1 - 7.3/600) = 691.5 >= 665. Forged: 11 forgers x 1,000 refused commands x 0.486
  = ~5,350 shots / 600 s = ~8.9/s -> ~691. One client fully silent: 700 x 47/48 = 685.4.
- Step 3 done. Step 4 (draft) next: rev 2 written whole with the Write tool.
