# rust-reviewer, review round 1, PR #48 (foundation 2.2, the toolchain)

Head under review: 4d78ad61e652f2e35ddf51c49557e725420248c8 (branch claude/laughing-pasteur-fwya24); base origin/main 2aa5e9c.
Worktree: /home/user/Shamble-Frontlines/.gate/foundation-toolchain-wt (read only, except this file).
Scratch builds: /home/user/Shamble-Frontlines/.gate/rust-reviewer-r1/ (copies of the Rust half only).
Agent memory: none exists (no .claude/agent-memory*/rust-reviewer); no earlier baselines.
Commits after 4d78ad6 on the branch (ef33a3b...): only .checkpoints/ files (git diff --stat 4d78ad6 HEAD), so the files
reviewed equal the head named in the task.

## Plan
1. Read the report draft (docs/reports/2026-11-12-foundation-2-2.md), then ADR 0002 parts 2, 3, 4, 7, 9 and migration
   step 2.2, as needed the plan and the checkpoint.
2. Read the Rust half of the diff: rust-toolchain.toml, Cargo.toml, Cargo.lock, crates/*/Cargo.toml, crates/*/src/lib.rs,
   and the Rust steps of .github/workflows/toolchain.yml (and .gitignore for target/).
3. Check against the ADR: resolver 3, edition 2024, rust-version, publish = false, empty [workspace.dependencies], lints
   table, release overflow overrides, crate names and the dependency direction (part 2 / part 3).
4. Profiles: does overflow-checks = true reach every profile that matters (dev, release, test, bench, and any custom
   profile)? Check Cargo book inheritance rules (bench inherits release, test inherits dev), and what profile
   overrides / build-override do. Confirm with a mutation in a scratch copy (cargo test --release, cargo bench, cargo
   test --profile bench).
5. Overflow tests: expected = "attempt to add with overflow" — does the message differ on any target / toolchain? Is
   the test using black_box so the compiler cannot const-fold to a compile error (arithmetic_overflow deny lint)?
6. wasm32 build of five crates: which target, which crates, is it `cargo build --target wasm32-unknown-unknown` on a
   workspace exclusion, does CI do what the report says (read the CI job logs by run id)?
7. Lints: [workspace.lints] wired with `lints.workspace = true` in every crate? clippy -D warnings? missing_docs?
   unsafe_code forbid/deny? Run cargo clippy, cargo fmt --check, cargo test, cargo doc locally on a copy.
8. Cargo.lock: what it contains, version 4, does it match the manifests (cargo metadata --locked / --frozen).
9. rust-toolchain.toml: pinned exact version vs channel; components; targets; profile minimal; does CI use it.
10. Forward look: what bites when the first real code or the first third-party crate enters (panic strategy for wasm,
    opt-level, lto, codegen-units, strip, cargo-deny/audit wiring, wasm-bindgen version lock, getrandom on wasm, float
    determinism flags, MSRV vs rust-version, resolver 3 MSRV-aware resolution, workspace.dependencies inheritance).
11. Record findings here, a line each, then return them via StructuredOutput.

## Checks done
- C1 (done): read the report draft, ADR 0002 parts 2, 3 (table), 4, 7 (Rust), migration 2.2/2.3, plan Part 2-3 and 1.4.
- C2 (done): scratch copy by `git archive 4d78ad6 rust-toolchain.toml Cargo.toml Cargo.lock crates .gitignore` into
  .gate/rust-reviewer-r1/. Toolchain: rustc 1.99.1 (58cf8f9c3 2026-10-13), cargo 1.99.1 (5f94df478 2026-08-27),
  rustup 1.28.2, "overridden by .../rust-reviewer-r1/rust-toolchain.toml".
- C3 (done): `cargo test --workspace --locked` 6/6 pass; `cargo test --release -p sim -p protocol --locked` 2/2 pass;
  `cargo build --locked --target wasm32-unknown-unknown -p sim -p protocol -p room -p client-endpoint -p sim-wasm` ok
  (dev profile, rlibs only).
- C4 (done): `cargo clippy --workspace --all-targets --locked -- -D warnings` clean; the same for wasm32 on the five
  crates clean; with `-W clippy::pedantic -W clippy::nursery` clean; `cargo fmt --all --check` exit 0;
  `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked` clean; `-W missing_docs` 0 warnings (no pub items).
- C5 (done): profiles, in .gate/rust-reviewer-r1-mut/ (a second archive copy). `cargo test --profile bench -p sim -p
  protocol --locked` 2/2 ok; `--profile test` 2/2 ok; NOTE `cargo bench` lists both tests as "ignored" (libtest's
  --bench mode), so the report's "pass under the bench profile" holds only as `cargo test --profile bench` (it does).
  Control, both [profile.release.package.*] blocks removed, `--no-fail-fast`: bench and release both FAIL in sim AND
  protocol ("test did not panic as expected at crates/{protocol,sim}/src/lib.rs:10:8"), so both blocks are proven
  locally under release and bench (CI's M5 proved protocol only). A custom `[profile.dist] inherits = "release"`
  (opt-level "z") passes 2/2: package overrides are inherited by custom profiles.
- C6 (done): cross-crate overflow (the main experiment). sim gains `add_plain` (no attribute), `add_inline`
  (#[inline]), `add_generic<T>`, `Tick::next` (#[inline], u32 + 1), `add_never` (#[inline(never)]); room depends on sim
  (path) and calls each with black_box(MAX), black_box(1) under #[should_panic(expected = "attempt to add with
  overflow")]. `cargo test -p room` (test profile): 5/5 panic as expected. `cargo test --release -p room`: plain,
  inline, generic, Tick::next all FAIL "test did not panic as expected"; only add_never panics. `-v` after a clean:
  "crate-name sim --crate-type lib opt-level=3 -C overflow-checks=on"; "crate-name room --crate-type lib opt-level=3"
  (no overflow flag). So the override covers only code that is code-generated inside sim's own compilation.

## Findings
- RR-1 (High, draft): ADR part 4's per-package release override does not reach sim's code where sim runs: sim's small,
  #[inline] and generic functions are code-generated in the calling crate (room, sim-wasm, devtools, later server), whose
  release profile has overflow checks off, and their overflow checks vanish (C6). The two overflow tests run inside
  sim's and protocol's own test binaries, so they cannot see it; the report's "CI proves the release overflow setting"
  is true only for code generated inside those two crates. Fix candidates: `[profile.release] overflow-checks = true`
  for the whole workspace (simplest; covers custom profiles by inheritance), or the same override on every crate that
  runs sim code (room, client-endpoint, sim-wasm, devtools, later server, navbake), plus a test in a dependent crate.
