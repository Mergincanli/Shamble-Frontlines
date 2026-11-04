# LOOP V2: the core loop (living spec)

Written by Mustafa and the Advisor, October 2026. This replaces the loop in GAME_DESIGN.md §5–6 and parts of
DESIGN_NOTES.md (listed in §12). It is written for readers with no other context, including Claude Code.
Updated on 2026-11-04 with Mustafa's design walk: the lines dated 2026-11-04, and §13.

**Status tags.** `[DECIDED]` Mustafa decided it. `[DEFAULT]` the Advisor proposed it and it stands unless Mustafa
vetoes it. `[OPEN]` needs a decision before it gets built. `[LATER]` not in the greybox prototype. `[IDEA]` Mustafa's
idea, a sketch to iterate: not decided (tag added 2026-11-04).
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
- `[DECIDED]` Spawn protection: 3 s; firing ends it early. (Mustafa, 2026-11-04)
  - `[OPEN]` Still open (OQ-0005-AB14 in docs/adr/README.md): what it covers (damage only, or damage and every effect
    another team applies), and whether using an ability counts as firing.
- `[DEFAULT]` Spawn safety: enemies cannot enter a barracks interior.

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
| Tech 2 | T2 units (Officer, Javelin, each faction's infiltrator) and vehicles (Juggernaut; Hammer tank once built) |
| Tech 3 | T3 units and heavy options |

## 5. Units
- `[DECIDED]` Tier 1, $100 each:
  - **Conscript:** AMK rifle; ability Molotov.
    - `[DECIDED]` The Molotov in v2 is a real fire patch that denies an area, not a grenade. (Mustafa, 2026-11-04)
  - **Peacekeeper:** shotgun; ability Riot Shield.
  - **Imperial Warrior:** SMG, strong up close; ability Banzai blade (one-hit kill while active).
  - Their guns and the guns' first amp ladders: UNITS.md §0 (Mustafa, 2026-11-04).
- `[DECIDED]` Factions: the counter triangle and tier one stay shared; the differences grow with tech (tier-two
  specialists and vehicles are unique). (Mustafa, 2026-11-04; personalities and each faction's units: UNITS.md §0.)
  The counter triangle: Warrior beats Conscript beats Peacekeeper beats Warrior.
- `[DEFAULT]` A free fallback unit: the **Recruit**, armed with a Luger pistol. Death costs the unit you bought: you
  respawn as a Recruit and buy again, keeping your money.
- `[DECIDED]` **Javelin:** a rocket unit, strong against vehicles and aircraft, weak against infantry, with a Luger
  sidearm. `[DEFAULT]` Tier 2. Its rockets knock infantry back: little damage, big impact, so it still feels good.
- `[DEFAULT]` The **Officer** moves to Tier 2.
- `[DECIDED]` Shared or per faction: decided unit by unit when Mustafa writes the unit list, with two fixed rules:
  every faction has an anti-vehicle answer from tech 1; and a role every faction has (anti-vehicle, infiltration)
  shares one core with a faction twist on top, like the infiltrators. (Mustafa, 2026-11-04, B3-Q1 (c).) This covers
  the Javelin and the Officer, shared at tier two today, and the Juggernaut (§6); their lines stay as they are until
  the unit list decides them. Today the Javelin, the only listed answer to vehicles, is a Tech 2 unit (§4 and its line
  above): which unit meets the tech-1 rule is for the unit list.
- `[DECIDED]` `[LATER]` Infiltrators (tier two, every faction): one shared core, where entering an enemy building does
  the same for every faction (power plant: cuts their power; refinery: steals money; tech building: steals tech), plus
  a faction verb: the Allies' Spy disguises; the Empire's Shinobi is stealthy and fast, and sabotages; the Soviets' KGB
  agent interrogates a downed enemy to briefly reveal the enemy team on the map. (Mustafa, 2026-11-04; each faction's
  infiltrator: UNITS.md §0.)
  - `[DECIDED]` **The Spy** (the Allies' infiltrator), inspired by the Red Alert 3 and Team Fortress 2 spies. It can
    disguise itself as the enemy. It is built for real players, who create the memorable spy moments (bots make poor
    spy-checkers).
  - `[DECIDED]` The "tech building" is the HQ: no new building: the HQ holds the team's tech, so infiltrating the HQ
    steals tech; "cuts their power" works like a destroyed power plant for a set time. (Mustafa, 2026-11-04, B3-Q3
    (i).)
  - `[DECIDED]` The KGB agent's "downed enemy": a short window on a fresh body, no downed state; the reveal shows only
    that enemy's team. (Mustafa, 2026-11-04, B3-Q3 (ii).)
  - `[OPEN]` Mustafa's idea is that it costs tech. The Advisor recommends gating it behind Tech 3 and charging money,
    so one player can't spend the team's shared tech. (2026-11-04: infiltrators are tier two, so the Tech 3 gate no
    longer fits; the cost, tech or money, stays open.)
  - Also open: what breaks the disguise, what reveals the Shinobi's stealth, what its sabotage does, how long the hack
    takes, how much it steals, and how defenders are warned.

## 6. Vehicles
- `[DECIDED]` The **Juggernaut** stays as the shared heavy walker.
  - Shared or per faction: decided unit by unit in the unit list (§5; Mustafa, 2026-11-04, B3-Q1 (c)).
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
- `[DECIDED]` Gears-style **active reload**: tap R at the right moment. Active reload on every gun, gentle: a perfect
  reload is faster, with little or no damage bonus; a miss is a normal reload, no jam. Amps can widen the sweet spot.
  (Mustafa, 2026-11-04)
- `[DECIDED]` Fix the AMK's rhythm (one shot, then a 3.5 s reload): a small magazine, the active reload, or both.
- `[DECIDED]` Every gun gets a short sprint-to-fire delay, longer for heavy guns. (Mustafa, 2026-11-04)
- `[IDEA]` Weapon weight could set movement speed. (Mustafa, 2026-11-04)

## 8. Winning and account progression
- `[DEFAULT]` **Points race.** Points come from uplink cashouts (the most), harvester deliveries, kills and structure
  kills. Destroying an HQ is a big points swing, not an elimination, so nobody sits out a match. First team to the
  target wins; otherwise the leader when the timer (about 20 min) ends.
- `[DECIDED]` After the match, the winning team keeps a share of the tech it earned as account progression; losing
  teams keep a smaller share. Nothing carries into the next match: everyone starts at 0.
- `[DEFAULT]` The account currency gets its own name (working name: **Service points**), so it can't be confused with
  in-match Tech.
- `[DECIDED]` Account progression gives breadth only (class variants, cosmetics), never power. (Mustafa, 2026-11-04)
- `[DECIDED]` Account progression gives class variants and cosmetics only. New classes come through the team's
  tech in the match, so nobody is ever missing a counter. (Mustafa, 2026-11-04, B3-Q2 (a).)
- `[DECIDED]` Unlocking works like WARDOGS, through three gates:
  1. Your account level (the existing career rank) makes an item eligible.
  2. A one-time unlock fee in Service points makes it yours for good.
  3. In a match you buy it each life with match money, at a low price. Dying loses the copy, never the unlock.
  - In a match, the team's tech tier still decides when a unit or vehicle becomes buyable.
- `[DEFAULT]` Every role that counters something (the three T1 classes, the Javelin, the Recruit) is available from
  level 1. Unlocks add variants, so new players are never short of an answer.
- `[LATER]` The unlock catalogue itself. The prototype only records and shows Service points (KVStore).
- `[OPEN]` The war around the matches, in its own design session. (Mustafa, 2026-11-04)

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
- `[DECIDED]` Bots can loot weapons and pick amps. (Mustafa, 2026-11-04; §13)
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
  - The Spy's hack target is no longer the terminal: since 2026-11-04, every faction's infiltrator enters an enemy
    power plant, refinery or tech building (the HQ) (§5).
  - The Momentum question is largely answered: in-match progression is the team's tech tiers, which unlock abilities,
    units and vehicles, and each player's weapon amps (§13).
- STATUS.md queue: the tutorial, ui-step-2, ui-step-3, build-spike and the ADR run are paused until the greybox loop
  is playable. The scale test moves to run on the Octopon-3 greybox.
- Mustafa's design walk of 2026-11-04 (the lines dated 2026-11-04, and §13):
  - Inside this file it replaced, word for word: in §2, "`[DEFAULT]` Spawn safety: 3 s of spawn protection, and
    enemies cannot enter a barracks interior."; in §4, "T3 units (the Spy)"; in §5, "`[DEFAULT]` All three are shared
    by every faction, because the counter triangle needs them (Warrior beats Conscript beats Peacekeeper beats
    Warrior). Factions differ through uniforms and signature vehicles.", and in the Spy line "(Tier 2 or 3)" and ", and
    hack an enemy harvester building (refinery) to steal that team's tech"; in §7, "for a faster reload and a short
    damage boost"; in §8, "`[DECIDED]` Unlocks give breadth, never raw power: more classes, sidegrade variants,
    cosmetics." and, in the role line, " and specialists"; in §12, "The Spy's hack target is now the enemy **harvester
    building**, not the terminal."
  - Outside this file, pointers only (the drafts take them up at their next revision; no ADR is edited here):
    UPGRADE-SCOPE's "Every unit and every weapon has its own roguelike upgrade track" (STATUS.md; ADR 0005's settled
    inputs and AB-4; ADR 0001 part 12's "per unit type") now holds for weapons only (§13). ADR 0005's Molotov row ("an
    area blast", no fire area) meets the fire patch (§5). ADR 0005 AB-3's "active reload's short damage boost" meets
    "little or no damage bonus" (§7). ADR 0005's OQ-0005-AB4 and AB5 (lines 1263 and 1278-1283) quote §8's "sidegrade
    variants", and AB5's options weigh unlocking classes ("the catalogue grows in classes", "as unlocking a class");
    account progression now gives class variants and cosmetics only (§8). ADR 0004 part 17's spawn protection
    (`[DEFAULT]`, line 502) and ADR 0005's spawn protection row (line 991) meet §2's `[DECIDED]` rule; ADR 0004 part
    18's "one track per unit type" (line 518) meets §13's weapons only; ADR 0005's LOOP_V2 inputs (lines 796-797: the
    counter triangle `[DEFAULT]`, the Spy alone) and its Spy row (line 994) meet §5's shared triangle and tier one and
    its three infiltrators. The register (docs/adr/README.md) points each open question his words touch.

## 13. Weapons in the match: amps, loot and ammo
Added 2026-11-04. Mustafa's rules for the weapons a player holds in a match, in his words: his frame of 2026-10-31
(UPGRADE-SCOPE) and his answers of 2026-11-03, as STATUS.md records them word for word, and his design walk of
2026-11-04. Each line carries its date. The guns and their first amp ladders: UNITS.md §0.

- `[DECIDED]` Amps (amplifications) are on weapons only. No class perks and no learn-by-doing layer. (2026-11-04)
  This narrows the frame of 2026-10-31, "Every unit and every weapon has its own roguelike upgrade track (pick one of
  three per level), owned by the player.", to weapons.
  - Name: the in-match picks are "amplifications" ("amps"). (2026-11-03)
  - `[DEFAULT]` The Advisor's reading (2026-11-04): "no learn-by-doing layer" rules out a separate layer that grows a
    class or a player from use; guns still level from XP earned while using them (the levelling line below), as the
    same walk says "when a gun levels up". Mustafa may veto it.
- `[DECIDED]` Tracks survive death and switching (switch back and the track is still there). They reset at the start of
  every match. (2026-10-31)
- `[DECIDED]` Progress belongs to the weapon type, not the physical gun: when a gun is lost, its track waits for the
  next gun of that type. (2026-10-31)
- `[DECIDED]` Rejoining keeps a player's amp tracks; bots can loot weapons and pick amps. (2026-11-04)
- `[DECIDED]` Levelling: XP from kills, assists and objective work while using that unit or weapon; fast early levels;
  a track that's behind the player's best levels faster. (2026-11-03; amps are on weapons only since 2026-11-04, so
  only the weapon half applies.)
- `[DECIDED]` Picking an amp: when a gun levels up, three cards wait at the screen's edge; pick with Alt+1/2/3.
  Rebindable, and tested early in Brave and Firefox, because Alt can steal browser focus or switch tabs; fallback:
  hold B + 1/2/3. (2026-11-04)
  - What fills the other cards when a ladder level names fewer than three amps (UNITS.md §0) is OQ-0005-AB8's
    question in docs/adr/README.md: whether the candidates always differ, and what a track with fewer candidates than
    the offer's size does.
- `[DECIDED]` Upgrade visibility: enemies can see a player's build. (2026-11-03)
- `[DECIDED]` A class always comes with its own weapon: no weapon choice when buying. (2026-10-31) How an
  account-unlocked weapon then reaches a match (§8's third gate) is open: OQ-0005-AB5 in docs/adr/README.md.
- `[DECIDED]` Other weapons come only from killed enemies' drops; dropped weapons are world pickups. (2026-10-31)
- `[DECIDED]` A looted weapon replaces the main (class) weapon; the sidearm stays as backup. (2026-11-03)
- `[DECIDED]` Looting keeps the class ability (a Conscript with a looted shotgun still throws Molotovs). Rare strong
  combos are welcome; watch for any combo that becomes a must-have. (2026-11-04; it confirms the 2026-11-03 "Assumed,
  to confirm: the class's own abilities stay.")
- `[DECIDED]` Ammo: limited but generous, refilled at the barracks and at supply points (keeps an engineer or supply
  role possible later). (2026-11-04)
