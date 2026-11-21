# REPO_MAP: where everything is

Read this instead of exploring. It says which file answers which question; `MODULE_MAP.md` maps the inside of the
engine files, and `START_HERE.md` explains how they work and why. docs-keeper keeps this map current and checks it with
`node .claude/scripts/repo-map-check.mjs`. `.checkpoints/` holds in-progress task checkpoints
(`.claude/CHECKPOINTS.md`); they are deleted before a PR is ready, so they are not listed. Last checked: 2026-11-17.

## Top level
| Path | What it is |
|---|---|
| `CLAUDE.md` | The rules for Claude Code: v2 phase 0 (the look brief is `docs/LOOK_V2.md`), protocol (risk tiers, the review budget, the severity scale, rollback), models, checkpoints (at most 3 agents at a time), reporting, agents, never-list |
| `README.md` | The project in one page, for people |
| `FIRST_PROMPTS.md` | The first prompts Mustafa used (Phase 0, scale test, build spike, ADR run): history |
| `legacy/` | The legacy build, frozen (critical fixes only): the three engine files and the two pages. `shamble-core.js`: engine (tuning, arsenal, look pipeline, characters, kits, maps, KVStore, the SIM math); `shamble-game.js`: gameplay, AI, economy, HUD, menus, animation, the loop and `bootGame`; `shamble-editor.js`: map editor, Character Lab, Anim Lab (`editor.html` only); `index.html`: the player build (three.js r128 → core → game → `bootGame();`); `editor.html`: the authoring build (adds the editor and `bootEditor();`). Moved from the root with `git mv` in foundation step 2.1b (PR #51), bytes unchanged, so `file:line` citations keep their numbers: `shamble-game.js:N` means `legacy/shamble-game.js:N` |
| `rust-toolchain.toml` | The pinned Rust toolchain for v2 (ADR 0002 part 2; foundation step 2.2): channel 1.99.1, the minimal profile, clippy, rustfmt and the `wasm32-unknown-unknown` target. CI installs it with rustup before any cargo call and checks `rustc --version` against it; each bump is its own change-set |
| `Cargo.toml` | The v2 Rust workspace, a virtual one (ADR 0002 parts 2-4 and 7): members `crates/*`, resolver 3, edition 2024, `rust-version` equal to the pin, `publish = false`; `[workspace.dependencies]`, where each shared version enters with the first crate that needs it (so far ts-rs `=12.0.1` and wasm-bindgen `=0.2.130`); the workspace lints (`unsafe_code` denied, clippy's `all` at warn), which every crate opts into; overflow checks in every profile for every workspace crate (`[profile.release] overflow-checks = true`) and, by a named override each, for the six third-party crates compiled into our libraries and tests (cfg-if, thiserror, ts-rs, unicode-ident, wasm-bindgen, wasm-bindgen-shared); `"*"` keeps the other dependencies (proc-macros, build scripts) at the release default (the register row OVERFLOW-ALL; a new third-party crate joins the list in the change that adds it) |
| `Cargo.lock` | Its lockfile; every CI cargo command takes `--locked` |
| `crates/` | The v2 Rust crates (ADR 0002 part 3): `sim` (the simulation: state, phases, tick, the navigation runtime; so far the world, its step and its state hash), `protocol` (message types, the codec, decoders that never panic, the loopback transport; so far the first eight message types), `room` (the room's match logic, with no I/O and no clock; still an empty stub with one test), `client-endpoint` (prediction and reconciliation, interpolation, snapshot decoding, the aim cast, the block writer and its layout table; so far the layout table, the block writer and the local schedule), `sim-wasm` (the wasm-bindgen bindings the Worker loads, change-set 2), `devtools` (the replay runner, hash comparer, tick bench, legacy-golden reader, headless protocol clients; still an empty stub with one test). The tests in `sim` and `protocol` pass only while overflow checks are on |
| `crates/protocol/src/messages.rs` | The first eight message types as plain Rust types, no codec yet (change-set 1, PR #58): `Join`, `JoinAccepted`, `Input`, `SnapshotAck`, `Command`, `AimPoint`, `ViewTime` and `Snapshot`'s header; a field the drafts leave open says "provisional" in its doc line |
| `crates/protocol/src/ts_export.rs` | Test builds only: the message types' export test. ts-rs (`=12.0.1`, default features off, a dev-dependency of `protocol` alone) writes their TypeScript into a new scratch folder; the test fails when `packages/types/src/generated/protocol/` does not hold the same files |
| `crates/client-endpoint/src/layout.rs` | The output block's layout table (`BLOCK_LAYOUT`: a header with `tick` at byte 0 and `layout_hash` at byte 4) and its hash (`LAYOUT_HASH`, FNV-1a, 32 bits, computed when the crate compiles), with their tests (change-set 1, PR #58); the lookups by name `section` and `field` and `Section::align`, which the block writer reads in constants, so a missing name stops the build (change-set 2) |
| `crates/client-endpoint/src/layout_ts.rs` | Test builds only: our own small generator, which prints the layout table as TypeScript, and its export test |
| `crates/client-endpoint/src/block.rs` | `BlockWriter`: the block the simulation hands the page each tick, one buffer allocated once and never moved, every value written at the layout table's offset, little-endian, with no `unsafe`; so far the header (`tick`, `layout_hash`) and an optional padding for the hand-off readings, capped at `MAX_PAD_BYTES` (1 MiB; `new` returns `None` above it) (change-set 2) |
| `crates/client-endpoint/src/schedule.rs` | `LocalSchedule`: when the next tick is due in local play (ADR 0004 part 5's plain accumulator); it reads no clock: the Worker's host passes its time in and sets its timer from the due time it gets back (change-set 2) |
| `crates/sim/src/world.rs` | `World`: the whole state of one match, so far the tick counter and the match seed (`TICK_HZ` 30, `SEED_BYTES` 32); `authority_step` (one call is one tick, a checked add), the canonical bytes (tick, then seed) and `state_hash` (change-set 2) |
| `crates/sim/src/state_hash.rs` | `fnv1a_64`, the state hash's function over the canonical bytes; provisional until PM-0004-21's report; it covers the seed, so its value never leaves the Worker or the server (change-set 2) |
| `crates/sim-wasm/src/lib.rs` | The wasm-bindgen bindings the Worker loads, and nothing else: `Sim` (`new`, `input`, `step`, `rebase`, `block_ptr`, `block_len`, `newest_input`), `tick_hz()`, `MODE_LOCAL`; the checks live in plain Rust (`Sim::create`, tested natively); per tick two calls with plain numbers. A `cdylib` for the Wasm and an `rlib` for the native tests (`crates/sim-wasm/Cargo.toml`) (change-set 2) |
| `package.json` | The v2 npm workspaces (`packages/*`; ADR 0002 parts 2 and 3), private, exact versions only: TypeScript, Vitest, Vite and @playwright/test as dev dependencies, Node 22 in `engines`; no `"type"` field. The legacy toolkit keeps its own `tools/package.json` |
| `package-lock.json` | Its lockfile (CI runs `npm ci`, then `npm ci --offline` with the network cut off) |
| `packages/` | The v2 TypeScript packages (ADR 0002 part 3), each with its tests (`npm test --workspaces`, which CI's `toolchain` job runs: Vitest in `client`, `sim-worker`, `net` and `types`; Playwright's `node` project in `e2e`): `client` (`@shamble/client`, the player entry: the app shell, the overlay and rendering; change-set 2's page, below), `sim-worker` (hosts `sim-wasm`, paces the tick, runs the local match; below), `net` (the browser's two transport backends, WebTransport and WebSocket; still empty with one stub test), `types` (types generated from Rust), `e2e` (end-to-end tests with Playwright Test; below) |
| `packages/types/src/generated/` | Generated from Rust by the two export tests and committed; never edited by hand (the tests compare every file in `protocol/` and `block/` byte for byte and refuse a file directly in this folder, but a third folder here goes unnoticed [verified 2026-11-18]; CI also writes the folder again and compares it with git, PR #60). To write it again: `SHAMBLE_WRITE_TYPES=1 cargo test --workspace --locked`. No two copies of the repository share a cargo target folder: the export tests stop a test binary that runs outside the copy it was built in |
| `packages/types/src/generated/protocol/` | The eight message types in TypeScript, one file each, written by ts-rs (`ts_export.rs`) |
| `packages/types/src/generated/block/layout.ts` | The block's layout in TypeScript (`LAYOUT_HASH` and one object, `BLOCK_LAYOUT`), written by our generator (`layout_ts.rs`) |
| `packages/types/src/layout-check.ts` | `checkLayoutHash`: the page's check of a block's layout hash, which refuses a block of another layout; its tests in `layout-check.test.ts` |
| `packages/types/src/index.ts` | The package's entry: re-exports the eight message types, `BLOCK_LAYOUT`, `LAYOUT_HASH` and `checkLayoutHash`; its tests in `index.test.ts` |
| `packages/sim-worker/src/` | The Worker (change-set 2): `worker.ts` (the entry: loads the Wasm, draws the match seed, starts the host), `host.ts` (`SimHost`: paces the tick, makes the two calls per tick, copies the block once into a pooled buffer and transfers it to the page), `pacing.ts` (`Pacer`, at most `MAX_TICKS_PER_WAKE` 7 ticks a wake, a longer gap dropped), `block-pool.ts`, `read-block.ts` (`copyBlock`, through a fresh view of the Wasm's memory each call), `stats.ts`, `messages.ts` (the page-Worker messages; none carries the seed), `index.ts` (what the page may import: the messages' types and the statistics helpers); a test file beside each. Exports `.` and `./worker` |
| `packages/sim-worker/boundary/sim-wasm.test.mjs` | The boundary tests: the built Wasm and its glue, loaded in Node; they need the build (`build.mjs --wasm-only`), so the package's `test` script, which CI runs, leaves this folder out. The build writes the Wasm and its glue to `packages/sim-worker/pkg/`, ignored by git |
| `packages/client/index.html` | The player's page (change-set 2): the canvas, and the plain line shown when the page is opened as a file ("run node serve.mjs") |
| `packages/client/vite.config.mjs` | The page's build and dev-server settings: relative addresses, no file inlined, the build's two constants written in by `define` (a production build cuts the development code out), dev and preview servers on 127.0.0.1 only, the cross-origin isolation headers only with `SHAMBLE_ISOLATION=1` |
| `packages/client/serve.mjs` | The zip's small server: `node serve.mjs [port]` serves the folder it lies in at `http://127.0.0.1:4173`, to this computer only (GET and HEAD, its own Host only, no path out of its folder, a fixed table of file types, a strict content security policy on every answer); Node's own modules only; copied beside a development build, never in a production one; its tests in `serve.test.mjs` |
| `packages/client/scripts/build.mjs` | The build, from the repository root: `SHAMBLE_WASM_BINDGEN=<the tool's absolute path> node packages/client/scripts/build.mjs [--wasm-only] [--production] [--out-dir <dist-...>]`: the Wasm and its bindings into `packages/sim-worker/pkg/`, then the page with Vite into `packages/client/dist/` (ignored by git; an `--out-dir` folder must be named `dist-<something>`, which `.gitignore` does not cover yet); the tool's file is checked before every run, `--version` included |
| `packages/client/scripts/build-checks.mjs` | The build's checks, each a small function tested without a build (`build-checks.test.mjs`): the wasm-bindgen version in `Cargo.lock`, the tool's version and its file (size and sha256), the size table and the 25 MiB limit per served file, nothing in the built page that a strict content security policy would stop (no inline script or style, no `data:` or `blob:` URL, no `eval`), the production bundle free of development code, the output folder's name |
| `packages/client/scripts/wasm-bindgen-tool.mjs` | The one wasm-bindgen command-line tool the build may run, by its exact values (version 0.2.130, the archive's address, size and sha256, the program's size and sha256), with Mustafa's words of TOOL-SOURCE: a change to any value is a question for him first |
| `packages/client/src/picture.ts` | The interface file between the app shell and the picture (build plan, Part 15.2): `Picture`, `PictureInfo`, `PictureFrame`, `RenderCounters`, `CreatePicture`; only `src/render/` implements it and imports three. engine-architect reviews every change to it |
| `packages/client/src/app/` | The app shell: `boot.ts` (starts the Worker and the picture, the frame loop, the hidden-tab pause, the failure line), `worker-link.ts` (`WorkerLink`: the page's side of the Worker messages, the layout check on the first block after each start), `error-hook.ts`, `flags.ts` (`?backend=webgl`, `?pad=<bytes>`); tests beside them |
| `packages/client/src/render/` | The picture on three.js 0.187.0's WebGPU renderer with its WebGL 2 fallback: `three-picture.ts` (names the backend and the adapter, reads the render counts), `empty-scene.ts` (an empty lit ground with a 1 m grid, placeholder colours, not the look), `index.ts` (the folder's one entry, `createPicture`) |
| `packages/client/src/overlay/` | The development readout and its "Copy report" button (`overlay.ts`, `report.ts`); development builds only, cut out of a production build |
| `packages/client/src/harness/` | The measuring helpers: `frame-times.ts` (`FrameTimes`), `run-header.ts` (what names a run: commit, browser, display) |
| `packages/e2e/playwright.config.ts` | Two projects: `node` (the browser-free tests in `tests/node/`, what the package's `test` script and CI run) and `chromium` (the browser tests in the rest of `tests/` on a built page that `SHAMBLE_DIST` names; not in CI yet, his CI-WASM (a)). Its report goes to `packages/e2e/playwright-report/`, ignored by git |
| `packages/e2e/tests/skeleton/` | change-set 2's browser tests: the page on both backends and the hidden-tab pause (`skeleton.spec.ts`, `render-counts.json`), cross-origin isolation, the hand-off's allocation, Playwright's clock and the Worker, the page opened as a file; `report.ts` reads the readout's numbers |
| `docs/` | Status, design, vision, research, decisions, reports (below) |
| `tools/` | The legacy verification toolkit: never shipped (below) |
| `spikes/` | Throwaway v2 spikes, one folder each (SPIKE-SCOPE (a): exempt from the legacy hard constraints). Nothing outside `spikes/` may reference them; the spike's test command greps for that (below) |
| `.claude/` | Agents, skills, rules, hooks, scripts and settings for Claude Code (below) |
| `.github/` | CI (below) |
| `.gitattributes` | LF line endings everywhere |
| `.gitignore` | node_modules; `.gate/` (generated output: renders, exports, builds); `dist/`, `*.log`, `.env*`, local Claude settings and agent memory; `/target/` (cargo's build folder) and `test-results/` (Playwright Test's output); `packages/sim-worker/pkg/` (the Wasm and its glue, written by the build) and `packages/e2e/playwright-report/` |

## docs/
| Path | What it is · when to read it |
|---|---|
| `docs/STATUS.md` | Where things stand, at most 120 lines (Mustafa's STATUS-DIET, 2026-11-10): read first |
| `docs/DECISIONS.md` | Mustafa's decisions, word for word, newest first, and the Advisor's that he relayed; the queue of his words still to carry out; an index of his decisions recorded elsewhere |
| `docs/STATUS_ARCHIVE.md` | The rest of STATUS, moved there by line ranges on 2026-11-10: the done change-sets, the old phase text, the routines' history, old measurements and the advisor agent's rulings; two live sections, "Follow-ups" and "Open bugs and risks" |
| `docs/REPO_MAP.md` | This map |
| `docs/START_HERE.md` | Legacy architecture, hard constraints with their WHY, the do-NOT list, lessons |
| `docs/MODULE_MAP.md` | Who owns what inside the legacy engine files, load order, layering |
| `docs/TECHNICAL.md` | How the legacy engine works, in depth |
| `docs/VISION_V2.md` | v2 north star, pillars, modes and scale, targets, what the engine must support (§6) |
| `docs/V2_PHASE0_BRIEF.md` | v2 phase 0: order of work, kit-v3-0, routines, research topics R1–R7, decision drafts |
| `docs/LOOP_V2.md` | The v2 core loop (living spec; status tags DECIDED / DEFAULT / OPEN / LATER / IDEA; §13: amps, loot and ammo) |
| `docs/EDITOR_V2.md` | The map editor rebuild (draft spec) |
| `docs/LOOK_V2.md` | The v2 look brief (Mustafa and the Advisor, 2026-10-31): the input of ADR 0003 and of ADR 0005's asset pipeline |
| `docs/ENGINE_OVERVIEW.md` | The v2 engine on one page: the parts, the Rust ↔ TypeScript boundary, the data flow per tick and per frame, where each VISION_V2 §6 need lives |
| `docs/SPIKE_PLAN.md` | Every v2 spike the drafts ask for (Proposed): what it proves, which pass marks judge it (by id; the numbers live in the drafts), who judges, its place in the order, its rough size; the foundation's acceptance checks and the asset-pipeline research |
| `docs/EDITOR_V1_INVENTORY.md` | Every legacy map-editor feature with a keep / drop / improve mark |
| `docs/LEGACY_INVENTORY.md` | Every legacy system: what, where, evidence, and a port / rewrite / redesign / drop mark (R7) |
| `docs/LEGACY_TUNING.md` | Every legacy tuning table and constant, rendered from `legacy-tuning.json` (R7) |
| `docs/legacy-tuning.json` | The legacy tuning numbers as data, dumped from the live build: v2's starting values |
| `docs/GAME_DESIGN.md` | Legacy game design and vision (§5–6 superseded by LOOP_V2) |
| `docs/DESIGN_NOTES.md` | Mustafa's design decisions, open questions, art lessons |
| `docs/ART_DIRECTION.md` | The locked "Living Comic" look of the legacy build |
| `docs/ANIMATION_PLAN.md` | Legacy animation system (ANIM v2) and its metrics |
| `docs/AUDIT_BUGS_PERF.md` | The 2026-09-26 bug and performance audit: methods and what was kept on purpose |
| `docs/UNITS.md` | Planned unit roster (not built); §0: Mustafa's v2 roster decisions and ideas (2026-11-04) |
| `docs/ROADMAP.md` | History and plan, terse |
| `docs/ui_theme_mockup.html` | The war-room UI mockup (legacy UI source of truth) |
| `docs/adr/` | Decision records; `0000-template.md` is the format; the v2 drafts 0001–0006 (Status: Proposed): netcode, code structure and toolchain, rendering, simulation core, content and abilities and assets, infrastructure |
| `docs/adr/README.md` | The index of the v2 drafts and the register: the decisions the drafts ask for, DESIGN-owned research ids, features whose need is outside VISION_V2 §6, the loose ends, the codes the drafts cite |
| `docs/design/` | Design records; `ws-reconnect.md`: the WebSocket fallback's fast-reconnect requirement before the first online playtest, its target and the evidence behind it (ADR 0001 OQ-0001-19) |
| `docs/design/netcode-proof-messages.md` | The netcode proof test's message catalogue (Proposed; netcode-architect): ADR 0001's subset plus the test's controls, each type with its channel, visibility class, authority and size limit; the spike's class audit test (`crates/proto/tests/catalogue.rs`) reads its four tables; a protocol change in the spike updates it first |
| `docs/research/` | v2 phase 0 research, one file per topic; `README.md` gives the shape, the owners and each topic's status |
| `docs/research/r1-simulation-core.md` | R1: the simulation core (ECS, tick, determinism, the Rust ↔ TypeScript boundary, abilities, data and hot reload) |
| `docs/research/r2-netcode.md` | R2: netcode (transport, sync and prediction, interest, rooms, bandwidth budgets) |
| `docs/research/r3-rendering.md` | R3: rendering and presentation (WebGPU and the WebGL 2 fallback, characters, budgets, look techniques, audio; never decides the look) |
| `docs/research/r4-physics-navigation.md` | R4: physics and navigation (physics engine, navigation, floors, line of sight, obstacles placed mid-match) |
| `docs/research/r5-toolchain-quality.md` | R5: toolchain and quality (repository layout, lint, tests, CI, previews per PR, the v2 gate) |
| `docs/research/r6-infrastructure.md` | R6: infrastructure (hosting, costs, monitoring, telemetry); the security baseline goes into the ADR 0006 draft as open and is researched before the first online playtest (BASELINE-WHEN (b)) |
| `docs/research/GLOSSARY.md` | One definition per term the six topics share, starting with the tick budgets; §8 the phase 1 terms (the foundation, the netcode proof test, the 60 Hz margin) |
| `docs/research/UNREAD_SOURCES.md` | Every source R1–R6 could not read, with what each was needed for |
| `docs/research/asset-export-rules.md` | The rules a model file follows so the engine loads it and colours it per faction (graphics-engineer; the build plan's change-set 0; OQ-0005-AS3 and AS4 answered in part): the file and its export, scale, axes and origin, names, joints and sockets, caps; ten checks, each with a planted defect (PM-0005-47 to -56); materials to slot classes; open points for Mustafa (O1, O2). On paper: no export has gone through them yet |
| `docs/research/soldier-joint-list.md` | engine-architect's ruling on Slice 1's soldier (change-set 0): one list of 20 joints, `root` as the node above the skeleton, the sockets on a gun and how the hands meet them, the origin and facing check, what is left open and what it changes for change-set 10. On paper |
| `docs/reference/` | Renegade X and Kane's Wrath reference: inspiration, not what's built |
| `docs/reports/` | One advisor report per change-set (`YYYY-MM-DD-<slug>.md`): the decision trail |
| `docs/reports/2026-10-31-v2-research.md` | The R1–R6 research report (PR #28): outcome, evidence, risks, decisions needed |
| `docs/reports/2026-10-31-v2-research-appendix.md` | Its appendix: coverage table, the 12 resolutions and the advisor's rulings, fetch audit, check outputs |
| `docs/reports/2026-11-01-v2-decision-drafts.md` | The v2 decision drafts report (PR #33): outcome, evidence, reviews, risks, decisions needed |
| `docs/reports/2026-11-01-v2-decision-drafts-appendix.md` | Its appendix: the drafts brief in full; Mustafa's decisions, the advisor's plan review and the plan's rules and steps, word for word; the steps and their commits; where each of the advisor's conditions is met; verification, the reviews and the final round; after the stop (2026-11-03): where each answer landed, and the advisor's reviews of plan revisions 4, 5 and 6; Mustafa's ten inputs files, byte for byte |
| `docs/reports/2026-11-17-build-plan-slice-1.md` | The build plan for foundation steps 2.4 and 2.5 and Slice 1 (fourteen change-sets; his go of 2026-11-17), copied by script from commit 68addf9, with "Since the plan" on top: what his answers and changes do to it (PR #56) |
| `docs/reports/2026-11-17-build-plan-slice-1-appendix.md` | Its appendix, A1 to A11: his messages of 2026-11-14 and 2026-11-17 and the Art lead's texts, byte for byte, and the three Art sheets' text as the build chat typed it |
| `docs/reports/2026-11-17-cs0-model-rules.md` | The report of change-set 0 of the build plan (PR #56) |
| `docs/reports/2026-11-17-build-plan-slice-1-part-15.md` | Part 15 of the build plan, "The second lane" (which change-sets, folders and files belong to the second builder chat, how the two chats' pull requests take turns, who owns each CI file, how change-set 11 uses Shamble VFX): revision 2, copied by script from commit ea851a8, with Mustafa's six answers of 2026-11-17 on top and his two messages byte for byte (P1, P2) (PR #57) |
| `docs/reports/2026-11-17-part-15-second-lane.md` | The report of the change-set that carries Part 15 to main (PR #57) |
| `docs/reports/2026-11-18-cs1-types.md` | The report of change-set 1 of the build plan, Types (PR #58) |
| `docs/reports/2026-11-18-kit-hold-ci.md` | The report of the kit hold and two CI edits (PR #60): the checkpoint script holds a push that changes a manifest or a lockfile until its security check is recorded (`.claude/CHECKPOINTS.md` §3); the `toolchain` job runs with no cache, writes the generated types again and compares them, and type-checks `packages/types` |
| `docs/reports/2026-11-19-cs2-first-page.md` | The report of change-set 2 of the build plan, First page (PR #61): the Rust simulation as WebAssembly in one Web Worker at 30 ticks a second, one block per tick to the page, an empty lit ground on three.js's WebGPU renderer (WebGL 2 as the fallback), the small server and the build |

## .claude/
| Path | What it is |
|---|---|
| `.claude/CHECKPOINTS.md` | The checkpoint system: never lose work to a usage limit. For every session: at most 3 agents at a time, workflow agents included (§4, §7); each agent writes its plan first and saves each block as it goes, and a restart trusts the blocks its notes mark done (§4); scratch text and small scripts in the notes folder, builds and binaries under `.gate/<task-slug>/` (§4); the background saver checked at every update while agents run, and restarted if it has stopped (§3, §5); a dependency change held: the script never pushes a change to a manifest or a lockfile until `.checkpoints/<task-slug>/dependency-check.md` records security-auditor's answer on that content, and nothing that holds the change is built or pushed by hand meanwhile (§3, Mustafa's DEP-HOLD, PR #60); disk hygiene (§10); kept raw logs and the old build cache, removed only by `kept-logs.mjs` (§11) |
| `.claude/NIGHT_SHIFT.md` | The night shift's rules (window 00:10–07:30 Amsterdam) |
| `.claude/settings.json` | Permissions (allow, deny, empty ask), `effortLevel: "high"` (the cloud routines run at the effort set on the routine instead: STATUS_ARCHIVE, Routines), the syntax hook and the routine guard (PreToolUse / PermissionRequest / ConfigChange; each of the three handlers starts `guard.mjs` through a one-line `node -e` bootstrap) |
| `.claude/claude-security-guidance.md` | Security guidance for the security plugin |
| `.claude/security-patterns.json` | Security plugin patterns (server, shared sim, HTML sinks) |
| `.claude/agents/` | The crew (one file each, below) |
| `.claude/agents/advisor.md` | Plan and PR reviews (a plan's checkpoint, the 3-agent cap, models and effort), risk tiers with per-tier checklists, the review budget, the revert steps, merges by the PR Advisor routine, escalations to Mustafa |
| `.claude/agents/verifier.md` | The gate plus change-type evidence; research claim spot-checks |
| `.claude/agents/docs-keeper.md` | STATUS (with DECISIONS and STATUS_ARCHIVE), ROADMAP, this map, doc drift |
| `.claude/agents/art-director.md` | The legacy look (Living Comic) and UI; v2 rendering work against the look brief `docs/LOOK_V2.md`, never choosing the look |
| `.claude/agents/perf-auditor.md` | Frame time, allocation, draw calls; v2 budgets |
| `.claude/agents/security-auditor.md` | Security and anti-cheat; the security baseline (open in the ADR 0006 draft, researched before the first online playtest: BASELINE-WHEN (b)) |
| `.claude/agents/netcode-architect.md` | Authority, tick, prediction, lag compensation, protocol; R2 |
| `.claude/agents/engine-architect.md` | Module boundaries, the Rust ↔ TypeScript boundary; R1, R4, the synthesis |
| `.claude/agents/graphics-engineer.md` | WebGPU and TSL, the render pipeline, GPU budget, techniques toward `docs/LOOK_V2.md` (never choosing the look); R3 |
| `.claude/agents/rust-reviewer.md` | Rust safety, speed, identical results on Wasm and native, Wasm size |
| `.claude/agents/test-engineer.md` | Test strategy, golden tests, network and load tests, gate v2; R5 |
| `.claude/agents/infra-engineer.md` | CI, preview links, hosting, monitoring, telemetry, costs; R6 |
| `.claude/agents/game-designer.md` | Rules against LOOP_V2 and VISION_V2, balance math, tuning tables |
| `.claude/skills/` | Slash commands (below) |
| `.claude/skills/changeset/` | `/changeset`: one change-set end to end (reviewers at most 3 at a time) |
| `.claude/skills/gate/` | `/gate`: the legacy verification gate |
| `.claude/skills/render-ab/` | `/render-ab`: before/after renders |
| `.claude/skills/advisor-report/` | `/advisor-report`: the end-of-change-set report |
| `.claude/rules/` | Path-scoped rules that load with the files they name (below) |
| `.claude/rules/engine-core.md` | Rules for `shamble-core.js` |
| `.claude/rules/engine-game.md` | Rules for `shamble-game.js` |
| `.claude/rules/engine-editor.md` | Rules for `shamble-editor.js` |
| `.claude/rules/pages.md` | Rules for the two pages |
| `.claude/rules/tools.md` | Rules for `tools/` |
| `.claude/rules/netcode.md` | Rules for future server, shared and net folders |
| `.claude/hooks/` | Hooks (below) |
| `.claude/hooks/syntax-gate.mjs` | After every Edit or Write: `node --check` on JS and each page's inline scripts |
| `.claude/hooks/guard.mjs` | The routine-guard launcher: loads the rules, prints their deny as JSON (or an ask, for a day session's PreToolUse), fails closed for routines; when it is missing or can't load, the `settings.json` bootstrap lets a day session through and blocks a routine, and it also blocks a routine when this file ends without its own exit |
| `.claude/hooks/routine-guard.mjs` | The routine-guard rules (`decide`): git/gh option forms and the merge check for every session, plus the deny-by-default shell, no-web and no-protected-write rules for the two cloud routines, and in day sessions the ask before kit and gate edits and main-folder switches (an agent's kit edit refused) |
| `.claude/hooks/routine-guard.test.mjs` | Tests for the rules: the case table, real git on a throwaway repo, launcher error modes, mutation runs |
| `.claude/hooks/settings.test.mjs` | Tests for `settings.json`: empty ask, high effort, guard registered on the three events, routine commands allowed and nevers denied, and the GUARD-FILE tests (the hook command run with `guard.mjs` removed, broken, unfinished and present) |
| `.claude/scripts/` | Kit scripts (below) |
| `.claude/scripts/checkpoint.mjs` | Commits and pushes `.checkpoints/` only: `save`, and `watch` (background saver). Both hold a push that changes a manifest or a lockfile (`Cargo.toml`, `Cargo.lock`, `package.json`, `package-lock.json`, `npm-shrinkwrap.json`, in any folder) against the remote branch (or main's merge-base for a branch not pushed yet), unless `dependency-check.md` names each file's sha256 or the file equals `origin/main`'s; a held `save` commits locally and exits 1 (PR #60) |
| `.claude/scripts/checkpoint.test.mjs` | Its tests: `node --test .claude/scripts/checkpoint.test.mjs` |
| `.claude/scripts/kept-logs.mjs` | KEPT-LOGS (`CHECKPOINTS.md` §11): `list` writes a closed run set's raw-log list beside its results; `pack` packs the folder, reads every file's hash back and removes the folder only if all equal the committed list; `cache` removes the one old build cache, `.gate/u3c-review-target`; it refuses everything else |
| `.claude/scripts/kept-logs.test.mjs` | Its tests: `node --test .claude/scripts/kept-logs.test.mjs`; `KEPT_LOGS_DRY_RUN=<a real run-set folder>` adds the dry run on a copy |
| `.claude/scripts/repo-map-check.mjs` | Checks this map against the repository |

## tools/ (legacy verification toolkit; setup and commands in `tools/README.md`)
| Path | What it is |
|---|---|
| `tools/README.md` | Setup, every command, what check 5 covers |
| `tools/gate.mjs` | The gate: checks 1–4 and 5a–5d against a base ref |
| `tools/smoke.mjs` | Check 5a: both pages in headless Chromium |
| `tools/battery.js` | Checks 3–4 (symbols, editor references) as JSON |
| `tools/gate-globals.json` | Browser globals check 5b accepts |
| `tools/preview.js` | Headless 3D renders and `eval` of snippets on the real build |
| `tools/pageshot.js` | Real-page renders (DOM, HUD, CSS) in Chromium |
| `tools/sim.js` | Headless 90 s match (random AI: 3+ runs) |
| `tools/reach.js` | Spawn-to-goal reachability on the bot nav grid |
| `tools/perfbots.js` | Bot scaling cost at 12 / 24 / 48 soldiers |
| `tools/tuning-dump.js` | Dumps every tuning table as JSON through `preview.js eval` (read-only) |
| `tools/aabbs.js` | Map layout export |
| `tools/village.py` | Map layout design and validation |
| `tools/package.json` | The toolkit's pinned dependencies |
| `tools/package-lock.json` | Their lockfile |

## spikes/ (throwaway; each spike's `README.md` has its commands)
| Path | What it is |
|---|---|
| `spikes/netcode-proof/` | The netcode proof test (SPIKE_PLAN item 1; in flight: STATUS). Kept on main afterwards for the 60 Hz margin run and the boundary spike. Builds go to `.gate/netcode-proof-test/target` (`.cargo/config.toml`) |
| `spikes/netcode-proof/README.md` | What it is, its layout and dependency rules, every command |
| `spikes/netcode-proof/Cargo.toml` | The Cargo workspace: 15 crates, resolver 3, edition 2024, every build `--locked` (`Cargo.lock` committed); `rust-toolchain.toml` pins Rust 1.99.1 (clippy, rustfmt, the wasm32 target); `rustfmt.toml`; `deny.toml` (cargo-deny) |
| `spikes/netcode-proof/check.mjs` | The test command, run at every unit: fmt, clippy host and wasm32 (warnings as errors), cargo test, the dependency rules from `cargo metadata`, the no-reference grep, marks.json's check (it compiles in the judge, covers exactly the plan's marks, its rows unchanged), its own tests (`check.test.mjs`), redact-check |
| `spikes/netcode-proof/crates/` | The 15 crates (`npt-<name>`): leaf crates link, l1, sim, proto, room, client and log (build for wasm32; no transport crate or async runtime; clock reads banned by their `clippy.toml`); then server, net, client-wasm, bots (with npt-load), super, host, judge and lab. Built so far: npt-log (results-log records and writer), npt-judge (marks and logs to one verdict per mark) and npt-proto's catalogue audit test; the rest are stubs |
| `spikes/netcode-proof/marks/` | `README.md`: the contract between the results log, `marks.json` and npt-judge; `marks.json`: every mark of plan §6.2 as data (81, each with its draft row's hash); `SERIES.md`: every series and figure name the marks read (the naming contract for U2-U7). npt-judge's fixtures are in `crates/judge/fixtures/` |
| `spikes/netcode-proof/ops/operator/redact-check.mjs` | The redaction check for results (Node 22, Windows too), with its tests `redact-check.test.mjs` |

Not built yet (plan §4.1): `web/` (the client harness: TypeScript, Vite, three.js), `fuzz/` (one cargo-fuzz target
per message type), the rest of `ops/` (`cloud-init/`, `firewall/`, `runs/` R1–R7, `checklists/`, and the operator
settings, rules and prompts) and `results/` (committed verdicts and redacted summaries, never raw data).

## .github/
| Path | What it is |
|---|---|
| `.github/workflows/` | CI workflows |
| `.github/workflows/gate.yml` | Jobs `gate` (checks 1–4, 5b–5d) and `smoke` (5a) on every push and pull request |
| `.github/workflows/toolchain.yml` | Job `toolchain` (foundation step 2.2) on every push and pull request, with no cache restored or saved (`cache-mode: none`, PR #60): installs the pinned Rust toolchain with rustup, checks `rustc` against the pin, runs `cargo fetch --locked` and `npm ci` (the last steps with the network), then builds and tests with the network cut off (its own network namespace; a TCP probe must fail inside): `cargo test --workspace --locked`; the generated types written again (`SHAMBLE_WRITE_TYPES=1`) and compared with git, failing on any change in `packages/types/src/generated`; the release overflow tests of `sim` and `protocol`, the `wasm32-unknown-unknown` build of five crates, `npm ci --offline`, `npm test --workspaces`; then `packages/types` type-checked with strict flags (`tsc --ignoreConfig`: the shipped files, then the shipped files and their tests with `--skipLibCheck`). Step 2.3 replaces it with the gate v2's workflow |
