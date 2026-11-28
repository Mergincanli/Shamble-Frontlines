# U6b's one review round: the netcode seat (xhigh, read only)

Inputs, hashes checked 2026-11-23 (all four match): u6b-brief.md 32baaebd...b99b; advisor-u6b-brief.md 3c0f8121...018d;
wip/u6b-build.patch 49f4488f...c14c (base 21765e3, 29 files, 5,498 added, 254 removed); u6b-build.md be6807f4...3c01.
W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-a2661dc320c9e6d00/spikes/netcode-proof (read only).
Resumed after a spend-limit cut (17:33-21:10 UTC); before the cut I had done only the hash check.
This file is rewritten whole with Write each time (this agent has no Edit tool), as the rules allow.

## 1. Plan
1. Inputs read: the brief (rev 1.1, sections 0.1, 1-4, 7), the advisor's check, the builder's notes, U6a's fix pass
   (its carries section), my U6a findings NR-U6A-7 and NR2-U6A-2 and O2, advisor-u7-brief.md's C2 (the homes).
2. The pub surface: list every new or changed `pub` item in the patch (npt-net, npt-link, npt-proto, npt-server,
   npt-super); read plan.md §4.7's whole-surface rule and check each against it; `begin_tick`'s signature, default,
   doc text and the Box forward; `ConnCap` (condition 8).
3. Fix 1's path: WsLink's gate (begin_tick, send_unreliable, the announced count, the three conditions of
   `may_write`, owed, the plain path's inputs and free value); TickLoop's I/O send site (the per-tick batch boundary,
   `try_reserve_many`/the all-or-none reservation, `send_tick`, `ws_skipped_snaps_count`); check the row against
   SERIES.md:330 and against what the I/O side counts.
4. The reader stamp: delay.rs's closing block (the stamp R, the split into pass and reader, the early class, the
   identity and its violations count), the pool's reader parts with the window mark, the misattribution bite, the
   delay_names rows, the loopback self-test.
5. Added items 1 (npt-proto's three constants, `Channel::ws_tag`/`from_ws_tag`, catalogue line, the equality test),
   3 (NR-U6A-7: runtime enable, `io_handle`, `stop` off the async context), 4 (NR2-U6A-2: exited tids).
6. The new SERIES.md rows ("The transports (U6b)") against what R1 reads (R1.md in wip/u8-build.patch, the U8
   rulings C-U8-3, C-U8-5, plan.md:1256) and against the code that writes them (names and units match).
7. The carries of section 4 by number (SP-1 to SP-8, RC-1 to RC-12, C-U6-17's form, U6a's condition 6, C-U6-19's
   line), each met / moved / open with file:line.
8. The F-items: F-PING (rule it), F-MARK-HDR, F-NEG, F-POOL-BYTES, F-WT-UNSENT, F-SKIPPED-ROW, F-RT-IO, F-VALVE,
   F-JOIN-ONCE.
9. Tests through the runner only when a claim needs one (one command per call; exit 3 = disk floor, go on reading).
10. Verdict, findings table, F-item views, not verified, DONE.

## 2. Notes (one line per finding, saved as I go)
- Steps 1-2 read (21:1x UTC): brief, advisor's check, builder's notes, fix pass carries, my U6a notes, C2 homes; plan.md
  §4.4 and §4.7; the patch's link, cap, session, pin, ws_gate, ws_server, wt_server, delay (stamp, pool, mark),
  delay_names, stats, proto, main.rs, tick_loop (batch, send_tick, Connector), proc_read, super, SERIES.md. check5.log's
  tail read: all 11 steps PASS, exit 0 (17:26:53 UTC). No test run by me: every claim below is read from the code.
- OK: `begin_tick` signature, default `Sent`, the doc's caller duty, the Box forward (link/src/lib.rs:115-125,
  :157-159); `ws_tag`/`from_ws_tag` with a round-trip and an unknown-tag test; the catalogue line names 0, 1, 2; the
  proto's three constants (proto/src/lib.rs, end) are taken by the binary (server/src/main.rs:750-752), its private
  copies gone. C2's equality test is U7's half (advisor-u7-brief.md C2), so added item 1 is met.
