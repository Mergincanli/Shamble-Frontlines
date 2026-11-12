# U4e round 2, test-engineer seat (on what changed: FX-1 to FX-13)

Seat: test-engineer (role file read from origin/main). Copy: .claude/worktrees/wf_0fa6595a-3e2-1 (uncommitted on
97bb9d3). Log prefix: te (.gate/u4e/r2-te-*.log). Judged against R1-R8 and T1 (u4e-r2-packet.md :1761-2160).

## Plan
1. Start hashes: the 23 changed/new files under crates/ hashed, saved as .gate/u4e/r2-hashes-start.sha256, checked
   equal to fx-hashes-end.sha256 (`sha256sum -c` from the copy's root).
2. Read the builder's record, "Fix pass, round 1": each FX's test name and plant; read each FX's diff in the packet and
   in the copy; judge each test against R1-R6's form (what it must assert).
3. For each FX (FX-1 (a) and (b), FX-2 to FX-13's tests), plant by Edit (old/new text recorded below), run through the
   runner with `-x` and `--no-fail-fast`, read the test runner's summary line, restore by Edit, check the file's sha256
   against step 1's. My own form where I doubt the builder's.
4. Round-1 test findings in the fix pass (TE-2 to TE-6, TE-7 and TE-8 riders): each has a test a plant makes fail.
5. Settle by run what other round-2 seats' findings a run can settle (u4e-review-r2-{netcode,security,arch,rust}.md).
6. One counted check.mjs (`-x`), every step's line, the counts as deltas on 458 / 524 / 52 and on 473 / 541 / 52.
7. End: `sha256sum -c` of r2-hashes-start.sha256 from the copy's root; verdict.

## Step 1: start hashes
- 23 files hashed (sha256sum from the copy's root), saved as .gate/u4e/r2-hashes-start.sha256 with Write.
  `cmp` with fx-hashes-end.sha256: identical; `sha256sum -c fx-hashes-end.sha256`: 23 OK, exit 0. git status: 6 modified,
  17 new under crates/ (plus the record u4e-build.md, outside crates/).

## Findings and plant log
Each plant: Edit in place (old / new text below), the runner with `-x` and `--no-fail-fast`, restored by the reverse
Edit, then `sha256sum -c --quiet r2-hashes-start.sha256` from the copy's root (23 OK each time unless said).

- **FX-1 plant (a)** (R1's (a): case 3 put back to case 2's silent drop; the builder's P-1a form), client/src/lib.rs
  `old_header`: old `        if self` / `            .first` / `            .is_some_and(|f| Wrap::default().after32("tick",
  t, f))` / `        {` / `            self.refused += u64::from(count);` / `            self.bad_header(t);` / `        }
  else {` / `            self.stale += 1;` / `        }` / `        true`; new `        let _ = count;` / `        self.stale
  += 1;` / `        true`. Run r2-te-p1a.log: `cargo test --locked --all-features --no-fail-fast -p npt-server -p
  npt-client` (every server and client binary, bites build), exit 101. Only loopback fails: "test result: FAILED. 6
  passed; 1 failed", `bites::pm_0001_17_and_58_tick16_post_wrap_frames_are_filed_as_orphan_sentinels ... FAILED`
  (loopback.rs:328:13 "PM-0001-17/tick16: the figure fails"; printed "30 pairs compared, 0 bad; client 0's orphans [];
  at or past 65,536 []; client 0 stale 14", round 1's Q-1 silent drop); the clean twin `..._the_client_files_tick_65_
  537_under_itself ... ok`. Every other binary ok (client lib 2, bites 1, endpoint 13, gap 6, pump 18, schedule 17).
  Restored; 23 OK. FX-1 (a) holds.
- **FX-1 plant (b)** (R1's (b): s03'/s04', the bite in place in the clean test), server/tests/loopback.rs, the clean
  test: old `    let mut lab = tick16_lab(None);`; new `    let mut lab = tick16_lab(Some("PM-0001-17/tick16"));` (s03'),
  then `Some("PM-0001-58/tick16")` (s04'). Runs r2-te-p1b-s03.log, r2-te-p1b-s04.log: `cargo test --locked
  --all-features --no-fail-fast -p npt-server -p npt-client --test loopback tick16`, each exit 101, "test result:
  FAILED. 1 passed; 1 failed": `pm_0001_17_and_58_tick16_the_client_files_tick_65_537_under_itself ... FAILED`
  (loopback.rs:282:5, the 65,537 pair: "the server's hash at 65,537 Some(2768209054929083115), the client's None; 58
  pairs compared, 28 bad; client keys below 65,520 [(0, 0) .. (0, 13), (1, 0) .. (1, 13)]"), the twin ok. Restored to
  `tick16_lab(None)`; 23 OK. FX-1 (b) holds. Part (i) in the counted run: below (step 4).
- **Own plant P-form** (settles the netcode seat's NC2-L1 by run, and asks one more question: is R1's case 2 pinned?),
  client/src/lib.rs, two Edits as one plant. Edit 1, `old_header`: the same old text as plant (a); new `        let _ =
  count;` / `        self.bad_header(t);` / `        true` (every too-old header is a bad header: case 2 files the
  sentinel too, which is the netcode seat's round-1 "simpler form" R1 rejected; no `refused` count, no `stale`). Edit 2,
  `bad_header`: old `        if self.bad_headers.contains(&t) {` / `            return;` / `        }` / `        if
  self.bad_headers.len() >= BAD_HEADERS_HELD {`; new `        let _ = self.slot_for(t);` / `        self.ack = Some((t,
  1));` / `        if self.bad_headers.len() >= BAD_HEADERS_HELD {` (no "once", a ring slot taken, the bad tick
  acknowledged; a plain `acknowledge(t, 0)` would be a no-op, since `acknowledge` never moves back and a case-3 tick is
  behind every acknowledged one). Run r2-te-pform.log: `cargo test --locked --all-features --no-fail-fast -p
  npt-server -p npt-client -- --nocapture`: **exit 0**, every binary ok (client lib 2, bites 1, endpoint 13, gap 6,
  loopback 7, pump 18, schedule 17). The twin still prints 58 compared, 28 bad, orphans [0..13], none at or past
  65,536, but its server hash at 65,537 changed (3119198053681243615, against 2768209054929083115 unplanted): the forced
  acknowledgement reached the room and changed its snapshots, and no test saw it. Restored by both reverse Edits; 23
  OK.
  - So NC2-L1 holds by run, and wider: none of case 3's form (once, no ring slot, never acknowledged, counted
    `refused`) and not case 2's rule (a too-old tick from the first header on is dropped as `stale`, never the
    sentinel: the reason R1 gives for rejecting the simpler form, "a correct build meeting one very late group ... would
    end INVALID or `crash`") is read by any test; the twin and the clean test, which are what R1's unit proof asks
    for, read only what the judge counts. -> finding TE2-L1 (Low).
- **FX-2 (NC-M3), P-2**, server/src/pump.rs: old `                (State::Joined, Message::Pong(p)) => self.pong(&p,
  r.arrival_ns),`; new `... => self.pong(&p, now_ns),`. Run r2-te-p2.log (`cargo test --locked --no-fail-fast -p
  npt-server --test pump`): exit 101, "FAILED. 15 passed; 1 failed", `e10_the_rtt_sample_is_taken_at_the_pongs_arrival_
  not_at_the_pump ... FAILED` (pump.rs:332:5, left Some(120000000), right Some(100000000)). Restored; 23 OK. Holds.
- **FX-4 (A2), P-4**, pump.rs `rtt_ns`: old `        match self.rtt {` / `            RttSource::Link =>
  link.rtt_ns(),` / `            RttSource::Pings => self.samples.iter().copied().min(),` / `        }`; new `
  link.rtt_ns().or_else(|| self.samples.iter().copied().min())`. Run r2-te-p4.log (`cargo test --locked --no-fail-fast
  -p npt-server`, every server binary): exit 101; only pump fails, "FAILED. 14 passed; 2 failed":
  `a2_a_link_session_takes_the_links_rtt_never_a_ping_sample` (pump.rs:351:5, left Some(100000000), right None) and
  `a2_a_pings_session_takes_its_ping_sample_never_the_links_rtt` (pump.rs:342:5, left Some(0), right Some(100000000));
  gap 3, loopback 5, schedule 11 ok. Restored; 23 OK. Holds.
- **FX-11 (TE-5), P-C** (round 1's), pump.rs `Counters::refused`: old `            DecodeError::TooLarge =>
  self.too_large += 1,`; new `... => self.truncated += 1,`. Run r2-te-pc.log (pump binary): exit 101, "FAILED. 14
  passed; 2 failed": `e9_a_control_message_one_byte_over_the_streams_cap_is_too_large_and_closes_the_session`
  (pump.rs:391:5, left (0, 1), right (1, 1)) and `e9_an_input_one_byte_over_its_limit_is_counted_too_large`
  (pump.rs:366:5, left (0, 0, false), right (1, 0, false)). Restored; 23 OK. Holds; TE-5 closed (both cases read the
  refusal's kind, so a pad that a trailing-bytes check also refuses cannot stand in for the size check: TE-10's trap
  avoided).
- **FX-13 SEC-U4E-1, P-13a** (the builder's), pump.rs `pump`'s Err arm: old the two comment lines `// Before the
  `Join`, a forged kind first on the stream ...` / `// (catalogue :93; ...)`, then `                    if stream &&
  self.state == State::AwaitJoin {` / `                        self.counters.first_not_join += 1;` / `
  self.close_malformed(link, now_ns);` / `                        return;` / `                    }`, then `
  continue;`; new `                    continue;` alone. Run r2-te-p13a.log (pump binary): exit 101, "FAILED. 15 passed;
  1 failed", `sec_u4e_1_a_forged_kind_first_on_the_stream_closes_the_session ... FAILED` (pump.rs:416:5 "assertion
  failed: out.is_empty() && d.is_closed()"). Restored; 23 OK. Holds.
- **Own plant P-13a'** (R5's rule names two refusals, "wrong channel or wrong direction"; the test sends one), the same
  site: old `                    if stream && self.state == State::AwaitJoin {`; new `                    if stream &&
  self.state == State::AwaitJoin && matches!(e, DecodeError::WrongChannel) {` (a wrong-direction message first on the
  stream counted and passed over again, as before the fix). Run r2-te-p13a2.log (pump binary measured, then pump and
  gap in the bites build): **exit 0**, "ok. 16 passed; 0 failed", "ok. 6 passed" (gap), "ok. 18 passed" (pump).
  Restored; 23 OK. So the wrong-direction half of SEC-U4E-1's fix has no test; the committed test is Q-3's case (an
  `Input` framed on the control stream: wrong channel), which is what R5 asked for. -> finding TE2-L2 (Low).
- **FX-13 SEC-U4E-3, P-13b**, pump.rs: old `                _ => self.counters.joining_dropped += 1,`; new `
  _ => {}`. Run r2-te-p13b.log (pump binary): exit 101, "FAILED. 15 passed; 1 failed",
  `sec_u4e_3_a_stream_message_while_joining_is_counted ... FAILED` (pump.rs:438:5, left 0, right 1). Restored; 23 OK.
  Holds.
- **FX-3 (A1), P-3** (the builder's), room/src/room.rs `full_state`'s end: old `    c.ack.record(t, 1, frame,
  Vec::new());` / `    c.control.push(bytes);` / `}`; new the first line, then `    let _ = bytes;` / `    let mut bytes =
  Vec::new();` / `    let _ = npt_proto::encode(&msg, &mut bytes);` / `    c.control.push(bytes);` / `}`. Run
  r2-te-p3.log (`cargo test --locked --all-features --no-fail-fast -p npt-room --test room`): exit 101, "FAILED. 26
  passed; 1 failed", `bites::a1_take_control_hands_out_the_full_state_encoded_with_the_rooms_quant ... FAILED`
  (room.rs:1402:9 "the room's Quant's encoding", the two byte lists). Restored; 23 OK. Holds. The builder's reading of
  R2's "differ from `npt_proto::encode`'s" (the clean twin room's bytes, since `npt_proto::encode` of the decoded bitten
  message gives the bitten bytes back) is sound: the plant shows the test tells the two encoders apart.
- **FX-5 (A4), P-5** (the builder's), room.rs `tick`: old `            self.next_event =
  self.next_event.wrapping_add(1);` / `        }` / `        self.history.push(self.sim.hit_volumes());`; new the
  first two lines, then `        self.input.spawns.clear();` / `        self.input.teleports.clear();` / `
  self.input.attributes.clear();`, then the history line. Run r2-te-p5.log (`cargo test --locked --no-fail-fast -p
  npt-room --test room`): exit 101, "FAILED. 18 passed; 1 failed", `a4_tick_input_after_a_tick_is_that_ticks_whole_input
  ... FAILED` (room.rs:483:5, left [], right [Placement { slot: 0, position: [100, 200, 0] }]). Restored; 23 OK.
- **Own plant P-5'** (the test's second half: the tick after holds none), room.rs `fill_input`: old `
  self.input.teleports.clear();` / `        self.input.teleports.append(&mut self.scripted_teleports);`; new the second
  line alone (last tick's teleports stay in the input). Run r2-te-p5b.log (room binary): exit 101, "FAILED. 18 passed;
  1 failed", `a4_... FAILED` (room.rs:485:5 "nothing scripted"). Restored; 23 OK. FX-5 holds on both halves.
- **FX-6 (A5), P-6, with FX-12 (TE-6), P-57, one run** (independent sites, each read by its own test's name). P-6,
  room.rs `with_cover`: old `        self.cover = cover;`; new `        let _ = (&mut self, cover);`. P-57 (round 1's),
  room.rs `route_events`: old the comment line `                    // A second overflow inside an unacknowledged
  forced resync: Kick(overflow), the client dropped.` then `                    c.kick(&mut self.roster);`; new the
  comment line, then `                    c.queue.clear();`. Run r2-te-p6-p57.log (`cargo test --locked --all-features
  --no-fail-fast -p npt-room -- --nocapture`, every room binary, bites build): exit 101; room lib 20, bites 1,
  catalogue 1 ok; room "FAILED. 25 passed; 2 failed": `a5_a_room_with_cover_present_hands_the_sim_each_rewind_with_
  cover_present ... FAILED` (room.rs:557:5 "[Rewound, Rewound, Rewound]") and `carry_57_a_never_acker_overflows_once_
  then_a_second_overflow_kicks ... FAILED` (room.rs:1150:5 "Some(495)"; printed "first overflow resync at 434,
  Kick(overflow) at Some(495), the timeout's tick 495": with the second-overflow route off the timeout kicks, exactly
  round 1's Q-9 case). Restored by both reverse Edits; 23 OK. FX-6's test holds; FX-12 holds and TE-6 is closed (the
  edge's test now fails when the edge is broken; clean value 459 < 495, the builder's T5 print).
- **FX-7 (SEC-U4E-4's room half), P-7** (the builder's), room.rs `receive`'s Resync arm: old `
  .is_some_and(|f| next.wrapping_sub(f) < npt_sim::FULL_STATE_ACK_TICKS)`; new `... < 1)` (never true: the limit off).
  Run r2-te-p7.log (`cargo test --locked --no-fail-fast -p npt-room --test room -- --nocapture`): exit 101, "FAILED.
  17 passed; 2 failed": `sec_u4e_4_a_resync_sooner_than_full_state_ack_ticks_after_the_last_full_state_is_limited`
  (room.rs:450:5, left (59, 59, 0, 0, 60), right (0, 0, 59, 0, 1); printed "59 Resyncs within 60 ticks of the join's
  full state, 59 answered") and the re-pinned `e9_a_second_resync_before_the_full_states_ack_is_dropped_and_counted`
  (room.rs:393:5, left (2, 2, 0), right (1, 2, 1)). Restored.
- **Own plants P-7+ and P-7-** (each edge alone), the same line. P-7+: new `... <= npt_sim::FULL_STATE_ACK_TICKS)` (the
  Resync whose answer falls exactly the window after the last full state is limited too). Run r2-te-p7le.log: exit
  101, "FAILED. 17 passed; 2 failed": `sec_u4e_4_...` (room.rs:464:5 "answered at the window's end") and `e9_...`
  (room.rs:380:5, left (0, 0), right (1, 1): e9's first Resync, which now comes exactly the window after the join's
  full state). P-7-: new `... < npt_sim::FULL_STATE_ACK_TICKS - 1)` (one tick sooner answered). Run r2-te-p7lt.log:
  exit 101, "FAILED. 18 passed; 1 failed": `sec_u4e_4_...` (room.rs:450:5, left (1, 1, 58, 0, 2), right (0, 0, 59, 0,
  1)). Restored to the fixed line; 23 OK. So the limit's both edges are pinned exactly (59 limited at next - f = 1 ..
  59; answered at 60), as the netcode and rust seats read. FX-7 holds.
- **FX-8 (TE-2), P-A** (round 1's), room.rs `snapshot`: old `    let left = c.t.remainder;`; new `    let left =
  c.t.budget;`. Run r2-te-pa.log (`cargo test --locked --no-fail-fast -p npt-server --test loopback -- --nocapture`,
  the whole loopback binary): exit 101, "FAILED. 4 passed; 1 failed", `e6_the_pumps_bytes_shrink_the_snapshot_to_the_
  remainder ... FAILED` (loopback.rs:231:5 "1027 B sent of 467"; printed "budget 1067 B, remainder 467 B after 600 B
  more of the pump's; sent 1027 B in 1 groups; the twin with none added sent 1027 B"); the old
  `e6_the_window_counts_the_pumps_bytes` passes under it, as in round 1 (it never binds). Restored; 23 OK. FX-8 holds;
  TE-2 closed (the plan's "the window counting the pump's bytes" now has a test that fails when it is broken).
- **FX-13 TE-8 with RR-2, P-G** (round 1's), room.rs `route_events`: old `            c.counters.events_expired +=
  u64::from(c.queue.expire(t));`; new `... += u64::from(t & 0);`. Run r2-te-pg.log (`cargo test --locked --no-fail-fast
  -p npt-room -- --nocapture`, every room binary): exit 101; lib 20, catalogue 1 ok; room "FAILED. 18 passed; 1
  failed", `carry_50_the_room_sends_an_event_at_age_255_and_none_at_256 ... FAILED`, panicked at room/src/room.rs:1214:9
  "an event older than 255 ticks reached the encoder (carry 50)" (RR-2's debug assert, under the test's name).
  Restored; 23 OK. Holds: the room's `expire` call is now pinned (round 1's TE-8 gap closed).
- **Own plants on carry 50's two edges**, room/src/events.rs `EventQueue::expire`. P-G+ (one tick late): old `
  .retain(|q| now.wrapping_sub(q.tick) <= MAX_EVENT_AGE);`; new `... <= MAX_EVENT_AGE + 1);`. Run r2-te-pgplus.log
  (every room binary): exit 101; lib "FAILED. 19 passed; 1 failed" (`events::tests::carry_50_an_event_at_age_255_is_
  kept_and_at_256_dropped_and_counted`, events.rs:143:9) and room "FAILED. 18 passed; 1 failed" (`carry_50_the_room_
  sends_...`, again by the debug assert at room.rs:1214:9). P-G- (one tick early): new `... < MAX_EVENT_AGE);`. Run
  r2-te-pgminus.log (`--test room ... carry_50`): exit 101, `carry_50_... FAILED` at room/tests/room.rs:626:5 "carried
  at age 255: [0, 1, .. 254]" (printed "carried 255 times, at ages Some(0) to Some(254)"). Restored; 23 OK.
- **Own plant P-RR21** (settles the rust seat's RR2-1 by run): P-G+'s events.rs line kept, and in room.rs `snapshot`
  the fix's age read put back to round 1's cast: old `        let event_age = u8::try_from(t.wrapping_sub(q.tick));` /
  the `debug_assert!(` block (4 lines) / `        let Ok(event_age) = event_age else {` / `            continue;` / `
  };`; new `        let event_age = t.wrapping_sub(q.tick) as u8;` (an event at age 256 kept and coded as age 0). Run
  r2-te-prr21.log (`--test room ... carry_50`): **exit 0**, "ok. 1 passed", printed "event 17 of tick 402 carried 256
  times, at ages Some(0) to Some(255); ... events expired 61": the 256th carry went out at age 0 and the test did not see
  it. Restored (both files; then P-G- above ran, then restored); 23 OK. So RR2-1 holds by run: the test's "none at 256
  or older" assertion cannot fail by itself (it matches the event by number and header tick less age, and a wrapped age
  names another tick); the 256 edge is guarded by the fix's `u8::try_from` and debug assert, which P-G and P-G+ show
  the test reaches. Low, as the rust seat rates it; my reading adds nothing beyond its fix.
- **FX-13 RR-10, P-R10** (the builder's), room.rs `route_events`: old `            for q in gameplay {`; new `
  for q in gameplay.into_iter().take(0) {` (the room queues no event: carry 23's queue half would be vacuous). Run
  r2-te-pr10.log (`cargo test --locked --no-fail-fast -p npt-room --lib`): exit 101, "FAILED. 19 passed; 1 failed",
  `room::tests::carry_23_events_carry_their_tick_and_consecutive_32_bit_numbers_across_a_16_bit_wrap ... FAILED`
  (room.rs:1469:9 "the queue half checked a queued event (RR-10)"). Restored; 23 OK. Holds.
- **FX-9 (TE-3), P-B** (round 1's), client/src/lib.rs `with_bite`: old `            self.grid = grid;`; new `
  let _ = grid;`. Run r2-te-pb.log (`cargo test --locked --all-features --no-fail-fast -p npt-client`): exit 101;
  endpoint "FAILED. 12 passed; 1 failed", `bites::pm_0001_17_client_rounding_at_the_endpoints_call_site_files_another_
  hash ... FAILED` (endpoint.rs:432:9 "[(101, 9432435560552865056)]", the mirror's hash); client lib 2 and bites 1 ok
  (round 1: every test passed under this plant). Restored; 23 OK. FX-9 holds; TE-3 closed.
- **FX-10 (TE-4), P-D'** (round 1's), client/src/lib.rs `group`, after the `let Some(base) = base else { ... };` block:
  old `            return;` / `        };` / `        let mut tick = core::mem::take(&mut self.ring[i]);`; new the
  first two lines, then `        if g > 0 && self.ring[i].got & 1 == 0 {` / `            self.refused += 1;` / `
  self.sentinel(t);` / `            return;` / `        }`, then the `let mut tick` line. Run r2-te-pd2.log (`cargo test
  --locked --all-features --no-fail-fast -p npt-server -p npt-client`): exit 101; only endpoint fails, "FAILED. 12
  passed; 1 failed", `e3_a_split_tick_without_group_0_acknowledges_its_other_group_and_is_resolved_lost ... FAILED`
  (endpoint.rs:173:5 "one group of two: nothing filed"); gap 6, loopback 7, pump 18, schedule 17 ok (e2 still passes it,
  as in round 1: the new e3 case is the one that bites). Restored; 23 OK. FX-10 holds; TE-4 closed.
- **FX-13 SEC-U4E-7, P-13c, with P-H's Edit 1 alone, one run** (independent: the first matters only when a run's sum
  passes u32::MAX). P-13c, client/src/lib.rs `lost`: old `            && last.0.wrapping_add(last.1) == first` / `
  && let Some(sum) = last.1.checked_add(count)` / `        {` / `            last.1 = sum;`; new `            &&
  last.0.wrapping_add(last.1) == first` / `        {` / `            last.1 += count;`. P-H Edit 1 (settles the rust
  seat's RR2-2 point that `file()`'s different-hash arm is unreachable), `file`: old `            Filed::Hash(h) if h
  != hash => {` / `                self.ring[i].filed = Filed::Sentinel;` / `                self.out.hashes.push((t,
  0));` / `            }`; new `            Filed::Hash(h) if h != hash => {}`. Run r2-te-p13c-ph1.log (every server and
  client binary, bites build): exit 101; only endpoint fails, "FAILED. 12 passed; 1 failed",
  `sec_u4e_7_lost_runs_2_pow_31_apart_never_overflow_and_cover_the_ticks ... FAILED` (panicked at
  client/src/lib.rs:588:13 "attempt to add with overflow"); `carry_61_b_a_second_different_hash_for_one_tick_gives_the_
  sentinel` passes with `file()`'s arm off, and so does every other test. Restored by both reverse Edits; 23 OK.
  SEC-U4E-7 holds. RR2-2's "P-H fails TE-7 by its Edit 2 alone" holds by run (next item).
- **FX-13 TE-7, P-H** (round 1's two Edits as one plant): Edit 1 as above; Edit 2, `group`: old `            } else {` /
  `                self.refused += 1;` / `                self.sentinel(t);` / `            }` / `            return;`
  (after `                self.duplicates += 1;`); new `            } else {` / `                self.duplicates +=
  1;` / `            }` / `            return;`. Run r2-te-ph.log (`cargo test --locked --all-features --no-fail-fast -p
  npt-client`): exit 101, endpoint "FAILED. 12 passed; 1 failed", `carry_61_b_a_second_different_hash_for_one_tick_
  gives_the_sentinel ... FAILED` (endpoint.rs:205:5 "the same group with other bytes", left [], right [(101, 0)]).
  Restored by both reverse Edits; 23 OK. TE-7 closed as I worded it in round 1 ("feed one group twice with different
  bytes; assert the sentinel"). Its second case reaches the sentinel by the same byte-signature route (a full state
  leaves group 0's signature 0), not by two hashes; the rust seat's probe B (r2-rr-probe.log, a snapshot then a full
  state of another hash for one tick: nothing filed, one duplicate) is a real carry 61 (b) gap in that order, out of
  reach of U4e's room (it sends a client a full state or a snapshot per tick, never both). RR2-2 stands at Low; no new
  finding of mine.
- **Compile-time items, no plant (as R5 and R2 rule):** RR-3's two const asserts; FX-6's eight items pub(crate) (no
  test can see a visibility; the counted run's clippy, clippy-wasm32 and doc steps show no caller outside its crate
  names one). FX-6's runtime half (`with_cover`'s test) is P-6 above.
- **Scope (R8's bounds), checked:** `diff -rq -x target .gate/u4e-r1/spikes/netcode-proof <copy>/spikes/netcode-proof`:
  11 files differ, all under crates/client (src/lib.rs, tests/endpoint.rs), crates/room (src/ack.rs, src/room.rs,
  src/roster.rs, tests/room.rs) and crates/server (src/lib.rs, src/pump.rs, tests/common/mod.rs, tests/loopback.rs,
  tests/pump.rs); no manifest, Cargo.lock, schedule.rs, npt-proto, npt-sim, npt-link or npt-lab file. Every changed
  test is one of FX-1 to FX-13's or the re-pins the builder's plan named (e9's Resync test for FX-7; carry 57's
  `never_acker` for FX-12; carry 23's for RR-10; e10 and the lab for FX-3 and FX-4). Nothing beyond R8 in the tests.
- **Other seats' findings settled by run here:** NC2-L1 (P-form: holds, and wider: TE2-L1); RR2-1 (P-RR21: holds);
  RR2-2's unreachable `file()` arm (P-H Edit 1 alone: every test passes; holds). Not run, because a committed test or
  another seat's run already settles them: SEC-U4E-R2-1 (the committed `a_join_on_a_slot_with_a_live_session_replaces_
  it` shows a full state one tick after a replacement; it passes in the counted run), RR2-3 (the rust seat's probe C,
  r2-rr-probe.log), NC2-M1 (the SEC-U4E-4 test itself prints the drop; its consequence is a reading of later marks).
  A-R2-1, A-R2-2, SEC-U4E-R2-2 to -4: readings of unreachable or text cases; no run would settle more.
- No failure outside a plant in any of the runs above (every non-planted binary ok in each), so no TE2-5 record.

## Step 4: the counted run
- Started 16:15 UTC: `bash u4e-run.sh -x <copy> .gate/u4e/r2-te-check-counted.log cargo test --locked --workspace
  --no-run :: cargo test --locked --workspace --all-features --no-run :: node <copy>/spikes/netcode-proof/check.mjs`
  (the two `--no-run` builds first, as M5's form, so the log names the copy in its Compiling lines). Run in the
  background with a 1,500,000 ms limit instead of 600,000 ms: round 1's counted run of mine took 590 s
  (r1-te-check-counted.log, test step 505 s), too near 600 s for a run that must not be cut; the runner, the lock and the
  command are the brief's.
- Result (r2-te-check-counted.log): runner exit 0, "--- end 16:27:02 UTC, exit 0". Both `--no-run` builds exit 0
  (16:16:14, 16:17:23), their Compiling lines naming the copy (npt-client, npt-room, npt-server among them). `node
  check.mjs` exit 0 at 16:27:02 (579 s). Its step lines: PASS fmt 1.1 s; PASS clippy 20.7 s "measured build and bites
  build both linted"; PASS clippy-wasm32 7.8 s; PASS doc 66.2 s "host and wasm32, rustdoc warnings as errors"; PASS test
  475.2 s "measured build: 473 tests passed, 0 failed, 0 reported a skip; bites build: 541 tests passed, 0 failed, 0
  reported a skip"; PASS deps 2.1 s; PASS refs 0.0 s "757 tracked files"; PASS marks 3.5 s "81 marks read and checked";
  PASS self 1.3 s "52 of 52 tests passed"; PASS redact 0.8 s "10 of 10 tests passed; ... 255 item(s) checked, 0
  finding(s)"; "check: all 10 steps pass".
- Counts as deltas: on round 1's 458 / 524 / 52: +15 measured, +17 bites, +0 self. On the fix pass's 473 / 541 / 52
  (fx-counted1.log, fx-counted2.log): +0, +0, +0. The 17 new tests are the builder's list (15 measured, 2 bites-only),
  and each failed by name under a plant of mine above: endpoint e3 TE-4 (P-D'), carry_61_b (P-H), sec_u4e_7 (P-13c),
  bites TE-3 (P-B); room sec_u4e_4 (P-7), a4 (P-5), a5 (P-6), carry_50 (P-G), bites a1 (P-3); loopback e6 binding
  (P-A); pump e10 arrival (P-2), both a2 (P-4), both e9 size tests (P-C), sec_u4e_1 (P-13a), sec_u4e_3 (P-13b). The
  renamed twin (P-1a), the re-pinned clean tick16 test (s03', s04'), e9's re-pinned Resync test (P-7, P-7+), carry 57
  (P-57) and carry 23 (P-R10) also failed under theirs.

## Step 5: end state
- `sha256sum -c r2-hashes-start.sha256` from the copy's root: 23 of 23 OK, exit 0 (also after each plant and after
  the counted run). `git status --untracked-files=all --short`: 24 lines (the 23 files and the builder's record);
  HEAD 97bb9d3; nothing committed, nothing staged; every plant restored.

## Findings (ranked; severity on CLAUDE.md's one scale)
None Critical, High or Medium. Two Lows of mine, neither before the merge (follow-ups by R8's outcome rule).

**TE2-L1, Low, not before merge (extends the netcode seat's NC2-L1; settled by run, P-form).**
client/src/lib.rs:503-517 (`old_header`) and :521-530 (`bad_header`); the only test that reaches them is the loopback
twin (server/tests/loopback.rs:302-332), and no endpoint test feeds a too-old header.
- What: none of the properties R1 set for the two too-old cases is read by any test beyond what the judge counts.
  Case 3's form (the sentinel once per tick, no ring slot, never acknowledged, counted `refused`) and case 2's rule (a
  too-old tick from the first header on is dropped and counted `stale`, never the sentinel) can all be broken at once
  and every test passes.
- Scenario: P-form (case 2 files the sentinel like case 3, which is the "simpler form" R1 rejected; no once-check; a
  ring slot taken; the bad tick acknowledged; no `refused`/`stale` count): r2-te-pform.log, every server and client
  binary ok, exit 0. The forced acknowledgement reached the room and changed its snapshots (the twin's server hash at
  65,537 is 3119198053681243615 against 2768209054929083115 unplanted) and no test saw it. A later edit that folds case 2
  into case 3 would pass the suite and, by R1's own reasoning, make a correct build meeting one very late group (U6's
  real network) end INVALID or `crash` ("never both").
- Reach today: none in an honest U4 run (L1's lateness is far under the ring, the room sends nothing before the join's
  full state); a bitten run fails either way. R1's required unit proof (the twin and its clean test) is present and
  holds by run (plants (a) and (b)).
- Cheapest proof: the P-form run above. Fix (the next unit to edit npt-client, U4f by carry 80, or U4r's contract
  test): one endpoint test, joined at 100 and fed to about 140: a group of tick 60 (case 3) fed twice files `(60, 0)`
  once, the next `Input` acknowledges no tick 60, `newest_tick()` is unchanged, `refused()` rose by 2 and a delta on a
  live baseline still decodes; then a group of tick 105 (case 2) files nothing, `stale()` 1, never the sentinel.

**TE2-L2, Low, not before merge (settled by run, P-13a').** server/src/pump.rs:382-388 with
server/tests/pump.rs:406-422 (`sec_u4e_1_a_forged_kind_first_on_the_stream_closes_the_session`).
- What: R5's SEC-U4E-1 rule names two refusals that close a session in AwaitJoin, "wrong channel or wrong direction";
  the committed test (Q-3's case, as R5 asked) sends only the first (an `Input` framed on the control stream).
- Scenario: P-13a' keeps the close for `WrongChannel` only (a wrong-direction message first on the stream is counted
  and passed over, the pre-fix behaviour): r2-te-p13a2.log, pump 16 of 16 (measured) and 18 of 18 (bites), gap 6 of 6,
  exit 0.
- Reach: a client that opens with a room-to-client type (npt-proto returns `WrongDirection` before the channel check:
  proto/src/msg.rs:417-419) keeps its session open until its `Join`; the 2 s Join deadline still bounds it. The code is
  right by reading.
- Cheapest proof: the run above. Fix (with the next pump edit, U6's TickLoop or carry C-U6-12): a second case in the
  same test, a `JoinAccepted` framed on the control stream first: closed, `first_not_join` 1, `wrong_direction` 1,
  `Kick(malformed)`, the `Join` behind it never taken.

**Other seats' findings this seat settled by run** (no new id): NC2-L1 holds (inside TE2-L1); RR2-1 holds (P-RR21: an
event carried at age 256, coded 0, and the carry 50 test passes; the edge is guarded by the fix's try_from and debug
assert, which P-G and P-G+ show the test reaches); RR2-2's point that `file()`'s different-hash arm is unreachable holds
(P-H Edit 1 alone: every test passes; P-H fails TE-7 by Edit 2).

## Claim, proof, gap (the fix pass)
| Claim (R1-R6) | Proof (test; the plant it fails under) | Gap |
|---|---|---|
| FX-1 case 3: post-wrap headers filed as orphan sentinels | loopback twin (P-1a); clean test (s03', s04') | case 3's form and case 2's rule unread: TE2-L1 |
| FX-2 Pong sampled at arrival | e10 arrival test (P-2) | none |
| FX-3 take_control bytes in the room's Quant | a1 bites test (P-3) | JoinAccepted/Kick bytes unobservable under any bite (A-R2-2's silent drop: reading) |
| FX-4 RTT by session kind | two a2 tests (P-4) | none |
| FX-5 tick_input whole | a4 test (P-5, P-5') | none |
| FX-6 with_cover | a5 test (P-6) | visibility: compile-time only |
| FX-7 Resync limit, both edges | sec_u4e_4 and e9 (P-7, P-7+, P-7-) | none (NC2-M1 is a consequence, not a test gap) |
| FX-8 remainder binds | e6 binding test (P-A) | none |
| FX-9 client-rounding at the endpoint | bites endpoint test (P-B) | none |
| FX-10 group 1 without group 0 | e3 test (P-D') | none |
| FX-11 size checks through the pump | two e9 tests (P-C) | none |
| FX-12 second overflow kicks before the timeout | carry 57 (P-57) | none |
| SEC-U4E-1 forged kind before Join closes | sec_u4e_1 (P-13a) | wrong-direction half: TE2-L2 |
| SEC-U4E-3 Joining-state message counted | sec_u4e_3 (P-13b) | none |
| SEC-U4E-7 no lost-run overflow | sec_u4e_7 (P-13c) | none |
| TE-7 a second, different group gives the sentinel | carry_61_b (P-H) | the snapshot-then-full-state order (RR2-2, Low) |
| TE-8/RR-2 the room expires at 256 | carry 50 room test (P-G, P-G+, P-G-) | its "none at 256" assert alone (RR2-1, Low) |
| RR-10 carry 23's queue half not vacuous | carry_23 unit (P-R10) | none |
| RR-3 at most 8 groups | const asserts | compile-time |

## Round 1 findings of this seat that R8 put in the fix pass: each closed
- TE-1 (High, with NC-H1; FX-1): CLOSED. Plant (a) fails the twin by name; plant (b) (s03', s04') fails the clean test
  by name; the twin asserts what the judge counts (bad >= 1, 14 orphans below 65,520, none at or past 65,536).
- TE-2 (FX-8, P-A): CLOSED. TE-3 (FX-9, P-B): CLOSED. TE-4 (FX-10, P-D'): CLOSED. TE-5 (FX-11, P-C): CLOSED. TE-6
  (FX-12, P-57): CLOSED.
- TE-7 (rider, P-H): CLOSED as worded. TE-8 (rider with RR-2, P-G; edges P-G+ and P-G-): CLOSED.
- Each other FX's test fails under its plant: FX-2 (P-2), FX-3 (P-3), FX-4 (P-4), FX-5 (P-5, P-5'), FX-6 (P-6), FX-7
  (P-7, P-7+, P-7-), FX-13 SEC-U4E-1 (P-13a), SEC-U4E-3 (P-13b), SEC-U4E-7 (P-13c), RR-10 (P-R10); RR-3 and FX-6's
  visibility half are compile-time.

## Verdict
APPROVE WITH FINDINGS, Lows only (TE2-L1, TE2-L2), neither before the merge: by R8's outcome rule the merge goes ahead
and the two become follow-ups the main session records. Every FX's test fails by name under its plant (the builder's
forms re-run, and my own on the edges: P-5', P-7+, P-7-, P-G+, P-G-); FX-1's plants (a) and (b) hold; TE-1 to TE-8 are
closed; the counted check.mjs is all 10 PASS at 473 / 541 / 52 (+15 / +17 / +0 on round 1, +0 on the fix pass); no
failure outside a plant (no TE2-5 record); nothing changed beyond R8's scope and bounds; the copy ends at 23 of 23
start hashes. Not verified by me: the T5 prints' figures (the builder's), wasm32 test runs (none exist; clippy and doc
only), and anything outside crates/room, crates/server and crates/client.
