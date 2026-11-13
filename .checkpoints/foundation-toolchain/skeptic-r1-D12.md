# Skeptic r1, D12 (= EA-1, Low): the root [workspace] captures every Cargo package below the root

Plan: (1) confirm the worktree head vs 4d78ad6 differs only in .checkpoints/ and whether origin/main already has a root
Cargo.toml (does this PR introduce the capture?); (2) re-read the root Cargo.toml at 4d78ad6; (3) in a git-archive copy
of 4d78ad6 under .gate/skeptic-D12/, reproduce (a): a lone crate in spikes/, .gate/ and .checkpoints/ fails, one with
its own [workspace] builds; read Cargo's workspace-root discovery to confirm the mechanism; (4) re-run (b): a root
.cargo/config.toml reaches a nested workspace (target-dir and [env]); read ADR 0002:345-346 and plan Part 6 (no
exclude; the netcode spike's cargoConfigFiles check, PR #40); (5) re-grep the lasting docs at 4d78ad6 for any record;
(6) test the proposed exclude option and judge the fix lines (right, complete, what they break); (7) verdict.

## Checks
