# Legacy tuning: every number, as data

**Purpose.** R7 of the v2 phase 0 brief (`docs/V2_PHASE0_BRIEF.md` §4): "Extract the tuning numbers into data tables."
These are the legacy game's starting values for v2's content tables (LOOP_V2: every number in one data table). They are
what the legacy build does, not decisions for v2: LOOP_V2 replaces several of these rules (see `docs/LEGACY_INVENTORY.md`).

**Source.** `docs/legacy-tuning.json` is authoritative; this page renders it. Main at 48ffb51 (the engine files are
unchanged on main since). Produced by `tools/tuning-dump.js`, run through the real build:
`node tools/preview.js . eval tools/tuning-dump.js --pre=tools/tuning-dump.js [--map=tri]` (Linux: prefix `xvfb-run -a`).

**How to read it.**
- "Default" is the value after the engine scripts load and before `bootGame()`. Boot changes some values per map; they
  are listed at the end, per map.
- 17 dumped values were checked against their literal in the source: all equal (the R7 report lists them).
- Two runs per map gave byte-identical output.
- The look group is data only. The v2 look is not decided (Mustafa and the Advisor write the look brief).

## Contents

- **Gameplay: movement, combat, units, vehicles, teams, rules and economy:** `T`, `TUNE`, `TEAMS`, `WEAPONS`, `UNIT_TYPES`, `VEHICLE_TYPES`, `SOLDIER_KIT`, `ELITE_KIT`, `RIFLE_TIERS`, `RULES_DEFAULT`, `ECO`, `SILO`, `TBOX`, `HARV`, `JUG`, `JUG_PILOT`, `JUG_CANNON`, `T_CHARGE`, `STAM`, `FIRE_H`, `EJUMP_REACH`, `AIM_ARC_N`, `GROUND_W`, `GRID_SIZE`
- **Simulation: tick, input bits, hit bodies:** `FIXED_DT`, `BTN`, `BODY`, `PART`
- **Progression and presentation: ranks, prestige, words, camera styles, UI timings:** `RANK_TIERS`, `PRESTIGE`, `ROMAN`, `GUN_WORDS`, `KILL_WORDS`, `CAM_STYLES`, `VIG_W`, `WR_DEPLOY_MS`, `RR_C`, `OCC_FADE`, `SFX`
- **Look and builders (look brief pending: listed as data only, never a decision):** `ENV_DEFAULT`, `SUN_R`, `INK`, `WORLD_OUTLINE`, `CHAR_OUTLINE`, `RIM_COL`, `OUTLINE_GROUND_Y`, `TEAM_COL`, `TEAM_HEX`, `FACTION_PALETTES`, `CHAR_DEFAULT`, `CHAR_MALE`, `CHAR_FEMALE`, `CPARTS`, `GUN_KIT`, `BK`, `MELEE_IDLE`, `ANIM`, `ARM_L1`
- **Runtime state, not tuning (kept in the JSON for completeness):** `NAV`, `AI_STATE`
- **Boot changes per map**
- **Left out** (functions and objects the dump does not serialise)

## Gameplay: movement, combat, units, vehicles, teams, rules and economy

| Constant | Value | Where |
|---|---|---|
| `FIRE_H` | 1.9 | shamble-game.js:1419 |
| `EJUMP_REACH` | 3.0708333333333337 | shamble-game.js:1575 |
| `AIM_ARC_N` | 48 | shamble-game.js:1701 |
| `GROUND_W` | 372 | shamble-core.js:224 |
| `GRID_SIZE` | 10 | shamble-core.js:850 |

### `T` (shamble-core.js:66)

