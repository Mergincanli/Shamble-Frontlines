# Shamble Frontlines — Module Map

> **Read this right after `START_HERE.md` when starting a chat.** It tells you which file owns what, how the
> pieces depend on each other, and the one bug class that this structure can hide.
> Pair with `TECHNICAL.md` (the how, incl. the §8 verify gate), `GAME_DESIGN.md` (the why),
> `ART_DIRECTION.md` (the locked *Living Comic* look), `ROADMAP.md` (history/plan),
> `UNITS.md` (planned roster). The **source of truth for behaviour is the code**; grep to
> confirm before trusting any doc.

The game is no longer one HTML file. It is **classic multi-`<script>` files that share
globals**, loaded in a fixed order (NOT ES modules — see TECHNICAL §1 for why).

---

## The files

| File | Role |
|---|---|
| **`shamble-core.js`** | **Shared truth / engine.** Config, arsenal, render globals, NPR pipeline, character builder, faction system, building kit, map schema, and the fenced headless SIM math. Loaded by both pages. References nothing above it. |
| **`shamble-game.js`** | **Gameplay + the shared dispatch/boot.** All combat/AI/economy/HUD/FX/audio/vehicles/progression/`update()`, the world-registry arrays, **and** the control layer both pages share (`loop`/`backToMenu`/`deploy`/`setMenuCam`/input listeners) + `bootGame()`. Requires core. |
| **`shamble-editor.js`** | **Authoring tools.** Map editor (gizmos/tools/inspector/panels), Character Lab, analysis overlays, scatter, prefabs, undo/redo, save-load, `edLeaveToMenu()`, `bootEditor()`. Requires game + core. **Loaded by `editor.html` only.** |
| **`index.html`** | **Lean player build.** Loads THREE + core + game. Menu has **Play + Career** only. |
| **`editor.html`** | **Authoring build.** Loads THREE + core + game + editor. Same head/DOM as `index.html` but keeps the **Map Editor** menu button. |

Each file's top comment block is its manifest (Purpose / Provides / Requires / Load order /
Side-effects-on-load) — read it before editing that file.

---

## Dependency direction — **editor → game → core**

```
shamble-editor.js  ── requires ──▶  shamble-game.js  ── requires ──▶  shamble-core.js
   (authoring)                          (gameplay)                        (engine)
```

- **core** references **nothing above it** (no game, no editor symbols).
- **game** references **core** only. Its handful of editor references are all
  **dead-branch-safe** (behind `mode==='editor'`/`'charlab'`) or **`typeof`-guarded**, so
  none execute in the editor-less `index.html`.
- **editor** may reference **game + core** freely.

**Hard rule:** any symbol reachable on `index.html`'s live path must live in **core or game,
never editor** (see the lessons below).

---

## Load order per page

**`index.html`** (players):
```
THREE (r128 UMD, cdnjs)  →  shamble-core.js  →  shamble-game.js
→  inline:  bootGame();  + menu tail ( hudEl hidden; setMenuCam(); loop(); resize listener )
```

**`editor.html`** (authoring):
```
THREE  →  shamble-core.js  →  shamble-game.js  →  shamble-editor.js
→  inline:  bootGame();  bootEditor();  + the same menu tail
```

Execution on load: core's side-effects (renderer/scene/camera/lights/ground) run as core
loads → `bootGame()` builds the world and wires game input → `bootEditor()` (editor page
only) wires the editor → the tail opens the **menu** and starts `loop()`. Both pages open on
the menu; nothing auto-starts a match or the editor.

---

## Where to find X

