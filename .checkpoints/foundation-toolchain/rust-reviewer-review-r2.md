# rust-reviewer, review round 2, PR #48 (foundation 2.2, the toolchain)

Head under review: 4bccf55be7bbf54de6e114ddbc6aeadb2cab3f3e (branch claude/laughing-pasteur-fwya24); round 1's head 4d78ad6.
Scope: only what changed outside .checkpoints/ since 4d78ad6 (1de98a9, 4bd9dd5, 297d48a + 8d622ac, 8a4f9b0, 4bccf55).
Worktree: /home/user/Shamble-Frontlines/.gate/foundation-toolchain-wt (read only, except this file).
Scratch copies: /home/user/Shamble-Frontlines/.gate/rust-reviewer-r2/ (git archives of 4bccf55).
Agent memory: none exists (no .claude/agent-memory*/rust-reviewer); round 1's notes are the baseline
(rust-reviewer-review-r1.md, C1-C15, RR-1..RR-3). Advisor tool consulted twice: after the draft findings (test or cut
the unrun generics, a dependency-at-opt-3 control, source the MIR claim, drop the wasm-bindgen guess and "drop * now"
from the fix, read the round-1 dispositions, restructure this file) and before the handback (RR2-1's trigger to match
its conditions, the .cargo/ check, the throwaway git repositories in the leftovers, RR2-4(b)'s reason for Low, C14
narrowed); all folded below.

## Plan
1. Read the code diff (Cargo.toml:21-29, the four lib.rs doc comments), the ruling D1 (conditions 1-7), the deduped
   round-1 dispositions and the report's overflow lines (Outcome, As built, E1, M5, M7, OVERFLOW-* rows, OVERFLOW-ALL).
2. Focus 1 (reach): fresh git-archive copy of 4bccf55; re-run xcrate-check.sh; diff with xcrate-after-4bd9dd5.txt and
   xcrate-before-1de98a9.txt; -v flags per crate (release, native and wasm32); wasm32 by behaviour in Node.
3. Focus 2 ("*"): the Cargo book 1.99.1, cargo's source at the exact commit, and behaviour (path outside the root,
   path inside the root, excluded, git, [patch], named override over "*", named override on a member).
4. Focus 3 (profiles): bench, a custom `inherits = "release"`, dev, test, doc tests, build scripts.
5. Focus 4: the Cargo.toml comment at :21-24, sentence by sentence. Focus 5: the four doc comments against ADR 0002
   and the round-1 asks (EA-2, EA-3). Focus 6: M7's commits and CI runs against the report's claims.
6. Forward look on route A only. Record a line per check and per finding; final answer: verdict, findings, checked-right
   list, not verified.

## Checks (one line each, as done)
- C1: diff 4d78ad6..4bccf55 outside .checkpoints/: Cargo.toml (route A, :21-29), four lib.rs doc comments (sim :1-2,
  room :1-2, client-endpoint :1-4, sim-wasm :1-3), docs (REPO_MAP, ROADMAP, STATUS, STATUS_ARCHIVE, adr/README.md row
  OVERFLOW-ALL at :234, the report). No crate manifest, Cargo.lock, rust-toolchain.toml or workflow change. Toolchain
  here: rustc 1.99.1 (58cf8f9c3 2026-10-13), cargo 1.99.1 (5f94df478 2026-08-27), node v22.23.4; env RUST_BACKTRACE=1
  (explains the "3: extdep::add" backtrace line), no ~/.cargo/config.toml, no CARGO_PROFILE_* variables.
- C2: xcrate-check.sh 4bccf55 -> .gate/rust-reviewer-r2/xcrate (2.0 s wall): output byte-identical to
  xcrate-after-4bd9dd5.txt (diff exit 0); against xcrate-before-1de98a9.txt it differs in exactly: the four cross-crate
  tests (plain, inline, generic, Tick::next) x 3 profiles FAILED -> pass, room and sim_wasm "no overflow flag" ->
  "-C overflow-checks=on", wasm32 release "returns 0" x4 -> "traps: unreachable" x4.
