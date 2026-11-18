# Change-set 2 of the build plan, "First page" (foundation step 2.5, first half): the plan, DRAFT

_Drafted by engine-architect (Opus 5.5, effort max) on 2026-11-18, beside pull request #60, on Mustafa's words of that
day: "You may write change-set 2's plan beside it. No new library of change-set 2 is committed before that pull request
is merged." (`kit-hold-ci/mustafa-2026-11-18-1624-merge-58.md`, saved by script). For the main session (lane 1) to adopt
or change, then for the advisor agent's plan review. Nothing is built. No library was fetched, installed or built for
this draft: only documentation and registry pages were read (section 19). Base: main as it will stand once #60 has
merged (today main is 9b4b518). Line numbers are those of the working tree on 2026-11-18. Working notes, with one line
per source: `kit-hold-ci/engine-architect-cs2-notes.md`._

## 0. In one screen
- **Goal.** The straight path from the simulation to the screen: the Rust simulation, built as WebAssembly, runs in
  one module Web Worker that paces 30 ticks a second and hands the page one transferred block per tick. The page, which
  never touches WebAssembly, draws an empty lit ground with three.js's WebGPU renderer (WebGL 2 as the fallback) and
  names its backend. With it comes the small server that lets Mustafa open the build in Brave from a zip.
- **What he sees.** An empty ground with a 1 m grid, and a corner readout that names WebGPU or WebGL 2, counts ticks at
  about 30 a second and shows his frame rate. A "Copy report" button. The match pauses while the tab is hidden.
- **New libraries.** wasm-bindgen `=0.2.130` (Rust, in `sim-wasm` only); the wasm-bindgen command-line tool 0.2.130,
  which is not in any manifest; `three` 0.187.0; `@types/three` 0.187.0 and the six packages it pulls in; `vite`
  8.3.7, which becomes a direct dependency but is already in the lockfile. No random-number crate unless RNG-CRATE says
  so (section 16).
- **Planned size.** About 2,900 hand-written lines, against the build plan's 2,000. Which stop line holds is SIZE-2.
- **Steps.** 0 paper and preconditions · 1 the Rust core, with no new crate · 2 the one dependency step and its
  security answer · 3 the bindings and the build script · 4 the Worker · 5 the page · 6 the small server · 7 the
  browser tests · 8 readings, planted defects, docs, the zip, reviews.
