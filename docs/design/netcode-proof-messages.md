# The netcode proof test's message catalogue

**Status:** Proposed (a design doc for the throwaway spike in `spikes/netcode-proof/`; not an ADR)
**Date:** 2026-11-06
**Author:** netcode-architect
**Used by:** the main session: the class audit test in unit U1, and npt-proto in unit U4

## 1. What this is

- **The subset.** This is the netcode proof test's part of ADR 0001's message catalogue, plus the test's own
  controls: the session controls a browser sends with a capability, and the room-wide controls on RS's local Unix
  sockets. It is the last entry of the plan's §4.1 layout (`.checkpoints/netcode-proof-test/plan.md`, revision 3.2),
  and the catalogue `.claude/rules/netcode.md` asks for: a protocol change in the spike updates this doc first.
- **Which draft text.** ADR 0001 as PR #41 changed it, now on main (merged as `d8c485a`, 2026-11-06). "ADR 0001:667"
  and "row 667" are lines of that text: they were read at the PR's head `16763a7`, and its final head `1b4094a`
  changed only lines 1665-1667, keeping every line number. "Plan §N" is a section of the plan.
- **Throwaway.** Nothing outside `spikes/` uses these types. The foundation writes its protocol crate fresh
  (ADR 0001:1722-1726) and reads this doc only as a reference.
- **Numbers.** Each one is traced to an ADR line or a plan section, or marked "proposed" with its arithmetic.

### How the audit test reads it
- The doc has exactly four tables. The test finds each one by its header row, which must match exactly:
  - "Wire messages": `| Type | Direction | Channel | Class | Authority | Size limit | Spike status | Draft |`
  - "Field groups": `| Message | Field group | Class | Sent to | Notes |`
  - "Session controls": `| Variant | Fields | Who may send | Check | Effect | Draft |`
  - "Room-wide controls": `| Control | Fields | Who | Check | Effect |`
- One row per line. Cells are split on `|`, and no cell contains one.
- In the Type, Variant and Control columns, the name is the first text in backticks.
- A Size limit cell starts with the limit in bytes (`166 B`; commas separate thousands). The rest of the cell gives
  its source or its arithmetic. Rows that are not built say `n/a (not built)`.
- Spike status is either `built in U4` or starts with `omitted: ` followed by the reason.
- A built row's Authority starts with `Sender:`; an omitted row's starts with `Draft:` and gives the draft's checks.
- A Class cell holds one value from the vocabulary below, and nothing else.

### The class vocabulary
- **A message's class** (the "Wire messages" table; the Class column of ADR 0001's catalogue: "Every message type has
  a visibility class; one with none fails the audit test", row 661):
  - `n/a`: the message tells no client anything about the world or another player. This covers everything a client
    sends, and the link's own `Ping` and `Pong`;
  - `owner`: only the client it is about receives it;
  - `public`: any client in the room may receive it;
  - `per field`: each field group has its own class (the "Field groups" table).
- **A field group's class** (the "Field groups" table; ADR 0001's state table, 634-658, and part 9 (e), 406-408):
  - `owner`: only the slot's own client;
  - `team`: only clients on that team;
  - `spatial`: only clients whose relevance region holds it and whose area connects to it (part 9 (a), (b)), or, for
    an event, clients its radius and area test pass (part 9 (f));
  - `public`: every client;
  - `authority-only`: never sent to any client. The state table writes this class as "none", with "Sent to: nobody"
    (638): see Q4.

### Where each check runs (plan §4.4)
- **npt-server** (wall time, from an injected `now`): every inbound message is decoded on its I/O runtime, and every
  size limit is applied there at the socket, before any L2 delay buffering. It also holds the `Join` deadline, rate
  limits, the wrong-channel check, the `Join`'s version and content hash, and the capability check.
- **npt-room** (counted in ticks, no clock): it sees only decoded messages. It holds the command checks, the roster
  and its tokens, `Resync` and `ClientSettings` rules, the event queue and the rewind cap.
