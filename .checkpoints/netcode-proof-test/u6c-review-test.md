# U6c's review round: the test seat (test-engineer, xhigh). Read only.

Inputs checked by sha256 (2026-11-24): brief 8f56f37a…, advisor check 9f5ab8d0…, launch message bf8016a8…, snapshot
patch a4e63455…, builder notes 426b31da…: all five equal the launch's. W = .claude/worktrees/agent-a3a3e3fdf559de095/spikes/netcode-proof.
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
  one `#[ignore]` (tests/load.rs); none returns early or prints `SKIPPED`; 807 + 52 = 859 and 904 + 52 = 956 as
  logged. C7's +52 / +52 is met exactly, with no margin.
- N2 (plants): P1-P7, P9-P21 each fail their named test at the line and value the notes give (plant-p*.log read);
  p14.log is a compile error (E0658), so p14b.log is P14's run; P5's run fails at net probe_tests.rs:211 with :210
  passing (n = 1,000), which shows P4's plant was out of P5's run; P18 fails at server probe_tests.rs:226
  (`ran.expect`, 2.20 s).
- F-P8 (Medium): the end-to-end test's bound check (W/crates/server/src/probe_tests.rs:258-262, `udp_row_found == 1`
  at `after`) cannot fail on P8: ImpairSocket's Drop (W/crates/net/src/l2/socket.rs:466-471; :440-446 at base) only
  signals; the kernel socket's fds live in the reader thread (socket.rs:586-617) and in `Shared.send_sock` until both
  threads exit, so a read just after `drop(sock)` (W/crates/server/src/probe.rs:564) still finds the row. The test's
  doc (probe_tests.rs:177) still calls it "P8's test".
- F-P2 (Medium): P2 was planted in the test body; a shipped-code P2 is expressible (bind once in `run_probe`, carry
  the socket into each `run_leg`, `set_default`/`set_peer` per leg: socket.rs:244, :251) and no test would fail on it:
  the end-to-end test asserts none of P1's whole-life `l2_*` counts (`run_summary`, socket.rs:394-413), and P1's
  `probe_*` fields come from a fate log drained at P0's end, so they would not mix.
