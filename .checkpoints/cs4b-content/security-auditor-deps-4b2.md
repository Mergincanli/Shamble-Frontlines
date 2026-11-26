# security-auditor: dependency check of 4b-2 step 6 (parry3d =0.31.1 manifest batch)

Brief: `.checkpoints/cs4b-content/deps-4b2-brief.md`. Worktree `.gate/lane1-takeover`, branch `cs/4b-world`, from main
b9aad01. Batch: `.gate/cs4b/batch-4b2/` (inside the worktree, gitignored `.gate/`), five files. No build. No network.
No agent memory folder exists in this environment (checked `.claude/agent-memory*`, `~/.claude`): earlier decisions come
from this morning's notes (`security-auditor-deps-4b1.md`, `security-auditor-review-4b1*.md`).

## Plan
1. Digests: `sha256sum -c` the five files against `deps-4b2.sha256`; compare the three unchanged manifests with step 0's
   digests in `deps-batch.sha256`; check `deps-4b2.diff` is the batch against b9aad01 (re-diff myself).
2. Lockfile: parse the batch lockfile and step 0's (`.gate/cs4b/batch/`), list the 33 registry packages added against
   b9aad01's lockfile, compare name/version/checksum/source with step 0's lockfile and with rust-reviewer-step0.md;
   confirm parry3d 0.31.1, safe_arch 1.2.0, thiserror 2.0.21 exact. Verify checksums against the cached .crate files.
3. Build-time code: list build.rs and proc-macro crates among the 33 (from cached sources' Cargo.toml), read each
   build.rs for network/clock/env reads; read proc macros' surface.
4. `unsafe` on parry3d's enhanced-determinism path (no std, f32, dim3, alloc): grep parry3d and its deps' sources.
5. What the players' Wasm file gains: features that turn on SIMD, FMA, libm; what step 7's built-file check should hold.
6. devtools: `protocol` to dev-dependencies; grep who reads it.
7. `cargo tree`/`cargo metadata --offline --locked` in a copy at `.gate/security-auditor-4b2-scratch/`: does the lock
   resolve unchanged (no rewrite), which features are on per target (wasm32 and native).
8. Anything step 6 will need that the batch lacks (dev-deps for tests, features, `[lints]`, deny.toml entries).
9. Answer (<= 20 lines).

## Progress
- [ ] 1 digests
- [ ] 2 lockfile
- [ ] 3 build-time code
- [ ] 4 unsafe
- [ ] 5 players' file
- [ ] 6 devtools
- [ ] 7 resolve in a copy
- [ ] 8 missing pieces
- [ ] 9 answer

## Findings (one line each, as made)
- [x] 1 digests. `sha256sum -c deps-4b2.sha256` in `.gate/cs4b/batch-4b2/`: five OK. Step 0's batch (`.gate/cs4b/batch/`)
  re-checks against `deps-batch.sha256`: six OK. Root Cargo.toml (a854d5e3...), sim (1b19f15a...), sim-wasm (02a77a9a...)
  equal step 0's digests byte for byte. The branch over b9aad01 adds only `.checkpoints/` (git diff b9aad01 HEAD --stat),
  the tree's manifests are b9aad01's (git status: only packages/sim-worker/boundary/content.test.mjs, a comment, outside
  the batch). `patch -p1 -i deps-4b2.diff` on a copy of b9aad01's files gives exactly the five digests: the diff is the
  batch's full text and touches no sixth file. Scratch copies: `.gate/security-auditor-4b2-scratch/{base,patched,ws}`.
- [x] 2 lockfile (script: `security-auditor-deps-4b2-lock.py`, python -I, reads only). Lock format 4 in all three.
  b9aad01 23 packages -> batch 56: 33 added, all `registry+https://github.com/rust-lang/crates.io-index` (no git, no
  other registry, no path outside the repo), none removed; sim gains [content, parry3d], sim-wasm gains content.
  Against step 0's lockfile (`.gate/cs4b/batch/Cargo.lock`, f7233053...): same 56 (name, version) pairs, all 49
  registry entries identical in every field (version, source, checksum, dependency list); only path crates differ:
  content drops ts-rs (4b-1 merged content with no dependency), devtools adds protocol. parry3d 0.31.1 (7b626edc...),
  safe_arch 1.2.0 (42c6efa1...), thiserror 2.0.21 (09e52cb8...): exactly step 0's. The 33 = rust-reviewer's list
  (rust-reviewer-step0.md Q5: B1-7's 33 minus ena, slab plus serde, serde_derive), versions as named there.
  Checksums: for all 49 registry entries lock checksum = cached sparse-index `cksum`; for the 47 with a cached .crate
  also = sha256 of the .crate file; serde and serde_derive have no .crate (never compiled: cfg(any()) edges) and match
  the index. None yanked in the cached index (fetched 2026-11-26 ~13:05 UTC). Caveat: index and .crate files came
  through the same session proxy, so this proves consistency, not independent origin; CI's first `--locked` build on a
  GitHub runner re-downloads from crates.io and cargo checks each .crate against these checksums (the independent check).
