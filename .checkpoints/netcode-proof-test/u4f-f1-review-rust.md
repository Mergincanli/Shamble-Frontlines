# U4f-F1 review, rust-reviewer seat (one round, 2026-11-20)

Read only (git show/diff/grep, sed -n, sha256sum). Nothing run under cargo. Diff: worktree
`.claude/worktrees/agent-ad5f72ed146da5f93`, `git diff cb7e374 -- spikes/` (sha256 37d87a10bed52b81..., equal to
`.checkpoints/netcode-proof-test/wip/u4f-f1-build.patch`, 17,495 B: checked).

## Plan
1. seam.rs `edges_taken`: integer-only, const, wrap semantics, doc.
2. world.rs `run_command`: the loops (reload, fire, throw), bounds, cost per edge, allocation, order vs condition 1,
   1-edge path unchanged, clock once, the bites.
3. room.rs `run_one`: counts' arithmetic (overflow, saturating), doubled on repeat.
4. Determinism wasm32 vs native: any float, HashMap, usize-width dependence introduced.
5. Panic / overflow paths reachable from client input (255 edges, refusal Vec growth).
6. Tests T1, T2 from the Rust side (correct helpers, no flake); clippy/fmt evidence in logs.
7. Findings, verdict.

## Blocks
