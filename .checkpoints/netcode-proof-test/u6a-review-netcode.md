# U6a build review, netcode seat (round 1 of 1, NPT-TRIM), 2026-11-22

Inputs: u6a-brief.md rev 1.1 (sha256 c109691a... checked), wip/u6a-build.patch (sha256 a9f8b6e0... checked; base
aa5c62b6), built tree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a95ebf14368f277ff (read only).

## Plan
1. Read the brief's 0.1, 1-4, 7; advisor-u6a-brief.md (Q-U6A rulings); u6a-build.md (F1-F9, O1, O2).
2. Read the patch file list; then each new file in the worktree: tick_loop.rs, phase.rs, clock.rs, main.rs (settings,
   tokens), S4 readers (steal, rss, schedstat, affinity, taskset), room log writer, npt-super, client plan() cap,
   bots host_step, proto channel_of, room ClientSlot::relevant, log Machine::read_local, link Loopback Send.
3. Check scope items one by one; a notes line per finding, saved after each.
4. Read the evidence logs (check3, plants, starts) only to confirm claims I lean on.
5. Verdict, findings table, views on F1, F3, F4, F7, F9, O1, O2, not verified, DONE.

## Notes (one line per finding, as found)
