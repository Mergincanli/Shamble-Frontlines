# Advisor report: three-teams-core (3a) complete (2026-10-16, branch cs/three-teams-core, base main 7ca2cc4)

## Goal
Make the game three teams underneath: every team's bots fight every other team, shots and damage know who owns them,
objectives and spawns work per team, and a test arena (DM_Tri) shows three teams playing. The look and HUD stay
two-team for now; 3b does the three-team HUD and colours, 3c does spawning and buying.

## Outcome
**DONE.** All 16 steps are built, plus the fixes from a full five-lens review. Gate PASS on every step.
- Steps 1–6 and their reviews: `docs/reports/2026-10-16-three-teams-core-steps-1-6.md`. This report covers 7–16, the
  final review and the state of the whole branch.
- What you see on Cabana: you and 3 ally bots against 4 Soviet bots. Allies spawn outside your barracks door,
  Soviets outside theirs. `index.html?map=tri` opens the three-team arena (4 + 4 + 4 soldiers).
- Symbols +22 −0 vs main. That is the plan's 14, the 4 the Advisor approved under K5, 2 optional helpers and 2 extras:
  `presentTeam` (step 9) and `ovHostileSpawn` (editor, review fix).
- 26 commits; code and tools: 20 files, +961 −298 lines.

## What changed (steps 7–16, then the review fixes)
| Step | Commit | What it does |
|---|---|---|
| 7 | 303f2f9 | Shots skip the shooter's own team unless friendly fire is on. Bots keep `_botFireWorld`, whose players list is the shared SIM list (K1). The crosshair never turns red on an ally. |
| 8 | 64dd3bd | `applyDamage` (state) and `onDamageEvent` (presentation): one damage path for every soldier, with owner, team and credit. Every victim falls; feedback is local (K3). Blast reach unchanged (K2). |
| 9 | 6c321fc | Tech box and silos count every team's presence. Siege uses the per-team index. Objectives can come from map data (`placeObjectives`). Saved maps carry `version:1` (N5). |
| 10 | 4805b92 | Bots pick the nearest visible soldier of any other team, within a sight-check budget. Ally bots are live. Bars and tracers use relation colours. One forced SIM re-sync after you move (K4). |
| 11 | 99f49e2 | `teamSpawnPoint`: own map markers, else outside the own barracks door, else beside the HQ. Deterministic. |
| 12 | c4fa1ff | You win when every other playing team with HQs has lost all of them. You lose when all of yours are down. |
| 13 | ea848ab | Scoreboard: your column lists your allies with their kills and deaths. |
| 14 | 2e4712a | Bot-shot effects are skipped beyond 50 m of the camera, and sound beyond 90 m. Damage is never touched. |
| 15 | 1e9c7c8 | DM_Tri (three bases 120° apart, field in the middle, 3 tech points, 3 silos) behind `?map=tri`; truck lanes as map data. |
| 16 | 47a02b6 | Tools: sim.js (kill matrix and team stats), reach.js (routes on the nav grid), perfbots.js (12/24/48 soldiers); preview.js `--map`, `--pre`. |
| fixes | 9a4a757, 15b1971 | Review fixes (below), and `syncSimWorld` no longer builds an array per house per call. |
| docs | 870f623 | STATUS, ROADMAP, TECHNICAL, MODULE_MAP, START_HERE, ART_DIRECTION (D6 elite wording). |

