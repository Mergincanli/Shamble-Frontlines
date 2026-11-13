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

## Fix pass, round 1 (the advisor's ruling advisor-u4d2-r1.md, Blocks 1-8; a fresh builder, 2026-11-12)
- Start (nothing changed before this): read this record whole, the ruling's Blocks 1 to 8, the five seats' notes
  (rust, security, infra, arch, test), the brief's section 4 (u4d2-brief.md:648-733), the runner, and the copy's
  `git status --untracked-files=all` (9 M, 9 new with this record) and `git diff`. HEAD 51520b4; the 17 files' sha256
  equal the test seat's step-1 list (check.mjs e657f7b8, check.test.mjs 04edf795, loader.mjs 7c3c214b, hash-wasm
  lib.rs 9d339055, hash.rs 3b11bc2c, lab lib.rs da4e9b7e, Cargo.toml 16b1353c, Cargo.lock 6e628ab7). df 6.5G free;
  shared target 7.7G. No section "Fix pass, round 1" existed (a fresh start).
- Bounds kept: check.mjs, check.test.mjs, crates/hash-wasm/loader.mjs, crates/hash-wasm/clippy.toml (new),
  crates/lab/src/hash.rs, crates/lab/src/lib.rs (one doc line), Cargo.toml's libm comment; hash-wasm's src/lib.rs only
  as FX-2's and FX-3's plants, restored. No Cargo.lock, manifest, recordings.json, std-sin-scan.txt,
  co-initial-dump.txt or crates/sim change. New exports: check.mjs `buildInputProblems` only (FX-1, imported by its
  self-test); loader.mjs none. New Rust items: one private fn in hash.rs (FX-8), no public one.
- Each FX's proof below is one `-x -k` runner call (fx-<n>-*.log), the plant by Edit in place (or the environment, no
  file changed), restored by Edit and its sha256 checked against the fixed form before the next call.

