# U4e round 2, engine-architect seat (read only)

Started 2026-11-12. Fixed copy: /home/user/Shamble-Frontlines/.claude/worktrees/wf_0fa6595a-3e2-1 (uncommitted on 97bb9d3).
Round 1's read copy: /home/user/Shamble-Frontlines/.gate/u4e-r1. Packet: u4e-r2-packet.md.
Memory: none found (no agent-memory folder for engine-architect). I edited no copy and ran no cargo command.
This seat has no Edit tool, so this notes file is rewritten whole with Write (T4 item 1).
Paths below are relative to spikes/netcode-proof/crates/ in the fixed copy unless they say otherwise.

## Plan
1. Read R1-R8 and T1 (packet), T6 (advisor-u4e-r1.md), my round-1 notes (u4e-review-arch.md: A1, A2, A4, A5, list :57-66).
2. Check the copy matches fx-hashes-end.sha256 (sha256sum -c, read only).
3. A1: room take_control's type and body (bytes encoded once on the tick with the room's Quant; full_state's counted
   encode reused; JoinAccepted and Kick likewise); the lab's send in server/tests/common/mod.rs reads the channel from
   Limits::TYPES by the type byte and no longer encodes; the bites test and its plant.
4. A2: RttSource (pub, two variants, docs, where defined, how re-exported), Decoder::new's signature, the pump's rtt_ns
   choice; the two tests; e10's ping path passes Pings.
5. A4: the scripted-input field, fill_input, no clear after the step; tick_input()'s doc; the test.
6. A5: the eight items pub(crate); with_lead pub; with_cover's test (Rewind's cover Present).
7. T6: every pub item the fix pass adds or changes, from the diff (grep `pub ` in the + lines), against §4.1's rows and
   R8's one new item; anything beyond R8's scope and bounds.
8. Hidden coupling / boundary: who owns encoded bytes, double encode anywhere else, channel rule ownership.
9. Optional runs via the runner (prefix ea).
10. Verdict and findings.

## Notes (a line after each finding)

### Setup (checked)
- `sha256sum -c .gate/u4e/fx-hashes-end.sha256` in the copy: 23 of 23 OK. `git status`: 6 M and 17 ?? under crates/ plus
  the record, on 97bb9d3.
- `diff -rq .gate/u4e-r1/spikes/netcode-proof <copy>/spikes/netcode-proof -x target`: 11 files differ, all under
  crates/room (src/ack.rs, src/room.rs, src/roster.rs, tests/room.rs), crates/server (src/lib.rs, src/pump.rs,
  tests/common/mod.rs, tests/loopback.rs, tests/pump.rs) and crates/client (src/lib.rs, tests/endpoint.rs). No
  manifest, no Cargo.lock, no clippy.toml, no schedule.rs, no npt-proto/sim/link/lab file: R8's bounds hold.
- The builder's counted runs (.gate/u4e/fx-counted1.log, fx-counted2.log :122-132): "check: all 10 steps pass", measured
  473, bites 541, self 52 of 52; the deps step's line still holds the edges ("npt-proto's and npt-client's sources name
  only Channel from npt_link", "npt-room's sources hold no float", "no bite id is in two crates' sources"). The deltas
  (+15 measured, +17 bites) equal the new tests I count in the diff: room 4 (sec_u4e_4, a4, a5, carry_50), pump 7 (e10
  arrival, a2 x2, e9 input, e9 control, sec_u4e_1, sec_u4e_3), loopback 1 (e6), endpoint 3 (e3 TE-4, carry_61_b,
  sec_u4e_7); bites only +2 (a1, TE-3).

### A1 (round 1 Medium): CLOSED
- `take_control` now returns `Vec<Vec<u8>>` (room.rs:607); the doc says each is encoded once, when queued, with the
  room's Quant, and that the caller sends it on the channel its type byte's row of `Limits::TYPES` names (room.rs:
  603-606). `ClientSlot::control` is `Vec<Vec<u8>>` (room.rs:179-180).
