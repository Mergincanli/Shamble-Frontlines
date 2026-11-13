# REPO_MAP: where everything is

Read this instead of exploring. It says which file answers which question; `MODULE_MAP.md` maps the inside of the
engine files, and `START_HERE.md` explains how they work and why. docs-keeper keeps this map current and checks it with
`node .claude/scripts/repo-map-check.mjs`. `.checkpoints/` holds in-progress task checkpoints
(`.claude/CHECKPOINTS.md`); they are deleted before a PR is ready, so they are not listed. Last checked: 2026-11-12.

## Top level
| Path | What it is |
|---|---|
| `CLAUDE.md` | The rules for Claude Code: v2 phase 0 (the look brief is `docs/LOOK_V2.md`), protocol (risk tiers, the review budget, the severity scale, rollback), models, checkpoints (at most 3 agents at a time), reporting, agents, never-list |
| `README.md` | The project in one page, for people |
| `FIRST_PROMPTS.md` | The first prompts Mustafa used (Phase 0, scale test, build spike, ADR run): history |
| `legacy/` | The legacy build, frozen (critical fixes only): the three engine files and the two pages. `shamble-core.js`: engine (tuning, arsenal, look pipeline, characters, kits, maps, KVStore, the SIM math); `shamble-game.js`: gameplay, AI, economy, HUD, menus, animation, the loop and `bootGame`; `shamble-editor.js`: map editor, Character Lab, Anim Lab (`editor.html` only); `index.html`: the player build (three.js r128 → core → game → `bootGame();`); `editor.html`: the authoring build (adds the editor and `bootEditor();`). Moved from the root with `git mv` in foundation step 2.1b (PR #51), bytes unchanged, so `file:line` citations keep their numbers: `shamble-game.js:N` means `legacy/shamble-game.js:N` |
| `rust-toolchain.toml` | The pinned Rust toolchain for v2 (ADR 0002 part 2; foundation step 2.2): channel 1.99.1, the minimal profile, clippy, rustfmt and the `wasm32-unknown-unknown` target. CI installs it with rustup before any cargo call and checks `rustc --version` against it; each bump is its own change-set |
| `Cargo.toml` | The v2 Rust workspace, a virtual one (ADR 0002 parts 2-4 and 7): members `crates/*`, resolver 3, edition 2024, `rust-version` equal to the pin, `publish = false`; `[workspace.dependencies]`, empty for now, where each shared version enters with the first crate that needs it; the workspace lints (`unsafe_code` denied, clippy's `all` at warn), which every crate opts into; overflow checks in every profile for every workspace crate (`[profile.release] overflow-checks = true`, dependencies at the release default, the register row OVERFLOW-ALL) |
| `Cargo.lock` | Its lockfile; every CI cargo command takes `--locked` |
| `crates/` | The v2 Rust crates (ADR 0002 part 3), each still an empty stub with one test: `sim` (the simulation: state, phases, tick, the navigation runtime), `protocol` (message types, the codec, decoders that never panic, the loopback transport), `room` (the room's match logic, with no I/O and no clock), `client-endpoint` (prediction and reconciliation, interpolation, snapshot decoding), `sim-wasm` (the wasm-bindgen bindings the Worker loads; none until the boundary spike), `devtools` (the replay runner, hash comparer, tick bench, legacy-golden reader, headless protocol clients). The tests in `sim` and `protocol` pass only while overflow checks are on |
| `package.json` | The v2 npm workspaces (`packages/*`; ADR 0002 parts 2 and 3), private, exact versions only: TypeScript, Vitest and @playwright/test as dev dependencies, Node 22 in `engines`; no `"type"` field. The legacy toolkit keeps its own `tools/package.json` |
| `package-lock.json` | Its lockfile (CI runs `npm ci`, then `npm ci --offline` with the network cut off) |
| `packages/` | The v2 TypeScript packages (ADR 0002 part 3), each empty with one stub test (`npm test --workspaces`): `client` (`@shamble/client`, the player entry: input, camera, HUD, rendering), `sim-worker` (hosts `sim-wasm`, paces the tick, runs the local match), `net` (the browser's two transport backends, WebTransport and WebSocket), `types` (types generated from Rust), `e2e` (end-to-end tests with Playwright Test) |
| `docs/` | Status, design, vision, research, decisions, reports (below) |
| `tools/` | The legacy verification toolkit: never shipped (below) |
| `.claude/` | Agents, skills, rules, hooks, scripts and settings for Claude Code (below) |
| `.github/` | CI (below) |
| `.gitattributes` | LF line endings everywhere |
| `.gitignore` | node_modules; `.gate/` (generated output: renders, exports, builds); `dist/`, `*.log`, `.env*`, local Claude settings and agent memory; `/target/` (cargo's build folder) and `test-results/` (Playwright Test's output) |

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
| `docs/research/` | v2 phase 0 research, one file per topic; `README.md` gives the shape, the owners and each topic's status |
| `docs/research/r1-simulation-core.md` | R1: the simulation core (ECS, tick, determinism, the Rust ↔ TypeScript boundary, abilities, data and hot reload) |
| `docs/research/r2-netcode.md` | R2: netcode (transport, sync and prediction, interest, rooms, bandwidth budgets) |
| `docs/research/r3-rendering.md` | R3: rendering and presentation (WebGPU and the WebGL 2 fallback, characters, budgets, look techniques, audio; never decides the look) |
| `docs/research/r4-physics-navigation.md` | R4: physics and navigation (physics engine, navigation, floors, line of sight, obstacles placed mid-match) |
| `docs/research/r5-toolchain-quality.md` | R5: toolchain and quality (repository layout, lint, tests, CI, previews per PR, the v2 gate) |
| `docs/research/r6-infrastructure.md` | R6: infrastructure (hosting, costs, monitoring, telemetry); the security baseline goes into the ADR 0006 draft as open and is researched before the first online playtest (BASELINE-WHEN (b)) |
| `docs/research/GLOSSARY.md` | One definition per term the six topics share, starting with the tick budgets; §8 the phase 1 terms (the foundation, the netcode proof test, the 60 Hz margin) |
| `docs/research/UNREAD_SOURCES.md` | Every source R1–R6 could not read, with what each was needed for |
| `docs/reference/` | Renegade X and Kane's Wrath reference: inspiration, not what's built |
| `docs/reports/` | One advisor report per change-set (`YYYY-MM-DD-<slug>.md`): the decision trail |
| `docs/reports/2026-10-31-v2-research.md` | The R1–R6 research report (PR #28): outcome, evidence, risks, decisions needed |
| `docs/reports/2026-10-31-v2-research-appendix.md` | Its appendix: coverage table, the 12 resolutions and the advisor's rulings, fetch audit, check outputs |
| `docs/reports/2026-11-01-v2-decision-drafts.md` | The v2 decision drafts report (PR #33): outcome, evidence, reviews, risks, decisions needed |
| `docs/reports/2026-11-01-v2-decision-drafts-appendix.md` | Its appendix: the drafts brief in full; Mustafa's decisions, the advisor's plan review and the plan's rules and steps, word for word; the steps and their commits; where each of the advisor's conditions is met; verification, the reviews and the final round; after the stop (2026-11-03): where each answer landed, and the advisor's reviews of plan revisions 4, 5 and 6; Mustafa's ten inputs files, byte for byte |

## .claude/
| Path | What it is |
|---|---|
| `.claude/CHECKPOINTS.md` | The checkpoint system: never lose work to a usage limit. For every session: at most 3 agents at a time, workflow agents included (§4, §7); each agent writes its plan first and saves each block as it goes, and a restart trusts the blocks its notes mark done (§4); scratch text and small scripts in the notes folder, builds and binaries under `.gate/<task-slug>/` (§4); the background saver checked at every update while agents run, and restarted if it has stopped (§3, §5); disk hygiene (§10) |
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
| `.claude/scripts/checkpoint.mjs` | Commits and pushes `.checkpoints/` only: `save`, and `watch` (background saver) |
| `.claude/scripts/checkpoint.test.mjs` | Its tests: `node --test .claude/scripts/checkpoint.test.mjs` |
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

## .github/
| Path | What it is |
|---|---|
| `.github/workflows/` | CI workflows |
| `.github/workflows/gate.yml` | Jobs `gate` (checks 1–4, 5b–5d) and `smoke` (5a) on every push and pull request |
| `.github/workflows/toolchain.yml` | Job `toolchain` (foundation step 2.2) on every push and pull request: installs the pinned Rust toolchain with rustup, checks `rustc` against the pin, runs `cargo fetch --locked` and `npm ci` (the last steps with the network), then builds and tests with the network cut off (its own network namespace; a TCP probe must fail inside): `cargo test --workspace --locked`, the release overflow tests of `sim` and `protocol`, the `wasm32-unknown-unknown` build of five crates, `npm ci --offline`, `npm test --workspaces`. Step 2.3 replaces it with the gate v2's workflow |
