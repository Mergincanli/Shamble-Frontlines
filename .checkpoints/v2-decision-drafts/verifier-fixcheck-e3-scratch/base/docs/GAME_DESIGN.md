# Shamble Frontlines — Game Design & Vision

> Read this together with `TECHNICAL.md` (engineering), `ROADMAP.md` (full build history + what's next), and `UNITS.md` (inspiration roster + planned expansion). This file is the **why**; ROADMAP is the **what happened**; TECHNICAL is the **how**.

---

## 1. What it is (the 30-second version)

**Shamble Frontlines** is a self-contained, browser-based **top-down team shooter** (double-click to play — no install, no build step) built in Three.js (r128). You pilot **one soldier** in a small symmetric base-assault arena, fighting an AI enemy team. You pick a unit class, manage a two-currency economy, and **win by destroying the enemy HQ**.

It is a deliberate fusion of four games:
- **Foxhole** — the overhead tactical camera, the sense of scale, mouse-aim into a 3D world.
- **C&C: Renegade** (and **Renegade X**) — the base-assault loop: buy classes/kit at an **Armory terminal**, and **prices spike when your power plant dies**.
- **Red Alert 3** — the **bright, vibrant, slightly toy-like aesthetic**, the rock-paper-scissors unit philosophy, and the **three-faction split** (Allied / Soviet / Empire).
- **Gears of War** — gun *feel*: weight + one mechanic per gun; melee built into rifles.

It is **not** grimdark, not realistic, not a milsim. It's a joyful, readable, colourful toy-box battlefield.

---

## 2. Design pillars

1. **Readability first.** From an overhead camera, everything must read instantly — bright palette, bold silhouettes, cel-shaded forms, ink outlines, comic onomatopoeia for actions (BLAM / POW / BANZAI!).
2. **Class identity over loadout freedom.** Each unit is a fixed package (a primary weapon you cannot swap off + one signature ability). You change role by switching *class*, not by editing a loadout. This keeps every unit legible to both the player and their opponent.
3. **Rock-paper-scissors at the core.** Units counter each other (see the triangle below) so there's always a reason to switch class mid-fight.
4. **Gun feel = weight + a hook.** Each weapon has a distinct cadence and a mechanic (burst, scattergun, one-shot melee charge), à la Gears — not stat-sheet realism.
5. **Base-assault economy with momentum swings.** Destroying enemy infrastructure (power plant) raises their costs; capturing silos/tech boxes advances your team's era. Map control matters.
6. **Positive, vibrant energy.** The psychological feel is *upbeat* — the ground was deliberately moved from flat tan desert to **lush procedural grass** to push that RA3 brightness.
7. **Faction identity is worn, not just coloured.** Each of the three factions has its own uniform palette + trim so you can read allegiance at a glance (see §7.5).

---

## 3. The "Living Comic" aesthetic — and *why*

> **Locked art direction (2026-07-08): "Living Comic."** The full, exact recipe (palette hex, cel-ramp arrays, outline weights, lighting, halftone, the Empire-elite villain material) lives in **`ART_DIRECTION.md`** — the authoritative visual spec. This section is the vision; ART_DIRECTION is the numbers.

A custom **non-photorealistic (NPR) cel-shaded** look — flat, saturated, comic-booky:
- **MeshToonMaterial** with a **2-band gradient ramp** (hard light/shadow, no soft mid-band — flat and punchy). Metal keeps a bright specular band so weapons pop; skin keeps a warm ramp + rim.
- **Inverted-hull ink outlines** with a **differential weight**: the **world** gets a **bold** near-black line (`0x11121a`) for graphic punch, while **characters/units/weapons** get a **thinner** line (0.8×) so their shapes and gear stay legible. A back-face hull `ShaderMaterial` pushes each vertex along its normal (`position + normal*thickness`) and fills flat ink; parented as a child mesh so it follows animated limbs. The hull is clamped above the ground, so every object also gets a **contact line** where it meets the grass — nothing looks pasted on.
- **Fresnel skin rim** — a fake-subsurface warm rim injected into the skin's toon material via `onBeforeCompile` (rim colour/strength/power uniforms). Gives faces a soft lit edge without a second light.
- **Colour truth:** what's authored is what's on screen (linear output) — the palette isn't washed out by a double gamma lift.
- **Bright white key light + low, cool fill** so shadows land as real, defined shadows (not a grey wash), with a violet bounce term. Punchy blue sky. Buildings and characters get this richer, darker balance; the **environment** (grass, trees, hedges, sea) gets its own brightness gain so it stays lush and colourful.
- **Comic onomatopoeia** (canvas-drawn words) that pop on guns firing, kills, and abilities.
- **Two UI families, one palette:** **paper** for menu-context screens (title, pause, career, end of round); **dark field-kit** for overlays over the live game (armory, scoreboard, HUD) so they stay readable over the bright world.
- **Lush, punchy-green procedural grass** ground (was desert tan), bright comic sky.
- **Per-faction uniform palettes** (see §7.5) applied through a material-**role** system so the same mesh recolours per faction.
- **Halftone dots** (the classic comic print-shading texture) are a **tunable, default-off** overlay — dial strength/size or turn them fully off (currently off).

**Why this look:**
- It's **readable** from the overhead camera and **joyful** (RA3 energy, not grim).
- The cel look **hides the low-poly procedural geometry** and unifies everything (characters, buildings, props are all simple primitives) into one cohesive toy-box style.
- It **matches the procedural character rig** (rigid primitives) — a realistic shader would expose the blocky forms; the toon shader celebrates them.
- It's **cheap and self-contained** — no texture assets, no PBR maps; colour + ramp + outline do all the work.

---

## 4. Camera & controls

- **Isometric-style follow cam** (the default of four dev presets in `CAM_STYLES`): height 40, pulled back 56.6 (≈35° pitch), 45° yaw, FOV 15 — a long telephoto lens, so the view is near-orthographic and frames a patch of roughly 32×18 m around the soldier. `[` resets the camera yaw.
- **Mouse-aim**: the cursor projects to a ground point; the soldier faces it and fires toward it.
- **Dead-zone camera framing**: inside a centred screen box the camera does *not* react to crosshair movement (kills motion-sickness from micro-panning); as the crosshair nears the edge the camera eases to pan/look ahead. The aim itself is always instant.
- **Movement**: WASD relative to the camera; sprint on Shift (stamina-limited, per-class sprint multiplier).
- **Keys**: `1` = primary weapon · `3` = special weapon (bought in the shop) · `F` = unit ability · `G` = hold to charge a frag grenade, release to throw (must be bought) · `E` = context use (enter/exit the Juggernaut, open the Armory shop, resupply) · `R` = reload · hold `Tab` = team scoreboard · `Esc`/`P` = pause · backtick (`` ` ``) = DEV panel. You pick your class at the Armory — there's no 1–4 hot-swap, and the mouse wheel does nothing in play.

---

## 5. The core gameplay loop

> **Superseded as the design target by [`LOOP_V2.md`](LOOP_V2.md) §1–4 and §8 (October 2026: three teams, harvest
> money, uplink tech, points race).** This section still describes the build as it plays today, until the greybox
> prototype (LOOP_V2 §11) replaces it.

```
Spawn at your barracks
   → pick a unit class at the Armory terminal
   → push out, fight the enemy team
   → earn Pay (personal credits) + Tech (shared team era)
   → spend at the Armory terminal (E): grenades, resupply; era unlocks raise the team
   → destroy enemy infrastructure to raise their costs / capture silos & tech boxes
   → break the enemy base and destroy their HQ  → WIN the round
Death → brief respawn timer → redeploy at your barracks (keep your global rank progression)
```

It's a **continuous skirmish** (currently PvE vs an AI enemy team of 6), not a wave/round survival mode. The win condition is **destroy the enemy HQ** (Renegade-style).

---

## 6. Economy (two currencies)

> **Superseded as the design target by [`LOOP_V2.md`](LOOP_V2.md) §3–4 (money from harvesters, tech from uplinks;
> silos and tech boxes are replaced).** This section still describes today's build.

- **Pay** — *personal* credits the player earns from kills; spent on consumables (frag grenades, resupply).
- **Tech** — *shared, per-team* resource that advances the team's **era** (T1 → T4), gating higher-tier kit (planned). Earned per kill/headshot and from **capturable silos** and **tech boxes** on the map.
- **Power-plant price multiplier** — a live `priceMult` raises costs when your power infrastructure is down (Renegade mechanic). This is the lever that makes attacking the enemy economy meaningful.

---

## 6.5 The map — Cabana Republic

The shipped map is a **symmetric island** built from an ASCII grid (10 m cells): a ring of roads, a V of roads from the centre to each refinery (the harvester routes), each team's base in a bottom corner (HQ, barracks, refinery, power plant), two silos in the top corners, three tech points and the central harvester field. Since 2026-09-25 it carries a **Normandy village**: a four-house hamlet along each outer road (timber, thatch, brick), a brick farm per side, the **Norman manor** on the centre line, apple orchards, poplars lining the top road, oaks and bocage hedgerows around the side objectives. Designed as one half and mirrored for fairness, and kept off every bot route; the **centre stays open** as the main combat space. It's plain map data, so it round-trips through the editor.

---

## 7. Current roster (IMPLEMENTED) — 4 classes, 7 weapons, 1 vehicle  *(values verified against the build, 2026-09-25)*

> This is what's *in the build right now*. The broader **planned** roster (RA3 / Kane's Wrath / Renegade / Gears archetypes, era tiers) lives in `UNITS.md`.

### The R/P/S triangle
**Imperial Warrior → Conscript → Peacekeeper → Imperial Warrior** (the fast melee Warrior beats the mid-range Conscript, who kites the short-range Peacekeeper, whose shotgun+shield shreds the charging Warrior). Enemies are also drawn from these unit types, so the triangle exists in PvE.

### Units
| Unit | HP | Primary (locked) | Ability (F) | Identity |
|---|---|---|---|---|
| **Conscript** | 100 | AMK Rifle I (semi-auto) | **Molotov** (lobbed incendiary, AoE, bonus vs structures; **bypasses the shield**) | The kiting generalist |
| **Peacekeeper** | 150 | Bagpipe (scattergun) | **Round Shield** (blocks *frontal bullets only*, slows you) | Tanky short-range bruiser |
| **Imperial Warrior** | 120 | Shabzon I (high-RoF SMG) | **Banzai** (10s) — equips a **Beam Katana**, one-hit melee charge | Glass-cannon rusher (fast, **lunges**) |
| **Officer** *(tier 2 — costs 1250)* | 110 | Retro Lancer (3-round burst) | **Bayonet charge** — auto-drives toward the aim point and impales (one-shot, ignores shield, 10s cd) | Mid-range punch + a gap-closer |

### Weapons
| # | Name | Type | Key stats | Notes |
|---|---|---|---|---|
| 0 | **AMK Rifle I** | semi-auto | mag 1, dmg 34, reload 3.5s, recoil 2.0, range 300 | Deliberately slow single-shot musket-rifle; high reward per hit |
| 1 | **Bagpipe** | scattergun | mag 2, 8 pellets × 11 dmg, range 34, recoil 4.5, head ×1.2 | Lethal point-blank, useless far; pairs with the shield |
| 2 | **Shabzon I** | auto SMG | fireInterval 0.072, mag 36, dmg 8, falloff 15→35m | Shreds up close, **damage falloff** punishes range |
| 3 | **Molotov** | thrown (ability) | unlimited; rifle disabled while equipped | AoE + bonus vs buildings; bypasses shield |
| 4 | **Beam Katana** | melee (ability) | dmg 9999, arc 120°, swing 0.45s | One-hit kill while Banzai charge is active |
| 5 | **Retro Lancer** | 3-round burst | burstInterval 0.06, fireInterval 0.42, mag 24, dmg 9 | FAMAS-like; accurate, punchy per burst |
| 6 | **Torque Bow** | charge (special) | hold to charge; dmg 30, range 200, mag 1, reload 1.5 | Shop special (500 × price multiplier), key `3`; bolts fly under gravity, stick, then detonate |

### Vehicles — the Juggernaut (the first vehicle)
The **Juggernaut** is the game's first vehicle — the opening move of Phase 5.2 (ground vehicles) and the template the later vehicles build on. A Kane's Wrath-inspired **enterable heavy assault walker**. *(Verified against the build, 2026-09-25.)*
- **How you get it:** buy it at the Armory shop once your team reaches **era T2** — **2500 credits × the power-plant price multiplier**. It deploys right beside you.
- **Piloting:** `E` to enter or exit. Inside, your soldier is hidden and you drive the walker (move 5.5, turn 3.2) while its **cannon turret aims independently** at the cursor.
- **Weapon:** **triple artillery cannons** fired as a ripple (80 ms apart) — **130 damage per shell, 130 m range**; hits troops and **enemy buildings** (a siege tool).
- **Durability:** **1500 HP**, one seat. Heavy footfalls shake the camera when it walks near you.
- **Left alone** it idles where you parked it — no AI driving, no auto-fire.
- **Known gaps (design + polish):** the livery is hardcoded red + gold instead of your faction's palette; shell impacts are visual only (no splash damage); no enemy Juggernauts yet, so it has no counter-vehicle; the counter-play for infantry (e.g. anti-armour classes from UNITS.md) is still to be designed.

### Abilities & rules of note
- **Shield blocks bullets, not explosions.** A shielded Peacekeeper still dies to a Molotov or grenade — that's the intended counterplay.
- **Class primaries are hard-locked.** The shop sells extras only — frag grenades and the special weapon (Torque Bow, key `3`); legacy cross-class weapon-buying code is dead. You change your primary only by switching class.
- **Frag grenade** is a shop purchase (you spawn with none; carry up to 3), charged and thrown with `G` — distinct from the Conscript's Molotov.
- **Shots start from a fixed point**, not the animated muzzle: 1.9 m up, just ahead of the chest on the gun side (the old aim-pose height), so firing over cover is the same whether you aim or hip-fire. Animation is cosmetic.
- **Locational damage**: head / body / legs hitboxes (head crits; per-weapon head multipliers, e.g. Bagpipe only ×1.2 so it can't head-snipe at range).

---

## 7.5 Faction identity & the Character Lab (IMPLEMENTED)

The **"faction lens"** question (see the old roadmap) has been answered visually: there are **three factions, RA3-style — Soviet, Allies, Empire** — each with a distinct **uniform palette**. This is currently a **visual identity layer** (how a soldier is dressed/coloured), applied through a material-**role** system. The gameplay classes in §7 have the same rules in every faction; the uniform is the faction layer, and **gameplay spawns wear it** (`buildSoldier` dresses every unit per faction).

**Faction palettes** (role → colour):

| Role | Soviet | Allies | Empire |
|---|---|---|---|
| Base gear (fatigues) | coffee-brown `0x432818` | imperial-blue `0x00296B` | midnight-purple `0x240046` |
| Faction primary (accents) | flag-red `0xD41414` | bright-blue `0x1F6FF0` | electric-purple `0x7B2FD6` |
| Highlight trim | vanilla `0xFDF0D5` | yellow `0xFDC500` | neon-orange `0xFF6D00` |
| Pauldron (`PadColor`) | red `0xD41414` | yellow `0xFDC500` | orange `0xFF6D00` |

> **Living Comic update (locked):** the **faction primaries were brightened** for punch (Allies `0x003F88`→`0x1F6FF0`, Empire `0x5A189A`→`0x7B2FD6`; Soviet went to `0xE11020` and was later retuned in code to `0xD41414`, used for primary, pauldron and coat). Character **outlines are the shared dark ink** (`0x11121a`), *not* the faction colour — allegiance reads from the uniform, not the line. **Empire elites** are a planned **separate villain unit type**: near-black lacquered armour with a **purple** outline (so the silhouette reads) and a glowing violet visor — see `ART_DIRECTION.md` §7.

**The Character Lab** is an in-game tool (menu → Character Lab) for designing/previewing a soldier with the *real* in-game pipeline (what you sculpt is exactly what spawns):
- **Body**: Male / Female base bodies (distinct proportions; the female body is deliberately slighter).
- **Faction**: Soviet / Allies / Empire (recolours via the role system live).
- **Gear slots** (cycle-pickers): **Headgear** (None, Brodie helmet, M1 helmet, Field Cap), **Torso** (None, Tunic, Greatcoat), **Legs** (None, Trousers), **Feet** (None, Boots), **Webbing** (None, Webbing — leather belt + buckle + ammo pouches).
- **Hair** (cycle): Default, None (bald), Buzz Crop, Swept Back, Side Part, Bun, Ponytail, Mohawk. Auto-hidden under a helmet; follows the hair-colour picker.
- **Facial hair** (cycle): None, Stubble, Mustache, Goatee, Full Beard. Shows even under a helmet.
- **Face**: subtle eyebrows + a small nose are baked into both base bodies for character.
- **Proportions** sliders (head size/width, body girth, shoulders, arm/hand, hips, leg thickness, foot size) and **Colour** pickers (skin, hair, eyes).
- **Export / Import** a recipe as JSON; Spin / Reset view.

The design intent: build the **faction wardrobe** (period-military WW2/dieselpunk silhouettes — greatcoats, field caps, helmets, webbing) as reusable **CPARTS** so units and future tiers can be dressed per faction with consistent, readable colours.

---

## 8. Why each mechanic exists (designer's intent)

- **Loadout lock** → class legibility; you always know what an enemy can do by their silhouette/type.
- **Two currencies** → separates *personal skill reward* (Pay) from *team macro progress* (Tech/era), so a good player helps the team advance without simply out-buying everyone personally.
- **Power-plant pricing + silos/tech boxes** → gives map control and base-attacking a tangible payoff (momentum swings, the Renegade heart).
- **Recoil as an FOV "punch," not camera pitch** → you *feel* the shot (the zoom kicks out and snaps back) but the crosshair never drifts off target — overhead aim stays honest.
- **Procedural rig + code-driven animation** → see TECHNICAL; chosen to stay self-contained (no asset pipeline) and on-style.
- **Sprint stamina + per-class speed** → the Warrior is a genuine glass-cannon rusher; the Peacekeeper is a slow wall; movement is part of the R/P/S.
- **Material-role recolouring** → one mesh, three factions; the wardrobe is authored once and re-tinted, keeping the file small and colours consistent.

---

## 9. Inspirations (the DNA)

| Source | What we take |
|---|---|
| **Foxhole** | Overhead tactical camera, scale, mouse-aim into 3D |
| **C&C Renegade / Renegade X** | Base-assault FPS economy: buy classes at a terminal, power-plant pricing, destroy-the-HQ win |
| **Red Alert 3** | Bright vibrant aesthetic, R/P/S unit triangle, three-faction split (Allied / Soviet / Empire) |
| **Gears of War** | Gun feel: weight + a per-gun mechanic; **active reload** (planned); melee built into rifles (bayonet charge) |
| **Command & Conquer (Kane's Wrath)** | Deep archetype pool for the planned roster (see UNITS.md) |

---

## 10. Where it's going (summary — full detail in ROADMAP.md)

- **Recently landed (2026-09-25):** colour-true output + the environment brightness gain, contact ink, the populated village map, working save/load under `file://` (rank, maps, prefabs, tutorial), and a headless preview/verify toolkit that lets Claude render the real build.
- **Next — graphics:** comic cast shadows (cast shadows on the same hard 2-band ramp) → painted ground (colour patches, baked contact shadows, dirt roads) → **"Comic Impact"** juice (bright hit flash, hit-stop on kills, weighted shake, cel explosions, muzzle stars, impact frames).
- **Then:** real architecture for the base buildings (still placeholder boxes), fixing the candy-stripe farmhouse, more regional building packs; a review of the older technical restrictions with Mustafa.
- **Roster:** infantry guns **tier by tier** (T1 basics → T4 commandos/exotics) per UNITS.md — held until the progression design (Identity / Momentum / Mastery) is decided; richer enemy AI (shield / molotov / officer behaviours, plus pathfinding); then aircraft → **multiplayer (Phase 6)** on the deterministic SIM core.

---

## 11. How Mustafa works (collaboration notes — please honour these)

- **Fix bugs before adding features.** Root-cause via code inspection, not guesswork.
- **One change-set at a time**, then verify.
- **Mustafa playtests every build and reports back with screenshots** — the final gate. Claude renders headless before/after previews of the 3D scene (`preview.js`) for every visual change, but can't see the HUD/DOM layers or Brave's WebGL2 output, so the screenshot loop still closes every visual question.
- **Clean code**: no per-frame allocation, no memory leaks, "industry-level quality." Reuse temp vectors; pool/dispose FX.
- **Decimals use periods.** Native Dutch speaker; Rotterdam.
- **Think *with* him on design** — he invites Claude to **overrule other models and explain the reasoning**. Push back with substance when warranted.
- **Concise reasoning, and honesty about caveats** — flag limitations rather than hiding them.
