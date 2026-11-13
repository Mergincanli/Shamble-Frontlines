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
- TP10 (this seat's own form; check.mjs hashWasmProblems, after `if (allowed.has(m.index)) continue;`). Planted diff,
  old "    if (allowed.has(m.index)) continue;\n    const a = attrs.find(...)", new the same with, between the two:
  "    if (f === HASH_WASM_UNSAFE_FILE) {\n      const fnAt = code.lastIndexOf('pub extern \"C\" fn ', m.index);\n
  const open = fnAt < 0 ? -1 : mask.indexOf(\"{\", fnAt);\n      let inside = open >= 0 && open < m.index;\n      for
  (let k = open, depth = 0; inside && k < m.index; k++) {\n        depth += mask[k] === \"{\" ? 1 : mask[k] === \"}\" ?
  -1 : 0;\n        if (depth === 0) inside = false;\n      }\n      if (inside) continue;\n    }" (an `unsafe` skipped
  while the nearest export's body brace is still open; sha256 84d01add). My first two forms were vacuous, mine not
  the tests': `mask.lastIndexOf('pub extern "C" fn ')` never matches, as lexRust blanks literals in `mask` (`"C"`);
  r2-te-tp10b-scan-regression.log ran that form: 71 of 71 pass, not counted as a result. Corrected form run
  r2-te-tp10c-scan-regression.log (-x -k; runner exit 1 by design): `node --test check.test.mjs` "# tests 71 / # pass
  69 / # fail 2": **not ok 38** "the deps step is depsStep, and it calls every check ... (review C2R-5)" ("unsafeProblems
  (npt-hash-wasm, inside an export's body: U4d2 round 1, FX-3)'s plant is not reported"; the stray fn's line still
  reported, so the plant hit the body case only) and **not ok 54** "npt-hash-wasm: an `unsafe` block inside a listed
  export's body is refused at its line ..." (begin: `unsafe {}`, actual [], expected lib.rs:9's line). `node check.mjs
  --only deps` on the real tree under the plant: PASS (21.3 s; the real lib.rs has no body unsafe; this run also
  removed the cut's leftover failed-steps/deps.log). Restored by Edit: 343c59c5 = start. TP10 now fails by name: settled.
- TP11 (this seat's own form, not the builder's `false &&`): loader.mjs moduleProblems, old "    else if
  (LOADER_EXPORTS[name] !== kind) problems.push(", new "    else if (LOADER_EXPORTS[name] !== kind && kind ===
  \"table\") problems.push(" (the kind check kept for tables only; sha256 a15170f1). r2-te-tp11-loader-kind.log (-x -k;
  runner exit 1 by design): "# tests 71 / # pass 70 / # fail 1": **not ok 59** "the loader refuses a listed name
  exported as another kind: begin as a global, memory as a function (U4d2 round 1, FX-5: SEC-U4D2-8, TE-L1)" (actual
  [], expected 'the module exports "begin" as a global, not a function'). Round 1: 63 of 63 green under TP11. Restored
  by Edit: f2f2293f = start. TP11 now fails by name: settled.

## Step 4: plants where a proof was not shown or a seat's finding can be settled by run
- FX-2's self-test against its own rules (the builder's FX-2 proof was the live simd128 plant only, fx-2-tf.log; no
  plant showed the self-test itself bites). Three plants in loader.mjs, each one -x -k call, each restored to f2f2293f
  (sha256 checked after each):
  - A, the prefix rule: old `if (REFUSED_FEATURES.includes(name) && prefix !== "-") {`, new `... && prefix === "=") {`
    (sha256 3f2afc54): r2-te-fx2a-prefix.log "# pass 70 / # fail 1", **not ok 57** (the FX-2 test), at "+simd128":
    actual [], expected the "lists +simd128" line.
  - B, the count rule: old `if (sections.length !== 1) {`, new `if (sections.length === 0) {` (8e8bd064):
    r2-te-fx2b-count.log 70 / 1, **not ok 57** at the two-sections case (actual [], expected "has 2 target_features
    sections, not one").
  - C, the parse's end: old `if (at !== bytes.length) throw new Error(`, new `if (at > bytes.length) throw new Error(`
    (49b74e39): r2-te-fx2c-trailing.log 70 / 1, **not ok 57** (actual [], expected "does not parse: 1 byte(s) follow its
    last entry").
  The FX-2 self-test bites on each rule it names (deepEqual on whole problem lists). No finding.
- INF2-2 (infra, by reading) settled by run: check.mjs run(), old `maxBuffer: 1 << 28, ...(timeout ? { timeout } : {})
  });`, new `maxBuffer: 1 << 28 });` (the timeout never reaches spawnSync; sha256 be0a86f4).
  r2-te-inf2-2-run-timeout.log (-x -k; exit 0): "# tests 71 / # pass 71 / # fail 0". FX-7's self-test sees `call`'s
  wiring and the line, not run()'s: a dropped timeout leaves every test green. Restored by Edit: 343c59c5. Agree: Low
  (TE2-R2-1 below, the same item; the ruled proof was "one self-test", which it has).
- RR2-L3 (rust, by reading) settled by run: hash.rs compare, old `let armed = |e: &Entry| arm_for(arm.as_ref(),
  on.as_deref(), e);`, new `... arm_for(arm.as_ref(), Some(e.name.as_str()), e);` (every recording armed again, as
  before FX-8; sha256 4cfb4053). r2-te-rr2-l3-compare-arm.log (-x -k; exit 0): `cargo test --locked -p npt-lab --bin
  npt-lab --no-fail-fast` "test result: ok. 19 passed; 0 failed; 1 ignored"; the same `--all-features` "19 passed; 0
  failed; 1 ignored"; `node check.mjs --only wasm` PASS 54.9 s (a plant run, not a timing figure), "check: all 1 steps
  pass". The only trace is rustc's "unused variable: `on`" (my one-line form leaves `on` unused; clippy -D warnings
  would refuse that, but a clean revert of the two lines leaves nothing). Restored by Edit: 5677c39c. Agree: Low (check.mjs
  always passes `--on`, so no verdict depends on compare's default today).
- SEC-U4D2-R2-3 (security; built outside the copy) settled live in the copy: hash-wasm/src/lib.rs, two Edits: before
  begin's doc, new "macro_rules! m {\n    ($($t:tt)*) => {\n        $($t)*
  (\"/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/sec-r2-incl-gen.txt\")\n    };\n}\n\n"; as begin's
  first line, new "    let bite = bite.wrapping_add(m!(include!)).wrapping_sub(7);\n" (the security seat's gen file,
  outside the copy and the repo's tracked files, its text `unsafe { core::ptr::read_volatile(&7u32) }`; no new file in
  the copy; sha256 dfb43f46). r2-te-sec-r2-3-macro-include.log (-x -k; exit 0, every command): wasm32 release build
  exit 0 (Compiling npt-hash-wasm, the copy's path: the include resolved and the unsafe block compiled under begin's
  allow, the crate's deny notwithstanding); clippy wasm32 -D warnings exit 0; clippy host --all-targets -D warnings exit
  0; `cargo test -p npt-hash-wasm` "5 passed; 0 failed"; `node check.mjs --only deps` **PASS**, its held line still
  saying "every #[path] and include! literal names a .rs file in its own crate" and "npt-hash-wasm ... whose
  #[unsafe(no_mangle)] is its only unsafe code". Restored by two Edits: 9d339055; `sha256sum -c` 18 of 18 OK. Agree:
  Low (FX-6's ruled form, V4's two forms, is met; this near form is new, and review is the stated wall in the
  deny-level crates), with one addition: the deps step's PASS line asserts a claim that is false under it.

## Step 4 by reading: the eight new self-tests and FX-8's unit test (claim, what bites it, could it pass wrongly)
- FX-1 (packet-tests :343-379): deepEqual on buildInputProblems' whole list for the pair, the three SEC-U4D2-5 flags
  (beside buildFlagProblems [] for each: the name rule alone passes them, so the new rule is the one that bites), every
  env and config form, CARGO_BUILD_TARGET ("" included), three build.target forms; four negatives (the container's
  variables with empty RUSTFLAGS and CARGO_ENCODED_RUSTFLAGS, the spike's real .cargo/config.toml, a target table's
  linker, target-dir, `rustflags = []`, an [env] value holding "target = y"); and the pair through depsStep (C2R-5's
  rule). The builder's plant (fx-1-plant.log) and my TP2 run 3 bite it. Only the doc step gives its cargo children a
  flags variable (RUSTDOCFLAGS, check.mjs:1952-1953); no step sets a *RUSTFLAGS for its children, so the deps step reads
  what every step's cargo gets. Sound.
- FX-2 (:279-309): deepEqual throughout; built list and `-` names accepted; +/= of both names, the tf2 list, none, two,
  six malformed forms, exportProblems' path. My plants A, B, C each bite it (above). Sound.
- FX-3 (:129-148): every listed export x two forms, plus three deeper forms; the line computed from the planted text.
  TP10 (mine) bites it and the C2R-5 test. Sound.
- FX-4 (:311-326): four modules (Block 1 asks for three) and a pair, each beside the helper's clean target_features
  section, so FX-2's rule cannot stand in for FX-4's (the builder's note on sec-probe-imports holds); exact messages
  from moduleProblems and from instantiate (an engine LinkError would not match). fx-4-plant.log bites it. Sound.
- FX-5 (:328-334): two cases, deepEqual. TP11 (mine) bites the first; the second is checked after the counted run.
- FX-6 (:150-189): deepEqual on the whole list for each case: build.rs in each crate, the renamed include! and r#include,
  both manifest forms, both metadata targets; negatives (the macros by name, prose, a string, another crate, npt-bots'
  build script) and the real crates []. fx-6-plant.log disabled the .rs rule only, but the manifest and layout cases are
  deepEqual against [], so a removed rule there fails too (by reading). No `m!(include!)` case: SEC-U4D2-R2-3, settled
  above as a gap, not a wrong pass of this test.
- FX-7 (:440-453): each fake call's limit, exactly four zero limits, two timed-out calls named within main's tail. It
  cannot see run(): INF2-2, settled above (71 of 71 with the timeout dropped).
- FX-10 (:388-410): a temp dir passed as `keep` (no self-test writes into the real failed-steps/), the panic above the
  40 lines, the file whole, the path line, the last line, and the removal on the next run with another step's file
  kept. fx-10-plant.log bites it. No timing dependence (no clock read). Sound.
- FX-9 (one assert added to the bites test): the whole tail from the libc part to the sentence. fx-9-plant.log bites
  it. Sound.
- FX-8's unit test (hash.rs, a_digest_field_of_a_dump_of_another_length_says_the_layouts_differ): equal length gives
  the u64 digest, longer and shorter give "layouts differ", empty and one-index forms; fx-8-plant.log bites it. Its
  reach is a length change (P6's form, the one Block 3 names); two layouts of one length are not separable by length,
  which no ruling asks. h6's added asserts pin bitten_name and arm_for, not compare's use of them: RR2-L3, settled above.
- Block 4 asks that "both" FX-5 cases fail under TP11. My first form stopped at the first assert, so a second form:
  old "    else if (LOADER_EXPORTS[name] !== kind) problems.push(", new "... !== kind && kind === \"global\")
  problems.push(" (`begin` as a global still refused, `memory` as a function not; sha256 ae09294c).
  r2-te-tp11b-loader-kind-memory.log (-x -k; runner exit 1 by design): 70 / 1, **not ok 59** at its second assert
  (actual [], expected 'the module exports "memory" as a function, not a memory'). Restored by Edit: f2f2293f. Each of
  the two cases fails by name on its own: settled.

## Step 5: one counted check.mjs (done)
- r2-te-check.log (-x, in the background with a 2,400,000 ms limit; lock 08:36:53, end 08:45:00 UTC, runner exit 0):
  the six pre-builds each exit 0 (`cargo test --workspace --no-run`, the same `--all-features`, npt-lab release and
  `--features bites`, npt-hash-wasm wasm32 release and `--features bites`), then `node check.mjs` exit 0, "check: all 11
  steps pass": fmt 0.8 s; clippy 20.4 s; clippy-wasm32 2.7 s; doc 81.2 s; test 216.0 s "measured build: 417 tests
  passed, 0 failed, 0 reported a skip; bites build: 466 tests passed, 0 failed, 0 reported a skip"; deps 1.3 s; wasm
  34.8 s; refs 0.4 s (793 tracked files); marks 5.8 s (81 marks); self 1.6 s "71 of 71 tests passed"; redact 0.7 s (10
  of 10; 247 items, 0 findings).
- Counts as deltas on 416 / 465 / 63 of 63 and 11 steps: measured 417 (+1), bites 466 (+1), self 71 of 71 (+8), steps
  11 (+0); equal to the fix pass's fx-check1 and fx-check2. No failure anywhere, so no TE2-5 entry (npt-net's TE-M3
  test passed in both builds); failed-steps/ empty after the run.
- The wasm step's line: native 1 against 2 and native against Wasm 27000 ticks, 0 mismatching (SS-s1, ES-s2, TC-s3 9000
  each); h2 equal; hash-map native 1-2 caught at SS-s1 tick 1234, bite[0].hash_map_order (761badec63c37be5,
  48b7293ffebe2fe5), native-Wasm caught (761badec63c37be5, f5e8d0e818b168c5), Wasm 1-2 not exercised (f5e8d0e818b168c5
  both): not catchable on wasm32; std-sin caught at SS-s1 tick 2, vehicle[8].pos.y, as the scan found (other catch
  ES-s2 1000002). Split (s): builds 5.6, 5.7, 1.3, 1.3 (13.9); npt-lab hash 3.4; Wasm 2.8, 0.9, 0.1; compare 0.0; h2
  0.0 x5; bites 0.0; hash-map legs 2.3, Wasm 2.6, 2.7, compares 0.6, 0.3; std-sin legs 2.3, Wasm 2.7, compare 0.1 (rest
  20.9). The Wasm digest f5e8d0e8... equals round 1's and the fix pass's; the native hash-map digests vary per run as
  before. Provenance: 4 x npt-hash-wasm, 4 x npt-lab, 6 x npt-sim Compiling lines, each the copy's path.

## Step 6: end (done, 08:47 UTC)
- `sha256sum -c r2-hashes-start.sha256` from the copy's root: 18 of 18 OK, exit 0. `git diff --stat --
  spikes/netcode-proof/crates/sim`: empty. git status: 9 M and 10 new, as at the start; HEAD 51520b4; nothing committed.
  `diff -rq` of the copy's spikes/netcode-proof against .gate/u4d2-r2's: empty, exit 0. failed-steps/ empty. No file
  in the copy was created or removed by this seat (the SEC-R2-3 plant named a file outside the copy).

## Findings (ranked; none Medium or above)
- TE2-R2-1 (Low; with INF2-2, the same item; follow-up): check.mjs:1851-1856, run()'s timeout wiring is pinned by no
  test. Scenario: an edit drops `...(timeout ? { timeout } : {})` or reads the wrong error code, and a hung leg holds
  the exclusive lock again with every self-test green. Proof: r2-te-inf2-2-run-timeout.log (71 of 71 with it dropped).
  Cheapest fix: export run (or a wrapper) and one self-test, `process.execPath -e "setTimeout(()=>{},1e4)"` with a
  200 ms limit gives timedOut true. Block 8's ruled proof ("one self-test") is met; not before the merge.
- TE2-R2-2 (Low; with RR2-L3; follow-up): hash.rs:1036-1037, :1042, :1071, :1083: compare's one default for `--on` is
  pinned by no test. Proof: r2-te-rr2-l3-compare-arm.log (every recording armed again: 19 passed in both builds, wasm
  step PASS). Fix: one bin test that runs compare on two recordings with an `--on`-less arm and asserts the second
  replays unarmed. No verdict depends on it today (check.mjs always passes `--on`); not before the merge.
- TE2-R2-3 (Low; with SEC-U4D2-R2-3; settled live): the deps step PASSes, its held line saying "every #[path] and
  include! literal names a .rs file in its own crate", while begin's body includes a .txt outside the repository through
  `m!(include!)`, and its unsafe block compiles under begin's allow (r2-te-sec-r2-3-macro-include.log: wasm32 build,
  both clippy runs, the crate's tests and deps, each exit 0). The security seat's fix (no macro_rules! in the
  deny-level crates, or the lookahead `(?!\s*!\s*[([{])`) with one self-test case each; not before the merge (the same
  class as SEC-U4D2-4: a deliberate committed change, review the stated wall there).
- Process note (mine, no finding): two of my TP10 forms were vacuous (searched `mask`, where lexRust blanks `"C"`); the
  runs are kept and labelled (r2-te-tp10b), and only the corrected form's run (r2-te-tp10c) is counted.

## Claim / proof / gap
| Claim (Block 8) | Proof re-shown here | Gap |
|---|---|---|
| FX-1: every flagged source and a set target refused, by name | TP2 run 3 FAIL naming both sources (r2-te-tp2-flags.log); self-test deepEqual (read); no step gives its children a *RUSTFLAGS | none |
| FX-2: the section read refuses +/= simd128, relaxed-simd, none, two, unparsable | plants A, B, C each fail the FX-2 test by name; counted run accepts both built modules | none (SEC-U4D2-R2-1's link-arg route is the security seat's) |
| FX-3: an `unsafe` in an export's body refused at its line | TP4 live (deps FAIL at lib.rs:101, build and clippy exit 0); TP10 fails tests 38 and 54 | none |
| FX-4: every import refused by name before instantiating | read; the builder's plant; the security seat's runs | none |
| FX-5: both kind cases | TP11 (begin) and TP11b (memory), each not ok 59 | none |
| FX-6: V4's two forms refused | read (deepEqual); the builder's plant | macro-token include! (TE2-R2-3) |
| FX-7: 300 s on each non-build call, a named FAIL | read; the builder's plant | run()'s wiring untested (TE2-R2-1) |
| FX-8: "layouts differ"; one default for `--on` | the builder's plant on digest_text | compare's default untested (TE2-R2-2) |
| FX-9, FX-10, FX-11, FX-12 | the builder's plants (read); FX-10 live (failed-steps/ written and removed in my runs) | none |
| Counts and steps | r2-te-check.log: 417 / 466 / 71 of 71, 11 of 11 | none |
| Bounds | 18 of 18 equal at start and end; crates/sim untouched; diff -rq empty | none |

## Verdict
APPROVE WITH FINDINGS: three Lows (TE2-R2-1 to -3, each also another seat's finding, now settled by run), none
before the merge. Every FX meets its ruled form and proof; TP2 run 3, TP4, TP10 and TP11 (both cases) now fail by
name; one counted run passes all 11 steps at 417 / 466 / 71 of 71; every file is byte-identical to its start hash.