| Key | Value |
|---|---|
| `accel` | 14 |
| `adsCam` | "pitch" |
| `adsMoveMult` | 0.55 |
| `arenaX` | 186 |
| `arenaZ` | 126 |
| `barHealth` | 1000 |
| `bodyMult` | 1 |
| `camBack` | 56.6 |
| `camDeadX` | 0.6 |
| `camDeadY` | 0.52 |
| `camHeight` | 40 |
| `camKickDamping` | 12 |
| `camKickImpulse` | 24 |
| `camKickRange` | 34 |
| `camKickStiffness` | 110 |
| `camLerp` | 7 |
| `camLookLean` | 0.55 |
| `camPosLean` | 0.18 |
| `camPushLerp` | 9 |
| `camYawBase` | 0.7853981633974483 |
| `camYawLerp` | 14 |
| `camYawSens` | 0.006 |
| `concussionStrength` | 0.85 |
| `dustLife` | 1.05 |
| `eliteCount` | 1 |
| `eliteHealth` | 200 |
| `enemyBurst` | 1 |
| `enemyBurstPause` | 4.5 |
| `enemyCount` | 6 |
| `enemyDamage` | 34 |
| `enemyDetect` | 44 |
| `enemyFireInterval` | 0.22 |
| `enemyHealth` | 100 |
| `enemyMoveSpeed` | 4.2 |
| `enemyPreferRange` | 22 |
| `enemyRange` | 60 |
| `enemyRespawn` | 4 |
| `enemySiegeDamage` | 11 |
| `enemySpread` | 0.11 |
| `fov` | 15 |
| `fovADS` | 15 |
| `foxPanMax` | 16 |
| `fragChunks` | 14 |
| `fragCount` | 60 |
| `fragFriction` | 0.66 |
| `fragGravity` | 46 |
| `fragLaunchAngle` | 0.42 |
| `fragLife` | 1.05 |
| `fragRestitution` | 0.42 |
| `fragSpeed` | 64 |
| `gravity` | 24 |
| `grenadeBuildingDamage` | 240 |
| `grenadeDamage` | 140 |
| `grenadeMax` | 3 |
| `grenadeRadius` | 6.5 |
| `headMult` | 2.6 |
| `hqHealth` | 1600 |
| `incomeBase` | 1.5 |
| `incomeRefinery` | 9 |
| `jumpV` | 11 |
| `killReward` | 25 |
| `legMult` | 0.65 |
| `lookAheadBase` | 5.5 |
| `lookAheadFloor` | 3.5 |
| `lookAheadHip` | 8 |
| `lookBackRange` | 16 |
| `lookFwdGain` | 0.55 |
| `lookFwdRange` | 26 |
| `lookLerp` | 9 |
| `lookSideGain` | 0.55 |
| `lookSideMax` | 6 |
| `maxLean` | 26 |
| `meleeCooldown` | 0.55 |
| `meleeDamage` | 999 |
| `meleeRange` | 3 |
| `moveSpeed` | 6.3 |
| `playerHealth` | 100 |
| `posBackMax` | 2.2 |
| `posFwdGain` | 0.13 |
| `posFwdMax` | 3.2 |
| `posSideGain` | 0.18 |
| `posSideMax` | 2 |
| `powerHealth` | 700 |
| `refHealth` | 1000 |
| `regenDelay` | 3.5 |
| `regenRate` | 16 |
| `respawnDelay` | 2 |
| `resupplyCost` | 50 |
| `scorchLife` | 4 |
| `scorchRadius` | 2.4 |
| `shieldArcCos` | 0.3 |
| `shieldBlock` | 0.7 |
| `shieldCost` | 350 |
| `shieldMoveMult` | 0.72 |
| `shockRadius` | 7.5 |
| `sprintMult` | 1.4286 |
| `startCredits` | 1000 |
| `stepUp` | 0.55 |
| `weaponPitchMax` | 0.85 |

### `TUNE` (shamble-core.js:101)

| Key | Value |
|---|---|
| `ai.losCapS` | 0.1 |
| `ai.losPerS` | 480 |
| `ai.losRecheckS` | 0.1 |
| `ai.retargetS` | 0.25 |
| `ai.roles` | ["push", "objective", "hunt"] |
| `ff` | false |
| `fx.botCullR` | 50 |
| `fx.botSfxR` | 90 |
| `spawn.doorOut` | 4 |
| `spawn.lateral` | 2 |
| `teams.size` | 4 |

### `TEAMS` (shamble-core.js:95)

| Field | `allies` | `soviets` | `empire` |
|---|---|---|---|
| `faction` | "Allies" | "Soviets" | "Empire" |
| `id` | "allies" | "soviets" | "empire" |
| `name` | "Allies" | "Soviets" | "Empire" |
| `short` | "ALL" | "SOV" | "EMP" |