### The plan, one line per FX (proof; plant or environment; failures named in advance)
- **FX-1** (RR-M2/TE-M1, SEC-U4D2-5, -6): `buildInputProblems(env, configs)`, called by depsStep: every rustflags
  source rustflagSources reads that holds a flag is refused naming the source and its flags; CARGO_BUILD_TARGET (any
  value) and a `target` key in a cargo config (build.target, read in any table as the config keys are) refused.
  readBackCheck's doc and DEPS_CHECKED's limits say the read-back is exact because no source may hold a flag ("which
  fails closed" goes). Self-test (new): RR-M2's negation pair (both named), `--cfg feature="bites"`,
  `-C overflow-checks=off`, `-C llvm-args=-fp-contract=fast` in RUSTFLAGS, a config's rustflags, CARGO_BUILD_TARGET,
  `[build] target` and `build.target`, each by name; the container's environment and the spike's config pass; the
  pair through depsStep. Live (fx-1-env.log, environment only): r1-rr-superset.log's first environment `--only deps`
  FAILs naming both sources; CARGO_BUILD_TARGET and `RUSTFLAGS=--cfg feature="bites"` FAIL; the clean environment
  PASSes. Expected failures: none (the C2R-5 test's bad fixture gains one line, read by `includes`; I add
  buildInputProblems to its list in the same step). Plant (fx-1-plant.log): the refusal loop disabled in check.mjs:
  the new self-test FAILs by name.
- **FX-2** (RR-M2's second half, SEC-U4D2-7): loader.mjs's moduleProblems reads the `target_features` custom section
  (LEB128 count; per entry a prefix `+`, `-` or `=` and a LEB128-length name): refused when it lists simd128 or
  relaxed-simd with `+` or `=`, when there is no section or more than one, or when it does not parse (truncated, a
  bad prefix, bytes left over), each by name. Self-test (new): wasmModule-built modules for +simd128, +relaxed-simd,
  =simd128, no section, two, truncated, bad prefix; the real modules' list and -simd128 accepted. **Expected
  failures, ended in the same edit:** every existing loader test that builds a clean module with wasmModule
  (check.test.mjs's "the loader's list ..." test) would be refused for no section, so the helper gains a default
  clean section in the same step (kept short, so its "< 256 bytes" assert holds). Live (fx-2-tf.log): Block 2's
  plant, a safe `#[target_feature(enable = "simd128")]` fn under cfg(wasm32) called from `dump_word` in hash-wasm's
  lib.rs: wasm32 release build exit 0, `--only deps` PASS, `--only wasm` FAIL on the loader's refusal naming +simd128;
  restored to 9d339055. The two clean modules accepted: the counted runs' wasm step.
- **FX-3** (RR-M1, SEC-U4D2-2, TE-M2): self-test (new): an `unsafe {}` as the first line of each listed export's body
  refused by hashWasmProblems at its file:line; the C2R-5 fixture's stray-unsafe plant also put in begin's body (its
  new-fn case stays). **Expected:** that fixture's line numbers move (strayAt), re-computed in the same edit. Live
  (fx-3-tp4.log): TP4's plant (`let bite = unsafe { core::ptr::read_volatile(&bite) };` in begin's body): wasm32
  release build exit 0, `--only deps` FAIL at lib.rs:101; restored 9d339055. Plant (fx-3-tp10.log): TP10's scan
  regression in check.mjs (skip `unsafe` inside a listed export's body): the new self-test and the C2R-5 test FAIL by
  name; restored.
- **FX-4** (SEC-U4D2-1): moduleProblems refuses every `WebAssembly.Module.imports` entry by module and name, before
  anything is instantiated; instantiate passes `Object.create(null)`; the header and instantiate's doc say so.
  Self-test (new): modules importing `constructor.constructor`, `__proto__.valueOf` and `env.now`, each refused by
  name, by moduleProblems and by instantiate. **Expected failure, re-pinned in the same edit:** the existing loader
  test's assert of the old instantiate message ("does not instantiate with no imports (it imports env.now ...)").
  Plant (fx-4-plant.log): the import loop disabled: the new self-test FAILs by name; restored.
- **FX-5** (SEC-U4D2-8, TE-L1): self-test (new): `begin` exported as a global, `memory` as a function, each refused by
  name (wasmModule gains `kinds`). Plant (fx-5-tp11.log): TP11's disable (`false &&` on loader.mjs's kind branch): the
  new self-test FAILs by name; restored.
- **FX-6** (SEC-U4D2-4): in each exception crate whose level is deny (npt-tcpinfo, npt-hash-wasm): a build.rs file, a
  `build` key in its manifest (unsafeProblems) and a custom-build target in cargo metadata (layoutProblems) refused;
  `include` used as a path (not an include!, include_str! or include_bytes! call) refused in its sources;
  pathProblems' doc names npt-hash-wasm. Self-test (new): V4's two forms (r1-sec-scan.log: the build.rs writing
  `\x75nsafe` code; `use std::include as inc;` with `inc!(...)` in begin's body), the manifest key, the metadata
  target, each by name; the real crates pass. Expected: none (no deny crate has a build script or an `include` path).
  Plant (fx-6-plant.log): the build.rs rule and the include rule disabled: the new self-test FAILs by name; restored.
- **FX-7** (INF-4): wasmStep's `call` gives every non-cargo child call a 300 s timeout (run() passes spawnSync's
  `timeout` and reports `timedOut`); a timeout is a FAIL naming the call and the limit. Self-test (new): in a clean
  run each non-cargo call gets 300,000 ms and each cargo build none; a Wasm leg that times out FAILs naming it within
  main's 40 lines. Fixture: wasmWorld's runner records the limit and passes `timedOut` (no existing assert reads it).
  Expected: none. Plant (fx-7-plant.log): the timeout line dropped: the new self-test FAILs by name; restored.
- **FX-8** (RR-L3, EA-9): hash.rs: a private `digest_text` gives "layouts differ (<n> and <m> words)" in place of a
  digest when a dump's length is not this build's own; `compare` arms the bite on bitten_name's recording (`--on`'s,
  else the first listed: the legs' one default), replay_to no longer filters. Unit test (new, both builds) on
  digest_text; h6's bites test asserts the default (the first listed armed, the second not). Plant
  (fx-8-plant.log): digest_text's length check removed: the new unit test FAILs by name (`cargo test -p npt-lab
  --bin npt-lab`); restored. Expected: none (h6's added asserts are mine, in my step).
- **FX-9** (INF-3): the std-sin failure line adds that glibc picks `sinf` by the CPU's features when it loads (an
  IFUNC), so on another CPU or libc the scan is re-run there (u4d2-brief.md 2.4 (a) item 6). Self-test: the existing
  bites test's std-sin miss asserts the sentence (an assert added). Plant (fx-9-plant.log): the sentence dropped: that
  test FAILs by name; restored.
- **FX-10** (TE-L4): main(argv, steps, log, keepDir): before each step its file
  `<checkout>/.gate/netcode-proof-test/failed-steps/<step>.log` is removed; a failed step's whole output is written
  there and its FAIL lines name the path. Self-test (new): a recorded failing test step whose panic lies above the last
  40 lines: the file holds it, the printed lines name it, a later passing run removes it. Plant (fx-10-plant.log): the
  write dropped: the new self-test FAILs by name; restored. Expected: none (no test calls main).
- **FX-11** texts: check.test.mjs's three "two" (the tests at :328 and :529 renamed, the comment at :364); Cargo.toml's
  libm comment (sqrtf by `sqrtss`, sqrt by `sqrtsd`, libm 0.2.16 src/math/arch/x86.rs:13 and :26); SEC-U4D2-3's limit
  beside EXPORT_INTEGERS; TickHash.recording's doc. Proof: text, no test (a rename changes no count); a grep for "two
  exception" in the copy's spikes/ after. Expected: none.
- **FX-12** (RR-L2, EA-5): crates/hash-wasm/clippy.toml (HashMap/HashSet type bans and the std clock bans, lab's form
  without tokio lines; header citing plan.md §4.2's leaf rule). Proof: clippy -D warnings on npt-hash-wasm, host and
  wasm32, both builds, exit 0; plant (fx-12-plant.log): a ban of `std::cell::RefCell` (lib.rs uses it) added to the
  file: clippy on npt-hash-wasm FAILs naming the disallowed type; restored. Expected: none.
- Then: fmt (rustfmt's diff applied by Edit), clippy both builds and wasm32, doc; check.mjs counted twice (the six
  pre-builds first, -x), all 11 steps, counts as deltas on 416 / 465 / 63 of 63: expected measured 417, bites 466 (+1
  each: FX-8's unit test), self 71 of 71 (+8: FX-1 to FX-7 and FX-10, one new test each); the wasm step's split from
  both runs; fx-hashes-end.sha256.
- Stop rule kept: any failure the stop clause covers that is not named above stops that step group and is reported;
  a failure outside its crates in a counted run follows B2.3 (TE2-5); a wasm step mismatch is never re-run into green.

### Notes, one line per FX as it finishes
- Code first, all twelve FX, then the proofs. fx-self1.log (-s): node --test "# tests 71 / # pass 71 / # fail 0" at
  the first run (no test failed, existing or new). fx-rust1.log (-s -k): fmt check exit 1 (two rustfmt hunks in my new
  hash.rs test code; one applied by Edit as rustfmt gave it, the other by shortening the new test's name), npt-lab
  tests "19 passed; 0 failed; 1 ignored" (bin) and lib 5 in both builds, clippy -D warnings host (npt-lab,
  npt-hash-wasm, all targets, both builds) and wasm32 (npt-hash-wasm, both builds) exit 0; fx-rust2.log: fmt clean,
  the new test and h6's by name ok in both builds. fx-loader-probe.log (-x): the new loader on the step folder's two
  modules (48b5ec5f, d196dd19): one 132-byte target_features section each, imports [], exportProblems [].
- Fixed forms (the restore reference): check.mjs 343c59c5, check.test.mjs 8db13620, loader.mjs f2f2293f,
  hash-wasm/clippy.toml 583a796a, hash-wasm/src/lib.rs 9d339055 (unchanged), hash.rs 5677c39c, lab/src/lib.rs
  a3888046, Cargo.toml d211ca52, Cargo.lock 6e628ab7 (unchanged). New exports: check.mjs `buildInputProblems` and
  `FAILED_STEP_DIR` (FX-10's place, which its self-test pins), both imported by check.test.mjs; loader.mjs none.
- **FX-1 done.** Live, environment only (fx-1-env.log, -x -k, exit 1 by design): `node check.mjs --only deps` PASS
  ("given no rustflags"; the held line's new clause); r1-rr-superset.log's first environment
  (CARGO_ENCODED_RUSTFLAGS=-Ctarget-cpu=bleeding-edge RUSTFLAGS=-Ctarget-feature=-simd128,-relaxed-simd) FAIL:
  "CARGO_ENCODED_RUSTFLAGS in the environment holds \"-Ctarget-cpu=bleeding-edge\": no rustflags source may hold a
  flag ..." and "RUSTFLAGS in the environment holds \"-Ctarget-feature=-simd128,-relaxed-simd\": ..."; 
  CARGO_BUILD_TARGET=x86_64-unknown-linux-gnu FAIL ("sets the build target"); `RUSTFLAGS=--cfg feature="bites"` FAIL.
  Plant (fx-1-plant.log, -x -k): check.mjs's `if (!flags.length) continue;` -> `if (true || !flags.length) continue;`
  (sha256 2ae82fe8): "not ok 63 - no rustflags source may hold a flag ... (U4d2 round 1, FX-1)" and "not ok 38 - the
  deps step is depsStep, and it calls every check ..." ("# tests 71 / # pass 69 / # fail 2"), and RR-M2's pair PASSed
  again. Restored by Edit: 343c59c5.
- **FX-2 done.** Self-test (new, in fx-self1's 71): "the loader reads the module's target_features section ...
  (U4d2 round 1, FX-2)". Live plant (fx-2-tf.log, -x -k, exit 1 by design), Block 2's form, in hash-wasm's lib.rs
  (sha256 289ef83a): before dump_word's doc, `#[cfg(target_arch = "wasm32")] #[target_feature(enable = "simd128")] fn
  planted_simd(x: u32) -> u32 { use core::arch::wasm32::{u32x4_add, u32x4_extract_lane, u32x4_splat};
  u32x4_extract_lane::<1>(u32x4_add(u32x4_splat(x), u32x4_splat(x))) }`, and in dump_word's body `#[cfg(target_arch =
  "wasm32")] let i = planted_simd(i);` (a safe call, no `unsafe`). `cargo build --locked --release --target
  wasm32-unknown-unknown -p npt-hash-wasm`: exit 0 (Compiling npt-hash-wasm, the copy's path); `node check.mjs --only
  deps`: PASS; `node check.mjs --only wasm`: FAIL 26.4 s, 15 problems, among them "Wasm leg 1: loader: <copy>/.gate/
  netcode-proof-test/wasm-step/bin/npt-hash-wasm.wasm: refused: the module's target_features section lists +simd128,
  which plan.md §4.5 leaves off (simd128 and relaxed-simd)" for each Wasm leg, h2's four, and the bites module's
  hash-map and std-sin legs; native 1 against 2 still 27000/0. Restored by Edit: 9d339055. The two clean modules'
  acceptance: fx-loader-probe.log, then the counted runs. FX-10 seen live here too: fx-1-env's failed deps run wrote
  failed-steps/deps.log, this call's passing deps run removed it, and the failed wasm run wrote failed-steps/wasm.log.
- **FX-3 done.** Self-test (new): "npt-hash-wasm: an `unsafe` block inside a listed export's body is refused at its
  line ... (U4d2 round 1, FX-3: RR-M1, SEC-U4D2-2, TE-M2)" (every listed export, two forms each, and three deeper
  ones); the C2R-5 fixture's plant now also puts `unsafe {}` in begin's body (its new-fn case kept; strayAt moved by one
  line, re-computed in the same edit as planned). TP4 live (fx-3-tp4.log, -x -k, exit 1 by design): lib.rs's begin
  gains `let bite = unsafe { core::ptr::read_volatile(&bite) };` as its first line (sha256 afc08ca7): `cargo build
  --locked --release --target wasm32-unknown-unknown -p npt-hash-wasm` exit 0; `cargo clippy --locked --target
  wasm32-unknown-unknown -p npt-hash-wasm -- -D warnings` exit 0; `node check.mjs --only deps` FAIL
  "crates/hash-wasm/src/lib.rs:101 has `unsafe` outside an attribute; npt-hash-wasm's only unsafe is the
  #[unsafe(no_mangle)] of its listed exports (...)". Restored by Edit: 9d339055. TP10 (fx-3-tp10.log, -x -k): in
  check.mjs's hashWasmProblems, after `if (allowed.has(m.index)) continue;`, a line skipping any `unsafe` between a
  `pub extern "C" fn`'s `{` and the next "\n}\n" (sha256 95734421): "not ok 54 - npt-hash-wasm: an `unsafe` block
  inside a listed export's body is refused at its line ..." and "not ok 38 - the deps step is depsStep ..." ("# tests
  71 / # pass 69 / # fail 2"). Restored by Edit: 343c59c5.
- **FX-4 done.** loader.mjs: moduleProblems refuses each `WebAssembly.Module.imports` entry ("the module imports
  \"<module>.<name>\" (a <kind>): the loader gives a module nothing to import"); instantiate passes
  `Object.create(null)`; the header and instantiate's doc say so. Self-test (new): "the loader refuses every import by
  module and name before instantiating, Object.prototype's names included (U4d2 round 1, FX-4: SEC-U4D2-1)"
  (constructor.constructor, __proto__.valueOf, constructor.keys, env.now, and two at once). The re-pin named in the
  plan: the loader's list test's instantiate assert now expects that refusal (edited in the same step; no unnamed
  failure). Plant (fx-4-plant.log, -x -k): the loop's source `WebAssembly.Module.imports(module)` ->
  `WebAssembly.Module.imports(module).filter(() => false)` (sha256 82b42447): "not ok 58 - the loader refuses every
  import ..." and "not ok 56 - the loader's list is the seven exports ..." ("# tests 71 / # pass 69 / # fail 2"). The
  same call ran the security seat's sec-probe-imports.mjs on the copy: all four modules refused, but by FX-2's "0
  target_features sections" (the seat's modules carry none), so that probe no longer isolates the import rule; the
  self-tests' modules carry a clean section and do. Restored by Edit: f2f2293f.
- **FX-5 done.** Self-test (new): "the loader refuses a listed name exported as another kind: begin as a global,
  memory as a function (U4d2 round 1, FX-5: SEC-U4D2-8, TE-L1)" (wasmModule's new `kinds`). TP11 (fx-5-tp11.log, -x
  -k): `else if (LOADER_EXPORTS[name] !== kind)` -> `else if (false && LOADER_EXPORTS[name] !== kind)` (sha256
  fc1877fa): "not ok 59 - the loader refuses a listed name exported as another kind ..." ("# tests 71 / # pass 70 /
  # fail 1"). Restored by Edit: f2f2293f.
- **FX-6 done.** check.mjs: in EXCEPTION_LEVELS' deny crates (npt-tcpinfo, npt-hash-wasm) unsafeProblems refuses a
  build.rs at the crate root, a `build` key in the manifest and `include` named other than as an include!,
  include_str! or include_bytes! call; layoutProblems refuses a custom-build target there; pathProblems' and
  unsafeProblems' docs and DEPS_CHECKED name it. Self-test (new): "npt-tcpinfo and npt-hash-wasm, the deny-level
  exceptions, have no build script and name `include` only as a macro call: V4's two forms are refused by name (U4d2
  round 1, FX-6: SEC-U4D2-4)" (V4's build.rs in each crate; `use std::include as inc;` + `inc!(...)` in begin's body;
  `use core::r#include as i;` in tcpinfo; both manifest forms; both metadata targets; the macro calls by name, another
  crate's `use std::include` and npt-bots' build script pass; the real crates pass). Plant (fx-6-plant.log, -x -k):
  `if (EXCEPTION_LEVELS[crate] === "deny") {` -> `if (false && EXCEPTION_LEVELS[crate] === "deny") {` (sha256
  a4a5c527): "not ok 55 - npt-tcpinfo and npt-hash-wasm, the deny-level exceptions, ..." ("# tests 71 / # pass 70 /
  # fail 1"). Restored by Edit: 343c59c5.
- **FX-7 done.** check.mjs: run() takes a `timeout` (ms) for spawnSync and returns `timedOut` (ETIMEDOUT); wasmStep's
  `call` gives every non-cargo call WASM_CALL_LIMIT_MS (300,000; not exported) and the cargo builds none; a timeout
  adds "<label>: still running after 300 s, so it was stopped (the wasm step's limit on each call but its four cargo
  builds)". wasmWorld's runner records `limit` and passes `timedOut` (fixture; no existing assert changed). Self-test
  (new): "the wasm step gives each call but its four cargo builds 300 s, and a call stopped by it fails the step naming
  the call and the limit (U4d2 round 1, FX-7: INF-4)". Plant (fx-7-plant.log, -x -k): `if (r.timedOut)` -> `if (false
  && r.timedOut)` (sha256 d3d42f95): "not ok 69 - the wasm step gives each call but its four cargo builds 300 s ..."
  ("# tests 71 / # pass 70 / # fail 1"). Restored by Edit: 343c59c5. Node v22.23.4's spawnSync on a timeout, checked
  directly (`sleep 5` with timeout 1000): status null, signal SIGTERM, error.code ETIMEDOUT, 1003 ms: what run() reads.
- **FX-8 done.** hash.rs: private `digest_text(own_len, at, words)` ("layouts differ (<n> and <m> words)" when a dump's
  length is not this build's own) used by `--digest-field`; private `arm_for(arm, on, e)` used by serve and by
  compare, which now takes `on` from bitten_name over the recordings its lists hold (one default, EA-9); replay_to no
  longer filters (its callers pass arm_for's). Module doc updated. Unit test (new, both builds):
  `hash::tests::a_digest_field_of_a_dump_of_another_length_says_the_layouts_differ`; h6's bites test gains the
  default's asserts (first listed armed, the second not; `--on`'s armed; an `--on` not listed refused). Plant
  (fx-8-plant.log, -x -k, exit 101 by design): `if words.len() != own_len {` -> `if false && words.len() != own_len {`
  (sha256 511f6af8): "test hash::tests::a_digest_field_of_a_dump_of_another_length_says_the_layouts_differ ... FAILED"
  (left "0000000700000006", right "layouts differ (10 and 12 words)"), "test result: FAILED. 18 passed; 1 failed; 1
  ignored". Restored by Edit: 5677c39c.
- **FX-9 done.** check.mjs: the std-sin miss's problem line ends "; glibc picks its sinf by the CPU's features when
  it loads (an IFUNC), so on another CPU or libc the scan is re-run there (u4d2-brief.md 2.4 (a) item 6; INF-3)".
  Self-test: one assert added to the existing bites test ("the wasm step judges PM-0004-21's two bites ..."), the
  whole tail from the libc part to the sentence, within main's 40 lines. Plant (fx-9-plant.log, -x -k): the two
  sentence strings replaced by "" (sha256 24609228): "not ok 70 - the wasm step judges PM-0004-21's two bites ..."
  ("# tests 71 / # pass 70 / # fail 1"). Restored by Edit: 343c59c5. The scan file's format is unchanged (the card's
  CPU-record follow-up is not mine).
- **FX-10 done.** check.mjs: `FAILED_STEP_DIR` = <checkout>/.gate/netcode-proof-test/failed-steps (exported, pinned by
  its self-test); main(argv, steps, log, keep): before each step its `<step>.log` is removed; a failed step's whole
  output is written there and a line "      the step's whole output (<n> lines): <path>" follows its 40 lines; a write
  that fails is said, not thrown. Self-test (new): "a failed step's whole output is kept in its file under the
  checkout's .gate/, named in its FAIL lines, and removed when the step next runs (U4d2 round 1, FX-10: TE-L4)".
  Plant (fx-10-plant.log, -x -k): `writeFileSync(file, ...)` -> `if (false) writeFileSync(file, ...)` (sha256
  ae91bd34): "not ok 66 - a failed step's whole output is kept ..." ("# tests 71 / # pass 70 / # fail 1"). Restored
  by Edit: 343c59c5. Live: fx-1-env.log's FAIL lines name failed-steps/deps.log; fx-2-tf.log's wasm FAIL wrote
  failed-steps/wasm.log (20 lines, 4,994 B) and its passing deps run removed the older deps.log.
- **FX-11 done (texts; no test, no plant: a rename changes no count).** check.test.mjs: the tests "unsafe_code: an
  attribute relaxing it outside the three exceptions fails, in every form" and "unsafe code outside the three exception
  crates fails however the lint was relaxed (review S1)" renamed, the comment "The three exceptions' manifests set their
  own level"; a grep of the copy's spikes/ files in my lane for "two exception", "the two exceptions" and "two unsafe
  exception" finds nothing (exit 1). Cargo.toml's libm comment: "sqrtf by x86-64's `sqrtss` and sqrt by its `sqrtsd`"
  with "arch/x86.rs:13 and :26" cited (read in the registry: libm-0.2.16/src/math/arch/x86.rs:13 "sqrtss {x}, {x}",
  :26 "sqrtsd {x}, {x}"); carry 29's own text is not touched. check.mjs's EXPORT_INTEGERS doc states SEC-U4D2-3's
  limit (`type u32 = f64;`, `use core::primitive::f64 as u32;`; export types not read; Q7 (b) and C-U7-2).
  crates/lab/src/lib.rs: TickHash.recording's doc "The recording both legs replay (the same name on both legs), as its
  manifest names it." (the struct's own line, "the recording (or run) it replays", left as it is: one doc line).
- **FX-12 done.** crates/hash-wasm/clippy.toml (new, 583a796a): the HashMap and HashSet type bans and the std clock
  bans (types and `now` methods), lab's form without tokio lines, its header citing plan.md §4.2's leaf rule. clippy -D
  warnings host both builds and wasm32 both builds exit 0 (fx-rust1.log). Plant (fx-12-plant.log, -x -k, exit 101 by
  design): `{ path = "std::cell::RefCell", ... }` added to its disallowed-types (sha256 33635feb): `cargo clippy
  --locked -p npt-hash-wasm -- -D warnings` and the wasm32 bites build's "error: use of a disallowed type
  `std::cell::RefCell` --> crates/hash-wasm/src/lib.rs:45:1" (and :65:19, :66:9), exit 101 each: clippy reads the file
  for this crate. Restored by Edit: 583a796a.
- Every plant restored and checked: check.mjs 343c59c5, loader.mjs f2f2293f, lib.rs 9d339055, hash.rs 5677c39c,
  clippy.toml 583a796a. No crates/sim, Cargo.lock, manifest, recordings.json, std-sin-scan.txt or
  co-initial-dump.txt change.

### End checks
- Untouched, checked: `git diff --stat -- spikes/netcode-proof/crates/sim` empty; Cargo.lock 6e628ab7 (round 1's: its
  diff against HEAD is round 1's 7 lines); recordings.json 9979c409, std-sin-scan.txt 905ae057, co-initial-dump.txt
  76326a64, hash-wasm/Cargo.toml d3aef820 (each the test seat's step-1 value).
- fx-end-lint.log (-x, exit 0; df 6.6G): `node check.mjs --only fmt,clippy,clippy-wasm32,doc`: fmt PASS 0.6 s
  (rustfmt's two hunks were applied by Edit earlier: fx-rust1/2); clippy PASS 8.8 s; clippy-wasm32 PASS 1.4 s; doc
  PASS 68.3 s; "check: all 4 steps pass".
- **Counted run 1** (fx-check1.log, -x, exit 0; df 6.6G, shared target 7.6G before): the six pre-builds, each exit 0,
  then `node check.mjs`: "check: all 11 steps pass". fmt 0.6 s; clippy 3.7 s; clippy-wasm32 1.0 s; doc 69.4 s; test
  257.1 s "measured build: 417 tests passed, 0 failed, 0 reported a skip; bites build: 466 tests passed, 0 failed, 0
  reported a skip"; deps 1.5 s (read-back "given no rustflags": x86-64 fxsr, sse, sse2; wasm32 bulk-memory,
  multivalue, mutable-globals, nontrapping-fptoint, reference-types, sign-ext; npt-sim's features one set, measured and
  bites as round 1); wasm 41.8 s; refs 0.0 s (793 tracked files); marks 3.4 s (81 marks); self 1.9 s "71 of 71 tests
  passed"; redact 0.8 s (10 of 10; 247 items, 0 findings). Provenance: 4 x "Compiling npt-hash-wasm v0.0.0
  (<copy>/spikes/netcode-proof/crates/hash-wasm)", 4 x npt-lab's, 6 x npt-sim's.
- Run 1's wasm line: native 1 vs 2 27000/0 and native vs Wasm 27000/0 (SS-s1, ES-s2, TC-s3 9000 each); h2 equal;
  hash-map native 1-2 caught at SS-s1 1234 bite[0].hash_map_order (22269414978ac615, cbc363f6bf11cae5), native-Wasm
  caught (22269414978ac615, f5e8d0e818b168c5), Wasm 1-2 not exercised (f5e8d0e818b168c5 both): not catchable on
  wasm32; std-sin caught at SS-s1 tick 2, vehicle[8].pos.y (the scan's other catch ES-s2 1000002). Split (s): builds
  6.4, 6.4, 1.5, 1.5; npt-lab hash 4.3; Wasm SS-s1 3.3, ES-s2 1.0, TC-s3 0.1; compare 0.0; h2 0.1, 0.0, 0.0, 0.0,
  vectors 0.0; bites 0.0; hash-map native legs 3.0, Wasm 1 3.3, Wasm 2 3.3, compares 0.6, 0.3; std-sin legs 3.0, Wasm
  3.3, compare 0.1.
- **Counted run 2** (fx-check2.log, -x, exit 0; df 7.2G before): the six pre-builds, each exit 0, then `node
  check.mjs`: "check: all 11 steps pass". fmt 0.6 s; clippy 4.0 s; clippy-wasm32 0.9 s; doc 76.7 s; test 258.3 s
  "measured build: 417 tests passed, 0 failed, 0 reported a skip; bites build: 466 tests passed, 0 failed, 0 reported a
  skip"; deps 1.4 s (the same read-back, "given no rustflags", and features); wasm 40.5 s; refs 0.0 s; marks 0.8 s; self
  1.7 s "71 of 71 tests passed"; redact 0.7 s. Provenance: 4 x npt-hash-wasm's, 4 x npt-lab's, 6 x npt-sim's Compiling
  lines (the copy's path). Its wasm line: 27000/0 both pairs; h2 equal; hash-map native 1-2 caught (04a8a7c3df34e895,
  9e77033f5be90b25), native-Wasm caught (04a8a7c3df34e895, f5e8d0e818b168c5), Wasm 1-2 not exercised
  (f5e8d0e818b168c5 both): not catchable on wasm32; std-sin caught at SS-s1 tick 2, vehicle[8].pos.y. Split (s): builds
  5.8, 6.5, 1.5, 1.5; npt-lab hash 4.0; Wasm 3.3, 1.1, 0.1; compare 0.0; h2 0.1, 0.0, 0.0, 0.0, vectors 0.0; bites
  0.0; hash-map legs 2.9, Wasm 3.2, 3.4, compares 0.6, 0.3; std-sin legs 2.8, Wasm 3.2, compare 0.1. Shared target
  7.0G after (7.6G before run 1: it shrank by a hand not mine; this builder cleans nothing); df 7.2G.
- No failure in either counted run, so no TE2-5 entry (npt-net's TE-M3 test passed in both bites builds), and no
  wasm step mismatch at any time; the only failing runs were the plants' and the environments' own, each named above,
  and fx-rust1's `cargo fmt --all --check` (exit 1) on my new hash.rs test code, whose two hunks I applied by Edit
  (round 1's b8-rust1 had the same; the ruling's Block 6: within the write rule).
- **Counts as deltas on 416 / 465 / 63 of 63 (round 1's), 11 steps:** measured 417 (+1), bites 466 (+1): npt-lab's
  `hash::tests::a_digest_field_of_a_dump_of_another_length_says_the_layouts_differ` (FX-8), in each build; self 71 of 71
  (+8), the new tests: (1) "no rustflags source may hold a flag and no build target may be set, so the read-back is
  given what cargo gives rustc; RR-M2's negation pair fails naming both sources (U4d2 round 1, FX-1)"; (2) "the loader
  reads the module's target_features section: simd128 or relaxed-simd used or required is refused, and so is a module
  with none, two or one that does not parse; the built modules' list passes (U4d2 round 1, FX-2)"; (3) "npt-hash-wasm:
  an `unsafe` block inside a listed export's body is refused at its line, where that export's own allow lets rustc
  compile it (U4d2 round 1, FX-3: RR-M1, SEC-U4D2-2, TE-M2)"; (4) "the loader refuses every import by module and name
  before instantiating, Object.prototype's names included (U4d2 round 1, FX-4: SEC-U4D2-1)"; (5) "the loader refuses a
  listed name exported as another kind: begin as a global, memory as a function (U4d2 round 1, FX-5: SEC-U4D2-8,
  TE-L1)"; (6) "npt-tcpinfo and npt-hash-wasm, the deny-level exceptions, have no build script and name `include` only
  as a macro call: V4's two forms are refused by name (U4d2 round 1, FX-6: SEC-U4D2-4)"; (7) "the wasm step gives each
  call but its four cargo builds 300 s, and a call stopped by it fails the step naming the call and the limit (U4d2
  round 1, FX-7: INF-4)"; (8) "a failed step's whole output is kept in its file under the checkout's .gate/, named in
  its FAIL lines, and removed when the step next runs (U4d2 round 1, FX-10: TE-L4)". Steps 11 (unchanged). On <base>
  399 / 446 / 52 / 10: 399 + 17 + 1 = 417; 446 + 19 + 1 = 466; 52 + 11 + 8 = 71; 10 + 1 = 11.
- Existing tests changed, no count change: the C2R-5 deps test (buildInputProblems' line; FX-3's body plant and its
  strayAt), the loader list test (FX-4's re-pin; wasmModule's new options and default target_features section), the
  bites test (FX-9's assert), the two renamed (FX-11); wasmWorld records each call's limit (FX-7). Rust: h6's bites test
  (EA-9's asserts).
- **The end hashes:** /home/user/Shamble-Frontlines/.gate/u4d2/fx-hashes-end.sha256 (saved with Write from
  `sha256sum` of the 18 paths `git status --short --untracked-files=all -- spikes` lists, from the copy's root): round
  1's 17 files plus
  crates/hash-wasm/clippy.toml; changed in this pass: check.mjs 343c59c5, check.test.mjs 8db13620, loader.mjs f2f2293f,
  hash-wasm/clippy.toml 583a796a (new), hash.rs 5677c39c, lab/src/lib.rs a3888046, Cargo.toml d211ca52; the other 11
  equal round 1's (the test seat's step-1 list). `sha256sum -c` from the copy's root: 18 OK, exit 0. `git status
  --untracked-files=all`: 19 lines (the 18 and this record); HEAD 51520b4; nothing committed.
- For round 2 (notes, not changes): the security seat's sec-probe-imports.mjs modules carry no target_features section,
  so FX-2's rule refuses them before FX-4's is reached (fx-4-plant.log); a module with a clean section (check.test.mjs's
  wasmModule) isolates FX-4. One plan line said `buildInputProblems` would be the only new export; `FAILED_STEP_DIR`
  is a second, imported by FX-10's self-test to pin the file's place (named above). Nothing stopped: no item needed a
  file outside the bounds, and every new test failed by name under its plant.
- Done. Next: the main session (the snapshot by worktree_patch.py, the check against fx-hashes-end.sha256, round 2).

## Merge into the spike branch (the main session, 2026-11-13; advisor-u4d2-r1.md Block 9 as advisor-u4d2-r2.md Block 5 amends it)
- **Round 2 and its ruling:** all five seats APPROVE WITH FINDINGS (one Medium, INF2-1, and Lows, none before the
  merge); the advisor's round-2 ruling (advisor-u4d2-r2.md, sha256 e776f0ba…): the merge APPROVE on M1 to M9 as
  amended; INF2-1 carried (entry 84), the security Lows carried (entry 85); T2, T3 and T5 amended.
- **M1 met** (09:06 UTC): the copy's HEAD 51520b43391a453a3e37ce235b9f5753df55ab93; `git status
  --untracked-files=all` 19 lines (the 18 files under spikes/netcode-proof and this record); `sha256sum -c
  .gate/u4d2/r2-hashes-start.sha256` from the copy's root 18 of 18 OK; that file equals fx-hashes-end.sha256 (cmp).
- **M2 met:** at 99c7ed9, `git diff --quiet 51520b4 HEAD --` the lane paths (check.mjs, check.test.mjs, Cargo.toml,
  Cargo.lock, crates/hash-wasm, crates/lab, crates/proto/Cargo.toml, crates/client-wasm/Cargo.toml) and crates/sim:
  true; spikes/'s commits since 51520b4: c88eb31 (U4e's merge) and 1b4a3c8 (A13) only; A13's CI green (gate runs
  43330155579 push and 43330163225 pull_request); spikes/ clean in the main folder.
- **The texts' script:** u4d2_merge_text.py (written by an agent, wf_c228865c-3b4; sha256 af8e955e…; committed at
  3b7f0a6); both dry runs exit 0, every assert passed; phase A's diff read in full by the main session, phase B's T1
  and T3 rows read, T2 and T5 checked byte for byte by the script against their code blocks.
- **M3 met** (the saver stopped first, about 09:15 UTC): worktree_patch.py's new cut of the copy
  (.gate/u4d2-merge/u4d2-merge-cut.patch; no notes copied) `cmp`-equal to cb27931's wip/u4d2.patch (sha256
  9e71cda07d4a…, unchanged since cb27931: `git diff --quiet cb27931 --`); `git apply --check`, then `git apply`, in
  the main folder.
- **M4 met:** `sha256sum -c .gate/u4d2/r2-hashes-start.sha256` from the main folder's root: 18 of 18 OK.
- **M5 met:** phase A applied (T4; spikes/netcode-proof/marks/SERIES.md f118a8de… → 3a967369…); the machine's uptime
  3,140 s at the call's start (.gate/u4d2-merge/uptime-m5.txt); one main-run.sh call (.gate/u4d2-merge/m5-counted.log,
  09:16 to 09:25 UTC): the six pre-builds exit 0, their Compiling npt-hash-wasm (5) and npt-lab (6) lines naming the
  main folder's crates; "check: all 11 steps pass" (:175): measured 491 = 473 + 17 + 1, bites 561 = 541 + 19 + 1, 0
  failed, 0 skips (356.6 s); self 71 of 71 = 52 + 11 + 8; marks 81, the ADR rows unchanged; redact 264 items, 0
  findings; the deps line with the fix pass's two clauses, the read-back "given no rustflags", npt-sim's features one
  set across the npt-server, npt-lab and npt-hash-wasm builds; the wasm line (:170) 33.1 s, 27000 ticks and 0
  mismatching for both pairs, h2 equal, the hash-map bite caught natively and native-against-Wasm at SS-s1 tick 1234
  (Wasm 1 against 2 not exercised), std-sin caught at SS-s1 tick 2 `vehicle[8].pos.y`, no call stopped by the 300 s
  limit (INF2-1's interim rule not triggered; no npt-lab or loader process to list); h7 (ii) by name on release
  (:177-204): "test result: ok. 1 passed", P2 913db8e914bdb773…, B1 feabf2f02d023fa9…, P4 b8a04f2f8ddc0d39…, each
  equal to U2's full value (the test's assertion).
- **M6 met:** no failure in any run; nothing under TE2-5; the copy's failed-steps/ folder empty.
- **M7 met:** `node tools/gate.mjs`: every check PASS but smoke (5a), which could not start this container's Chromium
  (tools/'s Playwright expects /opt/pw-browsers/chromium_headless_shell-1194, not installed: gate-full.log); `node
  tools/gate.mjs --no-smoke`: GATE PASS (gate-nosmoke.log); both logs under .gate/u4d2-merge/. CI's gate and smoke run
  on the pushed head.
- **M8:** phase B applied (T1: u4-build-plan.md §4.1's npt-lab and npt-hash-wasm rows; T2: u4s-carries.md entries 82
  to 85 after entry 77; T3: plan.md's U7 row, C-U7-2; T5: u4d2-report.md's rulings section at its anchor; each read
  back byte for byte); T5's two places filled by u4d2_t5_fill.py from m5-counted.log:170, :177, :199-201 and :204 and
  uptime-m5.txt:3; u4d2-report.md written (its other sections in u4e-report.md's form); `git rm` wip/u4d2.patch; one
  commit. M9 after the push.
