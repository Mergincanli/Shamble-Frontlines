# U6c review, netcode seat (xhigh), one round (NPT-TRIM)

Inputs checked by sha256 (all five equal the launch's): u6c-brief.md 8f56f37a..., advisor-u6c-brief.md 9f5ab8d0...,
u6c-launch-prompt.txt bf8016a8..., wip/u6c-build.patch a4e63455..., u6c-build.md 426b31da.... U8's side read from
wip/u8-build.patch as it stands (sha256 0e7fbab2...) and u8-build.md. No netcode-architect memory folder exists
(.claude/agent-memory-local holds security-auditor and test-engineer only). W =
/home/user/Shamble-Frontlines/.claude/worktrees/agent-a3a3e3fdf559de095/spikes/netcode-proof (read only). This agent
has no Edit tool, so this file is rewritten whole with Write at each save. One runner call (rev-netcode-1.log).

## 1. Plan
1. Read the brief's binding sections (0.1, 1-4, 7), the advisor's check where it touches the netcode seat, the launch's
   items 1-6, and the builder's notes (account, not evidence).
2. Read under W, in this order: crates/net/src/l2/probe.rs (poller, FateEntry, samples/forms, ProbeDatagram, Pacer,
   probe_peer, leg_series) and socket.rs's `pretouch_fate_log`; crates/server/src/probe.rs and main.rs's probe mode;
   crates/bots/src/load.rs (glue host order, clock, F-JOIN-ONCE, header/footer, NC-SB-7/-2); the SERIES.md section;
   the tests of each (probe_tests.rs x2, load_tests.rs, tests/load.rs, tests/binary.rs).
3. Check against: C-U6-1 to -7 (brief section 4); SERIES.md:497-501; the E fields and leg-edge names (Q-U6C-5, C1 (d));
   the new pub items against plan §4.7; the glue's host order and clock; added items 2 (six NC-SB/INF-SB lines), 3
   (NC-R2-4), 4 (F-JOIN-ONCE), 5 (A2's command); the SERIES.md section against U8's probe_presence (u8-build.md's
   interface lines; wip/u8-build.patch's ops/ files); the carries in section 4 by number.
4. View on each F-item (F-ECHO-RACE, F-PORT, F-POLLER-STATE, F-WT-CLOSE, F-JOIN-DEADLINE-READ, F-LG-SHA, F-TERM), plus
   F-P8 which the notes add.
5. Tests only where a claim needs one, through the runner.
6. Write findings as found (section 2), then verdict, table, F-item views, not verified, DONE.

## 2. Notes (one line per finding, saved as found)
- N1 HIGH (NC-U6C-1): the window is a span of RS's socket clock, and each direction's records are the fates whose
  t_in falls in it (server probe.rs:449-450, :557-563; net probe.rs:692, :733). PM-0001-3 judges ten counts at
  `>= 1000000` (marks.json, "P1 up: 1,000,000 packets" to "B1 down": `second.s.probe_*_lost` count, parts L1 and L2).
  Up's t_in are LG's sends on LG's clock plus path and reader jitter: over 100 s the count is 1,000,000 plus or minus
  the edge jitter (one datagram either way) and minus LG's clock-rate error against RS's (1 ppm = 1 datagram; fresh
  VMs on the run day); B1's outside-burst count has the burst's two edges as well. Down's edges hold unless the pacer
  is 50 µs late at an edge (a late wake sends its catch-up burst at one stamp, so several can fall out). So each up
  check falls below 1,000,000 at random, and all five do if LG's clock runs a hair slow: PM-0001-3's L2 parts likely
  FAIL at R1 on an instrument edge, not on the network. The e2e test accepts 1,800..=2,100 for 2,000 packets (server
  probe_tests.rs:313-319), so nothing catches it. L1 counts by send index (lab/src/probe.rs:121-131).
- N2 MEDIUM (NC-U6C-2): A2's named command (u6c-build.md:73-77; tests/load.rs:288) passes `--exact` with the filter
  `a2_`, which matches no test: `cargo test --locked -p npt-bots --test load a2_ -- --ignored --exact --list` printed
  "0 tests, 0 benchmarks" (.gate/u6c/rev-netcode-1.log, 10:22 UTC). The main session's close reading would run
  nothing and exit 0.
- N3 checked, no finding: the RTT join. Up Delivered fates are logged in poll_recv's pop order (socket.rs:532-556) and
  the one poller hands datagrams to `Intake::take` in that order, so the k-th Delivered up fate is the k-th record;
  other sources' and refused datagrams keep their place (OTHER, NO_ECHO). The lateness set {0, 8, 9} equals the
  histogram's (impair.rs:680-684). Echo indices from LG reach only `get` (net probe.rs:748), the down table is bounded
  by the log's length (:706), the record list by its capacity (server probe.rs:297-301): nothing unbounded keyed by
  peer data.
- N4 LOW (NC-U6C-3): the down join reads the fate's flow index as the datagram's index (net probe.rs:704-712), but
  QueueDrop and PoolDrop take no index (impair.rs:15-17, :489-512): after one such down drop every later round trip is
  joined to a later datagram's t_in (100 µs short per drop), and nothing counts it. Latent at R1's profiles (P4's
  longest one-way 120 ms queues 1,200 a flow, under 2,048: config.rs:107-109).
- N5 MEDIUM, outside U6c's diff (NC-U6C-4): L2's one-way delays are ImpairSocket's stamps (injected + L), and the
  injected one-way is (profile RTT − base) ÷ 2 (config.rs:24-25), while PM-0001-3's jitter-end bounds are the
  profile's ± 5 % of the half jitter (P1: p001 >= 19.75 ms, p999 >= 29.75 ms). P1's p001 is about 20.01 ms − base/2 +
  L, so a measured P0 base above about 0.5 ms (two VMs, both release threads, LG's reader and RS's poller in it) fails
  P1's low ends; P2-P4 have wider bands (0.5, 0.75, 1 ms). The RTT checks are the ones the base was built for (R-9,
  "fails MP's 100 ± 5 ms"). To the advisor before R1's run plan, not U6c's to fix.
- N6 LOW (NC-U6C-5): F-JOIN-ONCE (2)'s count: a client whose link closes or stays Connecting past 10 s is state 2
  (load.rs:142-148) but no record or line carries the count; the run then ends at READY's 30 s with "no READY"
  (load.rs:658-659, :620-624). R1 sees the exit, not how many failed.
- N7 LOW (NC-U6C-6): NC-SB-2's "footer on every end": two exits after the header is written return with no footer: a
  client's connect error (load.rs:588-591) and probe_peer's bind error (load.rs:560-562).
- N8 LOW (NC-U6C-7): pub with no consumer outside its crate: npt-net `samples`, `Samples`, `forms`, `Forms`, `excess`,
  `nearest_rank` (net l2.rs export; used only inside npt-net and socket_tests); npt-server `leg_log` (server lib.rs
  export; used only in probe_tests); npt-bots `now_ns`, `Hosted`, `HostState` (bots lib.rs:36-37; the bin uses only
  `LoadSettings` and `run_load`). §4.7 keeps the rest of npt-net pub(crate) (plan.md:610-613). The brief listed most
  of them, so this is a narrowing for the next change, not a defect. Beyond the brief's list, each with a consumer:
  `send_down`, `Pacer` (its return), `Polled`, `ProbePeer`, `LegSeries`, `L2Config::b1`, `PROBE_LEGS`, `PEER_WAIT_NS`.
- N9 LOW (NC-U6C-8): new npt-log names with no SERIES.md row: LG's `probe_peer` event fields `probe_peer_sent_count`,
  `_send_err_count`, `_received_count`, `_recv_err_count`, `_refused_count`, `_echoed_count` (net probe.rs:577-584,
  load.rs:606-612) and LG's header params `room` and `mode` (load.rs:441-448). The section's text names the event
  (SERIES.md new section, last sentence) but has no row for it; the e2e row check reads RS's logs only (server
  probe_tests.rs:272-282). 27 name rows plus the header line.
- N10 LOW (NC-U6C-9): LG's phase is Measured from READY (load.rs:665-669), so the first measured second holds only part
  of the last-joined client's second, and on the room's close the last record is a part second too (load.rs:661,
  :689-692); V-S sums measured seconds × tick_hz within 1 % (eval.rs:744-777), so up to about 60 inputs a client go
  missing: a fail for any V-S run under about 200 s. Not R1 (V-S is R2's and R3's: marks.json validity lists).
- N11 LOW (NC-U6C-10): the glue passes no view (load.rs:159-161), so the brawl aims at its blind point
  (bots/src/lib.rs:1152-1166) on every network run; U8 hosts `brawl` everywhere (u8-build.patch:2186). The lab's L1
  hosts give it a view from a mirror (lab/src/scenario.rs:1672-1681). Harmless for R1's marks; R2's stress load and
  U6e's -30 runner (which reuse this glue) should decide whether a network brawler takes its view from its own
  endpoint's decoded state.
- N12 LOW (NC-U6C-11): F-P8: the end-edge read comes before the drop by order only (server probe.rs:549 then :564) and
  the plant cannot fail any test (plant-p8.log). `edge_events` taking `&ImpairSocket` (unused but borrowed) would make
  a read after `drop(sock)` fail to compile: a bite by type, no new pub item.
- N13 LOW (NC-U6C-12), the merge's alignment line (C9, NC-R2-4): U8's probe_presence looks for P1's slack and policy
  in the `leg_edge` events at both edges (u8-build.patch:2447, :2477-2478), while U6c writes them in the P1 header:
  `l2_timer_slack_ns` and `l2_sched_policy` (the end edge's header read) and `probe_start_timer_slack_ns` and
  `probe_start_sched_policy` (10 ms before the window: server probe.rs:509-517, :582-586). As built, every P1 would
  read INVALID; U8's reader must move to those four header names at U6c's merge. U6c's placement is plan.md:1256's
  pair (the window's start edge and the header read), so the change is U8's.
- N14 checked, no finding: U8's load_settings writes exactly LoadSettings' 24 fields (u8-build.patch:2180-2186 against
  load.rs:197-222, `deny_unknown_fields`); room 02's wt_port 30003 is the probe's wt_bind; leg logs
  `room-02<att>-<leg>.jsonl` match `leg_log` (server probe.rs:72-78) and probe_presence's open; the header's run,
  profile, process and layer (:565-569) and the footer (:352) match its reads; `udp_row_found` 1, `udp_drops_count`,
  `cpu_user_count` and the window counts are written at both edges as it reads them.
- N15 checked, no finding: the C-U6-7 shape. Inside the window the leg calls only `take_second` once a second and the
  steal read (server probe.rs:502-508); the start-edge read sits 10 ms before the window (:538); header_numbers,
  header_texts, run_summary and the drain run after the tail with the socket bound (:549-556); the log is written
  after the leg (:405-407). The drain's reserved Vec is first touched under the lock (:555-556), so its hold includes
  the page faults; it is reported, after the after-edge read, so a kernel drop it causes is outside P1's two reads.
- Notes DONE.

## 3. Verdict: REVISE
One High in U6c's own code (NC-U6C-1: the window's records are a clock span, so PM-0001-3's ten `>= 1,000,000` counts
fail at random at R1), one Medium in U6c's evidence (NC-U6C-2: A2's command runs no test), one Medium outside the diff
for the advisor (NC-U6C-4), and ten Lows. The rest of the netcode scope holds: C-U6-1 to -7 are met in form, the E
fields and leg-edge names are Q-U6C-5's, the glue's host order is C-U6-18's one entry point with one clock item, and
F-JOIN-ONCE (1) to (4) hold. A fix pass for NC-U6C-1 and -2 (with a test and a plant for the first), then a read of what
changed (the open High allows a second round under NPT-TRIM), would make this APPROVE WITH FINDINGS.

## 4. Findings
| Id | Severity | Finding | File:line under W | Smallest fix |
|---|---|---|---|---|
| NC-U6C-1 | High | Each direction's measured records are the fates whose t_in lies in a span of RS's clock, so the count is `packets` plus or minus the edge jitter and LG's clock-rate error; PM-0001-3's ten `>= 1000000` counts (up most of all, and B1's outside-burst counts) fail at random at R1; the e2e test accepts 1,800-2,100 for 2,000 | crates/server/src/probe.rs:447-450, :557-563; crates/net/src/l2/probe.rs:684-763 (:692, :733); crates/server/src/probe_tests.rs:313-319 | Define each direction's records by count, as L1 does: from the first fate at or after the window's start, in flow-index order, until `packets` fates lie outside the bursts (down: datagram index from warm / interval; up: the peer's flow), the tail already covers the extra sends; `probe_window_ns` per direction the span those records cover, for C-U8-4's rate. Cheaper alternative: a 10 ms margin on the window (100 datagrams: about 100 ppm of drift). Test: fixed fates with up spaced 100 µs × (1 + 5 × 10⁻⁶) and a late pacer at the end edge give at least `packets` a direction outside the burst; plant: the clock span back, which must fail it; the e2e bound becomes `>= packets` |
| NC-U6C-2 | Medium | A2's command selects no test (`--exact` with the filter `a2_`): "0 tests, 0 benchmarks" (rev-netcode-1.log) | crates/bots/tests/load.rs:288 (and u6c-build.md:73-77) | Drop `--exact` (no other test name starts `a2_`) or give the full name `a2_48_websocket_clients_at_p2_against_one_room`; fix the doc line and the notes' command |
| NC-U6C-4 | Medium (outside the diff) | L2's one-way delays are injected + L with the injected one-way net of base/2, against bounds of ± 5 % of the half jitter: P1's p001 and p999 low ends fail once P0's base passes about 0.5 ms | crates/net/src/l2/config.rs:24-25; marks.json PM-0001-3 checks | To the advisor before R1's run plan (a reading of the bound or of the one-way's form on L2); not U6c's |
| NC-U6C-3 | Low | The down join takes the flow index as the datagram's index; QueueDrop and PoolDrop take none, so after one host drop every later RTT is joined to the wrong datagram, uncounted (latent at R1's profiles) | crates/net/src/l2/probe.rs:704-712; impair.rs:489-512 | Count down fates with no index in the summary (`probe_down_unindexed_count`), or map datagram i to the i-th down fate in t_in order |
| NC-U6C-5 | Low | F-JOIN-ONCE (2): failed handshakes counted in memory only; R1 sees "no READY", not how many | crates/bots/src/load.rs:142-148, :658-659 | One count in LG's footer-time event or the failing line (`handshake_failed_count`) |
| NC-U6C-6 | Low | Two exits after the header is written leave no footer (a client's connect error, probe_peer's bind error), against NC-SB-2's "every end" | crates/bots/src/load.rs:560-562, :588-591 | `w.finish(Ended::Stopped)` before those returns |
| NC-U6C-7 | Low | Pub items with no outside consumer: npt-net's six E helpers, npt-server `leg_log`, npt-bots `now_ns`, `Hosted`, `HostState` (§4.7's narrow surface) | crates/net/src/l2.rs (export), crates/server/src/lib.rs (export), crates/bots/src/lib.rs:36-37 | pub(crate) at the next change of each crate, or named in the report with the brief's reason |
| NC-U6C-8 | Low | No SERIES.md rows for LG's six `probe_peer_*_count` fields and its params `room` and `mode` | crates/net/src/l2/probe.rs:577-584; crates/bots/src/load.rs:441-448, :606-612; marks/SERIES.md (new section) | Three rows in the new section |
| NC-U6C-9 | Low | LG's Measured phase starts mid-second at READY and may end on a part second; V-S loses up to about 60 inputs a client, a fail under about 200 s (R2, R3) | crates/bots/src/load.rs:661-692 | Measured from the first whole second after READY; a part last second written `after` |
| NC-U6C-10 | Low | The glue gives the brawl no view, so every network brawler aims blind | crates/bots/src/load.rs:159-161; bots/src/lib.rs:1152-1166 | Decide at R2's run plan or U6e (the view from the endpoint's own decoded state); nothing for R1 |
| NC-U6C-11 | Low | P8 cannot bite: the end-edge read's place before the drop is by order only | crates/server/src/probe.rs:176, :549, :564 | `edge_events(&sock, port, ph)` borrowing the socket, so a read after the drop does not compile |
| NC-U6C-12 | Low | U8's probe_presence reads P1's slack and policy from `leg_edge`; U6c writes them as four header params | crates/server/src/probe.rs:509-517, :582-586; u8-build.patch:2447, :2477-2478 | At the merge (C9, NC-R2-4): probe_presence reads header `l2_timer_slack_ns`, `l2_sched_policy`, `probe_start_timer_slack_ns`, `probe_start_sched_policy` |