### `WEAPONS` (shamble-core.js:108)

| Field | `amk` | `bagpipe` | `shabzon` | `molotov` | `katana` | `lancer` | `torque` |
|---|---|---|---|---|---|---|---|
| `auto` | false | false | true | false | true | false | false |
| `cost` | 0 | 0 | 0 | 0 | 0 | 0 | 500 |
| `damage` | 34 | 11 | 8 | 0 | 9999 | 9 | 30 |
| `era` | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
| `fireInterval` | 0.2 | 0.85 | 0.072 | 1.05 | 0.45 | 0.42 | 0.7 |
| `id` | "amk" | "bagpipe" | "shabzon" | "molotov" | "katana" | "lancer" | "torque" |
| `mag` | 1 | 2 | 36 | 1 | 1 | 24 | 1 |
| `minLevel` | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
| `name` | "AMK Rifle I" | "Bagpipe" | "Shabzon I" | "Molotov" | "Beam Katana" | "Retro Lancer" | "Torque Bow" |
| `pellets` | 1 | 8 | 1 | 0 | 0 | 1 | 0 |
| `range` | 300 | 34 | 55 | 0 | 3.6 | 95 | 200 |
| `recoil` | 2 | 4.5 | 0.3 | 1 | 1 | 1.3 | 2.4 |
| `reload` | 3.5 | 2.6 | 1.9 | 0.1 |  | 2.2 | 1.5 |
| `reserve` | 999 | 24 | 144 | 999 | 0 | 120 | 10 |
| `short` | "AMK" | "BAG" | "SMG" | "MOLO" | "KAT" | "LAN" | "TRQ" |
| `spreadADS` | 0.006 | 0.05 | 0.06 | 0 | 0 | 0.008 | 0 |
| `spreadDecay` | 0.3 | 0.5 | 0.7 | 1 | 1 | 0.5 | 1 |
| `spreadHip` | 0.022 | 0.1 | 0.075 | 0 | 0 | 0.02 | 0 |
| `spreadMax` | 0.05 | 0.16 | 0.16 | 0 | 0 | 0.06 | 0 |
| `spreadPerShot` | 0 | 0 | 0.005 | 0 | 0 | 0.003 | 0 |
| `tracer` | 16773312 | 16767392 | 16773312 | 0 |  | 16773312 | 0 |
| `headMult` |  | 1.2 |  |  |  |  |  |
| `falloffEnd` |  |  | 35 |  |  |  |  |
| `falloffMin` |  |  | 0.2 |  |  |  |  |
| `falloffStart` |  |  | 15 |  |  |  |  |
| `arcDeg` |  |  |  |  | 120 |  |  |
| `impact` |  |  |  |  | 0.13 |  |  |
| `melee` |  |  |  |  | true |  |  |
| `swingDur` |  |  |  |  | 0.4 |  |  |
| `burst` |  |  |  |  |  | 3 |  |
| `burstInterval` |  |  |  |  |  | 0.06 |  |
| `charge` |  |  |  |  |  |  | true |
| `special` |  |  |  |  |  |  | true |

### `UNIT_TYPES` (shamble-core.js:126)

| Field | `conscript` | `peacekeeper` | `warrior` | `officer` |
|---|---|---|---|---|
| `ability` | "molotov" | "shield" | "banzai" | "bayonet" |
| `accent` | 16735067 | 5613823 | 16765514 | 10194175 |
| `altWpn` | 3 |  | 4 |  |
| `burst` | 4 | 1 | 6 | 3 |
| `cost` | 0 | 0 | 0 | 1250 |
| `enemyDmg` | 8 | 6 | 5 | 7 |
| `enemyRange` | 80 | 24 | 48 | 70 |
| `fireInterval` | 0.13 | 0.8 | 0.09 | 0.35 |
| `id` | "conscript" | "peacekeeper" | "warrior" | "officer" |
| `maxHealth` | 100 | 150 | 120 | 110 |
| `name` | "Conscript" | "Peacekeeper" | "Imperial Warrior" | "Officer" |
| `preferRange` | 24 | 11 | 9 | 18 |
| `speedMult` | 1 | 0.95 | 1.5 | 1.05 |
| `sprintMult` | 1.214 | 1.165 | 1.62 | 1.214 |
| `swapCd` | 1.5 | 2 |  |  |
| `tier` | 1 | 1 | 1 | 2 |
| `wpn` | 0 | 1 | 2 | 5 |
| `banzaiCd` |  |  | 45 |  |
| `banzaiDur` |  |  | 10 |  |
| `lunge` |  |  | 48 |  |
| `bayonetCd` |  |  |  | 10 |
| `bayonetDur` |  |  |  | 1.3 |

