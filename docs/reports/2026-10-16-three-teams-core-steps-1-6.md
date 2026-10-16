# Advisor report: three-teams-core (3a) steps 1–6 (2026-10-16, branch cs/three-teams-core, base PR #11 head a3f0471)

## Goal
Build step 3a of the approved three-teams plan on top of PR #11. Where the plan doesn't cover a decision, write it down
and stop instead of guessing (Advisor instruction, 2026-10-16).

## Outcome
- **Built:** steps 1–6 of 16, one commit each, all pushed. None of them needs an answer to the open questions.
- **Stopped before step 7.** Steps 7, 8 and 10 need the Advisor's answers to K1, K2 and K4 (below), and most later
  steps build on 7.
- **No pull request.** The interim rule parks your own team's bots until step 10, so the branch is pushed for review
  only.
- **Gate:** PASS on every step, all 8 checks. Symbols +10 −0: `TEAMS`, `TUNE` (core); `PLAYER_TEAM`, `teamOf`,
  `sameTeam`, `teamFilter`, `TEAM_INDEX`, `buildTeamIndex`, `SOLDIERS`, `activateRoster` (game).
- **Reviews:** netcode-architect, perf-auditor, art-director and security-auditor all PASS WITH NOTES.
  - Fixed (7a7bb0b, 5695e56): two should-fixes, the parked-slot scene graph and the same-frame SIM entry, and three
    notes.
  - Deferred: two should-fixes, the hit lookup to step 7 and the ally health-bar colour to step 10, because those steps
    rewrite the same lines.
- **Diff (code):** 5 files, +193 −115 lines.

## What changed, step by step
| Step | Commit | What it does | Behaviour change |
|---|---|---|---|
| 1 | 1c05a06 | `TEAMS` (allies, soviets, empire) and `TUNE` (team size 4, friendly fire off, AI budget, spawn offsets, FX cull radii) in core. | none |
| 2 | bc9cf62 | `PLAYER_TEAM`, `teamOf` (the one place team strings are read; legacy 'friend' / 'hostile' and faction names map to ids), `sameTeam`, `teamFilter`. | none |
| 3 | ee57572 | Per-team tables for every id: `teamEco`, team colours, `powerDead`, `priceMult`, era-up and `addTech`. 'friend' / 'hostile' stay as aliases of the same objects until 3b. | `addTech('empire')` no longer throws |
| 4 | 9e5d3a2 | `TEAM_INDEX`: per-team HQs, refinery, power, barracks, markers, truck, dock and lane, rebuilt by `buildTeamIndex`. Every `findB` caller reads it (no closures per frame). | none |
| 5a | e2b1b1b | Every relation check goes through `sameTeam` / `PLAYER_TEAM`: shots, the Jug cannon, grenades, aim, truck payout, tech box, silos, armory panel, shop warning, kill tech, DEV buttons. `nearestFriendlyBuilding` loops the index. | none |
| 5b | 7ec86f0 | Buildings, barracks and trucks store canonical ids (`'allies'`, `'soviets'`, `'empire'`, `null` when teamless); colours come from the relation to your team with today's hex values. Editor: mirror, symmetry overlay, palette, labels and Team buttons use ids. | saves write the new ids, one-way: opened in an older build, every building is red, none can be damaged and the round can't be won; a mirrored Soviets spawn marker becomes Allies; a spawn faction named `constructor` now falls back to Soviets |
| 6 | 634c806 | `SOLDIERS`: 12 fixed slots (0 = you, 1–3 allies, 4–7 soviets, 8–11 empire); `en.id` is the SIM id. `activateRoster` parks the slots and trucks of teams without an HQ. One truck per team. No elite (D6). D8: no barracks gate on respawn. "Empire Spawn" label (N9). | Cabana is you vs 4 Soviets (was 6 bots with 1 Empire elite); bots respawn with their barracks down; Empire bots wear the standard soldier |

