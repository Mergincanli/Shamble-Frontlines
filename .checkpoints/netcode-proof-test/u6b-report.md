# U6b's report: the transports (2026-11-24)
Brief u6b-brief.md rev 1.2 (7bc69f1a…; section 0.0 binds the advisor's ruling advisor-u6b-r1.md dc6973da… by line
range). Base 21765e3 (U6a's merge 824d8e7 plus Block 0; measured 754, bites 850). One round (NPT-TRIM): netcode,
security and test seats; the ruling (R-PING, A1-A3); the fix pass (u6b-fix-pass.md 031742e4…; notes u6b-build.md
74e0d697…); the advisor's check of the fix diff in place of a second round (A3): **APPROVE WITH CONDITIONS C1-C2**
(advisor-u6b-fix.md bf8da516…). At the merge, M5 failed on one test's cap; the advisor's M6 ruling (advisor-u6b-m6.md
f4100ef7…): **APPROVE WITH CONDITIONS C1-C7**, the cap fixed in binary.rs, the probe, M5 again.

## What was built, and its size
- **WebTransport:** WtServer, WtLink, WtClient over l2/wt.rs's endpoint, with the server's own QUIC limits (4
  bidirectional and 6 unidirectional streams, 64 KiB stream and 1 MiB connection windows). **WebSocket:** WsServer,
  WsLink, WsClient over DelayStream. **Shared:** one connection cap of N + 8 permits for both listeners (ConnCap), the
  certificate pin, the accept stamp at the first packet or the TCP accept, a 2 s deadline over every handshake, and one
  closing sequence under one 2 s bound (CLOSE_WAIT) for both transports.
- **The send path:** `Link::begin_tick` and TickLoop's per-tick batch; WsLink's gate (fix 1: a tick written all or none,
  its inputs read once per tick); outside a tick only a Ping or Pong of at most 32 B with nothing owed below the link's
  own queue, counted (R-PING).
- **The record:** the header's transport rows; `ws_split` with pooled lateness L, the reader stamp R and the window mark
  (on DelayStream's clock and the room's); one `ws_summary` per connection; one accept event per transport; the `open`
  event's serving stamps.
- **The binary:** settings, the dev certificate and its hash file, OPEN after both listeners serve, every closing ended
  before the log ends (`TickLoop::stop_with`); F28 (npt-super's log line-flushed).
- **Size: 5,952 added, 267 removed, 28 files** in the fix pass's snapshot (wip/u6b-fix.patch e29c003c…ac64af1c, base
  21765e3); the merge adds M6's 14 + 3 lines (binary.rs's cap, Cargo.toml's sha2 line): **5,969 added, 279 removed, 29
  files** as merged. The build came in at 5,498; A1's removal took 100; the fix pass added 554 (the Mediums and R-PING
  to 5,874, four Lows to 5,952). Each Low started inside the 5,950 scope line; his 6,000 stop was not reached.