| Looking for… | File |
|---|---|
| Combat (`fireOnce`/`castShot`), abilities, grenades/charge | **game** |
| Enemy AI, harvesters, vehicles (Juggernaut) | **game** |
| Economy & match rules (`applyRules`/`RULES_*`), progression (`Prog`) | **game** |
| HUD, combat reticle/hitmarker/reload-ring (`placeCursor`), FX (`addImpact`/`addExplosion`), audio (`SFX`) | **game** |
| Movement collision (`resolveObstacles`), per-frame `update(dt)` | **game** |
| World-registry arrays + map build/serialize (`buildMap`/`clearMap`/`serializeMap`; `curMap` holds the built map's name for `serializeMap`) | **game** [verified] |
| **Shared dispatch** — `loop`, `backToMenu`, `deploy`, `setMenuCam`, input listeners, `bootGame` | **game** |
| NPR/cel pipeline (`toonify`/`makeOutlineMat`/`addHull`/`stylizeScene`/`INK`) | **core** |
| Materials, character builder (`buildEntity`/`assembleChar`), faction system (`applyFaction`/`FACTION_PALETTES`) | **core** |
| Building kit (`BK` + `makeTimberHouse`…`makeManorHouse`), map schema (`parseGridToMapDef`/`DM_*`) | **core** |
| Config/tuning (`T`), arsenal (`WEAPONS`/`UNIT_TYPES`/`VEHICLE_TYPES`), env (`applyEnv`/`ENV_DEFAULT`) | **core** |
| The SIM math (`rayAabb`/`rayStatic`/`rayStatics`/`resolveShot`/`hasLOS`/`SIM`/`FIXED_DT`) | **core** (fenced) [verified] |
| SIM bridge: `syncSimWorld`, `SIM_PLAYER`, `_botFireWorld`, `_eHitN` (bot-fire hit world + impact normal) | **game** [verified] |
| Persistence (`KVStore`: window.storage → localStorage → memory) | **core** |
| Ink hull directions (`ensureOutlineNormals`), colour output + nature gain (`_natureGain`) | **core** |
| Map data incl. the village (`DM_Cabana.houses`/`.trees`/`.props`/`.covers`) | **core** |
| Tree / prop kit (`BK.buildTree`/`BK.buildProp`) — defined inside `bootGame` | **game** |
| Soldier animation (`updateSoldierAnim`, `ANIM`, `animHit`/`animThrow`/`deathStart`/`animDeath`), held guns (`setHeldGun`), fire origin (`playerFireOrigin`) | **game** |
| Gun models + sockets (`GUN_KIT`, `buildGunModel`) | **core** |
| Anim Lab (Character Lab tab: `alTick`, `AL_ACTIONS`) | **editor** |
| Editor UI/gizmos/tools/inspector, building-maker, scatter, overlays | **editor** |
| Character Lab (`clRebuild`/`clAssembled`), save-load, `bootEditor`, `edLeaveToMenu` | **editor** |

(Grep targets per system: TECHNICAL §11.)

---

## Shared-global surface (who owns lifecycle)

- **Render globals — core.** `scene` / `camera` / `renderer` / `sun` / `hemiLight` /
  `ambLight` / `ground` / `sea` / `beach` are created by core **on load**. Everything draws
  into these.
- **World-registry arrays — game.** `obstacles` / `houses` / `covers` / `trees` / `props` /
  `paths` / `platforms` / `ramps` / `buildings` / `barracksList` / `enemies` / `harvesters` /
  `javelins` / `enemySpawns` are **game** globals; their lifecycle is owned by
  `buildMap` / `clearMap` (build populates, clear disposes). The editor reads/writes them
  through game functions.
- **SIM math — core, fenced.** `rayAabb` / `rayStatic` / `rayStatics` / `raySphere` / `rayCapsule` / `resolveShot` /
  `hasLOS` / `SIM` / `FIXED_DT` / `BODY` / `PART` sit contiguously at the bottom of core for a
  future headless lift-out (Phase 6). **Core is NOT headless-runnable today** — it owns the
  render globals; only the fenced block is pure. Lifting it out is future work, not a claim
  about today.
- **SIM ids — game.** `SIM.players[0]` is the player (id `SIM_PLAYER`=0, declared in `shamble-game.js`);
  `SIM.players[i+1]` mirrors `enemies[i]` (id i+1). Player-side shots pass `SIM_PLAYER`; readers map a hit back with
  `enemies[r.id-1]`; a death sets the victim's entry `alive=false` at once. [verified]
- **SIM sync guard — game.** `syncSimWorld` re-syncs at most once per `update()` (unless called with `force`, as the editor's cover-map overlay (`ovBuildCover`) does): `update()` does `_frameId++` at its top
  and `loop()` also increments it, so headless runs see live positions. [verified]

---

## Tools (not part of the build)

`gate.mjs`, `battery.js`, `preview.js`, `pageshot.js`, `sim.js`, `aabbs.js`, `village.py` live in `tools/` and are
**never loaded by either page**. They load the real `shamble-core.js` + `shamble-game.js` into
Node (headless WebGL) to render frames, run the verify battery's checks 3–4, simulate a match,
and generate/validate map layouts. Setup + commands: `START_HERE.md` §5.

---

## Two lessons (so they don't recur)

**1. Bucketing is by REFERENCE, not by LOCATION.** A symbol belongs in the file whose concern
it serves and that references it at load — **not** the section it physically sits in.
*Cautionary tale:* `placeCursor` (the combat reticle / hitmarker / reload-ring positioner)
sat late in the original file, inside the editor block, so a location-based split bucketed it
into `shamble-editor.js`. But `loop()` (game) calls it **every frame, unconditionally**, so
the editor-less `index.html` threw a `ReferenceError` on frame 1 and the render loop died. It
was moved to `shamble-game.js` where it belongs. **When adding or moving code, follow the
references, not the neighbourhood — and keep an override list for any game symbol that happens
to live late in the file.**

**2. The player-build self-sufficiency check exists to catch exactly that.** TECHNICAL §8's
fourth check reconstructs `index.html`'s symbol set (core ∪ game ∪ index-inline), strips the
dead `mode==='editor'`/`'charlab'` branches, and confirms every **unconditional** reference on
`index.html`'s live paths (`loop`, `bootGame`, `deploy`, the tail) resolves within that set —
**nothing may resolve only to `editor.js`.** Run it after any multi-file edit. Note the
line-level "is there a `mode===` guard on this line" check is **not** enough — `loop()` is a
dense one-liner with `mode===` elsewhere on it, which is why `placeCursor` slipped past a
weaker scan.
