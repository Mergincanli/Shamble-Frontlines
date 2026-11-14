# U4e round 1: security-auditor review (the room and the codec path)

Seat: security-auditor (workflow agent, opus). Read only: no edit to any copy; notes here only (no Edit tool in this
seat: the file is rewritten whole with Write, its content kept). Read copy: `.gate/u4e-r1` (97bb9d3 + the snapshot
patch). Base copy: `.gate/u4e-prefix` (1e8c188). Log prefix: `sec`. Cites below are into
`.gate/u4e-r1/spikes/netcode-proof/crates/` unless a path says otherwise.

## Plan
1. Threat model (block 1): who sends bytes to what, what each side trusts, the assets (other clients' state and
   tokens, the room's liveness, the counters U4r/U6 logs will read).
2. Read the earlier security notes for continuity (u4d-round2-security.md, u4j-round2-security.md, carries 51/53/54/55).
3. Read the code in `.gate/u4e-r1/spikes/netcode-proof/crates/{server,room,client}/src` and the proto decode path it
   calls (quant.rs, limits.rs, msg.rs, snapshot.rs's decode) as needed.
4. Check, block by block, writing a line after each finding:
   - B2 the pump (Decoder::pump): socket checks via Quant::decode counted by kind; stream error -> malformed close;
     wrong channel / wrong direction; datagrams and control before Join; Join deadline; Input bucket; Ping limit;
     unknown Pong; the hold time's cap; ConnLimits; Counters.
   - B3 roster and token compare: constant time over every entry, no early exit; bind at first accepted Join; Join on a
     live slot; roster full, wrong version, wrong content hash.
   - B4 the client's decode (ClientEndpoint): 1,150 B refusal; wrong channel; hostile server bytes; panics.
   - B5 bounds a client can push in the room: EventQueue cap, InputBuffer, AckRing (acks naming any tick), Resync one
     in flight, InterestGrid, Priority, rewind_at; panics (index, slice, overflow, unwrap/expect, debug_assert-only).
   - B6 leaks across clients: own-state section (W1-S1, carry 51/53), routing of own shots (server-echo), tokens.
   - B7 counters: can a hostile client make one lie or hide an attack.
5. Anti-cheat coverage table (player action -> where validated -> gap).
6. Verdict and findings in the schema.

## Blocks (written as each finishes)

### Block 1. Threat model (done)
- **Who sends what.** A client (browser Worker, npt-bots, or a forged script) sends bytes on three channels to one
  session's `Decoder::pump` (server/src/pump.rs); the pump decodes with npt-proto and hands decoded `Message`s to the
  caller, which passes `Join` to `Room::join` and the rest to `Room::receive(slot, msg)` (room/src/room.rs:607, :682).
  The server sends bytes to `ClientEndpoint::input` (client/src/lib.rs:353).
- **Trust.** Every client is hostile (rostered or not). An unrostered connection holds no token; a rostered one holds
  its slot's token and, after its first accepted Join, its bind value. The server is trusted by the client, but the
  catalogue still asks npt-client for "a decoder that never panics" (catalogue `Snapshot` row), so hostile server
  bytes are a robustness question (Low at most), not an authority one.
- **Assets.** (a) other clients' tokens and bind values; (b) other clients' owner-class state (own-state section,
  hit confirmations) and out-of-interest positions; (c) the room's liveness (no panic, bounded memory and CPU per
  client); (d) authority: positions, hits, ammunition, cooldowns derive from validated inputs; (e) the counters U4f's
  -21 series and U4r's logs read (they must not lie in a way that hides an attack or voids a run from outside).
- **Out of scope at U4e by the plan** (checked, not findings): AddrTable and the handshake cap (U6, §5 U4e); the lead
  bound, the move-vector length, the aim-bounds check and every other room-side command check with its -21 counter
  (U4f, u4-build-plan.md :796-797; the sim normalises a move vector over 127 anyway, sim/src/world.rs:493-498); the
  view-time tolerance and the rewind cap (U4h, §4.9; at U4e any view tick the 2 s ring holds is accepted,
  room/src/rewind.rs:18-33, room.rs:33).

### Block 2. The pump (server/src/pump.rs) (done)
Checked and holding:
- Every inbound message goes through `Quant::decode(Direction::ClientToRoom, channel, bytes)` (:332-335); each refusal
  counted by its `DecodeError` kind (:157-170); a stream refusal other than WrongChannel/WrongDirection closes with
  `Kick(malformed)` and `stream_malformed` (:341-347); a datagram refusal is dropped and counted (:348).
- Before Join: a decodable datagram is `before_join` (:365); a decodable non-Join control message first on the stream
  closes as malformed with `first_not_join` (:360-363), `SessionControl` included, never INVALID (no verdict here).
- Join deadline on the injected now, only in AwaitJoin (:307-311), 2 s from the accept (:13, :86).
- Input bucket 8, refill 2 x tick_hz a second (:84), integer token units (:30-65), no overflow (saturating); Ping
  bucket 10 at 10/s (:85); bucket state does not refill on a clock that goes back (:52-53).
- Pong: matched on (id, echoed time) among at most 8 outstanding (:414-421), else `pong_unknown`; hold capped at one
  tick period (:423, :96-98); samples kept 8 (:425-428); RTT inflation through late pongs is bounded by the 8 x 250 ms
  outstanding window (about 2 s) and is ADR 0001's accepted risk (OQ-0001-5), its cap U4h's.
- Ping answered with the server's own hold (arrival to now), not a client value (:376); no amplification (Pong 16 B
  for a Ping of at most 32 B).
