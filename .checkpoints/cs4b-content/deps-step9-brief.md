# For security-auditor: the dependency check of 4b-2's step 9 (the type writer as the content crate's dev-dependency)

Mustafa's DEP-HOLD: a manifest or lockfile change is not committed, pushed or built before your answer on exactly
that content. **Nothing of this batch is in the working tree or in any commit.** It lies outside the tree, under
`.gate/cs4b/batch-step9/` (two files at their paths), and as text in the notes:
- `.checkpoints/cs4b-content/deps-step9.diff`: the batch against the branch's head 54e89d2 (26 lines).
- `.checkpoints/cs4b-content/deps-step9.sha256`: the two digests. From `.gate/cs4b/batch-step9/`,
  `sha256sum -c ../../../.checkpoints/cs4b-content/deps-step9.sha256` gives two OK.

## What the batch is
| File | Change |
|---|---|
| `crates/content/Cargo.toml` | one table, `[dev-dependencies]`, with one line, `ts-rs = { workspace = true }`, and a comment; nothing else changes |
| `Cargo.lock` | the `content` entry gains `dependencies = ["ts-rs"]`; made by `cargo update --workspace --offline` in a scratch copy (`.gate/cs4b/ws-step9/`); it printed "Locking 0 packages": no package is added, removed or moved |

Why: step 9 writes the content kinds' TypeScript types by a test, as the protocol crate's are written
(`crates/protocol/src/ts_export.rs`, `crates/protocol/Cargo.toml`: the same line under `[dev-dependencies]`). ts-rs
12.0.1 is already in the workspace (`Cargo.toml:13`, `=12.0.1`, default features off) and in the lockfile, which you
read at the foundation and at 4b-1's step 1. The content crate's rule (`crates/devtools/tests/dependency_rules.rs`,
`the_content_crate_depends_on_nothing`) names this one change in its own doc and is edited in the same commit: the
tables become `["package", "dev-dependencies", "lints"]` and the one line is pinned to the letter.

What I ran on the batch, resolving only (nothing compiled; `cargo tree --locked --offline` in the scratch copy,
through `build-checks.mjs`'s own functions, script `.gate/cs4b/step9-lists.mjs`): the simulation's feature lists are
71, 71 and 76 entries, the same counts as on the head, `simFeatureDifferences` gives no difference, and no crate
lacks its overflow-check block.

## What I ask you to judge
1. The batch's exact content against its two digests. Is the lockfile's change exactly the one entry?
2. Does a dev-dependency of `content` reach any build that players get, or the simulation's normal tree? (`sim`
   depends on `content` on a normal edge; `sim-wasm` and `devtools` too. The players' build is
   `cargo build --release --target wasm32-unknown-unknown -p sim-wasm`; the test-only module is
   `cargo build -p devtools --lib` for the same target.) Does it change what the feature compare's second
   exception rests on (thiserror's `std`, turned on by the type writer in the native tests only)?
3. `content` forbids unsafe code and its lint guard denies panics outside tests: does the derive
   (`#[cfg_attr(test, derive(ts_rs::TS))]` on the kinds' types, as protocol does) put any code into a non-test
   build of `content`?
4. Is anything missing from the batch that step 9 will need, so that no second batch follows it? Step 9's other
   work: `Sim::create` takes the bundle's bytes; the generated types under
   `packages/types/src/generated/content/`; the frozen content of the first recording. I know of no other
   manifest change in it.

## Rules for you
You edit no source file, no manifest and no doc. You write only your notes
(`.checkpoints/cs4b-content/security-auditor-deps-step9.md`: your plan first, then each finding as you make it).
Git read-only, each git command alone in its shell call with nothing in front of it. **Do not build:** no
`cargo build`, `cargo test`, `cargo check`, `cargo fetch` or `cargo vendor` with this batch; reading the registry's
already-cached sources under `~/.cargo/registry/src` is fine, and `cargo tree` or `cargo metadata` only in a COPY of
the workspace under `.gate/security-auditor-step9-scratch/` with `--offline --locked` and the batch's files copied
in. No network. The repository is public. Another process may be running cargo in the main tree: do not run cargo
there at all.

## Your answer (at most 15 lines)
APPROVE the batch by its two digests (name them), or REVISE (the exact change). Then each finding with its
severity (Critical, High, Medium, Low), and what you did not check.
