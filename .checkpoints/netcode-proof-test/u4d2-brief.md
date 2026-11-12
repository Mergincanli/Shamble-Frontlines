# U4d2 builder brief: the Wasm-in-Node hash and the determinism step (rev 1, drafted by rust-reviewer, 2026-11-12)

Drafted by rust-reviewer on 2026-11-12 (U4 wave 2), in the form of the approved U4e brief (u4e-brief.md rev 3.3;
advisor-u4e-brief.md). The drafter ran no cargo build, test, clippy, doc or check.mjs (the shared target is in use:
W1-TARGET); the read-only commands it ran are listed in u4d2-brief-notes.md. Next: test-engineer's check of rev 1
(u4d2-brief-check.md), the drafter's rev 1.1, the main session's fold (section 7), then the advisor's check before the
build (advisor-u4.md:154-158, condition 12).

Every place where the brief must hold someone's words verbatim (the carries, Mustafa's words, the advisor's sentences
the card makes binding) is a placeholder `<<P...>>` naming its file and lines; the main session copies each by script
at launch (section 7.2). Every `<base>` and count is filled then too.

## 0. How to read the citations
- Every file:line is at the main folder's head when rev 1 was written, cea8209, whose spikes/ is identical to 28a85e7
  (U4e's L3, the carry-51 commit; `git diff --stat 28a85e7 HEAD -- spikes/` is empty), which sits on U4d-F's merge
  adb1bab and U4j's d50396a. U4e (wip/u4e.patch, in review) is not in it.
- `plan` is .checkpoints/netcode-proof-test/u4-build-plan.md (1,079 lines, sha256 686a583c…); `plan.md` is the proof
  test's plan (rev 3.3, 1,518 lines, sha256 2bef127b…); `carries` is u4s-carries.md (490 lines, sha256 cdab0240…);
  `advisor` is advisor-u4.md; `wave1` is advisor-u4-wave1.md; `close` is advisor-u4d-close.md; `df-ruling` is
  advisor-u4df.md; `j-r2` is advisor-u4j-r2.md; `e-ruling` is advisor-u4e-brief.md; `card` is
  .checkpoints/netcode-proof-test.md; all but the card in .checkpoints/netcode-proof-test/.
- A crate path (`lab/src/main.rs`, `replay.rs`) is under spikes/netcode-proof/crates/ (`replay.rs`, `world.rs`,
  `hash.rs`, `lib.rs` alone are sim/src/); `check.mjs`, `check.test.mjs`, `Cargo.toml` and `Cargo.lock` are
  spikes/netcode-proof/'s.
- **The plan's check.mjs cites have moved** since plan §5 U4d2 was written (U4a, U4d and U4j added code above them).
  This brief uses the lines at the head (U4D2-Q1 asks whether the plan text is re-pointed):

| Plan §5 U4d2 cites | What it names | At the head |
|---|---|---|
| check.mjs:25 | `LEAF` | check.mjs:31 |
| check.mjs:1110 | `WASM_CRATES` | check.mjs:1438 (`WASM_ARGS` :1439) |
| check.mjs:135-137 and :135 | `UNSAFE_EXCEPTIONS` and npt-tcpinfo's per-function allows | check.mjs:161-174 (`UNSAFE_EXCEPTIONS` :169, `EXCEPTION_LEVELS` :171, `TCPINFO_UNSAFE_FILE` and `_FNS` :173-174) |
| check.mjs:94-102 | npt-tcpinfo's dependency rule | check.mjs:107-121 (and "nothing depends on npt-lab" :152-157) |
| check.mjs:903-942 | the environment's rustflags and the cargo configs | check.mjs:1196-1284 (`LINT_LOWERING` :1202, `cargoConfigProblems` :1250-1284) |
| check.mjs:1156-1158 | the runner refusal's note ("no wasm test is planned") | check.mjs:1527-1529 (the refusal itself: :1263, :1281) |

- "marks S-HashMap" (plan :698) is the marks scout's finding S-HashMap (u4-scout-marks.md:18: std documents per-instance
  random seeding and nothing for wasm32-unknown-unknown), in the plan's naming of the scouts' findings (plan :12-15).
  PM-0004-21 and PM-0004-74 are ADR 0004's rows (docs/adr/0004-simulation-core.md:1142, :1184). None of the three is
  in marks/marks.json, whose marks are ADR 0001's PM-0001 rows, PM-0004-67 and PM-0006-11 to -27 (check.mjs:1355-1361):
  no U4d2 bite or step is judged by npt-judge.

---

## 1. Start

