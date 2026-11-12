# U4e round 1: test-engineer review notes (log prefix te)

Role: main's .claude/agents/test-engineer.md (read 2026-11-12). Copy under test:
/home/user/Shamble-Frontlines/.claude/worktrees/wf_0fa6595a-3e2-1 (uncommitted U4e change); read copy
/home/user/Shamble-Frontlines/.gate/u4e-r1. Runner: u4e-run.sh, logs /home/user/Shamble-Frontlines/.gate/u4e/r1-te-*.log.

## Plan
1. sha256 of every changed and new file under the copy's spikes/netcode-proof/crates (git status --untracked-files=all),
   compared with the same file in .gate/u4e-r1. Record the table here.
2. Read the packet (heads; §5 U4e; §4.1, §4.2, §4.10, §4.14; rulings W1-*, advisor-u4e-brief whole, B2.1, B5; carries)
   and the packet's tests side file. List U4e's bites (§5 U4e Bites line, §4.14, U4E-Q1, Q3, Q4, Q10, Q12, Q13).
3. For each bite: part (i) from the counted logs b11-counted1/2 (bites build: the bite's test passes, no clean test
   fails); part (ii): the builder's scratch switches s01..s15 read; my own switch by Edit where I doubt them or where
   none exists; each restored by Edit and checked by sha256.
4. The tests the plan lists (§5 U4e Tests, negative cases, -18 end to end, e1..e12): read each; assertion vs print;
   wrong-reason passes; missing tests or clauses.
5. Carries routed to U4e with a test: one plant each where cheap.
6. Settle by run any finding of the netcode, security, rust round-1 notes that a run can settle.
7. One counted check.mjs run at the end (-x); step lines and counts as deltas on base 399 / 446 / 52.
8. sha256 of every file again at the end, equal to step 1's.

## Blocks (written as each finishes)

### Block 1. Hashes at start (done)
Builder copy HEAD 97bb9d3; read copy .gate/u4e-r1 HEAD 97bb9d3 with the same 6 M and 17 ?? files under crates/.
All 23 files EQ (builder copy sha256 = read copy sha256). The full 64-hex list (sha256sum format, paths relative to
the copy root) is in .checkpoints/netcode-proof-test/u4e-review-test/hashes-start.sha256; every plant's restore and
block 8 check against it with `sha256sum -c`. Files (lines): client/clippy.toml 13, client/src/lib.rs 801,
client/tests/bites.rs 40, client/tests/endpoint.rs 314 (new), room/clippy.toml 13, room/src/lib.rs 46, room/src/{ack 292,
encoder 350, events 146, input 280, interest 245, rewind 33, room 1413, roster 187} (new), room/tests/{bites 33,
catalogue 80, room 1153} (new), server/src/lib.rs 17, server/src/pump.rs 430 (new), server/tests/{common/mod 378,
gap 263, loopback 292, pump 340} (new).

### Block 2. Read (done)
- Packet (heads; judged-against half whole: §5 U4e, §4.1, §4.2, §4.5-4.7, §4.9-4.11, §4.14, W1-S1/N2/PUB/U4E/TARGET,
  advisor-u4e-brief whole, B2.1, B5, Mustafa's words, carries 21-27, 39, 40, 50, 51, 53-57, 61, 62, 66, 68; left-out
  ids). Brief rev 3.3 §2 (2.1-2.8), e1..e12. Builder record (claims). Counted logs b11-counted1/2: both "check: all 10
  steps pass", measured 458 / bites 524 / self 52 of 52. s01..s15: each names the clean test FAILED by name with its
  panic (the switch text itself is not in the logs or the record: I redo the doubtful ones in my own form).
- Round-1 notes read: netcode (NC-H1 High; NC-M2..M6; NC-L1..L12), security (SEC-U4E-1..9; names 1, 3, 4, 5, 7 as
  run-settleable), rust (RR-1..10; RR-1/2/4 cite rewind.rs:73-77 and events.rs:327/:332-337, beyond those files'
  33 and 146 lines: the cites are off, the claims read against room.rs).
- Tests read whole: room/tests/{room,bites,catalogue}.rs, server/tests/{common/mod,gap,loopback,pump}.rs,
  client/tests/bites.rs; room.rs whole; pump.rs whole.

### Block 3. Candidate findings from reading (each to be settled by a plant; a line after each)
- TE-c1 e6 (loopback.rs:161-199) checks `left == budget - p` from the room's own reported pair and `bytes <= left`
  with 4-client SS snapshots of about 147 B against a remainder of about 1,029 B: the bound never binds, so an encoder
  that packs against the whole budget (room.rs:1117 `left = c.t.remainder`) passes. Plant P-A.
- TE-c2 client-rounding's clean test (client/src/lib.rs:776-779) calls `Held::hashed` with a grid passed in; nothing
  runs a `ClientEndpoint::with_bite("PM-0001-17/client-rounding")`, so the endpoint's wiring (with_bite's
  `self.grid = grid`, carry 24's first clause) is untested. Plant P-B.
- TE-c3 the pump's size check (U4E-Q11: "size, trailing bytes, wrong channel, the bucket ... and their unit tests"):
  no pump test sends an oversized datagram or stream message. Plant P-C.
- TE-c4 e2 (loopback.rs:102-156) drops group 1 only and asserts no group was acknowledged or decoded alone; "each
  group decoding alone" (plan :716-717) has no assertion; a client that cannot decode a group > 0 before group 0
  passes on in-order Loopback. Plant P-D.
