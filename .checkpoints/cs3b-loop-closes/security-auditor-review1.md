# security-auditor, change-set 3b round 1 (head bbbf09e4, base 28254a73)

Copy: /home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/security-auditor-cs3b (detached at bbbf09e4).

## Threat model (3b)
- Nothing hostile reaches the decoders in 3b (one Worker, loopback). Reviewed as what they will face online:
  room = any client (modified, replayed, scripted); endpoint = a server that may be hostile or broken.
- Page <-> Worker: a Worker whose script is not ours may post anything; the page must not read out of bounds,
  throw on a block, or write markup. The Worker guards page posts before they reach Sim::input.
- The seed: never leaves the Worker's Rust state (ADR 0004 part 8): no message, block field, log, panic text,
  Debug output or whole-state hash/bytes to the page.

## Plan
1. Read deps notes, netcode-architect-review1, engine-architect-review1 (no repeats; disagreements stated).
2. Manifests: git diff 1ee3e918 bbbf09e4 on manifests and lockfiles must be empty.
3. protocol: wire.rs, codec.rs, wrap.rs, loopback.rs, transport.rs: panics, arithmetic on decoded values,
   allocation caps, work per message, all-or-nothing decode.
4. room: lib.rs, queue.rs vs ADR 0001 part 5 checks; hunt more "run twice / hold queue / starve / lie" cases.
5. client-endpoint: endpoint.rs, press.rs, post.rs, block.rs, layout.rs: hostile-server inputs.
6. Seed: world.rs, OwnState, block, sim-wasm lib.rs/local_match.rs, worker messages/host, report, Debug derives.
7. Page: keyboard.ts, boot.ts, keys.ts, report.ts, worker-link.ts, press-tally.ts: DOM writes, bounds, prod reach.
8. Worker guards: messages.ts input post (six numbers), block startAt.
9. Test unsafe: decoder_inputs.rs, step_alloc.rs allocators; allow(unsafe_code) confined.
10. Run: cargo test, clippy, release decoder test, targeted probes (scratch tests in my copy only).
11. Slice 3 list. Answer <= 40 lines.

## Findings (one line each, as found)
- I7 manifests: git diff 1ee3e918 bbbf09e4 on *.toml, Cargo.lock, package*.json = 0 bytes; the six files at head
  have exactly the six sha256 values of my deps answer (F0); outside crates/packages only notes + the batch changed
  (no .github, tools, gate files). PASS.
- I1 read: wire.rs reads only through split_at_checked / split_first_chunk; codec.rs checks every count and length
  against its cap before reserve/copy (token <= 1024, mode id <= 64, acks <= 4, commands <= 8), copies only after
  finish(); tick gaps checked_sub; snapshot flags refuse unknown bits; Trailing on extra bytes. No plain arithmetic
  or index on a decoded value found (agree with netcode Q2). Loopback: bounded slots, no allocation after new.
- I6 test unsafe: unsafe appears only in crates/{protocol,sim-wasm,sim}/tests/*_alloc*/decoder_inputs.rs (integration
  test crates, never linked into a library or the Wasm); each has #![allow(unsafe_code)] at its own crate root and a
  SAFETY comment per block; workspace lint unsafe_code = "deny" (Cargo.toml:20-21) untouched. Sound: every call
  forwards to System with the caller's arguments; count() cannot unwind (try_with, Cell, wrapping_add); std's
  GlobalAlloc docs (doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html, Re-entrance): "std::thread_local" is
  "guaranteed to not use #[global_allocator] to allocate", and "It's undefined behavior if global allocators
  unwind" (they do not). PASS.
