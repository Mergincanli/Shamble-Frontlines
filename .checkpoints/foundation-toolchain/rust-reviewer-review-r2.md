# rust-reviewer, review round 2, PR #48 (foundation 2.2, the toolchain)

Head under review: 4bccf55be7bbf54de6e114ddbc6aeadb2cab3f3e (branch claude/laughing-pasteur-fwya24); round 1's head 4d78ad6.
Scope: only what changed outside .checkpoints/ since 4d78ad6 (1de98a9, 4bd9dd5, 297d48a + 8d622ac, 8a4f9b0, 4bccf55).
Worktree: /home/user/Shamble-Frontlines/.gate/foundation-toolchain-wt (read only, except this file).
Scratch copies: /home/user/Shamble-Frontlines/.gate/rust-reviewer-r2/ (git archive of 4bccf55).
Agent memory: none exists (no .claude/agent-memory*/rust-reviewer); round 1's notes are the baseline
(rust-reviewer-review-r1.md, C1-C15, RR-1..RR-3).

## Plan
1. Read the code diff (Cargo.toml:21-29, the four lib.rs doc comments), the ruling D1 (conditions 1-7), the deduped
   round-1 dispositions and the report's overflow lines (Outcome, As built, E1, M5, M7, OVERFLOW-* rows, OVERFLOW-ALL).
2. Focus 1 (reach): fresh git-archive copy of 4bccf55; re-run xcrate-check.sh; diff with xcrate-after-4bd9dd5.txt and
   xcrate-before-1de98a9.txt. Add: -v flags per crate (release, native and wasm32), every member + a non-member
   path dep; wasm32 by behaviour (Node) for sim inline/generic code compiled in a workspace cdylib.
3. Focus 2 ("*"): Cargo book 1.99.1 profiles.html (Overrides; precedence list), cargo source at the 1.99.1 tag
   (profiles.rs: ProfilePackageSpec::All match, the precedence of named over "*"), and behaviour: does "*" skip
   members; does it match a path dep outside the workspace; a path dep INSIDE the workspace folder but not a member
   (exclude); a git/registry-like dep (vendored or local registry if possible); does a named override beat "*";
   what if a member is ALSO named in a package override; warnings for unmatched names.
4. Focus 3 (profiles): bench (inherits release), a custom `inherits = "release"`, `--profile test`, dev, test;
   doc tests; build scripts / proc macros (build-override: does `[profile.release]` overflow-checks reach
   build-override? does "*" reach build deps?); `cargo test --release` uses which profile for the test harness.
5. Focus 4: the Cargo.toml comment at :21-24, sentence by sentence, against 2-4.
6. Focus 5: the four doc comments from 1de98a9 against ADR 0002 parts 3 and 5, the migration steps 2.3-2.5,
   OQ-0002-3 and OQ-0002-7.
7. Focus 6: M7 (297d48a, 8d622ac): the diffs (exact revert?), CI runs (red at the K1b line, inside the cut; green
   after); what the report claims for M7 and whether that claim is limited to what M7 can show.
