# U4d2 build notes: the Wasm-in-Node hash and the determinism step (builder, opus, xhigh; brief rev 1.2, sha256 146acf18)

## Block 0: start (done)
- Worktree /home/user/Shamble-Frontlines/.claude/worktrees/u4d2-build, branch u4d2-build, HEAD 51520b43391a453a3e37ce235b9f5753df55ab93.
- `<base>` bfda3cd2e4733efc3d3ec9f7c6c3c99bc6c2e8c4. `git diff --stat bfda3cd HEAD -- spikes/` printed nothing (exit 0).
- `git merge-base --is-ancestor adb1bab HEAD` exit 0.
- df -h /home/user at start: 7.6G avail (252G size, 81% used).
- Done; next: read the sources (brief 1.6), then the plan of blocks below.

## The plan (blocks, in order)
1. Manifests: Cargo.toml members + lints comment (third exception) + carry 29's libm comment; crates/proto/Cargo.toml
   :13-14 (carry 41); crates/client-wasm/Cargo.toml :9-13; crates/hash-wasm/Cargo.toml; the lock by `cargo metadata
   --offline --format-version 1` through the runner (its diff here).
2. npt-hash-wasm src/lib.rs (seven exports, thread_local state, crate doc with the third exception's invariant), the
   committed CO vector file, unit tests h2 (i) and h3.
3. npt-lab: `HashCompare` + `TickHash` (lib.rs), the `hash` subcommand (src/hash.rs; legs in lock step, D-2's form),
   recordings.json, clippy.toml (Q10 (a)), tests h4, h5, h6.
4. Carry 66: probe.rs + main.rs (v1 datagram SHA-256), tests h7 (i), h7 (ii) (#[ignore]).
5. The Node loader crates/hash-wasm/loader.mjs (export list, refusal fn, streaming, dumps).
6. check.mjs: wasm step; deps: features comparison, build-flag check, hash-wasm rules, third exception; carry 65; texts.
7. check.test.mjs: every self-test of 2.4; the four exception pins and the fixture pin re-pinned in the same step.
8. Runs: first builds + CARGO_TARGET_DIR check; std-sin scan; wasm step cold/warm; the plants of 2.7.
9. End checks: check.mjs twice with rule 3's six builds; h7 (ii)/(iii); sizes.

## Block 1: manifests and the lock (done)
- Cargo.toml: members + "crates/hash-wasm"; libm comment reworded (carry 29); lints comment names three exceptions.
  crates/proto/Cargo.toml :13-14 reworded (carry 41); crates/client-wasm/Cargo.toml :9-13 names three; new
  crates/hash-wasm/Cargo.toml (deny, the workspace's clippy keys, cdylib, bites -> npt-sim/bites, npt-sim only).
- Lock: first call `cargo metadata --offline --format-version 1 --no-deps` (my slip: --no-deps resolves nothing; it
  wrote nothing, b1-lock.log, exit 0); then the brief's call `cargo metadata --offline --format-version 1`
  (.gate/u4d2/b1-lock2.log, exit 0, shared lock). Cargo.lock's diff: one `[[package]]` npt-hash-wasm 0.0.0,
  dependencies ["npt-sim"], inserted after npt-client-wasm. Nothing else.
- df before: 7.7G free; shared target 6.8G before my first build.

## Block 2: npt-hash-wasm (done)
- src/lib.rs: seven exports, thread_local RefCell state, statuses private consts; inline test module (so the
  bite-home scan reads its one test-only id as a test).
- TE2-5 (expected, by design): h2 (i) failed in b2-hw-test2.log (both builds, 1 failed each) against a placeholder
  vector file `digest 0000000000000000`; its message printed the expected 260 words and digest b432ac332054c77b
  (= D-7's 12984630011248297851). Ended by writing crates/hash-wasm/co-initial-dump.txt with Write/Edit (one Edit added
  15 zero lines I had missed; checked against the log by diff: SAME). Also b2-hw-test1.log: E0277 (an extern "C" fn
  is no FnMut; a closure).
- b2-hw-test3.log (-k, shared): measured 5 passed 0 failed; bites 7 passed 0 failed; wasm32 release build exit 0
  (114,282 B); clippy host both builds and wasm32 bites, -D warnings, exit 0. No target/ in the worktree; artifacts in
  .gate/shared-target (CARGO_TARGET_DIR checked).
- The release module's exports (Node, WebAssembly.Module.exports): memory, begin, dump_len, dump_word, feed, hash_hi,
  hash_lo, step; imports: none. `memory` is the linker's (wasm-ld exports the memory of a wasm32-unknown-unknown
  cdylib); no __data_end or __heap_base at rustc 1.99.1.
- Next: block 3 (npt-lab).

## Resume after the cut (17:43 UTC)
- A fresh copy after a usage-limit cut: read these notes, the brief whole (sha256 146acf18 checked), `git diff` and the
  new files in the worktree, and the runner logs b3-lab1..4 and b4-lab5 (the cut fell after b4-lab5, before blocks 3
  and 4 were written here). Blocks 3 and 4 below are written from the diff and those logs.

## Block 3: npt-lab HashCompare, the manifest, the `hash` subcommand, clippy.toml (done)
- lib.rs: `TickHash` (:10-17) and `HashCompare` (:23-60: Equal{entries}, Empty, Mismatch{index, ours, theirs,
  compared, mismatching}); h4's five tests (:78-145).
- src/hash.rs (new): parse (modes legs, compare, vectors, bites; a bite in the measured build refused, exit 2),
  manifest reader, regenerate_all (generator checked for every entry before any is generated), the two native legs in
  lock step (current_exe, env NPT_LAB_HASH_LEG; the field diff by first_difference across the pipe), compare (counts
  per recording checked against the manifest; field diff by a native replay to the tick), vectors (h2 (ii)), bites;
  h5's three tests, h6's tests (two each build), a dump-file round trip test.
- crates/lab/recordings.json: three entries (SS-s1, ES-s2, TC-s3), generator 2, their bytes and SHA-256.
- crates/lab/clippy.toml (Q10 (a)): the HashMap/HashSet type bans and server/clippy.toml's clock ban.
- main.rs: `hash` dispatched (exit 0 equal, 1 failed or mismatch, 2 usage); docs :1-14.
- TE2-5 (expected, by design, as block 2's vector): the manifest's sha256 began as 64 zeros; `npt-lab hash` failed
  naming each entry in turn (b3-lab1: SS-s1 ab3cc842…; b3-lab2: ES-s2 e3257dbd…; b3-lab3: TC-s3 c018b650…), each
  written in by Edit. Ended at b3-lab4: all three "as the manifest's".
- TE2-5 (my test's error, not the code's): b3-lab1 h4_lists_of_different_lengths failed (the test list's names
  depended on its length; fixed to name by k < 20). b3-lab4 h5_a_regenerated_entry failed (the test asserted the
  wrong prefix on a changed byte's message); fixed; b4-lab5 green.
- **Slip:** b3-lab1, b3-lab2 and b3-lab3 ran `npt-lab hash` under the shared lock (-s), where section 4 rule 2 asks
  -x for every run of `npt-lab hash`. None of the three is a result I report (each was the placeholder's expected
  failure); every later `npt-lab hash` run is -x.
- b3-lab4 (-x): `native leg 1 against native leg 2: 27000 ticks compared, 0 mismatching, equal (SS-s1 9000, ES-s2
  9000, TC-s3 9000)`, exit 0.
- Done; next: block 4.

## Block 4: carry 66 (done)
- probe.rs: `fate_log_v1_datagram_sha256` in `Outcome`; `v1_datagram_sha256` (v2 tag skipped, 40-byte entries with
  flow code 0 hashed after v1's tag; a log whose first piece is not v2's tag refused); h7 (i)
  `h7_the_v1_datagram_hash_is_v1s_tag_and_the_logs_datagram_entries`; h7 (ii)
  `probe::tests::h7_carry_66_the_probes_v1_datagram_hashes_are_u2s_for_p2_b1_and_p4_seed_1` (#[ignore]).
- main.rs: header param `fate_log_v1_datagram_sha256` beside `fate_log_sha256`; the summary line ends "; v1 datagram
  fate log <first 16 hex>" (the whole-log hash's form).
- TE2-5 (my test's error): b3-lab4 h7 (i) failed `(6000, 0, 4876)` against `(6000, 0, 0)`: v2 logs acknowledgements
  beside the datagrams (56 bytes, flow code 3); the test now asserts acks > 0 and hashes datagrams only.
- b4-lab5 (-x): npt-lab measured: lib 5 passed, bin 18 passed 1 ignored; bites: lib 5, bin 18 passed 1 ignored;
  clippy -D warnings both builds exit 0; h7 (ii) by name on release: "test result: ok. 1 passed; 0 failed; 0
  ignored; 0 measured; 18 filtered out" with P2 913db8e9…1ecc, B1 feabf2f0…25bf, P4 b8a04f2f…602b (each U2's full
  value); whole-log v2 hashes P2 43745683…, B1 722f2071…, P4 432693ef….
- df at resume: 7.5G free; shared target 6.9G.
- Done; next: block 5, the Node loader.

## Block 5: the Node loader (done)
- crates/hash-wasm/loader.mjs (new): HASH_WASM_EXPORTS (the seven), LINKER_EXPORTS ({memory}), LOADER_EXPORTS (name ->
  kind), BEGIN, STEP; moduleProblems(module) / exportProblems(bytes) (the refusal: unlisted name or other kind, by
  name; a listed name missing); instantiate (empty import object, rethrows naming the imports); forEachChunk (64 KiB
  blocks, last word zero-padded); replay (begin, feed, step until NEED_MORE, `h` lines, `stepped n` or `failed ...`,
  dump at a tick or `end`); parse; main (exit 0, 1, 2).
- hash.rs: `compare --ours/--theirs` take one or more lines files, comma-separated, read in order (the loader writes
  one per recording; so check.mjs writes no lines file itself, and a by-hand run needs no shell write to join them).
- b5-loader1.log (-x -k, exit 0; a by-hand measured half, not the step): release builds (wasm32 hash-wasm, npt-lab),
  `npt-lab hash` (27000/0, equal), three loader runs (9000 ticks each; SS 3 s, ES 2 s, TC < 1 s), `compare`:
  "native-leg-1 against wasm-leg-1: 27000 ticks compared, 0 mismatching, equal (SS-s1 9000, ES-s2 9000, TC-s3
  9000)"; four header-only loader runs with --dump-at end; `vectors`: "h2: CO's wasm32 dump is the committed vector
  (260 words, digest b432ac332054c77b); every scene's initial dump and hash are equal natively and on wasm32 (SS 6488,
  ES 1944, CO 260, TC 340)".
- Done; next: block 6, check.mjs.

## Blocks 6 and 7 as one step (check.mjs and check.test.mjs edited together; self runs only after both)
- Read-only probe b6-tree1.log (-s): `cargo tree --locked -e normal --target <t> -p <pkg> [--features bites]
  --no-dedupe --prefix depth --format "{p} [{f}]"` prints `<depth><name> v<ver>[ (<path>)] [<f,f>]`; npt-sim's
  subtree in npt-server's x86-64 tree and hash-wasm's wasm32 bites tree: npt-sim, libm [arch,default], rand_chacha [],
  ppv-lite86 [simd], zerocopy [simd], rand_core []; `rustc --print cfg` from spikes/netcode-proof: wasm32 the six of
  notes F1; x86-64 with -C target-cpu=haswell lists fma among 21; `rustc 1.99.1 (58cf8f9c3 2026-10-13)`.
- The step's plan, named in advance (section 4's stop clause): the four pins that quote the exception list
  (check.test.mjs:288, :486, :508, :1141) re-pinned with the UNSAFE_EXCEPTIONS edit; and the fixtures that the new
  members of LEAF, UNSAFE_EXCEPTIONS and EXCEPTION_LEVELS make incomplete, rebuilt in the same step: clean()'s graph
  gains npt-hash-wasm; cleanCrates() and depsFixture() gain hash-wasm's clean lib.rs and Cargo.toml (so :1099's
  fixture count, 6 crate files, becomes 8: its conditional pin); tcpinfoLints' and "the two exceptions hold" test's
  manifest maps gain hash-wasm's (its two "both"/"cannot be read" asserts name three); depsStep gains its runner for
  `cargo tree` and `rustc` (the fixture test's call passes a fake, as it passes metadata). The steps pin gains the
  `wasm` step's commands.

## Blocks 6 and 7: check.mjs and check.test.mjs (built; self green; the real runs follow in block 8)
- check.mjs: header; imports HASH_WASM_EXPORTS from crates/hash-wasm/loader.mjs (the list's one home); LEAF +
  npt-hash-wasm; dependencyProblems: hash-wasm ships npt-sim only, nothing depends on it; UNSAFE_EXCEPTIONS and
  EXCEPTION_LEVELS + hash-wasm (deny), exceptionNames() ("a, b and c"); HASH_WASM_UNSAFE_FILE, EXPORT_INTEGERS,
  NO_MANGLE; exportAfter, hashWasmProblems (placed allows, the no_mangle's unsafe only, u32/i32 signatures) and its
  per-export count in unsafeProblems; texts two -> three (UNSAFE_CODE, lexRust, unsafeProblems, workspaceLint,
  exceptionLint, DEPS_CHECKED and its doc, the runner-refusal note); rustflagSources, buildFlagProblems, READ_BACK,
  readBackCheck; SIM_BUILDS, treeArgs, simSubtree, simFeatureCheck; failedRun's note, failingTests,
  failingTestsLine (carry 65); depsStep(here, env, metadata, runner); the wasm step: RECORDINGS_MANIFEST,
  STD_SIN_SCAN, H2_VECTOR, LOADER, WASM_STEP_DIR, WASM_BITES, HASH_MAP_ARM, HASH_MAP_FIELD, WASM_BUILDS,
  builtArtifact, readComparison, readScan, WASM_IO, wasmStep; STEPS.wasm after deps (11 steps).
- hash.rs: `npt-lab hash` writes the header-only recordings first and says so at once (println), so h2 runs even if a
  regeneration fails (2.4 (a): "a failed regeneration ends only the legs that need it").
- check.test.mjs: imports; clean() + npt-hash-wasm; cleanCrates and depsFixture + hash-wasm's clean lib.rs and
  manifest; the four exception pins re-pinned ("only npt-client-wasm, npt-tcpinfo and npt-hash-wasm may"); tcpinfoLints
  and the three-exceptions test + hash-wasm's manifest (its "allow"/"forbid" refused); depsFixture's runner
  (depsRunner, cfgModel, treeText), the fixture pin 6 -> 8 crate files with the read-back and features lines, the
  deps runner's calls pinned, five new plants (flag, read-back, features, hash-wasm unsafe, hash-wasm dependency) and
  the RUSTFLAGS target-cpu=native case; 11 new tests (hash-wasm rules; loader list and refusal; loader streaming and
  errors; build flags; read-back; features; carry 65; wasm step pin; wasm step failures; bites verdicts;
  readComparison/builtArtifact).
- TE2-5 (not named in the step's plan; a defect of my in-progress new rule, caught by an existing test): the first
  self run (node --test, direct, no cargo) was 62 of 63: "npt-sim ships with libm..." (check.test.mjs:205-222) got
  "npt-hash-wasm does not depend on npt-sim" beside "npt-sim is not a workspace member" (its "said once" rule). Cause:
  my new hash-wasm rule; fixed in check.mjs (the rule says nothing when npt-sim is not a member), the test unchanged.
  Second run: "# tests 63 / # pass 63 / # fail 0". I judged this within the step (the code under test was mine and
  unfinished), not a stop; the report says so.
- Next: block 8's runs (Rust tests/fmt/clippy for the hash.rs edits; deps step live; the std-sin scan; the wasm
  step cold/warm; the plants).

## Block 8: runs (in progress)
- b8-rust1.log (-x -k): fmt check FAILED with rustfmt diffs in blocks 2-4's files (never fmt-checked before the cut)
  and one E0596 (`let mut lines` lost in my hash.rs reorder); `node check.mjs --only deps` PASS live. Both fixed by
  Edit (rustfmt's 45 hunks applied by hand, section 4's write rule). TE2-5: my own errors, ended in b8-rust2.
- b8-rust2.log (-x -k, exit 0): fmt clean; npt-lab + npt-hash-wasm tests measured (hash-wasm 5, lab lib 5, lab bin 18
  + 1 ignored) and bites (7, 5, 18 + 1 ignored), all 0 failed; clippy -D warnings host both builds, wasm32
  npt-hash-wasm both builds, exit 0.
- The std-sin scan (b8-scan1.log, b8-scan2.log, -x -k; exit 1 = the compares that mismatch, by design): rustc 1.99.1
  (58cf8f9c3 2026-10-13); ldd (Ubuntu GLIBC 2.39-0ubuntu8.9) 2.39. SS-s1 armed at 0: native vs Wasm 1375 of 9000
  differ, first tick 2; native bitten vs clean 1375, first 2; Wasm bitten vs clean 0. ES-s2 armed at 1000000: 1326,
  first 1000002; native bitten vs clean 1326; Wasm bitten vs clean 0. Armed at T' with dumps: SS-s1 tick 2
  vehicle[8].pos.y (word 2148) c159186a vs c1591869; ES-s2 tick 1000002 vehicle[8].pos.y (word 996) c0099f7a vs
  c0099f79 (one ulp). Committed as crates/lab/std-sin-scan.txt (two catches; the step arms SS-s1's). So no std-sin
  scratch plant (Q5 (a): only if they never differ).
- The wasm step, first run (b8-wasm1.log, -x, exit 0): PASS 42.9 s (the release deps were already in the shared
  target from my by-hand runs; the workspace's crates rebuilt under the touch loop). Its line: native 1 vs 2 27000/0;
  native vs Wasm 27000/0 (9000 each); h2 equal; hash-map native 1-2 caught at SS-s1 1234 bite[0].hash_map_order
  (ecadfe40f54478b5, 3ececd16d1432605), native-Wasm caught (ecadfe40f54478b5, f5e8d0e818b168c5), Wasm 1-2 not
  exercised (f5e8d0e818b168c5 both): not catchable on wasm32; std-sin caught at SS-s1 tick 2 vehicle[8].pos.y.
  Seconds: builds 6.2, 6.6, 1.5, 1.6; npt-lab hash 4.4; Wasm 3.5, 1.2, 0.1; compare 0.0; h2 0.0 x5; bites 0.0;
  hash-map legs 3.0, Wasm 3.3, 3.5, compares 0.7, 0.4; std-sin legs 3.2, Wasm 3.5, compare 0.1. Shared target 7.1G
  before and after (du -sh); df 7.4G free.

- The wasm step, second run (b8-wasm2.log, -x, exit 0): PASS 43.5 s; builds 6.1, 6.7, 1.4, 1.5; npt-lab hash 4.8;
  Wasm 3.5, 1.1, 0.1; hash-map legs 3.1, Wasm 3.5, 3.5, compares 0.7, 0.3; std-sin 3.3, 3.6, 0.1.
- Two refinements after the warm run, before the plants (self 63 of 63 after): simFeatureCheck compares every pair of
  builds (so a difference names npt-lab's build too, 2.7's PM-0004-74 row); h2 (i)'s failure names the first
  differing word and its field.
- Clean sha256s before the plants: hash-wasm/Cargo.toml d3aef820…, Cargo.lock 6e628ab7…, hash-wasm/src/lib.rs
  9d339055…, co-initial-dump.txt 76326a64…, sim/src/world.rs 6e011013… (= HEAD's blob).

## Block 8b: the plants of 2.7 (done; each by Edit in place, one -x -k call, restored, sha256 checked)
- P1 PM-0004-74 (p1-features.log): hash-wasm's `npt-sim = { workspace = true, features = ["bites"] }`; `node
  check.mjs --only deps`: FAIL "npt-sim's npt-sim v0.0.0 resolves [] in the npt-server (x86_64-unknown-linux-gnu,
  measured) build and [bites] in the npt-hash-wasm (wasm32-unknown-unknown, measured) build" and the same for npt-lab's.
  Restored d3aef820…; Cargo.lock 6e628ab7… (no lock byte moved).
- P2 flags (p2-flags.log, no file changed): RUSTFLAGS="-C target-feature=+fma": FAIL by the name scan and the x86-64
  read-back (fma); RUSTFLAGS="-C target-cpu=haswell": FAIL by the read-back only (x86-64 lists fma; wasm32 lists no
  sign-ext); CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS="-C target-cpu=bleeding-edge": FAIL by the read-back only
  (wasm32 lists simd128 and relaxed-simd; x86-64's still has sse2).
- P3 stray unsafe (p3-unsafe.log): `fn stray() { unsafe {} }` in hash-wasm's lib.rs: deps FAIL "crates/hash-wasm/
  src/lib.rs:212 has `unsafe` outside an attribute; ..."; `cargo build -p npt-hash-wasm`: "error: usage of an `unsafe`
  block --> crates/hash-wasm/src/lib.rs:212:5", exit 101. Restored 9d339055….
- P4 eighth export (p4-export.log): `#[allow(unsafe_code, reason)] #[unsafe(no_mangle)] pub extern "C" fn extra()`:
  deps FAIL (lib.rs:211 relaxes unsafe_code on `pub extern "C" fn extra() -> u32 {`; lib.rs:212 has `unsafe` in
  #[unsafe(no_mangle)] on it); the wasm32 release build exit 0; the loader: "refused: the module exports "extra" (a
  function), which is not on the loader's list", exit 1. Restored 9d339055….
- P5 h2 vector word (p5-vector-word.log): word 3 00000001 -> 00000002: h2 (i) "the committed vector differs at word 3
  (soldier[0].alive): 00000002 committed, 00000001 built natively" (test result: FAILED. 0 passed; 1 failed); the
  wasm step FAIL "h2: CO's wasm32 dump differs from the committed vector at word 3 (soldier[0].alive): 00000002
  committed, 00000001 on wasm32", every pair still 27000/0. Restored 76326a64….
- P6 h2 usize in write_state (p6-usize.log; sim/src/world.rs, Q11 (b)): h2 (i) fails natively (262 words built, 260
  committed); the wasm step FAIL: native 1 vs 2 27000 compared, 0 mismatching; native vs Wasm 27000 compared, 27000
  mismatching, first at SS-s1 tick 0, field world[0].soldiers.len (word 4); h2 (ii): every scene's initial dump and hash
  differ on wasm32; and, as consequences, hash-map's native-Wasm pair and std-sin "NOT caught" (first mismatch at tick
  0). Restored 6e011013… (= HEAD's); `git diff --stat -- spikes/netcode-proof/crates/sim` empty.
- No std-sin scratch plant: the scan found catches (Q5 (a)).
- A cosmetic note seen under P6: with the word layouts differing by target, compare's --digest-field reads the Wasm
  dump at the native layout's indexes (the Wasm digest printed as one word, f5e8d0e8); without a word-size plant the
  layouts are equal (h2 (ii)), so the digest reads both words.
- Next: block 9, the end checks (check.mjs twice with rule 3's six builds; h7 (ii)/(iii); sizes; --list).

## Block 9: the end checks
- Regenerated recordings (node, reading only): SS-s1 16694921 B FNV-1a-64 35974458070191737; ES-s2 16700849 B
  15077317756489230056; TC-s3 799693 B 9126256229328065459: each D-7's pin (sim/tests/replay.rs:382-389).
- Q8 re-checked: `nm -D --defined-only libc.so.6` has, of the seven names, only `W step@GLIBC_2.2.5` (non-default).
- Run 1 (e1-check.log, -x, exit 0; df 7.3G, shared target 7.1G before): six builds (test --no-run both, release
  npt-lab both, release wasm32 npt-hash-wasm both) then `node check.mjs`: "check: all 11 steps pass". fmt 0.6 s;
  clippy 10.3 s; clippy-wasm32 1.3 s; doc 69.5 s; test 294.7 s "measured build: 416 tests passed, 0 failed, 0 reported
  a skip; bites build: 465 tests passed, 0 failed, 0 reported a skip"; deps 1.6 s (read-back: x86-64 fxsr, sse, sse2;
  wasm32 bulk-memory, multivalue, mutable-globals, nontrapping-fptoint, reference-types, sign-ext; npt-sim's features
  one set: measured npt-sim [], libm [arch, default], rand_chacha [], ppv-lite86 [simd], zerocopy [simd], rand_core [];
  bites the same with npt-sim [bites]); wasm 46.4 s (27000/0 both pairs; h2 equal; hash-map native 71db070315a81a25 /
  d04e6defd3bb7a45 caught, Wasm f5e8d0e818b168c5 both: not catchable on wasm32; std-sin caught at SS-s1 2
  vehicle[8].pos.y); refs 0.0 s; marks 3.3 s; self 1.5 s "63 of 63"; redact 0.8 s. Provenance: 4 x "Compiling
  npt-hash-wasm v0.0.0 (<worktree>/spikes/netcode-proof/crates/hash-wasm)", 4 x npt-lab's, 6 x npt-sim's.
- Deltas on <base> (399/446/52/10): measured +17 (hash-wasm 5, lab lib 5, lab bin 7), bites +19 (7, 5, 7), self +11,
  steps +1.
- Run 2 (e2-check.log, -x, exit 0): "check: all 11 steps pass"; fmt 0.6; clippy 4.2; clippy-wasm32 1.0; doc 68.4; test
  297.6 s (416 / 465, 0 failed, 0 skips); deps 1.6; wasm 46.5 s (27000/0 both pairs, h2 equal, hash-map native
  ecc3428c2a8d5215 / c0e59cf2d57c5255 caught, Wasm f5e8d0e818b168c5 both, std-sin caught at SS-s1 2); refs 0.0; marks
  1.1; self 2.1 "63 of 63"; redact 0.9. Provenance: 4 x npt-hash-wasm's and 4 x npt-lab's Compiling lines (worktree
  path).
- e3-h7.log (-x, exit 0): h7 (ii) "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 18 filtered out";
  h7 (iii): `npt-lab probe --set c-u2-pm-0001-3 --role measured --seed 1 --date 2026-11-12 --commit 51520b4… --run
  l1-<p>-s1 --profile <P2|B1|P4> --out <scratch>/h7-l1-<p>-s1.jsonl --twin l1-<p>-s1-twin`: P2 "...; fate log
  43745683ab36d67b; v1 datagram fate log 913db8e914bdb773", B1 "722f20713e18352e; feabf2f02d023fa9", P4
  "432693efb9698c05; b8a04f2f8ddc0d39"; `--list` names the 21 new tests.
- Wasm sizes (the step's copies): measured 114282 B raw, 39902 B gzip -9; bites 130368 B, 44188 B.
- df 7.5G at the end; shared target 7.1G; no target/ in the worktree; crates/sim diff empty.
- Done. Next: the return list (the main session).

## Design choices (plan silent), recorded as made
- Loader ticks: `step()` returns a status only, so the loader numbers ticks from the manifest's first tick (check.mjs
  passes it); the World's own tick is in the hashed state (`world[0].next_tick`), so a Wasm leg stepping other ticks
  mismatches by hash.
- Field diff: a native leg's live dump is compared by `StateDump::first_difference` (leg 1 against leg 2 in lock step,
  D-2's form; native against Wasm by an in-process native replay to the mismatching tick, deterministic once leg 1 and
  leg 2 agree). Two dumps that arrive as words alone (Wasm 1 against Wasm 2; native leg 1's dump file against Wasm 1's
  in the bites half) are compared word by word and named by the native replay's `StateDump::field`.