- **npt-client** checks what the room sends. It reads no clock (ADR 0001:242-247).
- Every refusal is counted by kind (PM-0001-21's counters).

### Framing and channels (proposed)
- **Channels** are npt-link's `Channel` (plan §4.7): `datagram` is `Datagram`; `control stream` is `Control`, the one
  bidirectional stream per session; `full-state stream` is `FullState`, a unidirectional stream the room opens for each
  full state (ADR 0001:425-426).
- **Datagrams:** one message each, starting with a 1 B type. Before `Join`, every datagram is dropped
  (ADR 0001:446). Any client datagram above 166 B (the largest client datagram limit, `Input`'s) is dropped before it
  is decoded.
- **Control stream:** each message is a 2 B little-endian length of what follows, then a 1 B type, then the body; the
  size limit counts all three. npt-server reads the length first and closes the session as malformed if the message
  would be above 512 B (the largest client message, `SessionControl`), and again, after the type, if it would be above
  that type's limit.
- **Full-state stream:** a 4 B length, then one `FullState`.
- **WebSocket:** one connection carries all three channels, so each binary message starts with a 1 B channel tag,
  followed by the same bytes as above. The server's WebSocket frame and message limits are 513 B (`SessionControl`'s
  512 B plus the tag: ADR 0001:452-453 asks for explicit limits). Its write buffer must hold the largest room message,
  a `FullState` of 65,536 + 5 B, so proposed 128 KB; plan E15's delaying wrapper still returns Pending above 64 KB.
- **Over a limit or malformed:** a datagram is dropped and counted; a stream message closes the session (with
  `Kick(malformed)` where the stream still works), counted by reason.

## 2. Wire messages

Built rows follow plan §4.7's order (13 messages, then `SessionControl`); the rows the spike leaves out follow, in the
draft's order. Omitted rows whose draft direction or channel lies outside the vocabulary keep the draft's words (Q5).

| Type | Direction | Channel | Class | Authority | Size limit | Spike status | Draft |
|---|---|---|---|---|---|---|---|
| `Input` | client → room | datagram | n/a | Sender: the joined session of a rostered slot, browsers and npt-bots alike (bots send the same commands: ADR 0001:515-517); before `Join`, dropped (446). npt-server: the size at the socket; trailing bytes or unknown fields drop the datagram whole (238); arriving on a stream, it is on the wrong channel: refused and counted as forged; a per-connection rate limit (proposed: a bucket of 8 datagrams, refilled at 2 × TICK_HZ a second, so 60 at 30 Hz). npt-room: the command's tick strictly newer than the last command run and inside the lead bound; the sequence moving forward, wrap-aware; the move vector's length at most 1; at most one press edge per button for each tick a command covers; the aim point inside the map bounds, and beyond the weapon's reach clamped, never refused or counted; fire, reload and cooldown edges against the sim's own state; the acknowledged snapshot one the room sent; at most `CMD_CATCHUP` = 8 commands per client per tick; the view time within one interval (33.3 ms) of the room's estimate, else the estimate (362-367); the input buffer's depth and the late-command counter (231-239, 250-255; plan §4.4, §4.6) | 166 B (proposed): the draft's rule is command size × the redundancy cap (443-444; row 667), and a datagram also has a header (Q9); with the draft's estimates, a header of about 6 B and whole commands of at most 20 B (706-710), that is 6 + 8 × 20 = 166 B. U4a's codec gives the maxima (input.rs's field table): a header of 5 B, a whole command of 23 B and a repeat of at most 19 B, so 5 + 23 + 7 × 19 = 161 B at the cap, inside the 166 B limit; the codec never codes a repeat larger than the whole command | built in U4 | row 667; part 5 (220-275); PM-0001-13, -20, -21, -58 |
| `Snapshot` | room → client | datagram | per field | Sender: the room only. npt-server decodes only client → room types, so a datagram with this type from a client is refused and counted as forged. npt-client: the size before decoding; a decoder that never panics (436); a group it cannot decode is dropped (row 668); each group decodes alone and is acknowledged on its own (293-294); arriving on a stream, it is on the wrong channel: dropped and counted (plan E7). Over WebSocket the backend sends only the newest snapshot (424; plan §4.7) | 1,000 B of WebTransport payload per datagram, at the send side, until PM-0001-36 measures; then min(the smallest `maxDatagramSize` the four browsers report, 1,150 B) (291-293; PM-0001-14). npt-client refuses a datagram above 1,150 B. All of one client's snapshot datagrams in one interval stay within about 1,067 B, counted as UDP payload plus 28 B each (the 256 kbps cap: 290-291) | built in U4 | row 668; part 6 (276-330); PM-0001-12, -14, -17, -57 |
| `FullState` | room → client | full-state stream | per field | Sender: the room only: at join, after a `Resync`, or when it forces a resync because the client's event queue overflowed (294-302); one stream per full state, and at most one open per session (proposed, from `Resync`'s one-in-flight rule). npt-server refuses a client-opened stream beyond its stream limit (443). npt-client: the length before it allocates; deltas resume once it has acknowledged the full state (295) | 65,536 B (proposed): about 10 × the stress scene's whole world, which is about 6.5 KB by my arithmetic with proposed widths: 48 soldiers × 90 bits = 540 B; 300 trajectories × about 16.5 B = 4,950 B; 4 vehicles × 20 parts × about 8.5 B = 680 B; 11 vehicles × about 17 B = 187 B; own state and header about 180 B. PM-0001-19 and -56 measure the real size (R2-XU-OQ15) | built in U4 | row 672; part 6:294-302; PM-0001-19, -22 |
| `Resync` | client → room | control stream | n/a | Sender: a joined, rostered session that has lost its state (a Worker restart, a long hidden tab: row 673). npt-server: the size; no body. npt-room: one in flight: a second `Resync` before the client acknowledges the answering `FullState` is dropped and counted (row 673, proposed there; plan §4.4); answered with `FullState` | 8 B (proposed: 2 B length + 1 B type = 3 B, rounded up) | built in U4 | row 673; part 6:296-297; PM-0001-19 |
| `Ping` | both ways | datagram | n/a | Sender: either end of a joined session. The room pings each client a few times a second (row 669, proposed there), and answers a client's `Ping` on its I/O runtime as it arrives (247-248). npt-server: the size; a rate limit (proposed: 10 a second per session; more are dropped and counted); before `Join`, dropped. npt-client: the size; it answers at its next tick (248) | 32 B (proposed: 1 B type + 1 B ping id + 8 B sender's time = 10 B, rounded up to `Pong`'s limit) | built in U4 | row 669; part 5:239-250; PM-0001-33, -47 |
| `Pong` | both ways | datagram | n/a | Sender: the end that received the `Ping`. npt-server: the size; the ping id one it sent on that session and still outstanding (proposed), else dropped and counted; the reported hold time counted up to one tick period, 33.3 ms at 30 Hz, never more (248-250; row 669). npt-client: the size; the ping id one of its own | 32 B (proposed: 1 B type + 1 B ping id + 8 B echoed time + 4 B receiver's tick + 2 B hold time in µs = 16 B, doubled) | built in U4 | row 669; part 5:247-250; PM-0001-33, -50 |
| `Join` | client → room | control stream | n/a | Sender: any connection, as the control stream's first message, within 2 s of its TCP accept or its QUIC handshake's first packet (npt-server, wall time, never counted from the end of an L2 delay: 445-446; plan §4.4). npt-server: the size; the protocol version and content hash equal to the room's (plan §10.3); any other type first on the stream closes the session as malformed, counted by reason and never INVALID (plan §6.1). npt-room: the test token compared in constant time with its slot's token in the roster; `bind` equal to the value that slot bound at its first accepted `Join` (notes below); no join beyond the roster's N; a `Join` on a slot that still has a live session replaces it (`Kick(replaced)`); refusals and strays counted per run, and a measured run that admitted a session outside its roster is INVALID (plan §6.1, §10.3) | 128 B (proposed: 2 B length + 1 B type + 16 B test token + 16 B bind + 2 B protocol version + 32 B content hash = 69 B, rounded up) | built in U4 | row 670; part 11:494-506, with a test token in place of the lobby's signed one (1641); plan §10.3; PM-0001-19, -25 |
| `JoinAccepted` | room → client | control stream | owner | Sender: the room only, once per accepted `Join`, to that client. npt-client: the size; each value in range; a `FullState` follows on a full-state stream | 256 B (proposed: 2 B length + 1 B type + 3 B slot, team and squad + 2 B mode id + 32 B mode hash + 32 B content hash + 4 B server tick + 4 B tick and snapshot rates = 80 B, rounded up) | built in U4 | row 671; part 11:506 (the room assigns team and squad) |
| `JoinRejected` | room → client | control stream | owner | Sender: the room only; it then closes the connection. It carries a reason code and nothing else (proposed codes: bad token, bind mismatch, roster full, wrong version, wrong content hash). npt-client: the size | 16 B (proposed: 2 B length + 1 B type + 1 B reason = 4 B, rounded up) | built in U4 | row 671; plan §10.3 |
| `ClientSettings` | client → room | control stream | n/a | Sender: a joined, rostered session, when its delay changes. npt-server: the size. npt-room: the announced interpolation delay clamped to the mode's range, 2 to 5 snapshot intervals (66.7 to 166.7 ms at 30 Hz: 337-339), and moved by at most one interval a second, that is 30 ticks (349-351); clamped, never refused (counted: proposed) | 8 B (proposed: 2 B length + 1 B type + 2 B delay = 5 B, rounded up) | built in U4 | row 674; part 7:337-351; PM-0001-46 |
| `RoomEnded` | room → client | control stream | public | Sender: the room only, to every session: at the run's end, or with the crash reason from the tick thread's top-level catch, where it can (488-490). npt-client: the size; it then closes the session | 64 B (proposed: 2 B length + 1 B type + 1 B reason (ended, stopped, crash) + 4 B final tick = 8 B, rounded up; the draft's "results" are empty here, since the stand-in sim keeps no score) | built in U4 | row 680; part 11:486-494; PM-0001-59 |
| `Leave` | client → room | control stream | owner | Sender: a joined session, about itself only. npt-server: the size; the reason code in range. npt-room: ends that session; the slot keeps its token for a rejoin (the spike has no reconnect grace window: 1640-1641) | 16 B (proposed: 2 B length + 1 B type + 1 B reason = 4 B, rounded up) | built in U4 | row 681 (`Leave` / `Kick`, "either way": here `Leave` is the client's and `Kick` the room's) |
| `Kick` | room → client | control stream | owner | Sender: the room only; it then closes the connection. Reasons (proposed): replaced (plan §10.3), malformed (a stream message that fails its framing, size or type checks), overflow (the event queue, when the room drops the client instead of resyncing it: 302), stopped (the operator). npt-client: the size | 16 B (proposed: as `Leave`) | built in U4 | row 681; plan §10.3; PM-0001-22, -25 |
| `SessionControl` | client → room | control stream | n/a | Sender: only a joined, rostered browser session holding the capability minted for its slot (plan §10.2, §10.3); never a bot, and never the runner (plan §9). npt-server: parsed only after `Join`, and one sent before `Join` closes the session as malformed, counted by reason, never INVALID; the size; the capability compared in constant time (plan §4.4); a control from a rostered session without a valid capability is refused and counted, and a measured run in which such a control was sent, or any control was applied without a valid capability, is INVALID (plan §6.1); every control and form record logged with its source. The variants' own checks: "Session controls" table | 512 B (proposed: the largest variant, `RecordStep`: 2 B length + 1 B type + 1 B variant + 16 B capability + 1 B step + 1 B result + 2 B note length + a note of at most 480 B = 504 B, rounded up) | built in U4 | plan §10.2, §4.7, E23: a spike-only type, not in ADR 0001's catalogue; the checklist's overlay and form (1663-1671) |
| `SessionStatus` | room → client | control stream | owner | Sender: the room only, to the session holding its slot's capability (never a bot, never the runner), on change and at most once a second (the advisor's A1 ruling, 2026-11-06). It carries the overlay's room-side values: the profile in force, the server's round trip, the rewind cap, the command-timing option and the clipped-shot count, and the form's last saved step. npt-client: the size; each value in range; arriving on another channel, dropped and counted | 32 B (proposed: 2 B length + 1 B type + 1 B profile + 4 B round trip in µs + 2 B rewind cap in ticks + 1 B command-timing option + 4 B clipped-shot count + 1 B last saved step = 16 B, doubled) | built in U4 | the advisor's A1 ruling (2026-11-06), plan §4.7: a spike-only type, not in ADR 0001's catalogue; the checklist's overlay and form (1663-1671); PM-0001-53 |
| `RoomState` | room → client | control stream | public | Draft: the room sends it on change; no server checks (row 675) | n/a (not built) | omitted: nothing in the proof test reads it, and what it carries does not exist in the spike's room. (1) Phase and countdown: the spike's room has no lobby, countdown or match length, since the room lifecycle is a slice 2 and 3 milestone (ADR 0001:1640-1641); it runs from Open to the run's end, which `RoomEnded` reports (PM-0001-59), and PM-0001-16 times spawn to Open at the supervisor stand-in, not at a client. (2) The roster, human or bot per slot: the spike has no server-side bots, since every soldier is driven by a protocol client (npt-bots, plan E1), so "human or bot" means nothing on the wire; a client needs only its own slot, team and squad, which `JoinAccepted` carries. (3) No pass mark measures it, and under PM-0001-19's join pattern (bots leave and rejoin, keeping the room at 47 to 48: plan §10.3) a roster sent on every change would add control-stream traffic no mark asks for to every measured run: 100 joins make about 200 changes, each sent to about 47 clients (my arithmetic). (4) The `public` class stays in use through `RoomEnded` and the snapshot header's tick. (5) In this doc "the roster" means plan §10.3's admission list of tokens per slot, which is never sent. `RoomState` comes back with the slot table and room phases (ADR 0001 migration steps 4 and 5, 1731-1738) | row 675 |
| `HostCommand` | client → room | control stream | n/a | Draft: the host only; values inside the mode's allowed overrides (row 676) | n/a (not built) | omitted: the spike has no private rooms and no host (slice 2 and 3 milestones: ADR 0001:1640-1641); its settings come from the room's start-up configuration and the local control sockets ("Room-wide controls") | row 676 |
| `PickUpgrade` | client → room | control stream | n/a | Draft: an open offer to this player, a choice in range, not already picked (row 677) | n/a (not built) | omitted: the stand-in sim has no economy, offers or pickups; part 12 is proven at slice 4 by its refusal tests (ADR 0001:1642-1643) | row 677; part 12 |
| `Buy` | client → room | control stream | n/a | Draft: the id exists in the mode's content, the team has its tier, the player can pay and stands at the team's buy terminal (row 678) | n/a (not built) | omitted: the same reason as `PickUpgrade`: no economy in the stand-in sim (ADR 0001:1642-1643) | row 678; part 12 |
| `ContentReload` | room → client | control stream | public | Draft: development builds only, and only under OQ-0005-CD4's option (b) (row 679) | n/a (not built) | omitted: a development-build message that exists only if OQ-0005-CD4 picks the mid-match swap; the spike's content is fixed for each run and checked by its hash at `Join` | row 679 |
| `Lobby calls (HTTPS)` | browser → lobby | HTTPS | n/a | Draft: the Origin check; rate limits per client, per address and global (row 682) | n/a (not built) | omitted: the proof test has no lobby (a slice 3 milestone: ADR 0001:1640-1641, 1734-1735); test tokens and capabilities are minted on the room's local socket and reach browsers in the URL fragment, and npt-host serves files only, never minting tokens (plan §10.3, §10.4) | row 682 |
| `Supervisor calls` | both ways | local to the VM | n/a | Draft: ADR 0006's (row 683) | n/a (not built) | omitted: the supervisor stand-in, npt-super, starts, stops and kills rooms as systemd units through `npt`'s fixed sudo wrappers (plan §10.2, §10.5), sends the room no message, and may not depend on npt-proto (plan §4.2); a room's spawn arguments are its unit's configuration; how npt-super times Open for PM-0001-16 is Q3 | row 683 |
| `Lobby ↔ supervisor` | both ways | the lobby ↔ VM link | n/a | Draft: ADR 0006 part 6, the link authenticated (row 684) | n/a (not built) | omitted: no lobby (as `Lobby calls (HTTPS)`); each run plan names its rooms' host and ports (plan §7.2, §8.1) | row 684 |

**Notes on the rows**
- **`bind`** (in `Join`; proposed): 16 B the client makes at its first `Join` and repeats in every later `Join` with
  that token. In a browser it is the resume secret kept in page memory, so a later copy of the URL is useless
  (plan §10.3); on the runner it is the job's nonce, made at run time (plan §9); a bot makes one per process. The
  room binds the slot's token to it at the first accepted `Join`.
- **The capability** (in `SessionControl`; proposed): 16 B, carried in every session control, so the check needs no
  session state beyond the slot the capability was minted for (Q7).
- **Mode id and content hash in the spike:** the mode id names the scene (the stress scene, the target course, the
  correction course); the content hash covers the scene data and the build (plan §10.3).
- **Nothing is echoed back:** `JoinRejected` and `Kick` carry a code, never the offending bytes, and the tracing log
  records a length or a hash of client data, never the data (plan E22).
- **Authority (ADR 0001 part 1):** no message carries client state, a hit or a client-reported RTT (1633-1634). A
  command carries inputs, an aim point and a view time, each checked; the `Pong`'s hold time counts up to one tick.

## 3. Field groups

The groups of the two `per field` messages, with their classes from ADR 0001's state table (634-658). The
`authority-only` groups are listed under both messages, because a "whole state" encoder is where such data would most
easily leak. The spatial groups are packed by the priority accumulator under the per-client cap: the client's own
state always, the nearest 8 enemies in every snapshot, every relevant entity at least every 4 snapshots (part 9 (d)).

| Message | Field group | Class | Sent to | Notes |
|---|---|---|---|---|
| `Snapshot` | header: server tick | public | every client | the full 32-bit server tick, in every datagram (318-321); the state table's "server tick and clock" (650) |
| `Snapshot` | header: baseline and acknowledgements | owner | that slot's client | the baseline as its 16-bit age in ticks from the header tick, 0 meaning coded against the per-type defaults (319); the group index and the group count, so the client knows when a tick is complete; the newest command received, which ends its repeats, and the last command run, where the replay starts, each as a 16-bit command sequence with a known flag (228-230: a sequence names the command, while the tick it ran at can differ from its stamp; U4a as built, the advisor's W1-N2 ruling) |
| `Snapshot` | own state | owner | that slot's client | position and velocity, unquantised or the room's own copy quantised the same way each tick; weapon state (ammo, fire timers, reload, active reload); the stand-in attribute rows own movement and firing read, for PM-0001-60; sent when changed (203-209; state table 636-637) |
| `Snapshot` | remote soldiers | spatial | clients whose region holds the soldier and whose area connects (part 9 (a), (b)) | E3: position to 1 cm in 16, 16 and 12 bits, yaw in 16 bits, pitch in 10 bits; health; 8 flag bits for stance and actions; weapon id; named by kind and index, with the generation in the record that brings it into the set; a respawn at a held index is a removal plus an addition (279-281, 309-318) |
| `Snapshot` | vehicles | spatial | clients whose region holds the vehicle's bounds plus its visual radius (part 9 (f)) | hull and turret transforms and health per part; parts travel with their parent (313-314); the stand-ins: 15 big vehicles, 4 of at least 20 parts (plan E1) |
| `Snapshot` | projectiles | spatial | clients the projectile is relevant to, from the tick it becomes relevant | trajectory records (type, start tick, start position, velocity, fuse), re-based to that tick and re-sent only when the trajectory changes; the blast that ends one is its last change (281-289, 304-306); the thrower gets it too (state table 641) |
| `Snapshot` | world pieces | spatial | clients whose region holds the piece | the doors that open and close once a second and the destructible-wall stand-in (plan E1); part of the draft's "spatial records" (row 668); proposed: a portal counts as inside both areas it joins |
| `Snapshot` | gameplay events | spatial | clients the event's radius and area test pass (part 9 (f)); shots at or hitting the viewer always | hits, deaths and kills, and blasts that end no projectile; stamped with their tick and resent until acknowledged, from a bounded queue (297-302); no shooter id when the shooter is not relevant to the viewer |
| `Snapshot` | hit confirmation | owner | the shooter's client | a gameplay event, resent until acknowledged (308-309; state table 639) |
| `Snapshot` | cosmetic events | spatial | clients the event's radius and area test pass, never the client whose own shot it is | shots, muzzle flashes, sounds, and action starts (fire, reload, throw, melee); sent once, never resent (303, 309-311) |
| `Snapshot` | team section | team | that team's clients | empty in the spike: the stand-in sim has no team-private state (no economy: 1642-1643), so the encoder writes none |
| `Snapshot` | public section | public | every client | empty in the spike: no uplinks, countdown or steals (state table 645), so the encoder writes none |
| `Snapshot` | weapon spread stream | authority-only | nobody | SEC-SPREAD (a): the spread's draws and state stay on the room (212-215; the state table's class "none", 638) |
| `Snapshot` | RNG state | authority-only | nobody | every random stream's state (`ChaCha8Rng`, seeded by the host: plan §4.5); the client's prediction draws from no stream (193) |
| `Snapshot` | room-only records | authority-only | nobody | the per-tick state hash (no hash is exchanged with clients: 322-326); the hit-volume history (part 8); other clients' commands, view times and RTTs, and the view-time and aim side file (377-385); the roster's tokens and capabilities |
| `FullState` | header: server tick | public | the client it is for | the full 32-bit tick the whole state is taken at |
| `FullState` | header: baseline and acknowledgements | owner | the client it is for | no baseline (a full state is coded from the per-type defaults: 276-278); the last command run, where the client's prediction starts |
| `FullState` | own state | owner | the client it is for | as in `Snapshot`, whole |
| `FullState` | spatial world | spatial | the client's interest set at that tick | every remote soldier, vehicle, projectile (re-based to that tick) and world piece the client may see, whole. Proposed: no event section; a resync clears the client's queue of unacknowledged events, since the state already holds their results (Q8) |
| `FullState` | team section | team | that team's clients | empty in the spike (as in `Snapshot`) |
| `FullState` | public section | public | the client it is for | empty in the spike (as in `Snapshot`) |
| `FullState` | weapon spread stream | authority-only | nobody | as in `Snapshot` |
| `FullState` | RNG state | authority-only | nobody | as in `Snapshot`; a full state is the client's view, never a sim save |
| `FullState` | room-only records | authority-only | nobody | as in `Snapshot` |

