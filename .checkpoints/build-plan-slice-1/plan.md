# Plan: foundation steps 2.4 and 2.5 and Slice 1, move and shoot (revision 2)

_Written by the build chat (session_019YNMUV9LFhASseJ9EP12kL, Mustafa's personal account, main model Fable 5.2) on
2026-11-14, for Mustafa and the Advisor. Base: main 4460ef5. His task and his addition of 15:03 stand word for word in
`mustafa-2026-11-14-task.md` and `mustafa-2026-11-14-addition.md` beside this file (copied by script from the
transcript). This is a plan: nothing is built, and no file outside `.checkpoints/` is edited. Revision 2 folds in the
advisor agent's review of revision 1 (Part 14) and the three Art sheets he sent during it (Part 4)._

## In one screen
- **The cut.** Twelve change-sets from step 2.4 to the end of Slice 1, plus two that stand beside them: a short
  research unit first (the rules a model file must follow, for the Art project's Blender brief), and the change-set in
  which the first Blender model replaces the code-built soldier, which takes its place as soon as the model exists.
  Each merges on its own, one pull request at a time, on your go.
- **The Art project's first three sheets are in** (the Conscript in three factions, the Conscript as a code-built
  stand-in with its sizes and four materials, the street-corner test scene): Part 4 says what the plan takes from
  them and what is still needed.
- **What you can open, and when.** Change-set 1 has nothing to open (shared data descriptions only). Change-set 2 is
  the first thing you open in Brave: an empty lit ground that names its graphics backend and counts ticks. From
  change-set 4 on you play: walk (with the live camera panel), then twelve soldiers, the comic look, fake lag, shooting,
  grenades, the body's movement, the guns' punch, and a closing target course.
- **Measured first, before more code rests on it:**
  1. the same result natively and in WebAssembly from the first tick: change-set 3 (the first recording, the maths
     functions and the random streams, before any formula uses them), then again on real movement in change-set 4;
  2. the cost of the hand-off between the page and the simulation: a first reading in change-sets 2 and 3, the mark at
     12 players in change-set 5;
  3. the frame time on the weakest machine with the look on: change-set 6, at 12, 24 and 48 soldiers;
  4. beside your three: the Worker's pacing and the press-to-tick delay (2 and 3), one WebAssembly instance or two (3),
     the time unit every timer uses (4), our own movement on the collision library (4), and the cost and exactness of
     prediction (7).
- **No mark is dropped.** Part 3 places every pass mark and open question of the six tests you named: 201 marks,
  counted by script, and Part 12 places the asset spike's 7. Most are measured on the real game at a named change-set
  or slice. A few compare the game against something it does not build (one instance against two, the transferred
  block against the shared ring, our movement against a library's controller): those stay small separate tests, each
  named with its place.
- **Three things nobody has done yet stand in the way of a "pass"** on any timed mark: the minimum laptop, the
  recommended desktop and the Mac are not named. Until they are, timed results are readings on your laptop, reported
  and never judged (your words of 2026-11-03).
- **Step 2.3 and this work.** I need one of its change-sets merged before I build: the docs change-set that records
  your decision of today. I would like its lint and format settings merged before change-set 2, and I do not wait for
  them. Until the gate v2 exists I run its checks by hand on every change-set and show the output. The gate and CI
  checks that the 2.3 plan lets "ride" steps 2.4 and 2.5 are gate work, which your words of 2026-11-12 keep with Opus
  5.5: they follow my change-sets from the 2.3 chat (decision GATE-FOLLOW).
- **Every merge needs your go.** `crates/` and `packages/` are outside the places the PR Advisor may merge
  (`.claude/hooks/routine-guard.mjs:84`), and from change-set 2 on each change-set also has a Brave check.
- **How a build reaches your laptop before preview links exist** is the one step not proven yet: a zip through this
  chat's file tool, and one command (Part 8). This plan comes to you as a file to try that route.
- **Estimate.** About 28,000 lines of hand-written code and tests, about 190 hours of session work (building plus
  reviews at effort max), in 14 units. With your checks and gos between them: four to six weeks. These are estimates,
  good to about half as much again either way; each change-set stops and tells you if it passes twice its planned size.

## Part 0. How to read this plan
- **Change-sets are numbered 0 to 12, plus M** (the first model). "2.3" means work of the step 2.3 chat (the gate v2
  and CI, branch `claude/sweet-mayer-grsukw`). "Slice 2" to "Slice 6" are the later slices of ADR 0002's migration plan.
- **Pass marks** keep the drafts' ids (PM-0002-4 is mark 4 of ADR 0002). Their numbers live in each draft's "Pass
  marks" table and are not restated here unless a line needs one; the digests beside this file
  (`marks-0001.md` to `marks-0005.md`) give every mark in one row with its line number.
- **"Met"** means every clause of a mark holds: its numbers, its machines, its run counts, its "in CI". **"Shown"**
  means the same check passed where this chat can run it (this container: native Rust, WebAssembly in Node, Chromium).
  **"Reading"** means a number taken where the mark cannot be judged yet (the stand-in laptop, a smaller scene): it is
  reported, never judged. A mark is met at the change-set or step where its last clause joins.
- **Machines** (the drafts' names): the minimum laptop, the recommended desktop, the Mac for Safari, your PC in Brave
  (which judges and never measures), the reference server (a rented CCX23), and CI's runners (counts and yes-or-no
  results only, never a time).
- **Each change-set still gets its own short plan** before any edit (files, the symbols it adds, risks, evidence,
  reviewers, and the Brave checklist for what a player sees, hears or feels), reviewed by the advisor agent, as
  CLAUDE.md's protocol asks. Your go on this plan covers all of them as long as a change-set's plan stays inside this
  one; a plan that leaves it comes back to you.
- **Risk tier.** Every change-set is risk tier 1: game code and tests; change-set 0 is a decision doc. None edits a
  kit, gate or CI file. One file may change that: once the 2.3 chat's guard change protects the root `Cargo.toml`, a
  change-set that adds a shared dependency line there is risk tier 2 for that file: it asks you first with the exact
  text and takes the security-auditor's review, which new dependencies get in any case.
- **What is only for development and tests.** The scripted walkers, the fake-lag switch, the bench mode with its
  "copy report" button, the camera panel's "copy values" button and the zip's small server serve the marks and your
  checks; none is a feature of the player's game, and each is kept out of the production build. The walkers go when
  Slice 2's bots come. The ink family you do not pick is removed in the change-set after your pick.
- **Evidence by kind of change** (CLAUDE.md, protocol step 4): logic by tests on the real code; netcode by
  deterministic tests with injected delay, jitter and loss (L1); performance by numbers with their method; anything
  visible by screenshots that name their backend, then your eyes in Brave.
- **Every change-set ends** with its report and with docs-keeper's pass over STATUS, ROADMAP and the repository map.
- **Models and effort.** This chat and every builder agent it starts: Fable 5.2. Every other agent: Opus 5.5. Agents
  that review, verify or design game and engine code: effort max, passed by the caller (the agent files still say
  high). The advisor tool stays off. At most 3 agents at a time; the saver runs; a checkpoint after every step.
- **Planned size** is counted in hand-written lines added (Rust, TypeScript, configuration and tests; generated files,
  lockfiles, fixtures and docs left out), read from `git diff --numstat` against the change-set's base. When a
  change-set passes twice its planned size, it stops and tells you in three lines: what grew, why, and the cheaper
  option.
- **Time** is session work: building, then the reviews (reviewers at effort max, at most 3 at a time, within the
  review budget), the verifier and the advisor agent's review of the final head. It leaves out your checks and waits.

## Part 1. The cut into change-sets (your item 1)

| # | Name | What you see or feel | Planned size | Session time | Brave check | Merge |
|---|---|---|---|---|---|---|
| 0 | Model rules (research, docs only) | Nothing to open: the short list of rules for the Blender brief, checked against the exporter's and the loader's documentation | about 150 lines of docs | about 6 h | no | your go (it answers two open questions in a decision draft) |
| 1 | Types (step 2.4) | Nothing to open: the page and the simulation share one generated description of their data, and a test fails when they drift | 600 | about 7 h | no | your go |
| 2 | First page (step 2.5, first half) | An empty lit ground with a metre grid; a corner label names WebGPU or WebGL 2, counts ticks at 30 a second and shows your frame rate | 2,000 | about 13 h | yes | your go |
| 3 | The loop closes (step 2.5, second half) | The same page, alive: press keys and a small key diagram lights up, echoed back by the local server through the real messages; counters show ticks, messages and zero lost presses | 2,700 | about 15 h | yes | your go |
| 4 | Walk | You walk and jump a plain stand-in soldier around the test scene (the Art sheet's street corner, in plain shapes, with a training yard beside it), aim with the mouse, and tune the camera live with sliders and five presets | 3,900 | about 18 h | yes | your go |
| 5 | Twelve | Eleven stand-in soldiers walk patrol loops around you; a bench button prints what the hand-off between page and simulation costs on your laptop, at 12 and at 48 | 1,500 | about 9 h | yes | your go |
| 6 | Look | The same scene and squad in the comic look: cel shading, black ink outlines, sun and soft shadows, the Conscript of the Art sheets built in code; a switch between the two outline methods and a frame-time readout | 2,300 | 17 to 21 h | yes (the look) | your go |
| 7 | Lag | A "fake lag" switch: at 100 ms of lag your own soldier still answers at once, and the others move smoothly a moment behind | 2,200 | about 15 h | yes | your go |
| 8 | Shoot | You shoot the SMG and the rifle at the walking dummies: tracers, a hit marker when the hit is confirmed, health bars, deaths and respawns, an ammo counter and reload | 4,400 | about 23 h | yes | your go |
| 9 | Throw | You throw the frag grenade and fire the bow and the shotgun: arcs, bounces, blasts that hurt by distance | 2,100 | about 13 h | yes | your go |
| 10 | Body | The soldiers move like soldiers: steps, the body reacting to strafing, an aim pose, recoil, the death fall | 1,800 | 13 to 16 h | yes | your go |
| 11 | Feel | The guns get their punch: comic muzzle flashes, tracers, impact puffs, the hit marker and kill confirm, dummies that flinch, camera kick, gun and grenade sounds, fragments and the concussion shake | 2,500 | 17 to 21 h | yes (sight, sound, feel) | your go |
| 12 | Close | A two-minute target course through the scene with a timer and a score, to judge moving and shooting as a whole; the slice's marks measured with everything on | 1,000 | about 10 h | yes | your go |
| M | The first model | The Blender soldier stands where the code-built one stood, in each faction colour | 1,200 | about 12 h | yes (the look) | your go |

Sum: about 28,200 lines and 188 to 199 hours.

**The order and what may move.**
- 0 runs first (Part 11): the Blender brief waits for it. It is research by an Opus agent and one docs pull request,
  and it needs nothing from your PC: its proof by a real export waits for the first file (Part 11), so it cannot hold
  change-set 1. It moves the first page by about half a day; decision RULES-WHO offers the other orders.
- 1, 2, 3 are the foundation's steps 2.4 and 2.5, in the drafts' order. Step 2.5 is cut in two so that the first page
  reaches you one change-set earlier (decision SPLIT-25).
- 4 to 12 are Slice 1. Two tracks run through them: the simulation (4 Walk, 7 Lag, 8 Shoot, 9 Throw) and the picture
  (6 Look, 10 Body, 11 Feel). Inside a track the order is fixed, because each rests on the one before.
- **6 Look waits for three inputs**: your look decisions of 13 and 14 November carried into the repo, the Art
  project's sheets, and a machine to measure on (Part 5). If one is missing when 5 merges, 7 Lag goes first and Look
  follows as soon as they are in. Look must merge before 10 Body and 11 Feel.
- **M takes the next free place after 6 Look** once the model file exists, the rules of change-set 0 are merged and
  you have answered whether a committed model may be public (decision MODEL-PUBLIC). After 10 Body it also moves; before
  10 it stands in one pose. It holds nothing else up.

**Each change-set, in short.** Reviewers are named per change-set; every one of them runs at effort max.

**0. Model rules.** The export and slot-naming part of the asset research (OQ-0005-AS3 and AS4). Part 11 says who,
when and what. No code.

**1. Types (step 2.4).** `packages/types` and its generators.
- Rust: the first message types of `protocol` (`Join`, `JoinAccepted`, `Input`, `Snapshot`, as types; the codec comes
  in 3); the block's layout table in `client-endpoint` (the header first: tick, layout hash, counts); our own small
  generator, which writes the block's offsets, strides, element types and layout hash for TypeScript from that one
  table; ts-rs for the message types, its derives kept out of shipped builds; an export test per crate that
  regenerates into a scratch folder and fails on any difference from the committed files.
- TypeScript: the generated files, committed; the layout-hash check the page runs at startup, with its own test.
- The mark's own planted defects are run once here and shown in the report: a Rust field renamed without
  regenerating fails, a generated file edited by hand fails, and regenerating after the rename passes.
- Not here: wasm-bindgen's own `.d.ts` (build output, arrives with the bindings in 2); content's schema and types
  (arrive with the `content` crate in 4).
- One departure from the draft's wording: ADR 0002 sets ts-rs's output folder in a root `.cargo/config.toml`; while
  the netcode proof test's rule holds, no such file may exist (Part 7), so the export test names the folder itself.
- Reviewers: engine-architect, rust-reviewer, test-engineer.

**2. First page (step 2.5, first half).** The straight path from the simulation to the screen.
- Rust: `sim` gets its world (a tick counter and the seeded random streams), its step and its state hash;
  `client-endpoint` writes the block (header only) into linear memory; `sim-wasm` gets the wasm-bindgen control calls
  (create, mode, one input call and one step call per tick).
- TypeScript: `sim-worker` hosts the Wasm in one module Worker, paces the tick from what the step returns, copies each
  block once into a pooled buffer and transfers it; `client` is the Vite entry: `await renderer.init()` on three.js's
  WebGPU renderer with the WebGL 2 fallback, the empty scene drawn from blocks, the overlay (backend, ticks, blocks,
  frame time, errors, the build's commit), the layout-hash check, the newest input posted once per drawn frame; the
  measurement harness's CPU side (frame times, the renderer's counters, labelled run sets); `e2e`: a Playwright run on
  the built client.
- The zip's small server (Part 8) is built and reviewed here: one file in the `client` package, Node's built-in
  modules only, bound to this computer alone (127.0.0.1), serving only the files of its own folder.
- Measured here: zero Wasm calls by the page; one block per tick; the backend named; a first reading of the hand-off
  with the block filled to the size of 12 and of 48 soldiers; a first reading of the Worker's pacing; the size of the
  Wasm file; what a double-click on the built page does in each browser (the editor's open question, first result).
- Reviewers: engine-architect, graphics-engineer, rust-reviewer; then perf-auditor, security-auditor (the new
  dependencies, the page's text, the small server), test-engineer.

**3. The loop closes (step 2.5, second half).** The real messages over the loopback, and the first recording.
- Rust: `protocol`'s codec (decoders that return errors and never panic) and the thin transport interface with its
  loopback backend; `room` (one slot, timers that count ticks, the command queue, the snapshot encoder);
  `client-endpoint` (the join exchange, the command built from the newest input with its press counters, the snapshot
  decoder); the Worker's one step in the draft's order (the command for tick T reaches the local server before it runs
  T); `devtools`: the replay runner (one state hash per tick), the comparer (the first differing tick), the link
  simulator L1 behind the loopback (seeded: delay, jitter, loss, reordering, duplication, a rate limit, a packet-fate
  log); the panic hook that keeps a trap's message.
- Tests that settle rules before code rests on them: every float operation on ADR 0004's allow-list and `sqrt`, and
  the first 10,000 outputs of each random stream, bit for bit natively and in WebAssembly; a scan test in the `sim`
  crate that fails on a hash map, a clock call or a float method outside the allow-list (decision GATE-FOLLOW).
- A small separate test, kept because the game builds only one of the two: the Worker with two Wasm instances beside
  the one-instance layout. Here it shows whether a trap in the client endpoint leaves the authority running; its
  counts, times and memory at 12 and 48 soldiers follow in change-set 5, where soldiers exist.
- If this change-set nears twice its size, the two-instance test and the table of build options move to a follow-up
  of their own first.
- Reviewers: netcode-architect, engine-architect, rust-reviewer; then test-engineer, security-auditor.

**4. Walk.** Slice 1 begins: the first playable build, with your camera panel.
- The `content` crate enters: JSON read and validated by one loader with one canonical writer, a format name and
  version per file, the movement table from the legacy tuning import, the test scene as small greybox map documents; its
  schema and TypeScript types generated like the rest; in development a saved content file reaches the Worker within
  a second.
- `sim`: the soldier table with its ids, the tick's loops, our own movement on parry3d's queries (walk, sprint,
  step-up, jump, with legacy's numbers), the time unit every duration will use, chosen from a rounding report over
  every duration in the legacy tuning before any timer exists. The jump is integrated at the tick and is not pinned to
  legacy's reach: which reach v2 keeps is an open rule question (OQ-0002-6), for the game-designer and you.
- The path: the command with its aim point (the Worker casts the page's cursor ray), soldier fields in the snapshot,
  the block's soldier section, interpolation between the two newest blocks on the page.
- The page: a plain stand-in figure at the Art sheet's sizes (it shows facing and aim; the Look change-set replaces
  it), the scene, the follow camera at legacy's framing, and **the live camera tuning panel** (Part 5).
- The test scene has two parts. The street corner of the Art sheet (two houses, an alley, a wall for cover, a
  street, at the sheet's sizes) is where you play; here its houses are plain solid shapes. Beside it a training yard
  is built from the marks' own shapes, so the tests and the play share them: a step of 0.55 m and one of 0.56 m,
  ledges and platforms with ramps, thin walls, a 50 m lane with range posts.
- Two rules must start somewhere before their design questions are answered: the jump's reach and what happens when
  two soldiers meet (decision START-RULES).
- Measured here: identical hashes natively, in Node and in Chromium on a movement recording; movement against its data
  (step, apex, no tunnelling); the collision tree byte for byte; ids over random spawns; even motion on your display.
- If this change-set nears twice its size, the far-corner test map and the size comparison against other libraries
  move out first; the camera panel stays.
- Reviewers: rust-reviewer, engine-architect, netcode-architect; then graphics-engineer, test-engineer,
  security-auditor; then game-designer, perf-auditor.

**5. Twelve.** The hand-off at 12 players, measured.
- The room fills 11 slots with scripted walkers (waypoint loops through the same input path a human uses; Slice 2's
  bots replace them). A switch sets 24, 36 or 48 walkers.
- A bench mode in the page runs the hand-off marks and shows a table to copy: the page's frame cost with and without
  the simulation, the block's delay, allocations, the press-to-tick delay, the simulation's step time in the Worker.
- Three small separate tests ride here, because the game builds one of each pair: the block's two layouts (per
  entity against per field), the shared ring against the transferred block, and the two-instance Worker of
  change-set 3 at 12 and 48 soldiers. Each is kept as a build you can run later on the named machines (Part 3.2).
- Reviewers: engine-architect, perf-auditor, test-engineer; then rust-reviewer, security-auditor (the bench table's
  text and its copy button).

**6. Look.** The look's techniques on the real game, measured on the weakest machine.
- Toon materials with shared ramps per material class; both ink families behind a switch (screen-space and hull), for
  your pick; the sun with soft shadows, a warm sun and cool shadows; the value hierarchy on the scene; glow only on
  emissive slots; the shared scene pass every effect reads; the quality tiers as data (Low and High); every material
  variant compiled behind the loading screen, so nothing stutters at first sight; the same look on WebGPU and on the
  WebGL 2 fallback.
- The soldier becomes the code-built stand-in from the Art sheets, built the way the loader builds a model: one
  skinned mesh per colour area on one shared skeleton, with the colour areas and the joints that change-set 0's rules
  name, so that the Blender model can replace it without engine changes.
- The measurement harness gets its GPU side (timestamp queries).
- Measured here: each technique's cost alone; both ink families at 12, 24 and 48 soldiers; the Low frame; draw calls;
  the same pixels on both backends; no compile after the loading screen.
- You judge: the look checklist's steps that a soldier, the scene and a weapon can show (Part 5 lists them).
- Reviewers: graphics-engineer, art-director, perf-auditor; then test-engineer.

**7. Lag.** Prediction and reconciliation, against the local server.
- `client-endpoint`: the prediction step for the local soldier, the reset and replay on each snapshot, the fade and
  the snap of a correction, the interpolation buffer and delayed clock for the other soldiers, the clock estimate.
- The room's command scheduler behind one seam, with both ways of timing commands built (run on arrival; buffered,
  one per tick), because the netcode proof test still chooses between them (Part 6) and the slice's mark runs under
  each.
- Headless tests over L1 under its profiles, with 48 scripted clients where a mark asks for 48; the same client
  endpoint also run as WebAssembly against a native room.
- In development builds, the fake lag switch puts L1 behind the loopback in the browser.
- Measured here: zero corrections above 1 mm when commands arrive in time; restore and continue, exact; the replay's
  cost in the Worker; commands in time under loss.
- Reviewers: netcode-architect, rust-reviewer, engine-architect; then test-engineer, perf-auditor.

**8. Shoot.** Weapons as data, hitscan, events.
- `content`: the legacy import for weapons with every value accounted for (mapped, or dropped with its reason), the
  weapon tables, the hashed bundle in local play with its three tampering cases, the fuzz corpus.
- `sim`: fire timing in the integer time unit, hitscan against soldiers' hit volumes and the world, spread from the
  stream only the authority holds, the damage pipeline, health, death and respawn, the events (shots, hits, deaths),
  the local player's own weapon state, predicted.
- The page: tracers, a muzzle flash, the hit marker on confirmation, health bars, an ammo counter. Plain on purpose:
  the punch is change-set 11.
- The first legacy goldens: 20,000 rays, shot resolution and locational damage, the imported tables.
- Playable here: the SMG (LOOP_V2's reference gun) and the rifle. The other legacy weapons convert as data and are
  checked against the legacy tables per shot.
- This is the largest change-set. Its own plan may cut it in two playable halves (the SMG first, then the rifle and
  the goldens) if the advisor agent asks.
- Reviewers: rust-reviewer, netcode-architect, engine-architect; then game-designer, test-engineer, security-auditor;
  then perf-auditor, graphics-engineer.

**9. Throw.** Projectiles and blasts.
- `sim`: the projectile table in a fixed-size pool, the one trajectory function shared by the aim and the authority,
  swept movement, bounces, blasts, pellets with their falloff; the block's projectile section.
- Playable here: the frag grenade, the bow and the shotgun.
- Reviewers: rust-reviewer, netcode-architect, game-designer; then test-engineer, perf-auditor.

**10. Body.** The procedural animation port.
- The page poses the skinned soldier from the block: steps, the reaction to strafing, the aim pose, recoil, the death
  fall, with legacy's animation metrics as goldens.
- Reviewers: graphics-engineer, art-director, perf-auditor; then test-engineer.

**11. Feel.** The feel kit's effects and voices for the slice's weapons.
- The feel kit per weapon as client data (kick, muzzle flash, tracer, impact, hit marker, sound, target flinch, kill
  confirm), tuned by saving the data file, which reloads within a second; pooled effects with no allocation per frame;
  fragments and the concussion shake for the grenade; voices on Web Audio with priorities and a cap.
- You judge sight, sound and feel together, beside legacy.
- Reviewers: graphics-engineer, art-director, perf-auditor; then game-designer, test-engineer.

**12. Close.** Slice 1's marks with everything on.
- The target course as content of the scene; the slice's recordings for the determinism check; the hand-off marks at 12
  players again with every section in the block; a match that ends, restarts and leaves nothing behind; STATUS,
  ROADMAP and the repository map.
- Its report lists, mark by mark, what of Slice 1's done-when is met and what is still only shown or still waits
  (Part 2, "What is still open when change-set 12 merges"), so that nobody reads "Slice 1 is done" into it early.
- Reviewers: test-engineer, engine-architect, perf-auditor; then docs-keeper.

**M. The first model.** Part 12.

## Part 2. Every done-when line and pass mark of steps 2.4 and 2.5 and Slice 1 (your item 2)

How to read "met at": a clause that says "in CI" for a WebAssembly run or a browser run cannot be met by this chat
alone, because CI runs neither for v2 today (`toolchain.yml` runs the native tests, builds for wasm32 and runs Vitest)
and CI files are the 2.3 chat's. So each row names the change-set that **shows** the clause (this container, and
today's CI job where a plain test can carry it) and the step where its CI clause joins. "2.3 after n" means: the gate
or CI check of the 2.3 chat that follows my change-set n (decision GATE-FOLLOW).

### Step 2.4: "the marks of the spike's item 4 hold in CI" (ADR 0002 :1471-1472)
| Line or mark | What it asks, in short | Shown at | Met at |
|---|---|---|---|
| PM-0002-15 | regenerating every generated type changes 0 bytes; a renamed Rust field without regeneration fails; a hand edit fails | 1 (the export tests run in today's `cargo test` job) | 1; its gate form (G4) 2.3 after 1 |
| PM-0002-16 | a layout change without regeneration fails at build or at page startup (the layout hash), by a committed test | 1 (the test); the page's startup check runs in 2 | 2.3 after 2: the mark's own method builds `sim-wasm` with a changed layout and runs the page's check in CI |
| PM-0002-17 | `tsc -b` on `packages/types` with the strict base settings: 0 errors; a parameter property and an `enum` are refused; an ambient `enum` recorded | 1 (tsc by its path, with the draft's flags) | 2.3: with its `tsconfig.base.json` and its `typescript` job (its CS-C and CS-E) |
| PM-0002-18 | CI fails when the wasm-bindgen CLI and the library in `Cargo.lock` differ | 2 (the build script compares them before it generates bindings) | 2.3 after 2 (its wasm-bindgen install step) |
| PM-0002-19 | two generator runs give identical bytes; 20 runs of `cargo test` on one commit give 20 identical sets | 1 (20 runs here, hashed) | 2.3 after 1 (the 20 runs once in CI) |
| PM-0002-63 | marks 15 to 19 hold in the foundation's CI with the three generators | 1 and 2 | 2.3 after 2 (the third generator, wasm-bindgen's, enters in 2) |
| PM-0002-43 | the gate's planted defects for G4 | none: gate work | 2.3 after 1 |
| ADR 0005's foundation line (:2439-2441) | the `content` crate's place; its generated schema and types diffed; glTF readers kept out of `sim` and `server` | 4 (the crate enters with Slice 1, as ADR 0002 part 3 places it; its schema and types are generated and diffed like the rest) | 4; the dependency rule against glTF readers is 2.3's G5 and must hold before M |

### Step 2.5: the client and local server skeleton (ADR 0002 :1473-1478)
| Line or mark | What it asks, in short | Shown at | Met at |
|---|---|---|---|
| ADR 0001's milestone | the Worker's client endpoint and local server exchange `Join`, `JoinAccepted`, `Input` and `Snapshot` over the loopback, with L1 behind it in CI | 3 (headless in today's `cargo test` job, L1 behind the loopback; in the Worker in Chromium here) | 3 for the headless half; the browser half in CI 2.3 after 3 |
| ADR 0003's milestone, first half | the page posts its newest input each frame, receives one block per tick and draws an empty scene from it, naming its backend | 2 | 2 |
| the same, second half | every tick runs on the newest post | 3 | 3 |
| "the page makes zero Wasm calls" (PM-0002-2) | 0 calls into Wasm and 0 instantiations by the page | 2 (stubs that throw) | 2 in Chromium; three engines in CI 2.3 after 3; the minimum laptop and the Mac when named |
| ADR 0004's milestone | the first recording's per-tick hashes match natively twice and in Wasm | 3 (two native processes in today's CI job; Wasm in Node and in Chromium here) | 3 natively; the Wasm half in CI 2.3 after 3 |
| PM-0002-64 | a 1-minute local match in three browser engines: every tick on the newest post, at most one post per drawn frame, blocks equal ticks within 1, 0 Wasm calls and 0 network messages by the page, the backend named per engine | 3 (Chromium here) | 2.3 after 3 (Firefox and WebKit run only in CI) |
| ADR 0003's foundation line (:1390-1393) | `await renderer.init()`; the backend in the overlay and in every test screenshot; CI gates the render counts; the preview link names its backend | 2 (the first two; the counts hook and a committed baseline) | the CI gate on counts 2.3 after 3 (PM-0003-36); the preview link at step 2.6 |
| ADR 0004's foundation line (:1252) | the hash-map ban and the float rules as lints; a replay runner; the first recording in CI | 3 (the runner, the recording, and a scan test for the rules) | the lints are 2.3's (its CS-C for the hash-map and clock bans; the float ban after 3's vectors) |
| PM-0002-49 | L1: the same seed gives the same packet-fate log and hashes; loss, delay and jitter within 5 % over 1,000,000 packets per profile | 3 | 3; its gate step (G11) and PM-0002-50's planted defects 2.3 after 3 |
| PM-0002-53 | a planted counter overflow panics in debug and release, natively and in Wasm | 3 | 3 natively; the Wasm half in CI 2.3 after 3 |
| "The foundation is done" (PM-0002-69) | an empty scene through the full stack in the browser, CI green, a preview link; it asks that every foundation mark (PM-0002-35 to -68) passes | 3 for the scene | step 2.6 (Part 8) and the last of step 2.3's marks, whichever comes later |

### Slice 1 (ADR 0002 :1487-1491, with the lines of ADR 0001, 0003, 0004 and 0005 it names)
| Line or mark | What it asks, in short | Shown at | Met at |
|---|---|---|---|
| ADR 0002: `content` enters, weapons as data | | 4 (the crate, movement), 8 (weapons) | 8 |
| ADR 0002: the block's soldier, projectile and event sections, and the owner-only section for the weapon state | | 4 (soldiers), 8 (events, owner-only), 9 (projectiles) | 9 |
| ADR 0002: the first legacy goldens (ray maths, shot resolution, the imported tables); PM-0002-71 | each golden fails past its tolerance; 20,000 of 20,000 rays; a regenerated fixture is byte-identical | 8 (movement's table in 4; the animation metrics in 10) | 8; the gate's planted moves 2.3 after 8 |
| ADR 0002: the determinism recordings in G10 | | 4 on (a recording per change-set that changes the simulation); the set fixed in 12 | G10 itself is 2.3 after 3 |
| ADR 0001 (:1785): a recorded input replay gives the same per-tick hash on two runs | | 4 | 4 |
| ADR 0001: R2-SYNC-REC2 marks 1 and 2 (PM-0001-20) on the loopback under L1 | commands in time under loss; 10,000 presses, none lost or doubled | 7 (headless over L1, 48 scripted clients, under both ways of timing commands, as the mark asks) | 7 for the L1 half; its real-network half at Slice 3 |
| ADR 0001: R2-SYNC-REC3 mark 1 (PM-0001-41) | 0 corrections above 1 mm when every command arrived in time | 7 | 7 for the L1 half; the minimum laptop over a real link at Slice 3 |
| ADR 0001: R2-SYNC-REC3 mark 2 (PM-0001-42) | small corrections in a crowd of 48 | 7 (the snapshot share, 48 scripted clients; the collision rule named) | Slice 3 for the per-frame half |
| ADR 0001: R2-SYNC-REC3 mark 3 (PM-0001-43) | own movement and own muzzle flash feel the same at any latency (a time mark on presented frames) | 7 (movement) and 8 (muzzle flash), in the browser with L1 behind the loopback: readings on your laptop | when the minimum laptop and the desktop are named. Our reading of "on the loopback under L1's profiles" for a time mark; netcode-architect confirms it in 7's plan |
| ADR 0004 (:1253): S4, one integer time unit | PM-0004-9, -10, -11 | 3 (no clock in the sim), 4 (the rounding report and the unit), 8 (every legacy fire rate) | 8 |
| ADR 0004: S5, snapshot and restore | PM-0004-12, -13 (-14 is not run under today's prediction scope) | 7 | 7 for exactness and the zero-allocation count; PM-0004-12's times need the stress scene (Slice 6) |
| ADR 0004: S6, ids | PM-0004-13, -15, -16 | 4 (-15), 7 (-13, -16 on the slice's recordings) | 7; the bots-only match form at Slice 2 |
| ADR 0004: S9, parry3d and our own movement | PM-0004-30 to -39 | 4 (-30, -32, -35's soldier pairs, -39, -33's own size), 8 (-36's rays), 9 (-35's ball pairs, -37) | 9 for what Slice 1 can hold. Later by their own nouns: -31 (the stress scene), -36's bot shots (Slice 2), -35's hull pairs and -38 (vehicles). Separate tests: -33's comparison builds, -34 (Part 3) |
| ADR 0004: "the determinism suite is green on slice 1 recordings on every target" | PM-0004-19 on the slice's recordings | 4 on: natively twice, Node, Chromium | Firefox and WebKit in CI (2.3 after 3); Safari on the Mac and Windows on the minimum laptop when they are on hand |
| ADR 0002: "the boundary marks of the spike's items 1 to 3 hold on the real slice at N = 12" (PM-0002-70: marks 2, 3, 4, 6, 7, 8, 9, 12, 13) | | 5 (all but the events mark), 8 (PM-0002-9, events), again in 12 with every section | 12 for the counts; the timed ones when the minimum laptop is named |
| ADR 0003 (:1397-1399): one skinned soldier | | 6 (the code-built stand-in as one skinned mesh), 10 (posed), M (the model) | 10 |
| ADR 0003: the toon materials and ink | | 6 | 6, with your pick of the ink family |
| ADR 0003: the feel kit's visual effects and voices; split feedback | | 8 (the hit marker on confirmation), 11 | 11 |
| ADR 0003's milestone: programs constant over a match (PM-0003-7) | | 6, again in 11 and 12 | 12 |
| ADR 0003's milestone: zero per-frame allocation in the render path (PM-0003-29) | | 6, 10, 11 | 11 |
| ADR 0003's milestone: your Brave check | | every change-set from 2 on | 12 |
| ADR 0005 (:2443-2445): CD-1 to CD-8 on weapons and movement, the import's tables, weapons as data, the bundle and its hash in local play | | 4 (the loader, the writer, versions, the movement table, development reload), 8 (weapons, the import report, the bundle and hash) | 8 |
| ADR 0005: content item 1 | every double of the legacy tuning and 10,000 random ones parse to the same bits natively, in Wasm and in `JSON.parse`, and write back the same | 4 (natively, Wasm in Node, `JSON.parse` in Node and Chromium) | when CI runs Wasm and the three browser engines (2.3 after 3); its Safari half on the Mac |
| ADR 0005: content item 2 | a fuzz corpus of 10,000 files: 0 crashes, 0 accepted invalid files, the same verdict natively and in Wasm | 4 (for the kinds that exist), 8 (with weapons) | 8 |
| ADR 0005: content item 6 | a stale or altered bundle is refused or fetched again (3 cases); a repeat visit loads it from the cache; the Rust hash equals WebCrypto's | 8 | 8 in local play; its record half with the match record (Slice 4) |
| ADR 0005: ability item 2 | per shot, each legacy weapon equals the legacy tables (integers exactly, floats within 1e-6) | 8 (single-ray weapons), 9 (pellets, the bow, the grenade) | 9 |

### What is still open when change-set 12 merges
Change-set 12 ends the building of Slice 1. It does not by itself make Slice 1's done-when lines true. Its report
lists every row above with its state, so that "Slice 1 is done" is not said early. By this plan the open ones will be:
- **Waiting for the 2.3 chat's CI jobs:** every "in CI" clause for WebAssembly and for Firefox and WebKit: the
  determinism suite on every target, PM-0002-64, the gate on render counts, the goldens' planted moves, and the gate
  forms of the type marks.
- **Waiting for named machines:** every timed clause: the hand-off marks inside PM-0002-70 (marks 4, 7, 8 and 13),
  PM-0001-43, the comparisons of Part 3.2; Safari on the Mac; Windows on the minimum laptop for the determinism suite.
- **Waiting for a later slice, by the mark's own nouns:** the real-network halves of PM-0001-20, -41 and -42 (Slice
  3); snapshot times and the movement cost in the stress scene (Slice 6); the bots-only match and the bot shots
  (Slice 2); the vehicle clauses; content item 2's later kinds (Slices 2 and 4); content item 6's mode id, real join
  and match record (Slices 2, 3 and 4); ability item 2's "a bot equals a human" (Slice 2).
- **Waiting for you:** your picks from change-set 6, and the first model if it has not landed.

Slice 2 may start with these written down: by your rule, a mark that fails later sends back the code that rests on it.

## Part 3. Every question and pass mark of the tests you named (your item 3)

**One rule for every row below.** A mark keeps all of its own clauses: its machines, its browsers, its run counts,
its "in CI", its nightly run. A row names the change-set that first shows the mark here (natively, in Node, in
Chromium) and, under "later clauses", what is special to it. The clauses that name CI's three browser engines join
when the 2.3 chat's CI job runs them; those that name the minimum laptop, the desktop or the Mac join when the
machine is named and on hand. Until every clause holds, the mark is shown or read, not met.

### 3.1 What is measured first, and why
Each of these would cost the most to learn late. Each is placed at the first build that can show it honestly.

| What | Marks | Where | What "honest" needs |
|---|---|---|---|
| The same result natively and in WebAssembly from the first tick | PM-0004-19, with -17, -18, -21 (the hash and the serializer), -22 (the maths functions and `sqrt`), -24, -27 (the random streams) | 3, before any formula is written; then every later recording | two native processes with different hash-map seeds, Wasm in Node and in Chromium here; Firefox and WebKit in CI; Safari and Windows on the named machines later |
| The float and hash-map rules enforced by the build | PM-0004-23, with -10 (no clock) and -74 (one feature set) | 3 (a scan test in the sim crate) | the lints themselves are the 2.3 chat's |
| The hand-off between page and simulation at 12 players | PM-0002-4, -6, -7, -8, -13, with -3 and -28 | a first reading in 2 and 3 (a block padded to 12 and 48 soldiers); the marks in 5 | the minimum laptop's own 60 Hz screen, named; until then a reading on your laptop |
| The Worker's pacing, and a press reaching its tick | PM-0002-26 (PM-0004-68 to -70), PM-0002-12, -13 | 2 and 3; even motion in 4 | screens at 60, 120 and 144 Hz for the smoothness clause |
| One WebAssembly instance or two | PM-0002-25 (OQ-0002-3, OQ-0004-7) | 3, as a small separate test | both layouts built; the game keeps one |
| The time unit every timer uses | PM-0004-11, then -9 | 4 (the unit, from the rounding report), 8 (every legacy fire rate) | counted in CI |
| Our own movement on the collision library | PM-0004-30, -20, -32, -39 | 4 | the ledge block in the yard; hashes natively against Wasm |
| The frame time on the weakest machine with the look on | PM-0003-9, -12, -13 (as a reading), -21's post-processing row, with -3, -7, -8, -10 | 6 | the named minimum laptop at Low; full-screen passes cost by resolution, so their early numbers hold |
| Prediction: its cost and its exactness | PM-0001-44, PM-0001-41, PM-0004-13 | 7 | the replay forced on every snapshot, in the Worker on the minimum laptop; a native authority against a Wasm client endpoint |

### 3.2 Questions the real game cannot answer: the separate tests that stay
Your rule: a test is built separately only where the plan shows a question the real game cannot answer. These marks
compare the game against a second version it will not contain. Each stays a small separate test at the change-set
that gives it its subject. None is merged into the game, and none is thrown away before its mark is judged: each is
kept two ways, as a patch file in its change-set's report (text, so it can be rebuilt) and as a build with the bench
page, which runs unchanged on the named machines later. Until a comparison is judged on the machine its mark names,
the side the game builds is provisional.

| Mark | The question | Built | Judged | Size |
|---|---|---|---|---|
| PM-0002-25 | one Wasm instance or two in a local match | 3 (does a trap in the client endpoint leave the authority running); 5 (calls, time and memory at 12 and 48 soldiers) | counts here; time and memory on the minimum laptop in Chrome, when named. Reported, as the mark says | half a day, twice |
| PM-0002-10 | the block laid out per entity or per field | 5 (48 walkers); again in 12, with the event and projectile sections | reported: every cell on the minimum laptop (Chrome, Edge, Firefox) and on the Mac (Safari), when named | half a day |
| PM-0002-11 | the transferred block or the shared ring | 5 | on the minimum laptop, when named (Safari reported). Until then the transferred block is provisional | one day |
| PM-0002-27 | the fallback pacing (`Atomics.wait`): by its own words only if PM-0002-26's jitter fails | then | as PM-0002-26 | one day |
| PM-0004-33's comparison builds | the size of our Wasm against the same with rapier3d and against an ECS stub | 4 | sizes are the same on every machine: recorded at 4, and by CI when its job exists | half a day |
| PM-0004-34 | Rapier's own character controller against our movement | after 4 (parity on the ledge block, size) | its cost clause on the reference server, with PM-0004-31, at Slice 6, on the kept build | one day (decision COMPARE) |
| PM-0005-16's size half | the bytes content adds: the Wasm with the loader against a build with a stub loader | 4 | recorded at 4 | an hour |
| PM-0004-26 | native AArch64 added to the determinism suite: by its own words only before an ARM host may be picked | then | an ARM machine | |
| PM-0004-40, -44 | both navigation generators | Slice 2's plan | | |
| PM-0003-27 | two other ways to draw soldiers against the chosen one | Slice 2's plan | | |
| PM-0003-31 | two ways to draw a field of props | Slice 5's plan | | |
| PM-0003-40 | occlusion rays for sound: by its own words only if ever proposed | then | | |

### 3.3 The boundary spike: PM-0002-1 to -34, -77, -78; PM-0004-8, -28, -61, -68 to -70; PM-0006-28 to -31
| Mark | Where on the real game | Later clauses |
|---|---|---|
| PM-0002-1 | 2 on: every result row names machine, browser, display and commit | complete when you name the minimum laptop and the Mac |
| PM-0002-2 | 2 | three engines in CI: 2.3 after 3; the named machines |
| PM-0002-3 | 3 (the count), 5 (the same count at 12 and at 48 walkers) | online at Slice 3 |
| PM-0002-4 | 5 (12, 24, 36, 48 walkers; the Worker padded to its budget) | judged on the minimum laptop; the full scene with vehicles at Slice 6 |
| PM-0002-5 | 3 (10 minutes, empty), 5 (48 walkers) | bots at Slice 2 |
| PM-0002-6 | 2; with sections 4 and 5 | |
| PM-0002-7 | 2 (the method), 5 | |
| PM-0002-8 | 2 (a first reading), 5 | judged on the minimum laptop's 60 Hz screen |
| PM-0002-9 | 8, again in 12 | 48 soldiers at Slice 2 |
| PM-0002-10, -11 | separate tests at 5, -10 again in 12 (3.2) | each judged on the minimum laptop and the Mac, when named |
| PM-0002-12 | 3 | |
| PM-0002-13 | 3 (a first reading), 5 | judged on the minimum laptop |
| PM-0002-14 | 4 (the aim in the post), 8 (shooting) | |
| PM-0002-77 | 7 (the walkers are the remote soldiers, one sprinting) | online at Slice 3 |
| PM-0002-15 to -19 | 1 and 2 (Part 2) | |
| PM-0002-20 | 2 | three engines in CI: 2.3 after 3 |
| PM-0002-21 | step 2.6, and by its own words only if production turns isolation on | |
| PM-0002-22, -23, -24 | Slice 3 (they need the online client) | |
| PM-0002-25 | separate test: the trap clause at 3, the numbers at 12 and 48 soldiers at 5 (3.2) | time and memory on the minimum laptop |
| PM-0002-78 | 5 (48 walkers, reported) | bots at Slice 2, online at Slice 3 |
| PM-0002-26 | 2 and 3 (jitter, the hidden tab), 4 (smoothness) | 48 soldiers at 5 and Slice 2; online at Slice 3; the named machines |
| PM-0002-27 | conditional (3.2) | |
| PM-0002-28 (PM-0004-8) | 5 (reported at 12 to 48 walkers), again with the Low renderer in 6 | its pass needs a player count from a design question (R1-TICK-OQ7); bots at Slice 2; the stress scene at Slice 6 |
| PM-0002-29 | 2 (sizes recorded), 3 (the build-option table), 4 and 8 (with content) | the first limit is the Advisor's to set |
| PM-0002-30 | 2 | |
| PM-0002-31 | 4 | |
| PM-0002-32 | 2 (the built player page by double-click, in Brave on your PC) | other browsers on the named machines; the editor's own entry at Slice 5 |
| PM-0002-33 | 3 (the panic message in the page's error log) | other browsers on the named machines; its Sentry half at Slice 3 |
| PM-0002-34 | your checklist, step by step: 4 (even walking; the tab hidden for 1 and for 6 minutes), 5 (ten minutes with 48), 2 (the double-click) | its online step at Slice 3 |
| PM-0004-28 | 4 (camera still and moved; 60 and 144 Hz), 11 (sound on and muted) | on the recommended desktop |
| PM-0004-61 | Slice 3 (it needs the server) | |
| PM-0004-68, -69, -70 | as PM-0002-26: 2 and 3, 4 | online halves at Slice 3 |
| PM-0006-28 to -31 | the first online playtest's milestone, as the draft says when no EU Sentry organisation exists | |

### 3.4 The sim core spike: PM-0004-1 to -61, -73, -74 (without the rows above)
| Mark | Where on the real game | Later clauses |
|---|---|---|
| PM-0004-1 | 3 on (a log header per machine) | the named machines |
| PM-0004-2, -3, -4, -5 | Slice 6 (the stress scene on the reference server) | |
| PM-0004-6 | 4 (zero allocations in the step, counted), 9 (the projectile pool's burst) | the effect pool at Slice 4; the full form at Slice 6 |
| PM-0004-7 | 5 (a reading at 12) | judged in the stress scene on the minimum laptop (Slice 6) |
| PM-0004-9 | 8 (the 13 legacy intervals) | its duel clause (the Warrior kills the Conscript in 0.86 s) with the units' data, at Slice 2, as PM-0005-25 |
| PM-0004-10 | 3 | |
| PM-0004-11 | 4, read again in 8 | |
| PM-0004-12 | 7 (the zero-allocation count) | its times at Slice 6 |
| PM-0004-13 | 7 | the bots-only match form at Slice 2 |
| PM-0004-14 | not run while the prediction scope is the draft's (its own condition) | |
| PM-0004-15 | 4 | |
| PM-0004-16 | 7 | the bots-only match form at Slice 2 |
| PM-0004-17 | 3, meaningful from 4 | Slice 2's form |
| PM-0004-18 | 3 | the stress scene's form at Slice 6 |
| PM-0004-19 | 3, then every recording from 4 on | Firefox and WebKit in CI, with its nightly run; Safari on the Mac; Windows on the minimum laptop once; the six bot matches at Slice 2 |
| PM-0004-20 | 4 | the stress scene's form at Slice 6 |
| PM-0004-21 | 3 (the same digest and bytes on 64 and 32 bits), 4 (the three planted divergences) | |
| PM-0004-22 | 3, run again whenever a function joins (4, 9) | Safari on the Mac |
| PM-0004-23 | 3 (the scan test) | the lints: 2.3 |
| PM-0004-24 | 3, 4 | with PM-0004-60's fuzzing |
| PM-0004-25 | 4 (a reading: the slice's recordings with SIMD on) | SIMD stays off until the mark's whole suite passes with it on: the bot matches of Slice 2, Firefox and WebKit in CI, Safari on the Mac |
| PM-0004-26 | conditional (3.2) | |
| PM-0004-27 | 3 (the vectors) | the cost at Slice 6 |
| PM-0004-29 | laid out from 8; shown at Slice 3, where the draft places it | |
| PM-0004-30 | 4 | |
| PM-0004-31 | 5 (a reading) | Slice 6 |
| PM-0004-32 | 4 | its build time on the reference server |
| PM-0004-33 | 4 (our own size and the rebuild time); the comparison builds as a separate test (3.2) | |
| PM-0004-34 | separate test after 4 (3.2) | its cost clause on the reference server at Slice 6 |
| PM-0004-35 | 4 (soldier pairs), 9 (the grenade's) | hull pairs when vehicles exist |
| PM-0004-36 | 8 (the 20,000 rays) | bot shots at Slice 2 |
| PM-0004-37 | 9 | |
| PM-0004-38 | when vehicles exist (Slice 6 at the latest; no slice names them: for the drafts' next revision) | |
| PM-0004-39 | 4 | |
| PM-0004-40 to -44 | Slice 2 (navigation enters) | |
| PM-0004-45, -46 | Slice 2 (counts) | their times at Slice 6 |
| PM-0004-47 to -51 | Slice 2 | -50's editor path at Slice 5 |
| PM-0004-52, -53 | Slice 2 (hashes, counts) | times at Slice 6 |
| PM-0004-54 | 4 (the yard has ramps) | |
| PM-0004-55 | Slice 6 | |
| PM-0004-56, -57 | Slice 4 (structures) | |
| PM-0004-58 | Slice 6 (countable at Slice 2) | |
| PM-0004-59 | 4 | |
| PM-0004-60 | 4 (movement commands), 8 (every command) | its nightly run is CI work |
| PM-0004-73 | 9 | |
| PM-0004-74 | 3 | `navbake` at Slice 2, `server` at Slice 3 |

### 3.5 The 60 Hz run: PM-0001-39, -40, -63; PM-0004-71, -72
All five decide at Slice 6, on the real simulation in the stress scene, as the drafts already say ("slice 6 decides").
The early signal of wave 1 ran on the sim core spike's throwaway simulation, which is no longer built; no mark is
lost with it, because it decided nothing. Three parts are shown earlier, as readings, in a 60 Hz build of the real
simulation: PM-0004-72's shot counts (every legacy interval fires the same number of shots at 30 and at 60 Hz) at
change-set 8, which also shows that change-set 4's time unit holds at 60 Hz; PM-0004-72's tunnelling clause at
change-set 9; and the 12-tick replay of PM-0001-40 from change-set 7 on. The tick rate stays one constant and every
duration is data, so 60 Hz stays possible.

### 3.6 The look proof test: PM-0003-1 to -17 (PM-0005-33 to -37 are in 3.8)
| Mark | Where on the real game | Later clauses |
|---|---|---|
| PM-0003-1 | your naming of the machines, before 6 | |
| PM-0003-2 | 2 (empty scene), 6 (the look on) | Edge, Firefox and Safari on the named machines |
| PM-0003-3 | 6, again in 11 | houses and big units when they exist |
| PM-0003-4 | 6 (a reading at 12) | as written at Slice 6 |
| PM-0003-5 | 2 | Firefox on Linux and Safari on the named machines |
| PM-0003-6 | 6 (the harness's GPU side) | |
| PM-0003-7 | 6, 11, 12 | complete at Slice 5 (damage stages, the roof fade) |
| PM-0003-8 | 6 (the soldier), 11 (effects) | complete at Slice 5 |
| PM-0003-9 | 6 (toon, both inks, ambient occlusion, glow, the shadow map, multisampling), 11 (effects) | the roof fade at Slice 5; halftone and line boil with the first big moment the server recognizes (Slice 4) |
| PM-0003-10, -11 | 6 | |
| PM-0003-12 | 6 (times at 12, 24 and 48 soldiers; the draw count with 48 stand-ins, as a reading, beside your pick of the ink family), 12 (the count again with health bars, effects and projectiles in) | the gated whole-frame draw count at Slice 6 |
| PM-0003-13 | 6 (a reading) | as written at Slice 6 |
| PM-0003-14 | 6 | |
| PM-0003-15 | Slice 5 (the lush patch; Slice 2 if its map holds one) | |
| PM-0003-16 | 2 (the records), 6 (memory with multisampling) | |
| PM-0003-17 | your look checklist, in pieces (Part 5): 2, 6, 9 and 11, M, then Slices 2, 4, 5 and 6 | |

### 3.7 The render budget bench: PM-0003-18 to -47
| Mark | Where on the real game | Later clauses |
|---|---|---|
| PM-0003-18 | Slice 6 (a reading at Slice 2) | |
| PM-0003-19 | 10 (a reading of the animation share) | Slice 2, Slice 6 |
| PM-0003-20 | 2, 5 | Slice 2 |
| PM-0003-21 | 6 (the post-processing row) | every row at Slice 6 |
| PM-0003-22 | 2 (the count baseline), 6 (render-target memory), M (a reading with the first model) | full at Slice 6 |
| PM-0003-23 | 6 | |
| PM-0003-24 | 2 (the method), 12 | Slice 6 |
| PM-0003-25 | 6 (48 stand-ins) | Slice 2 |
| PM-0003-26 | 10 (a reading) | Slice 2 |
| PM-0003-27 | separate test at Slice 2 (3.2) | |
| PM-0003-28 | 12 (a recorded match replayed to the page) | |
| PM-0003-29 | 6, 10 | 48 health bars at Slice 2 |
| PM-0003-30 | Slice 6 | |
| PM-0003-31 | separate test at Slice 5 (3.2) | |
| PM-0003-32 | 6 (a reading) | Slice 6 |
| PM-0003-33 | 6 | the 50 % cut on the full scene at Slice 6 |
| PM-0003-34 | 6 | on the recommended desktop |
| PM-0003-35 | 12 | |
| PM-0003-36 | 2 (the baseline file and the check, run here) | its CI job: 2.3 after 3 |
| PM-0003-37 | 2 (counts), 6 (times) | |
| PM-0003-38 | 11 (a reading with a scripted 32-voice load) | Slice 2 |
| PM-0003-39 | 11 | the cap under load at Slice 2 |
| PM-0003-40 | conditional (3.2) | |
| PM-0003-41 | 11 | |
| PM-0003-42 | 4 (smoothness per frame-cap option) | battery drain in a full local match (Slice 2) |
| PM-0003-43 | at each three.js upgrade after the pin | |
| PM-0003-44 | Slice 6 | |
| PM-0003-45 | 12 | the faction swap at Slice 2 |
| PM-0003-46 | 2 on: every render-path change-set carries a 60-second WebGPU smoke run. Until the minimum laptop is named it is your Brave check on your PC, as a reading: the page's report carries the errors, the counts against the baseline and the backend's name | the same run on the minimum laptop, when named; CI's own WebGPU run if a CI engine gets WebGPU |
| PM-0003-47 | 8 | |

### 3.8 The content and ability spike: PM-0005-1 to -32; PM-0004-62 to -66, -75. And the look proof test's share: PM-0005-33 to -37
| Mark | Where on the real game | Later clauses |
|---|---|---|
| PM-0005-1 | 4 on (a log header per machine and per CI image) | the named machines |
| PM-0005-2 | 4 (native, Wasm in Node, `JSON.parse` in Node and Chromium) | Firefox and WebKit in CI; Safari on the Mac |
| PM-0005-3 | 4 | |
| PM-0005-4 | 4 (the corpus for the kinds that exist: movement and the yard; whether the validation library runs on wasm32 is settled here), 8 (weapons) | maps, modes, units and factions at Slice 2; tracks at Slice 4 |
| PM-0005-5 | 4 (bounds and the schema scan), 8 | the pool report at Slice 2; the effect reserve at Slice 4 |
| PM-0005-6 | 4 (the loader's no-panic rules, with a bounded fuzz run here) | its lints are 2.3's; its nightly fuzz job is CI work |
| PM-0005-7 | 4 (movement fields), 8 (weapon fields) | each later kind as it enters |
| PM-0005-8 | 4 (content's schema and types regenerate unchanged), 8 (the import's files typed against them) | its gate form: 2.3 |
| PM-0005-9 | Slice 2 (the maps migrate there) | |
| PM-0005-10 | 4 (local play: a saved change checked within a second, the next match runs on it), 8 (with the SMG's damage, as the mark words it), 11 (a feel-kit change shows within a second) | the development server's half at Slice 3 |
| PM-0005-11 | conditional: only if a running match may take changed content (OQ-0005-CD4 (b)); Slice 3 | |
| PM-0005-12 | 8 (the three tampering cases, the hash against WebCrypto, the repeat visit from the cache) | the mode id at Slice 2; the hash at a real join at Slice 3; the hash in the match record at Slice 4 |
| PM-0005-13 | Slice 3 (the server's check of overrides) | the editor's half at Slice 5 |
| PM-0005-14 | 4 (the walk over every leaf from its first run; movement mapped; the look values listed), 8 (weapons) | every kind imported: Slices 2 and 4 |
| PM-0005-15 | 8 (weapon entries), 11 (the feel kit), M (the asset list and slot classes) | the full scenario with abilities at Slice 4 |
| PM-0005-16 | 4 (the bytes content adds to the Wasm file) | load time with maps and modes at Slice 2; again at Slice 6 |
| PM-0005-17 | Slice 4 (your content trial; its steps that need only local play can be tried from 8) | |
| PM-0005-18 | 8 (the legacy weapons as data, with the rename test), 9 (the grenade) | abilities at Slice 4 |
| PM-0005-19, -20 | Slice 4 (abilities and effects) | |
| PM-0005-21 | 8 (8 damage, then 9 in the next match) | the Conscript's 12th hit at Slice 2 |
| PM-0005-22 | 8 and 9 | bot equals human at Slice 2; the Juggernaut's cannon with vehicles |
| PM-0005-23 | 8 (weapons and movement) | units and vehicles at Slice 2; abilities and tracks at Slice 4 |
| PM-0005-24 | Slice 4 (tech tiers) | |
| PM-0005-25 | Slice 4; its 35 m clause at 9 | the duels without abilities at Slice 2 |
| PM-0005-26 to -31 | Slice 4 (upgrade tracks) | |
| PM-0005-32 | Slice 6 | |
| PM-0004-62, -63 | Slice 4 | the stress scene's form at Slice 6 |
| PM-0004-64, -65, -66 | Slice 4 | |
| PM-0004-75 | Slice 4, once the open question on full pools (OQ-0004-11) is answered | |
| PM-0005-33, -34 | M | |
| PM-0005-35 | M (one model, shared across copies) | each faction's colours at Slice 2 |
| PM-0005-36 | M (the soldier's cap) | the very large unit's cap with its first model |
| PM-0005-37 | Slice 2 at the earliest (your pick per asset family needs both forms of a family side by side) | |

### 3.9 The questions these tests decide, and where each gets its answer
The digests give each question's text. A question answered at a change-set gets its result written into that
change-set's report; the drafts themselves are edited only in docs change-sets that merge on your go.

| Where | Questions (the drafts' ids) |
|---|---|
| 0 | OQ-0005-AS3 and AS4, as rules (Part 11); proven on the first file at M |
| 1 | R5-LAYOUT-OQ4, R5-TSLINT-OQ3 |
| 2 | R1-BND-OQ3; R5-TSLINT-OQ4 and OQ6; R5-TEST-OQ1; R1-RR-OQ5 and R5-CI-OQ4 (the Wasm sizes); OQ-0002-1 (the double-click, first result); R3-WGPU-OQ1 and OQ2; R3-MEASURE-OQ1 and OQ3 |
| 3 | OQ-0002-3 and OQ-0004-7 (one instance or two, and recovery after a trap); R1-BND-OQ2, R1-TICK-OQ2 and R1-XU-OQ4 (pacing); R1-DET-OQ4, OQ8 and OQ10 (the maths functions and `sqrt`); R1-DET-OQ11 (an early cost); R1-RR-OQ1 (the build-flag part) |
| 4 | R1-DET-OQ1 and OQ2; R4-PHYS-OQ3; R4-PHYS-OQ4 and OQ-0004-3 (soldier shapes); R1-ECS-OQ1 (sizes); R4-PHYS-OQ2 (by its separate test); R1-ABL-OQ7 and R1-XU-OQ7 (the time unit, confirmed at 8); R1-DATA-OQ4, OQ5 and OQ8; R1-RR-OQ2 and OQ3 (the loader); OQ-0003-10 (smoothness per frame-cap option, for your pick) |
| 5 | R1-BND-OQ6, R1-RR-OQ6, R1-XU-OQ3 and OQ-0002-4 (the block's layout, its writer, the shared ring) |
| 6 | OQ-0003-2 (the ink family), -3 (colour output), -4 (shadow softness), -5 (the lighting), -12 (pixel ratio on Low), -15 (legacy touches), -16 (what Low may switch off): each your pick, with its measured cost; R3-GL2-OQ3; R3-LOOK-OQ1, OQ4 and OQ6; R3-CHAR-OQ1; R3-WGPU-OQ3 and R3-GL2-OQ1 (readings) |
| 7 | OQ-0001-15 (the numbers for your check: is the local player drawn one tick behind) |
| 8 | R1-DATA-OQ9; R1-DATA-OQ2 and OQ-0005-CD2 (content's size; its load time at Slice 2); OQ-0005-AB13's evidence for weapons |
| 11 | OQ-0003-7 (how effects are drawn: the look, and an early cost); OQ-0003-18 (reduced motion, with the shake); R3-AUDIO-OQ2 (a first count) |
| M | OQ-0005-AS3 and AS4 (proven), AS6 (the model's counts); R3-BUDGET-OQ5 (a first reading) |
| Slice 2 | R4-NAV-OQ1 to OQ3; R4-LOS-OQ5; R3-CHAR-OQ2; R3-AUDIO-OQ1 and OQ6; OQ-0003-19; OQ-0005-AP2; R1-ABL-OQ6 |
| Slices 3 to 5 | R6-OBS-OQ3 and OQ4 (Sentry, Slice 3); OQ-0005-CD1 and ABL-PREDICT (Slice 4); OQ-0003-6, -8 and -9, R3-LOOK-OQ3, R3-CHAR-OQ4 (Slice 5) |
| Slice 6 | R1-ECS-OQ4; R1-TICK-OQ5 and OQ9; R4-OBST-OQ4; R3-BUDGET-OQ6; R3-CHAR-OQ6; OQ-0005-AS1, AS2 and AS5 on the full scene |
| No build answers these; they are decisions | OQ-0003-1 (which machines: yours, decision MACHINES); R1-TICK-OQ7 (the player count PM-0004-8 is judged at: a design question in the register, for the game-designer and you, due before Slice 2 judges that mark; until then the mark is reported); OQ-0004-11 (which row gives way in a full pool: the game-designer, and you where a player would notice, before Slice 4); OQ-0005-AP1 (where the asset step runs: the rest of the asset research, Part 12) |

**Count** (checked by a script over this part's tables): 201 marks belong to the six tests of your decision (ADR 0002:
36; ADR 0006: 4; ADR 0004: 74; ADR 0003: 47; ADR 0005: 37; ADR 0001: 3), and every one has a row. PM-0004-67 belongs to
the netcode proof test and stays with it. The asset spike's seven marks are in Part 12.

## Part 4. What Slice 1 needs from the Art project (your item 4)
**Three sheets arrived while this plan was being finished** (you sent them into this chat on 2026-11-14; their text
and numbers are typed out in `art-sheets-2026-11-14.md` beside this file):
- **The soldier:** the Conscript's shared tier-one uniform as a stand-in "built in code from simple shapes", with its
  heights and widths in metres, front, side and in the game's camera, what must read at game size and what may be
  left out, and the same uniform in the three factions' colours with each faction's marking.
- **Its materials:** four per soldier, the rifle included: Coat, Top, Trim and Dark. That already fits the drafts'
  cap (Part 11, rule 16).
- **The test scene:** a small street corner: a street, grass, two houses with an alley between them and a wall for
  cover, with sizes, "palette A" as colour values, a sun and a shadow colour, and a line rule.

**What the plan takes from them.**
- The first soldier is the sheet's Conscript (decision FIRST-SOLDIER now only asks you to confirm it). The code-built
  stand-in follows the sheet's sizes; the enemy dummies are the same soldier in another faction's colours.
- The scene you play in is the sheet's street corner. Beside it stands a training yard built from the tests' own
  shapes (a step of 0.55 m and one of 0.56 m, ledges and platforms with ramps, thin walls, a 50 m lane with range
  posts), because the marks need those exact shapes. Its colours are the Art project's to say (the block below asks).
- The sheets call themselves blockouts, "not the design": that agrees with decision STAND-IN-LOOK's recommendation.
- **One point is not mine to settle.** The sheet's line rule draws units thicker than buildings and says it is final
  in the look proof test; `docs/LOOK_V2.md` on main says "thick black outlines, especially strong on buildings". The
  carry of your look decisions settles which stands, and change-set 6 waits for it (Part 5).
- **One point for the model rules.** The marking changes shape per faction (star, chevron, flower). A shape is not a
  colour, so the model needs a rule for it (Part 11, "Still open"). The sheet says its shape may be left out at game
  size, so the stand-in can leave it out.

**Still needed.** Paste the block below into the Art project. The rules for a model file are Part 11's list, which
goes into the Blender brief.

```
For the Art project: what Slice 1 (move and shoot) still needs, after the three sheets of 2026-11-14
(the Conscript in three factions; the Conscript as a stand-in; the street corner in palette A).
Sheets are enough: Slice 1's soldier is built in code from them, and the Blender model replaces it later.
Give every colour as a hex value and every size in metres. Where anything below differs from Mustafa's look
decisions of 13 and 14 November, those win.

A. The soldier: three small things
1. How he holds the gun when aiming and when running (the sheets show it across the chest).
2. Where the marking sits when its shape is left out at game size: is the plain shoulder board enough?
3. Whether the hat and the rifle are parts of the body's model or models of their own.

B. The guns (one sheet per gun: the rifle of the sheets and the SMG first; shotgun, grenade and bow can follow)
1. Side view and top view at the soldier's scale, with the length.
2. The three or four shapes that make its silhouette (never a plain tube).
3. Where the hands grip, where the muzzle is, where shells leave.
4. Its colours: the stand-in sheet draws the rifle in the Dark material. Does every gun stay Dark?

C. The scene: two small things
1. The training yard beside the street corner is plain blocks, ledges, ramps and range posts: which of palette A's
   colours do they take? (The loop spec's rule for greybox maps reads height through brightness, higher is
   lighter; say if the yard should follow it.)
2. How the ground looks where the street meets the grass, if it should be more than two flat colours.

D. Effects (one sheet per gun, one for the grenade). For each: a small drawing or a reference, its colours, its size
   beside the soldier, and how long it lasts.
1. Muzzle flash (shape, colour, glow).
2. Tracer (colour, width, length).
3. Impact on the world (dust, sparks, chips) and on a body.
4. Hit marker and kill confirm on the HUD.
5. Shell casings, if any.
6. The grenade: the blast's flash, fragments flying, smoke.
7. Sound, in words or references: how each gun should sound, the hit confirm, the grenade.

E. HUD pieces (one sheet): the ammo counter and reload, health bars over soldiers, the cursor, the hit marker.

F. Not needed for Slice 1: trees, vehicles, more buildings, other units.
```

## Part 5. Your queued words for Slice 1, and the look (your item 5)
**The camera panel.** Your words (`docs/DECISIONS.md`, "Queued"): "Slice 1 ships with a live camera tuning panel from
its first playable build, ported from the legacy dev panel's camera sliders and presets (LEGACY_INVENTORY rows GP5 and
CD5)."
- They land in **change-set 4**, the first playable build.
- Ported from GP5: the live sliders (boom, height, field of view, the pitch override, the aim-down-sights camera mode)
  and the five presets (Classic, Tactical, Iso, Iso+, Foxhole), on the backquote key as in legacy.
- Ported from CD5: the framing values the camera starts from (height 40, boom 56.6, field of view 15, yaw 45 degrees),
  the screen dead zone and the smoothing rates. CD5 records six keys that nothing reads; they are not ported.
- One addition, small: a "copy values" button, so the numbers you tune can be pasted into the chat and become the
  camera's data.
- GP5's performance overlay comes earlier, as change-set 2's corner label. GP5's cheats are not ported: your words name
  the camera sliders and presets.
- The item leaves "Queued" when change-set 4 merges.

**The look.** The look work is planned against `docs/LOOK_V2.md` as it stands on main (78 lines; your decisions of 13
and 14 November are not in it).
- **Wait for the carry:** 6 Look, 11 Feel and M (the first model). 10 Body waits only if the carry changes how
  soldiers move.
- **The Art sheets and the brief.** The sheets you sent today hold look choices that the brief on main does not hold
  yet: the factions' colours, palette A, and a line rule that draws units thicker than buildings, where LOOK_V2 §2
  says "thick black outlines, especially strong on buildings". For v2 work the brief wins (CLAUDE.md), so the carry
  brings them into it before change-set 6 is built. Until then the plan takes only sizes and layout from the sheets.
- **Do not wait:** 1 to 5, 7, 8, 9 and 12. Their figures and scene are plain on purpose and are judged for nothing
  but play.
- **What you judge in Brave, by change-set** (the look checklist of ADR 0003, PM-0003-17; step numbers are the draft's):
  - 2: step 1 (the page loads; the backend named).
  - 6: steps 2 and 3 (the whole frame: comic rendering, bold ink, not a children's game), 4 (the two ink families,
    your pick), 5 (colours as authored or tone-mapped, your pick), 6 (soft shadows), 7 (ink and corner shading on and
    off), 9 (one unit far and near), 16 (the scene one step calmer), 18 (warm sun, cool shadows), 28 (pixel ratio on
    Low, your pick), 31 (the Low frame beside High), and 29 in part (the legacy touches the brief does not name).
  - 9 and 11: steps 24 and 25 (the grenade: fragments, the shake), 26 (the SMG's whole feel kit), 17 in part (muzzle
    flashes glow).
  - 10: step 13 in part (the body reacts to strafing).
  - M: steps 10 and 12 (proportions; weapons and headgear), on the model.
  - Later slices: 11 and 13's crowd (Slice 2), 17's objectives (Slice 4), 14, 15, 19 to 23 (Slice 5: houses, the lush
    patch, dusk, interiors, the roof fade, damage stages), 8's vehicle and very large unit, 27 and 32 (Slices 4 to 6).
- **One point of the draft to settle** (decision STAND-IN-LOOK): ADR 0003 says proportions and weapon silhouettes
  cannot be judged on primitive parts. The code-built soldier follows the Art sheets, but it is still a stand-in.

## Part 6. What needs the netcode proof test's result (your item 6)
**Stays out until Slice 3:**
- both browser transports (WebTransport and the WebSocket fallback), the choice of the WebTransport crate, the fast
  reconnect;
- the `server` crate (the room process, its I/O runtime, its crash handling) and the `supervisor`;
- congestion control, the limits against hostile input, certificates and ports;
- lag compensation (the history of hit volumes, the rewind, its cap);
- interest management (who is sent what) and the enforced visibility classes;
- the lobby, tokens, private rooms, host commands, reconnect, and the full-state and resync messages;
- the second layer of the link simulator (real browsers over a real link), the reader client and the byte audit.

**Built now, behind a seam, because the proof test may still change it.** The local path needs these from change-set
3 or 7 on. Each sits in one place, so a different result costs one module or one data value.

| What the proof test may change | The draft's proposal today | Where it sits | Cost if it changes |
|---|---|---|---|
| How commands are timed at the server (OQ-0001-6): run on arrival, or buffered one per tick | undecided; an input buffer of 1 to 3 ticks, as data | one command scheduler in `room`; one function in the client endpoint that says when the next tick is due (change-set 7) | one module |
| Sub-tick time stamps for shots (OQ-0001-7) | a tick plus a view-time fraction only | the command codec carries a version; shot timing in one function (change-set 8) | one module |
| Snapshot field widths, the record sizes | positions to 1 cm, as the draft's table | one field table drives encoder and decoder (3) | data |
| The datagram size cap | 1,000 bytes until measured | a parameter of the splitter (4) | data |
| The encoder's cost per client | at most 50 µs per snapshot (judged on the reference server) | the codec is its own module, apart from the simulation | one module |
| Linear or smoother interpolation | linear first | one call (7) | one function |
| The clock estimate | Quake III's rules | one module of the client endpoint (7) | one module |
| Whether the local player is drawn one tick behind (OQ-0001-15) | decided after the proof test's numbers and your check | the page's drawing of one entity (4) | small |
| The interpolation delay, the fade, the snap distance, the repeat cap, the ring sizes | the draft's starting values | data | data |

**Not the proof test's, kept open the same way:** the tick rate (one constant; every duration is data in seconds, so
60 Hz at Slice 6 stays a data change), and the rule for soldiers bumping into each other (a design call; change-set
7's tests name the rule they ran under).

## Part 7. How the code is checked while the gate v2 is not built (your item 7)
**On every change-set, before its report, this chat runs the commands below and shows their output in the report.**
Tools run by their path: the settings deny `npx`, `npm run` and `npm test` in sessions.

| Check | Command | From |
|---|---|---|
| Rust format | `cargo fmt --all --check` (stable options: edition 2024, Unix line ends) | 1 |
| Rust lint, twice | `cargo clippy --workspace --all-targets --locked -- -D warnings`, and the same with `--target wasm32-unknown-unknown` | 1 |
| Rust tests | `cargo test --workspace --locked`; `cargo test --release --locked` for `sim` and `protocol` (and `content` from 4) | 1 |
| The Wasm build | `cargo build --locked --target wasm32-unknown-unknown` for the crates the Worker loads | 1 |
| The sim's rules | a test in the sim crate: no hash map, no clock, no float method outside ADR 0004's allow-list; `cargo tree` shows `sim` depends on `content` only | 3 |
| One feature set | `cargo tree -e features` for `sim`, equal for `devtools` and `sim-wasm` | 3 |
| Types in step | the export tests (regenerate, compare) inside `cargo test` | 1 |
| TypeScript types | `tsc` by its path, with ADR 0002's strict flags | 1 |
| TypeScript lint and format | ESLint and Prettier, once the 2.3 chat's change-set brings them (below) | when merged |
| TypeScript tests | Vitest by its path, per package | 1 |
| Wasm tests | the built `sim-wasm` loaded by Vitest in Node (the boundary layer); the unit vectors in Wasm | 2 |
| The build | the wasm-bindgen CLI at exactly the version `Cargo.lock` records (the build script refuses another), then Vite's build | 2 |
| The browser | Playwright on the built client with the container's Chromium, once on WebGPU or its fallback and once with `forceWebGL`, traces off | 2 |
| Determinism | the replay runner: natively in two processes, Wasm in Node, Wasm in Chromium; the first differing tick reported | 3 |
| The link simulator | the L1 tests under its profiles | 3 |
| Legacy | `node tools/gate.mjs`, unchanged and green (nothing in `legacy/` is touched) | 1 |
| CI | `gate`, `smoke` and `toolchain` green on the head, push and pull request, read by hand before every merge | 1 |

**What these cannot show**, said in every report: Firefox and WebKit (the container has Chromium only); Safari; any
timed mark on a named machine; the gate's planted-defect runs (`--prove`).

**The sim's rules, until the lints exist.** The advisor agent ruled in this plan's review (Part 14): the scan test in
the sim crate may stand in for the 2.3 chat's float ban until that lint merges, on three conditions. It checks exactly
ADR 0004 part 7's allow-list; it runs in today's CI test job; and change-set 3's report calls it interim and names the
ruling it replaces (the 2.3 plan's ruling that the float ban merges before any float arithmetic enters `sim`).

**The netcode proof test's rules, kept while its spike is in the tree** (as steps 2.2 and 2.3 keep them): no tracked
file that is not Markdown names the spike's folder; no root `.cargo/config.toml`; no `"type"` in the root
`package.json`. Each change-set that adds a manifest or a config checks the three rules by a search of its own files.
I do not put the netcode chat's branch on disk and I do not run its check: your words are "never touch spikes/". Its
check runs in its own chat when it next takes main; a breach it finds there is mine to fix.

**Which of step 2.3's change-sets I need merged first.**
- **Needed: its docs change-set (CS-A),** which records your decision of today, with your addition of 15:03. Your
  words: no build before that record is on main.
- **Wanted before change-set 2, not needed: its gate change-set (CS-C).** It brings the lint and format settings
  (`rustfmt.toml`, the `clippy.toml` files, `tsconfig.base.json`, a `tsconfig.json` per package, ESLint, Prettier). If
  it is not merged by then, I run `tsc` with the same flags from a scratch config and write no settings file of my
  own; when it lands, the next change-set takes its formatting as one mechanical commit. Rule between the two chats, so
  that no file has two authors: lint, format and type settings are the 2.3 chat's; build files (each crate's and
  package's manifest, the Vite config) are mine.
- **Changes how I work, not when: its guard change-set (CS-D).** By its option PIN-SCOPE (b), which that plan
  recommends and you have yet to answer, the root `Cargo.toml` becomes a protected file. A dependency that several
  crates share has its version there (ADR 0002 part 2), so each new shared dependency then asks you with its exact
  text, with the chat in Accept edits. I expect one such ask in most change-sets (serde, wasm-bindgen, parry3d, and so
  on). By its option (a) the root `package.json` and both lockfiles ask too, so every dependency change would.
- **Follow my change-sets** (decision GATE-FOLLOW): the gate and CI checks the 2.3 plan lets ride steps 2.4 and 2.5.
  That plan was written before your decision of today. It ties some of them to reports of the six tests, which no
  longer come first: those need your word to follow my change-sets instead. In my order: after 1, G4 (generated types
  in step); after 2, the wasm-bindgen install and the Wasm tests in CI; after 3, the browser jobs, the determinism
  step, and the float ban from change-set 3's vectors.
- **One check is tied to the netcode proof test, which is not one of the six and still runs:** the gate's network
  step (G11) re-proves the link simulator that test builds. Your words of 2026-11-13 still stand for it: "Every gate
  check that needs a result joins later, when that result is in." So the network step waits for the proof test's
  report unless you say otherwise (GATE-FOLLOW asks). My own L1 tests run in the plain test job from change-set 3
  either way, because step 2.5's done-when asks for L1 behind the loopback.

## Part 8. How you open each build in Brave, and when links come (your item 8)
**Until previews exist: a zip and one command.** A built game is a small folder of files. A browser will not run it
from a double-click (change-set 2 measures exactly what Brave says), so the zip carries a small server of its own:
one reviewed file of the repository, built in change-set 2, which serves that folder to your own computer only.

**How the zip reaches you: this is the one step not proven yet.** The route is this session's file tool, which puts
a file with a download button into the chat in the Claude app. Whether that download works on your laptop is not
shown by any record. Two proofs come before anything rests on it: this plan reaches you as such a file with the
message that ends this turn, and change-set 1's report carries a small test archive, so a zip is proven before
change-set 2 needs one. A zip is never committed to the repository: its public history would keep it for good. If
the route fails, decision BRAVE-ROUTE names the others.

Once, on your Windows laptop:
1. Open PowerShell and type `node --version`. Any version from 18 on will do. Your Windows run of 2026-11-11 used
   Node v24, so it is probably there already.
2. If it prints nothing: install Node.js from nodejs.org (the current LTS installer), then repeat step 1.

For every build:
1. In this chat I post a file, `shamble-v2-<number>-<name>.zip`, with the commit it was built from. Download it.
2. Unzip it into a folder of its own, for example `C:\shamble\04-walk`.
3. Open that folder in Explorer, click the address bar, type `powershell` and press Enter.
4. Type `node serve.mjs` and press Enter. It prints an address, `http://localhost:4173`.
5. Open Brave and go to that address.
6. Do the checklist from the chat. The page has a "copy report" button: press it and paste the result into the chat.
7. Back in PowerShell, press Ctrl+C to stop.

The page shows its commit and its backend in the corner, so a screenshot says what it was taken from.

**When the preview hosting check can run: now.** Your words of 2026-11-03: it "may run alongside any of them". It needs
your Cloudflare account, a test project and its token in a GitHub environment, and two throwaway workflows; it takes at
least two calendar days, because a closed pull request's build must be gone within 24 hours. It is hosting and CI work,
so by your words of 2026-11-12 it belongs in an Opus chat, not here. Its rows that name Safari wait for a Mac.

**When step 2.6 can run:** when four things hold: the hosting check has ended; change-set 2 is merged (there is a page
to deploy); the 2.3 chat's CI change-set is merged (the build job the deploy job hangs on); and your deploy token sits
in the `preview` environment, with the draft's interim position in place (production deployed only from main, a
reviewer on previews: ADR 0006 part 13, item B9) and the question of how production stays out of that token's reach
answered (OQ-0006-13). Its workflows are CI work, so an Opus chat builds step 2.6, not this one. If the hosting check
starts this week, links can arrive around change-sets 4 to 6. From then on every pull request has a link and the zip
is no longer needed.

## Part 9. The decisions (your item 9)
They stand at the end of this plan, under "Decisions needed", so that the asks come last.

## Part 10. What you do yourself (your item 10)
**Now**
1. Answer the decisions at the end, and give or refuse your go on this plan.
2. Give the 2.3 chat your decision of today **with the addition of 15:03**, so that its docs change-set records both;
   and your answer to GATE-FOLLOW, since it changes that chat's later parts.
3. Paste Part 4's block into the Art project. Paste Part 11's list into the Blender brief once change-set 0 has
   confirmed it (or now, marked as a draft).
4. Have the docs chat carry your look decisions of 13 and 14 November into the repo, before change-set 6.
5. Say whether the file that comes with this message can be saved on your laptop (Part 8: it proves the route the
   builds will take).

**For every change-set**
6. The Brave check, from change-set 2 on: the steps come with the zip; paste the page's report back.
7. Your go for the merge, in this chat, after the advisor agent has approved the final head: all 14 need it.
8. Once the 2.3 chat's guard change is merged: a yes to the exact text of each new shared dependency line in the root
   `Cargo.toml`, with the chat switched to Accept edits for that write.

**Once, at their time**
9. Name the minimum laptop and the recommended desktop (decision MACHINES), and run the bench page on each: the
   short re-run of every timed reading taken before they were named, then 6 (the look), 7 (prediction) and 12 (the
   closing run) as they come.
10. Start the preview hosting check in its own chat (decision PREVIEW-START): your Cloudflare account, a test project,
    its token. Later, for step 2.6: the deploy token in the `preview` environment.
11. Bring the Art project's sheets back before change-set 6.
12. Answer MODEL-PUBLIC before any model file is committed. Then run the Blender session on your PC (Claude through the
    Blender MCP, from the Art project's brief and Part 11's rules), export the GLB, and upload it to this chat.
13. When a Mac is available: one visit for the Safari rows collected so far.
14. Screens at 120 and 144 Hz, if you can reach one, for the smoothness rows of change-set 4.
15. Two feel calls in Brave, as design questions that stay yours (decision START-RULES): how far the jump reaches
    (change-set 4) and what happens when two soldiers meet (change-set 5).

**Not yours in this work:** nothing on the netcode proof test changes. Its chat needs nothing from this one.

## Part 11. The rules a model file must follow (your item 11)
**Who answers, and when.**
- This is the export and slot-naming part of the asset research: which checks an export passes (OQ-0005-AS3) and how
  a model's materials are named into the colour areas (OQ-0005-AS4). SPIKE_PLAN gives it to graphics-engineer,
  "verified like R3, in its own daytime session".
- **A first reading is done, today, for this plan.** graphics-engineer (Opus 5.5, effort max) read the drafts, the
  glTF 2.0 specification, the Blender exporter's documentation and source, and the three.js loader's source. The list
  below comes from it; every source line and address is in `graphics-engineer-asset-rules.md` and `asset-rules.md`
  beside this file.
- **Change-set 0 makes it firm, first after your go** (decision RULES-WHO), in about 6 hours of session work:
  1. graphics-engineer closes the open points listed under the block, as far as reading can close them;
  2. the verifier spot-checks the key claims against their sources;
  3. it writes this part's own pass marks, as SPIKE_PLAN asks of the research (the checks a file passes before it
     enters the game), and places them at change-set M;
  4. one docs pull request records it: a research file, the two open questions marked answered in ADR 0005 as rules
     with their proof still to come, and the one line your words of 2026-11-03 give this part: LOOK_V2 §8 says
     "Spline or Blender". That is a decision doc, so it merges on your go. If the docs chat that carries your look
     decisions has already written that line, change-set 0 leaves it.
- **The proof by a real export is not part of change-set 0.** It needs both ends: your Blender (one export of a cube
  and a small skinned figure, which the Blender session makes from rule 7's call) and the game's loader (change-set
  2). It runs when both exist, at the latest as the first step of change-set M, and it settles the facing, the scale
  and the names. Nothing is installed in this chat's container for it, so change-set 0 never waits for your PC and
  cannot hold change-set 1.
- **Until then the list is a draft.** It can go into the Blender brief today, marked as a draft: no rule in it is
  costly to change before the first model is made. The names in rules 12 and 21 are proposals of a naming convention,
  not of the look; they need your or the Advisor's OK, because the Art brief will use them.

**Does Slice 1's soldier need a skin? Yes.** ADR 0003's line for Slice 1 says "one skinned soldier", and ADR 0005
says "Soldiers are skinned": one mesh, with joints and weights. Your words of 2026-11-03 set the bar: "blockouts with a
skin, named colour slots and LOOK_V2's proportions are enough". The code-built stand-in of change-set 6 is therefore
built the way the loader builds a model: one skinned mesh per colour area on one shared skeleton, with the same joint
names and the same colour-area names, so the model replaces it without engine changes. The stand-in stays code: it is
named in the asset list as a builder, and the loading of a file is first used at change-set M. A blockout's skin may
be rigid: each part follows one joint.

**The list to paste** (a draft until change-set 0 confirms it):

```
Rules for a model file (draft of 2026-11-14)

The file and the export (Blender's own glTF 2.0 exporter)
1. One .glb file per model: Format "glTF Binary (.glb)".
2. Mesh: "+Y Up" on, "Apply Modifiers" on, "UVs" on, "Normals" on, "Tangents" off.
3. Materials: "Export" (never "No Export": it merges the colour areas into one); Images: None; vertex colour: None.
4. Skin: "Skinning" on, "Bone Influences" 4, "Include All Bone Influences" off, "Use Rest Position Armature" on.
5. Off, because the game does not read them: animations, shape keys, custom properties, cameras, lights.
6. No compression (Draco, Meshopt) and no detail levels.
7. The same as one call, for a Blender MCP script:
   bpy.ops.export_scene.gltf(export_format='GLB', export_yup=True, export_apply=True, export_texcoords=True,
   export_normals=True, export_tangents=False, export_materials='EXPORT', export_image_format='NONE',
   export_vertex_color='NONE', export_skins=True, export_influence_nb=4, export_all_influences=False,
   export_rest_position_armature=True, export_animations=False, export_morph=False, export_extras=False,
   export_cameras=False, export_lights=False, export_draco_mesh_compression_enable=False,
   export_meshopt_compression_enable=False)

Scale, axes, origin
8. One Blender unit is one metre: build at real size.
9. The model stands up along Blender's Z and faces Blender's -Y (the exporter turns that into +Y up, front +Z).
10. The origin is on the ground under the model's centre (for a soldier, between the feet). Apply all transforms
    (rotation and scale) before rigging and before export.

Names
11. Letters, digits and underscores only: no spaces, dots, colons, slashes or brackets, and no ".001" endings.
12. One colour area is one material. The material's name is the area's name, and it starts with its class:
    faction_ for an area that changes colour with the faction, unit_ for one that does not, emissive_ for one that
    glows (structure_ and environment_ are for buildings and ground). For the Conscript of the stand-in sheet:
    faction_coat, faction_top, faction_trim, unit_dark.
13. A faction_ area takes each faction's colour in the game: its colour in Blender is only a placeholder.
14. Only emissive_ areas glow: give each its glow colour as the material's Emission.
15. Every other area's colour is its material's base colour, set to the sheet's hex value.

Caps
16. A soldier as drawn (body, headgear and held weapon together) has at most 4 materials, so at most 4 colour areas.
17. Triangles: no cap is set yet. Keep a soldier with headgear and weapon near or under about 5,200 triangles (a
    quad counts as 2); that figure is a planning ceiling, and the real cap will be lower.
18. A weapon is its own .glb, by the same rules. A very large unit is one mesh per part under one parent, with
    parts times materials at most 25.

The skin
19. Yes, the soldier is skinned: one mesh for the whole body, on one armature.
20. Every vertex follows at least one bone and at most 4, and its weights add up to 1. A blockout may be rigid:
    each part follows exactly one bone.
21. The bones are exactly the game's joint list, with no helper or IK bones in the file. Proposed list: hips, spine,
    chest, head, upper_arm_l, lower_arm_l, hand_l, upper_arm_r, lower_arm_r, hand_r, upper_leg_l, lower_leg_l,
    foot_l, upper_leg_r, lower_leg_r, foot_r.

Not in the file
22. No baked light, no outlines or ink, no toon or custom shading: the game applies the look.
23. No textures and no vertex colours for now.
24. No model file goes into the repository before Mustafa has said whether models may be public.
```

**Where each rule comes from** (line numbers and addresses in full: `asset-rules.md`, sections 1 and 2).
| Rules | Source |
|---|---|
| 1 | ADR 0005, GPU rule 6 (:1803-1804): only glTF/GLB, through three.js's loader on both backends, never Spline's runtime; the exporter's documentation |
| 2, 4, 5 | the exporter's documentation and its option list in the source (the option names are those of its current main branch); the three.js loader binds one set of four joints and weights per vertex; ADR 0003 (:797): the procedural animation is ported, so the game drives the joints. "Apply Modifiers" on and "Tangents" off are proposals |
| 3, 12, 15 | the exporter writes one primitive per material that faces use, keeps each material's name, and merges them under "No Export" (its documentation and source); the loader gives one mesh per primitive and keeps the name; ADR 0005, GPU rules 1 and 2 (:1748-1770): slots name a material class, and the class list and names are content data, given only "for example". The prefix convention is a proposal |
| 6, 22, 23 | ADR 0005 (:1748-1755, :1810-1816) and SPIKE_PLAN (:390-391): a model carries no baked lighting and no ink; the first models are plain exports with no compression or LOD step. No textures or vertex colours yet is a proposal (OQ-0005-AS4) |
| 7 | the exporter's source (`__init__.py`): the same options by their script names |
| 8 | EDITOR_V2 `[DECIDED]`: "1 unit = 1 metre"; glTF: "The units for all linear distances are meters". No decision draft states the scale yet (already listed for the drafts' next revision) |
| 9, 10 | glTF: "+Y as up; the front side of a glTF asset faces +Z"; a skinned mesh ignores its own object's transform. The facing and the origin are proposals of the engine |
| 11 | the loader's source: node names lose `[ ] . : /`, and spaces become underscores, so "hand.L" would arrive as "handL" |
| 13, 14 | ADR 0005 (:1764-1769, :1582-1583): faction slots take the faction's colour at load; only emissive slots glow |
| 16, 18 | ADR 0005, GPU rules 3 and 4 (:1771-1795): at most 4 materials per pass for a soldier as drawn; a very large unit's parts are separate meshes under one group, at most 25 parts times materials |
| 17 | ADR 0005 (:1787-1792): about 5.2k triangles per soldier as drawn at Low, "if soldiers had the whole row": a planning input, not a budget (OQ-0005-AS5, AS6) |
| 19, 20 | ADR 0003 (:1397) and ADR 0005 (:1771-1774); the glTF specification on joints and weights; r3-rendering.md (R3-CHAR-REC1): rigid weights, one bone per part, is the baseline; an unweighted vertex gets a stray extra joint from the exporter |
| 21 | proposed by the engine: no draft gives a joint list. Its seed is what legacy's animation drives (LEGACY_INVENTORY GA2, GA7, GA8: two-bone arms and legs, spine, chest, head, hands on the gun) |
| 24 | your item 13 of today; ADR 0006 (:1105-1106) |

**Still open.**
- **Four colour areas are few, and the stand-in sheet already fits them:** Coat, Top, Trim and Dark, the rifle
  included; it folds skin, leather, wood and steel into Dark to get there. More areas would need another way to carry
  colour (a mask in vertex colours, or one texture), which nothing read so far researches (OQ-0005-AS4). Whether four
  stay enough is a look question, so it is yours and the Advisor's; the research gives the counts.
- **The faction's marking is a shape, not a colour** (star, chevron, flower). The rules recolour one model per
  faction; a shape that changes needs a rule of its own: a small part of its own per faction, or one model per
  faction. The sheet lets the stand-in leave the shape out at game size, so nothing in Slice 1 waits for this.
- The names: the class prefixes and the joint list are proposals of a naming convention, to confirm.
- Sharp or smooth edges in the normals: it depends on the ink family, your pick at change-set 6.
- Whether headgear is its own model, and the names of the weapon's grip points.
- Which Blender version runs on your PC: the option names were read on the exporter's current main branch.
- How the exporter cuts a vertex with more than 4 influences; whether Blender's scene unit scale changes the size.
- How many joints fit: three.js's skinning for WebGPU uses a uniform buffer and falls back to a bone texture for
  larger skeletons (its source); 16 joints are few either way.
- The checks a file must pass before it enters the game (OQ-0005-AS3), with candidates from this reading: a named
  material from the slot table on every primitive, normals present, one set of joints only, the joint list equal to
  the game's, no images or animations, a known-size object measuring its size in metres, the caps, the file size.
- The proof by a real export (above), which settles the facing, the scale and the names.

## Part 12. The first Blender model, and the asset spike's marks (your item 12)
**The change-set: M, "The first model".** It replaces the code-built soldier with the Blender one. It takes the next
free place after change-set 6 (Look) once three things hold: the model file exists; change-set 0's rules are merged;
you have answered MODEL-PUBLIC. After 10 (Body) the model also moves; before it, it stands in one pose.

**What M builds.**
- Loading: three.js's glTF loader on the page; the file is a plain GLB export, with no compression and no LOD step,
  as ADR 0005 and SPIKE_PLAN put it for the first models.
- The asset list as client content: each model by id, with its file, its slots and their classes, and its counts; a
  definition that names a missing id, or a slot with an unknown class, is refused at load, naming the file and field.
- Materials: each slot's name maps to a material class; the engine applies the look (the model carries none); one
  shared material per distinct slot content; faction colour per instance.
- The skin, bound to the same joints the code-built soldier uses, so the animation of change-set 10 drives both.
- An unlit debug view, to read a slot's colour back from the screen.
- The simulation is untouched: an asset decides nothing (a test changes every client value and compares hashes).
- Its first step is the proof of Part 11's rules on the real file, with the checks change-set 0 wrote.
- Reviewers: graphics-engineer, art-director, security-auditor (a file from outside enters the game); then
  perf-auditor, test-engineer.

**How the file reaches the repository.** Under MODEL-PUBLIC (a): you upload the GLB to this chat; I commit it in M's
pull request. Where model files live for good is an open question of the rest of the asset research (OQ-0005-AP1); M's
plan names a place and says it may move.

**One question of ADR 0005 comes nearer with this route:** whether a model drafted by an AI may ship (OQ-0005-AP3). It
is yours. Nothing in Slice 1 needs the answer: these are test builds.

**The asset spike's marks.** SPIKE_PLAN places the asset spike "before the first authored model enters the game". By
your addition of 15:03 it is treated like the tests of your decision: each mark is measured on the real game, here or
later, none dropped or loosened.

| Mark | What it asks, in short | At M | Later, and why |
|---|---|---|---|
| PM-0005-38 | the automated step processes every test model and writes its list entry; the entry's counts equal an independent recount | not at M: the first model enters as a plain export, with a hand-written entry. What M does show: the entry's triangle, material and texture counts equal a recount by script | when the automated step exists. The rest of the asset research names it (OQ-0005-AP1; compression, LOD levels and budgets with it): one research unit, after M has given the first real counts and before Slice 2's models |
| PM-0005-39 | every asset file's name holds its hash; served immutable; a repeat visit costs 0 bytes; under 25 MiB | the hash in the name and the size, at M | the cache header and the repeat visit on a preview link: at step 2.6, or at M if previews exist by then |
| PM-0005-40 | the gate fails an over-budget asset and a budget raised without the budget file | a test compares the soldier's entry with the two caps the drafts derive | the gate's own check with its planted defects: 2.3, after M; the budgets beyond the two caps come from the rest of the research (OQ-0005-AS5) |
| PM-0005-41 | a glTF reader planted in `sim` or in `server` fails the dependency check; the asset tool imported from the client fails | nothing new at M (M adds no Rust glTF reader) | the `sim` half with 2.3's dependency check; the `server` half at Slice 3; the tool's halves when the tool exists |
| PM-0005-42 | no slot without a class; every faction slot shows its faction's colour, read from the materials and probed on screen, on both backends | at M, for the soldier in the factions that exist as data by then | every faction at Slice 2; Edge, Firefox and Safari on the named machines |
| PM-0005-43 | the frame-time, GPU-time and count marks pass in the full fight built from real models | not at M | Slice 6, once every family in the fight has its model |
| PM-0005-44 | no flicker between detail levels; your verdict | conditional by its own words: only if the research proposes detail levels and a model carries them | with the first such model |

The look proof test's share of ADR 0005 lands here too (3.8): PM-0005-33 and -34 at M, PM-0005-35 and -36 in part.

## Part 13. Risks
1. **The drafts are built on before they are accepted.** A failed mark means redone code (your rule). The most
   exposed, each measured at its first build: the transferred block (change-set 5; its alternative is the shared ring
   behind the isolation flag), the timer pacing (2 and 3), our own movement (4; its alternative is the library's
   controller), the ink family and the Low frame (6), prediction's cost (7).
2. **CI lags behind the code.** Until the 2.3 chat's checks follow, Firefox, WebKit, and Wasm in CI are unchecked,
   and marks stand as "shown", not "met". A defect only another engine shows would surface late.
3. **Nothing timed can pass** until the machines are named. A surprise on the real minimum laptop would then come
   after code rests on a reading from your laptop.
4. **Two chats touch neighbouring files** (the root `Cargo.toml`, the root `package.json`, the lockfiles). The
   ownership rule of Part 7 keeps each file with one author; lockfile conflicts are resolved by regenerating.
5. **The estimate.** 14 units is a long chain with a merge and a check between each. Usage limits have cut this
   project's sessions before; every change-set keeps a checkpoint, so a cut loses one step.
6. **The look's inputs may come late** (the carry, the sheets, a machine). The order bends (Part 1); the look then
   lands later in the slice.
7. **The stand-in must match the model's contract** (slots, joints). If change-set 0's rules change after the
   stand-in is built, the stand-in is rebuilt: about a day.
8. **The netcode proof test may change the seams of Part 6.** Each is one module or one data value by design; the
   largest is how commands are timed.
9. **The newest three.js on WebGPU in Brave** is the first thing change-set 2 tries; if Brave falls back to WebGL 2
   on your PC, the label says so and the look work still holds, because every effect must exist on both.
10. **How a build reaches your laptop is not proven** (Part 8). If the file route fails, every Brave check waits for
    another route, and the cheapest of those needs the 2.3 chat's CI.
11. **The Art block goes out before your look decisions of 13 and 14 November are in the repo.** It says that those
    win; a sheet made from the older brief may still need a second pass.
12. **The rules for a model file are proven late.** They rest on documentation and source until the first real export
    (Part 11). A rule that the export disproves changes the stand-in (about a day) and the Blender brief.

## Part 14. This plan's review
Your rule: the advisor agent reviews the plan; no review rounds and no verifier.

**The review of revision 1.** The advisor agent (Opus 5.5 by its agent file, effort max passed by this chat) was
started after 14:43 UTC; its notes, `advisor-plan-r1.md` beside this file, were last written at 15:08 UTC and end
with DONE. Verdict: **REVISE**, with 3 High, 11 Medium and a list of Low findings.
- What it checked, by its notes: both of your messages; the plan in full; 49 pass-mark rows against the drafts' own
  tables; the done-when lines of ADR 0001 to 0005; the count script (201 of 201) and its expected set against
  SPIKE_PLAN's table; every quotation of yours against its source (none misquoted); LOOK_V2 in full; the guard's
  merge places and the settings' deny lines; the 2.3 plan.
- What it could not check, by its own words: whether a cloud chat can hand you a file; the Blender, glTF and three.js
  claims of Part 11 (change-set 0's verifier covers them); the session's own model.

| Its finding | Severity | Where revision 2 answers it |
|---|---|---|
| The route that brings a zip to your laptop has no evidence; the zip's server has no owner | High | Part 8 (the route named, two proofs before anything rests on it, never a committed zip); decision BRAVE-ROUTE (four routes); change-set 2 owns the server, with security-auditor |
| The default check ran the netcode spike's own check on a local merge, against "never touch spikes/" | High | Part 7: this chat searches its own files only and puts nothing of that branch on disk |
| Separate tests were thrown away before the machine their mark names exists; one stood before its subject | High | Part 3.2: each is kept as a patch and a build until judged; the side the game builds is provisional until then; the two-instance numbers move to change-set 5 |
| PM-0002-16 and -19 called met where they are only shown | Medium | Part 2 |
| SIMD decided on part of its suite | Medium | Part 3.4, PM-0004-25: a reading; off until the whole suite passes |
| PM-0004-72's CI clauses can be shown earlier | Medium | Part 3.5: readings at change-sets 8 and 9 in a 60 Hz build |
| PM-0001-20 runs under both ways of timing commands | Medium | change-set 7 builds both; Part 2 |
| No list of what stays open after change-set 12 | Medium | Part 2, "What is still open when change-set 12 merges"; change-set 12's report |
| The gate's network step is tied to the netcode proof test; option (b) of GATE-FOLLOW sets your words aside | Medium | Part 7 and decision GATE-FOLLOW (its own sub-question; option (b) says so) |
| MACHINES ignored your standing words on timing | Medium | decision MACHINES, rewritten from them |
| The Art block carried three look choices of the plan's own | Medium | Part 4, rewritten (also for the sheets that arrived): ranking left to the Art project, the cap worded as the drafts word it, the height rule cited to LOOP_V2 §9 and asked, your decisions of 13 and 14 November win |
| Change-set 0 could wait for your PC, or install Blender unreviewed | Medium | Part 11: nothing is installed; the proof by a real export runs when both ends exist, at the latest at change-set M; decision RULES-WHO |
| Words credited to you that are the drafts' | Medium | Part 12 |
| The jump's reach and soldiers meeting are your design calls, taken silently | Medium | decision START-RULES; Part 10 |
| Lows (a Part 9 heading; planted defects named; later clauses; reviewers for M and 5; risk tier 2; what is only for development; estimates of 3 and 4; step 2.6's conditions; Node; wording) | Low | folded where each stands |

One finding is answered differently from what it asked: it found no source for "higher is lighter" and asked to
delete it. The source is LOOP_V2 §9 ("height read through brightness (higher is lighter)", a default for greybox
maps); Part 4 now cites it and asks the Art project instead of stating it.

**Its ruling, inside its authority.** The scan test in the sim crate may stand in for the 2.3 chat's float ban until
that lint merges, on the three conditions Part 7 states.

**The re-read of revision 2.** <!-- reread -->

## Decisions needed
1. **SPLIT-25: is step 2.5 built as two change-sets?**
   - Why it matters: ADR 0002 says one change-set per step. Two bring the first page one change-set earlier and halve
     each review.
   - Options: (a) two: the first page, then the loop through the local server; (b) one.
   - Recommendation: (a).
2. **FIRST-SOLDIER: is the first soldier the Conscript of the Art sheets, and which gun is first?**
   - Why it matters: the stand-in, the Blender model and the first playable gun follow from it. The sheets you sent
     show the Conscript's shared tier-one uniform with a rifle; LOOP_V2's reference gun for feel is the Imperial
     Warrior's SMG, and it says to fix the AMK rifle's rhythm (one shot, then a 3.5 s reload).
   - Options: (a) the Conscript of the sheets, with the SMG and the rifle both playable in change-set 8, and the SMG
     as the gun whose feel you judge first; (b) the same soldier, with the rifle as the gun you judge first; (c)
     another soldier, with a sheet of its own.
   - Recommendation: (a).
3. **GATE-FOLLOW: who builds the gate and CI checks that follow steps 2.4 and 2.5, and when?**
   - Why it matters: your words of 2026-11-12: "The guard, the gate and CI work stays with Opus 5.5." The 2.3 plan
     lets these checks ride steps 2.4 and 2.5 and ties them to test reports; for the six tests of your decision those
     reports no longer come first. Until the checks exist, nothing in CI runs WebAssembly, Firefox or WebKit on the
     new code: marks that need them are only shown (they passed in this chat's container), not met.
   - Options: (a) the 2.3 chat builds each right after my change-set merges: after 1 (generated types in step),
     after 2 (WebAssembly in CI), after 3 (the browser jobs, the determinism step, the float ban from change-set 3's
     numbers); (b) this chat asks you for each gate and CI edit with its exact text: quicker, but for those edits it
     sets aside your words of 2026-11-12; (c) they wait until Slice 1 ends.
   - One check needs its own answer. The gate's network step re-proves the link simulator of the netcode proof test,
     which is not one of the six, and your words of 2026-11-13 still stand for it: "Every gate check that needs a
     result joins later, when that result is in." (i) It waits for the proof test's report, as those words say;
     (ii) it follows change-set 3 like the others.
   - Recommendation: (a), with (i).
4. **MACHINES: when do you name the minimum laptop and the recommended desktop?**
   - Why it matters: your words stand: "The laptop and desktop stay on my timing: rough checks first, then a short
     re-run once they're named", and you name them before the look proof test. That test is no longer separate: its
     first measurements on the real game are change-set 6. Until the naming, your laptop stands in "for rough checks
     only", so no timed mark can pass.
   - Options: (a) "before the look proof test" now reads "before change-set 6", and every timed reading taken earlier
     gets its short re-run on the named machines; (b) earlier, before change-set 5, so the hand-off is judged once and
     needs no re-run; (c) later than change-set 6, with the look's timed marks re-run too.
   - Recommendation: (a): it is your own timing, mapped onto the real game. The Mac: the Safari rows are collected
     and run in one visit when a Mac is available.
5. **BRAVE-ROUTE: how do builds reach your laptop before preview links exist?**
   - Why it matters: every merge from change-set 2 on waits for your Brave check, and the route in Part 8 is not
     proven yet.
   - Options: (a) a zip through this chat's file tool, and one command (Part 8); (b) a build made by CI and
     downloaded from the pull request's page on GitHub, which needs a CI job from the 2.3 chat first; (c) you build
     from the branch on your PC, which needs the Rust and Node toolchains, about half an hour once; (d) no Brave
     check until step 2.6's links, which holds every merge from change-set 2 until then.
   - Recommendation: (a), if you can save the file that comes with this message; if not, (c) until the CI exists,
     then (b).
6. **PREVIEW-START: does the preview hosting check start now, in its own Opus chat?**
   - Why it matters: its two-day wait decides when links arrive.
   - Options: (a) now; (b) after step 2.3's first part; (c) later.
   - Recommendation: (a).
7. **STAND-IN-LOOK: may proportions and weapon silhouettes be judged on the code-built soldier?**
   - Why it matters: ADR 0003 says these need authored models; three steps of your look checklist rest on them.
   - Options: (a) no: you judge them on the Blender model at change-set M, and judge the code-built soldier for the
     techniques only (ink, shading, colour, light); (b) yes: you judge them at change-set 6 on the code-built soldier.
   - Recommendation: (a).
8. **COMPARE: do the comparisons against things the game will not build still run?**
   - Why it matters: your rule keeps a separate test where the real game cannot answer a question. These marks ask
     such questions; inside Slice 1 they cost about four days in all: one Wasm instance or two (change-sets 3 and 5),
     the block's layout and the shared ring (5), our Wasm's size against two other libraries (4), and a library's
     character controller against our own movement (after 4). Each build is kept, so it can be judged on the named
     machines later.
   - Options: (a) all run; (b) the character-controller comparison (one day) runs only if our own movement misses
     its marks. That changes the wording of one mark (PM-0004-34), so it needs your word.
   - Recommendation: (a): no mark changes.
9. **MODEL-PUBLIC: may a committed model be public?** (OQ-0006-18)
   - Why it matters: this repository is public. A model file pushed to it can be read by anyone, and its history
     keeps it. The login on previews starts only with the first preview that reaches the test game server (Slice 3).
   - Options, as the draft gives them: (a) a login on previews is enough: models are committed, and the wall covers
     previews only; (b) models live in a private store outside the repository and the build fetches them with a
     secret: the build is then no longer free of secrets, and the store is not researched; (c) the repository becomes
     private: CI minutes are then counted against your plan, and the reviewer on previews needs a paid plan.
   - Recommendation: (a) for the first soldier and the other test models, as the infra review recommends "until the
     first model that must stay unseen", and the question again before such a model. Under (a) only the exported GLB
     is committed; the Blender file stays with you until the asset research says where sources live. No model file is
     committed before your answer.
10. **RULES-WHO: who answers the model-file rules, and when?** (Part 11)
    - Why it matters: the Blender brief waits for them, and you asked for them early. The draft list is in Part 11
      now; what is left is to confirm it and record it.
    - Options: (a) this chat, as change-set 0, first after your go: about half a day before change-set 1 starts;
      (b) this chat, after change-set 2: the first page comes half a day sooner, and the Blender brief works from the
      draft list until then; (c) a separate research chat beside this one: no delay here, one more chat for you to
      steer.
    - Recommendation: (a).
11. **SCOPE-RELOAD: is the active reload part of Slice 1?**
    - Why it matters: LOOP_V2 decides the active reload, the sprint-to-fire delay and the AMK's new rhythm; no draft
      places them in a slice. How a reload tap is judged depends on a question the netcode proof test still answers
      (time stamps inside a tick).
    - Options: (a) Slice 1 has the plain reload; the three come with Slice 4's feel pass; (b) the active reload joins
      change-set 8: about 400 lines more, and its judging may be redone after the proof test.
    - Recommendation: (a).
12. **START-RULES: what do the jump and two soldiers meeting start from?**
    - Why it matters: both are open design questions that stay yours, with the game-designer: how far the jump
      reaches (OQ-0002-6), and whether soldiers pass through each other, push softly or block (R2-SYNC-OQ5). Slice 1
      must start from something, and you will feel both in Brave.
    - Options: (a) start from legacy: the jump integrated at the tick from legacy's numbers, and soldiers treated as
      legacy treats them (change-set 4's plan reads that rule from the legacy code); you judge the jump at change-set
      4 and the meeting at change-set 5, and both questions stay open until you answer them; (b) you decide both now.
    - Recommendation: (a).

## For Mustafa
1. Answer the twelve decisions above.
2. Give or refuse your go on this plan. With your go, and once your decision of today is on main, change-set 0
   starts; its pull request also carries this plan to main as a report, with your two messages in its appendix.
3. Pass your decision of today, with the addition of 15:03, and your answer to GATE-FOLLOW to the 2.3 chat.
4. Paste Part 4's block into the Art project.
5. Say whether the file that comes with this message can be saved on your laptop (it proves the route of Part 8).
