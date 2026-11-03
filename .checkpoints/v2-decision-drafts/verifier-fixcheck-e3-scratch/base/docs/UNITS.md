# Shamble Frontlines — Phase 5 Unit & Weapon Reference

Source rosters for rebuilding the arsenal from scratch. Order of build: **infantry + their guns first**, then ground vehicles, then aircraft. Each entry notes the **weapon/role** so it can feed a gun directly. Verified against the C&C wikis (cnc.fandom, cncnz, strategywiki) and Gears references, Jun 2026.

> **Status (migration):** The **faction lens is decided** — three RA3-style factions (Soviet / Allies / Empire) exist as a **visual identity layer** (uniform palettes via a material-role system; see GAME_DESIGN §7.5 + TECHNICAL §5). The **art direction is locked** — "Living Comic" (see `ART_DIRECTION.md`). The **implemented roster** is the four classes in GAME_DESIGN §7 (Conscript, Peacekeeper, Imperial Warrior, Officer), the **Empire elite** enemy and the **Juggernaut** vehicle — all faction-dressed on the gameplay spawns (`buildSoldier` + `SOLDIER_KIT` / `ELITE_KIT`). This file remains the **planned** expansion source.
>
> **Empire elite — the "villain" (IMPLEMENTED):** a separate spawnable elite enemy with the Wolfenstein-menace material — near-black lacquered armour, a single specular glint band, a **purple** silhouette outline (`0x7B2FD6`, so it reads against the dark armour) and a glowing **violet** visor (`0xc24bff`); 200 HP. Its weapon/role can still grow from the Empire archetypes below. See `ART_DIRECTION.md` §7.

---

## 1) Red Alert 3 (2008) + Uprising — infantry

Each faction has a dog/scout, a basic rifleman, an anti-armor trooper, an anti-air or AT trooper, an engineer, and a hero **commando** (Tanya / Natasha / Yuriko).

### Allied
| Unit | Weapon / role |
|---|---|
| Attack Dog | melee, one-shot vs infantry; sonar paralyze |
| **Peacekeeper** | riot-shield + heavy **shotgun** (shield blocks frontal fire) |
| **Javelin Soldier** | lock-on **rocket** launcher, anti-tank **and** anti-air |
| Allied Engineer | unarmed; capture/repair; deploys a heal tent |
| Spy | disguise, bribe, infiltrate |
| **Tanya Adams** (commando) | dual pistols, C4 on tanks/buildings, time-belt dodge |
| Cryo Legionnaire *(Uprising)* | freeze beam that shatters frozen targets |

### Soviet
| Unit | Weapon / role |
|---|---|
| War Bear | melee + paralyzing roar; scout |
| **Conscript** | cheap rifle + **Molotov** cocktail (anti-garrison) |
| **Flak Trooper** | flak gun, anti-air + anti-armor |
| Combat Engineer | pistol; capture/repair; builds 5-man bunker |
| **Tesla Trooper** | electric melee walker-armor, EMP, too big to crush |
| **Natasha Volkova** (commando) | piercing **sniper** (line kill), calls airstrike, snipes pilots |

### Empire of the Rising Sun
| Unit | Weapon / role |
|---|---|
| Burst Drone | flying scout, latches/EMPs vehicles, self-destruct |
| **Imperial Warrior** | kinetic carbine **+ beam-katana Banzai charge** (one-hit melee) |
| **Tankbuster** | wave-force anti-armor beam; **burrows** to tank fire; no AA |
| Imperial Engineer | sprint; capture/repair |
| **Shinobi** | stealth assassin, sword, smoke bomb, walks on water |
| **Rocket Angel** | flying; photon rockets + **paralysis whip** (freezes a unit) |
| **Yuriko Omega** (commando) | telekinesis — crushes infantry/vehicles psychically |
| Archer Maiden *(Uprising)* | laser longbow, anti-air, splitting arrows vs infantry |

---

## 2) Kane's Wrath (2008) — infantry by faction + subfaction

