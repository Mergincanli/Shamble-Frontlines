# REPO_MAP: where everything is

Read this instead of exploring. It says which file answers which question; `MODULE_MAP.md` maps the inside of the
engine files, and `START_HERE.md` explains how they work and why. docs-keeper keeps this map current and checks it with
`node .claude/scripts/repo-map-check.mjs`. `.checkpoints/` holds in-progress task checkpoints
(`.claude/CHECKPOINTS.md`); they are deleted before a PR is ready, so they are not listed. Last checked: 2026-10-31.

## Top level
| Path | What it is |
|---|---|
| `CLAUDE.md` | The rules for Claude Code: v2 phase 0, protocol, models, checkpoints, reporting, agents, never-list |
| `README.md` | The project in one page, for people |
| `FIRST_PROMPTS.md` | The first prompts Mustafa used (Phase 0, scale test, build spike, ADR run): history |
| `index.html` | Legacy player build: three.js r128 → core → game → `bootGame();` |
| `editor.html` | Legacy authoring build: adds the editor and `bootEditor();` |
| `shamble-core.js` | Legacy engine: tuning, arsenal, look pipeline, characters, kits, maps, KVStore, the SIM math |
| `shamble-game.js` | Legacy gameplay, AI, economy, HUD, menus, animation, the loop and `bootGame` |
| `shamble-editor.js` | Legacy map editor, Character Lab, Anim Lab (editor.html only) |
| `docs/` | Status, design, vision, research, decisions, reports (below) |
| `tools/` | The legacy verification toolkit: never shipped (below) |
| `.claude/` | Agents, skills, rules, hooks, scripts and settings for Claude Code (below) |
| `.github/` | CI (below) |
| `.gitattributes` | LF line endings everywhere |
| `.gitignore` | node_modules, `.gate/` scratch, `.env*`, local Claude settings |

