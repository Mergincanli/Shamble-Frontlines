# U4e round 1: security-auditor review (the room and the codec path)

Seat: security-auditor (workflow agent, opus). Read only: no edit to any copy; notes here only. Read copy:
`.gate/u4e-r1` (97bb9d3 + the snapshot patch). Base copy: `.gate/u4e-prefix` (1e8c188). Log prefix: `sec`.

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
