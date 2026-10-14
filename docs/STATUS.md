# STATUS: the one-screen dashboard
_Kept current by docs-keeper after every change-set. Mustafa and the Advisor read this first._
_Last updated: 2026-10-13, Phase 0 on cs/phase-0._

## Phase
**Phase 0: environment + green baseline**: built and proven in the Linux container (portable toolkit, tools/gate.mjs
proven to bite, CI, START_HERE §1/§5 rewrite, new multiplayer target). Remaining: Mustafa's Windows acceptance run
(Prompt 0b: npm ci, gate, headed RTX renders, headless-gl, hook) and the merge.
Next: Phase 1 decisions (ADR-001, ADR-002), then the foundation refactor and the netcode MVP. FIRST_PROMPTS order:
Prompt 1 (scale test) and Prompt 2 (build spike) come before Prompt 3 (the ADRs).

## In flight
| Work | Branch | State | Waiting on |
|---|---|---|---|
| UI theme transplant, step 1 (menu → dossier, pause → clipboard, field manual) | main (baseline) | delivered, battery PASS | Mustafa's Brave check incl. DEV-panel FPS on the menu |
| Repo root move (CS-1): the web upload had nested the project in `shamble-frontlines-repo/` | claude/intelligent-brahmagupta-9oq3yb | gate PASS (main-session + subagent stand-ins), pure renames | Advisor OK, merge with a merge commit |
| Phase 0 setup | cs/phase-0 (stacked on CS-1) | gate PASS, verifier PASS WITH NOTES (2 holes fixed), CI added | Prompt 0b on Mustafa's PC, Advisor OK, merge after CS-1 |

## Targets
- **Multiplayer:** three teams, one per faction, of 12–16 players each in squads of 4 (12v12v12 to 16v16v16, possibly
  larger): design for 48+ players. Bots fill empty slots. Milestones: 2–8 players → 16 → full scale, with load tests at
  each step.

## Decisions pending
- **ADR-001 Multiplayer architecture** (authority, tick, transport, hosting, offline play): not started
- **ADR-002 Code structure & build** (modules, bundler, TypeScript, double-click build): under discussion with Mustafa;
  evidence first via the build spike
- **ADR-003 Three.js version** (stay on r128 vs upgrade): deferred until after ADR-002
- **Design: progression** (Identity / Momentum / Mastery): framework set, details open; see docs/DESIGN_NOTES.md.
  Affects what the server must persist

## Open bugs and risks
- First-deploy tutorial lists controls that no longer exist (V melee, B shield, 1–5 switch): every new player sees it
- Editor: `refreshPrefabList` declared inside `bootEditor()`, called by `enterEditor` → ReferenceError on every entry
- The 3D scene renders every frame behind the opaque main menu (wasted GPU)
- One bot wedged by the poplar row near (-79, -83) in 1 of 8 headless matches (pre-existing nav behaviour)
- The gate checks syntax and names, not execution: script load order, TDZ at load, undefined identifiers and layering
  (core → game) pass it; a check-5 load smoke test is proposed

## Health (latest measurements)
| Metric | Value | When | Source |
|---|---|---|---|
| Combat allocation (GC churn) | ~4.9 MB/s | 2026-09-26 | audit CS11 |
| Village draw calls | 202 (was 398) | 2026-09-26 | audit CS12 |
| Headless match stall % (4 runs each) | base 7.6 / 4.5 / 10.7 / 10.8 · UI-step-1 20.5 / 4.8 / 8.0 / 7.2 | 2026-10-10 | sim.js |
| Headless match stall % (3 runs, Linux container) | 20.5 / 9.8 / 8.9 (run 1 = the known poplar wedge at −79,−83) | 2026-10-13 | sim.js |
| Gate | PASS; core 134 / game 691 / editor 346 symbols, 44 editor refs all guarded, 0 flagged | 2026-10-13 | gate.mjs |
| Gate (official battery, older) | 0 flagged, 44 editor refs all guarded | 2026-10-10 | battery.js |
