# Plan (draft 3): change-set 4, "Walk", proposed as four change-sets, 4a to 4d

_Written by the main session (the chat Mustafa calls Builder V4, lane 1) on 2026-11-25, for Mustafa and his Advisor.
Base: main 311c0484f7a9697ae33e7a76a8cf0b507a2837c4. Nothing is built, and no file outside `.checkpoints/` is edited
before the advisor agent's APPROVE and his go. His words that set this plan: his takeover block
(`mustafa-2026-11-25-takeover.md`, points 7 and 8), his answer ORDER-3C (c) with four conditions
(`mustafa-2026-11-25-order-3c-answer.md`), his Testlines Export (`testlines-export.json`, 15,567 bytes, sha256
9ad1d9f9...6498) and the entries of `docs/DECISIONS.md` this plan names. Drafted from four designs in this folder,
each ending DONE: `netcode-architect-cs4.md` (the recording, every item named for 3c, the command and the snapshot),
`engine-architect-cs4.md` (the content crate, the simulation's structure, the block, lane 2's list, the page),
`rust-reviewer-cs4.md` (the collision library and its tree, the built-file check, our own movement, the time unit, the
simulation's marks) and `game-designer-cs4.md` (its check of the rules draft `rules-cs4.md`); and from the main
session's `testlines-cs4.md`. Those notes hold every symbol, every file:line and each test with its planted defect;
this plan holds what is decided, in which order, and what he is asked. Every line count is an estimate: no code is
written, and the designs give plus or minus 30 %._

## 0. In one screen
- **Walk is nearly four times its card.** The card (`docs/reports/2026-11-17-build-plan-slice-1.md`,
  lines 250 and 334-358) plans 3,900 lines. The designs, added up from the bottom, come to about **15,300 lines**,
  half of them tests. At the ratio change-sets 3a and 3b ran at (1.7 and 1.8 times their plans), that is about
  **27,500 lines**: nearly four times change-set 3b. Why: the card was written before ORDER-3C put the recording
  in (about 1,365 lines, where change-set 3's plan had 275), before SLICE-FEEL and his Testlines values; its one line
  on content hides a reader, a writer, a schema and three tools; and each pass mark the build plan places at Walk
  (about fifty) needs its own test and planted defect.
- **So this plan proposes four change-sets, each its own pull request with his go, in this order** (question
  WALK-CUT):
  | Part | Name | What he sees or plays | Bottom-up | Planned at 1.8 | Stop line |
  |---|---|---|---|---|---|
  | 4a | The recording | nothing new: the page as 3b left it | 1,365 | 2,450 | 3,050 |
  | 4b | Content and the world | the grey street corner and training yard, drawn from the game's first content files | 5,410 | 9,750 | 12,200 |
  | 4c | The first soldier | **he plays**: walk, sprint, jump, step up, aim with the mouse, the follow camera and its tuning panel | 7,315 | 13,150 | 16,450 |
  | 4d | Sights, and what is built last | hold the right button: the slow aimed walk and the camera's sights mode | 1,210 | 2,200 | 2,750 |
  After 4d, and before change-set 7 starts: **the link simulator's own small change-set** (about 700 bottom-up).
  A stop line is the planned size times 1.25: there the chat stops and comes back to him, whatever is left.
- **His fallback (d) is used, and this is the telling before anything moves:** the recording is too large to ride
  inside the first soldier's stop, so it goes first as its own change-set, 4a. It needs no browser check (nothing a
  player sees changes), and it gives his condition (b) by construction: nothing of movement exists when it merges.
- **The earliest he can play:** an early-look zip inside 4c, as soon as the soldier walks and turns to the mouse on
  screen and the camera panel works (after step 5 of 4c's steps: his words ask for the panel "from its first
  playable build"), before the big test runs and the reviews. It is a look, not a check: nothing is asked of him.
  The judged build comes at 4c's end. By change-set 3b's pace (about 7,000 lines from its first commit to its merge
  in 16 hours, reviews and his check included) the early look is about 50 hours of this chat's work away, and the
  judged soldier about 58, plus the waits for his gos (question WALK-CUT gives both for each way of cutting).
- **No proof gets thinner** (his condition a): section 9 places every mark the build plan puts at Walk in the part
  that merges the code it proves. Every mark on the recording is met on the movement recording before 4c merges, and
  again on the recording made anew before 4d merges, because 4d changes the simulation. Only one mark may move to a
  later part, the one the card itself names for that (the far-corner map, build plan lines 355-356); he is told
  before it moves.
- **Outside code:** the collision library the simulation draft names (parry3d) brings 33 new packages into the
  lockfile, 31 of them in the players' file's tree (section 5.2). The content reader is proposed as our own code, with
  no outside package (question CONTENT-SOURCE).
- **His Export was made at 60 steps a second** (its own warning line): no value is carried before his answer
  (STEP-60, asked in the chat on 2026-11-25).
- **To start 4a he needs to give only:** his go on this plan and on WALK-CUT. The other answers are needed later,
  each by the part named in section 13.

## 1. Goal
After 4a: a recorded match replays to the same per-tick hashes natively (in one process, in two, and two matches
interleaved) and in WebAssembly on the file players get, in Node and in Chromium, and a planted divergence is caught at
its tick. After 4b: the game's numbers and its test scene are content files read by one loader, the same on every
side, with the reader's own proofs; the collision world is built from the map and is identical natively and in
WebAssembly; the built file is checked for a fused multiply-add on every build. After 4c: one soldier, moved by
commands through the real messages on our own movement over the collision library, drawn between two ticks,
followed by the camera; the movement recording replays identically on every target, and a paced replay in the real
Worker shows that the picture never reaches the simulation. After 4d: aiming down sights by SLICE-FEEL on a
recording made anew, the development reload of content, the old tuning's import report and the frame cap's
readings: with it every mark the build plan places at Walk is met.

## 2. What Mustafa sees and plays after each part
- **4a:** nothing new. The zip of 3b still runs; one number more can be read from the players' file by a replay.
- **4b:** the page shows the street corner (two houses, an alley, a cover wall, a street) and the training yard
  (steps of 0.55 and 0.56 m, ledges, platforms with ramps, a thin wall, a 50 m lane) in plain greys (question
  GREY-COLOURS). The readout names the content files and their versions. A broken content file shows the loader's
  own sentence with the file and the line, and the page stays up.
- **4c:** he walks with W, A, S, D away from the camera, sprints with Shift until the stamina runs dry (a plain
  stamina bar shows it), jumps with Space (held Space jumps again on every landing, as the old game and his sandbox
  do), walks onto the 0.55 m step and is stopped by the 0.56 m one, reaches the 3.05 m ledge and not the 3.1 m one,
  and the plain stand-in turns to the mouse. The camera follows at the framing of his Export; the backquote key
  opens the camera panel with its sliders, the five old presets and "copy values". In the street corner he can jump
  onto the cover wall and onto the houses' roof edges (MOVE-RULES, row 5).
- **4d:** holding the right mouse button (and Q, if he takes MOVE-RULES row 6) he walks at 0.55 of his speed and
  cannot sprint, and the camera reacts in its sights mode.

## 3. Before any edit
1. The advisor agent's APPROVE of this plan, and his go on it with WALK-CUT. Each later part starts only after the
   advisor agent has read this plan again against that day's main (as for 3b), and after his answers that part needs
   (section 13). That re-read carries the part's exact symbols to add and to remove, taken from the designs' tables
   as they stand that day.
2. **4a needs no manifest change** (its tools are extra programs of the tools crate, which cargo finds without a
   manifest line), so no dependency hold stands before it.
3. **4b starts with one batch of manifest changes**, uncommitted and unbuilt until security-auditor has answered on
   exactly that content (`.claude/CHECKPOINTS.md` section 3; the list it must answer on: `rust-reviewer-cs4.md`,
   block 1). Lockfile by the resolve-only commands change-set 2 used; every other cargo command with `--locked`;
   `npm ci --ignore-scripts` only.
4. Before the first recording is committed (4a): the short report on the state hash's function and its cost per tick
   (PM-0004-21), written with rust-reviewer: a recording pins the hash.
5. Before 4b's build: test-engineer and security-auditor read this plan's test list (section 9 and the designs'
   mark tables) and say what a planted defect would slip past (Builder V3's advice from 3b's round 1). Their points
   are folded in before step 1.
6. A checkpoint per part (`cs4a-...` to `cs4d-...`); at most 3 agents at a time. **The background saver cannot push
   from this chat's worktree** (it pushes the local branch's own name, and this chat may push only to
   `claude/cool-pasteur-oiqk1r`): the main session commits and pushes the notes by hand at each step and whenever
   agents run, and says so in each report. The hold on dependency changes binds that hand push too: while a manifest
   or a lockfile differs from main, nothing is pushed before `dependency-check.md` exists
   (`.claude/CHECKPOINTS.md`, his DEP-HOLD).
7. The interim rule of his words of 2026-11-25 holds for the session and every agent: git and gh start their own
   command, with no other program in front.

## 4. Change-set 4a, "The recording"
Lane 1 throughout. Symbols, the file format and each test in full: `netcode-architect-cs4.md`, block 1.
| Step | What | Main files | Code / tests |
|---|---|---|---|
| 1 | One visitor in `sim` drives every writing of the state, so names and bytes cannot drift; the shared part is written on its own, so its buffer never holds a seed byte | `crates/sim/src/world.rs` | 60 / 50 |
| 2 | **The one new number on the players' file: `Sim::shared_hash()`** (his DEPART-3). It returns the shared hash only, is read only during a replay, and its buffer is private to the room. The test that pins the file's exports gains exactly this name | `crates/room/src/lib.rs`, `crates/sim-wasm/src/lib.rs`, `local_match.rs`, `packages/sim-worker/boundary/sim-wasm.test.mjs` | 25 / 30 |
| 3 | The recording's file format, version 1: a public test seed only (a real match seed is refused by the writer and by every replay), the host's time and every number of the input call for each tick, with room already for the move, the cursor's ray and the view time, so that 4c changes no format; a reader that checks every count before it allocates and never panics | `crates/devtools/src/recording.rs` | 140 / 120 |
| 4 | The replay runner (the same calls the Worker makes; one hash per tick), the comparer (the first differing tick), the state as named lines for a field-by-field difference | `crates/devtools/src/replay.rs`, `compare.rs` | 150 / 60 |
| 5 | The scripted inputs of the first recording, the two tools (`make_recordings` with a `--check` mode like the vector generators; `replay`), the test-only module's exports | `crates/devtools/src/scripts.rs`, `src/bin/`, `src/wasm.rs` | 150 / 0 |
| 6 | The first recording, 1,800 ticks on today's state, with its hashes and a manifest; replayed natively in one process, in two processes with different hash-map seeds (with a 36,000-tick scripted run), and as two matches interleaved in one process | `crates/devtools/recordings/`, `tests/recordings.rs`, `two_processes.rs`, `two_instances.rs` | 0 / 200 |
| 7 | The same recording in WebAssembly on the file players get: in Node, and in Chromium through a test-only Worker | `packages/sim-worker/src/recording.ts`, `boundary/replay.test.mjs`, `packages/e2e/tests/replay/` | 130 / 160 |
| 8 | The three planted divergences, each caught at its tick with the differing field; the word-size vector; and "no allocation" counted for hostile bytes in the room and the endpoint (3B-HOSTILE-ALLOC) | the notes' plants script; `crates/room/tests/any_bytes.rs`, `crates/client-endpoint/tests/any_bytes.rs` | 0 / 90 |
**4a: about 1,365 lines bottom-up (655 code, 710 tests); planned 2,450; stop line 3,050.**
- **The size ladder, fixed now** (a part moves whole with its tests, and he is told before it moves): at 2,450
  lines, 3B-HOSTILE-ALLOC moves to the link simulator's change-set (its other named home); at 2,750, the 36,000-tick
  run and the two interleaved matches move to 4c, where movement makes them meaningful. Never moved: the runner, the
  comparer, the first recording on its four runs, the export with its pin, one planted divergence.
- **Evidence** (logic and netcode: deterministic tests): every run of `netcode-architect-cs4.md`, block 1.6; each new
  check shown failing on its planted defect (one digit of a hash changed; the simulation changed at tick 900; an
  input call skipped in the Node loop; one byte of the recording changed; a counter kept outside the state); the
  players' file's size and sha256 here and on GitHub's runner; the legacy gate unchanged. No CI edit: the new tests
  run in the jobs CI already has.
- **Reviewers** (Opus 5.5, effort max, at most 3 at a time, two rounds at most): netcode-architect, rust-reviewer,
  test-engineer; then engine-architect (the one new export) and security-auditor (the reader of recorded bytes; that
  no seed can reach a dump). The verifier until PASS; the advisor agent on every pushed head; docs-keeper.
- **No browser check:** nothing a player sees, hears or feels changes. The merge is his go for the head the advisor
  agent approved, after his Advisor's merge check.
- **Its docs pass also carries what this chat owes:** his blocks of 2026-11-25 into `docs/DECISIONS.md` by script
  (the takeover block word for word, his answer ORDER-3C, his Export block and each later answer); change-set 3b's
  own docs pass, which `docs/STATUS.md` says is still owed; and the Hornet's entry in `docs/UNITS.md` section 0 by
  script from his message, the unit marked decided and its numbers a sketch. **Two lines of that section clash with
  the entry and are listed for him, not settled:** line 21-23, decided: "the counter triangle and tier one stay
  shared; the differences grow with tech (tier-two specialists and vehicles are unique)", while the Hornet is a
  tier-one unit of the Empire alone; and line 54-55, an idea: "Empire: a hover skimmer as its first vehicle", while
  the Hornet is a vehicle and tier one (he has said that line stays as it is). The advisor agent found no third
  clash inside section 0. Outside that section, for his eye only: `docs/LOOP_V2.md` lines 74-75 repeat "tier one
  stay shared", and his queued unit walk (`docs/DECISIONS.md` lines 48-79) holds early vehicles for the Empire.

## 5. Change-set 4b, "Content and the world"
Symbols and tests in full: `engine-architect-cs4.md`, blocks 1 and 3.6 to 3.7; `rust-reviewer-cs4.md`, blocks 1, 2
and 4.
| Step | What | Main files | Code / tests |
|---|---|---|---|
| 0 | The one manifest batch (section 3, item 3): the new `content` crate; `sim` on `content` and on the collision library; the 31 overflow lines the build's own check asks for; the tools crate's edges that still have no user dropped (section 8, item 22) | every `Cargo.toml`, `Cargo.lock` | about 110 lines of configuration |
| 1 | **The built-file check for the fused multiply-add** (section 5.3), with its planted defects, before the collision library is used by anything; the scan's two new rules; the feature compare by name and version (3A-FEATURE-VERSION) | `packages/client/scripts/build-checks.mjs`, `build.mjs`, `crates/sim/tests/rules_scan.rs` | 340 / 320 |
| 2 | The content crate's reader and writer: JSON to a tree with line and column, numbers, one canonical writer, a typed reader with field paths, ranges and length bounds from one field table, the schema, errors that name the file | `crates/content/src/` (`json.rs`, `number.rs`, `write.rs`, `read.rs`, `field.rs`, `schema.rs`, `error.rs`) | 790 / 500 |
| 3 | The bundle (one file set, whole-set checks, frozen), and the two kinds of this part: the movement table and the map document. If the content's types are the first generated types that nest, follow-up 3A-PART5-CLASH is decided here with engine-architect | `bundle.rs`, `movement.rs`, `map.rs`, `ts_export.rs` | 360 / 290 |
| 3b | **The reader's own proofs, in the part that merges the reader:** every number of the old tuning and 10,000 made ones parse to the same bits natively, in WebAssembly and in `JSON.parse`, in Node and in Chromium, and write back the same (content item 1, PM-0005-2); 10,000 broken files refused the same way natively and in WebAssembly, and a bounded run of 1,000,000 random inputs with no panic (content item 2, PM-0005-4, -6); the ranges on movement's fields (PM-0005-7) | `crates/content/tests/`, `crates/devtools/src/wasm.rs`, `packages/sim-worker/boundary/`, `packages/e2e/tests/` | 20 / 530 |
| 4 | The integer time unit, 3,000 units a second (100 per tick; every duration of the old game is a whole number of milliseconds and converts exactly), with the rounding report over a reviewed list of the old game's durations (PM-0004-11) | `crates/content/`, `crates/sim/src/time.rs`, a report tool | 240 / 80 |
| 5 | The tools: `content-bundle` (refuses a content file that is not in canonical form), `testlines` (section 11: `--check`, `write`, `compare-old`, `table`) | `crates/devtools/src/bin/` | 250 / 140 |
| 6 | The data: his Export, its manifest and the table under `content/source/` (not under `docs/`: section 11); `content/movement/soldier.json` written by the tool from his Export (after STEP-60 and his yes to the differences list); `content/maps/test-scene.json` written by hand from the Art sheet's sizes and the marks' shapes | data, not counted | |
| 7 | Content reaches the simulation once per match, through the constructor (no new call; nothing per tick changes); the build writes `content.json`; the page fetches it once and hands its bytes to the Worker | `crates/sim-wasm/src/lib.rs`, `local_match.rs`, `scripts/build.mjs`, `packages/client/src/app/content.ts`, `worker-link.ts`, `packages/sim-worker/src/` | 290 / 260 |
| 8 | The static collision world built from the map: every number checked, shapes in id order, the tree built once; its bytes and 10,000 rays identical natively and in WebAssembly (PM-0004-32); the ray query the aim will use | `crates/sim/src/collision.rs` | 260 / 170 |
| 9 | The page draws the scene from the map; the readout's content rows; the browser tests (the scene appears; a broken bundle shows the loader's sentence) | `packages/client/src/render/scene.ts`, `overlay/`, `packages/e2e/tests/` | 230 / 230 |
**4b: about 5,410 lines bottom-up; planned 9,750; stop line 12,200.** Under CONTENT-SOURCE (a) step 2 shrinks to
about 100 lines of glue and the part to about 4,280 (planned 7,700).
- **Its ladder, fixed now** (code moves with its proof, never a proof alone): at 9,750 lines the schema writer and
  its tests move to 4d (the panel's limits are written in the panel's code until then); at 11,000 the static
  collision world (step 8) moves whole, with PM-0004-32, to the start of 4c, where its first user is, and 4b's page
  draws the scene from the map alone. Never moved: the built-file check (his words tie it to the change-set that
  brings the collision library; it runs on every build, so it reads the library's code as soon as that links), the
  loader's seven stages with the reader's proofs of step 3b, the Testlines check.

### 5.1 The content crate, in short
One loader with seven stages, each of which refuses by name (a size cap; JSON only, with duplicate keys, NaN and
depth beyond 32 refused; a format name and a version per file; unknown keys refused; ranges and length bounds; the
whole set: one definition per id, every reference resolves, no fallback; then frozen). One canonical writer (keys
sorted, the number text a browser writes, so `JSON.stringify` and the writer agree on every number but -0). One
definition per file, one folder per kind: `content/movement/`, `content/maps/`, and in 4c `content/camera/`
(lane 1 owns that folder until change-set 5 merges; it passes to lane 2 with the client's `camera/` folder, build
plan Part 15.2). `content/source/` is no kind: it holds his Export, its manifest and the table (section 11); the
bundle tool skips that one folder by name and refuses any other folder it does not know. The
movement table's fields carry his Export's own names and units (section 11), so the check between the two is plain
equality; the simulation works out its own constants from them once, when a match starts (the take-off speed from
the reach, the gravity and the step-up; the easing share per tick; the stamina's rates; the time unit's counts),
and reports each rounding.

### 5.2 The collision library: what it brings (the simulation draft's choice, not reopened here)
`parry3d` at exactly 0.31.1, default features off, with `dim3`, `f32`, `alloc` and `enhanced-determinism` (its own
`std` feature off if that builds for both targets: the build's first step tries it; else on, with the paths that
need it avoided). By rust-reviewer's reading of the registry (to be confirmed by `cargo tree` after the hold): **33
new packages in the lockfile; 31 are compiled into the players' file's tree**, of which the linker keeps what our
calls reach; four of them run a build script (libm, num-traits, serde_core, heapless) and one is a new procedural
macro (num-derive); no copyleft licence. Estimated growth of the players' file: 60 to 160 KB on today's 53,813
bytes, measured in the first build step. What it does not give by itself, and how the plan closes each:
- its `enhanced-determinism` flag makes its internal maps ordered and its vector maths scalar; it does not make a
  fused multiply-add the same on every target (the library routine picks the processor's instruction at run time
  natively): closed by section 5.3 and by never calling one;
- it allocates when two shapes truly overlap (its own contact routine builds a workspace per call): our movement
  keeps a 2 mm skin, resolves an overlap with one workspace of our own that is reused, and the zero-allocation test
  plants overlaps;
- its tree walks with a fixed stack of 32: a map whose tree is deeper is refused when it loads;
- it calls std's `min`, `max` and `clamp` (which the draft's float rules do not allow in our own code): the same
  bits natively and in WebAssembly on today's toolchain (measured by rust-reviewer); they join the float vectors as
  observed operations, so a toolchain change that moves them fails a test.
