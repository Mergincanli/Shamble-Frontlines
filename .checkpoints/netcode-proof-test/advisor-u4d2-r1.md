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

### Block 6. Item 6: the builder's points (done)
- **TE2-5 mid-build (62 of 63 in check.test.mjs, a failure its step plan did not name): outside the stop clause's
  letter. The outcome stands; it is recorded as a process slip (Low) and nothing is redone.** The clause (u4d2-brief.md
  :707-710) says "a failure ... in check.test.mjs ... stops your step group (report it), unless your step's plan names
  it as expected (a pin your step re-pins, or a test a later step rebuilds)". The failing test (check.test.mjs:205-222)
  was neither of those, so the step group should have stopped and reported. "My own unfinished code" is not one of the
  clause's exceptions, and it should not become one: the clause exists for the moment a builder judges its own failure.
  The judgement was right here, and that is proven, not assumed. The test is byte-identical to `<base>`'s (the test
  seat's sed diff). The fix was in the builder's new rule (check.mjs:138's `&& all.has("npt-sim")`). TP9 (r1-te-tp9-te25.log)
  shows that undoing the fix makes the unchanged test fail by name, so the test still pins what it pinned. The
  fix-pass brief carries the clause as written, and the report records the slip in T5. The builder's other TE2-5
  entries (h2's placeholder vector, the manifest's zero hashes, its own test errors) were named in advance by design or
  were failures in its own new tests: within the clause.
- **The three early `npt-lab hash` runs under the shared lock (b3-lab1 to b3-lab3; rule 2 asks `-x`): a process slip
  (Low), recorded, no redo.** None is a reported result. Each printed a placeholder's expected failure, and the three
  SHA-256 values it supplied were checked under `-x` by b3-lab4 ("as the manifest's") and by every counted run since
  (manifest `check()` binds length and SHA-256). A run from another copy's artifact could only have printed a value
  that those `-x` runs would then refuse.
- **rustfmt's diffs applied by Edit after the cut: within the write rule** ("never `cargo fmt` writing files: apply
  rustfmt's diff by Edit"); b8-rust2 shows fmt clean. Nothing to rule.
- **Its open choices:** the cold run is INF-1 (Block 1: report text now, the cold figure after the between-waves clean,
  no exception). `--digest-field` at the native layout's indexes is RR-L3 (Block 3: FX-8).

Block 6 written 2026-11-12.

### Block 7. Item 7: the main session's points (done)
1. **The restamp's stale plan cites: confirmed, with one addition.** At 3f58ff0d (the plan at 51520b4) the brief's
   section 1 item 6 (:141-143) and its :173 read §5 U4d2 at :666-712, Lanes at :850-888, §6's U7 line at :898-900 and
   §7 at :908-931 (headings read: 666/714, 850/890, 898, 908/933). **Also stale in the same item (:143):** "U4r's lines
   that reuse your work (:766-767, :790-792)", which read :776-777 and :800-802 at 3f58ff0d (the same text, read at
   both stamps). §4.1, §4.11, §4.14, §4.15, U4d's lines and §2's rows did not move. Nothing was built short because of
   it. The builder built §5 U4d2 to its end, Q11's two plants included (P5, P6). The seats judged against the whole
   section, and the arch seat found every path inside the Lanes line's U4d2 list. The brief is a closed record of what
   was built, so no re-fold is needed: T5 says it in one line. For the fix pass's brief, cite the plan by section
   heading, not by line.
2. **Carry 66's header param row: the main session writes it, in U4d2's merge commit, by script from T4 below, applied
   before M5's counted run.** The marks step and npt-net's name tests read SERIES.md. marks/ is in no wave-2 lane
   (Lanes), and U4e's merge carried its own SERIES.md row the same way (advisor-u4e-r1.md T4). The builder's proposed
   words are kept, with the Values cell saying what the hash is. The Used-by cell is "-3 (recorded)", the form the
   table's other recorded params use.
3. **A13: right not to wait; it is now committed, and I checked the commit.** 1b4a3c8 changes one file,
   crates/server/clippy.toml (`git show --stat`: 5 in, 1 out). Its blob equals the working tree's (a82dd3e7…). spikes/
   is otherwise unchanged since c88eb31 (`git diff --quiet`, exit 0). The diff is the room file's two type entries
   plus a header naming Q9 (a), §4.11 and A13 (read). The plant log shows clippy on the main folder's npt-server
   refusing a scratch HashMap at lib.rs:20 and :21, exit 101 (a13-plant.log:13-36). The control exits 0. check.mjs
   counted passed all 10 steps, 473 / 541 / 52 of 52, marks 81 (a13-check-counted.log). Its log has no provenance lines
   because no pre-build ran; for a clippy.toml-only change the plant log's "Checking npt-server (<main folder>)" line is
   the provenance that matters. Why no wait: crates/server is in no open lane (U4e merged), U4d2 changes no server
   file, and check.mjs reads no server/clippy.toml (CLIPPY_TWINS is sim and proto only). Left: CI's gate and smoke on
   the pushed head, which M2 checks. U4d2's merge then applies on a head that holds it (M2 names it).
