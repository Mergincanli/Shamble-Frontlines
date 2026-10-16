# STATUS: the one-screen dashboard
_Kept current by docs-keeper after every change-set. Mustafa and the Advisor read this first._
_Last updated: 2026-10-16, 3a three-teams-core built on cs/three-teams-core (head 15b1971, PR to be opened). PRs #10, #11, #13 merged; PR #14 open._

## Phase
**Phase 0: environment + green baseline**: merged (PR #2, 2d30e4a, 2026-10-14, Advisor APPROVE; CS-1 / PR #1 approved
after the fact). Status stays **PARTIAL** until Mustafa's Windows run ("Check a new machine" in tools/README.md: npm ci,
gate, headed RTX renders, headless-gl, `/permissions`, syntax-hook test); Windows-only fixes become a small follow-up
change-set. Gate check 5 is now on main, so that machine needs a fresh `npm ci` in tools/ (eslint-scope) and Node 22.13+.
**Merged 2026-10-14:** editor-prefab-list (PR #4, c458fe7), gate-check-5 (PR #5, 8675367), kit-v1-2 (PR #6, ef019e0),
bot-poplar-wedge (PR #7, 159bec4).
**Merged 2026-10-15/16:** editor-undo-ground (PR #8, b5f4ecf), docs-loop-v2 (PR #9, 67cf55b), map-dedupe-on-load
(PR #10), team-prep-fixes (PR #11, 513f2fc), docs-editor-v2 (PR #12, fec5e67), editor-hotfix (PR #13, 8f4c3b6), map-dedupe-on-load merged as 7ca2cc4. **In flight:**
**3a three-teams-core** (cs/three-teams-core: steps 1-16 plus review fixes built, PR to be opened; waits on the
Advisor's APPROVE and Mustafa's Brave check) and PR #14 (kit-v1-3, reporting rule; open, waits on the Advisor's APPROVE).
Editor v2 comes after the playable greybox loop (Advisor's queue; LOOP_V2 §11 defines the loop but has no editor step).
**Greybox phase (LOOP_V2 §11):** the new core loop (three teams, harvest money, uplink tech, points race) is built as a
greybox prototype with bots, one change-set at a time; each plan says how it meets the [DECIDED] and [DEFAULT] items and
flags any [OPEN] item it needs decided; all numbers live in one tunable data table; [LATER] items are not built.
**Queue (Advisor, 2026-10-15; one change-set at a time):** greybox phase: 1. three-teams, split by the approved plan into
0. team-prep-fixes (merged) → **3a three-teams-core (built, PR to be opened)** → N1 (editor Play never rebuilds the nav
grid, a one-line change-set) → 3b three-teams-hud → 3c spawn-and-buy → "ramps as slopes in SIM" (lets `_botFireWorld` go) →
2. octopon-3-greybox (NC-7 blast reach by line of sight comes before its playtests) →
3. harvest-economy → 4. bot-replan-storm (the navSteer replan storm, bot-poplar-wedge PERF-2) → 5. uplinks →
6. win-and-scoring → then the scale test on the Octopon-3 greybox (N = 12, 24, 36, 48). **Paused until the greybox loop
is playable:** tutorial-field-manual, ui-step-2, ui-step-3, build-spike and the ADR run (Prompt 3). house-textures-seeded
stays low priority and unslotted (seeded house textures also make before/after renders exact). The old order (client
visual/UI work first, gameplay after the foundation refactor) is paused by LOOP_V2 §12.

## In flight
| Work | Branch | State | Waiting on |
|---|---|---|---|
| **3a three-teams-core** (teams as data, 12-slot soldier table, team-filtered shots, two-layer damage, per-team objectives, bot targeting with a LOS budget, DM_Tri arena at `?map=tri`, interim end rule; symbols +22 -0 vs main) | cs/three-teams-core (head 15b1971, 25 commits on main 7ca2cc4) | built, steps 1-16 plus review fixes; each commit message carries its evidence; PR to be opened | Advisor's APPROVE, then Mustafa's Brave check |
| kit-v1-3 (reporting rule) | PR #14 | open | Advisor's APPROVE |
| UI theme transplant, step 1 (menu → dossier, pause → clipboard, field manual) | main (baseline) | delivered, battery PASS | Mustafa's Brave check incl. DEV-panel FPS on the menu |
| kit-v1-1 (settings + 19 deny rules) | merged: PR #3, 5858864, 2026-10-14, Advisor APPROVE | done | Mustafa runs `/permissions` locally (Windows check) |
| kit-v1-2 (narrowed the playwright allow rule) | merged: PR #6, ef019e0, 2026-10-14, Advisor APPROVE | done | none |
| bot-poplar-wedge (`shamble-game.js` only: navBuild clearance 0.7 m + half a cell, navPath start from nearest open cell, 3-call warm-up at end of bootGame; symbols +0 -0) | merged: [PR #7](https://github.com/Mergincanli/Shamble-Frontlines/pull/7), 159bec4, 2026-10-14, Advisor APPROVE | done; report `docs/reports/2026-10-14-bot-poplar-wedge.md` | none |
| editor-undo-ground (`shamble-game.js` only: `addBox`, `addPath`, `makeBuilding`, `makeBarracks` and the `buildMap` terminal now call `stylizeOne()`; `clearMap` now clears `trees[]` and `props[]`; symbols +0 -0) | merged: [PR #8](https://github.com/Mergincanli/Shamble-Frontlines/pull/8), b5f4ecf, 2026-10-15, Advisor APPROVE | done: Mustafa's Brave check passed; report `docs/reports/2026-10-14-editor-undo-ground.md` | none |
| team-prep-fixes (B1 bots hit through the SIM hit world; B7 allocation-light LOS and shot checks via `rayStatics` (103 B / 648-750 B per call); SIM tick in headless runs; B2 bar width; B3 enemy armory; B5 scoreboard churn; B6 map names; B-labels are the three-teams plan's, not AUDIT_BUGS_PERF B1-B3; symbols +5 -0) | merged: [PR #11](https://github.com/Mergincanli/Shamble-Frontlines/pull/11), 513f2fc, 2026-10-16, Advisor APPROVE | done: Mustafa's Brave feel check passed; report `docs/reports/2026-10-15-team-prep-fixes.md` | none |
| docs-editor-v2 (docs only: EDITOR_V2.md + map-editor feature inventory with recommended keep / drop / improve marks) | merged: [PR #12](https://github.com/Mergincanli/Shamble-Frontlines/pull/12), fec5e67, 2026-10-16, Advisor APPROVE | done | Mustafa reviews the inventory marks |
| docs-loop-v2 (docs only: docs/LOOP_V2.md added; pointers in GAME_DESIGN §5–6, DESIGN_NOTES (death tax → Recruit, Spy target → harvester building, Momentum largely answered, three-team question partly answered) and START_HERE's doc list; STATUS / ROADMAP queue) | merged: [PR #9](https://github.com/Mergincanli/Shamble-Frontlines/pull/9), 67cf55b, 2026-10-15, Advisor APPROVE | done | none |
| map-dedupe-on-load (`dedupeMapDef` in game + `edFlashHint` in editor, symbols +2; drop exact duplicate trees / props when a saved map loads; status-bar note; no auto-save) | merged: [PR #10](https://github.com/Mergincanli/Shamble-Frontlines/pull/10), 2026-10-16, Advisor APPROVE | done: Mustafa's Brave check passed; report `docs/reports/2026-10-14-map-dedupe-on-load.md` | Mustafa loads and re-saves each old map once |
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
- **ADR-001 Multiplayer architecture** (authority, tick, transport, hosting, offline play): not started. Inputs logged so far: the 11 wall-clock reads (performance.now / Date.now) that make same-seed matches differ; no SRI/CSP on the three.js CDN tag; server-side map-def validation (SEC-MDL-2: `__proto__` key through `envMerge` / `rulesMerge`). Added by 3a: one hit world for every shooter is a multiplayer invariant (K1: bots still use `_botFireWorld`, a view of SIM without ramp slots, until ramps become slopes in SIM); `update()` must become phased (today's real order is in TECHNICAL §4a, K4); the server must repeat the team allow-list (`teamOf`); boot-time `Math.random` draws per slot (`bootGame` soldier loop, `enemyFire` spread, strafe and respawn type); three.js UUIDs draw from the shared `Math.random`; the bot-shot FX cull makes the shared `Math.random` sequence camera-dependent (cosmetic effects need their own RNG stream); `applyDamage` runs `addTech`, whose HUD writes and era toasts should become an event; humans take over bot slots; the LOS budget is sized for 12 soldiers (480 checks/s caps demand from about 24 soldiers in combat; fair since the review fix); map data needs a full schema check server-side (`placeObjectives` checks only the objective keys; `version:1` is written but never read) [verified in code: no `.version` read in shamble-*.js]
- **ADR-002 Code structure & build** (modules, bundler, TypeScript, double-click build): under discussion with Mustafa;
  evidence first via the build spike
- **ADR-003 Three.js version** (stay on r128 vs upgrade): deferred until after ADR-002
- **Design: progression:** LOOP_V2 §8 now settles the shape: team tech tiers in the match; Service points plus
  WARDOGS-style three-gate unlocks on the account; the unlock catalogue is [LATER]. Affects what the server must persist
- **Three-teams plan: APPROVED** with D1-D16 plus three tweaks: D6 (the 200 HP Empire elite leaves the bot roster, done in 3a [verified: `spawnEnemyMesh(_fac,false)`, `elite:false`]; its kit code
  stays for a possible Tier 3 or hero unit), D11 (your own HQ down = you lose; the win and loss rules stay placeholders until
  win-and-scoring) and DM_Tri (`?map=tri` follows LOOP_V2 §9's 6 m gap rule and gets the bot reachability check)
- **LOOP_V2 [OPEN] items:** Spy cost and rules (tech or money, disguise break, hack time, steal amount, warning);
  aiming at height with ledges (decide in octopon-3-greybox); Octopon-3 map size (start near Cabana, adjust with the scale test)

## Open bugs and risks
- **3a, ranked (all from the change-set's evidence; not re-measured by docs-keeper):**
  1. Cabana: buildings are not NAV cells, so 27.5 % of objective routes and 25 % of siege legs cross a building roof bots can't climb (`tools/reach.js`; R19: the `navBuild` building-cell fix with bot replanning)
  2. Allocation grows with team fights (garbage-included inspector profile, MB per game second inside update(), seeded): default Cabana natural match head 4.6-4.8 vs main 2.4-3.1 (head's ally and Soviet bots fight, main's bots don't), Cabana ring combat head 1.72-2.09 vs main 1.94-1.98; DM_Tri 12 soldiers 2.9-3.0 and 48 soldiers 7.2-8.2 after the review fixes (main Cabana at 48: 6.2-6.3). Driver: `updateSoldierAnim` hit/death states (the CS11 anim split); `syncSimWorld` runs twice per step since K4 (0.39-0.43 after the house-array fix, was 0.56-0.65; main 0.16 for its one call)
  3. Draw calls: about 120 per on-screen soldier (pre-existing). Merge per-material parts, prune shadow casters, add LOD before team size rises
  4. DM_Tri: Soviet and Empire enterable barracks doors face south (rot 0 for the nav grid); the HUD shows two teams, two silo dots and "Cabana Republic" (3b)
  5. Colour (3b review, Brave decides): ally teal bars and tracers have little value contrast on grass; unit-type accents tint bot faces red / blue on any team; the 'SUPPLY CACHE' world label clips (pre-existing)
  6. Truck lanes are map data, so moving a refinery in the editor leaves its dock (editor change-set)
  7. `teamSpawnPoint` at team size 16 can put a soldier inside a building, and the spawn line runs along world x (3c)
- First-deploy tutorial lists controls that no longer exist (V melee, B shield, 1–5 switch): every new player sees it (fix paused with tutorial-field-manual until the greybox loop is playable)
- team-prep-fixes follow-ups (low): NC-7 the grenade-arc preview still loops `rayStatic` per static (about 77.7 KB per call while aiming; small perf change-set); PERF-1 `resolveShot` about 112 B per live player (5.4 KB per shot at 48) and PERF-2 the 7 boxed `rayStatics` arguments (the 103 B left per LOS call), both deferred to multiplayer prep; player shots and LOS still see ramp boxes (needs a sloped-ramp primitive in SIM: queued as "ramps as slopes in SIM"); NC-1 (bot sight gate flat at 1.45 m): fixed in 3a, `acquireTarget` now uses each soldier's own height [verified: shamble-game.js:1588]. Label note: "NC-7" here is the grenade-arc preview; the 3a queue's NC-7 is blast reach by line of sight (ROADMAP)
- Editor: Ctrl+Z (undo) changes the ground textures (old, reported by Mustafa): fixed and merged (PR #8, b5f4ecf; Mustafa confirmed Ctrl+Z in Brave). Root cause: `stylizeScene` runs only at boot, so rebuilt roads / crates / buildings / barracks / terminal lost toon + ink; also `clearMap` never cleared trees / props, so every undo / redo / reset / load stacked another copy of all 70 trees and 36 props, also into snapshots and saved maps
- Maps saved on older builds after an undo may contain duplicate trees and props: map-dedupe-on-load drops exact duplicates on Load (PR #10, merged, Brave check passed). Limit: on an old stacked save, Cabana trees and props that the map had deleted or moved come back once. They are single entries, so they stay; Mustafa removes them by hand. Measured on the edited fixture: 69 / 35 intended, 71 / 37 after dedupe
- Deferred (map-dedupe-on-load): prefabs saved by marquee over a stacked area can stamp exact stacks again. Exposure is small: prefabs could only be saved since dbe24f8 (2026-10-14). The next map Save and Load drops them. Follow-up: dedupe on prefab load or stamp with the same key
- Deferred, info, older (SEC-MDL-2): `envMerge` / `rulesMerge` use `Object.assign`, so a stored `__proto__` key swaps the prototype of that local env/rules sub-object (`Object.prototype` stays clean). Belongs with the server-side map-def validation (ADR-001)
- Editor bugs reported by Mustafa (2026-10-15, editor.html on main): placed trees can't be deleted; rotation does nothing when the slider changes; the editor lags after a few placements. Fixed by editor-hotfix (PR #13, 8f4c3b6, merged after Mustafa's Brave check): delete, rotation, inspector selection capture; the lag needed no code fix, PR #8 and PR #10 removed its causes
- N1 (found in 3a): the editor's Play never rebuilds the nav grid; a one-line change-set, queued right after 3a (Advisor, 2026-10-16) [verified: `editorPlay` sets mode='game' itself and never calls `deploy()`, where `navBuild()` runs]
- Deferred, low (editor-undo-ground PERF-2): `_disposeObj3D` disposes the shared cached BK textures (houses / covers already did on base; trees / props add the well roof `tile`). r128 re-uploads them next frame, nothing breaks. Follow-up: skip `BK._tex` textures in `_disposeObj3D`
- Pre-existing, low (art review): `serializeMap` rounds to 0.1 m, so the resupply terminal snaps about 2 cm on a rebuild; the editor playtest camera clips into the barracks roof (no roof cutaway in playtest)
- Deferred from gate-check-5 reviews: no SRI/CSP on the three.js CDN tag (belongs with hosting, ADR-001)
- GitHub branch protection on `main` is the real guard against pushes to main (deny rules are fragile by design);
  recommend Mustafa enables it (GitHub Pro via the Student Developer Pack if the free plan lacks it for private repos)
- The 3D scene renders every frame behind the opaque main menu (wasted GPU)
- Bot nav, editor-made layouts (R1, tracked per the Advisor): rotated colliders at odd angles keep up to 0.34 m of solid inside free nav cells, and wall gaps of about 3-5.4 m can close for bots depending on grid alignment (built-in maps unaffected). Recommendation: an editor-side reachability check; new maps keep walkable gaps at least 6 m wide
- Bot nav, navSteer "replan storm" (old; bot-poplar-wedge PERF-2): a bot whose straight line flickers clear/blocked replans every few frames; the wedge fix hits it more on Romania (budget-limited frames 0.9% -> 1.8%). Advisor: its own change-set in the greybox phase (bot-replan-storm)
- Bot nav, A* gives up after 8000 nodes (old): a few far-corner routes fail and fall back to a straight line
- The game reads performance.now()/Date.now() in 11 places, so matches with the same seed differ run to run. Tracked as an input for ADR-001 (deterministic shared simulation)

## Health (latest measurements)
| Metric | Value | When | Source |
|---|---|---|---|
| Combat allocation (GC churn) | ~4.9 MB/s (audit harness, scripted combat; not comparable to the 2026-10-15 headless match) | 2026-09-26 | audit CS11 |
| Perf baselines, team-prep-fixes head (headless): `hasLOS` | 103 B per call (was 20-26 KB) | 2026-10-15 | reports/2026-10-15-team-prep-fixes.md |
| `resolveShot` / `enemyFire` / `syncSimWorld` | `resolveShot` 648-750 B at 6 players, about 5.4 KB at 48 (was 18.1 KB) · `enemyFire` 0.88-1.0 KB per call (was 3.7-7.0 KB) · `syncSimWorld` 561 B per call (pre-existing, the houses fallback array) | 2026-10-15 | reports/2026-10-15-team-prep-fixes.md |
| Allocation method (the audit row is stated to use it; the team-prep-fixes rows do not name their method [assumed the same]) | Inspector sampling profile with `includeObjectsCollectedByMajorGC` / `MinorGC` (garbage included). `node --heap-prof` counts only objects alive at exit and is NOT used for rates. 3a rows are in Open bugs and risks (item 2); they come from the 3a commit messages, not re-run here | 2026-10-16 | tools/perfbots.js header |
| Combat garbage per game second | close-combat ring 6 / 12 bots 2.5-3.2 / 2.0-2.5 MB (was 15.5-16.1 / 28.3-28.5) · natural 90 s match 1.79-1.97 MB (was 2.80-3.05) | 2026-10-15 | reports/2026-10-15-team-prep-fixes.md |
| Village draw calls | 202 (was 398) | 2026-09-26 | audit CS12 |
| Headless match stall % (tools/sim.js, 40 unseeded runs each) | base mean 8.82 / median 7.5 · bot-poplar-wedge mean 7.73 / median 6.5 | 2026-10-14 | sim.js |
| Seeded match pairs (240) | tree-face stall windows 1885 -> 0; mean stall 9.12% -> 8.50% | 2026-10-14 | scratch harness (report) |
| Editor rebuild (Cabana): full-map draw calls boot / 1 / 2 rebuilds | base 1992 / 2406 / 3502 · head 1992 / 1992 / 1992; heap after GC boot / 10 rebuilds: base 42.9 / 119.5 MB · head 42.7 / 44.5 MB | 2026-10-14 | preview.js harness + perf-auditor |
| Gate | on main 8675367: full gate PASS in 26 s (Advisor's independent run), all 8 checks (1-4, 5a-5d); core 134 / game 691 / editor 349, 44 editor refs guarded (re-measured on main + kit-v1-2), smoke 8/8 + 5/5 steps | 2026-10-14 | gate.mjs |
| Gate (official battery, older) | 0 flagged, 44 editor refs all guarded | 2026-10-10 | battery.js |
