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
     the simulation depends on no I/O: R2-TRN-REC4, r2-netcode.md lines 2673–2698); R5-LAYOUT-REC2 predates
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
   - Need: as the table says per crate (R5-LAYOUT-REC2's own "Serves", r5 lines 1171–1176 and 1188); the dependency rules:
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
     R2-ROOM-REC2 (r2-netcode.md, Recommendations, Brief question 5, lines 2762–2768, its fix-pass note on `panic = "unwind"`), R1-RR-OQ3 and
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
     a written invariant and a test) are weighed then. The SharedArrayBuffer ring (B3) is the spike's measured
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
     2256–2258); the block's sections: §6.4 (multi-part vehicles as records) and §6.8 (moments as events)
     (R1-BND-REC2, r1 line 2275); the pickups and owner-only sections: UPGRADE-SCOPE; generated types: §6.3 and §6.7
     (R1-BND-REC4, r1 lines 2302–2303).
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
     lines 2330–2332).
7. **Lint and format.**
   - **TypeScript:** one shared strict base `tsconfig` (`strict: true` written out, `noUncheckedIndexedAccess`,
     `exactOptionalPropertyTypes`, `noImplicitOverride`, `noFallthroughCasesInSwitch`, `noImplicitReturns`,
     `isolatedModules`, `verbatimModuleSyntax`, `erasableSyntaxOnly`, and each package's `lib` and `types` set
     explicitly); every package extends it and is a project reference built with `tsc -b`, beside every Vite build
     (Vite does not type-check). ESLint 10 flat config with typescript-eslint `strict-type-checked` plus
     `stylistic-type-checked`, pinned exactly, typed through `projectService: true`, `--max-warnings 0`, plus four
     project rules: `switch-exhaustiveness-check` with `considerDefaultExhaustiveForUnions: false` (a missed generated
     message kind fails), `no-restricted-imports` for part 3's layering rules, `no-restricted-globals` with a reviewed
     list (legacy's TL6 lesson: `event`, `name`, `status` and the rest), and `no-undef` off. Prettier formats
     `packages/**` and the root config files (not `docs/`, not `legacy/`), in check mode in the gate. The rule
     "dynamic or player-provided text: `textContent`, never `innerHTML`" holds by review until the security baseline
     names its lint rule (ADR 0006, part 13, item B6).
   - **Rust:** one workspace lint table every crate opts into (`[lints] workspace = true`, itself a gate check):
     `unsafe_code = "deny"`, clippy's `all` at warn, and the gate runs `cargo clippy --workspace --all-targets --locked
     -- -D warnings` once for the host and once for `wasm32-unknown-unknown`; `pedantic` at warn on `sim`, `protocol`
     and `content` only. [The hash-map ban](../research/GLOSSARY.md#the-hash-map-ban-in-the-sim-crate) and the clock and
     float bans live in `crates/sim/clippy.toml` (`disallowed-types`: std `HashMap`, `HashSet`, `Instant`,
     `SystemTime`; `disallowed-methods`: `Instant::now`, `SystemTime::now`, std's transcendental float methods and the
     `algebraic_*` methods), at deny, with `iter_over_hash_type` as a backstop; `suboptimal_flops` and
     `imprecise_flops` are allowed with a comment, and `cargo clippy --fix` never runs on `sim`. The only escape from
     the ban is a reviewed `#[allow]` with a written reason for a map that is only looked up and lies outside the
     serialized sim state (resolution 29). Map types from the sim's dependencies join `disallowed-types` from a list
     rust-reviewer makes from `cargo tree` of `sim` once ADR 0004's dependencies are fixed. If clippy cannot name std's
     float methods, a reviewed text scan of `sim` for those method names is the fallback check (R5-RUSTLINT-OQ1). In
     `protocol`'s decoders and `content`'s loader, `unwrap_used`, `expect_used` and `indexing_slicing` are at deny,
     allowed in tests; the sim does not enable them, since its indices come from its own pools. rustfmt runs with stable
     options only (`edition` and `style_edition` 2024, `newline_style = "Unix"`), in check mode in the gate. The
     `panic` and `arithmetic_side_effects` lints are not added: their sources were not read.
   - **The per-edit hook** checks only what changed (`tsc -b` on the touched project and ESLint on the file; `cargo
     check -p` on the crate), exits 2 on failure and stays fast; the full gate is the only judge. It is a `.claude/`
     change, so it lands in a kit change-set in phase 1 (nothing in `.claude/` changes in this PR).
   - Why: strict settings catch the bugs the boundary invites (an unhandled message kind, an unchecked index, an
     unawaited transport promise); only correctness lints deny by default; the hash-map type ban catches the iterator
     chains `iter_over_hash_type` misses.
   - Ids: R5-TSLINT-REC1 to R5-TSLINT-REC4, R5-RUSTLINT-REC1 to R5-RUSTLINT-REC3, R5-RUSTLINT-REC5, R5-RUSTLINT-OQ1,
     R5-RUSTLINT-OQ2, R5-RUSTLINT-OQ3, R5-RUSTLINT-OQ5, R5-TSLINT-OQ3, R5-TSLINT-OQ4, R5-TSLINT-OQ6
     (r5-toolchain-quality.md, Recommendations, lines 1250–1373; Open questions, lines 1770–1781, 1784–1799); cites
     R1-DET-REC2, R1-DET-REC5, R1-ECS-REC2 (ADR 0004's rules) and R5-TSLINT-OQ5 (ADR 0006's, the security baseline).
   - Need: GLOSSARY §8, the foundation only ("the toolchain"); the determinism lint set: VISION_V2 §4, §6.6 and §6.8
     (as R5-RUSTLINT-REC2 traces it, r5 lines 1334–1335); the panic lints: §6.1 (R5-RUSTLINT-REC3, r5 line 1350); the
     exhaustive switch: §6.1 (R5-TSLINT-REC2, r5 line 1277).
8. **Tests, and the tools behind ADR 0004's determinism suite.**
   | Layer | Tool | Proves | Runs |
   |---|---|---|---|
   | Rust unit and property | `cargo test`, proptest 1.x | sim rules, maths, decoders, content validation | gate (G9) |
   | Rust in Wasm | wasm-bindgen-test in Node | the same unit tests compiled to Wasm | gate (G9) |
   | The boundary | Vitest (Node) loading the built `sim-wasm` | the layout hash; every message type round-trips between Rust encoders and TypeScript decoders | gate (G9) |
   | TypeScript unit | Vitest (Node) | input mapping, interpolation between blocks, HUD state | gate (G9) |
   | Browser end-to-end | Playwright Test (Chromium, Firefox, WebKit) with the built client and a local server (`webServer`) | TL9's smoke shape, both transport backends, the backend string, ADR 0003's render counts | gate (G12) |
   | Real Safari | the determinism replay through safaridriver or WebdriverIO on macOS; the end-to-end suite in WebKit on macOS | what WebKit on Linux cannot show | OQ-0002-5 |
   | Legacy golden | `cargo test` on committed fixtures captured from the legacy oracle | kept behaviour equals legacy within each fixture's tolerance | gate (G9) |
   | Determinism | the `devtools` replay runner (below) | identical per-tick hashes on every target | gate (G10, short), nightly (long) |
   | Network | L1 (gate, short; nightly, long), L2 (nightly) | ADR 0001's marks under named profiles | gate (G11), nightly |
   | Soak and load | bots as headless Rust protocol clients | rooms of 48 for 30 minutes per mode | nightly, and before each playtest |
   | Fuzz | cargo-fuzz on every decoder and the content loader | 0 panics, 0 allocations above the size limits | nightly, Linux |
   | Benchmarks | the `devtools` tick bench; ADR 0003's client harness | sim step time and frame cost, with the method stated | change-sets touching the sim or the render path; never a gate pass or fail |
   - **The determinism suite's tools:** a `devtools` replay runner, built as the server builds the sim, plays a
     recorded match and writes one [state hash](../research/GLOSSARY.md#state-hash) per tick; the gate runs it twice in
     two processes and once as Wasm in Node; Playwright opens a replay page that runs the `sim-wasm` Worker in Chromium,
     Firefox and WebKit and returns the hash stream; a comparer reports the first mismatching tick and a field-level
     diff. The gate's set is three short recordings (two teams, three teams, bots only); nightly adds a 10-minute bot
     match at the largest mode's N (48) and every recording added since. The rules and the recordings are ADR 0004's.
   - **Golden tests against legacy:** one fixture per kept behaviour, captured once by a harness snippet on the real
     legacy build (`node tools/preview.js legacy eval <snippet>`), holding the legacy commit, the inventory row id, the
     inputs, the outputs and the tolerance; only rows marked "rewrite in Rust" or "port to TypeScript" whose rule
     LOOP_V2 keeps (the ray maths CS4–CS6, shot resolution and locational damage CS7 and CD8, the imported tuning
     tables, presence counting GE8, the animation port's GA5 and GA8 metrics); never a random-AI statistic or a legacy
     artefact such as the frame-rate-dependent jump (GW14; which jump v2 keeps is OQ-0002-6). v2's own outputs use insta
     snapshots, which CI never rewrites.
   - **The network simulator** in two layers, with ADR 0001's profiles P0–P4 and B1 by name: L1, our seeded link
     simulator on simulated time behind the thin transport interface (per direction, half the profile's round trip each
     way, proposed by R5; latency, jitter, loss, reordering, duplication, a rate limit, and a packet-fate log), running
     the real server code and headless bot clients in one process; L2, an impairment wrapper around the server's UDP
     socket under the real quinn endpoint for real browsers over WebTransport, with a delaying TCP proxy for WebSocket.
   - **Soak and load:** bots as headless Rust clients that speak the real protocol; one nightly run per mode of
     [the test-mode list](../research/GLOSSARY.md#the-test-mode-list) over L1 at P2, 30 minutes each, and one L2 run with
     4 real browser clients among the bots; measured: [server tick time](../research/GLOSSARY.md#server-tick-time) p50,
     p99 and max against [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) (ADR 0001's), bytes per
     client per second, the resident-memory slope, queue lengths, desyncs, panics and hangs, plus legacy TL13's match
     metrics; before each playtest one 3-hour soak.
   - **Flakes:** no retries (`retry: 0`, `retries: 0` written out; `failOnFlakyTests` and `forbidOnly` on in CI); one
     committed fixed `PROPTEST_RNG_SEED` in the gate, fresh seeds nightly with every failure committed to
     `proptest-regressions` with its fix; simulated time only in sim and network tests; browser tests wait on state
     through a test-hook API and drive page time with Playwright's clock; no shared global state in Rust tests; every
     failure keeps its seed, recording and trace as CI artefacts and gets an issue.
   - **The bench method** (critic rank 36): the tick bench is a mode of the `devtools` replay runner. It replays a
     recording of [the stress scene](../research/GLOSSARY.md#the-stress-scene), times every sim step and every phase
     with the host's monotonic clock outside the sim (the sim itself reads no clock), and reports mean, p50, p95, p99 and
     max per phase and in total over 3 runs per build, on
     [one vCPU](../research/GLOSSARY.md#the-measuring-unit-one-vcpu) of [the reference server](../research/GLOSSARY.md#the-reference-server)
     natively and in Wasm in the Worker on the minimum laptop, as R1-TICK-REC2's P1 and P2 ask. It reports; it never
     passes or fails the gate. No bench crate, coverage tool or cargo-nextest is adopted: none was read.
   - Why: each tool is the option the research read for its layer; a retry or a random seed hides a defect; timings on
     shared CI runners are not hardware results.
   - Ids: R5-TEST-REC1 to R5-TEST-REC6, R5-TEST-OQ1, R5-TEST-OQ2, R5-TEST-OQ3, R5-TEST-OQ5, R5-TEST-OQ6, R5-TEST-OQ7,
     R5-TEST-OQ8, R5-XT-OQ13, R1-XT-OQ15 (r5-toolchain-quality.md, Recommendations, Question 4, lines 1386–1505; Open
     questions, lines 1802–1820, 2030–2032; r1-simulation-core.md lines 3097–3100); the method follows R1-TICK-REC2
     (r1-simulation-core.md, lines 2044–2057).
   - Need: VISION_V2 §6.1–§6.9 through their tests and §4 (four desktop browsers), as R5-TEST-REC1 traces them (r5
     line 1406); the determinism tools: §4, §6.6 and §6.8 (R5-TEST-REC2, r5 lines 1425–1426); soak and load:
     §3, §6.1, §6.6 (R5-TEST-REC5, r5 line 1486).
9. **CI and the gate v2.** One Node command, the same locally and in CI (working name `node tools/gate.mjs`, v2
   edition; its path is OQ-0002-2), carrying today's conventions: the base-ref rule, exit codes 0, 1 and 2, `--json`,
   `--allow-removed` (exported Rust items and TypeScript exports the approved plan removes), `--quick` (G0–G5 and the
   tests of changed crates and packages, for local use between steps, never for a commit) and `--prove`. Its checks,
   fastest first:
   | Id | Stage | Check |
   |---|---|---|
   | G0 | setup | toolchain equals `rust-toolchain.toml`; Node equals the pin; `cargo --locked` and `npm ci` succeed; exit 2 on any setup failure |
   | G1 | format | `cargo fmt --all --check`; `prettier --check` on `packages/**` |
   | G2 | Rust lint | clippy `-D warnings` on the host and on `wasm32-unknown-unknown`; the determinism and panic lint sets; every crate opts into the workspace lints; the build-flag check of part 4 |
   | G3 | TypeScript | `tsc -b`; ESLint typed lint with `--max-warnings 0` |
   | G4 | boundary | generated types, layout and schema regenerate with zero diff; the wasm-bindgen CLI equals the library in `Cargo.lock` |
   | G5 | dependency rules | part 3's rules (cargo-deny bans and `cargo tree`); `multiple-versions` and `wildcards` at deny with a reviewed skip list; the same-feature check for `sim` |
   | G6 | build | native release (`server` once it exists, `devtools`); `sim-wasm` to Wasm; Vite build of both entries |
   | G7 | budgets | Wasm bytes (raw and gzip) and JS bytes per entry against the committed budget file; fail above it |
   | G8 | player build | walk Vite's manifest from the player entry: no chunk from `editor` |
   | G9 | tests | `cargo test --workspace` (unit, property with the fixed seed, legacy goldens, insta), Wasm tests in Node, Vitest |
   | G10 | determinism | the short recordings: native in two processes, Wasm in Node, Wasm in Chromium, Firefox and WebKit; first mismatch reported |
   | G11 | network | L1, one minute per profile P0–P4 and B1, with ADR 0001's marks that fit a minute |
   | G12 | end-to-end | Playwright on the built client and a local server: TL9's smoke shape, both transport backends, the backend string on every shot, ADR 0003's render counts against their committed baseline; every outbound request fails the run except to the local test server |
   | G13 | maps | for every committed map document: the navigation bake succeeds and ADR 0004's map checks pass (reachability, the 6 m gap rule) |
   - **`--prove`:** a committed mutation suite applies each planted defect in a throwaway worktree and expects exactly
     the named checks to fail and every other check to pass, with at least one mutation and one control per check id
     (R5-GATE-REC3's starting set, plus G13: a map with an unreachable spawn fails G13; a valid map passes). Every
     change to the gate, its checks or its budget file runs `--prove` and reports the table; the nightly run does too.
     cargo-mutants on `sim` and `protocol` reports test strength and never decides the gate.
   - **Budgets as data:** one committed file holds the size budgets, ADR 0003's count baselines and their 10 % limits,
     the gate's time budgets, the proptest case counts and every other draft's budget rows (decision map I-14: each
     number's owner fills its row); raising a budget is a reviewed change with its reason, like `gate-globals.json`
     today. Starting values come from the first spikes' measurements, not from guesses.
   - **CI (option G1):** one workflow on push and `pull_request` runs parallel jobs (Rust native: G1, G2, Rust tests
     and the hash-map ban; Rust wasm32: build, wasm-bindgen, G4; TypeScript: G3 and Vitest; Playwright end-to-end; and
     determinism), and one final job is the only required check: it runs whatever the other jobs did (`if: always()`,
     marked inferred in R5), fails unless every job it needs succeeded (a skipped job counts as a failure until a
     deliberate skip is allowed, which R5-GATE-OQ1 leaves to the first month's CI times), and compares the set of check ids that ran
     with the gate's own list, so a check cannot drop out of CI silently. No path filters. Every check runs on every
     pull request until the first month's CI times say otherwise. The preview deploy and the Safari run are not among
     the required jobs.
   - **Caching:** Swatinem/rust-cache v2.9.3 pinned by SHA, one `shared-key` per build kind (native, wasm32), saved only
     from pushes to `main`, restored by pull requests; no sccache at the start; setup-node with `cache: npm` on the root
     lockfile and `npm ci --ignore-scripts`; Playwright's browsers installed in each run, not cached; no automatic
     package caching in any job that holds a secret.
   - **Hygiene,** as today's CI: actions pinned by SHA, `contents: read`, `persist-credentials: false`, concurrency
     cancel, per-job timeouts; Dependabot for `github-actions` (whether it updates SHA pins is tried on one action in the
     foundation); workflow lockfiles wait until they are generally available on Mustafa's plan.
   - **Scheduled, not in the pull-request gate** (each failure opens an issue): soak and load, fuzzing, proptest with
     fresh seeds, cargo-mutants, long determinism recordings, the browsers' beta channels (ADR 0001's recurring
     transport run), the latest-dependencies job, and [the advisories check](../research/GLOSSARY.md#the-advisories-check),
     which also runs on every pull request that changes `Cargo.lock`; whether an advisory blocks a pull request is the
     security baseline's (BASELINE-WHEN (b); ADR 0006, part 13, item B1).
   - **WebGPU in CI:** standard runners have no GPU and headless Chromium renders on SwiftShader, so CI gates only
     counts (ADR 0003, R3-MEASURE-REC3) on whatever backend each runner gives, prints that backend on every shot, and
     runs the end-to-end suite once more with three.js's `forceWebGL` so the WebGL 2 fallback is always covered; frame
     and GPU times come only from the reference machines (ADR 0003). No GPU runner is proposed (a paid larger runner;
     ADR 0006 and Mustafa for any paid plan).
   - **No arm64 leg** in the determinism job: the first fleet is x86-64 (ADR 0006, part 5), and native AArch64 joins the
     hash test before ADR 0006 may pick Arm hosts (ADR 0004).
   - **Crash reporting and the end-to-end network rule** (critic rank 34): test builds carry no crash-reporting
     endpoint, so G12's rule (only the local test server) holds unchanged; the optional Sentry check of a planted Wasm
     trap runs in the boundary spike, not in the gate. The job that uploads debug files for Sentry (R6-OBS-REC3) runs
     only on builds that are deployed, in its own GitHub environment that holds the Sentry token, with no checkout of
     pull-request code, shaped like the preview deploy job (part 10); its token scope is ADR 0006's (part 13, item B9).
   - Why: one command keeps the conventions that make today's gate a real guard; one aggregate required check survives
     jobs being added or renamed, and a skipped dependent job could otherwise let a failing pull request merge;
     caching from `main` keeps fork-written caches out of what other runs restore.
   - Changed from R5-GATE-REC1 (r5 lines 1655–1682): G13 is added. R4's bake and map checks are not in R5's gate
     (R4-XT-OQ5), and the bake runs in CI by R4-NAV-REC1; the check follows R5-GATE-REC1's own rule that each check
     enforces one recommendation (here R4-NAV-REC1 and ADR 0004's map checks).
   - Ids: R5-GATE-REC1 (changed), R5-GATE-REC2, R5-GATE-REC3, R5-GATE-REC4, R5-CI-REC1, R5-CI-REC4, R5-CI-REC5,
     R5-GATE-OQ1, R5-GATE-OQ2, R5-GATE-OQ5, R5-CI-OQ1, R5-CI-OQ2, R5-CI-OQ3, R5-CI-OQ5, R5-CI-OQ6, R5-CI-OQ7, R5-XT-OQ12,
     R6-XT-OQ8 (r5-toolchain-quality.md, Recommendations, lines 1510–1522, 1547–1583, 1655–1744; Open questions, lines
     1825–1841, 1865–1870, 1877–1878, 2026–2029; r6-infrastructure.md, R6-XT-OQ8, lines 1030–1034).
   - Need: GLOSSARY §8, the foundation only ("CI gate v2"); G10: VISION_V2 §4, §6.6, §6.8; G11: §4's fairness target
     (R5-GATE-REC1's "Serves", r5 lines 1679–1680); G13: §6.2 (bots between levels) and EDITOR_V2 §3 (the validation
     panel's reachability and 6 m gap rule).
10. **The preview build.** Shape P1: the ordinary `pull_request` workflow builds with no secret and uploads `dist` as
    an artifact (kept 7 days, the retention ADR 0006 sets); a separate deploy job (`needs: build`) runs in a GitHub
    environment named `preview`, holds the only client-host token, never checks out or runs pull-request code, runs
    only for pull requests from this repository, has the permissions `contents: read`, `actions: read`,
    `deployments: write` and `pull-requests: write`, turns automatic package caching off, sets the environment's URL to
    the preview link and posts or updates one comment with it. Fork pull requests get the full CI and no preview. No
    `pull_request_target`, no `workflow_run`. The build generates the `_headers` file for
    [the client host](../research/GLOSSARY.md#the-client-host): [the production header set](../research/GLOSSARY.md#the-production-header-set-and-the-preview-header-set)
    for production and the preview header set for previews (`X-Robots-Tag: noindex`), with COOP and COEP only in a
    build made with the isolation flag on; after deploying, the job fetches the preview's HTML and `.wasm` and fails if
    a header is missing. The close job and the weekly sweep that delete previews are workflows here; the policy (24 h,
    7 days) and PREV-DELETE are ADR 0006's. **Multiplayer previews** (from slice 3; behind the access wall under
    PREVIEW-WALL (a)): CI's native job builds the room binary like any other artifact; a delivery job shaped like the
    deploy job, in its own environment, hands that binary to the staging game server's supervisor (ADR 0006) and
    receives its current development-certificate hash; the preview deploy job writes the hash into the preview's own
    files, which the page fetches from its own HTTPS origin (ADR 0001, part 10; R2-TRN-REC5). That shape is proposed:
    no research compared it with another; the delivery token's scope, and whether pull-request-built native code may run
    on the staging game server, wait for the security baseline (ADR 0006, part 13, item B9).
    - Why: a fork run gets no secrets, and the two triggers that hand them out are avoided; splitting build and deploy
      keeps the token away from every build script and dependency; deploying the artifact CI tested means the link shows
      the tested build.
    - Ids: R5-PREV-REC1, R5-PREV-REC4, R5-PREV-OQ5, R5-PREV-OQ6, R5-XT-OQ6, R2-XT-OQ10, R6-XT-OQ4
      (r5-toolchain-quality.md, Recommendations, Question 6, lines 1588–1604 and 1625–1641; Open questions, lines
      1858–1862, 1996–2001; r2-netcode.md lines 3384–3389; r6-infrastructure.md lines 1009–1014); cites R5-PREV-REC2,
      R5-PREV-REC3, R5-PREV-REC5, R6-HOST-REC1, R6-HOST-REC3 (ADR 0006's).
    - Need: GLOSSARY §8, the foundation only ("private preview links"); it carries Mustafa's Brave check for every
      player-facing §6 item (R5-PREV-REC1, r5 lines 1598–1599); multiplayer previews: VISION_V2 §6.1 (rooms, tested
      per pull request, R6-HOST-REC3).
11. **The editor build.** The editor is its own package and its own HTML entry in the same Vite config as the player
    entry (R5-LAYOUT-REC5); the player build never loads editor code (G8's manifest walk and part 7's import rule:
    EDITOR_V2 §5's rule, legacy row TL4). The editor runs ADR 0004's bake and map checks in a Worker from the `navbake`
    and `sim` crates built to Wasm, so its validation panel and its Test use the same code as CI (G13). Whether the v2
    editor still opens by double-clicking its HTML file is OQ-0002-1: the research did not test a Vite-built page with
    a module Worker and Wasm from `file://`, so this record does not promise it, and EDITOR_V2 §5's rule changes only if
    Mustafa says so. EDITOR_V2 §5's other line, "Written so it ports cleanly if ADR-002 moves the project to modules",
    is overtaken: the v2 stack is modules bundled by Vite (GLOSSARY §8, the foundation); this record leaves the text to
    its owners (DOCS-ALIGN). What the editor does, and its own order of work, stay EDITOR_V2's (a phase 1 track, slice
    5); its map document is ADR 0005's.
    - Why: one entry per build keeps the player page free of editor code without hand-synced pages (row PG1).
    - Ids: R1-DATA-OQ11, R1-XT-OQ10, R3-XT-OQ3, R5-XT-OQ8 (r1-simulation-core.md lines 2817–2823 and 3075–3078;
      r3-rendering.md lines 1437–1440; r5-toolchain-quality.md lines 2008–2011); cites R5-LAYOUT-REC5.
    - Need: EDITOR_V2 §5 only; the editor entry serves VISION_V2 §6.7 (the faction content pipeline is authored there,
      R5-LAYOUT-REC2, r5 line 1188).
12. **Its spike: the Rust ↔ TypeScript boundary spike**, with the netcode proof test and before the foundation, since
    its result fixes the skeleton the foundation builds; what it must prove is below. The foundation itself is this
    record's migration plan (phase 2). `docs/SPIKE_PLAN.md` gives the spike its final name and place.
    - Need: GLOSSARY §8, the foundation (the skeleton it fixes); VISION_V2 §6.2, §6.9 and §4 through parts 5 and 6.

## Options considered
One table per question this record answers, with the template's dimensions as columns. Options and their facts are
the research's: r5-toolchain-quality.md, Options, lines 992–1146, and r1-simulation-core.md, Options, unit
boundary-worker, lines 1731–1788, unless a row says otherwise. "Chosen" marks the proposed option; "–" means the
research gives nothing for that dimension. An option that contradicts a settled input appears as one line and is not
weighed: **a bundler other than Vite, or a language other than Rust for the simulation and TypeScript for the page,
is settled by GLOSSARY §8, the foundation ("a Cargo workspace plus TypeScript with Vite").**

### Q1. Where the two workspaces sit (r5 lines 997–1002)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **A. One repository, both workspaces at the root (chosen)** | low: one checkout, one gate, one pull request per change across the boundary | – | – | – | none | the repository exists; `legacy/` sits beside `crates/` and `packages/` | a boundary type changes with both sides in one pull request (R1-BND-REC4) | two lockfiles and two toolchain files at the root; tools must agree on paths |
| B. One repository, two roots (`rust/`, `web/`) | low–medium: every command needs a `cd` | – | – | – | none | as A | as A | path drift between roots; generated types cross a folder boundary |
| C. Several repositories | high: Wasm and server versions matched across repositories | – | – | – | none | new repositories | – | version skew breaks the identical-results rule (R1-DET-REC6) and the CLI-library pin (R1-BND-F2) |

### Q2. The TypeScript package manager (r5 lines 1004–1008)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **npm workspaces (chosen)** | low | none new | – | lockfile and `npm ci --ignore-scripts`, as today (row TL17) | none | the legacy toolkit already uses npm (row TL17) | read (R5-LAYOUT-F8) | the page read is marked "(Legacy)" for npm 11 |
| pnpm or Yarn workspaces | – | a second tool on every machine | not read | not read | none | new | – | not researched (pnpm.io and yarnpkg.com were outside the allow list), so not proposable |

### Q3. Where generated types live (r5 lines 1010–1014)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **Committed into one package, regenerated and diffed in CI (chosen)** | low | a CI diff check | none at run time | every boundary change visible in review | none | new | reviewable diffs; a TypeScript-only check runs without cargo; R1-DATA-REC3 already commits the schema this way | the diff check must exist (G4) |
| Generated at build time, never committed | low | none | none at run time | boundary changes invisible in review | none | new | no drift | every TypeScript build and editor session needs the Rust toolchain |

### Q4. The crate list and where the room logic and the bake live (r5 lines 1170–1193; SLICE-ORDER condition 1; R4-XT-OQ5, r4 lines 1195–1200)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| R5-LAYOUT-REC2 as written: rooms inside `server`, the bake unplaced | low | – | – | decoders in `protocol` either way | none | new | the smallest list | the local server of SLICE-ORDER condition 1 cannot run in the Worker without `server`'s async runtime and transport, which the default Wasm target cannot carry (R1-BND-F19); where the bake ships stays open (R4-XT-OQ5) |
| **R5-LAYOUT-REC2 plus an I/O-free `room` crate and a `navbake` crate above `sim` (chosen)** | low–medium: two more crates and two more dependency checks | – | the player's Wasm carries neither the bake nor the server's I/O | the same decoders and room checks run locally and online | none | new | one room logic for the local server and the room process; the bake shared by CI, map save and the editor | two placements no research compared with others (the critic's questions only frame them) |

### Q5. Which TypeScript (r5 lines 1019–1024)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **A. TypeScript 6.0.x everywhere, pinned exactly (chosen)** | low | – | a slower checker than 7 | – | none | new | typescript-eslint works (R5-TSLINT-F4); one compiler | 6.0 is a transition release |
| B. TypeScript 7 for checking, 6.0 for typed lint | medium | two pins | faster checking | – | none | new | speed | two compilers can disagree |
| C. TypeScript 7 plus another type-aware linter | unknown | – | – | – | none | new | – | the linter's pages were not read: not proposable |

### Q6. Lint configs and the formatter (r5 lines 1026–1038)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| `recommended-type-checked` + `stylistic-type-checked` | low | stable under semver | typed lint costs a project build (R5-TSLINT-F15) | correctness rules with types | none | new | stable | fewer bug-catching rules |
| **`strict-type-checked` + `stylistic-type-checked`, exact pin (chosen)** | low | a minor upgrade can add errors, so upgrades are their own change-set | as above | more bug-catching rules (R5-TSLINT-F13) | none | new | catches more | not semver-stable |
| `all` | – | – | – | – | none | – | everything | advised against (R5-TSLINT-F13) |
| **Prettier as the formatter (chosen)** | low | one more dev dependency | – | – | none | new | the formatter ESLint's post names (R5-TSLINT-F21) | – |
| Biome | low | – | – | – | none | new | formatter and linter in one | a second linter beside the typed one, without type-aware rules on record |
| No formatter, lint rules only | – | – | – | – | none | – | – | ESLint deprecated its formatting rules |

### Q7. The Rust lint shape (r5 lines 1040–1050)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| A. Defaults plus `-D warnings` | low | – | – | nothing against panics in decoders | none | new | simple | nothing for determinism or panics |
| **B. A plus a workspace lint table and per-crate tiers (chosen)** | medium: per-crate lint lists | – | – | panics in decoders and the content loader found at review time | none | new | determinism (the hash-map, clock and float bans) and panic lints where they matter | `restriction` picks need reviewed `#[allow]`s with reasons |
| C. B plus `pedantic` everywhere | medium | – | – | – | none | new | more style and edge cases | noise in tools and tests; every pedantic warning blocks |

### Q8. Where the client simulation runs (r1 lines 1733–1739)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| A1. Main thread | lowest: no worker | – | the tick competes with rendering and the DOM; a long tick drops frames | – | weak for §6.9 and §4's 30+ fps | legacy is this shape (rows GP13, GP14) | direct calls | frame drops under load |
| **A2. One dedicated module Worker (chosen)** | one boundary | one thread and a per-tick hand-off | sim step time leaves the render thread | – | serves §6.2, §6.9, §4 | new | module workers in all four browsers (R1-BND-F25); Vite bundles it | a hand-off per tick |
| A3. A2 plus rendering in another worker | highest | – | not assessed | – | – | – | – | ADR 0003's question; not proposed there |

### Q9. How snapshots and events leave the sim (r1 lines 1741–1752)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| B1. Structured clone of JS objects | low | every field copied, objects built per entity | allocation and per-entity work every tick | – | – | – | stable | breaks the "no call per entity" rule |
| **B2. One flat buffer, transferred from a pool (chosen)** | low | one block copy in the Worker; the transfer is zero-copy | no isolation needed | – | – | – | stable Rust, no Wasm threads | delivery waits for the page's message queue (to be measured); the pool must recycle buffers |
| B3. A SharedArrayBuffer ring | medium | one block copy into a ring slot | – | needs cross-origin isolation (Safari only with `require-corp`) | – | – | the page reads the newest slot | torn reads unless published with Atomics; a lossless event ring; isolation costs |
| B4. The page reads the sim's shared Wasm memory | high | none | – | needs isolation | – | – | zero copies | nightly Rust, atomics, a rebuilt std, a fixed memory maximum; views go stale on growth |

### Q10. How inputs go in (r1 lines 1754–1760)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **C1. A small message per tick (chosen)** | low | one message per tick | constant work | – | edge counters keep a press from being lost or doubled (row GP2) | the never-wired command (row CS1) gets wired | works without isolation | one message per tick |
| C2. An input slot in a SharedArrayBuffer | medium | – | saves a message | needs isolation | as C1 | – | no message | isolation |

### Q11. Generated TypeScript types (r1 lines 1762–1769)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **D1. wasm-bindgen's own `.d.ts` (chosen, for the control API)** | low | the CLI must match the library (R1-BND-F2) | calls only, a constant number per tick | – | – | – | needed whatever else is chosen | covers only the exported functions |
| D2. tsify | medium | – | per-field calls (R1-BND-F5) | – | – | – | serde types as TypeScript types | two forks, maintenance unclear; wrong for per-tick state |
| **D3. ts-rs (chosen, for protocol, message and content types)** | low | – | none at run time | – | – | – | written on `cargo test`; serves sim, server, client and tools | one crate to track |
| **D4. Our own layout generator (chosen, for the block)** | low: ours to keep, small | – | none at run time | – | – | – | the only option that covers the bulk block, with a layout hash | ours to maintain |

### Q12. Wasm threads in the client sim (r1 lines 1771–1774)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **E1. None: one single-threaded sim in one Worker (chosen)** | low | – | one core for the sim | – | – | – | stable Rust, the default target | no split of one tick across cores |
| E2. Threads | high | – | a tick across cores | needs isolation | – | – | – | nightly, atomics, a rebuilt std; no §6 item asks for it |

### Q13. Cross-origin isolation (r1 lines 1776–1782)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **G1. Not isolated (chosen for production at the start)** | low | none | timers at 100 µs | no header or embed constraints | – | – | B2 and C1 work everywhere | no B3, C2 or W3 |
| G2. Isolated in production | medium | every cross-origin `no-cors` resource needs CORP; frames must be isolated; popups lose their opener | 5 µs timers, the memory API; enables B3 and C2 | COOP and COEP on every page and worker script | – | – | finer timers | a later login flow must live outside the isolated page (R1-BND-OQ9) |
| **G3. Isolated in development and previews only, behind the isolation flag (chosen for profiling)** | low | – | profiling timers | production headers unchanged | – | – | the boundary code is the same as G1 | two header sets to test (the end-to-end suite runs with and without the flag) |

### Q14. Who owns the network connection (r1 lines 1784–1788)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| H1. The page, forwarding to the Worker | medium | two hops per packet | a busy frame delays snapshots and prediction | – | late corrections | – | – | two hops |
| **H2. The Worker (chosen)** | low | – | snapshots, prediction and reconciliation never touch the page | – | – | – | both transports open in workers (R1-BND-F29); one boundary for prediction and local play | – |

### Q15. Test tools (r5 lines 1055–1076)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **proptest (chosen)** | low | a dev-dependency | – | – | – | – | shrinking, committed regression files, seedable | – |
| quickcheck, bolero | – | – | – | – | – | – | – | not read: not proposable |
| **cargo-fuzz on decoders and the content loader (chosen, nightly)** | low | nightly compiler; Linux CI only | – | finds panics in the bytes clients send | – | – | coverage-guided | nightly toolchain |
| **Playwright Test (chosen, end-to-end)** | low | – | – | – | – | legacy's smoke is Playwright (row TL9) | Chromium, Firefox, WebKit; `webServer` | WebKit is not Safari |
| Vitest Browser Mode | – | – | – | – | – | – | tests inside a page | maturity not stated |
| **wasm-bindgen-test (chosen, Rust in Wasm and real Safari)** | low | – | – | – | – | – | the Rust tests compiled to Wasm, unchanged; Safari through safaridriver | the guide read may be older than the crate |
| **L1. Our own link simulator behind the transport interface (chosen)** | small, our code | – | deterministic, seeded | – | – | – | runs in the gate; replays a failure from its seed | no QUIC, no browser |
| turmoil | – | – | seeded | – | – | – | – | QUIC on it not stated (unverified) |
| **L2. An impairment socket under the real quinn endpoint (chosen)** | small | – | real time | – | – | – | tests QUIC and real browsers | WebSocket needs a separate proxy |
| OS-level impairment (`tc netem`) | – | – | – | – | – | – | – | its docs were not read: not proposable |

### Q16. The Rust build cache and the pins (r5 lines 1081–1097)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| A. No cache | none | $0 | every run builds all dependencies (not measured) | none from caches | – | – | – | slow feedback |
| **B. rust-cache v2.9.3, one shared key per build kind, saved only from `main` (chosen)** | low: one step per job | $0 within 10 GB | restores dependency builds; our own crates always rebuild | poisoning through pull requests limited by saving only on `main` | – | today's CI caches nothing for Rust | – | eviction past 10 GB |
| C. sccache with the GHA backend | medium | $0 within 10 GB | no hits for cdylib, bin, proc-macro or incremental crates (R5-CI-F9) | – | – | – | per-crate hits | a second cache in one budget |
| D. B and C together | higher | $0 within 10 GB | possibly more hits (unmeasured) | – | – | – | – | only if B is measured not enough |
| **T1. `rust-toolchain.toml` with an exact version (chosen)** | low | the cache rolls over on each bump | – | – | – | – | one file for local, CI and agents | bumps are explicit pull requests |
| T2. The `stable` channel | none | – | – | – | – | – | – | a new Rust changes the build silently; breaks identical-results checks between days |
| **W1. The wasm-bindgen CLI through install-action, `fallback: none` (chosen)** | low | one more action to pin | prebuilt | checksum-checked (R5-CI-F15) | – | – | no source build on a cache miss | – |
| W2. `cargo install wasm-bindgen-cli --version <lock>` | low | – | builds from source on every cache miss | – | – | – | no extra action | build time not measured |
| wasm-pack (R5-LAYOUT-OQ2) | – | – | – | – | – | – | – | its status and releases were not read; its repository was to be archived or handed over (R1-BND-F1): not proposable |

### Q17. How CI runs the gate (r5 lines 1099–1105)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **G1. One workflow, parallel jobs, one final required job (chosen)** | low–medium | free on public repositories' standard runners (R5-CI-F17) | wall time is the longest job | a failing or skipped job cannot slip past the final job | – | today's `gate.yml` hygiene carries (row KT8) | one stable required check name | the final job's logic must be right (proven by mutation) |
| G2. Several workflows with path filters, each required | – | – | – | path-filtered workflows leave required checks pending (R5-CI-F20) | – | – | – | rejected by the research |
| G3. One serial job | lowest | – | wall time is the sum of every step | – | – | – | simplest | slow |

### Q18. How the gate is run, and when long checks run (r5 lines 1133–1146)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **A. One Node orchestrator (chosen)** | one more script, with its own mutation proofs | – | – | – | – | today's gate is a Node script (row TL1) | today's base ref, exit codes and `--json` carry; works on Windows | needs the Rust toolchain for G0–G2, as B does |
| B. `cargo xtask gate` | medium | – | the gate compiles before any check | – | – | conventions rewritten in Rust (inferred) | one language | as A for the toolchain |
| C. make or just | – | – | – | – | – | – | familiar | not read; make is not native on Windows |
| D. CI only | none | – | – | – | – | – | nothing local | breaks CLAUDE.md's protocol (the gate must pass before a commit) |
| Everything on every pull request | – | – | pull-request wall time grows with soak, fuzz and Safari | strongest guard | – | – | – | too slow for the CI budget |
| **A fast set on every pull request, the long set nightly and before playtests (chosen)** | – | – | inside R5-CI-REC5's budget | a long-run failure lands a day later, as an issue | – | – | – | – |
| Path-based selection inside the gate | – | – | saves time | a wrong rule skips a check silently | – | – | – | not at the start (R5-GATE-OQ1) |

### Q19. The preview workflow's shape (r5 lines 1110–1118)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **P1. Build with no secret; a separate deploy job in environment `preview` holds the token; same-repository pull requests only (chosen)** | low | – | – | pull-request code never runs where the token is | Mustafa gets a link to the tested build | today's CI has no preview | small | forks get no preview |
| P2. P1, and forks get a preview after a reviewer's approval | low–medium | – | – | the approver must read the diff; a stranger's files on our preview domain (inferred) | – | – | – | only once outside contributors appear |
| P3. `workflow_run` deploys every pull request's artifact | medium | – | – | gets secrets and write tokens (R5-PREV-F3) | – | – | – | rejected for now |
| P4. `pull_request_target` | low | – | – | GitHub warns against building pull-request code with it (R5-PREV-F2) | – | – | – | rejected |
| P5. The host builds from Git | low on our side | – | – | the host builds fork code | the preview is not the artifact CI tested (inferred) | – | – | breaks "the preview is the tested build" |

The client host itself (Cloudflare Pages, with Workers static assets as the alternative) is ADR 0006's (part 1; GLOSSARY,
the client host, decided by the advisor on 2026-10-30, resolution 19).

## Trade-off analysis
- **One repository with mechanical boundaries** wins because the boundary changes on both sides at once (a Rust type,
  its generated TypeScript and the page that reads it), and because the version pins R1 needs (the CLI equal to the
  library; one compiler for native and Wasm) hold only inside one lockfile. The legacy fence leaked because nothing
  checked it (row CS11); here every rule is a gate check with a planted defect that proves it (G5, G8, `--prove`).
- **A Worker, one block out and one command in per tick** wins over the main thread and over shared memory because the
  tick leaves the render thread, the work per tick is constant in N, and nothing needs cross-origin isolation, nightly
  Rust or a fixed memory maximum. The strongest argument against it: one copy per tick and a block whose delivery waits
  for the page's message queue, which no source measured (R1-BND-REC2's option B2 notes it as "to be measured"); the
  boundary spike measures it against R1-BND-REC2's marks 2 and 3, and the SharedArrayBuffer ring is the measured
  alternative.
- **The local server in the same Worker** wins because SLICE-ORDER condition 1 allows no offline-only path, and one
  Worker build then serves local play, prediction and online play. The strongest argument against it: in a local match
  the Worker runs the authority's sim step, the loopback encoding and decoding, and the client endpoint's prediction
  and block writing, a load no research measured (ADR 0001, trade-off 5, counts it in the client's sim share); the
  boundary spike measures it, and OQ-0002-3 picks between one Wasm instance and two.
- **Two crates beyond R5's list** (`room`, `navbake`) cost two more dependency checks; without them the local server
  cannot run in the Worker, and the bake's generator would risk shipping in the player's Wasm. No research compared
  other placements, so the first foundation milestone that builds them is also their test.
- **Strict TypeScript and lint, pinned exactly** catch the bugs the boundary invites, at the price of churn: strict
  configs are not semver-stable, so every upgrade is its own change-set, and the cost of the strictest flags against
  three.js's own types is not measured yet (R5-TSLINT-OQ4; the boundary spike counts it).
- **The gate v2 as one Node command with `--prove`** keeps today's conventions and proves each check, at the price of
  one more script to keep and a `--prove` run that rebuilds per mutation (R5-GATE-REC3 proposes at most 30 minutes;
  R5-GATE-OQ5). Even TypeScript-only work needs the Rust toolchain for G0–G2 (R5-GATE-OQ1).
- **Previews in shape P1** keep the host token away from pull-request code; forks get no preview, which matters only
  once outside contributors appear.
- **The strongest argument against the record as a whole:** it builds a lot of machinery (two workspaces, a 14-check
  gate, previews, generated types, the skeleton) before slice 1's first gameplay. The foundation, which the Advisor
  defined, asks for exactly that order, and each piece is held to "no check for later": every check enforces one
  recommendation of R1, R4 or R5, and every crate enters with the slice that first needs it.

## Consequences
- **Easier:** a boundary change lands in one pull request with both sides, and a stale generated type or a hand edit
  fails the gate; the simulation cannot quietly pick up I/O, a clock, a hash-map order or a server-only feature; local
  play and online play run the same messages and the same page code; every pull request from this repository has a
  preview link of the build CI tested; one command runs every check locally and in CI; every budget is a reviewed line
  of data.
- **Harder:** every toolchain bump (Rust, TypeScript, typescript-eslint, wasm-bindgen, Node) is its own change-set; any
  machine that runs the gate needs the pinned Rust toolchain with the Wasm target, Node and Playwright's browsers,
  Mustafa's Windows machine included (R5-TSLINT-REC4's hook mark is measured there); two header sets (with and without
  the isolation flag) are both tested; the Worker carries the local server; the v2 player build no longer opens by
  double-click (below).
- **Revisit when:** the boundary spike reports (B2 against B3, the Worker's pacing and the isolation question, Wasm
  calls per tick in both modes, block size and copy time, the Wasm size); typescript-eslint supports TypeScript 7; the
  first month of CI times (deliberate skips, path selection, the Safari run); the security baseline is researched
  (the advisories policy, the CSP in the header sets, the preview and Sentry token scopes, the `textContent` lint);
  ADR 0006 decides where the lobby runs (its crate); the cache passes 8 GB; ADR 0006 picks Arm hosts (an arm64
  determinism leg); Mustafa answers OQ-0002-1.
- **Hard constraints in CLAUDE.md that change** (for v2 code only, once Mustafa accepts the drafts; spikes are exempt
  now under SPIKE-SCOPE (a); the legacy constraints keep applying to `legacy/`; nothing in `CLAUDE.md` or `.claude/`
  changes in this PR):
  - "Classic scripts sharing globals in a fixed load order": v2 code is ES modules bundled by Vite and Rust crates;
    imports replace the load order, and the compilers replace the syntax, concatenation, symbol and name checks (rows
    TL2, TL3, TL6, TL8: drop).
  - "the game runs by double-clicking index.html (file://)": the v2 player build is served, over HTTPS from the client
    host (ADR 0006, part 1) and on localhost from Vite's dev server or `vite preview` in development, the secure
    contexts R3-WGPU-REC4 asks for (r3-rendering.md, Recommendations Q1, lines 941–951); WebTransport needs HTTPS
    anyway (R2-TRN-F35). The double-click player build ends with the legacy engine (decision map part j, J2, ADR-002);
    `legacy/index.html` keeps it. Whether the v2 editor keeps double-click is OQ-0002-1.
  - "Layering is hard: core references nothing above it; anything on index.html's live path resolves in core ∪ game,
    never editor": becomes part 3's dependency rules, checked by G5 and G8.
  - "Three.js r128 UMD (no CapsuleGeometry). No addons, no post-processing (under review).": which three.js v2 uses is
    ADR 0003's; this record's build bundles it from npm with Vite, so v2 has no CDN script tag (the STATUS item CDN-SRI;
    ADR 0006, part 1).
  - "No external assets (procedural textures, synthesized audio)": ends with the legacy engine under LOOK_V2 §8; the
    pipeline is ADR 0005's, and this record's build serves model files, each under the client host's 25 MiB limit.
  - The other constraints ("`LinearEncoding` output", "Persistence only through `KVStore`", "`stylizeScene()` runs
    once", "Animation is cosmetic") are ADR 0003's, ADR 0001's and ADR 0006's, and ADR 0004's.
  - Outside the hard-constraint list, the change-set protocol's gate step (CLAUDE.md, "The change-set protocol", step 4)
    would run the gate v2 for v2 code and today's gate for `legacy/`: a kit change-set after acceptance writes it.
- **Legacy inventory rows** (the inventory file is not edited; it changes only once Mustafa accepts the drafts): this
  record changes no row's mark, and none of its rows is "look brief pending". The rows it carries out, so the verifier
  can check that nothing changed:

  | Row id | Mark now | Proposed mark | Why |
  |---|---|---|---|
  | TL1 (gate driver, line 386) | redesign | redesign (no change) | part 9: the gate v2 keeps the base ref, exit codes, `--json` and mutation proofs |
  | TL2, TL3, TL5, TL6, TL8 (lines 387–393) | drop | drop (no change) | the compilers and the bundler do their jobs; TL3's removal list becomes `--allow-removed`, TL6's reviewed list `no-restricted-globals` |
  | TL4, TL7 (lines 389, 392) | redesign | redesign (no change) | part 3's rules, G5 and G8 |
  | TL9, TL12 (lines 394, 397) | redesign | redesign (no change) | G12: TL9's shape, the backend on every shot, the network rule flipped |
  | TL10, KT9 (lines 395, 418) | redesign | redesign (no change) | part 7's per-edit hook, in a kit change-set |
  | TL11 (line 396) | drop | drop (no change) | kept runnable as the legacy oracle for golden fixtures until legacy retires (parts 1 and 8) |
  | TL15 (line 400) | redesign | redesign (no change) | part 8's tick bench; the client render bench is ADR 0003's |
  | TL17, KT8, KT19 (lines 402, 417, 428) | redesign | redesign (no change) | parts 2 and 9: pins, CI hygiene, the ignore list of part 1 |
  | KT7, KT12 (lines 416, 421) | drop | drop (no change) | the v2 gate and lint replace them; KT12's rules move with `legacy/` |
  | PG1, PG18, PG19 (lines 359, 376, 377) | drop | drop (no change) | two Vite entries replace the hand-synced pages; modules replace the script order; three.js is bundled |
  | CS11 (line 179) | redesign | redesign (no change) | part 3: the crate boundary replaces the comment fence |