8. Forward look on route A only: anything that changes behaviour beyond overflow (debug-assertions? no), cost in
   release for room/server code (panic vs wrap: security-auditor's), Wasm size impact (toy numbers only), the
   interaction with a future `panic = "abort"` / wasm32 trap.
9. Record a line per check and per finding below; final answer: verdict, findings RR2-n, checked-right list,
   not verified.

## Checks (one line each, as done)
- C1 (done): diff 4d78ad6..4bccf55 outside .checkpoints/: Cargo.toml (route A, :21-29), four lib.rs doc comments
  (sim :1-2, room :1-2, client-endpoint :1-4, sim-wasm :1-3), docs (REPO_MAP, ROADMAP, STATUS, STATUS_ARCHIVE,
  adr/README.md row OVERFLOW-ALL at :234, the report). No crate manifest, Cargo.lock, rust-toolchain.toml or workflow
  change. Toolchain here: rustc 1.99.1 (58cf8f9c3 2026-10-13), cargo 1.99.1 (5f94df478 2026-08-27), node v22.23.4;
  env RUST_BACKTRACE=1 (explains the "3: extdep::add" backtrace line), no ~/.cargo/config.toml, no CARGO_PROFILE_* vars.
- C2 (done): xcrate-check.sh 4bccf55 -> .gate/rust-reviewer-r2/xcrate (2.0 s wall): output byte-identical to
  xcrate-after-4bd9dd5.txt (diff exit 0); against xcrate-before-1de98a9.txt it differs in exactly: the four cross-crate
  tests (plain, inline, generic, Tick::next) x 3 profiles FAILED -> pass, room and sim_wasm "no overflow flag" ->
  "-C overflow-checks=on", wasm32 release "returns 0" x4 -> "traps: unreachable" x4.
- C3 (done): the script's filter hides "ok" (with --nocapture the panic message splits the line), so a pass is read
  from a missing "FAILED". Unfiltered re-run in the same copy: `cargo test --profile {release,bench,relcustom} -p room`
  and `cargo test -p room` each "test result: ok. 6 passed; 0 failed"; K1b `cargo test --release -p sim -p protocol
  --locked` 1+1 passed. So the after-file's blank results are passes (positive evidence now).

## Findings
- C4 (done): rustc flags on a pristine archive of 4bccf55 (.gate/rust-reviewer-r2/pristine; parser flags.sh there):
  release native --workspace --all-targets: all six members, lib and --test, "opt-level=3 -C overflow-checks=on";
  release wasm32 (CI's five crates): all five lib "opt-level=3 -C overflow-checks=on"; bench (own target dir; bench
  shares target/release so it was all Fresh otherwise): all six lib and --test "-C overflow-checks=on"; dev wasm32 and
  test profile: no flag at opt-level 0 (rustc's default: overflow checks follow debug-assertions, on at opt 0).
  `cargo build --release --workspace --locked`: no warning line ("*" matching nothing today is silent).
- C5 (done): Cargo book 1.99.1 profiles.html, Overrides: '"*" ... all dependencies (but not any workspace member)';
  precedence "first match wins": 1 named, 2 "*" "For any non-workspace member", 3 build-override, 4 [profile.dev],
  5 defaults; release default overflow-checks = false, dev true; test inherits dev, bench inherits release;
  "--release flag is equivalent to --profile=release"; "Overrides and generics": code "may be generated in the crate
  where it is instantiated, and thus may use the optimization settings of that crate".
- C6 (done): cargo's own source, the crate cargo-0.100.0 from static.crates.io, .cargo_vcs_info.json sha1
  5f94df4789f005f9a352888e8355ffc645b7ed0e = the local "cargo 1.99.1 (5f94df478 2026-08-27)" (GitHub API and raw for
  rust-lang/cargo refused here: 403/404). src/workspace/profiles.rs:461-491 ProfileMaker::get_profile merges
  [profile.<name>], then for-host defaults, then merge_toml_overrides; :497-538 merge order build-override (host units),
  then `if !is_member { overrides.get(&ProfilePackageSpec::All) }` (:510-511), then the one named Spec match (:513-536),
  later merges win, so named > "*" > build-override > profile. is_member = `ws.is_member(pkg)`
  (compiler/unit_dependencies.rs:870, ops/cargo_compile/unit_generator.rs:151) = member_ids.contains (workspace.rs:693).
  Membership: workspace.rs:943-995 find_path_deps adds a member's path dependency as a member when its folder is under
  the workspace root (or its own root search lands on this root) and it is not excluded (:960-972); only path sources
  are followed (:983-988), so git and registry crates are never members.
- C7 (done): "*" by behaviour, star.sh (.gate/rust-reviewer-r2/star.sh -> star2/, output star2-4bccf55.txt), route A as
  shipped, `cargo test --release -p room -v`, values by catch_unwind (255 + 1): room's own add panics; extdep (path
  crate OUTSIDE the root, not a member) no flag, and all four of its functions wrap, plain, #[inline], generic and
  #[inline(never)], although called from checked room (a dependency's MIR carries no checks, so its generic code stays
  unchecked even when instantiated in a member); gitdep (git source) no flag, wraps; patched (a [patch] whose path is
  INSIDE the root, vendor/patched) not a member, no flag, wraps; inside (a member's path dependency INSIDE the root,
  vendor/inside, outside `crates/*`) becomes a member automatically (cargo metadata), gets -C overflow-checks=on and
  panics. B: + [profile.release.package.extdep] overflow-checks = true -> extdep flag on, all four panic (named beats
  "*"). C: + [profile.release.package.room] overflow-checks = false -> room's flag gone, its own add wraps (a named
  override beats [profile.release] for a member: one line turns a member off; K1b would catch it only for sim and
  protocol). D: + exclude = ["vendor"] -> inside is no longer a member, no flag, wraps. "*" is inherited by bench and
  by the custom inherits = "release" profile (xcrate: extdep wraps there, where an un-inherited "*" would leave it on).
- C8 (done): doc tests and build scripts (.gate/rust-reviewer-r2/extras, archive of 4bccf55 + a `should_panic` doc test
  in sim + a room build.rs that adds 255 + 1 under catch_unwind). `cargo test --release --doc -p sim -v`: rustdoc gets
  only `--test -C embed-bitcode=no` (no opt-level, no overflow flag), so doc tests build at rustdoc's default (opt 0,
  checks on) whatever the profile: the doc test panics under route A AND with `[profile.release] overflow-checks =
  true` removed (M7's state), where sim's lib test fails "did not panic as expected at crates/sim/src/lib.rs:10:8".
  So no doc test can prove the release setting (matters for where PM-0002-53's test goes, not a defect here). A
  member's build script in release: route A "-C debug-assertions=off ... -C overflow-checks=on", the overflow panics;
  M7's state: wraps. Restored route A in the copy afterwards.
- C9 (done): M7. 297d48a deletes exactly Cargo.toml's `[profile.release]`, `overflow-checks = true` and the blank line
  (3 deletions, one file), "*" kept; 8d622ac adds them back (3 insertions); `git diff --stat 297d48a^ 8d622ac` empty
  (whole tree); 4bd9dd5..4bccf55 changes no Cargo.toml, Cargo.lock, crates/, rust-toolchain.toml or .github/ file.
  CI (gh, Mergincanli/Shamble-Frontlines): 43415093334 (push) and 43415101662 (pull_request) on 297d48a: toolchain
  failure at step 7 "Build and test with the network cut off", after "uid=1001(runner) gid=65534(nogroup)", "curl: (7)",
  "the network is cut", the test-profile tests ok (both overflow tests ok); then release: protocol "overflow_checks_are_on
  - should panic ... FAILED", "note: test did not panic as expected at crates/protocol/src/lib.rs:10:8", "error: test
  failed, to rerun pass `-p protocol --lib`", exit code 101. Cargo stopped at protocol's binary: sim's release test did
  not run in either M7 run (sim's failure under M7's state is local only: the ruling's C3, my C8). Revert runs
  43415458569 and 43415465726 (8d622ac): success, release protocol and sim "should panic ... ok". Head 4bccf55: toolchain
  push 43424298848 and pull_request 43424306336, gate 43424299180 and 43424306390, all success.