- TE-c5 carry 50's room wiring (route_events' `expire`, room.rs:943, and `events_expired`): only the queue's unit
  test (events.rs:137-145). Plant P-G.
- TE-c6 tick16's bites twin (loopback.rs:241-252) asserts only "server hash present and not equal to the client's" at
  65,537, which a silent stale drop meets; the brief's 2.6 row says "logs it under 1 or as the sentinel" (what the
  judged check counts). With NC-H1: probe Q-1.
- TE-c7 the endpoint's 1,150 B case (endpoint.rs:109-114) pads a valid snapshot with zero bytes, so TrailingBytes
  refuses it too: the test passes with no size check. Low; reasoning (msg.rs:397-409 checks size first; the
  sentinel path does not tell the kinds apart).
- TE-c8 carry 61 (b)'s "two different hashes for one tick give the sentinel" (client lib.rs:633-640, :555-558): no
  test. Plant P-H.

### Block 4. Plants and probes (each: planted diff, run, result, restore sha256)
Planted-diff form: each plant is one Edit; "old" is the anchor text, "new" is the anchor plus (or in place of) the
text shown. Every restore is the reverse Edit, then `sha256sum -c hashes-start.sha256` over all 23 files (all OK
unless said).

**Q-1 (settles NC-H1; TE-c6)**, server/tests/loopback.rs. Old: the 3 lines
`            assert!(server.is_some() && server != client, "{id}");` / `        }` / `    }` (end of
`bites::pm_0001_17_and_58_tick16_post_wrap_frames_have_no_equal_client_hash`). New: those 3 lines, then a new
`#[test] fn te_probe_q1_tick16_bitten_files_nothing_after_the_wrap()` inside `mod bites`: for each of
PM-0001-17/tick16 and PM-0001-58/tick16, `tick16_lab(Some(id))`, `tick16_case`, then collect client 0's filed keys at
ticks >= 65,536 or < 1,000, the server's slot-0 hashes at >= 65,536, `lab.compare()`, the sentinels, the last client
tick, `endpoint.stale()` and `conns[0].lost`; print them; `assert!(post.is_empty() && bad == 0 && server_post > 0)`.
- Run: r1-te-q1-tick16.log (first try without `-p npt-client` did not compile: npt-client's bites feature off; my
  command, not a failure). `cargo test --locked --all-features --no-fail-fast -p npt-server -p npt-client --test
  loopback -- --nocapture tick16`: exit 0, "test result: ok. 3 passed; 0 failed".
- Printed, both ids: "client 0 post-wrap keys []; server post-wrap hashes 14; compared 30, bad 0; sentinels 0; last
  client tick Some(65535); stale 14; lost []".
- So NC-H1 holds by run: under either tick16 bite the client files no hash, no sentinel and no lost record for any
  post-wrap tick (14 groups dropped as stale by `too_old`, client/src/lib.rs:466-471, :622-625), and the run's
  mismatch figure over what the client filed is 0. The twin's `server != client` (None) passes on a silent drop.
- Restored by Edit; sha256 of loopback.rs 9c936263...04a4 = start; all 23 OK.

**Q-2..Q-5 (settle NC-M3, SEC-U4E-1, SEC-U4E-3; Q-5 the positive control for P-C)**, server/tests/pump.rs. Old: the
4 lines ending the file (`        let q = Quant::with_bite("PM-0001-21/no-channel")...`, `        assert_eq!(socket_case(q).1,
0);`, `    }`, `}`). New: those 4 lines, then `mod te_probes { use super::*; ... }` with four tests:
- Q-2 `te_probe_q2_rtt_sample_counts_the_pumps_wait`: e10's setup (NoRtt, Join at 0, accept, pump at 1,000 sends the
  ping); the client's Pong (hold_us 0) sent at 1,000 + 100 ms; the pump runs at that + 20 ms;
  `assert_eq!(d.rtt_ns(&s), Some(120_000_000))`.