- **Block 0** (before the build): the lock diff `1 65` (235 packages; ring only, no aws-lc-rs); cargo-deny "advisories
  ok, bans ok, sources ok" (five duplicate warnings); cargo-audit 0 vulnerabilities, one allowed warning
  RUSTSEC-2026-0441; both databases fetched 2026-11-23 07:00 UTC (dependency-check.md's U6b section; u6b-block0.md). The
  build touched no manifest or lock.
- **Lane** (the ruling's item 10): TickLoop's stop step (NR-U6B-1) joined U6b's lane beside the accept seam and the I/O
  send site; crates/super for F28 and T-19's plant (P20, restored).

## Evidence
- **Counts:** the builder's counted check.mjs on the fix-pass tree (.gate/u6b/fx-check2.log, 23:02-23:18 UTC), all 11
  steps PASS: measured 807 (754 +53), bites 904 (850 +54), 0 failed, 0 skipped; test step 848.5 s (0.89 of base's
  954.8 s). Condition 5 (at least +45 and +45, plus each test the fix pass adds, by name; T-20): the fix pass adds 8
  tests, named in advisor-u6b-fix.md item (5); 807 = 799 + 8; 904 = 898 − 2 + 8 (the two pm_0001_18_* tests left with A1).
  **M5 in the main folder, run 1** (wip/u6b-fix.patch applied, 28 of 28 files byte-equal to the builder's tree;
  .gate/u6b-merge/m5-check.log, 04:34:46-04:55:29 UTC): **FAIL** at the test step (1,013.6 s): the measured build 807
  passed, 0 failed; the bites build stopped at `bites::pm_0001_16_double_load_fails_every_start_under_2_s`, "no first
  line within the 5 s cap (Timeout): the room was killed" (binary.rs:82). Every other step passed. The cause (the M6
  ruling): TR2-U6A-4, below, not U6b's code. **The fix** (C1, the main session, binary.rs only, +14 -12;
  .gate/u6b-merge/m6-hunk.diff c6bfe8a9…): `OPEN_CAP` (5 s) for every start's own work, `start()` takes its cap, the
  bitten start gets `OPEN_CAP` plus the bite's own 2 s; no assert changed. **The probe** (C3, m6-probe2.log,
  CARGO_INCREMENTAL=0): every test passed, but the start's own work was 2,908-3,050 ms (s8a 2,957.8, 3,018.9, 2,984.0
  ms; the bitten start 4,908-5,050 ms less its 2 s), alone as slow as in parallel, over the advisor's 2,500 ms bar (half
  the 5 s cap). The host had changed: "Xeon @ 2.10GHz" now against fx-starts.log's "@ 2.30GHz" (when, not shown). The
  cause: the start hashes its own executable (main.rs:368-370), 116,112,832 B in a debug build, with sha2 unoptimised
  (the dev profile optimised only rand_chacha and ppv-lite86). **The second ruling** (advisor-u6b-m6.md section 5):
  measure first. **R1** (m6-probe3.log, sha2 at opt-level 3 by `--config` only): s8a 120.1-120.5 ms, the bitten start
  2,125-2,157 ms (own work 125-157 ms). **R2 taken:** spikes/netcode-proof/Cargo.toml +3 lines after :120 (a comment,
  `[profile.dev.package.sha2]`, `opt-level = 3`; .gate/u6b-merge/m6-cargo.diff 53682b3a…), test builds only, release
  and the hash value unchanged, Cargo.lock unchanged. U6b's brief kept Cargo.toml out ("a further change stops the
  build", u6b-brief.md:433-435); the advisor lifted that for this one line only. **probe4** (m6-probe4.log, the edited
  tree, no `--config`): 3 parallel runs 5 of 5 each, the bite alone 1 of 1 in 2.46 s (under the brief's 5 s per
  test); s8a 116.9-122.1 ms, the bitten start 2,120.8-2,125.3 ms. **Run 2** (.gate/u6b-merge/m5-check2.log b0487c3a…,
  05:38:21-06:01:53 UTC, CARGO_INCREMENTAL=0, the probed tree's hashes; a container restart had killed its first try at
  the test step, m5-check2-cut.log): **all 11 steps PASS**; measured 807 passed, bites 904 passed, 0 failed, 0 skipped;
  test step 1,037.2 s (1.09x base's 954.8 s); deps PASS with the sha2 line; marks 81, self 78 of 78, redact 11 of 11
  (0 findings). Free 7,697 MiB before, 6,614 after.
- **Plants** (each failed its named test for its planted reason and was restored; advisor-u6b-fix.md item (4) has the
  table): P1-P11, P14, P14b, P15, P15b, P16-P20, all 20 re-applied to the fixed tree. Logs .gate/u6b/fx-plant-<id>.log;
  the diff files beside them since 2026-11-24 04:3x UTC (C1), sha256:

  | Plant | sha256 of .gate/u6b/plant-<id>.diff |
  |---|---|
  | P1 | 7ec59d965ec78ccfd2c67ddbd92cdf76f0bd6d0ef2a51bac07f8ac1ecfc4304d |
  | P2 | 639b70920e516b83a934e65ba33a58f77228990da1ae0a759b737d3715563ed3 |
  | P3 | 69fd62e937a3fbe70725618b4e7477502389775ae17d3d6100d21424c52bba05 |
  | P4 | 114c9aaac0b049180a84edfe0961fee4d6d2a3f62bbb159782d95e727dd3bf8d |
  | P5 | 033072011315a51230f77442dcbdb80d5ef11bc1af1df6e4127717f0360443b8 |
  | P6 | a73944630c186639e1de517990f0b96c0abbad1815f032c2138190a1d6dc368b |
  | P7 | a593a19da106f29ebae71c13462fa9f92aa6a3051b7bdeef5b5107e5a1ce2eb2 |
  | P8 | 59aa194035d62003f3794b869223d98036f4689ee4a0fd50bdc9e4578ad7c461 |
  | P9 | 272d9a8f69f2ee7ebd1d4e6f53c821cad105d8155411c9008341c77a9b36c774 |
  | P10 | cb1637cdd419a46acfbf0a7db7e242bd5f7dc5c570f3827964d56f9f1390ef57 |
  | P11 | 8597d6c2a40edb8c2d62331265eb2cbe6728aa503c1a77836a051caa7d110e13 |
  | P14 | 95ddb0fc39ed2df32ac076f58ebbacde5eacafd02092223161f18d8b9f1f1656 |
  | P14b | 558612e6f61f60ffffd69ba6b2a5b77e80f0fcb7866f549e0284bd8c0511153e |
  | P15 | 375ca2810a280e77cd2811f550469cacbf311bf92a153377af916dff5d2f5197 |
  | P15b | 8fac872790ea637179cbcf1059eee1daf139d95b7d4d6eb063c475a447003c08 |
  | P16 | eb944b18f9030dd1518fe3255d503cf28e37587ede64cfd002443fa60e5e9f1e |
  | P17 | 1d3383d4153c1e56cf5626c34a4d081a936e46fa12cedd8fd2b7b719e7ccd751 |
  | P18 | 90706e581310c3f7ff53679eed758ab4070f672c59ba443b1ffb372b22d072c0 |
  | P19 | 2def394a1e864a35c06d54dc655ae15a135b7de202e256ca4afd956aa6c74566 |
  | P20 | 255e4daeae9c223a188c01f97907f8fad111f1579266785e1196f6bac5c851dd |

- **Bite:** none in U6b's merged tree. PM-0001-18/ws-budget-x2 moved to U6e (A1; its hunks kept in wip/u6b-build.patch
  49f4488f). T-1, A1's line word for word: "closed for U6b by removing its subject, not by a fix: U6b's merged tree
  claims no ws-budget-x2 behaviour. It stays High as U6e's condition: U6e's round sees it met, never a STATUS
  follow-up."
