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
- C7 (done): wasm32 release, scratch sim-wasm as cdylib exporting `step(a,b)` (needs `#[allow(unsafe_code)]` for
  `#[unsafe(no_mangle)]`: the table refuses it, "usage of the unsafe `no_mangle` attribute", known WASM-ARTIFACT).
  ADR override only + `sim::add_plain`: sim_wasm.wasm 329 bytes, "attempt to add with overflow" count 0 (wraps).
  Same, + `sim::add_never` (#[inline(never)]): 20353 bytes, count 1. Fix A (`[profile.release] overflow-checks = true`
  + `[profile.release.package."*"] overflow-checks = false`) + add_plain: 20295 bytes, count 1; and room's 5 release
  tests all panic, sim/protocol 2/2 under release and bench.
- C8 (done): sources at tag 1.99.1 (raw.githubusercontent.com/rust-lang/rust/1.99.1/...):
  compiler/rustc_codegen_ssa/src/mir/block.rs:747-753 `if !bx.sess().overflow_checks() &&
  msg.is_optional_overflow_check() { const_cond = Some(expected); }` (the code-generating crate's setting wins);
  compiler/rustc_middle/src/mir/terminator.rs:166-170 (is_optional_overflow_check = OverflowNeg | Overflow(Add|Sub|Mul|
  Shl|Shr)); compiler/rustc_mir_transform/src/cross_crate_inline.rs:88-110 and 117-118 ("Assert terminators are similar
  to calls ... so those are just treated as statements": small checked fns are cross-crate inlinable when optimizing).
  Cargo book 1.99.1, profiles.html, "Overrides and generics": "the code may be generated in the crate where it is
  instantiated, and thus may use the optimization settings of that crate"; `"*"` = "any non-workspace member".
  ADR 0002:1319 PM-0002-53 plans "a committed test in `sim` run in the four builds": same blind spot.
- C9 (done): an integration test inside the sim package (crates/sim/tests/cross.rs calling add_plain) PASSES in release
  with the head's overrides: package overrides cover all targets of sim, so a discriminating test must sit in another
  package (room, devtools, sim-wasm). Reproduction script: rust-reviewer-overflow-xcrate.sh (this folder), re-run into
  .gate/rust-reviewer-r1-repro: same results (test profile 5/5; release 4 FAILED + never_inline ok; fix A 5/5).
- C10 (done): CI on the head 4d78ad6: push run 43312017234 (job 131510223737) and pull_request run 43312027194, both
  success (gate 43312016939, 43312026745 success). Log: "overridden by .../rust-toolchain.toml", "rustc 1.99.1
  (58cf8f9c3 2026-10-13)", "rustc equals the pin 1.99.1", inside the cut uid=1001 gid=65534(nogroup), "curl: (7)",
  "the network is cut", 6 unit tests ok, release protocol + sim overflow tests ok, wasm32 "Finished `dev` profile".
- C11 (done): lint table + per-crate table: `[lints.clippy] pedantic` next to `workspace = true` in sim -> cargo 1.99.1:
  "cannot override `workspace.lints` in `lints`, either remove the overrides or `lints.workspace = true` and manually
  specify the lints". Crate attribute `#![warn(clippy::pedantic)]` overrides a workspace `doc_markdown = "allow"`:
  "`#[warn(clippy::doc_markdown)]` implied by `#[warn(clippy::pedantic)]`".
- C12 (done): default-features: path crate depx (default = ["std"]) in [workspace.dependencies]; sim takes
  `{ workspace = true, default-features = false }`: no error on 1.99.1; `cargo tree -e features -p sim` -> depx with no
  features; with room taking `{ workspace = true }`, `-p sim -p room` -> depx "default" and "std" for both. Edition
  guide (rust-2024/cargo-inherited-default-features.html): "Starting with Rust 1.99, default-features on an inherited
  dependency in a 2024 Edition package overrides the value from [workspace.dependencies]" (1.85-1.98 rejected it).
- C13 (done): Cargo.lock byte-identical to `cargo generate-lockfile --offline`; `cargo metadata --locked` ok. A file in
  crates/ is ignored by the glob; a folder without Cargo.toml fails every cargo command (exit 101): loud, not a finding.
- C14 (done): rustup CHANGELOG.md:381-387 "## [1.28.1] - 2025-03-05 ... Automatic install is enabled by default but can
  be opted out by setting `RUSTUP_AUTO_INSTALL`": the workflow's comment (toolchain.yml:21) holds.
- C15 (done): fix A's `"*"` half, .gate/rust-reviewer-r1-star/: a fresh head copy (ws/) with fix A, sim depending on
  a path crate ext/ outside the workspace root (members: the six only). `cargo build --release -p sim -p room -v`:
  "crate-name ext opt-level=3" (no overflow flag), "crate-name room opt-level=3 -C overflow-checks=on", "crate-name sim
  opt-level=3 -C overflow-checks=on". Protocol was not run separately (same mechanism as sim).
  Note: .gate/rust-reviewer-r1-mut/ ended in the lint-test state and no longer shows RR-1; use the script and
  .gate/rust-reviewer-r1-repro/.
- Not checked: cargo audit / cargo deny (no third-party crate in Cargo.lock; cargo-deny enters at 2.3, G5); no
  benchmarks (no code); Wasm size (no cdylib; the scratch sizes above are raw toy numbers, not a budget); Windows host.

## Findings
- RR-1 (High) Cargo.toml:21-26. Not OVERFLOW-PER-CRATE (the report's follow-up: crates that hold sim arithmetic need
  the block): this is sim's own code compiled into callers that hold none, so that remedy does not close it. The per-package release override (ADR part 4's mechanism) covers only code generated
  inside sim's and protocol's own compilation. sim's small, #[inline] and generic functions are generated in the crate
  that calls them (room, sim-wasm, devtools, client-endpoint, later server), whose release profile has overflow checks
  off, and there the checks vanish (C6, C7; rustc 1.99.1 block.rs:747-753, cross_crate_inline.rs:117-118; Cargo book
  "Overrides and generics"). So the comment "Overflow checks in every profile for the simulation and the protocol" and
  ADR part 4's "a debug client and a release server never disagree on overflow" do not hold where the sim runs. E1, M5,
  K1b hold as worded (the flag reaches sim's and protocol's own compilation); they cannot see this, nor can PM-0002-53's
  "test in `sim`" or any test inside the sim package (C9). Not OVERFLOW-WASM: that finding treats native release as
  proven; native release has the same hole. Reach: zero today; it begins with the first sim arithmetic called from
  another crate (slice 1, the 2.5 skeleton). Fix: A `[profile.release] overflow-checks = true` plus
  `[profile.release.package."*"] overflow-checks = false` (all members checked, third-party at default; inherited by
  bench and custom profiles; tested C6/C7) or B the same override on every crate that runs sim code (fragile: each new
  crate must remember). Both widen ADR part 4's named mechanism: the advisor rules, the ADR owner rewords part 4 and
  PM-0002-53 (the test must live in a dependent package, run in release and on wasm32). The discriminating CI test
  changes the K1b line (a kit edit: his yes) or lands in 2.3. At least: narrow the comment and the report's wording now.
- RR-2 (Low) Cargo.toml:14-19; ADR 0002 part 7 (:433-436). Cargo refuses `[lints] workspace = true` together with a
  crate's own lint table (C11), so "pedantic at warn on sim, protocol and content only" cannot be written in those
  manifests while "every crate opts in with workspace = true" stays a gate check. Routes: a crate attribute
  `#![warn(clippy::pedantic)]` (it outranks the workspace table: a workspace allow of one pedantic lint must be repeated
  in the crate, C11) or the gate's own `-W clippy::pedantic` for those packages. For 2.3 (G2) to pick.
- RR-3 (Low) Cargo.toml:11-12; ADR 0002 part 3 (:212-217). On the pinned 1.99.1 a member's `default-features` overrides
  the workspace entry in edition 2024 (edition guide; C12), and one cargo command unifies features over the members it
  builds (C12: depx gains "default", "std" for sim when built with room). So "the maths-relevant features ... set once in
  [workspace.dependencies]" is not enforced by Cargo; 2.3's G5 should also refuse member-level `default-features` and
  `features` on sim's inherited dependencies, beside the `cargo tree -e features` comparison.


