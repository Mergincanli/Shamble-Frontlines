# Plan (draft 2): change-set 4, "Walk", proposed as four change-sets, 4a to 4d

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
  lines 250 and 334-358) plans 3,900 lines. The designs, added up from the bottom, come to about **15,000 lines**,
  nearly half of them tests. At the ratio change-sets 3a and 3b ran at (1.7 and 1.8 times their plans), that is about
  **27,000 lines**: nearly four times change-set 3b. Why: the card was written before ORDER-3C put the recording
  in (about 1,365 lines, where change-set 3's plan had 275), before SLICE-FEEL and his Testlines values; its one line
  on content hides a reader, a writer, a schema and three tools; and each pass mark the build plan places at Walk
  (about fifty) needs its own test and planted defect.
- **So this plan proposes four change-sets, each its own pull request with his go, in this order** (question
  WALK-CUT):
  | Part | Name | What he sees or plays | Bottom-up | Planned at 1.8 | Stop line |
  |---|---|---|---|---|---|
  | 4a | The recording | nothing new: the page as 3b left it | 1,365 | 2,450 | 3,050 |
  | 4b | Content and the world | the grey street corner and training yard, drawn from the game's first content files | 4,860 | 8,750 | 10,900 |
  | 4c | The first soldier | **he plays**: walk, sprint, jump, step up, aim with the mouse, the follow camera and its tuning panel | 6,665 | 12,000 | 15,000 |
  | 4d | Sights, and the proofs that are left | hold the right button: the slow aimed walk and the camera's sights mode | 2,120 | 3,800 | 4,800 |
  After 4d, and before change-set 7 starts: **the link simulator's own small change-set** (about 700 bottom-up).
- **His fallback (d) is used, and this is the telling before anything moves:** the recording is too large to ride
  inside the first soldier's stop, so it goes first as its own change-set, 4a. It needs no browser check (nothing a
  player sees changes), and it gives his condition (b) by construction: nothing of movement exists when it merges.
- **The earliest he can play:** an early-look zip inside 4c, as soon as the soldier walks and turns to the mouse on
  screen (after step 4 of 4c's 7), before the camera panel, the big test runs and the reviews. It is a look, not a
  check: nothing is asked of him. The judged build comes at 4c's end.
- **No proof gets thinner** (his condition a): section 9 places every mark the build plan puts at Walk, each in the
  part that merges the code it proves; every mark on the recording is met on the movement recording before 4c merges.
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
side; the collision world is built from the map and is identical natively and in WebAssembly; the built file is
checked for a fused multiply-add on every build. After 4c: one soldier, moved by commands through the real messages
on our own movement over the collision library, drawn between two ticks, followed by the camera; the movement
recording replays identically on every target. After 4d: aiming down sights by SLICE-FEEL, and every remaining mark
the build plan places at Walk.

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
   (section 13).
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
   `claude/cool-pasteur-oiqk1r`): the main session commits the notes by hand at each step and whenever agents run,
   and says so in each report.
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
  the Hornet is a vehicle and tier one (he has said that line stays as it is).

