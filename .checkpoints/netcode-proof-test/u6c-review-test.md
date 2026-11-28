# U6c's review round: the test seat (test-engineer, xhigh). Read only.

Inputs checked by sha256 (2026-11-24): brief 8f56f37a…, advisor check 9f5ab8d0…, launch message bf8016a8…, snapshot
patch a4e63455…, builder notes 426b31da…: all five equal the launch's. W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-a3a3e3fdf559de095/spikes/netcode-proof.
(No Edit tool in this launch: this file is rewritten whole with Write.)

## 1. Plan
1. Tie the counted run to the snapshot: end-check2.log's tree hash and counts against the patch (base 807/904).
2. Count the new tests per block against section 3 and C7 (+52 measured, +52 bites); check none is ignored or
   cfg-gated out of the bites build, and that each test's name matches what it asserts.
3. Read each plant's log (P1-P21, p14b): it ran the named test, failed at the claimed line, and the restore hashes.
4. Read each named test against its plant: can it fail; does it fail for the claimed reason; timing dependence.
5. The fake directory (issue_tests.rs): what Block 6's nine plus C1 (a) cover; the shipped path's reach.
6. F-JOIN-ONCE (launch item 4): the glue's Open-only step, Closed counted, step at once; the tests' no-Join-deadline
   assert; whether a plant of each would fail.
7. The five items: F-P8, P2, P14/P18/P4-P5, F-REDACT, F-SIZE-1/F-FMT-1.
8. Runs only where a claim needs one, through the runner.

## 2. Notes (one line per finding, as found)
- N0 (provenance, no finding): the worktree's 24 files equal the snapshot rebuilt from 8c81952 plus the patch; the
  snapshot's `git diff HEAD -- .` hash 80fb60a7… and untracked hash 5d7558ed… equal end-check2.log's line 3, so the
  counted run (859 / 956, all 11 steps PASS) ran on exactly the snapshot. The seven planted files' sha256 equal the
  notes' pre-plant hashes.
- N1 (counts, no finding): 52 new tests by name (Block 1 10, Block 3 8, Block 4 10, Block 5 11, Block 6 13) plus A2's
  one `#[ignore]` (tests/load.rs:289-291); none returns early or prints `SKIPPED`; 807 + 52 = 859 and 904 + 52 = 956 as
  logged. C7's +52 / +52 is met exactly, with no margin.
- N2 (plants): P1-P7, P9-P21 each fail their named test at the line and value the notes give (plant-p*.log read);
  p14.log is a compile error (E0658), so p14b.log is P14's run; P5's run fails at net probe_tests.rs:211 with :210
  passing (n = 1,000), which shows P4's plant was out of P5's run; P18 fails at server probe_tests.rs:226
  (`ran.expect`, 2.20 s). T1, T2, T5 and the self-test import the shipped pieces (socket_tests.rs's copies removed);
  P1 on the shipped poller fails T1, which confirms the re-run. The promoted `in_e_set` matches the removed copy.
- F-P8 (Medium): the end-to-end test's bound check (W/crates/server/src/probe_tests.rs:258-262, `udp_row_found == 1`
  at `after`) cannot fail on P8: ImpairSocket's Drop (W/crates/net/src/l2/socket.rs:466-471; :440-446 at base) only
  signals; the kernel socket's fds live in the reader thread (socket.rs:586-617) and in `Shared.send_sock` until both
  threads exit, so a read just after `drop(sock)` (W/crates/server/src/probe.rs:564) still finds the row. The test's
  doc (probe_tests.rs:177) still calls it "P8's test". The shipped order is right (probe.rs:549 before :564).
- F-P2 (Medium): P2 was planted in the test body; a shipped-code P2 is expressible (bind once in `run_probe`, carry
  the socket into each `run_leg`, `set_default`/`set_peer` per leg: socket.rs:244, :251) and no test would fail on it:
  the end-to-end test asserts none of P1's whole-life `l2_*` counts (`run_summary`, socket.rs:394-413), and P1's
  `probe_*` fields come from a fate log drained at P0's end, so they would not mix.
