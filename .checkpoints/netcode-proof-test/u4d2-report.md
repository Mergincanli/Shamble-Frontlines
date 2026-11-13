# U4d2 report: the Wasm-in-Node hash and the determinism step (npt-hash-wasm, npt-lab's `hash`, check.mjs's wasm step)

Merged 2026-11-13 into the spike branch by the main session (the commit "U4d2: the Wasm-in-Node hash and the
determinism step (npt-hash-wasm, npt-lab's `hash`, check.mjs's wasm step), merged after review round 2"). The approved
brief: u4d2-brief.md rev 1.2 (the advisor's check, advisor-u4d2-brief.md: APPROVE on L1 to L5), on `<base>` bfda3cd
(the spike branch with main merged in; its spikes/ is 28a85e7's). For U4's report, which folds this one.

## Who built it, and how
- **The build:** one builder (general-purpose, opus, xhigh) in `.claude/worktrees/u4d2-build` (branch u4d2-build at
  51520b4, whose spikes/ is `<base>`'s), every cargo run through u4d2-run.sh under W1-TARGET's lock (workflow
  wf_e3cb6af2-079). The spend limit cut it at 16:50 UTC in its block 3; it resumed at 17:42 as a fresh copy from its
  notes and the diff. Its two counted check.mjs runs passed all 11 steps: measured 416, bites 465, self 63 of 63, on
  `<base>`'s 399 / 446 / 52 of 52 and 10 steps. Record: u4d2-build.md.
- **Review round 1** (wf_9294c443-01a; the packet u4d2-packet.md, cut by review_packet.py, with advisor-u4d2-brief.md
  passed by ALLOWED's first addition: history.md's note at its end): rust-reviewer, security-auditor and test-engineer
  REVISE; infra-engineer and engine-architect APPROVE WITH FINDINGS. The spend limit cut batch 2 (the architect and
  test seats) at 20:52 UTC just after their plans, with no plant in place; both resumed at 22:53. Notes:
  u4d2-review-{rust,security,infra,arch,test}.md.
- **The advisor's round-1 ruling** (advisor-u4d2-r1.md: REVISE; Blocks 1 to 9, texts T1 to T5): the fix pass FX-1 to
  FX-12, round 2's seats, the merge's M1 to M9.
- **The fix pass** (wf_bb054eb3-70b): one builder in the same worktree, each FX with its proof failing by name and each
  restore checked by sha256 (FX-11 text only); no stop and no TE2-5 entry; its two counted check.mjs runs passed all 11
  steps: measured 417, bites 466, self 71 of 71. Its end hashes: .gate/u4d2/fx-hashes-end.sha256 (18 files).
- **Review round 2** (wf_969ab29d-637; the packet u4d2-r2-packet.md, round 1's tree against the fixed copy): the same
  five seats in two batches, all APPROVE WITH FINDINGS: one Medium (INF2-1) and Lows, none before the merge. The
  container's restart stopped it once (about 00:52 UTC, the rust seat returned); the spend limit cut it once (01:22 UTC,
  batch 2, with the test seat's TP10 plant in place, which the main session removed by Edit at about 08:26 UTC, the file
  back to its start sha256 and recorded in a labelled block of the seat's notes). Each time it resumed from its journal
  on the same bytes. Notes: u4d2-r2-review-{security,rust,infra,test,arch}.md.
- **The advisor's round-2 ruling** (advisor-u4d2-r2.md: APPROVE): INF2-1 carried (entry 84), the security Lows carried
  (entry 85); the merge on M1 to M9 with M5, M8 and M9 amended; T2, T3 and T5 amended; nothing for Mustafa.

## What U4d2's review settled (the advisor's U4d2 rulings: round 1, advisor-u4d2-r1.md; round 2, advisor-u4d2-r2.md)
- **Round 1** (five seats on one head, 51520b4 plus the snapshot at e2f2a5b; the spend limit cut the second batch at
  20:52 UTC, after the architect's and the test seat's plans and with no plant in place, and both resumed at 22:53 on
  the same bytes, so one round): rust-reviewer, security-auditor and test-engineer REVISE, infra-engineer and
  engine-architect APPROVE WITH FINDINGS; no Critical or High; six Mediums (RR-M2 = TE-M1; RR-M1 = SEC-U4D2-2 = TE-M2;
  SEC-U4D2-1; SEC-U4D2-7; INF-1; TE-M3). One fix pass (FX-1 to FX-12), then round 2 on what changed.
- **Round 2** (the same five seats on the fix pass's head, 51520b4 plus the snapshot at cb27931): all five APPROVE WITH
  FINDINGS; no Critical or High; one Medium (INF2-1, below), carried to U4r; the rest Low, each carried to U4r or U7,
  fixed in these texts, or a follow-up; none needed a push. It was cut twice, by a container restart at about 00:52
  UTC (resumed about 00:55) and by the spend limit at 01:22 UTC (resumed 08:25), and resumed each time from its
  journal on the same bytes, so it is one round. The second cut left the test seat's TP10 plant in check.mjs: the main
  session removed it by Edit (the file back to its start sha256, all 18 files equal to the start list, the copy equal
  to the read copy by `diff -rq`), and the seat re-planted TP10 in its own form, which failed two tests by name.
- **The build-flag guard was fail-open for one case, now closed (RR-M2; Medium; fixed).** The read-back gave rustc every
  rustflags source at once, but cargo uses only one of them, so a negation in a source cargo skips hid an enabling flag
  in the one it used: the deps step passed while the module built so listed +simd128 and +relaxed-simd. The advisor's
  U4D2-Q4 ruling had called that superset fail-closed; it was not. Since the fix the deps step refuses every rustflags
  source that holds a flag (the spike's builds use none), and a set CARGO_BUILD_TARGET or `build.target`, so the
  read-back reads exactly what cargo gives rustc; and the loader refuses a module whose `target_features` section lists
  simd128 or relaxed-simd, or that has none, which also covers a safe `#[target_feature]` function reaching the Wasm
  leg with no flag and no `unsafe` (SEC-U4D2-7; Medium; fixed). The section is the linker's record, so link arguments
  can rewrite it (SEC-U4D2-R2-1, below).
- **The third exception's guards (RR-M1, SEC-U4D2-1; Medium; fixed).** An `unsafe` block inside a listed export's body
  is accepted by rustc under the export's own allow, so only the deps scan refuses it: that case now has its self-test
  and was shown live (build exit 0, deps FAIL at its line). The loader's empty import object inherited
  Object.prototype, so a module importing `constructor.constructor` instantiated; it now refuses every import by name
  and instantiates with a null-prototype object. The kind refusal has its two self-tests (SEC-U4D2-8); a build script
  and `include` used as a path are refused in the deny-level exception crates (SEC-U4D2-4). Two macro forms still pass
  that scan (SEC-U4D2-R2-3, below).
- **The step's time (INF-1 and INF2-3; Medium and Low; report text).** Each figure is labelled. Warm for dependencies,
  the workspace rebuilt by the runner's touch loop: 41.8 and 40.5 s (the fix pass's two counted runs), 40.4 s (round 2's
  infra seat), 41.3 s (the same seat, with a process sampler running), 34.8 s (round 2's test seat's counted run) and
  33.1 s, 52 minutes after a machine restart (the merged tree's counted run), against a 5-minute budget. Round 2's first
  run, 73.7 s, was the first run after a machine restart (page cache cold, no dependency rebuilt). No cold run was taken
  before the merge: the shared target always held other lanes' release dependencies, and Mustafa's one-folder rule gives
  a copy no empty target. The first release builds in the lane took 16.04 s (npt-lab) and 3.57 s (npt-hash-wasm,
  wasm32), and the infra seat's bound for a cold step is about 80 s with a warm page cache; a first run after a restart
  adds about 30 s. The cold figure comes from one run after this merge, once the shared target's release builds for both
  targets are cleaned between waves (W1-TARGET item 3's method, narrowed so that the lane lock's file stays: INF2-4),
  and it is recorded on the card and in U4's report before U4r's brief.
- **The cold figure (INF-1; measured after the merge, M9).** 61.6 s for the wasm step (PASS), against a 5-minute
  budget, from one `node check.mjs --only wasm` in main-run.sh's exclusive call (2026-11-13 09:50 UTC, uptime 5215 s;
  .gate/u4d2-merge/cold-run2.log), after both targets' release folders were cleaned by `cargo clean --target-dir`: the
  host's in `<shared>`, the wasm32 one through its own tagged root `<shared>/wasm32-unknown-unknown` (the advisor's M9
  ruling, advisor-u4d2-m9.md). The four release builds took 41.8 s (npt-lab 31.2, its bites 5.4, npt-hash-wasm 3.9, its
  bites 1.3), the rest 19.8 s; no call stopped. Cold: every release artifact of both targets. Warm: the toolchain,
  cargo's registry cache and the debug and doc folders, which the step does not read. A first try stopped before the
  step at its own guard: `--release --target wasm32-unknown-unknown` with no `-p` cleans `<shared>/release` only.
- **A stopped call's bound and its processes (INF2-1; Medium; carried to U4r, entry 84).** The wasm step gives each of
  its calls but the four cargo builds its own 300 s (FX-7), so a hang in npt-sim's replay could hold the exclusive lane
  lock for 12 x 300 s, and a stopped `npt-lab hash` call can leave its two leg processes running. The advisor's
  round-1 form had called that per-call limit "the step's whole budget"; it was not. U4r's change to the step gives
  these calls one shared deadline and makes a stopped call take its processes with it, each with a self-test. No call
  has come near the limit: the largest since the fix pass took 4.3 s.
- **An npt-net test that fails under load (TE-M3; Medium; not U4d2's).** `l2::delay::tests::a_shutdown_s_never_
  completes_ends_at_its_deadline_without_spinning` failed once in the test seat's counted run (bites build) and passed
  on the immediate re-run; it failed the same way in U3c. It reads no U4d2 file and is in no crate U4d2 changes, so it
  did not stop the merge (B2.3's rule); its cause is a planned change of the main session's before U4r's merge (TE2-5
  (c)). Since FX-10, a failed step's whole output is kept in a file, so the next failure keeps its panic. It passed in
  the three counted runs since (the fix pass's two and round 2's one).
- **Process slips, none weakening the evidence.** The builder judged an unnamed check.test.mjs failure inside its step
  instead of stopping (its own new rule repeated a sentence the test pins once; the test is unchanged and still bites:
  TP9); three early `npt-lab hash` runs took the shared lock where `-x` was asked (none a reported result; each value
  re-checked under `-x`); the rust seat built one module in a scratch target off the shared one (38 MB; its removal by
  `cargo clean --target-dir` follows the merge) to keep a SIMD-built module out of it. The builder's brief cited four
  plan spans, and U4r's two lines, at the plan's earlier stamp; the build covered §5 U4d2 whole. In round 2, the
  architect seat wrote one scratch file by a shell redirect (read by nothing after); the rust seat's control run let
  cargo create /nonexistent-cargo-home (12 KB, outside the repository, left in place under Mustafa's LEFTOVER-DELETE);
  and the test seat's first two TP10 forms were vacuous by its own error, labelled and not counted.
- **Carries.** Met: 29 (Cargo.toml's libm comment; sqrtf is `sqrtss`, sqrt `sqrtsd`, a correction to the carry's own
  words), 41, 52 (only recordings by U4d-F's generator, adb1bab), 65 (the failing tests named; their panics kept since
  FX-10), 66 (its three v1 datagram hashes on the merged tree, P2 913db8e914bdb773…, B1 feabf2f02d023fa9… and P4
  b8a04f2f8ddc0d39…, each equal to U2's by h7 (ii), `cargo test --locked --release -p npt-lab -- --ignored --exact
  probe::tests::h7_carry_66_the_probes_v1_datagram_hashes_are_u2s_for_p2_b1_and_p4_seed_1`: 1 passed); 42 does not apply
  (entry 34 was met by U4a). Added: 82, 83, 84 and 85 (U4r), C-U7-2 (U7).
- **Open security findings (each Low and carried; none fixed in this change).** SEC-U4D2-3: the integer-signature rule
  reads the type's token (`u32` or `i32`), not the type; the limit is written beside the rule, and the type read is
  U7's (C-U7-2). From round 2, each carried to U4r (entry 85): SEC-U4D2-R2-1, the target_features read is the linker's
  record, which a RUSTC_WRAPPER's link arguments can rewrite, no linker setting is read, and two comments claim more
  for the read than that; SEC-U4D2-R2-2, a cargo config's per-package profile can turn npt-sim's overflow checks off
  with no rustflag, unread; SEC-U4D2-R2-3, `include!` handed to a macro as tokens, or a #[path] built from a macro
  fragment, brings an unread file under a listed export's allow in npt-hash-wasm. Every other security finding of the
  review is fixed in this change, SEC-U4D2-R2-4's wording in these texts included.

## Checks on the merged tree
- **M1:** the copy's HEAD 51520b43391a…; `git status --untracked-files=all` lists the 18 files under
  spikes/netcode-proof and the record; `sha256sum -c .gate/u4d2/r2-hashes-start.sha256` 18 of 18 OK, the same list as
  fx-hashes-end.sha256 (cmp).
- **M2:** at 99c7ed9, `git diff --quiet 51520b4 HEAD --` U4d2's lane paths and crates/sim: true; spikes/'s commits
  since 51520b4 are U4e's merge c88eb31 and A13's 1b4a3c8 only; A13's CI green (gate runs 43330155579 push and
  43330163225 pull_request).
- **M3:** worktree_patch.py's new cut of the copy (.gate/u4d2-merge/u4d2-merge-cut.patch) equals cb27931's
  wip/u4d2.patch (cmp; sha256 9e71cda0…; 18 files); `git apply --check` clean; applied in the main folder.
- **M4:** `sha256sum -c .gate/u4d2/r2-hashes-start.sha256` from the main folder's root: 18 of 18 OK.
- **M5:** T4 applied first by u4d2_merge_text.py (phase A; SERIES.md f118a8de… → 3a967369…, read back byte for byte);
  then one main-run.sh call (.gate/u4d2-merge/m5-counted.log; the machine up 3,140 s at its start: uptime-m5.txt): the
  six pre-builds exit 0, their Compiling npt-hash-wasm and npt-lab lines naming the main folder's crates; "check: all
  11 steps pass": the test step measured 491 = 473 + 17 + 1 and bites 561 = 541 + 19 + 1 passed, 0 failed (356.6 s),
  self 71 of 71 = 52 + 11 + 8, marks 81 read and checked with the ADR rows unchanged, redact 264 items with 0
  findings; the deps line with the fix pass's two clauses (no rustflags source holds a flag and nothing sets the build
  target; neither npt-tcpinfo nor npt-hash-wasm has a build script or names `include` but as a macro call), the
  read-back "given no rustflags", and npt-sim's features one set across the npt-server (U4e's library), npt-lab and
  npt-hash-wasm builds; the wasm line 27000 ticks and 0 mismatching for both pairs, h2 equal, the hash-map bite caught
  natively and native-against-Wasm at SS-s1 tick 1234 (Wasm 1 against 2 not exercised: not catchable on wasm32),
  std-sin caught at SS-s1 tick 2 `vehicle[8].pos.y`, no call stopped, 33.1 s; then h7 (ii) by name on release: "1
  passed", its three values in the rulings section's carries line. The fix pass's tests counted in the deltas: FX-8's
  unit test (`hash::tests::a_digest_field_of_a_dump_of_another_length_says_the_layouts_differ`, in each build) and
  eight self-tests (FX-1 to FX-7 and FX-10, named in u4d2-build.md's section "Fix pass, round 1").
- **M6:** no failure in any run; nothing recorded under TE2-5 (TE-M3's npt-net test passed in both builds).
- **M7:** `node tools/gate.mjs`: every check PASS but smoke (5a), which could not start this container's Chromium
  (tools/'s Playwright expects a headless shell that is not installed here: .gate/u4d2-merge/gate-full.log); with
  `--no-smoke`: GATE PASS (.gate/u4d2-merge/gate-nosmoke.log). CI's gate and smoke run on the pushed head.

## Public items
- npt-hash-wasm: its seven `#[unsafe(no_mangle)] pub extern "C"` exports (`begin`, `feed`, `step`, `hash_lo`,
  `hash_hi`, `dump_len`, `dump_word`), integer-only, beside wasm-ld's `memory`; its loader's twelve exports (T1's
  list). npt-lab's library: `HashCompare` (its `of`; `Equal`, `Empty` and `Mismatch`) and `TickHash`; the run header's
  param `fate_log_v1_datagram_sha256`. u4-build-plan.md §4.1's two rows (T1) name them.
- check.mjs: 25 new exports (round 1's 23 and the fix pass's `buildInputProblems` and `FAILED_STEP_DIR`), each imported
  by check.test.mjs; `main` gains three injected parameters (`steps`, `log`, `keep`: FX-10) (EA2-1).

## The Wasm module
npt-hash-wasm's release module, as the build measured it (round 1): measured build 114,282 B raw and 39,902 B by
`gzip -9`; bites build 130,368 B and 44,188 B. The fix pass changed no Rust in npt-hash-wasm or npt-sim. It exports
the seven functions and `memory`, imports nothing, and its `target_features` section lists neither simd128 nor
relaxed-simd (FX-2's refusal accepts both builds). No wasm-opt (C-U7-1).

## Follow-ups (Lows; none triggers a push)
- Round 1's (advisor-u4d2-r1.md, Blocks 4 and 5): RR-L4 with EA-1 (feed `replay_to` by chunks, as `serve` does, and
  cache the hash per `step` in hash-wasm's state, if U4r's Evidence decision or the cold figure brings time into
  question); INF-2 (on Windows the copied executable loses its `.exe`); INF-3's CPU record (when the std-sin scan is
  next re-run, its record names the CPU model and its fma and avx2 flags); INF-7 (the spike README, for docs-keeper at
  U4's close); plan §7.1 item 6's text (main-run.sh uses the shared target; at U4's close).
- Round 2's (advisor-u4d2-r2.md, Block 2): RR2-L4 (b) (the libm comment's fmaf cite); INF2-4's runner half, before
  U4r's brief (the runners fail closed when they cannot take the lock, and nothing makes the shared target's folder by
  hand); INF2-5 (a first line in each failed-step file naming the time, HEAD and argv); Block 6's lesson for briefs
  (never run cargo with CARGO_HOME or CARGO_TARGET_DIR at a path that does not exist).
- TE2-5 (c): the cause of npt-net's load-sensitive test (TE-M3), a planned change of the main session's after this
  merge and before U4r's merge (advisor-u4d2-r1.md, Block 5).
- The cold figure (INF-1; M9): one `node check.mjs --only wasm` after the release-only clean of the shared target on
  both targets, with the machine's uptime, recorded on the card and in U4's report before U4r's brief. Done after the
  merge: the cold figure's bullet, above.
- Housekeeping: the leftover folder `/nonexistent-cargo-home` (12 KB, outside the repository, from a round-2 probe)
  stays under LEFTOVER-DELETE (history.md:404), on the card's "Housekeeping later" list.

## Decisions needed
None.

## For Mustafa
Nothing. PR #40 stays needs-mustafa for its existing reasons. U4d2's open findings, each carried: four security Lows
(SEC-U4D2-3 to U7's C-U7-2; SEC-U4D2-R2-1 to -3 to U4r's entry 85) and one Medium (INF2-1, U4r's entry 84: a hang in
the step could hold the build lock for up to an hour and leave processes running; no call has hung in any run).