4. **The "two exceptions" text: confirmed, with one more place.** None outside U4d2's lane. Inside it there are three,
   all in check.test.mjs: :328 (a test name), :364 (a comment) and **:529** ("unsafe code outside the two exception
   crates fails however the lint was relaxed (review S1)", a test name). Each says "three", or names what it means, in
   FX-11. Renaming a test changes no count.
5. **The merge's counts: confirmed as the method, with the fix pass's deltas added.** On c88eb31 (473 / 541 / 52 of 52,
   10 steps; A13 left them unchanged), U4d2's round-1 deltas (+17, +19, +11, +1 step) give 490 / 560 / 63 of 63 and 11
   steps. The fix pass adds its own (FX-8's unit test, +1 in each build; its new self-tests). M5 expects 490, 560 and
   63 plus the fix pass's deltas, each named, the sums shown. Interactions only the merged run can show are named in M5:
   the features comparison now walks U4e's npt-server library, and the wasm step's npt-lab release build carries U4e's
   crates. The counted run goes in the background with a limit of at least 1,500,000 ms (INF-5; U4e round 2's Q4).

Block 7 written 2026-11-12.

### Block 8. Item 8: the fix pass, its builder, and round 2's seats. APPROVE on these terms.
**Scope (each item with its proof; nothing else):**
- **FX-1** RR-M2's first half, SEC-U4D2-5 and SEC-U4D2-6 (Blocks 1 and 3). The deps step refuses every rustflags source
  that holds a flag, and a set CARGO_BUILD_TARGET or a `build.target` key. readBackCheck's doc and DEPS_CHECKED's limits
  say the read-back is exact because no source may hold a flag. Self-tests: RR-M2's negation pair (both sources named);
  `--cfg feature="bites"`, `-C overflow-checks=off` and `-C llvm-args=-fp-contract=fast` in RUSTFLAGS; a config file's
  `rustflags`; CARGO_BUILD_TARGET; `build.target`; each FAILs by name, and an environment with none passes. Live:
  r1-rr-superset.log's first environment (`--only deps`) now FAILs, naming both sources.
- **FX-2** RR-M2's second half and SEC-U4D2-7 (Blocks 1 and 2). `moduleProblems` refuses a `target_features` section
  that lists simd128 or relaxed-simd (`+` or `=`), and a module with no such section, with more than one, or with one
  that does not parse. Self-tests: hand-built modules (check.test.mjs's wasmModule helper) for +simd128, +relaxed-simd,
  no section, two sections, a truncated section, and the clean list accepted. Live: Block 2's target_feature plant
  (build exit 0, the step FAILs on the loader's refusal), and the two clean modules accepted.
- **FX-3** RR-M1, SEC-U4D2-2 and TE-M2 (Block 1): the self-test case, the fixture plant in an export's body, and TP4's
  live plant once (the wasm32 build exits 0, deps FAILs at the line).
- **FX-4** SEC-U4D2-1 (Block 1): every import refused by name before instantiating; `Object.create(null)`; three
  self-tests; the loader's header and instantiate's doc corrected.
- **FX-5** SEC-U4D2-8 and TE-L1 (Block 4): the two kind cases.
- **FX-6** SEC-U4D2-4 (Block 4): no build script in a deny-level exception crate; `include` refused as a path there;
  pathProblems' doc; V4's two forms as self-tests.
- **FX-7** INF-4 (Block 3): a 300 s timeout on each of the wasm step's non-cargo child calls; a timeout is a named FAIL;
  one self-test.
- **FX-8** RR-L3 and EA-9 (Blocks 3 and 5): "layouts differ (<n> and <m> words)" in place of a digest, with one unit
  test; one default for `--on`, the legs', with h6 following.
- **FX-9** INF-3 (Block 3): the std-sin failure line's IFUNC sentence.
- **FX-10** TE-L4 (Block 5): a failed step's whole output kept in a file under .gate/netcode-proof-test/ (one per step,
  replaced at each run), named in its FAIL lines; one self-test.
- **FX-11** texts: check.test.mjs:328, :364 and :529's "two" (Block 7, point 4); Cargo.toml's libm comment, sqrtf by
  `sqrtss` (RR-L1); SEC-U4D2-3's limit beside EXPORT_INTEGERS; TickHash.recording's doc (EA-6).
- **FX-12** RR-L2 and EA-5 (Block 4): crates/hash-wasm/clippy.toml.
- **Not the builder's:** the report, marks/SERIES.md (T4), the plan files and the carries (T1-T3), INF-1's figures and
  words (T5), and A13 (done).

**Bounds.** Files only in U4d2's lane, and of those only: check.mjs, check.test.mjs, crates/hash-wasm (loader.mjs,
src/lib.rs only for FX-3's and FX-2's restored plants, and the new clippy.toml), crates/lab/src/hash.rs and
crates/lab/src/lib.rs (a doc line), and Cargo.toml's libm comment. No Cargo.lock change, no manifest change, no new
public Rust item, and no new loader or check.mjs export beyond what a self-test imports (each one named in the record).
No change to recordings.json, std-sin-scan.txt or co-initial-dump.txt, and no crates/sim file, not even as a plant.
Carry 52 holds: the recordings are U4d-F's generator's, and PM-0001-2's 9.5 floor does not move. Any of these needed,
or a test that cannot be made to fail under its plant, stops that item: the builder reports it, and it comes to me.

**Who builds:** one builder (general-purpose, Opus, effort xhigh), in the same worktree (`.claude/worktrees/u4d2-build`,
uncommitted on 51520b4), reading its record (u4d2-build.md), this ruling's Blocks 1 to 8 and the copy's diff first,
as U4e's fix pass did. It works under W1-TARGET's lock (the lane lock held for each cargo run, the shared target only,
offline) through u4d2-run.sh, and counts in the three-agent limit. Its brief carries, word for word, u4d2-brief.md
section 4's binding sentences: the write rule (T4 item 1's sentence), the plant rule (B6), the stop clause with the
crates as named there, B2.3's rule, "No retry for the `wasm` step", and Mustafa's builders' limits. It also carries
this ruling's note on the stop clause (Block 6: an unnamed failure stops the step group, whoever's code caused it). It
cites the plan by section heading, never by line. Its plan is written first into the record's new section "Fix pass,
round 1", then a notes line after each FX.

**Its evidence (in the record):** for each FX, the plant's diff or the environment, the run (`-x -k`), the step, test
or self-test that FAILED by name, the restore by Edit, and the file's sha256 equal to its fixed form. Then fmt
(rustfmt's diff applied by Edit), clippy in both builds and for wasm32, and doc. Then check.mjs counted twice in the
copy, all 11 steps, with the counts as deltas on 416 / 465 / 63 of 63 and each new test named, and the wasm step's
printed split from both runs. At the end, the sha256 of every changed or new file under spikes/, in `sha256sum` form
with paths from the copy's root, saved as .gate/u4d2/fx-hashes-end.sha256.

**Then the main session** re-cuts the snapshot by worktree_patch.py (wip/u4d2.patch replaced, committed), checks that
the copy's files equal fx-hashes-end.sha256, and cuts round 2's packet by review_packet.py on what changed (round 1's
patch at e2f2a5b, a526ef8e…, against the new one), with this ruling as the "judged against" half.

**Round 2's seats, on what changed** (Opus at xhigh; counted in the three-agent limit; each brief carries T4 item 1's
write rule and "the shared target only"; batches as round 1's, so that no timing run meets a plant):
- **Batch 1** (runs through the runner in the builder's copy while nobody plants):
  - **security-auditor:** FX-1 against every source form it probed (r1-sec-scan.log's name-rule cases, RR-M2's pair,
    CARGO_BUILD_TARGET); FX-2 on sec-probe-tf2's module (refused) and on both clean modules (accepted); FX-4 on
    sec-probe-imports' modules (each refused by name); FX-6 on V4's forms; FX-3's case; SEC-U4D2-3's comment and
    C-U7-2's words.
  - **rust-reviewer:** FX-1 against cargo's rustflags and build-target rules; FX-2's section parse (LEB128 lengths, the
    prefixes, a missing or duplicated section); FX-8 (panics, casts) and FX-12 (clippy in both builds and targets);
    FX-11's libm comment against libm 0.2.16; r1-rr-superset.log's first environment, which must FAIL now.
  - **infra-engineer:** FX-7 (which calls, the limit, a hang's FAIL line) and FX-10 (the file's place, its replacement
    at each run, its size); the step's time after the fix pass (`--only wasm` with `-x`, twice); T5's INF-1 words
    against its finding.
- **Batch 2:**
  - **test-engineer** (with Edit, in the copy): its start hashes first, as .gate/u4d2/r2-hashes-start.sha256; each FX's
    proof re-shown in its own form where it doubts the builder's; TP2 run 3, TP4, TP10 and TP11 re-planted, each now
    failing by name; one counted check.mjs (in the background, with a limit of at least 1,500,000 ms).
  - **engine-architect** (read only): T1's §4.1 rows against the final diff (a loader export the fix pass adds or drops
    is named there); FX-2's and FX-4's place in the loader against C-U7-2's split; carries 82 and 83 and C-U7-2's words
    against the code.
- **Round 2's outcome:** APPROVE, or APPROVE WITH FINDINGS with Lows only, lets the merge go ahead on M1-M9; its Lows
  become follow-ups that the main session records. A Medium comes to me first and becomes a follow-up or a carry, never
  a third round. A High or a Critical comes to me for a third round on it alone (the review budget's one exception);
  one still open after that stops U4d2 for Mustafa.

Block 8 written 2026-11-12.

### Block 9. Item 9: the merge into the spike branch, after round 2, on conditions M1 to M9 (U4e's form)
Checked now: the spike branch's head is 6c97718. Since 51520b4 its spikes/ changes are U4e's merge c88eb31
(crates/room, crates/server, crates/client, marks/README.md, marks/SERIES.md) and A13's 1b4a3c8
(crates/server/clippy.toml alone). U4d2's lane paths and crates/sim are unchanged since 51520b4 (`git diff --stat`
empty). wip/u4d2.patch applies on c88eb31 (the card). plan.md §4.2 already names the three exceptions, npt-hash-wasm
the third (plan.md:435-447, read), and plan.md §4.1's layout line holds it (:392-393): the merge commit changes neither.

The conditions (the main session's; each result goes in u4d2-build.md's new section "Merge into the spike branch"):
- **M1, the copy is the one reviewed.** Before the snapshot, the copy's HEAD is still 51520b43391a…. `git status
  --untracked-files=all` shows only the fix pass's final file list under spikes/netcode-proof (round 1's 17 files and
  the new crates/hash-wasm/clippy.toml) and the record. `sha256sum -c .gate/u4d2/r2-hashes-start.sha256` from the
  copy's root gives all OK, and that list equals fx-hashes-end.sha256. Anything else stops the merge and comes to me.
- **M2, the base still fits.** On the spike branch's head, `git diff --quiet 51520b4 HEAD --` U4d2's lane paths
  (spikes/netcode-proof/check.mjs, check.test.mjs, Cargo.toml, Cargo.lock, crates/hash-wasm, crates/lab,
  crates/proto/Cargo.toml, crates/client-wasm/Cargo.toml) and spikes/netcode-proof/crates/sim is true. Every other
  spikes/ change since 51520b4 is U4e's merge c88eb31 or A13's 1b4a3c8, listed in the notes. A13's CI (gate and smoke)
  is green on its pushed head. Anything else stops the merge.
- **M3, the snapshot and the apply.** The saver is stopped first. worktree_patch.py on the copy gives a patch equal
  (cmp) to the committed wip/u4d2.patch that round 2 reviewed (its commit and sha256 in the notes), listing exactly the
  final files. Then `git apply --check`, then `git apply`, in the main folder.
- **M4, the merged bytes are the reviewed bytes.** From the main folder's root, `sha256sum -c` of round 2's start
  hashes gives all OK. A patch that applies but differs stops the merge.
- **M5, the text, then check.mjs counted on the merged tree.** First T4 (marks/SERIES.md's row), by script as M8 says,
  since the marks step and npt-net's name tests read SERIES.md. Then check.mjs through main-run.sh (the lane lock
  exclusive and idle at the start, the 2 s touch loop, the shared target, offline), in the background with a limit of
  at least 1,500,000 ms, its log under .gate/u4d2-merge/. Before it, in the same call, run section 4 rule 3's six
  builds (`cargo test --locked --workspace --no-run` and the same with `--all-features`; the release npt-lab and the
  release wasm32 npt-hash-wasm, each in both builds), whose "Compiling npt-hash-wasm" and "Compiling npt-lab" lines
  name the main folder's path. Expected:
  - **all 11 steps pass;**
  - **the counts:** measured 490, bites 560 and self 63 of 63, each plus the fix pass's delta (each new test named, the
    sums shown); marks 81 read and checked, with the ADR rows unchanged; redact 0 findings;
  - **the deps line:** npt-sim's features one set across the server's, npt-lab's and npt-hash-wasm's builds on the
    merged tree, where npt-server's library is now U4e's; the read-back "given no rustflags";
  - **the wasm line:** 27000 ticks and 0 mismatching for both pairs; h2 equal; the hash-map bite caught natively and
    native-against-Wasm (Wasm 1 against 2 not exercised); std-sin caught at SS-s1 tick 2 `vehicle[8].pos.y`; the step's
    printed split, under 300 s (re-timed: INF-5).
  
  In the same main-run.sh call, run h7 (ii) by name on release, with libtest's summary "1 passed" (carry 66's three
  values, for T5). Any other count or line is explained in the notes before the commit. A deps or wasm FAIL on the
  merged tree stops the merge and comes to me: it would be an interaction with U4e's crates that the copy could not
  show.
- **M6, a failing test is never retried into green unnamed** (B2.3; Block 5's TE-M3 route). The failing build's crate
  is re-run at once through main-run.sh with `--no-fail-fast`. Its whole output, and FX-10's file of the failed step,
  are kept under .gate/u4d2-merge/, and the test's name and panic go in the notes. A failure in npt-hash-wasm, npt-lab
  or npt-sim, in check.mjs's self-tests, or in a test that reads U4d2's files stops the merge and comes to me. Any other
  failure (TE-M3's npt-net test among them) is recorded under TE2-5, and the merge goes ahead only on a whole counted
  run with all 11 steps passing, the earlier failure named.
- **M7, the gate:** `node tools/gate.mjs` before the commit. If smoke (5a) cannot start this container's Chromium, use
  `--no-smoke`, keep both logs and give the reason in the notes. CI's gate and smoke are green on the pushed head.
- **M8, one commit,** "U4d2: the Wasm-in-Node hash and the determinism step (npt-hash-wasm, npt-lab's `hash`,
  check.mjs's wasm step), merged after review round 2". It carries:
  - the spikes/ files exactly as applied, with T4 in marks/SERIES.md;
  - `git rm` of wip/u4d2.patch;
  - under .checkpoints/ only: u4d2-build.md (with "Fix pass, round 1" and "Merge into the spike branch");
    u4d2-report.md (new; T5 is its rulings section, and the rest takes u4e-report.md's form: who built it, the rounds,
    the checks on the merged tree, the carries met and added by number, the public items, the Wasm sizes, the timing
    as T5 labels it, the follow-ups, Decisions needed, For Mustafa); u4-build-plan.md's §4.1 rows (T1); u4s-carries.md
    (T2); plan.md's U7 row (T3); and the card.
  
  Every text is copied by script from this file's code blocks, never retyped: each old text found exactly once (line
  breaks read as spaces), each new text found in place after, and no changed prose line over 120 characters (a table
  line, as T1's, T3's and T4's, is not prose). If the fix pass's final diff adds or drops a loader export, T1's list
  follows it as round 2's engine-architect names it. That is the one allowed change to a text, and it is recorded.
  The commit's spikes/ part is the reviewed bytes plus T4, and nothing else.
- **M9, after the push:**
  - the saver restarted;
  - the card updated: U4d2 merged at <sha>; the base counts for U4r; carries 82 and 83 and C-U7-2; the open security
    finding SEC-U4D2-3 (Low, carried by C-U7-2) for PR #40's report; TE2-5 (c), TE-M3's planned change; the follow-ups
    (RR-L4 with EA-1, INF-2, INF-3's CPU record, INF-7, and §7.1 item 6's text: see "What I checked");
  - the copies removed per CHECKPOINTS.md §10 once CI is green: `.claude/worktrees/u4d2-build`, `.gate/u4d2-r1` and
    `.gate/u4d2-prefix`, and `.gate/u4d2-rr-target` by `cargo clean --target-dir`;
  - then, with wave 2 over and no lane running, the between-waves clean of the shared target (`cargo clean
    --target-dir .gate/shared-target`: W1-TARGET item 3), and one `node check.mjs --only wasm` through main-run.sh,
    whose seconds are INF-1's cold figure, recorded on the card and in U4's report before U4r's brief.

Block 9 written 2026-11-12.

## The texts (each copied by script from its code block; never retyped)
Each anchor below was found exactly once at the head (6c97718), counted with `grep -cF`. Carry numbers 82 and 83 and
the name C-U7-2 are free (no entry, no mention).

### T1. u4-build-plan.md §4.1: npt-lab's row and npt-hash-wasm's row (the merge commit)
(a) The row `| npt-lab | subcommands `scenario` (with `--record`), `replay` and `hash` | plan.md:590-591 |` becomes:
```
| npt-lab | subcommands `scenario` (with `--record`), `replay` and `hash` (`hash` with its modes: the two native legs, `compare`, `vectors` and `bites`); in the library, `HashCompare` (its constructor `of` and its variants `Equal`, `Empty` and `Mismatch`) and `TickHash` (the per-tick entry `of` takes); in the run header, the param `fate_log_v1_datagram_sha256` | plan.md:590-591; the advisor's condition 2 (`HashCompare` is U4r's -30 comparer, so it lives in the library and reads no file); §4.6's "0 compared fails" (`Empty`, as npt-judge's `eval::Outcome::Empty`); carry 66 (the v1 datagram hash, for §5 U4c's L-c8); the advisor's U4d2 round-1 ruling (EA-7) |
```
(b) The row `| a new crate | `npt-hash-wasm` (cdylib; integer-only exports) | U4-WASM (B) |` becomes:
```
| a new crate | `npt-hash-wasm` (cdylib): its seven `#[unsafe(no_mangle)] pub extern "C"` exports `begin`, `feed`, `step`, `hash_lo`, `hash_hi`, `dump_len` and `dump_word`, integer-only (u32), every const private, beside wasm-ld's `memory`; its Node loader crates/hash-wasm/loader.mjs, exporting `HASH_WASM_EXPORTS` (the export list's one home, which check.mjs imports), `LINKER_EXPORTS`, `LOADER_EXPORTS`, `BEGIN`, `STEP`, `moduleProblems`, `exportProblems`, `instantiate`, `forEachChunk`, `replay`, `parse` and `main` | U4-WASM (B); plan.md §4.2's third `unsafe` exception (ADR 0002:264-265); U7 splits the loader for the browser (C-U7-2); the advisor's U4d2 round-1 ruling (EA-7) |
```

### T2. u4s-carries.md §U4r, after entry 77 (its last), above "## The next change to crates/judge (whoever makes it; its first step)" (the merge commit)
```
82. **EA-8 with RR-L5, the recordings manifest (u4d2-review-arch.md, EA-8; u4d2-review-rust.md, RR-L5; the advisor's
    U4d2 round-1 ruling, Blocks 4 and 5):** "U4r's runner recordings join crates/lab/recordings.json (§4.15), whose
    entries today hold only the synthetic generator's inputs, read by hash.rs's `manifest` and generated by its
    `regenerate_all`: U4r changes that schema (a kind or script key, and the dispatch both functions need), and its
    brief says so. With that change, the reader checks each entry's name against `[A-Za-z0-9-]+` and bounds `ticks` by
    `bytes / 12` before it generates anything (today a bad name reaches a file path and the `h <recording> <tick>
    <hash>` line, and a large `ticks` reaches `Vec::with_capacity` before the length and SHA-256 check)."