- F-A2-EXACT (Medium): A2's named command (builder notes "A2's command"; W/crates/bots/tests/load.rs:288) ends in
  `--ignored --nocapture --exact` with the filter `a2_`; libtest's `--exact` matches whole names, so it selects no test
  and exits 0. Shown on the snapshot: rev-test-1.log (`a2_ -- --ignored --exact --list`: "0 tests"), rev-test-2.log
  (without `--exact`: "a2_48_websocket_clients_at_p2_against_one_room: test", "1 test").
- F-TIMING (Medium, three sites): real-clock rate bars in gate tests on 4 vCPUs, each failing on one scheduler stall:
  (a) W/crates/server/src/probe_tests.rs:313-319, both window counts in 1,800..=2,100 for a 1 s window at 2,000 a
  second (a 50 ms stall of RS's reader thread at the window's start pushes up past 2,100; 100 ms at its end pulls
  either under 1,800); (b) W/crates/bots/tests/load.rs:260-265, `probe_peer_sent_count >= 9_000` for 1 s at 10,000 a
  second, the peer stopped at `duration_s` (load.rs:602-607), beside two rooms and two loads in the same test binary;
  (c) W/crates/net/src/l2/probe_tests.rs:439-449, ten up reads after `down(8); down(9)` must hold an echo: a 50 ms stall
  of LG's reader, or of the test thread before `down(8)`, gives ten old datagrams and `echoes == []`.
- F-READINGS (Medium): section 3's container readings (u6c-brief.md:353-358) are not built and the notes do not say
  so: no W/crates/server/examples/ (the probe at 10,000 a second each way for 100 s, C-U6-6's loopback check), and the
  48-client reading is WebSocket only (A2), not "on each transport".
- F-JOIN-4 (Low): F-JOIN-ONCE (4) rests on READY alone, and soundly (a session closed at the Join deadline never gets
  JoinAccepted); the second assert, `deadline_closed_count == 0` (tests/load.rs:203-210), is the transport's handshake
  deadline (W/crates/net/src/session.rs:167-168, :187), not the room's Join deadline (W/crates/server/src/pump.rs:383).
- F-JOIN-3 (Low): F-JOIN-ONCE (3) "at once" is checked in polls, not time (load_tests.rs:242); a sleep after Open passes
  it; the 2 s Join deadline itself is held end to end by READY in `hosts_four`.
- F-FAILED (Low): `HostState::failed()` (W/crates/bots/src/load.rs:94-97) is read only by the glue test; `run_load`
  never counts or logs a failed handshake.
- F-HASH-PLUS (Low): serve.rs:199-202 takes NN by `len() == 2 && parse::<u8>()`, which accepts a leading `+`, so
  `/npt/cert-hash-+5.json` reads `room-+5.json` (no traversal) instead of a 404 with no read; not in the 404 list.
- F-NO-PEER (Low): no test of a leg with no peer datagram within `peer_wait_ns` (probe.rs:413-435) or of P0's no-echo
  end (probe.rs:521-522), C2's fail-closed rule; the binary test covers stdin's close only.
- F-FAKE-GAPS (Low): the fake directory never reads the CSR (issue_tests.rs:112-115) or the order's identifiers
  (:78-90), so the key on disk is not tied to the CSR's key or the name; the poll timeout's exit 4 (issue.rs:234,
  :246-252) is untested; INF-SB-6's temporary name and the key-before-chain order (issue.rs:272-275) are held by code
  order only (the test sees only that no temporary name is left).
- F-NAMES (Low): tests claiming more than they assert: `hosts_four` asserts each client's commands summed over the
  measured seconds > 0 (tests/load.rs:172-177), not "every measured second"; neither transport's test asserts how
  npt-load ended (by the close or by its duration: :214-222), so NC-SB-2's "footer on the server's close" is not shown
  to come from the close; the steal check is an `all` over seconds that may be empty (probe_tests.rs:266-271);
  section 3's test-local counting writer is not built (held by structure: `LegLog::write` after `run_leg`).
- F-5S (Low): `hosts_four("wt", 5)` (tests/load.rs:216) runs to its 5 s duration (b4-test3.log: the binary 5.15 s),
  over section 3's "real-clock tests at most 5 s each".
- F-STALE-BIN (Low): `room()` runs target/<profile>/npt-server (tests/load.rs:86-90) without checking it is this
  tree's; `cargo test --workspace` (the gate) rebuilds it, but a `-p npt-bots` run on the shared target may take
  another worktree's binary.
- F-SERIES-LOAD (Low): the SERIES.md cross-check (server probe_tests.rs:272-282) covers the server's leg logs only;
  npt-load's six `probe_peer_*_count` names (load.rs:606-614, written in tests/load.rs's probe test) have no SERIES.md
  row (W/marks/SERIES.md:920-921 names the event only).

