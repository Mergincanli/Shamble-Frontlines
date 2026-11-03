# SHAMBLE FRONTLINES — Art Direction Spec: "LIVING COMIC"

**Status:** LOCKED (2026-07-08) and **implemented** (§9 steps 1–5). **Updated 2026-09-25:** colour-true output + the environment gain (§4, §4b) and contact ink (§3). This is the authoritative visual target for the game.
**Source of truth:** the interactive `artstyle-preview.html` "Living Comic" style, as approved by Mustafa, translated onto the game's real NPR pipeline in `shamble-core.js`.
**Navigation:** references use function/constant NAMES (grep targets), not line numbers, because line numbers shift. Use the TECHNICAL.md §11 cheat-sheet to locate each.

---

## 0. The locked decision (one screen)

| Knob | Locked value |
|---|---|
| Style | **Living Comic** (flat high-contrast cel + bold ink) |
| Halftone dots | **OFF by default** (control exists, strength defaults to 0) |
| World outline | **Living Comic bold** (thick) |
| Character / unit / weapon outline | **thin** — Illustrated weight, `charOutline = 0.8x` |
| Character + world ink colour | **shared dark ink** `0x11121a` (only *thickness* differs) |
| Contact ink | **on** — every object is inked where it meets the ground (§3) |
| Colour output | **linear** — the authored hex *is* the on-screen colour (§4b) |
| Lighting balance | buildings + characters: sun 0.8 / hemi 0.32 / amb 0.12 (§4) |
| Environment brightness | **nature gain 1.56** on grass, foliage, hedges, sea (§4b) |

Everything below is the concrete recipe to hit that.

---

## 1. What "Living Comic" is, and why it won

Flat, saturated albedos read through a **2-band** cel ramp (hard light/shadow, no soft mid-band), a **bold near-black ink** on the world, a **bright white key light with low, cool fill** so shadows land as real, defined shadows rather than grey wash, and a punchy blue sky. Metal keeps a crisp bright specular band so weapons pop.

It won because: (a) the flat high-contrast bands make each building assert itself as a solid shape instead of receding into background — which is exactly what the detailed Normandy architecture needs; (b) the shadows read as shadows; (c) the environment is the most legible of the three to the human eye.

