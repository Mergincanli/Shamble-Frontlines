# ADR-0002: Code structure, build and toolchain

**Status:** Proposed. Part 1 accepted by Mustafa on 2026-11-10, his SCOPE (a), word for word: "SCOPE: (a). I accept ADR 0002's part 1 alone: the repository after the restructure, with the legacy hard constraints applying to legacy/ only once the files have moved. Record it in the ADR's Status line in 2.1b's docs pass, on my go. Nothing else in ADR 0002 is accepted by this." (`docs/DECISIONS.md`, the entry "Taken by Mustafa, 2026-11-10: FREEZE-FDN (a), SPLIT (a), SCOPE (a) and the build go for 2.1a, in the foundation chat")
**Date:** 2026-10-31
**Author:** engine-architect (the "Pass marks" table: test-engineer, plan step 4) ·
**Reviewed by:** (filled in at review) ·
**Signed off by Mustafa:** as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted except ADR 0002's part 1, accepted by Mustafa on 2026-11-10 (SCOPE (a))

This record decides how the v2 code is laid out, built, checked and shipped, and where the line between the Rust
simulation and the TypeScript page runs. One repository holds a Cargo workspace (the Rust crates) and npm workspaces
(the TypeScript packages, built with Vite), with every tool pinned to an exact version; the legacy game moves to
`legacy/` and stays playable by double-click; throwaway spikes live in `spikes/`. The simulation depends on nothing
above it, and a gate check proves that. In the browser the simulation runs in one Web Worker, which also runs the local
server from slice 1 and owns the network connection online; the page gets one block of data per tick, posts its newest
input once per drawn frame and never calls into Wasm, so nothing crosses the boundary per entity. TypeScript types are
generated from the Rust code. One command, the gate v2, runs the same checks locally and in CI and proves itself with
planted defects; every
pull request from this repository gets a [preview](../research/GLOSSARY.md#preview) link. The reason: the settled stack runs one Rust simulation natively on the server and as Wasm in the
browser, and VISION_V2 asks for four desktop browsers on a modest laptop and 48 players; [the foundation](../research/GLOSSARY.md#the-foundation) the Advisor defined is exactly this structure,
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
  integrated graphics runs [the largest mode](../research/GLOSSARY.md#the-largest-mode-and-its-n) at 30+ fps on Low; fair up to about 100 ms of ping. MODES-3 (a): the
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
12. [The client host](../research/GLOSSARY.md#the-client-host) serves no file larger than 25 MiB (r6-infrastructure.md, Findings §1, R6-HOST-F3, lines 72–79;
    decision map N37), and WebTransport runs only in secure contexts (r2-netcode.md, Findings, Brief question 4,
    R2-TRN-F35, lines 1268–1272).

### What this record takes from and gives to the other drafts
- **Takes:** the message protocol, the command's fields, the snapshot fields of remote entities, the interpolation
  design (its buffer in the Worker), the thin transport interface with its loopback backend for the local server, and
  the network profiles (ADR 0001); the fixed-tick mechanism, the Worker's pacing, the determinism rules, the panic-free
  sim and the restart after a trap (ADR 0004); the content schema and where content's TypeScript types come from (ADR
  0005); the frame budget rows, among them [the page's cost of the sim](../research/GLOSSARY.md#the-clients-sim-share), and the client reference machines (ADR 0003); the
  client host, its token scope, the access wall, the cleanup policy, the staging game server, the secrets policy and
  runner costs (ADR 0006).
- **Gives:** the repository, the crates and packages and the rules on who may depend on whom; the toolchains and build
  profiles; the Worker, the per-tick output block's layout and versioning, the command posting, the generated types and
  [the isolation flag](../research/GLOSSARY.md#preview-isolation-and-the-isolation-flag) (decision map interfaces I-2, I-3, I-5); the client + local server skeleton; lint, tests and the
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
- The headless "SIM fence", a fence made of comments, leaked: shape data came from the visual recipe and `hasLOS` read a
  global (row CS11; r1-simulation-core.md, legacy facts, lines 1176–1178); the frame loop stepped the world inside the
  render loop (rows GP13 and GP14; r1 lines 1159–1164); and the input command was declared but never wired (row CS1; r1
  lines 1150–1154): the crate boundary, the Worker and the per-tick command fix these.
- At 48 soldiers the legacy JavaScript combat step took 0.80–1.00 ms per update step on the main thread (row GC2;
  r1-simulation-core.md, lines 1168–1171): a scale for what the Worker takes off the page, not a v2 figure.
- Two hand-synced HTML pages exist only because of classic scripts and `file://` (row PG1, line 359), and the three.js
  r128 CDN tag has no SRI or CSP (row PG19, line 377; STATUS item CDN-SRI, decision map part j, J2).

## Decision

One repository, two pinned workspaces (Cargo for Rust, npm with Vite for TypeScript), and a list of crates and packages
in which the simulation depends on nothing above it, proven by gate checks. In the browser the simulation runs in one
module Web Worker that also runs the local server and owns the network connection; the page posts its newest input to
the Worker once per drawn frame and the Worker sends the page one transferred block per tick, so nothing crosses per
entity and the page never calls into Wasm.
Types that cross the boundary are generated from Rust. One gate command runs the same checks locally and in CI and
proves itself with planted defects; budgets live as data. Every pull request from this repository gets a preview link
built from the tested artifact. Each part says what, why, its research ids and the need it serves. Lines are at commit
9e7db28.

1. **The repository after the restructure.** In the foundation's first step the five legacy game files
   (`shamble-core.js`, `shamble-game.js`, `shamble-editor.js`, `index.html`, `editor.html`) move together into
   `legacy/`, which keeps their relative paths, so `legacy/index.html` still runs by double-click; the legacy hard
   constraints then apply to `legacy/` only (SPIKE-SCOPE (a)). Beside it at the root: `Cargo.toml` (a virtual
   workspace) with `crates/*`, `package.json` (npm workspaces) with `packages/*`, `spikes/` for throwaway spikes,
   `fuzz/` (outside the workspace's default members, because cargo-fuzz needs a nightly compiler: R5-TEST-F4, r5 lines
   430–434; it carries its own `rust-toolchain.toml` that pins one dated nightly, a channel form R5-CI-F12 quotes, r5
   lines 632–639, so the fuzz jobs' compiler is pinned like every other: part 2), and `docs/`.
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
   (Rust, clippy and rustfmt, the fuzz jobs' dated nightly, Node, TypeScript, wasm-bindgen) is its own change-set that
   reports the new warnings.
   A weekly job tests the latest dependency versions without blocking (`continue-on-error`).
   - Why: one file pins local, CI and agent builds; the identical-results check between native and Wasm means
     something only when both come from one compiler; typed lint needs TypeScript below 6.1.
   - Ids: R5-LAYOUT-REC1, R5-CI-REC2, R5-CI-REC3, R5-TSLINT-REC1, R5-RUSTLINT-REC6, R1-BND-REC4 (the CLI pin),
     R5-LAYOUT-OQ1, R5-LAYOUT-OQ2, R5-XQ-OQ2, R5-TSLINT-OQ1, R5-TSLINT-OQ2, R5-CI-OQ4, R1-BND-OQ7
     (r5-toolchain-quality.md, Recommendations, lines 1153–1169, 1523–1546, 1250–1264, 1374–1381; Open questions, lines
     1750–1755, 1766–1769, 1833–1834, 1895–1898; r1-simulation-core.md, R1-BND-F1, lines 886–893; R1-BND-REC4, lines
     2297–2308; R1-BND-OQ7, lines 2741–2744).
   - Need: GLOSSARY §8, the foundation only ("the toolchain"); through R1-DET-REC6's native-against-Wasm comparison,
     VISION_V2 §6.6 and §3, as R5-CI-REC2 traces it (r5-toolchain-quality.md, line 1532; R1-DET-REC6 serves §6.6 and
     §3, r1-simulation-core.md lines 2236–2237).
3. **Crates, packages and who may depend on whom.** The crates, each entering the workspace with the first slice that
   needs it (no crate "for later"):
   | Crate | What it holds | Depends on | Enters | Need |
   |---|---|---|---|---|
   | `sim` | the simulation: state, phases, tick, the navigation runtime (ADR 0004) | `content` only (and its maths dependencies, ADR 0004); no transport, no async runtime, no clock | the foundation | §6.1–§6.5 |
   | `content` | loads and validates content data (ADR 0005) | nothing above it | slice 1 (weapons are data) | §6.3, §6.7 |
   | `protocol` | message types, the codec, decoders that never panic, the Rust side of the thin transport interface with its loopback backend (ADR 0001) | `sim` and `content` types only, as ADR 0001 needs them; no transport crate, no async runtime (it builds for Wasm) | the foundation | §6.1 |
   | `room` | the room's match logic with no I/O, no async runtime and no clock (its timers count ticks): the slot table, bots filling slots through the input path, per-client command queues, per-client snapshot and event encoding through `protocol` (ADR 0001 decides what it does) | `sim`, `protocol`, `content` | the foundation | §6.1; SLICE-ORDER condition 1 |
   | `client-endpoint` | the client endpoint, with no bindings, no transport, no async runtime and no clock: prediction and reconciliation, the interpolation buffer, snapshot decoding through `protocol`, the aim cast, the block writer and the block's layout table (part 5); built for Wasm inside `sim-wasm` and natively for the headless clients of part 8 | `sim`, `protocol`, `content` | the foundation | §6.2, §6.9 and §4, as `sim-wasm`, whose code it holds; §3, §6.1 and §6.6 through the headless clients (R5-TEST-REC4, REC5) |
   | `sim-wasm` | the wasm-bindgen bindings the Worker loads, and nothing else: around the client endpoint (`client-endpoint`) and the local server (`room` + `sim`) | `client-endpoint`, `room`, `sim`, `protocol`, `content` | the foundation | §6.2, §6.9 |
   | `server` | the room process: transport on quinn and the WebSocket fallback, the async runtime, the supervisor hook, metrics (ADR 0001, ADR 0006) | `room`, `protocol`, `sim`, `content` | slice 3 (online) | §6.1, §6.5, §6.6, §6.8 |
   | `supervisor` | the small supervisor on every game VM (R6-HOST-REC2, r6-infrastructure.md lines 645–647; ADR 0006 part 5): it starts, watches and ends room processes; what else it does is ADR 0006's | what ADR 0006 gives it; it runs room processes as programs and links no `sim`, `room` or `sim-wasm` code | slice 3 (online) | §6.1 |
   | `devtools` | the replay runner, the hash comparer, the tick bench, the legacy-golden reader, the headless protocol clients of the network and soak layers (part 8) | anything | the foundation | §6.6, §6.8; the headless clients: §3, §6.1 (R5-TEST-REC5) |
   | `navbake` | the navigation bake (ADR 0004's generator), run by CI, on map save and in the editor's Test | `sim`, `content` | slice 2 (bots) | §6.2 (bots between levels) |

   The packages: `client` (the player entry: input, camera, HUD, rendering), `editor` (its own entry), `sim-worker`
   (hosts `sim-wasm`, paces the tick, runs the local match), `net` (the browser's two backends of ADR 0001's thin
   transport interface, WebTransport and WebSocket, which R2-TRN-REC4 puts in TypeScript), `types` (generated types, part 5) and `e2e`
   (Playwright tests). The lobby gets a crate when ADR 0006 decides where it runs (OQ-0006-3: inside the supervisor's
   binary or as its own service); until then no lobby crate is created.
   The rules, each a gate check (G5, G8, part 9): `sim` depends on `content` only, with no transport, no async runtime
   and no clock, checked with cargo-deny's `deny` plus `wrappers` and with `cargo tree`; `room` and `client-endpoint`
   build for `wasm32-unknown-unknown` and natively, with no async runtime, no transport crate and no wasm-bindgen (the
   same two tools), and read no clock (part 7's clock ban); `content` depends on
   nothing above it; `sim-wasm` never depends on `navbake` or `server`, so the player's Wasm carries neither the bake
   nor the server's I/O (`cargo tree` on the `sim-wasm` build); `client` never reaches `editor`, checked twice: an
   ESLint `no-restricted-imports` rule and a walk of Vite's manifest from the player entry through `imports` and
   `dynamicImports`; `sim-worker` and `net` never import rendering or DOM packages; nothing imports `legacy/` or
   `spikes/`; every Rust dependency comes from crates.io, checked by cargo-deny's `sources` class at deny (proposed:
   R5-GATE-F3 lists the class, r5 lines 954–960, but its options were not read; the security review's finding 10),
   so a git or other-registry dependency fails G5. `sim` is built with one feature set on every target: it exposes no
   feature that changes maths, the
   maths-relevant features of the dependencies it shares with `server` are set once in `[workspace.dependencies]`,
   and the gate compares the resolved feature list of every `sim` dependency (`cargo tree -e features`) across every
   build that runs the sim (`server`, `devtools`, `navbake` and `sim-wasm`), each made by the exact command CI and the
   release use, since Cargo unifies features over everything one command builds (R5-LAYOUT-F4).
   - Why: a mechanical check is the only kind that holds the boundaries R1 and R2 ask for (the legacy fence leaked,
     row CS11); Cargo unifies features over everything built together (R5-LAYOUT-F4), so the server could change the
     native sim's maths.
   - Changed from R5-LAYOUT-REC2 (r5 lines 1170–1193): three crates are added and none removed. `room`, because
     SLICE-ORDER condition 1 runs the local server in the Worker, and the Worker cannot carry the server's async
     runtime and transport (on the default Wasm target `std::thread::spawn` panics: R1-BND-F19, r1 lines 1019–1025;
     the simulation depends on no I/O: R2-TRN-REC4, r2-netcode.md lines 2673–2698); R5-LAYOUT-REC2 predates
     SLICE-ORDER and puts rooms inside `server`. `navbake`, because where the bake lives decides whether the generator
     ships in the player's Wasm (the critic's R4-XT-OQ5, r4-physics-navigation.md lines 1195–1200), and the bake runs in
     CI, on map save and in the editor's Test (R4-NAV-REC1, r4 lines 802–810), never in prediction (R4-NAV-REC4, r4
     lines 888–899). `client-endpoint`, because ADR 0001's network marks run the client endpoint's Rust code natively
     and headless, and so do the soak and load layers (part 8), while R5-LAYOUT-REC2 names no crate for that code but
     the bindings crate: the headless clients would otherwise need a native build of a bindings crate or a second copy
     of the endpoint. The code is the same; only its crate is new. The first two answer questions the critic gave this
     record and the third follows from ADR 0001's headless tests; no research compared them with other placements. All
     three are proposed (not researched): no research Options section offers a crate for the room logic, the client
     endpoint or the bake, and SLICE-ORDER condition 1 and R4-XT-OQ5 leave them to this record; plan rule 3 allows
     that (OQ-0002-8, answered (a) by the Advisor, 2026-11-03). The supervisor's crate (slice 3) is beyond R5's list
     too, but the research asks for it: R6-HOST-REC2 puts a small supervisor on every VM (r6-infrastructure.md lines
     645–647), which ADR 0006 part 5 adopts.
   - The cache question (critic rank 40) is answered by this list: no ECS crate (ADR 0004, part 1) and parry3d rather
     than Rapier (ADR 0004, part 10), quinn and the WebTransport crate the netcode proof test picks (ADR 0001, part 10);
     the cache is measured on this workspace against R5-CI-REC1's 8 GB mark in the foundation (part 9).
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
   is ADR 0004's (part 20) with ADR 0001's resync path. A trap carries no panic message, so the Wasm build's panic
   hook (ADR 0004 part 20, S20) writes the message and location into a fixed buffer in linear memory, and when the
   instance traps the Worker's host reads that buffer, through a fresh view of the memory (part 5), and attaches it to
   the trap's report. The server is built for baseline x86-64, never with `+fma`,
   `-C target-cpu=native` or the `relaxed-simd` target feature: the gate reads the build flags of `sim`, `room`,
   `client-endpoint`, `sim-wasm` and `server` and fails if one appears (the rule is ADR 0004's, part 7; the check is
   this record's). If wasm-opt is kept (R5-CI-OQ4), the check reads its command line too, because wasm-opt optimizes
   the shipped Wasm after rustc: it fails on binaryen's `--fast-math` (`-ffm`, "Optimize floats without handling
   corner cases of NaNs, rounding, and negative zero" in binaryen's own options, read by rust-reviewer at the step-10
   review; not in the research) and on any feature flag beyond the target's defaults (R1-BND-F19's list, r1 lines
   1019–1025), `relaxed-simd` above all.
   `unsafe_code` is denied in every crate; an exception needs a written invariant, a test and an `#[allow]` with its
   reason. The Wasm size budget has no research number, and this record owns it (R1-RR-OQ5): the boundary spike and
   the sim core spike measure the `sim-wasm` file (raw and gzip, after wasm-opt and without it, `opt-level` "s" against
   "z", with parry3d), the content and ability spike measures the share the `content` crate adds (ADR 0005,
   OQ-0005-CD2), and the Advisor sets the first limit in the budget file (part 9) from both measurements; until then
   G7 records the `sim-wasm` size and does not gate it. Every served file stays under the client host's 25 MiB
   per-file limit.
   - Why: Cargo's dev and release profiles differ on overflow by default (R5-RUSTLINT-F13), and one remote panic must not
     end a 48-player match.
   - Ids: R5-RUSTLINT-REC4, R1-RR-OQ4, R1-RR-OQ5, R5-RUSTLINT-OQ4 (r5-toolchain-quality.md, Recommendations, Question 3,
     lines 1353–1364; Open questions, line 1793; r1-simulation-core.md, Open questions, lines 2941–2950); cites
     R2-ROOM-REC2 (r2-netcode.md, Recommendations, Brief question 5, lines 2762–2772, its fix-pass note on `panic = "unwind"` and its "Serves"), R1-RR-OQ3 and
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
     (own weapon state, money, upgrade tracks and offers: decision map I-21; the floor layer and enclosure area id the
     roof fade needs: ADR 0004 part 14) and the event section (the
     events of ADR 0004 part 19: hits, deaths and kills, blasts, cosmetic shot events, purchases, uplink cache events,
     harvester deliveries and kills, structure kills and stage changes, tech tiers, upgrade picks and pickups, and the
     moments its moment phase emits; ADR 0001 sets their visibility). Online, remote entities are written at the
     interpolation delay from the Worker's interpolation buffer and the local player as predicted (ADR 0001 parts 4,
     6, 7); the fields of remote
     entities are ADR 0001's snapshot fields in this record's layout. The event section holds the events due at that
     block, each on the timeline of the entity it belongs to: the Worker releases a remote entity's spatial event (an
     action start, a death, a blast on the entity it hit) when its interpolation clock reaches the event's tick, and
     the local player's own, predicted and owner events at once; a local match does the same, since its client
     endpoint is the same code (the rule is ADR 0001 part 7's; r2-netcode.md, Recommendations, Brief question 1,
     R2-SYNC-REC6, line 2210: blasts "played on the timeline of the entity they hit"). The page binds typed-array views
     on the block with no copy, keeps the two newest blocks to interpolate between ticks (R1-TICK-REC5) and returns
     older ones to the pool. Whether a section is laid out per entity or per field is measured in the boundary spike
     (bytes per tick and copy time at N = 48); ADR 0003 reads typed arrays either way. The block writer serializes
     field by field (R1-DET-REC6's method for [the state hash](../research/GLOSSARY.md#state-hash), applied to the
     block), so struct padding never reaches the page and no `unsafe` is needed. R1-RR-OQ6's decider, rust-reviewer,
     confirmed this at the step-10 review on three conditions: one layout table in Rust drives the writer, the
     generated offsets and the layout hash, so the three cannot drift; every integer is written at a fixed width (never
     `usize`) and little-endian (`to_le_bytes`); each section is aligned to its largest element type, so the page's
     typed-array views bind with no copy. R1-RR-OQ6's two alternatives (a derive that rejects padding, or `unsafe` with
     a written invariant and a test) are weighed only if the writer's p99 in Wasm on the minimum laptop exceeds 5 % of
     the client's sim share at 30 Hz (0.05 × 13.33 = 0.67 ms; proposed by rust-reviewer: below that the writer cannot
     decide that mark). The SharedArrayBuffer ring (B3) is the spike's
     measured alternative, never the sim's own shared Wasm memory (B4). At the 30 Hz baseline that is 30 blocks a second
     (one per tick).
   - **The newest input, posted once per drawn frame,** from the page to the Worker (option C1, changed in its cadence):
     a sequence number, the move input, the held-button bitmask and press and release edge counters kept as running
     totals (their exact fields are ADR 0001's), plus the camera's cursor ray and the view time of the frame the page
     drew. At each tick the Worker takes the newest post (the running totals mean a superseded post loses no press),
     casts its ray against the state it published for that view time and fills the tick's one network command, aim
     point included (ADR 0001 part 5; decision map I-19), so the aim needs no second message. The Worker stamps the
     tick it applies the input to. Why per frame: the page has no tick clock (its only tick signal is the block), so a
     post per block can leave a press waiting up to two tick periods less the block's delivery before a tick applies it
     (2 × 1000 / 30 = 66.67 ms at 30 Hz), against R1-BND-REC3 mark 2's one tick plus one frame (33.33 + 16.67 =
     50.00 ms with a 60 Hz display; r1-simulation-core.md, Recommendations, unit boundary-worker, R1-BND-REC3, lines
     2286–2296); a post per frame bounds the wait by one frame plus one tick plus the message's delivery, and the
     page's work stays one small message per frame.
   - **The small control API** (create, mode flag, load the content bundle, one input call per tick, and the step) is
     wasm-bindgen's own, with its `.d.ts` (option D1). Each tick the Worker's host makes one input call and one step
     call. The input call carries the page's newest input; every datagram received since the last tick, each with the
     time it arrived (the host's `performance.now()` in that datagram's own message event, never the tick's time); the
     time the host sent the previous step's datagrams; and the host's current time, all as plain values, since Rust
     reads no clock of its own (part 7). Without arrival times every clock and jitter sample of ADR 0001 parts 5 and 7
     would carry up to one tick period of false jitter (0 to 1000 / 30 = 33.33 ms at 30 Hz); those samples use the
     stamps, and how a ping answered at the next tick reports its hold time is ADR 0001's (part 5). The step leaves the
     outgoing datagrams and the block in linear memory and returns where they lie and when the next tick is due: in
     local play from the plain accumulator, online from the client endpoint's server-time estimate, since online the
     Worker's tick runs ahead of the server's by ADR 0001's rule for keeping commands just in time (OQ-0001-6). The
     host sets its timer from that, never from a count of its own, and the catch-up it allows is counted per wake of
     the Worker, at most 7 ticks (ADR 0004 part 5, S18). The host reads the step's output through a fresh view of the
     instance's `memory.buffer` after every call, never a cached one, since growth makes an old view stale (proposed:
     R1-BND-REC2 names re-binding on growth as a cost of B4, r1 lines 2273–2274). The Worker's Wasm calls per tick,
     wasm-bindgen's glue calls included, are meant to stay within R1-BND-REC1 mark 2's four, which the spike counts
     (item 1); none is made per entity.
   - **Generated types, from three sources, into one committed package** `packages/types`: wasm-bindgen's `.d.ts` and
     glue are build output (ignored); our own small generator writes the block's layout (offsets, strides, element
     types) and its layout hash from the layout table the writer uses (above), and the page checks the hash at startup
     (option D4); ts-rs writes the protocol and message
     types into `packages/types/src/generated/` on `cargo test`, through `TS_RS_EXPORT_DIR` in `.cargo/config.toml`
     (option D3); content's types land in the same package, from the source ADR 0005 picks (critic rank 31). Not tsify.
     The generators' derives never reach a shipped build: ts-rs is a dev-dependency, and its derives sit behind
     `cfg_attr(test, …)` or, where one crate's types nest another's, behind a feature that only dev-dependency entries
     turn on ("Dev-dependencies do not activate features unless building a Cargo target that needs them": R5-LAYOUT-F4,
     r5 lines 83–91), so the builds part 3 compares and G7 measures are the shipped ones (proposed by rust-reviewer at
     the step-10 review); content's schema generator follows the same rule (ADR 0005, CD-3).
     The gate regenerates all of them and fails on any difference or hand edit (G4).
   - **Content files in development:** a small Vite plugin, in the `client` package's Vite config and never in a
     production build, pushes changed content files to the page, which posts them to the Worker (R1-DATA-REC5,
     r1-simulation-core.md lines 2545–2547; ADR 0005, CD-7); whether Vite's hot update reaches the Worker directly is
     PM-0002-31's to measure (R1-DATA-OQ8). What a running match does with a changed file is ADR 0005's
     (OQ-0005-CD4).
   - **Cross-origin isolation is off** for v2's start (option G1). [The isolation flag](../research/GLOSSARY.md#preview-isolation-and-the-isolation-flag)
     adds COOP `same-origin` and COEP `require-corp` only to profiling builds, locally or on
     [a preview](../research/GLOSSARY.md#preview) built with it on (option G3); the preview Mustafa checks is built
     with it off. Production turns isolation on only if the boundary spike's B2-against-B3 test passes R1-BND-REC2's
     mark 5, or if the Worker's timer pacing (W2, ADR 0004 part 5) fails its jitter mark and the `Atomics.wait`
     alternative (W3), which needs shared memory, passes it; then the default flips at once everywhere the client runs
     isolated, B3 and posted inputs through shared memory (C2) become available, and the cost list of R1-BND-REC5
     applies (OQ-0002-4). The client must keep working inside a third-party iframe, such as a web portal's, so a design
     that needs isolation (B3, C2 or W3, as OQ-0002-9 reads his rule) is picked only if a non-isolated fallback stays
     available (PORTAL-EMBED, Mustafa 2026-11-04; OQ-0002-9).
   - **The Worker owns the network connection** online (option H2): server snapshots, prediction and reconciliation
     never touch the page, which sees only the block. The received datagrams ride in the tick's one input call and the
     outgoing ones come back through the step (the control API above), so the Wasm calls per tick stay the same at
     every N (R1-BND-REC1 mark 2), even when a snapshot is split into several datagrams. What flows over the connection
     is ADR 0001's.
   - Why: the tick leaves the render thread; one bounded copy per tick needs neither isolation nor nightly Rust; one
     small message per frame is constant work and keeps a press within one tick plus one frame of the tick that applies
     it; the block's layout is the only thing tsify and ts-rs cannot type.
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
   [the authority](../research/GLOSSARY.md#the-authority), with bots filling slots), and the client endpoint reaches it through the loopback backend with
   the same encoded messages (ADR 0001 part 10; its migration plan puts the loopback in the protocol crate). The page never handles a network packet and cannot tell the two modes
   apart. The local server and the client endpoint share one Wasm instance, so the loopback's bytes never leave linear
   memory; whether a second instance is worth its cost, so that a trap in the client endpoint leaves the authority
   running, is measured in the boundary spike (OQ-0002-3). The Worker's host (TypeScript) receives the page's inputs,
   paces the tick (ADR 0004 part 5), steps the instance or instances, and posts each block; the inputs it forwards
   pass through it, outside the Wasm instance, which is what OQ-0004-7's recovery options need to know. In a local
   match one Worker step runs in this order: the client endpoint builds tick T's command from the newest input (its
   aim cast included) and predicts T with the sim's prediction step (ADR 0004 part 3); the command reaches the local
   server through the loopback; the local server runs tick T with the authority step and every slot's command (the
   bots' through the same input path) and encodes T's snapshot and events;
   the client endpoint decodes them, reconciles and writes the block. So the command for tick T reaches the local
   server before it runs T, and a local input never waits an extra step (the spike's item 3); with two instances
   (OQ-0002-3 (b)) the host makes its calls in the same order. The Worker as a whole has no budget in the research:
   the client's sim share covers the sim step only (GLOSSARY, the client's sim share, lines 214–218), so the boundary
   spike reports the Worker's busy time per tick in both modes and its JS heap and Wasm memory over a match (item 6),
   and this record sets any limit on them from those numbers. The foundation's skeleton ends with an empty scene drawn
   from blocks that came through this whole path (Migration plan, phase 2).
   - Why: SLICE-ORDER condition 1 asks for exactly this, and one boundary then serves local play, prediction and
     online play, so no code path exists only offline.
   - Ids: R1-BND-REC6, R1-BND-REC1 (the Worker), R5-LAYOUT-REC2 (the packages); cites R2-TRN-REC4 as ADR 0001 changes
     it (the loopback backend), R1-BND-OQ1, R1-XT-OQ8 and R5-XT-OQ4 (offline play as a shipped feature: a design
     question for Mustafa, listed in `docs/adr/README.md`'s design questions).
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
     names its lint rule (ADR 0006, part 13, item B6). So that a strict content security policy stays possible, the
     build is CSP-ready, as ADR 0006's interim position for B3 asks (proposed, not researched; the security review's
     finding 11): ESLint's core `no-eval` and `no-new-func` are on at error (their pages were not read), the built HTML
     holds no inline script or style, and the Worker and the Wasm load as files, never from `blob:` or `data:` URLs
     (checked in G8).
   - **Rust:** one workspace lint table every crate opts into (`[lints] workspace = true`, itself a gate check):
     `unsafe_code = "deny"`, clippy's `all` at warn, and the gate runs `cargo clippy --workspace --all-targets --locked
     -- -D warnings` once for the host and once for `wasm32-unknown-unknown`; `pedantic` at warn on `sim`, `protocol`
     and `content` only. [The hash-map ban](../research/GLOSSARY.md#the-hash-map-ban-in-the-sim-crate) and the clock and
     float bans live in `crates/sim/clippy.toml` (`disallowed-types`: std `HashMap`, `HashSet`, `Instant`,
     `SystemTime`; `disallowed-methods`: `Instant::now`, `SystemTime::now`, the `algebraic_*` methods and every std
     `f32` and `f64` method outside ADR 0004 part 7's allow-list and the exact methods the rules themselves use, such as
     `is_finite`, `is_nan`, `to_bits` and `total_cmp`), at deny, with `iter_over_hash_type` as a backstop. The float
     ban is written as the allow-list's complement because ADR 0004 part 7 is an allow-list (R1-DET-REC2,
     r1-simulation-core.md, Recommendations, lines 2146–2150): it names every other method on the pinned std's `f32`
     and `f64` pages and is read again at every Rust bump (part 2). So it catches more than the transcendental
     functions: `powi`, `hypot` and `cbrt` carry std's note that their precision varies by platform and Rust version
     (R1-DET-F9, r1 lines 648–654, inferred there; rust-reviewer confirmed the note for these three and for
     `to_degrees` and `to_radians` on std 1.99.1's `f32` page at the step-10 review), and `min`, `max` and `clamp` may
     return either zero for equal signed zeros, while `clamp` panics on bad bounds (R1-DET-F14, r1 lines 679–683). The
     sim uses three helpers of its own for those three, written as plain comparisons, with one defined result for
     equal zeros and no panic (proposed). `suboptimal_flops` and
     `imprecise_flops` are allowed with a comment, and `cargo clippy --fix` never runs on `sim`; the same set covers the
     parts of `content` the sim calls. The only escape from
     the hash-map ban is a reviewed `#[allow]` with a written reason for a map that is only looked up and lies outside
     the serialized sim state (resolution 29). Map types from the sim's dependencies join `disallowed-types` from a list
     rust-reviewer makes from `cargo tree` of `sim` once ADR 0004's dependencies are fixed. If clippy cannot name std's
     float methods, a reviewed text scan of `sim` for those method names is the fallback check (R5-RUSTLINT-OQ1). The
     clock ban (`Instant::now` and `SystemTime::now` in `disallowed-methods`, at deny) also lives in the `clippy.toml`
     of `room`, `protocol`, `client-endpoint` and `sim-wasm`, for two reasons: these crates run in the Worker, where
     std's clock calls panic on `wasm32-unknown-unknown` (std's own source, read by rust-reviewer at the step-10
     review; not in the research) and the target's `-Cpanic=abort` makes that panic a trap (R1-BND-F19, r1 lines
     1019–1025), which native tests never see; and a timer counted in ticks replays exactly, as SLICE-ORDER's
     replayable matches need. So room timers count ticks, with their lengths in data; the client endpoint takes time
     only as the values the host passes in (part 5); only `server` reads `Instant`. Which timers are tick-counted in
     `room` and which run on wall time in `server` (such as the join deadline and the per-connection rate limits) is
     ADR 0001's (parts 5 and 11). In
     `protocol`'s decoders and `content`'s loader, `unwrap_used`, `expect_used` and `indexing_slicing` are at deny,
     allowed in tests. In the sim they are at deny in the module that applies commands, where values a client or
     content can set enter (an offer and its choice, a purchase's unit id, a team or slot setting), so those values
     reach the rest of the sim only through checked lookups (ADR 0004 part 20; PM-0004-60 fuzzes it); elsewhere the
     sim's indices come from its own pools. rustfmt runs with stable
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
   | Soak and load | bots as headless Rust protocol clients | a room at every mode of the test-mode list for 30 minutes, up to the largest mode's N (48) | nightly, and before each playtest |
   | Fuzz | cargo-fuzz on every decoder and the content loader | 0 panics, 0 allocations above the size limits | nightly, Linux |
   | Benchmarks | the `devtools` tick bench; ADR 0003's client harness | [sim step time](../research/GLOSSARY.md#sim-step-time) and frame cost, with the method stated | change-sets touching the sim or the render path; never a gate pass or fail |

   - **The determinism suite's tools:** a `devtools` replay runner, built as the server builds the sim, plays a
     recorded match and writes one [state hash](../research/GLOSSARY.md#state-hash) per tick; the gate runs it twice in
     two processes and once as Wasm in Node; Playwright opens a replay page that runs the `sim-wasm` Worker in Chromium,
     Firefox and WebKit and returns the hash stream; every Wasm run uses the exact file G6 ships and G7 measures (after
     wasm-bindgen and wasm-opt, checked by its hash); a comparer reports the first mismatching tick and a field-level
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
     simulator on simulated time behind the thin transport interface (latency, jitter, loss, reordering, duplication, a
     rate limit, and a packet-fate log; per direction, latency and jitter split half each way and loss is as the
     profile defines it, ADR 0001 part 13: the Advisor's answer to OQ-0001-16, NET-PROFILES, 2026-11-03, where R5 had
     proposed the round trip's half split), running the real server code and headless bot clients in one process; L2,
     an impairment wrapper around the server's UDP socket under the real quinn endpoint for real browsers over
     WebTransport, with a delaying TCP proxy for WebSocket.
   - **Soak and load:** bots as headless Rust clients that speak the real protocol (on `client-endpoint`, part 3); one
     nightly run per mode of
     [the test-mode list](../research/GLOSSARY.md#the-test-mode-list) over L1 at P2, 30 minutes each, and one L2 run with
     4 real browser clients among the bots. On CI's shared runners a run is judged only by what needs no particular
     machine: desyncs, panics and hangs, the resident-memory slope, queue lengths, bytes per client per second, plus
     legacy TL13's match metrics; [server tick time](../research/GLOSSARY.md#server-tick-time) p50,
     [p99](../research/GLOSSARY.md#time-statistics) and max is reported there, and judged against
     [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) (ADR 0001's) only from a run on one vCPU
     of the reference server (PM-0002-72), since timings on shared runners are not hardware results. Before each
     playtest one 3-hour soak runs on the reference server, so its tick times count (its VM: the bench's path below).
   - **Flakes:** no retries (`retry: 0`, `retries: 0` written out; `failOnFlakyTests` and `forbidOnly` on in CI); one
     committed fixed `PROPTEST_RNG_SEED` in the gate, fresh seeds nightly with every failure committed to
     `proptest-regressions` with its fix; simulated time only in sim and network tests; browser tests wait on state
     through a test-hook API and drive page time with Playwright's clock; no shared global state in Rust tests; every
     failure keeps its seed, recording and trace as CI artefacts, for the retention ADR 0006 sets for failure
     artefacts (part 3: 30 days, proposed there; R5 leaves it to infra-engineer, r5 line 2083),
     and gets an issue that carries the seed, so the failure still replays after its artefacts expire.
   - **The bench method** (critic rank 36): the tick bench is a mode of the `devtools` replay runner. It replays a
     recording of [the stress scene](../research/GLOSSARY.md#the-stress-scene), times every sim step and every phase
     with the host's monotonic clock outside the sim (the sim itself reads no clock), and reports mean, p50, p95, p99 and
     max per phase and in total over 3 runs per build, on
     [one vCPU](../research/GLOSSARY.md#the-measuring-unit-one-vcpu) of [the reference server](../research/GLOSSARY.md#the-reference-server)
     natively and in Wasm in the Worker on the minimum laptop, as R1-TICK-REC2's P1 and P2 ask. It reports; it never
     passes or fails the gate. The bench needs the reference server from slice 1, before ADR 0006's fleet exists, and CI
     holds no token for it (timings on shared runners are not hardware results): it runs on a CCX23 created per bench
     session in the spikes' own Hetzner project and deleted after it, ADR 0006 part 8's cost line (proposed there: 20
     bench hours a month, 20 × 0.1386 = 2.77 EUR, R6-HOST-F15, r6 lines 150–158). Who creates and deletes it is
     answered (SPIKE-MACHINES, Mustafa, 2026-11-03; OQ-0006-17): "I create and delete the VMs myself for each run;
     sessions get SSH access only and no Hetzner token until the security baseline"; that this covers the bench and the
     3-hour soaks as well as the spikes is the advisor's reading (2026-11-03). The 3-hour soak before each playtest uses
     the same path. No bench crate, coverage tool or cargo-nextest is adopted: none was read.
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
   | G5 | dependency rules | part 3's rules (cargo-deny bans and `cargo tree`); `multiple-versions` and `wildcards` at deny with a reviewed skip list; the same-feature check for `sim` across `server`, `devtools`, `navbake` and `sim-wasm`, each built by the command CI and the release use; cargo-deny's `sources` at deny (crates.io only) |
   | G6 | build | native release (`server` once it exists, `devtools`); `sim-wasm` to Wasm; Vite build of both entries |
   | G7 | budgets | Wasm bytes (raw and gzip) and JS bytes per entry against the committed budget file; fail above it (the `sim-wasm` size is recorded, not gated, until its first limit is set: part 4) |
   | G8 | player build | walk Vite's manifest from the player entry: no chunk from `editor`; the built HTML of both entries holds no inline script or style, and no Worker or Wasm loads from a `blob:` or `data:` URL (part 7's CSP-ready build) |
   | G9 | tests | `cargo test --workspace` (unit, property with the fixed seed, legacy goldens, insta), Wasm tests in Node, Vitest |
   | G10 | determinism | the short recordings: native in two processes, Wasm in Node, Wasm in Chromium, Firefox and WebKit, each Wasm run on the exact file G6 ships (after wasm-opt, checked by its hash); first mismatch reported |
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
     and GPU times come only from the reference machines (ADR 0003). By the research's own findings every CI browser
     probably gets no WebGPU adapter and falls back to WebGL 2: Chrome on Linux offers WebGPU only on Intel Gen12+ GPUs
     (R3-WGPU-F17, r3-rendering.md lines 159–162), Firefox not on Linux (R3-WGPU-F18, r3 lines 164–169), any browser
     with no adapter falls back (R3-GL2-F12, r3 lines 279–283), and headless Chrome renders on "SwiftShader Device
     (Subzero)" (R3-MEASURE-F8, r3 lines 648–653); Playwright's WebKit on Linux, and whether headless Firefox and
     WebKit get WebGL 2 at all, were not researched. Then the default run repeats the `forceWebGL` run and CI never
     runs the WebGPU backend, so a shader that fails to build for WebGPU, a WebGPU validation error or a pipeline made
     mid-match only on WebGPU would pass CI. The foundation therefore records, per CI engine, the backend it gets (or
     none). If none gets WebGPU, the boundary spike tries a Chromium job with SwiftShader's WebGPU adapter turned on by
     flags, for counts and errors only (not researched; R3-MEASURE-F8 names GPU flags only for hardware runs); until a
     CI route works, every change-set that touches the render path carries a WebGPU smoke run (console errors, ADR
     0003's counts, the backend string) on a reference machine of ADR 0003 as stated evidence. No GPU runner is
     proposed (a paid larger runner; ADR 0006 and Mustafa for any paid plan).
   - **No arm64 leg** in the determinism job: the first fleet is x86-64 (ADR 0006, part 5), and native AArch64 joins the
     hash test before ADR 0006 may pick Arm hosts (ADR 0004). The netcode proof test's room server runs on the x86-64
     reference server and its only Arm machine is the load generator (ADR 0001's LG, CAX21 VMs), so the bot clients'
     aarch64 build is that spike's own; the playtest soak's bot VM is named when the first soak is planned, and if it
     is Arm, `devtools` gains an aarch64 build target then (a build, not a determinism leg), with the soak's desyncs
     judged by ADR 0001's rule that no pass mark's verdict rests on native aarch64 agreeing with x86-64 or Wasm.
   - **Crash reporting and the end-to-end network rule** (critic rank 34): test builds carry no crash-reporting
     endpoint, so G12's rule (only the local test server) holds unchanged; the optional Sentry check of a planted Wasm
     trap runs in the boundary spike, not in the gate. The job that uploads debug files for Sentry (R6-OBS-REC3) runs
     only on builds that are deployed, in its own GitHub environment that holds the Sentry token, with no checkout of
     pull-request code, shaped like the deploy job it follows and under that job's guard (part 10: `production`'s
     `main`-only branch rule, or `preview`'s required reviewer for a preview); its token scope is ADR 0006's (part 13,
     item B9).
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
    environment named `preview`, holds the client-host token for previews, never checks out or runs pull-request
    code, runs only for pull requests from this repository, has the permissions `contents: read`, `actions: read`,
    `deployments: write` and `pull-requests: write`, turns automatic package caching off, sets the environment's URL to
    the preview link and posts or updates one comment with it. Fork pull requests get the full CI and no preview. No
    `pull_request_target`, no `workflow_run`. One gap remains: a `pull_request` run takes its workflow from the pull
    request's own merge commit (the contrast R5-PREV-F2 draws with `pull_request_target`, r5 lines 770–774; the
    security review's reading at step 10), so any branch of this repository, which agent sessions push, can rewrite
    the deploy job and use its token. That token is account-scoped (R5-PREV-F10, r5 lines 823–830), and the deploy
    command's `branch` option that finding quotes lets one deploy name any branch, so the token can probably reach
    production too (an inference; the client host's production rules were not read). Until the security baseline
    rules on CI tokens, ADR 0006's interim position for B9 (part 13) closes the gap: `preview` has a required reviewer,
    so each preview deploy waits for Mustafa's approval ("a job cannot access environment secrets until one of the
    required reviewers approves it": R5-PREV-F6, r5 lines 796–804), unless OQ-0006-13 puts production where the preview
    token cannot reach it; whether a session, which posts as Mustafa's account (CLAUDE.md), could approve a pending
    deploy was not read and is part of OQ-0006-13. **Production** is deployed only by its own job, on pushes to
    `main`, from the artifact `main`'s CI built and tested, in its own environment `production` that holds its own
    client-host token
    behind a deployment-branch rule admitting `main` alone ("Deployment branches and tags are available for all public
    repositories": R5-PREV-F6; the repository is public: r5 lines 752–754); when a release goes out beside running
    rooms is ADR 0006's. The build generates the `_headers` file for
    [the client host](../research/GLOSSARY.md#the-client-host): [the production header set](../research/GLOSSARY.md#the-production-header-set-and-the-preview-header-set)
    for production and the preview header set for previews (`X-Robots-Tag: noindex`), with COOP and COEP only in a
    build made with the isolation flag on; after deploying, the job fetches the preview's HTML and `.wasm` and fails if
    a header is missing. The close job and the weekly sweep that delete previews are workflows here; the policy (24 h,
    7 days) and PREV-DELETE are ADR 0006's. **Multiplayer previews** (from slice 3; behind the access wall under
    PREVIEW-WALL (a)): CI's native job builds the room binary like any other artifact; a delivery job shaped like the
    deploy job, in its own environment with the same required reviewer as `preview` (B9's interim position), hands
    that binary to the staging game server's supervisor and receives the current and the next certificate hash; the
    preview deploy job writes both into the preview's own files, which the page fetches from its own HTTPS origin (ADR
    0001, part 10). One hash would go stale: a self-signed certificate lives at most 14 days (R2-TRN-REC5,
    r2-netcode.md, Recommendations, Brief question 4, lines 2700–2703), and R2-TRN-REC5 mark 2 asks that new
    connections succeed with the new hash within one reload (r2 lines 2720–2721), which a static file cannot give. So
    on the staging VM the supervisor, not each room process, holds the certificate, rotates it every 7 days and makes
    the next one a full period ahead, and a preview older than one rotation is redeployed (ADR 0006, part 4; proposed,
    not researched). If OQ-0006-5 instead gives the staging server, on both transports, the browser-trusted certificate
    its `wss` endpoint needs anyway (the kind R2-TRN-REC5 gives production, r2 line 2704), a multiplayer preview pins
    no hash and the delivery job hands over only the binary. The delivery token's scope, and whether
    pull-request-built native code may run on the staging game server, wait for the security baseline (ADR 0006, part
    13, item B9).
    - Why: a fork run gets no secrets, and the two triggers that hand them out are avoided; splitting build and deploy
      keeps the token away from every build script and dependency; deploying the artifact CI tested means the link shows
      the tested build; a production job on `main` alone keeps every release behind `main`'s review and gate, and the
      one path left to the preview token, a rewritten job on a branch, is what the required reviewer closes until the
      baseline (ADR 0006, B9; OQ-0006-13).
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
12. **Its spike: the Rust ↔ TypeScript boundary spike**, in wave 1 after the netcode proof test, the sim core spike and
    the 60 Hz run, one heavy spike at a time (SPIKE-ORDER (b), Mustafa, 2026-11-03), and before the foundation, since
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
| **R5-LAYOUT-REC2 plus an I/O-free `room` crate, a `client-endpoint` crate with no bindings and a `navbake` crate above `sim` (chosen)** | low–medium: three more crates and three more dependency checks | – | the player's Wasm carries neither the bake nor the server's I/O | the same decoders and room checks run locally and online | none | new | one room logic for the local server and the room process; one client endpoint for the Worker and the headless clients; the bake shared by CI, map save and the editor | not researched as a placement (no Options section offers a crate for the room logic, the client endpoint or the bake; SLICE-ORDER condition 1 and R4-XT-OQ5 leave it to this record): proposed, and allowed by the Advisor's ruling at the stop (OQ-0002-8 (a), 2026-11-03) |

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
| **C1. A small message per tick (chosen, posted once per drawn frame with the newest input: part 5)** | low | one message per tick (per frame as chosen) | constant work | – | edge counters keep a press from being lost or doubled (row GP2) | the never-wired command (row CS1) gets wired | works without isolation | one message per tick (per frame as chosen) |
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
- **A Worker, one block out per tick and the newest input in per frame** wins over the main thread and over shared
  memory because the tick leaves the render thread, the work per tick is constant in N, and nothing needs cross-origin
  isolation, nightly Rust or a fixed memory maximum. The strongest argument against it: one copy per tick and a block
  whose delivery waits
  for the page's message queue, which no source measured (R1-BND-REC2's option B2 notes it as "to be measured"); the
  boundary spike measures it against R1-BND-REC2's marks 2 and 3, and the SharedArrayBuffer ring is the measured
  alternative.
- **The local server in the same Worker** wins because SLICE-ORDER condition 1 allows no offline-only path, and one
  Worker build then serves local play, prediction and online play. The strongest argument against it: in a local match
  the Worker runs the authority's sim step, the loopback encoding and decoding, and the client endpoint's prediction
  and block writing, a load no research measured (ADR 0001, trade-off 5, counts it in [the client's sim share](../research/GLOSSARY.md#the-clients-sim-share)); the
  boundary spike measures it, and OQ-0002-3 picks between one Wasm instance and two.
- **Three crates beyond R5's list** (`room`, `client-endpoint`, `navbake`) cost three more dependency checks; without
  them the local server cannot run in the Worker, the headless clients of ADR 0001's network marks and of the soaks
  would need a native build of the bindings crate or a second copy of the client endpoint, and the bake's generator
  would risk shipping in the player's Wasm. No research compared other placements, so the first foundation milestone
  that builds them is also their test; rule 3 allows proposing them (OQ-0002-8, answered (a) by the Advisor,
  2026-11-03).
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
  determinism leg); Mustafa answers OQ-0002-1; the security baseline rules on CI tokens, or OQ-0006-13 is answered
  (B9's interim position ends).
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

## Open questions
- **OQ-0002-1. Does the v2 editor still open by double-clicking its HTML file?**
  - Why it matters: EDITOR_V2 §5 keeps the rule, and the v2 editor is a Vite-built page whose validation panel and Test
    run a module Worker with the bake and map checks in Wasm (part 11); whether such a page loads from `file://` was not
    researched (R1-DATA-OQ11). A `file://` page counts as a secure context on MDN's table (R1-DATA-OQ11's second
    verifier pass, r1 lines 2819–2823), so that rule alone does not settle it.
  - Options: (a) keep the rule, if the built editor's module Worker and Wasm load from `file://` in Chrome, Edge,
    Firefox, Safari and Brave; (b) change the rule: the editor opens from a local server (`vite` or `vite preview` on
    localhost) or from its preview link, which changes EDITOR_V2 §5; (c) a double-click editor that edits the document
    only, with the bake and map checks run elsewhere (not researched).
  - Decider: the boundary spike (does it load?), then Mustafa if EDITOR_V2 §5's rule must change.
  - What decides it: the spike's load test of the built editor entry from `file://` in the four browsers and Brave.
  - Research ids: R1-DATA-OQ11, R1-XT-OQ10, R3-XT-OQ3, R5-XT-OQ8.
- **OQ-0002-2. Where do the legacy gate and tools live after the move, and where does the gate v2's script live?**
  - Why it matters: today's gate must keep guarding critical fixes to `legacy/`, the legacy oracle must keep producing
    golden values, and R5's working name for the gate v2 is today's path.
  - Options: (a) both gates and the legacy tools in `tools/`, the legacy ones learning the `legacy/` path; (b) the
    legacy gate and tools inside `legacy/`, `tools/` for the gate v2.
  - Decider: the phase 1 restructure plan (the main session with docs-keeper), as R5 names it.
  - What decides it: which layout keeps R5-LAYOUT-REC6's marks (today's gate passes on `legacy/`, the four Phase 0
    mutation proofs still fail on the right checks, `docs/legacy-tuning.json` is reproduced byte for byte) with the
    fewest path changes, and where the legacy docs go (row KT21).
  - Research ids: R5-LAYOUT-OQ5, R5-GATE-OQ3.
  - Answered in part, 2026-11-11, by its decider, the phase 1 restructure plan (foundation step 2.1, plan revision 3;
    the main session with docs-keeper), with its reasons below; the evidence is in
    `docs/reports/2026-11-11-foundation-2-1b.md`.
    - (1) The legacy gate and tools stay in `tools/`, option (a), learning the `legacy/` path: 2 gate files edited in
      step 2.1a (PR #50: `tools/gate.mjs`, `tools/smoke.mjs`) and 2 kit files in step 2.1b (CLAUDE.md,
      `.claude/agents/verifier.md`), against option (b)'s cost (the guard's gate-file lists and its npm check, the 15
      `tools/` allow rules in `.claude/settings.json` (counted 2026-11-12), CI's `--prefix tools` and both gate steps,
      and kit lines in about 8 more files). R5-LAYOUT-REC6's marks hold (E2, E3 and E4 in the 2.1b report).
    - (2) The legacy docs stay in `docs/`, marked legacy (row KT21): no path changes, against, by the plan's count of
      2026-11-10, 27 lines in 12 files (8 of them protected), 2 broken relative links and about 25 dangling references
      for `legacy/docs/`. START_HERE gets rewritten for v2 later (KT21's mark "redesign"), not archived.
    - (3) The gate v2's script path is left to step 2.3's plan, which edits the guard anyway; so the question stays
      open, marked "answered in part", by the register's own rule. That status is Mustafa's REG-TRUE (b) of 2026-11-11
      (`docs/DECISIONS.md`, the entry of 2026-11-11, 21:39:44 UTC); the answer itself is the plan's.
- **OQ-0002-3. In a local match, do the local server and the client endpoint share one Wasm instance?**
  - Why it matters: it sets the Worker's Wasm calls per tick and its memory in local play, and whether a trap in the
    client endpoint also ends the authority (ADR 0004's OQ-0004-7).
  - Options: (a) one instance, with the loopback in Rust, in the protocol crate as ADR 0001's migration plan places
    it: the fewest calls; a trap ends both; (b) two instances in the one Worker, with the loopback's bytes passed
    between them by the Worker's host (TypeScript): a client trap leaves the authority running; a constant few more
    calls per tick and a second copy of the Wasm memory.
  - Proposed start: (a), because it matches ADR 0001's placement of the loopback and keeps the Worker's calls per tick
    within R1-BND-REC1's bound; (b) if OQ-0004-7 needs the authority to survive a client trap and the spike's marks
    below hold for it.
  - Decider: the boundary spike, with ADR 0001 (the interface) and ADR 0004 (OQ-0004-7).
  - What decides it: Wasm calls per tick, the same at N = 12 and N = 48 (R1-BND-REC1 mark 2); the Worker's time per tick
    in a local match on the minimum laptop against the client's sim share (ADR 0004); the memory of each layout.
  - Research ids: none of its own; it applies R1-BND-REC1 mark 2, and R2-TRN-REC4 as ADR 0001 changes it.
- **OQ-0002-4. Does production ever turn on cross-origin isolation, and can a later login popup live with it?**
  - Why it matters: isolation would make the SharedArrayBuffer ring (B3), inputs through shared memory (C2) and the
    `Atomics.wait` pacing (W3) available, and it costs a CORP header on every cross-origin `no-cors` resource and cuts
    cross-origin popups off from their opener (R1-BND-REC5, r1 lines 2309–2325).
  - Options: (a) stay non-isolated (G1); (b) isolate production (G2) when part 5's rule fires (B3 passes R1-BND-REC2
    mark 5, or the timer pacing fails its jitter mark and W3 passes), with the non-isolated path kept for a third-party
    iframe (PORTAL-EMBED, OQ-0002-9); the login VISION_V2 §5 plans then needs a page or a redirect outside the isolated
    game page, unless `same-origin-allow-popups` gives isolation (R1-BND-OQ9).
  - Decider: the boundary spike's results, then this record; for embeds and the login flow under isolation, Mustafa with
    ADR 0006 (R1-BND-OQ5, only if isolation is adopted).
  - What decides it: R1-BND-REC2 mark 5 and ADR 0004's jitter mark for the timer pacing; for the popup, a test of
    `crossOriginIsolated` under `same-origin-allow-popups` in the four browsers.
  - Research ids: R1-BND-OQ9.
- **OQ-0002-5. Does real Safari run on pull requests, and does the end-to-end suite need it?**
  - Why it matters: Playwright's WebKit is not Safari (R5-TEST-F8), so a Safari-only bug reaches players unless real
    Safari runs somewhere. While the repository is public the macOS runner costs nothing: GitHub's runner reference
    lists macOS among the standard runners, whose use "is free and unlimited on public repositories" (the page
    R5-CI-F17 read, r5 lines 673–678, which quoted only its ubuntu rows; its macOS rows were read by infra-engineer at
    the step-10 review). What is not known is the macOS job's wall time against CI's 10-minute mark (PM-0002-56). The
    netcode proof test first checks whether those runners' Safari is new enough for WebTransport and reaches the test
    servers (Mustafa's answer to MACHINES-TIMING, 2026-11-03, in SPIKE_PLAN's machines paragraph); that check is the
    spike's own and leaves this question open, since it asks about pull requests.
  - Options: (a) on pull requests that touch `client`, `net`, `sim-worker` or `sim` (R5-TEST-REC1); (b) nightly only;
    (c) also an end-to-end suite in real Safari through WebdriverIO (not researched).
  - Proposed interim: (b), which R5-XQ-OQ1 says breaks neither part, until the macOS job's wall time is known; the
    cost is not what holds it back.
  - Decider: this record, from the macOS job's wall time, which infra-engineer measures in the foundation; ADR 0006 and
    Mustafa only if the repository stops being public, since a private repository's runners draw on paid minutes (the
    same page, read at the step-10 review).
  - What decides it: the macOS job's wall time against PM-0002-56's 10 minutes; whether WebKit on macOS misses a Safari
    bug that a playtest or Mustafa's checks find.
  - Research ids: R5-XQ-OQ1, R5-TEST-OQ4.
- **OQ-0002-6. Which jump reach do the golden tests pin?**
  - Why it matters: the reach is 3.07 m by the formula and about 2.98 m as legacy integrates it at 60 Hz (rows GW14 and
    GN10, as R5-TEST-OQ3 cites them, r5 lines 1806–1808); with a fixed tick v2 gets one value, and bots and maps are tuned for it.
  - Options: (a) the formula's value (the intended behaviour); (b) legacy's integrated value; (c) whatever v2's
    integrator gives at its tick, with bots and maps tuned to it.
  - Decider: the game-designer with Mustafa, from the loop slice on (a rule question).
  - What decides it: which value LOOP_V2's map rules and the bots are tuned for; until then the jump is not golden
    (R5-TEST-REC3).
  - Research ids: R5-TEST-OQ3.
- **OQ-0002-7. Is wasm-bindgen's new home healthy for a multi-year project?**
  - Why it matters: the boundary rests on wasm-bindgen, which moved to a new organisation in 2025 and broke
    compatibility at least twice in 2026, in 0.2.123 and 0.2.128 (R1-BND-OQ7, r1 lines 2741–2744; R1-BND-F1, F2).
  - Options: (a) keep wasm-bindgen behind the exact pin and the CI mismatch check (part 2), with every bump its own
    change-set; no other binding generator was researched.
  - Decider: a by-day reading of its maintenance issue (#4533) and the guide's governance page, both on the allow list
    (VERIFY-FETCH (a)); this draft fetched nothing.
  - What decides it: what those two pages say about maintainers and release policy.
  - Research ids: R1-BND-OQ7.
- **OQ-0002-8. May this record propose a structure no research Options section offers, when a settled input forces it
  and the research leaves the question to this record?**
  - Why it matters: plan rule 3 says an option the research lacks is labelled "not researched" and cannot be the
    proposed decision. SLICE-ORDER condition 1 runs the local server in the Worker, which needs room logic that builds
    for Wasm, while R5-LAYOUT-REC2 puts rooms inside `server` (r5-toolchain-quality.md, lines 1173–1174); ADR 0001's
    network marks run the client endpoint natively and headless, which needs it outside the bindings crate (part 3);
    and the critic's questions hand structure to this record ("Decided by: ADR-0002"): R4-XT-OQ5, with no answer on
    one side (r4-physics-navigation.md lines 1195–1200, the bake's place), and R5-XT-OQ6, which names both sides (r5
    lines 1996–2001, the multiplayer-preview workflow, whose parts part 10 takes from research recommendations).
  - Options: (a) rule 3 allows it in such cases: the `room`, `client-endpoint` and `navbake` crates stay proposed,
    labelled "not researched", and the first foundation milestone that builds them is their test; (b) rule 3 holds:
    the three become open questions, a short layout research unit answers them before the foundation, and part 3
    keeps R5-LAYOUT-REC2's list until then.
  - Decider: the Advisor, at the stop.
  - Answered by the Advisor (2026-11-03): "(a)." Option (a) is decided: rule 3 allows it in such cases, so the
    `room`, `client-endpoint` and `navbake` crates stay proposed, labelled "not researched", and the first foundation
    milestone that builds them is their test.
  - What decides it: whether rule 3 is meant to cover structure that a settled input forces and the research hands to
    this record.
  - Research ids: none of its own; it concerns R5-LAYOUT-REC2 (changed), R4-XT-OQ5 (the same question as R5-XT-OQ7)
    and R5-XT-OQ6.
- **OQ-0002-9. Could the client run inside a web portal's iframe, and what must this record keep possible for it?**
  - Mustafa (2026-11-04), word for word: "PORTAL-EMBED (new, keep the door open; don't design it): the game may also
    launch on web portals such as CrazyGames and Poki, inside their iframe, sharing our servers and player pool. Add it
    as an open question to the README (and ADR 0002 and 0006) in the docs pass. The client must keep working inside a
    third-party iframe, so the boundary spike may only pick a design that needs cross-origin isolation (the
    SharedArrayBuffer ring, B3) if a non-isolated fallback stays available."
  - Why it matters: his rule binds the boundary spike now (part 5, the isolation bullet). It is read as covering every
    design part 5 can bring in that needs isolation: the ring (B3), inputs through shared memory (C2) and the
    `Atomics.wait` pacing (W3) (the advisor's reading, 2026-11-04, of "a design that needs cross-origin isolation"). For
    W3 the only non-isolated fallback ADR 0004 part 5 keeps is the timer pacing (W2), whose failed jitter mark is what
    would bring W3 in (ADR 0004 part 5, PM-0004-68); if that happens, the question returns to Mustafa with the spike's
    numbers.
  - Options: none weighed ("keep the door open; don't design it").
  - Decider: Mustafa.
  - What decides it: no timing is set.
  - Research ids: none of its own. Related: OQ-0002-4 (whether production ever turns isolation on); R1-BND-OQ5 and
    ADR 0006's OQ-0006-12 (the other direction: third-party content inside the game page); ADR 0006's OQ-0006-23 (the
    web-portal side: launches that share the servers and the player pool).

## What a spike must prove
**The Rust ↔ TypeScript boundary spike** (throwaway, in `spikes/`, SPIKE-SCOPE (a)) runs in wave 1, one heavy spike
at a time, after the netcode proof test, the sim core spike and the 60 Hz run (SPIKE-ORDER (b), Mustafa, 2026-11-03),
and before the foundation. It shares the netcode proof test's client harness and stand-in sim, and the sim core spike's
sim, which exists by then, serves the marks that need a sim beyond the stand-in (the advisor's reading, 2026-11-03).
Its results go into this record before Mustafa accepts it. The candidate marks below are the research's (cited) or are
marked proposed with the reason; test-engineer sets the final table.
1. **The Worker** (part 5; R1-BND-REC1, r1 lines 2252–2264): on the minimum laptop named in ADR 0003 (its
   `hardwareConcurrency` recorded), at N = 12, 24, 36 and 48 soldiers, the page's frame time p95 with the sim in the
   Worker is no worse than with no sim by more than 1.0 ms (the page's cost of the sim: ADR 0003 owns the budget row,
   which states it as a p99, decision map LE-B); the page makes zero calls into Wasm; the Worker makes the same number of
   Wasm calls per tick at N = 12 and N = 48, wasm-bindgen's glue calls included (part 5 batches the newest input and
   the received datagrams, with their arrival times, into one input call), at most 4 for the online client and for a
   local match in one instance
   (OQ-0002-3 (a)), at most 8 for a local match in two instances (b) (proposed: R1-BND-REC1's 4 per instance stepped,
   times two instances); the same build runs in the latest desktop Chrome, Edge, Firefox and Safari, and in Brave (Mustafa's
   browser).
2. **The block** (part 5; R1-BND-REC2, r1 lines 2265–2285): per tick at most one block copy in the Worker and zero
   copies and zero per-entity calls on the page; after warm-up no buffer or typed-array allocation per frame on the
   page, and boundary-caused heap growth under 0.1 MB/s over a 10-minute match (allocation sampling); tick T's block
   readable on the page at most one display frame (16.7 ms at 60 Hz) after the Worker finishes tick T, at p95, on a
   shared clock (`performance.timeOrigin + performance.now()` on both sides); zero lost events, and zero released out
   of order on their entity's timeline (part 5), over a
   10-minute match at the largest mode's N (48); bytes per tick and copy time at N = 48 for a per-entity and a per-field
   layout (R1-BND-OQ6); B2 against B3 on the four browsers: B3 is weighed only if it saves at least 0.5 ms of page time
   per frame or one frame of latency at p95 on the minimum laptop. The field-by-field writer's time is reported beside
   the copy and read against part 5's trigger, a p99 above 0.67 ms in Wasm on the minimum laptop (R1-RR-OQ6).
3. **The command** (part 5; R1-BND-REC3, r1 lines 2286–2296): with injected delay, duplication and reordering of
   commands, zero lost and zero doubled presses over 10,000 scripted presses; in local play, from the key event's
   timestamp to the tick that applies it, at most one tick plus one frame at p95, with the page posting its newest
   input once per drawn frame and every tick running the newest post (part 5); the cursor ray and view time reach the
   Worker in the same message (proposed: part 5 sends no second message).
4. **Generated types** (part 5; R1-BND-REC4, r1 lines 2297–2308; R5-LAYOUT-REC3, r5 lines 1194–1205): CI regenerates
   every type with zero diff against the committed output; a Rust layout change without regeneration fails at build or
   at startup (layout-hash mismatch), proven by a test; `tsc --strict` passes on the generated output with the base
   tsconfig, `erasableSyntaxOnly` included (R5-TSLINT-OQ3); CI fails when the wasm-bindgen CLI and library differ; two
   generator runs on one commit give byte-identical files (R5-LAYOUT-OQ4).
5. **Isolation** (part 5; R1-BND-REC5, r1 lines 2309–2325; R5-LAYOUT-REC5, r5 lines 1219–1232): with the isolation
   flag, `crossOriginIsolated === true` in Chromium, Firefox and WebKit under Playwright on both `vite` and
   `vite preview`, and `false` without it; if production isolation is adopted (OQ-0002-4), `crossOriginIsolated` is
   true in all four browsers on a preview link and a report-only run of a full match shows zero COEP violations.
6. **The connection and the skeleton** (part 6; R1-BND-REC6, r1 lines 2326–2338): the page handles zero network
   messages (a counter); a 100 ms busy loop injected on the page leaves no gap in the Worker's processing of server
   snapshots (Worker timestamps); the local match and the online client run the same Wasm file and the same page code,
   differing only in the mode flag; the Wasm calls per tick, the Worker's time per tick and the memory of one instance
   against two in a local match (OQ-0002-3); and, reported with no limit (the research sets none: part 6), the Worker's
   busy time per tick (p99) online and in a local match at N = 48, and its JS heap and Wasm memory at minutes 5 and 10
   of a match.
7. **The Worker's pacing and the client's sim share** (ADR 0004's marks, measured here as `docs/SPIKE_PLAN.md`
   places them, item 3): ADR 0004's S18 (tick start jitter, read online against the schedule the step returns, part 5;
   smoothness on 60, 120 and 144 Hz displays; hidden tabs, with catch-up counted per wake) and
   R1-TICK-REC2's P2 with the sim core spike's sim, which runs before this spike (SPIKE-ORDER (b)); if the timer
   pacing fails its jitter mark, W3 is measured on an isolated build (OQ-0002-4).
8. **Size and tools** (parts 2, 4 and 7): the `sim-wasm` file's size recorded raw and gzipped, with and without
   wasm-opt, at `opt-level` "s" and "z" (R1-RR-OQ5, R5-CI-OQ4); no pass mark (the research has no number): the
   measurement is one input to the budget's first limit, which the Advisor sets after the content and ability spike
   has measured the content crate's share (part 4); every served file stays under the client host's 25 MiB per-file
   limit; the count of type errors and casts the page needs against three.js's types under the base tsconfig
   (R5-TSLINT-OQ4); `tsc -b` time
   with `skipLibCheck` on and off (R5-TSLINT-OQ6); whether Playwright's clock reaches the Worker (R5-TEST-OQ1).
9. **The editor from `file://`** (part 11; OQ-0002-1): the built editor entry, opened from `file://`, loads its module
   Worker and Wasm in Chrome, Edge, Firefox, Safari and Brave, or the failure is recorded per browser.
10. **Optional, only if Mustafa has created the EU Sentry organisation by then** (ADR 0006): a deliberate Wasm trap in
    the Worker reaches Sentry with Rust function names in the four browsers (R6-OBS-REC3's mark).

**Where a person judges:** Mustafa runs the spike's page and a preview link in Brave on his Windows machine (marks 1
and 9 name Brave); test-engineer adds the Brave checklist with each step's expected result.

**The foundation** is not a spike: its acceptance checks are the milestones of the Migration plan's phase 2
(`docs/SPIKE_PLAN.md`, "The foundation's acceptance checks (not a spike)").

### Pass marks
Written by test-engineer (plan step 4). One row per mark: the Rust ↔ TypeScript boundary spike's first (PM-0002-1 to
PM-0002-34, with PM-0002-77 after PM-0002-14 and PM-0002-78 after PM-0002-25), then the foundation's own acceptance checks (PM-0002-35 to PM-0002-69), then the later milestones of
phase 1's slices (PM-0002-70 to PM-0002-76). Every number is a research mark (cited), arithmetic on research numbers
(shown), a settled input, or marked proposed with its reason. Line numbers are at 9e7db28: r1 is
`r1-simulation-core.md`, r2 `r2-netcode.md`, r3 `r3-rendering.md`, r5 `r5-toolchain-quality.md`, r6
`r6-infrastructure.md`, all in `docs/research/`.

**How to read the table**
- **Machines.** **ML**, the minimum laptop that ADR 0003 names (R3-BUDGET-REC1, r3 lines 1096–1103; the model is
  OQ-0003-1): Windows 11, its own 60 Hz panel unless a row names another display, Chrome, Edge and Firefox in their
  latest stable versions. **MAC**, a macOS machine with the latest Safari, for the rows that name Safari; the draft
  names none, so its model is recorded with each result and it carries no budget. **MP**, Mustafa's PC (Windows, RTX
  5080) in Brave: only what he checks, and the Windows half of the rows that ask for Windows; never a budget result.
  **CI**, GitHub's standard runner (ubuntu-24.04: 4 CPUs, 16 GB, no GPU: R5-CI-F17, r5 lines 673–676): counts,
  booleans and CI's own wall time only, never a hardware time (R3-MEASURE-REC3, r3 lines 1206–1209).
- **BX, the boundary scene** (proposed): [the netcode proof test](../research/GLOSSARY.md#the-netcode-proof-test)'s throwaway sim (movement, hitscan, bots as protocol
  clients) in the Worker, filling the block at the counts of [the stress scene](../research/GLOSSARY.md#the-stress-scene): N soldiers in
  three equal teams in one fight (16v16v16 at N = 48; the scale steps N = 12, 24 and 36 the same way), 15 big
  vehicles of which 4 are very large units of 20 parts, 300 live projectiles, and the tick's events. What the stand-in
  does not simulate (vehicles, projectiles) is a scripted mover that fills its section (proposed: the boundary's cost
  depends on the counts, not on the rules). The page draws the blocks with the shared client harness's plain
  renderer, not the look (the look is ADR 0003's). One script, one fixed seed. **BX-P**: BX with the Worker's step
  padded by a busy-wait to 13.33 ms per tick, [the client's sim share](../research/GLOSSARY.md#the-clients-sim-share) at 30 Hz, so the page is tested against the most
  CPU the Worker may use (proposed: the game's sim, from slice 1 on, may cost up to the share; the sim core spike's
  sim, which exists by then (that spike runs before this one: SPIKE-ORDER (b), Mustafa, 2026-11-03), also runs in
  this Worker, and the BX rows run again with it).
- **The 10-minute match**: BX at N = 48 in local mode (the Worker runs the local server and the client endpoint),
  600 s × 30 = 18,000 ticks. **Online**: the Worker's client endpoint connected to the netcode proof test's room
  process over WebTransport through that test's link impairment, at the profile the row names (R2's profiles, r2
  lines 1987–1990: P0 = 0 ms, no jitter, 0 % loss; P2 = 100 ms round trip, ±20 ms jitter, 2 % loss).
- **H, V and statistics**, as ADR 0003's pass marks define them: H is 3 runs of 600 frames after 300 warm-up frames
  (R3-MEASURE-REC1, r3 lines 1181–1185); frame time is the per-frame time from `performance.now()` in ADR 0003's
  client harness, never the `requestAnimationFrame` timestamp, which has a precision of 1 ms (R3-MEASURE-F7, r3 lines
  641–643); percentiles by nearest rank (p95 over 600 frames is the 570th smallest; over 18,000 ticks p95 is the
  17,100th and p99 the 17,820th); V, a timed run set is valid when its run medians lie within 3 % of each other
  (R3-MEASURE-REC1's mark, r3 lines 1192–1193), and an invalid set is neither a pass nor a fail. A count mark holds on
  every tick or frame (its statistic is the run's max). A time mark that names no statistic is a p99; a one-off
  duration (a build, a check step, a `--prove` run) is a plain limit that every measured run meets
  ([time statistics](../research/GLOSSARY.md#time-statistics), GLOSSARY lines 141–147).
- **U and P.** U, uncapped: the browser's frame-rate limit and vsync off, for every mark that compares two builds
  (on a 60 Hz display two builds that both reach 60 fps would compare as equal); P, presented: vsync on, refresh rate
  recorded, for marks about what a player sees or waits for. The switch per browser is not researched (as in ADR
  0003's pass marks).
- **Timers.** Rows on the production build (the isolation flag off) read `performance.now()` at 100 µs. Reports and
  marks under 1 ms (the copy time, B2 against B3) run on a build with
  [the isolation flag](../research/GLOSSARY.md#preview-isolation-and-the-isolation-flag) on, at 5 µs (R3-MEASURE-F7, r3 lines 641–643). Times across the page and the Worker use one clock,
  `performance.timeOrigin + performance.now()` on both sides, because a worker has its own time origin (R1-BND-REC2
  mark 3, r1 lines 2278–2280; R1-BND-F30).
- **The counting build** (proposed): the spike's measurement build wraps every export of the `sim-wasm` instance in
  the Worker with a counter, and replaces the page's `WebAssembly` entry points (`instantiate`,
  `instantiateStreaming`, `compile`, `Module`, `Instance`), `WebSocket` and `WebTransport` with counting stubs that
  throw. The counters run in both configurations of every comparison, so they cost the same on both sides.
- **Tick and profiles.** The tick is 30 Hz (TICK-30), so a block arrives 30 times a second; no mark here is set at
  60 Hz ([the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) is ADR 0001's). Every network row names its profile; the page-to-Worker command
  shim is not a network (posted messages are reliable), so it has none.
- **Bites.** Each automated mark names a planted defect that must make it fail on the spike's own build before its
  pass counts, because "a check without a failing case can't be shown to work" (R5-GATE-REC3, r5 lines 1727–1728).
  The planted defects are proposed unless a row cites them. **Rule B**, for a budget mark: a busy-wait in the code path
  under test, sized to the mark's measured margin plus 1 ms, must fail it.
- **The foundation's rows** run on CI unless they name a machine; CI times there are what Mustafa and the sessions wait
  for, not hardware results. Each gate check's rows are proven by `--prove`: a mutation must fail exactly its named
  check and nothing else, a control must pass (R5-GATE-REC3, r5 lines 1706–1709).

| mark | what it proves (decision part) | number and statistic | hardware | method and tool | runs and noise | basis |
|---|---|---|---|---|---|---|
| **The Rust ↔ TypeScript boundary spike** | | | | | | |
| PM-0002-1 | The machines and their records exist (part 5; decides R1-BND-OQ3's record) | ML named in ADR 0003 with CPU, GPU, RAM, screen and refresh rate, and `navigator.hardwareConcurrency` recorded in each browser; MAC's model and Safari's version; MP's Brave version; every display's model and refresh rate; every result row names its machine, browser, display and build commit | ML, MAC, MP | a results-log header per machine, written before the first run | checked before the first run and again before the last | R1-BND-REC1 mark 1 (r1, 2259–2261: "model named in the ADR, its `hardwareConcurrency` recorded"); R1-BND-OQ3 (r1, 2728–2731); R3-BUDGET-REC1 (r3, 1096–1103) |
| PM-0002-2 | The page never touches Wasm (part 5: the page "never instantiates Wasm and makes zero calls into it") | 0 calls into Wasm and 0 Wasm instantiations by the page, in every run of this spike | CI: Chromium, Firefox, WebKit under Playwright; ML: Chrome, Edge, Firefox; MAC: Safari | the counting build's page stubs (any call throws and is logged) | every run (a count). Bites: a debug readout that instantiates `sim-wasm` on the page | R1-BND-REC1 mark 2 (r1, 2261–2262) |
| PM-0002-3 | The Worker's calls per tick do not grow with N (part 5; part 6; OQ-0002-3) | the same number of Wasm calls per tick at N = 12 and N = 48 on every tick, with wasm-bindgen's glue calls included and no separate reading (proposed: wasm-bindgen's glue passes bytes through the instance's exported allocator, so the counting build counts those calls): at most 4 for the online client and for a local match in one instance (OQ-0002-3 (a)), at most 8 in two instances (b) | CI (counts); ML: Chrome | BX at N = 12 and N = 48, local mode and online at P2, 1,800 ticks each (one minute, proposed); the counting build logs every tick's calls per export, glue included | 1 run per N and mode (counts repeat). Bites: a per-soldier accessor (one call per soldier per tick: the count grows with N); a no-op export called 5 times per tick (over 4) | R1-BND-REC1 mark 2 as written (r1, 2261–2263: "the worker makes the same number of Wasm calls per tick at N = 12 and N = 48 (at most 4)"); part 5 (one input call per tick carries the command and the received datagrams; the step returns where the outgoing datagrams and the block lie); 8 = 4 per instance × 2 instances (the draft's proposal, spike item 1) |
| PM-0002-4 | The sim in the Worker does not cost the page its frame (part 5; the page's cost of the sim) | the page's frame time p95 with the sim in the Worker no worse than with no sim by more than 1.0 ms, at N = 12, 24, 36 and 48 on BX, and at N = 48 on BX-P. **Which statistic gates (decision map LE-B): both, each in its own spike:** this p95 difference here, and ADR 0003's p99 of the page's own sim work (at most 1.0 ms per frame, PM-0003-20) in the render budget bench. They measure different things: this row also catches what timing the page's code cannot see (garbage collection, the Worker sharing the laptop's cores and power); passing one does not imply the other | ML: Chrome, Edge, Firefox | 10 interleaved pairs per N (sim, no sim, sim, no sim …), each run H's 300 + 600 frames, mode U. "No sim": the same page drawing the same scene from BX's recorded blocks (1,200 blocks, 40 s) stepped from memory by the page's own 30 Hz clock, with no Worker (proposed: the baseline must draw the same scene, or the difference would measure the drawing). The mark reads the median of the 10 paired differences of p95 (the mean of the 5th and 6th smallest) | 10 pairs per N and browser (proposed: 1.0 ms is about the spread V allows between two runs, 3 % of a 28 ms median being 0.84 ms, so 3 runs cannot resolve it; ADR 0003's PM-0003-37 uses the same design); V on each configuration's 10 runs; rule B in the page's block handler | R1-BND-REC1 mark 1 (r1, 2259–2261); GLOSSARY, the client's sim share (lines 214–218); decision map N12 and LE-B; R3-BUDGET-REC2's median row (r3, 1124) |
| PM-0002-5 | One build runs in every target browser (part 5) | the 10-minute match runs to its end with 0 console errors and 0 page errors, 18,000 ± 1 ticks, and one block received per tick | ML: Chrome, Edge, Firefox; MAC: Safari; MP: Brave (spike checklist, step 1) | the page's error hook logs every console and page error; a status line shows ticks, blocks and errors | 1 match per browser; 0 errors in each. Bites: a `console.error` 2 s after load (R5-GATE-REC3's G12 mutation, r5 line 1723) | R1-BND-REC1 mark 3 (r1, 2263); Brave: the draft's spike item 1; 600 s × 30 = 18,000 |
| PM-0002-6 | One copy per tick, none on the page (part 5: B2) | per tick: at most 1 block copy in the Worker, 0 copies and 0 per-entity calls on the page, exactly 1 message from the Worker to the page; after a planted `memory.grow` between two ticks, the next tick's block equal byte for byte to the same tick's block in a run without it (part 5: the host takes a fresh view of `memory.buffer` after every call) | CI: Chromium, Firefox, WebKit (counts); ML: Chrome | the 10-minute match; the Worker's copy routine counts its calls; after each `postMessage` the Worker checks that the sent buffer is detached (`byteLength` 0: a transfer, not a clone); the page checks that every typed-array view's `buffer` is the received buffer and counts messages per tick; per-entity calls are 0 by PM-0002-2 | every tick of 1 match per browser. Bites: the block posted without its transfer list (a structured clone); a host that caches its view of `memory.buffer` across calls (after the grow the old view is detached and reads zero bytes) | R1-BND-REC2 mark 1 (r1, 2276–2277); the draft, part 5 (the fresh view); the Rust review of 2026-11-02, finding 6 |
| PM-0002-7 | The boundary makes no garbage on the page (part 5) | after the first 300 frames, 0 `ArrayBuffer` and 0 typed-array allocations per frame on the page; allocation attributed to the boundary's page code (the block handler, view binding, the pool, and the browser's message event per block) under 0.1 MB/s over the match, at most 60 MB in 600 s; the heap after a forced garbage collection at minutes 5 and 10 reported | ML: Chrome (the sampling profiler is Chrome's; Firefox and Safari are not measured by this method) | the 10-minute match; an inspector sampling heap profile that includes objects collected by minor and major collections (legacy TL15's method, `tools/perfbots.js` lines 9–10), allocations attributed by function; garbage collection and heap through the Chrome DevTools Protocol (proposed tool, as ADR 0003's PM-0003-24) | 3 matches (proposed: ADR 0003's run count); each passes. Bites: one planted 64-byte typed array per frame in the handler must be reported (proposed: sampling proves "zero" only if it sees a small leak) | R1-BND-REC2 mark 2 (r1, 2277–2278); 0.1 MB/s × 600 s = 60 MB |
| PM-0002-8 | The block reaches the page within a display frame (part 5) | from the Worker finishing tick T (its step returns, before the copy) to the page's views on T's block being bound, at most one refresh period of the display (16.7 ms at 60 Hz) at p95, the 17,100th of 18,000 | ML on its 60 Hz panel: Chrome, Edge, Firefox; MAC: Safari (reported) | the 10-minute match, mode P, both ends on the shared clock; the page's main thread loaded to ADR 0003's main-thread row (16 ms per frame on ML at Low) by a busy-wait in its frame callback (proposed: on the spike's light page the mark could not fail, and a real frame keeps the main thread that busy) | 3 matches per browser; each p95 within the mark. Bites: the Worker posting tick T's block only after stepping tick T + 1 | R1-BND-REC2 mark 3 (r1, 2278–2280); R3-BUDGET-REC2's CPU row (r3, 1125) |
| PM-0002-9 | No event is lost or reordered (part 5) | over the 10-minute match at N = 48: 0 lost and 0 doubled events, and 0 released out of order on their entity's timeline (part 5): within each timeline (the local player's own, predicted and owner events; each remote entity's) events arrive in tick order, each remote spatial event in the first block whose delayed time reaches its tick, and own, predicted and owner events in the block of the tick that emitted them | CI: Chromium, Firefox, WebKit (counts); ML: Chrome, Edge, Firefox; MAC: Safari | every event carries a per-match sequence number, its tick and its timeline (fields of the spike's event section, proposed); the page checks each sequence number arrives exactly once and, per timeline, in tick order, and logs each block's delayed time beside the events it carries; the Worker's emitted total equals the page's received total at the end (a sequence across all timelines no longer runs in order: remote events wait for the delayed time) | 1 match per browser. Bites: the page returning a block to the pool unread when two blocks arrive between frames; remote events released on arrival, into the newest block, instead of at their delayed time | R1-BND-REC2 mark 4 (r1, 2280–2281); N = 48 (MODES-3 (a)); the draft, part 5 (the event section's timing, ADR 0001 part 7's rule); the engine review of 2026-11-02, finding 1 (ADR 0001's PM-0001-61 times the drawn result) |
| PM-0002-10 | What the block weighs and what one copy costs (part 5; decides R1-BND-OQ6 and R1-RR-OQ6; critic rank 37) | reported, no pass number: bytes per tick (median and max) and the Worker's copy time (median, p95, p99) at N = 48, for a per-entity and a per-field layout of every section; the field-by-field writer's time beside the copy, read against part 5's trigger: the derive or `unsafe` is weighed only if the writer's p99 in Wasm on ML exceeds 0.67 ms (5 % of 13.33 ms, proposed by rust-reviewer); passes when every cell holds a number | ML: Chrome, Edge, Firefox; MAC: Safari | BX at N = 48, a 10-minute match per layout, on the isolation-flag build (5 µs timers); `performance.now()` around the writer and around the copy | 1 match per layout and browser | R1-BND-OQ6 (r1, 2737–2740); R1-RR-OQ6 (r1, 2951–2954); the draft's part 5 and spike item 2 |
| PM-0002-11 | The transferred block against the shared ring (part 5: B2 against B3; decides R1-XU-OQ3 and OQ-0002-4's first trigger) | B3 is weighed only if, on ML, it lowers the page's boundary time per frame (the block handler and view binding, p95) by at least 0.5 ms, or PM-0002-8's latency (p95) by at least one refresh period (16.7 ms at 60 Hz); otherwise B2 stands; Safari reported | ML: Chrome, Edge, Firefox; MAC: Safari (reported) | BX at N = 48; the B2 and B3 builds both with the isolation flag on (proposed: B3 needs isolation, which also changes timer resolution and may change scheduling, so both are compared on one footing); page time with `performance.now()`; latency by PM-0002-8's method | 10 interleaved pairs of H runs per browser (proposed, as PM-0002-4: 0.5 ms is below V's spread); the median of the paired differences; V | R1-BND-REC2 mark 5 (r1, 2282–2284); R1-BND-REC5 (r1, 2309–2311); R1-XU-OQ3 (r1, 2854–2860) |
| PM-0002-12 | Presses are never lost or doubled (part 5: one command per tick, C1) | 0 lost and 0 doubled presses, and 0 lost and 0 doubled releases, over 10,000 scripted presses, some shorter than one tick | CI: Chromium, Firefox, WebKit under Playwright (counts) | a seeded shim on the page's posting side delays each command by a time drawn uniformly from 0 to 100 ms and sends 10 % of commands twice, which reorders most neighbours (proposed: a spread wider than one 33.33 ms tick; R1 gives no amounts); the shim logs how many it delayed, doubled and reordered (each above 0, or the run is invalid); the Worker counts the presses and releases it applies through the edge counters. No profile: the network's own press mark is ADR 0001's (R2-SYNC-REC2 at P3) | 1 run per engine (counts). Bites: a Worker that reads only the held-button bitmask and ignores the edge counters; a Worker that takes the last post to arrive rather than the newest by sequence number (a reordered older post rolls the running totals back) | R1-BND-REC3 mark 1 (r1, 2292–2294) |
| PM-0002-13 | A press reaches the sim quickly in local play (part 5) | from the key event's `timeStamp` to the start of the tick that applies it, at most one tick plus one frame, 33.33 + 16.67 = 50.00 ms, at p95 (the 9,500th of 10,000) | ML on its 60 Hz panel: Chrome, Edge, Firefox; MAC: Safari (reported) | local mode, BX at N = 48, mode P, the page loaded as in PM-0002-8; 10,000 key events sent through Playwright, headed, so `timeStamp` is the event's own (Safari through safaridriver); the Worker stamps each tick's start on the shared clock | 1 run per browser. Bites: the page posting once per block instead of once per drawn frame (part 5's arithmetic: up to two tick periods less the block's delivery), plus one extra tick of buffering in the Worker | R1-BND-REC3 mark 2 (r1, 2294–2295); the draft, part 5 (the newest input posted once per drawn frame); the netcode review of 2026-11-02, finding 7; TICK-30 (1000 / 30 = 33.33 ms); a 60 Hz display (1000 / 60 = 16.67 ms) |
| PM-0002-14 | Aim travels in the same message (part 5) | 10,000 of 10,000 posts carry the cursor ray and the view time of the frame drawn, and every network command its aim point from the newest post; the Worker receives 0 messages of any other kind from the page during play | CI (counts) | PM-0002-12's run without the shim; the Worker counts message kinds | 1 run. Bites: a separate aim message | the draft's part 5 (proposed in its spike item 3: part 5 sends no second message) |
| PM-0002-77 | The Worker casts the aim against what the page drew (part 5; decision map I-19: the cursor ray against what the page draws) | for 1,000 of 1,000 commands whose page-side ray hits a remote soldier's hit volume at least 5 cm inside its edge (proposed margin: at the edge either side may win by rounding), the Worker's cast names the same entity, and its hit point lies within 1 cm of the page's (proposed: the protocol's position step, ADR 0001 part 6) | CI: Chromium (counts and distances); ML: Chrome | a test build: the page also casts each posted ray against the hit volumes at the instance transforms of the frame it drew (three.js maths on the page, never Wasm), and logs the entity and point with the post's sequence number; the Worker logs its cast per command with the sequence number of the post it used; a scripted cursor rests on a remote soldier sprinting at 9 m/s (ADR 0001's target course), in local mode and online at P2 | 1 run per mode. Bites: the Worker casting against the newest block instead of the view time's blend (up to one tick off, about 0.3 m at 9 m/s) | the draft, part 5 (the post's view time; the cast against the state published for it); decision map I-19; ADR 0001 part 5 (the aim point); the engine review of 2026-11-02, finding 4 (PM-0002-14 counts the message, ADR 0001's PM-0001-48 and -51 judge hits against the Worker's own state, so neither sees this seam) |
| PM-0002-15 | Generated types never drift (part 5; the gate's G4) | regenerating every generated type (the ts-rs output, the block's layout and its hash) gives 0 changed bytes against the committed files; a field renamed in an exported Rust type without regenerating fails; a hand edit to a generated file fails | CI | the gate's G4 step on the spike's tree; the two planted defects | 1 of 1 each. Control: a rename with regeneration passes | R1-BND-REC4 mark 1 (r1, 2304–2305); R5-LAYOUT-REC3's mark (r5, 1203–1204) |
| PM-0002-16 | A stale layout cannot run (part 5: the layout hash) | a Rust layout change without regeneration fails at build or at the page's startup (the page refuses a block whose layout hash differs), proven by a committed test | CI | the test builds `sim-wasm` with one field added to a section and runs the page's startup check against the old generated layout | 1 of 1 | R1-BND-REC4 mark 2 (r1, 2305–2306) |
| PM-0002-17 | Generated code passes the strict base (parts 5 and 7) | 0 errors from `tsc -b` on `packages/types` with the base tsconfig (`strict`, `erasableSyntaxOnly` included) | CI | `tsc -b` on the generated package | 1 run. Bites, each required: a generated `.ts` file with a class that has a parameter property, and one with an `enum` declaration (R5-TSLINT-REC1's plant, r5 lines 1261–1263; the tsconfig page's `erasableSyntaxOnly` entry lists `enum` declarations and parameter properties among the constructs it rejects: verifier source check, 2026-11-01). An ambient `enum` (`declare enum`, or one in a `.d.ts`) is not claimed either way: its result is recorded for R5-TSLINT-OQ3 (the skeptic check of 2026-11-02 read nothing on it) | R1-BND-REC4 mark 3 (r1, 2306); R5-TSLINT-OQ3 (r5, 1770–1772); R5-TSLINT-F12 (r5, 221–224), its list now confirmed at the source (typescriptlang.org/tsconfig, "Erasable Syntax Only"; the fixcheck-b verifier, finding 1) |
| PM-0002-18 | The binding tool matches its library (part 2; G4) | CI fails when the wasm-bindgen CLI and the library version in `Cargo.lock` differ | CI | a CLI one patch version off the lock | 1 of 1. Control: both at the lock's version pass | R1-BND-REC4 mark 4 (r1, 2306–2307); R5-CI-REC3 mark 1 (r5, 1544–1546); R5-GATE-REC3's G4 row (r5, 1716) |
| PM-0002-19 | The generators are deterministic (part 5; decides R5-LAYOUT-OQ4) | two generator runs on one commit give byte-identical files; 20 runs give 20 identical sets (proposed: R5-LAYOUT-OQ4 asks about parallel test threads writing into one folder, a race that may show only sometimes; R5-TEST-REC6's 20-run rule) | CI | `cargo test` with its default parallel threads, plus the layout generator, run 20 times on one commit; every generated file's sha256 compared | 20 runs | R5-LAYOUT-REC3's mark (r5, 1203–1204); R5-LAYOUT-OQ4 (r5, 1759–1760); R5-TEST-REC6's mark (r5, 1503) |
| PM-0002-20 | The isolation flag works in development (part 5: G3) | `crossOriginIsolated` true with the flag on and false with it off, on the page and in the Worker, in all 12 cases: Chromium, Firefox and WebKit, each under `vite` and under `vite preview`, flag on and off | CI (Playwright) | Playwright reads `crossOriginIsolated` on the player page and from the Worker | 1 run per case. Bites: a flag that sets COOP but not COEP (must read false) | R5-LAYOUT-REC5's mark (r5, 1229–1231); R1-BND-REC5 (r1, 2309–2311) |
| PM-0002-21 | Only if production ever isolates (part 5; OQ-0002-4) | `crossOriginIsolated` true on a [preview](../research/GLOSSARY.md#preview) link in Chrome, Edge, Firefox and Safari; 0 COEP violations in a report-only run of a full match; recorded: `crossOriginIsolated` under COOP `same-origin-allow-popups` in the four browsers (decides R1-BND-OQ9's test) | ML: Chrome, Edge, Firefox; MAC: Safari | a preview built with the isolation flag on; the 10-minute match under the report-only header (proposed reading of "a full match"); violations collected by a `ReportingObserver` on the page and in the Worker (proposed tool, not researched) | 1 match per browser; only if OQ-0002-4 adopts isolation | R1-BND-REC5's mark (r1, 2321–2324); R1-BND-OQ9 (r1, 2747–2752) |
| PM-0002-22 | The page never handles a network packet (parts 5 and 6: H2) | 0 network messages handled by the page over a 10-minute online session at P2 | ML: Chrome, Edge, Firefox; CI: Chromium, Firefox, WebKit against a local room process at P0 (counts) | the counting build's page stubs for `WebSocket` and `WebTransport`; the Worker connects and the session plays | 1 session per browser. Bites: the page opening the WebSocket fallback itself | R1-BND-REC6 mark 1 (r1, 2334–2335) |
| PM-0002-23 | A busy page does not stall the connection (part 5: H2) | with a 100 ms busy loop injected on the page once every 3 s (100 loops), no interval between two snapshots the Worker processes, among those overlapping a busy loop, longer than two snapshot intervals (2 × 1000 / 30 = 66.67 ms) at P0; the same reported at P2 | ML: Chrome, Edge, Firefox | online; the Worker stamps every processed snapshot; the page logs each busy loop's start and end on the shared clock (proposed reading of "no gap": a path through the page would show the whole 100 ms) | 1 run per browser and profile. Bites: snapshots relayed through the page | R1-BND-REC6 mark 2 (r1, 2335–2336); P0 and P2 (r2, 1989); 30 Hz snapshots (TICK-30) |
| PM-0002-24 | One build for local and online play (part 6) | the local match and the online client load byte-identical files (the same `.wasm`, page and Worker scripts by sha256), differing only in the mode flag | CI | both modes started from one build; every loaded file's hash taken from the browser's network log | 1 run | R1-BND-REC6 mark 3 (r1, 2336–2337) |
| PM-0002-25 | One Wasm instance or two in a local match (part 6; decides OQ-0002-3) | reported for (a) one instance and (b) two: calls per tick at N = 12 and 48 against PM-0002-3's marks; the Worker's time per tick (median, p95, p99) beside the client's sim share (13.33 ms at 30 Hz); each instance's Wasm memory after 10 minutes; for (b), gated: a trap planted in the client endpoint leaves [the authority](../research/GLOSSARY.md#the-authority)'s tick advancing and its [state hash](../research/GLOSSARY.md#state-hash) stream unbroken (proposed: the reason (b) exists, OQ-0004-7) | ML: Chrome; CI (counts) | BX in local mode, a 10-minute match per layout at N = 12 and 48; the Worker's time from `performance.now()` around each tick's Wasm work; memory from each instance's `memory.buffer.byteLength` | 1 match per layout and N; the trap test 1 of 1 | the draft's OQ-0002-3 ("What decides it"); R1-BND-REC1 mark 2 (r1, 2261–2263); GLOSSARY, the client's sim share (lines 214–218) |
| PM-0002-78 | What the Worker costs as a whole (part 6: reported, no limit until these numbers exist) | reported, no pass number: the Worker's busy time per tick (all its work: the host's message handling, its Wasm calls and the block's post; median, p95, p99) at N = 48, online at P2 and in a local match; its JS heap and each instance's Wasm memory at minutes 5 and 10, the heap read after a forced garbage collection and set beside ADR 0003's PM-0003-24 flat rule (at most 0.5 MB of growth from minute 5 to 10), for reading, not as a pass; passes when every cell holds a number | ML: Chrome | BX at N = 48: a 10-minute match in local mode and a 10-minute online session at P2; busy time from `performance.now()` at the start and end of each of the Worker's tasks (its message handlers and its timer callback); the heap through the Chrome DevTools Protocol on the Worker's target (proposed tool, not researched), Wasm memory from `memory.buffer.byteLength` | 1 run per mode | the draft, part 6 (the Worker as a whole has no research budget; this record sets any limit from these numbers) and spike item 6; GLOSSARY, the client's sim share (lines 214–218: the step only); ADR 0003's PM-0003-24; the perf review of 2026-11-02, finding 13 |
| PM-0002-26 | The Worker paces the tick (part 5 with ADR 0004's part 5: ADR 0004's S18; decides R1-TICK-OQ2, R1-BND-OQ2 and the pacing half of R1-XU-OQ4) | ADR 0004's PM-0004-68 to PM-0004-70, measured once for both records: their numbers, definitions, machines, runs and bites stand, and this row sets none of its own (ADR 0004's part 5 decides the pacing). Restated: foreground Worker tick start jitter p99 at most 4 ms over 18,000 ticks (10 minutes at 30 Hz); a constant-speed unit's per-frame on-screen displacement with a standard deviation at most 5 % of its mean, on 60, 120 and 144 Hz displays; after 60 s and after 6 minutes hidden, in local play no tick ran while hidden (the tick counter equal before and after) and play resumes where it stopped, and online, within 0.5 s of the tab's return, the client has applied the server's state and its commands are accepted again (PM-0004-70's proposed definition of "back in step"), never running more than 7 ticks in one frame | as PM-0004-68 to PM-0004-70: jitter and the hidden tab on ML (Chrome, Edge, Firefox) and MAC (Safari); smoothness on RD, ADR 0003's recommended desktop (Chrome and Firefox on the displays on hand, each recorded), on ML (Chrome at its own rate) and on MAC (Safari) | the setup PM-0004-68 to PM-0004-70 name: the boundary spike's Worker and page with the sim core spike's sim at N = 48; online against the netcode proof test's server at P2. Jitter: the Worker logs each tick's scheduled and actual start (p99 by nearest rank, the 17,820th of 18,000). Smoothness: a scripted mover at constant speed; the page logs its drawn position every frame; mode P. Hidden: on `visibilitychange`, the tick counter and the state hash are logged at hiding and at showing; ticks per frame counted. "More than 5 minutes" read as 6 (PM-0004-70's proposal; the reason: Chrome's intensive throttling starts at 5 minutes, so the run sees a minute of it) | as PM-0004-68 to PM-0004-70: jitter, 3 runs per browser; smoothness, 3 runs of 60 s per display and browser; hidden, 5 hides per duration and browser; each meets the mark. Bites, as theirs: ticks counted per timer callback instead of from elapsed time; the newest tick drawn without interpolation; a Worker that simulates the whole hidden gap | ADR 0004's PM-0004-68 to PM-0004-70 and S18; R1-TICK-REC5's marks (r1, 2109–2114: 15 ticks at 60 Hz, restated as 7 at 30 Hz by ADR 0004's part 5); R1-BND-OQ2 (r1, 2724–2727); R1-XU-OQ12 (r1, 2903–2906) |
| PM-0002-27 | The pacing fallback, only if the timer pacing fails (part 5; OQ-0002-4's second trigger) | only if PM-0002-26's jitter mark fails in a browser: W3 (`Atomics.wait`) on the isolation-flag build meets the same jitter mark in that browser; then OQ-0002-4's rule applies | as PM-0002-26 | PM-0002-26's jitter method on the isolated build | as PM-0002-26; conditional | R1-TICK-REC5 (r1, 2097: W3 "the measured alternative if W2 fails the jitter mark"); R1-XU-OQ4 (r1, 2861–2865) |
| PM-0002-28 | The sim fits the client's sim share in this Worker (ADR 0004's S2; R1-TICK-REC2 P2) | ADR 0004's PM-0004-8, measured here once for both records: its numbers, its one reading of "holds 30+ fps on Low" (ADR 0003's minimum-laptop frame rows, median at most 28 ms and p95 at most 33.3 ms, mode U), its machines, runs and bites stand, and this row sets none of its own (as PM-0002-26 does for pacing). Restated: p99 [sim step time](../research/GLOSSARY.md#sim-step-time) at most 13.33 ms at 30 Hz in Wasm in this Worker on ML while the page meets that frame row for the whole run; N for the pass from R1-TICK-OQ7 (a design question), the scale steps 12, 24, 36 and 48 reported meanwhile | ML: Chrome, Firefox | the sim core spike's sim in this Worker (that spike runs before this one: SPIKE-ORDER (b), Mustafa, 2026-11-03); step time from the Worker's `performance.now()` around the step; frame times in mode U (PM-0004-8's); the page draws with the plain renderer until ADR 0003's renderer runs at Low, and the run repeats with it then (proposed: "on Low" names ADR 0003's tier); run length and count as ADR 0004's pass marks set them | as ADR 0004's S2 | R1-TICK-REC2 P2 (r1, 2054–2057); GLOSSARY lines 214–218 (40 % of 33.33 ms = 13.33 ms); ADR 0004 S2 and PM-0004-8; the perf review of 2026-11-02, finding 5 (one mark, one reading) |
| PM-0002-29 | What the Wasm file weighs (part 4; with the sim core spike, an input to R1-RR-OQ5, which the Advisor decides; feeds R5-CI-OQ4) | reported, no pass number: the `sim-wasm` file's bytes raw and gzipped, with and without wasm-opt, at `opt-level` "s" and "z", with `lto` off and `"fat"` and `codegen-units` at its default and at 1, with parry3d once the sim core spike's sim is in, Brotli beside gzip; the shipped file measured as shipped (after the wasm-opt step the release keeps, its command line recorded for part 4's flag check); the wasm-opt version and the panic strategy (abort, the target's default) recorded; gated: every served file under 25 MiB (26,214,400 bytes). This measurement and the content and ability spike's share of the `content` crate (ADR 0005, OQ-0005-CD2) are the two inputs from which the Advisor sets the budget file's first limit (part 4); until then G7 records the `sim-wasm` size and does not gate it | CI | the spike's release builds; sizes read from the files, gzip at level 9 and Brotli at level 11 (proposed: what the client host serves a Brotli-capable browser was not read) | 1 build per setting (sizes repeat) | R1-RR-OQ5 (r1, 2946–2950); R5-CI-OQ4 (r5, 1833–1834); R6-HOST-REC1's mark (r6, 642; decision map N37); R5-GATE-REC4 (r5, 1737–1738); the draft's part 4 and spike item 8; 25 × 1,048,576 = 26,214,400; the Rust review of 2026-11-02, findings 7 and 8; the fixcheck-c verifier, finding 3 |
| PM-0002-30 | The tool facts the foundation needs (parts 7 and 8; decides R5-TSLINT-OQ4, R5-TSLINT-OQ6, R5-TEST-OQ1) | recorded: the type errors and the casts the spike's page needs against three.js's types under the base tsconfig; `tsc -b` time with `skipLibCheck` on and off (warm, median of 3 each); whether Playwright's clock reaches the Worker, per engine (yes or no) | CI | `tsc -b --extendedDiagnostics` (proposed); a Playwright test that advances the clock and reads a Worker timer | 3 runs per `skipLibCheck` setting; 1 per engine | R5-TSLINT-OQ4 (r5, 1773–1775); R5-TSLINT-OQ6 (r5, 1779–1780); R5-TEST-OQ1 (r5, 1802–1803) |
| PM-0002-31 | How content files reach the Worker in development (part 5, content files in development; decides R1-DATA-OQ8, which `docs/SPIKE_PLAN.md` item 3 and ADR 0005's CD-7 give this spike) | recorded per browser: whether Vite's hot update reaches the module Worker; then, by that route or by R1-DATA-REC5's page relay, a saved file's bytes in the Worker within 1 s of the save in 20 of 20 saves (ADR 0005's CD-7 mark) | ML: Chrome, Edge, Firefox | `vite` dev server; a script saves a content file 20 times; the Worker stamps arrival on the shared clock against the file's modification time (proposed) | 20 saves per browser | R1-DATA-OQ8 (r1, 2811–2812); R1-DATA-REC5 (r1, 2545–2547); ADR 0005 CD-7; the draft's part 5 ("whether Vite's hot update reaches the Worker directly is PM-0002-31's to measure"); `docs/SPIKE_PLAN.md`, item 3 (the boundary spike: "ADR 0005's content files reaching the Worker") |
| PM-0002-32 | Does the built editor open from `file://` (part 11; OQ-0002-1) | recorded per browser: its module Worker and Wasm load, or the failure with its console message; passes when all five have a result | ML: Chrome, Edge, Firefox; MAC: Safari; MP: Brave (spike checklist, step 6) | a minimal built editor entry (its own HTML, a module Worker, a Wasm file) opened by double-click; its status line names what loaded | 1 opening per browser | the draft's OQ-0002-1 and spike item 9; R1-DATA-OQ11 (r1, 2817–2823) |
| PM-0002-33 | A Wasm trap reaches error reporting with its panic message (parts 4 and 9; the Sentry half only if Mustafa has created the EU Sentry organisation) | a deliberate panic in the Worker's sim, with a known message: the trap's report carries the panic hook's message and location (ADR 0004 part 20's fixed buffer, read through a fresh view: part 4), in the page's own error log in Chrome, Edge, Firefox and Safari (4 of 4), always; and the report arrives in Sentry with Rust function names and that message (4 of 4), when Sentry exists | ML: Chrome, Edge, Firefox; MAC: Safari | a trap triggered by a test command; the Sentry event read in its web view | 1 trap per browser; the Sentry half conditional. Bites: a host that reads the hook's buffer through a view taken before the instance's memory grew (it reads zero bytes) | R6-OBS-REC3's mark (r6, 765–766); the draft, part 4 (the hook's buffer read after a trap); the Rust review of 2026-11-02, finding 9 |
| PM-0002-34 | Mustafa's run in Brave (rule 12; parts 5 and 11) | every step of the spike checklist below recorded; 0 steps "not yet" | MP, Brave on Windows | the spike checklist | one session | plan rule 12; the draft's "Where a person judges" |
| **The foundation's acceptance checks** (the draft's migration plan, phase 2) | | | | | | |
| PM-0002-35 | The legacy game stays guarded after the move (part 1; phase 2.1) | today's gate passes on `legacy/`; the four Phase 0 mutation proofs (a deleted function, an unguarded editor call, a syntax error, a cross-file duplicate) each still fail on the right checks | CI; the session's machine | `node tools/gate.mjs` on the restructure branch, then each planted defect (the path as OQ-0002-2 places the tools) | 1 run each | R5-LAYOUT-REC6's mark (r5, 1243–1245) |
| PM-0002-36 | The legacy oracle still gives golden values (part 1) | `node tools/preview.js legacy eval tools/tuning-dump.js` reproduces `docs/legacy-tuning.json` byte for byte | the session's Linux machine | `cmp` of the output with the committed file | 2 runs (R5-TEST-REC3: the dump "was byte-identical over two runs") | R5-LAYOUT-REC6's mark (r5, 1245); R5-TEST-REC3 (r5, 1444–1445) |
| PM-0002-37 | A fresh clone builds and tests offline (part 2; phase 2.2) | from a fresh clone on the CI runner, `cargo test --workspace --locked` and `npm ci && npm test --workspaces` pass with no network access after the install steps; a fresh runner builds native and `wasm32-unknown-unknown` from `rust-toolchain.toml` alone | CI | the install steps, then the tests with the network cut off (proposed tool: a network namespace with no interfaces; not researched) | 1 run | R5-LAYOUT-REC1's mark (r5, 1166–1168); R5-CI-REC2 mark 1 (r5, 1532–1533) |
| PM-0002-38 | Every tool is the pinned one (part 2) | `rustc`, `cargo clippy`, `rustfmt`, Node, TypeScript and the wasm-bindgen CLI print versions equal to their pins in every job's log; the same commit gives the same lint result on two CI runs a week apart | CI | the aggregate job reads the version lines from the logs (proposed) | every run; the lint pair once | R5-CI-REC2 mark 3 (r5, 1534–1535); R5-RUSTLINT-REC6's mark (r5, 1380–1381); R5-LAYOUT-REC1's mark (r5, 1168) |
| PM-0002-39 | Setup failures stop with exit 2 (parts 2 and 9: G0) | each a `--prove` mutation: deleting `rust-toolchain.toml` fails G0 with exit 2 and never uses another Rust; a pull request that edits `Cargo.toml` without `Cargo.lock` fails at the first cargo step; a missing Playwright browser gives exit 2 with the setup command; control: the toolchain bumped consistently in the file and in CI passes | CI; the session's machine | `--prove` | every `--prove` run | R5-LAYOUT-REC1's mark (r5, 1168–1169); R5-CI-REC2 mark 2 (r5, 1533–1534); R5-GATE-REC2's mark (r5, 1704–1705); R5-GATE-REC3's G0 row (r5, 1712) |
| PM-0002-40 | G1 bites (part 7: format) | each fails G1 only: a misformatted `.rs`, a `.rs` with CRLF endings, a misformatted `.ts`; control: formatted code with long string literals passes; after `prettier --write` on the tree ESLint reports 0 problems; the formatted tree is byte-identical on Windows and on Linux | CI; MP (Windows) | `--prove`; both formatters run on both systems and the trees compared by hash | every `--prove` run; the two-system check once | R5-GATE-REC3's G1 row (r5, 1713); R5-TSLINT-REC3's mark (r5, 1290–1291); R5-RUSTLINT-REC5's mark (r5, 1372–1373) |
| PM-0002-41 | G2 bites (parts 4 and 7: Rust lint) | each fails G2 only, with the reason in the message: a `HashMap` field in `sim`; `map.values().sum()` over a std map in `sim`; `Instant::now()` in a tick function; `x.sin()` on an `f32` in `sim` (clippy, or the reviewed text scan if clippy cannot name it); `x.powi(2)`, `deg.to_radians()` and `a.min(b)` in `sim`, each failing on its own (methods outside ADR 0004 part 7's allow-list, which std marks non-deterministic or which may return either signed zero); `Instant::now()` in `room` and in `client-endpoint` (the clock ban there: part 7); `.unwrap()` and `buf[i]` in a decoder; a planted correctness bug; a planted `style` warning; an `unsafe` block; a crate without `[lints] workspace = true`; `+fma`, `-C target-cpu=native` or `relaxed-simd` in the build flags of `sim`, `room`, `client-endpoint`, `sim-wasm` or `server`, and `-ffm` (`--fast-math`) in the wasm-opt step's command line (part 4); controls pass: `Instant::now()` and a `HashMap` in `server`, `.unwrap()` in a `#[test]`, and in `sim` the three min, max and clamp helpers and the allowed `abs` and `fract` | CI | `--prove` | every `--prove` run | R5-GATE-REC3's G2 row (r5, 1714); R5-RUSTLINT-REC1's mark (r5, 1318–1320), REC2's (1336–1339), REC3's (1351–1352); the draft's part 4 (the build-flag and wasm-opt checks) and part 7 (the float allow-list's complement, the clock ban in four crates); the Rust review of 2026-11-02, findings 1, 2 and 7 |
| PM-0002-42 | G3 bites (parts 3 and 7: TypeScript) | each fails G3 only: one violation per strict flag (an unchecked index read, `x = undefined` into an optional field, a missing `override`, a fall-through `case`, a path without `return`, a class with a parameter property and an `enum` declaration, each for `erasableSyntaxOnly` (both required, as in PM-0002-17: R5-TSLINT-REC1's `enum` and the construct the tsconfig page gives as the flag's example), a value import used only as a type); an `eval(…)` call and a `new Function(…)` in `client` (ESLint's core `no-eval` and `no-new-func`, part 7's CSP-ready build); an unawaited promise; a switch over a generated union missing one kind while having a `default`; an import of `editor` from `client`; a bare `event`; rendering or DOM code imported by `sim-worker` or `net`; an import from `legacy/` or `spikes/`; control: `import type` from `types` in `editor` passes; the clean tree has 0 errors and 0 warnings | CI | `--prove` | every `--prove` run | R5-TSLINT-REC1's mark (r5, 1261–1263); R5-TSLINT-F12 (r5, 221–224); R5-TSLINT-REC2's mark (r5, 1279–1281); R5-GATE-REC3's G3 row (r5, 1715); the draft's parts 3 and 7 (`no-eval` and `no-new-func`: the security review of 2026-11-02, finding 11, through ADR 0006's interim position for B3) |
| PM-0002-43 | G4 bites (part 5) | PM-0002-15's two defects and PM-0002-18's CLI one patch off each fail G4 only; control: regeneration after a rename passes | CI | `--prove` | every `--prove` run | R5-GATE-REC3's G4 row (r5, 1716) |
| PM-0002-44 | G5 bites (part 3: who may depend on whom) | each fails G5 only: `use` of the transport crate in `sim`; `tokio` or `quinn` added to `sim`; `tokio` added to `room`; `navbake` or `server` reachable from `sim-wasm` (each when that crate enters); wasm-bindgen or a transport crate added to `client-endpoint`; a git dependency in any crate (cargo-deny's `sources` at deny, crates.io only: part 3); a wildcard version; a second version of a crate outside the reviewed skip list; a server-only feature on a maths dependency shared with `sim` (the resolved feature lists from `cargo tree -e features` then differ between any two of the `server`, `devtools`, `navbake` and `sim-wasm` builds); controls pass: proptest as a dev-dependency of `sim`, and a dev-only feature on `devtools` that touches no `sim` dependency | CI | `--prove` | every `--prove` run | R5-LAYOUT-REC2's mark (r5, 1189–1192); R5-LAYOUT-REC4's mark (r5, 1216–1218); R5-GATE-REC3's G5 row (r5, 1717); the draft's part 3 (the `client-endpoint` crate; `sources`; the same-feature check across four builds); the engine, security and Rust reviews of 2026-11-02 (EA-8, SA-10, RR-10) |
| PM-0002-45 | G7 bites (parts 4 and 9: budgets as data) | a constant table that pushes the Wasm build over a limit planted in the budget file for the run fails G7 (the `sim-wasm` limit itself is set later, by the Advisor: part 4; until then G7 records that size and gates the rest of the file); raising a budget without changing the budget file fails G7; a change inside the budget passes; the file's history shows every raise with its reason | CI | `--prove`; `git log` of the budget file | every `--prove` run | R5-GATE-REC3's G7 row (r5, 1718; "its size is set once REC4's budget exists"); R5-GATE-REC4's mark (r5, 1743–1744); the fixcheck-b verifier, finding 3, upheld by the skeptic |
| PM-0002-46 | G8 bites (parts 3 and 11: the player build) | a dynamic `import()` of the editor from `client` fails G8; an inline `<script>` (and an inline style) in the built HTML, and a Worker or the Wasm loaded from a `blob:` or `data:` URL, each fail G8 (part 7's CSP-ready build); controls: the editor entry importing `client` code passes, and the Worker and the Wasm loaded as files pass; the built player page loads no file from `editor` | CI | `--prove`; Vite's manifest walked from the player entry through `imports` and `dynamicImports` | every `--prove` run | R5-LAYOUT-REC2's mark (r5, 1191–1192: "only the manifest walk sees a path built at run time"); R5-LAYOUT-REC5's mark (r5, 1231); R5-GATE-REC3's G8 row (r5, 1719); the draft, part 7 (the CSP-ready build, checked in G8); the security review of 2026-11-02, finding 11 |
| PM-0002-47 | G9 and each test layer bite (part 8) | each fails its own layer only, caught by the gate or by the nightly job the draft's table names: a rule change that breaks a property; a v2 value moved beyond a golden's tolerance (from slice 1); a changed insta snapshot; a byte-order swap in one encoder (the boundary layer only); a page error in deploy (end-to-end only); control: a refactor with equal outputs passes | CI | `--prove`; one planted defect per layer, the benchmark layer excepted (it never decides the gate) | every `--prove` run | R5-GATE-REC3's G9 row (r5, 1720); R5-TEST-REC1's mark (r5, 1407–1409) |
| PM-0002-48 | G10 bites, and it is quick (part 8: the determinism suite's tools; the rules are ADR 0004's) | R1-DET-REC6's three planted divergences (one std `sin`, one `HashMap` iteration, one presentation random draw inside the sim) are each reported at the first tick they change the state hash, with a field-level diff; control: a presentation random draw outside the sim passes; G10 takes under 3 minutes on the CI runner (a plain limit) | CI | `--prove`; G10's time from the job log | every `--prove` run; time over 5 runs (proposed: R5-CI-REC5's count) | R5-TEST-REC2's mark (r5, 1427–1429); R5-GATE-REC3's G10 row (r5, 1721) |
| PM-0002-49 | L1 is exact and true to its profiles (part 8: the network simulator; G11) | the same seed and profile give an identical packet-fate log and identical per-tick state hashes on two runs; over 1,000,000 packets per profile in each direction, measured loss, mean one-way delay and jitter each within 5 % (relative) of the per-direction values of P0–P4 and B1, as part 8 splits latency and jitter and ADR 0001 part 13 reads the loss (the advisor's reading, 2026-11-03), as PM-0001-3 reads them (proposed count: see the note under the table); G11's one-minute run per profile takes under 2 minutes in total (a plain limit) | CI | L1 headless in Rust; loss, delay and jitter computed from the fate log; jitter read against the distribution L1's own documentation states (proposed: R2 names none) | 2 runs for the identity; 1 per profile for the statistics; G11's time over 5 runs | R5-TEST-REC4's mark (r5, 1469–1473); profiles (r2, 1987–1990); the split each way: part 8 and ADR 0001 part 13 (OQ-0001-16, answered by the Advisor, 2026-11-03) |
| PM-0002-50 | G11 bites (part 8) | each fails G11 only: a decoder that drops a reordered packet; a snapshot buffer one slot too short for P2; control: reordering at P1 handled passes | CI | `--prove` | every `--prove` run | R5-GATE-REC3's G11 row (r5, 1722) |
| PM-0002-51 | G12 bites (parts 6 and 9: end-to-end) | each fails G12 only: a throw in the deploy path; a console error 2 s after load; an outbound request to an external host; the WebSocket backend broken (from slice 3); a 20 % rise in draw calls against ADR 0003's committed baseline; control: an endless CSS animation on a button passes; G12 also runs once with `forceWebGL` and names the backend on every shot, and records the backend each CI engine got (or none); whether any CI engine gets WebGPU, and the WebGPU smoke run if none does, are ADR 0003's PM-0003-36 and PM-0003-46, the WebGPU route row (the graphics review of 2026-11-02, finding 2) | CI (headless; counts only) | `--prove` | every `--prove` run | R5-GATE-REC3's G12 row (r5, 1723); R5-GATE-REC1's G12 (r5, 1670); ADR 0003's PM-0003-36 |
| PM-0002-52 | The gate proves itself (part 9: `--prove`) | `--prove` passes: every mutation of PM-0002-39 to PM-0002-51 fails exactly its named checks while every other check passes, every control passes, with at least one mutation and one control per check id (G0–G12; G13 from slice 2); one `--prove` run under 30 minutes (a plain limit; decides R5-GATE-OQ5) | CI | `node tools/gate.mjs --prove`, each mutation in a throwaway worktree | on every change to the gate, its checks or its budget file, and nightly | R5-GATE-REC3's mark (r5, 1731–1732); R5-GATE-OQ5 (r5, 1877–1878) |
| PM-0002-53 | One overflow rule on every target (part 4) | a planted `u16` counter that overflows with `+` panics in the dev and the release test builds, native and Wasm (4 of 4); the same counter with `wrapping_add` gives the same value in all four; the checks' cost in sim step time reported with the tick bench's method (no separate limit: the draft sets none; the cost sits inside ADR 0004's sim step share) | CI | a committed test in `sim` run in the four builds | 1 run per build | R5-RUSTLINT-REC4's mark (r5, 1362–1364) |
| PM-0002-54 | Lint and type checks stay fast (part 7) | on the CI runner, warm: `tsc -b` on the whole client under 30 s; typed lint of the whole TypeScript tree under 60 s; clippy on the whole workspace under 30 s, and under 3 minutes cold; plain limits, every run | CI | the step times from the job log | 5 runs each (proposed: R5-CI-REC5's count) | R5-TSLINT-REC1's mark (r5, 1263–1264); R5-TSLINT-REC2's mark (r5, 1281–1282); R5-RUSTLINT-REC1's mark (r5, 1319–1320) |
| PM-0002-55 | The gate is quick enough to run every step (part 9) | `--quick` under 60 s warm; the full gate on a clean tree under 10 minutes warm, on Linux and on Windows; plain limits, every run | Linux: the session's machine (its CPU recorded); Windows: MP (proposed: R5's "recent desktop") | `node tools/gate.mjs`, timed by the shell | 5 runs per system | R5-GATE-REC2's time budget and mark (r5, 1691–1692, 1703–1704) |
| PM-0002-56 | CI is quick (part 9) | the pull-request wall time at most 10 minutes at p50 over 5 runs with warm caches (the 3rd of 5), and at most 20 minutes cold, per job and in total; cold read as each of 3 runs after the caches are deleted (proposed: R5 gives no cold count) | CI | run timings from GitHub's run data | 5 warm, 3 cold | R5-CI-REC5 mark 2 (r5, 1579–1581); decision map N40 |
| PM-0002-57 | One required check that cannot be fooled (part 9) | a TypeScript-only pull request runs and passes the single required check; a pull request with one deliberately failing job, and one with a cancelled job, each turn it red and cannot merge; a skipped job turns it red; removing one check from the workflow fails the aggregate job (its set of check ids differs from the gate's list) | CI (test pull requests) | test pull requests on scratch branches; GitHub's merge box read | 1 of 1 each | R5-CI-REC5 marks 1 and 5 (r5, 1579, 1582–1583); R5-GATE-REC2's mark (r5, 1704); the draft's part 9 |
| PM-0002-58 | Forks get CI and no secrets (parts 9 and 10) | a fork pull request runs every job with no secret available; its deploy job is skipped and no log shows a secret | CI (a fork pull request from a test account, proposed) | the run's logs | 1 of 1 | R5-CI-REC5 mark 4 (r5, 1582); R5-PREV-REC1 mark 2 (r5, 1601–1602) |
| PM-0002-59 | Caches help and stay small (part 9; decides R5-CI-OQ1; critic rank 40) | on a warm cache no dependency crate is recompiled in a pull-request run; the repository's total Actions cache at or below 8 GB with every v2 job cached; reported: warm against cold time per Rust job, a warm `npm ci` against a cold one, the Playwright job's wall time against legacy's smoke job (57–67 s) | CI | `cargo build` output or `--timings`; the repository's Actions cache list | 3 runs each | R5-CI-REC1 marks 1–3 (r5, 1518–1521); R5-CI-REC4's marks (r5, 1553–1555); R5-CI-OQ1 (r5, 1825–1827) |
| PM-0002-60 | The binding tool installs exactly and fast (part 2) | the wasm-bindgen CLI install step installs exactly the lock's version and takes under 30 s (a plain limit) | CI | the step's time from the log | 3 runs (R5's count) | R5-CI-REC3 mark 2 (r5, 1545–1546) |
| PM-0002-61 | No flaky test (part 8: the flake rule) | the gate's whole suite run 20 times on one commit gives 20 identical verdicts; every layer's clean run passes 20 times in a row; a planted timing test (a 50 ms `setTimeout` raced against a fixed wait) is caught by the 20-run check; a planted test that passes only on retry fails the Playwright run | CI | 20 runs on one commit | 20 runs | R5-TEST-REC6's mark (r5, 1503–1505); R5-TEST-REC1's mark (r5, 1409–1410) |
| PM-0002-62 | The per-edit hook is fast (part 7; lands in its kit change-set, not in the foundation) | a planted type error in one file reported by the hook within 10 s at p95 over 20 edits (the 19th of 20), on Mustafa's Windows PC and on Linux; the hook works from any working folder | MP (Windows); Linux: the session's machine | 20 timed edits per system | 20 edits per system | R5-TSLINT-REC4's mark (r5, 1299–1301) |
| PM-0002-63 | Generated types hold in CI (part 5; phase 2.4) | PM-0002-15 to PM-0002-19 hold in the foundation's CI with the three generators of `packages/types` | CI | G4 and its `--prove` mutations; PM-0002-19's 20 runs once | every run | the draft's phase 2.4 |
| PM-0002-64 | The skeleton: one command and one block per tick (part 6; phase 2.5) | in a 1-minute local match (1,800 ticks; proposed: G12's time budget) in G12's three engines: every tick runs the newest posted input (the command built from the newest post by sequence number), the page posts at most once per drawn frame, and blocks received equal ticks run within 1 at start and stop; the page makes 0 Wasm calls and handles 0 network messages (PM-0002-2's and PM-0002-22's stubs); the empty scene names its backend, recorded per engine (none if the engine gets no adapter; the WebGPU route is ADR 0003's PM-0003-46); ADR 0001's loopback milestone and ADR 0004's first-recording milestone pass | CI: Chromium, Firefox, WebKit (counts) | G12 with the test-hook API's counters | every gate run | the draft's phase 2.5 and part 5 (the newest input, posted once per drawn frame); R1-BND-REC1 mark 2 (r1, 2261–2262); R1-BND-REC6 mark 1 (r1, 2334–2335); the netcode review of 2026-11-02, finding 7, and the graphics review, finding 2 |
| PM-0002-65 | The preview link arrives quickly (part 10; phase 2.6) | for a same-repository pull request, the link exists within 5 minutes after the build job ends, or, while `preview`'s required reviewer holds (part 10, ADR 0006's interim for B9), after the reviewer's approval, median of 5 runs | CI | the build job's end (or the approval's time) and the link comment's time, from GitHub's records | 5 runs | R5-PREV-REC1 mark 1 (r5, 1600–1601); decision map N39 |
| PM-0002-66 | The deploy job never runs pull-request code, and a rewritten job reaches no production token (part 10) | the deploy job holds no checkout of the pull request's head and no `npm ci` or `cargo` step; a test pull request that changes `package.json` scripts never runs them in the deploy job; a test pull request that edits the deploy job itself to print a marker and whether its token is set ("present" or "absent", never the value) gets no secret while `preview`'s required reviewer has not approved (the job waits); a job on a pull-request branch that names the `production` environment is refused by its deployment-branch rule and prints "absent" even when approved | CI (the test repository, ADR 0006's TR, for the edited-job tests; proposed: never against the game's own production) | a review of the workflow file; test pull requests whose changed script or edited job would print a marker in the deploy job's log (proposed) | 1 of 1 each. Bites: on the test repository, `production` without its deployment-branch rule (the `production` test must then print "present", or the check is not shown to work) | R5-PREV-REC1 mark 3 (r5, 1602–1603); R5-PREV-F2 (r5, 770–774: a `pull_request` run takes the pull request's own workflow) and R5-PREV-F6 (r5, 796–804: required reviewers; deployment branches); the draft, part 10 (production from `main` alone; the reviewer on `preview`); ADR 0006's B9 interim and OQ-0006-13; the security review of 2026-11-02, finding 1 |
| PM-0002-67 | The header sets are right (part 10) | the deploy job's header check passes, and fails when a header is removed; the production build and a preview built with the flag off carry no COOP or COEP | CI | the deploy job's fetch of the preview's HTML and `.wasm`; the removal as a test pull request | 1 of 1 each | R5-PREV-REC4 marks 1 and 3 (r5, 1636–1640) |
| PM-0002-68 | Isolation on previews only with the flag (part 10) | `crossOriginIsolated` true on a preview built with the isolation flag on, and false on one built with it off, in Chrome, Edge, Firefox, Safari and Brave | ML: Chrome, Edge, Firefox; MAC: Safari; MP: Brave (foundation checklist, step 4) | the page's status line | 1 per browser and build | R5-PREV-REC4 mark 2 (r5, 1638–1640) |
| PM-0002-69 | The foundation is done ([the foundation](../research/GLOSSARY.md#the-foundation)) | an empty scene runs through the full stack in the browser, with CI green and a preview link: PM-0002-35 to PM-0002-68 pass, and Mustafa opens the link in Brave without a login and sees the empty scene | MP, Brave on Windows | the foundation checklist, steps 3 and 5 | one session | GLOSSARY §8, the foundation; R5-PREV-REC5's mark (r5, 1648–1650) |
| **Later, at phase 1's slice milestones** | | | | | | |
| PM-0002-70 | The boundary holds on the real game (slice 1; parts 5 and 6) | PM-0002-2, -3, -4 (at N = 12), -6, -7, -8, -9, -12 and -13 hold on slice 1's real sim and page at N = 12 | as each row | each row's method | as each row | the draft's migration plan, step 3 |
| PM-0002-71 | The legacy goldens bite (slice 1; part 8) | each golden fails when its v2 value moves past its tolerance; CS6's fixture reproduces 20,000 of 20,000 rays within tolerance; a fixture regenerated from legacy at the same commit is byte-identical; the suite prints its goldens per inventory row | CI | `cargo test` on the committed fixtures; the planted moves as `--prove` mutations (G9) | every run | R5-TEST-REC3's mark (r5, 1451–1453) |
| PM-0002-72 | Maps and soaks (slice 2; parts 8 and 9) | G13 fails on a committed map with an unreachable spawn and passes on a valid one; a nightly 30-minute bot room at N = 48 over L1 at P2 runs with 0 panics, 0 hangs, 0 desyncs and a memory slope within 1 MB per 10 minutes after warm-up, and a planted leak (one buffer kept per snapshot) is flagged by the slope check within the run; the server tick time p99 against [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) (16.67 ms at 30 Hz, ADR 0001's) counts only from a run on [one vCPU](../research/GLOSSARY.md#the-measuring-unit-one-vcpu) of [the reference server](../research/GLOSSARY.md#the-reference-server), never from CI | CI (nightly); the reference server for the tick-time mark | `--prove`'s G13 pair; the soak harness of the draft's part 8 | 1 nightly run; the planted leak once | the draft's part 9 (G13's pair); R5-TEST-REC5's mark (r5, 1487–1489); GLOSSARY lines 167–172 |
| PM-0002-73 | Online end to end (slice 3; parts 9 and 10) | G12 passes on both transport backends; a multiplayer preview behind [the access wall](../research/GLOSSARY.md#the-access-wall) plays against [the staging game server](../research/GLOSSARY.md#the-staging-game-server) | CI; MP (Brave) | G12; Mustafa's playtest link | every gate run; one session | the draft's migration plan, step 5; PREVIEW-WALL (a) |
| PM-0002-74 | Every new message crosses the boundary (slice 4; part 5) | the boundary layer's round-trip test covers every message kind of the generated protocol union and every block section (tested kinds equal the union's kinds) | CI | Vitest loading the built `sim-wasm`; the kinds counted from the generated types | every run. Bites: a new message kind without a round-trip case | the draft's migration plan, step 6; R5-TEST-REC1's boundary row (r5, 1391) |
| PM-0002-75 | The editor stays out of the player build (slice 5; part 11) | G8 finds no editor chunk in the player build; the editor's map checks give G13's result on every committed map | CI | G8; the editor's checks run headless against G13's output | every run | the draft's migration plan, step 7 |
| PM-0002-76 | Scale to 48 (slice 6; part 8) | PM-0002-72's soak marks for every mode of [the test-mode list](../research/GLOSSARY.md#the-test-mode-list) (1v1, 4v4, 2v2v2, 6v6v6, 12v12v12, 16v16v16, 24v24, and N = 64 labelled headroom) over L1 at P2, 30 minutes each, with one L2 run of 4 real browser clients among the bots; before each playtest one 3-hour soak with 0 panics, 0 hangs and 0 desyncs, on the reference server, so its tick times count (part 8: the bench's path, a CCX23 per session, OQ-0006-17) | CI (nightly); the reference server for tick times | the soak harness | 1 run per mode, nightly | R5-TEST-REC5 (r5, 1475–1489); GLOSSARY lines 375–387; the draft's part 8 and migration plan, step 8 |

**Notes on two rows.**
- **PM-0002-4, the baseline.** "With no sim at all" (R1-BND-REC1 mark 1) needs the same scene on screen, so the
  baseline replays recorded blocks on the page; it keeps the page's own block reading, so the difference is the
  Worker's sim, the messages and the transfers, which is what PM-0003-20 cannot see.
- **PM-0002-49, the packet count.** At P1's 1 % loss, R5's 100,000 packets give a standard error of
  √(0.01 × 0.99 ÷ 100,000) = 0.031 percentage points, so the 5 % band (0.05 points, 1.6 standard errors) is missed by
  a correct simulator in about 1 seed in 9, and a seeded test then passes or fails by the seed chosen; 1,000,000
  packets give 0.010 points, so the band is 5 standard errors wide.

**When a mark fails.**
- *The spike* passes when every mark that applies passes: a conditional mark (PM-0002-21, -27, -33) only when its
  condition holds; a reported mark (PM-0002-10, -29, -30, -31, -32, -78, and PM-0002-25's report) when its record is
  complete. A failing run is never re-run until green: its data, build commit, machine and browser are kept and its
  cause is found and written down (R5-TEST-REC6, r5 lines 1490–1498). A failed mark goes back to the draft:
  engine-architect revises the decision part the row names, by the draft's own Rollback routes (B3 and C2 behind the
  isolation flag, W3, OQ-0002-3's other layout, a dependency moved to `server`), and the revised part returns to the
  advisor for review and to Mustafa and the Advisor with the measured numbers. A mark's number changes only in the
  draft with its reason, never inside a spike; PM-0002-26's and PM-0002-28's numbers change only in ADR 0004. A run
  set that fails V is neither a pass nor a fail.
- *The foundation's checks* are milestones of accepted work, not a spike: a failing mark blocks its step's change-set
  (the gate and the change-set protocol, step 4) and is fixed there. A mark that proves unreachable as set (a time
  budget, the cache size) goes back to ADR 0002 as a reviewed budget-file change with its reason (R5-GATE-REC4, r5
  lines 1734–1738), or as a revision of the decision part, never loosened quietly.
- A step Mustafa marks "not yet" goes back to the draft (the spike) or to its step's change-set (the foundation), and
  is shown to him again.

**What has no measurable mark here** (one line each)
- Part 1: where the legacy gate and tools live (OQ-0002-2): the restructure's marks hold in either layout.
- Part 2: the weekly latest-dependencies job never blocks; the move to TypeScript 7 is its own change-set;
  Dependabot's handling of SHA pins is tried on one action and recorded, with no number.
- Part 3: "each crate enters with the slice that needs it" holds by review; the lobby's crate waits for ADR 0006.
- Part 4: the server's `panic = "unwind"` with per-connection isolation is ADR 0001's mark (R2-ROOM-REC2 mark 6); the
  restart after a trap is ADR 0004's S20, run with this spike's Worker.
- Part 5: the interpolation buffer in the Worker (critic rank 4, its implementation side) has no mark of its own:
  ADR 0001's interpolation marks run through this Worker.
- Part 7: "dynamic or player-provided text: `textContent`, never `innerHTML`" holds by review until [the security
  baseline](../research/GLOSSARY.md#the-security-baseline) names its lint rule.
- Part 8: the tick bench reports and never gates; fuzzing's 0 panics are ADR 0001's (R2-TRN-REC4 mark 4) and ADR
  0005's marks; real Safari on pull requests waits for OQ-0002-5; the jump reach is not golden (OQ-0002-6).
- Part 9: WebGPU in CI is ADR 0003's count marks (PM-0003-36); no arm64 leg; the Sentry upload job and the advisories
  policy wait for Sentry and for the security baseline.
- Part 10: the delivery token's scope waits for the security baseline (ADR 0006, part 13, item B9); preview deletion
  (PREV-DELETE) is ADR 0006's preview hosting check.
- OQ-0002-7 (wasm-bindgen's health) is a reading, not a measurement.
- No mark here is set at 60 Hz: TICK-30's baseline is 30 Hz, and the 60 Hz margin is ADR 0001's. Its wave-1 run,
  after the sim core spike (SPIKE-ORDER (b), Mustafa, 2026-11-03), is an early signal only, and slice 6's run on the
  real sim decides (TICK-60-SETUP, the Advisor, 2026-11-03: "(a), as an early signal only. The final 60 Hz call is
  measured again with the real sim at slice 6.").
- No time here is a hardware result from a CI runner: CI gives counts, booleans and its own wall time.

#### The spike checklist (Mustafa, in Brave on his Windows PC)
Beside it, the measured marks: the page's frame time with the sim (PM-0002-4), the block's latency (PM-0002-8), the
pacing (PM-0002-26) and the editor from `file://` in the other four browsers (PM-0002-32). His PC is far above the
minimum laptop, so what he sees is whether the boundary works and looks smooth, never the budget. He records "achieved"
or "not yet" (with a note) for each step.

1. Open the spike's link the session sends, with Brave's Shields at his usual setting (note it). Expected: a local
   match of 48 soldiers moving, and a status line with ticks, blocks and errors. Leave it running for 10 minutes (he
   may do something else). Expected: errors 0, blocks equal to ticks, about 18,000 ticks. Record: the status line and
   Brave's version (brave://version).
2. Watch the marked soldier walking at constant speed for 30 s. Expected: it moves evenly, with no stutter, jump or
   hitch. Record: his display's refresh rate (Windows Settings, Display, Advanced display).
3. Switch to another tab for 1 minute, then come back. Expected: the match is exactly where he left it (nothing moved
   while he was away, and the tick count did not grow) and it carries on smoothly.
4. The same with 6 minutes away. Expected: the same as step 3.
5. Only if the session gives an online link (the netcode proof test's server reachable from his PC): play online for
   2 minutes, switch away for 1 minute, come back. Expected: the world is live again within about half a second,
   with no burst of fast motion.
6. Double-click the built editor's HTML file in the folder the session gives him (unzipped). Expected: the page opens
   and its status line says the Worker and the Wasm loaded. Record: loads, or the message it shows instead (this is
   OQ-0002-1's evidence; whether EDITOR_V2 §5's double-click rule changes is his call later, with all five browsers'
   results).

#### The foundation checklist (Mustafa, in Brave on his Windows PC)
1. After the restructure (phase 2.1): unzip a fresh copy of the branch and double-click `legacy/index.html`. Expected:
   the legacy game's menu opens, Deploy starts a match, and it plays as before (move, shoot, pause).
2. After the gate and CI (phase 2.3): in the repository's settings, name the single required check (the session gives
   its exact name) in main's branch protection. Expected: a test pull request shows that check as required.
3. After previews (phase 2.6): open the preview link from the pull request's comment. Expected: no login is asked;
   the empty scene appears and names its backend ("WebGPU" or "WebGL 2 fallback").
4. Open the two previews the session sends, built with the isolation flag on and off. Expected: the status line reads
   "isolated: yes" on the first and "isolated: no" on the second.
5. The foundation's end: on the last foundation pull request, CI shows green and the link of step 3 shows the empty
   scene. Expected: both. Record: done or not yet.


## Migration plan
In the slice order (SLICE-ORDER), with the netcode proof test first and the foundation before slice 1. The legacy game
stays playable throughout: frozen, critical fixes only, guarded by today's gate, in `legacy/` from phase 2 on.
0. **The drafts signed off** by Mustafa as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); ADR 0003 names the
   minimum laptop.
1. **The spikes before the foundation** (`docs/SPIKE_PLAN.md` orders them), one heavy spike at a time (SPIKE-ORDER (b),
   Mustafa, 2026-11-03): the netcode proof test (ADR 0001) first, then the sim core spike and the 60 Hz run, then this
   record's boundary spike, which shares the proof test's client harness. *Done when:* the boundary spike's marks are
   recorded in this record, OQ-0002-3 is answered, and OQ-0002-1 and OQ-0002-4 have their spike results.
   _Pointer (2026-11-11, foundation 2.1a): for foundation steps 2.1 and 2.2, overtaken by Mustafa's decision of
   2026-11-10 (`docs/DECISIONS.md`, the entry "Taken by Mustafa, 2026-11-10: the second account may plan and build
   foundation steps 2.1 and 2.2 now"); for the rest, his words there: "Steps 2.3 to 2.6 keep their order and wait for
   their results."_
   _Pointer (2026-11-13, foundation 2.3's first docs pass): for foundation step 2.3 only, his words of 2026-11-10 quoted
   above are changed by his decision of 2026-11-13 (`docs/DECISIONS.md`, the entry "Taken by Mustafa, 2026-11-13,
   18:47:55 UTC: his decision on step 2.3 (step 2.3 may be planned now), and step 2.3's task"): "Step 2.3 may be planned
   now. The parts of step 2.3 that need no proof test's result may be built before the proof tests report, after my go
   on the plan. Every gate check that needs a result joins later, when that result is in, with its pass marks unchanged.
   Steps 2.4 to 2.6 still wait for their results."_
2. **The foundation** ([GLOSSARY §8](../research/GLOSSARY.md#the-foundation)), one change-set per step:
   1. *The restructure.* The five legacy files move to `legacy/` (OQ-0002-2 places the tools). *Done when:* today's
      gate passes on `legacy/`, the four Phase 0 mutation proofs (a deleted function, an unguarded editor call, a syntax
      error, a cross-file duplicate) still fail on the right checks, `node tools/preview.js legacy eval
      tools/tuning-dump.js` reproduces `docs/legacy-tuning.json` byte for byte (R5-LAYOUT-REC6, r5 lines 1243–1245), and
      Mustafa starts a match from `legacy/index.html` by double-click in Brave.
   2. *The toolchain.* The two workspaces and the pins (part 2), the first crates (`sim`, `protocol`, `room`,
      `client-endpoint`, `sim-wasm`, `devtools`) and packages (`client`, `sim-worker`, `net`, `types`, `e2e`). *Done
      when:* from a fresh
      clone on the CI runner, `cargo test --workspace --locked` and `npm ci && npm test --workspaces` pass with no
      network access after the install steps; `rustc --version` in CI equals the pin; deleting the toolchain file fails
      G0 with exit 2 (R5-LAYOUT-REC1, r5 lines 1166–1169); a pull request that edits `Cargo.toml` without `Cargo.lock`
      fails at the first cargo step (R5-CI-REC2).
   3. *The gate v2 and CI.* G0–G12 (G13 joins with the first committed map, slice 2), `--prove`, the budget file, the
      workflow with its one required check. *Done when:* `--prove` passes with every mutation failing exactly its named
      checks and every control passing (R5-GATE-REC3); a TypeScript-only pull request runs and passes the single
      required check; with warm caches the pull-request wall time is at most 10 minutes at p50 over 5 runs and at most
      20 minutes cold (R5-CI-REC5 mark 2, proposed by R5: VISION_V2 sets no CI target); a failing and a cancelled job
      each turn the required check red; a fork pull request runs every job with no secret; the repository's Rust cache
      stays at or below 8 GB (R5-CI-REC1 mark 2); Mustafa names the required check in his branch protection (his
      setting).
   4. *Generated types.* `packages/types` with the three generators. *Done when:* the marks of the spike's item 4 hold
      in CI.
   5. *The client + local server skeleton.* *Done when:* the Worker's client endpoint and local server exchange
      `Join`, `JoinAccepted`, `Input` and `Snapshot` over the loopback backend with L1 behind it in CI (ADR 0001's
      milestone); the page posts its newest input each frame, every tick runs on the newest post, and the page receives
      one block per tick and draws an empty scene from it,
      naming its backend (ADR 0003's milestone); the page makes zero Wasm calls; the first recording's per-tick hashes
      match natively twice and in Wasm (ADR 0004's milestone).
   6. *Previews.* After ADR 0006's preview hosting check, with B9's interim position in place (ADR 0006, part 13): the
      build, deploy, close and sweep workflows, the production deploy job and the generated header sets. *Done when:* a
      same-repository pull request has its link within 5 minutes after the build job ends, or after the reviewer's
      approval while `preview` requires one
      (median of 5 runs, R5-PREV-REC1 mark 1); the deploy job's header check passes and fails when a header is removed
      (R5-PREV-REC4 mark 1); Mustafa opens the link in Brave and sees the empty scene (his Brave check).
   *The foundation is done when* an empty scene runs through the full stack in the browser, with CI green and a preview
   link (GLOSSARY §8).
3. **Slice 1, move & shoot:** `content` enters (weapons as data, ADR 0005); the block's soldier, projectile and event
   sections and the owner-only section for the local player's weapon state; the first legacy goldens (the ray maths,
   shot resolution, the imported tuning tables); the determinism recordings in G10. *Done when:* ADR 0001's and ADR
   0004's slice-1 milestones pass through this skeleton, and the boundary marks of the spike's items 1–3 hold on the
   real slice at N = 12.
4. **Slice 2, bots and three teams, played locally:** `navbake` enters; G13 runs on the first committed map; bots fill
   slots in the local server; nightly soaks over L1. *Done when:* G13 and its `--prove` pair pass, and a nightly
   30-minute bot room at N = 48 over L1 runs with 0 panics, 0 hangs and 0 desyncs (R5-TEST-REC5's mark).
5. **Slice 3, online:** `server` and `supervisor` enter; L2 in the nightly run; G12 over both transport backends; the
   first multiplayer preview behind the access wall, with the binary-delivery job once the security baseline has set its
   token's scope (BASELINE-WHEN (b): before the first online playtest); the Sentry upload job if Sentry is set up.
   *Done when:* G12 passes on both backends and a multiplayer preview plays against the staging game server.
6. **Slice 4, the loop:** the owner-only section carries upgrade tracks and offers, and the pickups section carries
   world pickups (UPGRADE-SCOPE; ADR 0004 part 18, ADR 0005). *Done when:* the boundary layer's round-trip test covers
   every new message and section.
7. **Slice 5, Octopon-3 and Editor v2:** the `editor` package and entry, with OQ-0002-1's answer applied; the editor
   runs `navbake` and the map checks in its Worker. *Done when:* G8 shows no editor chunk in the player build and the
   editor's checks give the same result as G13 on the same map.
8. **Slice 6, scale to 48:** soak and load for every mode of the test-mode list; the tick bench in the stress scene.
   *Done when:* R5-TEST-REC5's marks pass for every mode, with ADR 0001's server tick budget.

## Test plan
- **The gate proves itself:** `--prove` on every change to the gate, its checks or its budget file, and nightly: each
  planted defect fails exactly its named checks and each control passes (R5-GATE-REC3).
- **The boundary layer:** Vitest loads the built `sim-wasm`, checks the layout hash and round-trips every message type
  between the Rust encoders and the TypeScript decoders (G9).
- **Determinism by state hash:** the replay runner's per-tick hashes, native in two processes, Wasm in Node, Wasm in
  Chromium, Firefox and WebKit (G10), and real Safari on macOS (OQ-0002-5); the first mismatching tick and a field-level
  diff are reported (R5-TEST-REC2); the rules are ADR 0004's.
- **Network simulation:** L1 in the gate (one minute per profile, P0–P4 and B1, with latency, jitter, loss, reordering,
  duplication and a rate limit, seeded), L1 long runs and L2 (real browsers over WebTransport through an impairment
  socket under quinn, WebSocket through a delaying TCP proxy) nightly; ADR 0001's marks under its named profiles
  (R5-TEST-REC4).
- **Soak and load:** bots as protocol clients, one 30-minute room per mode of the test-mode list over L1 at P2,
  nightly, and a 3-hour soak before each playtest (R5-TEST-REC5).
- **Renders and real pages:** Playwright on the built client and a local server, the backend named on every shot, once
  with three.js's `forceWebGL`, and once with and once without the isolation flag; ADR 0003's counts against their
  baseline (G12); the WebGPU backend through part 9's route (a CI job if the boundary spike finds one, otherwise a
  smoke run on a reference machine for every change-set that touches the render path).
- **Flakes:** the gate's whole suite 20 times on one commit gives 20 identical verdicts (R5-TEST-REC6).
- **Playtests and Brave checks:** Mustafa's checks on preview links, starting with the empty scene of phase 2.

## Rollback
- **Before acceptance:** the record is a draft; if the boundary spike fails, Mustafa and the Advisor reject or revise
  it, and nothing is built.
- **The block or the pacing fails its marks:** the measured alternatives take over behind the isolation flag, which
  then becomes production's default wherever the client runs isolated (OQ-0002-4): the SharedArrayBuffer ring (B3) for
  the block, `Atomics.wait` (W3) for the pacing, inputs through shared memory (C2), with R1-BND-REC5's costs. Inside a
  third-party iframe the non-isolated path stays available (PORTAL-EMBED, OQ-0002-9, which sends the pacing case back
  to Mustafa). The page code that reads typed arrays does not change (part 5).
- **The local match overloads the Worker:** OQ-0002-3's other layout; running the local server in a second Worker was
  not researched and would need its own spike.
- **A crate placement fails** (for example `room` does not build for Wasm with a dependency the server needs): the
  dependency moves to `server`, behind the transport interface; G5 shows the violation before any merge.
- **The gate v2 is too slow:** `--quick` locally, the long set nightly, and path selection only with a tested selection
  rule (R5-GATE-OQ1).
- **Previews fail PREV-DELETE:** the deploy target changes to Workers static assets, which takes the same `_headers`
  file (ADR 0006); the workflow shape stays.
- **The restructure:** a move of five files, reverted by reverting its one change-set; the legacy game runs from
  either place, and today's gate proves it.
- **After acceptance:** a superseding ADR, as template 0000 asks.

## Appendix A: research dispositions
One row per id the decision map gives this record as owner (104 ids, in the map's order). Recommendations: adopted ·
changed (how, why) · rejected (why). Open questions: answered (how, which part) · open (its OQ) · spike (which).

| id | disposition | where or why |
|---|---|---|
| R1-BND-REC1 | adopted | part 5; the spike's item 1 |
| R1-BND-REC2 | changed: two sections added (weapon pickups; the local player's owner-only data, with the floor layer and enclosure area id of ADR 0004 part 14), remote entities written at ADR 0001's interpolation delay with their spatial events released on the same timeline, and mark 4 at N = 48 (MODES-3 (a)) | part 5: UPGRADE-SCOPE's pickups and the I-21 upgrade data must reach the page; ADR 0001 put the interpolation buffer in the Worker, and R2-SYNC-REC6 plays blasts on the timeline of the entity they hit; the spike's item 2 |
| R1-BND-REC3 | changed: the page posts its newest input once per drawn frame, and each tick the Worker builds the one command from the newest post; the message also carries the cursor ray and the view time | part 5: the page has no tick clock, so a post per block can miss the REC's own mark 2 (2 × 1000 / 30 = 66.67 ms against 50.00 ms); ADR 0001 part 5 has the Worker cast the aim ray (I-19); the fields are ADR 0001's; the spike's item 3 |
| R1-BND-REC4 | adopted | part 5 (content's source is ADR 0005's, critic rank 31); the spike's item 4 |
| R1-BND-REC5 | changed: a second trigger for production isolation, the timer pacing failing its jitter mark while W3 passes | part 5: R1-XU-OQ4 reads the pacing and the block together; OQ-0002-4 |
| R1-BND-REC6 | adopted ("offline" reads "local play", SLICE-ORDER) | parts 5 and 6; the spike's item 6 |
| R1-BND-OQ4 | answered: tsify is not used while ts-rs covers the types; the question reopens only if ts-rs proves short | part 5 |
| R1-BND-OQ6 | spike: the boundary spike (bytes per tick and copy time at N = 48, per entity and per field) | part 5; the spike's item 2 |
| R1-BND-OQ7 | open: OQ-0002-7 | part 2 (the pin is the hedge) |
| R1-BND-OQ9 | open: OQ-0002-4 | part 5 |
| R1-DATA-OQ11 | open: OQ-0002-1 | part 11 |
| R1-XU-OQ3 | spike: the boundary spike (B2 against B3, R1-BND-REC2 mark 5) | part 5; the spike's item 2 |
| R1-XU-OQ4 | spike: the boundary spike (the timer pacing's jitter; W3 on an isolated build only if it fails) | part 5's rule; the spike's item 7 |
| R1-RR-OQ4 | answered: overflow checks on for `sim` and `protocol` in every profile; explicit wrapping or checked counters | part 4 |
| R1-RR-OQ5 | spike: the boundary spike and the sim core spike measure the `sim-wasm` size, and the content and ability spike the content crate's share; the Advisor sets the first limit from both | part 4; the spike's item 8 |
| R1-RR-OQ6 | answered: confirmed by rust-reviewer, its named decider, at the step-10 review: the block writer serializes field by field, as the state hash does, with one layout table for the writer, the offsets and the hash, fixed-width little-endian integers and aligned sections; the derive or `unsafe` only if the writer's p99 exceeds 0.67 ms (proposed) | part 5; the spike's item 2 reports the writer's time (PM-0002-10) |
| R1-XT-OQ10 | open: OQ-0002-1 | part 11 |
| R1-XT-OQ15 | answered: the tick bench is a mode of the `devtools` replay runner (3 runs per build; mean, p50, p95, p99, max) | part 8 |
| R1-XT-OQ18 | answered: the crate list follows ADR 0004 (no ECS crate, parry3d) and ADR 0001 (quinn, the proof test's WebTransport crate); the cache is measured on it | part 3 |
| R2-XT-OQ10 | answered: the binary-delivery job; a certificate hash in a preview's static files goes stale, so the supervisor gives the current and the next hash and an older preview is redeployed (ADR 0006 part 4), unless OQ-0006-5 gives staging a trusted certificate and previews pin none; the token's scope waits on the security baseline, its guard is B9's interim reviewer | part 10 |
| R3-MEASURE-OQ4 | answered: the isolation flag, for profiling builds only | part 5 |
| R3-XT-OQ3 | open: OQ-0002-1 | part 11 |
| R4-XT-OQ5 | answered: the bake in its own `navbake` crate (proposed, not researched; allowed by OQ-0002-8 (a), the Advisor, 2026-11-03), never linked into `sim-wasm`; the map checks as G13 | parts 3 and 9 |
| R4-XT-OQ8 | answered: as R1-XT-OQ18 | part 3 |
| R5-LAYOUT-REC1 | adopted | parts 1 and 2 |
| R5-LAYOUT-REC2 | changed: the `room`, `client-endpoint` and `navbake` crates added (proposed, not researched; allowed by OQ-0002-8 (a), the Advisor, 2026-11-03), and the supervisor's crate at slice 3 (R6-HOST-REC2's supervisor, ADR 0006 part 5); nothing imports `spikes/` | part 3: SLICE-ORDER condition 1 runs the local server in the Worker; ADR 0001's network marks run the client endpoint natively; R4-XT-OQ5 |
| R5-LAYOUT-REC3 | adopted | part 5; the spike's item 4 |
| R5-LAYOUT-REC4 | adopted | part 3 (G5) |
| R5-LAYOUT-REC5 | adopted | parts 5 and 11; the spike's item 5 |
| R5-LAYOUT-REC6 | adopted | part 1; Migration plan, phase 2, step 1 (where the tools go: OQ-0002-2) |
| R5-TSLINT-REC1 | adopted | parts 2 and 7 |
| R5-TSLINT-REC2 | changed: the import rule also bars `spikes/` | part 7: SPIKE-SCOPE (a) makes spikes throwaway |
| R5-TSLINT-REC3 | adopted | part 7 |
| R5-TSLINT-REC4 | adopted | part 7 (a kit change-set builds the hook) |
| R5-RUSTLINT-REC1 | adopted | part 7 (G2) |
| R5-RUSTLINT-REC2 | adopted | part 7, with GLOSSARY §7's one escape |
| R5-RUSTLINT-REC3 | adopted | part 7 |
| R5-RUSTLINT-REC4 | adopted | part 4 |
| R5-RUSTLINT-REC5 | adopted | part 7 (G1) |
| R5-RUSTLINT-REC6 | adopted | part 2 |
| R5-TEST-REC1 | adopted | part 8 (the Safari row's schedule: OQ-0002-5) |
| R5-TEST-REC2 | adopted (the nightly match at N = 48, MODES-3 (a)) | part 8 |
| R5-TEST-REC3 | adopted | part 8 (the jump: OQ-0002-6) |
| R5-TEST-REC4 | adopted | part 8 (profiles: ADR 0001's) |
| R5-TEST-REC5 | adopted (N = 48 and the test-mode list, MODES-3 (a)) | part 8; Migration plan, phases 4 and 8 |
| R5-TEST-REC6 | adopted | part 8 |
| R5-CI-REC1 | adopted | part 9 |
| R5-CI-REC2 | adopted | part 2 |
| R5-CI-REC3 | adopted | part 2 |
| R5-CI-REC4 | adopted (the cache keys on the root lockfile) | part 9 |
| R5-CI-REC5 | adopted | part 9 |
| R5-PREV-REC1 | changed: a second client-host token, production's, in its own environment on `main` alone; the reviewer gate the REC allows "when wanted" on `preview` until the security baseline (ADR 0006, B9; OQ-0006-13) | part 10: a branch of this repository can rewrite the deploy job (R5-PREV-F2's contrast) |
| R5-PREV-REC4 | adopted | part 10 (the CSP waits on the security baseline) |
| R5-GATE-REC1 | changed: G13 (maps) added; G2 also reads the build flags and the workspace-lints opt-in; G12 names its network rule | part 9: R4-XT-OQ5 found R4's checks missing; ADR 0004's flag rule needs a check |
| R5-GATE-REC2 | adopted | part 9 (its path: OQ-0002-2) |
| R5-GATE-REC3 | changed: G13's mutation and control added | part 9 |
| R5-GATE-REC4 | adopted | part 9 (every draft's budgets, I-14) |
| R5-LAYOUT-OQ1 | answered: npm, the option read and already in use; pnpm and Yarn were not read | part 2 |
| R5-LAYOUT-OQ2 | answered: the wasm-bindgen CLI directly, not wasm-pack | part 2 |
| R5-LAYOUT-OQ3 | answered: `cargo tree` checks a path crate; whether cargo-deny also bans one is tried on the foundation's planted `server` dependency | part 3 |
| R5-LAYOUT-OQ4 | spike: the boundary spike (two generator runs give byte-identical files) | the spike's item 4 |
| R5-LAYOUT-OQ5 | answered in part: OQ-0002-2 | part 1 |
| R5-TSLINT-OQ1 | answered: TypeScript 6.0.x until typescript-eslint supports 7; the move is its own change-set | part 2 |
| R5-TSLINT-OQ2 | answered: not weighed (its pages were not read); revisit with R5-TSLINT-OQ1 | part 2 |
| R5-TSLINT-OQ3 | spike: the boundary spike (`tsc --strict` on the generated package) | the spike's item 4 |
| R5-TSLINT-OQ4 | spike: the boundary spike (errors and casts against three.js's types) | the spike's item 8 |
| R5-TSLINT-OQ6 | spike: the boundary spike (`tsc -b` time with `skipLibCheck` on and off) | the spike's item 8 |
| R5-RUSTLINT-OQ1 | answered: the method ban, written as the complement of ADR 0004 part 7's allow-list (every other std `f32` and `f64` method), with sim helpers for `min`, `max` and `clamp`, and a reviewed text scan of `sim` as the fallback; G2's planted `x.sin()` proves which | part 7 |
| R5-RUSTLINT-OQ2 | answered: not added (their sources were not read); REC3's lints and fuzzing cover panics, REC4 overflow | part 7 |
| R5-RUSTLINT-OQ3 | answered: rust-reviewer lists the map types of `sim`'s dependency tree, which join `disallowed-types` | part 7 |
| R5-RUSTLINT-OQ4 | spike: the sim core spike reports the overflow checks' cost with part 8's method; the limit is set from it | part 4 |
| R5-RUSTLINT-OQ5 | answered: settled by resolution 29 (GLOSSARY §7); this record states the rule | part 7 |
| R5-TEST-OQ1 | spike: the boundary spike | the spike's item 8 |
| R5-TEST-OQ2 | spike: the netcode proof test (the WebTransport crates' interop, ADR 0001) | part 8 |
| R5-TEST-OQ3 | open: OQ-0002-6 | part 8 |
| R5-TEST-OQ4 | open: OQ-0002-5 | part 8 |
| R5-TEST-OQ5 | answered: turmoil is not added; only if L1 cannot express a room case | part 8 |
| R5-TEST-OQ6 | answered: no OS-level layer; only if L2 cannot reproduce a playtest bug | part 8 |
| R5-TEST-OQ7 | answered: proptest, the option read | part 8 |
| R5-TEST-OQ8 | answered: the bench method in part 8; no cargo-nextest, coverage gate or bench crate (none was read) | part 8 |
| R5-CI-OQ1 | answered: measured on this workspace in the foundation against the 8 GB mark; fewer keys or Mustafa's paid limit if over | part 9; Migration plan, phase 2, step 3 |
| R5-CI-OQ2 | answered: no sccache at the start; weighed again only from measured dependency rebuild times | part 9 |
| R5-CI-OQ3 | answered: SHA pins and Dependabot until workflow lockfiles are generally available on Mustafa's plan | part 9 |
| R5-CI-OQ4 | spike: the boundary spike measures with and without wasm-opt; if kept, the foundation pins it like the CLI | part 2; the spike's item 8 |
| R5-CI-OQ5 | answered: CI gates counts on whatever backend the runner gives, plus a `forceWebGL` run, and records the backend each CI engine gets (probably WebGL 2 everywhere on a GPU-less runner); the WebGPU route is part 9's (a SwiftShader WebGPU job if the boundary spike finds one, else a smoke run on a reference machine); no GPU runner | part 9 |
| R5-CI-OQ6 | answered: a trial on one action in the foundation | part 9 |
| R5-CI-OQ7 | answered: no arm64 leg while the fleet is x86-64 | part 9 |
| R5-PREV-OQ5 | spike: the preview hosting check (ADR 0006) | part 10 |
| R5-PREV-OQ6 | spike: the preview hosting check (ADR 0006) | part 10 |
| R5-GATE-OQ1 | answered: every check runs on every pull request until the first month's CI times | part 9 |
| R5-GATE-OQ2 | answered: as R5-CI-OQ5, with ADR 0003's counts-only rule | part 9 |
| R5-GATE-OQ3 | answered in part: OQ-0002-2 | part 9 |
| R5-GATE-OQ5 | answered: measured on the first gate v2 build against R5-GATE-REC3's proposed 30 minutes | part 9; Migration plan, phase 2, step 3 |
| R5-XQ-OQ1 | open: OQ-0002-5 | part 8 |
| R5-XQ-OQ2 | answered: one root lockfile, which CI's npm cache keys on | part 2 |
| R5-XQ-OQ3 | answered: settled 2026-10-30 (GLOSSARY, the isolation flag); the preview Mustafa checks is built with the flag off | parts 5 and 10 |
| R5-XT-OQ6 | answered: as R2-XT-OQ10 | part 10 |
| R5-XT-OQ7 | answered: as R4-XT-OQ5 | parts 3 and 9 |
| R5-XT-OQ8 | open: OQ-0002-1 | part 11 |
| R5-XT-OQ12 | answered: test builds carry no crash-reporting endpoint, so G12's rule holds; the upload job runs only on deployed builds, in its own environment | part 9 |
| R5-XT-OQ13 | answered: as R1-XT-OQ15 | part 8 |
| R5-XT-OQ14 | answered: as R1-XT-OQ18 | part 3 |
| R6-XT-OQ4 | answered: as R2-XT-OQ10 | part 10 |
| R6-XT-OQ8 | answered: as R5-XT-OQ12 | part 9 |

## Sources
Every file below was read for this record; line numbers are at commit 9e7db28 (the topic files do not change in this
pull request).
- `docs/research/r5-toolchain-quality.md`: the header and TL;DR (lines 1–60); Findings, Questions 1–7 (lines 61–990:
  R5-LAYOUT-F4, R5-RUSTLINT-F1, F9, F13, R5-TSLINT-F4, F5, F13, F15, F21, R5-TEST-F8, R5-CI-F9, F15, F17, F20,
  R5-PREV-F1 to F3, R5-GATE-F2 and the legacy facts used, lines 534, 938, 969–990); Options (lines 992–1146);
  Recommendations (lines 1148–1744); Open questions, including R5-XQ, the shared and the cross-topic questions (lines
  1746–2036); the recommendations that hinge on a source not read (lines 2038–2056); the Verifier spot-check, its
  second pass and the consistency resolutions (lines 2108–2298).
- `docs/research/r1-simulation-core.md`: Findings, unit boundary-worker (lines 868–1178: R1-BND-F1, F2, F12, F19, F22,
  F25, F29, F33 and the legacy facts CS1, GP2, GP13, GP14, GC2, GA15, CS11); Options, unit boundary-worker (lines
  1731–1788); Recommendations, unit boundary-worker (lines 2250–2338); R1-TICK-REC2 (lines 2044–2062); Open questions
  (R1-BND-OQ1 to OQ9, lines 2719–2752; R1-DATA-OQ11, lines 2817–2823; R1-XU-OQ3, OQ4, lines 2854–2865; R1-RR-OQ3 to
  OQ6, lines 2936–2954; R1-XT-OQ10 to OQ18, lines 3075–3112); the hinge table (lines 3209–3229); the Verifier
  spot-check's counts (lines 3231–3256).
- `docs/research/r2-netcode.md`: R2-TRN-F35 (lines 1268–1272); R2-TRN-REC4 (lines 2673–2698); R2-ROOM-REC2 (lines
  2762–2772); R2-ROOM-OQ9 (lines 3104–3105); R2-XT-OQ10 (lines 3384–3389).
- `docs/research/r3-rendering.md`: R3-WGPU-F5 (lines 91–96); R3-MEASURE-F8, F9 (lines 648–658); R3-WGPU-REC4 (lines
  941–951); R3-MEASURE-REC3 (lines 1206–1215); R3-MEASURE-OQ4 (lines 1389–1391); R3-XT-OQ3 (lines 1437–1440).
- `docs/research/r4-physics-navigation.md`: R4-NAV-REC1 (lines 802–810); R4-NAV-REC4 (lines 888–899); R4-XT-OQ5
  (lines 1195–1200); R4-XT-OQ8 (lines 1211–1214).
- `docs/research/r6-infrastructure.md`: R6-HOST-F3 (lines 72–79); R6-HOST-REC1's mark (line 642); R6-HOST-REC3 (lines
  672–683); R6-OBS-REC3 (lines 754–768); R6-XT-OQ4 (lines 1009–1014); R6-XT-OQ8 (lines 1030–1034).
- `docs/research/GLOSSARY.md`: "How to read an entry" and the decision codes (lines 1–45); §1, tick budgets (lines
  46–224); §2, modes and scale (lines 335–394); §3, the stress scene (lines 395–437); §5, previews and hosting (lines
  560–665); §6, the security baseline (lines 667–725); §7, the hash-map ban (lines 727–742); §8, the phase 1 terms (the
  text in the decision map's addendum, part j, J1, appended in this pull request).
- `docs/research/README.md` (the topic table and the shape of a topic file).
- `docs/VISION_V2.md` §3, §4, §5 and §6; `docs/LOOP_V2.md` §11; `docs/EDITOR_V2.md` §2, §3 and §5; `docs/LOOK_V2.md`
  §8 and §9.
- `docs/STATUS.md`, "Decisions pending" (lines 93–210: SLICE-ORDER, MODES-3, PREVIEW-WALL, BASELINE-WHEN, TICK-30,
  the five carry-forwards, DOCS-ALIGN, R7-MARKS, SPIKE-SCOPE, the old ADR-001 and ADR-002 entries).
- `docs/LEGACY_INVENTORY.md`: the marks (lines 1–40) and rows CS1, CS11 (lines 169, 179), PG1, PG18, PG19 (lines 359,
  376, 377), TL1 to TL17 (lines 386–402), KT1, KT7, KT8, KT9, KT12, KT19, KT21 (lines 410–430).
- `docs/adr/0000-template.md`.
- The other decision drafts, cited by number and part: ADR 0001 (parts 4–7 and 10, its components diagram and its
  migration plan), ADR 0003 (parts 8 and 10), ADR 0004 (parts 1, 5, 9, 18–20, OQ-0004-7, S18), ADR 0006 (parts 1–6 and
  13).
