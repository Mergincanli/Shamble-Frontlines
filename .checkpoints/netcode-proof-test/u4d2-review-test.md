# U4d2 round 1: test-engineer review notes (log prefix te)

Role: test-engineer (origin/main .claude/agents/test-engineer.md). Copy I run in: .claude/worktrees/u4d2-build (the only runner there now).
Read copy (byte-identical): .gate/u4d2-r1. Base copy: .gate/u4d2-prefix (51520b4, spikes/ = bfda3cd).
Every cargo/check.mjs/loader run goes through u4d2-run.sh; logs .gate/u4d2/r1-te-<name>.log.

## Plan
1. sha256 of every changed and new file under spikes/netcode-proof in u4d2-build; compare with u4d2-r1.
2. Bites (§5 U4d2 Bites line; §4.14 hash-map row; U4D2-Q5, Q11, Q12):
   - the builder's six plants (p1-features .. p6-usize): read the logs; re-plant in my own form where I doubt it;
     each seen to fail by name in the check or test that must catch it.
   - PM-0004-21 hash-map and std-sin in the wasm step: can either verdict pass for the wrong reason
     (Wasm leg not run, equal digests because nothing hashed, committed scan stale)?
   - export-signature bite (an export taking an f64, Q7): plant it if the builder did not.