- JoinAccepted (room.rs:674) and Kick(overflow) (room.rs:319) go through `ClientSlot::send_control` (room.rs:325-332),
  which encodes with `self.encoder.quant`, the room's Quant copied in at join (`Encoder::new(self.tick_hz, self.quant)`,
  room.rs:654; `with_bite` sets `self.quant`, room.rs:506-516). The full state's one encode is kept and its bytes pushed
  (room.rs:1117-1130); its refusal still counted (carry 54). No other encode of a room message is left in the libraries
  (grep `.encode(`: room.rs:329, :1119, :1331 and encoder.rs:321's test only).
- The lab encodes nothing: `for b in self.room.take_control(slot) { conn.server.send_reliable(stream_of(&b), &b, now) }`
  (server/tests/common/mod.rs:304-307). The room's tests decode with `control_of` (room/tests/room.rs:93-102).
- Test `bites::a1_take_control_hands_out_the_full_state_encoded_with_the_rooms_quant` (room/tests/room.rs:1378-1410):
  bitten bytes equal `Quant::with_bite(id)`'s encode of what they decode to; the clean twin's equal `npt_proto::encode`'s;
  the two differ. The builder records its reading of R2's "differ from npt_proto::encode's" (u4e-build.md:228-231: a
  re-encode of the decoded bitten message with `npt_proto::encode` gives the bitten bytes back, since -57 coarsens before
  packing). The reading is sound (the test's own first assertion shows the decoded message re-encodes to the same bytes
  under the bitten Quant), and the test discriminates: P-3 failed it by name (fx-p3.log:12, :45, exit 101 at :48).
  Consequences (1)-(3) of my round-1 A1 are gone: one encode, the bite reaches the full state, no caller encodes.
- Residuals: A-R2-1 and A-R2-2 below (Lows).

**A-R2-1, Low, not before merge (follow-up, with R5's A9): the channel rule needs npt-proto's framing in every
caller.** `stream_of` (server/tests/common/mod.rs:44-58) finds the type byte by trying offset 2 as Control and offset
4 as FullState, the prefix sizes of npt-proto's private `framing` (proto/src/msg.rs:220-226). It is right for every
message the room sends today only by arithmetic: a FullState's 4 B little-endian length (its type and body) is at most
`Limits::FULL_STATE` = 65,536 (limits.rs:30; `Quant::encode` refuses more, msg.rs:308-321), so its byte 2 is 0 or 1,
and row 1 is Input, a Datagram row, so the Control probe misses and the FullState probe hits. A full state of 262,144
B or more would put 4 (Resync, a Control row) in byte 2 and go out on the control stream; a byte matching no row falls
back to Control silently (`map_or(Channel::Control, ..)`). Nothing pins either. The rule is the advisor's form (R2),
and the room has no npt-link edge (room/Cargo.toml:17-21), so the builder had no better in-bounds form for the bytes.
But U4r's lab and U6's TickLoop will each copy this probe: the per-caller wire knowledge A1 removed for the encode
comes back for the channel. Route: add to R5's A9 follow-up (the next npt-proto change already gains a header-tick read
and the overhead in `Limits`): a public read of a framed message's channel in npt-proto, or `take_control` handing out
each message's `Limits::TYPES` row index beside its bytes (no new edge, no framing in the caller). Until then a lab test
pinning `stream_of` on a FullState at the 65,536 B cap would hold today's arithmetic. Cheapest proof: by reading (the
two offsets against msg.rs:220-226, the cap against limits.rs:30).

**A-R2-2, Low, not before merge (follow-up): `send_control` drops a JoinAccepted or a Kick silently on an encode
error.** room.rs:327-332: `if self.encoder.quant.encode(msg, &mut out).is_ok() { self.control.push(out); }`, no count,
no assert. Unreachable today: JoinAccepted is 80 B framed (78 B against `Limits::JOIN_ACCEPTED` 256) and Kick 4 B (2 B
against 16), every field put at its full width with no error path (msg.rs:355-373), and only the Input, Snapshot,
FullState and session types' bodies can return an error (msg.rs:335-337, :374-375). The lab used to `expect` this
encode (.gate/u4e-r1 common/mod.rs:280, "the room's control messages encode"); the library now hides it, where the
full state's refusal beside it is counted (carry 54). If a later change made it fail, a client would wait for a
JoinAccepted that never comes with no trace. Fix: a `debug_assert!` on the error (or the full state's
`count_refusal`). Cheapest proof: by reading.

