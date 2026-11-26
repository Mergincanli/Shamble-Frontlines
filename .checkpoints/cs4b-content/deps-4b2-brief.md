# For security-auditor: the dependency check of 4b-2's step 6 (the collision library's manifest batch)

Mustafa's DEP-HOLD: a manifest or lockfile change is not committed, pushed or built before your answer on exactly
that content. **Nothing of this batch is in the working tree or in any commit.** It lies outside the tree, under
`.gate/cs4b/batch-4b2/` (five files at their paths), and as text in the notes:
- `.checkpoints/cs4b-content/deps-4b2.diff`: the batch against main b9aad01 (`git diff`, 515 lines).
- `.checkpoints/cs4b-content/deps-4b2.sha256`: the five digests. From `.gate/cs4b/batch-4b2/`,
  `sha256sum -c ../../../.checkpoints/cs4b-content/deps-4b2.sha256` gives five OK.

## What the batch is
| File | Change | Against step 0's batch (which you read this morning: `security-auditor-deps-4b1.md`, `deps-batch.sha256`) |
|---|---|---|
| `Cargo.toml` (root) | the `parry3d` line (`=0.31.1`, default features off, `dim3`, `f32`, `alloc`, `enhanced-determinism`; `std` off) and 29 overflow-check blocks | byte for byte step 0's file (a854d5e3...) |
| `crates/sim/Cargo.toml` | `content` (path) and `parry3d` (workspace) | byte for byte step 0's (1b19f15a...) |
| `crates/sim-wasm/Cargo.toml` | `content` (path) | byte for byte step 0's (02a77a9a...) |
| `crates/devtools/Cargo.toml` | `protocol` moves from `[dependencies]` to `[dev-dependencies]` (your own note on 4b-1's batch: only `tests/aim_reach.rs` names it; `grep -rn "protocol::" crates/devtools/src` finds nothing) | new (6b90a2de...) |
| `Cargo.lock` | 33 registry packages added, by `cargo update --workspace --offline` from the cached index | new digest (ad0f7680...): the same 56 packages as step 0's lockfile, every registry package with the same version and checksum; two path crates' own lists differ (`content` no longer lists ts-rs, which enters at step 9 with its own check; `devtools` lists `protocol`) |

## What I ask you to judge
1. The batch's exact content against its digests, and each of the 33 registry packages: name, version, checksum,
   source (crates.io only), against rust-reviewer's paper reading (`rust-reviewer-step0.md`) and your own
   reading of this morning. The advisor agent's condition: parry3d 0.31.1, `safe_arch` (1.2.0) and thiserror
   (2.0.21) must be exactly step 0's, since Mustafa's FEATURE-COMPARE (a) pins his yes to those versions; any
   other package that moved goes back to rust-reviewer and you. My compare found none moved.
2. Build scripts and proc macros among the 33 (what runs at build time on this machine and in CI), `unsafe`
   that the library's `enhanced-determinism` path reaches, anything that reads the network, the clock or the
   environment at build time; what the players' file gains (the built-file check of step 7 holds SIMD, fused
   multiply-add and the libm allow-list; say what else it should hold).
3. The devtools change: is `protocol` as a dev-dependency right (narrower), and does anything else read it?
4. The order after your answer: the five files are copied back from `.gate/cs4b/batch-4b2/`, checked against
   the digests, committed as step 6's first commit together with the foundation check's `sim` half
   (`crates/devtools/tests/dependency_rules.rs`: `sim`'s normal dependencies are `content` and `parry3d`) and
   the feature compare with the two named exceptions; only then is anything built. Is anything missing from the
   batch that step 6 will need (so that no second batch follows it)?

## Rules for you
You edit no source file, no manifest and no doc. You write only your notes
(`.checkpoints/cs4b-content/security-auditor-deps-4b2.md`: your plan first, then each finding as you make it).
Git read-only, each git command alone in its shell call with nothing in front of it. **Do not build:** no
`cargo build`, `cargo test`, `cargo check`, `cargo fetch` or `cargo vendor` with this batch; reading the
registry's already-cached sources under `~/.cargo/registry/src` is fine, and `cargo tree`/`cargo metadata` only
in a COPY of the workspace under `.gate/security-auditor-4b2-scratch/` with `--offline --locked` and the batch's
files copied in (they resolve, they do not compile). No network. The repository is public.

## Your answer (at most 20 lines)
APPROVE the batch by its five digests (name them), or REVISE (the exact change), or what must go to Mustafa
(only a version change of the three pinned packages would). Then each finding with its severity (Critical, High,
Medium, Low), and what you did not check.
