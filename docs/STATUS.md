# STATUS: the one-screen dashboard
_Kept current by docs-keeper after every change-set. Mustafa and the Advisor read this first._
_Last updated: 2026-10-10 (seeded by the Advisor before Phase 0)._

## Phase
**Phase 0: environment + green baseline** (git repo, Claude Code kit, portable toolkit, gate.mjs, CI).
Next: Phase 1 decisions (ADR-001, ADR-002), then the foundation refactor and the netcode MVP.

## In flight
| Work | Branch | State | Waiting on |
|---|---|---|---|
| UI theme transplant, step 1 (menu → dossier, pause → clipboard, field manual) | main (baseline) | delivered, battery PASS | Mustafa's Brave check incl. DEV-panel FPS on the menu |
| Phase 0 setup | cs/phase-0 | not started | first Claude Code session |

## Targets
- **Multiplayer:** up to 36 players per match: 16v16 (4 squads of 4 per team) or 12v12v12 (3 squads of 4; one team per
  faction). Bots fill empty slots. Milestones: 2–8 players → 16 → full scale, with load tests at each step.

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

## Health (latest measurements)
| Metric | Value | When | Source |
|---|---|---|---|
| Combat allocation (GC churn) | ~4.9 MB/s | 2026-09-26 | audit CS11 |
| Village draw calls | 202 (was 398) | 2026-09-26 | audit CS12 |
| Headless match stall % (4 runs each) | base 7.6 / 4.5 / 10.7 / 10.8 · UI-step-1 20.5 / 4.8 / 8.0 / 7.2 | 2026-10-10 | sim.js |
| Gate (official battery) | 0 flagged, 44 editor refs all guarded | 2026-10-10 | battery.js |
