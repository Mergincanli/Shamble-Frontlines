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
   lane or to an npt-sim item (2.9); a failure under the stop clause (`<<P4>>`, with `<<P5>>`); a manifest change beyond
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
     `std_sin` :608-635, `World::BITES` :934-938, `with_bite` :940-966, `bite_order_digest` :968-973, `sinf` :994-1000,
     `place_vehicles` :1215-1228), hash.rs (`StateDump` :66, `FieldDiff` :73-82, `words` :93, `hash` :98, `field` :107,
     `first_difference` :115), lib.rs (:13-57); sim/tests/replay.rs (D-2 :21-309, D-7 :311-390); crates/lab (all four
     src files and Cargo.toml); crates/tcpinfo (Cargo.toml, src/lib.rs:1-40, the two allows in src/linux.rs); l1's
     `FateLog` (l1/src/lib.rs:325-352) and l1/tests/l1.rs:478-532; ops/runs/c-u2-pm-0001-3.mjs; Cargo.toml; and
     check.mjs and check.test.mjs whole (you are their only editor in wave 2: plan :863-864).

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
  which records the line as printed (ops/runs/c-u2-pm-0001-3.mjs:64-72), records it unchanged. The whole-log
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
     relaxed-simd off, no wasm-opt; by U4D2-Q2): npt-lab for x86-64, measured then bites, each artifact copied out
     before the next build replaces it (as ops/runs/c-u2-pm-0001-3.mjs:51-58 does), and npt-hash-wasm for
     wasm32-unknown-unknown, measured and bites; each artifact's path read from cargo's `--message-format=json`, never
     from a guessed folder (`CARGO_TARGET_DIR` is the shared target in a lane and unset in the main folder: section 4
     rule 1; .cargo/config.toml);
  2. `npt-lab hash`: the manifest's recordings regenerated and checked, the two native legs;
  3. the loader under Node (`process.execPath`), once per recording: a script, so the runner refusal stands
     (check.mjs:1263, :1281) and its note at :1527-1529 is updated to say so (plan :675-676);
  4. the comparison, native against Wasm, and at a mismatch the field diff;
  5. h2's vectors on wasm32;
  6. the bites half, in the bites builds (2.7's PM-0004-21 rows).
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
  a hand-assembled module (a few dozen bytes, built in the test) with one export more is refused by name (plan :700:
  "an export not on the loader's list"), and one that imports a function fails to instantiate.
- **(e) The third exception** (advisor cond 7 (c), :135-136; plan :691-693): `UNSAFE_EXCEPTIONS` (:169) gains
  `"crates/hash-wasm/": "npt-hash-wasm"` and `EXCEPTION_LEVELS` (:171) `"deny"`. The texts that say two become three
  (:161-168; :205-206; :1509-1510; `DEPS_CHECKED`, :1531-1550, which also names every new check; the header, :1-18;
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
  arguments (P2, B1 and P4, seed 1, the default packet count and rate: main.rs:73-75; the -3 script's call, :63-72), on
  the release build, run directly with `npt-lab probe` (the -3 script itself refuses a dirty tree, :41-43, and reads its
  binary from the main folder's target, :53), each give U2's value (section 3's note on carry 66).
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
| PM-0004-21/std-sin (U4d's) | the vehicles' loops: `f32::sin` for `libm::sinf` from the arming tick (world.rs:626-635, :994-1000, :1216-1228) | by U4D2-Q5 (a): first the scan (the loops' inputs, `TAU * k / period` for each vehicle's period in SS, ES and TC, on which the bitten World's std `sin` and the clean `libm::sinf` give different bits natively, and the same on wasm32 through a run of the bitten World). If some input differs: a run, the `wasm` step's bites half, armed on the recording and tick the scan names, reporting the first tick where that input meets a loop, with its field. If none differs: recorded as not catchable at this site on these targets (ADR 0004:1142) and the 1-ulp stand-in planted (next column) | only if none differs: a scratch plant in place in world.rs's `std_sin` (`#[cfg(target_arch = "wasm32")]`, its result 1 ulp up), one bites run caught at its arming tick with its field, the file restored and sha256-checked (U4D2-Q5 (a) puts this one plant outside your lane) |
| PM-0004-74, an extra feature on a shared dependency | the features comparison | unit: 2.4 (b)'s self-test (recorded `cargo tree` outputs), with ADR 0004:1184's control | no (a live plant needs a manifest and lock change outside your lane) |
| each flag: `+fma`, `target-cpu=native`, `+simd128`, `+relaxed-simd` | the build-flag check | unit: 2.4 (c)'s self-tests, each source and form, and the read-back's three features | one live run of the deps step with `RUSTFLAGS="-C target-feature=+fma"` in its environment (no file changed): FAIL, by name |
| a stray `unsafe` block in hash-wasm | hash-wasm's rules | unit: 2.4 (d)'s self-test | a scratch plant in place (`unsafe {}` in hash-wasm's lib.rs): the deps step names file:line, and rustc refuses it under the crate's deny |
| an export not on the loader's list | the loader | unit: 2.4 (d)'s hand-assembled module | a scratch plant in place: an eighth `#[unsafe(no_mangle)]` export with its allow: the loader refuses it by name (and the deps scan refuses the allow on an unlisted export) |
| carry 65's unnamed failure | the test step | unit: 2.4 (f)'s self-test | no |
| h2's vectors | the committed vector file | by U4D2-Q11 (a): a scratch plant in place, one word of the vector file changed: h2's unit test and the step's vector check fail, naming the word | the same plant |

### 2.8 The seam rule, and what stops the build
- No item of npt-sim changes: `Replayer`, `Recording` and its `GENERATOR` and `synthetic`, `Step`, `StateDump`,
  `FieldDiff`, `World` and its bites, `Scene`, the seam (replay.rs, world.rs, hash.rs, lib.rs, seam.rs as merged).
  crates/sim, crates/room, crates/client, crates/server, crates/l1, crates/log, crates/judge, marks/ and docs/ are not
  your files; crates/proto only as 2.5 says. A needed change stops that part: write it in your notes and report (the
  item, the change, why, the site) and build the rest; it goes to the advisor through the main session; never a
  workaround, never a local copy of an npt-sim item.
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