- **Readings, never judged:** 20 starts to OPEN (.gate/u6b/fx-starts.log, release build, default features, no bites,
  binary f78dd20a…; this container, 4 vCPUs; the counted run's tree): 20 of 20 opened, none failed; **p50 19.46 ms, max
  25.17 ms**. These replace U6a's lower bound (p50 634 ms, max 979 ms, a debug build without the listeners or the
  certificate): both listeners and the certificate are now in the start. The 48-connection reading and the cap's
  memory bound at N = 48 moved to U6c's close (A2); the bound exists only as arithmetic (u6b-build.md:181-183).

## TR2-U6A-4, closed here
- **Its history:** raised Low in U6a's round 2 (u6a-r2-test.md:112: "The double-load bite's start shares the 5 s
  first-line cap although 2 s of it are the bite's own sleeps (2,660 ms here); a loaded runner can push it past 5 s and
  fail a correct bite"); named in U6a's fix pass (u6a-fix-pass.md:96-97) and U6a's report (u6a-report.md:66) only under
  "Not verified", with no taker. U6b's start (the dev certificate, both listeners) used up the margin, and it fired in
  M5's run 1 on a correct tree. **Re-rated Medium** by the advisor (F-M6-1). Fixed by C1's hunk, above.
- **F-M6-2's rule:** a Low whose fix is code is carried with a taker, never closed by a "Not verified" line.

