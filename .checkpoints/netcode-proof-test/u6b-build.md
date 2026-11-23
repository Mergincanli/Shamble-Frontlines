Task: U6b's build (the transports: WtServer/WtClient, WsServer/WsClient, the send path with fix 1, the L2 extras and the header, the binary's transport wiring and the dev certificate), u6b-brief.md rev 1.1 with the launch prompt's added items 1-6.

Start: `git merge --ff-only 21765e3` in the worktree; `git rev-parse HEAD` = 21765e332524464b19882e2b57b96ad79adbb7ea (= <base>). Base counts: measured 754, bites 850.

## Plan (blocks in order; added items placed)
- A. Notes and U6a's seams at <base> (file:line below).
- Block 1 (shared pieces): `ConnCap` (net/src/cap.rs), the pin (net/src/pin.rs, tls_test's `Pinned` promoted, hash pin via
  rustls's ring SHA-256), the stamps rule; added item 1 (npt-proto's three constants, `Channel::ws_tag`/`from_ws_tag`, the
  catalogue line); added item 3 (NR-U6A-7: `TickLoop::connector()`/`Connector`, `TickLoop::io_handle()`,
  `TickLoop::pin_to_io`, stop on the main thread); added item 4 (NR2-U6A-2: exited tids left out of `sibling_free`).