## 3. Verdict
**APPROVE WITH FINDINGS.** No Critical or High; five Mediums, each with a fix of a few lines or a named reading, for
the fix pass or an explicit deferral (no second round needed under NPT-TRIM). The counted run is tied to the snapshot,
C7's counts hold exactly (+52 / +52), 20 of the 21 plants fail their named test with every restore hash checked, and
the fake directory covers section 3's nine items plus C1 (a). Two Mediums should land before U6c's close readings:
F-A2-EXACT (else A2's reading runs nothing and exits 0) and F-READINGS (section 3 binds; the readings come before
Stop B's read). P8 must be reported as not failing (20 of 21), not counted.

## 4. Findings table (claim · proof · gap)
| # | Sev | Claim | Proof in the build | Gap | Smallest fix | file:line (W) |
|---|---|---|---|---|---|---|
| F-P8 | Medium | End-edge reads with the socket still bound (C-U6-2's step, P8) | Code order (probe.rs:549 before :564); test asserts the row found | The test cannot fail on P8: the kernel socket outlives the drop (unjoined threads) | `edge_events(sock: &ImpairSocket, port, ph)`, both calls pass `&sock` (a read after `drop(sock)` is then E0382); record P8 as held by the compiler; reword probe_tests.rs:177 | server/src/probe.rs:176, :461, :549, :564; server/src/probe_tests.rs:177, :258-262; net/src/l2/socket.rs:466-471 |
| F-P2 | Medium | C-U6-2: one socket a leg, summaries never mix | Unit test on two ImpairSockets; P2 planted in the test body | A shipped-code P2 (one socket carried across legs) fails no test | In the end-to-end test assert P1's `l2_down_delivered_count` ≤ P1's `probe_down_sent_count` (a carried P0 socket doubles it); run P2 as a shipped-code plant (bind once in `run_probe`, `set_default` per leg) | net/src/l2/probe_tests.rs:121-164; server/src/probe.rs:456; server/src/probe_tests.rs:284-328 |
| F-A2-EXACT | Medium | A2's command takes the 48-client reading | None (not run by the builder) | `--exact` with `a2_` selects 0 tests, exit 0 (rev-test-1.log, rev-test-2.log) | Drop `--exact` (or give the full test name) in the doc and the notes | bots/tests/load.rs:288 |
| F-TIMING | Medium | Window counts, LG's send count, LG's echo of 8/9 | Real-clock bands | (a) 1,800..=2,100; (b) ≥ 9,000 of 10,000; (c) ten reads must hold the echo: each fails on one 50-100 ms stall on 4 vCPUs | (a) 1,500..=2,600, or below 0.65 × `probe_down_sent_count` (still tells window from whole leg); (b) ≥ 5,000; (c) read until echo 9 or 1 s, then assert [9] or [8, 9], then ten reads with none | server/src/probe_tests.rs:313-319; bots/tests/load.rs:260-265; net/src/l2/probe_tests.rs:439-449 |
| F-READINGS | Medium | Section 3's container readings before Stop B | A2 (WebSocket 48 clients) only | No probe example (10,000 a second, 100 s), no WebTransport 48-client reading; the notes do not say so | Name the command pair with the shipped binaries (npt-server probe settings `probe_legs` ["P0","P1"], `probe_packets` 1,000,000; npt-load probe settings), or add the example; a transport switch on A2 | crates/server/examples/ (absent); bots/tests/load.rs:289-338 |
| F-JOIN-4 | Low | F-JOIN-ONCE (4): no session ends at the Join deadline | READY (sound) | `deadline_closed_count` read as the Join deadline; it is the handshake deadline | Reword the doc and the notes; keep READY as the proof | bots/tests/load.rs:126-131, :203-210; net/src/session.rs:167-168; server/src/pump.rs:383 |
| F-JOIN-3 | Low | (3) step at once after Open | First send at poll 5-6 | Polls, not time | Record `now` at each send in the fake; assert the first send's `now` equals the Open poll's | bots/src/load_tests.rs:242 |
| F-FAILED | Low | (2) a failed handshake counted | `st.failed()` in the glue test | Nothing in the shipped path reads it | Count it in the load's footer event or SUMMARY, or reword "counted" | bots/src/load.rs:94-97 |
| F-HASH-PLUS | Low | Target matched exactly, NN 00-31 | 404 list | `+5` passes the NN check | `n.bytes().all(\|b\| b.is_ascii_digit())`; add `/npt/cert-hash-+5.json` to the 404 list | host/src/serve.rs:199-202; host/src/serve_tests.rs:209-220 |
| F-NO-PEER | Low | C2: no peer within the wait ends non-zero | None | Untested fail-closed path | `run_probe` with `peer_wait_ns` 200 ms and no peer: Err((1, …)), no leg log | server/src/probe.rs:413-435, :521-522 |
| F-FAKE-GAPS | Low | Key is the CSR's; the name; poll timeout exit 4; INF-SB-6 | Key parses as PKCS#8; no temp left | Unchecked | The fake records the finalize's CSR; assert the saved key's public key bytes occur in it and the order names NAME; one "pending" order past the poll's timeout gives 4 | host/src/issue_tests.rs:78-90, :112-115; host/src/issue.rs:234, :246-252, :272-275 |
| F-NAMES | Low | "every measured second", "ends at the room's close / its duration", "the steal each second", no window write | Weaker asserts | See notes | `all(v[i] > 0)` from the second measured second; assert the last second's index; `!seconds.is_empty()` | bots/tests/load.rs:172-177, :214-222; server/src/probe_tests.rs:266-271 |
| F-5S | Low | Real-clock tests ≤ 5 s | — | WT test runs 5 s plus start | `hosts_four("wt", 4)` | bots/tests/load.rs:216 |
| F-STALE-BIN | Low | Tests run this tree's npt-server | `cargo test --workspace` in the gate | `-p npt-bots` on the shared target may take another tree's | Compare the binary's mtime with npt-load's, or say so in the doc | bots/tests/load.rs:86-90 |
| F-SERIES-LOAD | Low | Every new name has a SERIES.md row | Cross-check over the server's leg logs | npt-load's `probe_peer_*` rows missing, unchecked | Six rows; extend the cross-check to load.jsonl in the probe test | server/src/probe_tests.rs:272-282; marks/SERIES.md:920-921 |

Ranked: F-A2-EXACT, F-READINGS, F-P8, F-TIMING, F-P2, then the Lows as listed.

## 5. My view on each item
- **F-P8.** The claim is true and the plant is valid; the test cannot see it. The claim (the end edge read while the
  socket is bound) holds in the shipped code (probe.rs:549 before :564). The plant is a fair P8. The test's
  observable, the /proc/net/udp row, outlives the drop: ImpairSocket's Drop (socket.rs:466-471) only signals, and the
  fds live in the reader thread and in `Shared` until both threads wake and exit. So no black-box test can tell a read
  before the drop from one just after it. Joining in Drop is U6b's code and would change C2's 2 s rebind. **Smallest
  fix:** hold the claim by type. `edge_events` takes `&ImpairSocket` (it can read nothing from it), both calls pass
  `&sock`, and the planted move then fails to compile (E0382). Record P8 as "held by the compiler" with that log, and
  reword probe_tests.rs:177. **Can U6c merge with it open?** Yes, as a Medium, fixed in the fix pass or deferred as a
  named STATUS follow-up. The code is right today, and a later regression would show as a rare missing row, which U8's
  presence check turns into an INVALID leg (fail-safe and visible). It must be reported as 20 of 21 plants, P8 not
  failing.
- **P2 (plant in the test body).** It proves half the claim. The unit test's assertion can fail: a reused socket reads
  3, not 0, so ImpairSocket keeps no state across instances and `set_default` resets nothing. It does not prove the
  leg runner's half (one socket per leg in `run_leg`, probe.rs:456). The builder is right that no shipped-code plant
  reaches the never-mix test, but a shipped-code P2 is easy to write: bind once in `run_probe`, then `set_default` or
  `set_peer` per leg. No test catches it, because the end-to-end test reads no whole-life `l2_*` count of P1, and P1's
  `probe_*` fields come from a log drained at P0's end. Fix: F-P2's one assert, then P2 again as a shipped-code plant.
- **P14 (p14b).** It counts. p14.log never compiled (E0658), so it is no evidence either way. p14b is the same defect
  in a form that compiles, run once, and it fails the named test (`/../Cargo.toml` 200, not 404). The 404 list also
  holds `/main.js/` and `/main.jsx`, so a subtler prefix plant that spares `/` would fail too.
- **P18 (fails at `ran.expect`).** It counts. The plant (any source's first datagram pins the peer, so the pacer starts
  before the peer is known) fails the named test by name. The failure follows from the defect: the stranger is
  pinned, LG's datagrams count as another source's, and P0 has no round trip. The stranger assert (probe_tests.rs:
  228-231) would also fail if reached, since the pacer would fill the stranger's buffer. Two independent asserts
  catch it, and the first fires.
- **P4/P5 under one hash.** Both count. P4's run was P4 alone (09:38:28). P5's run fails at :211 with :210 passing
  (n = 1,000), which shows P4's plant was out of it. The two plants touch disjoint lines, so a wrong restore of either
  would leave the final hash different from 00b3dca6…, and it is equal. The missed hash check between them is a slip
  in procedure, not a gap in the evidence.
- **F-REDACT's fix.** Still a real assertion, and stronger than the header match. `PrivateKeyDer::from_pem_slice` needs
  PEM framing with valid base64, and it gives `Pkcs8` only for a "PRIVATE KEY" section. A SEC1 key, raw DER, an empty
  file or the chain written to the key path all fail it. P17 and P21 ran on the fixed test (09:44-09:45, after
  redact-fix3.log at 09:36), and redact is at 0 findings. It does not check that the key is P-256 or that it is the
  CSR's key (F-FAKE-GAPS, Low).
- **F-SIZE-1 and F-FMT-1.** No test consequence: every plant (09:36-09:46) and the counted run (09:46-10:07) ran on the
  formatted tree after `cargo fmt --check` passed (09:06, fmt2.log). The valve fired on the formatted added count,
  5,426, past C4's 5,400. 199 lines were removed (189 of them T-code promoted into probe.rs), so the net is 5,232. Which
  count C4 means is the main session's ruling; as written ("added lines") the valve stands. Test code is about 2,260
  of the 5,431 lines. What the valve costs this seat: TE2-TE4 (C-U6-8's stamp tests) go with P2 into MOVED's brief.
  Until then R1's self-test reads stamps whose placement no test pins, as C4 ruled. U6c's report should name that as
  open.
- **F-JOIN-ONCE.** Covered in substance:
  - (1) and (2): the fake-link test (load_tests.rs:208-246). A step while Connecting sends the Join before poll 5, and
    a step on a Closed link sends at all; either fails it.
  - (3): checked in polls only (F-JOIN-3). The 2 s deadline is held end to end by READY.
  - (4): READY is a sound proof (F-JOIN-4 is wording only).
  - Nothing reports the failed-handshake count (F-FAILED).

## 6. What I did not verify
- I ran no plant, no full suite and no gate. My two runs were `--list` calls on the snapshot (rev-test-1.log,
  rev-test-2.log), for F-A2-EXACT.
- F-TIMING's flake risk is reasoned from the code and the 4 vCPUs, not observed: no failing run exists.
- The behaviour of rustls-pki-types' `from_pem_slice` and of Rust's unsigned `parse` (leading `+`) is from their
  documented behaviour, not from a run here.
- The fake directory's match with Let's Encrypt's real answers, and the shipped `Account::builder()` path (compiled
  under `cfg(not(test))`, never run), wait for R1's staging run.
- I took no container reading and did not run A2.
- The netcode and security claims (C-U6-1 to -7, the E fields' meaning, the pub items, the host's and the issue mode's
  security reads) are the other seats'.

DONE