### A2 (round 1 Medium): CLOSED
- `pub enum RttSource { Link, Pings }` (pump.rs:180-191), `Clone, Copy, Debug, PartialEq, Eq`, documented against
  §4.9 :352-354 ("the pump's caller learns [the kind] from the listener that accepted it"; plan text at 1e8c188 :351-356:
  "The pump knows each session's kind from the listener that accepted it"). Defined in the private module and
  re-exported at the crate root (server/src/lib.rs:14), W1-PUB's one-path rule kept.
- `Decoder::new(.., rtt: RttSource)` (pump.rs:224-232), held in the decoder (pump.rs:210, :238); `rtt_ns` matches on it,
  `Link => link.rtt_ns()`, `Pings => min of samples` (pump.rs:295-303): one session never mixes the two.
- Tests: `a2_a_pings_session_takes_its_ping_sample_never_the_links_rtt` (server/tests/pump.rs:336-344: Loopback answers
  Some(0), one sample 100 ms, gives 100 ms), `a2_a_link_session_takes_the_links_rtt_never_a_ping_sample` (pump.rs:346-355:
  NoRtt with a sample held gives None; a link with an estimate, its own). e10 passes `RttSource::Pings` (tests/pump.rs:
  263). The lab passes `RttSource::Link` for its Loopback and L1 sessions (common/mod.rs:206-213, :243-250), as §4.9 names
  them. P-4 failed both by name (fx-p4.log:31-32, :91, exit 101 at :96). R2's note on `pump_report` (no RTT) is the
  report's, not code.

### A4 (round 1 Medium): CLOSED
- Three new private fields hold the next tick's script (room.rs:413-416); `script()` writes only them (room.rs:617-629);
  `fill_input` clears `input`'s three lists and moves the script in with `append` (room.rs:831-838); the clears after the
  step are gone (room.rs:813-818). Every use of `self.input` (room.rs:291, :580, :814, :832-877) is the room's own; no
  other path writes the scripted lists between ticks. `tick()` has no early return before `fill_input` (room.rs:805-826),
  so a script never slips to a later tick. `append` keeps the scripted Vecs' capacity: no allocation per tick after
  warm-up.
- `tick_input()`'s doc now says "whole ... and the tick's scripted spawns, teleports and attribute changes" (room.rs:
  577-581). Test `a4_tick_input_after_a_tick_is_that_ticks_whole_input` (room/tests/room.rs:467-485): the teleport is in
  that tick's input, none in the next. P-5 failed it by name (fx-p5.log:12, :45, exit 101 at :48).

### A5 (round 1 Medium): CLOSED
- pub(crate): `Roster::new` (roster.rs:59), `Roster::is_bound` (roster.rs:85-86, `cfg_attr(not(test), allow(dead_code))`),
  `Decoder::limits` (pump.rs:253-260), `Decoder::is_joined` (pump.rs:267-274), `ConnLimits::join_deadline_ns` (pump.rs:
  100-107), `ClientEndpoint::closed` (client lib.rs:326-333), `ClientEndpoint::unknown_pongs` (client lib.rs:368-375),
  `ClientSlot::skipped` (room.rs:268-275): the eight R2 names, each with `allow(dead_code, reason = ..)` where no crate
  consumer is left. `with_lead` stays pub (client lib.rs:291). Both builds and wasm32 lint and doc clean in the counted
  runs (fx-counted2.log:123-125), so no caller outside a crate named one of the eight.
- `Room::with_cover` keeps pub (room.rs:500-504) and has its test: `a5_a_room_with_cover_present_hands_the_sim_each_
  rewind_with_cover_present` (room/tests/room.rs:488-566; the wrapper `Covers` at :489), through a `SimCore` wrapper that
  records each Rewind's cover (Present under `with_cover(Present)`, Rewound at the default). P-6 failed it by name
  (fx-p6.log:12, :39, exit 101). The test uses the seam as a trait, which is the seam doing its job (no room internals
  reached).

