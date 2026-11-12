# The advisor's ruling on U4d2's round 1 (2026-11-12, from advisor-u4d2-r1-brief.md)

Seat: the advisor agent, run by the main session's workflow, working from main's `.claude/agents/advisor.md`
(read at origin/main). It rules; it edits no code and runs nothing that builds.

## Plan
1. Read the five seats' notes whole (rust, security, infra, arch, test), then the builder's record (claims only).
2. Read my own rulings that bind the form: advisor-u4d2-brief.md (L1 to L5, the conditions), advisor-u4e-r1.md and
   advisor-u4e-r2.md (M1 to M9's form).
3. Read the rules cited by place: CLAUDE.md's review budget (in context), u4-build-plan.md §7.1 items 1 and 7,
   u4s-carries.md entry 52 (the 9.5 floor), §5 U4d2, B2.3 / TE2-5 in u4d2-brief.md, carry 65.
4. Spot-check the load-bearing claims against the copy (`.gate/u4d2-r1`, the worktree's diff) and the seats' logs
   (`.gate/u4d2/r1-*`): RR-M2's run, SEC-U4D2-1's run, SEC-U4D2-7's run, TE-M3's logs, the counts.
5. Rule, one block per item (1 to 9 of the brief), each written to this file as it is finished.
6. End with the two lists (before the merge; for Mustafa) and the summary for the caller.

## Notes (one line per check, as done)
- Read whole: advisor.md (origin/main, 334 lines); the five seats' notes (rust 124, security 172, infra 150, arch 112,
  test 266 lines); the builder's record u4d2-build.md (256 lines, its claims).
- Read whole: my brief check advisor-u4d2-brief.md (756 lines: A1-A15, Q1-Q12, L1-L5; Q4 (a) chose the superset,
  "fail-closed"; A15 found the superset's first cost; block 7 set U4d2's review in batches).
- Read the form: advisor-u4e-r1.md R8 (fix pass, its builder, round 2's seats, the outcome rule) and R9 (M1-M9);
  advisor-u4e-r2.md Q3 (M1-M9 as amended), Q4 (background run, >= 1,500,000 ms), "Before the merge".
- The rules by place: u4-build-plan.md (now 52dd3186…) §5 U4d2 :666-712, Lanes :850-888, §6 :890-907, §7.1 :909-921;
  Mustafa's words behind §7.1 (history.md:411-412, 2026-11-09: "point all copies at one shared Cargo target folder
  (CARGO_TARGET_DIR), so each unit doesn't leave a 4–6 GB cache behind"); W1-TARGET item 3 (advisor-u4-wave1.md:576-580:
  "Between waves, with no lane running, the main session may empty the shared folder by `cargo clean --target-dir
  .gate/shared-target` ... §7.1 item 7 ... binds the copies, not the main session"); B2.3 (advisor-u4j-r2.md:146-171).
- TE-M3's test read: delay_tests.rs:2551-2603, `#[tokio::test(flavor = "multi_thread", worker_threads = 2)]`, counted
  asserts (`polls_after_deadline == 1`, `polls <= 3`, `s_calls <= polls`) after a 300 ms wait; U3c's failure
  (u3c-fix2-c2b.md:74-76: "panicked at delay_tests.rs:2002 ... once, under C1b's parallel load"). Which assert fails is
  unknown both times (no panic text kept).
- The copy, checked now: `.claude/worktrees/u4d2-build` HEAD 51520b43391a…; `git status` 9 M + 8 new + the record; the
  17 files' sha256 equal the test seat's step-1 list (first 16 hex each compared). wip/u4d2.patch a526ef8e… (5,366
  lines); the packet 9b670276… (as the card names it); the brief 146acf18… (as built from).
- Seats' logs read at the lines the findings cite: r1-rr-superset.log:145-158 (the negation env PASS twice, the control
  FAIL); r1-rr-superset-build.log:3-14 (`--target-dir .gate/u4d2-rr-target`, the module lists +simd128 and
  +relaxed-simd among 17 features); r1-sec-imports.log:5-7 (three Object.prototype imports INSTANTIATED, moduleProblems
  []); r1-sec-tf2.log:6 (+relaxed-simd +simd128 in a module built under deny(unsafe_code)); r1-te-final-check.log:154,
  :193 (the test step's FAIL and carry 65's name line); r1-te-te25-npt-net.log:17, :194 (the test ok, 169 passed).
- The spike branch since 51520b4: `git diff --stat 51520b4 HEAD` over U4d2's lane paths and crates/sim is empty; U4e's
  merge changed only crates/room, crates/server, crates/client and marks/ (README.md, SERIES.md +1 line). A13 is
  uncommitted in the main folder (crates/server/clippy.toml, the room file's two type entries and a header; read), its
  counted run under way (.gate/a13/a13-check-counted.log: fmt, clippy, clippy-wasm32, doc PASS so far). df 6.5 GB free.
- The main session's point 1, re-checked at 51520b4's plan (3f58ff0d): §5 U4d2 666-712, Lanes 850-888, §6's U7 line
  898-900, §7 908-931 (headings at 666/714, 850/890, 898, 908/933): as stated. Also stale in the same item 6 (brief
  :143): "U4r's lines that reuse your work (:766-767, :790-792)", which read :776-777 and :800-802 at 3f58ff0d (same
  text, read). §4.1, §4.11, §4.14, §4.15 and U4d's lines did not move.
- Point 4 re-checked: a grep of the copy's spikes/ finds three "two" texts, all in check.test.mjs: :328 and :364 (as the
  card says) and :529 ("unsafe code outside the two exception crates fails ...", a test name).
- §4.1 at HEAD read (:178-203): npt-lab's row lists only the subcommands; npt-hash-wasm's row "(cdylib; integer-only
  exports)". The loader's exports (copy): HASH_WASM_EXPORTS, LINKER_EXPORTS, LOADER_EXPORTS, BEGIN, STEP,
  moduleProblems, exportProblems, instantiate, forEachChunk, replay, parse, main; check.mjs 65 exports against <base>'s
  42 (+23, EA-7's count). SERIES.md's header-params table at HEAD: `fate_log_sha256` at :397.
- Neither exception crate with a deny level has a build script (crates/tcpinfo, crates/hash-wasm: no build.rs, no
  `build` key), so SEC-U4D2-4's build-script refusal costs nothing lawful.
- The code the forms rest on, read in the read copy: check.mjs rustflagSources (:1451-1468, every env var ending
  RUSTFLAGS and every config `rustflags` key), buildFlagProblems (:1478-1509, a deny-list of fma, simd128,
  relaxed-simd and target-cpu=native), READ_BACK and readBackCheck (:1516-1553: `sources.flatMap` of every flag, its
  doc "a superset of the one source cargo uses, which fails closed"); run() (:1767-1771, spawnSync, no timeout);
  failedRun, failingTests, failingTestsLine (:1815-1846); main's FAIL print (:2423, the last 40 lines); loader.mjs
  moduleProblems, exportProblems, instantiate (:48-87: names and kinds only; `new WebAssembly.Instance(module, {})`).
  plan.md §4.5's build flags (:533-535: "baseline x86-64; no `+fma`, no `target-cpu=native`, no relaxed-simd; simd128
  off ... `overflow-checks = true` for npt-sim and npt-proto in every profile").
- The packet: its heads, its outline (diffs :14-4254; judged against :4255-6130, my brief check whole at :5167-5920,
  the carries :5972-6130). I read the diffs' load-bearing parts in the copies themselves (above), not every packet line.

## Rulings (one block per item of the brief)

### Block 1. Item 1: the four Mediums marked before the merge (done)
**RR-M2 with TE-M1 (the read-back's superset fails open): the finding holds; REVISE, in the fix pass as FX-1 and FX-2.**
- Proven twice by run on the copy (r1-rr-superset.log:145-148, r1-te-tp2-flags.log run 3: deps PASS with
  CARGO_ENCODED_RUSTFLAGS=-Ctarget-cpu=bleeding-edge and RUSTFLAGS=-Ctarget-feature=-simd128,-relaxed-simd; the control
  FAILs), and the module cargo builds in that environment carries +simd128 and +relaxed-simd
  (r1-rr-superset-build.log:14). So condition 7 (a)'s "fails the step ... never only reports" did not hold for that
  case, and the step's limits text (readBackCheck's doc: "a superset of the one source cargo uses, which fails closed")
  is untrue. Severity stays Medium (its reviewers'): it needs two crafted rustflags sources, and no runner, config or
  session here sets any.
- **My error, corrected in writing.** My U4D2-Q4 (a) ruling said the superset "is a superset, fail-closed". A superset
  of flags is not a superset of features: a flag can take a feature away, and cargo drops every source but the first
  non-empty one. The rest of Q4 (a) stands: the read-back runs from spikes/netcode-proof, and nothing re-implements
  cargo's choice of source.
- **The form (FX-1, which is also SEC-U4D2-5's and SEC-U4D2-6's fix):** the deps step refuses every rustflags source
  that holds a flag, whatever the flag: each environment variable rustflagSources reads (every name ending RUSTFLAGS)
  and each `rustflags` key of each cargo config it reads, each refusal naming the source and its value. It also refuses
  a set CARGO_BUILD_TARGET and any `build.target` key in those configs. The spike's builds use no rustflags
  (e1-check.log: "given no rustflags") and pass no target config, so this refuses nothing a lawful build needs. It is
  stricter than plan §5 U4d2's four named flags, never looser: those four stay refused. The read-back stays as the plan
  asks ("read back with `rustc --print cfg` per target ... never only reports"), with its non-vacuity. When the step can
  pass, it is now given exactly what cargo gives rustc (no flags); readBackCheck's doc and DEPS_CHECKED's limits say
  so, and the words "which fails closed" go. No per-source read-back is needed, because no source may hold a flag.
  buildFlagProblems' named-flag messages may stay beside the new rule.
- **Second half (FX-2, which is also SEC-U4D2-7's fix):** the loader's `moduleProblems` reads the module's
  `target_features` custom section (`WebAssembly.Module.customSections(module, "target_features")`). It refuses the
  module when that section lists simd128 or relaxed-simd with the prefix `+` or `=`, when it has no such section or
  more than one, or when the section does not parse, each by name. This reads the artifact itself, so it also catches
  what no source read can: source-level `#[target_feature]` (Block 2) and a flag that reaches rustc by a route check.mjs
  does not read. Both built modules list neither feature today (the rust seat's wasminfo run), so this refuses no
  lawful build.
- **Proof:** see Block 8, FX-1 and FX-2.

**RR-M1 with SEC-U4D2-2 and TE-M2 (one point): the finding holds; REVISE, in the fix pass as FX-3.** TP4
(r1-te-tp4-body-unsafe.log) shows that an `unsafe` read inside `begin`'s body passes rustc, clippy -D warnings and the
unit tests, and is refused only by the deps scan (lib.rs:101). TP10 shows that a scan regression letting it through
leaves 63 of 63 self-tests and the live deps step green. So the scan is that case's only guard, and nothing pins it.
The form is the seats': one self-test case (an `unsafe {}` inside a listed export's body, refused by hashWasmProblems
at its file and line); the deps fixture's stray-unsafe plant also placed in an export's body (beside the new-fn case,
which stays); and TP4's live plant once, with the wasm32 build's exit 0 beside the deps FAIL. Round 2's test seat
re-plants TP10's regression, and the new case must fail by name.

**SEC-U4D2-1 (Object.prototype imports): the finding holds; REVISE, in the fix pass as FX-4.** r1-sec-imports.log:5-7:
a module importing `constructor.constructor` (JS's Function) instantiates with moduleProblems []. The loader's own
words ("a module that imports anything fails", loader.mjs:9-10, :77-78) are false, and the env.now self-test passes
because `env` is not on Object.prototype. The form: `moduleProblems` refuses every entry of
`WebAssembly.Module.imports(module)`, by module and name, before anything is instantiated, and `instantiate` passes
`Object.create(null)`. Self-tests: modules importing `constructor.constructor`, `__proto__.valueOf` and `env.now`, each
refused by name. The loader's header and instantiate's doc then say what is true.

**INF-1 (no cold time was measured): the finding holds; report text before the merge, and the cold figure measured
after the merge without any exception to §7.1. Nothing goes to Mustafa.**
- **Why the brief's "cold" could not be met (my miss as well):** the brief defines cold as "its first run: the release
  builds not yet in the shared target". But the shared target always held other lanes' release dependencies (tokio,
  quinn, ring and rustls were already there at b3-lab1 16:47), and Mustafa's rule (history.md:411-412, "point all
  copies at one shared Cargo target folder") with §7.1 items 1 and 7 leaves a copy no empty target. My brief check did
  not see that.
- **The route that needs no exception:** W1-TARGET item 3 (advisor-u4-wave1.md:576-580): "Between waves, with no lane
  running, the main session may empty the shared folder by `cargo clean --target-dir .gate/shared-target`, an approved
  method ... §7.1 item 7 ... binds the copies, not the main session." Wave 2 ends with U4d2's merge (U4e merged at
  c88eb31). So, after U4d2's merge and before U4r's brief, with no lane running, the main session empties the shared
  target and runs `node check.mjs --only wasm` once through main-run.sh (the lock exclusive, the shared target,
  offline). That run's seconds are the step's cold figure: every dependency is built for both targets, and the
  toolchain and the registry cache stay as they are. It feeds U4r's Evidence decision (plan :800-802: "in check.mjs if
  the step stays within 5 minutes, else through ops/runs/det-u4.mjs"). It is M9's last item. If it exceeds 300 s, that
  comes to me as U4r's det-u4.mjs question, not as a U4d2 defect.
- **The words before the merge (u4d2-report.md, T5):** every figure is labelled "warm for dependencies, the workspace
  rebuilt by the touch loop". The first release builds are given (b3-lab1, npt-lab 16.04 s; b2-hw-test3, npt-hash-wasm
  wasm32 3.57 s), with the infra seat's bound of about 80 s against 300 s. The report says why no cold run was taken
  before the merge, and where the cold figure comes from (M9). The budget verdict stands on the warm figures: 40.0 to
  46.5 s over seven runs.
- **The rust seat's scratch target.** Its `--target-dir .gate/u4d2-rr-target` build (r1-rr-superset-build.log:3,
  38 MB) was a cargo call in the builder's copy off the shared target, against §7.1 item 1's letter, which the infra
  seat saw. Its reason was sound: it kept a SIMD-built module out of the shared target, where a later run could have
  read it. It is recorded as a process slip in T5. The folder goes by `cargo clean --target-dir` (an approved method)
  with the copies at M9. No later seat or builder does it again: each round-2 brief says "the shared target only".

