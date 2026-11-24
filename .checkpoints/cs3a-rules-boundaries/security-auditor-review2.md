# security-auditor: round 2 review of change-set 3a (fix pass 79666ad9 over code head 5c923119)

Started 2026-11-24 15:28 UTC (date -u). Worktree .gate/ci-wasm-wt, branch cs/3a-rules-boundaries at 79666ad9.
Scope: only `git diff 5c92311 79666ad9 -- . ':!.checkpoints'` (23 files, +667/-204). Round 1 notes:
security-auditor-review1.md (M-TREE; L-DEVSERVER, L-ROOM, L-STATS, L-PYI, L-CASE, L-REMAPHOME).
Shared worktree read-only for me; plants, builds and servers only in my copy under .gate/security-auditor-cs3a/.

## Plan
1. Manifest/lock hold: diff --stat on *.toml *.lock *package*.json (empty = hold untouched); kit/CI paths in the diff.
2. M-TREE: runtimeCrates(treeText, root, rootCrate) + missingOverflowOverrides; tests; plants foreign-source,
   tree-any-root in plants.py; my tree-parse probe re-run on the new code (git source, other registry, path crate
   outside crates/, crates/../ spelling, symlink, root line not sim-wasm, empty tree, Windows separators).
3. L-DEVSERVER / cond. 2 (d): vite.config.mjs fs.allow; isolation.spec.ts dev-server case; in my own copy, probe
   the dev server and the preview server for pkg-rules by /@fs/ (other spellings, case, //, %2e, encoded slash),
   ?raw, ?url, ?import, ?inline, /node_modules/@shamble/sim-worker/..., /@id/, symlink into pkg-rules, and the
   preview server's dist only.
4. L-REMAPHOME: registryRemapFlag(cargoHome, homeDir); CARGO_ENCODED_RUSTFLAGS in cargoEnv for every cargo child;
   refusedBuildSettings upper-case (L-CASE); does the env var shut out config-file rustflags (Cargo book), what stays
   outside (config-file [profile], [build] other keys, target.<triple>.linker/runner, [env], [unstable]).
5. L-ROOM: panic-message.ts PANIC_TEXT_ROOM = 1016 check; trapMessage; tests. L-PYI: run lines `python3 -I -B`.
6. New surface: rest of the diff (panic.rs, rng.rs, rules_scan.rs, f32 vectors/generators, host.ts, messages.ts,
   worker.ts, sim-wasm.test.mjs) for anything that widens what the page, Worker, build or CI trusts.
7. Verdict.

## Findings and notes (one line each, as found)
- 15:29 Hold: `git diff 5c92311 79666ad9 --stat -- '*.toml' '*.lock' '*package*.json'` empty. 23 files changed.
