# LOOP V2: the core loop (living spec)

Written by Mustafa and the Advisor, October 2026. This replaces the loop in GAME_DESIGN.md §5–6 and parts of
DESIGN_NOTES.md (listed in §12). It is written for readers with no other context, including Claude Code.

**Status tags.** `[DECIDED]` Mustafa decided it. `[DEFAULT]` the Advisor proposed it and it stands unless Mustafa
vetoes it. `[OPEN]` needs a decision before it gets built. `[LATER]` not in the greybox prototype.
**Every number here is a starting value.** Keep them all in one data table so they can be tuned without code changes.

---

## 1. The match in one paragraph
Three teams, one per faction, fight on a three-base map. Money comes from harvesters working a contested field in the
centre. Tech comes from rotating uplinks, where a carried tech cache is cashed out, and can be stolen. Tech lifts the
whole team through tiers that unlock abilities, stronger units and vehicles. The team that reaches the points target
first (or leads when time runs out) wins. Winning converts part of the match's tech into account progression.

## 2. Teams and spawning
- `[DECIDED]` Three teams, one per faction: Soviets, Allies, Empire. The target is 12v12v12 to 16v16v16, possibly
  more, in squads of 4. Bots fill empty slots.
- `[DEFAULT]` Team count and team size are data. The prototype starts at 4 per team and scales after the scale test.
- `[DECIDED]` Everyone spawns inside their own barracks, in front of the buy terminal.
- `[DEFAULT]` Spawn safety: 3 s of spawn protection, and enemies cannot enter a barracks interior.

## 3. Money (personal $)
- `[DECIDED]` Everyone starts each match with $300.
- `[DECIDED]` Harvesters gather from the field in the centre and deliver to their team's refinery. Every delivery pays
  every player on the team (Renegade-style).
- `[DECIDED]` The centre field is rich but contested. A small, safe field near each base gives a trickle, so a team's
  income never drops to zero.
- `[DECIDED]` Harvesters can be killed to deny enemy income.
- `[DECIDED]` A destroyed harvester is rebuilt with money from the whole team, split in equal shares across the
  current team. Nobody pays more than they have; the rest spreads over the others. The HUD shows each player's share
  ("Harvester rebuilt: −$X").
  - `[DEFAULT]` If the whole team can't cover it, the rebuild waits until it can. Rebuild delay about 20 s.
- `[DEFAULT]` Kills pay a small amount, so fighting pays but harvesting matters more.
- `[DEFAULT]` Power-plant pricing stays: a destroyed power plant raises that team's prices.

## 4. Tech (team) and the uplinks
- `[DECIDED]` Tech comes from uplinks, modelled on the cashout in The Finals:
  1. A tech cache spawns somewhere on the map; everyone is told where.
  2. A player picks it up and carries it: slow, and they can't shoot while carrying.
  3. They place it in an uplink and the countdown starts.
  4. Enemies steal the uplink by holding E for a few seconds. The countdown keeps running, now for the new owner.
  5. When the countdown ends, the owning team gets the tech. A new cache spawns somewhere else.
  - Uplink sites rotate. At full scale, two run at the same time.
- `[DEFAULT]` Uplinks replace silos and tech boxes: one tech source, the exciting one.
- `[DEFAULT]` Tech is a team total that climbs through tiers. Players never spend it, so no single player can drain
  the team's progress. A tier, once reached, is never lost. Tech stolen from a team slows its next tier.
- `[DECIDED]` Class abilities are locked at the start and unlock when the team reaches Tech 1, which makes tech worth
  fighting for. Every team reaches Tech 1 by minute 5 at the latest, so a team shut out of uplinks still gets its
  abilities.
- `[DEFAULT]` Tier ladder (thresholds to be tuned):

| Tier | Unlocks |
|---|---|
| Tech 0 | T1 units, guns only |
| Tech 1 | T1 class abilities (Molotov, Riot Shield, Banzai blade) |
| Tech 2 | T2 units (Officer, Javelin) and vehicles (Juggernaut; Hammer tank once built) |
| Tech 3 | T3 units (the Spy) and heavy options |