**Honest caveat (written when nobody could see the render; since 2026-09-25 Claude renders headless previews, and Mustafa's Brave screenshots stay final):** the preview validated this look on *simple* geometry under an orbit camera. The game has denser geometry and a *top-down* camera (objects sit smaller on screen), so absolute outline thicknesses and the flatness of 2-band on faces may read differently. Every value marked **[dial]** below is a justified *starting point* to confirm against a screenshot, not a final number.

---

## 2. Cel ramps (the bands)

**Where:** `_toonGrad`, `_groundGrad`, `_skinGrad`, `_clothGrad`, `_metalGrad`, helper `_mkGrad(arr)`, selector `gradForKind(kind)`. Ramps are `DataTexture`s of RGB triplets with `NearestFilter` (hard bands). *(Implemented: `_mkGrad` uses `arr.length/3`, and `applyEnv` writes the ground's lit band into triplet 2 — values below verified against the build 2026-09-25.)*

```js
function _mkGrad(arr){ const n=arr.length/3; const t=new THREE.DataTexture(new Uint8Array(arr),n,1,THREE.RGBFormat);
  t.magFilter=THREE.NearestFilter; t.minFilter=THREE.NearestFilter; t.needsUpdate=true; return t; }
```

Target ramps (preview `ramp([80,240])` neutral + `ramp([48,86,150,255])` metal, plus warmth retained on skin/cloth):

| Ramp | Bands | Values (RGB triplets) | Notes |
|---|---|---|---|
| `_toonGrad` (default) | **2** | `80,80,80, 240,240,240` | flat punchy; shadow 0.31, light 0.94 |
| `_clothGrad` | **2** | `96,92,86, 252,249,244` | bright top keeps saturated uniforms from muddying |
| `_skinGrad` | **2** | `150,96,84, 255,244,230` | keep warm shadow (fake SSS); keep the Fresnel rim (`attachSkinRim`, `RIM_*`) |
| `_metalGrad` | **4** | `48,48,48, 86,86,86, 150,150,150, 255,255,255` | the *exception* — hard bright top band = cel "specular" so weapons/steel pop |
| `_groundGrad` | **2** | `80,80,80, <litCap>,<litCap>,<litCap>` | see litCap note below |

**litCap wiring:** `applyEnv` writes the ground's lit band via `gd[6]=gd[7]=gd[8]=litCap` — that index targets the **3rd** triplet. After `_groundGrad` becomes 2-band, the lit band is the **2nd** triplet, so change that write to `gd[3]=gd[4]=gd[5]=litCap`. (Grep `litCap` — it appears in `applyEnv` and `ENV_DEFAULT`.)

**Flatness risk on faces [dial]:** 2-band can flatten facial form. Keep skin's warm rim, and if faces read too flat in a screenshot, the cheapest fix is to re-add a single mid-band to `_skinGrad` only (`150,96,84, 214,168,150, 255,244,230`) — skin becomes 3-band while the rest of the world stays 2-band. Don't pre-emptively do this; confirm first.

---

## 3. Ink outlines — the differential system

This is the headline feature: **bold world ink, thin character ink, same colour.**

**Colour.** Set the global `INK` from `0x1d3557` (navy) to **`0x11121a`** (bold near-black). `makeOutlineMat` reads `INK`, so world *and* character hulls inherit it. Only thickness differs — exactly the locked decision.

**Thickness split (implemented).** Hulls are added by `addHull(mesh, thickness)`. World objects use `WORLD_OUTLINE = 0.09` (bold); character parts always carry `userData.part`, so `addHull` multiplies their weight by `CHAR_OUTLINE = 0.8` (thin: authored per-part `ow` ~`0.028` → ~`0.022`, measured median 0.024). Weapons live under the character group and inherit the thin weight. Both constants sit at the top of the NPR block for live dialing.

**How the hull is drawn (2026-09-25).** The hull shader offsets each vertex in **world units** along a **smoothed, mitred outline direction** (`ensureOutlineNormals`) — so line weight ignores object/part scale and hard corners close — and **clamps the shell just above the ground** (`OUTLINE_GROUND_Y` 0.02). The clamped bottom face becomes an ink rim around every footprint: the **contact line**. Before this, inverted hulls dipped below the grass and objects had no line where they met the ground, which made them look pasted on. Line weights were measured unchanged by the switch.

**Decided — character outline colour:** characters use the shared **dark** ink (`applyFaction` leaves outlines at `INK`); allegiance reads from the uniform, not the line. The one exception is the Empire villain look (§7), which no bot wears since 3a (reserved for a future hero unit).

---

## 4. Light, sky & atmosphere

**Where:** `ENV_DEFAULT` is the authoritative preset — `applyEnv` mutates the live scene from it and overrides the init-time light literals, so the look lives in `ENV_DEFAULT` (editor-tunable in the World panel).

The Living Comic mood = **pure white key, low cool fill** so shadows read as real shadows, with a violet bounce. **Implemented values (verified 2026-09-25):**

| `ENV_DEFAULT` field | Value | Why |
|---|---|---|
| `sun.color` / `sun.int` | `#ffffff` / **0.8** | pure white comic key; with linear output, 0.94 (lit band) × 0.8 + fill ≈ 1.0 → the lit band shows the authored colour |
| `sun.az` / `sun.el` | `-52.6` / `49.4` | shadow direction |
| `hemi.sky` / `hemi.ground` / `hemi.int` | `#a0c8ff` / `#4a3a66` / **0.32** | cool skylight + violet bounce; low fill → defined shadows |
| `amb.color` / `amb.int` | `#303055` / **0.12** | dark blue-violet, low |
| `nature.gain` | **1.56** | environment-only brightness (§4b) |
| `sky.bg` | `#8fd0f0` | comic blue |
| `fog.color` / `near` / `far` | `#bfe4f5` / 260 / 760 | comic horizon; keep the game's distances (the preview's 70/320 would fog the whole map) |

**History:** the first implementation used sun 1.35 / hemi 0.4 / amb 0.18 — tuned, unknowingly, against a double-brightened output (§4b). After the colour-truth fix Mustafa compared renders and chose the richer "option C" balance for buildings and characters, with the environment kept at its brighter look via the nature gain.

**Ground albedo** (`ENV_DEFAULT.ground`, painted by `generateGrassTexture`): `base #4aab24`, `detailDark #3f9e1e`, `detailLight #5fbf2e`, `litCap 195`; density/contrast/patch/tiling editor-tunable.

---

## 4b. Colour truth + the environment gain (2026-09-25)

- **Colour truth.** The renderer outputs **`LinearEncoding`**. Three.js r128 never converts input colours, so the earlier `sRGBEncoding` output brightened every authored hex and every canvas texture twice: grass authored `#4aab24` rendered as `#a5f17d`, roads authored grey rendered near-white, and the 2-band shadow (31 %) displayed at ~59 % — exactly the grey wash this spec forbids. Now **the palette in this document is what's on screen** in the lit band. Don't switch back without converting every input colour and texture.
- **Environment gain.** `ENV_DEFAULT.nature.gain` multiplies only the environment — the grass plane, and every material tagged `userData.nature` (tree bark + foliage, hedge foliage + earth bank, the sea) — so the world stays **lush and colourful** while buildings and characters keep the richer lighting. It's the lever for the future "lush environment" direction (World panel: "Nature brightness"). New nature pieces: tag their source material `userData.nature = true`; `toonify` and `applyEnv` do the rest.
- **Measured:** grass under the gain matches the chosen bright look within one 8-bit step; a crate matches the chosen building look exactly.

---

## 5. Halftone dots — OFF, but with a real toggle

Locked **off** (strength 0), but the control must exist and be tunable, as validated in the preview.

**Constraint-safe implementation.** No `EffectComposer`/post-processing (hard rule). Do it as a **CSS multiply-blend overlay `<div>` over the game canvas** — the same technique the preview used, which is a DOM element, not a render pass:

```
#halftone { position:fixed; inset:0; pointer-events:none; z-index:<below HUD>;
            mix-blend-mode:multiply; display:none; /* strength 0 = display:none */
            background-image: radial-gradient(circle, rgba(20,20,40,.9) <r>px, transparent <r2>px);
            background-size: <size>px <size>px; }
```

- Globals `htOpacity = 0` (off) and `htSize = 5`, plus `updateHalftone()` that sets `opacity`/`display`/`backgroundSize`/dot radius (`r = size*0.18`, `r2 = size*0.3`), mirroring the preview.
- Expose strength (0–0.45) + size (3–14) wherever dev/world tuning lives (the editor World panel is the natural home).
- **Caveat:** a screen-space CSS overlay also dots the HUD/UI. Acceptable while it defaults off. If halftone is ever promoted to on-by-default and the HUD dotting bothers us, the upgrade path is a manual full-screen quad reading a luminance target (scene-only, dark areas get more dots) — but that flirts with the post-processing line, so it's explicitly **out of scope now**.

---

## 6. Surface texture recipes (for the architecture, when we build it)

**Status (2026-09-25):** built and placed in the shipped village — timber house, thatched cottage, brick-and-flint farmhouse (its red/white banding reads as "candy stripes" — to fix), Norman manor (round tower + conical roof), plus the `BK` kit-of-parts (one region preset: `normandy`). The base buildings (HQ / barracks / refinery / power plant) are still placeholder boxes. The recipe below stays the target. All textures: **256x256 canvas, `NearestFilter`, `RepeatWrapping`**, painted flat then given comic detail, tiled per surface. Approach mirrors the existing `generateGrassTexture` (flat base albedo + layered blobs/strokes for variation) and the preview's `drawTimber/drawStone/drawRoof/drawThatch/drawDirt`.

Comic palette (from the approved preview `comic.pal`):

| Surface | Albedo | Recipe (flat base + comic detail) |
|---|---|---|
| Grass | `0x5fbf2e` top / `0x3f9e1e` blade | see §4 ground |
| Dirt / road | `0xb0803f` | base fill + darker gravel speckle, soft edge blend |
| Timber (plaster wall) | plaster `0xf0e2b6`, beams `0x342414` | cream plaster fill + **dark near-black beam frame** painted as thick strokes (the Fachwerk grid); beams get their own bold ink edge from geometry |
| Stone | `0xe8cf78` (Caen golden limestone) | warm limestone fill + block seams as thin darker lines + subtle 2-tone block variation |
| Roof (tile) | `0xd0431f` (terracotta) | saturated tile fill + horizontal course lines, slightly darker every 3rd course |
| Thatch | `0xd8a838` | straw-gold fill + vertical straw strokes, darker at the eave; fat geometry gable |
| Wood (door/trim) | `0x8a5a28` | plank fill + vertical plank seams |
| Steel (weapons) | `0xccd4dc` | flat; the 4-band `_metalGrad` does the specular work |

Building geometry cues to keep the world "asserting itself": generous wall thickness, fat gable roofs (steep pitch), and the **conical witch-hat tower** on Norman-manor HQs (preview `normanManor`). Windows/doors as inset flat accents with the bold world ink.

---

## 7. Empire villain material (Wolfenstein menace)

**Implemented in code, not spawned (D6, three-teams 3a, 2026-10-16):** the 200 HP Empire elite no longer spawns; regular Empire bots wear the standard soldier in the Empire palette. The villain look (`applyVillain`, `ELITE_KIT`) stays in the code, reserved for a future hero unit or Tier 3 unit; the Character Lab's 'EmpireElite' preview still uses it. [verified: shamble-game.js:3211-3212, 125, 3364; shamble-editor.js:288]

The look itself (values unchanged, still locked): **near-black lacquered armour + a single bright specular cel band + a glowing visor.** Reads as sinister against the bright world.

- **Lacquer:** near-black blue-violet albedo; ramp `_lacquerGrad` = `18,18,26, 30,30,42, 120,120,140, 235,235,245` (mostly dark, one sharp wet-lacquer glint band), selected by `matKind:'lacquer'`.
- **Visor:** emissive **violet `0xc24bff`** (the original spec said red; violet was chosen to match the Empire palette). `toonify` preserves emissive, so it survives the cel conversion. No bloom — an additive sprite behind the visor is the upgrade path if it needs more punch [dial].
- **Ink exception:** the elite's outline is Empire **purple `0x7B2FD6`**, not the dark ink — near-black armour with dark ink would vanish into its own silhouette.

---

## 8. Faction palettes & value targets

**Where:** `FACTION_PALETTES` + the role system (`applyFaction`, and `mats` in `buildEntity`: roles `base`/`primary`/`trim`/`coat`(dead)/`pad`). 70/20/10 = `BaseGear` (dark grounding, ~70%) / `FactionPrimary` (bold identity, ~20%) / `HighlightTrim` (bright accent, ~10%), with `PadColor` for the pauldrons.

**Readability rule:** against the brighter Living Comic grass (~`#4aab24`), keep **BaseGear dark** (high-contrast silhouette), make **FactionPrimary a mid-bright pop**, keep **HighlightTrim brightest**. Rough luminance targets: BaseGear ~15–25%, FactionPrimary ~35–55%, HighlightTrim ~85–95%.

**Brighten FactionPrimary toward the preview's punch [dial]** (base gear already grounds the silhouette, so the 20% primary can afford to pop):

| Faction | Field | Current | Target [dial] |
|---|---|---|---|
| Soviets | FactionPrimary | `0xC1121F` | `0xE11020` (preview red) → **implemented `0xD41414`** (also PadColor + CoatAccent) |
| Allies | FactionPrimary | `0x003F88` | `0x1F6FF0` (preview blue — big jump; navy base still grounds) |
| Empire | FactionPrimary | `0x5A189A` | `0x7B2FD6` (brighter purple) *— unless Empire = villain lacquer* |

BaseGear (Soviet `0x432818`, Allies `0x00296B`, Empire `0x240046`) and HighlightTrim (`0xFDF0D5` / `0xFDC500` / `0xFF6D00`) are as specified; Allies and Empire primaries are the targets above; PadColor is Soviet `0xD41414` / Allies `0xFDC500` / Empire `0xFF6D00`. *(Verified against `FACTION_PALETTES`, 2026-09-25.)*

---

## 9. Implementation order & verification

1. **Base look transform** (2-band ramps, `ENV_DEFAULT`, ground greens, `INK`) — **done**.
2. **Differential outlines** (`WORLD_OUTLINE` / `CHAR_OUTLINE`) — **done**.
3. **Faction primary brightening + dark character ink** — **done** (Soviet later retuned to `0xD41414`).
4. **Empire villain material** — **done** in code (`applyVillain`); the elite unit no longer spawns since 3a (D6), the look is reserved for a future hero unit.
5. **Halftone plumbing**, default off — **done**.
6. **Architecture** (§6) — **in progress**: four house types + the kit are placed in the village; base buildings still placeholder.
7. **Contact ink** (smoothed + mitred, world-space, ground-clamped hulls) — **done** 2026-09-25.
8. **Colour truth + environment gain** (§4b) — **done** 2026-09-25, chosen from rendered options.
9. **Next:** comic cast shadows (cast shadows on the same hard 2-band ramp, not soft PCF gradients) → painted ground (macro colour patches, baked contact shadows, dirt roads).

**Verification now:** every visual step gets a headless before/after render (`preview.js`) and then Mustafa's Brave screenshot. **Live-tuning constants** stay grouped at the top of the NPR block: `INK`, `WORLD_OUTLINE`, `CHAR_OUTLINE`, `OUTLINE_GROUND_Y`, `htOpacity`, `htSize`, the ramp arrays; `ENV_DEFAULT` (incl. `nature.gain`) is editor-tunable. **Preservation invariants:** the headless SIM core is untouched by visual work; runtime-spawned objects go through `stylizeOne`; no per-frame allocation; hulls stay `noRay` / non-shadow.

---

## 10. Decisions (all resolved)

1. **Empire villain → a separate elite unit type** (normal Empire troops keep the purple uniform). Built; no longer spawned since 3a (D6), kept for a possible hero unit.
2. **Character outline colour → shared dark ink**, with the purple exception for the villain look. Implemented in code (no bot wears it since 3a).
3. **Faction primaries → brightened**; Soviet later retuned to `0xD41414`. Implemented.
4. **Colour balance (2026-09-25) → option C for buildings + characters, the bright look for the environment** via the nature gain. Implemented.
