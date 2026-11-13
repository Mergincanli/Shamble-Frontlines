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
- C10 (done): wasm32 control, xcrate copy, route A release (own target dir): rustc "sim ... wasm32 ... opt-level=3
  -C overflow-checks=on", "wasmcheck --crate-type cdylib ... wasm32 ... -C overflow-checks=on"; wasmcheck.wasm 20,510
  bytes (toy); in Node 22: plain/inline/generic(1,1) -> 2, tick(41) -> 42, plain(254,1) -> 255; plain/inline/generic
  (255,1) and tick(4294967295) -> "traps: unreachable". So the release traps are the overflow checks, not a trap on
  every input.
- C11 (done): the four doc comments (1de98a9) against ADR 0002: crate table :186 (sim), :189 (room), :190
  (client-endpoint: same list, built for Wasm inside sim-wasm and natively for the headless clients of part 8), :191
  (sim-wasm "the wasm-bindgen bindings the Worker loads, and nothing else"); migration :1465 (2.4 generated types; the
  layout hash comes from the writer's layout table, :344, which client-endpoint holds), :1467-1472 (2.5 skeleton: the
  Worker's client endpoint and local server exchange Join/JoinAccepted/Input/Snapshot; sim's per-tick hashes);
  OQ-0002-3 (:1022, one Wasm instance or two; decider the boundary spike) and OQ-0002-7 (:1077, wasm-bindgen's health;
  decider "a by-day reading of its maintenance issue (#4533) and the guide's governance page"); the report's
  EMPTY-SLOTS (:259-263) puts that reading before the step that adds wasm-bindgen. All four accurate. Not changed this
  round and out of scope: sim's "depends on `content` only" omits the table's "(and its maths dependencies, ADR 0004)".
- C12 (done): pristine 4bccf55: `cargo fmt --all --check` exit 0; clippy --workspace --all-targets -D warnings clean;
  fresh target dir with -W clippy::pedantic -W clippy::nursery clean (the new doc comments pass doc_markdown); wasm32
  clippy on the five crates clean; RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps clean; cargo test
  --workspace 6 passed; Cargo.lock byte-identical to `cargo generate-lockfile --offline`.
- C13 (done, NEW): shared generics leak "*" into members. Copy share/ (archive of 4bccf55; room depends on extsum, a
  path crate outside the root whose #[inline(never)] ext_sum does `v.iter().copied().sum()` on &[u8]; room's own
  #[inline(never)] room_sum does the same; both called with [255, 1]), `cargo test --release -p room` with
  CARGO_PROFILE_RELEASE_OPT_LEVEL: opt 3 and 2 -> room_sum panics (checked); opt 1, "s", "z" -> room_sum WRAPS (returns
  0), with route A as shipped. Controls (share-controls.sh -> share-controls.txt), opt "s": (a) extsum a dependency but
  never named in room's code -> checked; (a2) no dependency -> checked at 1, s, z (std alone does not supply this
  instance); (c) extsum instantiates sum::<u16> instead -> checked; (r1) the original pair again -> WRAPS; (d) route A
  without the "*" table -> both checked at s and z; (e) route A + [profile.release.package.extsum] overflow-checks =
  true -> both checked at s and z; (f) dev/test profile -> both checked. So at opt-levels where rustc shares generic
  instances, a member that uses a non-member crate links to that crate's unchecked copy of a std generic that inherits
  the caller's overflow setting (here <u8 as Sum>::sum::<Copied<Iter<u8>>>), and the member's own `sum()` wraps.
  Reach today: zero (no dependency; release opt-level 3 = default). ADR 0002 part 4 / PM-0002-29 measure opt-level "s"
  and "z" for the sim-wasm build.