## 5. Units
- `[DECIDED]` Tier 1, $100 each:
  - **Conscript:** AMK rifle; ability Molotov.
  - **Peacekeeper:** shotgun; ability Riot Shield.
  - **Imperial Warrior:** SMG, strong up close; ability Banzai blade (one-hit kill while active).
- `[DEFAULT]` All three are shared by every faction, because the counter triangle needs them (Warrior beats Conscript
  beats Peacekeeper beats Warrior). Factions differ through uniforms and signature vehicles.
- `[DEFAULT]` A free fallback unit: the **Recruit**, armed with a Luger pistol. Death costs the unit you bought: you
  respawn as a Recruit and buy again, keeping your money.
- `[DECIDED]` **Javelin:** a rocket unit, strong against vehicles and aircraft, weak against infantry, with a Luger
  sidearm. `[DEFAULT]` Tier 2. Its rockets knock infantry back: little damage, big impact, so it still feels good.
- `[DEFAULT]` The **Officer** moves to Tier 2.
- `[DECIDED]` `[LATER]` **The Spy** (Tier 2 or 3), inspired by the Red Alert 3 and Team Fortress 2 spies. It can
  disguise itself as the enemy, and hack an enemy harvester building (refinery) to steal that team's tech. It is
  built for real players, who create the memorable spy moments (bots make poor spy-checkers).
  - `[OPEN]` Mustafa's idea is that it costs tech. The Advisor recommends gating it behind Tech 3 and charging money,
    so one player can't spend the team's shared tech.
  - Also open: what breaks the disguise, how long the hack takes, how much it steals, and how defenders are warned.

## 6. Vehicles
- `[DECIDED]` The **Juggernaut** stays as the shared heavy walker.
- `[DECIDED]` Each faction gets a signature tank with its own trick. The Hammer (Soviet) comes first. The Guardian
  (Allies) comes later. The Empire tank is open.
- `[DECIDED]` `[LATER]` **Hammer** (our own variant): its leech steals an enemy tank's item when it lands the last
  hit or tick. Stolen items only last until the Hammer dies.
- `[DECIDED]` `[LATER]` Research memorable tanks from other games for more variety.

## 7. Combat feel
- `[DECIDED]` A per-weapon **feel kit**, kept as data: kick, muzzle flash, tracer, impact, hit marker, sound, target
  flinch, kill confirm. It is tuned live in a small **Feel Lab**, with the Imperial Warrior's SMG as the reference.
- `[DECIDED]` Split feedback: the shot feels instant; hit feedback arrives when the hit is confirmed.
- `[DECIDED]` Targets react: flinch and knockback.
- `[DECIDED]` Gears-style **active reload**: tap R at the right moment for a faster reload and a short damage boost.
- `[DECIDED]` Fix the AMK's rhythm (one shot, then a 3.5 s reload): a small magazine, the active reload, or both.

## 8. Winning and account progression
- `[DEFAULT]` **Points race.** Points come from uplink cashouts (the most), harvester deliveries, kills and structure
  kills. Destroying an HQ is a big points swing, not an elimination, so nobody sits out a match. First team to the
  target wins; otherwise the leader when the timer (about 20 min) ends.
- `[DECIDED]` After the match, the winning team keeps a share of the tech it earned as account progression; losing
  teams keep a smaller share. Nothing carries into the next match: everyone starts at 0.
- `[DEFAULT]` The account currency gets its own name (working name: **Service points**), so it can't be confused with
  in-match Tech.
- `[DECIDED]` Unlocks give breadth, never raw power: more classes, sidegrade variants, cosmetics.
- `[DECIDED]` Unlocking works like WARDOGS, through three gates:
  1. Your account level (the existing career rank) makes an item eligible.
  2. A one-time unlock fee in Service points makes it yours for good.
  3. In a match you buy it each life with match money, at a low price. Dying loses the copy, never the unlock.
  - In a match, the team's tech tier still decides when a unit or vehicle becomes buyable.