### GDI — core
Rifleman Squad · Missile Squad (AT/AA) · **Grenadier Squad** · **Sniper Team** (sniper + spotter, marks Juggernaut targets) · Engineer · **Zone Trooper** (power-armor railgun, AT, Tib-immune, drop-pods) · **Zone Raider** *(KW; female jump-jet, dual sonic, can hit air)* · **Commando** (burst rail-carbine + C4, jetpack).
- **Steel Talons** (mech focus): swaps Engineer → **Combat Engineer**; keeps Riflemen/Missile/Zone Troopers; identity is in walkers (Titan, Wolverine, Mammoth Mk II), not infantry.
- **ZOCOM** (sonic/Red-Zone): **Zone Raiders** are the signature infantry; **sonic** weapons throughout; infantry get **Tiberium field suits**; no railguns/Juggernaut.

### Nod — core
**Militant Squad** (cheap chainguns) · **Militant Rocket Squad** (AT/AA) · **Fanatics** (suicide explosives) · **Black Hand** (elite flamethrower troopers, clear garrisons) · **Shadow Team** (stealth, fusion rifle + demo charges, glide) · **Saboteur** (capture, hijack downed mechs) · **Commando** (twin lasers + satchels).
- **Black Hand** (infantry subfaction): **Confessor Cabal** replaces Militants as backbone (machine guns → particle beams, morale aura, psycho grenades) · Black Hand Troopers (flame) · Fanatics · **Commando** (not stealthed, trained Heroic). Flame-heavy.
- **Marked of Kane** (cyborgs): **Awakened** replace Militants (arm cannon + **EMP**, toughest tier-1 armor) · **Enlightened** (heavier cyborgs) · **Tiberium Troopers** replace Black Hand (liquid-Tiberium sprayer + Tib resistance). Expensive.

### Scrin — core
**Buzzers** (swarm of flying blades, shred infantry) · **Disintegrators** (anti-armor beam, no AA) · **Assimilator** (capture/repair) · **Shock Trooper** (plasma-disc launcher, AT/AA, blink) · **Mastermind** (mind-control, no normal attack) · **Ravager** *(KW; Tiberium shard launcher, anti-infantry)*.
- **Reaper-17** (heavy/shock): **Ravagers** prominent; tougher ground units + shields.
- **Traveler-59** (speed/teleport/psionic): **Cultists** (mass mind-control, no build limit, no normal attack) · **Prodigy** (upgraded Mastermind — AoE mind-control + personal blink).

---

## 3) Renegade (2002) + Renegade X — infantry classes

This is the closest model to **our** game: a base-assault FPS where you **buy character classes at a terminal**, prices rise when your **power plant** dies (we already have that!). Renegade X is the free modern remake — same class structure, better feel/balance, good to reference for moment-to-moment gunplay.

### GDI classes
| Class | Weapon | Tier |
|---|---|---|
| Soldier | assault rifle | free |
| Shotgun Trooper | combat shotgun | free |
| Grenadier | grenade launcher (indirect) | free |
| Engineer | repair gun + timed/remote C4 | free |
| Officer | chaingun | cheap |
| Rocket Soldier Officer | rocket launcher (AT) | cheap |
| Deadeye / Marksman | sniper rifle | mid |
| Gunner | rapid rockets | mid |
| Patch | "staple gun" rapid Tiberium | mid |
| **Hotwire** | adv. engineer — 2× C4, **landmines**, fast repair | mid |
| Sydney Mobius | Tiberium auto-rifle (+ smoke) | mid |
| Mobius | Volt auto-rifle (anti-armor) | mid |
| **Sydney (PIC)** | **Personal Ion Cannon** — 1 shot/clip, huge | high |
| **Havoc** (commando) | ramjet rifle + C4 | high |

### Nod classes
| Class | Weapon | Tier |
|---|---|---|
| Soldier / Shotgun / Grenadier / Technician | same basics (Technician = engineer) | free |
| Officer | chaingun | cheap |
| Rocket Soldier Officer | rocket launcher | cheap |
| **Chem Warrior** | Tiberium **chem sprayer** (immune to Tib fields) | mid |
| **Laser Chaingunner** | laser chaingun (long-range anti-armor) | mid |
| **Black Hand Stealth Soldier** | laser rifle + **cloak** | mid |
| Black Hand Sniper | sniper rifle | mid |
| Mendoza | Volt auto-rifle | high |
| Sakura | ramjet sniper rifle | high |
| **Raveshaw** | **Railgun** (pinpoint, fast reload vs PIC) | high |
*(Renegade X also adds a dedicated **Flame Trooper** to Nod.)*