3. h1..h8 (brief's numbering) and check.test.mjs's new and re-pinned self-tests: assertion vs print, wrong-reason pass,
   missing test/clause.
4. Carries with a test: 52 (generator refusal), 65 (failing-test names), 66 (#[ignore] test run by name): one plant each where cheap.
5. The builder's TE2-5 entry: the check.test.mjs test that failed mid-build; does it still pin what it pinned at <base>?
6. One counted `node check.mjs` with -x at the end; step lines; counts as deltas on 399 / 446 / 52 / 10 steps.
7. End: every file byte-identical to (1)'s hashes; `git diff --stat -- spikes/netcode-proof/crates/sim` empty.
Also: settle by run any finding of rust/security/infra notes a run can settle.

## Notes (a line after each finding)

### Step 1: sha256 at start (u4d2-build = u4d2-r1 for all 17; `diff -rq` of spikes/ empty)
6e628ab736b6957504049ee913533b9292a8065907092e1acda272333733e082 Cargo.lock
16b1353c97000f60b7a148e757062b93cb3d2323e64dc2e6e75aac2626fece78 Cargo.toml
e657f7b8a70efb6752f4238196209928752384283f66851cd21be7797d30710e check.mjs
04edf7955cba6943dba52071ae0bc251f04fb96b09fceb0e62cb3f9f966a17cf check.test.mjs
c68877f33877359caf2a30965b94015f6a3f8f3f9542d48273cd6c5d7d0fab20 crates/client-wasm/Cargo.toml
da4e9b7ee9345dcb49313005ecacca96286ee056c87845c90a1136f3bd023bf8 crates/lab/src/lib.rs
50531c4fab4e4f5329dee1bfe9394f24be2d81ca4af3408828f99886f229e5d2 crates/lab/src/main.rs
64f48629f8d75d71e2050fca977cff2f97c211628481559f84eeeeb6b645dfd1 crates/lab/src/probe.rs
c1cc693bb3887225a02e3f93176981aa9617dad53ed21e3501c6610e884e594e crates/proto/Cargo.toml
d3aef8209bbec3beeca3d02bb2cdd151b205f26e7496af2dc7f27f9c728b57a4 crates/hash-wasm/Cargo.toml
76326a6455d94746cdf4c536f9622fbd742c46098b587ca18e142cc50c36314d crates/hash-wasm/co-initial-dump.txt
7c3c214b33e7b9a44bd6dda19c21c271d7c201f9d8e4d7027e37206ddac2418c crates/hash-wasm/loader.mjs
9d339055169e22dcd5f19f41121d7af62c6613dc84e6b305fa824a69e9e52d8b crates/hash-wasm/src/lib.rs
08b20e900953e72e950bd2a68d4739b6f2b332d46c16ca8001d1dcfd20a18896 crates/lab/clippy.toml
9979c409286b58e77f9638d11f511bfcbd3cca14973a1b741efed8011c3e376f crates/lab/recordings.json
3b11bc2c38ba27fd360e346c4f610f04270253b3b07323e2ff6025ea38e55cbe crates/lab/src/hash.rs
905ae0576ae6146ffdefc09bd6d5540e5b522489f7b80ee157d6395d9e98bc2f crates/lab/std-sin-scan.txt

### Resume after a usage cut (2026-11-12, about 21:00 UTC)
- Re-hashed all 17 files in u4d2-build: each equals step 1's list (no plant was left in place); git status shows the
  same 9 modified + 8 new files; `git diff --stat -- spikes/netcode-proof/crates/sim` empty. No r1-te-* log existed, so
  no run of mine had started. Re-read: role file (origin/main), runner, packet §5 U4d2 / §4.11 / §4.14 / §4.15, brief
  2.1-2.7 and 3.1, the builder's record, the rust/security/infra notes, hash-wasm lib.rs and loader.mjs, lab lib.rs and
  hash.rs (legs, serve, compare, vectors, tests), probe.rs/main.rs diff, check.mjs's wasm step, check.test.mjs's new
  tests (1905-2638).
- The builder's six plant logs read (p1..p6): each fails by name in the check or test that must catch it (P1 deps names
  npt-sim [bites] in the hash-wasm build vs the server's and npt-lab's; P2 three FAILs, name scan and read-back; P3 deps
  lib.rs:212 + rustc exit 101; P4 deps misplaced allow + unsafe, loader "refused: ... "extra""; P5 h2 (i) FAILED and the
  wasm step's h2 word 3; P6 h2 (i) 262 vs 260 words, native 1-2 27000/0, native-Wasm 27000 mismatching from SS-s1 tick 0).
  Doubted: P1 (npt-sim's own feature, not a dependency below it, and not one target's), P3 (rustc refuses it too).

## My plants and runs (planned; each one runner call, -x -k, restored by Edit, sha256 checked)
- TE-R0 (no plant): hash-wasm tests both builds with --test-threads=1 (h3's "before begin" leans on a fresh thread_local).
- TP1 PM-0004-74 my form: crates/sim/Cargo.toml gains a wasm32-only `libm` feature (force-soft-floats); deps must FAIL
  naming libm and the builds; Cargo.lock's sha256 unchanged.
- TP2 flags, env only: CARGO_ENCODED_RUSTFLAGS -C\x1ftarget-feature=+simd128; CARGO_BUILD_RUSTFLAGS -Ctarget-cpu=native;
  and RR-M2's fail-open (CARGO_ENCODED_RUSTFLAGS bleeding-edge + RUSTFLAGS -simd128,-relaxed-simd: deps PASS expected).
- TP3 a cargo config source: .cargo/config.toml [build] rustflags ["--codegen", "target-feature=+relaxed-simd"].
- TP4 an `unsafe` block inside a listed export's body (RR-M1 / SEC-U4D2-2): rustc and clippy accept, deps must refuse.
- TP5 Q7's export taking an f64 (`feed(word: f64)`): wasm32 build exit 0, deps must FAIL by name; the loader on it.
- TP6 carry 52: the generator refusal moved after generation: h5's generator test must FAIL (kept != 0).
- TP7 carry 65: failingTests keeps only the first name: check.test.mjs's carry-65 test must FAIL.
- TP8 carry 66: the v1 tag dropped from v1_datagram_sha256: h7 (i) and h7 (ii) by name on release must FAIL.
- TP9 TE2-5: the builder's `&& all.has("npt-sim")` removed: the base test "npt-sim ships with libm..." must FAIL.
- End: one counted check.mjs (-x, the six pre-builds), hashes, crates/sim diff.

## Results (a line after each)
- TE-R0 r1-te-r0-threads.log (-x -k, exit 0): hash-wasm `--test-threads=1` "test result: ok. 5 passed; 0 failed" and
  bites "ok. 7 passed; 0 failed": h3's "before begin" holds when it does not run first (libtest gives each test its own
  thread, so the thread_local is fresh). No finding.
- TP1 r1-te-tp1-features.log (-x -k, exit 1 by design). Planted diff, crates/sim/Cargo.toml after `rand_core.workspace =
  true`: old "rand_core.workspace = true\n", new "rand_core.workspace = true\n\n[target.'cfg(target_arch = \"wasm32\")'
  .dependencies]\nlibm = { workspace = true, features = [\"force-soft-floats\"] }\n". deps FAIL, 4 lines: "npt-sim's libm
  v0.2.16 resolves [arch, default] in the npt-server (x86_64-unknown-linux-gnu, measured) build and [arch, default,
  force-soft-floats] in the npt-hash-wasm (wasm32-unknown-unknown, measured) build", the same for npt-lab measured and
  for both in the bites set. `cargo tree --locked ... -i libm` exit 0 (no lock change). Restored: fb98df48 = HEAD's
  blob; Cargo.lock 6e628ab7. So the live check reads cargo's per-target resolution below npt-sim, not only npt-sim's own
  features (the builder's P1). No finding.
- TP2 r1-te-tp2-flags.log (-x -k, no file changed): CARGO_ENCODED_RUSTFLAGS=-Ctarget-feature=+simd128 FAIL (name scan +
  read-back); CARGO_BUILD_RUSTFLAGS=-Ctarget-cpu=native FAIL (name scan, x86-64 fma, wasm32 non-vacuity);
  **CARGO_ENCODED_RUSTFLAGS=-Ctarget-cpu=bleeding-edge with RUSTFLAGS=-Ctarget-feature=-simd128,-relaxed-simd: PASS
  ("check: all 1 steps pass", exit 0)**; the control, bleeding-edge alone: FAIL on simd128 and relaxed-simd. Cargo uses
  the first set of CARGO_ENCODED_RUSTFLAGS, RUSTFLAGS, target and build rustflags (cargo config docs), so the read-back's
  superset is fail-open when a negation sits in a source cargo skips. RR-M2 settled by run a second time (agree, Medium,
  before merge). The self-test (check.test.mjs:2173-2186's argv pin) pins the superset and has no negation case.
- TP3 r1-te-tp3-config.log (-x -k, exit 1 by design). Planted: .cargo/config.toml old "target-dir = \"../../.gate/
  netcode-proof-test/target\"\n", new the same + "rustflags = [\"--codegen\", \"target-feature=+relaxed-simd\"]\n". deps
  FAIL: "<copy>/.cargo/config.toml's rustflags passes \"-C target-feature=+relaxed-simd\"" and the wasm32 read-back lists
  simd128 and relaxed-simd. Restored 9ff1aa86 = HEAD's blob. No finding.
- TP4 r1-te-tp4-body-unsafe.log (-x -k, exit 1 by design). Planted in crates/hash-wasm/src/lib.rs, begin's body: old
  "pub extern \"C\" fn begin(bite: u32, arm_tick: u32) -> u32 {\n    let Some(replayer) =", new the same with
  "    let bite = unsafe { core::ptr::read_volatile(&bite) };\n" after the brace. Release wasm32 build exit 0; clippy
  wasm32 -D warnings exit 0; hash-wasm tests "ok. 5 passed; 0 failed"; deps FAIL "crates/hash-wasm/src/lib.rs:101 has
  `unsafe` outside an attribute". So rustc, clippy and the tests accept it; the scan is the only layer and catches it
  today. RR-M1 / SEC-U4D2-2 settled by run: agree (Medium, before merge: the self-test at check.test.mjs:1916, the deps
  fixture plant at :1280 and the builder's P3 all plant in a new fn, where rustc's deny also refuses, so all would pass
  with a scan that skipped export bodies). Restored 9d339055.
- TP5 r1-te-tp5-f64.log (-x -k, exit 1 by design). Planted: old "pub extern \"C\" fn feed(word: u32) {" ...
  "r.feed(word);", new "pub extern \"C\" fn feed(word: f64) {" ... "r.feed(word as u32);". Release wasm32 build exit 0
  (Compiling npt-hash-wasm, worktree path); clippy wasm32 -D warnings exit 0; deps FAIL "crates/hash-wasm/src/lib.rs:118
  exports feed with parameter word: f64: ... u32 or i32 only (the advisor's U4D2-Q7 ruling)". And the loader ran the f64
  module on TC-s3: "9000 ticks stepped", and `npt-lab hash compare` against the clean step's TC-s3 lines: "9000 ticks
  compared, 0 mismatching, equal". So the Q7 source scan is the invariant's only guard: the loader checks names and kinds,
  not types, and the determinism comparison cannot see an f64 that carries integers. Fits SEC-U4D2-3 (Low: the source
  scan reads tokens; Q7 (b), the loader reading the type section, would back it). Restored 9d339055.
- TP6 r1-te-tp6-carry52.log (-x -k, exit 101 by design). Planted in crates/lab/src/hash.rs regenerate_all: the first
  loop (the generator refusal over every entry) removed and the same `if e.generator != Recording::GENERATOR { return
  Err(...) }` put after `keep(e, &bytes)?;` in the generation loop (old: refusal loop, then "let mut lines ...; for e in
  entries { let bytes = generate(e)?; check(e, &bytes)?; keep(e, &bytes)?; lines.push(" ; new: "let mut lines ...; for e
  in entries { ...keep(e, &bytes)?; if e.generator != ... { return Err(...) } lines.push("). FAILED by name:
  hash::tests::h5_a_generator_other_than_recording_generator_is_refused_before_anything_is_generated, "assertion `left ==
  right` failed: the good entry before it was not generated either, left: 2, right: 0"; "test result: FAILED. 5 passed; 1
  failed". Carry 52's "refused before generating" bites. Restored 3b11bc2c.
- TP7 r1-te-tp7-carry65.log (-x -k, exit 1 by design). Planted in check.mjs failingTests: old "if
  (!names.includes(name)) names.push(name);", new "if (!names.length) names.push(name);". `node --test check.test.mjs`:
  "not ok 59 - the test step names every failing test from libtest's failures: list ... (carry 65; j-r2 B2.3)" (only
  hash::tests::h5_a named of two); "# tests 63 / # pass 62 / # fail 1". Carry 65 bites. Restored e657f7b8.
- TP8 r1-te-tp8-carry66.log (-x -k, exit 101 by design). Planted in probe.rs v1_datagram_sha256: old "h.update(V1_TAG);",
  new "h.update(&V1_TAG[..0]);" (v1's tag dropped). h7 (i) FAILED by name (probe.rs:566, left b9810ef6..., right
  50028c02...); h7 (ii) by name on release FAILED: "P2 seed 1", left d4e02112..., right 913db8e9...14b11ecc; "test result:
  FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 18 filtered out". Carry 66's #[ignore] test bites when run by name.
  Restored 64f48629.
- TP9 r1-te-tp9-te25.log (-x -k, exit 1 by design). Planted in check.mjs (the builder's TE2-5 fix undone): old
  "if (!shippedDeps.includes(\"npt-sim\") && all.has(\"npt-sim\")) problems.push(", new "if
  (!shippedDeps.includes(\"npt-sim\")) problems.push(". "not ok 9 - npt-sim ships with libm, rand_chacha and rand_core
  alone ..." (actual ['npt-sim is not a workspace member', 'npt-hash-wasm does not depend on npt-sim']); 62 of 63.
  Restored e657f7b8.
- Step 5 (TE2-5 entry): the test (check.test.mjs:205-222) is byte-identical to <base>'s (sed range diff: SAME). Its
  last assert pins `gone` deepEqual ["npt-sim is not a workspace member"]: a missing npt-sim said once. The builder's
  change is to its own new rule (check.mjs:138, `&& all.has("npt-sim")`): with npt-sim not a member, hash-wasm's "does not
  depend on npt-sim" is a second message for the same cause, which the test's "said once" exists to refuse (TP9 shows
  the test catches it). The new rule's own bite stays tested where npt-sim is a member (check.test.mjs:1959:
  ["npt-hash-wasm does not depend on npt-sim"]). So the test still pins what it pinned at <base>; the fix is right and
  the test was not weakened. The builder's self-classification (judged within its step, not a stop) is a process
  call for the advisor: the failure was in check.test.mjs, which the brief's stop clause names, and it was not named in
  the step's plan in advance; it was recorded, explained and ended in the same step. No finding of mine (Low at most).
- TP10 r1-te-tp10-scan-regression.log (-x -k, exit 0: the gap shown). Planted in check.mjs hashWasmProblems' unsafe loop,
  after "if (allowed.has(m.index)) continue;": two lines that skip any `unsafe` after a listed export's
  `pub extern "C" fn <name>` and before its closing "\n}\n" (tcpinfo's "unsafe inside the listed functions" pattern,
  the regression RR-M1 and SEC-U4D2-2 name). `node --test check.test.mjs`: "# tests 63 / # pass 63 / # fail 0"; `node
  check.mjs --only deps`: PASS. So no self-test, fixture plant or live plant pins the export-body case (TP4 shows the
  real scan catches it today). RR-M1 / SEC-U4D2-2 fully settled: Medium, before merge (a self-test case plus TP4's plant).
  Restored e657f7b8.
- TP11 r1-te-tp11-loader-kind.log (-x -k, exit 0: the gap shown). Planted in loader.mjs:53: old "else if
  (LOADER_EXPORTS[name] !== kind) problems.push(", new "else if (false && LOADER_EXPORTS[name] !== kind) problems.push(".
  self: 63 of 63 pass. SEC-U4D2-8 settled by run: the kind refusal is untested (Low, follow-up). Restored 7c3c214b.
- SEC-U4D2-1 (Object.prototype imports) was already settled by the security seat's own run (r1-sec-imports.log):
  `constructor.constructor` instantiates with moduleProblems []. Not re-run (read-only probe; same code, same copy).

## Step 2's wrong-reason question, by reading plus the runs above
- hash-map: a Wasm leg that did not run fails (loader exit != 0 is a problem; a missing lines file leaves no comparison);
  an unarmed or wrongly armed Wasm leg gives a dump without the two `bite[0].hash_map_order` words (SS's unarmed dump is
  6,488 words, the field sits at [6488, 6489]), so its digest reads "none" and the pair is "not judged", a FAIL; each Wasm
  leg is its own Node process with its own dump file; the native-Wasm pair is exercised (native keys are per process) and
  must be first caught at (SS-s1, 1234, the field). "Not catchable on wasm32" is the record of equal Wasm digests, which
  std 1.99.1's address-keyed wasm32 RandomState predicts; it never passes the step alone (no pair exercised fails).
- std-sin: the committed scan cannot carry a stale pass: each run re-arms at T' and needs the first native-Wasm mismatch
  at (R, T', F), so a build where the bite no longer differs, or differs earlier, FAILs (P6 shows "NOT caught ... first
  mismatch SS-s1 tick 0"). Residual path (TE-L3 below): the step does not re-show that the bite causes the T' mismatch
  (bitten against clean); for the committed catch (SS-s1, tick 2) the hash-map pair's equal prefix (bites builds,
  native = Wasm on SS-s1 ticks 0-1233) covers it; for a catch on ES-s2 or TC-s3, or past tick 1,234, nothing would.
- "Digests equal because nothing was hashed": the clean half's comparator is the same `hash compare` code that catches
  std-sin's one-ulp change at its first tick and P6's word-size change at tick 0, and the hashes are npt-sim's state hash
  (h3: the exports equal Replayer's own per tick). No path found.

## Step 6: the counted check.mjs (r1-te-final-check.log, -x, runner exit 1)
- Before: every file = step 1's hashes (plus crates/sim/Cargo.toml fb98df48 and .cargo/config.toml 9ff1aa86 = HEAD's);
  git status 18 lines (9 M, 8 new, the builder's notes file); df 7.3G free; shared target 6.9G.
- Six pre-builds 23:07:47-23:09:44, each exit 0; `node check.mjs` 23:09:44-23:15:05. Step lines: fmt PASS 0.8 s;
  clippy PASS 21.3 s; clippy-wasm32 PASS 4.3 s; doc PASS 70.9 s; **test FAIL 174.9 s** "the bites build's tests
  (measured build: 416 tests passed, 0 failed, 0 reported a skip; bites build: 342 tests passed, 1 failed, 0 reported a
  skip) failed: error: test failed, to rerun pass `-p npt-net --lib`", with carry 65's line "the bites build's failing
  tests (libtest's failures: list): l2::delay::tests::a_shutdown_s_never_completes_ends_at_its_deadline_without_spinning";
  deps PASS 1.3 s; wasm PASS 40.0 s (27000/0 both pairs, h2 equal, hash-map caught natively and native-Wasm, Wasm 1-2
  not exercised, std-sin caught at SS-s1 2); refs PASS 0.3 s; marks PASS 5.8 s; self PASS 1.5 s "63 of 63"; redact
  PASS 0.8 s; "check: 1 step(s) failed".
- **TE2-5** (not planted; not U4d2's crate): npt-net's
  `l2::delay::tests::a_shutdown_s_never_completes_ends_at_its_deadline_without_spinning` (crates/net/src/l2/delay_tests.rs:
  2552, a multi-thread tokio test that counts pump runs, at most 3) failed once in the bites build of the full test
  step. Its panic text is not recoverable: check.mjs keeps cargo's output in memory and prints the last 40 lines, and
  carry 65's line kept the name but the panic block was above them. Re-run at once through the runner,
  r1-te-te25-npt-net.log (`cargo test --locked -p npt-net --all-features --lib --no-fail-fast`, -x): "test result: ok.
  169 passed; 0 failed; 12 ignored", the test "... ok". The same test failed once under parallel load in U3c
  (u3c-fix2-c2b.md:74, delay_tests.rs:2002 then). Not in a crate U4d2 changes and reads none of its files, so by B2.3 it
  does not stop U4d2's merge; it is a recurring flake in U3's code to root-cause (the main session's, as a STATUS
  follow-up). Carry 65's naming worked live here (a positive check of the carry); its panic did not survive the 40 lines.
- For the counts, after the TE2-5 record: r1-te-final-test2.log (-x; `cargo test --no-run` both builds, every npt-*
  crate "Compiling" twice from the worktree path, then `node check.mjs --only test`): "PASS test 251.5 s measured build:
  416 tests passed, 0 failed, 0 reported a skip; bites build: 465 tests passed, 0 failed, 0 reported a skip".
- Counts as deltas on <base> (399 / 446 / 52 of 52 / 10 steps): measured 416 (+17: hash-wasm 5, lab lib 5, lab bin 7),
  bites 465 (+19: 7, 5, 7), self 63 of 63 (+11), steps 11 (+1, `wasm` after `deps`). Each equals the builder's e1/e2.
  The counted run's provenance: 5 x "Compiling npt-hash-wasm ... (<worktree>/spikes/netcode-proof/crates/hash-wasm)",
  5 x npt-lab's, 6 x npt-sim's (4 pre-build lines each for hash-wasm and lab, plus one inside the test step's printed
  tail). wasm step seconds: builds 5.7, 6.0, 1.3, 1.4; npt-lab hash 4.0; Wasm 3.5, 1.2, 0.1; hash-map 2.8, 3.1, 3.2, 0.6,
  0.3; std-sin 3.0, 3.4, 0.1; step 40.0 s.

## Step 7: the end state
- All 17 files' sha256 equal step 1's and the read copy's (.gate/u4d2-r1); `diff -rq spikes/` against the read copy
  empty; crates/sim/Cargo.toml fb98df48 and .cargo/config.toml 9ff1aa86 (HEAD's); `git diff --stat --
  spikes/netcode-proof/crates/sim` empty; git status 18 lines, its md5 as before my runs. df 7.2G free; shared target
  7.0G. Nothing committed; no copy cleaned.

## Findings (ranked)
- TE-M1 Medium, before merge (= RR-M2, settled by my run as well): check.mjs readBackCheck (:1522-1553) is given every
  rustflags source at once and DEPS_CHECKED's limits text says that "fails closed"; cargo uses only the first non-empty
  source, so a negation in a source cargo skips masks the flag it uses. Proof: r1-te-tp2-flags.log run 3, deps PASS with
  CARGO_ENCODED_RUSTFLAGS=-Ctarget-cpu=bleeding-edge and RUSTFLAGS=-Ctarget-feature=-simd128,-relaxed-simd (control
  FAILs); the module cargo builds there carries +simd128 +relaxed-simd (r1-rr-superset-build.log). The argv-pin self-test
  (check.test.mjs:2173-2186) pins the superset and has no such case. Fix: read back what cargo uses (each source alone,
  or cargo's precedence per target) and/or refuse a built module whose target_features section lists simd128 or
  relaxed-simd; one self-test with the negation case.
- TE-M2 Medium, before merge (= RR-M1 / SEC-U4D2-2, settled by TP4 and TP10): the one case the scan alone guards, an
  `unsafe` block in a listed export's body, has no self-test, no fixture plant and no live plant (check.test.mjs:1916,
  :1280 and P3 all plant a new fn, which rustc's deny refuses too). TP4: rustc, clippy -D warnings and the unit tests
  accept it, deps refuses it (lib.rs:101). TP10: a scan regression that lets it through passes 63 of 63 self-tests and
  the live deps step. Fix: one self-test assertion (an `unsafe {}` inside begin's body refused at its line) and the live
  plant once (build exit 0 beside deps FAIL).
- TE-L1 Low, follow-up (= SEC-U4D2-8, settled by TP11): loader.mjs:53's kind refusal is untested (63 of 63 pass with it
  disabled). Fix: two cases in the loader test (a listed name as a global; `memory` as a function).
- TE-L2 Low, follow-up (supports SEC-U4D2-3): Q7's export-signature bite is unit-tested only (check.test.mjs:1948); TP5's
  live plant shows the deps scan catches `feed(word: f64)` by name, and that nothing else would: the loader runs the f64
  module and its TC-s3 hashes equal the clean module's over 9,000 ticks. The scan reads the token; the loader's type
  section read (Q7 (b)) would be the second layer.
- TE-L3 Low, follow-up: std-sin's step verdict (check.mjs:2333-2349) does not re-show causality: it never compares the
  bitten native leg with the clean one at T' (the clean half's native-1.lines already holds R's clean hashes), and the
  `std-sin native legs` call's own result is unread. Sound for the committed catch (SS-s1, tick 2, inside the hash-map
  pair's equal prefix); unguarded if the first catch moves to another recording or past tick 1,234. Fix: require the
  bitten native leg's first difference from a clean bites-build native leg to be (R, T'), or assert the catch lies on
  rec0 before HASH_MAP_ARM.
- TE-M3 Medium, not before U4d2's merge (the main session's follow-up): npt-net's
  `l2::delay::tests::a_shutdown_s_never_completes_ends_at_its_deadline_without_spinning` (delay_tests.rs:2552) failed once
  in my counted check.mjs (bites build, under the full workspace's parallel load) and passed on the TE2-5 re-run and the
  second test-step run; it failed the same way in U3c (u3c-fix2-c2b.md:74). A recurring flake in U3's code that can fail
  a merge's counted run. Not U4d2's crate; to root-cause (its pump-run count under load), never retried into green.
- TE-L4 Low, follow-up: carry 65 names the failing test (worked live above) but its panic is lost: check.mjs keeps the
  test output in memory and prints 40 lines, so when the TE2-5 re-run passes, B2.3's "name and panic" cannot be met.
  Fix: save a failed cargo step's whole output under .gate/ and print its path, or print each failing test's panic line
  beside its name.

## Claim, proof, gap (role output)
| Claim | Proof | Gap |
|---|---|---|
| 0 mismatches native x2 and Wasm over 27,000 ticks (h1) | final run's wasm line; e1/e2; per-recording counts judged | none |
| cross-target vectors (h2) | h2 (i) unit; P5, P6 plants | none |
| exports = Replayer (h3) | unit tests; --test-threads=1 (TE-R0) | none |
| HashCompare (h4) | 5 unit tests | none |
| manifest + carry 52 (h5) | unit tests; TP6 | none |
| hash args (h6) | unit tests | exit codes by mapping, not asserted (fine) |
| carry 66 (h7) | h7 (i), (ii) by name; TP8 | none |
| features like for like | self-test; P1; TP1 (per-target libm) | none |
| build flags refused, read-back fails closed | self-tests; P2; TP2, TP3 | superset fails open (TE-M1) |
| hash-wasm unsafe only in no_mangle | self-tests; P3, P4; TP4 | export-body case untested (TE-M2) |
| integer-only exports | self-test; TP5 | source scan only (TE-L2) |
| loader refuses extra or wrong-kind exports, imports | self-test; P4 | kind untested (TE-L1); Object.prototype imports (SEC-U4D2-1) |
| PM-0004-21 bites verdicts | step runs; self-tests | std-sin causality not re-shown (TE-L3) |
| carry 65 | self-test; TP7; live in the counted run | panic lost (TE-L4) |

## Verdict
REVISE: two Mediums before the merge, both found by the rust and security seats and settled here by run (TE-M1 = RR-M2,
TE-M2 = RR-M1 / SEC-U4D2-2), each a few lines and one self-test; TE-M3 (npt-net flake) not before U4d2's merge; Lows
TE-L1 to TE-L4 as follow-ups. Every other bite named in §5 U4d2 bites by name; counts +17 / +19 / +11 / +1 step.
