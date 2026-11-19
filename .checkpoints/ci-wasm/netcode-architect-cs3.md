Task: change-set 3 ("The loop closes"), the netcode half of its plan (netcode-architect, part "cs3").

## Plan (blocks, in order)
- Block 0: cs3-context.md read (done before this file was made).
- Block 1: Read: card + Step 2.5 table (build plan 316-333, 463-476); ADR 0001 by grep (join exchange, command + press
  counters, snapshot, transport interface + loopback, L1, Worker step order); ADR 0002 (crate list, PM-0002-49/-50/-53/-64);
  code: protocol (messages.rs, ts_export.rs), room lib.rs, client-endpoint (block.rs, layout.rs, schedule.rs), sim
  (world.rs, state_hash.rs), sim-wasm lib.rs, devtools lib.rs, packages/sim-worker/src (host.ts, pacing.ts, messages);
  cs2 report; STATUS_ARCHIVE PR #61 block "For change-set 3" (NA-N1..N7).
- Block 2: Design crate by crate (protocol, room, client-endpoint, Worker step, devtools/L1).
- Block 3: Tests that prove it.
- Block 4: Size and cuts.
- Block 5: Where the build plan / drafts are wrong; questions for Mustafa.
- Then DONE, then answer.

## Notes (one line per finding)
- B1 read: build plan card 316-333, Step 2.5 table 463-476 read. ADR 0001: command contents :225-239 (tick, seq, move,
  buttons, press counter per button, aim point <=80 bits, view time; ack of newest snapshot decoded; server checks incl.
  "press counter advances by at most one edge per button per tick covered", "trailing bytes or unknown fields dropped
  whole", seq wrap handled); catalogue :682-686 (Join: token, protocol version, content hash; JoinAccepted: slot, team,
  squad, mode id, mode hash, content hash, server tick, tick and snapshot rates; Snapshot header: 32-bit server tick,
  baseline tick, newest command received, last processed command, group index); transport interface :441-446 (send
  unreliable, send reliable on a named channel, receive, link state and RTT; decoders never panic, check sizes before
  allocating; sim thread never parses raw bytes); profiles part 13 :583-600 (P0..P4, B1; half latency and half jitter
  each way, uniform jitter; loss per direction at the profile's rate; explicit reorder 1 % of adjacent pairs);
  counter-wrap test :1823-1827 (start server tick a few below a multiple of 65,536; every 16-bit counter near wrap).
- B1 read: ADR 0002 crate rows :186-194; lints :462-474 (protocol decoders: unwrap_used, expect_used, indexing_slicing
  at deny; room timers count ticks; client endpoint takes time only as host values); control API :330-346 (one input
  call + one step call per tick; input call = newest input + datagrams with arrival times + previous send time + host's
  current time; step returns where outputs lie and when next tick is due; <= 4 Wasm calls per tick incl. glue); part 6
  :402-407 the step order (endpoint builds T's command + predicts T; command via loopback; local server runs T with
  every slot's command, encodes T's snapshot; endpoint decodes, reconciles, writes block); L1 :520-526 (in one process,
  real server code + headless bot clients); PM-0002-49 :1323 (2 runs identity; 1 per profile stats; 1,000,000 packets
  per profile per direction; within 5 % relative of per-direction values of P0-P4 and B1; G11's one-minute run per
  profile under 2 minutes total; "jitter read against the distribution L1's own documentation states"); note :1357-1360
  (1,000,000 packets: band = 5 standard errors at 1 %); PM-0002-53 :1327 (planted u16 overflow with + panics in dev and
  release, native and Wasm; wrapping_add equal in all four); PM-0002-64 :1338.
- FINDING F-P0: PM-0002-49's "within 5 % (relative)" is undefined at P0 (0 ms, 0 jitter, 0 % loss): a relative band
  around zero is zero width. Needs an absolute reading at P0 (exactly 0 lost, delay exactly 0 ticks/us). (ADR 0002 :1323)
- FINDING F-B1: B1's "one burst of 1 s at 100 % loss" inside 1,000,000 packets: loss over 1,000,000 packets is then
  P2's 2 % plus the burst's share, which depends on the packet rate; the mark must say what B1's expected loss is
  (per-direction P2 loss + burst packets / total). (ADR 0001 :585, ADR 0002 :1323)
- B1 code facts (main f5030a3 = this branch's crates): protocol has messages.rs (Join{token Vec<u8>, protocol_version
  u16, content_hash [u8;32]}, JoinAccepted{slot u8, team u8, squad u8, mode_id String, mode_hash, content_hash,
  server_tick u32, tick_rate_hz u8, snapshot_rate_hz u8}, Input{acks Vec<SnapshotAck{group u8, tick u32}>, commands
  Vec<Command>}, Command{tick u32, sequence u16, move_x i8, move_y i8, buttons u16, presses [u8;16], aim AimPoint{x i16,
  y u16, z i16}, view_time ViewTime{tick u32, fraction u16}}, Snapshot{server_tick u32, baseline_tick Option<u32>,
  newest_command_received Option<u16>, last_processed_command Option<u16>, group u8}); no codec; ts_export.rs writes the
  4 messages' TS into packages/types/src/generated/protocol (any field change regenerates it). protocol has NO deps.
  room and devtools: empty stubs. client-endpoint: no deps; block.rs (BlockWriter: header tick + layout_hash only),
  layout.rs (BLOCK_LAYOUT header stride 8), schedule.rs (LocalSchedule f64 anchor; next_due_ms; ticked; rebase).
  sim: World{tick u32, seed [u8;32]}, World::new(seed) at tick 0, authority_step() checked_add, write_state (tick then
  seed, 36 B), state_hash fnv1a_64 (provisional until PM-0004-21's report, cs2 report risk 5: "before the first
  recording is committed (change-set 3)"). sim-wasm: Sim{world, writer, schedule, newest_input u32}; wasm calls:
  new(seed, mode, pad, now_ms), block_ptr, block_len, input(sequence u32), newest_input() (tests only, EA-F10), step()->f64
  (NO now: NA-N1), rebase(now)->f64, tick_hz(). sim-worker: host.ts (HostSim{input(sequence), step(), rebase, block_ptr,
  block_len}; Pacer step = input(newestSequence) then step(); visibility hidden -> pacer.pause()), pacing.ts (Pacer: no
  finite check on the due time step() returns: NA-N7; while loop `nextDueMs <= now` with NaN never runs and returns NaN
  sleep -> Math.ceil(NaN) -> setTimeout(NaN) = 0 ms busy loop), messages.ts (input = {sequence, returned}; isCount
  0..2^32-1). Page: boot.ts:120-123 `sequence` per drawn frame (NA-N5: rename to post number); worker-link.ts:121
  frame(sequence).
- B1 NA-N1..N7 text (STATUS_ARCHIVE :1085-1091): N1 step gets the host's time; N2 hidden-tab pause from the mode once
  online lands, held buttons cleared on hide; N3 who picks the blend fraction (one sentence in picture.ts now, a field at
  cs4; events read from every block); N4 seed = authority-only section written last, shared state a prefix; N5 page's
  per-frame number is not the command sequence: name it the post number; N6 a start tick in the authority's match
  setup; N7 Pacer throws on a non-finite due time; plus: checked_sub or wrap-aware compares on decoded ticks.
- B1 ADR 0004 part 8 :309-332 (seed from host, never sent/logged; streams' state part of authority's state + hash;
  every stream starts authority-only); part 9 :338-356 (recorded match = seed, map and mode data, content hash, per-tick
  inputs; first mismatching tick + field-level diff; PM-0004-21 picks the hash function and its cost per tick).
- FINDING F-ECHO: the card's row (build plan :249) wants keys "echoed back by the local server through the real
  messages" and "zero lost presses", but the Snapshot type has a header only (no own-state section). An echo needs the
  authority to hold the slot's applied input (buttons + press totals) in hashed state and the snapshot to carry it in a
  minimal owner section; otherwise the first recording hashes only tick + seed (inputs change nothing, so "same hashes
  natively and in Wasm" proves nothing about inputs).
- FINDING F-PRESS: the page posts only a per-frame number today; a tap shorter than a frame is lost unless the page
  counts presses cumulatively per button (keydown, not repeat) and posts the running counts; the endpoint must then pace
  them out at most one edge per button per tick (ADR 0001 :235's server check), carrying any backlog to the next tick.
- B1 DONE. Next: Block 2 (design, crate by crate).
- FINDING F-SHIPPED-REPLAY: ADR 0002 :506-510 says every Wasm determinism run uses "the exact file G6 ships" (after
  wasm-bindgen and wasm-opt), through a replay page that runs the sim-wasm Worker. So the Wasm replay entry point must be
  in the shipped Wasm (not a test-only build); this is not EA-F10's "test-only code" case. The hash may leave the Worker
  only in replay mode, where the seed is the recording's published test seed, never a live match's.
- FINDING F-CI-NO-CLIPPY: CI runs no clippy (toolchain.yml:84-91: cargo test debug, write-types, cargo test --release
  -p sim -p protocol, wasm32 build, npm test). So ADR 0002 :470's deny lints in protocol's decoders are inert in CI until
  2.3's lint step; in cs3 "decoders never panic" is proven by tests (overflow checks are on in every profile, so any
  overflow in a decoder panics under test). devtools' tests run in debug only in CI.
- FINDING F-LAYER: the local step needs room + client-endpoint together; only sim-wasm and devtools may depend on both
  (ADR 0002 :186-194). One step function serving the Worker and the L1 tests means devtools depends on sim-wasm's rlib.

## BLOCK 2. The design, crate by crate (DONE when the line "B2 DONE" follows)
Rules for every crate below (protocol, room, client-endpoint, the sim changes, sim-wasm's step): never panic on bytes from
a link or on values decoded from them (decode errors are returned and counted; arithmetic on a decoded value only through
checked_/wrapping_ or the wrap module, never `+`/`-`: overflow checks are on in every profile, so a plain operator on a
decoded value is a remote crash, later a server DoS); no allocation per tick in the hot path after warm-up (reused Vecs
whose capacity persists, fixed arrays and rings); no clock (time only as the host's numbers); no hash map (fixed arrays,
rings indexed by sequence). Lint attributes on the decoder modules: deny(clippy::unwrap_used, expect_used,
indexing_slicing, arithmetic_side_effects), allowed in tests (ADR 0002 :470 names the first three; the fourth is my
addition for the never-a-plain-minus rule). CI runs no clippy yet (F-CI-NO-CLIPPY), so tests carry the proof in cs3.

### protocol (no new dependency; still no dependency on sim: room asserts BUTTONS equal)
- wire.rs (new): `Reader<'a>` { new(&[u8]); u8, u16, u32, i8, i16 -> Result<_, DecodeError>; bytes(n) -> Result<&'a [u8]>;
  array::<N>(); finish(self) -> Result<(), DecodeError> (Trailing if bytes remain) }, every read through `get(at..end)`
  with checked_add; writer side: put_u8/put_u16/put_u32/put_i8/put_i16/put_bytes on `&mut Vec<u8>` (little-endian).
  Byte-aligned on purpose: no cs3 field needs fewer than 8 bits; a bits(n) reader joins with the first sub-byte field
  (the snapshot's entity sections, cs4/cs5). ~55 code, ~30 tests.
- wrap.rs (new): `seq_newer(a: u16, b: u16) -> bool` (d = a.wrapping_sub(b); d != 0 && d < 0x8000; d == 0x8000 is
  "neither", refused); `seq_ahead(from: u16, to: u16) -> Option<u16>` (forward distance 1..0x7FFF, else None);
  `ticks_after(earlier: u32, later: u32) -> Option<u32>` (= later.checked_sub(earlier): 32-bit ticks never wrap in a
  match, so "out of order" is None, never a huge number); `press_edges(old: u8, new: u8) -> u8` (wrapping_sub). This is
  NA-N1..7's extra answer: every compare of a decoded tick, sequence or counter goes through these. ~25 code, ~30 tests.
- messages.rs (changed): consts PROTOCOL_VERSION: u16 = 1; BUTTONS: usize = 16; REDUNDANCY_CAP: usize = 8 (ADR 0001
  :221 "data, proposed 8"; a const until content data exists); MAX_ACKS: usize = 4; MAX_TOKEN_BYTES: usize = 1024;
  MAX_MODE_ID_BYTES: usize = 64. New types: `JoinRejected { reason: RejectReason }`, `enum RejectReason {
  ProtocolVersion, ContentHash, RoomFull }`; `OwnState { move_x: i8, move_y: i8, buttons: u16, presses: [u8; 16] }`
  (provisional: the first entry of ADR 0001's own-state section, the command the authority last applied; the soldier's
  own state joins in cs4); `Snapshot.own: Option<OwnState>`. ts_export.rs: +JoinRejected (OwnState, RejectReason come
  along); the generated TS in packages/types changes (regenerated, counted apart). ~45 code, +3 export.
- codec.rs (new): `enum DecodeError { Truncated, Trailing, Kind(u8), Flags(u8), TooMany { what: &'static str, count:
  usize, max: usize }, TooLong { what, len, max }, Utf8, Gap }`; `enum EncodeError { TooMany{..}, TooLong{..},
  TickGap }`; `enum Control { Join(Join), JoinAccepted(JoinAccepted), JoinRejected(JoinRejected) }`;
  `encode_control(&Control, &mut Vec<u8>) -> Result<(), EncodeError>`, `decode_control(&[u8]) -> Result<Control,
  DecodeError>` (allocates the token and mode id once per match, after checking each length against its cap AND against
  the bytes left); `encode_input(&Input, &mut Vec<u8>) -> Result<(), EncodeError>`, `decode_input(&[u8], into: &mut
  Input) -> Result<(), DecodeError>` (clears and refills into's Vecs: no allocation once their capacity is reached; counts
  checked before any push); `encode_snapshot(&Snapshot, &mut Vec<u8>)`, `decode_snapshot(&[u8], into: &mut Snapshot)`.
  Wire (byte-aligned, little-endian, a one-byte kind first; one channel decodes only its own kinds):
  Join 0x01: token len u16 + bytes, version u16, content hash 32 | JoinAccepted 0x02: slot, team, squad u8; mode id len u8
  + UTF-8; mode hash 32; content hash 32; server tick u32; tick and snapshot rates u8 | JoinRejected 0x03: reason u8 |
  Input 0x10: ack count u8 (<= MAX_ACKS), per ack group u8 + tick u32; command count u8 (0..=REDUNDANCY_CAP); the newest
  command: tick u32, sequence u16, fields; each older one: tick gap u8 (>= 1, Gap error at 0; its tick =
  newer.checked_sub(gap)), fields; their sequences are implied consecutive (newest - i, wrapping), so non-consecutive
  sequences cannot be sent | fields = move_x i8, move_y i8, buttons u16, presses 16 x u8, aim x i16 / y u16 / z i16, view
  tick u32 + fraction u16 (32 B) | Snapshot 0x11: server tick u32; flags u8 (bit 0 baseline, 1 newest received, 2 last
  processed, 3 own; any other bit set = Flags error); baseline as a u16 back-offset (>= 1; checked_sub from the server
  tick); newest received u16; last processed u16; group u8; own 20 B.
  Commands are sent whole (U1), not delta-coded (U2): see Block 5. ~205 code, ~140 tests.
- transport.rs (new; ADR 0001 :441-442's four operations): `enum Side { Client, Server }`, `enum Channel { Control }`
  (named reliable channels; the full-state stream joins with FullState), `enum Delivery { Datagram, Reliable(Channel) }`,
  `enum LinkState { Open, Closed }`, `enum SendError { Full, TooLarge, Closed }`, `trait Transport { fn
  send_unreliable(&mut self, &[u8]) -> Result<(), SendError>; fn send_reliable(&mut self, Channel, &[u8]) -> Result<(),
  SendError>; fn receive(&mut self, out: &mut Vec<u8>) -> Option<Delivery>; fn link_state(&self) -> LinkState; fn
  rtt_us(&self) -> Option<u32>; }` (whole microseconds: no float in protocol; the loopback says None), `trait
  TransportPair { type End<'a>: Transport where Self: 'a; fn end(&mut self, Side) -> Self::End<'_>; }` (one object
  holds both ends so one step can drive each side in turn; L1 implements it too). ~55 code.
- loopback.rs (new): `Loopback::new(capacity_bytes: u32, capacity_packets: u16) -> Loopback`, `LoopbackEnd<'a>`, impl
  TransportPair / Transport. Two FIFO packet queues (one per direction), each a byte arena (Box<[u8]>) plus a ring of
  (offset, len, Delivery), allocated once; send copies in, receive copies out; full -> Err(Full), longer than the arena
  -> Err(TooLarge); never reorders, never allocates after new. The Worker sizes it 4 KiB / 16 packets per direction (one
  step moves at most 3 packets each way). ~75 code, ~45 tests.

### room (deps: sim, protocol; path dependencies only)
- lib.rs: `struct Room`, `struct RoomCounters { decode_errors, refused_not_newer, refused_too_far_ahead,
  refused_move, refused_presses, refused_ack, duplicates, stale, queue_full, late, missing, joins, rejections }`
  (u32 each, wrapping_add), consts LEAD_MAX_TICKS: u32 = 8, CATCH_UP_CAP: usize = 2 (<= REDUNDANCY_CAP, ADR 0001 :239),
  MODE_ID: &str = "skeleton"; `Room::new(setup: sim::MatchSetup, content_hash: [u8; 32]) -> Room`; `on_control(&mut self,
  bytes: &[u8], reply: &mut Vec<u8>) -> bool` (Join -> JoinAccepted, or JoinRejected for version, content hash, slot
  taken; local play accepts an empty token, online's token check is slice 3); `on_datagram(&mut self, bytes: &[u8])`
  (decode_input into a reused Input; each command offered to the queue newest-first; acks checked: an ack newer than the
  newest snapshot sent is refused and counted); `step(&mut self, snapshot: &mut Vec<u8>) -> bool` (takes up to
  CATCH_UP_CAP due commands, checks each against the last RUN command, runs World::authority_step with the accepted ones,
  encodes this tick's snapshot for the slot: header + own = the world's player record; false while no slot is joined);
  `world(&self) -> &World`; `counters(&self) -> &RoomCounters`. Checks (ADR 0001 :231-239, the subset with something to
  check in cs3): tick strictly newer than the last run command's (ticks_after > 0) and at most LEAD_MAX_TICKS past the
  server's; sequence newer than the last run (else stale, dropped silently: an honest late repeat) ; move length <= 1 as
  x*x + y*y <= 127*127 in i32; each button's press_edges(last_run, cmd) <= ticks covered (cmd.tick - last_run.tick); the
  first command after join is compared with the join tick and zero counters. Not checked yet, with the reason: aim inside
  the map (no map), view time (no lag compensation), cooldowns/ammo (no weapons), rate limits (no network: slice 3).
  A command for tick T is "late" when T <= the tick being run; it still runs (counted), because in lockstep under L1 every
  command is late by the link's delay, and the press counters keep its presses either way.
- queue.rs: `CommandQueue` (QUEUE_LEN = 16 = 2 x REDUNDANCY_CAP), a fixed ring of Option<(tick, PlayerCommand)> indexed by
  sequence offset from the last run (seq_ahead); `insert(seq, tick, cmd) -> Inserted { Queued, Duplicate, Stale,
  TooFarAhead }`; `take_due(&mut self, out: &mut [Queued; CATCH_UP_CAP]) -> usize`: the contiguous next ones; when the next
  sequence is missing but a newer one waits, it skips ahead at once (the newer command's counters carry the skipped
  presses; the skipped tick's held buttons are lost, which matters from cs4, where the input buffer of 1-3 ticks decides
  waits: OQ-0001-6).
- No TickTimer: nothing in cs3 has a duration (no phases, no countdown); the join deadline is the server's wall time
  (ADR 0001 :454, ADR 0002 :466-469). See Block 5.
- `const _: () = assert!(protocol::messages::BUTTONS == sim::BUTTONS);`
- Never: panic on any bytes; allocate per tick (reused Input, reused snapshot Vec, fixed queue); read a clock; hash map.
- ~185 code (lib 125, queue 60), ~190 tests.

### client-endpoint (deps: sim, protocol; path dependencies only)
- post.rs (new): `struct Post { number: u32, move_x: i8, move_y: i8, buttons: u16, presses: [u8; BUTTONS] }` (the page's
  newest input; `number` is the post number, NA-N5: the page's per-frame count, never the protocol's command sequence;
  `presses` are the page's running press counts per button, mod 256), `Post::NONE`. ~20 code.
- press.rs (new): `PressPacer { seen: [u8; B], backlog: [u16; B], counters: [u8; B], flooded: u32 }`; `new()`;
  `next(&mut self, page_counts: &[u8; B]) -> [u8; B]` (new presses = press_edges(seen, page); each button's counter
  advances by at most one per command, the rest waits in its backlog; a backlog over MAX_BACKLOG = 255 counts `flooded`:
  a human never reaches it); `backlog_total() -> u32`; `clamp_move(x: i8, y: i8) -> (i8, i8)` (inside x*x + y*y <=
  127*127 by an integer rule: the 8 keyboard directions are exact ((+-127, 0), (0, +-127), (+-89, +-89)); anything else
  steps toward 0 until inside; no float). Why: a tap shorter than a frame is never lost (the page counts it), a resent
  command never doubles it (counters, not edges), and the room's "one edge per tick covered" check never refuses an
  honest client (F-PRESS). ~50 code, ~50 tests.
- endpoint.rs (new; EA-F9: the endpoint's state leaves sim-wasm): `enum EndpointState { Joining, Joined { slot: u8 },
  Rejected(RejectReason) }`; `enum EndpointError { Decode(DecodeError), Unexpected, TickRate(u8), ContentHash }`;
  `struct Readout { post_ran: u32, own: OwnState, presses_sent: u32, presses_confirmed: u32, presses_lost: u32,
  presses_doubled: u32, backlog: u32, stale_ticks: u32, commands_sent: u32, snapshots: u32, stale_snapshots: u32,
  decode_errors: u32, host_time_bad: u32 }`; SENT_RING: usize = 64; `Endpoint::new(content_hash: [u8; 32],
  first_sequence: u16)` (first_sequence lets the wrap test start near 65,535; the Worker passes 0); `join(&self, out)`;
  `on_control(&mut self, bytes) -> Result<(), EndpointError>` (checks tick_rate_hz == sim::TICK_HZ and the content hash);
  `build_input(&mut self, post: &Post, out: &mut Vec<u8>) -> bool` (false until joined AND the first snapshot after the
  join is decoded, which sets next_tick = its server tick + 1, so the first command is for the tick the room runs next;
  then: the command for next_tick with next_sequence, counters from the pacer, move clamped, buttons from the post, aim
  0, view time {newest decoded snapshot tick, 0}; the command, its post number and its running press total enter the sent
  ring; the Input holds the ack (group 0, newest snapshot decoded) and every command newer than the room's newest
  received, newest first, at most REDUNDANCY_CAP; next_tick checked_add, next_sequence wrapping_add); `on_datagram(&mut
  self, bytes)` (decode_snapshot into a reused Snapshot; error -> decode_errors; a tick not newer than the newest decoded
  -> stale_snapshots; else: newest decoded = its tick, retire commands up to newest received (seq_newer), look up
  last processed in the ring and compare own.presses / buttons / move with what was sent there: per button d =
  press_edges(sent, echo) as i8, d > 0 doubled, d < 0 lost; post_ran = that command's post; stale_ticks += 1 when post_ran
  is not the post this step's command was built from (local play only); presses_confirmed = that command's running
  total); `readout(&self) -> &Readout`; `state(&self)`.
- block.rs (changed): writes Readout into the block's new section(s); the layout and the page's key diagram are
  engine-architect's (do not duplicate); ~30 code there.
- schedule.rs unchanged (LocalSchedule already takes host ms).
- Never: panic on any snapshot bytes; allocate per tick (reused Snapshot and Vec, fixed ring); clock; hash map.
- ~270 code (post 20, press 50, endpoint 200), ~200 tests.

### sim (world.rs; NA-N4, NA-N6; integer only, so the scan test passes)
- `pub const BUTTONS: usize = 16`; `pub struct MatchSetup { pub seed: [u8; SEED_BYTES], pub start_tick: u32 }` (NA-N6:
  the authority's match setup; the Worker passes 0; tests start near a multiple of 65,536); `pub struct PlayerCommand {
  pub move_x: i8, pub move_y: i8, pub buttons: u16, pub presses: [u8; BUTTONS] }` (what the sim reads of one command;
  aim and view time join when a system reads them); `World::new(&MatchSetup)` (tick = start_tick); `authority_step(&mut
  self, commands: &[PlayerCommand])` (the slot's commands this tick, oldest first; the player record becomes the newest;
  none: it holds, so held buttons stay held); `player(&self) -> &PlayerCommand`; `pub const SHARED_STATE_BYTES: usize =
  24`; `shared_hash(&self, scratch) -> u64`. write_state order (NA-N4): tick u32 | player (move_x, move_y, buttons u16,
  presses 16) = the shared prefix, 24 B | the authority-only section, last: the seed (32 B) now, the streams' positions
  when a system first draws (ADR 0004 part 8). A hash compared with a client covers the prefix only (ADR 0004 :667). One
  slot in cs3 (one player record); the soldier table replaces it in cs4. Known answers re-made by a second
  implementation (a script), as today. ~45 code, ~55 tests.
- F-ECHO answered: inputs now change the hashed state, so the first recording proves the loop, not a counter.

### sim-wasm: the Worker's one step (and what crosses wasm-bindgen)
- local_match.rs (new, plain Rust, tested natively): `struct LocalMatch<P: TransportPair = Loopback> { room: Room,
  endpoint: Endpoint, link: P, writer: BlockWriter, schedule: LocalSchedule, post: Post, scratch: Vec<u8>, reply:
  Vec<u8>, last_host_ms: f64 }`; `new(setup: MatchSetup, pad_bytes: u32, now_ms: f64, link: P) -> Result<Self,
  CreateError>` (queues the Join on the control channel); `set_post(&mut self, Post)`; `step(&mut self, now_ms: f64) ->
  f64`; `rebase(&mut self, now_ms) -> f64`; `authority_hash(&self, scratch) -> u64` (Rust only, never exported live);
  `block()`, `readout()`, `room()`, `endpoint()`.
  step(now_ms), in ADR 0002 :402-407's order: (0) now_ms kept if finite and not earlier than the last (else
  host_time_bad += 1; NA-N1); (1) the endpoint builds tick T's command from the newest post and the client end sends the
  Input; (2) the server end receives everything waiting (Join -> reply on control; Input -> room.on_datagram); the room
  runs T and the server end sends T's snapshot; (3) the client end receives (JoinAccepted, the snapshot); the endpoint
  decodes; (4) the block is written from the endpoint's readout (tick = the decoded snapshot's tick); (5) the schedule
  ticks; returns the next due time. So the command for T reaches the server before it runs T, and a press posted before
  step k is in the authority's state and in block k. A JoinRejected in local play panics with its reason (same build on
  both sides: a bug; the panic hook keeps the message: engine-architect's). Prediction (ADR 0002 :403) enters with the
  soldier in cs4. ~110 code, ~90 tests.
- replay.rs (new): `const RECORDING_MAGIC: [u8; 8] = *b"SHMBLREC"`; `RECORDING_VERSION: u16 = 1`; `MAX_RECORDING_TICKS:
  u32 = 108_000` (one hour); `struct Recording { seed: [u8; 32], start_tick: u32, posts: Vec<Post>, hashes: Vec<u64> }`;
  `enum RecordingError { Magic, Version(u16), Truncated, TooLong(u32), Trailing }`; `Recording::decode(&[u8]) ->
  Result<Self, RecordingError>` (tick count checked against the cap and the bytes left before allocating);
  `Recording::encode(&self, out)`; `replay(rec: &Recording, hashes: &mut Vec<u64>)` (a LocalMatch<Loopback> with the
  recording's seed and start tick, one post per step, synthetic host times, the authority's full state hash after each
  step). Format: magic 8 | version u16 | seed 32 | start tick u32 | tick count u32 | per tick: post number u32, move_x,
  move_y, buttons u16, presses 16 (24 B) | per tick: hash u64. ~80 code, ~40 tests.
- lib.rs (changed): `Sim` wraps LocalMatch<Loopback>; `new(seed, mode, pad_bytes, now_ms)` unchanged (start tick 0);
  `input(post: u32, move_xy: u32, buttons: u32, presses0: u32, presses1: u32, presses2: u32, presses3: u32)` (unpacked
  with masks, never panics: move x in bits 0-7 and y in 8-15 as i8; buttons the low 16 bits; button i's count in byte i
  of the four words); `step(now_ms: f64) -> f64` (NA-N1); `rebase(now_ms) -> f64`; `block_ptr`, `block_len`, `tick_hz`
  unchanged; `newest_input` removed (EA-F10); new free function `replay_hashes(recording: &[u8]) -> Result<Vec<u8>,
  JsError>` (the per-tick hashes, little-endian), in the shipped Wasm because ADR 0002 :509 runs every Wasm determinism
  run on the exact shipped file (F-SHIPPED-REPLAY); it runs only the recording's own published test seed, so no live
  hash or seed leaves the Worker. Per tick still two calls of plain numbers (input, step); one type; WB-HEALTH holds.
  ~50 code changed, ~40 tests changed.

### The Worker's host (TypeScript, packages/sim-worker; the page's keys and diagram are engine-architect's)
- messages.ts: the input post becomes { type: "input", post, move, buttons, presses (four u32 words), returned } (NA-N5:
  `post`, not `sequence`); the guard checks post in [0, 2^32), move's x and y in [-127, 127] packed, buttons in [0, 2^16),
  each presses word in [0, 2^32). ~25 code, ~20 tests.
- host.ts: HostSim.input(post, move, buttons, p0, p1, p2, p3) and step(nowMs); the host keeps the newest post; on
  "visibility hidden" it zeroes the held buttons and the move in that post (the press counts stay) and pauses (local
  play). NA-N2: now = held input cleared on hide (here) and on blur/hidden in the page (engine-architect's page part);
  waits for online play (slice 3) = the pause decided by the mode: an online match never pauses, it keeps ticking with
  empty held input. ~25 code, ~35 tests.
- pacing.ts: PacedSim.step(nowMs) gets the time the pacer read; every due time the sim returns (constructor's rebase,
  each step, each rebase) goes through one check that throws "the simulation returned a due time that is not a finite
  number" (NA-N7); the host's catch turns it into the page's error message and the match stops. ~10 code, ~15 tests.
- worker.ts: passes the new calls through. ~3.
- NA-N3 (who picks the blend fraction): cs3 changes nothing on the page: there is nothing to blend (no entity moves) and
  the page draws the newest block. The Worker picks it (only the endpoint knows the interpolation clock: ADR 0001 part 7),
  as a block field in cs4 with the page's rule; events are read from every block (cs8). One sentence in picture.ts now
  if it is not there yet (picture.ts:24 names the two blocks but not who picks the fraction).

### devtools: replay runner, comparer, L1 (deps: sim, protocol, room, client-endpoint, sim-wasm; path deps only)
- replay.rs: `run_file(path: &Path) -> Result<(Recording, Vec<u64>), ReplayError>` (reads, decodes, calls
  sim_wasm::replay::replay: the same code the Wasm runs). compare.rs: `struct Difference { tick: u32, expected:
  Option<u64>, got: Option<u64> }`, `first_difference(start_tick: u32, expected: &[u64], got: &[u64]) ->
  Option<Difference>` (the first differing tick, or the first tick one stream lacks; tick = start_tick + index + 1 with
  checked_add). Field-level diffs (ADR 0004 :352) wait for state with fields (cs4). ~45 code, ~30 tests.
- link.rs, L1 (ADR 0002 :520-526; ADR 0001 part 13): on simulated time in whole microseconds (u64; tick T's time =
  T * 1_000_000 / 30, integer, exact every 3 ticks), integer only. `struct Profile { name, delay_us: u32 (one way,
  mean), jitter_us: u32 (one way, uniform in [-j, +j]), loss_ppm: u32, reorder_ppm: u32 (share of adjacent datagram
  pairs swapped), duplicate_ppm: u32, rate: Option<RateLimit { bits_per_s: u32, queue_bytes: u32 }>, burst:
  Option<Burst { start_us: u64, len_us: u64 }> }`; consts P0 {0, 0, 0, 10_000}, P1 {25_000, 5_000, 10_000, 10_000}, P2
  {50_000, 10_000, 20_000, 10_000}, P3 {75_000, 15_000, 50_000, 10_000}, P4 {100_000, 20_000, 50_000, 10_000}, B1 = P2 +
  burst {start 10 s, 1 s}, every named profile duplicate 0 and no rate limit (ADR 0001 names no rate for either);
  CLEAN (everything 0: the transparency test); PROFILES = [P0, P1, P2, P3, P4, B1]. `struct LinkSeed(pub u64)`; `enum
  Fate { Delivered, Lost, LostInBurst, RateDropped }`; `struct FateRecord { side_from: Side, index: u32, sent_us: u64,
  bytes: u16, delivery: Delivery, base_delay_us: u32, fate: Fate, at_us: u64, reordered: bool, duplicate_at_us:
  Option<u64> }`; `struct FateLog` (a running FNV-1a 64 over each record's canonical little-endian bytes, the counts and
  sums the statistics need, and optionally the first N records kept for a failing seed); `struct L1Link`, `L1Link::new(
  profile, seed, keep_records: usize)`, `send(&mut self, from: Side, delivery: Delivery, bytes: &[u8])`, `set_now(&mut
  self, now_us: u64)`, `receive(&mut self, at: Side, out: &mut Vec<u8>) -> Option<Delivery>` (next packet due at or
  before now), `log(&self) -> &FateLog`; `L1End<'_>` implements Transport and L1Link implements TransportPair, so
  LocalMatch<L1Link> runs the Worker's own step over L1.
  Fates: each datagram takes exactly five draws from its direction's generator, in a fixed order (loss, delay, reorder,
  duplicate, the duplicate's delay), used or not, so one packet's fate never shifts another's and two profiles on one seed
  see the same draws against other thresholds. Delay = delay_us + uniform[-j, +j] by multiply-shift (no modulo bias);
  loss = draw * 1_000_000 >> 32 < loss_ppm; reorder: the packet is held until the next datagram the same way and
  delivered after it (at the later of the two times); duplicate: a second copy at its own delay; burst: every packet
  sent inside it is lost; rate: a bottleneck queue (a packet starts when the previous has gone, takes bytes*8/rate; over
  queue_bytes waiting it is dropped). Reliable messages are never lost or reordered: a "lost" one is redelivered one round
  trip later, and none overtakes an earlier one on its channel (a stream). The statistics read loss from the fates, and
  delay and jitter from base_delay_us of delivered packets (the reorder hold and duplicates are recorded beside it),
  jitter as the standard deviation, against uniform's j / sqrt(3): this is "the distribution L1's own documentation
  states" (PM-0002-49), written in link.rs's doc.
  Kept apart from the sim's streams: its generator is SplitMix64 (Steele, Lea, Flood, OOPSLA 2014; 10 lines, private to
  devtools), seeded by LinkSeed (the per-direction seeds are SplitMix64's first two outputs), never by the match seed;
  the sim cannot reach devtools (dependency direction), L1 never calls the sim's generator, and the World's hash covers
  the World only. A run is replayed from (match setup, posts, profile, LinkSeed). ~230 code, ~90 unit tests.
- Never (L1 is a test tool): reads no clock, no hash map, no float in a fate (floats only in the statistics test).
- B2 DONE. Next: Block 3 (the tests).

## BLOCK 3. The tests that prove it (netcode evidence: deterministic, injected latency, jitter and loss)
Where they run: every Rust test below runs in `cargo test --workspace --locked` (CI's plain test job, debug; that job
already runs the workspace twice, toolchain.yml:84-85). Proposed CI text (2.3 lane, exact): toolchain.yml:88 becomes
`cargo test --release -p sim -p protocol -p room -p client-endpoint -p sim-wasm -p devtools --locked`, so the loop, L1
and the recording also run in a release build in a separate process (overflow checks on there too).
- T1 (ADR 0001's milestone, headless): crates/devtools/tests/loop_over_l1.rs, `four_messages_cross_every_profile`:
  LocalMatch<L1Link> (the Worker's own step) at P0..P4 and B1, 1,800 ticks each (G11's minute), scripted posts. Pass:
  Joined at a tick <= the profile's round trip in ticks + 2; the fate log holds Join and JoinAccepted delivered on the
  control channel, and Inputs and Snapshots as datagrams; 0 decode errors on both sides; 0 refusals; snapshots decoded
  within 2 % (absolute share) of 1,800 x (1 - loss); commands late counted (> 0 above P0), never refused.
- T2 (PM-0002-49, identity): `same_seed_same_fates_and_hashes`: two runs per profile, same LinkSeed and posts: equal
  fate-log digests and equal 1,800 per-tick authority hashes; a different LinkSeed at P2 gives another fate digest.
- T3 (L1 adds nothing by itself): `clean_link_equals_the_loopback`: LocalMatch<L1Link> with CLEAN replays the first
  recording's posts and gives exactly its hashes, which LocalMatch<Loopback> gives too.
- T4 ("10,000 presses, none lost or doubled", as far as one slot and no soldier allow; PM-0001-38's P3):
  `ten_thousand_presses_survive_loss`: at P3, B1 and DUP (P2 + 1 % duplication); 10,000 scripted presses on 4 buttons,
  posted at a 60 Hz page cadence (2 posts per tick; some ticks carry 2 presses of one button, so the pacer's backlog
  works); run until the backlog is 0 and the newest command is processed. Pass: per button, the authority's presses
  (the harness sums press_edges of the world's player record each tick, exact as u32) equal the scripted count; the
  endpoint's presses_lost = 0 and presses_doubled = 0; room refusals = 0; pacer flooded = 0. The u8 counters wrap ~39
  times per button on the way.
- T5 (ADR 0001 :1823's counter wraps): `wraps_cross_cleanly`: start tick 131,069 (2 x 65,536 - 3) and first sequence
  65,533, at P2, 600 ticks: presses exact, 0 refusals, 0 decode errors, two runs give equal hashes. (In cs3 the only
  16-bit wire counter is the command sequence; the snapshot's 16-bit baseline offset is coded and round-trip tested but
  unused until deltas exist.)
- T6 (PM-0002-49, statistics): crates/devtools/tests/l1_profiles.rs, `profiles_hold_within_five_percent`: per profile
  (P0..P4, B1) and per direction, 1,000,000 datagrams of 64 B sent at 30 per second of simulated time through L1Link,
  all received. From the fate log: loss share, mean base delay, and jitter (standard deviation of base delay) each within
  5 % relative of loss_ppm, delay_us and jitter_us / sqrt(3); at P0 each is exactly 0 (a relative band around 0 has no
  width: F-P0); reorder share within 5 % of 1 % (above 0, as PM-0001-20 needs); B1: P2's bands, and every packet sent
  inside the burst lost (30 of 30 per direction), none outside it lost to the burst (F-B1). 12,000,000 fates in all;
  estimate 6-12 s in debug, about 1 s in release (to be measured in the build and reported); in the default run. If debug
  passes 30 s, it moves to the release line only (one CI word). Unit tests in link.rs: duplication share within 5 % of
  1 % over 1,000,000; the rate limit's delivery times equal the formula and it drops over queue_bytes; reorder always
  inverts the pair; a reliable message is never lost or overtaken; five draws per datagram whatever its fate.
- T7 (ADR 0004's milestone, the first recording): crates/devtools/tests/first_recording.rs. Written once with
  SHAMBLE_WRITE_RECORDING=1 (like SHAMBLE_WRITE_TYPES): 1,800 ticks of scripted posts (8 move directions, 4 held-button
  patterns, presses at varied gaps incl. 2 in one tick, repeated post numbers for a slow page, skipped ones for a fast
  page), public test seed 0x00..0x1F (never a real match seed), start tick 0, through LocalMatch<Loopback>. File:
  crates/devtools/recordings/first-loop.rec, binary, 8 + 2 + 32 + 4 + 4 + 1,800 x 24 + 1,800 x 8 = 57,650 B.
  Natively: `first_recording_replays_to_its_hashes` decodes the committed file, replays it (devtools::replay::run_file,
  the code the Wasm runs) and needs first_difference == None, in each of the two workspace runs and the release run
  (three processes, debug and release). Wasm in Node: packages/sim-worker/boundary/sim-wasm.test.mjs gains a case that
  passes the file to the built Wasm's `replay_hashes` and compares with the file's hash section, naming the first
  differing tick. Wasm in Chromium here: the same export from the Worker in a Playwright spec (test-engineer's part).
  Precondition: PM-0004-21's report names the hash function before the file is committed (cs2 report, risk 5); if FNV-1a
  is replaced, one rerun with SHAMBLE_WRITE_RECORDING=1 rewrites the hashes (posts unchanged).
- T8 ("every tick runs on the newest post", ADR 0003's second half): sim-wasm native
  `every_tick_runs_the_newest_post`: 1,800 steps, a new post number before each step and sometimes three set_post calls
  before one step: the block's post_ran equals the last post set before that step, every tick (stale_ticks = 0);
  `a_press_reaches_the_authority_in_the_same_step`: a press set before step k is in the authority's player record after
  step k and in block k's own section (the command for T reaches the server before it runs T). host.test.ts: each step
  is preceded by exactly one input call carrying the newest post the host holds, also in a 7-tick catch-up wake; after
  "visibility hidden" the held buttons and move are 0 and the press counts unchanged. In Chromium here (PM-0002-64):
  the readout's stale ticks 0, posts <= drawn frames, blocks = ticks within 1 (test-engineer's spec reads these).
- T9 (decoders never panic; no new dependency): in codec.rs and replay.rs tests, a test-only xorshift64* (8 lines,
  fixed seeds): per decoder (control, input, snapshot, recording) 100,000 random byte strings (length 0-600, half with a
  valid first byte) and 100,000 mutations of valid encodings (bit flips, cut at a random point, a byte inserted, a count
  set to 255): never a panic (overflow checks are on, so an overflow fails the test), every Ok re-encodes and decodes to
  the same value; 10,000 random valid messages with extremes (0 and max of each width, counts 0 and the cap, a token at
  MAX_TOKEN_BYTES, a mode id with multi-byte UTF-8) round-trip; exhaustively: every proper prefix -> Truncated, one
  byte more -> Trailing, each unknown kind -> Kind, each unknown flag bit -> Flags, counts and lengths over their caps
  -> TooMany / TooLong, a length beyond the bytes left -> Truncated with the target Vec's capacity unchanged (nothing of
  the claimed size allocated). About 1 s in debug. Coverage-guided fuzzing (cargo-fuzz) needs nightly and a new tool: not
  proposed (his decision if ever).
- T10 (the rest, unit): room (join accepted; rejected for version, content hash, slot taken; a command for T runs at T;
  duplicate dropped; reordered queued and run in sequence; a gap skipped with its presses carried; catch-up cap; late
  counted; each refusal; an ack newer than sent refused; sequence 65,535 -> 0; the snapshot's own section is the world's
  player record); endpoint (no command before the first snapshot after join; the ring retires up to newest received;
  lost and doubled detected on a planted mismatch; post_ran; stale snapshots ignored); press pacer (one edge per tick,
  backlog drains, wraps at 256, flooded counted); clamp_move (all 65,536 (x, y) pairs land inside the circle,
  exhaustively, and the 8 keyboard directions are unchanged); loopback (FIFO, full, too large, 10,000 packets through the
  ring with the arena's pointer unchanged); wrap (seq_newer on the edges 0x7FFF / 0x8000 / 0x8001 and a seeded sample,
  ticks_after on out-of-order input); sim (known answers for the 56-byte state from a second implementation; the shared
  prefix holds no seed byte; start tick; empty commands hold); pacing.test.ts (NaN and Infinity from step, rebase and the
  constructor throw; the host posts the error and stops); messages.test.ts (each field's range).
- Allocation: pointer and capacity of every reused buffer unchanged over 1,000 steps after warm-up (as world.rs's
  scratch test does). A counting global allocator needs `unsafe impl GlobalAlloc` (unsafe_code is deny): not proposed
  unless rust-reviewer allows a test-only exception.
- Not mine (named so nothing falls between): PM-0002-53's planted overflow, the float and stream vectors, the scan test,
  the two-instance test, the panic hook, the page's key diagram and its e2e (engine-architect, test-engineer,
  rust-reviewer).
- B3 DONE. Next: Block 4 (size and cuts).

## BLOCK 4. Size and cuts (estimates in added lines, doc comments included, as cs2's report counted; +-30 %)
| Crate / file | Rust code | Rust tests | TS code | TS tests |
|---|---|---|---|---|
| protocol: wire.rs 55/30, wrap.rs 25/30, messages.rs +45, codec.rs 205/210 (incl. T9's property tests), transport.rs 55, loopback.rs 75/45, lib.rs +6, ts_export +3 | 469 | 315 | | |
| room: lib.rs 125/150, queue.rs 60/40, Cargo.toml +3 | 188 | 190 | | |
| client-endpoint: post.rs 20, press.rs 50/50, endpoint.rs 200/150, lib.rs +4, Cargo.toml +3 (block.rs's section writer, ~30, counted by engine-architect with the layout) | 277 | 200 | | |
| sim: world.rs +45/+55, lib.rs +2 | 47 | 55 | | |
| sim-wasm: local_match.rs 110/90, replay.rs 80/60, lib.rs +50/+40, Cargo.toml +2 | 242 | 190 | | |
| devtools: link.rs 230/90, replay.rs + compare.rs 45/30, tests/loop_over_l1.rs 170, tests/l1_profiles.rs 90, tests/first_recording.rs 60, lib.rs +6, Cargo.toml +8 | 289 | 440 | | |
| sim-worker: messages.ts 25/20, host.ts 25/35, pacing.ts 10/15, worker.ts 3, boundary/sim-wasm.test.mjs +25 | | | 63 | 95 |
| Totals | 1,512 | 1,390 | 63 | 95 |
My part: about 3,060 lines (Rust 2,902, TypeScript 158), tests about 48 %. Not counted: the regenerated TS in
packages/types (about 30 generated lines), the recording file (57,650 B binary), the CI line (one edit).
Against the card: 2,700 planned for all of change-set 3; my part alone is about 1.8 x the 1,700 I was given, and with
engine-architect's part (vectors, scan test, two-instance test, key diagram, EA follow-ups: their notes, not read here)
the change-set is likely about 4,000-4,300, about 1.5-1.6 x the card, under the card's stop line at twice its size
(5,400). cs2 ran 1.54 x its plan for the same reason (4,477 against 2,900: its report :84-87): the evidence is the bulk.
Cuts, in the order they move (first to last):
1. (the card's own first cut, not mine) the two-instance test and the build-options table.
2. L1-FULL: L1's rate limit and duplication (and T4's DUP profile, the two unit tests) move to a follow-up before G11
   (2.3 after 3): about -90 (55 code, 35 tests). A card departure (:322 lists both); PM-0002-49 uses neither.
3. CHROMIUM-REPLAY: the recording's Chromium run moves to 2.3 after 3 (the Node run stays): test-engineer's lines.
4. If the plan must stay near the card: L1 as a whole (link.rs, T1, T2, T4-T6: about -600) becomes change-set 3b right
   after 3 and before 4 (4's own milestone needs L1's profiles); ADR 0001's milestone and PM-0002-49 are then met at 3b.
5. Last resort: the first recording with its Wasm export and T7 (about -300) joins 3b; ADR 0004's milestone moves too.
Must not move (the loop and its safety): the codec with never-panicking decoders and T9; wrap-aware compares; the
transport interface and the loopback; the room's checks and queue; the press counters, the pacer, the echo and the
lost/doubled counters (the card row's "zero lost presses"); the join exchange; the step order with T8; the World's
shared prefix and start tick (NA-N4, NA-N6); NA-N1, N2, N5, N7 (a few lines each). Shrinking tests is not a cut
(CLAUDE.md: never shrink evidence).
- B4 DONE. Next: Block 5 (where the plan or drafts are wrong; questions).