---

## 4) Gears of War 5 — weapon *feel* to steal

GoW's signature is **weight + a mechanic per gun**, not realism. The two systems worth lifting:
- **Active Reload** — a timed bar on reload; nail the sweet spot for a faster reload + briefly boosted damage, miss it and you jam. Turns reloading into a skill beat.
- **Chainsaw / bayonet melee** built onto rifles (Lancer chainsaw, Retro Lancer bayonet charge) — melee is *part of* the gun, not a separate slot.

Iconic guns & their hook:
| Weapon | Hook |
|---|---|
| **Lancer** | AR with a **chainsaw bayonet** (close-range execute) |
| **Retro Lancer** | heavier AR + **bayonet charge** (sprint impale) |
| **Gnasher** | the duel **shotgun** — whole close game revolves around it |
| **Hammerburst** | semi-auto **burst** marksman rifle, manual fire = accuracy |
| **Longshot** | bolt-action sniper, one-shot headshot |
| **Embar** | **charge** railgun-marksman (hold to power up the shot) |
| **Torque Bow** | charge an arrow that **sticks then explodes** |
| **Boomshot** | grenade launcher (arcing, AoE) |
| **Dropshot** | fires a hovering **drill** that drops & detonates |
| **Buzzkill** | launches **ricocheting saw blades** off walls |
| **Scorcher** | flamethrower |
| **Mulcher / Tri-Shot** | heavy chainguns (overheat, mounted feel) |
| **Boltok** | hand-cannon revolver (slow, hits hard) |

---

## 5) Synthesis → our gun/unit archetypes

Across all four games the same **infantry archetypes** recur. These should become our era-gated classes/weapons:

1. **Rifleman / basic** — Conscript, Imperial Warrior, Rifleman, GDI Soldier → AR (Lancer feel, active reload, melee built in).
2. **Anti-armor** — Javelin, Tankbuster, Missile Squad, Rocket Officer, Mobius → rockets / charge-beam (Torque Bow / Embar feel).
3. **Anti-air** — Flak Trooper, Archer Maiden, Missile Squad → flak / lock-on.
4. **Shotgun / breach** — Peacekeeper (+ shield!), Shotgun Trooper, Gnasher → close shotgun; pairs with our existing **Round Shield**.
5. **Flame / chem** — Black Hand, Chem Warrior, Scorcher → flamethrower / sprayer (anti-garrison, DoT).
6. **Stealth** — Shinobi, Shadow Team, Stealth Black Hand → cloak + burst.
7. **Sniper / marksman** — Deadeye, Black Hand Sniper, Sniper Team, Natasha, Longshot, Hammerburst → one-shot precision / charge rail (Raveshaw/PIC/Embar).
8. **Heavy-armor shock** — Tesla Trooper, Zone Trooper, Shock Trooper → slow, tanky, EMP/plasma.
9. **Engineer** — every faction → repair/capture + C4 + mines (Hotwire). Ties into our base-assault economy.
10. **Commando / hero** — Tanya, Natasha, Yuriko, Havoc, Raveshaw → premium single-purchase, ramjet/PIC/railgun, building-deleting C4.
11. **Exotic / mind-control / swarm** — Yuriko, Mastermind, Cultists, Buzzers, Dropshot, Buzzkill → the "weird tier" for flavor.

**Era mapping idea (4 tiers):** T1 basics (rifle/shotgun/engineer) → T2 specialists (AT, flame, sniper) → T3 shock/stealth (Tesla/Zone, cloak) → T4 commandos + exotics (PIC/railgun/mind-control). Slots map onto the existing `WEAPONS` array with `era` + `minLevel` gates already in place.

> Next step: the wardrobe is already on the gameplay spawns (done). Infantry guns go **tier by tier** (T1 basics → T4 commandos/exotics), replacing the placeholder AMK/Bagpipe/Shabzon/etc. — **held until the progression design is decided** (see ROADMAP). Slots map onto the existing `WEAPONS` array with `era` + `minLevel` gates.
