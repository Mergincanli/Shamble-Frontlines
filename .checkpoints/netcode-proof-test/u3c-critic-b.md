# U3c build B completeness critic (test-engineer), 2026-11-07

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_87550c07-bbd-2 (uncommitted on ebe82b2).
Plan: u3c-plan.md rev 2, scope 1.8 [C12] (a)-(g) + check.mjs deps checks. Read-only on the worktree.

## Plan
1. git diff + new files: Cargo.toml, Cargo.lock, net/Cargo.toml, net/lib.rs, tcp_kernel.rs, tcpinfo/*, check.mjs,
   check.test.mjs, root script.
2. Per item (a)-(g), (d) deps checks: met / missing / wrong / unproven with file:line.
3. Run check.mjs with CARGO_TARGET_DIR=.gate/u3v3-model-target; record result.
4. Adversarial: tests that cannot fail, wall-clock thresholds, public items beyond rev 2, grep bypasses.

## Findings (as I go)