## Evidence (base a3f0471 → head; headless Chromium / SwiftShader; harnesses run the real code)
| Check | Result |
|---|---|
| 5a: shots and blasts on all 8 buildings and 2 trucks, nearest building on 81 points, payouts and toasts, tech box, kill and DEV tech, shop warning, armory panel | byte-identical output on base and head |
| 5b game: counts, every building/barracks/truck colour, trickle, payout, armory, shots, blasts, red reticle over each building, era-up toasts, victory and defeat, DM_Romania, save → load | identical; only the stored and saved ids differ |
| 5b editor (real editor.html): palette, labels, mirror twin, symmetry overlay, legacy prefab stamp, inspector highlight, set team by click, save → load, undo | identical; differences are only the ids and the mirrored marker faction |
| 5b renders of the 5 base sites (both HQs, both barracks, refinery) | pixel-identical |
| 6 roster harness | PASS: 12 slots, ids, 12 SIM entries with ids and teams; Cabana 4 active Soviets, 7 parked; 600 frames with no revival and 0 explosions; respawn 172 frames (refinery up) / 241 (down); respawn with barracks down; 3-HQ map → Cabana keeps every id; no-HQ map → Soviets; scoreboard 4 rows; menu shows 4 |
| 6 editor.html | the editor hides all bots; Play on a 3-HQ map shows 8, back on Cabana 4; 0 page errors |
| sim.js ×3, steps 4 / 5a / 5b / 6 | 0 errors everywhere; stall 5.9–12.5 / 4.9–9.1 / 5.8–11.7 / 13.5–22.6 % (base 10.2–18.9 %) |
| Stalls away from buildings (siege-aware variant) | base 3.5 / 2.5 / 3.6 %; head 1.4 / 3.4 / 2.6 % (step 6) and 3.3 / 3.3 / 4.9 % (after the review fix, which changes no per-frame bot code), so within random-AI noise. The rest of step 6's stall figure is bots standing still to shell buildings, a larger share with 4 bots than with 6 |
| Allocation per sim second | no change. With 4 bots on both builds (perf-auditor, 600-frame warm-up): base 1.90–4.01 MB, head 1.92–2.03 MB. As shipped, base runs 6 bots and head 4, so head reads lower (3.63–4.18 vs 3.90–5.92 MB). Each active bot costs about 2.4 KB per frame |
| Scene matrix update (after the review fix) | 0.61–0.74 ms per call, base 0.71–0.90 ms; 2,180–2,197 scene nodes, base 2,467–2,486 |
| Legacy strings (`'friend'`/`'hostile'` quoted; `.friend`/`.hostile`) | game 54 / 29 → 7 / 1; what is left: the objective owners (step 9), `teamOf` itself, one comment, one CSS class token and the `teamEco` aliases (3b) |

Harnesses and outputs are in the session scratchpad: `tt3a/h/`, `tt3a/s5a`, `s5b`, `s6`, `s6f`, `r5b`, `r6`, `ed6`, and
the perf-auditor's `perf3a/`.

## Reviews
- **art-director: PASS WITH NOTES.**
  - No locked value was touched. The five base-site renders are byte-identical. Dropping the elite matches
    ART_DIRECTION §7 and §9.
  - Should-fix, **deferred to step 10:** bot health bars are always red (`spawnEnemyMesh`). That is right while every
    active bot is hostile, but ally bots will need relation colours when step 10 unparks them.
  - Note: the Empire coat is very dark (about 5 % luminance against a 15–25 % target), so the outline gets lost in it.
    It is an existing palette value, offered to 3b's colour review.
  - Note: arm and crest colours come from the unit, not the faction.
  - Note: ART_DIRECTION §7 and §9 still say the elite spawns. The wording goes to the docs pass at the end of 3a.
  - It asked for a render of the new Cabana lineup. Taken: `tt3a/r6/ab_cabana.png` (base: the elite and 5 Soviets;
    head: 4 Soviets, no stray bar or mesh).
- **security-auditor: PASS WITH NOTES.**
  - `teamOf` compares with `===` only, so prototype keys can't pass. Every new bracket lookup is keyed by `teamOf`
    output or fixed ids. No map text reaches `innerHTML`.
  - Note 1, **fixed:** the DEV log of an unknown team now prints at most 40 characters, or the value's type. A bare
    `String(x)` would itself throw on map data like `{"toString":5}`. Checked with 14 hostile inputs: no throw, longest
    log 63 characters.
  - Note 2: the future server must apply the same allow-list itself; the client is not a trust boundary (ADR-001 list).
  - Note 3, outside this diff: the prefab store reads `this.mem[n]` with a typed name, so 'constructor' returns a
    built-in. Logged for an editor change-set.
- **netcode-architect: PASS WITH NOTES.** The id scheme is sound and matches NC-4 and NC-5. Ids are fixed at boot and
  survive buildMap, resetRound and map switches. Every damage path checks `alive`. Its own probe showed a ray passing
  a parked slot to the active bot behind it.
  - Should-fix 1, **fixed (5695e56):** a slot parked mid-frame stayed alive in SIM until the next frame.
    `activateRoster` now forces a re-sync. The harness checks it: a slot parked and shot in the same frame is dead in
    SIM, and the ray passes it.
  - Should-fix 2, **deferred to step 7:** hits are looked up with `enemies[r.id-1]`, not `SOLDIERS[r.id]` (updateAim,
    the Jug, castShot, bot fire). They are correct by today's invariant. Steps 7 and 8 rewrite exactly these lines for
    the team filter and attacker credit, so changing them twice would add risk for no gain.
  - Note, **comment fixed:** the slot layout gives the player's team size-1 bot slots. ADR-001 should consider `size`
    slots per team, with humans taking over bot slots.
  - Note, for the ADR-001 debt list: at boot every slot draws `Math.random` whatever the map. Move those draws into the
    round reset when a seeded RNG arrives.
  - Note, **done:** the harness now fires rays and blasts through a parked slot.