### T6's §4.1 rows against the final diff: they match (no finding)
Base: 28a85e7's three crates had only npt-client `BITES`, `grid_with_bite` and npt-server's schedule items (`git show
28a85e7:...`, grep of pub items). Final pub items (grep of `pub fn|struct|enum|const|use` over room/src, server/src,
client/src) against advisor-u4e-r1.md:681-712:
- npt-room: §4.7's eleven (room/src/lib.rs:23-30); `ClientCounters`; `InputBuffer::CMD_CATCHUP` and `buffer_depth_ticks`
  (input.rs:61, :64); Room's 20 methods (room.rs:431-805: new, with_event_cap, with_event_start, with_hashes, with_cover,
  with_bite [bites], event_cap, sim, roster, last_tick, tick_input, reports, client, take_datagrams, take_control,
  script, join, leave, pump_report, receive, tick); ClientSlot's 17 getters (room.rs:195-295); `Roster::live`, `is_live`
  (roster.rs:75, :80); bites `BITES` (lib.rs:38). Each is in T6's new row; none else.
- npt-server: `Decoder` with new, counters, is_closed, accept, reject, take_sent_bytes, rtt_ns (pump.rs:224-298) and
  pump (pump.rs:331, in the row's kept start: u4-build-plan.md:195); `RttSource` (pump.rs:183); `ConnLimits::new`,
  `input_refill_per_second`, `hold_cap_ns` (pump.rs:80-96); `Counters` (pump.rs:112); modules private (lib.rs:11-17).
  Each in T6's row.
- npt-client: `StepOut`'s six fields (client lib.rs:189-202); `ClientEndpoint`'s new, with_lead, with_starts, with_bite
  [bites], slot, newest_tick, last_run_command, wrong_channel, refused, stale, duplicates, input, step (client lib.rs:
  253-415); bites `BITES` (3 ids, lib.rs:28) and `grid_with_bite` (lib.rs:38). Each in T6's row.
- What the fix pass added to the public surface: `RttSource` (R8's one new item), `take_control`'s changed type (R8's),
  and two new pub fields of pub counter types, `ClientCounters::resyncs_limited` (room.rs:108-110) and
  `Counters::joining_dropped` (pump.rs:145-146). Both names are the rulings' own (R3: "counted under its own name
  (`resyncs_limited`)"; R5: SEC-U4E-3 "counted under its own name"), the builder flagged both (u4e-build.md:252-253,
  :298-299), and T6's rows cover them ("its fields the per-client counts"; "`Counters`' fields"). Not beyond R8's bounds
  in my reading; the advisor may say so in one line when it rules.
- Nothing else is new in the libraries: the test lab's `pub fn stream_of` is a test helper (tests/common), not a library
  item.

### Scope (checked): nothing beyond R8
Every hunk of the 11 files maps to an FX: room.rs (A1, A4, A5, SEC-U4E-4, RR-2, RR-10), ack.rs and client lib.rs's
const asserts (RR-3), roster.rs (A5), server lib.rs (RttSource's export), pump.rs (NC-M3, A2, A5, SEC-U4E-1, SEC-U4E-3),
client lib.rs (R1's case 3, SEC-U4E-7, A5), the tests (FX-1 to FX-13; e9's Resync test re-pinned for FX-7, named in the
builder's plan, u4e-build.md:152-154). The window's arithmetic (room.rs's snapshot budget) has no hunk. No pub item but
those above.

### Boundaries and ownership (checked, no further finding)
- Edges unchanged (deps step PASS; no manifest change). npt-room still names no npt-link type: its tests read the
  channel as a `Limits::TYPES` row value (room/tests/room.rs:93-102).
- The room owns the reliable outbox as bytes until `take_control` drains it (`mem::take`, room.rs:607-613); one Vec per
  message, as before (the typed Message held its own Vecs), and one encode fewer per full state.
- The session's RTT source is the caller's knowledge passed once at construction, never read from a run parameter by
  the room (§4.9's "so the room never reads a run parameter").
- SEC-U4E-4's state (`last_full_state`, room.rs:173-174) lives in the ClientSlot beside `in_flight`; the room alone
  writes it (full_state, room.rs:1125).

## Runs
None. Each finding above is settled by reading; no run would add to it. The test-engineer re-proves each FX's plant.

## Verdict
APPROVE WITH FINDINGS. My four round-1 fix-pass items (A1, A2, A4, A5) are closed, and T6's three rows match the final
diff. Two Lows (A-R2-1, A-R2-2), neither before the merge: follow-ups.
