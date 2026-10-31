# ADR-0002: Code structure, build and toolchain

**Status:** Proposed
**Date:** 2026-10-31
**Author:** engine-architect (the "Pass marks" table: test-engineer, plan step 4) · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no

This record decides how the v2 code is laid out, built, checked and shipped, and where the line between the Rust
simulation and the TypeScript page runs. One repository holds a Cargo workspace (the Rust crates) and npm workspaces
(the TypeScript packages, built with Vite), with every tool pinned to an exact version; the legacy game moves to
`legacy/` and stays playable by double-click; throwaway spikes live in `spikes/`. The simulation depends on nothing
above it, and a gate check proves that. In the browser the simulation runs in one Web Worker, which also runs the local
server from slice 1 and owns the network connection online; the page gets one block of data per tick and posts one
command per tick, so no call crosses the boundary per entity or per frame. TypeScript types are generated from the Rust
code. One command, the gate v2, runs the same checks locally and in CI and proves itself with planted defects; every
pull request gets a preview link. The reason: VISION_V2 asks for one simulation shared by server and client, four
desktop browsers on a modest laptop, and 48 players; the foundation the Advisor defined is exactly this structure,
and the R1 and R5 research found one way to build it that is checked by machines rather than by memory.

## Context

### Requirements
- **The foundation** ([GLOSSARY §8](../research/GLOSSARY.md#the-foundation); brief §2), quoted word for word:
  > - **The foundation.** Phase 1's work before slice 1's gameplay:
  >   - the repository restructure, moving the old game to `legacy/`;
  >   - the toolchain: a Cargo workspace plus TypeScript with Vite;
  >   - CI gate v2, and private preview links;
  >   - TypeScript types generated from the Rust code;
  >   - the client + local server skeleton: the simulation in a Web Worker, behind the same message protocol the network uses.
  >
  >   It ends when an empty scene runs through the full stack in the browser, with CI green and a preview link.
- **SLICE-ORDER** (STATUS, "Decisions pending"; brief §1): the slices 1 move & shoot → 2 bots and three teams (played
  locally) → 3 online → 4 the loop → 5 Octopon-3 and Editor v2 → 6 scale to 48, with three conditions: from slice 1
  the game runs as a client plus a local server (the simulation in a Web Worker, the network's own messages, no
  offline-only shortcuts); movement and shooting are input-driven and replayable from day one;
  [the netcode proof test](../research/GLOSSARY.md#the-netcode-proof-test) runs before the foundation.
- **SPIKE-SCOPE (a)** (STATUS, "Decisions pending"): once the drafts are approved, the legacy hard constraints apply to
  `legacy/` only, and `spikes/` is throwaway.
- **VISION_V2 §6**, the items this record serves through structure: §6.2 (a true 3D simulation, kept off the render
  thread; aiming at height crosses the boundary), §6.9 (the render thread keeps its frame budget), §6.3 and §6.7 (units,
  abilities and faction content typed once, from Rust), §6.4 and §6.8 (multi-part vehicles as records and moments as
  events in the per-tick block), §6.6 and §6.8 (re-runnable matches, which the determinism suite's tools check), §6.1
  (rooms and private matches: the crate list and the decoders that never panic).
- **VISION_V2 §3 and §4:** design for 48 players, bots fill empty slots; Chrome, Edge, Firefox and Safari; a laptop with
  integrated graphics runs the largest mode at 30+ fps on Low; fair up to about 100 ms of ping. MODES-3 (a): the
  largest mode is 24v24, so [the largest mode's N](../research/GLOSSARY.md#the-largest-mode-and-its-n) is 48.
- **TICK-30** (STATUS, "Decisions pending"): the server tick is 30 Hz at the baseline; 60 Hz only with
  [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) of at least 25 % (ADR 0001 owns it). The per-tick
  block of this record therefore arrives 30 times a second at the baseline.
- **PREVIEW-WALL (a)** (Mustafa): previews that reach [the staging game server](../research/GLOSSARY.md#the-staging-game-server)
  are behind [the access wall](../research/GLOSSARY.md#the-access-wall) from the first one. **BASELINE-WHEN (b)** (the
  Advisor): [the security baseline](../research/GLOSSARY.md#the-security-baseline) is listed open in ADR 0006 with interim
  positions.
- **EDITOR_V2 §5:** "It still runs by double-clicking editor.html. The player build (index.html) must never load editor
  code." and "Written so it ports cleanly if ADR-002 moves the project to modules."
- **LOOK_V2 §8:** "The legacy rule 'no external assets' ends with the legacy engine"; v2 may load authored glTF/GLB
  models, so the build serves model files (their pipeline is ADR 0005's).

### The findings the decision turns on
1. Since wasm-bindgen 0.2.128 the CLI and the library must be the same version ("Requires matching `wasm-bindgen` and
   CLI versions (schema bump)"), and releases broke callers in 2026 (r1-simulation-core.md, Findings, unit
   boundary-worker, R1-BND-F2, lines 894–902; relied on by R1-BND-REC4, lines 2297–2308).
2. Module workers work in all four target browsers, and both planned transports open inside a worker
   (r1-simulation-core.md, R1-BND-F25, lines 1073–1077; R1-BND-F29, lines 1098–1103).
3. Shared memory between the page and a worker needs a cross-origin-isolated page, and Safari isolates only with COEP
   `require-corp` (r1-simulation-core.md, R1-BND-F12, lines 964–970; R1-BND-F22, lines 1047–1052), so one transferred
   buffer per tick works everywhere without isolation (R1-BND-REC2, lines 2265–2285).
4. On the default Wasm target `std::thread::spawn` panics and the build uses `-Cpanic=abort`, so one panic ends the
   sim's instance (r1-simulation-core.md, R1-BND-F19, lines 1019–1025).
5. Cargo builds a dependency with the union of every feature asked for it in one build, so the server's choices can
   change the native sim (r5-toolchain-quality.md, Findings, Question 1, R5-LAYOUT-F4, lines 83–91).
6. Only clippy's correctness group denies by default, so without `-D warnings` every other lint only prints
   (r5-toolchain-quality.md, Findings, Question 3, R5-RUSTLINT-F1, lines 306–311), and `iter_over_hash_type` sees only
   `for` loops (R5-RUSTLINT-F9, lines 348–354).
7. typescript-eslint supports TypeScript `>=4.8.4 <6.1.0` and not TypeScript 7 yet (r5-toolchain-quality.md, Findings,
   Question 2, R5-TSLINT-F4, lines 192–195; R5-TSLINT-F5, lines 196–201).
8. Playwright cannot drive branded Safari (r5-toolchain-quality.md, Findings, Question 4, R5-TEST-F8, lines 454–462).
9. A job that depends on a failed job "is skipped and may not block merging", and path-filtered workflows leave
   required checks pending (r5-toolchain-quality.md, Findings, Question 5, R5-CI-F20, lines 695–700).
10. Fork pull requests get no secrets, while `pull_request_target` and `workflow_run` hand secrets and write tokens to
    runs a pull request triggers (r5-toolchain-quality.md, Findings, Question 6, R5-PREV-F1 to F3, lines 765–782).
11. Vite's size limit only warns (r5-toolchain-quality.md, Findings, Question 7, R5-GATE-F2, lines 950–953).
12. The client host serves no file larger than 25 MiB (r6-infrastructure.md, Findings §1, R6-HOST-F3, lines 72–79;
    decision map N37), and WebTransport runs only in secure contexts (r2-netcode.md, Findings, Brief question 4,
    R2-TRN-F35, lines 1268–1272).

### What this record takes from and gives to the other drafts
- **Takes:** the message protocol, the command's fields, the snapshot fields of remote entities, the interpolation
  design (its buffer in the Worker), the thin transport interface with its loopback backend for the local server, and
  the network profiles (ADR 0001); the fixed-tick mechanism, the Worker's pacing, the determinism rules, the panic-free
  sim and the restart after a trap (ADR 0004); the content schema and where content's TypeScript types come from (ADR
  0005); the frame budget rows, among them the page's cost of the sim, and the client reference machines (ADR 0003); the
  client host, its token scope, the access wall, the cleanup policy, the staging game server, the secrets policy and
  runner costs (ADR 0006).
- **Gives:** the repository, the crates and packages and the rules on who may depend on whom; the toolchains and build
  profiles; the Worker, the per-tick output block's layout and versioning, the command posting, the generated types and
  the isolation flag (decision map interfaces I-2, I-3, I-5); the client + local server skeleton; lint, tests and the
  tools of the determinism suite (I-17); CI, the gate v2 and the one file where every draft's budgets live as data
  (I-14); the preview build and its workflows (I-13); the editor build.

### What the legacy game proves (`docs/LEGACY_INVENTORY.md`)
- Today's gate is a real guard with conventions worth keeping: an explicit base ref, exit 2 for a setup problem and
  never a silent skip, `--json`, and mutation proofs: 22 of 22 planted defects failed exactly the expected checks
  (row TL1, line 386; r5-toolchain-quality.md, legacy facts, lines 534 and 938). Its checks themselves exist only for
  classic scripts (rows TL2, TL3, TL5, TL6, TL8, lines 387–393: drop).
- The player build never reaching editor code is a rule worth keeping, enforced today by heuristics (row TL4, line 389)
  and wanted by EDITOR_V2 §5.
- The smoke test's shape (state waits, console errors as failures, the renderer string on every shot, blocked network)
  carries over; its network rule flips to allow the local test server (row TL9, line 394).
- The legacy oracle runs the real code and gives reproducible dumps: the tuning dump was byte-identical over two runs
  (row TL11, line 396; r5-toolchain-quality.md, R5-TEST-REC3, lines 1444–1445).
- Exact pins, a lockfile, `npm ci` and the CI hygiene (SHA-pinned actions, read-only token, `persist-credentials:
  false`, concurrency cancel) are proven here (rows TL17 and KT8, lines 402 and 417).
- The headless "SIM fence" leaked through comments (row CS11; r1-simulation-core.md, legacy facts, lines 1176–1178),
  the frame loop stepped the world inside the render loop (rows GP13 and GP14, lines 1159–1164), and the input command
  was declared but never wired (row CS1, lines 1150–1154): the crate boundary, the Worker and the per-tick command fix
  these.
- At 48 soldiers the legacy JavaScript combat step took 0.80–1.00 ms per update step on the main thread (row GC2;
  r1-simulation-core.md, lines 1168–1171): a scale for what the Worker takes off the page, not a v2 figure.
- Two hand-synced HTML pages exist only because of classic scripts and `file://` (row PG1, line 359), and the three.js
  r128 CDN tag has no SRI or CSP (row PG19, line 377; STATUS item CDN-SRI, decision map part j, J2).

## Decision

One repository, two pinned workspaces (Cargo for Rust, npm with Vite for TypeScript), and a list of crates and packages
in which the simulation depends on nothing above it, proven by gate checks. In the browser the simulation runs in one
module Web Worker that also runs the local server and owns the network connection; the page and the Worker exchange one
command per tick one way and one transferred block per tick the other way, so nothing crosses per entity or per frame.
Types that cross the boundary are generated from Rust. One gate command runs the same checks locally and in CI and
proves itself with planted defects; budgets live as data. Every pull request from this repository gets a preview link
built from the tested artifact. Each part says what, why, its research ids and the need it serves. Lines are at commit
9e7db28.

1. **The repository after the restructure.** In the foundation's first step the five legacy game files
   (`shamble-core.js`, `shamble-game.js`, `shamble-editor.js`, `index.html`, `editor.html`) move together into
   `legacy/`, which keeps their relative paths, so `legacy/index.html` still runs by double-click; the legacy hard
   constraints then apply to `legacy/` only (SPIKE-SCOPE (a)). Beside it at the root: `Cargo.toml` (a virtual
   workspace) with `crates/*`, `package.json` (npm workspaces) with `packages/*`, `spikes/` for throwaway spikes,
   `fuzz/` (outside the workspace's default members, because cargo-fuzz needs a nightly compiler), and `docs/`.
   Nothing in `crates/` or `packages/` imports from `legacy/` (R5-TSLINT-REC2's import rule) or from `spikes/` (the
   same rule; spikes are throwaway). Today's gate and the legacy oracle (`tools/preview.js`, `sim.js`, `battery.js`,
   `tuning-dump.js`) keep working on `legacy/`, so critical fixes stay guarded and golden values can still be captured
   until legacy retires; `.gitignore` adds `target/`, `pkg/`, `dist/`, `coverage/`, `playwright-report/`,
   `test-results/` and proptest's and insta's pending files. Whether the legacy gate and tools stay in `tools/` (and
   learn the `legacy/` path) or move into `legacy/` is the phase 1 restructure plan's call (OQ-0002-2).
   - Why: one checkout, one gate and one pull request carry a boundary change on both sides, and several repositories
     would break the version matching R1 requires.
   - Ids: R5-LAYOUT-REC1 (option A), R5-LAYOUT-REC6, R5-LAYOUT-OQ5, R5-GATE-OQ3 (r5-toolchain-quality.md,
     Recommendations, Question 1, lines 1153–1169 and 1233–1245; Options, Question 1, lines 997–1002; Open questions,
     lines 1761–1762 and 1871–1872).
   - Need: GLOSSARY §8, the foundation only ("the repository restructure"); SPIKE-SCOPE (a).
2. **Toolchains, pinned exactly.** `rust-toolchain.toml` pins one exact stable Rust (1.99.1 when the research read it)
   with `clippy`, `rustfmt`, the `wasm32-unknown-unknown` target and `profile = "minimal"`; CI installs it with
   `rustup toolchain install` before any cargo call. The root `Cargo.toml` sets `resolver = "3"`, `edition = "2024"`
   through `[workspace.package]`, a `rust-version` equal to the pin and every shared dependency version in
   `[workspace.dependencies]`; `Cargo.lock` is committed and every CI cargo command takes `--locked`. npm workspaces
   with one root `package-lock.json` and exact versions; that root lockfile is the one CI's npm cache keys on
   (`cache-dependency-path`). Node is pinned the way the legacy toolkit pins 22 (row TL17). TypeScript 6.0.x, pinned
   exactly; the move to TypeScript 7 is its own change-set once typescript-eslint supports it. The wasm-bindgen CLI is
   installed at exactly the version `Cargo.lock` records (taiki-e/install-action pinned by SHA, `fallback: none`) and
   compared with the lock before bindings are generated; we use the wasm-bindgen CLI directly, not wasm-pack, because
   the CLI is what R5-CI-REC3 pins and checks, and wasm-pack sits among the rustwasm repositories that were to be
   archived or handed to their maintainers (R1-BND-F1; wasm-pack's own pages were not read). Each toolchain bump
   (Rust, clippy and rustfmt, Node, TypeScript, wasm-bindgen) is its own change-set that reports the new warnings.
   A weekly job tests the latest dependency versions without blocking (`continue-on-error`).
   - Why: one file pins local, CI and agent builds; the identical-results check between native and Wasm means
     something only when both come from one compiler; typed lint needs TypeScript below 6.1.
   - Ids: R5-LAYOUT-REC1, R5-CI-REC2, R5-CI-REC3, R5-TSLINT-REC1, R5-RUSTLINT-REC6, R1-BND-REC4 (the CLI pin),
     R5-LAYOUT-OQ1, R5-LAYOUT-OQ2, R5-XQ-OQ2, R5-TSLINT-OQ1, R5-TSLINT-OQ2, R5-CI-OQ4, R1-BND-OQ7
     (r5-toolchain-quality.md, Recommendations, lines 1153–1169, 1523–1546, 1250–1264, 1374–1381; Open questions, lines
     1750–1755, 1766–1769, 1833–1834, 1895–1898; r1-simulation-core.md, R1-BND-F1, lines 886–893; R1-BND-REC4, lines
     2297–2308; R1-BND-OQ7, lines 2741–2744).
   - Need: GLOSSARY §8, the foundation only ("the toolchain"); through R1-DET-REC6's native-against-Wasm comparison,
     VISION_V2 §6.6 and §6.8 (re-runnable matches).
3. **Crates, packages and who may depend on whom.** The crates, each entering the workspace with the first slice that
   needs it (no crate "for later"):
   | Crate | What it holds | Depends on | Enters | Need |
   |---|---|---|---|---|
   | `sim` | the simulation: state, phases, tick, the navigation runtime (ADR 0004) | `content` only (and its maths dependencies, ADR 0004); no transport, no async runtime, no clock | the foundation | §6.1–§6.5 |
   | `content` | loads and validates content data (ADR 0005) | nothing above it | slice 1 (weapons are data) | §6.3, §6.7 |
   | `protocol` | message types, the codec, decoders that never panic (ADR 0001) | `content` ids only | the foundation | §6.1 |
   | `room` | the room's match logic with no I/O and no async runtime: the slot table, bots filling slots through the input path, per-client command queues, per-client snapshot and event encoding through `protocol` (ADR 0001 decides what it does) | `sim`, `protocol`, `content` | the foundation | §6.1; SLICE-ORDER condition 1 |
   | `sim-wasm` | the wasm-bindgen bindings the Worker loads: the client endpoint (prediction, snapshot decoding, the block writer) and the local server (`room` + `sim`) | `sim`, `room`, `protocol`, `content` | the foundation | §6.2, §6.9 |
   | `server` | the room process: transport on quinn and the WebSocket fallback, the async runtime, the supervisor hook, metrics (ADR 0001, ADR 0006) | `room`, `protocol`, `sim`, `content` | slice 3 (online) | §6.1, §6.5, §6.6, §6.8 |
   | `devtools` | the replay runner, the hash comparer, the tick bench, the legacy-golden reader | anything | the foundation | §6.6, §6.8 |
   | `navbake` | the navigation bake (ADR 0004's generator), run by CI, on map save and in the editor's Test | `sim`, `content` | slice 2 (bots) | §6.2 (bots between levels) |
   The packages: `client` (the player entry: input, camera, HUD, rendering), `editor` (its own entry), `sim-worker`
   (hosts `sim-wasm`, paces the tick, runs the local match), `net` (the client side of ADR 0001's thin transport
   interface: WebTransport, WebSocket and the loopback backend), `types` (generated types, part 5) and `e2e`
   (Playwright tests). The lobby and the room supervisor get a crate when ADR 0006 decides where the lobby runs
   (OQ-0006-3: the supervisor's binary or its own service); until then no crate is created for them.
   The rules, each a gate check (G5, G8, part 9): `sim` depends on `content` only, with no transport, no async runtime
   and no clock, checked with cargo-deny's `deny` plus `wrappers` and with `cargo tree`; `room` builds for
   `wasm32-unknown-unknown` with no async runtime and no transport crate (the same two tools); `content` depends on
   nothing above it; `sim-wasm` never depends on `navbake` or `server`, so the player's Wasm carries neither the bake
   nor the server's I/O (`cargo tree` on the `sim-wasm` build); `client` never reaches `editor`, checked twice: an
   ESLint `no-restricted-imports` rule and a walk of Vite's manifest from the player entry through `imports` and
   `dynamicImports`; `sim-worker` and `net` never import rendering or DOM packages; nothing imports `legacy/` or
   `spikes/`. `sim` is built with one feature set on every target: it exposes no feature that changes maths, the
   maths-relevant features of the dependencies it shares with `server` are set once in `[workspace.dependencies]`,
   and the gate compares the resolved feature list of every `sim` dependency (`cargo tree -e features`) between the
   `server` build and the `sim-wasm` build.
   - Why: a mechanical check is the only kind that holds the boundaries R1 and R2 ask for (the legacy fence leaked,
     row CS11); Cargo unifies features over everything built together (R5-LAYOUT-F4), so the server could change the
     native sim's maths.
   - Changed from R5-LAYOUT-REC2 (r5 lines 1170–1193): two crates are added and none removed. `room`, because
     SLICE-ORDER condition 1 runs the local server in the Worker, and the Worker cannot carry the server's async
     runtime and transport (on the default Wasm target `std::thread::spawn` panics: R1-BND-F19, r1 lines 1019–1025;
     the simulation depends on no I/O: R2-TRN-REC4, r2-netcode.md lines 2673–2697); R5-LAYOUT-REC2 predates
     SLICE-ORDER and puts rooms inside `server`. `navbake`, because where the bake lives decides whether the generator
     ships in the player's Wasm (the critic's R4-XT-OQ5, r4-physics-navigation.md lines 1195–1200), and the bake runs in
     CI, on map save and in the editor's Test (R4-NAV-REC1, r4 lines 802–810), never in prediction (R4-NAV-REC4, r4
     lines 888–899). Both placements answer questions the critic gave this record; no research compared them with
     other placements.
   - The cache question (critic rank 40) is answered by this list: no ECS crate and parry3d rather than Rapier (ADR
     0004, part 1), quinn and the WebTransport crate the netcode proof test picks (ADR 0001, part 10); the cache is
     measured on this workspace against R5-CI-REC1's 8 GB mark in the foundation (part 9).
   - Ids: R5-LAYOUT-REC2 (changed), R5-LAYOUT-REC4, R5-LAYOUT-OQ3, R5-XT-OQ7, R5-XT-OQ14, R1-XT-OQ18, R4-XT-OQ5,
     R4-XT-OQ8 (r5-toolchain-quality.md, Recommendations, lines 1170–1218; Open questions, lines 1756–1758, 2002–2007,
     2033–2036; r1-simulation-core.md lines 3109–3112; r4-physics-navigation.md lines 1195–1200 and 1211–1214).
   - Need: as the table says per crate (R5-LAYOUT-REC2's own "Serves", r5 lines 1171–1176); the dependency rules:
     GLOSSARY §8, the foundation; the one feature set: VISION_V2 §4 (prediction needs one sim on client and server, as
     R5-LAYOUT-REC4 traces it, r5 lines 1213–1214), §6.6 and §6.8.
4. **Build profiles, overflow and panics.** Overflow checks are on for `sim` and `protocol` in every profile
   (`[profile.release.package.sim] overflow-checks = true`, the same for `protocol`), and counters and tick numbers
   use explicit `wrapping_*` or `checked_*` operations, so a debug client and a release server never disagree on
   overflow. The panic strategy: the server's room process keeps Rust's default `panic = "unwind"`, with each
   connection's work isolated from the room's tick and decoders that never panic (R2-ROOM-REC2); the Wasm build keeps
   the target's default `-Cpanic=abort` (R1-BND-F19), so a trap ends the sim's instance, and the restart after a trap
   is ADR 0004's (part 20) with ADR 0001's resync path. The server is built for baseline x86-64, never with `+fma`,
   `-C target-cpu=native` or the `relaxed-simd` target feature: the gate reads the build flags of `sim`, `room`,
   `sim-wasm` and `server` and fails if one appears (the rule is ADR 0004's, part 9; the check is this record's).
   `unsafe_code` is denied in every crate; an exception needs a written invariant, a test and an `#[allow]` with its
   reason. The Wasm size budget has no research number: the boundary spike and the sim core spike measure the
   `sim-wasm` file (raw and gzip, after wasm-opt and without it, `opt-level` "s" against "z", with parry3d), and the
   budget file (part 9) takes the measured value as its first limit; every served file stays under the client host's
   25 MiB per-file limit.
   - Why: Cargo's dev and release profiles differ on overflow by default (R5-RUSTLINT-F13), and one remote panic must not
     end a 48-player match.
   - Ids: R5-RUSTLINT-REC4, R1-RR-OQ4, R1-RR-OQ5, R5-RUSTLINT-OQ4 (r5-toolchain-quality.md, Recommendations, Question 3,
     lines 1353–1364; Open questions, line 1793; r1-simulation-core.md, Open questions, lines 2941–2950); cites
     R2-ROOM-REC2 (r2-netcode.md, rooms recommendations, the fix-pass note on `panic = "unwind"`), R1-RR-OQ3 and
     R1-XT-OQ12 (ADR 0004's), R6-HOST-REC1's 25 MiB mark (decision map N37, r6 line 642).
   - Need: VISION_V2 §4 (client and server must not disagree on a tick, as R5-RUSTLINT-REC4 traces it, r5 lines
     1360–1361), §6.6 and §6.8; the panic strategy: §6.1 (rooms stay up, R2-ROOM-REC2's "Serves").
5. **The Rust ↔ TypeScript boundary.**
   - **One dedicated module Worker** runs the client's sim, single-threaded, on stable Rust (options A2 and E1). The page
     keeps input listeners, the camera, the HUD and rendering, never instantiates Wasm and makes zero calls into it.
   - **One flat, versioned output block per tick** (option B2): written by Rust into preallocated linear memory, copied
     once into a pooled `ArrayBuffer` and transferred to the page. It holds a header (tick, layout hash, counts per
     entity kind), aligned sections per entity kind (soldiers, vehicle parts, projectiles, structures, weapon pickups,
     which are sim entities under UPGRADE-SCOPE, ADR 0004 part 18), one section of owner-only data for the local player
     (own weapon state, money, upgrade tracks and offers: decision map I-21) and the event section for the tick (hits,
     deaths, blasts, cosmetic shot events, purchases, captures, pick results, drops, moments: ADR 0004 part 19 emits
     them, ADR 0001 sets their visibility). Online, remote entities are written at the interpolation delay from the
     Worker's interpolation buffer and the local player as predicted (ADR 0001 parts 4, 6, 7); the fields of remote
     entities are ADR 0001's snapshot fields in this record's layout. The page binds typed-array views on the block
     with no copy, keeps the two newest blocks to interpolate between ticks (R1-TICK-REC5) and returns older ones to
     the pool. Whether a section is laid out per entity or per field is measured in the boundary spike (bytes per tick
     and copy time at N = 48); ADR 0003 reads typed arrays either way. The block writer serializes field by field, as
     the state hash already does (R1-DET-REC6), so struct padding never reaches the page and no `unsafe` is needed; if
     the spike's copy time says otherwise, R1-RR-OQ6's two alternatives (a derive that rejects padding, or `unsafe` with
     a written invariant and a test) are weighed then. The sharedArrayBuffer ring (B3) is the spike's measured
     alternative, never the sim's own shared Wasm memory (B4). At the 30 Hz baseline that is 30 blocks a second
     (one per tick).
   - **One command per tick, posted** from the page to the Worker (option C1): the input fields ADR 0001 defines (seq,
     move, aim, the held-button bitmask, press and release edge counters), plus the camera's cursor ray and the view time
     the page drew, so the Worker can cast the aim ray against the state it published (ADR 0001 part 5; decision map
     I-19); one message, no extra one for the aim. The Worker stamps the tick it applies the command to.
   - **The small control API** (create, mode flag, load the content bundle, step, the block's pointer and length) is
     wasm-bindgen's own, with its `.d.ts` (option D1); it is called a constant number of times per tick, never per
     entity.
   - **Generated types, from three sources, into one committed package** `packages/types`: wasm-bindgen's `.d.ts` and
     glue are build output (ignored); our own small generator writes the block's layout (offsets, strides, element
     types) and its layout hash, which the page checks at startup (option D4); ts-rs writes the protocol and message
     types into `packages/types/src/generated/` on `cargo test`, through `TS_RS_EXPORT_DIR` in `.cargo/config.toml`
     (option D3); content's types land in the same package, from the source ADR 0005 picks (critic rank 31). Not tsify.
     The gate regenerates all of them and fails on any difference or hand edit (G4).
   - **Cross-origin isolation is off** for v2's start (option G1). [The isolation flag](../research/GLOSSARY.md#preview-isolation-and-the-isolation-flag)
     adds COOP `same-origin` and COEP `require-corp` only to profiling builds, locally or on
     [a preview](../research/GLOSSARY.md#preview) built with it on (option G3); the preview Mustafa checks is built
     with it off. Production turns isolation on only if the boundary spike's B2-against-B3 test passes R1-BND-REC2's
     mark 5, or if the Worker's timer pacing (W2, ADR 0004 part 5) fails its jitter mark and the `Atomics.wait`
     alternative (W3), which needs shared memory, passes it; then the default flips everywhere at once, B3 and posted
     inputs through shared memory (C2) become available, and the cost list of R1-BND-REC5 applies (OQ-0002-4).
   - **The Worker owns the network connection** online (option H2): server snapshots, prediction and reconciliation
     never touch the page, which sees only the block. What flows over the connection is ADR 0001's.
   - Why: the tick leaves the render thread; one bounded copy per tick needs neither isolation nor nightly Rust; one
     small message per tick is constant work; the block's layout is the only thing tsify and ts-rs cannot type.
   - Ids: R1-BND-REC1 to R1-BND-REC6, R5-LAYOUT-REC3, R5-LAYOUT-REC5, R1-XU-OQ3, R1-XU-OQ4, R1-BND-OQ4, R1-BND-OQ6,
     R1-BND-OQ9, R1-RR-OQ6, R3-MEASURE-OQ4, R5-XQ-OQ3 (r1-simulation-core.md, Recommendations, unit boundary-worker,
     lines 2252–2338; Options, lines 1731–1788; Open questions, lines 2732–2733, 2737–2740, 2747–2752, 2854–2865,
     2951–2954; r5-toolchain-quality.md, lines 1194–1205, 1219–1232, 1899–1910; r3-rendering.md, R3-MEASURE-OQ4, lines
     1389–1391).
   - Need: VISION_V2 §6.2 (the sim off the render thread; aiming at height in the command), §6.9 (the render thread
     keeps its budget), §4 (four browsers; 30+ fps on the minimum laptop), as R1-BND-REC1 traces them (r1 lines
     2255–2257); the block's sections: §6.4 (multi-part vehicles as records) and §6.8 (moments as events)
     (R1-BND-REC2, r1 line 2275); the pickups and owner-only sections: UPGRADE-SCOPE; generated types: §6.3 and §6.7
     (R1-BND-REC4, r1 lines 2303–2304).
6. **The client + local server skeleton.** From slice 1 the same `sim-worker` build runs in two modes behind one mode
   flag, with the same page code: online, the Worker's client endpoint talks to the room process through `net`'s
   WebTransport or WebSocket backend; in a local match, the same Worker also runs the local server (`room` and `sim`,
   the authority, with bots filling slots), and the client endpoint reaches it through `net`'s loopback backend with
   the same encoded messages (ADR 0001 part 10). The page never handles a network packet and cannot tell the two modes
   apart. Whether the local server and the client endpoint share one Wasm instance or run as two, and so whether the
   loopback stays inside Rust or passes through the TypeScript interface, is measured in the boundary spike
   (OQ-0002-3). The Worker's host (TypeScript) receives the page's commands, paces the tick (ADR 0004 part 5), steps
   the instance or instances, and posts each block; the commands it forwards pass through it, outside the Wasm
   instance, which is what OQ-0004-7's recovery options need to know. The foundation's skeleton ends with an empty
   scene drawn from blocks that came through this whole path (Migration plan, phase 2).
   - Why: SLICE-ORDER condition 1 asks for exactly this, and one boundary then serves local play, prediction and
     online play, so no code path exists only offline.
   - Ids: R1-BND-REC6, R1-BND-REC1 (the Worker), R5-LAYOUT-REC2 (the packages); cites R2-TRN-REC4 as ADR 0001 changes
     it (the loopback backend), R1-BND-OQ1, R1-XT-OQ8 and R5-XT-OQ4 (offline play as a shipped feature: a design
     question, decision map S11).
   - Need: SLICE-ORDER condition 1; GLOSSARY §8, the foundation ("the client + local server skeleton"); VISION_V2 §3
     ("Bots fill empty slots") and §4 (prediction is not delayed by a busy frame), as R1-BND-REC6 traces them (r1
     lines 2331–2335).
