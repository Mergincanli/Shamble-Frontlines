# U4e round 1: netcode-architect review notes

Reviewer: netcode-architect (read only; log prefix nc). Started 2026-11-12.
Read copy: /home/user/Shamble-Frontlines/.gate/u4e-r1 (97bb9d3 + U4e snapshot patch). Builder's copy (uncommitted):
/home/user/Shamble-Frontlines/.claude/worktrees/wf_0fa6595a-3e2-1. Base: /home/user/Shamble-Frontlines/.gate/u4e-prefix (1e8c188).
Paths below are under spikes/netcode-proof/crates/ of the read copy unless stated. I ran nothing (runs are optional for
me); every number quoted from a run is from the builder's logs under .gate/u4e/ (named per line).

## Plan
1. Read the packet's "judged against" half (§5 U4e, §4.1, §4.2, §4.5-4.7, §4.9-4.11, §4.14, W1-*, advisor-u4e-brief,
   B2.1, B5, Mustafa's words, carries 21-27, 39, 40, 50, 51, 53-57, 61, 62, 66, 68, the left-out ids).
2. Read the source in the read copy (not only the packet diff): room (tick, InputBuffer, EventQueue, AckRing, Encoder,
   InterestGrid, AreaFlood, Priority, rewind_at, Roster), server lib (Decoder::pump, ConnLimits, Counters), client
   (ClientEndpoint), sim Scene::effect_rows.
3. Focus checks, one block each, a line per finding:
   a. room tick: InputBuffer option B + catch-up bound; carry 21 order; rewind_at integer half (carry 22, U4E-Q5);
      events stamped once into one list (23, 26); EventQueue cap as data, overflow resync + kick, age bound (U4E-Q6;
      50, 57); one Resync in flight.
   b. baselines/window (§4.5): AckRing ages, newest fully acked tick, partly acked never a baseline, window remainder
      after pump bytes at 30/60 Hz, groups and the split, encoder refusal skips that client's snapshot (54).
   c. own-state section (W1-S1; 51, 53, 55, 56, 68): own rows only via Scene::effect_rows; 56's measurement; 68's
      condition on ES byte figures.
   d. client (§4.6; 61; B2.1): complete-tick rule, sentinel causes, when lost, stale group decode/drop (B5 reading),
      acks + group mask, -58 rebuilds across wraps (W1-N2), the 1,150 B refusal.
   e. pump (§4.5 RTT; §4.9): pings, Pongs hold capped at one tick, RTT reported, bytes counted.
   f. -18 end to end (cond. 6): baseline held or not under the bite; gap alignments; 500 ms window (U4E-Q14).
   g. e2 forced split, e7 mirror over Loopback at P0 for 1,000 ticks: test what plan/brief ask, every clause.
   h. later judged runs (U4r, U4f) INVALID risks / marks passing too easily; provisional values named.
   i. scope: files/symbols outside the plan and rulings.
4. Verdict + ranked findings (severity, file:line, scenario, cheapest proof, before-merge).

## Blocks

### Block a. The room's tick (done)
Met as the plan and rulings say:
- Option B: `InputBuffer::take` (room/src/input.rs:129-151) runs the command stamped for the tick, else the last run
  again (`repeated`), drops buffered ones stamped earlier as late; one command per slot per tick. Catch-up bound: at most
  `CMD_CATCHUP` (= `Limits::REDUNDANCY_CAP`, const assert :155) buffered, the rest `Full` (:114-116); `buffer_depth_ticks`
  exposed (:63-66). Commands arrive newest first and are offered oldest first (room.rs:752; client lib.rs:381, :403).
- Carry 21: `canonical` (room.rs:344-350), slot ascending, tick wrap-aware within a slot, as seam.rs:116-121; tested
  in-crate across the 32-bit wrap (room.rs:1347-1356) and over two arrival orders with doubled datagrams
  (room/tests/room.rs:223-267).
