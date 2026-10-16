# STATUS: the one-screen dashboard
_Kept current by docs-keeper after every change-set. Mustafa and the Advisor read this first._
_Last updated: 2026-10-15, team-prep-fixes on cs/team-prep-fixes ([PR #11](https://github.com/Mergincanli/Shamble-Frontlines/pull/11) open). PR #9 docs-loop-v2 merged._

## Phase
**Phase 0: environment + green baseline**: merged (PR #2, 2d30e4a, 2026-10-14, Advisor APPROVE; CS-1 / PR #1 approved
after the fact). Status stays **PARTIAL** until Mustafa's Windows run ("Check a new machine" in tools/README.md: npm ci,
gate, headed RTX renders, headless-gl, `/permissions`, syntax-hook test); Windows-only fixes become a small follow-up
change-set. Gate check 5 is now on main, so that machine needs a fresh `npm ci` in tools/ (eslint-scope) and Node 22.13+.
**Merged 2026-10-14:** editor-prefab-list (PR #4, c458fe7), gate-check-5 (PR #5, 8675367), kit-v1-2 (PR #6, ef019e0),
bot-poplar-wedge (PR #7, 159bec4).
**Merged 2026-10-15:** editor-undo-ground (PR #8, b5f4ecf), docs-loop-v2 (PR #9, 67cf55b). **In flight:**
team-prep-fixes ([PR #11](https://github.com/Mergincanli/Shamble-Frontlines/pull/11), step 0 of the three-teams plan;
waits on the Advisor's APPROVE and Mustafa's Brave feel check; next is 3a three-teams-core after the APPROVE),
map-dedupe-on-load (PR #10, Advisor APPROVE, waits on Mustafa's Brave check), and a read-only diagnosis of three editor
bugs (trees can't be deleted, rotation slider does nothing, lag after a few placements). docs-editor-v2 (docs only:
EDITOR_V2.md plus the map-editor feature inventory) will run alongside, after the editor diagnosis; editor v2 itself
comes after the playable greybox loop (Advisor's queue; LOOP_V2 §11 defines the loop but has no editor step).
**Greybox phase (LOOP_V2 §11):** the new core loop (three teams, harvest money, uplink tech, points race) is built as a
greybox prototype with bots, one change-set at a time; each plan says how it meets the [DECIDED] and [DEFAULT] items and
flags any [OPEN] item it needs decided; all numbers live in one tunable data table; [LATER] items are not built.
**Queue (Advisor, 2026-10-15; one change-set at a time):** map-dedupe-on-load (approved, Brave check) → greybox phase: 1. three-teams,
split by the approved plan into 0. team-prep-fixes (PR #11) → 3a three-teams-core → 3b three-teams-hud →
3c spawn-and-buy (docs-editor-v2, docs only, runs alongside) → 2. octopon-3-greybox →
3. harvest-economy → 4. bot-replan-storm (the navSteer replan storm, bot-poplar-wedge PERF-2) → 5. uplinks →
6. win-and-scoring → then the scale test on the Octopon-3 greybox (N = 12, 24, 36, 48). **Paused until the greybox loop
is playable:** tutorial-field-manual, ui-step-2, ui-step-3, build-spike and the ADR run (Prompt 3). house-textures-seeded
stays low priority and unslotted (seeded house textures also make before/after renders exact). The old order (client
visual/UI work first, gameplay after the foundation refactor) is paused by LOOP_V2 §12.

## In flight
| Work | Branch | State | Waiting on |
|---|---|---|---|
| UI theme transplant, step 1 (menu → dossier, pause → clipboard, field manual) | main (baseline) | delivered, battery PASS | Mustafa's Brave check incl. DEV-panel FPS on the menu |
| kit-v1-1 (settings + 19 deny rules) | merged: PR #3, 5858864, 2026-10-14, Advisor APPROVE | done | Mustafa runs `/permissions` locally (Windows check) |
| kit-v1-2 (narrowed the playwright allow rule) | merged: PR #6, ef019e0, 2026-10-14, Advisor APPROVE | done | none |
| bot-poplar-wedge (`shamble-game.js` only: navBuild clearance 0.7 m + half a cell, navPath start from nearest open cell, 3-call warm-up at end of bootGame; symbols +0 -0) | merged: [PR #7](https://github.com/Mergincanli/Shamble-Frontlines/pull/7), 159bec4, 2026-10-14, Advisor APPROVE | done; report `docs/reports/2026-10-14-bot-poplar-wedge.md` | none |
| editor-undo-ground (`shamble-game.js` only: `addBox`, `addPath`, `makeBuilding`, `makeBarracks` and the `buildMap` terminal now call `stylizeOne()`; `clearMap` now clears `trees[]` and `props[]`; symbols +0 -0) | merged: [PR #8](https://github.com/Mergincanli/Shamble-Frontlines/pull/8), b5f4ecf, 2026-10-15, Advisor APPROVE | done: Mustafa's Brave check passed; report `docs/reports/2026-10-14-editor-undo-ground.md` | none |
| team-prep-fixes (B1 bots hit through the SIM hit world; B7 allocation-light LOS and shot checks via `rayStatics` (103 B / 648-750 B per call); SIM tick in headless runs; B2 bar width; B3 enemy armory; B5 scoreboard churn; B6 map names; B-labels are the three-teams plan's, not AUDIT_BUGS_PERF B1-B3; symbols +5 -0) | cs/team-prep-fixes (ea1ebb8, 751c694; main merged as ca493e6); [PR #11](https://github.com/Mergincanli/Shamble-Frontlines/pull/11) open | built; gate PASS (8 checks); perf-auditor and netcode-architect PASS WITH NOTES; art-director FAIL on AD-1, fixed; report `docs/reports/2026-10-15-team-prep-fixes.md` | Advisor APPROVE; Mustafa's Brave feel check (incoming fire about 25 % weaker on open ground) |
| docs-editor-v2 (docs only: EDITOR_V2.md + map-editor feature inventory) | not started | will run alongside the greybox queue; editor v2 itself comes after the playable greybox loop (Advisor's queue) | after the editor diagnosis |
| docs-loop-v2 (docs only: docs/LOOP_V2.md added; pointers in GAME_DESIGN §5–6, DESIGN_NOTES (death tax → Recruit, Spy target → harvester building, Momentum largely answered, three-team question partly answered) and START_HERE's doc list; STATUS / ROADMAP queue) | merged: [PR #9](https://github.com/Mergincanli/Shamble-Frontlines/pull/9), 67cf55b, 2026-10-15, Advisor APPROVE | done | none |
| map-dedupe-on-load (drop exact duplicate trees / props when a saved map loads; status-bar note; no auto-save) | cs/map-dedupe-on-load; [PR #10](https://github.com/Mergincanli/Shamble-Frontlines/pull/10), now based on main | built; gate PASS; 29 harness checks; security-auditor and art-director PASS WITH NOTES; Advisor APPROVE | Mustafa's Brave check, then merge |
| Diagnosis (read-only): poplar wedge + editor Ctrl+Z re-rolls ground textures | n/a | done, both plans approved (wedge: bot-poplar-wedge; Ctrl+Z: editor-undo-ground) | none |
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
- **ADR-001 Multiplayer architecture** (authority, tick, transport, hosting, offline play): not started. Inputs logged so far: the 11 wall-clock reads (performance.now / Date.now) that make same-seed matches differ; no SRI/CSP on the three.js CDN tag
- **ADR-002 Code structure & build** (modules, bundler, TypeScript, double-click build): under discussion with Mustafa;
  evidence first via the build spike
- **ADR-003 Three.js version** (stay on r128 vs upgrade): deferred until after ADR-002
- **Design: progression:** LOOP_V2 §8 now settles the shape: team tech tiers in the match; Service points plus
  WARDOGS-style three-gate unlocks on the account; the unlock catalogue is [LATER]. Affects what the server must persist
- **Three-teams plan: APPROVED** with D1-D16 plus three tweaks: D6 (the 200 HP Empire elite leaves the bot roster; its kit code
  stays for a possible Tier 3 or hero unit), D11 (your own HQ down = you lose; the win and loss rules stay placeholders until
  win-and-scoring) and DM_Tri (`?map=tri` follows LOOP_V2 §9's 6 m gap rule and gets the bot reachability check)
- **LOOP_V2 [OPEN] items:** Spy cost and rules (tech or money, disguise break, hack time, steal amount, warning);
  aiming at height with ledges (decide in octopon-3-greybox); Octopon-3 map size (start near Cabana, adjust with the scale test)

## Open bugs and risks
- First-deploy tutorial lists controls that no longer exist (V melee, B shield, 1–5 switch): every new player sees it (fix paused with tutorial-field-manual until the greybox loop is playable)
- team-prep-fixes follow-ups (low): NC-7 the grenade-arc preview still loops `rayStatic` per static (about 77.7 KB per call while aiming; small perf change-set); PERF-1 `resolveShot` about 112 B per live player (5.4 KB per shot at 48) and PERF-2 the 7 boxed `rayStatics` arguments (the 103 B left per LOS call), both deferred to multiplayer prep; player shots and LOS still see ramp boxes (needs a sloped-ramp primitive in SIM); NC-1 the bot sight gate is still flat at 1.45 m (shamble-game.js:2663), planned fix in 3a item 8 (approved plan)
- Editor: Ctrl+Z (undo) changes the ground textures (old, reported by Mustafa): fixed and merged (PR #8, b5f4ecf; Mustafa confirmed Ctrl+Z in Brave). Root cause: `stylizeScene` runs only at boot, so rebuilt roads / crates / buildings / barracks / terminal lost toon + ink; also `clearMap` never cleared trees / props, so every undo / redo / reset / load stacked another copy of all 70 trees and 36 props, also into snapshots and saved maps
- Maps saved on older builds after an undo may still contain duplicate trees and props; map-dedupe-on-load (PR #10, approved) drops exact duplicates on load
- New editor bugs reported by Mustafa (2026-10-15, editor.html on main): placed trees can't be deleted; rotation does nothing when the slider changes; the editor lags after a few placements. Read-only diagnosis under way; a hotfix plan follows if the fix is small
- Deferred, low (editor-undo-ground PERF-2): `_disposeObj3D` disposes the shared cached BK textures (houses / covers already did on base; trees / props add the well roof `tile`). r128 re-uploads them next frame, nothing breaks. Follow-up: skip `BK._tex` textures in `_disposeObj3D`
- Pre-existing, low (art review): `serializeMap` rounds to 0.1 m, so the resupply terminal snaps about 2 cm on a rebuild; the editor playtest camera clips into the barracks roof (no roof cutaway in playtest)
- Editor three-bug entry (prefab list, Maps list, Save as prefab): fixed and merged (PR #4, c458fe7), Brave check passed
- Gate default base (stale local `main`): fixed and merged (PR #5, 8675367)
- The kit allow rule `npx --no -- playwright *` was too broad: fixed and merged (PR #6, ef019e0; now only `install chromium`)
- Deferred from gate-check-5 reviews: no SRI/CSP on the three.js CDN tag (belongs with hosting, ADR-001)
- GitHub branch protection on `main` is the real guard against pushes to main (deny rules are fragile by design);
  recommend Mustafa enables it (GitHub Pro via the Student Developer Pack if the free plan lacks it for private repos)
- The 3D scene renders every frame behind the opaque main menu (wasted GPU)
- Bot wedge at the poplar row near (-79, -83): fixed and merged (PR #7, 159bec4)
- Bot nav, editor-made layouts (R1, tracked per the Advisor): rotated colliders at odd angles keep up to 0.34 m of solid inside free nav cells, and wall gaps of about 3-5.4 m can close for bots depending on grid alignment (built-in maps unaffected). Recommendation: an editor-side reachability check; new maps keep walkable gaps at least 6 m wide
- Bot nav, navSteer "replan storm" (old; bot-poplar-wedge PERF-2): a bot whose straight line flickers clear/blocked replans every few frames; the wedge fix hits it more on Romania (budget-limited frames 0.9% -> 1.8%). Advisor: its own change-set in the greybox phase (bot-replan-storm)
- Bot nav, A* gives up after 8000 nodes (old): a few far-corner routes fail and fall back to a straight line
- The game reads performance.now()/Date.now() in 11 places, so matches with the same seed differ run to run. Tracked as an input for ADR-001 (deterministic shared simulation)
- The gate did not check execution (load order, TDZ at load, undefined identifiers, layering): addressed by check 5,
  merged (PR #5, 8675367), live in CI

## Health (latest measurements)
| Metric | Value | When | Source |
|---|---|---|---|
| Combat allocation (GC churn) | ~4.9 MB/s (audit harness, scripted combat; not comparable to the 2026-10-15 headless match) | 2026-09-26 | audit CS11 |
| Perf baselines, team-prep-fixes head (headless): `hasLOS` | 103 B per call (was 20-26 KB) | 2026-10-15 | reports/2026-10-15-team-prep-fixes.md |
| `resolveShot` / `enemyFire` / `syncSimWorld` | `resolveShot` 648-750 B at 6 players, about 5.4 KB at 48 (was 18.1 KB) · `enemyFire` 0.88-1.0 KB per call (was 3.7-7.0 KB) · `syncSimWorld` 561 B per call (pre-existing, the houses fallback array) | 2026-10-15 | reports/2026-10-15-team-prep-fixes.md |
| Combat garbage per game second | close-combat ring 6 / 12 bots 2.5-3.2 / 2.0-2.5 MB (was 15.5-16.1 / 28.3-28.5) · natural 90 s match 1.79-1.97 MB (was 2.80-3.05) | 2026-10-15 | reports/2026-10-15-team-prep-fixes.md |
| Village draw calls | 202 (was 398) | 2026-09-26 | audit CS12 |
| Headless match stall % (tools/sim.js, 40 unseeded runs each) | base mean 8.82 / median 7.5 · bot-poplar-wedge mean 7.73 / median 6.5 | 2026-10-14 | sim.js |
| Seeded match pairs (240) | tree-face stall windows 1885 -> 0; mean stall 9.12% -> 8.50% | 2026-10-14 | scratch harness (report) |
| Editor rebuild (Cabana): full-map draw calls boot / 1 / 2 rebuilds | base 1992 / 2406 / 3502 · head 1992 / 1992 / 1992; heap after GC boot / 10 rebuilds: base 42.9 / 119.5 MB · head 42.7 / 44.5 MB | 2026-10-14 | preview.js harness + perf-auditor |
| Gate | on main 8675367: full gate PASS in 26 s (Advisor's independent run), all 8 checks (1-4, 5a-5d); core 134 / game 691 / editor 349, 44 editor refs guarded (re-measured on main + kit-v1-2), smoke 8/8 + 5/5 steps | 2026-10-14 | gate.mjs |
| Gate (official battery, older) | 0 flagged, 44 editor refs all guarded | 2026-10-10 | battery.js |