- I3 seed, read: World and MatchSetup have no Debug (world.rs:23-25, 48-49); write_state/state_hash/shared_hash
  are called from tests only in 3b (grep, non-test code: none); the block holds tick, layout hash, echo, link counts
  only (block.rs:65-79); bindings export block_ptr/len, input, step, rebase, tick_hz, panic_buffer (no state, hash
  or seed accessor; Sim::local() is not #[wasm_bindgen]); host.ts copies exactly block_len bytes from block_ptr
  (read-block.ts) and transfers that copy, never the memory; CreateError texts carry lengths and numbers only;
  reachable panic texts (local_match.rs:173,176; world.rs:93) carry no state; worker.ts zeroes its seed array.
  PASS (to confirm by probes: the Wasm's export list; no Debug on Room).
- SOURCE S1 (read 2026-11-25; shipped game's code, Valve's Source SDK 2013, src/game/server/player.cpp, raw on
  raw.githubusercontent.com, first 100,000 of 265,981 chars): `static ConVar sv_maxusrcmdprocessticks(
  "sv_maxusrcmdprocessticks", "24", FCVAR_NOTIFY, "Maximum number of client-issued usrcmd ticks that can be replayed
  in packet loss conditions, 0 to allow no restrictions")`. CBasePlayer::PhysicsSimulate: "// Grant the client some
  time buffer to execute user commands" m_nMovementTicksForUserCmdProcessingRemaining++ (one per server tick);
  DetermineSimulationTicks: commands simulated are capped at that budget; when the budget exceeds nMaxTicks ("Client
  %s dropped too many packets, simulating last cmd") it runs a copy of the last command and "// still make sure to
  avoid speedhax" clamps the budget to nMaxTicks. So the engine ADR 0001 OQ-0001-6 (a) cites caps the BANK, not only
  the per-tick count. Confidence: high (code read). ADR 0001 OQ-0001-6 (b) (:1161-1162): "late commands dropped and
  catch-up limited to the redundancy window". The netcode design (netcode-architect-cs3.md:168-169): a late command
  "still runs (counted)", no lower bound. The built room: same (queue.rs:99-107 takes any due command; lib.rs:195 has
  an upper lead bound only).
- PROBE crates/sim-wasm/tests/sa_probe_bank.rs (my copy, untracked; debug build, 2 passed): HOSTILE client sends
  nothing for N ticks after the join, then two past-dated commands a tick (consecutive sequences and ticks from the
  join on, one press per button each): N = 30 / 300 / 3,000 -> 30 / 300 / 3,000 room ticks with 2 commands run,
  each run unbroken; refused 0, stale 0, queue_full 0; commands_run equals the room's ticks (the ADR's total bound
  holds), but the rate is double for as long as the client idled; 2 presses per button applied per room tick
  throughout. HONEST real Endpoint over an outage of the same lengths (both ways dropped, a press a tick): 7 room
  ticks with 2 commands run in each case (the redundancy window), refused 0, missing N-7. Also honest: the first
  command after the outage applies 25 / 39 / 179 presses of one button in one room tick (cumulative counters,
  PRESS-PACE (a): nothing lost), within the ADR's per-covered-tick rule.
- FINDING M1 (Medium) room has no bound on how late a command may run: queue.rs:99-107 takes any command whose tick
  is <= running however old; lib.rs:195 bounds the lead only; admit (lib.rs:219-235) compares with the last RUN
  command, so a backlog dated from the join passes every check. Exploit: idle (or lag-switch) 60 s, release a
  past-dated backlog -> the room runs 2 commands a tick for 60 s; once commands carry movement (change-set 4) that is
  double speed for the idle time. ADR 0001 part 5 calls the catch-up cap "the lag-switch guard"; OQ-0001-6 (b) has
  "late commands dropped and catch-up limited to the redundancy window"; Source (S1) clamps the bank ("still make
  sure to avoid speedhax"). Fix: drop and count (outside refused(), like stale) a command whose tick lies more than
  REDUNDANCY_CAP (8) ticks before `running`, or a Source-style budget (+1 per room tick, capped at 8, -1 per command
  run); a test: idle 3,000 ticks then the backlog -> at most 8 ticks with 2 commands; the honest outage cases keep 0
  refused. Not the same as netcode F2 (one sequence twice before the first take, nothing gained): this one gains rate.
- RUNS (my copy, bbbf09e4 + my untracked probe): debug protocol 19 + decoder 8, room 12, client-endpoint 25, sim 17 +
  rules 5 + alloc 1, sim-wasm 12 + panic_hook 1 + step_alloc 1 (+ my probe 2): all pass. Release decoder_inputs: 8
  passed in 6.46 s, per type 10,000,000 random + 10,000,000 mutated, 0 panics; largest single allocation Join 1024,
  JoinAccepted 64, JoinRejected 0, Input 0, Snapshot 0; plants: unchecked index at input 17, over-allocation at
  input 0 (identical to netcode's run: same seeds). cargo clippy --workspace --all-targets: the one warning is in my
  probe; shipped code 0 (the denied lints included).
- I2 room vs ADR 0001 part 5 (:231-239) + catalogue (:682, :685): PRESENT lead <= 8 (lib.rs:195), strictly newer
  tick (lib.rs:224), sequence forward after the first take (queue.rs:69-74), presses <= covered ticks (lib.rs:228-
  232), trailing/unknown refused (wire.rs:67-73, codec.rs:369), catch-up cap 2 <= 8 (lib.rs:40, :243), ack group 0
  and not after the newest sent (lib.rs:173-185). PARTIAL ack: no lower bound (netcode F5; agree; the join tick is
  the bound: snapshots start at joined_at + 1). ABSENT move length <= 1, aim inside the map, view-time tolerance,
  fire/ability/reload edges vs cooldowns, Input rate limit, Join token (stated lib.rs:131), Join within 2 s.
  Stated in code with a reason: the token only (lib.rs:131); move/aim/view time are stated in the sim
  (world.rs:32-33 "join when a system reads them"), not at the room's check site (lib.rs:199-206 drops them
  silently); the netcode design's sentence "Not checked yet, with the reason: aim inside the map (no map), view time
  (no lag compensation), cooldowns/ammo (no weapons), rate limits (no network: slice 3)" (netcode-architect-cs3.md:
  165-167) did not reach the code.
- FINDING L1 (Low) room/lib.rs:165-166 + :199-206: the deferred checks are not stated where they will go. Fix: one
  doc paragraph on on_datagram naming each check not made yet and why (move length and aim: enter with the record's
  movement, change-set 4; view time: lag compensation; edges vs cooldowns: weapons; rate, size, Join deadline: the
  connection layer, Slice 3; the ack's lower bound).
- I2 more hunting: hold the queue (a low sequence dated running+8 at the head, queue.rs:102) stalls only the
  sender's own commands <= 8 ticks; a queue of 16 refused commands drains in one step (<= 16 takes, bounded work);
  the snapshot cannot be made to say anything untrue about the room: newest_command_received echoes what that client
  itself sent (by design, lib.rs:189-194), last_processed/own/server_tick are the room's own; refused commands do not
  move last_run. After the first take no sequence is stored twice (all waiting lie within 32,767 of `taken`).
- I4 page: no innerHTML/outerHTML/insertAdjacentHTML/eval/Function in client/src, sim-worker/src, types/src (grep);
  keys.ts:51 textContent of constant names; overlay.ts:36 textContent, :45 value; notice textContent (boot.ts:33);
  error log strips control chars, 300 chars (error-hook.ts:15-16). worker-link.ts:236-243: layout hash (never throws,
  layout-check.ts:21-24), exact length, >= BLOCK_END (44) before reads at <= 40; a structured clone cannot deliver a
  detached buffer. Key listeners: only the six game keys' bits and counts are kept (keyboard.ts:60-61); text boxes
  skipped (boot.ts:59); nothing logged. Readout, diagram, __shambleReport behind the literal false + dynamic imports
  (boot.ts:99-149); DEV_MARKERS gains "key-diagram" (build-checks.mjs:133). Production build NOT built by me (rule);
  engine-architect read CI's eb0d2314 log: dev chunks only in development. PASS.
- I5 Worker guard: input = six whole numbers 0..2^32-1, held <= 0xFFFF (messages.ts:124-134); the bindings take u32
  and Post::from_words is total (post.rs:30-41); set_post compares with wrapping (post.rs:47-50); the pool refuses
  other sizes and caps at 4 (block-pool.ts). startAt only feeds the page's keyToTick ring (worker-link.ts:265). PASS.
- AGREE/DISAGREE: netcode F1 (refused mixes honest events) agree, it matters for detection, which reads refused;
  F2, F3, F5, F6 agree; netcode's PASS missed M1 (its outage probe was honest). engine-architect: devtools' three
  unused path edges (my L2): agree, leave the manifest, say so in the report (no outside code, devtools never ships,
  a change reopens the batch); Sim.prototype not pinned by the boundary test: agree, and it is the test that would
  catch a state or seed accessor joining the players' file (3c adds the hash export); LocalMatch's panic on a second
  JoinAccepted (local_match.rs:172-177): unreachable in 3b; the online driver must count and close instead.
- MEMORY LINES OWED (my only write is this file; for the session to add to my memory): (1) 2026-11-24 cs3b deps
  batch: yes, no conditions (see security-auditor-deps.md). (2) 2026-11-25 cs3b round 1 on bbbf09e4: CHANGES NEEDED,
  M1 (no bank bound: idle N ticks -> N ticks at 2 commands each; fix: Source-style budget capped at REDUNDANCY_CAP or
  a late bound; source S1) + L1, L2, L3. Threat-model line: commands carry cumulative press counters, so an honest
  client after an outage applies up to 255 presses of one button in one tick: every system that reads edges applies
  at most what its own cooldown allows per tick.