Only one module of the simulation names it (`crates/sim/src/collision.rs`), held by a scan rule; our own state uses
our own small vector type.

### 5.3 The check on the built file for the fused multiply-add (his MUL-ADD (a); follow-up 3A-FMA-CHECK)
His words: "The check on the built file (no fused multiply-add inside the players' WebAssembly) is built in the same
change-set that brings the first outside crate into the simulation ... That change-set's plan names it." It is 4b's
step 1, and it runs on every build, development and production, right after the bindings tool has written the
players' file. What it refuses, in the file as built:
1. **by name**, read from the names-kept twin that cargo writes (the exact input of the bindings tool): any function
   named `fmaf`, `fma` and their kin, or whose path ends in `fma`, `fmaf`, `mul_add` and their kin, from any crate;
2. **by instruction**, walking the production file's own code: any SIMD instruction at all while SIMD is off (the
   four fused ones named in the message); an instruction the walker does not know fails the check;
3. **by import**: any host function that names a fused multiply-add;
4. and, at no extra cost, every std maths routine the float rules ban (sine, cosine, powers, the float remainder
   and the rest), now for every crate linked into the file, where today's scan reads only our own simulation's source.
It reports, without refusing, each maths routine of the pinned `libm` that is linked, against the list that has
vectors (PM-0004-22): Walk adds vectors for the two it uses on purpose, the exponential (the easing share, worked out
once per match) and the arc tangent (the angles the page draws). Planted defects, each must stop the build: a
`mul_add` call exported from our own bindings; the same through the outside vector library; a relaxed-SIMD
multiply-add; an imported function named `fma`; a sine in our bindings. Controls: the real build passes, and a plain
`a * b + c` passes. The native side: every native build targets the plain x86-64 baseline (no processor-specific
flags; a planted `target-cpu=native` is refused). **If the first build shows a fused routine linked from the
collision library's tree, 4b stops for his decision**: his MUL-ADD (a) bans it in the simulation.