### `VEHICLE_TYPES` (shamble-core.js:141)

| Field | `juggernaut` |
|---|---|
| `cost` | 2500 |
| `desc` | "Enterable heavy assault walker (Kane's Wrath-inspired) — heavy armour, triple artillery cannons." |
| `id` | "juggernaut" |
| `maxHealth` | 1500 |
| `name` | "Juggernaut" |
| `seats` | 1 |
| `tier` | 2 |

### `SOLDIER_KIT` (shamble-game.js:124)

| Key | Value |
|---|---|
| `face` | "mustache" |
| `feet` | "boots" |
| `hair` | "default" |
| `headgear` | "brodie" |
| `legs` | "trousers" |
| `torso` | "greatcoat" |
| `webbing` | "webbing" |

### `ELITE_KIT` (shamble-game.js:125)

| Key | Value |
|---|---|
| `face` | "none" |
| `feet` | "boots" |
| `hair` | "crop" |
| `headgear` | "none" |
| `legs` | "trousers" |
| `torso` | "greatcoat" |
| `webbing` | "webbing" |

### `RIFLE_TIERS` (shamble-game.js:788)

| # | `name` | `reload` | `span` |
|---|---|---|---|
| 0 | "AMK Rifle I" | 5 | 5 |
| 1 | "AMK Rifle II" | 3.5 | 10 |
| 2 | "AMK Rifle III" | 2.5 | 0 |

### `RULES_DEFAULT` (shamble-game.js:40)

| Key | Value |
|---|---|
| `eco.resupply` | 50 |
| `harv.dock` | 3 |
| `harv.load` | 3.5 |
| `harv.respawn` | 12 |
| `silo.capture` | 3.5 |
| `tbox.active` | 120 |
| `tbox.capture` | 7 |
| `tbox.first` | 8 |
| `tbox.gap` | 6 |

### `ECO` (shamble-game.js:689)

| Key | Value |
|---|---|
| `eraCost` | [60, 90, 120] |
| `eraNames` | ["T1", "T2", "T3", "T4"] |
| `payHarvester` | 600 |
| `paySilo` | 1 |
| `payTrickle` | 1 |
| `payTrickleDead` | 0.25 |
| `techBox` | 10 |
| `techHarvester` | 5 |
| `techPerHeadshot` | 2 |
| `techPerKill` | 1 |

### `SILO` (shamble-game.js:972)

| Key | Value |
|---|---|
| `captureTime` | 3.5 |
| `points[0].x` | -140 |
| `points[0].z` | -90 |
| `points[1].x` | 140 |
| `points[1].z` | -90 |
| `pool[0].x` | -140 |
| `pool[0].z` | -90 |
| `pool[1].x` | 140 |
| `pool[1].z` | -90 |
| `pool[2].x` | 0 |
| `pool[2].z` | 0 |
| `range` | 5 |

### `TBOX` (shamble-game.js:706)

| Key | Value |
|---|---|
| `activeTime` | 120 |
| `captureTime` | 7 |
| `firstDelay` | 8 |
| `gap` | 6 |
| `names` | ["Alpha", "Bravo", "Charlie"] |
| `points[0].name` | "Alpha" |
| `points[0].x` | 0 |
| `points[0].z` | -60 |
| `points[1].name` | "Bravo" |
| `points[1].x` | -70 |
| `points[1].z` | 0 |
| `points[2].name` | "Charlie" |
| `points[2].x` | 70 |
| `points[2].z` | 0 |
| `pool[0].name` | "Alpha" |
| `pool[0].x` | 0 |
| `pool[0].z` | -60 |
| `pool[1].name` | "Bravo" |
| `pool[1].x` | -70 |
| `pool[1].z` | 0 |
| `pool[2].name` | "Charlie" |
| `pool[2].x` | 70 |
| `pool[2].z` | 0 |
| `range` | 5.5 |