- **perf-auditor: PASS WITH NOTES.** No per-frame allocation and no new deopts. `teamOf`, `sameTeam`, `syncSimWorld`
  and `nearestFriendlyBuilding` all reach TurboFan; `sameTeam` costs 7.45 ns per call with 0 bytes.
  - Should-fix, **fixed (5695e56):** parked soldiers stayed in the scene graph, and three r128 updates the matrices of
    hidden objects too. Parked groups now leave the scene, and a group coming back goes through `stylizeOne`. The
    reviewer's fix as written would have left the 7 bots parked at boot unstyled: `stylizeScene` runs after the boot
    activation (game:3181 vs 3045).
    - Scene nodes on Cabana: 3,189–3,225 → 2,180–2,197 (base 2,467–2,486).
    - `scene.updateMatrixWorld`: 0.92–1.06 → 0.61–0.74 ms (base 0.71–0.90 ms; 3 × 1,000 calls, headless).
    - Styling is the same with forced unit types, for every slot, including the Empire slots now styled on
      activation: 105 meshes, 49 outlines, the same materials, colours, gradients and outline widths.
    - The late-styled Empire bots render with the normal toon shading and outlines (`tt3a/r6/fix_bots_crop.png`).
  - Note, **applied below:** the allocation figures compared 6 bots with 4. With equal bot counts the builds match, so
    the right reading is "no change".
  - Note, not done: `powerDead` loops all buildings. It runs per frame only while the resupply prompt shows, over at
    most 10 buildings.
  - Note, not done: a DEV log flood if a bad value reached a per-frame call site. Per-frame call sites only see stored
    canonical ids, `null` or the internal objective strings, which never log; only map build can log.

## Decisions and deviations
- **px / pz** are pre-declared as `undefined`, not `0` as the checklist says: `tickVertical` tests `!==undefined`, and
  `0` would run a wall check from (0, 0) on a bot's first step.
- **TEAM_INDEX** gains two fields the checklist doesn't list: `blds` (all of a team's buildings, so
  `nearestFriendlyBuilding` has no closure) and `active`. `firstEnemy` is also set when the index is declared, because
  HUD code can read it before the first map loads.
- **Trucks count deliveries** (`hv.docks`). `tools/sim.js` already reads this field; until now it was always 0.
- **Teamless barracks store `null`, not `'neutral'`.** The symmetry overlay then treats a teamless barracks on the axis
  as its own mirror; before, it expected a friendly twin.
- **`T.enemyCount` and `T.eliteCount` are no longer read.** The roster size comes from `TUNE.teams.size`.
- **Parked soldiers are out of the scene graph** (review fix). A slot parked at boot is styled when it first plays.
  Because three.js draws `Math.random` for every new material's id, that styling also shifts the shared random
  sequence at that moment. Nothing is seeded today, so nothing changes in play. It adds to the ADR-001 case for a sim
  RNG of its own.
- **Covered items built as listed for veto:** N2, N3, N9 and R16 (see `DECISIONS-3a` in the session).
- **Logged, not fixed (N1):** the editor's Play button never rebuilds the nav grid. That is its own one-line
  change-set.

## The questions that stop 3a (for the Advisor)
- **K1: keep `_botFireWorld`** instead of the planned drop? Since PR #11's ramp fix it holds the ramp-free statics
  that bot fire uses; dropping it brings the ramp regression back. Recommended: keep it, make its player list
  `SIM.players`, and reword NC-4. **Blocks** step 7, and through it steps 8, 10, 13 and 14.
- **K2: blast reach by line of sight (NC-7)?** Built as written, it makes buildings and trucks immune to grenades and
  makes 80 % of bolt blasts on surfaces lose their damage. Recommended: defer NC-7 to its own change-set. **Blocks**
  the blast part of step 8.
- **K4: the SIM update order (NC-3).** The code doesn't match the plan's order. Recommended: write down the real order
  and add one forced re-sync after the player moves (about 21 µs per step). **Blocks** step 10.

## Not verified
- **Brave on Windows or a real GPU:** all runs are headless SwiftShader.
- **A real three-team map:** DM_Tri is step 14. The 3-HQ map here is synthetic (Cabana plus an Empire HQ and refinery
  at z −100).
- **PR #10's stacked-map fixture:** #10 is not in this branch.
- **A full match by hand:** sims are 90 s headless runs.

## For Mustafa (only if you want to try the branch now; the real Brave check comes when 3a is done)
1. Play Cabana: 4 Soviet bots, no Empire elite, and none of your own team's bots yet (expected until step 10).
2. Destroy the Soviet barracks: Soviet bots still respawn.
3. Editor: the palette says "Empire Spawn". Mirror a building: the twin switches team. The Team buttons highlight
   correctly.
4. Keep test saves from this build in this build. In an older build they show every building red, and the round
   can't be won.

## Proposed next step
1. The Advisor answers K1, K2 and K4.
2. Then steps 7–16, ending with a PR and the Brave check.