- OK: the tick thread reserves a tick's datagrams all or none (server/src/tick_loop.rs:719-730; tokio 1.54.1's
  `try_reserve_many(0)` gives an empty iterator, so an empty tick sends no `last` and announces nothing); the I/O side
  batches across polls and calls `begin_tick` once, then sends every datagram (tick_loop.rs:874-883), all before the
  pump runs (tick_loop.rs:887-914), so no ping lands inside an announced tick. WsLink's gate: inputs read once per tick
  (ws_server.rs:664-682), `snapshot_budget`/`may_write` called through TickWriter (ws_gate.rs:118-139), never copied;
  owed = DelayStream's owed or, on the plain path, tungstenite's and rustls's unsent bytes, plus the link's own queue
  (ws_server.rs:619-648); the plain path's free value `u64::MAX` stated with its reason (ws_server.rs:18-20); a failed
  SIOCOUTQ read refuses every write (ws_server.rs:640). All-or-none holds at the link too: a tick is let through only
  with owed 0, so the link's own queue is empty when its datagrams are queued.
- OK: the reader stamp R is taken under the lock at the pump's closing block when the pass released up work
  (net/src/l2/delay.rs:2141-2145), on every up record a pump step released and the reader has not read
  (delay.rs:767-787); APP releases never get R (early class, delay.rs:737-739); the identity is checked with
  P (or T) <= R <= (3) on the four parts (woken: clock + hop + pass + reader) or three (other: wait + pass + reader)
  (delay.rs:716-748); pooled L is recorded per release (delay.rs:705); the misattribution bite (R at the release) is
  caught by the "R is a closing wake's stamp" check (delay_tests.rs, `reader_split`). No allocation on the release path
  in production (`reader_wakes` is cfg(test)). The window mark is taken off the release path, behind a Mutex only the
  reader side takes (stats.rs:349-380).
- OK: NR-U6A-7 met: `enable_all` (tick_loop.rs, the runtime builder), `io_handle`, `Connector`, the "where it lives"
  doc text (tick_loop.rs:1279-1284), `serve`'s `enter` guard drops at its return (main.rs:620-645), so `finish` calls
  `stop` on the main thread outside any runtime context; `u6b_a_connector_carries_the_accept_stamp_to_the_join_deadline`
  runs `stop` on a plain thread. NR2-U6A-2 met: `io_allowed` leaves out a tid with no /proc entry, `None` only for a
  live one (proc_read.rs, `io_allowed`), with its test; nothing in U6b starts a tokio blocking thread (grep: no
  `spawn_blocking`, `tokio::fs` or `block_in_place` in net/src or server/src), so `io_tids`'s dead entries stay moot.
  F28 met: npt-super's Writer over `LineWriter` (super/src/main.rs, `log`), with its test.
- NR-U6B-1 (Medium): `Net::end` runs before `TickLoop::stop` closes the sessions (main.rs:777-779), and a WsLink pushes
  its `ws_summary` only when its closing ends (ws_server.rs:856-858), which then nobody reads: every WebSocket
  connection still open at the room's end gets no `ws_summary`. R1's LG-loaded rooms end at their own duration while LG
  runs (wip/u8-build.patch:1684-1686, NC-SB-2), so all 48 are lost there; and `ws_split` is read while those 48 still
  release records (delay.rs:1297: "Read it once the traffic has ended"). The binary test drops its clients before
  stdin closes and asserts no `ws_summary` (binary.rs:330-354), so nothing catches it. No mark and no R1.md line reads
  `ws_summary` (marks.json: 0; u8-build.patch: 0), so Medium, not High.
- NR-U6B-2 (Medium): the gate's tick period is a literal, `TICK_NS = 33_333_333` (ws_server.rs:74-75), used for every
  connection's N (ws_server.rs:498, :583), while the room sends a snapshot per tick (`snapshot_hz` = `tick_hz`,
  room/src/room.rs:882) and plan §4.6 has one tick constant from which "every tick-counted timer" derives, the 60 Hz run
  changing npt-sim only (sim/src/ticks.rs:10-12). At 60 Hz the gate would size N for 30 Hz (srtt 100 ms: 6 instead of
  10, so about twice the skips). npt-net cannot reach `npt_sim::TICK_HZ` without a manifest change, so the value must
  come from the caller.
