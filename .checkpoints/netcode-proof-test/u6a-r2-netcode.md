# U6a round 2, netcode seat (NPT-TRIM: what the fix pass changed only), 2026-11-23

Inputs: u6a-fix-pass.md (FP-1 to FP-12); u6a-build.md's fix-pass lines (:212-346); wip/u6a-build.patch (sha256
022ddb84d01ec3b913917919d2052bc91c8f54ac1c2c1a8754bbf018bbef98e1 checked; 5,843 added); .gate/u6a/fp*.log, check4.log,
fp-restored.log. Tree: W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-a95ebf14368f277ff/spikes/netcode-proof
(read only). No Edit tool here: this file is rewritten whole with Write after each block.

## Plan
1. NR-U6A-1: super/src/main.rs flags, Flags, log() header; the new npt-super test; binary.rs's double-load bite;
   against judge eval.rs:47-61, :542-564 (and :576-584 footer); is the U8 carry all U8 needs.
2. NR-U6A-2: TickLoop::io_on_a_tick_sibling and main.rs sibling_refusal(), its place in run(), its test.
3. NR-U6A-3 and -4: proc_read alone(), sibling_free_read(), cpu_ns Option; tick_loop affinity(), Seconds values/write;
   the "no jiffy" gap side effect.
4. FP-5: the two counted offers.
5. FP-6: counted()'s lab rule; the 90-tick outcome.
6. Anything the fixes broke in round 1's scope (diff the changed hunks: FP-7 to FP-12 only as they touch my scope).
7. Verdict, NR2 table, not verified, DONE.

## Notes (one line per item, as checked)