83. **EA-6, EA-3 and TE-L3, the comparer for -30 and the wasm step's verdicts (u4d2-review-arch.md, EA-3 and EA-6;
    u4d2-review-test.md, TE-L3; the advisor's U4d2 round-1 ruling, Block 5):** "(a) For -30's pre-check, U4r gives
    `HashCompare::of` two lists that name the same recording on both legs and line up tick for tick (it compares by
    position, names included), and says where its printed comparison line comes from (`compare_text` is private to
    npt-lab's binary). (b) At U4r's change to check.mjs's `wasm` step (its recordings: entry 82), the clean half also
    requires exit 0 from `npt-lab hash` and `npt-lab hash compare` (today the verdict is read from their English lines
    alone, which fails closed), with one fixture case each. (c) In the same change, the std-sin verdict re-shows that
    the bite causes the mismatch it judges: the bitten native leg's first difference from a clean bites-build native leg
    is the committed (recording, tick, field), or the step FAILs when the committed catch is not on the manifest's
    first recording before the hash-map bite's arming tick (today it is sound only because SS-s1's tick 2 lies inside
    the hash-map pair's equal prefix)."
```

### T3. plan.md's U7 row: C-U7-2 (the merge commit)
The text `or states why not |` (found once, at the end of the U7 row's last cell) becomes the text below, on the row's
one line (no "|" added inside the cell):
```
or states why not. **C-U7-2** (the advisor's U4d2 round-1 ruling, 2026-11-12: EA-2, EA-4, INF-6, SEC-U4D2-3 and TE-L2): U7's Chromium hash check loads U4d2's npt-hash-wasm.wasm as it is (no imports; the seven integer exports and `memory`), and splits crates/hash-wasm/loader.mjs into a browser-safe module (the export list, its one home kept; the status numbers, with one home or a test binding them to lib.rs's constants; `moduleProblems` with its import, kind and `target_features` refusals; the replay loop over a chunk source) beside the Node script; `moduleProblems` also reads each export's type from the module's type section and refuses any but i32 (U4d2's source scan reads the token `u32` only), and it runs on the wasm-opt output C-U7-1 checks, so U7 keeps that output's `target_features` section or states how it checks the features; U7's brief names how its Chromium leg and wasm-opt get pinned tools (no CI job runs spikes/; a CI home is .github/ work, risk tier 2, Mustafa's go) |
```

### T4. marks/SERIES.md, the header-params table, after `fate_log_sha256`'s row (the merge commit, before M5's run)
After the line `| `fate_log_sha256` | hex string | -3 |` (found once, :397 at the head), insert:
```
| `fate_log_v1_datagram_sha256` | hex string: the SHA-256 of v1's tag and the v2 fate log's datagram entries (carry 66), which §5 U4c's L-c8 compares with U2's committed hashes | -3 (recorded) |
```

### T5. u4d2-report.md's section on the review's rulings (new file, beside u4e-report.md)
Copied by script as it is. Each `<…>` is a value the main session fills from the named source (round 2's notes, the
fix pass's record, or the merged tree's M5 lines), and the notes name the log and line of each fill; no other word
changes.
```
## What U4d2's review settled (the advisor's U4d2 rulings: round 1, advisor-u4d2-r1.md; round 2, <round 2's ruling file>)
- **Round 1** (five seats on one head, 51520b4 plus the snapshot at e2f2a5b; the spend limit cut the second batch at
  20:52 UTC, after the architect's and the test seat's plans and with no plant in place, and both resumed at 22:53 on
  the same bytes, so one round): rust-reviewer, security-auditor and test-engineer REVISE, infra-engineer and
  engine-architect APPROVE WITH FINDINGS; no Critical or High; six Mediums (RR-M2 = TE-M1; RR-M1 = SEC-U4D2-2 = TE-M2;
  SEC-U4D2-1; SEC-U4D2-7; INF-1; TE-M3). One fix pass (FX-1 to FX-12), then round 2 on what changed: <round 2's
  verdicts>.
- **The build-flag guard was fail-open for one case, now closed (RR-M2; Medium; fixed).** The read-back gave rustc every
  rustflags source at once, but cargo uses only the first non-empty one, so a negation in a source cargo skips hid an
  enabling flag in the one it used: the deps step passed while the module built so listed +simd128 and +relaxed-simd.
  The advisor's U4D2-Q4 ruling had called that superset fail-closed; it was not. Since the fix the deps step refuses
  every rustflags source that holds a flag (the spike's builds use none), and a set CARGO_BUILD_TARGET or
  `build.target`, so the read-back reads exactly what cargo gives rustc; and the loader refuses a module whose
  `target_features` section lists simd128 or relaxed-simd, or that has none, which also covers a safe
  `#[target_feature]` function reaching the Wasm leg with no flag and no `unsafe` (SEC-U4D2-7; Medium; fixed).
- **The third exception's guards (RR-M1, SEC-U4D2-1; Medium; fixed).** An `unsafe` block inside a listed export's body
  is accepted by rustc under the export's own allow, so only the deps scan refuses it: that case now has its self-test
  and was shown live (build exit 0, deps FAIL at its line). The loader's empty import object inherited
  Object.prototype, so a module importing `constructor.constructor` instantiated; it now refuses every import by name
  and instantiates with a null-prototype object. The kind refusal has its two self-tests (SEC-U4D2-8); a build script
  and `include` used as a path are refused in the deny-level exception crates (SEC-U4D2-4).
- **The step's time (INF-1; Medium; report text).** Every figure here is warm for dependencies, the workspace rebuilt
  by the runner's touch loop: <the fix pass's and round 2's wasm step seconds>, and <the merged tree's> on the merged
  tree, against a 5-minute budget. No cold run was taken before the merge: the shared target always held other lanes'
  release dependencies, and Mustafa's one-folder rule gives a copy no empty target. The first release builds in the
  lane took 16.04 s (npt-lab) and 3.57 s (npt-hash-wasm, wasm32), and the infra seat's bound for a cold step is about
  80 s. The cold figure comes from the first run after the between-waves clean of the shared target (W1-TARGET item 3):
  <its seconds, from the card>.
- **An npt-net test that fails under load (TE-M3; Medium; not U4d2's).** `l2::delay::tests::a_shutdown_s_never_
  completes_ends_at_its_deadline_without_spinning` failed once in the test seat's counted run (bites build) and passed
  on the immediate re-run; it failed the same way in U3c. It reads no U4d2 file and is in no crate U4d2 changes, so it
  did not stop the merge (B2.3's rule); its cause is a planned change of the main session's before U4r's merge (TE2-5
  (c)). Since FX-10, a failed step's whole output is kept in a file, so the next failure keeps its panic.
- **Process slips, none weakening the evidence.** The builder judged an unnamed check.test.mjs failure inside its step
  instead of stopping (its own new rule repeated a sentence the test pins once; the test is unchanged and still bites:
  TP9); three early `npt-lab hash` runs took the shared lock where `-x` was asked (none a reported result; each value
  re-checked under `-x`); the rust seat built one module in a scratch target off the shared one (38 MB, removed by
  `cargo clean --target-dir`) to keep a SIMD-built module out of it. The builder's brief cited four plan spans, and
  U4r's two lines, at the plan's earlier stamp; the build covered §5 U4d2 whole.
- **Carries.** Met: 29 (Cargo.toml's libm comment; sqrtf is `sqrtss`, sqrt `sqrtsd`, a correction to the carry's own
  words), 41, 52 (only recordings by U4d-F's generator, adb1bab), 65 (the failing tests named; their panics kept since
  FX-10), 66 (<the three v1 datagram hashes and their commands>); 42 does not apply (entry 34 was met by U4a). Added:
  82 and 83 (U4r), C-U7-2 (U7).
- **Open security finding:** SEC-U4D2-3 (Low): the integer-signature rule reads the token `u32`, not the type; the
  limit is written beside the rule, and the type-section read is U7's (C-U7-2). Every other security finding of the
  review is fixed in this change.
```

## Before the merge (in this order)
1. **The fix pass** (Block 8): FX-1 to FX-12 in the builder's copy, with their proofs; the snapshot re-cut and
   committed; round 2's packet.
2. **Round 2** on what changed (Block 8's seats and batches), and its outcome under Block 8's rule (a Medium to me
   first; a High or Critical a third round on it alone).
3. **M1 to M4:** the copy is the one reviewed; the base still fits (U4e's merge and A13's commit the only other spikes/
   changes; A13's CI green); the snapshot equal to the committed one, applied; the merged bytes equal round 2's start
   hashes.
4. **M5:** T4 by script; then check.mjs counted on the merged tree in the background (limit at least 1,500,000 ms),
   after the six builds, all 11 steps, the counts 490 / 560 / 63 plus the fix pass's named deltas, marks 81, redact 0,
   the deps and wasm lines as Block 9 says, and h7 (ii) by name.
5. **M6 and M7:** a failing test re-run at once and named (TE-M3's flake does not stop the merge, but the merge needs a
   whole clean run); `node tools/gate.mjs`, with `--no-smoke` only alongside both logs and the reason.
6. **M8:** one commit carrying the reviewed spikes/ bytes plus T4, and under .checkpoints/ the record, u4d2-report.md
   (T5), u4-build-plan.md (T1), u4s-carries.md (T2), plan.md (T3) and the card; `git rm` of wip/u4d2.patch.
7. **M9 after the push:** the saver; the card (carries 82, 83 and C-U7-2; SEC-U4D2-3 open; TE2-5 (c); the follow-ups);
   the copies and the rust seat's scratch target removed; then the between-waves clean and the cold `--only wasm` run
   (INF-1), before U4r's brief, which names carries 82 and 83.

## Verdict, per item of the brief
| Item | Verdict | In one line |
|---|---|---|
| 1. RR-M2 (= TE-M1) | REVISE: fix pass, FX-1 and FX-2 | every rustflags source with a flag refused, plus the module's `target_features` read; my Q4 (a)'s "fail-closed" corrected |
| 1. RR-M1 (= SEC-U4D2-2 = TE-M2) | REVISE: fix pass, FX-3 | the export-body self-test, the fixture plant in a body, TP4's live plant |
| 1. SEC-U4D2-1 | REVISE: fix pass, FX-4 | every import refused by name; `Object.create(null)` |
| 1. INF-1 | APPROVE as report text (T5) | the cold figure after the between-waves clean (W1-TARGET item 3); no §7.1 exception, nothing to Mustafa |
| 2. SEC-U4D2-7 | APPROVE as a finding: fix pass, FX-2 | not a carry: the same section read, failing closed |
| 3. SEC-U4D2-5, -6; INF-3; INF-4; RR-L3 | APPROVE routes: FX-1, FX-1, FX-9 (+ a follow-up), FX-7, FX-8 | each rides the needed push |
| 4. the other Lows | APPROVE routes (Block 4) | FX-5, FX-6, FX-11, FX-12; carries 82 and C-U7-2; follow-ups; M5's wording |
| 5. EA-1 to EA-9 | APPROVE WITH FINDINGS accepted; routes (Block 5) | FX-8, FX-11, FX-12; carries 82, 83, C-U7-2; T1; a follow-up |
| 5. TE-M1, -M2; TE-L1 to -L4 | accepted; FX-1 to -5, FX-10, FX-11, carry 83, C-U7-2 | the test seat's runs stand as evidence |
| 5. TE-M3 | APPROVE as a finding, not before the merge | M6's rule at the merge; its cause a planned change before U4r's merge (TE2-5 (c)); B2.3's rule stays until then |
| 6. TE2-5 mid-build | outside the stop clause's letter; outcome stands; Low process slip in T5 | the clause stays as written |
| 6. the three `-s` runs; rustfmt by Edit | slip recorded (Low); within the rule | no redo |
| 7. points 1-5 | confirmed, 1 and 4 with one addition each; A13 checked at 1b4a3c8 | Block 7 |
| 8. the fix pass and round 2 | APPROVE on Block 8's terms | FX-1 to FX-12; one builder, same worktree; five seats, two batches |
| 9. the merge | M1 to M9 (Block 9) | 11 steps; 490 / 560 / 63 plus the fix pass's deltas; T1-T5 by script |

Risk tier 1 (spike code and the spike's own check on PR #40's branch; check.mjs is not one of tools/' gate files), as
at the brief check. Round count: round 1 done; one fix pass; round 2 on what changed; a third only for an open Critical
or High.

## Decisions needed
None. Every point is a technical or test choice inside the documented vision, a route between units, or a correction
of my own earlier ruling (U4D2-Q4 (a)'s "fail-closed"), all of which I decide. INF-1's cold figure needs no exception to
Mustafa's shared-target rule: W1-TARGET item 3's between-waves clean is an approved method within his one-folder words
(history.md:411-412). No word or decision of Mustafa's changes, and nothing a player sees, hears or feels is touched.

## For Mustafa
Nothing. PR #40 stays needs-mustafa for its existing reasons. U4d2's report will list one open security finding
(SEC-U4D2-3, Low, carried to U7 by C-U7-2), with no action for him now. For his information: one reviewer seat ran one
cargo build into its own 38 MB scratch folder instead of the shared one, a slip against his one-folder rule. Its
reason was sound, it is in the report, and the folder is removed at M9.

## What I checked, and what I did not
**Checked myself (reading, git and sha256; no build, run or plant):**
- My role file from main, whole; the brief, whole; the five seats' notes and the builder's record, whole; my brief
  check (advisor-u4d2-brief.md), whole; U4e's R8, R9 and round-2 Q3 and Q4, for the form.
- The rules by place: §5 U4d2, Lanes, §6, §7.1 and Mustafa's words behind it; W1-TARGET item 3; B2.3; carries 29, 41,
  42, 52, 65 and 66; plan.md §4.2 and §4.5.
- The copy: HEAD, status, and the 17 sha256 against the test seat's start list. The code each fix's form rests on
  (check.mjs's rustflags sources, flag rules, read-back, run, failedRun and main's FAIL print; the loader's refusal
  and instantiate). The seats' logs at the lines the Mediums cite.
- The main session's points: the plan's headings at 1e8c188 and 51520b4; the brief's stale cites; the three "two"
  texts; A13's commit, its plant log and its counted run; §4.1 at the head; the loader's and check.mjs's exports; every
  anchor of T1 to T4, found once.
- **A drift seen on the way (a follow-up, not U4d2's):** u4-build-plan.md §7.1 item 6 says the main folder's merge
  checks run on its own cache with CARGO_TARGET_DIR unset. main-run.sh, which U4j's round-2 M5 and every merge since
  have used, sets the shared target. The practice follows a later ruling, so the plan's text should say so; the main
  session edits it at U4's close, or names the ruling.

**Not checked:**
- I built, ran and planted nothing. The fix forms rest on the seats' runs and on my reading of the code. I did not
  confirm that a safe `#[target_feature(enable = "simd128")]` function with no SIMD intrinsic in its body makes wasm-ld
  list +simd128; Block 2 names the relaxed_madd form the security seat proved as the fallback for FX-2's plant.
- My reading that, on x86-64, calling a `#[target_feature]` function from code built without that feature needs
  `unsafe` (Rust's target_feature 1.1) is from memory. I did not check it against the reference this run. It decides
  only why the native legs are not SEC-U4D2-7's concern.
- I did not read the packet's diff sections line by line. I read the changed files' load-bearing parts in the copies.
- I did not re-run any seat's timing, and I did not check A13's CI run myself; M2 asks for it.