## 4. Session controls

The variants of `SessionControl` (plan §4.7, §10.2). Every variant obeys these rules, which the Check column does not
repeat:
- **The capability:** 128 random bits, minted through the room's local socket for one browser slot (`MintCapability`
  below), delivered with that slot's token in the URL fragment, which the page drops with `history.replaceState` before
  the first render (plan §10.3), and compared in constant time by npt-server (plan §4.4).
- **Who:** only a joined, rostered session on that slot. A session control before `Join` closes the session as
  malformed, counted by reason, and never makes a run INVALID, so an outsider cannot void a run (plan §6.1).
- **Without a valid capability:** refused and counted. In a measured run, a rostered session that sends one without a
  valid capability, or a control applied without one, makes the run INVALID (plan §6.1).
- **The bites:** a control or form post from a plain join (a rostered session without the capability) is refused and
  counted; a control before `Join` closes the session (plan §10.2).
- **Logging:** every control and form record is logged with its source (the slot, the session, the server tick).

| Variant | Fields | Who may send | Check | Effect | Draft |
|---|---|---|---|---|---|
| `SetProfile` | `cap` (16 B); `profile`: P0, P1, P2, P3, P4 or `default` (1 B) | the capability holder: Mustafa's Brave session in the checklist (steps 2, 3 and 12), and the laptop's browser legs (proposed: PM-0001-45 and -46's rough checks) | npt-server: the profile is one of the six values | L2 applies the profile to every connection from the holder's source address, new ones included, from QUIC's first packet and the TCP accept, until `default` clears it or the run ends (plan §10.2); each direction gets (the profile's RTT − the holder's base RTT) ÷ 2 of delay, with the jitter, loss and reorder step of part 13 (568-586; plan R-9; PM-0001-3) | plan §10.2; ADR 0001:1665-1667, checklist steps 2, 3 and 12 (1675-1677, 1701); PM-0001-45, -46, -53 |
| `TriggerBurst` | `cap` (16 B) | the capability holder: Mustafa's Brave session (checklist step 11), and the laptop's browser legs (proposed: PM-0001-45's B1 run) | npt-server: refused while a burst on this session is still running (proposed) | one second of 100 % loss in both directions on this live session only, then back to its profile (B1: 570); both directions because step 11 has the player's own commands lost too (1697-1700, my reading); its start and end logged | plan §10.2; checklist step 11 (1697-1700); PM-0001-45, -53 |
| `BlockUdp` | `cap` (16 B); `on`: true or false (1 B) | the capability holder: Mustafa's Brave session (checklist step 13); PM-0001-29's own runs use the socket's `BlockPeerUdp` | npt-server: none beyond the shared rules | L2 drops every UDP datagram to and from the holder's source address, new connections included, until cleared or the run ends (plan §10.2); the live WebTransport session dies, and the page falls back to WebSocket after its 1.5 s timeout (423); the clearing control travels over WebSocket | plan §10.2; checklist step 13 (1705-1707); PM-0001-53 |
| `RecordStep` | `cap` (16 B); `step`: 1 to 14 (1 B); `result`: achieved, not yet, or observation (1 B); `note`: UTF-8, at most 480 B, with a 2 B length | the capability holder: Mustafa's session in R7 (plan §11, item 3) | npt-server: the step in 1 to 14 (the checklist's count, held as data); the result one of the three; the note valid UTF-8 with no Cc or Cf characters (proposed: part 11's rule for display names, 497-499) and at most 480 B | one form record in the results log (npt-log, never the tracing log: plan §6.1, E22), with its source and the server tick; nothing in the room changes | ADR 0001:1669-1671 (the form); PM-0001-53; plan §11 |

## 5. Room-wide controls

These controls never travel on the wire (plan §10.2, E23). Rules shared by every row:
- **The sockets:** one local Unix socket per room slot, `/run/npt/ctl/NN`. The directory is 0750 `root:npt-ctl` and
  each socket 0660; the group `npt-ctl` holds `npt` alone, and no room user is in it. The room checks with
  `SO_PEERCRED` that the peer's uid is `npt`.
- **Who drives them:** the laptop session over SSH, or the run script on RS.
- **Where they run:** npt-server owns the sockets (plan §4.4). It applies the L2, address-table, panic and stall
  controls itself, and passes the roster, token and rewind-cap controls to npt-room.
- **Size:** at most 1,024 B a message (proposed: the largest is a peer set of up to 16 addresses, 4 LG VMs with an
  IPv4 address and an IPv6 /64 each and twice that for margin, × 17 B (a 1 B family + 16 B) = 272 B, plus a header
  under 16 B). They get the same panic-free parsing as network messages (plan §10.2). A reply carries a status code,
  plus the 16 B token or capability where one is minted.
- **Logging:** every control is logged in the results log with its source and the server tick.
- **Not here:** start-up settings (Q2) and `npt-ctl peers` and the other sudo wrappers (plan §10.5), which are not
  room controls.

| Control | Fields | Who | Check | Effect |
|---|---|---|---|---|
| `MintToken` | `slot` (1 B); `kind`: bot, browser or runner (1 B) | the run script on RS, or the laptop session over SSH, both as `npt` | the peer's uid is `npt`; the slot below the room's N (48 in the stress scene); a runner token only for the dedicated runner slot (plan §9); npt-room keeps the roster, counting in ticks | replies with 128 random bits, the slot's token, replacing any earlier one (plan §10.3). Bots get theirs by file over the internal path, browsers in the URL fragment. Every token expires at the run's end; the runner's expires 30 minutes after minting if unused (54,000 ticks at 30 Hz), and once bound lives until its session ends, at most 60 minutes (108,000 ticks) (plan §9) |
| `MintCapability` | `slot` (1 B) | as `npt`: the run script on RS, or the laptop session over SSH | the peer's uid is `npt`; the slot is a browser slot, never a bot's and never the runner's (plan §9) | replies with 128 random bits, valid only for session controls sent by that slot's session (Q7); it expires at the run's end |
| `SetRewindCap` | `ticks` (2 B) | as `npt`: the run script on RS, or the laptop session over SSH | the peer's uid is `npt`; at least 1 and at most the hit history's length minus its margin (the history keeps the cap plus a margin, at most 1 MB: 374-375; sized for the test cap: plan R-20); npt-room, in ticks | from the next tick, every rewind is cut at that many ticks; recorded in the results log and shown read-only on the overlay (plan R-28; ADR 0001:1667-1669); the test cap for PM-0001-48, then the rule cap for PM-0001-49 and the checklist (plan §11) |
| `RaiseAddrCap` | `cap`: sessions per address (2 B); `addrs`: IPv4 addresses and IPv6 /64 prefixes | as `npt`: the run script on RS, or the laptop session over SSH | the peer's uid is `npt`; every address on the run's LG list (proposed: the list `npt-ctl peers` records at bring-up, plan §10.5); npt-server's bounded address table | the per-address cap of those addresses only, raised to the run's session count, for PM-0001-25's "cap raised" run (and ADR 0006's spike item 2); every other address keeps the cap; the setting goes into the results header |
| `InjectPanic` | `target`: the tick thread, or one slot's connection task (1 B); `at`: a server tick (4 B) | as `npt`: the run script on RS, or the laptop session over SSH | the peer's uid is `npt`; the tick in the future | PM-0001-59: the tick thread panics at that tick, and the top-level catch sends `RoomEnded` with the crash reason where it can, marks the match record crashed, keeps the crash report and exits non-zero (488-490). PM-0001-26: that connection's task panics, and only that connection closes |
| `InjectStall` | `ms` (2 B); `at`: a server tick (4 B) | as `npt`: the run script on RS, or the laptop session over SSH | the peer's uid is `npt`; at most 1,000 ms (proposed: the largest stall PM-0004-67 and PM-0006-23 inject); the tick in the future | the tick thread sleeps that long inside that tick: 200 ms and 1 s for PM-0004-67 (ADR 0004:1193), 1 s for PM-0006-23 (ADR 0006:1378) |
| `SetPeerProfile` | `peers`: a set of addresses, or `default`; `profile`: P0 to P4 (1 B) | as `npt`: the run script on RS, or the laptop session over SSH | the peer's uid is `npt`; the profile one of the five; every address on one of the run's lists (LG's, or the laptop's for its rough checks) | L2 applies the profile to those peers from their next packet, as `SetProfile` does; `default` sets the profile each new peer gets (plan §10.2) |
| `SetPeerThrottle` | `peers`; `kbps` (2 B); `secs` (2 B) | as `npt`: the run script on RS, or the laptop session over SSH | the peer's uid is `npt`; the rate above 0; at most 600 s (proposed: the longest impairment, PM-0001-24's 10 minutes) | PM-0001-23: the downstream to those peers limited to that rate (128 kbps for 10 s), with R-17's queue depths (plan E15) |
| `SetPeerBottleneck` | `peers`; `percent` of the snapshot rate (1 B); `secs` (2 B) | as `npt`: the run script on RS, or the laptop session over SSH | the peer's uid is `npt`; the share between 1 and 100 %; at most 600 s | PM-0001-24: the downstream cut to that share of the snapshot rate (50 %), for 10 minutes per congestion controller |
| `PeerBurst` | `peers`; `ms` (2 B) | as `npt`: the run script on RS, or the laptop session over SSH | the peer's uid is `npt`; at most 1,000 ms (proposed: B1's one second) | PM-0001-18: 100 % loss in both directions for that long (B1: 570), 10 bursts a run |
| `BlockPeerUdp` | `peers`, or `all`; `on`: true or false (1 B) | as `npt`: the run script on RS, or the laptop session over SSH | the peer's uid is `npt`; every address on one of the run's lists | PM-0001-29: every UDP datagram of those peers dropped until cleared, so WebTransport fails and the browsers fall back to WebSocket |
| `npt-load-ctl` | its verbs: start, feed and stop the load; copy token files | RS's run script as `npt`, with the ed25519 key cloud-init made on RS, which never leaves RS (plan §10.5) | not a room socket: LG's `authorized_keys` entry for `npt-load` is `from="<RS IPv4>",restrict,command="/usr/local/bin/npt-load-ctl"` (plan §10.5); proposed: npt-load-ctl accepts only its verbs, with no free arguments | starts, feeds and stops the load on LG and copies the bots' token files while the laptop is away; listed here because plan §10.5 puts the RS-to-LG key in the catalogue's control-plane table; the key dies with the VMs |