## The round
- Netcode APPROVE WITH FINDINGS (Mediums NR-U6B-1, -2; twelve Lows). Security REVISE (Mediums SEC-U6B-1, -2; no
  Critical or High). Test REVISE (High T-1, the bite's evidence; Mediums T-2 to T-9).
- The ruling (advisor-u6b-r1.md): R-PING (NR-U6B-3's outside-a-tick rule, reworded); A1 (the bite leaves U6b, T-1 its
  condition in U6e); A2 (the 48-connection reading at U6c's close); A3 (no second round: the advisor checks the fix
  diff).
- The fix pass, items 0-8 in the ruling's order: A1's removal; SEC-U6B-1, -2; NR-U6B-1, -2 with R-PING; T-2 to T-9; the
  Lows NR-U6B-6, SEC-U6B-4, SEC-U6B-5, NR-U6B-4; the end checks; the 20 starts.
- The advisor's check: all eight items of A3 MET, F-TICK-ROUND confirmed; Critical 0, High 0, Medium 0; Lows F-1 to
  F-4. C1 done (the diff files above); C2 is this report.
- **F-TICK-ROUND:** the gate's tick period is `1_000_000_000u64.div_ceil(TICK_HZ)` (N = 10 at 60 Hz and srtt 100 ms, as
  the netcode seat meant; truncating gives 11); the ruling's line (advisor-u6b-r1.md:230) is corrected to it; the runs
  are 30 Hz, where both give 6, so no mark, judge rule or run value moves.