### `HARV` (shamble-game.js:704)

| Key | Value |
|---|---|
| `dockTime` | 3 |
| `fieldGrp` | null |
| `fieldX` | 0 |
| `fieldZ` | 0 |
| `hp` | 900 |
| `lanes` | null |
| `loadTime` | 3.5 |
| `respawn` | 12 |
| `speed` | 7 |

### `JUG` (shamble-game.js:1188)

| Key | Value |
|---|---|
| `fireInterval` | 3.4 |
| `maxSpeed` | 6 |
| `range` | 34 |
| `rippleMs` | 80 |
| `speed` | 3.2 |
| `targetH` | 5.6 |

### `JUG_PILOT` (shamble-game.js:1189)

| Key | Value |
|---|---|
| `aimTurn` | 9 |
| `moveSpeed` | 5.5 |
| `turn` | 3.2 |

### `JUG_CANNON` (shamble-game.js:1190)

| Key | Value |
|---|---|
| `damage` | 130 |
| `headMult` | 1 |
| `range` | 130 |
| `tracer` | 16765562 |

### `T_CHARGE` (shamble-game.js:1696)

| Key | Value |
|---|---|
| `bow.capApex` | 0.7 |
| `bow.col` | 16734762 |
| `bow.floor` | 0.05 |
| `bow.grav` | 14 |
| `bow.pitch` | 0.04 |
| `bow.sMax` | 94 |
| `bow.sMin` | 30 |
| `bow.time` | 1.35 |
| `jav.col` | 3727792 |
| `jav.floor` | 0.05 |
| `jav.grav` | 14 |
| `jav.pitch` | 0.05 |
| `jav.sMax` | 64 |
| `jav.sMin` | 32 |
| `max` | 1 |
| `nade.col` | 16756832 |
| `nade.floor` | 0.26 |
| `nade.grav` | 20 |
| `nade.pitch` | 0.72 |
| `nade.sMax` | 26 |
| `nade.sMin` | 13 |

### `STAM` (shamble-game.js:2216)

| Key | Value |
|---|---|
| `lockUntil` | 0.3 |
| `max` | 7 |
| `regen` | 0.2 |

## Simulation: tick, input bits, hit bodies

| Constant | Value | Where |
|---|---|---|
| `FIXED_DT` | 0.016666666666666666 | shamble-core.js:1479 |

### `BTN` (shamble-core.js:1480)

| Key | Value |
|---|---|
| `ADS` | 2 |
| `FIRE` | 1 |
| `GRENADE` | 16 |
| `MELEE` | 8 |
| `RELOAD` | 4 |
| `SHIELD` | 32 |
| `SPRINT` | 64 |

### `BODY` (shamble-core.js:1482)

| Key | Value |
|---|---|
| `crouchDrop` | 0.55 |
| `headR` | 0.4 |
| `headY` | 2.12 |
| `legBotY` | 0.12 |
| `legR` | 0.42 |
| `legTopY` | 0.95 |
| `torsoBotY` | 0.95 |
| `torsoR` | 0.52 |
| `torsoTopY` | 1.5 |

### `PART` (shamble-core.js:1483)

| Key | Value |
|---|---|
| `body` | 1 |
| `head` | 2.6 |
| `legs` | 0.65 |

## Progression and presentation: ranks, prestige, words, camera styles, UI timings

| Constant | Value | Where |
|---|---|---|
| `VIG_W` | 300 | shamble-game.js:2347 |
| `WR_DEPLOY_MS` | 280 | shamble-game.js:2999 |
| `RR_C` | 100.53096491487338 | shamble-game.js:1099 |
| `OCC_FADE` | 0.24 | shamble-game.js:2555 |

### `RANK_TIERS` (shamble-game.js:1959)

