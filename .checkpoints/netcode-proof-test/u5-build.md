U5's early builder, first stint: Block 0 only (u5-brief.md :101-120, narrowed by advisor-u5-early.md conditions 1-10), up to DEP-HOLD's stop; build nothing, run nothing networked, no snapshot; then return.

## Start (u4f-brief.md:292-295's step; 2026-11-21 14:48 UTC)
- `git merge --ff-only bd88fa756c6cc6287d261572571b26f2eaa34d32`: fast-forward f5030a3..bd88fa7, OK.
- `git log -1 --format=%H`: bd88fa756c6cc6287d261572571b26f2eaa34d32 (holds).
- `git rev-parse HEAD:spikes/netcode-proof`: 2b7647ffc3c1927e0bb8d6269bbf240217b8f6db (holds).
- `git diff --stat 40e20fdfcc4bde320cfe9a9ddca902c962befb5c HEAD -- spikes/netcode-proof`: prints nothing (holds).
- `df -B1 /home/user/Shamble-Frontlines`: 9,317,527,552 B available (>= 6.5 GB: holds).

## Plan (blocks)
- B0.1 Read the manifests there now: root Cargo.toml, Cargo.lock's shape, crates/{sim,proto,net,room}/Cargo.toml,
  rust-toolchain.toml, deny.toml, a crate with its own `[lints]` (tcpinfo) for the form; ADR 0002:269 for the cite.
- B0.2 Write crates/alloc/Cargo.toml (std only, no build script, no `include`) and the root Cargo.toml's edits
  (members, `npt-alloc` internal dep, `exclude = ["fuzz"]`, the cite at ADR 0002:269).
- B0.3 Dev-deps on npt-alloc: crates/sim (carry 48, T2-L2) and crates/proto (section 3's per-type case, Block 3);
  NOT crates/room (C-U5-1 waits); crates/net's `fuzz` feature.
- B0.4 fuzz/Cargo.toml (own `[workspace]`, `[lints]` equal to the workspace's key for key, `bites` feature, 17 targets
  as `[[bin]]`) and fuzz/Cargo.lock seeded by copy of the workspace lock (Write of the same bytes).
- B0.5 The root Cargo.lock's update for the new path member: an offline resolve through the runner if cargo can
  (no `cargo update`, no `generate-lockfile`); else hand-written entry, read whole.
- B0.6 tool-versions note: cargo-fuzz, cargo-deny, cargo-audit, libfuzzer-sys, nightly-2026-11-01.
- B0.7 sha256 of every Block-0 file; the main session's networked commands; STOP.

## Findings (one line each, as found)
- F1: ADR 0002's sentence starts at :269 ("`unsafe_code` is denied in every crate; an exception needs a written invariant, a
  test and an `#[allow]` with its reason", :269-270): the root comment's cite moved there. The same stale cite
  (ADR 0002:264-265) also stands in crates/tcpinfo/Cargo.toml:12, crates/hash-wasm/Cargo.toml:11, crates/tcpinfo/src/lib.rs:12,
  linux.rs:44 and :121, crates/hash-wasm/src/lib.rs:28 and seven allow reasons (:96-:198), check.mjs:185 and :268, and
  check.test.mjs:270: not in Block 0's list, left as they are (Low; the two manifests would need a read of their own;
  check.mjs's two can move at Block 1).
- F2: the root comment now names npt-alloc as the fourth exception and the table as (allow, deny, deny, deny), and the fuzz
  crate's lints: a later comment edit would be a manifest change (a second read, condition 9's byte rule), so it is
  written final now; check.mjs states these only after Block 1 (until then check.mjs's deps step fails on crates/alloc:
  "does not inherit the workspace lints" and "sets unsafe_code"; no check.mjs run early, condition 8).
- F3: crates/proto's dev-dependency is in (u5-brief.md:108-109, "section 3's per-type case", Block 3, an early block;
  ruling section 5 opens "the other Block-0 manifests"); crates/room's is not (C-U5-1 at the re-base, condition 1).
- F4: crates/alloc/src/lib.rs is written as a two-line doc stub: cargo refuses a package with no target, so the offline
  lock update needs it; Block 1 replaces it.
- F5: S2's reference needs no new manifest line: ws_frame.rs:462-470 already reaches tungstenite as
  `tokio_tungstenite::tungstenite`, so npt-net's `fuzz = []` entry can hold both sides and the fuzz crate needs no
  tungstenite line.

