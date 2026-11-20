# U4f-F1 review: security seat (scoped by advisor condition 6 to hostile client input)

Reviewer: security-auditor. Date: 2026-11-20. Diff: `.checkpoints/netcode-proof-test/wip/u4f-f1-build.patch` (base cb7e374),
worktree `.claude/worktrees/agent-ad5f72ed146da5f93`. Read-only seat: no cargo, no scripts, nothing under spikes/ or .gate/.

## Plan
1. Read the patch, the plan (rev 1.1), advisor-u4f-f1-plan.md conditions 1-9 (esp. 6) and Blocks 1-3, the build record.
2. Bounds: every new loop's trip count as a function of client-chosen bytes; cost per command (CPU, allocation) at the
   worst case; the grenade-pool rescan (rust seat RR-F1-1).
3. A session's first command (ADV-U4FF1-3): what a hostile client buys with counters chosen freely on its first command
   (up to 255 edges per button), and the honest rejoin case (RR-F1-3).
4. The path with PM-0001-21 / no-press-edge bitten (room/src/input.rs:293): what still holds without the check.
5. Room counts (seen/lost/doubled) as a client-controlled number: overflow, log/telemetry inflation.
6. Panic / overflow paths reachable from input; wasm32 vs native.
7. Findings, anti-cheat coverage rows touched, verdict.

## Progress
- [ ] 1 read  - [ ] 2 bounds  - [ ] 3 first command  - [ ] 4 bitten path  - [ ] 5 counts  - [ ] 6 panics  - [ ] 7 verdict
