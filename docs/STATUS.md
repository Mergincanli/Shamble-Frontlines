# STATUS: the one-screen dashboard
_Kept current by docs-keeper after every change-set. Mustafa and the Advisor read this first._
_Last updated: 2026-10-14, kit-v1-2 on cs/kit-v1-2._

## Phase
**Phase 0: environment + green baseline**: merged (PR #2, 2d30e4a, 2026-10-14, Advisor APPROVE; CS-1 / PR #1 approved
after the fact). Status stays **PARTIAL** until Mustafa's Windows run ("Check a new machine" in tools/README.md: npm ci,
gate, headed RTX renders, headless-gl, `/permissions`, syntax-hook test); Windows-only fixes become a small follow-up
change-set. Gate check 5 is now on main, so that machine needs a fresh `npm ci` in tools/ (eslint-scope) and Node 22.13+.
**Merged today:** editor-prefab-list (PR #4, c458fe7), gate-check-5 (PR #5, 8675367). **In flight:** kit-v1-2
(narrows one allow rule), plus read-only diagnosis of the poplar wedge and the editor Ctrl+Z bug.
**Queue (Advisor, 2026-10-14; one change-set at a time):** 1. kit-v1-2 → 2. bot-poplar-wedge → 3. tutorial-field-manual
(the first-deploy tutorial becomes the clipboard's field-manual mode: one controls list, no dead V / B / 1–5 keys) →
4. ui-step-2 (service record + end of round, war-room paper family) → 5. scale-test (Prompt 1) → 6. ui-step-3 (armory +
scoreboard, dark field-kit crate) → 7. build-spike (Prompt 2), then the ADR run (Prompt 3, docs only). Why: visual and UI
work lives in the client and survives the multiplayer refactor; gameplay systems (progression, new units, Spy, squads,
three teams) wait for the foundation refactor so they are built once in the shared simulation. Not yet slotted: the
editor Ctrl+Z ground-texture bug (plan coming with the wedge plan).

## In flight
| Work | Branch | State | Waiting on |
|---|---|---|---|
| UI theme transplant, step 1 (menu → dossier, pause → clipboard, field manual) | main (baseline) | delivered, battery PASS | Mustafa's Brave check incl. DEV-panel FPS on the menu |
| kit-v1-1 (settings + 19 deny rules) | merged: PR #3, 5858864, 2026-10-14, Advisor APPROVE | done | Mustafa runs `/permissions` locally (Windows check) |
| kit-v1-2 (allow rule `Bash(npx --no -- playwright *)` → `Bash(npx --no -- playwright install chromium)`; one stale START_HERE line fixed) | cs/kit-v1-2, PR to be opened | gate PASS; Advisor's fix for the finding deferred from gate-check-5 | Advisor APPROVE, then Mustafa's go to merge |
| Diagnosis (read-only): bot-poplar-wedge + editor Ctrl+Z re-rolls ground textures | none yet | plans to follow | approval of each plan |
| editor-prefab-list | merged: PR #4, c458fe7, 2026-10-14 | done: Mustafa's Brave check of editor.html passed (no problems), Advisor APPROVE | none |
| gate-check-5 (5a smoke, 5b names, 5c layering, 5d script order; CI jobs gate + smoke) | merged: PR #5, 8675367, 2026-10-14 | done: Advisor APPROVE after his own full-gate run (PASS, 26 s) and mutations that failed correctly | none |
| Phase 0 setup | merged (2d30e4a) | PARTIAL | Mustafa's Windows run |

## Targets
- **Multiplayer:** three teams, one per faction, of 12–16 players each in squads of 4 (12v12v12 to 16v16v16, possibly
  larger): design for 48+ players. Bots fill empty slots. Milestones: 2–8 players → 16 → full scale, with load tests at
  each step.

## Decisions pending
- Taken by the Advisor 2026-10-14: gl advisories accepted for now (drop gl for Playwright-only renders if it won't
  install natively on Windows); hook path (`$CLAUDE_PROJECT_DIR`) waits for the Windows run; check 5 is its own
  change-set after the prefab fix; standing permissions granted (see CLAUDE.md)
- **ADR-001 Multiplayer architecture** (authority, tick, transport, hosting, offline play): not started
- **ADR-002 Code structure & build** (modules, bundler, TypeScript, double-click build): under discussion with Mustafa;
  evidence first via the build spike
- **ADR-003 Three.js version** (stay on r128 vs upgrade): deferred until after ADR-002
- **Design: progression** (Identity / Momentum / Mastery): framework set, details open; see docs/DESIGN_NOTES.md.
  Affects what the server must persist

## Open bugs and risks
- First-deploy tutorial lists controls that no longer exist (V melee, B shield, 1–5 switch): every new player sees it
- Editor: Ctrl+Z (undo) changes / re-rolls the ground textures (old, reported by Mustafa): being diagnosed, plan to follow
- Editor three-bug entry (prefab list, Maps list, Save as prefab): fixed and merged (PR #4, c458fe7), Brave check passed
- Gate default base (stale local `main`): fixed and merged (PR #5, 8675367)
- The kit allow rule `npx --no -- playwright *` was too broad: fixed on cs/kit-v1-2 (now only `install chromium`; other
  playwright commands ask), pending merge
- Deferred from gate-check-5 reviews: no SRI/CSP on the three.js CDN tag (belongs with hosting, ADR-001)
- GitHub branch protection on `main` is the real guard against pushes to main (deny rules are fragile by design);
  recommend Mustafa enables it (GitHub Pro via the Student Developer Pack if the free plan lacks it for private repos)
- The 3D scene renders every frame behind the opaque main menu (wasted GPU)
- One bot wedged by the poplar row near (-79, -83) in 1 of 8 headless matches (pre-existing nav behaviour); diagnosis under way, fix is the next change-set
- The gate did not check execution (load order, TDZ at load, undefined identifiers, layering): addressed by check 5,
  merged (PR #5, 8675367), live in CI

## Health (latest measurements)
| Metric | Value | When | Source |
|---|---|---|---|
| Combat allocation (GC churn) | ~4.9 MB/s | 2026-09-26 | audit CS11 |
| Village draw calls | 202 (was 398) | 2026-09-26 | audit CS12 |
| Headless match stall % (4 runs each) | base 7.6 / 4.5 / 10.7 / 10.8 · UI-step-1 20.5 / 4.8 / 8.0 / 7.2 | 2026-10-10 | sim.js |
| Headless match stall % (3 runs, Linux container) | 20.5 / 9.8 / 8.9 (run 1 = the known poplar wedge at −79,−83) | 2026-10-13 | sim.js |
| Gate | on main 8675367: full gate PASS in 26 s (Advisor's independent run), all 8 checks (1-4, 5a-5d); core 134 / game 691 / editor 349, 44 editor refs guarded (re-measured on main + kit-v1-2), smoke 8/8 + 5/5 steps | 2026-10-14 | gate.mjs |
| Gate (official battery, older) | 0 flagged, 44 editor refs all guarded | 2026-10-10 | battery.js |