## Block log
- B0.1-B0.3 done (14:5x UTC): crates/alloc/Cargo.toml + src/lib.rs stub new; root Cargo.toml (member, exclude, internal
  dep, comment); crates/sim and crates/proto dev-deps; crates/net `fuzz = []`. Next: B0.5's offline resolve through the
  runner, named here before it is sent: `bash u5-run.sh <wt> .gate/u5/u5-early-b0-lock.log cargo tree --offline -i
  npt-alloc -e normal,build,dev :: cargo tree --locked --offline -i npt-alloc -e normal,build,dev` (shared lock; a
  resolve that writes Cargo.lock, no compile; the second proves the lock complete).
- B0.5 done (14:58 UTC): log .gate/u5/u5-early-b0-lock.log, both calls exit 0, no Compiling line (nothing built). The
  root lock's diff, read whole: +4 lines `[[package]] name = "npt-alloc" version = "0.0.0"` (before npt-bots), +1
  `"npt-alloc"` in npt-proto's dependencies, +1 in npt-sim's; nothing else. `cargo tree -i npt-alloc`: dev edges only
  (npt-proto, npt-sim).
- B0.4 (manifest) written: fuzz/Cargo.toml (own [workspace] members ["."], resolver 3; lints key for key; `bites`; deps
  libfuzzer-sys "0.4" + 7 path crates + rand_chacha/rand_core at the workspace pins; 17 [[bin]]; the two release
  overflow-checks overrides). Next, named before it is sent: `bash u5-run.sh <wt> .gate/u5/u5-early-b0-fuzzmeta.log
  cargo metadata --offline --no-deps --format-version 1 --manifest-path fuzz/Cargo.toml` (no resolve, no lock, no
  network: a parse check of the manifest), then the lock copy by Write.
- B0.4 done (15:00-15:2x UTC): fuzzmeta log exit 0: 17 bin targets, edition 2024, workspace_root fuzz/, members npt-fuzz
  only, no lock written, no Compiling line. fuzz/Cargo.lock written by Write + two Edit appends as a copy of the updated
  root lock: `cmp` exit 0 (byte-equal), sha256 31c2c7ea9fdf04f78391b0e6d90165f14b74483c87f235c5daf4ad5f8993a7a8 both.
- B0.6 done: .checkpoints/netcode-proof-test/u5-tools.md (sha256 2c6cca66abef4651802f7f5c85687c2ac798cf8c50c72acf678da7f683503e8f).
- B0.7 sha256 (worktree-relative, spikes/netcode-proof/...): Cargo.toml 3bf4d76008175aa7e779d2c1f504b09c5979a5e0e49ddcded8380b7045383b04;
  Cargo.lock 31c2c7ea9fdf04f78391b0e6d90165f14b74483c87f235c5daf4ad5f8993a7a8; crates/alloc/Cargo.toml
  fb5324f1ac53251ef03accb89a9182d62506eb98d074aed6165c00cc49efebf1; crates/alloc/src/lib.rs (stub)
  e64d0d9efcaaeeace5ff6c38416dd035f89b801f77b090b6121e33a9d042d297; crates/sim/Cargo.toml
  b5385fcdaa5dcc2344c41177b5afec923cea9d2004512245fc6d776a2f0c329a; crates/proto/Cargo.toml
  670914131fb08661aafdf8299bea21c18a15e2e7a33f754c295b1bbd61ea1b12; crates/net/Cargo.toml
  afd0952b15865881f763b1cba9d0feac715d2cf104dc4b2c6d13d3ab87f7b265; fuzz/Cargo.toml
  7606244c186ebb95c486c2ef8c100740383cdcdf81f9e985b9495c1e328a369d; fuzz/Cargo.lock = the root lock's.
- Block 0 DONE, STOP at DEP-HOLD (15:2x UTC). `git diff --stat 40e20fd -- spikes/netcode-proof`: 5 files, +43 -15
  (Cargo.lock +6, Cargo.toml +24/-15 net, net +3, proto +4, sim +6); new files: crates/alloc/Cargo.toml 26,
  crates/alloc/src/lib.rs 2, fuzz/Cargo.toml 189; fuzz/Cargo.lock 2,319 (counted apart). Size so far about 260 added
  lines. df -B1 at the stop: 9,315,704,832 B. No target/ in the worktree; nothing built, installed or fetched; no snapshot.
  Next (after dependency-check.md): the main session's networked steps (return item 3), the read, then Block 1.