- Q-3 `te_probe_q3_a_forged_kind_first_on_the_stream_does_not_close`: on a fresh session, an `Input` framed on Control
  (socket_case's framing), then a valid Join, one pump at 0; assert out = [Join], first_not_join 0, wrong_channel 1,
  session open.
- Q-4 `te_probe_q4_a_stream_message_while_joining_is_dropped_uncounted`: Join pumped (Joining, no accept), then a
  Resync on Control pumped at 1; assert out empty and `*d.counters() == before`.
- Q-5 `te_probe_q5_an_oversized_input_is_counted_too_large`: joined; an Input padded to `Limits::INPUT + 1` = 167 B
  sent as a datagram; assert too_large 1, out empty.
- Run r1-te-q2-q5-pump.log (`-p npt-server --test pump -- --nocapture te_probe`): exit 0, "test result: ok. 4 passed;
  0 failed". Printed: "Q-2: rtt Some(120000000) (arrival-based would be 100000000)"; "Q-3: out ["Join"];
  first_not_join 0; wrong_channel 1; closed false"; "Q-4: out 0; counters unchanged true; closed false"; "Q-5:
  too_large 1, trailing 0, out 0, closed false".
- Settled: NC-M3 holds (the RTT sample is the pump's now less the echo, pump.rs:389, :424: 20 ms of pump wait read
  as RTT); SEC-U4E-1 holds (a forged first stream message is counted and the session stays open, Join accepted
  after it); SEC-U4E-3 holds (a stream message in Joining is dropped with no counter).

**P-C (TE-c3: the pump's size refusal)**, with Q-2..Q-5 still in place. server/src/pump.rs:159. Old:
`            DecodeError::TooLarge => self.too_large += 1,`. New: `            DecodeError::TooLarge => self.truncated += 1,`.
- Run r1-te-pc-size.log (`cargo test --locked --all-features --no-fail-fast -p npt-server -p npt-client`): exit 101.
  Per binary: client lib ok 2, bites ok 1, endpoint ok 9, gap ok 6, loopback ok 6 (71.03 s), pump "FAILED. 14
  passed; 1 failed" (the one: `te_probes::te_probe_q5_an_oversized_input_is_counted_too_large`, my probe, panicked
  at pump.rs:434), schedule ok 17.
- So every U4e test passes with the size refusal miscounted; only my probe sees it. TE-c3 confirmed: the size check
  through the pump has no test (U4E-Q11's "size ... and their unit tests").
- Restored: pump.rs (src) by the reverse Edit, then pump.rs (tests) by removing `mod te_probes`. sha256: src
  9472b975...bc22, tests 7d8fa132...9513, both = start; all 23 OK.

**P-A (TE-c1: the window's remainder)**, room/src/room.rs:1117. Old: `    let left = c.t.remainder;` (with the next
line `    let empty = Frame::default();` as anchor). New: `    let left = c.t.budget;` (same anchor).
- Run r1-te-pa-budget.log (`cargo test --locked --all-features --no-fail-fast -p npt-room -p npt-server -p
  npt-client`): exit 0; every binary ok (room lib 20, room bites 1, catalogue 1, room 22, gap 6, loopback 6, pump 11,
  schedule 17, client lib 2, bites 1, endpoint 9).
- So an encoder that packs against the whole per-tick budget, ignoring the pump's bytes, passes every test, e6
  included: e6's `bytes <= left` never binds at 4 clients (snapshots of about 147 B against about 1,029 B). TE-c1
  confirmed.
- Restored by the reverse Edit; room.rs 8c6ec6ca...4848 = start; all 23 OK.

**The main session, 2026-11-12 07:35 UTC (not the test-engineer's text):** this agent was cut at about 06:42 UTC by
the org's spend limit, after two runs with no notes line here: r1-te-pb-rounding.log (P-B, 06:38:50-06:40:19, exit
0) and r1-te-pd-group-alone.log (P-D, 06:40:35-06:41:50, exit 0). P-B's plant was no longer in place at 07:35. P-D's
plant was found in place, client/src/lib.rs:670-674, these five lines after the `let Some(base) = base else { ... };`
block: `if g > 0 && self.ring[i].got & 1 == 0 {` / `self.refused += 1;` / `self.sentinel(t);` / `return;` / `}`.
The main session removed them by Edit: client/src/lib.rs sha256 bd83870c...d9cfd9 = start; `diff -rq` of the
copy's crates/ against .gate/u4e-r1's: no difference; the copy's patch re-cut by worktree_patch.py equals
wip/u4e.patch byte for byte (cmp). A resumed copy reads both logs for P-B's and P-D's results and records them here.

**Resumed (test-engineer, 2026-11-12, after the cut).** At resume: both copies `sha256sum -c hashes-start.sha256`
23 of 23 OK; builder copy HEAD 97bb9d3, `git status` the same 6 M and 17 ?? under crates/ (plus the builder's record).
P-B's log (r1-te-pb-rounding.log): exit 0, every binary ok, but its planted diff is not in these notes, so it proves
nothing on its own; P-D's log (r1-te-pd-group-alone.log): exit 0, every binary ok (client lib 2, bites 1, endpoint 9,
gap 6, loopback 6, pump 11, schedule 17), with the plant the main session found (above). I redo both in my own form
below, with the planted diff recorded, so each result rests on a diff in these notes.

**P-B (TE-c2: the endpoint's client-rounding wiring)**, client/src/lib.rs:293-294 (`with_bite`). Old:
`        if let Some(grid) = grid_with_bite(bite) {` / `            self.grid = grid;`. New: the same first line, then
`            let _ = grid;` (the endpoint built with PM-0001-17/client-rounding keeps the wire's grid: the bite silently off).
Command: `cargo test --locked --all-features --no-fail-fast -p npt-server -p npt-client` (every test that builds an
endpoint). Log r1-te-pb2-rounding.log. (Planted before the run; if this line has no result after it, the plant may
still be in place: check client/src/lib.rs's sha256 against the start list.)
- Run: exit 0 (npt-client recompiled: the log's "Compiling npt-client" line 11). Every binary ok: client lib 2, bites 1,
  endpoint 9, gap 6, loopback 6 (70.09 s), pump 11, schedule 17.
- So an endpoint built with `with_bite("PM-0001-17/client-rounding")` that keeps the clean grid passes every test:
  the bite's call-site wiring in `ClientEndpoint` (carry 24's "`ClientEndpoint` takes the grid for every call it makes
  into `npt_sim::grid` from `npt_client::grid_with_bite`") has no test; the clean test and the part (i) twin call
  `Held::hashed` with a grid passed in (lib.rs:753-799), never an endpoint. TE-c2 confirmed (finding TE-3).
- Restored by the reverse Edit; client/src/lib.rs bd83870c...d9cfd9 = start; all 23 OK.

**P-D (TE-c4: each group decoding alone)**, my own form, client/src/lib.rs, in `group()` after the
`let Some(base) = base else { ... };` block. Old: `            return;` / `        };` /
`        let mut tick = core::mem::take(&mut self.ring[i]);`. New: the same first two lines, then
`        if self.ring[i].got & ((1u8 << g) - 1) != (1u8 << g) - 1 {` / `            self.refused += 1;` /
`            self.sentinel(t);` / `            return;` / `        }`, then the `let mut tick` line (a client that decodes a
group only once every lower-numbered group of its tick has: in-order decoding, stronger than the main session's
group-0 form of the cut run). Command as P-B's; log r1-te-pd2-in-order.log. (Planted before the run.)
- Run: exit 101; npt-client recompiled (log line 11). One FAILED, by name:
  `e2_a_forced_split_decodes_group_by_group_and_the_mirror_equals_the_clients_hash`, panicked at loopback.rs:145:5
  "a tick with a lost group is never filed" (after "e2: 3024 split ticks, 3023 compared equal": group 2 of the dropped
  tick 330 arrives after the missing group 1, the plant refuses it and files the sentinel). Every other binary ok
  (client lib 2, bites 1, endpoint 9, gap 6, loopback 5 of 6, pump 11, schedule 17). So e2 catches in-order decoding.
- Restored by the reverse Edit (the five planted lines removed); all 23 OK.

**P-D' (TE-c4, the group-0 form)**, the same site. New: `        if g > 0 && self.ring[i].got & 1 == 0 {` /
`            self.refused += 1;` / `            self.sentinel(t);` / `            return;` / `        }` (a group above 0
refused unless group 0 of its tick has arrived: the case "each group decoding alone" names that e2's drop of group 1
never reaches). Command as P-B's; log r1-te-pd3-group0.log. (Planted before the run.)
- Run: exit 0; npt-client recompiled (log line 11); every binary ok (client lib 2, bites 1, endpoint 9, gap 6,
  loopback 6 (68.41 s), pump 11, schedule 17). The cut run's log r1-te-pd-group-alone.log gives the same, for the
  same plant.
- So a client that cannot decode a group above 0 while group 0 of its tick is missing (it files the sentinel where
  the tick should stay pending, then be resolved lost) passes every test. e2 drops group 1 only (loopback.rs:120-122)
  and e3 drops group 1 of 2 (endpoint.rs:122); no test delivers a later group with group 0 lost. The plan's "a forced
  group split, each group decoding alone" (§5 U4e Tests) is met for "group 2 without group 1" (P-D) but not for "group
  1 without group 0" (P-D'). TE-c4 confirmed in part (finding TE-4).
- Restored by the reverse Edit; all 23 OK.

**P-H (TE-c8: carry 61 (b), "two different hashes for one tick give the sentinel")**, client/src/lib.rs, two Edits as
one plant (both routes to a second, different hash switched off). Edit 1, in `file()`: old
`            Filed::Hash(h) if h != hash => {` / `                self.ring[i].filed = Filed::Sentinel;` /
`                self.out.hashes.push((t, 0));` / `            }`; new `            Filed::Hash(h) if h != hash => {}`.
Edit 2, in `group()`: old `            } else {` / `                self.refused += 1;` / `                self.sentinel(t);` /
`            }` / `            return;` (the re-received group with other bytes); new `            } else {` /
`                self.duplicates += 1;` / `            }` / `            return;`. Command as P-B's; log
r1-te-ph-second-hash.log. (Planted before the run.)
- Run: exit 0; npt-client recompiled (log line 11); every binary ok (client lib 2, bites 1, endpoint 9, gap 6,
  loopback 6 (74.94 s), pump 11, schedule 17).
- So a client that keeps the first hash of a tick and drops a second, different one (by either route) passes every
  test: carry 61 (b)'s "two different hashes for one tick give the sentinel" has no test (the carry's own unit test
  asks only for the reorder and the duplicate, which carry_61_... meets: endpoint.rs:157-203). TE-c8 confirmed
  (finding TE-6, Low: the carry's named unit test is there; this clause has none).
- Restored by the two reverse Edits; all 23 OK.

**P-G + Q-6 + Q-7, one run (each read by its own test's name).**
- P-G (TE-c5, carry 50's room wiring), room/src/room.rs:943. Old:
  `            c.counters.events_expired += u64::from(c.queue.expire(t));`. New:
  `            c.counters.events_expired += u64::from(t & 0);` (the room never expires an event; the encoder's
  `t.wrapping_sub(q.tick) as u8`, room.rs:1170, then codes an age of 256 or more modulo 256).
- Q-6 (settles SEC-U4E-4), room/tests/room.rs: after the file's last line `}` (the end of
  `pm_0001_22_the_never_ackers_headroom_never_goes_below_0`), a `mod te_probes` with
  `te_probe_q6_an_acked_resync_is_answered_again_at_once`: `Scene::stress()`, `world_room(&scene, 1, 100, 1)`, one
  acked tick, then 30 times { `receive(Resync)`; `tick()`; collect; count and encode the full state if one came;
  `ack.take`; `receive(Input(ack.input(vec![])))` }; prints the count, the bytes and the counters; asserts at least 25
  full states in the 30 ticks.
- Q-7 (settles SEC-U4E-7), client/tests/endpoint.rs: after the file's last line `}` (the end of `mod bites`), a
  `mod te_probes` with `te_probe_q7_four_incomplete_groups_2_pow_31_apart_overflow_the_lost_run`: inside
  `catch_unwind`, `joined(endpoint(), 100)`, then one `feed` of four `Snapshot { tick: 101 + k * (2^31 - 1), group 0,
  groups 2, .. }` (k = 0..3); prints the outcome; asserts it panicked.
- Command: `cargo test --locked --all-features --no-fail-fast -p npt-room -p npt-server -p npt-client -- --nocapture`
  (the room's, the server's and the client's tests: every test that runs a room). Log r1-te-pg-q6-q7.log. (Planted
  before the run.)
- Run: exit 0; npt-room, npt-client and npt-server recompiled (log lines 11, 12, 31). Every binary ok: client lib 2,
  bites 1, endpoint 10 (9 + Q-7), room lib 20, room bites 1, catalogue 1, room 23 (22 + Q-6), gap 6, loopback 6
  (68.70 s), pump 11, schedule 17 (its three panics are its `should_panic` tests).
- P-G: with the room's `expire` call gone, every test passes: carry 50's room half (the room drops an event at age
  256 from the resend and counts it) is proven only by the queue's unit test (events.rs:137-145), which calls
  `expire` itself. Carry 50 asks for "A unit test", which is there; the brief's e12 "an event at age 255 is still
  sent, none at 256" is shown for the queue, not for what the room sends. With the call gone the encoder's `as u8`
  (room.rs:1170) would code a 256-tick-old event as age 0 and no test would see it. TE-c5 confirmed (finding TE-5,
  Low).
- Q-6 printed "Q-6: 29 full states in 30 ticks, 256418 B; full_states 1 -> 30; resyncs 29; dropped 1": SEC-U4E-4
  holds by run. A client that acks each full state and asks again gets about 29 full states a second, 256,418 B
  (about 8.8 KB each), 8.0 x the 32,000 B a second the window allows, none of it counted against the window.
- Q-7 printed "Q-7: panicked: None Some("attempt to add with overflow")" at client/src/lib.rs:526:13: SEC-U4E-7
  holds by run (a hostile server's four incomplete groups 2^31 - 1 ticks apart panic the debug client).
- Restored: room.rs (src) by the reverse Edit, room/tests/room.rs and client/tests/endpoint.rs by removing
  `mod te_probes`; all 23 OK.
- SEC-U4E-5 is not settled by run here: the room's `receive(slot, msg)` (room.rs:682) has no session argument, so the
  room-side half holds by reading; the caller half (the lab ignores `replaced`, common/mod.rs:238-249) is U4r's and
  U6's code, and a lab test would test the lab, not U4e. Said so; not run.

**Step 2, part (ii) read (s01..s15):** each log is one `-x`, `--all-features` run by name in the bites build, and shows
exactly the clean test named in the builder's record FAILED, with a panic value that is the bite's own defect (s01 "F
363 coded at age Some(34)"; s02 "no fresh delta within 500 ms"; s03/s04 "Some(2768209054929083115) None"; s05/s06
left (0,1,1) / (1,0,1) against (1,1,0); s07 left 7920; s08 Some(131071); s09 two hashes; s10..s12 (1,0,0) / (0,24,0)
/ (0,0,34); s13 "232 own shots sent"; s14 -234; s15 "client 0's full state holds Soldier 1 gen 0"). The switch texts are
not recorded, but the values tie each failure to its bite; I accept them, with two limits found by my own plants:
s07 shows the floor grid at `Held::hashed`, not the endpoint's wiring (TE-3); s03/s04's failure is "the client files
nothing", the silent drop (TE-1, with NC-H1). Part (i): b11-counted1/2 both "bites build: 524 tests passed, 0 failed",
and b11-list.log names each twin; so each twin passes and no clean test fails in the bites build.

**P-21 + P-22 + P-57, one run (three independent sites, each read by its own test's name).**
- P-21, room/src/room.rs:835. Old: `        canonical(&mut self.input.commands);`. New:
  `        let _ = &mut self.input.commands;` (the room steps commands in arrival order).
- P-22, room/src/rewind.rs:10. Old:
  `        && (r.fraction == 0 || history.get(r.tick_a.wrapping_add(1)).is_some())`. New:
  `        && (r.fraction == 0 || r.fraction > 0)` (`rewind_at` names `tick_a + 1` whether or not the ring holds it).
- P-57, room/src/room.rs, route_events. Old: the comment line
  `                    // A second overflow inside an unacknowledged forced resync: Kick(overflow), the client dropped.`
  then `                    c.kick(&mut self.roster);`. New: the comment line, then `                    c.queue.clear();`
  (a second overflow clears the queue and never kicks).
- Expected to fail by name: `carry_21_commands_received_in_two_orders_give_one_tick_input`,
  `carry_22_rewind_at_never_names_tick_n_plus_1_and_accepts_every_held_view_time`,
  `carry_57_a_never_acker_overflows_once_then_a_second_overflow_kicks`. Command `cargo test --locked --all-features
  --no-fail-fast -p npt-room`; log r1-te-p21-22-57.log. (Planted before the run.)
- Run: exit 101; npt-room recompiled (log line 7). room lib 20 ok, room bites 1 ok, catalogue 1 ok, room "FAILED. 20
  passed; 2 failed": `carry_22_rewind_at_never_names_tick_n_plus_1_and_accepts_every_held_view_time` (room.rs:293:5,
  left `Some(Rewind { tick_a: 1019, fraction: 1, cover: Present })`, right None) and
  `carry_22_the_room_counts_each_firing_command_its_ring_cannot_rewind` (room.rs:338:5, left 0, right 1).
- P-22: caught twice. P-21: not caught. `carry_21_commands_received_in_two_orders_give_one_tick_input` passes with no
  sort, because `fill_input` (room.rs:796-834) builds the list in slot order, one command per slot under option B,
  so it is already canonical; the sort's case (two commands of one slot, ticks across the wrap) arises only under
  option A (U4f). The function is pinned by its unit test (room.rs `carry_21_two_arrival_orders_sort_...`), and its
  call site by reading only (finding TE-8, Low, a carry for U4f). P-57: not caught.
  `carry_57_a_never_acker_overflows_once_then_a_second_overflow_kicks` passes with the second-overflow kick gone,
  because the never-acker is also kicked by the forced full state's timeout (room.rs:1014-1019). Probe Q-9 below.
- Restored: the three reverse Edits; all 23 OK.

**Q-9 (P-57's follow-up, clean build)**, room/tests/room.rs: after the file's last line `}`, a `mod te_probes` with
`te_probe_q9_which_route_kicks_the_never_acker`: the setup of carry_57_a_never_acker_... (stress, seed 4, tick 400,
8 slots, cap 48, slot 0 never acks), 300 ticks, recording the first overflow-resync tick f and the tick k of the first
`Kick::Overflow` to slot 0; prints f, k, f + FULL_STATE_ACK_TICKS + 1 and the overflow-resync count; asserts k is
Some. Command `cargo test --locked --all-features --no-fail-fast -p npt-room --test room -- --nocapture te_probe`;
log r1-te-q9-kick-route.log. (Probe in place before the run.)
- Run: exit 0, "test result: ok. 1 passed; 0 failed; ... 22 filtered out". Printed "Q-9: first overflow resync f
  Some(434); kick k Some(459); f + window + 1 = Some(495); overflow resyncs 1".
- So in the clean build the second-overflow route kicks (tick 459, before the timeout's 495); with that route off
  (P-57) the timeout kicks at 495, inside the test's 300 ticks (400..700), and the test's
  `assert!(kicked && c.kicked() && c.counters().kicked == 1)` (room.rs:889) passes either way: the test named for the
  second-overflow edge does not pin its route. Brief e12's carry 57 "as two tests, a second overflow before the
  FullState's ack, and no ack within FULL_STATE_ACK_TICKS, each sends Kick::Overflow ... (both edges)": the first
  edge's test cannot fail when that edge is broken. Finding TE-7, Medium (a one-line fix: assert the kick's tick is
  below f + FULL_STATE_ACK_TICKS + 1, or count the two routes apart).
- Restored by removing `mod te_probes`; all 23 OK.
- NC-H1's own cheapest proof (netcode notes :216-217: `lab.compare()` and slot 0's keys in the tick16 twin) is what
  Q-1 ran: compared 30, bad 0, no key at or past 65,536 and none at 0..30. NC-H1 holds by run.

**P-23 + Q-10, one run (two crates, each read by its own test's name).**
- P-23 (settles RR-10), room/src/room.rs, route_events. Old: `                            gameplay.push(Queued {` /
  `                                number,` / `                                tick,`. New: the same first two lines,
  then `                                tick: tick.wrapping_add(1),` (each queued event carries the wrong tick; the room's
  `stamped` list stays right). Expected to fail if the queue half is not vacuous:
  `room::tests::carry_23_events_carry_their_tick_and_consecutive_32_bit_numbers_across_a_16_bit_wrap`.
- Q-10 (settles NC-M5), client/tests/endpoint.rs: after the file's last `}`, a `mod te_probes` with
  `te_probe_q10_event_numbers_far_from_the_clients_start_rebuild_wrong`: `joined(endpoint(), 100)` (event start 0),
  one snapshot of tick 101 with events numbered 200,000 and 200,001; prints the filed hash against `client_hash` of
  the right numbers; asserts they differ.
- Command `cargo test --locked --all-features --no-fail-fast -p npt-room -p npt-client -- --nocapture`; log
  r1-te-p23-q10.log. (Planted before the run.)
- Run: exit 101; npt-room and npt-client recompiled (log lines 7, 8). room lib "FAILED. 19 passed; 1 failed":
  `room::tests::carry_23_events_carry_their_tick_and_consecutive_32_bit_numbers_across_a_16_bit_wrap`, panicked at
  room.rs:1404:21 with `Queued { number: 65598, tick: 903, ... }`; every other binary ok (client lib 2, bites 1,
  endpoint 10, room bites 1, catalogue 1, room 22).
- P-23: caught, by the queue half: in this run client 0's queue held events, so RR-10's "can pass vacuously" did not
  happen here; the missing "at least one queued event was checked" assertion stays a Low robustness note (RR-10, as
  the rust-reviewer wrote it; settled: real, not biting today).
- Q-10 printed "Q-10: filed 8109853925905384090, right numbers' hash 16249022625642323684, equal false": NC-M5 holds
  by run (an endpoint whose event counter starts at 0 rebuilds events numbered 200,000 wrong, and the hash reads
  them).
- Restored: room.rs by the reverse Edit, endpoint.rs by removing `mod te_probes`; all 23 OK.

**Q-11 (settles NC-M4)**, server/tests/gap.rs: after the file's last `}` (the end of `mod bites`), a `mod te_probes`
with `te_probe_q11_snapshots_after_the_gap_carry_events_before_entities`: gap_case's clean AfterSend setup (stress,
4 clients, gap of ACK_RING_TICKS periods), recording per tick client 0's (skipped, bytes, groups, events_sent,
events_resent, baseline_age); prints ticks F-2..F+7; asserts F was recorded. Command `cargo test --locked
--all-features --no-fail-fast -p npt-server -p npt-client --test gap -- --nocapture te_probe`; log
r1-te-q11-nearest8.log. (Probe in place before the run.)
- Run: exit 0, "test result: ok. 1 passed; 0 failed; ... 6 filtered out". Printed, (skipped, bytes, groups, sent,
  resent, age): 361 (401, 1026, 1, 40, 416, 0); 362 (401, 1026, 1, 40, 439, 0); 363 F (408, 1026, 1, 42, 462, 0); 364
  (362, 1027, 1, 42, 462, 1); 365 (320, ...); 366 (280); 367 (239); 368 (199); 369 (157); 370 (114), resent 462 from F
  on.
- So NC-M4's scenario does not occur in the gap run: the 462 resent events are cumulative over the gap's dropped
  snapshots (416 already at 361, about 23 a snapshot), F resends 23 and none are resent after it, and every snapshot
  from F on still carries entity changes (held count 408 -> 362 -> 320 ..., about 42 a tick, at the full 1,026 B).
  What holds by reading: events are packed before entities (room.rs:1164-1180), so a backlog larger than a
  snapshot's remainder would still starve entity changes, and no U4e test asserts the nearest-8 rule. NC-M4 partly
  settled (its figures, no; its order, yes; for the advisor with NC-M4).
- Restored by removing `mod te_probes`; all 23 OK; `git status --short` 24 lines (the 23 files and the builder's
  record), as at the start.

**Step 5: the counted check.mjs run (-x).** Command `node <copy>/spikes/netcode-proof/check.mjs`; log
r1-te-check-counted.log.

### Block 5. Findings, ranked (each with its proof run above)
These numbers supersede the ids written inline in Block 4: P-H's "TE-6" is TE-7, P-G's "TE-5" is TE-8, P-21's "TE-8"
is TE-9, Q-9's "TE-7" is TE-6; P-B's TE-3 and P-D''s TE-4 stand.
- **TE-1, High, before merge** (with NC-H1; settled by Q-1). server/tests/loopback.rs:241-252 (the tick16 twin),
  client/src/lib.rs:466-471 and :620-625 (`too_old`, the stale drop before `see`). Under PM-0001-17/tick16 or
  PM-0001-58/tick16 the client files no hash, no sentinel and no lost record for any post-wrap tick (Q-1: "client 0
  post-wrap keys []; server post-wrap hashes 14; compared 30, bad 0; sentinels 0; stale 14; lost []"). The twin
  asserts `server.is_some() && server != client` with client None, which a silent drop meets; brief 2.6's row asks
  that the bitten client log the frame "under 1 or as the sentinel" and §4.14 "hashes differently or is refused".
  So the part (i) proof does not show what the judged check counts: its figure reads 0. Evidence that does not hold.
  Fix: NC-H1's (the advisor rules the client's form); the twin then asserts an orphan client hash or the sentinel.
- **TE-2, Medium, before merge** (P-A). server/tests/loopback.rs:161-199 (e6); room/src/room.rs:1117. An encoder
  that packs against the whole per-tick budget, ignoring the pump's bytes, passes every test: e6's `left == budget -
  p` reads the room's own pair and `bytes <= left` never binds (4-client SS snapshots of about 147 B against about
  1,029 B). The plan's "the window counting the pump's bytes" is not proven. Fix: run e6 where the remainder binds
  (48 clients, or a pump report large enough), and assert at least one snapshot that would have exceeded `left` with
  the pump's bytes ignored.
- **TE-3, Medium, before merge** (P-B). client/src/lib.rs:292-294 (`with_bite`), :753-799 (the tests). An endpoint
  built with `with_bite("PM-0001-17/client-rounding")` that keeps the wire's grid passes every test: the clean test
  and the twin call `Held::hashed` with a grid passed in, never an endpoint; s07 shows the floor grid, not the
  endpoint's wiring. §5 U4e's Bites line "client-rounding (the client's call site)" and carry 24's first clause have
  no test. Fix: a bites-only twin that feeds one decoded snapshot to a clean and a bitten endpoint and shows the
  filed hashes differ.
- **TE-4, Medium, before merge** (P-D', with P-D as its control). server/tests/loopback.rs:102-156 (e2),
  client/tests/endpoint.rs:120-150 (e3). A client that refuses a group above 0 while group 0 of its tick is missing
  (filing the sentinel where the tick should stay pending, then lost) passes every test; e2 drops group 1 only. P-D
  (in-order decoding) is caught by e2 at loopback.rs:145. The plan's "each group decoding alone" holds for "group 2
  without group 1", not "group 1 without group 0". Fix: e2 (or e3) also drops group 0 of a split tick and asserts the
  other groups are acknowledged and the tick is lost, never the sentinel.
- **TE-5, Medium, before merge** (P-C). server/src/pump.rs:159; server/tests/pump.rs. With the pump's `TooLarge`
  counted as `truncated`, every U4e test passes; no pump test sends an oversized datagram or stream message. The
  advisor's U4E-Q11 (a): "U4e builds the server-side checks (size, trailing bytes, wrong channel, the bucket) and
  their unit tests". Fix: Q-5's test (an Input of `Limits::INPUT + 1` B: too_large 1, nothing out).
- **TE-6, Medium, before merge** (P-57, Q-9). room/tests/room.rs:868-895. In the clean build the second overflow
  kicks (tick 459; the timeout would at 495); with that route off the timeout kicks at 495 inside the test's 300
  ticks and the test passes: brief e12's carry 57 edge "a second overflow before the FullState's ack ... sends
  Kick::Overflow" cannot fail. Fix: assert the kick's tick is below the first resync's tick + FULL_STATE_ACK_TICKS + 1.
- **TE-7, Low** (P-H). client/src/lib.rs:548-561, :633-640. Carry 61 (b)'s "two different hashes for one tick give
  the sentinel" has no test: a client keeping the first hash passes. The carry's named unit test (reorder, duplicate)
  is there. Fix: feed one group twice with different bytes; assert the sentinel.
- **TE-8, Low** (P-G; RR-2). room/src/room.rs:943, :1170. The room's `expire` call is pinned by no test (the queue's
  unit test calls `expire` itself); with it gone the encoder's `as u8` codes a 256-tick-old event as age 0 and every
  test passes. Carry 50 asks "A unit test", which is there.
- **TE-9, Low, a carry for U4f** (P-21). room/tests/room.rs:222-268; room.rs:796-835. Carry 21's room test passes
  with no sort: under option B the list is built in slot order. The sort's own case comes with option A.
- **TE-10, Low.** client/tests/endpoint.rs:109-114: the 1,150 B case pads a valid snapshot with zero bytes, so the
  trailing-bytes refusal refuses it too and the test passes with no size check. Fix: a datagram whose declared
  content fills 1,151 B, or assert the refusal's kind.
- **RR-10 (the rust-reviewer's), settled:** real, not biting today (P-23 is caught by the queue half).

### Block 5b. Claim, proof, gap (§5 U4e's Tests, negative cases, -18 end to end, bites; brief e1..e12)
| Claim | Proof (test; plant that it catches) | Gap |
|---|---|---|
| e1 layout = Field-group rows, no authority-only group | room/tests/catalogue.rs | none found |
| e2 forced split, each group alone | loopback.rs e2 (catches P-D) | group 1 without group 0 (P-D' passes): TE-4 |
| e3 lost group unlogged and counted; refused one the sentinel | endpoint.rs e3 | none found |
| e4 baseline rule, age 31 kept / 32 evicted, partial ack never a baseline | ack.rs e4 (:226-241) | none found |
| e5a/e5b gap evicts B; first snapshot after it from defaults | gap.rs (s01 catches -18) | NC-L2's age-32 edge not met (not run) |
| -18 end to end | gap.rs pm_0001_18_a_fresh... (s02) | none found |
| e6 window counts the pump's bytes | loopback.rs e6 | cannot fail (P-A passes): TE-2 |
| e7 mirror = client hash, P0, 1,000 ticks | loopback.rs e7 | resent clause moved to the gap test (stated) |
| e8 Send asserts | room.rs, pump.rs, endpoint.rs | none (compile-time) |
| e9 negatives (Join reasons, pre-Join, bucket, Ping, deadline, Resync, wrong channel, room checks) | pump.rs, roster.rs, input.rs, room.rs, endpoint.rs | size check through the pump untested (P-C): TE-5; 1,150 B case not size-specific: TE-10 |
| e10 F2 pins at 30 and 60 Hz | encoder.rs, pump.rs | RTT counts the pump's wait (NC-M3, Q-2) |
| e11 bitten and clean on two threads | gap.rs bites::e11 | none found |
| carry 21 | room.rs unit + room test | room test cannot fail at option B (P-21): TE-9 |
| carry 22 | room tests (catch P-22 twice) | none |
| carry 23 | room.rs unit (catches P-23) | vacuity guard missing (RR-10, not biting) |
| carry 24 (endpoint takes the grid from grid_with_bite) | none at the endpoint | P-B passes: TE-3 |
| carry 50 | events.rs unit | room's `expire` call unpinned (P-G): TE-8 |
| carries 51, 53, 56 | room test (s15 catches owner-to-all) | none found |
| carry 54 (and 61 (a)) | room test | none found |
| carry 57 | two room tests | second-overflow edge cannot fail (P-57, Q-9): TE-6 |
| carry 61 (b)-(d) | endpoint.rs carry_61 | (b)'s "two different hashes give the sentinel" (P-H passes): TE-7 |
| tick16 bites | loopback.rs twin and clean test (s03/s04) | silent drop, judged figure 0 (Q-1): TE-1 |
| client-rounding bite | client lib.rs tests (s07) | the endpoint's wiring (P-B): TE-3 |
| lt-ack, lt-cmd-seq, lt-press, lt-tick-offset, lt-event-seq, server-echo, unbounded, owner-to-all, no-trailing, no-channel | twins pass (counted 524/0); s05, s06, s08-s15 | none found |

### Block 6. Other reviewers' findings settled by run
NC-H1 holds (Q-1). NC-M3 holds (Q-2: RTT Some(120,000,000) against 100,000,000 at arrival). NC-M4 partly (Q-11: its
figures no; its order yes). NC-M5 holds (Q-10). SEC-U4E-1 holds (Q-3). SEC-U4E-3 holds (Q-4). SEC-U4E-4 holds (Q-6: 29
full states in 30 ticks, 256,418 B, about 8 x the window). SEC-U4E-7 holds (Q-7: the overflow panic at lib.rs:526).
RR-2 (P-G) and RR-10 (P-23) as above. Not settled by run: SEC-U4E-5 (the room half holds by reading, the caller half
is U4r's and U6's), NC-M6, NC-L2 (a gap one period shorter; not run).