| # | `metal` | `motif` | `name` | `upTo` | `n` | `rockers` | `star` |
|---|---|---|---|---|---|---|---|
| 0 | "bronze" | "plain" | "Recruit" | 3 |  |  |  |
| 1 | "bronze" | "chevron" | "Private" | 8 | 1 |  |  |
| 2 | "bronze" | "chevron" | "Private First Class" | 14 | 1 | 1 |  |
| 3 | "silver" | "chevron" | "Corporal" | 21 | 2 |  |  |
| 4 | "silver" | "chevron" | "Sergeant" | 29 | 3 |  |  |
| 5 | "silver" | "chevron" | "Staff Sergeant" | 37 | 3 | 1 |  |
| 6 | "silver" | "chevron" | "Technical Sergeant" | 45 | 3 | 2 |  |
| 7 | "gold" | "chevron" | "Master Sergeant" | 53 | 3 | 3 |  |
| 8 | "gold" | "chevron" | "First Sergeant" | 60 | 3 | 3 | 1 |
| 9 | "gold" | "chevron" | "Sergeant Major" | 67 | 3 | 3 | 2 |
| 10 | "gold" | "bar" | "Second Lieutenant" | 73 | 1 |  |  |
| 11 | "silver" | "bar" | "First Lieutenant" | 79 | 1 |  |  |
| 12 | "silver" | "bar" | "Captain" | 85 | 2 |  |  |
| 13 | "gold" | "oak" | "Major" | 90 | 1 |  |  |
| 14 | "silver" | "oak" | "Lieutenant Colonel" | 94 | 1 |  |  |
| 15 | "gold" | "eagle" | "Colonel" | 97 |  |  |  |
| 16 | "gold" | "stars" | "Brigadier General" | 99 | 1 |  |  |
| 17 | "gold" | "stars" | "General of the Army" | 100 | 5 |  |  |

### `PRESTIGE` (shamble-game.js:2050)

| # | `center` | `gem` | `label` | `metal` | `points` | `wreath` |
|---|---|---|---|---|---|---|
| 0 | "star" | "#cfd6dd" | "First Honor" | "silver" | 5 |  |
| 1 | "gem" | "#ff5a5a" | "Iron Star" | "silver" | 8 |  |
| 2 | "star" | "#ffd24d" | "Gilded Cross" | "gold" | 6 |  |
| 3 | "gem" | "#ff4d4d" | "Crimson Sunburst" | "gold" | 8 |  |
| 4 | "orb" | "#9fd3ff" | "Azure Order" | "silver" | 12 |  |
| 5 | "gem" | "#5fe06a" | "Emerald Cross" | "gold" | 6 |  |
| 6 | "eagle" | "#ffe08a" | "Eagle Standard" | "gold" | 10 |  |
| 7 | "gear" | "#eaf4ff" | "Platinum Aegis" | "platinum" | 8 |  |
| 8 | "star" | "#ff5a5a" | "Marshal Wreath" | "gold" | 12 | true |
| 9 | "star" | "#ffd24d" | "Supreme Command" | "platinum" | 16 | true |

### `ROMAN` (shamble-game.js:1860)

["I", "II", "III", "IV", "V", "VI", "VII", "VIII"]

### `GUN_WORDS` (shamble-game.js:45)

["BLAM", "POW", "BANG", "TRA", "DAKKA", "RAT-A-TAT", "BRRT", "THUNK", "BRRR.."]

### `KILL_WORDS` (shamble-game.js:46)

["POW!", "KO!", "BAM!", "BOOM!", "ZAP!"]

### `CAM_STYLES` (shamble-game.js:2195)

| Key | Value |
|---|---|
| `1.adsCam` | "pitch" |
| `1.camBack` | 16 |
| `1.camHeight` | 24 |
| `1.fov` | 52 |
| `1.fovADS` | 52 |
| `1.id` | 1 |
| `1.yaw` | 0 |
| `2.adsCam` | "pitch" |
| `2.camBack` | 10.5 |
| `2.camHeight` | 15.5 |
| `2.fov` | 52 |
| `2.fovADS` | 52 |
| `2.id` | 2 |
| `2.yaw` | 0 |
| `3.adsCam` | "pitch" |
| `3.camBack` | 56.6 |
| `3.camHeight` | 40 |
| `3.fov` | 15 |
| `3.fovADS` | 15 |
| `3.id` | 3 |
| `3.yaw` | 0.7853981633974483 |
| `4.adsCam` | "pitch" |
| `4.camBack` | 65.5 |
| `4.camHeight` | 41.5 |
| `4.fov` | 12 |
| `4.fovADS` | 12 |
| `4.id` | 4 |
| `4.yaw` | 0.7853981633974483 |
| `5.adsCam` | "pan" |
| `5.camBack` | 42 |
| `5.camHeight` | 52 |
| `5.fov` | 18 |
| `5.fovADS` | 18 |
| `5.id` | 5 |
| `5.yaw` | 0.7853981633974483 |