Counts: Critical 0, High 1, Medium 2 (one outside the diff), Low 10.

Carries met by number (section 4): C-U6-1 met (task on the caller's runtime, 32 × 1,472 B until Pending, net
probe.rs:40-85; T1 on it, P1 fails T1). C-U6-2 met in the code's shape (a socket bound a leg and dropped,
server probe.rs:456, :564; no `set_default`; the never-mix test is npt-net's, its plant P2 in the test body: test
seat). C-U6-3 met by Q-U6C-2 (a) (one log a leg sized for the leg, pre-touched before the warmup edge, one drain after
the end edge, `probe_drain_hold_ns`). C-U6-4 met (`steal_permille` each second over `Cpus_allowed_list`,
`params.steal_cpus`). C-U6-5 met in names, forms, sample set and fixed-entry tests; "exactly -3's measured records"
waits on NC-U6C-1. C-U6-6 met by composition (10,000 fresh ups each with at most one echo; 10,000 downs; window counts
recorded). C-U6-7 met (N15). C-U6-8 not built (Block 2, lever (1): MOVED). C-U6-19's line holds (honest behaviours,
`clamp` aims beyond reach as the sim allows; no rejoin). u6a condition 5 holds (StealReader pub(crate)); condition 3
struck by C1 (c), npt-load ends at `duration_s`. U8's C10: Q-U8-5's reads with C1 (d)'s names, the probe's port (room
02's wt_bind), npt-load's interface (one argument, U8's fields: N14). advisor-u6-brief condition 1: rows named, three
missing (NC-U6C-8); condition 8 U6-Q2: the glue after C-U6-18 with its clock allow. Added items: (2) NC-SB-1, -3, -7,
-9, -16 met; NC-SB-2 but for NC-U6C-6; INF-SB-6 met in form (key synced, chain to a temporary name, hard-linked into
place, which refuses an existing name: host issue.rs:170-175, :272-275; the rest is the security seat's); (3) NC-R2-4
named, U8 aligns (NC-U6C-12); (4) F-JOIN-ONCE (1)-(4) met (load.rs:137-158; READY and `deadline_closed_count` 0,
tests/load.rs:145-149, :197-210), its count unrecorded (NC-U6C-5); (5) A2's command broken (NC-U6C-2).

## 5. The F-items
- **F-ECHO-RACE: right.** Each up carries the newest down not yet echoed (`take()`, net probe.rs:531), so 8 and 9 are
  both echoed once when an up leaves between their arrivals: C3 holds, every sample is one whole round trip, and the
  pick does not depend on a datagram's delay, so the RTT mean is not biased. R1 loses nothing; no fix.
- **F-PORT: right.** npt-server names no quinn trait, and the leg binds exactly the settings' port (refused at 0:
  main.rs `probe_settings`), so the row by that port is the socket's own; at R1 it is room 02's 30003. R1 loses
  nothing. NC-U6C-11's borrow would also tie the read to a live socket.
- **F-POLLER-STATE: right.** Returning the state with the count keeps a lock out of the poller's per-datagram path; a
  panic in the closure loses the state and the leg fails with the poller's error (server probe.rs:545-547), which is
  the right end. R1 loses nothing; no fix.
- **F-WT-CLOSE: right in effect, its cause likely but unproven.** WtServer's Drop closes the endpoint with no
  `wait_idle` (wt_server.rs:205-215) before the process exits; RoomEnded most likely reaches the client (the control
  stream is finished and its acknowledgement awaited, wt_server.rs:538-545) and the endpoint records it
  (client/src/lib.rs:412-418, pub(crate)), but the glue cannot read it, so a WT client keeps stepping for up to 30 s
  (b4-test2.log: wt 30.16 s against ws 3.31 s, from the test's earlier form). R1 loses nothing (LG ends at its
  duration; the probe block waits for the room's own exit), but LG's WT log gains up to 30 s of Measured seconds after
  the room's end. Smallest fix: the glue ends a client when its endpoint reports the room's end (npt-client's `closed`
  made pub, one item, with C-U6-14's session ending at U6d); `wait_idle` under FLUSH at npt-server's end is the other
  half (U6d/U6e).
- **F-JOIN-DEADLINE-READ: right.** READY needs every JoinAccepted, and a session closed at the Join deadline never gets
  one, so READY with `deadline_closed_count` 0 proves F-JOIN-ONCE (4) in these tests. R1 loses the reason when a
  client fails (no counter in any log: U6e's metrics). The brief's "no kind-10 refusal" line (section 3, Block 4)
  stays unproven, since LG counts commands given, not taken: U6c's report should name it open, for U6d/U6e's refusal
  counts.
- **F-LG-SHA: right.** The judge's build set (practice.rs:303-308) takes LG's "" in every run alike, so no verdict
  moves; R1 loses LG's binary identity in its own log, which U8's shell record gives. A hashing crate in npt-bots waits
  for a manifest change (DEP-HOLD); nothing now.
- **F-TERM: right, one correction.** No handler can be built (no tokio `signal` feature, no libc, unsafe forbidden).
  LineWriter writes through at newlines, but a record longer than its 1 KiB buffer (the header) reaches the file in
  parts, so a TERM can cut inside a line; U8's helper keeps the lines up to the last whole one (NC-R2-1), so nothing is
  lost. R1 loses nothing: `stop` is the abort path. No fix.
- **F-P8 (the notes' own): right.** The order is correct by reading; the fix that makes it bite is NC-U6C-11.

## 6. What I did not verify
- No probe ran here at any scale; NC-U6C-1 is arithmetic from the code and marks.json, not a measured shortfall. Its
  size rests on the edge jitter alone; the clock-rate part depends on RS's and LG's clocks on the day (not measured).
- NC-U6C-4's base on two Hetzner VMs is not known; the bound arithmetic is from config.rs and marks.json only.
- F-WT-CLOSE's cause (no CONNECTION_CLOSE sent) was read, not traced; whether RoomEnded reaches a WT client was
  reasoned from wt_server.rs, not observed.
- The page host, the issue mode, the token and pin reads, and the plants' coverage are the security and test seats';
  I read INF-SB-6's write order only. Block 2 (C-U6-8) is not built and not read.
- One runner call only (rev-netcode-1.log: a `--list`); I re-ran no test and no plant; the builder's end-check2.log
  and plant logs were not re-derived (P8's and b4-test2.log's lines were read).
- U8's side was read at wip/u8-build.patch 0e7fbab2... (after its fix pass), not at the advisor's e9fd605c....

DONE
