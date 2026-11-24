# U6c review, netcode seat (xhigh), one round (NPT-TRIM)

Inputs checked by sha256 (all five equal the launch's): u6c-brief.md 8f56f37a..., advisor-u6c-brief.md 9f5ab8d0...,
u6c-launch-prompt.txt bf8016a8..., wip/u6c-build.patch a4e63455..., u6c-build.md 426b31da.... No netcode-architect
memory folder exists (.claude/agent-memory-local holds security-auditor and test-engineer only). W =
/home/user/Shamble-Frontlines/.claude/worktrees/agent-a3a3e3fdf559de095/spikes/netcode-proof (read only).

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
