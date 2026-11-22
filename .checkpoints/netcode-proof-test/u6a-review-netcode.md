# U6a build review, netcode seat (round 1 of 1, NPT-TRIM), 2026-11-22

Inputs: u6a-brief.md rev 1.1 (sha256 c109691a... checked), wip/u6a-build.patch (sha256 a9f8b6e0... checked; base
aa5c62b6), built tree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a95ebf14368f277ff (read only).
(No Edit tool in this seat: the notes file is rewritten whole with Write after each block.)

## Plan
1. Read the brief's 0.1, 1-4, 7; advisor-u6a-brief.md (Q-U6A rulings); u6a-build.md (F1-F9, O1, O2).
2. Read the patch file list; then each new file in the worktree: tick_loop.rs, phase.rs, clock.rs, main.rs (settings,
   tokens), S4 readers (steal, rss, schedstat, affinity, taskset), room log writer, npt-super, client plan() cap,
   bots host_step, proto channel_of, room ClientSlot::relevant, log Machine::read_local, link Loopback Send.
3. Check scope items one by one; a notes line per finding, saved after each.
4. Read the evidence logs (check3, plants, starts) only to confirm claims I lean on.
5. Verdict, findings table, views on F1, F3, F4, F7, F9, O1, O2, not verified, DONE.

## Notes (one line per finding, as found)
- Step 1 DONE: brief 0.1, 1-4, 7 read; advisor-u6a-brief.md Q-U6A-1..7 terms read (Q5's terms: env_clear, ids from
  /proc/thread-self, read-back, a failed pin written as 0 and on stderr, no tick vCPU = no pin and no event, the I/O set
  checked against SMT siblings at start with exit 2); u6a-build.md read.
- tick_loop.rs read whole (1,447). Clock: one item (clock.rs:21) with its allow; no other Instant/SystemTime in the
  diff (pump.rs:238's SystemTime is base's wallclock bite). No with_lead_bound_ms anywhere in the diff (LEAD-BOUND 400 ms
  holds: room.rs:547). Room has no time input at all (room.rs:899 pump_report, :912 receive): F1 confirmed at base;
  register 96's reader was "U4h's tolerance" (u4s-carries.md:628-632), and U4h left the spike.
- Closing order read: join() offers Kick then close() (store Release) at tick_loop.rs:661-667; poll() loads the flag
  (Acquire) BEFORE draining (:844-859), so the Kick is always drained before the link drops. Holds on Loopback.
- Candidate (Low, carry): the one inbound queue (INBOUND_CAP 4,096, tick_loop.rs:51) is shared by every session, and
  the decoder passes every Joined stream message other than Ping/Pong with no rate limit (pump.rs:476). One session's
  Resync flood fills it; another session's Join is then dropped and that session closed (tick_loop.rs:875-882).
  No hostile client in U6a (C-U6-19's gate line); for U6b/U6d: a per-session share per tick.
