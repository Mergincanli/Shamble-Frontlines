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

