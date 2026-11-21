# U4e round 2: security-auditor review (on what changed: the fix pass FX-1 to FX-13)

Seat: security-auditor (workflow agent, opus). Read only: no edit to any copy; notes here only, written with the Write
tool (rewritten whole each time, content kept). Fixed copy: `.claude/worktrees/wf_0fa6595a-3e2-1` (uncommitted on
97bb9d3). Round 1's tree: `.gate/u4e-r1`. Log prefix: `sec`. Cites are into the fixed copy's
`spikes/netcode-proof/crates/` unless a path says otherwise. Judged against advisor-u4e-r1.md R1-R8 and T1 (the packet,
u4e-r2-packet.md).

## Plan
1. Check the copy equals fx-hashes-end.sha256 (sha256sum -c, read only) and that the diff stays inside R8's bounds
   (files only under crates/room, crates/server (not schedule.rs), crates/client; no manifest, Cargo.lock, npt-proto,
   npt-sim, npt-link or npt-lab file; one new public item `RttSource` and `take_control`'s changed type).
2. The pump (server/src/pump.rs): NC-M3 (`pong` from `r.arrival_ns`: can a client make the arrival lie?), A2
   (`RttSource`: can a ws client pick `Link`, or a wt client make its RTT what it wants), SEC-U4E-1 (pre-Join forged
   kind closes; does it close any legitimate path, does it reach Joining/Joined), SEC-U4E-3 (`joining_dropped`: is the
   catch-all now hiding any other state), TE-5's size checks (do the tests prove the cap the pump enforces).
3. The client's decode path (client/src/lib.rs): R1's case 3 (old_header, bad_header) against T1's sentence; bounded
   memory under hostile server bytes (bad_headers deque, out.hashes growth per input call, lost runs); SEC-U4E-7's
   checked add; any new panic (wrapping, indexing, the const assert).
4. The Resync limit (FX-7, room.rs): who can trigger a full state, how often; the forced (overflow) path; the join's;
   what one costs the room (encode, bytes, ack ring); can the limit be bypassed by Leave/re-Join or by a slot replace;
   does `last_full_state` reset where it must; wrap of `next.wrapping_sub(f)`.
5. A1's bytes (room.rs `send_control`, `full_state`, `take_control`; server/tests/common/mod.rs `stream_of`): what is
   dropped when an encode fails (silent?), the channel the caller reads from the type byte (a confusion that sends a
   FullState on Control or the reverse), the library contract for U6's caller.
6. Anything else in the changed code: new panics, unbounded growth (scripted_* vectors), cross-client leaks; the
   riders RR-2 (event age), RR-3, RR-10; anything beyond R8's scope.
7. Anti-cheat coverage table updated for what changed; verdict and findings in the schema.

## Blocks (written as each finishes)

### Block 1. The tree and R8's bounds (done)
- `sha256sum -c .gate/u4e/fx-hashes-end.sha256` in the fixed copy: 23 of 23 OK (before and after my one run).
- `git diff --no-index --stat .gate/u4e-r1/spikes/netcode-proof <copy>/spikes/netcode-proof` (target/ left out): 11 files,
  all under crates/client, crates/room, crates/server (src: client lib.rs, room ack.rs, room.rs, roster.rs, server lib.rs,
  pump.rs; tests: endpoint.rs, room.rs, common/mod.rs, loopback.rs, pump.rs). No manifest, Cargo.lock, schedule.rs,
  npt-proto, npt-sim, npt-link or npt-lab file. No dependency change, so no `cargo audit`/`cargo deny` delta.
- Public surface: `RttSource` (the one new item, re-exported at server/src/lib.rs:14), `take_control`'s type
  (room.rs:607), and two new pub fields of pub counter structs, `ClientCounters::resyncs_limited` (room.rs:110) and
  `Counters::joining_dropped` (pump.rs:146). Both fields are the names R3 and R5 ask for ("counted under its own name
  (`resyncs_limited`)"; "counted under its own name"), and a named counter of a pub-field struct can only be a pub field,
  so I read them as inside R3/R5's words; the builder flagged both for the advisor (u4e-build.md:252-253, :298). The
  public-item count is the engine-architect's seat; no security finding.

### Block 2. The pump (server/src/pump.rs) (done)
- **NC-M3 (:430, :457-473): holds.** The sample is `r.arrival_ns - echo - min(hold, one tick)`. `arrival_ns` is the
  link's stamp (server side), not a client value; `echo_ns` must equal the outstanding entry's own send time (:461), so a
  client cannot forge it; `hold_us` stays capped at one tick period (:467). A client can lower a sample by at most one
  tick (less RTT, less rewind: no gain) and raise it only by really delaying its Pong, inside the 8 x 250 ms outstanding
  window, ADR 0001's accepted risk (OQ-0001-5) with its cap at U4h. All arithmetic saturating; no panic.
- **A2 (:180-190, :298-303): holds.** The source is fixed at `Decoder::new` by the caller from its listener; nothing a
  client sends selects it, and one session never mixes the two. A ws client can influence only its own ping samples
  (as above); a wt session reads the transport's estimate (QUIC's ack-delay rules, RFC 9002 §5.3, are U6's transport
  concern, unchanged here).
- **SEC-U4E-1 (:382-388): closed.** In AwaitJoin a stream message refused as WrongChannel or WrongDirection now closes
  with `Kick(malformed)` and `first_not_join`; every other stream refusal already closed (`stream_malformed`, :375-381);
  a decodable non-Join closes (:401-405). So no stream message but a valid Join survives AwaitJoin. Joining and Joined
  keep §4.10's count-and-go-on (:389). Datagram refusals before Join stay counted and dropped (not "on the stream").
  Test `sec_u4e_1_a_forged_kind_first_on_the_stream_closes_the_session` (server/tests/pump.rs): the Join after it is
  never read; plant P-13a fails it by name (u4e-build.md:290-297).
- **SEC-U4E-3 (:432-434): closed.** I walked the match: (AwaitJoin, Join), (AwaitJoin, _, stream), (AwaitJoin|Joining,
  _, datagram), (_, Join), and the four Joined arms leave exactly (Joining, non-Join, stream) for the catch-all; Closed
  never reaches the loop (:338-340) and every close returns. So the new counter hides no other state. Plant P-13b fails
  the test by name.
- **TE-5's size checks: hold, and they test the socket check, not only the counter.** `channel_max(ClientToRoom,
  Control)` = 2 + (512 - 2) = 512 (proto msg.rs:242-283: SessionControl's 512 B row, its 2 B length counted), and the
  test's cap is that same row (512), so its 513 B message is refused at the channel check (msg.rs:398) before any type
  or body work; `channel_max(ClientToRoom, Datagram)` = 166 (Input's row), so the 167 B Input is refused there too. The
  stream one closes the session (`stream_malformed` 1) with `Kick(malformed)` last; the datagram one is counted and
  the session stays open. Neither test shows the boundary value itself taken (166 B or 512 B), which is npt-proto's
  (U4a's) to show; not a gap of the pump.
- No new panic, index or unchecked arithmetic in the changed lines.

### Block 3. The client's decode path (client/src/lib.rs) (done)
- **R1's case 3 (:493-530, :553): matches R1's form and T1's sentence.** `old_header` runs before `see` and `slot_for`
  in `group` (:684), `full_state` (:777) and the refused datagram's `sentinel` (:597); a tick at least the ring behind
  the newest (:493-497) and before `first` (:507-509) files `(t, 0)` once (`bad_header`), never acknowledged, the newest
  unmoved, no ring slot touched; else `stale` (case 2). `refused` counts it once (count=false where `receive` counted
  already, :473-474). `first` is set only by the first header that reaches `see` (:551-554), which in an honest run is
  the join's full state (the room sends no snapshot while it is unacknowledged), as R1 argues. One wording nuance for
  the netcode seat, no finding: `first` may also come from a refused snapshot datagram's raw tick (the field's doc,
  :220-222), where T1 says "the client's first snapshot"; in an honest run the two are the same tick.
- **Bounded memory under hostile server bytes: holds.** `bad_headers` holds at most `ACK_RING_TICKS` (32) ticks
  (:525-528; `contains` is O(32)); the ring stays 32 frames; pings at most 8 (:466); `out.hashes` and `out.lost` grow by
  at most one entry per received message between steps (drained at `step`, :454), so in proportion to bytes the host
  already holds. A hostile server can make the deque evict and so file one old bad tick's sentinel a second time after
  32 other distinct bad ticks; that needs a hostile server (trusted, behind TLS), never an honest or bitten run (one
  tick's groups arrive together, well inside 32), and a duplicate `(t, 0)` is the same hash twice. Observation only.
- **Case 3 cannot be used to corrupt live state:** it never calls `see` or `slot_for`, so a hostile header before
  `first` cannot overwrite ring slot `t % 32` (R1's point) or move `newest`. A far-ahead header still moves `newest`
  (NC-L7, carried to U4f: carry 80).
- **SEC-U4E-7 (:584-593): closed.** `checked_add`; a sum past `u32::MAX` starts a new run whose start is the old run's
  end (wrapping), so the runs stay contiguous; the `see` walk is still at most 32 steps plus one run (:568-579). Test
  `sec_u4e_7_lost_runs_2_pow_31_apart_never_overflow_and_cover_the_ticks`; plant P-13c panics "attempt to add with
  overflow" by name (u4e-build.md:303-309). No other add on a server-sized value in the changed lines.
- RR-3's const assert (:169-172) is compile-time.

### Block 4. The Resync limit (FX-7, room/src/room.rs) (done)
- **The rule as built (:716-738):** a `Resync` is answered only when no full state is due or in flight and either none
  was sent yet or `next_tick - last_full_state >= FULL_STATE_ACK_TICKS` (60 at 30 Hz); else dropped and counted
  (`resyncs_dropped` or `resyncs_limited`). `last_full_state` is set by every full state the room sends (:1125: the
  join's, a forced one, an answered one), only when its encode succeeded. The ack is no longer what frees the next
  full state: the 2 s spacing holds whatever the client acknowledges, which is the rule I recorded in round 1. Wrap:
  `f` is always a tick already run, so `next.wrapping_sub(f)` is the true distance (>= 1). No panic, no allocation.
- **Who can trigger a full state, and how often:** (1) a `Resync`: at most one per 60 ticks per client slot; (2) the
  room's overflow: not client-paced (it needs 1,024 gameplay events relevant to the client; carry 57's kick ends a
  second one inside an unacknowledged forced resync), and R3 leaves it outside the limit; (3) an accepted `Join`:
  every one builds a fresh `ClientSlot` with `full_state_due: Some(false)` and `last_full_state: None` (:648-673, :685),
  so a replacement (the slot's token and bind on a new session) or a re-join after `Leave` gets the join's full state
  at the next tick whatever the old slot's clock said. The committed test
  `a_join_on_a_slot_with_a_live_session_replaces_it` (room/tests/room.rs:213-224) already shows a fresh full state one
  tick after the first. The pump refuses a second `Join` on one session (`join_again`, pump.rs:407), so each such full
  state costs the client a new connection; the room has no per-slot join rate, and U6's handshake cap and address
  table are what bound it. -> SEC-U4E-R2-1 (Low).
- **What one costs the room:** one encode of the client's relevant set (about 6.5 KB for the stress world by the
  catalogue's arithmetic, at most 65,536 B), the frame cloned for the message and kept in the ack ring, the queue
  cleared; that client's snapshots stop until its ack (:1083-1089), so the tick's encode work for it is replaced, not
  added. Bytes outside the 256 kbps window: by `Resync`, at most one full state per 2 s per client (about 26 kbps at
  6.5 KB), against Q-6's 29 in 30 ticks in round 1. 48 coordinated clients can line their answers up on one tick every
  2 s; each replaces that client's snapshot encode, so the spike is in bytes on the reliable streams, not in CPU.
- **The contract text:** the catalogue's `Resync` row (docs/design/netcode-proof-messages.md:90, in the copy) still
  gives only the one-in-flight rule; the room now also drops, with no reply, a `Resync` sooner than 60 ticks after the
  last full state, which a client author must know to retry. `.claude/rules/netcode.md`: every protocol change updates
  the message catalogue. -> SEC-U4E-R2-2 (Low).
- **The rate:** the limit names `npt_sim::FULL_STATE_ACK_TICKS` by path (R3's words), the 30 Hz value, while
  `Room::new` takes `tick_hz`; at 60 Hz it is 1 s. That is RR-6's existing card follow-up (the room's full-state
  deadline and ring use 30 Hz constants); its list should name this new site. -> SEC-U4E-R2-3 (Low).
- Tests: `sec_u4e_4_a_resync_sooner_than_full_state_ack_ticks_after_the_last_full_state_is_limited` (the join's full
  state, 59 asked, 0 answered, then one at the window's end) and the re-pinned e9 (after an answered `Resync` and its
  ack, a new one limited, then taken past the window: the Q-6 attack's order). Plant P-7 fails the first by name
  (u4e-build.md:252-260).

### Block 5. A1's bytes (done)
- `ClientSlot::send_control` (room.rs:327-332) and `full_state` (:1117-1130) encode once with the room's `Quant`; the
  bytes are per client, from that client's own relevant set and own-state records: no cross-client path. The lab no
  longer encodes (server/tests/common/mod.rs:304-307).
- An encode refusal in `send_control` is dropped uncounted, but it is unreachable for the two messages that use it:
  `JoinAccepted` is fixed-width (80 B of its 256 B limit; proto msg.rs:355-365) and `Kick` one byte (3 B of 16 B).
  `full_state`'s refusal stays counted (carry 54) and retried the next tick, as in round 1. Observation, no finding.
- **The channel:** `take_control` hands out bytes with no channel, so a caller must find the type byte at an offset
  that depends on the channel it is trying to learn (2 on the control stream, 4 on the full-state stream). The lab's
  `stream_of` (common/mod.rs:47-58) reads offset 2 first: right for every message the room encodes today, because a
  full state's 4 B length is at most 65,536 (`Limits::FULL_STATE`), so its byte 2 is 0 (no type) or 1 (`Input`, a
  datagram row), and it falls through to offset 4. A full state of 4 x 65,536 B or more (if the limit ever grew) would
  read byte 2 as a control row's type and go on the control stream. Not reachable by a client (the bytes are the
  room's own) and its effect is a liveness failure (the client refuses its own full state), never a leak. It is the
  form R2 ruled ("the caller sends each on the channel its type byte's row ... names"), and test code; U6's TickLoop
  should not copy the probe. -> SEC-U4E-R2-4 (Low, a follow-up beside A9).
- `control` per slot is bounded by the full-state rate above plus one `JoinAccepted` and at most one `Kick`, and is
  drained each time the caller takes it.

### Block 6. The rest of the changed code (done)
- A4's `scripted_*` vectors (room.rs:410-413, :617-629, :833-838): server API, replaced per call and drained into the
  tick's input; not client-sized.
- RR-2 (room.rs:1211-1220): `u8::try_from` with a debug assert and a `continue`; `route_events` expires every event older
  than 255 ticks before the encode on every tick (room.rs:985, events.rs:85-91), queue under the bites-only unbounded
  flag included, so the assert is an invariant check no client input reaches (P-G reaches it only by removing the
  expiry).
- RR-10, A5's visibility changes, the const assert in ack.rs: no security effect.
- No new `unwrap`/`expect`/index on network data in src; the new `expect`s and panics are in tests only.
- Nothing changed outside R8's scope and bounds (block 1); the two new counter fields are inside R3/R5's words.

### Block 7. My run (done)
- `bash u4e-run.sh <copy> .gate/u4e/r2-sec-tests.log cargo test --locked --all-features -p npt-server --test pump ::
  cargo test --locked --all-features -p npt-client --test endpoint :: cargo test --locked --all-features -p npt-room
  --test room -- sec_u4e_4 e9_a_second_resync a_join_on_a_slot_with_a_live_session` (shared lock): exit 0. "test
  result: ok. 18 passed; 0 failed" (pump), "ok. 13 passed; 0 failed" (endpoint), "ok. 3 passed; 0 failed; ... 24
  filtered out" (room). 23 of 23 hashes OK after it. No failure, so no TE2-5 record. Plants are the test-engineer's.

### Block 8. Findings (done)
Severity on CLAUDE.md's one scale. None is Critical, High or Medium; none must be fixed before the merge.

**SEC-U4E-R2-1, Low, not before merge (route: C-U6-12 or C-U6-14's text, the advisor's).** room.rs:648-673 (a fresh
`ClientSlot`: `full_state_due: Some(false)` :656, `last_full_state: None` :658), :685; room.rs:1083-1085.
- What: FX-7's clock lives in the `ClientSlot` every accepted `Join` rebuilds, so a replacement or a re-join after
  `Leave` gets a full state at the next tick whatever the slot's last full state was.
- Exploit sketch: a player's script opens a new session each tick with its own token and bind; each `Join` replaces
  the live one and is answered with `JoinAccepted` and a full state (6.5 KB here, up to 64 KB), bypassing the
  once-per-2 s limit; the counters reset with the slot.
- Evidence: the committed test room/tests/room.rs:213-224 (a fresh full state one tick after the join's); pump.rs:407
  (one `Join` per session, so each costs a new connection).
- Reach: the slot's own token and bind (the player itself), one connection handshake per full state; U6's handshake
  cap and address table bound it, and C-U6-14 closes the replaced session.
- Fix (U6): keep the slot's last full-state tick outside `ClientSlot` (for example in the roster entry) and treat the
  join's full state of a replacement or re-join inside the window like a limited `Resync` (the join still answered,
  its full state deferred to the window's end), or a per-token join rate at TickLoop; a test of N replacements in N
  ticks giving at most one full state per window.

**SEC-U4E-R2-2, Low, not before merge (route: a catalogue line in the merge commit, like T1, or with C-U6-12's
catalogue rows).** docs/design/netcode-proof-messages.md:90 (the `Resync` row); room.rs:726-732.
- What: the room now drops, with no reply, a `Resync` sooner than `FULL_STATE_ACK_TICKS` after the last full state it
  sent that client; the catalogue's row gives only the one-in-flight rule.
- Sketch: no attack; a client written to the catalogue sends one `Resync` and waits for an answer that never comes.
- Fix: the row gains "and at most one answered per `npt_sim::FULL_STATE_ACK_TICKS` ticks after the last full state the
  room sent that client; a sooner one dropped and counted (`resyncs_limited`), so a client resends" (the netcode rule:
  every protocol change updates the message catalogue).

**SEC-U4E-R2-3, Low, not before merge (route: RR-6's card follow-up gains the site).** room.rs:728.
- What: the limit uses the 30 Hz constant by path while `Room::new` takes `tick_hz`; at 60 Hz it is 1 s, two answered
  `Resync`s per 2 s. As R3 worded it; the same class as RR-6 (room.rs:1045; ack.rs:82-84 in the fixed copy, round 1's
  :76-77).
- Fix: when a unit first runs a room at another rate, `full_state_ack_ticks_at(tick_hz)` here too.

**SEC-U4E-R2-4, Low, not before merge (route: a follow-up beside A9; U6's TickLoop).** room.rs:603-613;
server/tests/common/mod.rs:44-58.
- What: `take_control`'s bytes carry no channel; the caller infers it from a type byte whose offset depends on the
  channel. The lab's `stream_of` is right for every message the room sends today only because a full state's length
  stays at most 65,536 B (its length byte 2 is 0 or 1).
- Sketch: no client can reach it; a future full state of 4 x 65,536 B or more would go on the control stream and the
  client would refuse it (liveness, never a leak).
- Fix: hand out each message with its `Limits::TYPES` row (or its channel), or an npt-proto helper that reads it, at
  the next npt-proto or TickLoop change; TickLoop does not copy `stream_of`.

**Round 1's findings of this seat that R8 put in the fix pass:** SEC-U4E-1 closed; SEC-U4E-3 closed; SEC-U4E-7 closed;
SEC-U4E-4 room half closed (the transport half stays open as C-U6-12, so SEC-U4E-4 stays listed open in PR #40's
report for that half). Still open by R3/R5's routes (unchanged, not in the fix pass): SEC-U4E-2 (C-U6-12), SEC-U4E-5
(carry 76, C-U6-14), SEC-U4E-6 (known deviation until slice 3), SEC-U4E-8 (carry 81), SEC-U4E-9 (carry 80).

### Block 9. Anti-cheat coverage table, round 2 (what changed marked) (done)
| Player action | Where the server validates it | Gap |
|---|---|---|
| Connect and send nothing (slow loris) | 2 s Join deadline (pump.rs:341-345) | AddrTable, handshake cap, idle timeout after Join: U6 |
| Join with a guessed or stolen token | constant-time roster compare, bind at first accepted Join (roster.rs) | stale sessions routed by slot (SEC-U4E-5, C-U6-14) |
| Join with a wrong version or content hash | pump, before the room (pump.rs:440-452) | none |
| Datagrams or controls before Join | `before_join`; any non-Join stream message first closes, a forged kind included (pump.rs:375-405) **(changed: SEC-U4E-1 closed)** | none |
| Stream messages while the Join is answered | dropped and counted `joining_dropped` (pump.rs:434) **(changed: SEC-U4E-3 closed)** | none |
| Malformed, oversized, trailing or unknown-field bytes | `Quant::decode` at the pump, counted by kind; a bad stream message closes; size refused at the channel cap first (proto msg.rs:398) **(TE-5's tests added)** | none |
| Flood Inputs | Input bucket 8, 2 x tick rate a second (pump.rs:408-414) | none |
| Flood Pings or Pongs | Ping bucket 10/s; a Pong matched among 8 outstanding (pump.rs:415-430, :457-473) | Pongs unlimited but O(8) each and counted |
| Flood control-stream messages | none | SEC-U4E-2 (C-U6-12) |
| Command another slot's soldier | the session's slot written over the command's (room.rs:792) | the caller's routing (SEC-U4E-5) |
| Replay or duplicate commands; extra presses | sequence-forward and press-edge compares (input.rs) | none |
| Commands far in the future; speed or aim beyond bounds | buffer cap 8; the sim's clamps | the lead bound, length and aim checks: U4f |
| Fake a view time for more rewind | `rewind_at` takes only ticks the 2 s ring holds | tolerance and cap: U4h; reruns keep an old view time (SEC-U4E-8) |
| Fake latency through Pongs | sample from the Pong's arrival, hold capped at one tick; ws sessions from pings only, wt/L1 from the link only (pump.rs:298-303, :457-473) **(changed: NC-M3, A2)** | ADR 0001's accepted risk (OQ-0001-5); the cap: U4h |
| Forged acknowledgements | must name a snapshot sent to this client (room.rs:756-790); no longer frees a full state sooner than 2 s **(changed)** | none in the room |
| Never acknowledge | queue cap, one overflow resync, then `Kick(overflow)` before the timeout (room.rs:1014-1036, :1057-1063) **(TE-6's test tightened)** | none |
| Request resyncs | one in flight, and at most one answered per 60 ticks after the last full state (room.rs:716-738) **(changed: SEC-U4E-4's room half closed)** | a replacement or re-join resets the clock (SEC-U4E-R2-1); the transport's one-open-stream rule (C-U6-12) |
| Read another player's ammunition, timers, velocity or effects | own-state section by slot; control bytes encoded per client **(A1: per-client bytes, no new path)** | none |
| See enemies out of view (wallhack) | 64 m plus the area flood | line of sight and heights: slice 3 |
| Learn a hidden shooter's identity | events routed by interest | the shooter id is always sent (SEC-U4E-6) |
| Learn another player's token | no token in any message, echo or `Debug` | none |
| Report state (position, hit, health, ammunition) | no client-to-room message carries state | none |
| Crash the room with input | no panic path on network data in the changed code (blocks 2, 4-6) | SEC-U4E-9 for U4f |
| Crash or bloat a client with server bytes (hostile server) | case 3 bounded (32 ticks), lost runs never overflow, pings 8 (client/src/lib.rs) **(changed: R1, SEC-U4E-7 closed)** | NC-L7's far-ahead header (carry 80) |

### Block 10. Verdict and durable notes (done)
- **Verdict: APPROVE WITH FINDINGS**, Lows only (four new: SEC-U4E-R2-1 to -4), none before the merge. By R8 that lets
  the merge go ahead and the Lows become follow-ups. Each open security finding keeps PR #40 needs-mustafa (it already
  is).
- Durable notes for this seat (no memory folder in this repo; they live here, as in round 1): an acknowledgement is a
  client claim and must never gate server work by itself (now met for `Resync` in the room); any per-client limit kept
  in `ClientSlot` resets on a replacement or re-join, so a limit that must survive a reconnect lives in the roster or
  at the transport; the client trusts the server, but its decoder stays bounded and panic-free.
- Not checked: the plants (the test-engineer re-proves them), the counted check.mjs run (the test-engineer's), the
  netcode seat's reading of R1's cases beyond what bears on memory and panics, and npt-proto's boundary acceptance
  (U4a's).
