# U4f-F1 review round, test-engineer seat (2026-11-20)

Diff: wip/u4f-f1-build.patch, sha256 37d87a10bed52b81... (checked: equal to `git diff cb7e374 -- spikes/` in the
builder's worktree agent-ad5f72ed146da5f93). Read only; nothing run.

## Plan
1. T1 (room/tests/commands.rs:618 in the worktree) against condition 3: option B, two late drops, the 2-edge command;
   seen/lost/doubled; the sim refusal with that seq; the plant edits the shared fn only and fails T1 by name.
2. T2 (sim/tests/world.rs, cases a-f) against condition 4: the d10 twin form (what `twins` asserts), each case's
   refusal list exact and count +1, (f) apart; the two plants and where they failed (logs b3-plant1, b3-plant2).
3. Tests that cannot fail / missing negatives: doubled path (repeat) under the new rule, wrap across 255->0, melee,
   dead/plate slot, a further fire edge after the bite paths, room's lost > 0 reachable at all.
4. The +7 counts in check1/check2 logs; the wasm step.
5. What stays unproven until -20's re-run (condition 5).
6. Verdict, findings.

## Notes (a line per finding as it comes)