## 6. Change-set 4c, "The first soldier"
Symbols and tests in full: `rust-reviewer-cs4.md`, blocks 3 and 5; `netcode-architect-cs4.md`, blocks 3 to 5;
`engine-architect-cs4.md`, blocks 2 to 4.
| Step | What | Main files | Code / tests |
|---|---|---|---|
| 1 | The simulation: the soldier table with ids that go stale on reuse; the rules taken from content once per match; our own vector type and the three float helpers; the sweeps on the collision world; **our own movement**: walk and sprint with stamina, the easing, the jump on the exact arc, the slide along walls, the step-up, the snap to a slope going down, the soft push between two soldiers; the vectors for the two maths routines that join | `crates/sim/src/` (`ids.rs`, `soldiers.rs`, `rules.rs`, `fmath.rs`, `collision.rs`, `movement.rs`, `world.rs`) | 1,240 / 1,090 |
| 2 | The command path: the page's post gains the move and the cursor's ray; one input call of 13 plain numbers; the endpoint casts the ray against its own copy of the world; the room refuses a move longer than 1 and an aim outside the map; the jump's rule (one jump per command however many presses it brings: 3B-ONE-PER-TICK) | `crates/client-endpoint/src/post.rs`, `endpoint.rs`, `crates/room/src/lib.rs`, `crates/sim-wasm/src/lib.rs`, `packages/sim-worker/src/` | 385 / 530 |
| 3 | The snapshot's own soldier and the block: the header's soldier count, 48 soldier rows of 40 bytes, the owner's section; each block stamped with the time its tick was due; the page blends between ticks; the Worker's pool of buffers grows (3B-POOL-16) | `crates/protocol/src/`, `crates/client-endpoint/src/layout.rs`, `block.rs`, `packages/client/src/app/blend.ts`, `packages/sim-worker/src/host.ts` | 320 / 340 |
| 4 | The page: the plain stand-in at the Art brief's sizes on one root, the follow camera at his Export's framing, the keys and the mouse (pointer, move, the ray), the development aim line, a plain stamina bar, the readout's new rows | `packages/client/src/soldier/`, `camera/follow.ts`, `input/`, `render/`, `app/`, `overlay/`, `picture.ts` | 800 / 540 |
| 5 | The camera panel: sliders, the five old presets, "copy values"; the camera's content file; development-only code kept out of the production build (3B-CSS-PROD). **The early-look zip is built after this step** | `packages/client/src/camera/panel.ts`, `settings.ts`, `crates/content/src/camera.rs`, `content/camera/` | 360 / 300 |
| 6 | The movement recording (about 2,400 ticks on the street corner and the yard) and every mark that rides it, with the three planted divergences run again on movement; the first recording re-made | `crates/devtools/`, `recordings/` | 320 / 230 |
| 6b | The paced replay in the real Worker, in development builds only: a recording runs at live pace with the camera still and with the camera moved, and its hashes are the recording's own (PM-0004-28: nothing of the picture reaches the simulation) | `packages/sim-worker/src/`, `packages/client/src/app/`, `packages/e2e/tests/replay/` | 130 / 110 |
| 6c | 2,000,000 hostile movement commands into a running match with no panic (PM-0004-60); the yard at the far corner of a 2 km map (PM-0004-39) | `crates/room/tests/`, `crates/sim/tests/`, `crates/devtools/` | 30 / 180 |
| 7 | The browser tests of the page (walk, jump, the two steps, aim on a ramp, the camera follows, keys let go on blur: 3B-BLUR-TEST, the hidden tab with a soldier, allocation measured again); then the reviews, the verifier, his check | `packages/e2e/tests/` | 0 / 410 |
**4c: about 7,315 lines bottom-up; planned 13,150; stop line 16,450.** (Step 4's code is engine-architect's own parts
added up, `engine-architect-cs4.md` block 4.11; drafts 1 and 2 had it 200 lines low.)
- **Its ladder, fixed now:** at 13,150 lines the far-corner map (PM-0004-39) moves to 4d: the card's own first
  rung (build plan lines 355-356), and he is told before it moves. Nothing else moves: at the stop line the chat
  stops and he decides. Never moved: the movement recording on its four runs with PM-0004-17, -19, -21 and -24, the
  paced replay (PM-0004-28), the stale-id run (PM-0004-15), the hostile commands (PM-0004-60), the parity marks
  PM-0004-30 and -59, the camera panel (his words: "from its first playable build").
- **The camera panel and the early look.** His queued words: "Slice 1 ships with a live camera tuning panel from
  its first playable build". So the early look of section 6.3 is built after the panel (step 5), not before it:
  the first build he plays has it.