## 5. Change-set 4b, "Content and the world"
Symbols and tests in full: `engine-architect-cs4.md`, blocks 1 and 3.6 to 3.7; `rust-reviewer-cs4.md`, blocks 1, 2
and 4.
| Step | What | Main files | Code / tests |
|---|---|---|---|
| 0 | The one manifest batch (section 3, item 3): the new `content` crate; `sim` on `content` and on the collision library; the 31 overflow lines the build's own check asks for; the tools crate's edges that still have no user dropped (section 8, item 22) | every `Cargo.toml`, `Cargo.lock` | about 110 lines of configuration |
| 1 | **The built-file check for the fused multiply-add** (section 5.3), with its planted defects, before the collision library is used by anything; the scan's two new rules; the feature compare by name and version (3A-FEATURE-VERSION) | `packages/client/scripts/build-checks.mjs`, `build.mjs`, `crates/sim/tests/rules_scan.rs` | 340 / 320 |
| 2 | The content crate's reader and writer: JSON to a tree with line and column, numbers, one canonical writer, a typed reader with field paths, ranges and length bounds from one field table, the schema, errors that name the file | `crates/content/src/` (`json.rs`, `number.rs`, `write.rs`, `read.rs`, `field.rs`, `schema.rs`, `error.rs`) | 790 / 500 |
| 3 | The bundle (one file set, whole-set checks, frozen), and the two kinds of this part: the movement table and the map document | `bundle.rs`, `movement.rs`, `map.rs`, `ts_export.rs` | 360 / 290 |
| 4 | The integer time unit, 3,000 units a second (100 per tick; every duration of the old game is a whole number of milliseconds and converts exactly), with the rounding report over a reviewed list of the old game's durations (PM-0004-11) | `crates/content/`, `crates/sim/src/time.rs`, a report tool | 240 / 80 |
| 5 | The tools: `content-bundle` (refuses a content file that is not in canonical form), `testlines` (section 11: `--check`, `write`, `compare-old`, `table`) | `crates/devtools/src/bin/` | 250 / 140 |
| 6 | The data: his Export, its manifest and the table under `docs/tuning/`; `content/movement/soldier.json` written by the tool from his Export (after STEP-60 and his yes to the differences list); `content/maps/test-scene.json` written by hand from the Art sheet's sizes and the marks' shapes | data, not counted | |
| 7 | Content reaches the simulation once per match, through the constructor (no new call; nothing per tick changes); the build writes `content.json`; the page fetches it once and hands its bytes to the Worker | `crates/sim-wasm/src/lib.rs`, `local_match.rs`, `scripts/build.mjs`, `packages/client/src/app/content.ts`, `worker-link.ts`, `packages/sim-worker/src/` | 290 / 260 |
| 8 | The static collision world built from the map: every number checked, shapes in id order, the tree built once; its bytes and 10,000 rays identical natively and in WebAssembly (PM-0004-32); the ray query the aim will use | `crates/sim/src/collision.rs` | 260 / 170 |
| 9 | The page draws the scene from the map; the readout's content rows; the browser tests (the scene appears; a broken bundle shows the loader's sentence) | `packages/client/src/render/scene.ts`, `overlay/`, `packages/e2e/tests/` | 230 / 230 |
**4b: about 4,860 lines bottom-up; planned 8,750; stop line 10,900.** Under CONTENT-SOURCE (a) step 2 shrinks to
about 100 lines of glue and the part to about 3,730 (planned 6,700).
- **Its ladder, fixed now:** at 8,750 lines the schema writer (with the panel's limits written in the panel's code
  until then) moves to 4d; at 9,800 the tree's byte dump and its 10,000 rays (PM-0004-32) move to 4c. Never moved:
  the built-file check (his words tie it to the collision library's change-set), the loader's seven stages, the
  Testlines check.

### 5.1 The content crate, in short
One loader with seven stages, each of which refuses by name (a size cap; JSON only, with duplicate keys, NaN and
depth beyond 32 refused; a format name and a version per file; unknown keys refused; ranges and length bounds; the
whole set: one definition per id, every reference resolves, no fallback; then frozen). One canonical writer (keys
sorted, the number text a browser writes, so `JSON.stringify` and the writer agree on every number but -0). One
definition per file, one folder per kind: `content/movement/`, `content/maps/`, and in 4c `content/camera/`. The
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
| 4 | The page: the plain stand-in at the Art brief's sizes on one root, the follow camera at his Export's framing, the keys and the mouse (pointer, move, the ray), a plain stamina bar, the readout's new rows. **The early-look zip is built after this step** | `packages/client/src/soldier/`, `camera/follow.ts`, `input/`, `app/`, `overlay/`, `picture.ts` | 600 / 540 |
| 5 | The camera panel: sliders, the five old presets, "copy values"; the camera's content file; development-only code kept out of the production build (3B-CSS-PROD) | `packages/client/src/camera/panel.ts`, `settings.ts`, `crates/content/src/camera.rs`, `content/camera/` | 360 / 300 |
| 6 | The movement recording (about 2,400 ticks on the street corner and the yard) and every mark that rides it, with the three planted divergences run again on movement; the first recording re-made | `crates/devtools/`, `recordings/` | 320 / 230 |
| 7 | The browser tests of the page (walk, jump, the two steps, aim on a ramp, the camera follows, keys let go on blur: 3B-BLUR-TEST, the hidden tab with a soldier, allocation measured again); then the reviews, the verifier, his check | `packages/e2e/tests/` | 0 / 410 |
**4c: about 6,665 lines bottom-up; planned 12,000; stop line 15,000.**
- **Its ladder, fixed now:** at 12,000 lines the two interleaved matches on movement (PM-0004-17) and the SIMD
  reading move to 4d; at 13,500 the stale-id run (PM-0004-15's 10,000 cases) moves to 4d. Never moved: the movement
  recording on its four runs with PM-0004-19, -21 and -24, the parity marks PM-0004-30 and -59, the camera panel (his
  words: "from its first playable build").
- **The camera panel and the early look.** His queued words ask for the panel "from its first playable build". The
  early look of section 6.3 comes one step before the panel, to reach him sooner; the first build he judges, and the
  first that merges, has it. If he wants the panel in the early look too, steps 4 and 5 swap their order and the
  early look comes one step later.

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
  as two times. (6) The content holds the jump's **reach**, as START-RULES names it and his Export carries it, and
  the take-off speed is worked out from it (engine-architect's table stored the speed; game-designer raised it as a
  question: it is a sandbox difference, asked in MOVE-RULES, row 8). The reviewers confirm these at 4c's steps 1
  and 3; if one is rejected it goes back to the advisor agent.
- **The blend between two ticks** (follow-up NA-N3): the Worker stamps each block with the time its tick was due; the
  page picks the pair of blocks and the share between them, never looks ahead, and places (never slides) a soldier
  whose id changed. netcode-architect's design keeps the **three** newest blocks instead of the drafts' two, because
  with two, the Worker's lateness he reported (14 ms at the 99th percentile) breaks the 5 % smoothness mark on a
  144 Hz screen: a departure listed in DEPART-4, proven by a test of the blend alone that must fail with two.
- **What the first picture after a pause shows** (3B-RESUME-FLASH, his decision: question RESUME-FLASH).

### 6.3 The early look (not a check)
After step 4: a zip of the development build from a commit on the branch, with a README that says it is an early
look, what is not in yet (the panel, the big test runs, sights) and that nothing is asked of him. It runs on the
values of the Export he has cleared by then (STEP-60).

## 7. Change-set 4d, "Sights, and the proofs that are left"
| Step | What | Code / tests |
|---|---|---|
| 1 | Aiming down sights by SLICE-FEEL (section 10): the right button held (and Q on his word, MOVE-RULES row 6), 0.55 of the speed, no sprint, the flag in the snapshot and the block (lane 2's W1), the camera's sights mode | 140 / 150 |
| 2 | Content's remaining proofs: every number of the old tuning and 10,000 made ones parse to the same bits natively, in WebAssembly and in `JSON.parse` (content item 1); 10,000 broken files refused the same way everywhere and a bounded run of 1,000,000 random inputs with no panic (content item 2); the walk over every leaf of the old tuning with its report (PM-0005-14); a saved content file reaching the Worker within a second in development (PM-0005-10, PM-0002-31) | 320 / 920 |
| 3 | The yard at the far corner of a 2 km map (PM-0004-39); 2,000,000 hostile movement commands with no panic (PM-0004-60) | 30 / 180 |
| 4 | The paced replay in the real Worker (presentation never reaches the simulation: PM-0004-28), the frame-cap setting and the smoothness reading per option (PM-0003-42), the lane recording, the SIMD reading (PM-0004-25) | 210 / 170 |
**4d: about 2,120 lines bottom-up; planned 3,800; stop line 4,800.** Its ladder (the card's own two first, build plan
lines 355-356): the far-corner map, then the size comparison's subject, to a follow-up before Slice 2; he is told
before a part moves.

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
| 4b | ADR 0005's foundation line (line 461); the content rules CD-1 to CD-5 shown on movement and the map; PM-0005-1, -3, -5, -8; PM-0004-11 (the time unit); PM-0004-32 (the collision tree byte for byte); PM-0004-33 (our own size, before and after the library and content); PM-0002-29 (the file's weight); 3A-FMA-CHECK; PM-0004-23's "a build check fails on a std transcendental call" for every linked crate |
| 4c | **On the movement recording, before 4c merges** (his condition a): ADR 0001's replay line (line 485), the recordings of line 484, PM-0004-19 (line 494), -17, -20, -21 (three of three, on movement fields), -24. On the movement itself: PM-0004-6, -15, -22 (run again: the two new routines), -30, -35, -54, -59; START-RULES' two rules. On the path and the page: PM-0002-14 (the aim in the post), PM-0002-26's smoothness (the blend's own test; a reading on his displays), PM-0002-34's walking steps (his checklist); 3B-ONE-PER-TICK; lane 2's rows of section 12 |
| 4d | Content items 1 and 2 (lines 503-504) with PM-0005-2, -4, -6, -7; PM-0005-10 and PM-0002-31; PM-0005-14; PM-0004-25 (a reading), -28, -39, -60; PM-0003-42 |
| Lane 2, after 4c merges (Part 15.2) | the two size comparisons (PM-0004-33's comparison builds; PM-0005-16's size half) |
Three readings of a mark that the plan states (DEPART-4): PM-0004-15 by our own seeded generator instead of the tool
the row names; PM-0004-17's "different seeds" as two different input streams, because nothing draws a random number
before change-set 8; PM-0004-59's planted defect as the old game's order of the two sums, because the formula alone
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
  start. In 4b it moves, byte for byte, to `docs/tuning/` with its manifest.
- **c.** One table, a data file (`docs/tuning/testlines-table.json`), printed into the docs by the tool: each value
  Slice 1 uses, its name in the Export, its name in the content, its unit, the change-set that uses it and its home.
  The first draft by names and units is in `testlines-cs4.md`, section 3. **The comparison is run** (by script, on
  the saved file against `docs/legacy-tuning.json`): 83 old-game names, 60 equal, **23 different**, none without a
  counterpart (`testlines-compare.txt`). The 23: the Conscript's sprint multiplier (1.214 in the old game, 1.52 in
  his Export); sixteen camera values (his Experimental profile against the old camera); the SMG's recoil and four
  spread values (all 0 in his Export); the grenade's launch angle (41.253 degrees in the old game, 41.3 in the
  Export: the sandbox's own rounding). Beside them, values with no old number in the old game's tables (the body
  radius 0.4 m, where the old game's 0.7 m is written into its code and so in no table; the push strength and
  radius; the three sprint-to-fire delays; the active-reload windows; the aiming camera's floor of 36 degrees,
  where the old game's code has 28.5). Question DIFF-LIST asks his yes to both lists before any becomes the
  starting table.
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
1. **WALK-CUT: how is Walk cut?** It matters because he wants to play the first soldier as soon as that is safe,
   and one stop of about 27,000 lines would hold it behind everything. Needed now.
   - (a) Four change-sets as section 0 has them: the recording, content and the world, the first soldier, sights
     and the rest. He plays in the third, with an early look in its middle.
   - (b) Three: 4b and 4c built as one stop (about 20,650 planned lines), so the early look comes without waiting
     for 4b's reviews and his go; that one stop is then three times change-set 3b.
   - (c) Two, as the card's spirit: everything up to the first soldier in one stop (about 23,100 planned), then
     sights and the rest.
   - Recommended: **(a)**. Each part can be reviewed whole, the outside library is checked before movement rests on
     it, and the wait it adds before the early look is one round of reviews and his go on 4b.
2. **STEP-60: what to do with an Export made at 60 steps a second** (asked in the chat). Needed before 4b's step 6.
   (a) He plays the sandbox at 30 and exports again; (b) carry this one and judge at 30 in the game; (c) carry only
   the camera's values now. Recommended: (a).
3. **DIFF-LIST: his yes to the two lists of section 11c** (the 23 differences from the old game, and the values
   with no old number). It matters because by his point 8c they are his tunings and become the starting table only
   on his word. Needed before 4b's step 6. (a) Yes to all as the Export has them; (b) yes with the rows he changes.
   Recommended: (a), after STEP-60.
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
5. **GREY-COLOURS: what colours do the grey scene and the plain stand-in take before the look exists?** The look is
   his. Needed before 4b's step 9. (a) LOOP_V2 section 9's own default, "plain blocks, with height read through
   brightness (higher is lighter)", in neutral greys, the stand-in one mid grey with a darker gun; (b) the Art
   lead's palette proposal and the brief's faction colours as placeholders; (c) one flat grey. Recommended: (a).
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
      head bumps a block above (nothing in Walk's scene hangs overhead). The jump is tuned by its reach, as
      START-RULES names it, and the take-off speed is worked out (the old game: the other way round; the same jump
      today, they part when gravity or the step height is tuned). The map's edge is the map's own bounds and the
      speed into it is dropped (the old game: the ground less 1 m, the speed kept).
   - (a) All eight rows, as Testlines has them.
   - (b) The rows he names; the rest stay as the old game.
   - (c) The old game's rule wherever they differ.
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
   - Recommended: **(b)**.
8. **TUNE-HOME: where does he tune once Walk exists?** His point 8g: one home per value. Needed before 4c.
   - (a) The sandbox stays the home of every value; the game's camera panel is for trying only.
   - (b) One home per group: movement, jump, stamina, pushing and the aiming rule stay in the sandbox (he exports,
     the tool rewrites the content, the check holds them equal); the camera's values start from his Export once
     and then live in the game's camera file, tuned in the panel on the real picture, "copy values" pasted in the
     chat (the build plan's Part 5).
   - (c) Everything moves to the game at Walk.
   - Recommended: **(b)**. With it the game's camera starts from his Export's values (his Experimental profile), and
     the five old presets are ported at the old numbers, as Part 5 says.
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
    - (a) His Testlines chat makes them sliders and he exports again; Walk takes them from there.
    - (b) The movement design picks them, starting from the sandbox's own numbers where it has one; each is a row
      of the table marked "not from your Export", lives in the game's content, and he changes it by telling this
      chat.
    - (c) Drop the rules that need them and keep the old game's (for the soft push that is not possible: the old
      game has none, and START-RULES asks for it).
    - Recommended: **(b)**, so Walk waits on no other chat and each number still has one home (his point 8g).
11. **DEPART-4: does he accept the places where Walk reads or departs from the decision drafts?** As DEPART-3 for
    change-set 3; each is recorded by a dated pointer at the drafts' next revision, never by rewording now. Needed
    before 4b. (a) Accept all; (b) accept with his changes. Recommended: (a). The list:
    - the page keeps the three newest blocks, where the netcode and structure drafts say two (section 6.2);
    - the blend's time travels on the block's message, not as a field of the block (follow-up NA-N3's wording);
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
  file by then, each such edit asks him first with its exact text, with the chat in Accept edits.

## 15. Follow-ups placed
| Id | Where |
|---|---|
| 3A-FMA-CHECK, 3A-FEATURE-VERSION, 3B-HOSTILE-ALLOC (4a) | 4a and 4b, as above |
| 3A-SCAN-HELPERS (the scan and our three float helpers), NA-N3, H2, 3B-ONE-PER-TICK (movement's half), 3B-POOL-16, 3B-CSS-PROD, 3B-BLUR-TEST, 3B-RESUME-FLASH | 4c |
| 3A-RULES-STEP (the tools crate's first native-only dependency must be target-gated) | not met in Walk: no such dependency enters; said in each report |
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
Game and engine code, tests and the page: **risk tier 1** in every part; no kit, gate or CI file. Evidence by kind:
logic by tests on the real code, each with its planted defect, run by a plants script as in 3a and 3b; netcode by
deterministic tests (the recording on four runs; no link simulator before its own change-set); performance by
numbers with their method (the players' file's size at each step; allocations counted; the Worker's lateness and the
smoothness as readings on his screens); what a player sees by screenshots that name their backend, then his eyes.
- **4a:** no browser check. **4b:** a short check in Brave and once in Firefox (about 5 minutes: the scene, "errors:
  0", the content rows, "Copy report"). **4c:** his main check (about 25 minutes, Brave then Firefox), each step
  written as what he will see: walk, sprint until the stamina locks, jump and held jump, the two steps, the three
  ledges, a ramp up and down, the thin wall at a sprint, aim on the ground, on a ledge and on a ramp, the camera
  panel and its five presets with "copy values", another tab for 30 seconds while walking, a covered window, 2
  minutes of walking the lane for the smoothness numbers, "Copy report". **4d:** a short check (about 10 minutes):
  the aimed walk, no sprint while aiming, the camera's sights mode by preset. The numbered lists are written with
  each part's zip.
- **Reviewers per part** (Opus 5.5, effort max, at most 3 at a time, two rounds at most, the second on what
  changed): 4a as section 4. 4b: rust-reviewer, engine-architect, security-auditor (the manifest batch, the reader
  of content bytes, the page's new fetch); then test-engineer, perf-auditor. 4c: rust-reviewer, netcode-architect,
  engine-architect; then graphics-engineer (the page's drawing code), test-engineer, security-auditor; then
  game-designer (the rules against his words), perf-auditor. 4d: test-engineer, rust-reviewer, game-designer; then
  security-auditor, engine-architect. In every part the verifier until PASS, the advisor agent on every pushed
  head, docs-keeper.

## 18. Not in it
The link simulator (its own change-set, section 8). Other soldiers, walkers and the hand-off's marks (change-set 5).
The look (change-set 6, lane 2). Prediction (change-set 7). Guns, the per-gun delays and spreads, health, deaths,
events (change-set 8). The Hornet (not part of Slice 1). Modes as data (Slice 2). The content bundle's hash and the
tampering cases (change-set 8). Any kit, gate or CI file. Lane 2's second page in the build setup (his word when due).

## 19. Not verified
Nothing of this plan is built or run. Every line count is an estimate. The collision library's tree was read from
the registry on paper, not resolved by cargo. The choices of section 6.2 are the main session's between two
designs; the two architects have not read them. The Export's values are saved and compared, and none is carried.
The advisor agent read draft 1 (f789b24). Draft 2 differs from it by game-designer's check, folded in after that
read began: sections 2, 6.1, 6.2 (choices 4 to 6), 10, 11c, 12 and 15, questions MOVE-RULES (rewritten),
SPRINT-DELAY-4 (was ADS-DELAY, now three options) and NO-SOURCE (new), a plain stamina bar in 4c's step 4 (40 lines
more, so 4c is 6,665 bottom-up, 12,000 planned, stop line 15,000), and the total (about 27,000).

## For Mustafa
1. Your go on this plan, with a letter for WALK-CUT. On it, 4a starts.
2. A letter for STEP-60 (and the new Export if (a) or (c)), your yes to DIFF-LIST, and letters for CONTENT-SOURCE,
   GREY-COLOURS and DEPART-4: before 4b.
3. Letters for MOVE-RULES, NO-SOURCE, RESUME-FLASH and TUNE-HOME: before 4c. A letter for SPRINT-DELAY-4: before
   4d.
