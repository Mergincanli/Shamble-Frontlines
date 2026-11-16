# C62H-BOUND round 1: rust-reviewer notes (effort xhigh, read only)

Diff: LANE_BASE 044fab188419e8859dbca699d851563c44c078c9 .. 42b964618eed741f65ac04c1e5a06da0a82524d0 (4 commits),
worktree /home/user/Shamble-Frontlines/.claude/worktrees/c62h-bound (branch c62h-bound-build).

## Plan
1. Read brief sec 0-5, 8.4, 10-11 (done), texts v3, freeze, design, rulings, step returns, build notes.
2. Read the diff per crate: l1 (lib.rs, tests.rs), lab (scenario.rs, logs.rs), judge (figures.rs, engine.rs,
   fixtures.rs), runner (c-u4r-runs.mjs, only for the Rust-adjacent points).
3. Checklist per brief sec 5: HashMap order, float, overflow (u32 sums, u64 totals), docs on pub items, clippy logs
   (step1-check.log, step2-check-2.log, step3-check.log, step4-check.log), bites cfg, call-site Err handling
   (TE-CBB-7 (d)).
4. Rule on builder points (a) lint allow on Logs::room_tick, (b) BTreeSet of lost pairs, (c) engine.rs fixture
   encoding (-1 null, [fn(&mut [Log]); 8]), (d) DatagramCounts and server_datagrams_since read-only claim.
5. Findings with file:line and smallest fix; verdict.

## Progress