- C3: the script's filter hides "ok" (with --nocapture the panic message splits the line), so a pass is read from a
  missing "FAILED". Unfiltered re-run in the same copy: `cargo test --profile {release,bench,relcustom} -p room` and
  `cargo test -p room` each "test result: ok. 6 passed; 0 failed"; K1b `cargo test --release -p sim -p protocol
  --locked` 1+1 passed. So the after-file's blank results are passes.
- C4: rustc flags on a pristine archive of 4bccf55 (.gate/rust-reviewer-r2/pristine; parser flags.sh): release native
  --workspace --all-targets: all six members, lib and --test, "opt-level=3 -C overflow-checks=on"; release wasm32
  (CI's five crates): all five lib "opt-level=3 -C overflow-checks=on"; bench (own target dir; bench shares
  target/release, so it was all Fresh otherwise): all six lib and --test "-C overflow-checks=on"; dev wasm32 and the
  test profile: no flag at opt-level 0 (rustc's default: overflow checks follow debug-assertions, on at opt 0).
  `cargo build --release --workspace --locked`: no warning ("*" matching nothing today is silent).
- C5: Cargo book 1.99.1 profiles.html, Overrides: 'To override the settings for all dependencies (but not any workspace
  member), use the "*" package name'; precedence "first match wins": 1 named package, 2 "*" "For any non-workspace
  member", 3 build-override, 4 the profile, 5 defaults; release default overflow-checks = false, dev true; test inherits
  dev, bench inherits release; "The --release flag is equivalent to --profile=release"; "Overrides and generics": code
  "may be generated in the crate where it is instantiated, and thus may use the optimization settings of that crate".
- C6: cargo's own source: the crate cargo-0.100.0 from static.crates.io (downloaded and read only, never built or run),
  .cargo_vcs_info.json sha1 5f94df4789f005f9a352888e8355ffc645b7ed0e = the local "cargo 1.99.1 (5f94df478 2026-08-27)"
  (GitHub API and raw for rust-lang/cargo refused here, 403/404). src/workspace/profiles.rs:461-491
  ProfileMaker::get_profile merges [profile.<name>], then the for-host defaults, then merge_toml_overrides; :497-538
  merges build-override (host units), then `if !is_member { overrides.get(&ProfilePackageSpec::All) }` (:510-511), then
  the one named Spec (:513-536); later merges win, so named > "*" > build-override > profile. is_member =
  `ws.is_member(pkg)` (compiler/unit_dependencies.rs:870, ops/cargo_compile/unit_generator.rs:151) = member_ids.contains
  (workspace.rs:693). Membership: workspace.rs:943-995 find_path_deps adds a member's path dependency as a member when
  its folder is under the workspace root (or its own root search lands on this root) and it is not excluded
  (:960-972); only path sources are followed (:983-988), so git and registry crates are never members.
- C7: "*" by behaviour, star.sh -> star2/ (output star2-4bccf55.txt), route A as shipped, `cargo test --release -p room
  -v`, values by catch_unwind (255 + 1): room's own add panics; extdep (path crate OUTSIDE the root) no flag, and all
  four of its functions wrap, plain, #[inline], generic and #[inline(never)], although called from checked room;
  gitdep (git source) no flag, wraps; patched (a [patch] whose path is INSIDE the root) not a member, no flag, wraps;
  inside (a member's path dependency INSIDE the root, outside `crates/*`) becomes a member automatically (cargo
  metadata: member), gets -C overflow-checks=on and panics. B: + [profile.release.package.extdep] overflow-checks = true
  -> extdep flag on, all four panic (named beats "*"). C: + [profile.release.package.room] overflow-checks = false ->
  room's flag gone, its own add wraps (a named override beats [profile.release] for a member; K1b would catch that only
  for sim and protocol). D: + exclude = ["vendor"] -> inside is no longer a member, no flag, wraps. "*" is inherited by
  bench and by the custom inherits = "release" profile (xcrate: extdep wraps there; an un-inherited "*" would leave it
  checked).