- `[DEFAULT]` Every role that counters something (the three T1 classes, the Javelin, the Recruit) is available from
  level 1. Unlocks add variants and specialists, so new players are never short of an answer.
- `[LATER]` The unlock catalogue itself. The prototype only records and shows Service points (KVStore).

## 9. The map: Octopon-3 (greybox first)
- `[DECIDED]` A three-team map inspired by Red Alert 3's Battlebase Octopon: a contested harvest field in the centre,
  verticality, and structures players can enter `[LATER]`.
- `[DEFAULT]` The layout:
  - Rotational symmetry at 120°, with each base on its own lobe.
  - A raised centre plateau with ramps, where the harvest field sits.
  - Uplink sites between each pair of bases, rotating, plus one in the centre.
  - A safe, low-yield field near each base.
  - Drop-downs from the high ground for fast rotations.
- `[DEFAULT]` **Greybox first:** plain blocks, with height read through brightness (higher is lighter). Played with
  bots before any art.
- `[DEFAULT]` Keep walkable gaps at least 6 m wide: the bots' 2 m grid can close gaps of 3 to 5.4 m (found in
  bot-poplar-wedge). Run the bot reachability check (every spawn to every goal) on this map.
- `[OPEN]` (technical) **Aiming at height.** Today the cursor aims at the ground. With ledges, aiming must ray-cast
  into the world, with gentle aim help onto bodies. Solve it in the map change-set if ledges are walkable.
- `[OPEN]` Map size. Start near Cabana's footprint and adjust with the scale test.

## 10. Bots (needed for the prototype)
- `[DEFAULT]` Bots must fight across three teams, buy T1 units, escort and hunt harvesters, and carry, defend and steal
  uplinks.
- `[DEFAULT]` Each team gets a simple role mix (for example 40 % centre, 30 % uplinks, 30 % harvesters), rebalanced
  when objectives change. This is the biggest prototype risk: keep it simple and observable.
- `[DEFAULT]` Fix the known replan storm in `navSteer` (PERF-2 in the bot-poplar-wedge report) before uplinks: moving
  objectives will make bots replan constantly.

## 11. Greybox prototype v0: scope and order (for Claude Code)
One change-set each, in this order, following CLAUDE.md's protocol (plan first, evidence, Brave check for visuals):
1. **three-teams:**
   - Three teams with factions as data; bots fight across all three; HUD and scoreboard for three.
   - $300 start, the Recruit, $100 T1 purchases at the terminal, spawning inside the barracks.
2. **octopon-3-greybox:** the §9 layout as map data, plus aiming at height if ledges are walkable.
3. **harvest-economy:** centre and safe fields, deliveries paying the team, harvester kills, team-funded rebuilds.
4. **bot-replan-storm:** the PERF-2 fix (§10), with sims before and after.
5. **uplinks:**
   - Cache, carry, place, countdown, steal, rotation.
   - Tech tiers, abilities locked until Tech 1, and the minute-5 floor.
6. **win-and-scoring:** the points race, a simple end-of-match screen, and Service points recorded.

Evidence for each: the gate; `tools/sim.js` with three teams, at least 3 runs per build; objective counters (cashouts,
steals, deliveries, harvester kills); renders; Mustafa's Brave checklist. Then run the scale test on this map
(N = 12, 24, 36, 48).
Not in v0: the Spy, the Hammer and the Guardian, the Javelin, the feel pass, the character rework, enterable
structures, multiplayer.

## 12. What this supersedes
- GAME_DESIGN.md §5 (core loop) and §6 (economy): replaced by §1–4 and §8 here.
- DESIGN_NOTES.md:
  - The death tax now respawns you as a **Recruit**, not a Conscript.
  - The Spy's hack target is now the enemy **harvester building**, not the terminal.
  - The Momentum question is largely answered: in-match progression is the team's tech tiers, which unlock abilities,
    units and vehicles.
- STATUS.md queue: the tutorial, ui-step-2, ui-step-3, build-spike and the ADR run are paused until the greybox loop
  is playable. The scale test moves to run on the Octopon-3 greybox.