## Evidence (headless Chromium / SwiftShader; harnesses run the real code)
| Check | Result |
|---|---|
| Harnesses h6–h13, h15 (roster, rays, damage, objectives, targeting, spawns, end rule, HUD, DM_Tri) | all PASS on the final head. h5a is retired (it looks for the 'friend' id that step 5b removed). |
| End rule (h12) | 13 cases PASS: Soviet HQ → VICTORY; your HQ → DEFEAT; barracks, refinery and power end nothing; three teams need both enemy HQs; multi-HQ teams; no enemy HQ never wins. |
| FX cull, damage untouched | Seeded random, effects stubbed, cull off vs on: identical damage logs (60 s, 69 events; 180 s, 207 events including 26 building and 1 truck siege shot). Peak bot-shot lights 3 → 1–2. |
| DM_Tri | 120° symmetry within 0.007 m (enterable barracks 0.65 m, rounded to even metres for the nav grid); doorways and spawns open on the nav grid; 4 deliveries per team in 120 s; all 6 team pairs trade hits. |
| reach.js | DM_Tri: 180 routes 100 %, objective climbs 100 %, siege chain 100 %. Cabana (report only): routes 100 %, objective climbs 72.5 %, siege chain 75 % (buildings aren't nav cells, R19). |
| URL switch | `?map=tri` accepts 5 forms and rejects 13 (`trip`, `TRI`, `xmap=tri`…); no flag or `--map=bogus` gives Cabana. |
| Sims ×3 (90 s Cabana, pre-fix 47a02b6 vs final) | stall 0 / 0 / 0.4 % vs 0 / 0.8 / 0.7 %; 0 errors; 2 deliveries per team in every run. |
| DM_Tri 3 × 300 s (final) | kills by team: Allies 3/3/4 (your slot idles in sims), Soviets 5/3/8, Empire 6/6/6; 10 deliveries per team per run; 0 errors. Every team pair ≥ 1 kill in 2 of 3 runs (pre-fix: 1 of 3). |
| Fair bot turn order (48 soldiers, reviewers' own probe) | scans per bot per second, Allies / Soviets / Empire: before 3.57 / 1.00 / 0.03, after 2.39 / 2.42 / 2.25. Share of time with a target: before 0.99 / 0.45 / 0.00, after 0.97 / 0.99 / 0.95. Same at 144 fps. Route denials level (52–57 %). |
| Frame-rate independence | 12 soldiers: 4.00 scans per bot per second at 60 and 144 fps (before: 3.69–3.77 at 60, 4.00 at 144). |
| Allocation (method: inspector sampling profile including collected garbage, MB per game second inside update()) | Cabana ring combat: head 1.72–2.09, main 1.94–1.98. Cabana natural match: head 4.6–4.8, main 2.4–3.1 (head's allies and Soviets fight; main's bots don't). DM_Tri: 2.9–3.0 at 12 soldiers, 7.2–8.2 at 48; main Cabana at 48: 6.2–6.3. `syncSimWorld`: 0.39–0.43 after the house-array fix (was 0.56–0.65). |
| Frame cost (perfbots, mean ms per update step, 3 reps; head DM_Tri vs main Cabana) | 12 soldiers: combat 0.14–0.21 vs 0.17–0.23, march 0.15–0.34 vs 0.17–0.29. 48 soldiers (head): combat 0.80–1.00, p95 0.97–1.54 (main at 48 before: 1.08–1.12). |
| Fairness (perfbots, after the fix) | 12 soldiers: 4.0 scans per bot per second for every team, target share 0.99–1.00. 48: 2.55 / 2.54 / 2.47, target share 0.95 / 0.95 / 1.00. |
| Draw calls (one render pass) | Bots on a 26 m ring, mostly off screen: 354 (head, 12) vs 421 (main, 12); 1,145 at 48. Every soldier on screen: 1,480 vs 1,470 at 12; 5,843 at 48. Shader programs with every soldier on screen 16 vs 9 (more uniforms; each compiles once on first sight). |
| Renders | DM_Tri top-down, three uniforms fighting, pageshots of the HUD and scoreboard on both maps, editor overlay: 0 page errors. |
| Gate | PASS, all 8 checks, on every commit. |

**Correction:** the step 9 and step 10 allocation figures (0.094 and 0.117 MB/s) came from `node --heap-prof`, which
counts only objects still alive at exit. They were about 20× too low. The figures above use the right method.
tools/README, perfbots.js and START_HERE now say how to measure.

## Reviews (art-director, perf-auditor, security-auditor, netcode-architect, correctness bug-hunter; each finding then attacked by a skeptic)
All five: **PASS WITH NOTES**. 38 findings: 4 major, 15 minor, 19 nits.
- **Fixed:**
  - **Bots served in slot order (major; found by three reviewers):** the sight budget and the route budget went to
    the lowest slots first, so at 36–48 soldiers the Empire was nearly blind. The bot loop now starts one slot later
    every step.
  - **Timers dropped their remainder:** bots reacted 7–12 % faster at 144 fps. They now carry the remainder.
  - **Bad barracks values:** a non-numeric rotation or scale put a team's bots at NaN. Values are now checked.
  - **HUD "them":** the HUD could follow a team that isn't playing. It now follows the first playing team.
  - **Editor Spawn Distances:** the overlay measured from the old north band. It now uses the real enemy spawn.
  - **Small fixes:** ally scoreboard rows get a teal diamond; aabbs.js lists only live silos; sim.js counts only
    applied kills; comments corrected.
  - **Wrong allocation figure (major, perf):** corrected above.
- **Deferred, with reasons:**
  - **Allocation at 48 soldiers 1.4–1.8× main's (minor):** it comes from more hit and death animations in fights.
    It goes with the CS11 animation split, before team size rises.
  - **Draw calls, about 120 per soldier on screen (minor, older than 3a):** roadmap item before team size rises.
  - **Colours (minor):** ally teal has little contrast on grass, and unit accents tint faces red or blue on any team.
    Both go to 3b's colour review; Brave decides.
  - **Camera-dependent random sequence from the FX cull; `applyDamage` runs `addTech`'s HUD writes:** ADR-001 debt.
  - **Big-team spawn line (team size 16):** 3c.
  - **`version:1` never read, map schema check:** server debt.
  - The rest are nits.

## Decisions and deviations
- **Reach measure:** reach.js judges push routes as the bots actually walk them (nearest live structure first, stop
  15 m short). Routes straight to a rear building are reported for information only (DM_Tri 86.5 %).
- **Cull helper:** the optional cull helper (K5's `_botFxCulled`) is `_botShotD2`. It returns a distance because
  effects and sound use two radii.
- **Barracks orientation:** DM_Tri's enterable barracks keep rot 0 so the doorway stays open on the nav grid. The
  Soviet and Empire doors face south rather than into their own base.
- **Per-slot spawns:** spawns are fixed per slot (deterministic), so a bot respawns at the same spot each time
  (D13 interim).
- **Flinch strength:** it now uses the final damage (headshots flinch harder, leg shots less). I left it as is and
  list it here.

## Risks
1. Cabana: about a quarter of bot routes cross a building roof the bot can't climb, so bots stall against buildings
   (R19).
2. The sight budget is sized for 12 soldiers. Since the fairness fix every team shares it equally, but at 48
   soldiers in combat each bot scans 2.3 times a second instead of 4.
3. Allocation and draw calls grow fast past 24 soldiers (5,843 draw calls with 48 on screen). Keep team size 4 until
   the animation split and draw-call work land.
6. Seven more shader programs than main when all three uniforms are on screen. Each compiles once, so there may be a
   one-time hitch the first time a faction appears (Brave check 8).
4. On DM_Tri the HUD is still two-team (two team rows, two silo dots, "Cabana Republic" header) until 3b.
5. Allies' teal may be hard to read on grass. Brave decides.

## Proposed next step
N1: editor Play never rebuilds the nav grid. A one-line change-set, as the Advisor queued it.

## Decisions needed
1. **R19-ORDER: when does the nav-grid building fix go in?**
   - Today buildings aren't part of the bot pathfinding grid. On Cabana about a quarter of objective and siege
     routes cross a building roof, so bots walk into walls and stall.
   - Why it matters: it shows in every playtest on the default map.
   - Options: (a) right after N1, before 3b; (b) after 3c; (c) with the octopon-3 greybox.
   - **Recommend (a):** a small, local fix in the grid builder and route checks, and reach.js gives a ready
     acceptance test.
2. **LOS-SIZE: sight-check budget at 48 soldiers.**
   - The bots' line-of-sight budget (480 checks per second) covers 12 soldiers easily. At 48 it caps a fight's demand
     of about 1,100 per second, so bots react about half as often.
   - Why it matters: big-fight feel at the multiplayer target size.
   - Options: (a) keep 480 and size it in the 48-player performance change-set, with measured cost; (b) scale it now
     with the soldier count (about 20 per soldier, roughly 0.5 ms per game second at 48).
   - **Recommend (a):** today's game has 12 soldiers, and sizing belongs with the animation and draw-call work.
3. **K4-MEASURE: how to judge slot-order fairness from now on.**
   - The plan asked for "every team pair at least one kill per DM_Tri run". That holds in 2 of 3 runs. Kill counts are
     small (12–18 per 300 s run) and your idle slot lowers the Allies' share.
   - Options: (a) keep the kill rule with longer runs; (b) judge fairness by per-team scans and share of time with a
     target (perfbots fairness, the netcode reviewer's suggestion) and keep kill matrices as information.
   - **Recommend (b):** it measures the mechanism directly, and today it shows the three teams within 7 %.

## For Mustafa
Brave checks on the PR head (screenshots for each numbered item):
1. **Cabana (index.html), Deploy:** your 3 allies come out of your barracks door with teal health bars, and the
   Soviets out of theirs. Take a screenshot of the allies at spawn and of an allies-vs-Soviets fight over grass, with
   teal ally tracers and red Soviet ones.
2. **Hold Tab:** the left column shows You plus Ally 01–03 (teal diamonds, kills and deaths); the right column shows
   Hostile 01–04. Take a second screenshot after an ally dies: that row fades.
3. **Aim at an ally:** the crosshair stays white, and your bullets pass through allies.
4. **Ally death:** let a Soviet kill an ally near you. The body falls, and you get no damage number or hitmarker.
5. **Distant fights:** stay at spawn while bots fight far off. Gunfire fades with distance and stops beyond about
   90 m; effects appear within about 50 m. Tell me if anything visibly pops in.
6. **End of round:** destroy the Soviet HQ → VICTORY. Destroying a Soviet refinery or barracks ends nothing.
7. **index.html?map=tri:** take a screenshot of each of these:
   - the start view;
   - the first fight at the centre;
   - tech point Alpha;
   - a silo;
   - the Soviet and the Empire base;
   - Tab (8 hostiles).

   Known: the HUD still shows two teams.
8. **Frame rate:** note the DEV panel (backtick) FPS during a fight on Cabana and on `?map=tri`.
9. **editor.html:** check the Markers cards (Soviet / Allies / Empire Spawn). Turn on Spawn Distances: the hostile
   point should now sit outside the Soviet barracks door.
10. **Merge:** approve the merge after the Advisor's APPROVE. PR #14 (reporting rule) also waits on the APPROVE.