## 6. What the class audit test asserts

The main session writes the test. This is what it checks.

**At U1, on this doc alone:**
1. The doc holds exactly the four tables, found by their exact header rows (section 1), and no other table.
2. "Wire messages" holds plan §4.7's 15 names (`Input`, `Snapshot`, `FullState`, `Resync`, `Ping`, `Pong`, `Join`,
   `JoinAccepted`, `JoinRejected`, `ClientSettings`, `RoomEnded`, `Leave`, `Kick`, `SessionControl`, and
   `SessionStatus` by the advisor's A1 ruling), each exactly once with Spike status `built in U4`, and no other row is
   `built in U4`.
3. Each built row has a Direction of `client → room`, `room → client` or `both ways`; a Channel of `datagram`,
   `control stream` or `full-state stream`; an Authority that starts with `Sender:` and names at least one of
   npt-server, npt-room and npt-client; a Size limit that starts with a whole number of bytes; and a Draft that is
   not empty.
4. Every row's Class, omitted rows included, is one of `n/a`, `owner`, `public` and `per field`: a type without a
   class fails (ADR 0001:661).
5. `RoomState`, `HostCommand`, `PickUpgrade`, `Buy`, `ContentReload`, `Lobby calls (HTTPS)`, `Supervisor calls` and
   `Lobby ↔ supervisor` are each present, with a Spike status of `omitted: ` and a reason that is not empty.
