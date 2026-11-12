# U4e round 1: security-auditor review (the room and the codec path)

Seat: security-auditor (workflow agent, opus). Read only: no edit to any copy; notes here only. Read copy:
`.gate/u4e-r1` (97bb9d3 + the snapshot patch). Base copy: `.gate/u4e-prefix` (1e8c188). Log prefix: `sec`.
Cites below are into `.gate/u4e-r1/spikes/netcode-proof/crates/` unless a path says otherwise.

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

Findings:
- SEC-U4E-1 (Low): a forged kind first on the stream does not close the session before Join. See findings.
- SEC-U4E-2 (Low): control-stream messages have no per-connection rate limit. See findings.
- SEC-U4E-3 (Low): in the Joining state a stream message other than Join is dropped uncounted (:391's `_ => {}`).