### 6.1 The movement, in short (his rules and his values; the float side by rust-reviewer)
A vertical capsule (radius from his Export, height the stand-in's 1.80 m, which is his placeholder: one number scales
both). Each command that runs is one movement step: the wish direction from the move, scaled back to length 1 so
that a direction rounded to whole numbers on the wire still gives full speed; the speed, one of three (walk; walk
times the Conscript's own sprint multiplier; in 4d walk times the sights multiplier); the velocity eased toward it
by a share worked out once per match from his speed-up and stop times; stamina as the old game has it (it drains
while he wants to sprint, in the air and against a wall too; it refills with no pause; run dry, sprint is locked
until the bar is back at its share; a jump costs none); the jump when Space is held or was pressed, only from the
ground; **the exact arc for constant gravity** (the old game's order of the two sums tops out 18 cm low at 30 steps
a second: 2.89 m where his START-RULES say 3.07 m; the exact arc gives 3.0700 m); up to four slides; the step-up
(up, across, down), **in the air too**, tested against the arc's true top: that is how a jump gains a ledge up to
the reach (the arc's top of 2.52 m plus the 0.55 m step-up), and why the yard has ledges at 3.05 m (gained) and
3.10 m (not); the landing; the snap to a slope. The facing turns straight to the point under the cursor, with no
turn rate, as in the old game. Soldiers never block each other: after all have moved, overlapping pairs are pushed
apart softly, and the push is swept against the world so it never puts anyone in a wall (built and proven here with
two soldiers, as the card places it; he feels it from change-set 5). No trigonometry runs in the step; nothing
allocates. The card's sentence that the jump's reach is an open question (build plan lines 341-342) is overtaken by
his START-RULES: the reach is 3.07 m.

### 6.2 What crosses to the page (the block), and six choices between the designs
- The header gains `soldier_count`. The soldier section has **48 rows of 40 bytes** (a fixed capacity per build, the
  live count in the header, row i is slot i): the id (slot and generation), position, velocity, yaw, pitch, the flags
  (occupied, alive, on the ground, sprinting, aiming), the team, and `ready` (how far the swing from the sprint pose
  to the aiming pose has got; full until the delay's rule exists). A new owner's section holds the local soldier's
  id, his team and the aim point the server tests. The block is 1,988 bytes at any number of soldiers.
- **Where the designs differed, and which the main session took:** (1) the soldier row is engine-architect's (it
  owns the layout), with the vertical velocity and `ready` from netcode-architect's; no separate "placement"
  counter: a new generation at a row means place, never slide, and Slice 1 has no teleport. (2) The input call takes
  13 numbers: the view time joins in change-set 8 with its first reader (the recording's format already has its
  place). (3) The tools crate's three unused edges are dropped in 4b's manifest batch and come back with their
  first user (section 8). (4) rust-reviewer's movement steps up "only while grounded"; game-designer's check shows
  the reach of START-RULES needs the step-up in the air too, as the old game and the sandbox both have it: the plan
  takes that, and rust-reviewer confirms the sweep for it at 4c's step 1. (5) engine-architect's movement table
  names the old game's unused `T.sprintMult` and gives the easing the unit of an acceleration: the content's fields
  carry the Export's own names and units instead (section 5.1), with the Conscript's own multiplier and the easing
  as two times. (6) The content holds the jump's **reach**, as START-RULES names it ("a reach of 3.07 m") and his
  Export carries it, and the take-off speed is worked out from it (engine-architect's table stored the speed;
  game-designer raised it as a question; the advisor agent rules it settled by START-RULES). The advisor agent let
  all six stand; the reviewers confirm them at 4c's steps 1 and 3, and one that is rejected goes back to it.
- **Recordings and content** (the advisor agent's ruling): each recording runs on a frozen copy of the content
  files it was made with, kept beside it, so a value he tunes later never changes an old recording's hashes; making
  a recording anew is its own step, named in the part that does it.
- **The blend between two ticks** (follow-up NA-N3): the Worker stamps each block with the time its tick was due; the
  page picks the pair of blocks and the share between them, never looks ahead, and places (never slides) a soldier
  whose id changed. netcode-architect's design keeps the **three** newest blocks instead of the drafts' two, because
  with two, the Worker's lateness he reported (14 ms at the 99th percentile) breaks the 5 % smoothness mark on a
  144 Hz screen: a departure listed in DEPART-4, proven by a test of the blend alone that must fail with two.
- **What the first picture after a pause shows** (3B-RESUME-FLASH, his decision: question RESUME-FLASH).

### 6.3 The early look (not a check)
After step 5: a zip of the development build from a commit on the branch, with a README that says it is an early
look, what is not in yet (the movement recording, the big test runs, sights), that no reviewer has read this code
yet, and that nothing is asked of him. It runs on the values of the Export he has cleared by then (STEP-60).

## 7. Change-set 4d, "Sights, and what is built last"
Every step here builds code and proves it in the same part; no proof of 4b's or 4c's code waits for 4d.
| Step | What | Code / tests |
|---|---|---|
| 1 | Aiming down sights by SLICE-FEEL (section 10): the right button held (and Q on his word, MOVE-RULES row 6), 0.55 of the speed, no sprint, the flag in the snapshot and the block (lane 2's W1), the camera's sights mode. **Because this changes the simulation, the movement recording gains aimed stretches and is made anew, and its four runs and its marks run again** (build plan line 484: a recording per change-set that changes the simulation; PM-0004-19, -21) | 170 / 210 |
| 2 | The import of the old tuning with its report, walked over every leaf (PM-0005-14); a saved content file reaching the Worker within a second in development (PM-0005-10, PM-0002-31) | 300 / 390 |
| 3 | The frame-cap setting, and the smoothness reading for each of its options on the lane recording, run through 4c's paced replay (PM-0003-42; the build plan's open question OQ-0003-10: the readings are for his pick of the cap); the SIMD reading (PM-0004-25: a reading that cannot fail; SIMD stays off) | 80 / 60 |
**4d: about 1,210 lines bottom-up; planned 2,200; stop line 2,750.** Its ladder: at 2,200 lines the SIMD reading
moves to a follow-up before Slice 2 (it decides nothing: SIMD stays off either way); nothing else moves. If the
far-corner map came over from 4c, it is built and met here.

## 8. Every item named for 3c, placed by name (his condition c)
Sources: `docs/STATUS.md` ("Next", item 2), `docs/STATUS_ARCHIVE.md` lines 1453-1557, Builder V3's handover note part
c, change-set 3's plan at 80a51e42. The full table with who named each: `netcode-architect-cs4.md`, block 2.
| # | Item | Home |
|---|---|---|
| 1-4 | The replay runner; the comparer and the field-by-field difference; the first recording natively and in WebAssembly; "a replay runner and the first recording in CI" | **4a** |
| 5-8 | The state's hash on the players' file; that it returns the shared hash only, as plain numbers, read only during a replay, its buffer private to the room; the host's time recorded with every tick; the pin of the file's exports | **4a** |
| 9-10 | The hash report before the first recording is committed; the begun parts of PM-0004-17, -18, -19 and the three planted divergences | **4a**, run again on movement in 4c |
| 11 | 3B-HOSTILE-ALLOC, the counted "no allocation" for hostile bytes | **4a** (the ladder may move it to the link simulator's change-set) |
| 12 | His RELEASE-COUNT (a): no release counter | both: nothing is built |
| 13-15 | The link simulator itself; the foundation's exchange "with L1 behind it" (build plan line 466); its own mark PM-0002-49 (line 474) | **the link simulator's change-set** |
| 16-21 | Its clock through the local match from recorded times; its reliable channel delivering exactly once; loss and the rate limit as silent drops; corruption as loss only; a `link()` accessor; `late` checked against its own reorder count | **the link simulator's change-set** |
| 22 | The tools crate's three path edges, declared in 3b and unused | dropped in **4b**'s manifest batch if still unused there; each comes back in **the link simulator's change-set** with its first user |
| 23-27 | The simulator with everything off equal to the loopback on the first recording; 10,000 presses under loss; counters across their wraps; "anything over a link that delays, drops or reorders" | **the link simulator's change-set** |
Named near 3c and belonging to neither: 3B-PLAUSIBLE and 3B-LATE-BOUND (change-set 7), 3B-ACKS and 3B-OUTAGE-256
(before online play), 3B-BLUR-TEST (4c, with the mouse). **The link simulator's change-set** follows 4d and is merged
before change-set 7 starts, as his answer says; the foundation's two marks at lines 466 and 474 stay open until then.

## 9. The marks the build plan places at Walk, each with its part
Each mark's test, where it runs and its planted defect: the designs' mark tables (`netcode-architect-cs4.md` blocks
1.6 and 5; `rust-reviewer-cs4.md` block 5; `engine-architect-cs4.md` blocks 1.12 and 2.9). A mark is met in the part
that merges the code it proves.
| Part | Marks |
|---|---|
| 4a | The foundation's recording lines (build plan lines 470, 473) natively, in Node and in Chromium; PM-0004-17, -18, -19 and -21 on today's state; PM-0004-21's hash report and word-size vector |
| 4b | ADR 0005's foundation line (line 461); the content rules CD-1 to CD-5 shown on movement and the map; **the reader's proofs: content items 1 and 2 (lines 503-504) with PM-0005-1 to -8**; PM-0004-11 (the time unit); PM-0004-32 (the collision tree byte for byte); PM-0004-33 (our own size, before and after the library and content); PM-0002-29 (the file's weight); 3A-FMA-CHECK; PM-0004-23's "a build check fails on a std transcendental call" for every linked crate |
| 4c | **On the movement recording, before 4c merges** (his condition a): ADR 0001's replay line (line 485), the recordings of line 484, PM-0004-19 (line 494), -17, -20, -21 (three of three, on movement fields), -24, and -28 (the paced replay in the real Worker: the automated run here, his 60 Hz and 144 Hz screens in his check). On the movement itself: PM-0004-6, -15, -22 (run again: the two new routines), -30, -35, -39, -54, -59, -60; START-RULES' two rules. On the path and the page: PM-0002-14 (the aim in the post), PM-0002-26's smoothness (the blend's own test; a reading on his displays), PM-0002-34's walking steps (his checklist, with the tab hidden for 1 and for 6 minutes); 3B-ONE-PER-TICK; lane 2's rows of section 12 |
| 4d | The recording made anew with sights, with PM-0004-19 and -21 again (line 484); PM-0005-10 and PM-0002-31 (development reload); PM-0005-14 (the import's report); PM-0003-42 with OQ-0003-10's readings for his pick; PM-0004-25 (a reading that cannot fail) |
| Lane 2, after 4c merges (Part 15.2) | the two size comparisons (PM-0004-33's comparison builds; PM-0005-16's size half) |
| Not run, by his word | PM-0004-34, the comparison against a library's character controller: his COMPARE (b), "runs only if our own movement misses its marks" |
Three readings of a mark that the plan states (DEPART-4): PM-0004-15 by our own seeded generator instead of the tool
the row names; PM-0004-17's two matches differ in their public test seed and in their input stream (netcode-architect's
test), and until change-set 8 only the input stream makes them behave differently, because nothing draws a random
number before then; PM-0004-59's planted defect as the old game's order of the two sums, because the formula alone
is within the mark's 1 mm of the exact arc and cannot fail it.

## 10. SLICE-FEEL and the movement rules
The old game's rules line by line, where his sandbox differs, and each line's source: `rules-cs4.md` (the main
session's draft) as corrected by game-designer's check (`game-designer-cs4.md`, verdict REVISE on the draft; where
the two differ, the check stands, and this section and questions MOVE-RULES, SPRINT-DELAY-4 and NO-SOURCE are written
from it).
- **Aiming down sights** (4d). The input: the right mouse button, held, as in the old game (one more held button in
  the post and the command; the page stops the browser's own menu on the canvas); click-to-toggle is a page setting
  in the old game and in the sandbox alike. Q as a second aiming key is a sandbox difference (its reason: in
  Firefox, Shift with a right click always opens the browser's menu), so it enters only on his word: MOVE-RULES,
  row 6. The rule is the same in both games, so it needs no word: 0.55 of the walk speed while held; no sprint, and
  aiming ends a sprint the same step; letting go with Shift still held resumes the sprint at once; stamina refills
  while aiming; he may aim in the air, and Space still jumps. The fields: the aiming flag in the simulation, the
  snapshot and the block (W1). The camera: its sights mode and its numbers in the camera's data and panel (the old
  game wrote three of them into its code; his Export names them); the sandbox's two changes to that camera are
  MOVE-RULES, row 7. In change-set 8: each gun's spread resting at its sights value, and the sight line (which has
  a sandbox difference of its own, for that change-set's plan).
- **The sprint-to-fire delay.** His words (SLICE-FEEL): "Coming out of a sprint, firing and aiming down sights each
  wait a short delay, set per gun." What his sandbox does: only the gun waits. Aiming itself starts at once (the
  slow walk, the camera, the spread's resting value); the gun swings up over its own delay and the first shot waits
  for it; a click during the wait is kept and fires then. Walk has no gun, so by the sandbox's rule nothing waits
  in Walk; a wait on aiming itself in 4d (netcode-architect's design, on one stand-in value) would make Walk feel
  different from Testlines, against "the real game must not feel different from it". His words and his sandbox
  point two ways here, so question SPRINT-DELAY-4 asks him. Either way the simulation keeps the time since the
  sprint ended from 4c on (hashed), and the block carries `ready` (W4), filled as "gun up" until change-set 8, so
  lane 2's layout does not move again.
- **The lines they add:** in Walk about 290 bottom-up (4d's step 1), 70 more if SPRINT-DELAY-4 is (b); in
  change-set 8 about 175 (the gun's wait and kept click, the spread at sights), by netcode-architect's count.
- **Seventeen differences** between his sandbox's movement and the old game's were found (game-designer's D1 to
  D17). Two are covered by his START-RULES (the jump's exact reach; the soft push). One is change-set 8's (a held
  fire button ends a sprint: part of the gun's wait). One is not taken (the sandbox holds the facing inside a
  wider circle round the soldier; Walk keeps the old game's 2 cm). The other thirteen are asked in MOVE-RULES, in
  eight rows; by his point 8e none enters without his word.
- **Numbers the sandbox keeps in its code**, not in his Export: question NO-SOURCE.
- **The jump:** one jump per command, only from the ground; a press in the air is dropped, never kept for the
  landing; held Space jumps again on every landing. All of it is the same in the old game and in his sandbox, and
  nothing in LOOP_V2, VISION_V2 or his decisions speaks against it (game-designer, block 4); his Brave check can
  change it.
- **Open in the design docs and left open:** LOOP_V2 section 9's "aiming at height" (shots and aim help). In Walk
  the aim point only turns the facing, by the old game's rule (the cursor ray's first hit, else the ground).
- **He should know before his check:** with his 3.07 m reach and blocks at their real heights, the soldier can jump
  onto the houses' roof edges (2.6 m) in Walk's street corner; in the old game houses were walls of endless height
  (MOVE-RULES, row 5).

## 11. His Testlines values (his point 8, a to g)
In full: `testlines-cs4.md`. In short:
- **a.** The sandbox was read, no code copied: branch `claude/sleepy-edison-ygmwfe` at be608cad (3c07269 when his
  Advisor read it); its README, `src/params.ts` and the code that writes the Export.
- **b.** His Export is saved whole by script from the chat: 15,567 bytes, sha256
  9ad1d9f914acc27c56cbfbe7c4e6c7e654ba24fa0e9ae75cb4e3b93103f06498, format "testlines-export" version 1, built from
  4c34e750f5edda22547e3bbe4074faadee5961ae, camera profile "Experimental", 18 values changed from the sandbox's
  start. In 4b it moves, byte for byte, to `content/source/` with its manifest. It is kept out of `docs/` on
  purpose: a change under `docs/` can be a risk-tier-0 pull request that the PR Advisor merges alone, and these
  files hold his values and decide what the check reads. **Any change to a file of `content/source/` is risk tier 1
  and needs his go.**
- **c.** One table, a data file (`content/source/testlines-table.json`), printed by the tool when asked (no copy is
  kept under `docs/`): each value
  Slice 1 uses, its name in the Export, its name in the content, its unit, the change-set that uses it and its home.
  The first draft by names and units is in `testlines-cs4.md`, section 3. **The comparison is run** (by script, on
  the saved file against `docs/legacy-tuning.json`): 83 old-game names, 60 equal, **23 different**, none without a
  counterpart (`testlines-compare.txt`). The 23: the Conscript's sprint multiplier (1.214 in the old game, 1.52 in
  his Export); sixteen camera values (his Experimental profile against the old camera); the SMG's recoil and four
  spread values (all 0 in his Export); the grenade's launch angle (41.253 degrees in the old game, 41.3 in the
  Export: the sandbox's own rounding). Beside them, values with no old number in the old game's tables (the body
  radius 0.4 m, where the old game's 0.7 m is written into its code and so in no table; the push strength and
  radius; the three sprint-to-fire delays; the active-reload windows; the aiming camera's floor of 36 degrees,
  where the old game's code has 28.5). Question DIFF-LIST carries both lists whole, printed by script, and asks his
  yes before any becomes the starting table.
- **d.** The tool `testlines` has a `--check` mode: it fails, naming the row, when a content value differs from the
  saved Export through the table, when a row names a place that does not exist, or when the saved Export's bytes do
  not match the manifest. A plain test runs the same check in the job CI already has. Planted: one content value
  moved by 0.001; one byte of the saved Export changed; an Export value of 0 against a small old value (the mistake
  this chat's first comparison run made, caught against his "changed from start" list).
- **e.** Not taken, each by name with one line (`testlines-cs4.md`, section 4): the Hornet; the explosion picture;
  the metrics room; the measuring grid; the session's own choices (faction, gun); and every sandbox rule that
  differs from the old game, of which those that touch movement are asked in MOVE-RULES (section 10 counts them).
- **f.** The Export says 60 steps a second: he was told before anything was used (STEP-60).
- **g.** Where he tunes once Walk exists, one home per value: question TUNE-HOME.
Three things the Export's rounding does, said so he knows: the speed-up and stop times read 0.164 s where the old
game's 14 per second is 0.16447 s (the game takes 0.164: easing 0.3 % faster); the reach reads 3.071 m (the
formula: 3.0708); the launch angle as above.

## 12. Lane 2's first list (issue #59), row by row
A lane request, never his word. The full answers with each field's name and type: `engine-architect-cs4.md`, block
3.8. Nothing on the list is refused.
| Lands | Rows |
|---|---|
| in the block today | H1 (the tick) |
| 4c | H2 (the soldier count), F1 position, F2 one yaw (gap 5: aim and facing are one angle in Slice 1), F3 pitch, F4 velocity (the owner's exact; other soldiers' worked out by the Worker from change-set 5), F5 on the ground, F6 sprinting, F7 alive (always set until change-set 8), F9 the id, F10 the generation (a change means place), F11's team, F12 the local soldier and his team (gap 8: in the block), W4's field `ready`; gaps 1, 5, 8, 10 |
| 4d | W1, aiming down sights |
| change-set 8 | F8, F13, E1 to E4, E6, W2; gaps 3 and 9; W4's rule (the gun's wait; under SPRINT-DELAY-4 (b) a first part of it in 4d) |
| change-set 9 | F14, E5, W3; gap 2 |
| later | F11's faction table (Slice 2); gap 7 (Slice 3) |
| no field needed | W5 |
| already his decisions | gaps 4 (PROJ-DRAW) and 6 (SLICE-FEEL) |

## 13. Decisions needed
Each with the part that needs it. To start 4a he needs to answer only WALK-CUT.
1. **WALK-CUT: how is Walk cut?** It matters because he wants to play the first soldier as soon as that is safe
   ("I would rather play a first part early than wait for one large stop"), and one stop of about 27,500 lines
   would hold it behind everything. Needed now. In every option the recording goes first as its own small
   change-set, 4a (his fallback (d): 2,450 planned lines, no browser check), and the early look comes at the same
   point of the work: after about 22,850 planned lines, about 50 hours of this chat's work at change-set 3b's pace.
   The options differ in how many reviews and gos stand before it, and in how much one review has to carry.
   - (a) **Four change-sets**, as section 0 has them: 4a, then content and the world (9,750), then the first
     soldier (13,150), then sights (2,200). Before the early look: two gos of his (4a's and 4b's) and 4b's short
     check. The judged soldier is the third stop.
   - (b) **Three.** 4a; then 4b and 4c as one stop (about 22,900 planned lines); then sights. This is the shape his
     answer's point 3 names ("cut in two with the first playable soldier in the first part"), after the recording.
     Before the early look: one go (4a's); it comes sooner than under (a) by 4b's two rounds of reviews, its check
     and the wait for his go. The judged soldier is the second stop, and that one review reads 3.3 times
     change-set 3b, with the outside collision library, our own content reader and all of movement in it at once.
   - (c) **Two.** 4a; then everything else as one stop (about 25,100 planned lines): "Walk stays one stop", after
     the recording. The early look as under (b); the judged soldier comes with sights, about 5 hours of work later
     than under (b), in one review of 3.6 times change-set 3b.
   - Recommended: **(a)**. Each part can be reviewed whole; the outside library and our own reader are checked
     before movement rests on them; and what it costs before the early look is one round of reviews and one go.
     If he puts the early look first, (b) is the fair second choice.
2. **STEP-60: what to do with an Export made at 60 steps a second?** (Asked in the chat on 2026-11-25; his words:
   "ask me before you carry values that depend on it".) It matters because the game runs at 30 steps a second, and
   values he judged by feel at 60 may feel different at 30. Needed before 4b's step 6. What depends on the step
   rate, by the sandbox's README and code (read, not measured):
   - **By their arithmetic, none of the values Walk uses.** The jump follows its exact arc at either rate; the
     easing of the speed, of the camera and of the soft push are written per second; stamina is a plain rate.
   - **By feel, these, because at 60 a key press reaches the next step within 17 ms and the picture runs at most
     17 ms behind the simulation, and at 30 both are 33 ms:** the sprint multiplier (1.52), the two times for
     speeding up and stopping (0.164 s: five steps at 30, ten at 60), the camera's 21 values (its follow rates and
     look-ahead above all) and the aiming camera's four.
   - **From change-set 8 on, by arithmetic too:** the per-gun delays, fire intervals and reload windows land on
     whole steps (33 ms at 30); that change-set's plan says which.
   - (a) He plays the sandbox at 30 steps and exports again; Walk carries that Export.
   - (b) Walk carries this Export, and he judges the values at 30 in the game (the early look).
   - (c) Walk carries only the values that do not depend on feel now (the jump, gravity, the step height, stamina,
     the body's radius), and the rest after he has played the sandbox at 30.
   - Recommended: **(a)**: a few minutes of his, and every value then has been felt at the game's rate.
3. **DIFF-LIST: his yes to the list below.** It matters because by his point 8c these are his tunings, and they
   become the starting table only once he has confirmed the list. Needed before 4b's step 6.
   <!-- difflist:begin -->
   _Printed by `testlines-difflist.mjs` from the saved Export (sha256 9ad1d9f9...6498) and from
   `testlines-compare.txt`; no value is typed by hand._

   **A. The 23 names where his Export differs from the old game's tables** (`docs/legacy-tuning.json`):
   | The old game's name | The old game | His Export |
   |---|---|---|
   | `UNIT_TYPES.conscript.sprintMult` | 1.214 | 1.52 |
   | `T.camHeight` | 40 | 24.5 |
   | `T.camBack` | 56.6 | 34.7 |
   | `T.fov` | 15 | 13 |
   | `T.camYawBase` | 0.7853981633974483 | 1.117011 |
   | `T.lookLerp` | 9 | 20.5 |
   | `T.camPushLerp` | 9 | 2.5 |
   | `T.lookAheadBase` | 5.5 | 3.4 |
   | `T.lookAheadFloor` | 3.5 | 4.5 |
   | `T.lookAheadHip` | 8 | 4.9 |
   | `T.lookFwdRange` | 26 | 15.9 |
   | `T.lookBackRange` | 16 | 9.8 |
   | `T.lookSideMax` | 6 | 3.7 |
   | `T.posFwdMax` | 3.2 | 0 |
   | `T.posBackMax` | 2.2 | 1.3 |
   | `T.posSideMax` | 2 | 0 |
   | `WEAPONS.shabzon.recoil` | 0.3 | 0 |
   | `WEAPONS.shabzon.spreadHip` | 0.075 | 0 |
   | `WEAPONS.shabzon.spreadPerShot` | 0.005 | 0 |
   | `WEAPONS.shabzon.spreadMax` | 0.16 | 0 |
   | `WEAPONS.shabzon.spreadDecay` | 0.7 | 0 |
   | `T_CHARGE.nade.pitch (deg)` | 0.72 rad = 41.253 deg | 41.3 |
   | `T.foxPanMax` | 16 | 9.8 |

   **B. What he changed from the sandbox's start values** (the Export's own list: 18 entries,
   3 of them in the groups the game does not take: the grid, the metrics room, the Hornet):
   | Group and name | Start | His value |
   |---|---|---|
   | `simulation.step_rate_hz` | 30 | 60 |
   | `soldier.faction` | "soviets" | "empire" |
   | `movement.sprint_multiplier` | 1.214 | 1.52 |
   | `smg.recoil_view_kick` | 0.3 | 0 |
   | `smg.spread_rest_rad` | 0.075 | 0 |
   | `smg.spread_per_shot_rad` | 0.005 | 0 |
   | `smg.spread_max_rad` | 0.16 | 0 |
   | `smg.spread_recovery_rad_per_s` | 0.7 | 0 |
   | `aim_down_sights.ads_flatten_floor_deg` | 28.5 | 36 |
   | `camera.lens_vertical_fov_deg` | 15 | 13 |
   | `camera_framing.look_follow_rate_per_s` | 9 | 20.5 |
   | `camera_framing.edge_push_rate_per_s` | 9 | 2.5 |
   | `camera_framing.look_ahead_min_m` | 2.1 | 4.5 |
   | `camera_framing.camera_shift_forward_m` | 2 | 0 |
   | `camera_framing.camera_shift_side_m` | 1.2 | 0 |

   **C. Values with no number in the old game's tables, used by Walk:**
   | His Export's name | His value | The old game |
   |---|---|---|
   | `movement.body_radius_m` | 0.4 | a number in its code, not in a table: `resolveObstacles(player.position,0.7)` (legacy/shamble-game.js:2780) |
   | `movement.air_control_pct` | 100 | no number: full steering in the air |
   | `pushing.push_strength_per_s` | 8 | no such rule: soldiers pass through each other |
   | `pushing.push_radius_m` | 0.4 | no such rule |
   | `aim_down_sights.ads_flatten_floor_deg` | 36 | a number in its code: `const _tgt=Math.max(28.5,Math.min(_base,_toAim))` (legacy/shamble-game.js:2826) |
   | `aim_down_sights.ads_dead_zone_scale` | 0.45 | a number in its code: `const dzx=ps.ads?T.camDeadX*0.45:T.camDeadX` (legacy/shamble-game.js:2809) |
   | `aim_down_sights.ads_camera_ease_per_s` | 7 | a number in its code: `adsPitchK+=(_t-adsPitchK)*(1-Math.exp(-7*dt))` (legacy/shamble-game.js:2822) |

   **The same kind, used from change-sets 8 and 9 on** (their plans ask again; shown so the list is whole):
   | His Export's name | His value | The old game |
   |---|---|---|
   | `smg.sprint_to_fire_delay_s` | 0.15 | no such rule (SLICE-FEEL adds it) |
   | `smg.active_reload_window_start_pct` | 40 | no such rule (SCOPE-RELOAD adds it) |
   | `smg.active_reload_window_width_pct` | 20 | no such rule |
   | `rifle.sprint_to_fire_delay_s` | 0.25 | no such rule (SLICE-FEEL adds it) |
   | `rifle.active_reload_window_start_pct` | 40 | no such rule (SCOPE-RELOAD adds it) |
   | `rifle.active_reload_window_width_pct` | 20 | no such rule |
   | `shotgun.sprint_to_fire_delay_s` | 0.3 | no such rule (SLICE-FEEL adds it) |
   | `shotgun.active_reload_window_start_pct` | 40 | no such rule (SCOPE-RELOAD adds it) |
   | `shotgun.active_reload_window_width_pct` | 20 | no such rule |
   | `shotgun.falloff_start_m` | 15 | no falloff on this gun |
   | `shotgun.falloff_end_m` | 34 | no falloff on this gun |
   | `shotgun.falloff_min_share` | 1 | no falloff on this gun |
   | `grenade.grenade_refill_s` | 5 | not among the Export's old-game names |
   | `grenade.bounce_kept` | 0.4 | not among the Export's old-game names |
   | `grenade.slide_kept` | 0.6 | not among the Export's old-game names |
   | `grenade.fuse_s` | 1.5 | not among the Export's old-game names |
   <!-- difflist:end -->
   - (a) Yes to all, as the Export has them.
   - (b) Yes, with the rows he names changed or left at the old game's value.
   - Recommended: **(a)**, on the Export that STEP-60 leaves standing (a new Export is compared again by the same
     script, and he sees the list again only where it differs).
4. **CONTENT-SOURCE: who writes the code that reads the content files?** It matters because that code runs on the
   server and in every browser for years, and each outside package in the players' file is held to the same checks
   as the collision library. Needed before 4b.
   - (a) The packages the content draft names (serde, serde_json and what they need, a path-to-error helper, garde):
     about 1,130 fewer lines of ours; five to eight more outside packages in the players' file; whether garde runs
     in WebAssembly is unread; the writer's number text differs from the browser's, so "copy values" would need a
     pass through Rust.
   - (b) serde_json's untyped tree with our own typed reader and ranges: about 440 fewer lines than (c); five
     outside packages; duplicate keys pass silently.
   - (c) All our own: no outside package; one reader on every side; the canonical text equal to the browser's own;
     about 1,130 more lines (about 2,000 planned), and a reader of ours that must be fuzzed.
   - Recommended: **(c)**, engine-architect's choice, for the reason he gave with RNG-SOURCE (c). It costs the first
     soldier a few hours.
5. **GREY-COLOURS: a default he can veto, no answer needed.** The look is his, and Walk's scene has to have some
   colour before the look exists. Unless he says otherwise before 4b's step 9, the scene follows the default line
   of `docs/LOOP_V2.md` section 9, "Greybox first: plain blocks, with height read through brightness (higher is
   lighter)", in neutral greys, and the plain stand-in is one mid grey with a darker gun. His other choices: (b)
   the Art lead's palette proposal and the brief's faction colours as placeholders; (c) one flat grey.
6. **MOVE-RULES: which of his sandbox's movement rules that differ from the old game does Walk build?** It matters
   because his point 8e lets none in without his word in this plan, and each changes how the soldier meets walls,
   steps and slopes. Needed before 4c (rows 6 and 7: before 4d). The rows (`game-designer-cs4.md`, block 2):
   1. **The body's size against walls.** His Export's radius (0.4 m), with the Art stand-in's 1.80 m height for
      headroom. The old game: 0.7 m, written into its code (so no table shows it); 1.4 m wide, he would have 0.1 m
      to spare in the street corner's 1.5 m alley.
   2. **Steps and edges.** He rises onto a step as soon as his body touches it, and he holds a little past an edge
      (the body's round foot rests on it). The old game: he rises only once his centre crosses the step's edge, and
      drops off a ledge as his centre passes it. Walk leaves out the sandbox's pull onto the block: the blend
      between ticks smooths the rise.
   3. **Walls.** He slides along a wall and loses the speed that went into it; corners are round. The old game: he
      is pushed out, the speed is kept, corners are square.
   4. **Slopes and stairs going down.** He stays on them. The old game: he falls down them in small hops.
   5. **Real heights.** Every block has its real height: a jump that reaches a top lands on it. In Walk's street
      corner he gets onto the 1.0 m cover with less of a jump, and onto the houses' roof edges (2.6 m, under his
      3.07 m reach). The old game: walls and houses are endless in height; only platforms, ramps and cover can be
      stood on.
   6. **Q aims too**, beside the right button (4d). The old game: the right button only.
   7. **The sights camera** tips toward the point the cursor marks in the resting view, so it settles, and never so
      far that his feet leave the screen (4d). The old game: it tips toward the live aim point, to a fixed floor.
   8. **Listed only: nothing he feels today.** Two values for speeding up and for stopping, and one for steering in
      the air (the old game: one value, full steering; his Export has the old game's values in all three). The
      head bumps a block above (nothing in Walk's scene hangs overhead). The map's edge is the map's own bounds and
      the speed into it is dropped (the old game: the ground less 1 m, the speed kept).
   Not in these rows, because his START-RULES already decided them: the jump's reach of 3.07 m, which the content
   holds as the reach (the take-off speed is worked out from it), and the soft push.
   - (a) All eight rows as written above (row 2 without the sandbox's pull onto the block).
   - (b) The rows he names; the rest stay as the old game.
   - (c) The old game's rule in every row.
   - Recommended: **(a)**. He has played them, and his values were tuned under them. Rows 3, 4 and 5 follow from
     moving a real 3D body by the collision library's queries (the build plan's Walk card, his go of 17 November):
     the old game's rule for them would have to be built against the library on purpose.
7. **RESUME-FLASH: what does the first picture after a pause show?** It matters because he saw a key box stay lit
   for a moment in 3b's check, and a soldier now stands where that box was. Needed before 4c.
   - (a) The match as it stood when the tab hid; one tick later the keys count as let go and the soldier slows to a
     stop over about half a metre. No rule changes.
   - (b) As (a), and the development key diagram goes dark the moment the tab hides, so what he saw in 3b is gone.
   - (c) Keys held when the tab hid count as held until his first input: not recommended (it undoes 3b's rule that
     a hidden tab lets every key go).
   - (d) The soldier stops dead at the return: not recommended (the hidden tab would become an input of the
     simulation).
   - Recommended: **(b)**. netcode-architect, whose design this is, recommends (a), "(b) if he also wants the 3b
     flash gone"; the main session recommends (b) because that lit box is what he saw and asked about in 3b's
     check, and (b) removes it for about 10 lines and one test.
8. **TUNE-HOME: where does he tune once Walk exists?** His point 8g: one home per value. It matters because his
   Export block says "Values go one way, from this Export into the game, and from nowhere else": option (a) keeps
   that rule whole; **a yes to (b) or (c) changes it for the values they move**, which from then on are tuned in
   the game and no longer come from an Export. Needed before 4c.
   - (a) The sandbox stays the home of every value; the game's camera panel is for trying only, and what he finds
     there he sets in the sandbox and exports.
   - (b) One home per group: movement, jump, stamina, pushing and the aiming rule stay in the sandbox (he exports,
     the tool rewrites the content, the check holds them equal); the camera's values start from his Export once
     and then live in the game's camera file, tuned in the panel on the real picture, "copy values" pasted in the
     chat (the build plan's Part 5).
   - (c) Everything moves to the game at Walk.
   - Recommended: **(b)**: the camera is judged on the real picture, and his queued words name the panel as its
     tool. With it the game's camera starts from his Export's values (his Experimental profile), and the five old
     presets are ported at the old numbers, as Part 5 says.
9. **SPRINT-DELAY-4: after a sprint, what waits?** It matters because SLICE-FEEL says "firing and aiming down sights
   each wait a short delay, set per gun", while his Testlines holds back only the gun (aiming slows him and moves
   the camera at once; the gun swings up over its delay and fires only then), and Walk has no gun. Needed before
   4d.
   - (a) As Testlines: aiming starts at once; only the gun waits (its swing up and its first shot, each gun's own
     delay), all of it in change-set 8. Nothing waits in Walk.
   - (b) Aiming waits too, from Walk on: after a sprint the slow walk and the camera start only after one stand-in
     delay (one gun's value from his Export), replaced by each gun's own in change-set 8. Walk then differs from
     Testlines, and the rule must also say that holding the aim button ends the sprint at once.
   - (c) As (a) in Walk; from change-set 8 aiming also waits each gun's delay (his words taken to the letter,
     beyond what Testlines does).
   - Recommended: **(a)**: it is what he has played, and a per-gun value keeps one home.
10. **NO-SOURCE: where do the numbers come from that Testlines keeps in its code and not in its Export?** It
    matters because his point 8b takes values "from my Export and from nothing else", and Walk needs a few numbers
    the Export does not carry. Needed before 4c. The numbers: the steepest slope he can walk; the soft push's four
    limits (how far a head-on push turns sideways; that a push never undoes more than half of a walker's own step;
    the most one step may push; the height gap above which two soldiers do not push); the body's height (the Art
    stand-in's 1.80 m). Not needed in Walk, because its movement has no such number: the sandbox's pull onto a
    block, its foot circle and its allowance for staying on a slope (Walk uses the step height there).
    - (a) His Testlines chat makes them sliders and he exports again; Walk takes them from there. This keeps
      "from my Export and from nothing else" whole.
    - (b) The movement design picks them, starting from the sandbox's own numbers where it has one; each is a row
      of the table marked "not from your Export", lives in the game's content, and he changes it by telling this
      chat. **A yes to (b) is an exception to "from nowhere else" for exactly these numbers.**
    - (c) Drop the rules that need them and keep the old game's (for the soft push that is not possible: the old
      game has none, and START-RULES asks for it).
    - Recommended: **(b)**, so Walk waits on no other chat and each number still has one home (his point 8g).
11. **DEPART-4: does he accept the places where Walk reads or departs from the decision drafts?** As DEPART-3 for
    change-set 3; each is recorded by a dated pointer at the drafts' next revision, never by rewording now. Needed
    before 4b. (a) Accept all; (b) accept with his changes. Recommended: (a). The list:
    - the page keeps the three newest blocks, where the netcode and structure drafts say two (section 6.2);
    - the time the page blends by travels on the block's message, not as a field inside the block (the follow-up
      from change-set 3b's review named NA-N3 worded it as a field);
    - DEPART-3, which he accepted for change-set 3, said of the network draft (its line 221): "three limits are
      code, not data, until the content crate exists in change-set 4". The crate exists from 4b, and the three
      limits stay code until change-set 7, which rebuilds the room's command scheduling behind one seam, so that
      they move once (engine-architect's proposal; netcode-architect confirms it at 4b's re-read);
    - the card's "the follow camera at legacy's framing" (build plan line 346) becomes his Export's framing, with
      the old game's five presets kept at their old numbers (TUNE-HOME);
    - the block has a fixed capacity of 48 soldier rows with the live count in its header, where change-set 1's
      note planned a count and an offset per section;
    - the aim point's 16-bit centimetres reach 327.67 m from the map's origin, so the playable map lies inside that
      square until the codec widens, and the 2 km test (PM-0004-39) runs on the simulation alone;
    - the build plan's line 470 places the recording's WebAssembly half in CI later: it runs in today's CI at 4a;
    - the movement table's values come from his Export, where the card says "from the legacy tuning import"; the
      import maps and reports, and PM-0005-14's "every kept value equal to legacy's" reads as every value his Export
      does not replace;
    - the three readings of section 9 (PM-0004-15, -17, -59), and three smaller ones: PM-0004-33's size "after
      wasm-opt" is recorded without it (the toolchain has no such tool yet); PM-0004-32's tree is read node by node
      through the library's own public walk; the index of moving soldiers is a pair loop in slot order (48 at most),
      not a tree;
    - the build plan's row "`cargo tree` shows `sim` depends on `content` only" (Part 7) reads as content and the
      collision library's tree;
    - the simulation draft leans on the library's `enhanced-determinism` flag alone for the same result on every
      target; by rust-reviewer's reading it also takes the library's `std` off, typed calls only and the built-file
      check (section 5.2); and the card's "with legacy's numbers" keeps the old numbers but not the old game's order
      of the jump's two sums (section 6.1);
    - the easing of the speed is the old game's and the sandbox's (an exponential): its share per tick is worked
      out once per match with the pinned maths library's exponential, which gets its own vectors (the draft allows
      such maths only through that library);
    - under CONTENT-SOURCE (c): the content draft's serde_json, garde and schemars are not used (its own fallback
      for garde is hand-written checks), and PM-0005-7's "garde range" reads as the field table's range;
    - the collision library's own `std` feature is off if that builds, so its maths goes through the pinned `libm`
      on both targets; and it runs std's `min`, `max` and `clamp` inside itself, which the float rules ban only in
      our own code (section 5.2).

## 14. Kit, gate and CI
- **No kit, gate or CI file is written in any part** (his words: CI edits belong to the CI Builder chat). The legacy
  gate (`tools/gate.mjs`) stays untouched and green.
- **Requests for the CI Builder chat, written out when each part's numbers exist:** if the browser run nears its 5
  minutes (3.4 used today; 4a, 4c and 4d add specs), a raise of that limit; the tools' own `--check` modes
  (`make_recordings`, `testlines`, `content-bundle`) beside the vector generators' (3A-GEN-CHECK), though a plain test
  already runs each check in today's jobs; `content` in the release test line.
- The root `Cargo.toml` takes the collision library's line and the overflow lines in 4b. If the guard protects that
  file by then, the file is **risk tier 2** in that pull request (build plan lines 219-222): each such edit asks him
  first with its exact text, with the chat in Accept edits, and security-auditor reviews it, as it reviews every
  new dependency in any case.

## 15. Follow-ups placed
| Id | Where |
|---|---|
| 3A-FMA-CHECK, 3A-FEATURE-VERSION, 3B-HOSTILE-ALLOC (4a) | 4a and 4b, as above |
| 3A-SCAN-HELPERS (the scan and our three float helpers), NA-N3, H2, 3B-ONE-PER-TICK (movement's half), 3B-POOL-16, 3B-CSS-PROD, 3B-BLUR-TEST, 3B-RESUME-FLASH | 4c |
| 3A-RULES-STEP (the tools crate's first native-only dependency must be target-gated) | not met in Walk: no such dependency enters; said in each report |
| 3A-PART5-CLASH (the build's feature compare would refuse the feature the structure draft plans for generated types that nest) | 4b's step 3, if the content's types are the first that nest; decided there with engine-architect |
| OQ-0003-10, the build plan's open question at change-set 4: smoothness for each frame-cap option, for his pick | 4d's step 3: the readings go into 4d's report as a decision for him |
| PM-0004-34, the comparison against a library's character controller | not run unless our own movement misses its marks (his COMPARE (b)); said in 4c's report either way |
| 3B-KEY-POST, 3B-ALLOC-METHOD | change-set 5, as placed |
| 3B-LATE-BOUND: per-command movement makes its doubled rate real movement over a network | change-set 7, unchanged: no room goes behind a network transport before it |
| New, HOP-WATCH (game-designer): a jump costs nothing in either game, so a soldier hopping without pause may be hard to hit once shots aim at the cursor's point; measured over several runs before anyone calls it strong; any answer is a new rule for his word | change-set 8's plan |
| New, AIM-HEIGHT: LOOP_V2 section 9's open item (shots at height, aim help) stays open; Walk's aim point only turns the facing | change-set 8's plan |
| New, SIGHT-LINE-8E: the sandbox's sight line differs from the old game's (it ends where the shot stops): his word under point 8e | change-set 8's plan |

## 16. Risks
1. **Size.** These are estimates made the way change-set 3's were, which ran 1.7 and 1.8 times. Each part has its
   stop line and a ladder fixed before its step 1; the count is reported at each step. The same growth, applied to
   the rest of the build plan, would make Slice 1 several times its 28,200 lines: this plan re-plans no later
   change-set, it only says so.
2. **The collision library's tree** (section 5.2): one security round on 33 packages; its share of the players'
   file; and the built-file check's first result, which can stop 4b for his decision.
3. **Not yet tried, each a first step of its build step:** the library built for both targets with its `std` off;
   our own number text against `JSON.stringify` on 12,038 numbers; 13 numbers through one bindings call with no
   allocation in the glue; the Worker stamping each tick's due time inside a catch-up; ts-rs on the content types;
   the tree's bytes compared across targets; the instruction walker on a real players' file.
4. **CI's browser run** may pass its 5 minutes (a CI edit, the CI Builder's, his yes first).
5. **The allocation test's per-frame limit** with a longer input message: measured again, never loosened by guess.
6. **Two copies of the static world in one WebAssembly instance** (the room's and the endpoint's): small for the
   test scene; measured.
7. **His waits.** Four parts mean four gos; while a part waits for him, this chat cannot build the next one on its
   one branch.

## 17. Change type, risk tier, evidence and his checks
Game and engine code, tests and the page: **risk tier 1** in every part; no kit, gate or CI file. Two files are
named apart: any change under `content/source/` is risk tier 1 and needs his go (section 11b), and the root
`Cargo.toml` is risk tier 2 once the guard protects it (section 14). Evidence by kind:
logic by tests on the real code, each with its planted defect, run by a plants script as in 3a and 3b; netcode by
deterministic tests (the recording on four runs; no link simulator before its own change-set); performance by
numbers with their method (the players' file's size at each step; allocations counted; the Worker's lateness and the
smoothness as readings on his screens); what a player sees by screenshots that name their backend, then his eyes.
- **4a:** no browser check. **4b:** a short check in Brave and once in Firefox (about 5 minutes: the scene, "errors:
  0", the content rows, "Copy report"). **4c:** his main check (about 35 minutes, Brave then Firefox), each step
  written as what he will see: walk, sprint until the stamina locks, jump and held jump, the two steps, the three
  ledges, the cover wall and a roof edge, a ramp up and down, the thin wall at a sprint, aim on the ground, on a
  ledge and on a ramp, the camera panel and its five presets with "copy values", **the tab hidden for 1 minute and
  once for 6 minutes while walking** (PM-0002-34, as the build plan's line 602 and the structure draft's checklist
  have it), a covered window, the replay button on his 60 Hz and on his 144 Hz screen (it must say the hashes are
  the recording's: PM-0004-28), 2 minutes of walking the lane for the smoothness numbers, "Copy report". **4d:** a
  short check (about 10 minutes): the aimed walk, no sprint while aiming, the camera's sights mode by preset, the
  frame cap's options. The numbered lists are written with each part's zip.
- **Reviewers per part** (Opus 5.5, effort max, at most 3 at a time, two rounds at most, the second on what
  changed): 4a as section 4. 4b: rust-reviewer, engine-architect, security-auditor (the manifest batch, the reader
  of content bytes, the page's new fetch); then test-engineer, graphics-engineer (the page's scene), perf-auditor.
  4c: rust-reviewer, netcode-architect, engine-architect; then graphics-engineer (the page's drawing code),
  test-engineer, security-auditor; then game-designer (the rules against his words), perf-auditor. 4d:
  netcode-architect (the command, the snapshot and the block gain the sights flag), rust-reviewer, game-designer;
  then test-engineer, security-auditor, engine-architect; then perf-auditor (the frame cap, the Worker's paced
  replay under it). In every part the verifier until PASS, the advisor agent on every pushed head, docs-keeper.
- **Before 4b's build**, test-engineer and security-auditor read the test list (section 3, item 5): by the advisor
  agent's ruling that read is allowed and is not a review round.

## 18. Not in it
The link simulator (its own change-set, section 8). Other soldiers, walkers and the hand-off's marks (change-set 5).
The look (change-set 6, lane 2). Prediction (change-set 7). Guns, the per-gun delays and spreads, health, deaths,
events (change-set 8). The Hornet (not part of Slice 1). Modes as data (Slice 2). The content bundle's hash and the
tampering cases (change-set 8). Any kit, gate or CI file. Lane 2's second page in the build setup (his word when due).

## 19. Not verified
Nothing of this plan is built or run. Every line count is an estimate. The collision library's tree was read from
the registry on paper, not resolved by cargo. The choices of section 6.2 are the main session's between two
designs; the advisor agent let them stand, and the two architects have not read them. The Export's values are saved
and compared, and none is carried. What the step rate changes (STEP-60) is read from the sandbox's README and code,
not measured. The hours in section 0 and in WALK-CUT are change-set 3b's pace applied to planned lines: a rough
yardstick, not a promise. The names in DIFF-LIST's part C are the main session's reading of the Export against its
own old-game names; the kept tool's table makes that reading exact in 4b.

## 20. The advisor agent's review, and where each of its conditions is met
The advisor agent (Opus 5.5, from main's `.claude/agents/advisor.md`) read draft 1 (f789b24) and re-checked against
draft 2 (c0e6139, 639ce96): **REVISE**, eleven conditions (`advisor-cs4-plan.md`). Draft 2 had already folded in
game-designer's check (MOVE-RULES in eight rows, SPRINT-DELAY-4, NO-SOURCE, the stamina bar). Draft 3 answers the
eleven:
| # | Its condition | Where it is met |
|---|---|---|
| 1 (High) | His condition (a) was claimed and not met: proofs of 4b's and 4c's code sat in 4d, 4c's ladder moved two more there, and 4d changed the simulation without a new recording | Content items 1 and 2 with PM-0005-2, -4, -6, -7 are 4b's step 3b; PM-0004-28, -60 and -39 are 4c's steps 6b and 6c; PM-0004-17, -28, -15 and -60 are on 4c's never-moved list; 4c's ladder moves only the far-corner map, which the card itself names; 4d's step 1 makes the recording anew; PM-0004-25 stays in 4d as a reading that cannot fail. Sections 0, 5, 6, 7 and 9; every sum made again |
| 2 (High) | The early look without the camera panel narrowed his words "from its first playable build" | The early look is built after the panel (4c's step 5): sections 0, 6 and 6.3 |
| 3 | DEPART-4 missed DEPART-3's three limits and the card's "legacy's framing" | Both are in DEPART-4 |
| 4 | WALK-CUT: say that (b) is the shape his point 3 names; keep 4a apart in (c); give the times; fix the sums | WALK-CUT, rewritten |
| 5 | TUNE-HOME and NO-SOURCE give values a home in the game, against "from nowhere else" | Both quote his words and say what a yes changes |
| 6 | DIFF-LIST must carry the list he confirms | The list is in the question, printed by `testlines-difflist.mjs` |
| 7 | STEP-60: why it matters, and which values depend on the step rate | STEP-60, rewritten |
| 8 | His Export, its manifest and the table must not live under `docs/` | `content/source/`, and any change there is risk tier 1 with his go: sections 5, 5.1, 11 and 17 |
| 9 | 4c's checklist must hide the tab for 1 and for 6 minutes | Section 17 |
| 10 | Reviewers: graphics-engineer in 4b; netcode-architect and perf-auditor in 4d | Section 17 |
| 11 (Low) | Twelve smaller points | MOVE-RULES (a) names the pull left out, and the reach is out of its rows (settled by START-RULES); `content/camera/`'s owner (5.1); 3A-PART5-CLASH, OQ-0003-10 and PM-0004-34 placed (sections 9 and 15); stop lines are planned times 1.25 (section 0); recordings on frozen content (6.2); the hand push and the dependency hold (section 3); the early look's README (6.3); each part's re-read lists its symbols (section 3); 4c's step 4 at engine-architect's own sum; the root `Cargo.toml` as risk tier 2 (sections 14 and 17) |
Its rulings, taken: the six choices of section 6.2 stand; recordings run on frozen content copies; the read by
test-engineer and security-auditor before 4b is not a review round; GREY-COLOURS is a default he can veto. It
withdrew its own first ruling on what waits after a sprint: his words and his sandbox point two ways, so
SPRINT-DELAY-4 is his.

## For Mustafa
1. Your go on this plan, with a letter for WALK-CUT. On it, 4a starts.
2. Before 4b: a letter for STEP-60 (and the new Export if (a) or (c)), your yes to DIFF-LIST, and letters for
   CONTENT-SOURCE and DEPART-4. GREY-COLOURS needs no answer unless you want other colours.
3. Before 4c: letters for MOVE-RULES, NO-SOURCE, RESUME-FLASH and TUNE-HOME.
4. Before 4d: a letter for SPRINT-DELAY-4.
5. The Hornet's entry clashes with two lines of `docs/UNITS.md` section 0 (section 4's last bullet): nothing is
   settled; they are listed for you.