6. In "Field groups", every Message is a built `per field` type, and each `per field` type has at least one row. Each
   Class is one of `owner`, `team`, `spatial`, `public` and `authority-only`. Every `authority-only` row is sent to
   `nobody`, and no other row is. Both messages list the weapon spread stream and the RNG state as `authority-only`.
7. "Session controls" holds exactly `SetProfile`, `TriggerBurst`, `BlockUdp` and `RecordStep` (plan §4.7), and each
   row's Fields start with `cap`.
8. Every "Room-wide controls" row's Who names `npt`.
9. Each assertion has a broken copy of the doc that must fail it (one class removed, `RoomState`'s reason deleted, one
   §4.7 type missing, and so on): the plan's "bites first" rule (§6.1).

**At U4, against npt-proto and the encoder:**
1. npt-proto's message types equal the built rows' Type names, and `SessionControl`'s variants equal the "Session
   controls" rows, both ways round: a type in the code with no row fails, and so does a row with no type.
2. Each type's channel and direction in the code equal its row's. A message received on another channel is refused and
   counted, on both sides (plan E7).
3. npt-proto's `Limits` equal the Size limit column. For each type, a buffer one byte over its limit is refused before
   anything is allocated (checked with PM-0001-34's counting allocator). (The allocation half, refused before anything
   is allocated, is checked with PM-0001-34's counting allocator in U5: the advisor's Q8 ruling; U4 proves the
   size-first refusal.)
