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