### `SFX` (shamble-game.js:2219)

| Key | Value |
|---|---|
| `ctx` | null |
| `master` | 0.85 |
| `muted` | false |

## Look and builders (look brief pending: listed as data only, never a decision)

| Constant | Value | Where |
|---|---|---|
| `SUN_R` | 65.8 | shamble-core.js:167 |
| `INK` | 1118746 | shamble-core.js:240 |
| `WORLD_OUTLINE` | 0.09 | shamble-core.js:241 |
| `CHAR_OUTLINE` | 0.8 | shamble-core.js:242 |
| `RIM_COL` | 16742994 | shamble-core.js:254 |
| `OUTLINE_GROUND_Y` | 0.02 | shamble-core.js:294 |
| `ARM_L1` | 0.46 | shamble-game.js:209 |

### `ENV_DEFAULT` (shamble-core.js:169)

23 values; see `docs/legacy-tuning.json` → `tables.ENV_DEFAULT`.

### `TEAM_COL` (shamble-game.js:709)

6 values; see `docs/legacy-tuning.json` → `tables.TEAM_COL`.

### `TEAM_HEX` (shamble-game.js:710)

6 values; see `docs/legacy-tuning.json` → `tables.TEAM_HEX`.

### `FACTION_PALETTES` (shamble-core.js:635)

15 values; see `docs/legacy-tuning.json` → `tables.FACTION_PALETTES`.

### `CHAR_DEFAULT` (shamble-core.js:421)

260 values; see `docs/legacy-tuning.json` → `tables.CHAR_DEFAULT`.

### `CHAR_MALE` (shamble-core.js:463)

375 values; see `docs/legacy-tuning.json` → `tables.CHAR_MALE`.

### `CHAR_FEMALE` (shamble-core.js:508)

408 values; see `docs/legacy-tuning.json` → `tables.CHAR_FEMALE`.

### `CPARTS` (shamble-core.js:567)

7 values; see `docs/legacy-tuning.json` → `tables.CPARTS`.

### `GUN_KIT` (shamble-core.js:785)

40 values; see `docs/legacy-tuning.json` → `tables.GUN_KIT`.

### `BK` (shamble-core.js:1296)

72 values; see `docs/legacy-tuning.json` → `tables.BK`.

### `MELEE_IDLE` (shamble-core.js:1434)

2 values; see `docs/legacy-tuning.json` → `tables.MELEE_IDLE`.

### `ANIM` (shamble-game.js:132)

130 values; see `docs/legacy-tuning.json` → `tables.ANIM`.

## Runtime state, not tuning (kept in the JSON for completeness)

### `NAV` (shamble-game.js:2617)

17 values; see `docs/legacy-tuning.json` → `tables.NAV`.

### `AI_STATE` (shamble-game.js:1581)

4 values; see `docs/legacy-tuning.json` → `tables.AI_STATE`.

## Boot changes per map

Values that `bootGame()` changes after the defaults, by map. NAV and BK entries are the navigation grid and the
building-kit registries filled at boot (state, not tuning).

### DM_Cabana (default): 18 changes

| Path | Default | After boot |
|---|---|---|
| `BK.PROPS` | null | (37 values; see the JSON) |
| `BK.TREES` | null | (31 values; see the JSON) |
| `HARV.fieldGrp` | null | null |
| `NAV.blk` | null | null |
| `NAV.came` | null | null |
| `NAV.closed` | null | null |
| `NAV.f` | null | null |
| `NAV.g` | null | null |
| `NAV.heap` | null | null |
| `NAV.hn` | 0 | 30 |
| `NAV.nx` | 0 | 185 |
| `NAV.nz` | 0 | 125 |
| `NAV.ok` | false | true |
| `NAV.sid` | 0 | 3 |
| `NAV.stamp` | null | null |
| `NAV.tmp` | null | null |
| `NAV.x0` | 0 | -185 |
| `NAV.z0` | 0 | -125 |