- NR-U6B-3 (Low; F-PING): the built outside rule is right in purpose, but (a) its two counts are test-only
  (`gate_counts` is cfg(test), ws_server.rs:650-660), so no run's log shows a datagram sent or refused outside a tick;
  (b) its condition 1 counts the link's own queue (`inp.owed += queued`, ws_server.rs:646, read at :701-703), so a ping
  or pong the pump sends in the same 1 ms poll as a tick's batch (tick_loop.rs:887-914; POLL 1 ms, :59) is refused
  although the batch leaves microseconds later: by reading, about 1 in 33 on a healthy link at 30 Hz (not measured);
  (c) no size bound, so a snapshot sent without `begin_tick` would pass conditions 1-2; (d) the trait's doc
  (link/src/lib.rs:115-121) does not state the rule.
- NR-U6B-4 (Low): `begin_tick`'s edges in WsLink: an empty `lens` is gated as a 0 B snapshot and counted skipped when
  anything is outstanding (ws_server.rs:668-676 with ws_gate.rs:36-38); `TooLarge` returns before the announced count
  is decremented (ws_server.rs:685-687), leaving one slot that the next outside datagram takes, ungated when the tick
  was allowed. TickLoop never does either today; the trait doc forbids neither.
- NR-U6B-5 (Low; F-POOL-BYTES): the pool now holds 16 histograms (10 parts, 2 pooled L, 4 reader parts: delay.rs:213,
  :221) and the mark copies all 16 (16 x 135,040 B = 2,160,640 B), while `ws_split_fixed_bytes` stays
  `Pool::heap_bytes()` = 10 (delay.rs:303-306, :1362); the advisor's Q-U6B-3 term (1) wanted the added heap in the
  header; the builder's notes (u6b-build.md:49-50, :181-183) count 2 added and 12 marks, missing the reader's 4 + 4
  (540,160 B of heap and as much again at the mark).
- NR-U6B-6 (Low; F-MARK-HDR): `ws_split_mark_ns` is on DelayStream's clock only (delay.rs:1316; delay_names MARK_NS),
  and the log records DelayStream's origin on no other clock, so the mark cannot be placed on the tick rows' (room)
  clock, while R1.md makes the run INVALID with "fewer than 48 connections up 60 s before the mark"
  (wip/u8-build.patch:1697-1698).
- NR-U6B-7 (Low; F-SKIPPED-ROW): the row as built counts the previous tick's refusals (since the last row:
  tick_loop.rs:575-576; the I/O side sends after the row is made), while SERIES.md:330 says "this tick"; a dropped row
  loses its share (the swap is made before `offer_row`, tick_loop.rs:575, :583-588; only `rows_dropped` counts it).
