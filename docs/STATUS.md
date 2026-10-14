# STATUS: the one-screen dashboard
_Kept current by docs-keeper after every change-set. Mustafa and the Advisor read this first._
_Last updated: 2026-10-14, editor-prefab-list on cs/editor-prefab-list._

## Phase
**Phase 0: environment + green baseline**: merged (PR #2, 2d30e4a, 2026-10-14, Advisor APPROVE; CS-1 / PR #1 approved
after the fact). Status stays **PARTIAL** until Mustafa's Windows run ("Check a new machine" in tools/README.md: npm ci,
gate, headed RTX renders, headless-gl, hook); Windows-only fixes become a small follow-up change-set.
**In flight (Advisor's order):** kit-v1-1 and editor-prefab-list (stacked on it). **Next:** gate-check-5 →
bot-poplar-wedge → Prompt 1 (scale test) → Prompt 2 (build spike) → Prompt 3 (ADRs).

## In flight
| Work | Branch | State | Waiting on |
|---|---|---|---|
| UI theme transplant, step 1 (menu → dossier, pause → clipboard, field manual) | main (baseline) | delivered, battery PASS | Mustafa's Brave check incl. DEV-panel FPS on the menu |
| kit-v1-1 (settings.json applied exactly + standing-permission paragraph in CLAUDE.md; proposed amendment: 19 deny rules) | cs/kit-v1-1, PR open | gate PASS, no code touched; deny rules verified live | Advisor OK on the amendment; after merge Mustafa runs `/permissions` locally |
| editor-prefab-list (3 editor functions hoisted out of `bootEditor()`) | cs/editor-prefab-list, PR open, stacked on PR #3 | gate PASS (+3 editor symbols), verifier PASS, all 3 ReferenceErrors gone in a headless run | Advisor's APPROVE; Mustafa's Brave check in editor.html; PR #3 merging first |
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
- Editor three-bug entry (prefab list, Maps list, Save as prefab): fixed on cs/editor-prefab-list (pending merge +
  Brave check); see the in-flight row
- The gate's default base is the local `main` even when it is behind `origin/main`: a stale local main silently gives an
  older base. Proposed fix: prefer the newer of the two, in gate-check-5
- GitHub branch protection on `main` is the real guard against pushes to main (deny rules are fragile by design);
  recommend Mustafa enables it (GitHub Pro via the Student Developer Pack if the free plan lacks it for private repos)
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
| Gate | PASS; core 134 / game 691 / editor 349 symbols (+3 −0 vs 2d30e4a), 44 editor refs all guarded, 0 flagged | 2026-10-14 | gate.mjs |
| Gate (official battery, older) | 0 flagged, 44 editor refs all guarded | 2026-10-10 | battery.js |