### DM_Tri (?map=tri): 41 changes

| Path | Default | After boot |
|---|---|---|
| `BK.PROPS` | null | (37 values; see the JSON) |
| `BK.TREES` | null | (31 values; see the JSON) |
| `HARV.fieldGrp` | null | null |
| `HARV.fieldZ` | 0 | -23 |
| `HARV.lanes` | null | (12 values; see the JSON) |
| `TBOX.points[0].x` | 0 | 39.84 |
| `TBOX.points[0].z` | -60 | 0 |
| `TBOX.points[1].x` | -70 | 0 |
| `TBOX.points[1].z` | 0 | -69 |
| `TBOX.points[2].x` | 70 | -39.84 |
| `TBOX.pool[0].x` | 0 | 39.84 |
| `TBOX.pool[0].z` | -60 | 0 |
| `TBOX.pool[1].x` | -70 | 0 |
| `TBOX.pool[1].z` | 0 | -69 |
| `TBOX.pool[2].x` | 70 | -39.84 |
| `SILO.points[0].x` | -140 | 76.21 |
| `SILO.points[0].z` | -90 | 21 |
| `SILO.points[1].x` | 140 | 0 |
| `SILO.points[1].z` | -90 | -111 |
| `SILO.points[2]` | null | {"x": -76.21, "z": 21} |
| `SILO.pool[0].x` | -140 | 76.21 |
| `SILO.pool[0].z` | -90 | 21 |
| `SILO.pool[1].x` | 140 | 0 |
| `SILO.pool[1].z` | -90 | -111 |
| `SILO.pool[2].x` | 0 | -76.21 |
| `SILO.pool[2].z` | 0 | 21 |
| `NAV.blk` | null | null |
| `NAV.came` | null | null |
| `NAV.closed` | null | null |
| `NAV.f` | null | null |
| `NAV.g` | null | null |
| `NAV.heap` | null | null |
| `NAV.hn` | 0 | 25 |
| `NAV.nx` | 0 | 185 |
| `NAV.nz` | 0 | 125 |
| `NAV.ok` | false | true |
| `NAV.sid` | 0 | 3 |
| `NAV.stamp` | null | null |
| `NAV.tmp` | null | null |
| `NAV.x0` | 0 | -185 |
| `NAV.z0` | 0 | -125 |

## Left out

36 paths the dump does not serialise (functions, mostly the character-part and gun-model makers):

`CPARTS.face.beard (function)`, `CPARTS.face.goatee (function)`, `CPARTS.face.mustache (function)`, `CPARTS.face.none (function)`, `CPARTS.face.stubble (function)`, `CPARTS.feet.boots (function)`, `CPARTS.feet.none (function)`, `CPARTS.hair.bun (function)`, `CPARTS.hair.crop (function)`, `CPARTS.hair.default (function)`, `CPARTS.hair.mohawk (function)`, `CPARTS.hair.none (function)`, `CPARTS.hair.ponytail (function)`, `CPARTS.hair.sidepart (function)`, `CPARTS.hair.swept (function)`, `CPARTS.headgear.brodie (function)`, `CPARTS.headgear.cap (function)`, `CPARTS.headgear.m1 (function)`, `CPARTS.headgear.none (function)`, `CPARTS.legs.none (function)`, `CPARTS.legs.trousers (function)`, `CPARTS.torso.greatcoat (function)`, `CPARTS.torso.none (function)`, `CPARTS.torso.tunic (function)`, `CPARTS.webbing.none (function)`, `CPARTS.webbing.webbing (function)`, `GUN_KIT.amk.make (function)`, `GUN_KIT.bagpipe.make (function)`, `GUN_KIT.lancer.make (function)`, `GUN_KIT.shabzon.make (function)`, `GUN_KIT.torque.make (function)`, `BK.build (function)`, `BK.buildCover (function)`, `BK.roll (function)`, `BK.tex (function)`, `AI_STATE.rotatePush (function)`