- Block 2: WtServer, WtLink, WtClient (net/src/wt_link.rs over l2/wt.rs's endpoint and config).
- Block 3: WsServer's accept path, WsLink, WsClient (net/src/ws.rs).
- Block 4: `Link::begin_tick` (npt-link), TickLoop's I/O send site (per-tick batch, all-or-none reservation), WsLink's
  gate (TickWriter per connection; DelayStream's `gate_inputs` or the plain path's kernel readings).
- Block 5 (less the valve): the header (ws_wrapper, DelayStream's numbers from &L2Config, l2_* from the bound socket,
  cert_validity_ns, transport, cc, crate, cap, POOL rule), `ws_split` with pooled L and the window mark (condition 3),
  `ws_summary` per connection, one event per transport.
- Block 6: settings, cert.rs (dev certificate, hash file), OPEN's move, the binary's wiring, end-to-end tests; added
  item 5 (F28: npt-super's log line-flushed).
- Valve (added item 6), last: Block 5's reader stamp and `Bite::BudgetX2`, only if added lines <= 5,400 then.

## U6a's seams at <base>
- TickLoop's accept seam: server/src/tick_loop.rs:1404 `connect(link, rtt)`: no accept stamp; `Decoder::new`'s
  accept_ns is `clock::now_ns()` at :1421 (via SessionIo::open :806-845). U6b adds the stamp (Connector::connect).
- The I/O side's outbound: the tick thread offers each datagram as its own `Outbound::Datagram` (:708-712), no batch
  boundary; SessionIo::poll sends them per message (:851-863). U6b adds the boundary and `begin_tick` there.
- OPEN: server/src/main.rs:507 (after TickLoop::start :502).
- Token file: main.rs:264-278 (`create_new`, 0600, "{slot} nptj_<base64url>" per line).
- Settings: main.rs:60-103 (struct), :125-222 (parser; deny_unknown_fields).
- Clock: server/src/clock.rs:21 `now_ns`.
- Writer thread: tick_loop.rs:921-943; header params main.rs:367-385.

## Findings (one line each)
- F-NEG (added item 2): web-transport-quinn 0.13.6 does not expose the negotiated WebTransport draft at accept: its
  `Settings` (settings.rs:26-33) keeps only the two streams, the parsed `web_transport_proto::Settings` is dropped after
  `supports_webtransport()` (settings.rs:46-57); `Session::protocol()` (session.rs:614) is the WT-Protocol subprotocol,
  not the draft. No key written.
- F-PING (condition 1, to rule): "a datagram send no current begin_tick announced ... is answered BufferFull and
  counted, never written ungated" would refuse the room's own pings and the pongs the client's clock reads (the
  decoder sends them between ticks; a 40 B ping fails may_write's third condition, outstanding <= (N-1) x 40 B,
  whenever a snapshot is in flight). Built: a datagram outside an announced tick (or past its count: the same case)
  passes fix 1's first two conditions only (nothing owed, room in the socket) and is counted (sent, refused); the
  tick's announced datagrams are gated all or none on all three. ws_server.rs module text; test b4_a_datagram_outside.
- F-SKIPPED-ROW: `tick.v.ws_skipped_snaps_count` is the count since the last tick row (the I/O side sends a tick's
  datagrams after that tick's row is made), all sessions.
- F-MARK-HDR (condition 3): the mark is taken after OPEN, so "whether it was taken" cannot be in the header (written
  before OPEN): the header records the planned mark time (settings), and `event.ws_split` carries `ws_split_mark_set`
  and `ws_split_mark_ns` (the mark's stamp on DelayStream's clock).
- F-POOL-BYTES: pooled L adds 2 x 135,040 B to the split's pool (and the mark 12 x 135,040 B once taken);
  `ws_split_fixed_bytes` keeps its row's meaning (the five parts per direction), so neither is in it.
- F-WT-UNSENT: WtLink's `unsent_unreliable` is bytes in quinn's datagram send buffer (quinn reports only its free
  space), not datagrams.
- Block 1 done (with Block 2's first run): ConnCap (net/src/cap.rs), pin (net/src/pin.rs), stamps (session.rs);
  added items 1 (proto lib.rs consts, link ws_tag/from_ws_tag, catalogue :73-75), 3 (Connector, io_handle, now_ns,
  pin_to_io in tick_loop.rs), 4 (proc_read::io_allowed). Tests: link 2, proc_read 1, pin 1, ws_server b1 x3.
- Block 2 done: wt_server.rs (WtServer, WtLink, WtClient); 9 tests pass (b2_*; +1 all-features).
- Block 3 done: ws_server.rs (WsServer accept path, WsLink, WsClient); tests b3 x8 pass, b4 C1c pass.
- Block 4 (npt-net half) done: begin_tick in npt-link; WsLink's gate; tests b4 x8 pass (b4_* in ws_server_tests).
  TickLoop half (batch boundary, try_reserve_many, send_tick, ws_skipped) built; its tests next.
- Block 4 TickLoop half tests pass: u6b_each_tick_is_announced_whole_and_a_refused_one_is_counted,
  u6b_a_connector_carries_the_accept_stamp_to_the_join_deadline.
- Block 5 (less the valve) built: DelayStream::header_numbers_from / header_texts_from (&L2Config), split_rows pub
  with pooled L both E forms and the window mark (DelayStream::mark_split; stats::Base; Pooled::mark/summary_at),
  WsServer::take_summaries (ws_summary at each connection's end), event_rows per transport; SERIES.md new section
  "The transports (U6b)" (40 rows). Tests: stats u6b x2.
- Block 6 built: cert.rs (dev certificate, days-to-civil, hash file), settings (transport, wt_bind, ws_bind,
  ws_cert_chain, ws_cert_key, hash_path, ws_split_mark_s, cc; U8's names), Net (bind before the log, params, serve on
  the I/O handle, summaries, end events), OPEN after both listeners serve; WsServer::bind_pem; L2Config::named.
  Tests: cert x3, main u6b settings x1, binary u6b_open x1, tick_loop u6b_clients x1 (e2e over both at P2).
- Added item 5 (F28) built: npt-super's Writer over io::LineWriter (each record flushed at its newline); test
  u6b_each_record_reaches_the_file_while_npt_super_runs.
- F-JOIN-ONCE: npt-client's endpoint sends its Join once in its first step; a Connecting WtClient/WsClient answers
  BufferFull, so the Join is lost and the room closes the session at its 2 s deadline. The e2e test steps each client
  only once its link is Open; U6c's glue (npt-bots, npt-load) must do the same (or the endpoint resend).
- DISK (07:49 UTC): the runner stopped (exit 3) at 3,741 MiB free, under the 4 GB floor; df: 252G size, 35G used,
  3.7G avail. Nothing cleaned (not mine). Waiting/re-checking before the next build.
- DISK answered (11:1x UTC): the main session part-cleaned the shared target's dev profile (7.5 GiB); free 10,849 MiB.
  The next cargo run rebuilds cold. No plant was in place at the stop; the one half-made edit is the valve's start
  (delay.rs: `Freed::reader` and the reader parts' constants), carried on below.
- Size before the valve (07:5x, before rustfmt): 4,731 added (git diff --shortstat 21765e3, new files intent-to-add),
  plus BudgetX2's ~70 written since: under 5,400, so the valve is built (added item 6).
- Valve, part 1 written (not yet compiled): `Bite::BudgetX2` under cfg(any(test, bites)) (ws_gate.rs),
  `WsServer::with_bite` per instance (ws_server.rs), the binary arms it by id (main.rs), bites tests x2
  (pm_0001_18_* in ws_server_tests.rs).
- Valve, part 2 (the reader stamp, RC-12) built: `Freed::reader`, `Conn::stamp_reader` at the pump's closing block
  (when it wakes the reader), the up split into pass and reader with the identity checked, the early class,
  `SplitBite::ReaderAtRelease`, the pool's reader parts with the mark, `delay_names::reader_rows` (27 rows), SERIES.md
  rows. Tests: u6b_the_reader_stamp_splits_up_work... (delay_tests), b5_the_loopback_self_test... (ws_server_tests).
- F-RT-IO: TickLoop's runtime had only `enable_time`; the listeners need I/O: now `enable_all` (tick_loop.rs, the
  binary tests caught it: "IO is disabled").
- Valve done; both builds' tests of npt-net, npt-server, npt-super, npt-link, npt-proto pass (11:29 UTC; logs
  .gate/u6b/v-test3.log): measured npt-net lib 196, server lib 26, server binary 3; bites npt-net lib 204.
- rustfmt: 88 hunks applied by Edit (fmt1.log); `cargo fmt --all -- --check` clean (fmt2.log, 11:34 UTC).
- SIZE (11:35, after rustfmt, the valve in): 5,460 added, 254 removed, 29 files (git diff --shortstat 21765e3 --
  spikes/netcode-proof docs). Under the 6,000 stop.
- End checks: check.mjs run 1 started 11:3x UTC (.gate/u6b/check1.log, -x, after the provenance --no-run builds);
  the plants follow it, then check.mjs run 2.
- Plants planned (file: the plant; the test it must fail): P1 ws_server.rs Writer::run forgets the permit
  (b1_one_cap...); P2 WsLink::poll_recv stamps at poll (b1_arrival...); P3 wt_server.rs deadline over QUIC only
  (b2_the_deadline_covers_http3...); P4 ws_server.rs Acceptor::run's deadline 10 s later, so TLS is not under it
  (b3_the_deadline_covers_tls...); P5 wt_server.rs close() quinn's immediate close (b2_a_kick...); P6 Writer::run
  sends Close before the queued messages once the link is gone (b3_a_kick...); P7 bind_parts' ws_config from the
  513 B constant (b3_one_source...); P8 send_unreliable reads the inputs per datagram (b4_the_inputs_are_read_once...);
  P9 the gate per datagram (b4_a_tick_is_written_all_or_none); P10 owed without rustls's term (b4_owed...); P11 a
  close error counted whatever the peer did (b4_a_close_error...); P14 main.rs OPEN before Net::bind
  (u6b_open_comes_once... in tests/binary.rs); P15 main.rs writes the key beside the hash file (the same binary test);
  P16 link/lib.rs's Box forward of begin_tick removed (b4_begin_tick_through_a_boxed_link...).
- check.mjs run 1 (11:35-11:53 UTC, .gate/u6b/check1.log; the tree had one type-alias edit to stats.rs after its
  clippy step): fmt PASS; clippy FAIL (stats.rs:330 type_complexity: first error only); clippy-wasm32 PASS; doc PASS;
  test PASS 906.9 s (base 954.8 s): measured 798 (+44 on 754), bites 897 (+47 on 850); deps FAIL (the bite's id
  literal in two crates: ws_server.rs and main.rs); wasm PASS; refs PASS; marks PASS (81); self PASS (78 of 78);
  redact FAIL (main.rs test's "1.2.3.4" read as an IPv4 address).
- Fixes after run 1: stats.rs `type Mark`; WtLink::new renamed `parts` (clippy new_ret_no_self); the bite id one
  home: `npt_net::WS_BUDGET_BITE` (pub, bites builds), main.rs takes it by path; the test address 192.0.2.1 (TEST-NET);
  +1 measured test (config.rs u6b_a_named_profile..., L2Config::named) for the +45.
- DISK again (11:54 UTC): the runner stopped (exit 3) at 1,624 MiB free (the cold rebuild and check.mjs's builds grew
  the shared target to 11 GB). Nothing cleaned (not mine). Clippy's confirmation, the plants and check.mjs run 2 wait
  for space.
- F-VALVE (added item 6): R1.md (wip/u8-build.patch:1560-1568) reads `event.ws_split`'s pooled-L fields only; R1's
  bites block (:2160-2171) runs PM-0001-1, PM-0006-11, PM-0001-16, not ws-budget-x2. So the reader stamp and BudgetX2
  are a valve.
- DISK answered (16:2x UTC, after the second spend-limit cut 12:28-16:22): the main session part-cleaned the shared
  target's dev profile (10.4 GiB removed); free 10,776 MiB (df 16:27: 11 GB). Runner change: u6b-run.sh now exports
  CARGO_INCREMENTAL=0 (each log's header shows it; cache only, no result changes), so the end checks should fit
  above the 4 GB floor without another clean. Resume check (16:27): git status shows no plant in place and no half-made
  edit; config.rs's u6b_a_named_profile... test is whole. Next: clippy both builds, then the plants, then check.mjs run 2.
- Clippy confirmation: clippy2.log (16:28-16:31) found fmt in config.rs's new test (applied by Edit) and
  cloned_ref_to_slice_refs at tick_loop/tests.rs:1526 (`let chain = [der];` shared by both binds); clippy3.log
  (16:41): fmt --check, clippy on both builds (-D warnings, --all-targets, with and without --all-features) exit 0.
- Plants' restore hashes (16:42): ws_server.rs 346af9fe...ce26, wt_server.rs a9872ed6...8139, link/lib.rs
  5f9fcbd8...b6, server/main.rs 8215ef58...63fd.
- P1 (Writer::run `mem::forget` of the permit): b1_one_cap... FAILED at ws_server_tests.rs:153 ("the permit came
  back"), plant-p1.log; restored, hash ok. P2 (poll_recv stamps `_now_ns`): b1_arrival... FAILED :218 ([9000, 9000]
  vs [1000, 1000]), plant-p2.log; restored ok. P3 (timeout over `incoming` only, the outer one +3,600 s):
  b2_the_deadline_covers_http3... FAILED wt_server_tests.rs:269 ("closed by the server in time"), plant-p3.log;
  restored ok. P4 (the WS deadline +10 s, so TLS is not under it): b3_the_deadline_covers_tls... FAILED :254,
  plant-p4.log; restored ok. P5 (close(): quinn's immediate close, no finish or stopped): b2_a_kick... FAILED
  wt_server_tests.rs:328 (16 of 20 read the kick), plant-p5.log; restored ok. P6 (Writer::run breaks once
  `orx.is_closed()`, so the Close goes before the queue): b3_a_kick... FAILED :420 (1 of 20), plant-p6.log; restored
  ok. P7 (ws_config from `L2Config::WS_MAX_MESSAGE_BYTES`): b3_one_source... FAILED :232 (one_source refused),
  plant-p7.log; restored ok. P8 (`self.inputs()` read per datagram in send_unreliable): b4_the_inputs_are_read_once...
  FAILED :564 (20 readings vs 5), plant-p8.log; restored ok. P9 (each announced datagram gated by `may_write` on its
  own size): b4_a_tick_is_written_all_or_none FAILED :585 (the 40 B one BufferFull), plant-p9.log; restored ok. P10
  (owed without rustls's term): b4_owed... FAILED :927 (120,004 vs 143,909), plant-p10.log; restored ok. P11 (close
  error counted whatever the peer did): b4_a_close_error... FAILED :497 (close_failed_count 1), plant-p11.log;
  restored ok (ws_server.rs 346af9fe...ce26 after each). P16 (the Box forward of begin_tick removed):
  b4_begin_tick_through_a_boxed_link... FAILED :616 (Sent vs BufferFull), plant-p16.log; restored ok (link/lib.rs
  5f9fcbd8...b6). P14 (OPEN printed before Net::bind, the later one removed): u6b_open... FAILED binary.rs:235 (the
  hash file not there at OPEN), plant-p14.log; restored ok. P15 (a file written beside the hash file, a stand-in text,
  never the key): u6b_open... FAILED binary.rs:325 (["room.json", "room.key"]), plant-p15.log; restored ok (main.rs
  8215ef58...63fd). All 14 plants (P1-P11, P14-P16) fail their named tests; all four files back at their hashes
  (17:0x UTC). P12 and P13 are U6e's (the lever). Next: check.mjs run 2.
- check.mjs run 2: a first start (check2.log) was stopped by me during its --no-run build (my background timeout was
  10 min, too short for an 18 min run; no cargo left after); restarted as check3.log (17:1x UTC, -x, the same three
  commands), the counted run.
- check.mjs run 2 (check3.log, 16:50-16:57 UTC): fmt, clippy, clippy-wasm32, doc, deps, wasm, refs, marks (81), self
  (78 of 78), redact (11 of 11; 0 findings) PASS: run 1's three fixes hold. test FAIL: npt-server lib's
  u6b_an_exited_io_thread... read the joined thread's /proc entry still there (2 entries, both [0, 1, 2, 3]): `join`
  returns at the kernel's clear of the tid futex (exit_mm), before release_task unhashes the tid. A test race, not
  io_allowed's: the test now waits (bounded 5 s) for /proc/self/task/<dead> to go (proc_read.rs, +11 lines);
  fmt and the test pass (b6-test3.log, 16:58). Next: check.mjs run 3 (check4.log), the counted one.
- check.mjs run 3 (check4.log, 16:59): clippy FAIL at once (the wait used `Instant::now`, a disallowed method);
  stopped by me during its test step (no cargo of mine left). The wait is now a count (at most 5,000 waits of 1 ms, no
  clock read); clippy4.log (17:09): fmt, clippy both builds, the test: exit 0. Disk 6.7 GB free. Next: check5.log.
- COUNTED check.mjs (check5.log, 17:10-17:26 UTC, -x, after both --no-run builds; free 6.7 GB before and after): all
  11 steps PASS. test 891.5 s (base 954.8 s: 0.93x); measured 799 passed (+45 on 754), bites 898 (+48 on 850), 0
  failed, 0 skips. marks 81; self 78 of 78; redact 11 of 11, 0 findings; deps held (one bite id home).
- Plant files after the counted run: the four hashes as recorded (ws_server 346af9fe, wt_server a9872ed6, link
  5f9fcbd8, main 8215ef58). No Cargo.toml, Cargo.lock, marks.json, judge or .claude change (git diff --stat empty).
- SIZE (17:27): 5,498 added, 254 removed, 29 files. By block (by file; files that span blocks under their main
  block): B1 with items 1, 3, 4 ~479 (cap 72, pin 108, session 152, proto 9, link 63, catalogue 3, proc_read 49,
  net lib/l2 19, tls_test 4); B2 1,103 (wt_server 671, its tests 418, l2/wt 14); B3-B4 npt-net 1,962 (ws_server 989,
  its tests 929, ws_gate 8, config 36); B4 TickLoop half 461 (tick_loop 164, its tests 296 incl. B6's e2e, lib 1);
  B5 with the valve 664 (delay 295, delay_names 105, delay_tests 55, stats 133, SERIES.md 76); B6 782 (cert 252,
  main 353, binary 177); item 5 (F28) 47. Brief's likely 3,650-4,050; under the 6,000 stop.
- F-PROC-RACE: `JoinHandle::join` returns before the thread's /proc/self/task entry goes (the tid futex is cleared in
  exit_mm, the tid unhashed later in release_task); io_allowed is right for npt-server's reads (long after exit), the
  test now waits for the entry. F-P14: P14 fails at the hash file's read (written in Net::bind, ms after the planted
  OPEN): timing-dependent in principle, failed at once here. F-P15: the plant writes a stand-in text as room.key,
  never key bytes.
- Memory bound at N = 48 (arithmetic, not a container reading): (48 + 8) x ~0.31 MB = ~17.4 MB of DelayStream
  connections, plus the fixed part: ImpairSocket's pool 12.3 MB (8,192 x 1,500 B), pooled L 270 KB, the mark 1.62 MB
  once taken (12 x 135,040 B), and ws_split_fixed_bytes as the header has it.
- replan-units.md:77-78's "as l2/wt.rs does per crate" is wrong: l2/wt.rs:176-179 names ring once
  (`wtq_server::provider()`), since Block 0 turned web-transport-quinn's aws-lc-rs default off.
- DONE: the build and the end checks. Left: the container readings (see above); the report.
- Container readings (brief section 3: 20 starts to OPEN, the 48-connection WsServer example at P2 for 60 s in a
  release build, the memory bound): NOT built. The coordinator's resume lists the end checks only; the example is new
  code (crates/server/examples/) and a release build on a disk that has hit the floor twice. Left for the coordinator
  to place (here after the check, or at U6b's close as the brief's section 8 "Close" line has it).