## docs/
| Path | What it is · when to read it |
|---|---|
| `docs/STATUS.md` | Where things stand: read first |
| `docs/REPO_MAP.md` | This map |
| `docs/START_HERE.md` | Legacy architecture, hard constraints with their WHY, the do-NOT list, lessons |
| `docs/MODULE_MAP.md` | Who owns what inside the legacy engine files, load order, layering |
| `docs/TECHNICAL.md` | How the legacy engine works, in depth |
| `docs/VISION_V2.md` | v2 north star, pillars, modes and scale, targets, what the engine must support (§6) |
| `docs/V2_PHASE0_BRIEF.md` | v2 phase 0: order of work, kit-v3-0, routines, research topics R1–R7, decision drafts |
| `docs/LOOP_V2.md` | The v2 core loop (living spec; status tags DECIDED / DEFAULT / OPEN / LATER) |
| `docs/EDITOR_V2.md` | The map editor rebuild (draft spec) |
| `docs/EDITOR_V1_INVENTORY.md` | Every legacy map-editor feature with a keep / drop / improve mark |
| `docs/LEGACY_INVENTORY.md` | Every legacy system: what, where, evidence, and a port / rewrite / redesign / drop mark (R7) |
| `docs/LEGACY_TUNING.md` | Every legacy tuning table and constant, rendered from `legacy-tuning.json` (R7) |
| `docs/legacy-tuning.json` | The legacy tuning numbers as data, dumped from the live build: v2's starting values |
| `docs/GAME_DESIGN.md` | Legacy game design and vision (§5–6 superseded by LOOP_V2) |
| `docs/DESIGN_NOTES.md` | Mustafa's design decisions, open questions, art lessons |
| `docs/ART_DIRECTION.md` | The locked "Living Comic" look of the legacy build |
| `docs/ANIMATION_PLAN.md` | Legacy animation system (ANIM v2) and its metrics |
| `docs/AUDIT_BUGS_PERF.md` | The 2026-09-26 bug and performance audit: methods and what was kept on purpose |
| `docs/UNITS.md` | Planned unit roster (not built) |
| `docs/ROADMAP.md` | History and plan, terse |
| `docs/ui_theme_mockup.html` | The war-room UI mockup (legacy UI source of truth) |
| `docs/adr/` | Decision records; `0000-template.md` is the format (v2 drafts 0001–0006 land here) |
| `docs/research/` | v2 phase 0 research, one file per topic; `README.md` gives the shape, the owners and each topic's status |
| `docs/research/r1-simulation-core.md` | R1: the simulation core (ECS, tick, determinism, the Rust ↔ TypeScript boundary, abilities, data and hot reload) |
| `docs/research/r2-netcode.md` | R2: netcode (transport, sync and prediction, interest, rooms, bandwidth budgets) |
| `docs/research/r3-rendering.md` | R3: rendering and presentation (WebGPU and the WebGL 2 fallback, characters, budgets, look techniques, audio; never decides the look) |
| `docs/research/r4-physics-navigation.md` | R4: physics and navigation (physics engine, navigation, floors, line of sight, obstacles placed mid-match) |
| `docs/research/r5-toolchain-quality.md` | R5: toolchain and quality (repository layout, lint, tests, CI, previews per PR, the v2 gate) |
| `docs/research/r6-infrastructure.md` | R6: infrastructure (hosting, costs, monitoring, telemetry; the security baseline deferred) |
| `docs/research/GLOSSARY.md` | One definition per term the six topics share, starting with the tick budgets |
| `docs/research/UNREAD_SOURCES.md` | Every source R1–R6 could not read, with what each was needed for |
| `docs/reference/` | Renegade X and Kane's Wrath reference: inspiration, not what's built |
| `docs/reports/` | One advisor report per change-set (`YYYY-MM-DD-<slug>.md`): the decision trail |
| `docs/reports/2026-10-31-v2-research.md` | The R1–R6 research report (PR #28): outcome, evidence, risks, decisions needed |
| `docs/reports/2026-10-31-v2-research-appendix.md` | Its appendix: coverage table, the 12 resolutions and the advisor's rulings, fetch audit, check outputs |

## .claude/
| Path | What it is |
|---|---|
| `.claude/CHECKPOINTS.md` | The checkpoint system: never lose work to a usage limit |
| `.claude/NIGHT_SHIFT.md` | The night shift's rules (window 00:10–08:30 Amsterdam) |
| `.claude/settings.json` | Permissions (allow, ask, deny) and the syntax hook |
| `.claude/claude-security-guidance.md` | Security guidance for the security plugin |
| `.claude/security-patterns.json` | Security plugin patterns (server, shared sim, HTML sinks) |
| `.claude/agents/` | The crew (one file each, below) |
| `.claude/agents/advisor.md` | Plan and PR reviews, merges by the PR Advisor routine, escalations to Mustafa |
| `.claude/agents/verifier.md` | The gate plus change-type evidence; research claim spot-checks |
| `.claude/agents/docs-keeper.md` | STATUS, ROADMAP, this map, doc drift |
| `.claude/agents/art-director.md` | The legacy look and UI; v2 rendering research, never choosing the look |
| `.claude/agents/perf-auditor.md` | Frame time, allocation, draw calls; v2 budgets |
| `.claude/agents/security-auditor.md` | Security and anti-cheat; R6 security baseline |
| `.claude/agents/netcode-architect.md` | Authority, tick, prediction, lag compensation, protocol; R2 |
| `.claude/agents/engine-architect.md` | Module boundaries, the Rust ↔ TypeScript boundary; R1, R4, the synthesis |
| `.claude/agents/graphics-engineer.md` | WebGPU and TSL, the render pipeline, GPU budget; R3 |
| `.claude/agents/rust-reviewer.md` | Rust safety, speed, identical results on Wasm and native, Wasm size |
| `.claude/agents/test-engineer.md` | Test strategy, golden tests, network and load tests, gate v2; R5 |
| `.claude/agents/infra-engineer.md` | CI, preview links, hosting, monitoring, telemetry, costs; R6 |
| `.claude/agents/game-designer.md` | Rules against LOOP_V2 and VISION_V2, balance math, tuning tables |
| `.claude/skills/` | Slash commands (below) |
| `.claude/skills/changeset/` | `/changeset`: one change-set end to end |
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