- C8: doc tests and build scripts (.gate/rust-reviewer-r2/extras, archive of 4bccf55 + a `should_panic` doc test in sim
  + a room build.rs adding 255 + 1 under catch_unwind). `cargo test --release --doc -p sim -v`: rustdoc gets only
  `--test -C embed-bitcode=no` (no opt-level, no overflow flag), so doc tests build at rustdoc's default (opt 0, checks
  on) whatever the profile: the doc test panics under route A AND with `[profile.release] overflow-checks = true`
  removed (M7's state), where sim's lib test fails "did not panic as expected at crates/sim/src/lib.rs:10:8". So no
  doc test can prove the release setting. A member's build script in release: route A "-C debug-assertions=off ... -C
  overflow-checks=on", the overflow panics; M7's state: wraps.
- C9: M7. 297d48a deletes exactly Cargo.toml's `[profile.release]`, `overflow-checks = true` and the blank line (3
  deletions, one file), "*" kept; 8d622ac adds them back; `git diff --stat 297d48a^ 8d622ac` empty (whole tree);
  4bd9dd5..4bccf55 changes no Cargo.toml, Cargo.lock, crates/, rust-toolchain.toml or .github/ file. CI (gh,
  Mergincanli/Shamble-Frontlines): 43415093334 (push) and 43415101662 (pull_request) on 297d48a: toolchain failure at
  step 7 "Build and test with the network cut off", after "uid=1001(runner) gid=65534(nogroup)", "curl: (7)", "the
  network is cut" and the test-profile tests (both overflow tests ok); then release: protocol "overflow_checks_are_on -
  should panic ... FAILED", "note: test did not panic as expected at crates/protocol/src/lib.rs:10:8", "error: test
  failed, to rerun pass `-p protocol --lib`", exit code 101. Cargo stopped at protocol's binary: sim's release test did
  not run in either M7 run. Revert runs 43415458569 and 43415465726 (8d622ac): success, release protocol and sim
  "should panic ... ok". Head 4bccf55: toolchain push 43424298848 and pull_request 43424306336, gate 43424299180 and
  43424306390, all success.
- C10: wasm32 control, xcrate copy, route A release: rustc "sim ... wasm32 ... opt-level=3 -C overflow-checks=on",
  "wasmcheck --crate-type cdylib ... wasm32 ... -C overflow-checks=on"; wasmcheck.wasm 20,510 bytes (toy); in Node 22:
  plain/inline/generic(1,1) -> 2, tick(41) -> 42, plain(254,1) -> 255; plain/inline/generic(255,1) and
  tick(4294967295) -> "traps: unreachable". So the release traps are the overflow checks, not a trap on every input.