4. The encoder's layout table names exactly the "Field groups" rows for `Snapshot` and `FullState`, each with the same
   class. The `authority-only` groups have no layout entry at all.
5. In an L1 run (the stress scene, 48 clients in 3 × 16, P2, 3 seeds), every group the encoder writes into each
   client's snapshots and full states is one its class lets that client see: `owner` only to its own slot, `team`
   only to that team, `spatial` only inside that client's interest set or event test, `public` to anyone. A bite that
   sends one `owner` group to every client must fail.
6. The control-socket messages decode with the 1,024 B limit and never panic (plan §10.2; Q6).

## 7. The design doc's other parts

This doc is the spike's catalogue only; the rest of a netcode design lives here:
- **Requirements** (48 clients, 30 Hz, P0 to P4, the budgets): ADR 0001 parts 2, 3 and 13, and its "Budgets for the
  netcode proof test" (724-742); plan §2.
- **Components and data flow:** ADR 0001:597-631; plan §4.3 and §4.4.
- **State ownership:** ADR 0001:634-658. The "Field groups" table is the spike's part of it.
- **Bandwidth:** ADR 0001:689-722. Snapshots stay within a snapshot interval's 1,067 B (290-291). The upstream mark
  is a mean of 200 B per datagram with 57 B of IPv4 transport overhead (710-713); `Input`'s ceiling, 166 + 57 = 223 B,
  passes it only at the cap with whole commands, which U2's delta-coded repeats avoid (221-225).