- Carries 22 and U4E-Q5: `rewind_at` (room/src/rewind.rs:18-33) refuses a view time whose `tick_a`, or `tick_a + 1` when
  `fraction > 0`, the history lacks; the room counts it and keeps each such seq (room.rs:817-822; ClientSlot
  `rewinds_refused`, the carry 40 counter). Edges tested (room/tests/room.rs:278-345).
- Carries 23 and 26: the step's events are copied once into `Room::stamped` with (32-bit number, `TickInput::tick`)
  before `hit_volumes`, `reports` or `entity_records` are read (room.rs:772-786); every client reads that one list
  (room.rs:947). No per-tick seam workaround.
- EventQueue (room/src/events.rs): cap as data (`with_event_cap`, `PROVISIONAL_CAP` 1,024, `event_cap()` returns the
  provisional mark: room.rs:437-438, :455-459, :532-534); overflow forces one resync, a second overflow inside an
  unacknowledged forced one, or no ack within `FULL_STATE_ACK_TICKS`, kicks (room.rs:973-994, :1015-1021); age bound:
  `expire` drops at age 256 and counts (events.rs:87-92, room.rs:943) before the snapshot codes age as u8
  (room.rs:1170). Carry 57's per-client count and per-tick flag exist (room.rs:990-991).
- One Resync in flight: refused while a full state is due or unacknowledged, counted (room.rs:685-698).
- NC-L5 (Low) rewind_at's debug assert (rewind.rs:28-31) re-checks the condition the line above already returned on, so
  it can never fire; §4.2's "the debug assert that a correct room never sends one" is met by construction only.
  Follow-up wording; no defect.

### Block b. Baselines, the window, groups (done)
Met:
- AckRing (room/src/ack.rs): `npt_sim::ACK_RING_TICKS` by path; baseline = the newest complete entry at age 1-31
  (:174-186); a partly acknowledged tick never a baseline (`complete()`: :43-45; e4 :219-244); `last_acked` the newest
  fully acked, held or evicted (:149-151, :189-191); `evicted(now)` at age >= 32 (:195-198); acks of a never-sent tick
  inside the span refuse the datagram's commands, older ones count `acks_old` (:154-164, room.rs:734-750).
- Window: `tick_budget` carries the fractional bytes so a second sums to 32,000 B at 30 and 60 Hz (encoder.rs:26-31,
  pinned :239-251); the pump's bytes come off that tick's budget whether or not a snapshot goes out (room.rs:1010-1013;
  e6 server/tests/loopback.rs:161-199, log b11: "e6: 392 snapshots, 48 on ticks with pump bytes, e.g. (38, 1067, 1029,
  147)").
- Groups: own section in group 0 only, events and entities fill the last group or open one, at most `Limits::GROUPS`,
  each within min(link max, 1,000 B) and the remainder at payload + 28 B each (encoder.rs:106-157); every group carries
  the header with one baseline; the size arithmetic equals the encoding (debug assert room.rs:1277-1281, test
  encoder.rs:254-327).
- Carry 54 and the advisor's A4: any `EncodeError` skips that client's tick, counted by kind, no server hash
  (room.rs:1288-1291, :1089-1096); tested with a planted out-of-range OwnState field (room/tests/room.rs:752-830).
- NC-L6 (Low) the skipped snapshot has already moved state: entities placed were marked sent in the priority
  accumulator (room.rs:1229-1231) and each queued event's `sent` count rose (:1174), so after a carry-54 skip the
  accumulator under-counts skips and the next carriage of those events counts as `events_resent`. Hash and mirror are
  unaffected (both come from the baseline). Follow-up.
- NC-M4 (Medium) gameplay events are packed before every entity change (room.rs:1164-1180, then :1181-1237), so after
  an event backlog the nearest 8 enemies are not "in every snapshot" (catalogue :129-130; ADR 0001 part 9 (d); brief
  2.1's Priority bullet). Scenario: the gap test resends 439-462 events (log: "2.5 AfterSend ... 462 events resent",
  "2.5 BeforeSend ... 439"); at about 74 bits each that is about 4 KB, so the first 3-4 snapshots after a burst carry
  events and no entity change at all; F after the gap is coded from the defaults, so the client holds no remote soldier
  for those ticks. No U4e test asserts the nearest-8 rule. Cheapest proof: in gap.rs's clean run, read client 0's
  snapshot entity counts (or `ClientSlot::skipped`) for F..F+4: entities 0 while events fill each group. Fix: place the
  nearest-8 changes before the events (then the backlog), or take it to the advisor as a packing rule. Not before merge
  (no U4r mark judges it; a follow-up with the advisor).

