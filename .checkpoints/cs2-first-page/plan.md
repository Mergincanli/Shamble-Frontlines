# Change-set 2 of the build plan, "First page" (foundation step 2.5, first half): the plan

_Adopted by the main session (lane 1, the first builder chat, Fable 5.2) on 2026-11-18 at 21:03 UTC (from the clock),
from engine-architect's draft (`cs2-first-page/plan-draft.md`, Opus 5.5 at effort max, carried over byte for byte from
the notes at commit f167f061; its sources: `cs2-first-page/engine-architect-plan-notes.md`). What the main session
changed is listed in section 20; everything else is the draft's text. **Revision 2, 2026-11-18 21:34 UTC (from the clock):** the
advisor agent's plan review (APPROVE with eight conditions, `cs2-first-page/advisor-plan.md`) is worked in; section 22
says where each condition is met. **Revision 3, 2026-11-18 22:00 UTC:** the verifier's check of the web facts (PASS after four
corrections) and security-auditor's reading on the wasm-bindgen tool's proof of origin (provable here only in part:
question TOOL-SOURCE) are worked in. For the build. Nothing is built. No library was fetched, installed or built for this plan: documentation and registry pages
were read, and four addresses were asked for their headers only (section 6). Base: main
b763ba3e5f34c67fc4f875cfbd9266167fb095f3 (the merge of #60); the branch is level with it. Line numbers are those of the
working tree on 2026-11-18. His words that order this change-set: `cs2-first-page/mustafa-2026-11-18-2059-merge-60.md`
("Then change-set 2: its plan, its review, the build and its pull request, then stop for my go.")._

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
- **Planned size.** About 2,900 hand-written lines, against the build plan's 2,000. The stop line is 4,000, twice the
  build plan's number (section 14).
- **Steps.** 0 paper and preconditions · 1 the Rust core, with no new crate · 2 the one dependency step, and during
  its security wait the small server, the Worker's pure modules and the build script's checks · 3 the answer applied
  and the tool installed · 4 the bindings and the build · 5 the Worker's entry and the boundary tests · 6 the page ·
  7 the browser tests · 8 readings, planted defects, docs, the zip, reviews.
- **Open questions for Mustafa:** RNG-CRATE, WB-HEALTH, TOOL-SOURCE, DEPART-2 and CI-WASM (section 16). OVERFLOW-3P and SIZE-2 are
  no longer questions: the plan follows the register row and the build plan's stop line (section 16, at its end).
- **What holds which step.** Steps 0 and 1 need the advisor agent's APPROVE only, which is given: step 1's code stands
  under every answer, and RNG-CRATE (a) or (b) only adds to it. Step 2, the dependency step, waits for his answers to
  RNG-CRATE, WB-HEALTH and DEPART-2. The tool's download and first run (steps 3 and 4) wait for TOOL-SOURCE. CI-WASM
  holds no step.
- **Where the build plan is wrong or out of date for change-set 2:** twelve points (section 17). The largest: OQ-0002-7
  (is wasm-bindgen's new home healthy) must be read before wasm-bindgen enters, and the build plan places it nowhere;
  rand itself no longer depends on ADR 0004's random-number crate (rand 0.10 uses `chacha20`); the crate keeps a home
  and owners in the rand project but has had no release since 0.10.0 (2026-02-02).

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
1. **#60 is merged: met.** Merge commit b763ba3 (2026-11-18 20:30:46 UTC, on his go of 20:20:09 UTC). His limit "No
   new library of change-set 2 is committed before that pull request is merged" is met with it.
2. **The advisor agent's APPROVE** on this plan: given on 2026-11-18 (its notes end at 21:26:53 UTC), with eight
   conditions (section 22). His go on the build plan covers each change-set "as long as a
   change-set's plan stays inside this one; a plan that leaves it comes back to you" (build plan Part 0, :215-218). So
   the questions of section 16 go to him with this plan, DEPART-2 among them: it lists every place where this plan
   leaves the build plan's text.
3. **His answers to RNG-CRATE, WB-HEALTH and DEPART-2 before step 2.** RNG-CRATE changes what the manifests hold, and
   the security answer covers exactly that content; WB-HEALTH is whether wasm-bindgen enters at all; DEPART-2 holds the
   address the small server prints. Step 1's code stands under every answer (RNG-CRATE (a) or (b) adds the streams'
   state to the world and its bytes after step 3) and starts on the advisor agent's APPROVE. Step 0's reading is done
   (`cs2-first-page/oq-0002-7-reading.md`): it ran between 21:04:49 and 21:07:52 UTC, before the APPROVE, as reading
   and notes only. **TOOL-SOURCE before the tool is downloaded or run** (steps 3 and 4): the advisor agent's condition 5
   says "if it cannot be verified, stop and ask Mustafa", and security-auditor's reading is that the origin can be
   proven here only in part. CI-WASM holds no step.
4. **main taken in: met.** The branch is main b763ba3 plus this checkpoint's commits.
5. **The "Lane requests" issue (#59): read** on 2026-11-18 at 20:54 UTC with the GitHub tools: no comment. Only
   comments by the `Mergincanli` account count, and none is his word.
6. **The zip route: proven.** His message of 2026-11-18, 20:44:29 UTC ("the zip saved and unpacked", with a screenshot
   whose checksum equals the expected one; `cs2-first-page/mustafa-2026-11-18-zip-saved.md`). Route A of his
   BRAVE-ROUTE answer ("A, if not possible: B", build plan :27) stands: no CI job is needed for the Brave check.
7. **The session:** model claude-fable-5-2, read at 20:54 UTC; the background saver runs; the `advisor` agent is
   main's file (`.claude/agents/advisor.md` last changed on 2026-11-13, this session started on 2026-11-14, and this
   branch does not change it).

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
| `src/picture.ts` | the interface file between the tracks | lane 1; engine-architect reviews every change (15.2) | cs2: start, draw, counters, resize, stop. The other members Part 15.2 names (warm up, set quality, the cursor's ray, the muzzle's point, the hook for lane 1's plain lines) are built by lane 1 in change-sets 3 to 5, on the plain stand-in, before `render/` passes to lane 2: "Lane 1 builds all of these in change-sets 2 to 5 [...] so that 7, 8 and 9 need nothing new from lane 2" (part-15 :120-121) |
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
| `packages/sim-worker/pkg/` | build output, ignored | wasm-bindgen's glue, `.d.ts` and `.wasm`: "build output (ignored)" (ADR 0002 part 5) |
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
| `packages/e2e/playwright.config.ts` | new | two projects: `node` (`tests/node/`, no browser) and `chromium` (the rest of `tests/`); the small server as `webServer` only when `SHAMBLE_DIST` names a built folder (section 4.4) |
| `packages/e2e/tests/node/stub.spec.ts` | moved from `tests/stub.spec.ts` | the browser-free test that CI's `npm test` runs today |
| `packages/e2e/tests/skeleton/{skeleton,isolation,clock,allocation,file-url}.spec.ts` | new | lane 1's browser tests. Part 15.2 names lane 2's later folders beside it: `tests/look/`, `tests/assets/`, `tests/feel/` (part-15 :106, :108, :110) |
| `packages/e2e/tests/skeleton/render-counts.json` | new (a fixture) | the empty scene's render counts per backend: ADR 0003's committed baseline (build plan :472) |
| `.gitignore` | edit | `packages/sim-worker/pkg/` (wasm-bindgen's output) and `packages/e2e/playwright-report/`, each by its full path. Not a kit or gate file: CLAUDE.md's protected list does not name it |
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

### 4.4 What main's CI runs on this pull request, with no CI edit, and why it must stay green
The `toolchain` job (`.github/workflows/toolchain.yml` as #60 left it) runs the lines below on every push, inside the
network cut with loopback up. This change-set edits no line of it, so its own files must fit them:
- `cargo fetch --locked` and `npm ci --ignore-scripts` before the cut: they fetch the new crates and packages. That is
  why no push holds a manifest change before the security answer (section 7).
- `cargo test --workspace --locked`, the same with `SHAMBLE_WRITE_TYPES=1`, and the wasm32 build of the five crates:
  `sim-wasm` with wasm-bindgen must compile and test natively, so no native test calls a function that needs
  JavaScript (`JsError` among them), and the generated types must not change.
- `npm test --workspaces`: `types` and `net` are unchanged; `sim-worker` runs `vitest run --dir src` (the unit tests;
  the boundary tests need the built Wasm, which CI does not build before CI-WASM); `client` runs `vitest run` (its unit
  tests, the small server's tests on `127.0.0.1` and the build script's tests: none may need the built Wasm, a built
  `dist/`, the wasm-bindgen tool or a browser); `e2e` runs `playwright test --project=node`, which reads `tests/node/`
  only and must start no server and need no build.
- The two `tsc` lines read `packages/types/src` only; those files do not change.
- The rule that follows: **every file that CI's `npm test` loads resolves without build output.** `sim-worker`'s
  `index.ts` never imports `worker.ts` or the glue; the client's tested modules import `@shamble/sim-worker` for its
  types and its statistics helpers only. Vitest reads the client's `vite.config.mjs`; that file holds nothing that
  changes a test run in Node (checked at step 6).
- Read on the first push that holds the new crates: the job's time against its 15-minute limit (risk 13).

`gate` and `smoke` (the legacy game's checks) are not touched by anything here.

## 5. The exact symbols
**Rust (lane 1).**
- `sim`: `pub const TICK_HZ: u32 = 30` (TICK-30; ADR 0004 part 4: one build-time constant). `pub struct World`, holding
  `tick: u32` and `seed: [u8; 32]`. `World::new(seed: [u8; 32]) -> World`, `World::tick(&self) -> u32`,
  `World::authority_step(&mut self)` (named as ADR 0004 part 3 names it; it advances the tick with `checked_add`, and a
  counter that would pass `u32::MAX` (4.5 years at 30 Hz) stops loudly and never wraps: ADR 0004 part 4 asks for an
  explicit operation, and rust-reviewer reads the choice in the round), `World::write_state(&self, out: &mut Vec<u8>)` (the
  canonical bytes: fixed field order, fixed widths, little-endian, no `usize`: ADR 0004 part 9) and
  `World::state_hash(&self, scratch: &mut Vec<u8>) -> u64`. In `state_hash`: `pub fn fnv1a_64(bytes: &[u8]) -> u64`,
  with RFC 9923's 64-bit offset basis and prime. Without a generator the seed is kept and hashed; ADR 0004 part 8 puts
  the streams' state in the hash. FNV-1a is this change-set's pick for a world of 36 bytes, and it is provisional: ADR
  0004 part 9 leaves the pick to the build ("which function is picked in the sim core spike; the research names none"),
  and PM-0004-21's report at change-set 3 (the function picked and its cost per tick) confirms or replaces it before
  the first recording is committed. The state hash stays inside the Worker: it is not in the block, the readout, the
  copy report or any log, because it covers the match seed and FNV-1a can be worked backwards (ADR 0004 part 8: the
  seed never reaches the client's page or a log).
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
  create; the buffer never moves; whether the pointer's Rust type is `u32` or `*const u8` is settled at step 4, by what
  the glue takes without a lossy cast), `input(&mut self, sequence: u32, now_ms: f64)`, `step(&mut self) -> f64` (runs one
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
- `client/scripts/build.mjs`: `lockedWasmBindgenVersion(lockText)`, `checkCli(version, locked)`, `checkCliFile(path, sha256,
  size)` (before any run of the tool), `sizeTable(files)`, `main()`.

## 6. New libraries
Versions were read from the registries on 2026-11-18 (section 19). The exact set of new lockfile entries comes from the
lockfile diff at step 2 and is exactly what security-auditor answers on. The lists below are what the registries
declare.

| Library | Exact version | Where, and what kind | Why it is needed | What it pulls in | VISION_V2 §6 trace |
|---|---|---|---|---|---|
| `wasm-bindgen` | `=0.2.130` (2026-10-30, Rust 1.81 or newer, MIT OR Apache-2.0) | `sim-wasm` only, normal dependency; the version set once in the root `[workspace.dependencies]` | the control calls the Worker makes are wasm-bindgen's own, with its `.d.ts` (ADR 0002 part 5, option D1); `sim-wasm` is "the wasm-bindgen bindings the Worker loads, and nothing else" (part 3) | declared: `cfg-if`, `wasm-bindgen-macro` and `wasm-bindgen-shared` (both `=0.2.130`), `rustversion` (build); the macro pulls in `wasm-bindgen-macro-support`, which pulls in `bumpalo`, `proc-macro2`, `quote` and `syn` 3 with `visit`, `visit-mut`, `full` and `extra-traits`; `wasm-bindgen-shared` pulls in `unicode-ident`. Already in `Cargo.lock` from ts-rs: `proc-macro2`, `quote`, `syn` 3, `unicode-ident` | §6.2 and §6.9, `sim-wasm`'s row in ADR 0002 part 3 |
| the wasm-bindgen command-line tool | 0.2.130, the release's `wasm-bindgen-0.2.130-x86_64-unknown-linux-musl.tar.gz` | a tool, in no manifest: under `.gate/tools/wasm-bindgen-0.2.130/` here, and in CI after CI-WASM | writes the glue and the `.d.ts` from the `.wasm`; it must equal the library in `Cargo.lock` (PM-0002-18; ADR 0002 part 2) | one binary; the release also publishes a `.sha256sum` file and a release attestation | the same as the library |
| `three` | 0.187.0 (r187, three.js's newest release, 30 October) | `packages/client`, dependency | the renderer the stack settles: one `WebGPURenderer` with its WebGL 2 backend (ADR 0003 parts 1-4) | nothing: no dependencies and no install scripts in its registry entry (MIT, 21,030,536 bytes unpacked) | §6.9 (ADR 0003 part 1's need) |
| `@types/three` | 0.187.0 | `packages/client`, dev dependency | three.js ships no types; the page is type-checked against them (ADR 0002 part 7), and PM-0002-30 and R5-TSLINT-OQ4 ask how many errors and casts that costs | six packages, by its registry entry: `fflate` ~0.8.3, `@types/webxr` >=0.5.17, `meshoptimizer` ~1.1.1, `@types/stats.js` *, `@tweenjs/tween.js` ~23.1.3, `@dimforge/rapier3d-compat` ~0.12.0 (a physics library, 7,522,790 bytes unpacked, no dependencies, no install scripts). Used for types only: nothing of them is bundled, because nothing imports them | ADR 0002 part 7 traces type checking to the foundation (GLOSSARY §8); it serves `three`'s §6.9 use |
| `vite` | 8.3.7 | root, dev dependency | the build and dev server of the `client` entry (ADR 0002 parts 2 and 5; GLOSSARY §8, "TypeScript with Vite") | nothing new: `vite` 8.3.7 is already in `package-lock.json` as Vitest's peer, with `rolldown` 1.2.18, `lightningcss` 1.33.0 and `postcss` 8.5.32. Only the direct declaration is new. The registry's newest is 8.3.8: that bump is its own change-set, as ADR 0002 part 2 asks of every toolchain bump | ADR 0002 part 2's own trace: the foundation (GLOSSARY §8), not a §6 item; it builds the page that serves §6.9 |

**Checked by the main session on 2026-11-18 at 20:58 UTC, without fetching any library** (headers only): the
tool's tarball (11,860,397 bytes by its header) and its `.sha256sum` file (131 bytes), the crates.io index entry of
`wasm-bindgen` and npm's entry of `three` 0.187.0 all answer 200 through this container's proxy. The claims RNG-CRATE
rests on were read again at their sources (section 16).

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
   dependencies and the `exports`, `test` and workspace-dependency lines alike, because a manifest edit that changes no
   dependency is held too and still takes security-auditor's short answer (#60's plan, `kit-hold-ci/plan.md` :40-41 in git at f167f061,
   and the advisor agent's M2 on it, :112). A manifest edit found later takes its own, smaller answer.
2. **Prepared off the main folder.** The edits are made in a scratch worktree, `git worktree add .gate/cs2-deps HEAD`,
   so the main folder keeps working (step 2's list) during the wait. Nothing is built in that worktree. Once the diff
   files are saved in the notes, the edits there are taken back with `git restore` and the worktree is removed
   without force.
3. **Resolved, not built.** Rust: `cargo update --workspace --dry-run`, then `cargo update --workspace`. Per cargo's
   documentation, `--workspace` updates only workspace packages, other packages "only if they don't already exist in the
   lockfile", and it is "useful for updating Cargo.lock after you've changed version numbers in Cargo.toml". These are
   the change-set's only cargo commands without `--locked`, and they compile nothing. npm: the manifests are edited
   by hand with exact versions, then `npm install --package-lock-only --ignore-scripts --no-audit --no-fund` with no
   package named, which per npm's documentation uses "only the package-lock.json, ignoring node_modules". **The
   lockfile rule:** both lockfile diffs may add entries and change the lines of our own crates and packages. On an
   entry that is already there, only npm's flags may change (`peer`, `dev`, `optional`, `devOptional`: declaring
   `vite` directly is expected to drop `"peer": true` from it and from what it pulls in), and each such change is
   listed for security-auditor. A changed version, `resolved` address, `integrity` or checksum value, or dependency
   list of an entry that is already there stops the step.
4. **What security-auditor answers on:** the text diff of every manifest the step changes (seven by this plan: two
   `Cargo.toml` and five `package.json`; eight with a generator crate) and of both lockfiles (saved as a `.diff` file in
   the notes, never as files named like manifests), the sha256 of each of those files, and the tool's
   tarball: its URL, its sha256, the release's published `.sha256sum` and its release attestation. The tool's check is
   recorded in a notes file of its own (`.checkpoints/cs2-first-page/wasm-bindgen-cli-check.md`), because
   `dependency-check.md` takes only manifest and lockfile lines. security-auditor runs on Opus 5.5 at effort max, reads
   registry and release pages, and builds, installs and runs nothing of the new code.
5. **After the answer (step 3):** the same diff is applied in the main folder (sha256 compared with the reviewed files),
   `dependency-check.md` is written (one `<sha256>  <path>` line per file, naming the answer's notes file), the files are
   committed by path, and the checkpoint script pushes them, which releases its own hold. The record is written before
   the manifests are committed. Every push of this step goes through `checkpoint.mjs save` in the foreground and none
   by hand: "a plain `git push` is not held" (the background saver does not push commits outside `.checkpoints/`:
   `checkpoint.mjs:226`). Only then: `npm ci --ignore-scripts`, the tool's route below, and the first build that holds
   the new crates.
5a. **The tool's route** (the advisor agent's condition 5; it is code from the internet that no lockfile covers).
   What the release offers and what can be checked here is in `cs2-first-page/security-auditor-tool.md`
   (security-auditor, Opus 5.5 at effort max, 2026-11-18): the release is marked immutable by GitHub; the tarball's
   sha256 is 9687f3bb551b9f66f37ed32a40d7913a00f7962db653054e56857bc306c3227f by three statements that agree (the
   project's `.sha256sum`, the digest GitHub states for the file, and GitHub's signed release statement, whose
   signature checks against the key in its own certificate); its size is 11,860,397 bytes. **Not provable here:**
   that the statement's certificate chains to GitHub's root (the tool for that, `gh release verify-asset`, is not
   usable in this container), and how the project built the binary (no build record exists). So the first run here
   needs his answer to TOOL-SOURCE. Under its (a), the route is:
   1. a new folder made with plain `mkdir` under `.gate/` (neither `.gate` nor `.gate/tools` a link); the download
      with `curl --fail --proto '=https' --proto-redir '=https' --max-filesize 12000000`;
   2. the size compared with 11860397, then the sha256 with the value above, before any `tar` call;
   3. the member list read with `tar -tzf` and `tar -tvzf` and recorded: exactly the folder
      `wasm-bindgen-0.2.130-x86_64-unknown-linux-musl/` with `README.md`, `LICENSE-MIT`, `LICENSE-APACHE`,
      `wasm-bindgen`, `wasm-bindgen-test-runner` and `wasm2es6js`; regular files and folders only, no name twice, no
      member above 64 MiB; anything else stops the step;
   4. only the one member taken out, into the new empty folder `.gate/tools/wasm-bindgen-0.2.130/`, with
      `--no-same-owner --no-same-permissions --strip-components=1` (this container runs as root, where `tar` would
      keep owners and permission bits);
   5. before any run: it is a regular file and no link; `chmod 0555` on the file and its folder; `readelf -h` and
      `readelf -l` show an ELF64 x86-64 file with no interpreter (static);
   6. the binary's sha256 and size go into `cs2-first-page/wasm-bindgen-cli-check.md`, and `build.mjs` compares them
      before every run, `--version` included (a build script runs as the same user and could replace the file), and
      refuses to run the tool at all when the expected value is not given. The
      expected values reach `build.mjs` through the environment (`SHAMBLE_WASM_BINDGEN_SHA256`), read from that notes
      file here and written in the workflow in CI.
   No other source is used without a security answer of its own.
6. **No shared cargo target folder.** The planted-defect worktrees build into folders of their own
   (`CARGO_TARGET_DIR=.gate/cs2-plant-target`). Change-set 1's export tests also stop when they run outside the copy they
   were built in.
7. **The netcode proof test's three rules,** checked over this change-set's own files: no root `.cargo/config.toml`
   (none is planned: the build script passes its settings on the command line); no `"type"` in the root
   `package.json`; no tracked file that is not Markdown names the spike's folder. No `.npmrc` is added either: the guard
   protects that name at any depth (`.claude/hooks/routine-guard.mjs:38`).

## 8. Build steps, in order
Each step ends with the checks of section 9 that apply to it, all exit 0, then a commit by path and a checkpoint. A
step that changes no manifest pushes normally; step 3's push goes through the hold.

0. **Paper.** The preconditions of section 3. OQ-0002-7's by-day reading: the main session has a fetch agent read
   wasm-bindgen's maintenance issue (#4533) and the guide's governance page on the day, and writes what the two pages
   say about maintainers and release policy into `cs2-first-page/oq-0002-7-reading.md` ("What decides it: what those
   two pages say about maintainers and release policy", ADR 0002 :1092). Its decider "comes before the step that adds
   wasm-bindgen" (EMPTY-SLOTS, STATUS_ARCHIVE :850-856), and the result goes into the report (build plan Part 3.9's
   rule). If the pages show no active maintainer or no release policy, the change-set stops before step 2 and tells
   him. **Done on 2026-11-18:** maintainers yes, no written release policy, so it goes to him as WB-HEALTH (section
   16). His answers to RNG-CRATE, WB-HEALTH and DEPART-2.
1. **The Rust core, with no new crate.** `sim`: the world, the authority step, the canonical bytes and the state hash.
   `client-endpoint`: `Section::align()`, the lookups by name, the block writer and the local schedule. Tests: the hash
   against RFC 9923's FNV-1a 64-bit test values; the state bytes at fixed widths, with no `usize` and with a stated
   length; the same seed giving the same hashes, a different seed a different hash; the header's bytes at the table's
   offsets; the padding left zero; the lookups; `align`; the schedule without drift over 1,000,000 ticks, and `rebase`.
   No manifest changes, so this step pushes normally.
2. **The one dependency step** (section 7), prepared in the scratch worktree; the planned manifest text is below this
   list. While security-auditor's answer is awaited, the main folder goes on with the work that needs no new package,
   each part tested with the Vitest already installed and committed by path:
   - **the small server,** `serve.mjs`, with Node's own modules only, running on Node 18 or newer (Part 8). It binds
     `127.0.0.1` only and prints `http://127.0.0.1:4173`. It answers `GET` and `HEAD` only, serves files inside its own
     folder only (paths decoded once and resolved, real paths checked against the folder, so a `..`, an encoded `..`, a
     backslash or a link that leads outside is refused), sends a fixed table of content types (`application/wasm` for
     `.wasm`), `X-Content-Type-Options: nosniff` and `Cache-Control: no-store`, refuses a `Host` header that is not its
     own address (against DNS rebinding), and stops with a clear message if the port is taken. Its tests cover each of
     these. It is never bundled into the page;
   - **the Worker's pure modules** (`pacing`, `block-pool`, `read-block`, `stats`, `messages`) and their unit tests: the
     pacer on a fake clock (7 ticks per wake at most, sleep until the returned due time, pause, rebase, the counters),
     the pool, the statistics;
   - **the build script's checks** (`lockedWasmBindgenVersion`, `checkCli`, `sizeTable`) and their tests.
   Nothing that imports `three`, its types or the Wasm glue is written before the answer: it could not be checked.
3. **The answer applied.** The reviewed diff is applied in the main folder (each file's sha256 equal to the reviewed
   one), `dependency-check.md` is written, the files are committed by path, and the checkpoint script pushes them through
   its hold. Then `npm ci --ignore-scripts`, and, once TOOL-SOURCE is answered, the tool by its route (section 7, item
   5a): downloaded, its size and sha256 compared, its member list read, the one binary taken out into
   `.gate/tools/wasm-bindgen-0.2.130/`, checked without running it, its own sha256 recorded.
   Then `cargo tree --workspace -e normal,dev` names every third-party crate compiled into our libraries and tests
   (ts-rs and thiserror are dev-dependencies, so the `dev` edges are needed; proc-macro crates are left out, as the
   rule says), against the overflow overrides of step 2 (section 16, at its end).
4. **The bindings and the build.** `sim-wasm`'s `Sim`. The build path of `build.mjs`: it reads wasm-bindgen's version
   from `Cargo.lock`, refuses any other tool version before it generates bindings (PM-0002-18), runs `cargo build
   --locked --release --target wasm32-unknown-unknown -p sim-wasm`, then `wasm-bindgen --target web --out-dir
   packages/sim-worker/pkg`, records the sizes (raw, gzip at level 9 and Brotli at level 11, PM-0002-29's settings, from
   Node's own `zlib`), runs Vite's build through its JavaScript API with `VITE_SHAMBLE_COMMIT` and, unless
   `--production`, `VITE_SHAMBLE_DEVTOOLS=1`; in the development flavour only it copies `serve.mjs` into `dist/` (Part
   0: the zip's small server "is kept out of the production build"); it fails if any served file reaches 26,214,400
   bytes (PM-0002-29's gated clause), if the built `index.html` holds an inline script or style, or if a built file
   loads the Worker or the Wasm from a `blob:` or `data:` URL (ADR 0002 :434-436, CSP-ready; a scan that stands in for
   G8 until the gate v2 has it). Here the cost of the overflow overrides is read too: the Wasm's size and the step's
   time with the lines and, in a scratch worktree with its own target folder, without them. It runs commands as argument arrays (`execFileSync`), never through a
   shell string. **Risk 1 is settled here:** whether the workspace's `unsafe_code = "deny"` accepts `#[wasm_bindgen]`'s
   expansion.
5. **The Worker's entry and the boundary tests.** `worker.ts` and `index.ts`. Boundary tests on the built Wasm: a block
   written by Rust passes `checkLayoutHash` (PM-0002-16's page half, which change-set 1 left to this writer); after
   `memory.grow` between two steps, the next block is byte for byte the block of a run without the grow (PM-0002-6); two
   steps advance the tick by two; mode 1 and a 31-byte seed are refused; per tick the host makes exactly two calls,
   `input` and `step`, and a reading of the generated glue shows each call reaching exactly one export (PM-0002-3 is
   counted in full in change-sets 3 and 5).
6. **The page.** The app shell, `picture.ts`, `render/`, the readout and the copy report, the harness's CPU side,
   `index.html` (with no inline script or style: ADR 0002 part 7, CSP-ready), `styles.css`, and `vite.config.mjs`:
   `base: './'`, `build.assetsInlineLimit: 0` (so the Wasm never becomes a `data:` URL), `worker.format: 'es'` (a module
   Worker; Vite's default is `'iife'`), `server.host` and `preview.host` `127.0.0.1`, `server.fs.allow` narrowed to what
   the page loads (Vite's default serves the whole workspace root, which here is the repository), and COOP and COEP
   headers on the dev and preview servers only when `SHAMBLE_ISOLATION=1` (the isolation flag, PM-0002-20). Unit
   tests: the report's text, the frame-time statistics, the flags, the Worker link against a fake Worker (the newest
   two blocks kept, the older ones returned on the next frame's post, the layout check refusing a block of another
   layout).
7. **The browser tests.** `playwright.config.ts`: project `node` (`tests/node/`, no browser; this is what `npm test`
   runs) and project `chromium` (the rest of `tests/`; lane 1's specs in `tests/skeleton/`, as Part 15.2 lays the
   folder out), the small server as `webServer` on the built `dist/` only when
   `SHAMBLE_DIST` names it (without it the `chromium` project stops with a line that says so, and the `node` project
   starts no server: section 4.4), retries 0, `forbidOnly` in CI, traces off, the browser's path from
   `SHAMBLE_CHROMIUM` when set. Specs:
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
   move while hidden and that no wake runs more than 7 ticks. The page records each pause itself (how long it was
   hidden, and the ticks that ran meanwhile), and the readout and the copy report show the last one: that is what
   makes the hidden tab checkable by eye. A real hidden tab is tested in Brave (section 13).
8. **Readings, planted defects, docs, the zip, reviews.** The readings and planted defects of section 9. The production
   flavour is built once and shown to hold none of the development-only code (Part 0: each development tool "is kept
   out of the production build"): a scan of `dist-prod` for named markers of the readout, the copy report, the
   harness, the `pad` flag and the report's test hook finds none, `serve.mjs` is not in it, and the same scan on the
   development build finds each marker (the control). `?backend=webgl` stays in both flavours: it is one option
   passed to the renderer, and the fallback must be testable on the build players get. The writer's padding is a
   parameter of the one Wasm both flavours load (one feature set, PM-0004-74); the production page always passes 0. Then the report, docs-keeper's pass, his words by script, ADR 0002's pointer for
   SPLIT-25, and the zip, `shamble-v2-02-first-page.zip` (the built `dist/` with `serve.mjs`, a `README.txt` with the
   steps and the commit, and `SHA256SUMS.txt`), made with `python3 -I -m zipfile -c` from `.gate/`, never committed,
   sent through the chat's file tool. Then the reviews (section 12), the verifier until PASS, and the advisor agent on
   the final head.

**Planned manifest text for step 2** (fragments, not whole files; the `//` lines are labels, not JSON; the reviewed
content may differ from these only by the lockfiles' resolution):
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
```toml
# Cargo.toml (root), after [profile.release.package."*"]: one block per third-party crate compiled into our libraries
# and tests (register row OVERFLOW-ALL: "a named override for each such crate"); the list is read from the lockfile
# in step 2. By this plan: wasm-bindgen and the runtime crates its lockfile entry lists, and ts-rs and thiserror.
[profile.release.package.wasm-bindgen]
overflow-checks = true
```
Under RNG-CRATE (a) or (b), `crates/sim/Cargo.toml` and the root `[workspace.dependencies]` gain the generator's
line, with `default-features = false`, and the generator's crates join the overflow list. All of it happens inside
this same step, before the answer.

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
**The build (from step 4):**
```
SHAMBLE_WASM_BINDGEN="$PWD/.gate/tools/wasm-bindgen-0.2.130/wasm-bindgen" SHAMBLE_WASM_BINDGEN_SHA256=<the binary's recorded sha256> node packages/client/scripts/build.mjs
SHAMBLE_WASM_BINDGEN="$PWD/.gate/tools/wasm-bindgen-0.2.130/wasm-bindgen" SHAMBLE_WASM_BINDGEN_SHA256=<the binary's recorded sha256> node packages/client/scripts/build.mjs --production --out-dir .gate/cs2-first-page/dist-prod
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
SHAMBLE_CHROMIUM=/opt/pw-browsers/chromium SHAMBLE_DIST="$PWD/packages/client/dist" node node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=chromium --trace off
cargo tree --locked --workspace -e normal,dev --prefix none    # every third-party crate that is no proc macro has its overflow block (proc macros are told apart by cargo metadata)
node node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=node --trace off
```
**The rest:**
```
node tools/gate.mjs --no-smoke              # smoke cannot start here (Chromium 1194 wanted, 1243 installed: cs1's report); CI runs it
node .claude/scripts/repo-map-check.mjs
test ! -e .cargo/config.toml
node -e "process.exit('type' in JSON.parse(require('fs').readFileSync('package.json','utf8')) ? 1 : 0)"
git grep -n -I -e 'spikes/' -- $(git diff --name-only origin/main...HEAD -- . ':!*.md')    # prints nothing
```
CI: `gate`, `smoke` and `toolchain` green on the head, for the push and for the pull request. Each package's `test`
script is also run here as CI runs it (from the package's folder, with the tool by its path), so a red `npm test` in
CI is seen before the push (section 4.4).

**Planted defects**, each in a throwaway worktree with its own target folder, never committed, removed without force;
each must fail the named check, and its control must pass:
1. the page calls `WebAssembly.instantiate` (a debug readout: PM-0002-2's own bite) → `skeleton` counts 1 and fails;
2. a block posted without its transfer list (PM-0002-6) → the Worker's undetached count is above 0, and `skeleton`
   fails;
3. a host that caches its view of `memory.buffer` (PM-0002-6) → the `memory.grow` boundary test fails;
4. a header field added in Rust without regenerating (PM-0002-16) → `generated_layout_is_current` fails, and the
   boundary test's `checkLayoutHash` refuses the Rust-written block as `layout-hash`;
5. a tool one patch off: a stand-in printing `wasm-bindgen 0.2.129` (PM-0002-18) → the build script exits 1 before it
   writes any binding; control: 0.2.130 passes; and a tool file whose sha256 is not the recorded one → the build
   script exits 1 before it runs the file at all;
6. a misspelt field name in the writer's lookup → the build fails at compile time;
7. a 64-byte typed array allocated per frame in the page's block handler (PM-0002-7's bite) → `allocation` reports it;
8. a Worker that runs the hidden gap instead of rebasing (PM-0004-70's bite) → the visibility check sees the counter
   move and more than 7 ticks in one wake;
9. a `serve.mjs` that lets an encoded `..` through → its test fails;
10. an isolation flag that sets COOP but not COEP (PM-0002-20's own bite, ADR 0002 :1292) → `isolation` reads
    `crossOriginIsolated` false where the flag is on, and fails.

**Readings** (reported, never judged; the machines are named before change-set 6, MACHINES (a)):
- the hand-off as a curve over block sizes (8 bytes, the header alone; 1, 4, 16 and 64 KiB): the copy time in the
  Worker (p50, p99) and the delivery from the Worker's finished step to the page's bound views on a shared clock
  (`performance.timeOrigin + performance.now()`; PM-0002-8's method, p95): all five sizes in the container's Chromium,
  and three of them (8 bytes, 4 KiB, 64 KiB) in Brave on his laptop from his reports (section 13, step 10), which is
  the build plan's "a reading on your laptop" (:540);
- the cost of the overflow overrides: the Wasm's size and the step's time with the lines and without them (the
  register row's "with its cost measured");
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
1. **`unsafe_code = "deny"` against `#[wasm_bindgen]`'s expansion.** The lint catches `unsafe` blocks, `no_mangle`,
   `export_name` and `link_section` (rustc's lint list), the kind of items exported bindings are made of. No source read
   here shows what 0.2.130's expansion holds, or whether rustc reports the lint inside an external proc macro's
   expansion. Step 4's first build settles it. Fallback: `#[allow(unsafe_code)]` on the
   one bindings module, with its written reason and a test, which ADR 0002 part 4 allows. Step 2.2's WASM-ARTIFACT
   follow-up already expected "its `unsafe_code` exception" to "come with the bindings".
2. **The container's Chromium is not Playwright 1.65.0's build** (1243 installed, 1254 expected; Chrome for Testing 153
   also present). Playwright's docs: "Playwright only works with the bundled Chromium, Firefox or WebKit, use at your
   own risk." If neither browser launches, there is no browser evidence here. Stop and tell: no browser is downloaded
   without a security answer. This environment's own notes name `/opt/pw-browsers/chromium` as the path for a project
   that pins another Playwright version, so the route is the one meant for this container; what it cannot show is
   said in the report.
3. **Headless Chromium here probably gets no WebGPU adapter** (ADR 0002 part 9's reading of R3), so both browser runs
   may be WebGL 2, and WebGPU is then seen only in Brave on his PC. The report says which backend each run got.
4. **Brave may hide the adapter's details** (secondary sources say Brave 1.93 does, by default; not an official source)
   **or not expose WebGPU** (R3-WGPU-OQ1 is open). The label says what runs, and an empty adapter line is normal.
5. **The dependency answer may refuse something,** for example one of `@types/three`'s six packages. Fallback, his to
   choose then: a small local declaration file for the handful of three.js names change-set 2 uses, which loses
   PM-0002-30's reading until the real types return.
6. **The zip route:** proven on 2026-11-18 (section 3, item 6). No longer a risk.
7. **wasm-bindgen broke compatibility "at least twice in 2026, in 0.2.123 and 0.2.128"** (OQ-0002-7's text, ADR 0002
   :1086-1087). The exact pin
   and the tool check hold it. Each bump is its own change-set.
8. **CI runs none of the Wasm build, the boundary tests or the new packages' type check until CI-WASM's edit merges,**
   and no browser until after change-set 3. Until then those marks are "shown", not "met". Adding the build to the
   `toolchain` job may also press on its 15-minute limit: measured on the CI-WASM run.
9. **three.js moves on.** r187 is the newest today and equals the only version the Shamble VFX runtime accepts (Part
   15.5), so nothing conflicts now. ADR 0003 part 2 pins "the newest when the look proof test starts", which is change-set
   6. If r188 is out by then, Part 15.5's question applies there.
10. **The size.** The change-set stops and tells him in three lines when it passes 4,000 lines (section 14).
11. **Every timed number is a reading** until the machines are named (MACHINES (a)).
12. **The dev server and the isolation flag:** Vite serves the workspace root by default, which here is the whole
    repository, and COEP can block a resource the page needs. The allow-list is narrowed and the four Chromium cases run
    in step 7.
13. **The unchanged `toolchain` job has 15 minutes,** and from step 3's push on it also fetches and compiles
    wasm-bindgen (natively for the tests, and for wasm32). Its time is read on that push. If it comes near the limit,
    that is a CI question for him (the limit is a CI line), not something this change-set edits.
14. **This container's proxy refuses github.com pages of other repositories** (seen on 2026-11-18 at 20:58 UTC on a
    pull request page), while release downloads answer. If the tool's tarball is refused at step 3: stop and tell
    him. No other source is used without a new security answer.

## 11. Change type and risk tier
Logic (the simulation's world, the writer, the schedule, the pacer, the pool), visual (the scene and the readout),
performance (readings), security (a new local server, new dependencies, page text). **Risk tier 1:** game code, pages,
tests; no kit, gate or CI file. It is `needs-mustafa` in any case: there is a Brave check, and `crates/` and `packages/`
lie outside the places the PR Advisor may merge (build plan Part 0). Under CI-WASM (b) the pull request becomes risk
tier 2.

## 12. Reviewers
The build plan's list (Part 1, :313-314) and netcode-architect, each at effort max, at most three at a time; the
seven together (three, three, one) are one round (CLAUDE.md's review budget):
- engine-architect (module boundaries, the interface file, the folder cut, the Worker contract), graphics-engineer (the
  renderer's set-up, the backend report, the render counts), rust-reviewer (the world, the writer, the bindings, the
  `unsafe_code` outcome, the Wasm build);
- then perf-auditor (the frame and tick paths, the pool, allocation, the readings' method), security-auditor (the new
  dependencies, the page's text, the small server, the dev server's settings, the build script, the overflow
  overrides), test-engineer (is each claim proven; the planted defects; the browser tests);
- then netcode-architect (CLAUDE.md step 5: "sim, server, protocol"): `sim` gets its first code here (the world, the
  tick counter's overflow rule, the canonical bytes, the state hash), and the schedule is the one ADR 0004 part 5
  shares with online play. The advisor agent set the same condition on change-set 1.
- **art-director is not called,** on purpose: the scene is a placeholder that decides nothing of the look ("for v2
  work, LOOK_V2.md is the look brief and wins; ART_DIRECTION.md describes the legacy game only", his LANE-RULES), the
  readout is a development tool that no player sees, and his Brave check is the gate for what is visible. The look's
  first review is change-set 6's, in lane 2.
Outside the round: security-auditor's dependency answer at step 2 (a condition of building, not a review), the advisor
agent's plan review, the verifier until PASS, and the advisor agent on the final head. Models: lane 1's main model is
Fable 5.2 (his decision of 2026-11-17); every agent is Opus 5.5. The `advisor` agent type is main's file (section
3, item 7).

## 13. What Mustafa checks in Brave (about 18 minutes, 9 of them waiting)
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
6. Switch to another tab for 1 minute, then come back. Expected: the readout's "last pause" line says about 60 s
   hidden and "ticks while hidden: 0", and the tick counter counts on from where it stood. (A match that ran while
   hidden would show about 1,800 ticks for the minute.)
7. The same with 6 minutes away. Expected: about 360 s hidden, ticks while hidden 0.
8. Press "Copy report" and paste the result into the chat.
9. Open `http://127.0.0.1:4173/?backend=webgl`. Expected: the readout says "WebGL 2 fallback", with the same scene and 0
   errors. Press "Copy report" again and paste that too.
10. Open `http://127.0.0.1:4173/?pad=4096`, wait 30 seconds, press "Copy report" and paste it; then the same with
    `?pad=65536`. Expected: the same scene, errors 0. These two runs read, on your laptop, what it costs to hand a
    larger block to the page; they are readings, not judged.
11. Open `brave://version` in a new tab and paste its first line.
12. Back in PowerShell, press Ctrl+C to stop the server.

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

The build plan's row says 2,000. Its own estimates are "good to about half as much again either way" (:198-200). The stop
line is 4,000 lines, twice the build plan's number: the tighter of the two readings of his rule ("when one passes
twice its plan while it is built, stop and tell me in three lines"), with about 2,900 as this plan's estimate, inside
the build plan's own margin. Time: the build plan's "about 13 h", scaled by the line count, is about 19 h. That is
arithmetic on the plan's estimate, not a measurement.

## 15. Not in it
- `protocol`'s codec, the loopback, `room`, L1, the replay runner, the panic hook's buffer, the first recording, the
  maths and stream vectors, the scan test in `sim`: change-set 3. The two-instance test: lane 2's comparison, after
  change-set 3 (Part 15.2).
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
- Why it matters: it is pinned for good, and rand itself no longer depends on the crate ADR 0004 names.
- What was found (read again by the main session on 2026-11-18: the changelog and the registry directly, the pull
  request page through a fetch agent; then checked at the sources by the verifier, PASS after four corrections:
  `cs2-first-page/verifier-claims.md`): ADR 0004 part 8 names `rand_chacha`'s `ChaCha8Rng`. rand 0.10.0's changelog
  says "The dependency on `rand_chacha` has been replaced with a dependency on `chacha20`". The rand project took
  `rand_chacha` out of its main repository (pull request #1658, "Remove rand_chacha", merged on 2026-01-23: "Source
  moved: rust-random/rngs#68"); its newest release, 0.10.0, is of 2026-02-02. One maintainer (dhardy) wrote there on
  2026-01-20: "I no longer see a reason to maintain `rand_chacha`." Asked by vks, a contributor, whether to make a
  final release that marks it deprecated, dhardy answered that he sees no need to force people to migrate. After that,
  newpavlov proposed an empty 0.11 that points users to `chacha20` ("thus effectively deprecating `rand_chacha`");
  nothing on the page answers that proposal. Against that stand three facts: the `rngs` repository's README says
  "This repository houses extra generators maintained by the project"; `rand_chacha`'s changelog there says "The
  crate was moved from the rand repo ([rand#1658]) to the rngs repo ([#92])"; and crates.io lists as its owners
  dhardy and the team `github:rust-random:maintainers`. So the crate has a home and owners in the rand project; what
  it has not had is a release since 0.10.0. `chacha20` 0.10.2 (2026-08-27, from RustCrypto; 0.10.0 and 0.10.1 were
  yanked) has the feature `rng` with its own `ChaCha8Rng`. rand's changelog says of its own 12-round generator that
  after the move "the output remains the same"; nothing read says the same of `ChaCha8Rng`, and `chacha20`'s docs
  do not say that its output equals `rand_chacha`'s. Change-set 3's 10,000-output vectors pin whichever crate is
  chosen.
- Options: (a) `rand_chacha` 0.10.0 in change-set 2, as ADR 0004 and the build plan say: a pinned, frozen copy whose
  tree adds `rand_core`, `ppv-lite86` and `zerocopy`; (b) `chacha20` 0.10.2's `ChaCha8Rng` (feature `rng`) in change-set
  2: the RustCrypto crate that rand 0.10 itself uses, with the same algorithm and the same calls for the seed, the
  stream and the position, and a smaller tree (`cfg-if`, `rand_core`, and `cpufeatures` on x86 only) when taken with
  `default-features = false` (its default feature `cipher` adds the `cipher` crate). This changes ADR
  0004's crate name, recorded by a dated pointer at the drafts' next revision; (c) no generator in change-set 2:
  nothing there draws a random number, so the change-set carries only the seed path (the Worker draws 32 bytes, the world
  keeps them, the state hash covers them). The generator enters in change-set 3 beside its 10,000-output test vectors
  (PM-0004-27), with (a) or (b) answered by then.
- Recommendation: (c). A generator nothing calls is code "for later", and the proof of the streams is change-set 3's
  anyway. Which crate it is can then be asked with change-set 3's plan, after both are read side by side; today's
  leaning is (b), because rand 0.10 itself depends on `chacha20` (feature `rng`) and it is the only one of the two
  with a release after February (0.10.2, 2026-08-27). If you answer (a) or (b) now, that crate enters in change-set 2.

**WB-HEALTH: wasm-bindgen's project has maintainers but no written release policy. Does the game still build on it?**
- Why it matters: every call between the simulation and the browser goes through wasm-bindgen, for years, and the
  engine draft left this question open until the day wasm-bindgen enters (OQ-0002-7). That day is step 2.
- What was found (the reading of 2026-11-18, `cs2-first-page/oq-0002-7-reading.md`, each line with its source): the
  project names three core maintainers, decides by a majority vote of that core team, requires a review on every
  change, and has made about one release a month through 2026 (the newest, 0.2.130, on 2026-10-30). Its guide and its
  maintenance issue hold no release schedule and no promise that a new release keeps working with older code; the one
  policy found is a 2-year rule for the Rust version a release needs. The engine draft itself notes that
  compatibility broke "at least twice in 2026, in 0.2.123 and 0.2.128" (ADR 0002 :1086-1087).
- Options: (a) go on, behind the exact pin (`=0.2.130`), with the build refusing a command-line tool of any other
  version and every bump as its own change-set. This is the engine draft's option for this question, and the only one
  it researched; (b) stop before step 2 and research another way to bind Rust to the browser first (none is researched;
  it would be a research task of its own, and change-set 2 waits for it).
- Recommendation: (a). The pin and the tool check are the guard for exactly this missing promise, and the project is
  active.

**TOOL-SOURCE: the origin of wasm-bindgen's command-line tool can be proven in this container only in part. May it
run here on that proof?**
- Why it matters: the tool is a program from the internet that no lockfile covers, it runs with this container's
  full rights at every build, and your rule is that no unreviewed code fetched from the internet is run. The advisor
  agent's condition on this plan: if its origin cannot be verified before its first run, stop and ask you.
- What was found (security-auditor's reading of 2026-11-18, `cs2-first-page/security-auditor-tool.md`): wasm-bindgen
  publishes the tool as a ready-built program in its release 0.2.130, which GitHub marks as unchangeable. Three
  statements agree on the file's checksum: the project's own checksum file, the checksum GitHub states for the file,
  and a statement about the release that GitHub signed. Here I can check that the download equals that checksum, and
  that the signature fits the key inside the statement. Here I cannot check that this key belongs to GitHub: the
  program that does that check (`gh release verify-asset`) needs GitHub's pages and API for another project, which
  this container's network proxy refuses (the signed statement's download answered 403), and this session keeps the
  `gh` program to code search; it is not a permission you could simply grant in chat. About how the program was
  built: the project's workflow file gives the build steps, but it publishes no signed record of the build, and it
  commits no lockfile and builds without `--locked`, so the versions of the libraries inside the release's program
  are, in security-auditor's words, "recorded nowhere we can read". security-auditor rates what remains as Medium,
  and compares it with how the Rust toolchain we already run is checked (a checksum from a list, over an encrypted
  connection).
- Options: (a) yes: the ready-built program, pinned by that checksum and by its size, unpacked by the careful route
  of section 7 (item 5a), its own checksum compared before every run; and the full check against GitHub's key added
  to CI when CI starts building the Wasm (CI-WASM), so that every later run there proves it; (b) build the tool here
  from its published source instead. That removes the trust in the project's release machinery, but it needs a
  security check of the tool's own libraries first: its published lockfile has 254 entries, against about 5 new ones
  for the library (security-auditor: roughly 50 times the library's review). That count includes crates for other
  systems and test-only crates, and fewer would compile with the tool's network feature off; the exact number needs
  a fetch-only step. An estimated 30 to 40 of them run code while building. The same build would run again in CI;
  (c) wait: first a one-off CI job that runs GitHub's own check on the file, and only after it has passed does the
  tool run here. Its costs: a workflow file on this branch, which makes the branch risk tier 2 while it is there
  (your yes to its exact text, and security-auditor's read of it); it must be taken out again before the pull
  request is marked ready; and the proof reaches this container only through the checksum match, which holds
  because the release cannot change.
- Recommendation: (a), which is security-auditor's. (c) is the cautious middle if you want the full proof before
  the first run; it costs one more round between us.

**CI-WASM: how does CI start building and testing the new parts (the Wasm bindings, the Worker's tests on the real
Wasm, the type check of the two new packages)?**
- Why it matters: until a CI edit, CI runs none of these and they are checked by hand; each CI edit is yours to approve
  with its exact text (GATE-FOLLOW (b)).
- Options: (a) a small pull request of its own after change-set 2 merges, as #60 followed #58, with the exact text in
  change-set 2's report (draft in section 18) and the tool downloaded from wasm-bindgen's own release, checked against
  a sha256 written in the workflow and then by GitHub's own check of the release file (`gh release verify-asset`,
  the full proof TOOL-SOURCE (a) promises for CI); (b) the same edit inside change-set 2's pull request, which makes
  it risk tier 2; (c) as (a), but with the tool installed by the third-party action ADR 0002 part 2 names
  (`taiki-e/install-action`, pinned by SHA). That puts one more JavaScript action into the job that runs the network
  cut, which part 2 of your CI-CUT answer moves away from, and the action's own download would not carry GitHub's
  check unless a step after it runs that check on the installed file.
- Recommendation: (a).

**DEPART-2: this plan leaves the build plan's text in six small places. Do you accept them?**
- Why it matters: your go on the build plan covers a change-set's plan only "as long as a change-set's plan stays
  inside this one; a plan that leaves it comes back to you".
- The six (section 17 has each with its source):
  1. The hand-off is read as a curve over block sizes (8 bytes, then 1, 4, 16 and 64 KiB), not "with the block filled
     to the size of 12 and of 48 soldiers": no source gives the size of a soldier's row before change-set 4.
  2. The small server prints `http://127.0.0.1:4173`, not `http://localhost:4173`: it binds 127.0.0.1 only, and a
     browser may resolve `localhost` to another address.
  3. The unit vectors in Wasm start in change-set 3, with the maths functions and streams they test. Part 7's table
     puts them at change-set 2, where there is nothing to run them on.
  4. The type check's time is read with `--ignoreConfig`, not with `tsc -b`: the 2.3 chat's config files are not on
     main.
  5. Two pieces of work are added that the build plan places nowhere: the reading of wasm-bindgen's health (OQ-0002-7)
     before wasm-bindgen enters, and the way its command-line tool reaches this container (one download from the
     project's own release, by the route TOOL-SOURCE decides).
  6. Under RNG-CRATE (c), the card's "seeded random streams" move to change-set 3 and change-set 2 keeps the seed's
     path only. It is listed here for completeness: your answer to RNG-CRATE decides it.
- Options: (a) yes, all six; (b) yes, except the ones you name; (c) no: the build plan's text stands, and I come back
  with what that costs.
- Recommendation: (a).

### No longer questions (the advisor agent's plan review, findings F1 and L12)
- **OVERFLOW-3P is withdrawn: the plan follows the register row.** The row OVERFLOW-ALL (`docs/adr/README.md:234`; "the
  advisor agent's rulings D1 and D1-R2 [...] never his decision; his go for this row: PR #48's merge") says: "`"*"` is
  decided, with its cost measured, when the first third-party crate enters any workspace crate's dependency tree, and
  again before such a profile moves to one of those opt-levels: a named override for each such crate (it beats `"*"`),
  or dropping `"*"` (which checks every dependency)." The draft's recommendation, to keep `"*"` as it is, was a third
  option the row does not offer. This plan takes the row's first: a named override for each third-party crate
  compiled into our libraries and tests (not proc-macro crates and build scripts, which run on the build machine
  only), with the cost read at step 4 (the Wasm's size and the step's time, with the lines and without) and reported.
  The row's trigger was first reached by ts-rs in change-set 1, as a dev-dependency, and change-set 1's report does
  not speak of it; ts-rs and thiserror are therefore on the list too (section 17, point 15). The row's other
  option, dropping `"*"`, is weighed again when the next third-party crate enters. rust-reviewer and security-auditor
  read the choice; the lines are part of step 2's manifest content.
- **SIZE-2 is information, not a question.** The stop line stays at 4,000 lines, twice the build plan's number, and
  about 2,900 is this plan's estimate, inside the build plan's own margin ("good to about half as much again either
  way", :198-200). If he wants the scope cut toward 2,000 instead, the candidates are the isolation flag, the
  allocation method, the clock fact and the hidden-tab pause (about 300 lines, moved to change-set 3).

## 17. Where the build plan is wrong or out of date for change-set 2
1. **OQ-0002-7 is placed nowhere** (Part 3.9, :762-777). Its decider, "a by-day reading of its maintenance issue
   (#4533) and the guide's governance page" (ADR 0002 :1085-1093), "comes before the step that adds wasm-bindgen"
   (EMPTY-SLOTS), and `sim-wasm`'s stub says its bindings wait for it (`crates/sim-wasm/src/lib.rs:1-3`). Step 0 does
   the reading.
2. **The project behind ADR 0004's generator crate has moved off it as a dependency** (RNG-CRATE): rand 0.10 itself
   depends on `chacha20`; one maintainer wrote that he sees no reason to maintain `rand_chacha`, and another proposed
   an empty 0.11 that points users to `chacha20` (proposed, not answered, not released: crates.io's newest is
   0.10.0). The crate still has a home in the project's `rngs` repository and the project's team among its owners.
   The card's "the seeded random streams" inherits that.
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
   `unsafe_code` exception) and OVERFLOW-PER-CRATE (followed: section 16, at its end).
9. **The planned size of 2,000 looks low** by change-set 1's measure (about 2,900 planned; the stop line stays 4,000).
10. **Part 7's "Wasm tests" row puts "the unit vectors in Wasm" at change-set 2** (:939), but Part 3.4 places the maths and
    stream vectors at change-set 3 (PM-0004-22, -27). With no maths functions and no streams in change-set 2, they start in
    change-set 3.
11. **PM-0002-30's `tsc -b` time** needs the 2.3 chat's configs, which will not be on main ("The 2.3 chat stays stopped
    for now", his words of 2026-11-17). Change-set 2 reports `--ignoreConfig` times instead, as change-set 1 did.
12. **Part 13's risk 9 and Part 15.5's version worry do not bite yet:** the newest three.js (0.187.0) equals the VFX
    runtime's only accepted version.
13. **The control calls differ in two details from ADR 0002 part 5's text** (:330, :338: the step "returns where they
    lie and when the next tick is due"): here the step returns only when the next tick is due, because the block's
    place is read once after create (the buffer never moves), and a `rebase` call is added for a pause or a stall (ADR
    0004 part 5's rule that a gap is dropped). Both stay inside PM-0002-3's count of calls per tick. For the draft's
    next revision.
14. **OVERFLOW-XCRATE-CI comes due "(or the first crate to depend on sim)"** (STATUS.md :95): `sim-wasm` depends on
    `sim` from this change-set. Its counter test is PM-0002-53's, which the build plan places at change-set 3 (:475);
    it moves there with this line, so it is not lost.
15. **The register row OVERFLOW-ALL's trigger was first reached in change-set 1** (ts-rs, a dev-dependency), and
    change-set 1's report does not speak of it. This change-set follows the row for ts-rs's crates and for
    wasm-bindgen's (section 16, at its end).

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
Since security-auditor's reading (section 7, item 5a) the final text also gets: `gh release verify-asset 0.2.130
"$RUNNER_TEMP/$wb_file" --repo wasm-bindgen/wasm-bindgen` after the sha256 check and before any run (the full check
against GitHub's key, which this container cannot make); the size check; the member list against the six expected
names; only the one binary taken out; the binary's own sha256 given to `build.mjs`. To be confirmed on the scratch
run before the text is put to him: the tarball's inner folder (`--strip-components=1`
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

## 20. What the main session changed from the draft
The draft stays beside this file, byte for byte (`cs2-first-page/plan-draft.md`), so `diff` shows every changed line.
1. **The header, section 0 and section 3:** the preconditions that are met are marked met, with their records (#60
   merged; main taken in; issue #59 read; the zip route proven by his message of 20:44:29 UTC; the session's model, the
   saver and the `advisor` agent). What holds which step is said once: steps 0 and 1 start on the advisor agent's
   APPROVE; step 2 waits for three of his answers.
2. **Section 4.4 is new:** what main's unchanged CI runs on this pull request and what that asks of the files. It was
   found by reading `toolchain.yml`: CI's `npm test --workspaces` runs the `e2e` package's `node` project, and a
   `webServer` in its config would start for that project too, on a `dist/` CI never builds. So the server is
   configured only when `SHAMBLE_DIST` is set (4.2, step 7, section 9). The same reading gives the rule that every
   file CI's `npm test` loads resolves without build output.
3. **Section 5:** the tick advances with `checked_add` (the draft left checked or wrapping open); the state hash's
   function is called provisional, with ADR 0004 part 9's own words and PM-0004-21 as the place that confirms it; the
   pointer's Rust type is settled at step 4.
4. **Section 6:** the four header-only reachability checks of 20:58 UTC.
5. **Section 7:** how the scratch worktree is cleaned (without force); the npm command names no package, because the
   manifests are edited by hand; the lockfile rule (additions and our own lines only); what security-auditor may do;
   the record is written before the manifests are committed, because the background saver also pushes.
6. **Section 8:** step 0 names who does OQ-0002-7's reading, where it is written and what stops the change-set.
7. **Section 9:** the browser commands use `/opt/pw-browsers/chromium`, `SHAMBLE_DIST` and `--trace off`; each
   package's `test` script is also run here as CI runs it.
8. **Section 10:** risk 2 names this container's intended path; risk 6 (the zip) is closed; risks 13 (the CI job's 15
   minutes) and 14 (the proxy and github.com pages) are new.
9. **Section 16:** RNG-CRATE's finding is worded from the sources as read again today: one maintainer's sentence, the
   move to the `rngs` repository, the release of 0.10.0 after it, the other maintainer's proposal and the answer to
   it. The draft's "the makers [...] see no reason to keep maintaining it" said more than the page does. DEPART-2 is
   new: the six places where this plan leaves the build plan's text, put to him as one question, because his go on
   the build plan covers only a plan that stays inside it.
10. **Section 17:** point 2 is reworded the same way. **Section 21 is new.**

11. **Revision 2** (after the advisor agent's plan review): the header; section 0 and section 3 (what holds which
    step; step 0's reading and when it ran); 4.1's interface row and 4.2's `e2e` rows (Part 15.2's folders); the
    state hash kept off the page; the lockfile rule; the tool's route (7, item 5a); steps 3, 4, 7 and 8 (the
    production check's method, the CSP scan, the pause record); the overflow overrides in the manifest text; planted
    defect 10; the hand-off readings in Brave; reviewers (netcode-architect in, art-director out with its reason);
    the Brave checklist (the pause line, two padded runs); section 16 (WB-HEALTH in; OVERFLOW-3P and SIZE-2 out, with
    the reasons); section 17, points 13 to 15; section 22.

12. **Revision 3:** RNG-CRATE's finding as the verifier corrected it (the order of the comments; the three facts
    that speak for `rand_chacha`'s home in the project; the default feature of `chacha20`; "at least twice"), and its
    recommendation narrowed to (c), with the crate asked at change-set 3's plan; TOOL-SOURCE and the tool's fuller
    route (section 7, item 5a; step 3; `checkCliFile`; planted defect 5; section 18's note).

13. **After the advisor agent read the changed lines** (2026-11-18, its notes under "Changed lines, read once"; "the
    questions may go to him: yes, after the five wording fixes"): TOOL-SOURCE says exactly what security-auditor
    found about the build, why GitHub's check cannot run here, what the count of 254 includes, and what option (c)
    costs; CI-WASM's options name GitHub's check; the overflow check's command includes the dev edges; `build.mjs`
    refuses to run the tool without its expected checksum. Its ruling: the three manifest-free parts of step 2 (the
    small server, the Worker's pure modules, the build script's pure checks) may be built now, within four limits
    (no import by package name of what step 2's manifests bring in; no use of Vite's API yet; commits by path and
    pushes only through the checkpoint script; CI's `npm test` stays green).

## 21. Records due with this change-set (beyond section 4.2's docs row)
- `docs/DECISIONS.md`, by script, never retyped: his message of 2026-11-18, 20:20:09 UTC (his go on #60, WRITE-MODE
  (a), REC-OLD (a) with one limit, the order of work), then his answers to this plan's questions as they come. His
  message of 20:44:29 UTC ("the zip saved and unpacked") is quoted by script where the open question is closed.
- STATUS, ROADMAP and the archive: #60 merged (b763ba3, 2026-11-18 20:30:46 UTC, his go); its open findings move from
  its row to "Follow-ups"; "Waiting on Mustafa" loses the zip question. Under RECORD-FILES and OLD-LINES every moved
  line is listed in the report.
- The map: the three rows for `.claude/CHECKPOINTS.md`, `.claude/scripts/checkpoint.mjs` and
  `.github/workflows/toolchain.yml` are corrected (his REC-OLD words: "The three map rows for the files this pull
  request changed are corrected in change-set 2's docs pass"); the report lists each with the old and the new text.
- The ready report quotes each release record (`dependency-check.md`) and the security answer it names (his words of
  20:20:09 UTC: "every ready report of a pull request that changes a manifest or a lockfile quotes each release record
  and the security answer it names").
- The checkpoint is folded into the report and deleted in the last commit before the pull request is marked ready
  (`.claude/CHECKPOINTS.md` §9); the report names the commit that still holds the notes.
- The report says, with its times, that step 0's reading ran before the advisor agent's APPROVE (between 21:04:49 and
  21:07:52 UTC on 2026-11-18, as reading and notes only), and that its result made WB-HEALTH.

## 22. The advisor agent's plan review, and where each condition is met
`cs2-first-page/advisor-plan.md` (Opus 5.5 at effort max; 2026-11-18, 21:05 to 21:27 UTC by its own lines): **APPROVE,
with eight conditions.** Step 1 may start; nothing of step 2 before his answers and conditions 1 to 4.

| Condition | When | Where it is met |
|---|---|---|
| 1. OVERFLOW-3P reworded, the register row quoted word for word, or the row followed with its cost measured and no question (F1) | before the questions go to him | the row is followed: section 16, at its end; the manifest text of step 2; steps 3 and 4; section 9; section 17, point 15 |
| 2. WB-HEALTH in section 16 and in "what holds which step" | before the questions | section 16; section 0; section 3, item 3 |
| 3. Part 15.2 kept, or both departures listed: the interface members (F3) and the `e2e` folders (F7) | before the questions | Part 15.2 is kept: 4.1's `picture.ts` row; 4.2's `e2e` rows; step 7; 4.4 |
| 4. The verifier spot-checks the key claims of RNG-CRATE and of the wasm-bindgen reading at their sources (F6) | before the questions | `cs2-first-page/verifier-claims.md`: FAIL on the first read (two Mediums, two Lows), all four corrected in section 16 and the reading; PASS on the re-check (21:50 UTC), its three wording Lows fixed too |
| 5. The tool's route: sha256 before unpacking, the member list, a new empty folder, the origin proven before any run, stop and ask him if it cannot be (F2) | before step 3 | section 7, item 5a; step 3. security-auditor's reading (`cs2-first-page/security-auditor-tool.md`): the origin is provable here only in part, so the condition's last clause applies and it goes to him as TOOL-SOURCE (section 16) |
| 6. The lockfile rule allows npm's flag-only changes and lists them (L4) | before step 3 | section 7, item 3 |
| 7. netcode-architect in the round; art-director called, or the reason written (F4, L9) | before the review round | section 12 |
| 8. The padded runs in the Brave checklist, and hidden-tab steps he can check (F5, L8) | before the Brave check | section 13, steps 6, 7 and 10; step 7 of section 8 (the pause record); section 9's readings |

Its Lows, each worked in or carried: L1 (step 1 "stands under every answer": sections 0 and 3); L2 (`.gitignore` by
full path: 4.2); L3 (OVERFLOW-XCRATE-CI: section 17, point 14); L5 (the server copied in the development flavour
only; the production check's method: steps 4 and 8); L6 (the CSP scan: step 4); L7 (the control calls: section 17,
point 13); L10 (the state hash off the page: section 5); L11 (PM-0002-20's bite: planted defect 10); L12 (SIZE-2 as
information: section 16, at its end). Its process note (step 0's reading before the APPROVE): section 21. Its note on
the saver (it never pushes code commits): section 7, item 5.