- NR-U6B-8 (Low): pub items beyond the brief's list (section 1, "New pub items", plus `ConnCap`): npt-net `Listen`,
  `Accepted` (net/src/lib.rs re-exports; condition 8's "the form both binds share"), `DelayStream::mark_split`
  (delay.rs:1316), `header_texts_from`, `L2Config::named` (config.rs:121), `WtServer::socket`,
  `WsServer::bind_pem/wrapper/take_summaries/event_rows`, `with_bite` and `WS_BUDGET_BITE` (bites builds); npt-server
  `Connector`, `TickLoop::connector/io_handle/now_ns/pin_to_io` (tick_loop.rs:1269, :1508-1529); npt-proto's three
  constants. Each is an addition in std or the crates' own types (§4.7 holds: nothing removed or renamed), but the
  brief asks each named in the report with its reason.
- NR-U6B-9 (Low; F-VALVE): plan.md:1256 puts "the split built in U3c (b5c9009) plus U6's reader stamp" in R1's
  DelayStream self-test and sends a miss "the split with the reader stamp to the advisor first": the reader stamp is
  R1's, not a valve (it is built, so no code change); only ws-budget-x2 is R3's.
- NR-U6B-10 (Low; F-NEG): right that web-transport-quinn 0.13.6 drops the client's SETTINGS (its settings.rs:46-57),
  but its `Request` derefs to `ConnectRequest` (server.rs:234), whose `headers` are the CONNECT's raw headers
  (web-transport-proto 0.6.4 connect.rs:117-118), and the crate always answers `sec-webtransport-http3-draft: draft02`
  (connect.rs:403): a per-room count of sessions whose CONNECT named a draft-02 header (its presence only, never a
  value: E22) plus the answered draft in the header would give -56 item 2 (marks.json, PM-0001-56's notes) a
  server-side source.
- NR-U6B-11 (Low; F-WT-UNSENT): `unsent_unreliable` is "Datagrams handed to the backend and not yet on the wire"
  (link/src/lib.rs:112-113); WtLink returns bytes (wt_server.rs:409-412), WsLink 0 though its own queue holds the
  tick's datagrams (ws_server.rs:742-744). No reader before U6d (grep: none outside tests).
- NR-U6B-12 (Low): condition 3's third test ("the identity exact across the mark") is one Pooled against a fresh
  histogram (stats.rs:812-837); no test checks the split's parts against pooled L over one marked window.
- NR-U6B-13 (Low): a session the loop refuses (`Connector::connect` false: stopped or its queue full) is dropped
  uncounted (main.rs:627-635), after the transport counted it in `accepted_count`.
- NR-U6B-14 (Low): the plain path's owed (ws_server.rs:633-641) takes `framed - plain` saturating: tungstenite's own
  frames (pong replies, the Close) reach `plain` but never `framed`, so after a client's WebSocket pings the term
  under-reads by their bytes for the rest of the connection; the `queued` term still covers the usual stall. No R1
  client sends WebSocket pings (C-U6-19's line: no hostile client).
- Carries met by number (section 4): SP-1 met (ws_server.rs:619-682, ws_gate.rs:25-38); SP-2 met through WsLink
  (`b4_the_writer_keeps_outstanding_within_n_snapshots_through_ws_link`, ws_server_tests.rs:701-722); SP-3 met
  (`b4_the_plain_path_keeps_siocoutq_within_n_snapshots_and_stops`, :864); SP-4 met (ws_gate.rs:61-72,
  ws_server.rs:244-259, main.rs bite arming in `Net::bind`); SP-5 met (`begin_tick`, above); SP-6 MOVED to U6e
  (condition 5); SP-7 met (the reads run in `send_tick` on the session's I/O task, tick_loop.rs:874-883, run_session
  :953-957); SP-8 met (`ws_wrapper` first of `header_numbers_from`, delay.rs:1324-1327; weapon and health U6a's,
  present at main.rs:457-465). RC-1 met (once per tick, P8); RC-2 MOVED to U6e; RC-3 met (`header_numbers_from` once,
  before OPEN, main.rs:590-615); RC-4 met (`ConnCap::new(clients)` N + 8, main.rs:555; the bound's arithmetic short:
  NR-U6B-5); RC-5 CUT; RC-6 met (`ws_config(cfg.ws_max_message_bytes)` and `DelayStream::wrap(.., &self.cfg, ..)` from
  one cfg, ws_server.rs:324, :478; P7); RC-7 met (`timeout_at(accept + 2 s)` over TLS, the upgrade and the hand-off,
  ws_server.rs:441-450; over QUIC, HTTP/3 and the answer, wt_server.rs:201-232); RC-8 met; RC-9 Block 0's
  (security's); RC-10 met (`ws_split_pool_rule` and the mark, main.rs:153, :590-605); RC-11 met (ws_server.rs:852-854,
  P11); RC-12 met in code (above), its 48-connection example reading not built (the main session's choice, fix pass).
  C-U6-17's form: `u6b_clients_over_both_transports_join_at_p2_with_no_kind_10_refusal` (tick_loop/tests.rs, on the
  bench, not the binary). U6a's condition 6: OPEN after both listeners serve (main.rs:766-772, P14); `Kick(replaced)`
  before the close on both (P5, P6); the token read is security's. C-U6-19's line: stated in the end-to-end test's doc.
  §4.4's "npt-room sees `SendResult::BufferFull`" stays U6d's (the I/O side counts and does not read results,
  tick_loop.rs:890, U6a's deferral).
- Steps 2-8 DONE (22:1x UTC). Next: verdict, table, F-items, not verified.

## 3. Verdict: APPROVE WITH FINDINGS
No Critical or High, so no second round (NPT-TRIM). Two Mediums, each a small fix for the fix pass before the merge
(NR-U6B-1: the end's order; NR-U6B-2: the gate's tick period from the caller). Twelve Lows: fix in the same pass where
cheap (NR-U6B-3, -4, -5, -7, -8, -9 are a few lines or report text), else carried by number to their takers
(NR-U6B-10 to U7/U8's record step or U6e, -11 to U6d, -6 to U8's next edit with a one-line server field, -12 to the
test seat's list, -13 and -14 to U6d). The pub surface stays inside §4.7 (additions only); fix 1's path, the
all-or-none reservation and the reader stamp are right as built.

## 4. Findings

| Id | Sev | Where (under W) | Finding | Smallest fix |
|---|---|---|---|---|
| NR-U6B-1 | Medium | crates/server/src/main.rs:777-779 (and :774-775); crates/net/src/ws_server.rs:856-858 | `Net::end` writes the summaries and `ws_split` before `TickLoop::stop` closes the sessions; a WsLink's `ws_summary` is pushed only at its closing's end, which nothing reads after `Net::end`. Every WebSocket connection open at the room's end (all 48 in R1's LG-loaded rooms, which end while LG runs) gets no `ws_summary`, and `ws_split` is read while they still release records. | End the sessions first: a TickLoop step that sets stop, joins the tick thread and awaits the session tasks within FLUSH (as `shutdown` does) while the writer stays open (e.g. `stop_with(f)` calling `f(&self)` after the tasks end and before `self.events = None`), then `Net::end`, then the log's end. A binary test: a WsClient still connected when stdin closes gives one `ws_summary`, and `ws_split` comes after it. |
| NR-U6B-2 | Medium | crates/net/src/ws_server.rs:74-75, :498, :583 | The gate's N uses a literal 30 Hz tick (`TICK_NS = 33_333_333`), a second home for the tick rate against plan §4.6 (one tick constant; the 60 Hz run changes npt-sim only). At 60 Hz N is sized for 30 Hz (6 instead of 10 at srtt 100 ms). | `Listen` (or `WsServer::bind`) takes `tick_ns`; npt-server passes `1_000_000_000 / u64::from(npt_sim::TICK_HZ)`; every `TickWriter::new` takes it; `TICK_NS` stays in tests only; one test with 16,666,666 ns gives N = 10 at srtt 100 ms. |
| NR-U6B-3 | Low | crates/net/src/ws_server.rs:646, :650-660, :684-711; crates/link/src/lib.rs:115-121 | F-PING's built rule: the outside counts are cfg(test) only (never logged); condition 1 counts the link's own queue, so a ping or pong sent in the same 1 ms poll as a tick's batch is refused (about 1 in 33 on a healthy link, by reading); no size bound, so a snapshot sent without `begin_tick` would pass conditions 1-2; the trait doc does not state the rule. | Outside datagrams: refused (`BufferFull`, counted) above the largest Ping/Pong encoding; condition 1 without the link's own queue (owed below it still counts); both counts in `event.ws_accept` (two new SERIES rows); the trait doc names the rule; the advisor records it as replacing condition 1's sentence. |
| NR-U6B-4 | Low | crates/net/src/ws_server.rs:664-687; crates/link/src/lib.rs:118-121 | An empty `lens` is gated as a 0 B tick and counted skipped when anything is outstanding; `TooLarge` returns before `announced` is decremented, so the next outside datagram takes the tick's slot (ungated if the tick was allowed). Neither happens with today's TickLoop. | `begin_tick(&[])` returns `Sent` without a tick; decrement `announced` before the size check; the doc says "never with no datagrams, and nothing else sent between `begin_tick` and the tick's last datagram". |
| NR-U6B-5 | Low | crates/net/src/l2/delay.rs:213, :221, :303-306, :1362 | The split's pool grew from 10 to 16 histograms and the mark copies 16 (2.16 MB once taken); `ws_split_fixed_bytes` still reports 10 (its row's meaning), and no header row reports the added heap (advisor's Q-U6B-3 term (1)); the builder's notes miss the reader's 4 + 4 (about 1.08 MB) in F-POOL-BYTES and the memory bound. | One new header row (e.g. `ws_split_added_bytes` = 6 x 135,040 B and the mark's 16 x 135,040 B, or two rows) written from `Pooled::heap_bytes()`; the report's memory bound recounted. |
| NR-U6B-6 | Low | crates/net/src/l2/delay.rs:1316; crates/server/src/main.rs:636-641 | The mark's stamp is on DelayStream's clock only, so the mark cannot be placed on the tick rows' (room) clock, while R1.md's INVALID rule counts connections "up 60 s before the mark". | npt-server's mark task records `TickLoop::now_ns()` at the mark and the `ws_split` event carries it (one new row, e.g. `ws_split_mark_room_ns`); U8's next edit reads it. |
| NR-U6B-7 | Low | crates/server/src/tick_loop.rs:575-576, :583-588; marks/SERIES.md:330 | `tick.v.ws_skipped_snaps_count` as built is the count since the last row (the previous tick's refusals), not "this tick"; a dropped row loses its share. | One sentence in the U6b section's lead text states the one-row shift and the dropped-row case; the main session decides whether :330's wording changes (not U6b's lane). |
| NR-U6B-8 | Low | crates/net/src/lib.rs (re-exports), crates/net/src/l2/config.rs:121, crates/net/src/l2/delay.rs:1316, crates/server/src/tick_loop.rs:1269, :1508-1529, crates/proto/src/lib.rs (end) | New pub items beyond the brief's list: `Listen`, `Accepted`, `mark_split`, `header_texts_from`, `L2Config::named`, `WtServer::socket`, `WsServer::bind_pem/wrapper/take_summaries/event_rows`, `with_bite`/`WS_BUDGET_BITE` (bites), `Connector` and four `TickLoop` functions, the three proto constants. All additions in std or own types. | The report names each with its reason (the brief's rule); no code change. |
| NR-U6B-9 | Low | u6b-build.md:119-121 (F-VALVE) against plan.md:1256 | The reader stamp is R1's: plan.md:1256 puts "the split ... plus U6's reader stamp" in R1's DelayStream self-test and its miss route; only ws-budget-x2 is R3's. | The report calls the reader stamp R1's (built), and U8's next edit names the reader rows (`ws_late_up_{pass,reader,other_pass,other_reader}_*`, `ws_late_up_early_count`) beside pooled L. |
| NR-U6B-10 | Low | crates/net/src/wt_server.rs:211-226 | F-NEG: the SETTINGS are dropped by the crate, but the CONNECT's raw headers reach `Request` and the crate's answer is always draft02, so a server-side draft record is possible without a manifest change. | A per-room count of sessions whose CONNECT named a draft-02 header (presence only; security confirms E22) in `event.wt_accept`, and the answered draft in the header; or carry -56 item 2 by number to U7/U8's record step. |
| NR-U6B-11 | Low | crates/link/src/lib.rs:112-113; crates/net/src/wt_server.rs:409-412; crates/net/src/ws_server.rs:742-744 | `unsent_unreliable`'s unit is datagrams by the trait; WtLink gives bytes (quinn exposes only free space), WsLink 0 though its own queue holds datagrams. No reader before U6d. | The trait doc states the unit per backend (or U6d's brief carries it by number); WsLink counts the datagrams in its own queue. |
| NR-U6B-12 | Low | crates/net/src/l2/stats.rs:812-837 | Condition 3's "identity exact across the mark" is tested on one Pooled only, not the split's parts against pooled L over one marked window. | One test: records before and after a `Pool::mark`, then the window's parts' sums against pooled L's sum (exact with no traffic during the mark). |
| NR-U6B-13 | Low | crates/server/src/main.rs:627-635 | A session the loop refuses (`Connector::connect` false) is dropped uncounted, after the transport counted it accepted. | Count it (e.g. `room_refused_count` in each transport's event, one new row each). |
| NR-U6B-14 | Low | crates/net/src/ws_server.rs:633-641, :830-834 | The plain path's `framed - plain` term saturates and drifts low after tungstenite's own frames (pong replies, Close), which `framed` never counts. | Count what tungstenite writes from the stream's side only (`plain` against a `framed` that includes its own control frames), or drop the term and rely on `queued` plus `tls - sock`, stated. |

Counts: Critical 0, High 0, Medium 2, Low 12.

## 5. The F-items
- **F-PING (condition 1's letter against the built rule): the built rule is right; the letter must not hold.** The
  letter ("a datagram send no current `begin_tick` announced ... is answered BufferFull and counted") would refuse every
  ping and pong on WebSocket: the room's RTT for a WebSocket session comes from its own pings (`RttSource::Pings`,
  main.rs:634) and the client's clock sync from the room's pongs, so the WebSocket path would run with no RTT and no
  clock sync at all. The letter's purpose (no snapshot written ungated) holds as built only because TickLoop announces
  every snapshot datagram, which `u6b_each_tick_is_announced_whole_and_a_refused_one_is_counted` proves by decoding
  every outside datagram as Ping or Pong. My fix (NR-U6B-3): keep the rule, and (1) refuse an outside datagram above
  the largest Ping/Pong encoding (BufferFull, counted), so a snapshot sent without `begin_tick` can never pass; (2)
  leave the link's own queue out of condition 1 for outside datagrams (stuck bytes below it still refuse); (3) write
  both counts to the log; (4) state the rule in the trait doc. The advisor records the ruling as replacing condition
  1's sentence in the brief's next revision.
- **F-MARK-HDR (condition 3): right.** The header is written before OPEN and the mark after it, so "whether it was
  taken" can only be in the event; the planned time in the header (`ws_split_mark_after_open_ns`) plus
  `ws_split_mark_set` meets RC-10's "stated in the run's header" in purpose. Missing: the mark on the room's clock
  (NR-U6B-6).
- **F-NEG: half right.** The SETTINGS are dropped (settings.rs:46-57), but the CONNECT's headers are exposed and the
  answer is fixed at draft02; a presence-only count is possible (NR-U6B-10). If not built now, -56 item 2 needs its
  source named by number in U7's or U8's record step, else it reads no value at R-10.
- **F-POOL-BYTES: keep `ws_split_fixed_bytes`'s meaning, but the count is short.** The pool adds 6 histograms (2 pooled
  L, 4 reader parts), not 2, and the mark copies 16, not 12 (NR-U6B-5); a new header row carries the added bytes, and
  the report's memory bound adds about 1.08 MB.
- **F-WT-UNSENT: bytes is the only honest reading quinn allows** (it reports only free space); accepted if the trait's
  doc or U6d's brief states the unit per backend (NR-U6B-11), before U6d's rate adaptation reads it.
- **F-SKIPPED-ROW: the built meaning is sound** (the sum over a run is exact; each refusal lands in the next row), but it
  differs from SERIES.md:330's "this tick": state the shift (NR-U6B-7). -18 only records it, so no mark moves.
- **F-RT-IO: right and needed.** The listeners' sockets and quinn's driver need the I/O driver on the I/O runtime;
  `enable_all` adds no thread. With `io_handle`, the doc text and `stop` on the main thread outside any runtime context,
  NR-U6A-7 is met.
- **F-VALVE: half right.** ws-budget-x2 is R3's (judged at R3's L2 B1 ws part), so it was a fair valve item; the reader
  stamp is R1's by plan.md:1256 (NR-U6B-9). Building both under the 5,400-line valve (4,801 at the decision,
  u6b-build.md:80-81) was right.