### Block c. The own-state section (done)
Met:
- W1-S1 and carry 53: each client's own section is built from its own slot alone: `(Soldier, slot)`, effect rows
  `slot * effect_rows .. (slot + 1) * effect_rows`, `(OwnState, slot)` (room.rs:1022-1040), in snapshots and full
  states (room.rs:1041-1048, :1055-1087); the owner-only kinds never enter the entity section (find_relevant skips
  the own soldier, room.rs:885-887; Effect and OwnState have no interest point, room.rs:326-332).
- Carry 51: the rows per soldier are read once from `Scene::effect_rows()` (room.rs:430), never a literal 8; L3's
  accessor (sim/src/scene.rs:324-326) is the one public home, as Mustafa deferred it ("defer it to U4e in writing").
- Carry 53's test: 48 clients, 1,287 ticks, a generation change asserted (log: "carry 53: SS seed 1, 1287 ticks from
  tick 200, 48 clients, 1 generation changes"), every Snapshot and FullState read per record, a missing snapshot fails
  (room/tests/room.rs:425-535, :647-673). The owner-to-all bite writes the next slot's records in place of the client's
  own (room.rs:1023-1027): within the counts, so carry 54 does not skip it; its twin fails on client 0
  (room/tests/room.rs:1113-1122).
- Carry 55: no Team or Public row added (crates/proto untouched).
- Carry 56: measured (log: SS 20.58 B mean, 32 B p99, 121 B max per snapshot, OwnState 9.13 B mean, changed in 0.9995;
  ES 15.55 B mean, 21 B p99, 25 B max, OwnState 8.27 B, 1.0000). The SS mean is above R2's 16 B: the report must put it
  beside that figure, not judged (carry 56's words).
- NC-M2 (Medium) carry 68 not met: the ES figure comes from `Recording::synthetic`'s ES recording (room/tests/room.rs:
  676-700 through `recorded`, :62-71), and neither the test's line nor the builder's record states the yaw and pitch
  shares (about 48 % and 18 % of soldier-ticks) beside R2's (yaw in every moving update, pitch half the time:
  r2-netcode.md:1751). The own section holds the client's own Soldier record, whose yaw (16 bits) and pitch (10 bits)
  are those shares, so carry 68's sentence applies as worded ("Any ES byte figure U4e reports from that recording
  states those two shares beside R2's"). Cheapest fix: the ES test prints the own soldier's yaw-change and pitch-change
  shares over its 300 ticks and the report sets them beside R2's; or the report drops the ES figure. Before merge (a
  report line).