## Rows written (SERIES.md: one new section, "The transports (U6b)"; 73 rows, none edited; no mark reads them yet)
`event.ws_split`: pooled L each way (count, p50, p99, p999, max, excess, over, zero_excess, zero_over), the reader
stamp's classes (woken, other, early) with each class's pass and reader parts, the mark (set, DelayStream ns, room ns);
`event.open` (wt_serving_ns, ws_serving_ns, open_ns); `event.wt_accept` (7 counts); `event.ws_accept` (10 counts, the
two outside-tick ones included); `header.params` (transport, wt_crate, conn_cap_count, ws_split_pool_rule,
ws_split_mark_after_open_ns). Notes the rows need (report text):
- NR-U6B-7: `tick.v.ws_skipped_snaps_count` as built counts since the last row (the previous tick's refusals), not
  "this tick" as SERIES.md:330 says; a dropped row loses its share. Recorded only; no mark moves.
- NR-U6B-9: the reader stamp is R1's (built; plan.md:1256), not a valve; only ws-budget-x2 is R3's.
- T-5: the reader-stamp identity is a range check for R (R is the first closing wake at or after the release), asserted
  in the loopback self-test. T-6: the window's parts sum to pooled L exactly when no release lands during the mark;
  under traffic one in flight can land on either side of each histogram's copy.

## New pub items, with reasons (NR-U6B-8; plan §4.7: additions only, nothing removed or renamed)
- The brief's list: npt-net WtServer, WtClient, WsServer, WsClient (the two transports), `ConnCap` (the cap both binds
  share), `DelayStream::set_profile` (the per-run profile), DelayStream's header from `&L2Config`,
  `DelayStream::split_rows` (the split's rows); npt-link `Link::begin_tick` (the per-tick batch boundary, Q-U6B-1 (a)).
- Beyond it: npt-net `Listen` (the form both binds share, condition 8; its fields `tick_ns` and `outside_max` carry the
  caller's tick period, NR-U6B-2, and R-PING's 32 B bound), `Accepted` (what a listener hands the room),
  `DelayStream::mark_split` (the window mark, condition 3), `header_texts_from` and `L2Config::named` (the header's
  numbers from one config), `WtServer::socket` (the bound socket's l2_* rows), `WsServer::bind_pem`, `wrapper`,
  `take_summaries`, `event_rows` (PEM settings, the header's ws_wrapper, the ws_summary events, the ws_accept rows);
  npt-server `Connector` and `TickLoop::connector`, `io_handle`, `now_ns`, `pin_to_io` (NR-U6A-7's hand-off and I/O
  runtime, the room clock for the mark), `TickLoop::stop_with` (NR-U6B-1); npt-proto's three constants with
  `Channel::ws_tag`/`from_ws_tag` (added item 1). Gone with A1: `with_bite` on WsServer and `WS_BUDGET_BITE`.

## Carries, by number (advisor-u6b-fix.md section 6)
| Taker | Items |
|---|---|
| U6e | PM-0001-18/ws-budget-x2 (wip/u6b-build.patch 49f4488f's hunks) with T-1 as its condition, word for word; SP-4 and SEC-U6BD-2; N4 (the binary's arming in no test); T-4's bite half; T-11; T-14; T-16 (from U6d); T-18; T-21; NR-U6B-5 (not built: the scope line); T-12 (not built); F-1 (the WebSocket half of the closing bound untested); F-2 (T-4's test timed alone); F-M6-3 (u6b_open reads its first line with no cap, binary.rs's u6b_open test: a room that never opens hangs the test step; its handshakes' margin against the room's 2 s deadline unmeasured) |
| U6d | NR-U6B-11, NR-U6B-13, NR-U6B-14 |
| U6c's close (A2) | the 48-connection DelayStream self-test reading and the cap's memory bound at N = 48 (NR-U6B-5's recount of the pool's heap in it); F-3 (the room-clock mark read non-zero) |
| U7 | SEC-U6B-2's unidirectional stream count (6), confirmed in its headless Chromium run |
| U7's or U8's record step | NR-U6B-10 (F-NEG: a presence-only draft count, or -56 item 2 by number) |
| U8's next edit (R1.md and the unit) | NR-U6B-6's room-clock mark read by the INVALID rule; NR-U6B-9's reader rows beside pooled L; T-5's note; T-17 (the hash file's 0644 and the umask: UMask=0022 in the room's unit; not built) |
| The security baseline; PR #40's closing list | SEC-U6B-3 (Retry before the permit): a security finding left open (advisor.md step 4) |
| Closed in U6b | TR2-U6A-4 (U6a's, Medium by F-M6-1: the bitten start's cap); SEC-U6B-1, -2, -4, -5; NR-U6B-1, -2, -3 (R-PING), -4, -6 (the row), -12 (by T-6); T-2 (both halves), T-3 to T-9, T-10 (with C1), T-13, T-15 (= SEC-U6B-4), T-19 (P20); T-1 closed for U6b by A1, open as U6e's condition |

## Not verified
No RS, no load, no browser (U7's headless Chromium is the first contact; whether a browser's CONNECT names a draft-02
header is from web-transport-proto's comments only); the 48-connection reading and the memory bound (A2: U6c's close);
each plant log's diff hash against its diff file (the advisor's check needed an alternate git index; that call was
stopped at a permission prompt and not run: each failure matches its diff's edit instead); the WebSocket half of the
closing bound (F-1); T-4's test alone (F-2); the mark row only at 0 (F-3); quinn's Debug text as a stable form for
SEC-U6B-2's test; the mark's cost on an I/O worker (arithmetic only); the 20 starts are one container's, never judged;
what slowed M5's run 1 beyond the host change (its test step 1,013.6 s against the builder's 848.5 s; incremental builds
were on in that run, F-M6-4, off since: C2), and the bitten start's own time in that run; the hash's share of the
start was shown by the A/B (about 2.9 s of 3.0 s), not timed inside the binary.