- **Open questions for Mustafa:** RNG-CRATE, OVERFLOW-3P, CI-WASM, SIZE-2 (section 16).
- **Where the build plan is wrong or out of date for change-set 2:** twelve points (section 17). The largest: OQ-0002-7
  (is wasm-bindgen's new home healthy) must be read before wasm-bindgen enters, and the build plan places it nowhere;
  ADR 0004's random-number crate is being dropped by its own project.

## 1. Goal
Build foundation step 2.5's first half (the build plan, Part 1, change-set 2, lines 298-314; SPLIT-25 (a)): the path
simulation, block, Worker, page, renderer, from end to end. Measure on it what the build plan puts here first (Part 3.1
and Part 3.3: the Worker's pacing, the hand-off's cost, the Wasm file's size, the backend each browser gets, what a
double-click on the built page does). Give Mustafa a build he can open in Brave.

It closes these lines of step 2.5's done-when (ADR 0002 :1495-1500), as far as change-set 2 can:
- "the page posts its newest input each frame [...] and the page receives one block per tick and draws an empty scene
  from it, naming its backend" (ADR 0003's milestone, first half; build plan Part 2, :467);
- "the page makes zero Wasm calls" (PM-0002-2; :469).

The loopback and its four messages, every tick on the newest post, and the first recording's hashes are change-set 3's.

## 2. What a player or Mustafa can see after it
- **The scene:** an empty ground with a grid of 1 m squares, seen from above at an angle, lit by a sun and a soft fill
  light. Colours and light are plain placeholders, written as such in the code. LOOK_V2 owns the look, and lane 2's
  change-set 6 builds it. Nothing here decides it.
- **The readout,** in development builds only: the backend ("WebGPU" or "WebGL 2 fallback", the two names of ADR 0003
  part 4) with the adapter's info when the browser gives it; the tick counter and its rate (about 30 a second); blocks
  received; frame rate and frame time; errors (0); isolation (yes or no); the build's commit; a "Copy report" button.
- **A hidden tab pauses the match:** coming back, the tick counter carries on from where it stopped (ADR 0004 part 5).
- **`?backend=webgl`** forces the WebGL 2 fallback (three.js's `forceWebGL`).
- **A double-click on `index.html`:** if the browser will not run the page's module scripts from a file, a plain line
  says how to serve the page instead. What Brave does there is a measurement (OQ-0002-1's first result).

## 3. Before any edit
1. **#60 is merged,** on his go (his words above). Until then nothing of change-set 2 is committed on lane 1's branch,
   not code and not a checkpoint. The branch `claude/wonderful-hamilton-jdmlnf` is #60's head, so any commit there
   would join #60, which his words keep to the kit hold and the two CI edits ("nothing else in it"). Until then this
   plan and its notes stay in `.checkpoints/kit-hold-ci/`.
2. **The advisor agent's APPROVE** on this plan. His go on the build plan covers each change-set "as long as a
   change-set's plan stays inside this one; a plan that leaves it comes back to you" (build plan Part 0, :215-218). The
   departures in section 17 and the questions in section 16 go to him first.
3. **His answers to RNG-CRATE and OVERFLOW-3P before step 2**, because both change what the manifests hold, and the
   security answer covers exactly that content. CI-WASM and SIZE-2 hold no step.
4. **main taken in** by `git merge --ff-only origin/main`, which CLAUDE.md's Never line allows in the main folder.
5. **The "Lane requests" issue (#59)** read at the start (Part 15.3). Only comments by the `Mergincanli` account count,
   and none is his word.
6. **Whether change-set 1's test zip saved and unpacked on his laptop** (STATUS, "Waiting on Mustafa"). His BRAVE-ROUTE
   answer is "A, if not possible: B" (build plan, :27). If the zip route failed, route B (a build made by CI) needs a CI
   job, asked of him with its exact text, and the Brave check waits for it.

## 4. Files and folders, with their lane (Part 15)
Every file below is lane 1's (Part 15.2: `crates/**`, `packages/types`, `packages/sim-worker`, `packages/e2e`, the build
files, and in `packages/client` the app shell, the overlay and counters, the interface file and, until its hand-over,
the first drawing code). No kit, gate or CI file is edited in this change-set (section 18).

### 4.1 The client package, cut by owner from its first day (Part 15.1, condition 1)
Part 15 says the folder names "are the ones change-set 2's plan starts from; engine-architect reviews them there"
(part-15 :97-98). Only the folders marked cs2 are created now. The rest are named so that each hand-over is a list of
folders.

| Path under `packages/client/` | Holds | Owner | Created |
|---|---|---|---|
| `index.html`, `vite.config.mjs`, `serve.mjs`, `scripts/` | the page, the build configuration, the zip's small server, the build script | lane 1 (build files: 15.1, condition 2) | cs2 |
| `src/main.ts`, `src/app/` | the app shell: boot order, the Worker link, the page's side of the hand-off, the frame loop, the error hook, the flags | lane 1 | cs2 |
| `src/picture.ts` | the interface file between the tracks | lane 1; engine-architect reviews every change (15.2) | cs2: start, draw, counters, resize, stop. Later members join when a change-set needs them: the cursor ray (4), warm-up and quality (lane 2's need for 6), the muzzle's point and the plain-line hook (8) |
| `src/overlay/` | the HTML readout and counters, the copy report | lane 1 | cs2 |
| `src/harness/` | the measurement harness's CPU side | lane 1 | cs2 |
| `src/render/` | the renderer's set-up and the drawing | lane 1 until change-set 5 merges, then lane 2 (15.2's hand-over) | cs2 |
| `src/input/` | input capture into the newest input | lane 1 | 3 |
| `src/dev/` | development panels other than the camera's (the bench mode) | lane 1 | 5 |
| `src/soldier/`, `src/camera/` | the stand-in soldier; the camera with its tuning panel | lane 1 until 5 merges, then lane 2 | 4 |
| `src/plain/` | the plain tracers, flash and hit marker | lane 1 until 9 merges, then lane 2 | 8 |
| `src/fx/`, `src/fx/proof/`, `src/audio/`, `src/soldier/anim/`, `src/assets/` | lane 2's units (15.2) | lane 2 | 11, the VFX proof, 11, 10, M |

One proposal of this plan: the harness's GPU half (timestamp queries, change-set 6) lives inside `render/`, not in a
folder of its own. Lane 2 builds it after `render/` has passed to lane 2, so it never changes hands.

### 4.2 Every file this change-set adds or edits
| File | New or edit | What |
|---|---|---|
| `crates/sim/src/lib.rs` | edit | the modules; the existing overflow test stays |
| `crates/sim/src/world.rs` | new | the world: the tick counter, the match seed, the authority step, the canonical state bytes |
| `crates/sim/src/state_hash.rs` | new | the state hash (FNV-1a, 64 bits) and its tests |
| `crates/client-endpoint/src/layout.rs` | edit | `Section::align()`; lookups by name evaluated at compile time (change-set 1's follow-ups) |
| `crates/client-endpoint/src/block.rs` | new | the block writer, header only, into one preallocated buffer |
| `crates/client-endpoint/src/schedule.rs` | new | the local schedule: when the next tick is due, and dropping a gap |
| `crates/client-endpoint/src/lib.rs` | edit | the modules |
| `crates/sim-wasm/Cargo.toml` | edit | crate types `cdylib` and `rlib` (WASM-ARTIFACT); its dependencies |
| `crates/sim-wasm/src/lib.rs` | rewrite | the wasm-bindgen control calls; the stub test goes |
| `Cargo.toml` (root) | edit | one line in `[workspace.dependencies]` |
| `Cargo.lock` | regenerated | by one resolve-only command (section 7) |
| `package.json` (root) | edit | `vite` 8.3.7 in `devDependencies` |
| `package-lock.json` | regenerated | by one resolve-only command (section 7) |
| `packages/types/package.json` | edit | an `exports` entry (change-set 1's follow-up; PACKAGE-ENTRIES) |
| `packages/sim-worker/package.json` | edit | `exports`, its dependency on `@shamble/types`, `test` limited to `src/` |
| `packages/sim-worker/src/{worker,pacing,block-pool,read-block,messages,stats,index}.ts` | new (`index.ts` rewritten) | the Worker's host; the stub `PACKAGE` and its test go |
| `packages/sim-worker/src/*.test.ts` | new | unit tests in Node |
| `packages/sim-worker/boundary/sim-wasm.test.ts` | new | the boundary tests: Vitest in Node on the built Wasm |
| `packages/client/package.json` | edit | `three`, `@types/three`, the two workspace dependencies |
| `packages/client/index.html`, `vite.config.mjs`, `serve.mjs`, `serve.test.mjs` | new | the page, the build configuration, the small server and its tests |
| `packages/client/scripts/build.mjs`, `build.test.mjs` | new | the build script and the tests of its checks |
| `packages/client/src/main.ts`, `picture.ts`, `styles.css` | new | the entry, the interface file, the readout's style |
| `packages/client/src/app/{boot,worker-link,error-hook,flags}.ts` and tests | new | the app shell |
| `packages/client/src/render/{three-picture,empty-scene}.ts` | new | the renderer and the empty scene |
| `packages/client/src/overlay/{overlay,report}.ts` and tests | new | the readout and the copy report |
| `packages/client/src/harness/{frame-times,run-header}.ts` and tests | new | the harness's CPU side |
| `packages/client/src/index.ts`, `index.test.ts` | removed | the stubs |
| `packages/e2e/package.json` | edit | `test` runs the browser-free project only |
| `packages/e2e/playwright.config.ts` | new | two projects (`node`, `chromium`); the small server as `webServer` |
| `packages/e2e/browser/{skeleton,isolation,clock,allocation,file-url}.spec.ts` | new | the browser tests |
| `packages/e2e/browser/render-counts.json` | new (a fixture) | the empty scene's render counts per backend: ADR 0003's committed baseline (build plan :472) |
| `.gitignore` | edit | `pkg/` (wasm-bindgen's output) and `playwright-report/`. Not a kit or gate file: CLAUDE.md's protected list does not name it |
| `docs/reports/2026-11-DD-cs2-first-page.md`, `docs/STATUS.md`, `docs/ROADMAP.md`, `docs/REPO_MAP.md`, `docs/DECISIONS.md`, `docs/adr/0002-code-structure-build-toolchain.md` | docs pass | the report; docs-keeper's pass; his words by script; ADR 0002's dated pointer for SPLIT-25 ("change-set 2's docs pass carries that line", build plan :23) |

Not touched: `crates/protocol`, `crates/room`, `crates/devtools`, `packages/net`, `packages/types/src/**`, `legacy/`,
`tools/`, `.claude/`, `.github/`. The generated types do not change: the header stays `tick` plus `layout_hash`, so
`LAYOUT_HASH` stays `0x2df78c30`. The two doc lines of #58 that "claim more than the code does" sit in `ts_export.rs`,
`layout_ts.rs` and `messages.rs`; this change-set touches none of them, so their correction stays with the next pull
request that does (his words of 2026-11-18).

### 4.3 Import rules (by review now; the 2.3 chat's lint later)
- Only `src/render/` imports three, and later lane 2's folders. `app/`, `overlay/` and `harness/` reach the picture
  only through `picture.ts`, so the hand-over at change-set 5 moves no import.
- The page never imports the Wasm glue (`packages/sim-worker/pkg/`) or `@shamble/net`. From `@shamble/sim-worker` it
  imports only the Worker's constructor (`@shamble/sim-worker/worker?worker`, Vite's query suffix) and the message
  types and statistics helpers. This is checked at run time (PM-0002-2) and by a scan of the built page's chunks (no
  glue in any chunk but the Worker's). That closes part of CLIENT-NET-RULE (STATUS_ARCHIVE :807-812) before the 2.3
  chat's lint exists.
- `sim-worker` is type-checked with the `webworker` library and no `dom` library, so a DOM name fails the check (ADR
  0002 part 3: "`sim-worker` and `net` never import rendering or DOM packages").
- Nothing imports `legacy/` or the spike's folder.

## 5. The exact symbols
**Rust (lane 1).**
- `sim`: `pub const TICK_HZ: u32 = 30` (TICK-30; ADR 0004 part 4: one build-time constant). `pub struct World`, holding
  `tick: u32` and `seed: [u8; 32]`. `World::new(seed: [u8; 32]) -> World`, `World::tick(&self) -> u32`,
  `World::authority_step(&mut self)` (named as ADR 0004 part 3 names it; it advances the tick with an explicit checked or
  wrapping operation, ADR 0004 part 4, chosen with rust-reviewer), `World::write_state(&self, out: &mut Vec<u8>)` (the
  canonical bytes: fixed field order, fixed widths, little-endian, no `usize`: ADR 0004 part 9) and
  `World::state_hash(&self, scratch: &mut Vec<u8>) -> u64`. In `state_hash`: `pub fn fnv1a_64(bytes: &[u8]) -> u64`,
  with RFC 9923's 64-bit offset basis and prime. Without a generator the seed is kept and hashed; ADR 0004 part 8 puts
  the streams' state in the hash.
- `client-endpoint::layout`: `Section::align(self) -> u32` (the largest field's size). `pub const fn section(name: &str)
  -> Section` and `pub const fn field(section: &str, field: &str) -> Field`, which panic on a missing name; used in a
  `const`, that panic fails the build (change-set 1's follow-up: "the block writer takes offsets through a by-name
  handle that fails the build on a missing name").
- `client-endpoint::block`: `pub struct BlockWriter`, holding one `Box<[u8]>` allocated once, of the header's stride
  plus an optional padding that development builds use for the hand-off readings. `BlockWriter::new(pad_bytes: u32)`,
  `write_header(&mut self, tick: u32)` (writes `tick` and `LAYOUT_HASH`, each a `u32` in little-endian, at offsets taken
  by name, with their element types asserted at compile time), `bytes(&self) -> &[u8]`, `byte_len(&self) -> u32`. No
  `unsafe`.
- `client-endpoint::schedule`: `pub struct LocalSchedule`. `new(now_ms: f64, tick_hz: u32)`, `next_due_ms(&self) -> f64`
  (anchor plus (ticks since the anchor plus 1) times the tick period: the plain accumulator of ADR 0004 part 5, without
  float drift), `ticked(&mut self)`, `rebase(&mut self, now_ms: f64)` (drops a gap, for a hidden tab or a stall).
- `sim-wasm`: `#[wasm_bindgen] pub struct Sim`, holding the world, the writer, the schedule and the newest input's
  sequence number. `Sim::new(seed: &[u8], mode: u8, pad_bytes: u32, now_ms: f64) -> Result<Sim, JsError>` (mode 0, local,
  is the only one accepted until Slice 3), `block_ptr(&self) -> u32`, `block_len(&self) -> u32` (each read once after
  create; the buffer never moves), `input(&mut self, sequence: u32, now_ms: f64)`, `step(&mut self) -> f64` (runs one
  tick, writes the block, returns when the next tick is due) and `rebase(&mut self, now_ms: f64)`. The per-tick calls
  take and return plain numbers only. A `Result`, a string or a slice would add glue calls per tick against PM-0002-3's
  limit of four. The seed check is a plain Rust function, tested natively. The binding is a thin wrapper around it.

**TypeScript (lane 1).**
- `sim-worker/messages.ts`: `PageToWorker` (`start`, `input` with `sequence`, `viewTime` and `returned: ArrayBuffer[]`,
  `visibility`, `report-request`), `WorkerToPage` (`ready`, `block` with `buffer` and `doneAt`, `error`, `report`),
  `WorkerStats`, `MODE_LOCAL = 0`. No message carries the seed.
- `sim-worker/pacing.ts`: `MAX_TICKS_PER_WAKE = 7` (ADR 0004 part 5, S18) and `Pacer`, a pure state machine driven by a
  clock it is given: step while the next tick is due, at most 7 steps per wake, then sleep until the due time the step
  returned, or rebase after a pause or a longer stall, counting each.
- `sim-worker/block-pool.ts`: `BlockPool(blockBytes, size)`, `take()` (counts a miss when empty, never drops a tick),
  `give(buffer)` (refuses a wrong size), `misses`.
- `sim-worker/read-block.ts`: `copyBlock(memory, ptr, len, into)`, through a fresh view of `memory.buffer` on every call
  (ADR 0002 part 5).
- `sim-worker/stats.ts`: `Ring` (a preallocated `Float64Array`) and `percentile` (nearest rank, as the drafts define
  p99). The client's harness imports them.
- `sim-worker/worker.ts`: the entry. It loads the glue, draws the seed with `crypto.getRandomValues` (ADR 0004 part 8),
  creates `Sim`, paces, copies each block once into a pooled buffer, posts it with its transfer list and checks that
  the sent buffer is detached (PM-0002-6), pauses on `visibility`, and reports a trap as an `error` message.
- `client/picture.ts`: `Picture` (`start`, `draw`, `counters`, `resize`, `stop`), `PictureInfo` (`backend`, `adapter`,
  `timestampQuery`), `PictureFrame` (the two newest blocks and the time; events join in change-set 8),
  `RenderCounters`.
- `client/render/three-picture.ts`: `createThreePicture()`: `WebGPURenderer`, `await renderer.init()` before the first
  frame (ADR 0003 part 3; the Renderer docs), the backend read from `renderer.backend.isWebGPUBackend`, the adapter's
  info from `GPUDevice.adapterInfo` where it exists, `renderer.hasFeature('timestamp-query')`, `renderer.info`, the loop
  on `setAnimationLoop`.
- `client/render/empty-scene.ts`: `buildEmptyScene()`: ground, a 1 m grid, a sun and a fill light, node materials only
  (ADR 0003 part 1: no `ShaderMaterial`), placeholder colours.
- `client/app/boot.ts` (`boot`), `app/worker-link.ts` (`WorkerLink`: the newest and previous block, the layout-hash check
  on the first block of every Worker start (change-set 1's follow-up), returned buffers riding the next frame's input
  post, one message per frame), `app/error-hook.ts` (`installErrorHook`), `app/flags.ts` (`readFlags`: `devtools` from
  `import.meta.env.VITE_SHAMBLE_DEVTOOLS`, `forceWebGL`, `padBytes`, `runLabel`).
- `client/overlay/overlay.ts` (`Overlay`, written through `textContent` only, at most 4 times a second) and
  `overlay/report.ts` (`formatReport`, `copyReport`, with a read-only text box if the clipboard is refused).
- `client/harness/frame-times.ts` (`FrameTimes`) and `harness/run-header.ts` (`runHeader`: machine, browser, display
  and commit for every result row, PM-0002-1).
- `client/serve.mjs`: `createServer({ root, port })` and a `main()` that runs only when the file is executed.
- `client/scripts/build.mjs`: `lockedWasmBindgenVersion(lockText)`, `checkCli(version, locked)`, `sizeTable(files)`,
  `main()`.

## 6. New libraries
Versions were read from the registries on 2026-11-18 (section 19). The exact set of new lockfile entries comes from the
lockfile diff at step 2 and is exactly what security-auditor answers on. The lists below are what the registries
declare.

| Library | Exact version | Where, and what kind | Why it is needed | What it pulls in | VISION_V2 §6 trace |
|---|---|---|---|---|---|
| `wasm-bindgen` | `=0.2.130` (2026-10-30, Rust 1.81 or newer, MIT OR Apache-2.0) | `sim-wasm` only, normal dependency; the version set once in the root `[workspace.dependencies]` | the control calls the Worker makes are wasm-bindgen's own, with its `.d.ts` (ADR 0002 part 5, option D1); `sim-wasm` is "the wasm-bindgen bindings the Worker loads, and nothing else" (part 3) | declared: `cfg-if`, `wasm-bindgen-macro` and `wasm-bindgen-shared` (both `=0.2.130`), `rustversion` (build); the macro pulls in `wasm-bindgen-macro-support`, which pulls in `bumpalo`, `proc-macro2`, `quote` and `syn` 3 with `visit`, `visit-mut`, `full` and `extra-traits`; `wasm-bindgen-shared` pulls in `unicode-ident`. Already in `Cargo.lock` from ts-rs: `proc-macro2`, `quote`, `syn` 3, `unicode-ident` | §6.2 and §6.9, `sim-wasm`'s row in ADR 0002 part 3 |
| the wasm-bindgen command-line tool | 0.2.130, the release's `wasm-bindgen-0.2.130-x86_64-unknown-linux-musl.tar.gz` | a tool, in no manifest: under `.gate/tools/wasm-bindgen-0.2.130/` here, and in CI after CI-WASM | writes the glue and the `.d.ts` from the `.wasm`; it must equal the library in `Cargo.lock` (PM-0002-18; ADR 0002 part 2) | one binary; the release also publishes a `.sha256sum` file and a release attestation | the same as the library |
| `three` | 0.187.0 (r187, three.js's newest release, 30 October) | `packages/client`, dependency | the renderer the stack settles: one `WebGPURenderer` with its WebGL 2 backend (ADR 0003 parts 1-4) | nothing: no dependencies and no install scripts in its registry entry (MIT, 21,030,536 bytes unpacked) | §6.9 (ADR 0003 part 1's need) |
| `@types/three` | 0.187.0 | `packages/client`, dev dependency | three.js ships no types; the page is type-checked against them (ADR 0002 part 7), and PM-0002-30 and R5-TSLINT-OQ4 ask how many errors and casts that costs | six packages, by its registry entry: `fflate` ~0.8.3, `@types/webxr` >=0.5.17, `meshoptimizer` ~1.1.1, `@types/stats.js` *, `@tweenjs/tween.js` ~23.1.3, `@dimforge/rapier3d-compat` ~0.12.0 (a physics library, 7,522,790 bytes unpacked, no dependencies, no install scripts). Used for types only: nothing of them is bundled, because nothing imports them | the same as `three` |
| `vite` | 8.3.7 | root, dev dependency | the build and dev server of the `client` entry (ADR 0002 parts 2 and 5; GLOSSARY §8, "TypeScript with Vite") | nothing new: `vite` 8.3.7 is already in `package-lock.json` as Vitest's peer, with `rolldown` 1.2.18, `lightningcss` 1.33.0 and `postcss` 8.5.32. Only the direct declaration is new. The registry's newest is 8.3.8: that bump is its own change-set, as ADR 0002 part 2 asks of every toolchain bump | GLOSSARY §8, the foundation |

**Not added, with the reason:**
- **A random-number crate:** see RNG-CRATE. Nothing in change-set 2 draws a random number.
- **wasm-bindgen-test:** the boundary layer runs the built Wasm in Vitest (ADR 0002 part 8). Unit vectors in Wasm
  start with change-set 3's maths functions and streams.
- **wasm-opt (binaryen):** sizes are recorded without it. With it, the build-option table is change-set 3's (Part 3.3,
  PM-0002-29). R5-CI-OQ4 stays open.
- **`@types/node`:** the files Node runs are plain `.mjs` (`serve.mjs`, `build.mjs`, `vite.config.mjs`), so nothing that
  is type-checked needs Node's types. The small server is guarded by its tests and by security-auditor's review
  instead.
- **Vite plugins, `js-sys`, `web-sys`, `trybuild`, `jsdom`, any server library:** none is needed. Vite handles module
  Workers and `new URL(…, import.meta.url)` assets itself. The control calls take only numbers. The compile-time name
  check is shown failing once by hand, not by a compile-fail harness. The page's unit tests touch no DOM. The small
  server uses Node's own modules.
- **A Playwright browser download:** the container's Chromium is used (risk 2). CI's browser jobs come after change-set
  3 (build plan Part 7, :977-979).

## 7. The dependency rule, as this change-set applies it
The rule that binds the build: a new or bumped dependency's manifest and lockfile edits stay uncommitted until
security-auditor has answered on exactly that content; nothing builds a dependency graph that holds them before that
answer; every cargo command takes `--locked`; the checkpoint script holds a push that changes `Cargo.toml`,
`Cargo.lock`, `package.json`, `package-lock.json` or `npm-shrinkwrap.json` until
`.checkpoints/<slug>/dependency-check.md` records the check (`.claude/CHECKPOINTS.md` §3, :60-69, from #60).

1. **One dependency step, one answer.** Every manifest edit of the change-set is made at once, in step 2: the
   dependencies and the `exports`, `test` and workspace-dependency lines alike, because "a manifest edit with no
   dependency change still takes a security answer" (the advisor agent on #60's plan,
   `kit-hold-ci/advisor-plan.md`). A manifest edit found later takes its own, smaller answer.
2. **Prepared off the main folder.** The edits are made in a scratch worktree, `git worktree add .gate/cs2-deps HEAD`,
   so the main folder keeps building step 1's code during the wait. Nothing is built in that worktree.
3. **Resolved, not built.** Rust: `cargo update --workspace --dry-run`, then `cargo update --workspace`. Per cargo's
   documentation, `--workspace` updates only workspace packages, other packages "only if they don't already exist in the
   lockfile", and it is "useful for updating Cargo.lock after you've changed version numbers in Cargo.toml". These are
   the change-set's only cargo commands without `--locked`, and they compile nothing. npm: `npm install
   --package-lock-only --ignore-scripts --save-exact` for the new entries, which per npm's documentation uses "only the
   package-lock.json, ignoring node_modules".
4. **What security-auditor answers on:** the text diff of the five manifests and the two lockfiles (saved as a `.diff`
   file in the notes, never as files named like manifests), the sha256 of each of those seven files, and the tool's
   tarball: its URL, its sha256, the release's published `.sha256sum` and its release attestation. The tool's check is
   recorded in a notes file of its own (`.checkpoints/cs2-first-page/wasm-bindgen-cli-check.md`), because
   `dependency-check.md` takes only manifest and lockfile lines.
5. **After the answer:** the same diff is applied in the main folder (sha256 compared with the reviewed files),
   `dependency-check.md` is written (one `<sha256>  <path>` line per file, naming the answer's notes file), the files are
   committed by path, and the checkpoint script pushes them, which releases its own hold. Only then: `npm ci
   --ignore-scripts`, the tool's download and unpacking (checked against the recorded sha256), and the first build that
   holds the new crates. No push is made by hand: "a plain `git push` is not held".
6. **No shared cargo target folder.** The planted-defect worktrees build into folders of their own
   (`CARGO_TARGET_DIR=.gate/cs2-plant-target`). Change-set 1's export tests also stop when they run outside the copy they
   were built in.
7. **The netcode proof test's three rules,** checked over this change-set's own files: no root `.cargo/config.toml`
   (none is needed: wasm-bindgen asks for no compiler flags); no `"type"` in the root `package.json`; no tracked file
   that is not Markdown names the spike's folder.

## 8. Build steps, in order
Each step ends with the checks of section 9 that apply to it, all exit 0, then a commit by path and a checkpoint. A
step that changes no manifest pushes normally; step 2's push goes through the hold.

0. **Paper.** The preconditions of section 3. OQ-0002-7's by-day reading: wasm-bindgen's maintenance issue #4533 is
   read (section 19); the guide's governance page is still to read. Its decider "comes before the step that adds
   wasm-bindgen" (EMPTY-SLOTS, STATUS_ARCHIVE :850-856), and the result goes into the report (build plan Part 3.9's
   rule). His answers to RNG-CRATE and OVERFLOW-3P.
1. **The Rust core, with no new crate.** `sim`: the world, the authority step, the canonical bytes and the state hash.
   `client-endpoint`: `Section::align()`, the lookups by name, the block writer and the local schedule. Tests: the hash
   against RFC 9923's FNV-1a 64-bit test values; the state bytes at fixed widths, with no `usize` and with a stated
   length; the same seed giving the same hashes, a different seed a different hash; the header's bytes at the table's
   offsets; the padding left zero; the lookups; `align`; the schedule without drift over 1,000,000 ticks, and `rebase`.
   No manifest changes, so this step pushes normally.
2. **The one dependency step** (section 7). The planned manifest text is below this list. The build waits for the
   answer. Step 1's review notes can be read in the meantime, and nothing else is built.
3. **The bindings and the build script.** `sim-wasm`'s `Sim`. `build.mjs` reads wasm-bindgen's version from
   `Cargo.lock`, refuses any other tool version before it generates bindings (PM-0002-18), runs `cargo build --locked
   --release --target wasm32-unknown-unknown -p sim-wasm`, then `wasm-bindgen --target web --out-dir
   packages/sim-worker/pkg`, records the sizes (raw, gzip at level 9 and Brotli at level 11, from Node's own `zlib`),
   runs Vite's build through its JavaScript API with `VITE_SHAMBLE_COMMIT` and, unless `--production`,
   `VITE_SHAMBLE_DEVTOOLS=1`, copies `serve.mjs` into `dist/`, and fails if any served file reaches 26,214,400 bytes
   (PM-0002-29's gated clause). It runs commands as argument arrays (`execFileSync`), never through a shell string.
   **Risk 1 is settled here:** whether the workspace's `unsafe_code = "deny"` accepts `#[wasm_bindgen]`'s expansion.
4. **The Worker.** `sim-worker`'s seven modules. Unit tests: the pacer on a fake clock (7 per wake, sleep to the
   returned due time, pause, rebase, the counters), the pool, the stats. Boundary tests on the built Wasm: a block
   written by Rust passes `checkLayoutHash` (PM-0002-16's page half, which change-set 1 left to this writer); after
   `memory.grow` between two steps, the next block is byte for byte the block of a run without the grow (PM-0002-6); two
   steps advance the tick by two; mode 1 and a 31-byte seed are refused; per tick the host makes exactly two calls,
   `input` and `step`, and a reading of the generated glue shows each call reaching exactly one export (PM-0002-3 is
   counted in full in change-sets 3 and 5).
5. **The page.** The app shell, `picture.ts`, `render/`, the readout and the copy report, the harness's CPU side,
   `index.html` (with no inline script or style: ADR 0002 part 7, CSP-ready), `styles.css`, and `vite.config.mjs`:
   `base: './'`, `build.assetsInlineLimit: 0` (so the Wasm never becomes a `data:` URL), `worker.format: 'es'` (a module
   Worker; Vite's default is `'iife'`), `server.host` and `preview.host` `127.0.0.1`, `server.fs.allow` narrowed to what
   the page loads (Vite's default serves the whole workspace root, which here is the repository), and COOP and COEP
   headers on the dev and preview servers only when `SHAMBLE_ISOLATION=1` (the isolation flag, PM-0002-20). Unit
   tests: the report's text, the frame-time statistics, the flags, the Worker link against a fake Worker (the newest
   two blocks kept, the older ones returned on the next frame's post, the layout check refusing a block of another
   layout).
6. **The small server.** `serve.mjs` with Node's own modules only, running on Node 18 or newer (Part 8). It binds
   `127.0.0.1` only and prints `http://127.0.0.1:4173`. It answers `GET` and `HEAD` only, serves files inside its own
   folder only (paths decoded once and resolved, real paths checked against the folder, so a `..`, an encoded `..`, a
   backslash or a link that leads outside is refused), sends a fixed table of content types (`application/wasm` for
   `.wasm`), `X-Content-Type-Options: nosniff` and `Cache-Control: no-store`, refuses a `Host` header that is not its
   own address (against DNS rebinding), and stops with a clear message if the port is taken. Its tests cover each of
   these. It is never bundled into the page.
7. **The browser tests.** `playwright.config.ts`: project `node` (`tests/`, no browser; this is what `npm test` runs)
   and project `chromium` (`browser/`, run by path), the small server as `webServer` on the built `dist/`, retries 0,
   `forbidOnly` in CI, the browser's path from `SHAMBLE_CHROMIUM` when set. Specs:
   - `skeleton`: the backend named in the readout and in every screenshot, on the default run and on `forceWebGL`; zero
     Wasm calls and instantiations by the page (an init script replaces the page's `WebAssembly` entry points with stubs
     that count and throw; the Worker, which init scripts do not reach, still runs); blocks equal ticks within 1; one
     message per tick; no sent buffer left undetached; errors 0; the render counts equal to the committed baseline; no
     request leaves `127.0.0.1` (any other request fails the run, as G12 will);
   - `isolation`: `crossOriginIsolated` on the page and in the Worker under Vite's dev and preview servers, flag on and
     off (Chromium's 4 of the 12 cases);
   - `clock`: does Playwright's clock reach the Worker (R5-TEST-OQ1)? Recorded yes or no, not judged;
   - `allocation`: the method of PM-0002-7: a sampling heap profile through the Chrome DevTools Protocol, attributed to
     the boundary's page functions;
   - `file-url`: what Chromium does with the built `index.html` opened from `file://` (OQ-0002-1, first result).
   The visibility check drives the page's own handler (hidden, then shown) and asserts that the tick counter does not
   move while hidden and that no wake runs more than 7 ticks. A real hidden tab is tested in Brave (section 13).
8. **Readings, planted defects, docs, the zip, reviews.** The readings and planted defects of section 9. The production
   flavour is built once and shown to hold none of the development-only code (Part 0: each development tool "is kept
   out of the production build"). Then the report, docs-keeper's pass, his words by script, ADR 0002's pointer for
   SPLIT-25, and the zip, `shamble-v2-02-first-page.zip` (the built `dist/` with `serve.mjs`, a `README.txt` with the
   steps and the commit, and `SHA256SUMS.txt`), made with `python3 -I -m zipfile -c` from `.gate/`, never committed,
   sent through the chat's file tool. Then the reviews (section 12), the verifier until PASS, and the advisor agent on
   the final head.

**Planned manifest text for step 2** (the exact reviewed content may differ only by the lockfiles' resolution):
```toml
# Cargo.toml (root), [workspace.dependencies], after the ts-rs line:
# The bindings the Worker loads (ADR 0002 parts 2 and 5), in sim-wasm only. The CLI that writes the glue must equal
# this version (PM-0002-18); each bump is its own change-set.
wasm-bindgen = "=0.2.130"
```
```toml
# crates/sim-wasm/Cargo.toml, after [package]:
# A cdylib for the .wasm file the Worker loads, an rlib for the native tests (WASM-ARTIFACT).
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
client-endpoint = { path = "../client-endpoint" }
sim = { path = "../sim" }
wasm-bindgen = { workspace = true }
```
```json
// package.json (root), devDependencies:
"@playwright/test": "1.65.0", "typescript": "6.0.3", "vite": "8.3.7", "vitest": "5.1.0"
// packages/client/package.json:
"dependencies": { "@shamble/sim-worker": "0.0.0", "@shamble/types": "0.0.0", "three": "0.187.0" },
"devDependencies": { "@types/three": "0.187.0" }
// packages/sim-worker/package.json:
"exports": { ".": "./src/index.ts", "./worker": "./src/worker.ts" },
"dependencies": { "@shamble/types": "0.0.0" },
"scripts": { "test": "vitest run --dir src" }
// packages/types/package.json:
"exports": { ".": "./src/index.ts" }
// packages/e2e/package.json:
"scripts": { "test": "playwright test --project=node" }
```
Under RNG-CRATE (a) or (b), `crates/sim/Cargo.toml` and the root `[workspace.dependencies]` gain the generator's
line, with `default-features = false`. Under OVERFLOW-3P (b) or (c), the root `[profile.release.package…]` lines
change. Both happen inside this same step, before the answer.

## 9. Evidence, by kind (CLAUDE.md step 4), with the exact commands
Tools run by their path: the session's settings deny `npx`, `npm run` and `npm test` (build plan Part 7, :925).

**Rust (every step that touches it):**
```
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo clippy --workspace --all-targets --locked --target wasm32-unknown-unknown -- -D warnings
cargo test --workspace --locked
cargo test --release -p sim -p protocol --locked
cargo build --locked --target wasm32-unknown-unknown -p sim -p protocol -p room -p client-endpoint -p sim-wasm
SHAMBLE_WRITE_TYPES=1 cargo test --workspace --locked
git status --porcelain --untracked-files=all -- packages/types/src/generated      # prints nothing: no generated file changes
```
**The build (from step 3):**
```
SHAMBLE_WASM_BINDGEN="$PWD/.gate/tools/wasm-bindgen-0.2.130/wasm-bindgen" node packages/client/scripts/build.mjs
SHAMBLE_WASM_BINDGEN="$PWD/.gate/tools/wasm-bindgen-0.2.130/wasm-bindgen" node packages/client/scripts/build.mjs --production --out-dir .gate/cs2-first-page/dist-prod
```
**TypeScript** (the strict flags of CI-TYPES, `toolchain.yml` :93, with each package's own `lib` and `types`, as ADR 0002
part 7 asks; no tsconfig is committed, since type settings are the 2.3 chat's; TS5112 is why `--ignoreConfig` is
there):
```
node node_modules/typescript/bin/tsc --ignoreConfig <CI-TYPES flags> <packages/types shipped files>
node node_modules/typescript/bin/tsc --ignoreConfig <flags without --lib> --lib es2023,webworker --types vite/client <sim-worker src> packages/sim-worker/pkg/sim_wasm.d.ts
node node_modules/typescript/bin/tsc --ignoreConfig <flags without --lib> --lib es2023,dom,dom.iterable --types vite/client <client src>
```
Each again with the tests added and `--skipLibCheck`. The generated `sim_wasm.d.ts` is checked with `skipLibCheck`
off, which answers R5-TSLINT-OQ3's wasm-bindgen half (change-set 1 answered ts-rs's and ours).

**Unit and boundary tests** (Vitest by its bin path, read from its `package.json` at build time):
```
node node_modules/vitest/vitest.mjs run --root packages/types
node node_modules/vitest/vitest.mjs run --root packages/sim-worker --dir src
node node_modules/vitest/vitest.mjs run --root packages/sim-worker --dir boundary     # after the build
node node_modules/vitest/vitest.mjs run --root packages/client
```
**The browser:**
```
SHAMBLE_CHROMIUM=<the container's Chromium under /opt/pw-browsers/chromium-1243> node node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=chromium
node node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=node
```
**The rest:**
```
node tools/gate.mjs --no-smoke              # smoke cannot start here (Chromium 1194 wanted, 1243 installed: cs1's report); CI runs it
node .claude/scripts/repo-map-check.mjs
test ! -e .cargo/config.toml
node -e "process.exit('type' in JSON.parse(require('fs').readFileSync('package.json','utf8')) ? 1 : 0)"
git grep -n -I -e 'spikes/' -- $(git diff --name-only origin/main...HEAD -- . ':!*.md')    # prints nothing
```
CI: `gate`, `smoke` and `toolchain` green on the head, for the push and for the pull request.

**Planted defects**, each in a throwaway worktree with its own target folder, never committed, removed without force;
each must fail the named check, and its control must pass:
1. the page calls `WebAssembly.instantiate` (a debug readout: PM-0002-2's own bite) → `skeleton` counts 1 and fails;
2. a block posted without its transfer list (PM-0002-6) → the Worker's undetached count is above 0, and `skeleton`
   fails;
3. a host that caches its view of `memory.buffer` (PM-0002-6) → the `memory.grow` boundary test fails;
4. a header field added in Rust without regenerating (PM-0002-16) → `generated_layout_is_current` fails, and the
   boundary test's `checkLayoutHash` refuses the Rust-written block as `layout-hash`;
5. a tool one patch off: a stand-in printing `wasm-bindgen 0.2.129` (PM-0002-18) → the build script exits 1 before it
   writes any binding; control: 0.2.130 passes;
6. a misspelt field name in the writer's lookup → the build fails at compile time;
7. a 64-byte typed array allocated per frame in the page's block handler (PM-0002-7's bite) → `allocation` reports it;
8. a Worker that runs the hidden gap instead of rebasing (PM-0004-70's bite) → the visibility check sees the counter
   move and more than 7 ticks in one wake;
9. a `serve.mjs` that lets an encoded `..` through → its test fails.

**Readings** (reported, never judged; the machines are named before change-set 6, MACHINES (a)):
- the hand-off as a curve over block sizes (8 bytes, the header alone; 1, 4, 16 and 64 KiB): the copy time in the
  Worker (p50, p99) and the delivery from the Worker's finished step to the page's bound views on a shared clock
  (`performance.timeOrigin + performance.now()`; PM-0002-8's method, p95), in the container's Chromium and in Brave from
  his reports;
- the Worker's pacing: tick start jitter p99 against its own schedule, the most ticks in one wake, rebases (PM-0002-26,
  PM-0004-68);
- the Wasm file's size, raw, gzip and Brotli, after wasm-bindgen, without wasm-opt, on the default release profile
  (PM-0002-29; R1-RR-OQ5), and every served file's size;
- the type errors and casts the page needs against `@types/three` 0.187.0 under the strict flags, and `tsc` time with
  `skipLibCheck` on and off, median of 3, warm (PM-0002-30; R5-TSLINT-OQ4 and OQ6). These use `--ignoreConfig`, not
  `tsc -b`, because the 2.3 chat's configs are not on main;
- which backend headless Chromium gets here, and one try with SwiftShader's WebGPU adapter turned on by flags (ADR 0002
  part 9 proposes that try; it is not researched), recorded either way;
- from every report: `hardwareConcurrency` (R1-BND-OQ3), `renderer.hasFeature('timestamp-query')` (R3-MEASURE-OQ1,
  R3-WGPU-OQ2), `renderer.info` (R3-MEASURE-OQ3), the adapter's info (R3-WGPU-OQ1).

**Visual:** screenshots of the scene on each backend the container gives, each naming its backend, and Mustafa's Brave
check as the final word. **Performance:** readings only, with their method, as above. **Netcode:** none in this
change-set; L1 and the loopback are change-set 3's. **Security:** the dependency answer (section 7), the small server's
tests, the readout's `textContent`, the dev server's binding and file allow-list, the build script's argument arrays.

## 10. Risks
1. **`unsafe_code = "deny"` against `#[wasm_bindgen]`'s expansion.** The lint names `export_name`, `link_section` and
   `unsafe` blocks (rustc's lint list), which the macro emits. No source read here says whether rustc reports the lint
   inside an external proc macro's expansion. Step 3's first build settles it. Fallback: `#[allow(unsafe_code)]` on the
   one bindings module, with its written reason and a test, which ADR 0002 part 4 allows. Step 2.2's WASM-ARTIFACT
   follow-up already expected "its `unsafe_code` exception" to "come with the bindings".
2. **The container's Chromium is not Playwright 1.65.0's build** (1243 installed, 1254 expected; Chrome for Testing 153
   also present). Playwright's docs: "Playwright only works with the bundled Chromium, Firefox or WebKit, use at your
   own risk." If neither browser launches, there is no browser evidence here. Stop and tell: no browser is downloaded
   without a security answer.
3. **Headless Chromium here probably gets no WebGPU adapter** (ADR 0002 part 9's reading of R3), so both browser runs
   may be WebGL 2, and WebGPU is then seen only in Brave on his PC. The report says which backend each run got.
4. **Brave may hide the adapter's details** (secondary sources say Brave 1.93 does, by default; not an official source)
   **or not expose WebGPU** (R3-WGPU-OQ1 is open). The label says what runs, and an empty adapter line is normal.
5. **The dependency answer may refuse something,** for example one of `@types/three`'s six packages. Fallback, his to
   choose then: a small local declaration file for the handful of three.js names change-set 2 uses, which loses
   PM-0002-30's reading until the real types return.
6. **The zip route is proven for a file, not yet for a zip** (section 3, item 6).
7. **wasm-bindgen broke compatibility in 0.2.123 and 0.2.128** (OQ-0002-7's text, ADR 0002 :1086-1088). The exact pin
   and the tool check hold it. Each bump is its own change-set.
8. **CI runs none of the Wasm build, the boundary tests or the new packages' type check until CI-WASM's edit merges,**
   and no browser until after change-set 3. Until then those marks are "shown", not "met". Adding the build to the
   `toolchain` job may also press on its 15-minute limit: measured on the CI-WASM run.
9. **three.js moves on.** r187 is the newest today and equals the only version the Shamble VFX runtime accepts (Part
   15.5), so nothing conflicts now. ADR 0003 part 2 pins "the newest when the look proof test starts", which is change-set
   6. If r188 is out by then, Part 15.5's question applies there.
10. **The size** (SIZE-2). The change-set stops and tells him in three lines when it passes the stop line he picks.
11. **Every timed number is a reading** until the machines are named (MACHINES (a)).
12. **The dev server and the isolation flag:** Vite serves the workspace root by default, which here is the whole
    repository, and COEP can block a resource the page needs. The allow-list is narrowed and the four Chromium cases run
    in step 7.

## 11. Change type and risk tier
Logic (the simulation's world, the writer, the schedule, the pacer, the pool), visual (the scene and the readout),
performance (readings), security (a new local server, new dependencies, page text). **Risk tier 1:** game code, pages,
tests; no kit, gate or CI file. It is `needs-mustafa` in any case: there is a Brave check, and `crates/` and `packages/`
lie outside the places the PR Advisor may merge (build plan Part 0). Under CI-WASM (b) the pull request becomes risk
tier 2.

## 12. Reviewers
The build plan's list (Part 1, :313-314), each at effort max, at most three at a time; the six together are one round
(CLAUDE.md's review budget):
- engine-architect (module boundaries, the interface file, the folder cut, the Worker contract), graphics-engineer (the
  renderer's set-up, the backend report, the render counts), rust-reviewer (the world, the writer, the bindings, the
  `unsafe_code` outcome, the Wasm build);
- then perf-auditor (the frame and tick paths, the pool, allocation, the readings' method), security-auditor (the new
  dependencies, the page's text, the small server, the dev server's settings, the build script), test-engineer (is each
  claim proven; the planted defects; the browser tests).
Outside the round: security-auditor's dependency answer at step 2 (a condition of building, not a review), the advisor
agent's plan review, the verifier until PASS, and the advisor agent on the final head. Models: lane 1's main model is
Fable 5.2 (his decision of 2026-11-17); every agent is Opus 5.5.

## 13. What Mustafa checks in Brave (about 15 minutes, and 6 more for step 7)
Once, if not done: in PowerShell, `node --version` prints 18 or newer (Part 8; his Windows run of 2026-11-11 used
Node 24).
1. Download `shamble-v2-02-first-page.zip` from the chat; the message names the commit it was built from. Unzip it into
   a folder of its own, for example `C:\shamble\02-first-page`.
2. Double-click `index.html` in that folder. Expected: Brave opens it from a file. You see either a line saying to run
   `node serve.mjs`, or the page itself. Note which, and any message shown. This is the first answer, for Brave, to
   whether a built page opens from a file.
3. In that folder, click the address bar, type `powershell` and press Enter. Type `node serve.mjs` and press Enter.
   Expected: it prints `http://127.0.0.1:4173`.
4. Open that address in Brave, with Shields at your usual setting (note the setting). Expected: an empty ground with a
   grid of 1 m squares, lit from above, in plain placeholder colours (not the look). A corner readout names "WebGPU" or
   "WebGL 2 fallback", ticks count up at about 30 a second, the frame rate shows, blocks equal ticks, errors show 0, and
   the commit equals the one in the chat.
5. Leave it for 2 minutes. Expected: errors still 0, blocks still equal to ticks (within 1), the tick rate still about
   30.
6. Switch to another tab for 1 minute, then come back. Expected: the tick counter shows the number it had when you
   left, then counts on from there, with no jump.
7. The same with 6 minutes away. Expected: the same as in step 6.
8. Press "Copy report" and paste the result into the chat.
9. Open `http://127.0.0.1:4173/?backend=webgl`. Expected: the readout says "WebGL 2 fallback", with the same scene and 0
   errors. Press "Copy report" again and paste that too.
10. Open `brave://version` in a new tab and paste its first line.
11. Back in PowerShell, press Ctrl+C to stop the server.

## 14. Planned size
Hand-written lines added (Rust, TypeScript, configuration and tests; generated files, lockfiles, fixtures and docs left
out), by `git diff --numstat` against the change-set's base (build plan Part 0). This plan's own count, part by part,
checked against change-set 1's measure (940 lines against 600 planned, about 620 of them tests):

| Part | Code | Tests | Lines |
|---|---|---|---|
| `sim`: world and state hash | 90 | 130 | 220 |
| `client-endpoint`: lookups by name, `align`, writer, schedule | 140 | 190 | 330 |
| `sim-wasm`: bindings | 70 | 40 | 110 |
| `sim-worker`: host, pacer, pool, stats, messages; unit and boundary tests | 300 | 290 | 590 |
| `client`: app shell, interface, render, readout, harness, page and style | 560 | 200 | 760 |
| `client`: the small server | 120 | 150 | 270 |
| `client`: the build script and Vite's configuration | 190 | 50 | 240 |
| `e2e`: configuration and five specs | 60 | 290 | 350 |
| manifests and `.gitignore` | 40 | 0 | 40 |
| **Total** | **1,570** | **1,340** | **about 2,900** |

The build plan's row says 2,000. Its own estimates are "good to about half as much again either way" (:198-200). Which
stop line holds is SIZE-2. Time: the build plan's "about 13 h", scaled by the line count, is about 19 h. That is
arithmetic on the plan's estimate, not a measurement.

## 15. Not in it
- `protocol`'s codec, the loopback, `room`, L1, the replay runner, the panic hook's buffer, the first recording, the
  maths and stream vectors, the scan test in `sim`, the two-instance test: change-set 3.
- Keyboard and mouse into the newest input, and the key diagram (change-set 3). The input post exists now, with its
  sequence number and view time.
- The random-number generator, unless RNG-CRATE says (a) or (b).
- The `content` crate, soldiers, the camera and its panel, the test scene: change-set 4. The bench mode and the walkers:
  change-set 5.
- The look, the GPU timing, warm-up and quality tiers: change-set 6, lane 2.
- wasm-opt and the build-option table (change-set 3); wasm-bindgen-test.
- Every CI edit (CI-WASM: its own pull request); Firefox, WebKit and Safari runs; any timed mark judged.
- Previews and the deploy workflows (step 2.6, lane 2); the editor entry (Slice 5); `packages/net` (Slice 3); lint,
  format and type settings files (the 2.3 chat's); anything in `legacy/`, `tools/` or the kit.

## 16. Open questions for Mustafa
**RNG-CRATE: which library gives the simulation its random streams, and in which change-set does it enter?**
- Why it matters: whatever enters is pinned for good, since every recording's hashes rest on it, and the crate ADR
  0004 names is being dropped by its own makers.
- What was found: ADR 0004 part 8 names `rand_chacha`'s `ChaCha8Rng`. rand 0.10.0's changelog says "The dependency on
  `rand_chacha` has been replaced with a dependency on `chacha20`". The rand project removed `rand_chacha` from its
  repository on 2026-01-23 (pull request #1658), and its maintainer wrote: "I no longer see a reason to maintain
  `rand_chacha`."
- Options: (a) `rand_chacha` 0.10.0 in change-set 2, as ADR 0004 and the build plan say: a pinned, frozen copy whose
  tree adds `rand_core`, `ppv-lite86` and `zerocopy`; (b) `chacha20` 0.10.2's `ChaCha8Rng` (feature `rng`) in change-set
  2: the RustCrypto crate that rand 0.10 itself uses, with the same algorithm and the same calls for the seed, the
  stream and the position, and a smaller tree (`cfg-if`, `rand_core`, and `cpufeatures` on x86 only). This changes ADR
  0004's crate name, recorded by a dated pointer at the drafts' next revision; (c) no generator in change-set 2:
  nothing there draws a random number, so the change-set carries only the seed path (the Worker draws 32 bytes, the world
  keeps them, the state hash covers them). The generator enters in change-set 3 beside its 10,000-output test vectors
  (PM-0004-27), with (a) or (b) answered by then.
- Recommendation: (c), with (b) for change-set 3. A generator nothing calls is code "for later", the proof of the
  streams is change-set 3's anyway, and (b) is the crate that is maintained.

**OVERFLOW-3P: do the first third-party crates in a build players run get overflow checks?**
- Why it matters: your register row OVERFLOW-ALL keeps every dependency at Rust's release default (no overflow checks)
  through the root profile's `"*"` line. Its follow-up says "the first third-party crate that any workspace crate uses
  decides" between a named override per crate and dropping `"*"` (OVERFLOW-PER-CRATE, STATUS_ARCHIVE :817-831).
  Change-set 2 puts wasm-bindgen into the player's Wasm.
- Options: (a) keep `"*"` as it is. wasm-bindgen's runtime code does none of the simulation's arithmetic, and the
  player's Wasm stays at the release profile's optimisation level, where the reuse path the follow-up names (shared
  generic code at opt-level 0, 1, "s" or "z") stays closed. The question comes back when an arithmetic or random-number
  crate enters, or when the shipped build moves to "s" or "z" (change-set 3's size table may propose that); (b) a named
  override for each such crate, measured for size and step time; (c) drop `"*"`, so that every dependency is checked,
  with security-auditor reading it, measured the same way.
- Recommendation: (a).

**CI-WASM: how does CI start building and testing the new parts (the Wasm bindings, the Worker's tests on the real
Wasm, the type check of the two new packages)?**
- Why it matters: until a CI edit, CI runs none of these, and they are checked by hand, as with change-set 1. You asked
  that every CI edit be put to you with its exact text (GATE-FOLLOW (b)).
- Options: (a) a small pull request of its own after change-set 2 merges, as #60 followed #58, with the exact text in
  change-set 2's report (draft in section 18) and the tool downloaded from wasm-bindgen's own release, checked against
  a sha256 written in the workflow; (b) the same edit inside change-set 2's pull request, which makes it risk tier 2;
  (c) as (a), but with the tool installed by the third-party action ADR 0002 part 2 names (`taiki-e/install-action`,
  pinned by SHA). That puts one more JavaScript action into the job that runs the network cut, which part 2 of your
  CI-CUT answer moves away from.
- Recommendation: (a).

**SIZE-2: which stop line holds for change-set 2?**
- Why it matters: your rule stops a change-set at twice its planned size. The build plan planned 2,000 lines; this
  plan's own count is about 2,900, most of the difference tests, as in change-set 1 (1.57 times its plan).
- Options: (a) the stop line stays at 4,000, twice the build plan's number, with about 2,900 written down as this
  plan's estimate; (b) the plan becomes 2,900 and the stop line 5,800; (c) cut scope toward 2,000 by moving the isolation
  flag, the allocation method, the clock fact and the hidden-tab pause to change-set 3, which saves about 300 lines.
- Recommendation: (a). Your control stays where the build plan put it.

## 17. Where the build plan is wrong or out of date for change-set 2
1. **OQ-0002-7 is placed nowhere** (Part 3.9, :762-777). Its decider, "a by-day reading of its maintenance issue
   (#4533) and the guide's governance page" (ADR 0002 :1085-1093), "comes before the step that adds wasm-bindgen"
   (EMPTY-SLOTS), and `sim-wasm`'s stub says its bindings wait for it (`crates/sim-wasm/src/lib.rs:1-3`). Step 0 does
   the reading.
2. **ADR 0004's generator crate is leaving its project** (RNG-CRATE). The card's "the seeded random streams" inherits
   that.
3. **"A first reading of the hand-off with the block filled to the size of 12 and of 48 soldiers"** (:310-311): no
   source gives a soldier row's size before change-set 4 (the only figures found are ADR 0001 :313's 90-bit and 60-bit
   wire records, which are compressed). The reading becomes a curve over block sizes, and change-sets 4 and 5 read the
   real rows.
4. **Part 8's "It prints an address, `http://localhost:4173`"** (:1008): the server binds `127.0.0.1` and prints that
   address. Both are secure contexts by the W3C's Secure Contexts rules, and Vite's own docs warn that "browsers may use
   a different resolved address" for `localhost`.
5. **Part 7's "Playwright on the built client with the container's Chromium"** (:941): the container's Chromium (build
   1243) is not the one Playwright 1.65.0 expects (1254), so runs are "at your own risk" (risk 2).
6. **Nothing says how the wasm-bindgen tool reaches the container or CI.** None is installed here. It is code from the
   internet, so it goes through the security answer. ADR 0002 part 2's action route and CI-CUT's direction disagree
   (CI-WASM).
7. **Vite is not new:** 8.3.7 is in the lockfile as Vitest's peer. Only the direct declaration is new.
8. **Two step 2.2 follow-ups bind this card without being in it:** WASM-ARTIFACT (the crate type; the expected
   `unsafe_code` exception) and OVERFLOW-PER-CRATE (OVERFLOW-3P).
9. **The planned size of 2,000 looks low** by change-set 1's measure (SIZE-2).
10. **Part 7's "Wasm tests" row puts "the unit vectors in Wasm" at change-set 2** (:939), but Part 3.4 places the maths and
    stream vectors at change-set 3 (PM-0004-22, -27). With no maths functions and no streams in change-set 2, they start in
    change-set 3.
11. **PM-0002-30's `tsc -b` time** needs the 2.3 chat's configs, which will not be on main ("The 2.3 chat stays stopped
    for now", his words of 2026-11-17). Change-set 2 reports `--ignoreConfig` times instead, as change-set 1 did.
12. **Part 13's risk 9 and Part 15.5's version worry do not bite yet:** the newest three.js (0.187.0) equals the VFX
    runtime's only accepted version.

## 18. Kit, gate and CI edits, with their exact text
- **Kit:** none. **Gate files:** none (`node tools/gate.mjs` runs unchanged). **`.gitignore`:** not a protected file.
- **CI, CI-WASM (a): a draft of the exact text** for `.github/workflows/toolchain.yml` as #60 leaves it. It is not part
  of this change-set. Its final text goes to Mustafa in change-set 2's report after one run on a scratch copy (the stop
  rule for new CI lines). `<sha256>` stands for the value security-auditor's answer records at step 2: this draft
  downloaded nothing, so it holds no hash.

In the step "Fetch every dependency (the last step with the network)", after `npm ci --ignore-scripts --no-audit
--no-fund`:
```
          # The wasm-bindgen CLI the build script needs (it refuses any version but Cargo.lock's), from the project's own
          # release, checked against the sha256 written here, which security-auditor's answer for change-set 2 recorded.
          wb_version=0.2.130
          wb_sha256=<sha256>
          wb_file="wasm-bindgen-$wb_version-x86_64-unknown-linux-musl.tar.gz"
          curl -sSfL --proto '=https' --tlsv1.2 -o "$RUNNER_TEMP/$wb_file" \
            "https://github.com/wasm-bindgen/wasm-bindgen/releases/download/$wb_version/$wb_file"
          printf '%s  %s\n' "$wb_sha256" "$RUNNER_TEMP/$wb_file" | sha256sum -c -
          mkdir -p "$HOME/.local/wasm-bindgen-$wb_version"
          tar -xzf "$RUNNER_TEMP/$wb_file" -C "$HOME/.local/wasm-bindgen-$wb_version" --strip-components=1
```
In the step "Build and test with the network cut off", one more line in the `env -i` list, after
`RUSTUP_AUTO_INSTALL=0 npm_config_update_notifier=false \`:
```
              SHAMBLE_WASM_BINDGEN="$HOME/.local/wasm-bindgen-0.2.130/wasm-bindgen" \
```
and at the end of the single-quoted script, after the last `npx --no -- tsc` line:
```
              node packages/client/scripts/build.mjs
              npx --no -- vitest run --root packages/sim-worker --dir boundary
              common="${strict% --lib es2023}"
              worker=$(find packages/sim-worker/src -name "*.ts" ! -name "*.test.ts" | LC_ALL=C sort)
              client=$(find packages/client/src -name "*.ts" ! -name "*.test.ts" | LC_ALL=C sort)
              [ -n "$worker" ]
              [ -n "$client" ]
              npx --no -- tsc --ignoreConfig $common --lib es2023,webworker --types vite/client $worker packages/sim-worker/pkg/sim_wasm.d.ts
              npx --no -- tsc --ignoreConfig $common --lib es2023,dom,dom.iterable --types vite/client $client
```
To be confirmed on the scratch run before the text is put to him: the tarball's inner folder (`--strip-components=1`
assumes one top folder), and whether `vite/client` resolves inside the cut. The lines that type-check the tests are
added the same way once the scratch run shows which flags they need.

## 19. Sources read for this draft
Registry pages, all read on 2026-11-18. Confidence is high for every version below (read directly):
- crates.io API: `wasm-bindgen` (newest 0.2.130, 2026-10-30, rust_version 1.81, features `default = ["std"]`, checksum
  `a0181c14…3d1c`), its dependencies; `wasm-bindgen-macro`, `wasm-bindgen-macro-support`, `wasm-bindgen-shared` 0.2.130
  dependencies; `rand_chacha` (0.10.0, 2026-02-02) and its dependencies; `rand_core` (0.10.1, no dependencies);
  `ppv-lite86` (0.2.21, needs `zerocopy` ^0.8.23); `zerocopy` (0.8.63); `chacha20` (0.10.2, 2026-08-27; 0.10.0 and
  0.10.1 yanked) and its dependencies and features.
- registry.npmjs.org: `three/latest` (0.187.0, no dependencies, no lifecycle scripts); `@types/three/latest` (0.187.0,
  six dependencies, `typeScriptVersion` 5.6); `vite/latest` (8.3.8); `@dimforge/rapier3d-compat/0.12.0`. This host is
  not on the settings' documentation list: four fetches, kept to versions.
- docs.rs: `rand_chacha` 0.10.0 `ChaCha8Rng` (`get_seed`, `get_stream`, `get_word_pos`); `chacha20` 0.10.2 `ChaCha8Rng`
  (the same, plus `serialize_state`; feature `rng`).
- github.com: three.js releases (r187 "Latest", 30 Oct; r186 "Introduce `dispose()` async"); three.js r187's
  `package.json` (exports `./webgpu`, `./tsl`) and `src/renderers/webgpu/WebGPUBackend.js` (`isWebGPUBackend`, no stored
  adapter, `featureLevel: 'compatibility'`); DefinitelyTyped `types/three/package.json`; wasm-bindgen release 0.2.130
  (17 assets, among them the musl tarball, its `.sha256sum` and a release attestation; "the library MSRV is 1.81"); issue
  #4533 ("New home, new maintainers", open; maintainers daxpedda and guybedford); rust-random/rand's README, CHANGELOG and
  pull request #1658.
- Documentation: rustc's lint list (`unsafe_code`); cargo's `cargo update`; npm's `npm install` options; Vite's
  features (Web Workers, `?worker`, WebAssembly), worker options (`worker.format` default `'iife'`), build options
  (`assetsInlineLimit`, "Set to 0 to disable inlining altogether"), relative base, env variables (`VITE_` prefix;
  static replacement), server options (`host`, `fs.allow`, `fs.deny`); Vitest's CLI (`--dir`, `--root`); Playwright's
  clock (silent on Workers) and `executablePath`; three.js's WebGPURenderer (`forceWebGL`, the fallback) and Renderer
  (`init`, `setAnimationLoop`, `render`, `dispose`); wasm-bindgen's guide, without a bundler (`--target web`, `init`)
  and its move to wasm-bindgen.github.io; MDN `GPUDevice.adapterInfo` (limited availability); the W3C's Secure Contexts
  (127.0.0.0/8 and `localhost` are potentially trustworthy); RFC 9923 (FNV, 64-bit offset basis and prime;
  non-cryptographic).
- Secondary, low confidence (Brave and WebGPU): community and press reports, found by search, that Brave exposes
  WebGPU and that Brave 1.93 hides the GPU's details by default.
- In the repository: the build plan (Since the plan; Parts 0, 1, 2, 3.1, 3.3, 3.4, 3.9, 7, 8, 13), Part 15, change-set 1's
  report, ADR 0002 parts 2-9 and its migration plan, pass marks and spike checklist, ADR 0003 parts 1-4 and its
  migration plan, ADR 0004 parts 3-9 and its pass marks, `docs/DECISIONS.md` (2026-11-17 and 2026-11-18), his two
  messages of 2026-11-18 in the notes, `.claude/CHECKPOINTS.md` §3, `STATUS_ARCHIVE.md`'s follow-ups (WASM-ARTIFACT,
  EMPTY-SLOTS, PACKAGE-ENTRIES, CLIENT-NET-RULE, OVERFLOW-PER-CRATE, OVERFLOW-XCRATE-CI, TWO-PLAYWRIGHTS, TSCONFIG), the
  crates, the packages, both lockfiles and `toolchain.yml`; the container's tools (Node 22.23.4, Rust 1.99.1 with
  wasm32, no wasm-bindgen tool, Chromium 1243 under `/opt/pw-browsers`, Chrome for Testing 153).