### Block d. The client (done)
Met:
- Complete-tick rule (client/src/lib.rs:161-165, :702-710); the sentinel for no baseline, a header mismatch between
  groups, a decode or size error, read from the header's 32-bit tick (:446-462, :642-669, :684-689); a lost group files
  nothing and the tick is resolved lost only when it leaves the ring by the newest header tick, a tick no group named
  included (:489-520); carry 61 (b) duplicates by byte signature (:631-641), a second different hash gives the sentinel
  (:548-561); carry 61 (d): a group of an older tick still in the ring is decoded and hashed under its tick (the
  advisor's lean; test client/tests/endpoint.rs:157-203).
- Acks: the newest tick a group of which decoded, with its group mask (lib.rs:563-570, :398); FullState acked as group
  0 (:742).
- W1-N2 and -58: newest-received and last-run rebuilt to the client's 32-bit command counter by `Wrap::widen16`, the
  last-run wrap exposed as its own counter (lib.rs:574-604); event numbers rebuilt and their wrap exposed
  (:607-616); tested across the wraps (endpoint.rs:232-281).
- The 1,150 B refusal: `decode`'s TooLarge on `SNAPSHOT_RECEIVE` gives the sentinel (endpoint.rs:100-115).
- NC-H1 (High) see "Block h" (tick16): the client's stale drop makes -17/tick16 and -58/tick16 unable to fail the check
  §4.14 names.
- NC-L4 (Low) B5's reading: the client decodes stale groups inside its ring; it drops only groups whose tick is at least
  `ACK_RING_TICKS` behind its newest header tick (lib.rs:466-471, :622-625, counted as `stale()`), ticks already
  resolved when they left the ring. So the build's answer to B2.1 is "decode", B5's extra reading is not needed, and
  carry 62 (g)'s drop count does not apply; the report must say so in those words, and that `stale()` has no SERIES.md
  row (it is not carry 61 (d)'s drop). Before merge (a report line).
- NC-L7 (Low) the sentinel path calls `see(t)` with a tick read from bytes the decoder refused (lib.rs:451-461, :538):
  one datagram whose bytes 1..5 read far ahead moves `newest` there, resolves the whole span lost in one entry
  (:516-519) and drops every later real snapshot as stale. L1 neither corrupts nor duplicates, so no judged run reaches
  it; a hostile or corrupted datagram stops the client. Follow-up (file the sentinel without moving `newest` past a
  bound).
- NC-L8 (Low) `step` drops the whole `Input` when it fails to encode (lib.rs:399-409), acks included, with no counter;
  one command the codec refuses stays in `unacked` for up to 8 steps and blocks the acks that long. The view-time case
  is guarded (:375-380); others (an aim or move outside its field) are not. Follow-up (count it, drop the bad command).

### Block e. The pump (done)
Met: pings every 250 ms on the injected clock (pump.rs:15-16, :312-329), counted with their bytes (payload + 28 B:
:278-286); the room's Pong to a client Ping carries the hold since arrival (:374-388); the client's Pong hold counted
up to one tick period (`hold_cap_ns` = 10^9 / tick_hz, pinned at 30 and 60 Hz: server/tests/pump.rs:257-258); RTT =
the link's for L1 and wt, else the lowest of the last 8 samples (pump.rs:267-269); Pongs naming no outstanding ping
dropped and counted (:413-421); the Input bucket and Ping limit exact in integer ns (:30-65).
- NC-M3 (Medium) the RTT sample uses the pump's `now_ns`, not the Pong's arrival (pump.rs:389 passes `now_ns`;
  :424 `now_ns - echo - hold`), while the Ping answer beside it uses `r.arrival_ns` (:376). When the pump runs once per
  tick (the lab does: server/tests/common/mod.rs:229-262), a Pong that arrived up to one tick period before the pump
  counts that wait as RTT: a ws session's RTT (the min of 8 samples, §4.9 :351-353) reads up to 33 ms high at 30 Hz,
  which feeds -50's WebSocket gap check and U4h's tolerance. The e10 test pumps exactly at the arrival instant
  (pump.rs:272-286), so it cannot see it. Cheapest proof: in that test send the Pong at `at` and pump at `at +
  20_000_000`: the RTT becomes 120 ms - hold, not 100 ms - hold. Fix: pass `r.arrival_ns` to `pong`. Before merge.

### Block f. -18 end to end (done)
Met, as U4E-Q14 rules: the gap is `ACK_RING_TICKS` tick periods computed in the test from the constant and the rate
(server/tests/gap.rs:54-57), at two alignments, 1 ns after and 1 ns before the send of T, the clients stepping 5 ms
after each tick (gap.rs:14-24, :74-81); e5a asserts B's age >= 32 before `burst_eviction` (gap.rs:130-149; log: "e5a
AfterSend: T 330 ... F 363, B Some(329) at age Some(34), evicted true", "e5a BeforeSend: ... F 362, B Some(329) at age
Some(33)"); the 1 s gap reported, not asserted (log: age 31 at both, evicted false, as the advisor predicted); e5b
asserts the defaults at F (gap.rs:151-165); 2.5 measures from the gap's end to the send of the first tick the client
decoded with the server's hash, within 500 ms (gap.rs:167-205; log: 33 ms and 0 ms); the bitten twins assert F coded
at ages 34 and 33 and no fresh delta, with 29 and 30 sentinels (gap.rs:207-239; log "2.5 bitten ..."); e11 runs a
bitten and a clean room on two threads (gap.rs:241-262).
- Condition 6, my reading: under the bite the baseline is one the client no longer holds by its ring's rule: the
  client refuses any `baseline_age >= ACK_RING_TICKS` (client/src/lib.rs:655-663), the same ages as the AckRing (§4.5:
  "The client keeps its complete frames for the same ages"). So the bite is a defect on L1, and condition 6's
  escalation is not triggered on that reading.
- NC-L1 (Low) but the frame itself is still in memory: B = 329 sits in ring slot 329 % 32 = 9, and F = 363 or 362 uses
  slot 11 or 10, so no tick reused slot 9 (F - B = 34 or 33, not a multiple of 32). The refusal rests on the age test at
  lib.rs:658 alone; a client that looked frames up by tick would decode the bitten delta. The report must state this
  (B, F, the ages, the slot still holding B) so the advisor can confirm the reading in one line before -18's L1 part
  counts at U4r. Before merge (a report line).
- NC-L2 (Low) under the bite, B at age exactly 32 is not bitten: `baseline()` excludes age 32 (ack.rs:178), and the
  stale copy is taken only when B's slot is overwritten, by the record of tick B + 32 = F itself, after F is coded
  (ack.rs:102-107; room.rs:1121-1125, :1314). So the bitten room codes F from the defaults, F decodes and becomes the
  baseline, and the bite heals, while `burst_eviction` says 1 (age >= 32: ack.rs:195-198), "could bite" overstated at
  that edge. The gap test never meets it (33 and 34). Cheapest proof: gap.rs with a gap one tick period shorter at the
  AfterSend alignment (B's age at F = 33 - 1 = 32 if the arithmetic holds; assert it first), bitten: F's
  `baseline_age` 0. Fix: under the bite take the stale copy as soon as `last_acked` reaches age 32 (or code against the
  still-present entry). Follow-up unless the fix pass is in the room anyway.

### Block g. e2 and e7 (done)
- e7 (server/tests/loopback.rs:43-100; log b11-e7print: "48432 pairs compared, 0 differ or are the sentinel; per client
  at least 1009; split ticks 0, skipped 17184, events carried 562794, resent 0, non-default baselines 48384"): 48
  clients of SS over 1,010 ticks on zero-delay Loopback; asserts >= 1,000 filed per client, 0 bad, skipped >= 1, events
  carried >= 1, a non-default baseline. The split clause goes to e2 as the advisor's A2 allows (e2 log: "3024 split
  ticks, 3023 compared equal").
- NC-L3 (Low) e7's "and at least one resent" clause (brief 2.4 e7) is not asserted in e7 (resent 0 on zero-delay
  Loopback: an ack always lands before the next tick) and was moved to gap.rs's end-to-end test (gap.rs:202-203; 236
  pairs, 0 differ, 439-462 resent), with no ruling: A2 covered only the split clause. The move is sound (the mirror is
  compared over resent events there), but the report must say which test carries it, for the advisor. Before merge (a
  report line).
- e2 (loopback.rs:102-156): 300 B links force splits; the mirror equals the client on 3,023 split ticks; group 1 of
  one split tick dropped leaves the tick unfiled and resolved lost. Groups decode "alone" is shown for group 0 without
  group 1; no test drops group 0 (the one carrying the own section) or delivers a split tick's groups out of order. The
  code handles both (lib.rs:670-673 bases every group on the baseline; own only in group 0). A test-engineer item, not
  a finding of mine.

### Block h. Later judged runs (done)
- NC-H1 (High) -17/tick16 and -58/tick16 cannot fail the check §4.14 names. Under the bite the header carries
  `tick & 0xffff` (proto/src/quant.rs:133-139), so after 65,535 the client reads 0, 1, 2 ... Each is "not after" its
  newest (65,535) and at least `ACK_RING_TICKS` behind it, so `too_old` drops it as stale (client/src/lib.rs:466-471,
  :622-625) without `see()`, a hash, the sentinel or a lost record. §4.14's "check it fails" is "decoded hash:
  post-wrap frames logged under the wrong tick (an orphan pair) or as the sentinel"; SERIES.md:266 counts "a server hash
  with no client hash" as loss, not compared. So in U4r's judged bite runs the mismatch figure stays 0: the client logs
  nothing after the wrap, which is either a coverage gap (INVALID, never caught) or, if the writer resolves the ticks
  lost at the stop, 0 mismatches over the pre-wrap ticks (the bite missed). The unit proof (server/tests/
  loopback.rs:241-252) asserts only "server hash present, client hash absent" at 65,537 (log s03/s04: "Some(...) None";
  the builder's record: "14 post-wrap groups dropped as stale"), which is exactly the loss case: evidence that does not
  hold for the check. Carry 61 (d)'s drop permission and B5's reading make it worse: a dropped stale group is "lost".
  Cheapest proof: in the bites build, in `pm_0001_17_and_58_tick16_post_wrap_frames_have_no_equal_client_hash`, also
  print `lab.compare()` and slot 0's client keys: expect (n, 0) and no key >= 65,536 and none at 0..30, so the judged
  figure would be 0. Fix (my lean; the plan reading is the advisor's since it touches carry 61 (d)): a header tick at
  least the ring behind the newest names a tick already resolved, so treat it as §4.6's "bad header" and file the
  sentinel under the tick read (an orphan the figure counts), keeping "decode" for stale groups inside the ring; the
  unit proof then asserts what the judge counts (a client hash at a tick the server never logged, or the sentinel).
  Before merge.
- NC-M5 (Medium) the client's event-number rebuild has no base on the wire: `event_near` starts at 0 or at
  `with_starts`' test value (client/src/lib.rs:259, :282-286) and moves only with events seen (:607-616); JoinAccepted
  carries the tick but no event number, and FullState has no event section. A client that joins after the room's
  counter has moved 32,768 or more from its guess, or sees no event for that long (about 30 s of SS's events), rebuilds
  every event number wrong, and the event numbers are hashed (§4.6), so a correct build mismatches: -19's join hash
  (C-U6-11) is the first judged case; -58/lt-event-seq's L1 runs work only if U4r's lab passes the room's event start
  to every endpoint. Cheapest proof: endpoint.rs's `event_case` with `with_starts(0, 0)` against events numbered from
  200,000: the hash differs. Not before merge; carries: U4r passes the room's event start to each endpoint (and
  `cmd` start), and the join's event base goes to the advisor (a wire field, or the hash's event number defined
  without the high half).
- NC-M6 (Medium) `ClientEndpoint` stamps each command `newest + lead` with lead 1 by default (client/src/lib.rs:246,
  :368). On any link whose one-way delay is above about half a tick, every command reaches the room after its tick ran
  and is dropped as late; no command ever runs, so `last_run` stays None and option B has nothing to repeat
  (input.rs:135-141). U4r (which depends on U4e, before U4f's controller) runs -17 at P2, P3 and B1 and -18 at B1 with
  this endpoint: with the default lead the soldiers stand still and fire nothing, and -17 compares a frozen world (a
  mark passing too easily; -12's bytes too). Cheapest proof: the Lab with a delaying link (or the lab's offsets above
  one tick period) shows `late` equal to the commands sent and `commands_run` 0. Not before merge; a carry for U4r: set
  `with_lead` from the profile's RTT and jitter, and its contract test asserts late commands near 0 and
  `commands_run` near one per client per tick.
- NC-L9 (Low) for U4r: -58's wrap check counts `event.counter_wrap` records ("count >= 6": marks.json -58), and the
  room and the client expose wraps per client slot (room.rs:251-254; client StepOut.wraps), so a writer that logs per
  client lets one counter's 48 per-client wraps pass it. The writer must log one record per counter name (SERIES.md:150
  "per counter that wrapped"). Carry note.
- NC-L10 (Low) for U4r: the room sends no snapshot while a full state is unacknowledged (room.rs:1045-1047), so at
  each join the ticks between the full state and its ack are unsent and the client resolves them lost; carry 62 (h)'s
  proposed setup check (snap_incomplete sum 0 on L1 P0, window "all") will read above 0 from the warm-up joins. Its
  cause is benign and carry 62 (h) already routes a non-zero sum to the advisor; the report should name this cause.
- Provisional values: the event cap is data with its provisional mark exposed (met, U4E-Q6). `HISTORY_MS` 2 s is named
  provisional until U4h (room.rs:32-33; the plan leaves the length to U4h). The ping interval (250 ms) and the interest
  radius (64 m) and cell are values the plan does not call provisional.
- NC-L11 (Low) the radius is called "provisional data" (room.rs:28) but is a module const with no setter or accessor,
  so no run header can record it, and ADR 0001 part 9 (a) holds the region "as data per mode"; plan silent on the
  value. Also Hit events go to a client that sees only the victim with the shooter's id (room.rs:954-968), where ADR
  0001 part 9 (f) and catalogue :141 say "no shooter id when the shooter is not relevant"; the wire has no form for it
  (U4a's), so it is a slice-3 follow-up, not U4e's. Follow-ups.
- NC-L12 (Low) SERIES.md:83's `second.v.own_cosmetic_sent_count` names npt-room as its writer; the room counts all
  cosmetic events sent (`cosmetic_sent`), not the client's own, so -62's judged run needs a count the room does not
  expose yet (the server-echo unit proof counts own shots from decoded snapshots in the test). Follow-up for the -62
  run's unit.

### Block i. Scope (done)
- Files: the builder's copy changes only crates/room (src, tests, clippy.toml), crates/server (src/lib.rs, src/pump.rs,
  tests), crates/client (src/lib.rs, tests, clippy.toml) and L3's scene.rs accessor (git status in the builder's copy).
  No manifest, proto, sim (but L3), lab, link or l1 file. schedule.rs untouched; its module made private with the
  re-exports kept (server/src/lib.rs:11-17), as W1-PUB's nit asks.
- Public items beyond §4.1's rows, for engine-architect: `npt_room::ClientCounters` (a pub struct with pub fields,
  room.rs:63-133) is not in plan.md:596-598's list; the rest are methods of listed types or the expected bites-only
  constructors. Not a netcode finding.
- clippy bans: HashMap and HashSet in room's and client's clippy.toml (U4E-Q8); `#![deny(clippy::float_arithmetic)]`
  (room/src/lib.rs:12; carry 25).

## Findings, ranked
1. NC-H1 High: tick16's judged check cannot fail (Block h). Before merge.
2. NC-M3 Medium: the ws RTT sample counts the pump's wait (Block e). Before merge.
3. NC-M2 Medium: carry 68 not met (Block c). Before merge (report line or print).
4. NC-M5 Medium: no event-number base on the wire (Block h). Carry, not before merge.
5. NC-M6 Medium: default lead 1 makes every command late above P0 (Block h). Carry for U4r, not before merge.
6. NC-M4 Medium: events packed before the nearest 8 enemies (Block b). Follow-up with the advisor.
7. NC-L1 Low: condition 6's answer rests on the age rule; report it (Block f). Before merge (report).
8. NC-L3 Low: e7's resent clause moved to gap.rs without a ruling; report it (Block g). Before merge (report).
9. NC-L4 Low: say "decode" for B2.1; `stale()` is not carry 61 (d)'s drop (Block d). Before merge (report).
10. NC-L2 Low: no-fallback does not bite at age exactly 32 (Block f). Follow-up.
11. NC-L5 .. L12 Low: follow-ups and carry notes (Blocks a, b, d, h).

## Verdict
REVISE: one High (NC-H1) and two Mediums with one-line fixes (NC-M3, NC-M2) before merge, plus three report lines.
Everything else in my focus meets the plan, the rulings and the carries as worded.