- C11: the four doc comments (1de98a9) against ADR 0002: crate table :186 (sim), :189 (room), :190 (client-endpoint),
  :191 (sim-wasm); part 5 :342-344 (2.4's own generator writes the block layout and hash "from the layout table the
  writer uses", which part 3 puts in client-endpoint, so its "(2.4-2.5)" holds); migration :1465 (2.4), :1467-1472 (2.5:
  the Worker's client endpoint and local server exchange Join/JoinAccepted/Input/Snapshot; sim's per-tick hashes);
  OQ-0002-3 (:1022, one Wasm instance or two; decider the boundary spike); OQ-0002-7 (:1077, decider "a by-day reading
  of its maintenance issue (#4533) and the guide's governance page"). Accurate. Out of scope (unchanged this round):
  sim's "depends on `content` only" omits the table's "(and its maths dependencies, ADR 0004)".
- C12: pristine 4bccf55: `cargo fmt --all --check` exit 0; clippy --workspace --all-targets -D warnings clean; a fresh
  target dir with -W clippy::pedantic -W clippy::nursery clean (the new doc comments pass doc_markdown); wasm32 clippy
  on the five crates clean; RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps clean; cargo test --workspace 6
  passed; Cargo.lock byte-identical to `cargo generate-lockfile --offline`.
- C13 (NEW): shared generics carry "*" into members. Copy share/ (archive of 4bccf55; room depends on extsum, a path
  crate outside the root whose #[inline(never)] ext_sum does `v.iter().copied().sum()` on &[u8]; room's own
  #[inline(never)] room_sum does the same; both called with [255, 1]), `cargo test --release -p room` with
  CARGO_PROFILE_RELEASE_OPT_LEVEL: opt 3 and 2 -> room_sum panics (checked); opt 1, "s", "z" -> room_sum WRAPS (0),
  route A as shipped. Controls (share-controls.sh -> share-controls.txt), opt "s": (a) extsum a dependency but never
  named in room's code -> checked; (a2) no dependency -> checked at 1, s, z (std alone does not supply this instance);
  (c) extsum instantiates sum::<u16> instead -> checked; (r1) the original pair again -> WRAPS; (d) route A without the
  "*" table -> both checked at s and z; (e) route A + [profile.release.package.extsum] overflow-checks = true -> both
  checked at s and z; (f) dev/test profile -> both checked.
- C14: sources at tag 1.99.1 (raw.githubusercontent.com/rust-lang/rust/1.99.1/...): compiler/rustc_session/src/
  config.rs:1521-1529 `share_generics()`: None => OptLevel::No | Less | Size | SizeMin => true, More | Aggressive =>
  false (opt 0, 1, "s", "z" share; 2, 3 do not); compiler/rustc_middle/src/ty/instance.rs:214-243
  `upstream_monomorphization`: with share-generics on (or the item #[inline(never)]), a non-local generic instance is
  taken from `upstream_monomorphizations_for(def)` instead of being instantiated locally. core's items that inherit the
  caller's overflow setting: library/core/src/iter/traits/accum.rs:52-89 (Sum and Product fold closures,
  #[rustc_inherit_overflow_checks]; `fn sum`/`fn product` themselves carry no #[inline]); iter/adapters/enumerate.rs:
  78-161 (Enumerate; #[inline] read on `next` :77 and `nth` :91 only); iter/traits/iterator.rs:231 (count, #[inline]
  at :222), :3140 (position, #[inline] at :3127).
- C15: C13 on wasm32: share/crates/wasmsum (a member cdylib; depends on extsum; exports own_sum(a, b) =
  `[a, b].iter().copied().sum()` and ext_sum), route A, run in Node 22: opt 3: own_sum(1,2) -> 3, own_sum(255,1) traps,
  ext_sum(255,1) -> 0; opt "s" and "z": own_sum(255,1) -> 0 (wrapped).
- C16: share-more.sh -> share-more.txt (every extsum function named in room). Route A, release: opt 3 all four
  checked; opt "s": integer Sum by value WRAPS, Sum<&u8> WRAPS, Product WRAPS, Enumerate::nth (#[inline]: optimized
  builds copy it locally) checked. (m2) profile at "s" with [profile.release.package.extsum] opt-level = 3 (the
  dependency does not share) -> all checked. (m3) fat LTO + codegen-units 1 at "s" -> the same three WRAP. count and
  position (#[inline] like Enumerate::nth): not run, expected local. So the leak needs all of: the workspace crate and
  the dependency both at a sharing opt-level (0, 1, "s", "z"); the workspace crate naming the dependency (control a);
  the same instantiation (control c); a non-inline overflow-inheriting core generic (integer Sum, Product).
- C17: the MIR mechanism, sourced: compiler/rustc_mir_build/src/builder/mod.rs:761-771 at 1.99.1 ("Some functions
  always have overflow checks enabled, however, they may not get codegen'd, depending on the settings for the crate they
  are codegened in"): check_overflow = #[rustc_inherit_overflow_checks] || sess.overflow_checks() || const/static;
  builder/expr/as_rvalue.rs:456, :480 emit the checked op and its Assert only when check_overflow. So a non-member built
  with checks off has no overflow Assert in its MIR (C7's extdep inline/generic wrap), and an inherit item's Assert is
  kept or dropped by the crate that codegens it (round 1's C8: codegen_ssa block.rs:747-753). C13 is that second case,
  with the dependency as the codegen crate.
- C18: the auto-member, by command (star2): `cargo test --workspace` runs unit tests for client_endpoint, devtools,
  inside, protocol, room, sim, sim_wasm (not extdep, gitdep, patched); `cargo clippy --workspace -v`: inside and room
  run through clippy-driver; extdep, gitdep and patched through plain rustc.
- C19: the round-1 asks behind the doc comments (review-r1-deduped.md :34-37; review-r1-findings.json EA-2, EA-3;
  skeptic-r1-D13.md :64-76). EA-2: sim and room "Empty until the skeleton step (foundation 2.5)" (done); client-endpoint
  takes part 3's cell (done) and, per the D13 skeptic's caveat 1, "(2.4-2.5)" (done); REPO_MAP:18 adds "the aim cast,
  the block writer and its layout table" (skeptic's caveat 3): NOT done, REPO_MAP:18 at 4bccf55 still reads
  client-endpoint "(prediction and reconciliation, interpolation, snapshot decoding)", while the report's D13 row (:156)
  says "fixed: 1de98a9". EA-3: sim-wasm "until the boundary spike fixes its exports (OQ-0002-3) and wasm-bindgen's
  by-day reading is done (OQ-0002-7)"; shipped as "...fixes its exports (OQ-0002-3); wasm-bindgen's by-day reading
  (OQ-0002-7) comes first", where "first" is open; EMPTY-SLOTS (report :259-263) puts the reading before the foundation
  step that adds wasm-bindgen to crates/, "not the throwaway boundary spike, which adds it first".
- C20: `git ls-tree -r --name-only 4bccf55`: no tracked `.cargo/` path and no cargo config file anywhere, so the
  archives of five named paths (rust-toolchain.toml, Cargo.toml, Cargo.lock, crates, .gitignore) missed no profile key.

## Verdict
APPROVE. Critical 0, High 0, Medium 1 (RR2-1), Low 3 (RR2-2, RR2-3, RR2-4). Route A does what RR-1 asked for the shipped
configuration; nothing here is High, so under the review budget the four become STATUS follow-ups and no round 3.

## Findings
- RR2-1 (Medium) Cargo.toml:23-24; the same claim in docs/adr/README.md:234 (row OVERFLOW-ALL) and the report :19-20,
  :76-77, :254-257 (OVERFLOW-PER-CRATE). The record says "*" only keeps dependencies at the release default and ties
  the "*" decision to "the first third-party crate in sim's dependency tree", but when a workspace crate and a
  dependency it uses both build at an opt-level where rustc shares generic instances (0, 1, "s", "z"), the workspace
  crate links to the dependency's unchecked copy of core's integer Sum and Product, so the workspace crate's own
  `iter().sum()` / `product()` wraps in release, native and wasm32. Evidence: C13, C15, C16 (opt 3 and 2 checked; 1,
  "s", "z" wrap; controls a, a2, c, d, e, f, m2, m3), C14, C17. Conditions, all needed: both crates at a sharing
  opt-level (m2: the dependency at 3 -> checked; a2: no dependency -> checked), the workspace crate naming the
  dependency (a), the same instantiation (c), a non-#[inline] overflow-inheriting core generic (Enumerate::nth stays
  checked). Reach: zero today (no dependency; release opt-level 3 by default; dev and test check every crate of ours and
  every dependency). It opens only when both hold: a third-party crate that a workspace crate uses, and a whole
  release-derived profile at opt-level 1, "s" or "z" (ADR 0002 part 4 and PM-0002-29 measure "s" and "z" for the
  sim-wasm file); then a Wasm build at "s"/"z" wraps where a native build at 3 panics (C15), the split part 4 exists to
  prevent. Fix (follow-up; the row OVERFLOW-ALL still waits for Mustafa's go, so the session may choose to fold the
  wording before that go): (1) the row's and OVERFLOW-PER-CRATE's trigger becomes "decide '*' at the first third-party
  crate any workspace crate uses (not only sim's tree), with the release-derived opt-levels in view; reopen it if such a
  profile later moves to opt-level 0, 1, 's' or 'z' while a dependency exists"; (2) their text says "*" also reaches
  workspace crates through shared generics at those opt-levels (integer Sum and Product, tested); (3) the closing
  options stay: a named override per dependency (C13 e) or dropping "*" (C13 d), both tested; whether to drop "*"
  sooner is the advisor agent's to rule (ruling D1 condition 1 keeps it); (4) OVERFLOW-XCRATE-CI's test (PM-0002-53)
  adds an integer `sum()` in a workspace crate with a dependency present, at the shipped opt-level, native and wasm32,
  and not as a doc test (C8).
- RR2-2 (Low) Cargo.toml:23-24 ('"*" matches only crates outside the workspace, so dependencies keep the release
  default, unchecked'; the same words in the row, the report and REPO_MAP:16). Cargo matches "*" by membership, not by
  folder: a workspace crate's plain path dependency inside the repository, even outside `crates/*`, joins the workspace
  automatically, so it is checked in release, run by `cargo test --workspace` and linted by clippy; a [patch] path or
  an excluded path inside the repository stays a non-member, unchecked. Evidence: C6 (workspace.rs:943-995;
  profiles.rs:510-511), C7 (inside, patched, D), C18. Effect: more checks, not fewer; reach zero today. Fix: wording at
  the next touch: '"*" matches every non-member (crates.io, git, a path crate outside the repository, a [patch] or
  excluded path); a path dependency inside the repository joins the workspace and is checked unless excluded' (also a
  third way to check a vendored crate, for OVERFLOW-PER-CRATE).
- RR2-3 (Low) report :70-71 and :117 (M7). M7 shows what its row says (red at the K1b line inside the cut, exit 101;
  exact revert; green), but both runs stopped at protocol's release binary ("error: test failed, to rerun pass `-p
  protocol --lib`"), so sim's release test never ran under M7 in CI, which the M5 row (:115) states for M5 and M7's
  row does not; and ":71 M7 proves the setting the tree ships" reads wider than M7 can show: it cannot see the "*" half
  (removing "*" leaves CI green today), the cross-crate reach or wasm32 (E12's, local; OVERFLOW-XCRATE-CI :231 already
  says K1b and M7 cannot tell route A from the old blocks). Evidence: C9, C8. Fix: M7's row adds "(protocol's binary:
  cargo stops at the first failing binary; sim's test fails in the same state locally: the ruling's C3)"; :71 becomes
  "M7 proves CI turns red when the release line goes; E12 (local) proves its reach".
- RR2-4 (Low) crates/sim-wasm/src/lib.rs:2-3 and the report :156 (D13). The comments fix what EA-2 and EA-3 asked, with
  two loose ends: (a) "wasm-bindgen's by-day reading (OQ-0002-7) comes first" leaves "first" open; read as "before the
  boundary spike" it contradicts EMPTY-SLOTS (:259-263: the reading comes before the foundation step that adds
  wasm-bindgen to crates/, "not the throwaway boundary spike, which adds it first"); (b) D13 is marked "fixed: 1de98a9"
  but EA-2's REPO_MAP:18 half (its skeptic: add "the aim cast, the block writer and its layout table") is not applied.
  Low, not High, for (b): D13 is itself a Low, its skeptic rated the REPO_MAP half as polish ("REPO_MAP rows are
  summaries"), nothing gates on that row's text (repo-map-check reads only the first backticked path), and no result
  anyone relies on changes. Evidence: C19. Fix: (a) EA-3's own wording, "until the boundary spike fixes its exports
  (OQ-0002-3) and wasm-bindgen's by-day reading is done (OQ-0002-7)"; (b) apply the skeptic's REPO_MAP:18 words, or
  relabel D13 "fixed in the crates; REPO_MAP:18 deferred".

## Checked and found right
- Reach (focus 1): route A puts -C overflow-checks=on on every member's lib and test targets in release and bench
  (native) and on the five Wasm crates in wasm32 release; dev and test are on by rustc's default (C4); no tracked
  .cargo/ config overrides it (C20).
- sim's plain, #[inline], generic, Tick::next and #[inline(never)] functions called from room: checked in release,
  bench and an inheriting custom profile, "6 passed" unfiltered (C2, C3); the wasm32 release cdylib traps only on
  overflow (C10).
- xcrate-check.sh on 4bccf55: byte-identical to xcrate-after-4bd9dd5.txt, and it differs from xcrate-before-1de98a9.txt
  exactly where expected (C2).
- Dependencies at the release default (focus 1): path-outside, git and [patch] crates stay unchecked in release,
  including their #[inline] and generic code called from members, at opt 2 and 3 (C7, C17); RR2-1 is the exception.
- "*" (focus 2): non-members only (Cargo book; cargo 0.100.0 = 5f94df4789f0, profiles.rs:497-538); it skips members
  (C4, C7), matches a path dependency outside the workspace and a git dependency (C7), and a named override beats it
  (C7 B; the book's precedence); a named `false` on a member beats [profile.release] (C7 C).
- Inheritance (focus 3): bench and a custom inherits = "release" carry both tables (C4, C7, xcrate); test inherits dev;
  doc tests always build with checks on, whatever the profile (C8); a member's build script follows [profile.release]
  (C8).
- The comment (focus 4): :21-23 accurate (every profile, every workspace crate; why not part 4's form; RR-1; the row
  OVERFLOW-ALL exists at README:234); :23-24 accurate as to membership at the shipped opt-level, with RR2-1 and RR2-2.
- The doc comments (focus 5): accurate against ADR 0002 (C11) and they fix EA-2's timing and constraints and EA-3's
  decider (C19; RR2-4's two loose ends); fmt, clippy (pedantic, nursery, wasm32), doc and tests clean; Cargo.lock
  byte-identical (C12).
- M7 (focus 6): an exact 3-line removal with "*" kept; an exact revert (whole-tree diff empty); red in both runs inside
  the cut at the K1b line, after the dev tests; the revert green with both release tests; head 4bccf55 green in
  toolchain and gate (C9). M7 cannot tell route A from part 4's blocks, and the report says so (OVERFLOW-XCRATE-CI).
- The register row's technical statements hold, apart from RR2-1's omission: part 4's form reaches only sim's and
  protocol's own compilation; a test inside sim's package cannot see the gap; dependency arithmetic can differ between
  dev and release.

## Not verified
- Registry (crates.io) sources: I ran path, git and [patch] only; cargo's rule tests membership only
  (profiles.rs:510-511).
- Whether the prebuilt std exports any shared overflow-inheriting instance beyond the one tried (C13 a2). Sharing is on
  at opt-level 0, so "checked in every profile" for dev rests on std not supplying such instances, which I did not
  establish.
- Thin LTO; LTO other than one fat + codegen-units 1 run at "s" (m3).
- count and position (#[inline], expected local like Enumerate::nth): not run.
- Windows, macOS and aarch64 hosts; any toolchain other than 1.99.1.
- Cost: no code exists, so no benchmark and no Wasm-size budget; the byte counts above are toy numbers.
- The docs-only commits' other text (STATUS, STATUS_ARCHIVE, ROADMAP, report sections beyond the overflow lines and
  D13/D14); the logs of 4bccf55's runs beyond their conclusions; the smoke job.
- The security effect of panic-instead-of-wrap in room and the future server (security-auditor's).

## Leftovers (deletes not attempted after the first refusal; all under /home/user/Shamble-Frontlines/.gate/rust-reviewer-r2/)
- Folders: star/ (the failed first star.sh run; its `rm -rf` was refused), star2/, xcrate/, pristine/, extras/,
  share/, share-ext/, cargo-src/ (cargo-0.100.0.crate and its unpacked source: downloaded and read only, never built
  or run), rustc-src/, src/ (404 bodies), target-bench/, target-test/, target-pedantic/, target-wasmflags/,
  share-target-1/, share-target-2/, share-target-3/, share-target-s/, share-target-z/, share-target-ctl/,
  share-more-target/, share-wasm-3/, share-wasm-s/, share-wasm-z/, star2-clippy/, star2-clippy2/, star2-clippy3/
  (206 MB in all).
- Throwaway git repositories made by star.sh as local git-dependency sources (git init and one commit each, inside
  the scratch folder; not the project's repository, where nothing was committed): star/gitsrc/gitdep/.git,
  star2/gitsrc/gitdep/.git, star2/gitsrc/patched/.git; cargo's checkout of them under star2/cargo-home/git/
  (checkouts/, db/, CACHEDIR.TAG).
- Files: flags.sh, star.sh, share-controls.sh, share-more.sh; outputs xcrate-4bccf55.txt, xcrate-4bccf55.trim.txt,
  star-4bccf55.txt, star2-4bccf55.txt, share-controls.txt, share-more.txt; copies extras-Cargo.toml.routeA,
  share-Cargo.toml.routeA, share-more-Cargo.toml.routeA, share-room-Cargo.toml.orig, share-room-lib.rs.orig,
  share-extsum-lib.rs.orig, Cargo.lock.head. Nothing written elsewhere (no /tmp file; the worktree only gained this
  notes file).
