# U4d2 review round 2: test-engineer (on what changed: the fix pass FX-1 to FX-12)

Seat: the advisor's U4d2 round-1 ruling (advisor-u4d2-r1.md), Block 8, batch 2, test-engineer; role file read from
origin/main (.claude/agents/test-engineer.md). Copy run in (the only seat running there now):
/home/user/Shamble-Frontlines/.claude/worktrees/u4d2-build, every cargo and node run through u4d2-run.sh. Read copy
.gate/u4d2-r2; round 1's tree .gate/u4d2-r1. Logs: .gate/u4d2/r2-te-*.log. Plants by Edit in place (old and new text
recorded here), run with -x -k (and --no-fail-fast for cargo test), restored by Edit, each restore checked by sha256
against step 1's hashes. Never commit, never clean, the shared target only.

## Plan
1. Start hashes: sha256sum of the 18 changed and new spike files (from the copy's root), saved with Write as
   .gate/u4d2/r2-hashes-start.sha256; equal to fx-hashes-end.sha256; read copy 18 of 18; crates/sim diff empty.
2. Read the code the fix pass changed (check.mjs's new rules, main, wasmStep's call; loader.mjs's moduleProblems and
   targetFeatures; hash.rs's compare, arm_for, digest_text) and the 8 new self-tests, FX-9's assert, FX-8's unit test
   and h6's asserts: does each assert what it claims; could it pass for the wrong reason.
3. Round 1's plants re-planted (Block 8: each must now fail by name):
   - TP2 run 3 (environment only): CARGO_ENCODED_RUSTFLAGS=-Ctarget-cpu=bleeding-edge with
     RUSTFLAGS=-Ctarget-feature=-simd128,-relaxed-simd, `--only deps`: FAIL naming both sources; control PASS; FX-10's
     failed-steps/deps.log written by the FAIL, removed by the control.
   - TP4 (lib.rs, begin's body: `let bite = unsafe { core::ptr::read_volatile(&bite) };`): wasm32 release build exit 0,
     clippy wasm32 -D warnings exit 0, `--only deps` FAIL at lib.rs:101.
   - TP10 (check.mjs hashWasmProblems: skip `unsafe` inside a listed export's body): node --test, FX-3's new test and
     the C2R-5 deps test FAIL by name.
   - TP11 (loader.mjs: the kind branch disabled): node --test, FX-5's new test FAILs by name.
4. Plants where I doubt a proof or a seat's finding can be settled by run:
   - FX-2's count rule disabled (`sections.length !== 1` never true): the FX-2 self-test FAILs by name?
   - INF2-2 (infra): run()'s spawnSync timeout dropped: do all 71 self-tests stay green (the FX-7 test cannot see it)?
   - RR2-L3 (rust): compare arms every recording again (its armed closure ignoring `on`): do npt-lab's bin tests stay
     green (--all-features, --no-fail-fast)?
   - SEC-U4D2-R2-3 (security): `macro_rules! m` + `m!(include!)` in begin's body naming a file with an unsafe block:
     wasm32 build exit 0 and `--only deps` PASS while the gap stands?
5. One counted `node check.mjs` (the six pre-builds first), -x, in the background with a limit of 2,400,000 ms; every
   step's line; counts as deltas on 416 / 465 / 63 of 63 and 11 steps; the wasm step's split. TE2-5 on any failure I
   did not plant (re-run that crate at once with --no-fail-fast; name and panic, FX-10's file kept).
6. End: `sha256sum -c` of step 1's file (18 of 18), `git diff --stat -- spikes/netcode-proof/crates/sim` empty, git
   status as at the start.
7. Findings ranked, claim/proof/gap table, verdict.

## Step 1 (done, 01:18 UTC)
- r2-hashes-start.sha256 written (18 lines: `git status --short --untracked-files=all -- spikes` in the copy, sorted,
  sha256sum from the copy's root); `cmp` with fx-hashes-end.sha256: equal. `sha256sum -c` from the copy's root: 18 OK;
  from .gate/u4d2-r2's root: 18 OK. HEAD 51520b4; status 9 M, 10 new (the 18 and the builder's record).
  `git diff --stat -- spikes/netcode-proof/crates/sim`: empty. failed-steps/ empty. df 7.2G free; shared target 7.0G;
  no cargo or rustc running.

## Results (a line after each)
- TP2 r2-te-tp2-flags.log (-x -k, environment only, no file changed; runner exit 1 by design). Run 3 (RR-M2's pair,
  CARGO_ENCODED_RUSTFLAGS=-Ctarget-cpu=bleeding-edge RUSTFLAGS=-Ctarget-feature=-simd128,-relaxed-simd, `--only deps`):
  **FAIL, exit 1** (round 1: PASS), its 40-line tail naming both sources: "CARGO_ENCODED_RUSTFLAGS in the environment
  holds \"-Ctarget-cpu=bleeding-edge\": no rustflags source may hold a flag ..." and "RUSTFLAGS in the environment holds
  \"-Ctarget-feature=-simd128,-relaxed-simd\": ..."; no read-back line (the superset still hides simd128: the source
  rule alone catches it, as FX-1 rules). FX-10 live: "the step's whole output (2 lines): <copy>/.gate/netcode-proof-test/
  failed-steps/deps.log" (503 B, 2 lines). Round 1's runs 1, 2 and the control re-run: +simd128 in
  CARGO_ENCODED_RUSTFLAGS FAIL (read-back + FX-1), CARGO_BUILD_RUSTFLAGS=-Ctarget-cpu=native FAIL (non-vacuity + FX-1),
  bleeding-edge alone FAIL (read-back relaxed-simd + FX-1), each exit 1. Clean `--only deps`: PASS, exit 0, and
  failed-steps/ empty after it (the passing run removed deps.log). TP2 run 3 now fails by name: settled.
- TP4 r2-te-tp4-body-unsafe.log (-x -k, runner exit 1 by design). Planted diff in crates/hash-wasm/src/lib.rs, old
  "pub extern \"C\" fn begin(bite: u32, arm_tick: u32) -> u32 {\n    let Some(replayer) = replayer_for(bite, arm_tick)
  else {", new the same with "    let bite = unsafe { core::ptr::read_volatile(&bite) };\n" after the brace (sha256
  afc08ca7, the builder's fx-3-tp4 form and round 1's). `cargo build --locked --release --target wasm32-unknown-unknown
  -p npt-hash-wasm` exit 0 (Compiling npt-hash-wasm, the copy's path); `cargo clippy --locked --target
  wasm32-unknown-unknown -p npt-hash-wasm -- -D warnings` exit 0; `cargo test --locked -p npt-hash-wasm
  --no-fail-fast` "test result: ok. 5 passed; 0 failed"; `node check.mjs --only deps` **FAIL, exit 1**:
  "crates/hash-wasm/src/lib.rs:101 has `unsafe` outside an attribute; npt-hash-wasm's only unsafe is the
  #[unsafe(no_mangle)] of its listed exports (...)", its line. Restored by Edit: 9d339055 = start. As round 1: the scan
  is the only layer for this case and it bites.

## The main session, after the cut (labelled; not this seat's own line)
- The org's spend limit cut this seat at 01:22:58 UTC ("You've hit your org's monthly spend limit · ask your admin for
  a higher limit · your 5-hour limit resets 3:50am (UTC)"), with TP10's plant in place: two lines added in
  check.mjs's hashWasmProblems after `if (allowed.has(m.index)) continue;`, `const fnAt = code.lastIndexOf('pub extern
  "C" fn ', m.index);` and a `continue` for an `unsafe` inside a listed export's body. Its run's log
  .gate/u4d2/r2-te-tp10-scan-regression.log was written at 01:22:57; no result line was recorded here.
- Restored by the main session by Edit at about 08:26 UTC: check.mjs's sha256 343c59c5… = r2-hashes-start.sha256's;
  `sha256sum -c` of r2-hashes-start from the copy's root: 18 of 18 OK; `diff -rq` of spikes/ against .gate/u4d2-r2:
  equal. TP10 is to be re-planted in this seat's own form after the resume (its 01:22:57 log is not a recorded
  result).

## Resume (this seat, after the cut)
- Re-checked at the resume: `sha256sum -c r2-hashes-start.sha256` from the copy's root 18 of 18 OK; `cmp` with
  fx-hashes-end.sha256 equal; git status 9 M, 10 new; HEAD 51520b4; crates/sim diff empty; no cargo, rustc or check.mjs
  running (the main session's checkpoint saver only). The copy's failed-steps/ holds deps.log (235 B, 01:22, TP4's
  `--only deps` FAIL: lib.rs:101's line), left by the cut; a passing deps run removes it (FX-10). Read again: the
  packet's diffs, its tests side file, Block 8; the three batch-1 seats' notes; the builder's "Fix pass, round 1".
  Continuing at step 3 (TP10, TP11), then step 4, 5, 6.