Block 1 written 2026-11-12.

### Block 2. Item 2: SEC-U4D2-7 (source-level target features reach the Wasm leg): this fix pass, FX-2. APPROVE as a finding.
- The facts hold (r1-sec-tf.log, r1-sec-tf2.log:6). Under `#![deny(unsafe_code)]` and with no rustflag, a safe
  `#[target_feature(enable = "relaxed-simd")]` function compiles for wasm32 and emits `f32x4.relaxed_madd`, and the
  linked module's `target_features` section lists +relaxed-simd and +simd128. No scan, ban or read-back sees it. The
  exposure is npt-sim's since U4s, but U4d2's step is the one that says simd128 and relaxed-simd are off, so U4d2 makes
  that true.
- **Route: the fix pass, not a carry.** It is FX-2's own check: the same reading of the section, failing closed when
  the section is missing, so it costs nothing beyond RR-M2's second half. A carry beside C-U7-1 would leave the step
  claiming a property it does not check until U7.
- **Not taken:** the optional source scan (refusing `target_feature` attributes and `core::arch` or `std::arch` paths in
  the crates the Wasm leg compiles). Reading the artifact is the decisive check for the Wasm leg. On x86-64, calling a
  `#[target_feature]` function from code built without that feature needs `unsafe` (Rust's target_feature 1.1 rules,
  my reading), which npt-sim forbids and the scan refuses in npt-hash-wasm, so the native legs are not exposed this way.