- **F-JOIN-ONCE: right; what U6c's glue must do.** npt-client's endpoint sends its `Join` once, at its first step, and a
  Connecting WtClient/WsClient answers BufferFull, so U6c's glue (npt-bots, npt-load) must (1) step each endpoint only
  once its link reads Open, never while Connecting; (2) never step a link that reads Closed first (a failed handshake:
  count it, never step); (3) step at once after Open, because the 2 s Join deadline runs from the server's accept stamp
  (first QUIC packet or TCP accept) and the handshake already spends 2-3 round trips of it (QUIC plus CONNECT; TCP,
  TLS 1.3 and the upgrade); (4) U6c's tests assert no session ends at the Join deadline (the room's decoder counter).
  U7's page does the same on `ready`. The alternative, WtClient and WsClient queuing reliable sends while Connecting
  (bounded by their send queue), would serve every host at once, but it changes npt-net after U6b: U6c's brief picks
  one.

## 6. What I did not verify
- No cargo command run by me: every finding is read from the code; the "about 1 in 33" pings and pongs refused
  (NR-U6B-3) is arithmetic from POLL (1 ms) and the tick (33 ms), not measured.
- The container readings (20 starts to OPEN, the 48-connection WsServer example at P2 for 60 s, the memory bound) are
  not built (the main session's choice, before Stop B's read); so the reader stamp's four parts and pooled L have never
  been read under 48 connections, and NR-U6B-1's loss was not seen in a run.
- I did not read wt_server_tests.rs, cert.rs, binary.rs's other tests, or the plant logs in full (the test and security
  seats'), nor the security reads (the cap and deadlines as a security cap, the pin, the key, the token file).
- No browser: whether Chrome's or Firefox's CONNECT carries a draft-02 header (NR-U6B-10) is from web-transport-proto's
  own comments, not observed; U7's headless Chromium is the first contact.
- The mark's cost on an I/O worker (16 x 8,440 bucket loads and a 2.16 MB allocation once) was not measured; by
  arithmetic it moves the self-test's E by far less than 1 µs.
- R1.md (wip/u8-build.patch) is a work-in-progress snapshot; its lines cited here may move.

DONE