## Stint 2 (coordinator 15:3x UTC): Block 1, then Block 2's sim half (carry 48, T2-L2), stop before Block 3
- Re-hash at 15:30 UTC: all 8 manifests and locks equal dependency-check.md's lines (fuzz/Cargo.lock now 3117b930…, the
  main session's fetch). df 9,295,695,872 B. No nightly installed (`rustup toolchain list`: stable, 1.99.1).
- Advisor's S1 (advisor-u5-early.md:289-314) folded into Block 1's check.mjs: libfuzzer-sys 0.4.13 and arbitrary 1.5.0
  pinned with their full checksums (copied from fuzz/Cargo.lock by Grep), no derive_arbitrary; one fuzz_target! per
  fuzz_targets/*.rs and none elsewhere; no fuzz_mutator, fuzz_crossover, no_mangle, export_name, link_section, extern,
  macro_use in fuzz/; libfuzzer_sys only as the path libfuzzer_sys::fuzz_target (no glob, group, rename, extern crate).
- B1.1 written: crates/alloc/src/lib.rs (Counting, Counts, start, stop; one allow on the impl; root clippy deny),
  crates/alloc/tests/window.rs (7 tests).
- B1.2 written: check.mjs (alloc the fourth exception: UNSAFE_EXCEPTIONS/EXCEPTION_LEVELS, allocProblems,
  crateRootDeny/clippyNameProblems shared with tcpinfo, itemStart/blockEnd helpers; npt-alloc dependency rule; fuzz/ walked
  by crateFiles(dir, "fuzz") and scanned in unsafeProblems + fuzzProblems; fuzzLintProblems; lockPackages +
  fuzzLockProblems with FUZZ_LOCK_ONLY; DEPS_CHECKED and comments; cites :185, :268 -> ADR 0002:269) and check.test.mjs
  (fixtures + 5 new tests; :270's cite -> :269).
- F6: check.test.mjs's own tests: run 1 (.gate/u5/u5-early-b1-selftest-1.log) 5 fail: tcpinfoLints' read map and the
  UNSAFE_EXCEPTIONS literal lacked crates/alloc (fixtures, not check.mjs); fixed; run 2 (selftest-2.log): 83 of 83 pass
  (78 at base + 5 new), exit 0.
- B1.3 npt-alloc alone (15:49 UTC, df before 9,320,062,976 B by runner 8,910 MiB): `cargo test --locked -p npt-alloc`
  exit 0, window.rs 7 of 7 (log u5-early-b1-alloc-test.log); no lint error under the crate's deny (the allow on the
  impl and the test binary's #[global_allocator] compile).
- B1.4 plants, each by Edit, run `-x -k` (`cargo test --locked -p npt-alloc --test window`), restored by Edit, sha256
  6652f24d82ade66769bb2422e0524583e7b45622494967cd025fbc0ffa59d1bc checked after each:
  P1 dealloc counted (note in dealloc): a_dealloc_is_not_counted FAILED (left 1/8/8), 6 pass (plant1-dealloc.log);
  P2a realloc at its old size (note(layout.size())): a_realloc_is_one_request_at_its_new_size FAILED (16 vs 64);
  P2b realloc not counted: the same test FAILED (0 vs 64); P3 largest off by one (max(size-1)): 5 FAILED, among them
  the_largest_is_the_largest_single_request and one_request_is_counted_with_its_size (plant3-largest.log).
- B1.5 fmt + clippy (-x, alloc-lint.log): clippy -p npt-alloc --all-targets, plain and --all-features, -D warnings:
  exit 0; fmt --check: 2 diffs in tests/window.rs (three assert_eq! wrapped), applied by Edit.
- B1.6 the whole-workspace run, both builds (15:5x UTC; df before 9,320,062,976 B; no nightly installed):
  `cargo test --locked --workspace :: cargo test --locked --workspace --all-features` (-k), log
  u5-early-b1-workspace.log, in the background.
  Result: measured exit 0 (15:51:37-16:00:01 UTC), 649 passed, 0 failed, 15 ignored = 642 + 7 (window.rs);
  bites exit 0 (16:00:01-16:10:18), 746 passed, 0 failed, 15 ignored = 739 + 7. No warning or error line. Provenance:
  all 18 members "Compiling ... (<this worktree>/...)" twice. df after: 4,683,751,424 B (used 4,636,311,552 B).
- F7 (stops the build calls): 4.68 GB free after Block 1's run is under condition 7's "no build call under 5.5 GB", so
  Block 2's test build, its plants and the early-stop clippy cannot run in this stint; the main session's clean (P27)
  comes first. `cargo fmt --check -p npt-alloc` (no build) exit 0 at 16:11 (fmt.log).
- Re-hash at 16:11 UTC: the 8 manifests and locks equal dependency-check.md's lines.
- Block 1 sources: crates/alloc/src/lib.rs 6652f24d82ade66769bb2422e0524583e7b45622494967cd025fbc0ffa59d1bc (144 lines);
  crates/alloc/tests/window.rs 826a82bad05f60e1f2b2b4391ee1d80f1b9deb26e5a7568fa90753dc520f20fa (124); check.mjs
  923ff52f8e371b207185c16ce77af2bd8cbbeeb10c1753087d32227c401fff6d; check.test.mjs
  cd99e4be93f98b731c980952574e37da9ed636da10500bb5cb9ae1f634b8d80d.
- Block 1 DONE (16:1x UTC). `git diff --stat 40e20fd -- spikes/netcode-proof`: 7 files +694 -110 (check.mjs +449,
  check.test.mjs +297 counted with Block 0's 43); new files: crates/alloc/Cargo.toml 26, src/lib.rs 144, tests/window.rs
  124, fuzz/Cargo.toml 189; fuzz/Cargo.lock 1,181 (apart). Added lines 1,177 (S line about 1,400). Next: snapshot
  1, then Block 2's sim test file written (its build waits for disk).
- Snapshot 1 (16:1x UTC): `python3 -I worktree_patch.py <wt> wip/u5-early.patch u5-build`: base bd88fa7, 141,513 B, 12
  files, notes copied; sha256 7d19c5e3c2fabef3345e10980ba574c7fba0daf5af7def14715fb8881b835706.
- B2.1 written (16:12 UTC, not built then: F7): crates/sim/tests/allocations.rs, 2 tests (carry 48; T2-L2); rustfmt's
  diff applied by Edit (b2-fmt.log), `cargo fmt --check -p npt-sim` exit 0 (b2-fmt-2.log).
- Stopped 16:12-17:41 UTC (spend-limit cut). The main session meanwhile: P27 dev-profile clean of the shared target (cold
  now), the nightly and cargo-fuzz 0.13.2 installed. Resume at 17:4x: df 9,359,749,120 B; the 8 manifests and locks equal
  dependency-check.md; crates/sim/src unchanged from HEAD (history.rs f11e9b59…, replay.rs 802842b0…). No cargo-fuzz call
  and no nightly in this stint.
- B2.2 test build (17:51 UTC, df before 9,359,749,120 B): `cargo test --locked -p npt-sim --test allocations` exit 0, 2
  of 2 (b2-sim-alloc-test.log); the #[global_allocator] compiles under the workspace's forbid (U5-Q3 term 5).
- B2.3 plants (`-x -k`, each restored by Edit, sha256 checked; crates/sim/src equal to HEAD after, `git diff` empty):
  P1 push grows its storage (history.rs:68 `= volumes.clone()`): a_push_into_a_full_hit_history_allocates_nothing FAILED
  (24 requests, 53,760 B over 12 pushes) (b2-plant1-push.log); P2 reserve trusts the header (replay.rs:326
  `with_capacity(h.ticks)`, the O4 form): a_short_recording_claiming_2_20_ticks_reserves_only_what_its_bytes_can_hold
  FAILED (1 request, 109,051,904 B = 2^20 x 104) (b2-plant2-reserve.log); P3 room off by one (`min(room + 1)`): the same
  test FAILED (104 B reserved where room is 0) (b2-plant3-room.log).
- B2.4 end checks (`-x -k`, b2-end.log, df before 9,263,968,256 B, after 8,966,742,016 B): `cargo test -p npt-sim`
  exit 0, 59 passed, 0 failed, 1 ignored; `--all-features` exit 0, 67 passed, 0 failed, 1 ignored (the 2 new tests in
  both); clippy -p npt-sim -p npt-alloc --all-targets, plain and --all-features, -D warnings: exit 0; fmt --check of both:
  exit 0. rustdoc with RUSTDOCFLAGS=-Dwarnings on npt-alloc, both builds: exit 0 (b2-doc.log; df after 8,966,418,432 B).
  Workspace counts by delta (Block 2 adds only a new npt-sim test file, condition 7: its own crate's tests): 651 measured
  and 748 bites (642 + 9, 739 + 9), not re-run workspace-wide.
- F8: the fuzz crate's fmt and clippy (condition 8's early stop) cannot run yet: its 17 [[bin]] files are Block 3's, and
  cargo refuses a manifest whose target files are missing. Named for Block 3's first `cargo check`.
- Re-hash at 17:57 UTC: the 8 manifests and locks equal dependency-check.md. crates/sim/tests/allocations.rs
  3bbc13c181d1e8ad4205ce28967f1da08700591eb26db13406bfcd4c270a48c4 (145 lines).
- Block 2's sim half DONE (17:57 UTC). Added lines 1,322 (Block 1's 1,177 + 145; fuzz lock apart), under the S line
  (about 1,400). Next: snapshot 2, then STOP before Block 3 (it waits for SEC-U5D-4's guard rev 4 and its plant test,
  the main session's) and return.
- Snapshot 2 (17:58 UTC): wip/u5-early.patch, base bd88fa7, 146,889 B, 13 files, notes copied; sha256
  303972b91a95ede8503cfcaf68736d996a7eba27ee82d2382241a873aa32ff12 (replaces snapshot 1's 7d19c5e3… at that path).
  df at the stop 8,966,193,152 B. STOPPED before Block 3.

## Stint 3 (coordinator 18:0x UTC): Block 3 through guard rev 4; stop at the first block end past about 1,400
- Guard u5-fuzz-guard.sh sha256 7413834d3a8d8d4e664631c1463cb66d5b85b0243876e082c7c90471e1d9edb4 (= rev 4) checked at
  18:1x before the first call; header lines 1-50 read (allowed options, libFuzzer flags, corpus and artifact under
  .gate/).
- Plan: Block 3 split (it runs past about 400 lines): 3a = the shared core (fuzz/src/lib.rs: the part table with option
  (a)'s bars, framing, judge, check, header and record lines, the S1 doc at the site), the Input target, the core's
  unit tests (planted decoder, over-limit, Input's bar reached on stable), S1's first check, the fuzz crate's fmt and
  clippy, Input's rate with and without ASan; 3b (the other 13 plain targets, RoomControl, the seed generator, proto's
  per-type case) after his U5-SIZE answer. 3a's end passes 1,400: stop there and report.
- B3a.1 written: fuzz/src/lib.rs, fuzz/fuzz_targets/input.rs (one comment line above its fuzz_target! call).
- B3a.2 S1's first check (18:08 UTC, df before 8,963,325,952 B, after 8,748,490,752 B): `cargo check --locked
  --manifest-path fuzz/Cargo.toml --lib --bin input` on stable 1.99.1, exit 0, no warning or error line (b3a-check.log);
  libfuzzer-sys 0.4.13 compiled (its build script built libFuzzer's C++), arbitrary 1.5.0 checked, input.rs's
  fuzz_target! expanded. The read's two reasoned points, recorded (S1): (1) the forbid stays silent: rustc 1.99.1 raised
  no unsafe_code error on the expansion's two #[no_mangle] exports under fuzz/Cargo.toml's forbid; (2) edition 2024
  accepts the bare #[no_mangle] from libfuzzer-sys's 2018-edition tokens: no "unsafe attribute used without unsafe"
  error. No #[allow], no lowering.
- B3a.3 fmt (rustfmt --check on the two files) and clippy (`--lib --tests --bin input -D warnings`): clippy exit 0;
  rustfmt's diff applied by Edit (the part table rewritten as one-line `row(...)` calls through a const fn), then fmt
  exit 0, clippy exit 0 (b3a-lint-2.log). Condition 8's fuzz-crate fmt and clippy: these two files; the other 16
  [[bin]] files do not exist yet (F8), so `--all-targets` waits.
- B3a.4 the core's unit tests on stable: `cargo test --locked --manifest-path fuzz/Cargo.toml --lib` 4 of 4 (planted
  length-trusting decoder: "one request of 204800 B, over its bar of 256 B"; an over-limit input asking before its
  refusal, and one accepted, both refused by name; npt-proto's own decoder on 167 B: 0 requests; Input's 8-command
  message: largest 256 B, its bar exactly) (b3a-lint-2.log). df after 8,295,837,696 B.
- B3a.5 check.test.mjs on the real fuzz files: 83 of 83 (b3a-selftest.log).
- F9: the Write tool is held to this worktree, so the run's corpus and artifact folders are under the worktree's
  .gate/u5/ (condition 7's place; the guard takes <worktree>/.gate/): corpus-input-asan/, corpus-input-none/ (one
  seed file "0" each) and art-input/.
- B3a.6 U5-Q5 term 1, Input's rate: two runs through the guard, `-x`, each `run -s <address|none> input <corpus> --
  -artifact_prefix=<wt>/.gate/u5/art-input/ -max_total_time=60 -seed=1 -max_len=229 -print_final_stats=1` (229 =
  Input's 166 B limit less the harness's type byte, plus 64 B: Block 5's rule); df before and after each.
  - ASan (18:11-18:13 UTC, b3a-rate-asan.log; guard hash checked before; df before 8,295,731,200 B, after
    7,810,056,192 B): GUARD PASSED (8 listed files match, 5 held manifests equal HEAD, NPT_BITE unset), release build
    59.6 s + 11.1 s (every spike crate compiled from this worktree; the second step recompiled them after the runner's
    touch); 3,598,973 inputs in 61 s, **58,999 exec/s**, 271 new units, peak RSS 495 MB, no crash; GUARD AFTER
    unchanged, exit 0.
  - None (18:14-18:16 UTC, b3a-rate-none.log; df before 7,810,056,192 B, after 7,540,957,184 B): GUARD PASSED, build
    38.2 s + 7.6 s; 17,073,618 inputs in 61 s, **279,895 exec/s** (4.7 times ASan's), 337 new units, peak RSS 37 MB, no
    crash; GUARD AFTER unchanged, exit 0.
  - At 10,000,000 inputs: about 170 s with ASan, about 36 s without (Input; FullState through the client, the slowest,
    is Block 4's, so U5-Q5's choice waits for that figure).
  - The bar re-measured on the nightly (section 3): both runs' record lines reach "largest request 256 B" = Input's
    option (a) bar, the same figure as stable 1.99.1's unit test. No over-bar and no over-limit line.
- F10: cargo-fuzz 0.13.2 makes fuzz/artifacts/input/ (empty) before our later -artifact_prefix takes over; nothing was
  written there (art-input/ holds only its README). Empty, untracked by git, not in the snapshot; left in place (a
  delete is not mine). The guard's header could name it.
- Re-hash at 18:17 UTC: the 8 manifests and locks equal dependency-check.md. fuzz/src/lib.rs
  b1d18a2eb65b405ed8e4b76ced0f2c434e10d692130033708cbc2df531f342c9 (329 lines); fuzz/fuzz_targets/input.rs
  a808e58134cb26f3ec98d315d9863143ca4e4c588ed250b32e30fc2a3fa517f4 (7).
- Block 3a DONE (18:17 UTC). Added lines 1,658 (1,322 + 336; fuzz lock apart): past the S line (about 1,400), so the
  build STOPS here (condition 2; coordinator's size rule). Left for Block 3b (after his U5-SIZE answer): the other 13
  plain targets (Resync, Ping and Pong on both ways, Join, JoinAccepted, JoinRejected, ClientSettings, RoomEnded, Leave,
  Kick, SessionControl, SessionStatus) and RoomControl's (its own decode and part), the seed generator, proto's
  per-type case (one byte over each limit: nothing asked for; each bar reached), the fuzz crate's `--all-targets` fmt and
  clippy.
- Snapshot 3 (18:18 UTC): wip/u5-early.patch, base bd88fa7, 161,256 B, 15 files, notes copied; sha256
  afd767ac2aa8f102473c320a9960a3ee363da9434b03b1960553b465ef870a19 (replaces snapshot 2's at that path). df at the stop
  7,540,801,536 B. STOPPED at Block 3a's end (the S line).