1. **Base:** `<base>` (placeholder `<<P6>>`, filled by the main session at launch: the spike branch's head, which holds
   U4d's merge 39f4290 and U4d-F's adb1bab, U4d2's dependency (plan :666) and the commit carry 52 makes it name). In
   your worktree: `git merge --ff-only <base>`; if that cannot fast-forward, `git switch -c u4d2-build <base>`; then
   `git log -1 --format=%H` shows `<base>` and `git merge-base --is-ancestor adb1bab HEAD` exits 0. Record both in your
   notes before anything else.
2. **Who and where:** one builder, general-purpose, opus, effort xhigh, in an isolated worktree under
   `/home/user/Shamble-Frontlines/.claude/worktrees/` (`isolation: worktree`), under plan §7.1's shared-target rule and
   W1-TARGET's lock (section 4; plan :896-908; wave1 :558-586). U4d2 runs in wave 2 beside U4e's lane (plan :863-867;
   e-ruling L4, :335-336): two lanes, no shared file (U4e's patch touches crates/room, crates/client and crates/server
   only: wip/u4e.patch's file list). Reviewers count toward the three-agent limit (plan :872-873).
3. **Your notes:** `.checkpoints/netcode-proof-test/u4d2-build.md` in your worktree. Its first line names this task;
   then your plan as blocks (one block: one file's item with its tests, one check with its self-test, one bite and its
   proof, one run); each finished block written at once, a notes line per finding, one line after each block (done,
   what next). On a restart, read your notes and `git diff` in the worktree first, then continue after the last block
   marked done (the builders' limits, `<<P2>>`).
4. **The runner:** `<<P7>>` (the main session writes it at launch: u4e-run.sh with the worktree's path and the log
   folder `/home/user/Shamble-Frontlines/.gate/u4d2/`). `bash <runner> [-x] <worktree> <log> <command...> [:: <command...>]`
   takes the lane lock (`-x` exclusive), waits until no other cargo or rustc runs, checks 4 GB free, touches every file
   under your copy's crates/ first and every 2 s while the commands run, runs each command from your copy's
   spikes/netcode-proof with the shared target and `CARGO_NET_OFFLINE=true`, and appends each exit code and the
   provenance lines (u4e-run.sh:1-12). A run's result is its exit code and the test runner's own summary lines, never
   a grep (`<<P2>>`).
5. **What stops you** (each written in your notes and report, then build the rest): a change to a file outside your
   lane or to an npt-sim item (2.8); a failure under the stop clause (`<<P4>>`, with `<<P5>>`); a manifest change beyond
   2.5; a new external dependency; disk below section 4 rule 6's floor; a stop named in a part's own text (2.7's
   std-sin row; 2.4 (c)'s read-back). Every U4D2-Q of section 8 is ruled before launch (7.2 item 4); where this brief
   says "by U4D2-Qn" it follows the recommendation, and the main session's fold puts the ruling's words in its place.
6. **Read before you build** (and nothing you do not need):
   - plan §5 U4d2 (:666-702); §4.1 (:178-203: the npt-lab row :198, npt-hash-wasm's :202); §4.11 (:392-407); §4.14's
     hash-map row (:476); §4.15 (:478-504); U4d's D-2, D-7 and bites lines (:612-614, :629, :640-642); U4r's lines that
     reuse your work (:766-767, :790-792); Lanes (:840-875); §6's U7 line (:885-887); §7 (:895-918); §2's Features and
     C-U7-1 rows (:157, :159);
   - plan.md §4.1's hash-wasm line (:392-393), §4.2 (:421-454; the three exceptions :435-447), §4.5 (:527-551), §4.7's
     npt-lab line (:617);
   - advisor: conditions 2 (:90-101), 7 (:130-136), 8 (:137-139), 10 (:143-149) and 12 (:154-158); the Q3 ruling
     (:198-199); the F2 ruling's point (3) (:450-458: "in wave 2 after U4d2 has merged");
   - wave1: W1-PUB's U4d part (:387-416), W1-TARGET (:558-586); close: B1.2 and B1.3 (:42-180) for carry 52;
     df-ruling: R1 (:28-87), R4.1 (:213-231), R6 (:301-322), T4 (:459-470); j-r2: B2.3 (:146-171); e-ruling: F2-ROUTE
     (:277-310), L4 (:335-336);
   - carries §U4d2 (:407-446), quoted in section 3; the recordings section (:351-379), for 3.2;
   - docs/adr/0002-code-structure-build-toolchain.md:252-266 (the build flags, wasm-opt) and :500-505 (the determinism
     suite's tools); docs/adr/0004-simulation-core.md:1142 (PM-0004-21), :1143 (PM-0004-22), :1184 (PM-0004-74);
   - the code at `<base>`: sim/src/replay.rs (the layout :1-15, `Recording` :39-54, `GENERATOR` :285-288, `synthetic`
     :346-373, `Step` :608-619, `Replayer` :621-770), world.rs (`Bite` and `BITE_IDS` :585-606, `hash_map_order` and
     `std_sin` :608-635, the loops' periods :794, `World::BITES` :934-938, `with_bite` :940-966, `bite_order_digest`
     :968-973, `sinf` :994-1000, `dump` :1018, `place_vehicles` :1215-1228), hash.rs (the word sink :14, `StateDump`
     :66, `FieldDiff` :73-82, `words` :93, `hash` :98, `field` :107, `first_difference` :115), lib.rs (:13-57);
     sim/tests/replay.rs (D-2 :21-309, D-7 :311-390); crates/lab (all four src files and Cargo.toml); crates/tcpinfo
     (Cargo.toml, src/lib.rs:1-40, the two allows in src/linux.rs); l1's `FateLog` (l1/src/lib.rs:325-352) and
     l1/tests/l1.rs:478-532; ops/runs/c-u2-pm-0001-3.mjs; Cargo.toml; and check.mjs and check.test.mjs whole (you are
     their only editor in wave 2: plan :863-864).

---

## 2. What to build

U4d2 builds plan §5 U4d2 (:666-702) exactly: its Builds (option B), its check.mjs steps and rules, its Tests and its
Bites, plus the carries of section 3. It moves no mark (none is named; section 0's last bullet). Out of scope, by the
plan: wasm-opt and its flags (C-U7-1: plan :159, :885-887; advisor :137-139), the Chromium run (plan.md §4.5:548's
"once in local Chromium" is U7's: plan :885-886), npt-client-wasm and wasm-bindgen (U7), U4r's runner recordings
(`scenario --record`) and -30's pre-check (U4r, plan :762-786, which reuses `HashCompare`), the counting allocator (U5).
Where this brief finds the plan silent it says "plan silent" and keeps to the least that makes the item testable
(u4d2-brief-notes.md, "Choices"); it adds no feature.

### 2.1 npt-hash-wasm (crates/hash-wasm/: Cargo.toml, src/, the loader of 2.2, the vector file of h2)
- **The manifest** (plan :667, :670, :684-686; plan.md:392-393, :442-446): package `npt-hash-wasm`, the workspace's
  package fields; `[lib] crate-type = ["cdylib"]`; `[features] bites = ["npt-sim/bites"]`; `[dependencies]
  npt-sim.workspace = true` and nothing else, no dev dependency; its own `[lints.rust] unsafe_code = "deny"` and
  `[lints.clippy]` equal to the workspace's key for key (Cargo.toml:93-99), in npt-tcpinfo's form, with its comment
  saying why it leaves the workspace's lints (tcpinfo/Cargo.toml's `[lints]` comment and tables).
- **The exports** (plan :667-669): thin wrappers over `npt_sim::replay::Replayer` (replay.rs:621-770), each
  `#[allow(unsafe_code, reason = "...")]` (a reason with words: check.mjs:209-216) directly on
  `#[unsafe(no_mangle)] pub extern "C" fn`. In edition 2024 `no_mangle` takes the `unsafe(...)` form and the
  unsafe_code lint fires on it (check.mjs:199-207), so the allow is what compiles it under the crate's deny. No
  `unsafe` block, fn, impl, trait or extern block exists (plan.md:444-445), so the state is held without `static mut`:
  a `thread_local!` RefCell (plan silent; per thread, so the native unit tests cannot share it; on wasm32 without the
  atomics feature it is one static). Every parameter and result is a 32-bit integer, u32 or i32 (plan silent on the
  width; the plan's lo/hi halves imply it; Node passes BigInt for i64). The seven, with what each does:
  - `begin(bite, arm_tick) -> u32` (by U4D2-Q3 (b); under (a) the plan's `begin(seed_lo, seed_hi, scene, bite,
    arm_tick)`, failing at the header when scene or seed differs): a fresh `Replayer` (`Replayer::new`, :647), or, for
    `bite` k > 0, `Replayer::with_bite(World::BITES[k - 1], arm_tick)` (:665-670; world.rs:934-938) in the bites build;
    k outside 1..=4, or any k > 0 in the measured build, is refused by its return status and changes nothing. The index
    is mapped from the id at run time (2.3: npt-lab's bites build prints `World::BITES`), never from a copy of the ids
    in hash-wasm's sources or in JS (one home: the bite-home rule, check.mjs:689, :734-758);
  - `feed(word)` (`Replayer::feed`, :673-675);
  - `step() -> u32`: `Replayer::step` (:693-706) as a status: NeedMore, Stepped, Done, Failed (`Step`, :608-619), each a
    named constant in the crate doc and in the loader;
  - `hash_lo() -> u32`, `hash_hi() -> u32`: the low and high 32 bits of `Replayer::state_hash()` (:683-685);
  - `dump_len() -> u32`: takes `Replayer::dump()` (:688-690) once and keeps it; `dump_word(i) -> u32`: word i of that
    kept dump (plan silent: a dump per call would rebuild an SS dump of 6,488 words for each of its words:
    sim/tests/replay.rs:321).
  - What each call does before `begin`, at NeedMore, after Done and after Failed, and past the dump's end, is written
    in the crate doc and unit-tested (h3).
- **The third exception** (advisor cond 7 (b), :133-134; plan :688-693; plan.md:442-446; ADR 0002:264-265: "an
  exception needs a written invariant, a test and an `#[allow]` with its reason"): the crate doc names it the spike's
  third `unsafe` exception (tcpinfo/src/lib.rs:11-23's form) and states the written invariant, which each allow's
  reason repeats in short: no exported name collides with another symbol, and every export has integer signatures
  only; and that no `unsafe` block exists. The test is the loader's export list (2.2). What the invariant was checked
  against goes in the doc (U4D2-Q8: on wasm32 the module's other exports are the linker's; natively, the one
  same-named symbol in this container's glibc 2.39 is the compat `step@GLIBC_2.2.5`, which no new link binds: `nm -D`).
- **Not built:** a seed or scene parameter that the header already carries (U4D2-Q3), an export for an order digest
  or for sin (the hash-map digest is read from the dump: 2.7), any other export.

### 2.2 The Node loader (a script in crates/hash-wasm/; plan :670-671, :690-691)
- Reads the `.wasm` file it is given; builds a `WebAssembly.Module`; lists every export with
  `WebAssembly.Module.exports` and refuses, before instantiating, any name not on its list, naming it and exiting
  non-zero (the third exception's test: advisor cond 7 (b); plan :690-691). The list: the seven names of 2.1 and the
  linker's own exports of a wasm32-unknown-unknown cdylib (`memory`, and any other the release build emits, each named
  in your report with what emits it); nothing else.
- Instantiates with an empty import object, so a module that imports anything fails to instantiate (plan silent; a
  LinkError is a refusal): no host clock or random source can reach the Wasm leg.
- Streams a recording in (plan :670-671): the file's bytes as little-endian 32-bit words, the last zero-padded
  (sim/tests/replay.rs:31-43's `words`), fed in chunks with `step` called until NeedMore between them, never the whole
  file before the first step (the driver drains past 64 KiB: replay.rs:701-704). Per tick it writes the recording's
  name, the tick and the 64-bit hash (from `hash_hi` and `hash_lo`) in the line form npt-lab's legs write (2.3), so
  `HashCompare` reads both.
- A trap (`WebAssembly.RuntimeError`: the wasm32 build aborts on a panic, `rustc --print cfg` shows `panic="abort"`) or
  a Failed status ends it non-zero, naming the recording and the last tick stepped. On request it writes the dump's
  words at one tick T (`dump_len`, `dump_word`), for the field diff. A bite comes in as an index with its arming tick.
- The list of names lives in one module that check.mjs imports, or the reverse (plan silent: one home), and
  check.test.mjs pins it (2.4 (d)). Its refusal is a function check.test.mjs can call on a module's bytes (2.4 (d)).

### 2.3 npt-lab: `HashCompare`, the recordings manifest, the `hash` subcommand, carry 66 (crates/lab/)
- **`HashCompare`** (lib.rs; plan.md:617; plan :672-673): takes two per-tick hash lists (each entry a recording, a tick
  and a hash) and returns either "equal over N entries" or the first mismatch (its index, recording, tick and both
  hashes). Lists of different lengths, or an entry whose recording or tick differs from its partner's, are a mismatch
  at the first such index; 0 entries compared is its own result, which no caller passes. It reads no file and knows no
  recording, so U4r's -30 pre-check feeds it two legs' per-tick server hashes (plan :766-767; advisor cond 2, :93-94).
- **The recordings manifest** (plan §4.15 :502-504; carry 52): a committed file in crates/lab/ (plan silent on its home;
  npt-lab regenerates), one entry per recording: its name, scene id, seed, first tick, tick count, span (ms), generator
  version, byte length and SHA-256 (the "script" of :502 is, for a synthetic entry, the generator's other arguments:
  `Recording::synthetic(scene, seed, first_tick, ticks, span_ms)`, replay.rs:367-373). Its three entries are D-2's
  (sim/tests/replay.rs:21-29): SS seed 1 from tick 0, ES seed 2 from 1,000,000, TC seed 3 from u32::MAX - 4,499; 9,000
  ticks each; span 2,000 ms; generator 2 (`Recording::GENERATOR`, replay.rs:288). An entry whose generator is not
  `Recording::GENERATOR` is refused before anything is generated (carry 52: only GENERATOR 2's recordings are pinned).
- **The `hash` subcommand** (release build; plan :670-672; main.rs:186-213 dispatches `probe` alone today):
  1. regenerates each manifest entry with `Recording::synthetic` into a folder under the checkout's .gate/ (never
     committed: §4.15 :502-504), and fails, naming the recording, on any SHA-256 or length that differs;
  2. runs two native legs in two processes (itself re-run through `std::env::current_exe()`, D-2's form:
     sim/tests/replay.rs:144-171), each replaying every recording through `Replayer` and writing its per-tick lines;
  3. compares leg 1 with leg 2 through `HashCompare`, and leg 1 with a Wasm leg's lines when given them;
  4. at a first mismatch, prints the field diff: both sides' dumps at that tick, compared by
     `StateDump::first_difference` (hash.rs:115), with the recording, the tick, the word, the field and both values
     (D-2's form: sim/tests/replay.rs:122-134, :224-238);
  5. takes a bite and its arming tick in the bites build (refused, exit 2, in the measured build), and in the bites
     build prints `World::BITES` in order, which the loader's index is read from (2.1);
  6. reads no clock (check.mjs times the step: :1643-1646) and uses no `HashMap` or `HashSet` (U4D2-Q10).
- **Carry 66** (section 3): probe.rs and main.rs compute SHA-256 of (v1's tag + the v2 fate log's datagram entries)
  through `FateLog::for_each_canonical` (l1/src/lib.rs:351), skipping the v2 tag and keeping the 40-byte entries with
  flow code 0 after v1's tag, as l1/tests/l1.rs:479-505 walks them (there with FNV-1a-64); and write it in the log
  header's params beside `fate_log_sha256` (main.rs:116) and in the summary line (main.rs:167-183), so -3's script,
  which records the line as printed (ops/runs/c-u2-pm-0001-3.mjs:65-72), records it unchanged. The whole-log
  `fate_log_sha256` stays as it is (U4c's v2 values: results/c-u2-pm-0001-3/65869918809f/runs.txt). The param's name
  and its proposed row in marks/SERIES.md's header-params table (:396 holds `fate_log_sha256`'s) go in your report
  for the main session: marks/ is not your lane.
- The command's docs follow (main.rs:1-12; lib.rs:1-6 already names `HashCompare`).

### 2.4 check.mjs and check.test.mjs (plan :674-693; you are their only editor in wave 2: plan :863-864)
Each item has its self-test bite in check.test.mjs (plan :674), and every new deps-step check gets its plant in the deps
fixture test, which fails if a check is unhooked from `depsStep` (check.test.mjs:1094-1165; review C2R-5).
- **(a) The `wasm` step** (plan :675-676), after `deps` in `STEPS` (:1594-1627; plan silent on the place: the flags it
  builds with are checked first), so check.mjs has 11 steps:
  1. the release builds, with no flag added (the build flags are plan.md §4.5:533-535's: baseline x86-64, simd128 and
     relaxed-simd off; wasm-opt is not run here: C-U7-1; by U4D2-Q2), overflow checks staying on for npt-sim in release
     on both targets (Cargo.toml:108-112): npt-lab for x86-64, measured then bites, each artifact copied out before the
     next build replaces it (as ops/runs/c-u2-pm-0001-3.mjs:52-59 does), and npt-hash-wasm for wasm32-unknown-unknown,
     measured and bites; each artifact's path read from cargo's `--message-format=json`, never from a guessed folder
     (`CARGO_TARGET_DIR` is the shared target in a lane and unset in the main folder: section 4 rule 1;
     .cargo/config.toml);
  2. `npt-lab hash`: the manifest's recordings regenerated and checked, the two native legs;
  3. the loader under Node (`process.execPath`), once per recording: a script, so the runner refusal stands
     (check.mjs:1263, :1281) and its note at :1527-1529 is updated to say so (plan :675-676);
  4. the comparison, native against Wasm, and at a mismatch the field diff;
  5. h2's vectors on wasm32;
  6. the bites half, in the bites builds (2.7's PM-0004-21 rows; plan.md §4.5:551 makes the bites part of the check).
  It passes only with 0 mismatching ticks over 27,000 ticks in each pair (leg 1 with leg 2; leg 1 with Wasm: plan
  :694; plan.md §4.5:546-550), h2's vectors equal, and each bite reported as 2.7 says. Its last line (the line main
  prints: :1646) gives the ticks compared per pair, the mismatches, each bite's outcome and each part's wall time; main
  prints the step's seconds (:1645). Budget 5 minutes (plan :676): over it, your report gives the split, cold and warm,
  and the step stays in check.mjs (moving it to ops/runs/det-u4.mjs is U4r's, on the advisor's ok: plan :790-792).
  Self-test bite: through a recorder runner, as the steps pin does (check.test.mjs:1617-1622): the commands it runs, in
  order; a recorded native-against-Wasm mismatch fails it, naming the recording, the tick and the field; a recorded
  SHA-256 mismatch fails it, naming the recording; a recorded trap fails it; a run with 0 ticks compared fails it.
- **(b) The features comparison, like for like** (plan :677-678, §2 :157: "a deps-step line and its bite"; A12;
  plan.md:536; ADR 0004:1184): a deps-step line. npt-sim's resolved features (npt-sim's own, and each package in
  npt-sim's normal-dependency subtree with its enabled features, from `cargo tree --locked -e normal,features` or its
  `--format "{p} [{f}]"`) under the server build (`-p npt-server --target x86_64-unknown-linux-gnu`) against the
  hash-wasm build (`-p npt-hash-wasm --target wasm32-unknown-unknown`), the measured builds together and the bites
  builds (`--features bites`) together; by U4D2-Q6 (a), npt-lab's build (`-p npt-lab`, x86-64: the native legs'
  build) joins as a third. Walk npt-sim's subtree; never grep the whole tree for names (it also meets getrandom and
  cfg-if under other crates: u4d2-brief-notes.md F4). At the head all three builds give npt-sim [], libm
  [arch, default], ppv-lite86 [simd], rand_chacha [], rand_core [], zerocopy [simd] (notes F4: `cargo tree`, offline,
  `--locked`). Any difference fails the deps step, naming the package, the two builds and the features.
  Self-test bite (PM-0004-74's, ADR 0004:1184): recorded outputs where the server build's libm has one extra feature
  fail, naming libm and the feature; the row's control passes ("a dev-only feature on `devtools` that touches no `sim`
  dependency passes": an extra feature on a package outside npt-sim's subtree); and a measured list is never paired
  with a bites list (the pairing pinned).
- **(c) The build-flag check** (A8; advisor cond 7 (a), :131-132; plan :679-683; ADR 0002:256-258), a deps-step line:
  - (i) in the sources the deps step already reads (the environment's `RUSTFLAGS`, `CARGO_ENCODED_RUSTFLAGS`,
    `CARGO_BUILD_RUSTFLAGS`, `CARGO_TARGET_<triple>_RUSTFLAGS`, and every cargo config's `rustflags`:
    check.mjs:1228-1284), `+fma`, `target-cpu=native`, `+simd128` and `+relaxed-simd` are refused in every form rustc
    reads (`-C target-feature=+fma`, `-Ctarget-feature=+fma`, `--codegen target-feature=+fma`, a comma list holding
    one, `-C target-cpu=native`), split across two tokens as cargoConfigProblems splits them (:1259, :1270) or not;
  - (ii) the read-back: `rustc --print cfg --target <t>` for x86_64-unknown-linux-gnu and wasm32-unknown-unknown, given,
    by U4D2-Q4 (a), every rustflag the deps step read from every source (a superset of the one source cargo uses:
    https://doc.rust-lang.org/cargo/reference/config.html, build.rustflags), failing the step on `target_feature="fma"`
    for x86-64 and on `target_feature="simd128"` or `"relaxed-simd"` for wasm32, never only reporting (advisor
    :131-132). At the head the read-back gives fxsr, sse, sse2 for x86-64 and bulk-memory, multivalue,
    mutable-globals, nontrapping-fptoint, reference-types, sign-ext for wasm32 (notes F1). A read-back that cannot run
    fails the step (stop and report if rustc cannot be run from check.mjs at all).
  Self-test bites: each of the four flags in each form and source refused, by name; recorded read-backs listing
  `target_feature="fma"` (x86-64), `"simd128"` and `"relaxed-simd"` (wasm32) each fail; a clean read-back passes; a
  read-back that exits non-zero fails.
- **(d) hash-wasm's rules** (A13; plan :684-687), deps-step lines:
  - its manifest: its own `[lints]` tables, unsafe_code `"deny"`, the rest the workspace's key for key
    (`exceptionLintProblems` through `EXCEPTION_LEVELS`: check.mjs:1023-1052, :171);
  - its sources (`unsafeProblems`, :489-526, gains hash-wasm's rules in npt-tcpinfo's pattern, :382-452): each listed
    export carries exactly one `#[allow(unsafe_code, reason = "...")]` (`PLACED_ALLOW`, `isPlacedAllow`: :209-216)
    directly on it; every `unsafe` in the crate is the `unsafe` of a `#[unsafe(no_mangle)]` on a listed export; any
    other `unsafe` (a block, an unsafe fn, impl or trait, an `unsafe extern` block, `#[unsafe(export_name = ...)]` or
    `#[unsafe(link_section = ...)]`) and `global_asm!` are refused by file:line, and so are an allow anywhere else, an
    inner allow, `cfg_attr`, `expect` or `warn`, a listed export with no allow or two, and `unsafe_code` named outside
    an attribute. `itemAfter` (:301-312) and `fnBodies` (:364-380) read only `pub fn`, so `pub extern "C" fn` needs
    reading (in the mask the `"C"` is blanked: :237-241). By U4D2-Q7 (a), each listed export's parameters and result
    hold u32 and i32 only;
  - its dependencies: npt-sim its only shipped one, and nothing depends on it in any kind (npt-tcpinfo's rule,
    :107-121; npt-lab's, :152-157);
  - it joins `LEAF` (:31), so `WASM_CRATES` (:1438) and the clippy-wasm32 and wasm32 doc steps take it (plan :686).
  Self-test bites: a stray `unsafe {}` block in hash-wasm (plan :699); an `#[unsafe(no_mangle)]` export not on the list,
  or a listed one with no allow; an allow on a helper; a second allow; by Q7 (a), an export taking an f64; hash-wasm
  depending on another crate; another crate depending on hash-wasm; its manifest at `"allow"` or `"forbid"`: each
  refused by name, the clean fixture passing. And the loader: its list pinned to the seven names and the linker's;
  a hand-assembled module (a few dozen bytes, built in the test) with one export more is refused by name (plan :699:
  "an export not on the loader's list"), and one that imports a function fails to instantiate.
- **(e) The third exception** (advisor cond 7 (c), :135-136; plan :688-693): `UNSAFE_EXCEPTIONS` (:169) gains
  `"crates/hash-wasm/": "npt-hash-wasm"` and `EXCEPTION_LEVELS` (:171) `"deny"`. The texts that say two become three
  (:161-168; :204-206; :1509-1510; `DEPS_CHECKED`, :1531-1550, which also names every new check; the header, :1-18;
  Cargo.toml:73-92, in 2.5), and the tests that pin two follow (for example the deps fixture's line "only
  npt-client-wasm and npt-tcpinfo may", check.test.mjs:1141). plan.md §4.2 already says three, hash-wasm named
  (plan.md:435-447), and plan.md §4.1 has its layout line (:392-393): those two plan.md edits (plan :687, :692-693)
  are done.
- **(f) Carry 65** (section 3; j-r2 B2.3, :158-160): the test step names every failing test, from libtest's `failures:`
  list, beside the summary, in the lines main prints under a FAIL (today `run` joins stdout before stderr, :1402, and
  main prints the last 40 lines, :1649); its self-test feeds a recorded failing run and asserts the name is printed.
  If the main session made this change sooner (7.2 item 6), you list carry 65 as met by its commit and do not redo it.
- **(g)** check.test.mjs's steps pin (:1617-1658) gains the `wasm` step's commands; its imports (:12-44) gain what the
  new tests call.

### 2.5 The workspace manifest, the lock, and the other manifests
- Cargo.toml's `members` (:6-23) gains `"crates/hash-wasm"`. No `[workspace.dependencies]` entry for it: nothing
  depends on it.
- Cargo.toml's lints comment (:73-92: "Two crates are ADR 0002:264-265's exceptions") names the third, in its form.
- Carry 29 (section 3): the libm comment (Cargo.toml:50-53) is reworded at this edit.
- Cargo.lock changes by npt-hash-wasm's own `[[package]]` entry only (plan :509: "Cargo.lock changed only by the
  dependencies named"): one cargo call without `--locked`, offline, its command and the lock's diff in your notes;
  every other call keeps `--locked`.
- crates/lab/Cargo.toml: no change (npt-lab already has serde_json and sha2: lab/Cargo.toml). crates/lab/clippy.toml:
  new, by U4D2-Q10 (a) (the clock ban as server/clippy.toml:1-13 words it, with the `HashMap` and `HashSet` type bans).
- crates/proto/Cargo.toml: carry 41's comment lines (:13-14) only, by U4D2-Q9 (a); no feature or dependency change.
- Any other manifest change stops that part (1.5).

### 2.6 Tests (plan :694-695), each named in your report with its file
- **h1, the determinism check** (plan :694; plan.md §4.5:546-550): the `wasm` step on the measured builds: 27,000
  ticks (3 × 9,000), leg 1 with leg 2 and leg 1 with Wasm, 0 mismatching ticks, the counts printed. It is the judged
  release-build check D-2 stands in for in the debug suite (plan :612-614); D-2 stays as merged.
- **h2, PM-0004-21's cross-target vectors** (T22; plan :694-695; ADR 0004:1142: "the picked function gives the same
  digest for a committed byte string on native x86-64 and on wasm32, and one committed state serializes to the same
  bytes on both"; plan silent on which: one state's dump is both the byte string and the state, its digest the state
  hash, `StateDump::hash`, hash.rs:98, as D-7 asserts: sim/tests/replay.rs:329-330): CO's initial world
  (`World::new(&Scene::course(), 1, 0)`, D-7's form, :326-337: 260 words) committed as a vector file in
  crates/hash-wasm/ with its digest. (i) A hash-wasm unit test: natively, the dump's words equal the file word for word
  and its hash equals the committed digest. (ii) In the `wasm` step: on wasm32, through a header-only recording (0
  ticks: `Replayer::step` builds the world and returns Done, replay.rs:709-739), CO's dump equals the file and its hash
  the digest; and every scene's initial dump and hash are equal natively and on wasm32 (equality, so D-7's pins keep
  their one home in sim/tests/replay.rs:320-325). Its shown failure: U4D2-Q11.
- **h3, the exports** (native unit tests in hash-wasm's src): over a short synthetic recording (TC, 60 ticks, as
  sim/tests/replay.rs:514-515 uses), the exports' per-tick hashes and dumps equal `Replayer`'s own; `begin` refuses an
  index outside `World::BITES` and, in the measured build, any bite; each call's status before `begin`, at NeedMore,
  after Done and after Failed; `dump_word` past `dump_len`; in the bites build, an armed hash-map index gives the world
  `Replayer::with_bite` builds (its `bite_order_digest` set from the arming tick on: world.rs:968-973).
- **h4, `HashCompare`**: equal lists; the first mismatch at an index above 0; lists of different lengths; a tick out of
  step; 0 entries compared reported as such, never as equal.
- **h5, the manifest**: a unit test on a short entry generated in the test: its SHA-256 recorded and re-checked; one
  changed byte fails, naming the entry; a generator other than `Recording::GENERATOR` refused before generating. The
  three real entries are checked by h1's run; your report gives each regenerated file's length and FNV-1a-64 beside
  D-7's pins (sim/tests/replay.rs:382-389: 16,694,921 and 35974458070191737; 16,700,849 and 15077317756489230056;
  799,693 and 9126256229328065459).
- **h6, the `hash` subcommand's arguments** (main.rs's test form, :215-275): usage errors exit 2; a bite in the measured
  build refused.
- **h7, carry 66**: a probe unit test: the new SHA-256 equals one the test computes from `for_each_canonical`'s
  datagram entries after v1's tag (l1/tests/l1.rs:479-505's walk, with SHA-256); and the three probe runs with -3's
  arguments (P2, B1 and P4, seed 1, the default packet count and rate: main.rs:73-75; the -3 script's call, :65-72), on
  the release build, run directly with `npt-lab probe` (the -3 script itself refuses a dirty tree, :43-45, and reads its
  binary from the main folder's target, :55), each give U2's value (section 3's note on carry 66).
- **h8, check.test.mjs**: every self-test of 2.4, each with its bite and its clean case, and the deps fixture test's
  new plants.
- **Plan silent (not built):** a third PM-0004-21 injection (ADR 0004:1142's "one presentation random draw inside the
  sim"; plan.md §4.5:551 names two); PM-0004-17's two-instances check (ADR 0004:1138); a Chromium run; wasm-opt. Say
  in your report that each is not built, and why (the plan's line).

### 2.7 Bites (plan :696-700): which are unit-shown here, which are shown by a run, which are only planted
U4d2 plants no new compiled-in bite: its crates spell no bite id, and hash-wasm's `bites` feature forwards to npt-sim's
(plan :670). PM-0004-21's two bites are U4d's, compiled into npt-sim's bites build and chosen per instance through
`Replayer::with_bite` (world.rs:585-606, :934-966); a cross-target catch cannot be a cargo test, so they are shown by
the `wasm` step's runs. The rest are check.mjs's rules, unit-shown by check.test.mjs's self-tests, some also planted
live once to show the real tree is caught. A scratch plant is made in place, one at a time, under `flock -x`, the
file restored and its sha256 checked against the clean file, the run's output line in your report (section 4).

| Bite | Its site | Shown in U4d2 by | Planted live |
|---|---|---|---|
| PM-0004-21/hash-map (U4d's) | the World's arming tick: a std `HashMap` of 64 keys, its order folded into a state word (world.rs:608-623) | a run, the `wasm` step's bites half: SS seed 1 armed at tick 1,234, D-2's form (sim/tests/replay.rs:284-309). Leg 1 against leg 2 report recording 0, tick 1,234, field `bite[0].hash_map_order` (or, with equal order digests, "not exercised" and no tick differing: plan :476); leg 1 against Wasm the same, the Wasm leg's digest read from its dump at the arming tick (plan silent: no export added); every leg's digest printed. "Caught, or recorded as not catchable on wasm32" (plan :698): the report says which, with both digests | no (compiled in) |
| PM-0004-21/std-sin (U4d's) | the vehicles' loops: `f32::sin` for `libm::sinf` from the arming tick (world.rs:626-635, :994-1000, :1216-1228) | by U4D2-Q5 (a): first the scan, a run of the bitten World's legs, native and Wasm, over each recording whose scene has vehicles, armed at its first tick: each loop's period is at most `ticks_for_ms(60,000 + 5,000 × 14)`, 3,900 ticks at 30 Hz (world.rs:794), under 9,000, so every loop input `TAU × k / period` is met; the bitten World against the clean one on each target is reported beside it. If native and Wasm differ: the `wasm` step's bites half arms std-sin on that recording and reports the first differing tick and its field. If they never differ: recorded as not catchable at this site on these targets (ADR 0004:1142) and the stand-in planted (next column) | only if they never differ: a scratch plant in place in world.rs's `std_sin`, its result 1 ulp up under `#[cfg(target_arch = "wasm32")]`, one bites run caught at its arming tick with its field, the file restored and sha256-checked (U4D2-Q5 (a) puts this one plant outside your lane) |
| PM-0004-74, an extra feature on a shared dependency | the features comparison | unit: 2.4 (b)'s self-test (recorded `cargo tree` outputs), with ADR 0004:1184's control | no (a live plant needs a manifest and lock change outside your lane) |
| each flag: `+fma`, `target-cpu=native`, `+simd128`, `+relaxed-simd` | the build-flag check | unit: 2.4 (c)'s self-tests, each source and form, and the read-back's three features | one live run of the deps step with `RUSTFLAGS="-C target-feature=+fma"` in its environment (no file changed): FAIL, by name |
| a stray `unsafe` block in hash-wasm | hash-wasm's rules | unit: 2.4 (d)'s self-test | a scratch plant in place (`unsafe {}` in hash-wasm's lib.rs): the deps step names file:line, and rustc refuses it under the crate's deny |
| an export not on the loader's list | the loader | unit: 2.4 (d)'s hand-assembled module | a scratch plant in place: an eighth `#[unsafe(no_mangle)]` export with its allow: the loader refuses it by name (and the deps scan refuses the allow on an unlisted export) |
| carry 65's unnamed failure | the test step | unit: 2.4 (f)'s self-test | no |
| h2's vectors | the committed vector file; by Q11 (b) also npt-sim's serializer | by U4D2-Q11 (b): two scratch plants in place, each alone: one word of the vector file changed (h2's unit test and the step's vector check fail, naming the word); a `usize`-width length written into the state (world.rs's `dump`, :1018, through hash.rs's word sink, :14; ADR 0004:1142's word-size bite: the dumps' lengths differ between the targets, so the step's equality check and the 27,000-tick comparison fail at their first tick) | the same two plants, each restored and sha256-checked |

### 2.8 The seam rule, and what stops the build
- No item of npt-sim changes: `Replayer`, `Recording` and its `GENERATOR` and `synthetic`, `Step`, `StateDump`,
  `FieldDiff`, `World` and its bites, `Scene`, the seam (replay.rs, world.rs, hash.rs, lib.rs, seam.rs as merged).
  crates/sim, crates/room, crates/client, crates/server, crates/l1, crates/log, crates/judge, marks/ and docs/ are not
  your files; crates/proto only as 2.5 says; a scratch plant of 2.7 is restored before anything else runs. A needed
  change stops that part: write it in your notes and report (the item, the change, why, the site) and build the rest;
  it goes to the advisor through the main session; never a workaround, never a local copy of an npt-sim item.
- Your work moves no synthetic recording and no World outcome on one, so carries 67, 71 and 72 (3.2) do not bind it;
  if a stop is ever lifted into a crates/sim change, they bind whoever makes it.
- A tick count or rate-derived constant: none is foreseen (the arming tick and the recordings' ticks are data). If one
  is needed, no local form; carry 39's route (carries :169-179; advisor :450-458), the need in your report.

### 2.9 Public items (A19; plan §7.2 :916-917)
- npt-hash-wasm: the seven exports (plan §4.1 :202: "cdylib; integer-only exports"), nothing else pub.
- npt-lab's library: `HashCompare` (plan.md:617) and the types its signature needs, listed by name as beyond §4.1's
  npt-lab row (:198, which names the subcommands), for engine-architect; the `hash` subcommand (:198).
- Everything else `pub(crate)` (plan.md:454). Your report lists every pub item you add, by crate, and the loader's
  exported names (its list and its refusal function) and check.mjs's new exports; engine-architect checks them
  (check.mjs does not: :1502-1506, plan :916-917).

---

## 3. The carries

### 3.1 Word for word
Quoted from carries :407-446, the file's section "U4d2 or the main session" as it stands (the advisor's U4s ruling,
condition 4: advisor :334-335; the card's "carries word for word from u4s-carries.md", :158-160). The main session
copies the block by script at launch and checks it by a `diff` of the two line ranges (7.2 item 3). Your report says
of each, by number, "met" (file:line and the test that shows it), "not met" (why) or "not applicable" (why).

```
<<P1: u4s-carries.md lines 407-446 (the heading "## U4d2 or the main session (the workspace Cargo.toml,
crates/proto/Cargo.toml and check.mjs; wave 2 or later)" and carries 29, 41, 42, 52, 65 and 66), copied by script>>
```

Six carries (29, 41, 42, 52, 65, 66). Notes for the builder, which change no word:
- carry 29: at this edit of the workspace Cargo.toml (2.5); its facts are the carry's own (the rust review's reading of
  libm 0.2.16); your report quotes the comment as reworded. Its cite holds: the comment is Cargo.toml:50-53.
- carry 41: by U4D2-Q9 (a), the two comment lines (crates/proto/Cargo.toml:13-14, which still say the grid's bites are
  chosen through its config) and nothing else; under (b) or (c) your report gives it as not met (not your lane).
- carry 42: not applicable. Entry 34 (carries :62-69) was met by U4a: its record lists the four fixes
  (u4a-build.md:125-126), the card at the time recorded "carries 1-5 and 34 met" (history.md:1335), and its reviews
  agree (u4ab-review-test.md:90; u4ab-review-rust.md:27). So carry 42's condition does not hold and this brief carries
  no word of entry 34.
- carry 52: `<base>` holds U4d-F's adb1bab (1.1); the manifest pins only recordings that `Recording::GENERATOR` 2 writes
  (replay.rs:288; 2.3), and ES's only from U4d-F's scene (the ES entry is regenerated by the code at `<base>`, which
  holds U4d-F); your report names adb1bab, "U4d-F: the estimate scene fix (npt-sim), merged after review round 2".
- carry 65: 2.4 (f). Its cites hold at the head (check.mjs:1402, :1649). If the main session made the change sooner
  (7.2 item 6), report it met by that commit.
- carry 66: 2.3 and h7. "U2's committed 913db8e9…, feabf2f0… and b8a04f2f…" are U2's v1 fate-log SHA-256s for P2, B1
  and P4 seed 1, committed as 16-hex prefixes in results/c-u2-pm-0001-3/81c6d01e371b/runs.txt:16, :34 and :28; the
  full values, in U2's raw logs (.gate/netcode-proof-test/runs/c-u2-pm-0001-3-81c6d01e371b/l1-p2-s1.jsonl,
  l1-b1-s1.jsonl and l1-p4-s1.jsonl, header `fate_log_sha256`; read only), are:
  P2 913db8e914bdb773f6f19ac404c7098f711798790fcd1a861502502764b11ecc,
  B1 feabf2f02d023fa9784b2f9630ec6e2ceeed0bcdd28507c203b0ba40de1c25bf,
  P4 b8a04f2f8ddc0d39624d853730636c50ffb354ddb187938066fabc156c1f602b.
  Each new value equals its full value (and so its committed prefix). Its other cites hold (l1/src/lib.rs:351;
  l1/tests/l1.rs:481-532 holds the walk and the test; ops/runs/c-u2-pm-0001-3.mjs).

### 3.2 The recordings section (carries :351-379: entries 67, 71 and 72): not U4d2's
These bind "any later change to npt-sim's synthetic recordings, or to the World's outcome on them". U4d2's work, as
planned here, is not such a change: it edits no crates/sim file (2.8), regenerates each recording with
`Recording::synthetic` as merged (generator 2) and pins the bytes D-7 already pins by length and FNV-1a-64
(sim/tests/replay.rs:382-389), and replays them unchanged; D-7's ES digest and t_m1's line do not move. The scratch
plants of 2.7 change no committed byte and are restored before the next run. So 67 (RM), 71 (3b's switch) and 72
(a_m1's non-vacuity) are not quoted and not met here; your report says "not applicable, no change to the recordings or
the World" with your final `git diff --stat <base> -- spikes/netcode-proof/crates/sim` (empty).

---

## 4. Build rules

**The shared target** (Mustafa's rule, 2026-11-09: plan §7.1 :896-908; the advisor's W1-TARGET, wave1 :558-586),
binding on every cargo, rustc and rustdoc call in your worktree, check.mjs's and the `wasm` step's included:
1. `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target` and `CARGO_NET_OFFLINE=true` on every call (the
   runner sets both: u4e-run.sh:16-17). Checked once after the first build: the artifacts are in that folder, and no
   `target/` appears in your worktree (advisor nit (g), :169-171).
2. **The lock** (wave1 :570-573): every such call runs under `flock` on
   `/home/user/Shamble-Frontlines/.gate/shared-target/.lane.lock` (the runner takes it): shared (`-s`) for an ordinary
   build; exclusive (`-x`) for a counted run (check.mjs, the `--list` and clippy confirmations), every run of the
   `wasm` step or of `npt-lab hash`, each bite run and each scratch plant's run (a plant in crates/sim makes your
   npt-sim differ from the other lane's, so no other lane may build while it is in place).
3. **Provenance** (wave1 :574-575): a counted run's log shows cargo's "Compiling npt-hash-wasm v0.0.0 (<your
   worktree>/spikes/netcode-proof/crates/hash-wasm)" line, and the same for npt-lab, in each cargo invocation that
   builds them: the `cargo test --locked --workspace --no-run` and `--all-features --no-run` builds inside the same
   exclusive lock before check.mjs (U4e's form: u4e-brief.md:734-742), and the `wasm` step's release builds in its own
   output. Match on the path, not the version. A missing line means "not counted", whatever the count says.
4. Touch before each build: `find <copy>/spikes/netcode-proof/crates -type f -exec touch {} +` (plan :899; the runner
   does it).
5. **Counts are deltas** (plan :900-901, :870-871), on check.mjs's measure (`testBothBuilds`, :1493-1500;
   `cargoCounts`, :1411-1421). The base: `<<P6>>` (measured, bites, self and the step count at `<base>`; at 28a85e7
   they were measured 399, bites 446, self 52 of 52, 10 steps: card :23-25). Each run's count is the base plus the
   tests you added; check.mjs's step count is the base's plus 1 (`wasm`); the self count is the base's plus your new
   check.test.mjs tests. A mismatch means another copy's artifact ran: touch, rebuild, never report that run. `--list`
   (`cargo test --locked --workspace --all-features -- --list`) only names the new tests; it is not a count.
6. **Disk** (wave1 :576-580): `df -h /home/user` before each build, the reading in your notes. At least 4 GB free before
   any wasm32 or release build, which every check.mjs run now is (clippy-wasm32 and doc build wasm32: :1438-1439,
   :1472-1489; the `wasm` step builds release for both targets); at least 2 GB before every other build; below that,
   stop and report; never clean anything (plan :906; the between-waves clean is the main session's: wave1 :578-580).
   Report the shared target's size (`du -sh`) before and after your first `wasm` step run (U4d2's release triples add
   to it: wave1 :576).
7. After the last bite or plant, touch and rebuild before any further claim (plan :902).
8. Cargo's own lock still serializes builds: expected (plan :903). The main folder's cache is not yours (plan :904-905).

**Your files, and only these** (Lanes :863-865; advisor cond 10 (b), :145-146): check.mjs and check.test.mjs;
Cargo.toml (2.5's lines) and Cargo.lock (2.5); crates/hash-wasm/ (new); crates/lab/ (src/; clippy.toml by U4D2-Q10;
its Cargo.toml unchanged); crates/proto/Cargo.toml's two comment lines (by U4D2-Q9). Nothing else: not crates/sim (a
scratch plant of 2.7 aside, restored), crates/room, crates/client, crates/server (U4e's lane), crates/l1, crates/log,
crates/judge, marks/, docs/, the ops/ scripts or results/.
- Files: `<<P3>>` (the advisor's U4d-F ruling, T4 item 1: the card :169-171, from df-ruling :462-464, word for word).
  A scratch plant is made by Edit in place, or its diff saved beside the run's output (j-r2 B6: the card :166-167),
  then restored and its sha256 checked against the clean file.
- Git only in your worktree (`cd <worktree> && git ...`), never `git -C` or `git -c`; commit nothing and push nothing;
  the main session merges, one lane at a time (plan :910-913).
- **The stop clause:** `<<P4>>` (T4 item 2: the card :172-175, from df-ruling :465-468, with `<crate>` read as
  "npt-hash-wasm, npt-lab or check.mjs (its self-tests)"); and, until carry 65 is met, `<<P5>>` (TE2-5's rule: the
  card :162-165, from j-r2 :167-171). Expected failures you can name in advance: none (your plan re-pins nothing of
  npt-sim's); a scratch plant's own run fails by design and is reported as that plant's proof, never as a TE2-5
  failure.
- **The builders' limits** (Mustafa, 2026-11-11): `<<P2>>` (history.md:291-293, word for word).
- .claude/rules/netcode.md binds proto, room, client and server code (plan §7.1 item 8, :907-908); you edit none of
  their sources (crates/proto's manifest comment aside), and npt-lab's `probe` and `hash` read no network input.

---

## 5. End checks

Run from spikes/netcode-proof/ in your worktree, each as a counted run under section 4's rules (`-x`; `df` before each):
1. check.mjs twice, each with every step passing: 10 steps at the head (fmt, clippy, clippy-wasm32, doc, test, deps,
   refs, marks, self, redact: :1594-1627) plus `wasm`, so "check: all 11 steps pass" (:1652); if U4e merges first, the
   count at your merge is the main session's to state. Report both runs' last lines, the test step's two counts, the
   self count, the `deps` and `wasm` lines and seconds, and each run's provenance lines.
2. clippy on the host and on wasm32, in both builds, warnings as errors: check.mjs's clippy and clippy-wasm32 steps
   (`lintBothBuilds`, :1456-1462; `WASM_ARGS`, :1439, which takes npt-hash-wasm through `LEAF`); state each step's line.
3. fmt: check.mjs's fmt step (`cargo fmt --all --check`, :1595); a diff is applied by Edit (`<<P3>>`).
4. The `wasm` step's wall time: cold (after the first touch, nothing built in release yet) and warm, per part (the
   builds, the regeneration, each leg, the comparisons, the bites half), against the 5-minute budget (plan :676).
5. The Wasm file's size: the release `.wasm` of npt-hash-wasm, raw bytes and gzip -9 bytes, measured and bites builds
   (reported, not judged: ADR 0002:264-266 leaves the budget to the spikes; wasm-opt is U7's).

---

## 6. Return list (your report, in this order)

1. `<base>`, the `git merge-base --is-ancestor adb1bab HEAD` result, and your final diff's file list (`git status
   --porcelain` and `git diff --stat <base>`), each file inside section 4's lane; `git diff --stat <base> --
   spikes/netcode-proof/crates/sim` (empty).
2. Each build item of 2.1-2.5: built (file:line) or not (why); the loader's export list as built, with what emits each
   linker export; the manifest's three entries; Cargo.lock's diff.
3. Tests h1-h8 and any you added: name, file, what each shows; the counts in both builds as deltas from `<base>`'s, on
   section 4 rule 5's measure, with the `--list` lines that name the new tests.
4. Each bite of 2.7: its id or rule, its site (file:line), its unit test (name) or its run's output line, each plant's
   output line and its restore's sha256; which bites were unit-shown, which shown by a run, which only planted.
5. The determinism check: the ticks compared per pair (27,000 each), the mismatches (0), the three regenerated files'
   SHA-256, length and FNV-1a-64 beside D-7's, h2's vector and equality results, the wall time split (cold, warm), the
   Wasm file's size.
6. PM-0004-21's two bites: hash-map's digests (each native leg, Wasm) and outcome (caught at its tick and field, or not
   exercised; "not catchable on wasm32" or not); std-sin's scan (each recording, the first differing tick or none, the
   bitten-against-clean result on each target) and outcome under U4D2-Q5's ruling.
7. Carries 29, 41, 42, 52, 65 and 66, by number: met (file:line, test), not met (why) or not applicable (why); carry
   66's three values beside U2's full values, with the commands; carry 52 naming adb1bab; 3.2's finding for 67, 71 and
   72.
8. Every pub item you add, by crate, against §4.1 and plan.md §4.7 (2.9), any beyond them named as such.
9. What stopped and goes to the main session: seam changes; manifest needs; carry 66's param name and its proposed
   SERIES.md row; texts outside your lane that still say "two" exceptions (crates/client-wasm/Cargo.toml:9-13's
   comment: "one of the workspace's two unsafe_code exceptions"); anything the advisor's rulings left open.
10. The end checks (section 5): check.mjs's two runs (each step's line), clippy host and wasm32 lines in both builds,
    fmt; `CARGO_TARGET_DIR` checked after the first build; each `df` reading and the shared target's size before and
    after; each counted run's provenance lines and its `flock -x` use.
11. How each ruling was applied: U4D2-Q1 to Q11 (section 8), and every "plan silent" choice of section 2 as built.
12. What you did not verify, and what is left.
13. Who built it (the advisor's U4S-BUILDER terms, advisor :365-371).

---

## 7. The facts this brief rests on, and what is left for the main session

### 7.1 The facts, read at the head (recheck the rows marked at launch)

| Row | Fact | Where | Use | Recheck at launch |
|---|---|---|---|---|
| R1 | `Replayer`: `new`, bites-only `with_bite(&str, arm_tick)`, `feed(u32)`, `step() -> Step`, `state_hash`, `dump`, `world`; scene, seed, first tick, TICK_HZ and pools come from the recording's header | replay.rs:8-15, :113-140, :608-770 | 2.1, Q3 | if crates/sim changed |
| R2 | `Recording::GENERATOR` is 2 (U4d-F) | replay.rs:285-288 | carry 52, 2.3 | yes |
| R3 | `Recording::synthetic(scene, seed, first_tick, ticks, span_ms)` | replay.rs:367-373 | the manifest | if crates/sim changed |
| R4 | The three recordings and their pins: SS 1 from 0, ES 2 from 1,000,000, TC 3 from u32::MAX - 4,499, 9,000 ticks, 2,000 ms; 16,694,921 / 16,700,849 / 799,693 B with their FNV-1a-64 | sim/tests/replay.rs:21-29, :382-389 | h1, h5 | if crates/sim changed |
| R5 | `World::BITES` in order: hash-map, std-sin, server-speed-1pct, server-quant-differs; `with_bite` takes the arming tick only | world.rs:934-966 | 2.1's index, Q5 | if crates/sim changed |
| R6 | The std-sin site: the vehicles' loops, periods `ticks_for_ms(60,000 + 5,000 v)` | world.rs:794, :994-1000, :1216-1228 | 2.7, Q5 | if crates/sim changed |
| R7 | A 0-tick recording builds the World and returns Done | replay.rs:709-739 | h2 | if crates/sim changed |
| R8 | Release keeps overflow checks for npt-sim and npt-proto | Cargo.toml:108-112 | 2.4 (a) | no |
| R9 | `rustc --print cfg` (1.99.1): x86-64 fxsr, sse, sse2; wasm32 bulk-memory, multivalue, mutable-globals, nontrapping-fptoint, reference-types, sign-ext; wasm32 `panic="abort"` | notes F1 | 2.4 (c), 2.2 | no |
| R10 | `cargo rustc --print` is unstable on cargo 1.99.1 (refused before any build) | notes F2 (cargo issue 9357) | Q4 | no |
| R11 | npt-sim's subtree features equal in npt-server, npt-lab (x86-64) and npt-sim (wasm32) | notes F4 | 2.4 (b) | yes |
| R12 | glibc 2.39's only same-named symbol is the compat `step@GLIBC_2.2.5` | notes F5 | 2.1, Q8 | no |
| R13 | Node v22.23.4; no wasm-opt installed | notes F6 | 2.2; C-U7-1 | no |
| R14 | check.mjs: 10 steps (:1594-1627); `depsStep` (:1563-1590) and its fixture test (check.test.mjs:1094-1165); the steps pin (:1617-1658); section 0's moved cites | check.mjs, check.test.mjs | 2.4 | yes (U4e edits neither) |
| R15 | plan.md §4.1 and §4.2 already hold hash-wasm's line and "three named exceptions" | plan.md:392-393, :435-447 | 2.4 (e) | no |
| R16 | No crates/lab/clippy.toml; server/clippy.toml has the clock ban, no hash-map ban | `ls crates/*/clippy.toml`; server/clippy.toml:1-13 | Q10 | no |
| R17 | U4e's patch touches crates/room, crates/client, crates/server only | wip/u4e.patch | 1.2 | yes (merged or not) |
| R18 | U2's full v1 fate-log SHA-256s and their committed prefixes | 3.1's note on carry 66 | h7 | no |
| R19 | Carry 65 not yet made: `run` joins stdout before stderr, main prints 40 lines | check.mjs:1402, :1649 | 2.4 (f) | yes |
| R20 | The base counts at 28a85e7: measured 399, bites 446, self 52 of 52, 10 steps | card :23-25 | section 4 rule 5 | yes (`<<P6>>`) |
| R21 | npt-lab has serde_json and sha2; it dispatches `probe` alone | lab/Cargo.toml; main.rs:186-213 | 2.3, 2.5 | no |
| R22 | The -3 script records each probe line as printed and reads its binary from the main folder's target | ops/runs/c-u2-pm-0001-3.mjs:43-45, :52-59, :65-72 | carry 66, h7 | no |

### 7.2 Left for the main session at launch
1. **`<base>` and its counts (`<<P6>>`).** The spike branch's head. If spikes/ at `<base>` differs from 28a85e7
   (`cd /home/user/Shamble-Frontlines && git diff --stat 28a85e7 <base> -- spikes/netcode-proof`), give its counts from
   a counted run in the main folder on its own cache (main-run.sh, as U4e's 7.2 item 1: the test step's two counts,
   self's "K of K", the step count); if not, they are 399, 446, 52 of 52 and 10 steps (card :23-25).
2. **The runner (`<<P7>>`).** u4d2-run.sh: u4e-run.sh with the log check's folder `.gate/u4d2/` (u4e-run.sh:30-32) and
   its header naming U4d2; nothing else changes.
3. **The placeholders, by script, each checked after the copy** (the old text found once, the new text in place):
   `<<P1>>` carries :407-446 (and a `diff` of the two line ranges, empty); `<<P2>>` history.md:291-293; `<<P3>>` the
   card :169-171; `<<P4>>` the card :172-175 with `<crate>` read as "npt-hash-wasm, npt-lab or check.mjs (its
   self-tests)" (said beside it, as the T4 item asks: "with the unit's crate named"); `<<P5>>` the card :162-165;
   `<<P6>>`; `<<P7>>`.
4. **The advisor's rulings on U4D2-Q1 to Q11** (section 8), folded by script into the places marked "by U4D2-Qn"; any
   plan text a ruling allows (Q1's cites, Q2's "§4.5", Q3's `begin`, Q6's comparison line, Q8's names, Q10's
   clippy.toml line, Q11's bite line) in the main session's own commit, as the U4e brief ruling's L2 did.
5. **U4e.** If it merged before launch, `<base>` holds it and item 1's counts include it; else the two lanes run side by
   side (e-ruling L4) and merge one at a time, the second merge recomputing the counts (plan :870-871).
6. **Carry 65.** If the main session makes it before launch (j-r2 :158-160), name its commit here and in 2.4 (f).
7. **This brief's checks:** test-engineer's check of rev 1 (u4d2-brief-check.md), the drafter's rev 1.1, then the
   advisor's check before the build (advisor :157).

---

## 8. Questions for the advisor (U4D2-Q1 to Q11)

**U4D2-Q1 (the plan's check.mjs cites have moved).** Plan §5 U4d2 cites check.mjs:25, :1110, :135-137, :94-102,
:903-942, :1156-1158 and :135; each has moved (section 0's table).
- Why it matters: a builder reading the plan's lines lands on other code; the brief and the plan then disagree.
- Options: (a) the brief's table is the reading, and the plan text stays as approved (the U4e brief's way with moved
  cites: u4e-brief.md:127-130); (b) the main session re-points the plan's cites in its own commit.
- Recommendation: (a): the plan is a record approved at a hash; the table names every moved line, and check.mjs moves
  again with this very unit.

**U4D2-Q2 ("§4.5's flags").** Plan :675's "the release build with §4.5's flags" names a section that, in this plan,
is "Baselines, groups and the window" (plan :292-304) and holds no flag.
- Why it matters: the step's builds must use, and the flag check must hold, a named set of flags.
- Options: (a) it is plan.md §4.5's build flags (plan.md:533-535: baseline x86-64, no `+fma`, no `target-cpu=native`, no
  relaxed-simd, simd128 off, wasm-opt without `--fast-math`), wasm-opt being U7's (C-U7-1); the plan's text reads
  "plan.md §4.5's"; (b) another set, named by the advisor.
- Recommendation: (a): plan.md §4.5 is the section the rest of the row's checks come from (plan RQ4, :117).

**U4D2-Q3 (`begin`'s seed and scene).** Plan :668 has `begin(seed_lo, seed_hi, scene, bite, arm_tick)`, but
`Replayer` takes no seed or scene: it reads both from the recording's header (replay.rs:8-15, :709-736), and `World`
has no seed accessor (world.rs's public items).
- Why it matters: an argument the wrapper cannot use is either unchecked (a signature that says what it does not do)
  or checked by a second reader of the header's layout, whose one home is replay.rs.
- Options: (a) keep the five; the wrapper compares scene and seed with the header words it is fed (words 2 to 4) and
  `step` fails on a difference; (b) `begin(bite, arm_tick)`: the header carries scene and seed, and the manifest's
  SHA-256 check binds each file to its entry before a word is fed (2.3); plan :668 edited by the main session; (c)
  keep the five, unused.
- Recommendation: (b): it keeps one reader of the header and loses no check (the SHA-256 is stricter than two fields).

**U4D2-Q4 (the read-back's flags).** Plan :680-682 reads the effective target features "with `rustc --print cfg` per
target". A bare `rustc --print cfg` sees no flag cargo would add from the environment or a config, and `cargo rustc
--print cfg`, which would, is refused by stable cargo 1.99.1 as unstable (notes F2).
- Why it matters: as worded, the read-back can only fail on a toolchain change, so the "never only reports" check
  (advisor cond 7 (a)) would never meet a flag set in `RUSTFLAGS` or a config.
- Options: (a) `rustc --print cfg --target <t>` given every rustflag the deps step read from every source (a superset
  of the one source cargo uses: build.rustflags's four sources, first one used); fail-closed; (b) reproduce cargo's
  choice of source (CARGO_ENCODED_RUSTFLAGS, then RUSTFLAGS, then the matching target tables joined, then
  build.rustflags), which needs `cfg(...)` table matching; (c) the bare form, as worded.
- Recommendation: (a): it fails on any flag anywhere that enables a refused feature (a `target-cpu=haswell` the name
  scan cannot see included), with no cargo logic to get wrong.

**U4D2-Q5 (the std-sin bite: its input and its stand-in).** Plan :696-697 takes the bite's input "from a scan for
inputs where glibc's `sinf` and the wasm32 builtin differ; if none differs, recorded as not catchable on these
targets and a 1-ulp planted change stands in" (ADR 0004:1142: "the sin input taken from PM-0004-22's table"). U4d's
site takes its inputs from the vehicles' loops (world.rs:1216-1228), and `World::with_bite` takes only an arming tick
(:944-966): U4d2 cannot choose the input. The stand-in needs a change that differs by target inside npt-sim, which is
not U4d2's lane (crates/sim has no wave-2 owner: close :117-118).
- Why it matters: without a reading, the bite is either unshowable as worded or needs a crates/sim edit.
- Options: (a) the scan is a run of the bitten World's legs, native and Wasm, from each recording's first tick: the
  loops' periods are at most 3,900 ticks (world.rs:794), so every loop input is met within a recording; if native and
  Wasm differ, the step's bites half arms std-sin there and reports its first differing tick and field; if they never
  differ, "not catchable at this site on these targets" is recorded and the stand-in is shown once as a scratch plant
  in place in world.rs's `std_sin` (1 ulp up under `#[cfg(target_arch = "wasm32")]`), caught at its arming tick,
  restored and sha256-checked, never committed; (b) as (a), but the stand-in is a committed bite (a new `World` bite
  id and site, by the main session in its own commit: a crates/sim change, so carries 67-72 are checked for it, and a
  new pub-items line); (c) record "not catchable" and show no stand-in.
- Recommendation: (a): it shows the step catches a 1-ulp sin difference at its tick, changes no committed npt-sim byte
  (so no recording or outcome moves), and leaves a committed injection to the sim core spike, where PM-0004-21 is
  judged.

**U4D2-Q6 (which builds the features comparison compares).** Plan :677-678 compares "npt-sim's resolved features under
the server build against the hash-wasm build". The native legs of the determinism check run in npt-lab's build (plan
:670-671), which that comparison does not cover; ADR 0004:1184 asks it of "every build that runs the sim", and ADR
0002:501 runs the replay "built as the server builds the sim".
- Why it matters: the Wasm leg is compared with npt-lab's native legs, so npt-lab's feature set is the one that has to
  equal the Wasm build's for the 27,000-tick result to mean "the server's sim".
- Options: (a) three builds, like for like: npt-server, npt-lab (x86-64) and npt-hash-wasm (wasm32), measured with
  measured and bites with bites; (b) the plan's two.
- Recommendation: (a): one more `cargo tree` pair, equal today (notes F4); npt-client-wasm joins at U7.

**U4D2-Q7 (the invariant's second clause has no test).** The written invariant has two clauses (plan.md:444-446;
advisor cond 7 (b)): no exported name collides, and every export has integer signatures only. The test the plan names
(the loader's export list, plan :690-691) checks names only.
- Why it matters: ADR 0002:264-265 asks for a test of the exception; a clause no test can fail is a written claim only.
- Options: (a) the deps scan, which already finds the listed exports in the source, refuses any parameter or result
  type but u32 and i32 (what rustc lowers to Wasm's i32); (b) the loader reads the module's type section and refuses an
  export whose types are not i32; (c) as worded, the clause held by review.
- Recommendation: (a): a few lines in the scan the step already needs, a self-test bite (an f64 parameter), and the
  Rust signature is the source of the Wasm one.

**U4D2-Q8 (the export names and "no exported name collides").** The plan names the exports `begin`, `feed`, `step`,
`hash_lo`, `hash_hi`, `dump_len` and `dump_word` (plan :668-669). On wasm32 nothing else exports these; but the native
test build links this container's glibc 2.39, which defines a compat `step@GLIBC_2.2.5` (notes F5).
- Why it matters: the invariant is meant to hold by construction; one name already meets a libc symbol on the host.
- Options: (a) keep the plan's names; the written invariant records what it was checked against (wasm32: only the
  linker's exports; native: the compat, non-default `step@GLIBC_2.2.5`, which no new link binds); (b) a prefix on every
  export (for example `npt_hash_begin`), so no libc name can match; plan :668-669 renamed.
- Recommendation: (a): the native build is never loaded as a library, a non-default version cannot bind, and the
  names stay those U7's check will load (plan :885-886); (b) if the advisor wants the invariant true without the note.

**U4D2-Q9 (which manifest lines are U4d2's).** Carries 29 and 41 sit in the section "U4d2 or the main session (the
workspace Cargo.toml, crates/proto/Cargo.toml and check.mjs; wave 2 or later)" (carries :407), and the advisor's U4s
ruling routed carry 41 to "the next edit of crates/proto/Cargo.toml (U4d2 or later)" (advisor :324-326); Lanes gives
U4d2 "Cargo.toml's members" and does not name crates/proto/Cargo.toml (plan :863-865). The workspace manifest's lints
comment (Cargo.toml:73-92, "Two crates are ... exceptions") must also change with the third exception.
- Why it matters: without a ruling, a builder either edits a file outside its listed lane or leaves two carries and a
  wrong comment behind.
- Options: (a) U4d2's lane covers the workspace Cargo.toml's lines of 2.5 (members, the lints comment, carry 29's libm
  comment) and crates/proto/Cargo.toml's two comment lines (carry 41), comments only, no feature, dependency or lock
  change; (b) members and the lints comment only, carries 29 and 41 to the main session; (c) members only.
- Recommendation: (a): the carries' own section names U4d2 and these files, no other wave-2 lane holds either file,
  and comment lines change nothing cargo resolves.

**U4D2-Q10 (npt-lab's clippy.toml).** §4.11 gives npt-lab a new clippy.toml with the clock ban (plan :393-395), and the
advisor's Q9 ruling, (a), adds the `HashMap` and `HashSet` type bans for lab (plan :1002-1005; advisor :207). None
exists (`ls crates/*/clippy.toml`), and no sub-unit's row builds it. U4d2 is the only wave-2 editor of crates/lab
(plan :865) and adds its first U4 code there, the comparer of a determinism check. (In passing: server/clippy.toml has
the clock ban but not Q9 (a)'s hash-map bans, server/clippy.toml:1-13; not U4d2's lane.)
- Why it matters: `HashCompare` and the native legs must not take an order from a `HashMap` or read a clock, and today
  nothing but review holds that in npt-lab.
- Options: (a) U4d2 adds crates/lab/clippy.toml: the clock ban as server/clippy.toml words it, and the two type bans
  (crates/lab/src holds no `HashMap`, `HashSet`, `Instant` or `SystemTime` today: grep), the clippy steps showing the
  existing probe still lints; (b) U4r adds it with the runner; (c) only §4.11's clock ban, now.
- Recommendation: (a), and server/clippy.toml's missing bans go to the main session as a follow-up for U4e's or a
  later server edit.

**U4D2-Q11 (h2's shown failure, and ADR 0004:1142's word-size bite).** T22 replaced "no usize" by PM-0004-21's
cross-target vectors (plan :82, :694-695). ADR 0004:1142 gives that clause its bite ("a planted `usize` field in the
serialized state must change the bytes between the two targets"), which U4d2's Bites line (:696-700) does not list,
so as planned h2 has no shown failure.
- Why it matters: a vector test never shown failing proves nothing about word size.
- Options: (a) one scratch plant: a word of the committed vector file changed (h2 fails, naming the word); (b) as (a),
  and a second scratch plant in place in npt-sim's serializer (world.rs's `dump`, :1018, through hash.rs's word sink,
  :14: a `usize`-width length written into the state), shown to change the bytes between the targets (the step's
  equality check and the 27,000-tick comparison fail at their first tick), restored and sha256-checked; (c) none.
- Recommendation: (b): the first shows the comparison against the file can fail; only the second shows the check
  catches what T22 exists for; both are scratch, restored, and change no committed npt-sim byte.