- C14 (done): sources for C13 at tag 1.99.1 (raw.githubusercontent.com/rust-lang/rust/1.99.1/...):
  compiler/rustc_session/src/config.rs:1521-1529 `share_generics()`: None => OptLevel::No | Less | Size | SizeMin =>
  true, More | Aggressive => false (opt 0, 1, "s", "z" share; 2, 3 do not); compiler/rustc_middle/src/ty/instance.rs:
  214-243 `upstream_monomorphization`: with share-generics on (or the item #[inline(never)]), a non-local generic
  instance is taken from `upstream_monomorphizations_for(def)` instead of instantiated locally. core's items that
  inherit the caller's overflow setting: library/core/src/iter/traits/accum.rs:52-89 (Sum and Product fold closures,
  #[rustc_inherit_overflow_checks]), iter/adapters/enumerate.rs:78-161 (Enumerate), iter/traits/iterator.rs:231
  (count), :3140 (position).
- C15 (done): C13 on wasm32: share/crates/wasmsum (a member cdylib; depends on extsum; exports own_sum(a, b) =
  `[a, b].iter().copied().sum()` and ext_sum), route A, CARGO_PROFILE_RELEASE_OPT_LEVEL, run in Node 22: opt 3:
  own_sum(1,2) -> 3, own_sum(255,1) traps, ext_sum(255,1) -> 0; opt "s" and "z": own_sum(255,1) -> 0 (wrapped). So a
  sim-wasm-like member built at "s"/"z" with a dependency loses its std-iterator overflow checks, while a native build at
  opt 3 keeps them: a Wasm/native split on overflow, what part 4 exists to prevent.

## Findings (draft, before the advisor consultation)
- RR2-1 (Medium) Cargo.toml:23-24; docs/adr/README.md:234 (OVERFLOW-ALL); report :19-20, :76-77, :254-257
  (OVERFLOW-PER-CRATE). "*" does more than keep dependencies at the release default: at opt-levels where rustc shares
  generic instances (0, 1, "s", "z"), a workspace crate that uses a dependency links to that dependency's unchecked copy
  of core's overflow-inheriting generics (Sum, Product, count, position, Enumerate), so the member's own arithmetic
  wraps in release, native and wasm32 (C13, C15; rustc config.rs:1521-1529, instance.rs:214-243; core accum.rs:52-89).
  The record anchors the "*" decision to "the first third-party crate in sim's dependency tree" and names neither this
  nor the opt-level trigger. Reach zero today (no dependency, release opt-level 3; dev/test check every crate); trigger
  = any third-party crate any workspace crate uses (wasm-bindgen in sim-wasm is the likely first) + a release-derived
  profile at 1/"s"/"z" (PM-0002-29 and spike item 8 measure "s" and "z"). Fix as a follow-up: the trigger becomes "the
  first third-party crate any workspace crate uses, or a release-derived profile leaving opt-level 2/3, whichever
  first"; the text says "*" reaches members through shared generics there; dropping "*" or a named override per crate
  closes it (C13 d, e); PM-0002-53's dependent-crate test includes a core iterator sum with a dependency present, at the
  shipped opt-level. Or drop "*" now (identical today).
- RR2-2 (Low) Cargo.toml:23-24 (same words in the row and the report). Cargo matches "*" by membership, not by
  folder: a member's plain path dependency inside the repository joins the workspace automatically (even outside
  `crates/*`) and is checked (also linted and tested by --workspace commands); a [patch] or excluded path crate inside
  the folder stays a non-member, unchecked (C6 workspace.rs:943-995; C7 inside/patched/D). Fix: wording at the next
  touch ('"*" matches every non-member: registry, git, a path crate outside the repository, a [patch] or excluded
  path; a path dependency inside the repository becomes a member and is checked').
- RR2-3 (Low) report :70-71, :117 (M7). Both M7 runs stopped at protocol's release binary ("error: test failed, to
  rerun pass `-p protocol --lib`"); sim's release test never ran under M7 in CI (the M5 row states that limit, M7's
  does not); and "M7 proves the setting the tree ships" reads wider than M7 shows (CI turns red when the release line
  goes; the "*" half, the cross-crate reach and wasm32 are E12's, local). Fix: name protocol's binary and the local
  --no-fail-fast run for sim in M7's row; :71 "M7 proves CI catches the release line's removal; E12 its reach".
