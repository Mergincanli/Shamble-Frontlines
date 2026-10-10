# Renegade X — Full Arsenal Reference

> **Purpose:** complete unit/vehicle/weapon/equipment roster for **Renegade X** (the free standalone remake of C&C Renegade, by Totem Arts) — the single closest existing model to Shamble Frontlines (base-assault FPS: buy classes/vehicles at a terminal, power-plant pricing, destroy-the-HQ win). Companion to `UNITS.md`.
>
> **On damage numbers:** exact per-shot damage in Ren X is **patch-versioned** (balance patches routinely nudge damage / rate-of-fire / reload). Hard numbers would go stale and mislead tuning, so this file captures the **stable** design data — cost, weapon type, role, relative power, tier — and points to the live wiki for current exact values. Sources: wiki.ren-x.com / wiki.totemarts.games (official), cncnz.com, Totem Arts forums, verified Jul 2026. Costs are default (they rise when your **Refinery/Power Plant** is down — the Renegade pricing lever Shamble already has).

---

## The economy & tier model (the part that maps onto Shamble)

- **Currency = Credits.** Passive trickle from your **Refinery** (fed by an auto **Harvester** convoy) + points for damage/kills. One pool, personal.
- **Everyone starts free.** You spawn as a basic Soldier and either switch to another **free** class or **buy up** to advanced infantry ($150–$1000) or vehicles ($300–$1500) at any Purchase Terminal.
- **Death = lose the unit, keep the credits.** Die and you respawn at the free tier; your bought unit is gone and must be re-purchased. (This is exactly Shamble's chosen death model.)
- **Respawn timer grows** the longer the match runs.
- **Mystery Crates** can swap your current unit for another for free — including, rarely, a **Spy** (an enemy-faction unit you can walk into their base with). Note for the Shamble Spy idea.
- **No account-level unlocks, no in-match roguelike.** Ren X progression is *purely* the economy tier. It validates Shamble's Identity + economy spine but is silent on Momentum/Mastery — those are Shamble's additions.
- **⚠️ Design note for Shamble:** Ren X tiers are *partly raw damage* — a $1000 Personal Ion Cannon effectively one-shots most things. That's precisely the "power = bigger number" model you said you want to avoid. Treat Ren X as the **economy/terminal** reference, not the **tier-power** reference — lean more on sophistication than Ren X does.

---

## GDI infantry (specializes in brute strength)

| Unit | Cost | Primary weapon | Sidearm / kit | Role / tier |
|---|---|---|---|---|
| **Soldier** | Free | Automatic rifle | Pistol, Timed C4 ×1 | Basic all-rounder |
| **Shotgunner** | Free | Combat shotgun | Pistol, Timed C4 ×1 | Close-quarters |
| **Grenadier** | Free | Grenade launcher (arcing) | Pistol, Timed C4 ×1 | Indirect / anti-group |
| **Marksman** | Free | Semi-auto marksman rifle | Pistol, Timed C4 ×1 | Basic ranged/anti-infantry |
| **Engineer** | Free | Pistol | Repair gun, Timed C4 ×1, mines | Repair/capture; disarms C4 & beacons |
| **Officer** | 175 | Chaingun / minigun | Pistol, Timed C4 ×1 | Anti-infantry, shreds light vehicles/air |
| **McFarland** | 150 | Flamethrower | Pistol, Timed C4 ×1 | Anti-infantry / anti-garrison |
| **Rocket Soldier** | 225 | Rocket launcher (lock-on) | Pistol, Timed C4 ×1 | Anti-vehicle **and** anti-air |
| **Gunner** | 400 | Rapid rocket launcher | Pistol, Timed C4 ×1 | Advanced anti-vehicle (fast rockets) |
| **Patch** | 450 | Rapid Tiberium auto-rifle ("staple gun") | Pistol, Timed C4 ×1 | High-RoF anti-infantry |
| **Deadeye** | 500 | Sniper rifle | Pistol, Timed C4 ×1 | Marksman (mid sniper) |
| **Hotwire** | 350 | Repair gun | Pistol, Timed C4 ×2, Remote C4, Proximity mines | **Advanced engineer** — solo-demo a building |
| **Havoc** *(hero)* | 1000 | Ramjet sniper rifle (pierces armour) | Pistol, Timed C4 | Tier-3 sniper |
| **Sydney** *(hero)* | 1000 | **Personal Ion Cannon** (1 shot/clip, huge) | Pistol, Timed C4 | Anti-everything nuke-gun |
| **Mobius** *(hero)* | 1000 | Volt auto-rifle | Pistol, Timed C4 | Anti-armour, strong at short–mid |
| *Armoured Sydney* | rare crate | Personal Ion Cannon + armour suit | — | Tankier Sydney variant |

## Nod infantry (specializes in stealth & Tiberium)

| Unit | Cost | Primary weapon | Sidearm / kit | Role / tier |
|---|---|---|---|---|
| **Soldier** | Free | Automatic rifle | Pistol, Timed C4 ×1 | Basic all-rounder |
| **Shotgunner** | Free | Combat shotgun | Pistol, Timed C4 ×1 | Close-quarters |
| **Flame Trooper** | Free | Flamethrower | Pistol, Timed C4 ×1 | Anti-infantry / anti-garrison |
| **Marksman** | Free | Semi-auto marksman rifle | Pistol, Timed C4 ×1 | Basic ranged/anti-infantry |
| **Engineer** | Free | Pistol | Repair gun, Timed C4 ×1, mines | Repair/capture; disarms C4 & beacons |
| **Officer** | 175 | Chaingun | Pistol, Timed C4 ×1 | Anti-infantry, shreds light vehicles/air |
| **Chemical Trooper** | 150 | Tiberium chem sprayer | Pistol, Timed C4 ×1 | Anti-infantry; **immune to Tiberium fields** |
| **Rocket Soldier** | 225 | Rocket launcher (lock-on) | Pistol, Timed C4 ×1 | Anti-vehicle **and** anti-air |
| **Stealth Black Hand** | 400 | Laser rifle | **Cloak**, Pistol, Timed C4 | Stealth infiltrator / saboteur |
| **Laser Chaingunner** | 450 | Laser chaingun | Pistol, Timed C4 ×1 | Long-range anti-armour |
| **Black Hand Sniper** | 500 | Sniper rifle | Pistol, Timed C4 ×1 | Marksman (mid sniper) |
| **Technician** | 350 | Repair gun | Pistol, Timed C4 ×2, Remote C4 ×3, Proximity mines ×5 | **Advanced engineer** — solo-demo a building |
| **Sakura** *(hero)* | 1000 | Ramjet sniper rifle (pierces armour) | Pistol, Timed C4 | Tier-3 sniper |
| **Raveshaw** *(hero)* | 1000 | **Railgun** (pinpoint, fast reload vs PIC) | AT mines, EMP grenade, Timed C4 | Anti-armour specialist |
| **Mendoza** *(hero)* | 1000 | Volt auto-rifle | Pistol, Timed C4 | Anti-armour, strong at short–mid |
| *Mutant Raveshaw* | rare crate | Railgun + Tiberium mutation | — | Tankier Raveshaw variant |

---

## Equipment & consumables

| Item | Who | Use |
|---|---|---|
| **Timed C4** | all (×1; adv. engineers ×2) | Sticks and detonates on a timer — building demo (stack on the MCT) |
| **Remote C4** | advanced engineers | Detonate on command — building demo |
| **Proximity mines** | advanced engineers (team-capped count) | Area denial / base defence |
| **Repair gun** | engineers, Hotwire, Technician | Repair vehicles/structures/infantry; **disarms enemy C4 & beacons** |
| **Ion Cannon Beacon** (GDI) | any, 1000 | Superweapon — plant on an enemy structure (or their pedestal for instant win); must be defended while it counts down |
| **Nuclear Strike Beacon** (Nod) | any, 1000 | Nod equivalent of the Ion Cannon Beacon |
| **A-10 Airstrike** (GDI) | any, ~700–800 | Called strafing run |
| **AC-130 Airstrike** (Nod) | any, ~700–800 | Called gunship run |

> The **beacon** mechanic (plant a superweapon on the enemy base, they must find & disarm it) is a strong base-assault twist worth noting for Shamble — a high-stakes, defendable objective, cousin to the Spy's tech-tap.

---

## GDI vehicles (bought at the War Factory)

| Vehicle | Cost | Seats | Weapon(s) | Role |
|---|---|---|---|---|
| **Humvee** | 350 | 2 | Machine gun | Recon / anti-infantry (armour ~300) |
| **APC** | 500 | 6 | Machine gun | Troop transport (engineer rushes); heavy armour, survives an Obelisk hit |
| **Mobile Rocket Launcher System (MRLS)** | 450 | 2 | 6 rockets/burst | Siege / anti-building & vehicle; light armour |
| **Medium Tank** | 800 | 2 | Cannon | Main battle tank; good armour+speed balance |
| **Mammoth Tank** | 1500 | 2 | Dual cannon + 4 Tusk heat-seeking missiles | Heavy assault; **self-heals armour**; very slow/large |
| **Harvester** | auto | — | (unarmed) | Collects Tiberium → Refinery income |

## Nod vehicles (bought at the Airstrip)

| Vehicle | Cost | Seats | Weapon(s) | Role |
|---|---|---|---|---|
| **Buggy** | 300 | 2 | Machine gun | Cheap fast recon / anti-infantry |
| **APC** | 500 | 6 | Machine gun | Troop transport |
| **Mobile Artillery** | 450 | 2 | Long-range arcing shells | Siege; slow, lightly armoured, big punch |
| **Light Tank** | 600 | 2 | Cannon | Fast MBT (≈300 health + 300 armour); loses a straight fight to a Medium, wins on speed |
| **Flame Tank** | 800 | 2 | Dual flamethrowers | Heavy-armour anti-building / anti-tank brawler; slow |
| **Stealth Tank** | 900 | 1 | Homing missiles + **Cloak** | Ambush / anti-vehicle; can run over unaware infantry |
| **Harvester** | auto | — | (unarmed) | Collects Tiberium → Refinery income |

---

## Aircraft (only on air-enabled maps; all are helicopters — hover/ascend/descend)

| Aircraft | Faction | Cost | Weapon(s) | Role |
|---|---|---|---|---|
| **Orca Fighter (VTOL)** | GDI | 900 | Armour-piercing chaingun + missiles | Air superiority / base-assault / anti-infantry |
| **Apache** | Nod | 900 | Chaingun + missiles | Fast raids / base-assault / anti-infantry |
| **Transport Helicopter (Chinook)** | both | 700 | Mounted MGs (passengers fire) | Troop insertion (drop engineers on roofs, etc.) |

> Ren X aircraft are **helicopters**, not planes — they hover and hold position (matches an overhead-friendly flight model, relevant when Shamble reaches aircraft in Phase 5.3).

---

## The one-line takeaways for Shamble

1. **Economy tier = progression, and death = lose-the-unit-keep-the-credits.** Ren X is living proof the model works — it's the whole game. Steal the spine wholesale (you already have).
2. **Two production buildings gate two arms:** Barracks/Hand-of-Nod → advanced infantry; War-Factory/Airstrip → vehicles. Killing one removes that arm. Clean economy-as-weapon design.
3. **Stealth infiltrator + spy-crate + plant-a-beacon** are Ren X's "sneak into the enemy base and do damage" verbs — direct precedent for the Shamble Spy + tech-tap.
4. **But Ren X leans on raw-damage tiers** (the $1000 PIC one-shots) — the thing to *avoid*. Take the economy shape, not the power curve.