- **Failure modes:** ADR 0001:743-763; for messages, the Authority column and "Framing and channels" above.
- **Anti-cheat:** ADR 0001:765-786; for messages, the Authority column, the capability and the socket rules.
- **What the spike must prove:** the PM rows named in the Draft column, with PM-0001-34 fuzzing every type here.
- **Migration:** plan §12.1 (U1 writes the audit test; U4 builds npt-proto); ADR 0001's migration step 2 writes the
  foundation's protocol crate fresh (1722-1726).
- **Tests:** section 6, plus plan §6.2's rows.

## 8. Questions for the main session

**Answered by the advisor, 2026-11-06** (`.checkpoints/netcode-proof-test/advisor-u1.md`, items A1-A9):
- Q1: (a), `SessionStatus`, now a built row with its own fuzz target.
- Q2: `params.cmd_timing` and `params.cc`, start-up settings recorded in every header, never changed in a run.
- Q3: systemd's readiness notice, with `TimeoutStartSec` well above 2 s; npt-super's timing unit-tested, and first
  checked for real at R1's bring-up, before any PM-0001-16 start counts.
- Q4: `authority-only` only; the close's ADR 0001 results note the word against the draft's "none".
- Q5: as recommended; Q6: one fuzz target for the socket messages (and one for `SessionStatus`).
- Q7: one capability per browser slot (plan §10.2's text follows).
- Q8: approved for the spike; whether a full state carries the pending reliable events stays open for the foundation.
- Q9: "plus the header", a reading written into ADR 0001 at the close.

The questions as asked:
- **Q1, the overlay's status.** The overlay shows values only the room knows (the profile in force, the server's round
  trip, the rewind cap, the command-timing option, the clipped-shot count: ADR 0001:1663-1669), and the form gets no
  "saved" answer, but none of plan §4.7's room → client types carries them. Options: (a) a 15th type,
  `SessionStatus`, room → client on the control stream, class `owner`, sent only to capability holders, on change and
  at most once a second; (b) a spike-only `owner` group in `Snapshot`'s own state. Recommendation: (a), so the
  measured snapshot path stays as the draft has it and bots never receive it.
