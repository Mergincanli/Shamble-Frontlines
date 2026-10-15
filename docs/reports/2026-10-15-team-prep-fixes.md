# Advisor report: team-prep-fixes (2026-10-15, branch cs/team-prep-fixes, base main b5f4ecf)

## Goal
- Bots hit you through the same hit world your bullets use, no longer through your animated model.
- Line-of-sight and shot checks stop producing garbage memory.
- Headless sims see live positions.
- Four small bugs fixed: bar width, enemy armory, scoreboard churn, map names.

This is step 0 of the approved three-teams plan (bugs before features). PR:
[#11](https://github.com/Mergincanli/Shamble-Frontlines/pull/11). The B labels are the three-teams plan's, not the
2026-09-26 B1–B3 in `AUDIT_BUGS_PERF.md`.

## Outcome
- **All six fixes are done**, and each has its own before/after measurement (table below).
- **Gate:** PASS, all 8 checks; symbols +5 −0.
- **Reviews:**
  - perf-auditor: PASS WITH NOTES.
  - netcode-architect: PASS WITH NOTES.
  - art-director: FAIL, on one high finding (AD-1: ramps blocked bot fire). It is fixed in the second commit.
- **Commits:**
  - ea1ebb8: the fixes.
  - 751c694: the review fixes AD-1, AD-2, NC-3 and NC-6.
- **Still needed:**
  - the Advisor's APPROVE;
  - Mustafa's Brave feel check: incoming fire is about 25 % weaker on open ground (Risk 1).

## What changed (3 files, +68 −39; symbols +5 −0)
- **shamble-core.js:**
  - `rayStatics` loops over every static with the box test inlined (B7).
    - `hasLOS` and `resolveShot` use it.
    - Their signatures are unchanged.
  - `SIM._nrm` also holds the hit t.
- **shamble-game.js:**
  - **SIM tick:** `update()` advances the SIM sync guard.
  - **SIM ids:**
    - The player is in `SIM.players` as id 0 (`SIM_PLAYER`). Bots are their `enemies` index + 1.
    - The 4 readers are remapped, and every player-side shot passes `SIM_PLAYER`.
    - A death clears the victim's SIM entry at once.
  - **Bot fire (B1):**
    - `enemyFire` resolves through `resolveShot` on a SIM view (`_botFireWorld`, `_eHitN`).
    - That view holds only the player and every static except ramps.
    - It aims at the same SIM snapshot it hit-tests, from a chest point that follows both heights.
  - **B2:** bar fill uses the bot's real max health.
  - **B3:** only the player's own armory opens the shop.
  - **B5:** `sbSync` writes through the change-only setters.
  - **B6:** `curMap` keeps the built map's name for `serializeMap`.
- **shamble-editor.js:** Save stamps the save name into the stored map (one line in the Save handler).

## Evidence: each fix before → after
Harnesses are in the session scratchpad `tpf/` and `tpf/rev/`. Both builds use the same seed. Byte counts are exact;
µs vary up to 2× on this shared machine (perf-auditor), so they are ranges.

| Fix | main (before) | this branch |
|---|---|---|
| **SIM tick:** largest gap between SIM and the live bots after 600 headless steps | 42.0 m | 0.07 m (one step of movement) |
| **B7:** line-of-sight check per call | 6.9–16 µs, 20–26 KB | 2.7–5.7 µs, 103 B |
| **B7:** shot check per call (220 statics, 6 soldiers) | 8.1–17 µs, 18,105 B | 5.0–8.6 µs, 648–750 B |
| **B7:** same results as the old code, 20,000 random rays incl. rotated pieces (1,640 rotated hits) | 20,000 / 20,000 | 20,000 / 20,000 |
| **B1:** raycasts against your animated model per 9,000 bot shots | 9,000 | 0 |
| **B1:** bot hit rate on you, open ground, 6 bots ringed, 500 shots each, at 15 / 30 / 45 m | 27.6 / 4.2 / 1.8 % | 20.0 / 2.9 / 1.2 % |
| **B1:** one bot shot | 30–93 µs, 3.7–7.0 KB | 5.5–12 µs, 0.88–1.0 KB |
| **B1:** dirt puffs floating > 30 cm above a ramp, 1,200 bot shots at a player on the 4 m ramp | 0 | 0 |
| **B1:** bot hit rate on you at 25 / 50 / 75 % up each Cabana ramp (1,200 shots per spot) | see the ramp table | see the ramp table |
| **B1 + B7:** combat garbage, close-combat ring, 3 seeds, 6 / 12 bots | 15.5–16.1 / 28.3–28.5 MB per game second | 2.5–3.2 / 2.0–2.5 MB per game second |
| **B1 + B7:** garbage in a natural 90 s match | 2.80–3.05 MB per game second | 1.79–1.97 MB per game second |
| **B2:** bar fill at full health (Conscript / Peacekeeper / Warrior / Officer / elite) | 1 / 1.5 / 1.2 / 1.1 / 2 | 1 / 1 / 1 / 1 / 1 |
| **B3:** shop opens at your / the enemy armory | yes / yes | yes / no |
| **B5:** scoreboard open, DOM changes per frame (real page) | 38 | 0 |
| **B6:** map name after boot / after building a map named "My Test Map" | DM_Romania / DM_Romania | Cabana Republic / My Test Map |
| **B6:** editor Save as "Name Test", stored name (real page) | DM_Romania | Name Test |

**Ramps (the AD-1 fix):** bot hit rate at 25 / 50 / 75 % up each ramp, 6 bots ringed at 15 m.

| Ramp | main | first commit (from the review) | this branch |
|---|---|---|---|
| (-68.9, 101.9), 2.5 m | 26.3 / 15.1 / 10.4 % | 0.9 % at 25 % | 16.7 / 13.2 / 14.3 % |
| (-52.9, 103.9), 4 m | 20.3 / 8.7 / 0.6 % | 0.0 / 0.0 at 25 / 50 % | 18.6 / 17.4 / 12.9 % |
| (-100, 90), 4 m, next to the spawn | 16.6 / 5.8 / 0.3 % | 0.0 / 0.0 at 25 / 50 % | 14.3 / 10.0 / 11.6 % |
| open ground (control) | 26.4 % | 20.3 % | 20.3 % |

- **Floating puffs:** on the first commit, 657 of 1,000 dirt puffs over the 4 m ramp floated more than 30 cm, up to
  3.51 m. On this branch, 0 float.
- **The 200 puffs that land over that ramp's footprint:**
  - They sit on the face of the platform at the ramp's top end.
  - They come from one ring bot that the harness put inside that platform, which can't happen in play.
- **Other runs:**
  - `tools/sim.js` ×3 stall %: main 13.0 / 5.7 / 7.3 (mean 8.67); this branch 3.4 / 16.1 / 6.5 (mean 8.67). This is within the usual
    noise. The main-branch sims ran with SIM frozen at the first step, so their line-of-sight used stale statics.
  - update() per frame in a natural match, headless: 1.62–1.82 ms → 1.26–1.58 ms.
- **Gate:** PASS, all 8 checks; symbols +5 −0 (`SIM_PLAYER`, `_botFireWorld`, `_eHitN`, `curMap`, `rayStatics`).

## Reviews
- **perf-auditor: PASS WITH NOTES.** Its verdict is an improvement with no hot-path regression and no new per-frame
  allocation. Opt status is TurboFan on all changed hot functions, and there is no deopt loop.
  - PERF-1 (low): `resolveShot` still allocates about 112 B per live player.
    - That is 0.65 KB per shot at 6 players and 5.4 KB at 48.
    - Deferred to multiplayer prep as its own change-set (an inline broad-phase reject).
  - PERF-2 (low): the 7 float arguments of `rayStatics` are boxed, which is the 103 B per line-of-sight check left. It
    is deferred together with PERF-1.
  - PERF-3: my first draft quoted the best seed (1.97 MB/s at 6 bots). This report now gives the 3-seed range.
  - PERF-4 (info): `syncSimWorld`'s 561 B per call predates this change-set. It comes from the houses fallback array,
    and the new player row adds 0 B. Logged.
  - PERF-5 (info): the change-only writes cost +16 B per scoreboard update, and they save 38 DOM writes per frame.
  - PERF-6 and PERF-7: method notes and baselines. The baselines are saved in STATUS.
- **netcode-architect: PASS WITH NOTES.** It verified the following:
  - bit-identical hits on 30,000 rays;
  - the id scheme, with all 4 readers and all 4 shooters remapped;
  - one sync per update;
  - death clearing within the same step;
  - no new clock or random reads.

  Its findings:
  - NC-1 (low): the bot sight check still uses a flat 1.45 m eye and target height. So bots that see you on a ledge
    now hit at your real height, but they rarely decide to shoot you there. This is unchanged from main and is fixed in
    3a item 8 (the sight check uses the same heights as the shot).
  - NC-2 (low): see Deviations.
  - NC-3 (low): bots aimed at the live player but hit-tested the snapshot. Fixed: aim and hit test now share one
    snapshot.
  - NC-4 and NC-5 (low): 3a plan notes, added to the 3a plan.
    - Reuse `SIM_PLAYER`.
    - List `_botFireWorld` as a planned drop.
    - Bots pass their own SIM id plus the team filter.
    - Parked slots stay in `SIM.players` as dead entries, so `SIM.players[id].id===id` always holds. The 3a roster
      harness asserts it.
  - NC-6 (info): a NaN ray now counts as blocked. Comment added.
  - NC-7 (info): the grenade-arc preview (`shamble-game.js:1516`) still loops `rayStatic`, at 77.7 KB per call while
    you aim a grenade. It gets its own follow-up perf change-set.
  - NC-8 (info): doc drift. Fixed by docs-keeper in this change-set.
- **art-director: FAIL → fixed.** No locked look values were touched.
  - AD-1 (high): ramps sit in SIM as full-height boxes, so you were nearly unhittable on ramps, and dirt puffs floated
    in mid-air. Fixed: bot fire skips the ramp slots, as main's sight meshes never held ramps. Evidence is in the ramp
    table above.
    - Your own shots at bots on ramps still use the box. This is older, and a sloped-ramp test in SIM is logged for later.
  - AD-2 (low): a hit closer than the muzzle drew the tracer from 0.9 m ahead. Fixed: it is now drawn from the chest.
  - AD-3 (info): about a quarter of bot shots now leave a ground puff, which reads as good near-miss feedback. The Brave
    check judges whether it is noise.
  - AD-4 (info): the bars are confirmed fixed in render.
  - AD-5 (info): incoming pressure on open ground is about −25 %. Tune only after the Brave check.
  - AD-6: the Brave checklist (below).

## Decisions and deviations
- **Bots still only shoot at the player.**
  - Their hit query uses a SIM view that holds the player alone, so other bots don't block their shots, as before.
  - The plan's team filter in `resolveShot` (pass through teammates) comes in 3a and replaces this view.
- **Bot ids are `enemies` index + 1, with the player as 0, as in the plan.** 3a's slot table keeps this scheme.
- **The sync guard (NC-2):** it reuses the render counter `_frameId` (one `_frameId++` at the top of `update()`)
  instead of the plan's separate sim-step counter.
  - The behaviour is the same today: one sync per update and per frame, verified.
  - It can't serve as a network tick. The real tick belongs to ADR-001, so I left it.
- **Ramps:** they are left out of bot fire only. Player shots and line of sight still see the ramp box, as on main.
- **Symbols:** the plan expected "possibly a state object and a slab helper". The actual adds are 5 small names (listed
  above), with no drops.
- **B6, old saves:** saves made before this keep their stored name "DM_Romania" until they are saved again (Save now
  stamps the save name).

## Risks and open questions
1. **Balance:** bots now hit you about 25 % less often on open ground. Their shots use the same hit boxes as yours, and
   trees and props block them as they block your bullets. *Recommendation: keep it; your Brave feel check
   decides.* If it feels too easy, tune `T.enemySpread` or the per-class damage, not the hit shape.
2. **Not verified:** Brave on a real GPU, and frame time in a real browser (all numbers are headless).
3. **Being shot on ramps is still rare in play.** The sight check (NC-1) is older and flat. It is fixed in 3a.

## For Mustafa in Brave
1. Play a match on Cabana and stand on open ground under fire from 3 or more bots for about 10 s.
   - Does incoming fire feel the same, fairer, or too weak?
   - Do the ground dirt puffs read as near misses, with no audio spam?
2. **Bot health bars:**
   - All bars at full health are the same width, with no red past the dark backing (Peacekeepers and Warriors used to
     overflow).
   - The elite still reads as the elite.
3. Walk to the **enemy** barracks armory and press E: the shop should not open. Your own still does.
4. Hold Tab during a fight: the scoreboard works as before. Check the DEV-panel FPS while it's open.
5. Fight near and beside the ramps, including the spawn ramp. No dirt puffs should hang in the air above a slope, and
   tracers should end on visible geometry.
6. A bot hugging a wall corner while firing at you: look for backward tracers or a muzzle flash on the wrong side of the
   wall.

## Proposed next step
three-teams-core (3a), with the approved tweaks:
- the elite kit code is kept;
- the win and loss rules are stated as placeholders;
- DM_Tri follows the 6 m gap rule and gets the reachability check;
- the NC-4 and NC-5 plan notes are added.

Separately, a small perf change-set for the grenade-arc loop (NC-7).
