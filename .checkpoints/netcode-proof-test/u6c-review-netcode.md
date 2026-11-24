# U6c review, netcode seat (xhigh), one round (NPT-TRIM)

Inputs checked by sha256 (all five equal the launch's): u6c-brief.md 8f56f37a..., advisor-u6c-brief.md 9f5ab8d0...,
u6c-launch-prompt.txt bf8016a8..., wip/u6c-build.patch a4e63455..., u6c-build.md 426b31da.... No netcode-architect
memory folder exists (.claude/agent-memory-local holds security-auditor and test-engineer only). W =
/home/user/Shamble-Frontlines/.claude/worktrees/agent-a3a3e3fdf559de095/spikes/netcode-proof (read only). This agent
has no Edit tool, so this file is rewritten whole with Write at each save.

## 1. Plan
1. Read the brief's binding sections (0.1, 1-4, 7), the advisor's check where it touches the netcode seat, the launch's
   items 1-6, and the builder's notes (account, not evidence).
2. Read under W, in this order: crates/net/src/l2/probe.rs (poller, FateEntry, samples/forms, ProbeDatagram, Pacer,
   probe_peer, leg_series) and socket.rs's `pretouch_fate_log`; crates/server/src/probe.rs and main.rs's probe mode;
   crates/bots/src/load.rs (glue host order, clock, F-JOIN-ONCE, header/footer, NC-SB-7/-2); the SERIES.md section
   (28 rows); the tests of each (probe_tests.rs x2, load_tests.rs, tests/load.rs, tests/binary.rs).
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
  `>= 1000000` (marks.json, "P1 up: 1,000,000 packets" ... "B1 down": `second.s.probe_*_lost` count, parts L1 and L2).
  Up's t_in are LG's sends on LG's clock plus path and reader jitter: over 100 s the count is 1,000,000 plus or minus
  the edge jitter (one datagram either way) and minus LG's clock-rate error against RS's (1 ppm = 1 datagram); B1's
  outside-burst count has the same two edges again inside. Down's edges hold unless the pacer is 50 µs late at an
  edge. So each up check falls below 1,000,000 at random (and all five do if LG's clock is a hair slow): PM-0001-3
  likely FAILS at R1 on an instrument edge, not on the network. The e2e test accepts 1,800..=2,100 for 2,000
  (server probe_tests.rs:313-319), so nothing catches it. L1 counts by send index (lab/src/probe.rs:121-131).
- N2 MEDIUM (NC-U6C-2): A2's named command (u6c-build.md:73-77; tests/load.rs:288) passes `--exact` with the filter
  `a2_`, which matches no test: `cargo test --locked -p npt-bots --test load a2_ -- --ignored --exact --list` printed
  "0 tests, 0 benchmarks" (.gate/u6c/rev-netcode-1.log, 10:22 UTC). The main session's close reading would run
  nothing and exit 0.
- N3 checked, no finding: the RTT join. Up Delivered fates are logged in poll_recv's pop order (socket.rs:532-556) and
  the one poller hands datagrams to `Intake::take` in that order, so the k-th Delivered up fate is the k-th record;
  echo records of other sources and refused datagrams keep their place (OTHER, NO_ECHO). The lateness set {0, 8, 9}
  equals the histogram's (impair.rs:680-684).
- N4 LOW (NC-U6C-3): the down join reads the fate's flow index as the datagram's index (net probe.rs:704-712), but
  QueueDrop and PoolDrop take no index (impair.rs:15-17, :489-512): after one such down drop every later round trip is
  joined to a later datagram's t_in (100 µs short per drop), and nothing counts it. Latent at R1's profiles (P4's
  longest one-way 120 ms queues 1,200 a flow, under 2,048: config.rs:107-109).
- N5 MEDIUM, outside U6c's diff (NC-U6C-4): L2's one-way delays are ImpairSocket's stamps (injected + L), and the
  injected one-way is (profile RTT − base) ÷ 2 (config.rs:24-25), while PM-0001-3's jitter-end bounds are the profile's
  ± 5 % of the half jitter (P1: p001 >= 19.75 ms, p999 >= 29.75 ms). P1's p001 is about 20.01 ms − base/2 + L, so a
  measured P0 base above about 0.5 ms (two VMs, both release threads, LG's reader and RS's poller in it) fails P1's
  low ends; P2-P4 have wider bands (0.5, 0.75, 1 ms). The RTT checks are the ones the base was built for. To the
  advisor before R1's run plan, not U6c's to fix.
- N6 LOW: F-JOIN-ONCE (2)'s count: a client whose link closes or stays Connecting past 10 s is state 2
  (load.rs:142-148) but no record or line carries the count; the run then ends at READY's 30 s with "no READY"
  (load.rs:658-659, :620-624). R1 sees the exit, not how many failed.
- N7 LOW: NC-SB-2's "footer on every end": the early exits after the header is written return with no footer: a
  client's connect error (load.rs:588-591) and probe_peer's bind error (load.rs:560-562).