- **Q2, start-up settings.** OQ-0001-6's option (plan E3: "switchable") and the congestion controller (NewReno or
  Cubic, PM-0001-24) are on neither control list. Recommendation: settings in the room unit's start-up
  configuration, recorded in each run's header and never changed during a run.
- **Q3, timing Open.** With the supervisor calls omitted, and npt-super barred from npt-proto (plan §4.2), how does it
  time spawn to Open for PM-0001-16? Recommendation: systemd's readiness notice (a `Type=notify` unit that reports
  ready at Open), read through `systemctl`; to be verified at U8.
- **Q4, "none" or `authority-only`.** ADR 0001's state table gives weapon spread the class "none" (638); this doc
  calls never-sent groups `authority-only`. Should the draft's word change at the close, or should the audit accept
  both?
- **Q5, the omitted rows' words.** The lobby and supervisor rows keep the draft's direction and channel words
  ("browser → lobby"; "HTTPS"; "local to the VM"; "the lobby ↔ VM link"), outside the vocabulary. Recommendation: the
  U1 test checks Direction and Channel on built rows only (section 6, item 3).
- **Q6, fuzzing the sockets.** PM-0001-34 fuzzes "every message type of the catalogue" (1574). Does that include the
  room-wide controls? My reading: yes, one more fuzz target, since plan §10.2 gives them panic-free parsing.
- **Q7, one capability per what.** Plan §10.2 says "a per-run capability"; this doc reads it as one per browser slot
  in the run, minted for that slot, since it binds to one session and a run can have several browser legs. Confirm,
  or keep one per run.
- **Q8, resync and the event queue.** Proposed here: a `FullState` has no event section, and a resync clears the
  client's queue of unacknowledged events, since the state holds their results; the hit markers of that window are
  lost. PM-0001-22's "resynced" path depends on it. Confirm.
- **Q9, the draft's `Input` limit.** ADR 0001 drops an input datagram "larger than the command size times the
  redundancy cap" (443-444; row 667), which leaves no room for the header: a full datagram at the cap would be
  refused. This doc adds the header (6 + 8 × 20 = 166 B). Should the draft's words say "plus the header"?