- No `unwrap`, `expect`, index or unchecked arithmetic on client data in the file; `out.len() as u32` sizes are the
  server's own messages.
- Version and content hash are public values; a plain compare is right (:398-404).
- A client message on its own full-state stream is capped at 4 + 510 B by `channel_max` (proto msg.rs:270-283).

Findings:
- SEC-U4E-1 (Low): a forged kind first on the stream does not close the session before Join. See findings.
- SEC-U4E-2 (Low): control-stream messages have no per-connection rate limit. See findings.
- SEC-U4E-3 (Low): in the Joining state a stream message other than Join is dropped uncounted (:391's `_ => {}`).

### Block 3. Roster and token compare (room/src/roster.rs) (done)
- `same` (:64-71): XOR-OR fold over all 16 bytes, `black_box` on the fold, the answer by arithmetic
  (`(d - 1) >> 8 & 1`, precedence checked: `as` binds before `&`); no branch on the bytes. Equal gives 1, any one bit
  off gives 0 (unit test :191-201, all 128 single-bit flips).
- `admit` (:141-171): every entry compared (`same` called for each, its result masked by `hit == 0`, never skipped:
  `black_box` keeps the call), first match wins, no early exit; the only branches are on the outcome the reply reveals
  anyway (bad token, bind mismatch, roster full). Shown by review, as the catalogue asks; I agree it holds.
- Bind: refused when the slot is bound and the value differs (:156-160, constant-time compare); set only after the
  roster-full check passes (:165), so a refused Join binds nothing. Observation (no finding): `Room::join`'s
  `i >= clients.len()` path (room.rs:610-612) runs after `admit` set the bind, so a roster longer than the scene's slots
  binds a slot the room then refuses; unreachable when the caller passes at most `scene.slots()` tokens.
- Replacement: needs the token and the slot's bind value (only the first binder holds it after the first accepted Join);
  a replacement adds no live session (:161-163). Built and unit-tested (roster.rs:227-229; room/tests/room.rs:201-212).
- Wrong version, wrong content hash: the pump's, before the room (pump.rs:397-409); bad token, bind mismatch, roster
  full: the room's. All five reach the client end to end (server/tests/pump.rs:61-124).
- No token in any `Debug`: `Roster`'s is custom (:81-93), `Entry` has none, `Join`'s shows lengths (proto msg.rs:67-82).
- SEC-U4E-5 (Low, see findings): nothing sends `Kick(replaced)` or closes a replaced, left or kicked session's
  `Decoder`, and `Room::receive` takes any message by slot alone.

### Block 4. The client's decode (client/src/lib.rs) (done)
- Every received message goes through `Quant::decode(RoomToClient, channel, bytes)` (:432): the 1,150 B receive cap
  (proto msg.rs:242-250), the channel (WrongChannel counted, :446), the direction.
- A refused snapshot datagram gives the sentinel under its header's tick read by `get(1..5)` (:451-461), no slicing
  panic. Group index and count come from 3-bit fields with `group < groups` (proto snapshot.rs:578-581), so
  `1 << g` on a u8 and `sigs[g]` (:633, :697) and `1u16 << groups` (:163) are in range.
- Pings held at most 8 (:440). The `see` walk is bounded by the ring (at most 32 steps), a long gap one `lost` run.
- Tests: wrong channel counted and never filed, a datagram above 1,150 B refused with the sentinel
  (client/tests/endpoint.rs:100-115).
- SEC-U4E-7 (Low, see findings): `lost` merges runs with `last.1 += count` (:526), a u32 add a hostile server can
  overflow (debug panic; release wraps, the count lies; npt-client has no release overflow-checks: Cargo.toml:108-112).

### Block 5. Bounds a client can push, and panics (room/src) (done)
- Event queue: cap as data (1,024 provisional), refused at the cap (events.rs:52-55), the overflow forces one resync and
  a second inside it kicks (room.rs:973-994), a forced full state unacknowledged past `FULL_STATE_ACK_TICKS` kicks
  (:1015-1021), age 256 expired and counted (events.rs:87-92). Unbounded only under the bites-only flag (room.rs:493-503).
- Input buffer: at most `CMD_CATCHUP` = 8 (input.rs:114-116), commands per `Input` at most 8 by the decoder (proto
  input.rs:208-211), the session's slot written over the command's (room.rs:753, :828-831).
- AckRing: fixed 32 entries; an ack is widened near the newest sent tick and must name a held, old or never-sent tick
  (ack.rs:138-165); a never-sent one refuses the datagram's commands (room.rs:744-749). Group shifts are below 8
  (Packer caps groups at `Limits::GROUPS`, encoder.rs:144).
- Priority: a BTreeMap keyed by entity id, retained to the relevant set each snapshot (room.rs:1242-1246): bounded by
  the sim's entities, never by client data. InterestGrid and AreaFlood are sized from content data, not the network.
- rewind_at: a pure lookup in the 2 s ring (rewind.rs:8-33); its debug_assert follows an early return (redundant,
  harmless). Tolerance and cap: U4h.
- Panics: no `unwrap`/`expect` on network data (the one `expect` is `Room::new`'s hit history, construction only:
  room.rs:423-424); indices are slot-checked (`get`/`get_mut`) or server-side; arithmetic on client values is wrapping
  or saturating. The size `debug_assert_eq` (room.rs:1277-1281) checks server arithmetic, and release still refuses an
  oversized group (:1282-1284). Every `EncodeError` kind is counted and skips that client's tick (:1089-1096; carry 54).
- A client-advertised tiny datagram size (a WebTransport peer's limit) gives `budget_short`, never a panic
  (room.rs:1128-1132, encoder.rs:108-118).
- SEC-U4E-4 (Medium, see findings): `Resync`'s one-in-flight rule is cleared by the client's own acknowledgement, so a
  client can have a full state sent every tick or two, outside the 256 kbps window.
- SEC-U4E-9 (Low, carry for U4f): `canonical`'s wrap-aware comparator (room.rs:344-350) is not a total order for one
  slot's ticks 2^31 or more apart; Rust's `sort_by` may panic on a non-total order. Unreachable at U4e (one command per
  slot per tick); option A must keep it so.

### Block 6. Leaks across clients (done)
- Own-state section: built from the receiving client's own slot only: Soldier and OwnState at index = slot, Effect rows
  `slot * effect_rows .. (slot + 1) * effect_rows` read from `Scene::effect_rows()` (room.rs:430, :1022-1040; carries 51
  and 53 met; their test checks every own-section record's identity and generation for 48 clients over at least 300
  ticks through a generation change: room/tests/room.rs:425-535, :647-673). The owner-to-all bite is the only other
  route and is bites-only. `full_state` uses the same `own` (:1062-1073).
- Entity section: only records with a point (Soldier, Vehicle, Projectile) within 64 m and an area the flood connects,
  parts of relevant hulls, doors and the wall by area (room.rs:862-931); the own soldier skipped (:885-887); OwnState
  and Effect have no point, so never enter it, and the codec refuses them there (W1-S1). Full states carry the same
  relevant set, never the whole world. No Team or Public row added (carry 55).
- Own shots: a cosmetic `Shot` goes to clients that see the shooter, never the shooter's own (room.rs:949-953; the
  echo only under the bites-only flag); the hit confirmation reaches the shooter (`shooter == me`, :961).
- SEC-U4E-6 (Low, see findings): a `Hit` or `Kill` goes to a client that sees only the victim with the shooter's slot
  (and a `Hit` with the shooter's command sequence), where ADR 0001 part 9 (f) and the catalogue's gameplay-events row
  say "no shooter id when the shooter is not relevant to the viewer".
- Tokens: no token or bind value reaches any client message, log or `Debug`.

### Block 7. Counters (done)
- Every counter is per session or per client: a hostile client can inflate only its own (by design for -21).
- Holes or mixes: SEC-U4E-3 (Joining-state stream messages uncounted); SEC-U4E-1 (a forged kind before Join is counted
  under its decode kind and the session stays open, so `first_not_join` does not see it); SEC-U4E-8's
  `rewinds_refused` also counts option B's reruns of an old command, which carry 40 makes VOID a -30 pre-check leg.
- Observation (no finding): -58's per-client wrap names (`ack` at room.rs:724-728, `cmd_seq` at input.rs:85-87) count a
  change of the high half, so a client acking alternately either side of a 2^16 boundary, or jumping its sequence by
  about 32,767 a command, inflates its own `counter_wrap`; -58 judges honest clients, and the count is that client's.
- SEC-U4E-8 (Low, carry for U4h): option B reruns the last command with its original view time (room.rs:803-826), so a
  client that stops sending while holding fire gets each rerun shot rewound to an ever older view time, up to the
  ring; U4h's tolerance must judge a rerun's view time at the tick it runs.

### Block 8. The findings, in full (done)
Severity on CLAUDE.md's one scale. None is Critical or High. Each says whether it must be fixed before the merge.
U4e adds no dependency: no Cargo.toml, Cargo.lock or deny.toml change (`git diff --stat` in `.gate/u4e-r1`: empty).

**SEC-U4E-4, Medium, not before merge (the advisor routes it: U4e's fix pass or U6).** room.rs:685-699 (Resync),
:734-742 (the ack clears `in_flight`), :1055-1087 (`full_state`), ack.rs:138-153.
- Scenario: a joined client sends `Resync`; the room sends a full state at tick f. The client acks f in its next `Input`
  without reading the full-state stream (the ack check asks only that the room sent f), which clears `in_flight`, and
  sends `Resync` again. Result: one full state every tick or two (15 to 30 a second), each the client's whole relevant
  set (the catalogue's estimate for the stress world is about 6.5 KB), outside the 256 kbps window (the window budgets
  only snapshot datagrams and the pump's bytes, room.rs:1008-1013), with a full encode per tick and no transport
  backpressure at U4e. An 8 B request becomes kilobytes, at up to about 6 x the per-client budget every other bound
  assumes.
- The code meets the catalogue's words ("a second `Resync` before the client acknowledges the answering `FullState` is
  dropped"); the gap is that the acknowledgement is the client's own claim. The catalogue's FullState row has the
  transport-side guard ("at most one open per session", proposed), which nothing builds before U6.
- Cheapest proof (a room unit test; the test-engineer can settle it by run): 30 iterations of { `receive(Resync)`;
  `tick()`; `receive(Input { ack_tick: last_tick as u16, ack_groups: 1, .. })` } on `Scene::stress()`; today
  `counters().full_states` grows by about one per iteration.
- Fix: a room-side cap counted under its own name (for example, at most one answered `Resync` per
  `FULL_STATE_ACK_TICKS` or per second, the rest dropped and counted), or full-state bytes counted against the window;
  and a U6 carry for the transport's one-open-full-state-stream rule. Which one is the advisor's (the plan is silent).

**SEC-U4E-5, Low, not before merge (carry to U4r's runner and U6's TickLoop).** room.rs:607-664 (`join`), :659-664
(`leave`), :682-703 (`receive` keyed by slot); server/tests/common/mod.rs:238-249 (the lab ignores `replaced` and never
clears `conn.slot`).
- Scenario: after a replacement (token and bind), a `Leave` or a `Kick(overflow)`, the old session's `Decoder` stays
  Joined unless the caller closes it, and `Room::receive(slot, msg)` takes any message for that slot. The old
  connection's Inputs then drive the new session's soldier, and its `Leave` ends the new session. Nothing in U4e sends
  `Kick(replaced)`. Reach: whoever holds the slot's token and bind value (the same player), so small.
- Cheapest proof: a lab test with two connections joining slot 0 with the same `Join`; the second replaces; a `Leave`
  on the first leaves `room.client(0)` empty.
- Fix: the caller sends `Kick(replaced)` and closes the old `Decoder` before routing anything else, and `Room::join`
  returns a session generation that `receive` and `leave` check, a stale one dropped and counted; a test.

**SEC-U4E-1, Low, not before merge (the advisor rules which text wins).** pump.rs:337-349 with :352-364.
- Scenario: a connection's first control-stream message is a known type on the wrong channel (an `Input` framed on the
  control stream) or the wrong direction (a room-to-client type); the pump counts it and continues, and a valid `Join`
  after it is accepted. Catalogue :93 says "any other type first on the stream closes the session as malformed";
  §4.10's "no forged kind closes the session" is written for joined sessions (the twins of the forged suite).
- Impact: none on authority (nothing is processed and the 2 s deadline still closes the session); `first_not_join`
  does not see it.
- Cheapest proof: a pump test: wrong-channel `Input` on Control, then a valid `Join`: today `out` holds the Join,
  `first_not_join` is 0 and the session is open.
- Fix: in AwaitJoin, close any refused or non-Join stream message as first-not-join, keeping §4.10's rule for joined
  sessions; or the advisor rules that §4.10 covers the pre-Join case and the catalogue line says so.

**SEC-U4E-2, Low, not before merge (carry to U6 and the catalogue).** pump.rs:390 (`(State::Joined, m) => out.push`),
room.rs:682-703.
- Scenario: a joined client writes `ClientSettings`, `Resync` or `SessionControl` messages on its control stream as fast
  as the transport takes them; each is decoded and pushed onto `out` (bounded per pump call only by what the link has
  delivered), and the room ignores most of them at U4e. The checklist's per-connection message rate has no home for
  the control stream; the catalogue gives buckets for `Input` and `Ping` only.
- Cheapest proof: 10,000 `ClientSettings` in one pump call: 10,000 messages on `out`, no counter moves.
- Fix (U6): a control-stream bucket, counted (over it: dropped, or closed as malformed), or a per-pump cap, beside the
  transport's flow-control window; the catalogue's rows gain the limit.

**SEC-U4E-3, Low, not before merge (one line; fold into the fix pass if one runs).** pump.rs:391 (`_ => {}`).
- Scenario: a caller that pumps a session between passing its `Join` and answering it (U6's I/O runtime while the tick
  thread answers) drops every stream message the session sends in that window, with no counter.
- Cheapest proof: pump a `Join`, then (no `accept`) a `ClientSettings` in a second pump call: every counter stays 0.
- Fix: count it under a named counter (or close as first-not-join).

**SEC-U4E-6, Low, not before merge (a known deviation until slice 3).** room.rs:954-969 (routing); the wire form
always carries the shooter (encoder.rs:65-67's bits; proto's event layout).
- Scenario: a client whose interest holds the victim but not the shooter (the shooter behind a closed door, or beyond
  64 m) receives the `Hit` or `Kill` with the shooter's slot, and a `Hit` with the shooter's command sequence. ADR 0001
  part 9 (f) (docs/adr/0001-multiplayer-netcode.md:409-411) and the catalogue's gameplay-events row: "no shooter id when
  the shooter is not relevant to the viewer". The wire has no form without it; ADR 0001:1687-1688 proves part 9 (f) at
  slice 3.
- Cheapest proof: a room test with the shooter outside the viewer's interest and the victim inside it.
- Fix: record it as a known deviation (U4r's class audit and PR #40's report name it), or strip the id at the room
  with a codec form, if the advisor wants it in the spike.

**SEC-U4E-7, Low, not before merge (one line).** client/src/lib.rs:522-530 (`last.1 += count` at :526).
- Scenario: four snapshot groups (each group 0 of 2, baseline age 0) at ticks t, t + M, t + 2M, t + 3M with M = 2^31 - 1,
  in one `input` call. Each `see` adds about 2^31 to one merged lost run (the incomplete ticks stay Pending, so the
  runs merge), and the third jump overflows u32: a panic in the debug and test profile; in release (no
  overflow-checks for npt-client) the count wraps and the lost run lies. Hostile server only (trusted, behind TLS),
  but the catalogue asks for a client decoder that never panics.
- Cheapest proof: that endpoint test in a debug build (expected: panics with "attempt to add with overflow").
- Fix: `checked_add`, starting a new run when the sum would overflow (or `saturating_add`); a test.

**SEC-U4E-8, Low, not before merge (carry to U4h).** room.rs:803-826 with input.rs:135-141.
- Scenario: a client holding fire stops sending; option B reruns its last command each tick with the original view
  time, so each rerun's shot rewinds to that view tick, older every tick, up to the 2 s ring. At U4e any held view
  time is accepted anyway, so nothing is gained now; at U4h, unless the tolerance judges a rerun's view time at the
  tick it runs, starving the uplink buys rewind. Also `rewinds_refused` counts reruns whose view tick has left the
  ring, and carry 40 makes any such count VOID a -30 pre-check leg.
- Fix (U4h): judge each run's view time against the estimate at the run tick (or run a rerun with fire released, or
  advance its view time a tick per rerun), and count reruns apart from claims.

**SEC-U4E-9, Low, not before merge (carry to U4f).** room.rs:344-350.
- Scenario: option A (U4f) may put several commands of one slot in one tick; with ticks 2^31 or more apart (no lead
  bound), the wrap-aware comparator is not a total order, and Rust's `slice::sort_by` may panic on one ("May panic if
  the implementation of Ord is not total", since Rust 1.81). Unreachable at U4e: one command per slot per tick, slots
  unique.
- Fix (U4f): the lead bound before option A, or a total sort key; a test with one slot's ticks 2^31 apart.

### Block 9. Anti-cheat coverage table (done)
| Player action | Where the server validates it (U4e) | Gap |
|---|---|---|
| Connect and send nothing (slow loris) | 2 s Join deadline on the injected now (pump.rs:307-311) | AddrTable, handshake cap, idle timeout after Join: U6 |
| Join with a guessed or stolen token | constant-time compare over the whole roster, bind at first accepted Join (roster.rs:141-171) | `Kick(replaced)` unsent, stale sessions routed by slot (SEC-U4E-5) |
| Join with a wrong version or content hash | pump, before the room (pump.rs:397-409) | none |
| Datagrams or controls before Join | `before_join`; a non-Join control first closes (pump.rs:360-365) | a forged kind first does not close (SEC-U4E-1) |
| Malformed, oversized, trailing or unknown-field bytes | `Quant::decode` at the pump, counted by kind; a bad stream message closes (pump.rs:332-349) | none |
| Flood Inputs | Input bucket 8, 2 x tick rate a second (pump.rs:367-373) | none |
| Flood Pings or Pongs | Ping bucket 10/s; a Pong is matched among 8 outstanding (pump.rs:374-389, :411-429) | Pongs unlimited but O(8) each and counted |
| Flood control-stream messages | none | SEC-U4E-2 |
| Command another slot's soldier | the session's slot written over the command's (room.rs:753, :828-831) | the caller's routing (SEC-U4E-5) |
| Replay or duplicate commands | sequence-forward compare, wrap-aware; one command per tick, late ones dropped (input.rs:81-125) | none |
| Extra presses (fire faster) | press-edge compare (input.rs:90-106); cooldown, ammunition, reload edges in the sim (§4.2) | none |
| Commands stamped far in the future | the buffer's cap of 8 (input.rs:114-116) | the lead bound: U4f |
| Speed hack through the move vector | the sim scales a vector over 127 (sim world.rs:493-498) | the room's length check and its -21 counter: U4f |
| Aim beyond reach or outside the map | the sim's clamp (§4.2); the codec's field ranges | the aim-bounds check: U4f |
| Fake a view time for more rewind | `rewind_at` accepts only ticks the 2 s ring holds (rewind.rs:18-33) | tolerance and cap: U4h; reruns keep an old view time (SEC-U4E-8) |
| Fake latency through Pongs | hold counted up to one tick; outstanding window about 2 s (pump.rs:411-429) | ADR 0001's accepted risk (OQ-0001-5); the cap: U4h |
| Forged acknowledgements | must name a snapshot sent to this client, else the datagram's commands are refused (room.rs:717-750) | a forged ack frees `Resync` (SEC-U4E-4) |
| Never acknowledge | queue cap, one overflow resync, then `Kick(overflow)` (room.rs:973-994, :1015-1021) | none |
| Request resyncs | one in flight (room.rs:685-699) | SEC-U4E-4 |
| Read another player's ammunition, timers, velocity or effects | own-state section by slot (room.rs:1022-1040); the codec refuses owner-only kinds elsewhere | none |
| See enemies out of view (wallhack) | 64 m radius plus the area flood through open portals (room.rs:862-931); full states the same set | line of sight and heights: slice 3 (ADR 0001 part 9) |
| Learn a hidden shooter's identity | events routed by interest (room.rs:954-969) | the shooter id is always sent (SEC-U4E-6) |
| Learn another player's token | no token in any message, echo or `Debug` | none |
| Void a run as an outsider | a control before Join closes, never INVALID (pump.rs:360-363) | none |
| Report state (position, hit, health, ammunition) | no client-to-room message carries state (catalogue authority note; proto `Message`) | none |
| Crash the room with input | no panic path found on network data (block 5) | SEC-U4E-9 for U4f |

### Block 10. Verdict, durable notes, and what I did not check (done)
- **Verdict: APPROVE WITH FINDINGS.** One Medium (SEC-U4E-4) and eight Lows; none is Critical or High, none must be
  fixed before the merge. Each open finding is a security finding, so PR #40 stays needs-mustafa (it already is).
  The two one-line fixes (SEC-U4E-3, SEC-U4E-7) are worth folding into U4e's fix pass if one runs for other findings;
  a Low never triggers a push on its own.
- **The checklist's items met by U4e:** the pump's socket checks, buckets, deadline and pong rule; the roster's
  constant-time compare and bind rule; the client's 1,150 B and wrong-channel refusals; every negative case of §5 U4e
  (T17, A15) has a test (server/tests/pump.rs, room/tests/room.rs:347-419, client/tests/endpoint.rs:100-115); carries
  51, 53, 54 (the code: every kind counted, the snapshot skipped; its series name is the report's), 55 and 57's
  security halves met.
- **Durable notes for this seat (the threat model as applied here; no memory folder exists in this repo, so they live
  in these notes):** the room's routing is by slot alone, so the caller's session handling is a security boundary;
  every acknowledgement is a client claim and must never gate server work by itself; the spike's events carry the
  shooter id until slice 3; the client trusts the server, but its decoder must not panic.
- **Not checked:** I ran nothing (runs are optional for this seat; the test-engineer runs after me and can settle
  SEC-U4E-4, SEC-U4E-5, SEC-U4E-1, SEC-U4E-3 and SEC-U4E-7 by the tests named above). I did not re-review npt-proto's
  decoder (U4a's, reviewed then) beyond the paths U4e calls, nor the sim's own checks (U4d's). Constant time is shown by
  review, not by a timing measurement or a look at the generated machine code.