- **Its live plant (FX-2's proof):** a safe `#[target_feature(enable = "simd128")]` function (or relaxed_madd, as the
  security seat's probe used), reached from a listed export in crates/hash-wasm/src/lib.rs by Edit. The wasm32 build
  exits 0, the deps step passes, and the loader refuses the module naming the feature, so the wasm step FAILs. Restore
  it and check the sha256.
- C-U7-2 (T3) adds that U7's check of wasm-opt's output runs this same refusal, so U7 must keep the section or say how
  it checks the features.

Block 2 written 2026-11-12.

### Block 3. Item 3: the Lows that touch a build's integrity or a verdict (done; labels stand; each rides the push the Mediums need anyway)
- **SEC-U4D2-5 (`--cfg feature="bites"`, `-C overflow-checks=off`, `-C llvm-args` pass the name rule): fix pass, FX-1.**
  The rule "every rustflags source that holds a flag is refused" is the seat's own fix. It closes all three, and also
  the plan.md §4.5 rule a rustflag could override ("`overflow-checks = true` for npt-sim and npt-proto in every
  profile"). Self-tests: each of the three in RUSTFLAGS, and a config file's `rustflags`, each FAILs by name.
- **SEC-U4D2-6 (CARGO_BUILD_TARGET or `build.target` builds npt-lab for another target): fix pass, FX-1,** in the
  cheaper of the two forms. The deps step refuses either, read as it reads the rustflags sources. Adding
  `--target x86_64-unknown-linux-gnu` to the step's native builds would move their artifacts, the six pre-builds'
  provenance and the shared target's layout; that is not needed when the input is refused. Self-tests: one of each.
- **INF-3 (the std-sin catch depends on the CPU through glibc's IFUNC `sinf`): the wording in the fix pass (FX-9), the
  data's CPU record a follow-up.** It never gives a false pass, only a false FAIL whose message names the libc alone.
  The step's std-sin failure line adds that glibc picks `sinf` by the CPU's features when it loads, so on another CPU or
  libc the scan is re-run there (u4d2-brief.md 2.4 (a) item 6). The scan file's format does not change now. Follow-up
  (card): when the scan is next re-run (a toolchain, glibc or machine change), its record names the CPU model and its
  fma and avx2 flags.
- **INF-4 (no child call of the wasm step has a timeout; a hang holds the exclusive lane lock): fix pass, FX-7.** Every
  child call of the step except its four cargo builds runs with a timeout of 300 s: the loader legs, the `npt-lab hash`
  legs and compares, vectors and bites. That is the step's whole budget, and each call takes under 5 s today
  (b8-wasm1.log's split). A timeout is a FAIL naming the call and the limit. The cargo builds are left without one: under
  the exclusive lock no other cargo can hold theirs, and a cold build's time is INF-1's to measure, not to cut. Self-test:
  a fake io whose call reports a timeout, and the FAIL line names it.
- **RR-L3 (`--digest-field` reads every dump at the native layout's indexes): fix pass, FX-8.** When a dump's length
  is not this build's own, `hash compare` prints "layouts differ (<n> and <m> words)" in place of a digest. One unit
  test (two dumps of different lengths). This answers the builder's open choice: P6's word-size plant, the one bite whose
  output this line exists to read, now reads true. EA-9 rides the same edit (Block 5).

Block 3 written 2026-11-12.

### Block 4. Item 4: the other Lows, each one's route (done; labels stand; none pushes on its own)
The rule I apply: a Low rides the fix pass when it is cheap, inside U4d2's lane, and in a file the pass edits anyway, or
when it is the exception's own missing test; a Low whose natural home is a later unit's change goes to that unit as a
carry, with its words in T2 or T3; the rest are card follow-ups the report names.
- **RR-L1 (carry 29's comment says `sqrtsd` for sqrtf): fix pass, FX-11.** Cargo.toml's libm comment says sqrtf by
  `sqrtss` and sqrt by `sqrtsd` (libm 0.2.16 src/math/arch/x86.rs:13 and :26, the seat's read). Carry 29's own text is
  a reviewer's quoted words and stays as it is. The report lists carry 29 as met, with this correction named.
- **RR-L2 with EA-5 (npt-hash-wasm, a LEAF crate, has no clippy.toml, so plan.md §4.2's "Clock reads are banned in them
  by clippy" is false for it): fix pass, FX-12.** A new crates/hash-wasm/clippy.toml holds the HashMap and HashSet type
  bans and the std clock bans in the room's and lab's form (no tokio lines: hash-wasm has no tokio), its header citing
  plan.md §4.2's leaf rule. That makes the plan.md sentence true without a plan.md edit, which the other option needed.
- **RR-L4 with EA-1 (replay_to feeds the whole recording, so the drain is quadratic; the Wasm leg hashes every tick
  twice): a card follow-up.** The step takes 40-46 s of its 300 s. If U4r's Evidence decision (plan :800-802) or the
  cold figure (INF-1) brings time into question, first feed replay_to by chunks, as `serve` does, and cache the hash
  per `step` in hash-wasm's State (no export change).
- **RR-L5 (manifest names unchecked; `ticks` reaches `Vec::with_capacity` before the length and SHA-256 check): U4r's
  carry, entry 82 (T2), with EA-8.** U4r rewrites the manifest's reader for its runner recordings, so the checks go in
  with that change. The manifest is committed data, and today the step fails closed.
- **SEC-U4D2-3 (the signature scan reads the token `u32`, not the type): its comment in the fix pass (FX-11), and
  Q7 (b)'s type-section read as C-U7-2 (T3).** The comment beside EXPORT_INTEGERS states the limit (a module-local
  `type u32 = f64;` or `use core::primitive::f64 as u32;` passes the token scan; the artifact's export types are not
  read). The type read is about 60 lines of binary parsing, and its natural home is U7's browser-safe `moduleProblems`,
  which must also run on wasm-opt's output (C-U7-1). It stays an open security finding (Low) until U7: the report lists
  it.
- **SEC-U4D2-4 (a build script plus a renamed `include!` reaches inside an export's allow): fix pass, FX-6.** The deps
  step refuses a build script in each exception crate whose level is deny (a build.rs file, a `build` key in its
  manifest, or a custom-build target in `cargo metadata`). Neither has one today (read), and without one `env!("OUT_DIR")`
  does not compile. It also refuses `include` used as a path (not as an `include!`, `include_str!` or `include_bytes!`
  call) in those crates' sources. pathProblems' doc (check.mjs:367-370) names npt-hash-wasm beside npt-tcpinfo.
  Self-tests: V4's two forms from r1-sec-scan.log, each refused by name.
- **SEC-U4D2-6:** Block 3 (FX-1).
- **SEC-U4D2-8 with TE-L1 (the loader's kind refusal is untested; TP11 shows 63 of 63 pass with it disabled): fix pass,
  FX-5.** Two self-test cases: `begin` exported as a global, and `memory` exported as a function. Round 2's test seat
  re-plants TP11's disable, and both must fail by name. It is the third exception's own test, which ADR 0002:264-265
  asks for.
- **INF-2 (on Windows the copied executable loses its `.exe`): a card follow-up.** plan.md:668 puts this check in this
  container, so nothing runs it on Windows. If it is ever run there, the fix is to keep `extname(path)` in the copied
  name, with a check.test.mjs case.
- **INF-5 (a counted run on the merged tree takes about 11 minutes): M5's wording** (Block 9: in the background with a
  limit of at least 1,500,000 ms, U4e round 2's Q4 form; the wasm step re-timed from that run's printed split).
- **INF-6 (U7's Chromium leg and wasm-opt rerun have no runner yet): C-U7-2 (T3).** U7's brief names how its tools
  come (pinned downloads) and that a CI home would be .github/ work (risk tier 2, Mustafa's go).
- **INF-7 (README.md lists neither the wasm step nor its prerequisites): a follow-up for docs-keeper at U4's close.**
  The README is outside U4d2's lane (Lanes), and it was stale before U4d2 ("15 crates").

Block 4 written 2026-11-12.

### Block 5. Item 5: the engine-architect's and the test-engineer's findings (done)
**The engine-architect's nine Lows (APPROVE WITH FINDINGS accepted; labels stand):**
- **EA-1:** with RR-L4, a card follow-up (Block 4).
- **EA-2 (the status numbers live in lib.rs and in loader.mjs; ticks numbered from `--first-tick`): C-U7-2 (T3).**
  U7 splits the loader. The numbers then get one home, or a test that binds lib.rs's consts to the loader's. Today a
  renumbering fails closed, caught at the step's first end-to-end run (loader.mjs:158).
- **EA-3 (check.mjs reads npt-lab's verdicts from their English lines, never their exit codes): U4r's carry, entry 83
  (T2).** Every path fails closed today. At U4r's change to the step (its recordings: EA-8), the clean half also
  requires exit 0 from `npt-lab hash` and `hash compare`.
- **EA-4 (U7 can reuse the .wasm file, not the loader): C-U7-2 (T3).** Building the split now would be a feature added
  "for later", which the phase's rules refuse.
- **EA-5:** with RR-L2, FX-12 (Block 4).
- **EA-6 (TickHash.recording's doc invites legs named by run ids; HashCompare compares by position): the doc in the fix
  pass (FX-11), the rest in U4r's carry 83.** lab/src/lib.rs's doc says "the recording both legs replay (the same name
  on both legs)". U4r's -30 pre-check aligns its two lists by tick and says where its printed comparison comes from
  (compare_text is private to the binary). My condition 2 holds as built: HashCompare reads no file and knows no
  recording.
- **EA-7 (the public items to record): T1 (§4.1's rows, in the merge commit) and the report.** The report lists
  check.mjs's 23 new exports by count, each imported by check.test.mjs. The loader's three exports that no file imports
  (forEachChunk, parse, main) stay as they are and are named in T1, and U7's split (C-U7-2) decides their form. Removing
  `export` now would be churn ahead of that.
- **EA-8 (the manifest holds only the generator's inputs; U4r's recordings change its format): U4r's carry, entry 82
  (T2),** with RR-L5.
- **EA-9 (`hash compare` and the legs arm `--on` differently): fix pass, FX-8.** One default, the legs' (the bite armed
  on the first recording replayed), with h6's test following. hash.rs is open for RR-L3 anyway.

**The test-engineer's findings (REVISE accepted; labels stand).** Its runs settle each point by exit code and by
name, and its end state is clean: 17 of 17 sha256 equal, `diff -rq` against the read copy empty, crates/sim untouched.
I re-checked the 17 hashes myself (Notes).
- **TE-M1** is RR-M2, fixed as FX-1 and FX-2 (Block 1). Its negation case becomes a self-test, and round 2 re-runs TP2
  run 3, which must FAIL by the source rule.
- **TE-M2** is RR-M1 with SEC-U4D2-2, fixed as FX-3 (Block 1). Round 2 re-plants TP4 and TP10.
- **TE-L1** is SEC-U4D2-8: FX-5. **TE-L2** supports SEC-U4D2-3: its comment (FX-11), and the type read in C-U7-2.
- **TE-L3 (the std-sin verdict never re-shows that the bite caused the mismatch): U4r's carry, entry 83 (T2).** It is
  sound for the committed catch (SS-s1 tick 2 lies inside the hash-map pair's equal prefix, ticks 0 to 1,233). The guard
  is needed only once the catch moves, which only a re-run scan or a new recording can do. U4r's recordings are the
  first such change.
- **TE-L4 (carry 65 names the failing test, but its panic is lost to the 40-line tail): fix pass, FX-10.** It is cheap,
  in the lane, and it is what TE-M3's route needs. When any step fails, check.mjs writes the step's whole output to a
  file under the checkout's .gate/netcode-proof-test/ (one per step, replaced at each run, as the wasm step's folder
  is), and its FAIL lines name the file. Self-test: a recorded failing test run whose panic block lies above the last
  40 lines; the file holds it, and the printed lines name the path.
- **TE-M3 (npt-net's `l2::delay::tests::a_shutdown_s_never_completes_ends_at_its_deadline_without_spinning` failed
  once in the bites build under the counted run's load; it failed the same way in U3c): Medium, not before U4d2's
  merge. APPROVE as a finding.** It is not U4d2's, and this is shown, not assumed. U4d2 changes no npt-net file. The
  test (delay_tests.rs:2551-2603) reads no file; it counts pump runs after a 300 ms wait on a two-worker tokio runtime.
  npt-net's tests that read marks/SERIES.md (names.rs:143, delay_names.rs:449, socket_tests.rs:390) are fixed
  `contains` checks, which T4's added row cannot fail and which cannot fail once in many runs of one build: B2.3's
  reasoning for U4j holds here.
  - **What the merge's counted run does if it recurs (B2.3's rule; M6):** check.mjs's FAIL is never retried into green
    unnamed. That crate is re-run at once through main-run.sh with `--no-fail-fast`, its whole output kept under
    .gate/u4d2-merge/, with FX-10's file of the failed step copied beside it, and the test's name and panic go into the
    notes, tagged TE2-5. Because this test reads no U4d2 file and is in no crate U4d2 changes, it does not stop the
    merge. But the merge goes ahead only on a whole counted run in which all 11 steps pass, never on a test-step-only
    run. Any failure in npt-hash-wasm, npt-lab or npt-sim, in check.mjs's self-tests, or in a test that reads U4d2's
    files, stops the merge and comes to me.
  - **Where its cause goes (B2.3 (b)'s last sentence: "that test's cause is found and fixed in a planned change of its
    own"):** a planned change of the main session's (npt-net is U3's code, and no U4 lane holds crates/net), plan first
    with me, after U4d2's merge and before U4r's merge, so that U4r's counted and judged runs do not meet it unnamed. Its
    first step is the panic: FX-10's saved output from the next failure, or a loop of the bites build's workspace tests
    under load as B2.3 (b) ran them, now watching this test by name. That tells which assertion fails
    (`polls_after_deadline == 1`, `polls <= 3`, `s_calls <= polls`, or the 30 s hang guard) and why it fails under
    load. Never a retry, and never a looser bound without its cause. The card records it as TE2-5 (c).
  - **B2.3's rule after U4d2's merge:** its "until carry 65 is met" is read as met for the name only. The rule stays in
    force until TE-M3's cause is found and fixed. With FX-10, a first failure keeps its panic, so the rule's "name and
    panic" can be met from that failure itself.
- **Its other results** (the builder's six plants and TP1 to TP11 each failing by name where they should; h1 to h8
  asserting what they claim; no wrong-reason path for the hash-map verdict): accepted as evidence. They rest on runner
  logs with exit codes and libtest's or node's own summary lines. I read only the lines cited above, and none
  contradicts the seat.

Block 5 written 2026-11-12.
