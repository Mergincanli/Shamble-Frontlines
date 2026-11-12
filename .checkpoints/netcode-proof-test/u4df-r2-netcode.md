# U4d-F round 2, netcode-architect (read only; log prefix nc2)

Started 2026-11-12. Packet: u4df-r2-packet.md. Copy: .claude/worktrees/wf_9ce88fa3-452-1 (uncommitted). Round 1 files:
.gate/u4df-r1.

## Plan
1. Confirm the diff (copy vs round 1) is exactly FX-1 (replay.rs tests module) and FX-2 (scene.rs two docs); nothing
   else changed (git diff in the copy, diff --no-index against .gate/u4df-r1).
2. Read the generator (`Recording::synthetic`, replay.rs) and the World's input path (world.rs `apply_inputs`,
   `move_velocity`, phase order, held-back / late commands, MoveSpeed clamp, spawn / teleport) and check FX-1 integrates
   what the World does with the recording's inputs, from the recording alone, through the World's own move rule.
3. Check each of the six counts measures what its name says and that each plant Q1-Q5 would be caught (reason from
   the generator code; read the builder's fx-* logs as claims; run FX-1 once in host through the runner).
4. Check FX-2's words against T3 (exact), against the measured facts (round 1 notes [9]-[10], NF-L2 numbers), and the
   120-char wrap.
5. Verdict per finding: TF-M1, NF-M1 (wording point), NF-L2 (wording point). New defects with severity, file:line.

## Notes (one line per finding as it lands)
