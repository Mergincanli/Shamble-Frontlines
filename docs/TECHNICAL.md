# Shamble Frontlines — Technical Reference

> The **how**. Pair with `GAME_DESIGN.md` (the why), `ROADMAP.md` (history), and **`ART_DIRECTION.md`** (the locked *Living Comic* visual spec — authoritative for palette/ramps/outlines/lighting). The source of truth for behaviour is the **code**, now **split** across `shamble-core.js` → `shamble-game.js` → `shamble-editor.js` and loaded by `index.html` (players) / `editor.html` (authoring). See **`MODULE_MAP.md`** for which file owns what (and §1a below). Navigate with the per-file grep cheat-sheet in §11; **do not reverse-engineer from scratch.**

---

## 1. Stack & hard constraints

- **Classic multi-`<script>` files sharing globals, loaded in a fixed order — NOT ES modules.** The former single file is split into `shamble-core.js` → `shamble-game.js` → `shamble-editor.js`, pulled in via plain `<script src>` tags by two entry pages (`index.html` for players, `editor.html` for authoring). See **§1a** + `MODULE_MAP.md`.
  - **Why not ES modules:** the build must run by **double-clicking the HTML off disk** (`file://`). Browsers block ES-module `import`/`export` under `file://` (CORS/module rules), which would break local play; plain scripts sharing one global scope work everywhere. No bundler or build step either — you edit the files directly.
  - No external assets.
- **Three.js r128**, loaded as a **UMD global** from cdnjs (`THREE.*`). **NO ES modules; no addons** (a project policy — r128's `examples/js` addons are classic scripts and *would* load; the no-addon and no-post-processing rules are queued for review with Mustafa). Consequences:
  - No `GLTFLoader`, no `OrbitControls`, no `EffectComposer`/post-processing passes.
  - No `CapsuleGeometry` (that's r142+). Use Cylinder/Sphere/Box/Cone/Lathe/Extrude/Tube/Torus.
  - Camera and all controls are hand-driven.
- **Vanilla JS** (ES2017-ish). No frameworks.
- **WebAudio**, fully **synthesized** SFX (oscillators + noise buffers) — no audio files.
- **Persistence**: only through **`KVStore`** (core) — `window.storage` → `localStorage` → memory, chosen once at load; never rejects. See §7.
- **Renderer**: `antialias`, pixel ratio capped at 2, `PCFSoftShadowMap` (2048² sun map, player-following frustum), **`LinearEncoding` output** (authored hex = on-screen colour — see §2), **NoToneMapping**.

If a new feature seems to need a module/addon/asset, it almost certainly needs a workaround instead — the no-module / no-addon / no-asset constraints are unchanged by the split.

---

## 1a. File structure (see `MODULE_MAP.md`)

The former single file is split into **three shared-global scripts + two entry pages**; dependency direction is **editor → game → core**:

- **`shamble-core.js`** — engine / shared truth: config (`T`), arsenal, render globals, the NPR pipeline, the character builder + faction system, the building kit, the map schema, and the fenced SIM math. References nothing above it.
- **`shamble-game.js`** — all gameplay **and** the shared dispatch (`loop` / `backToMenu` / `deploy` / `setMenuCam` / input listeners) + `bootGame()`; owns the world-registry arrays. Requires core.
- **`shamble-editor.js`** — authoring tools (map editor + Character Lab) + `bootEditor()` + `edLeaveToMenu()`. Requires game + core. **Loaded only by `editor.html`.**
- **`index.html`** — lean player build (THREE + core + game); menu = **Play + Career**.
- **`editor.html`** — authoring build (+ editor); identical head/DOM but keeps the **Map Editor** menu button.

Layering rule: **core references nothing above it; game references core (its few editor refs are dead-branch / `typeof`-guarded); editor references both.** Any symbol on `index.html`'s live path must live in **core or game, never editor**. `MODULE_MAP.md` is the orientation doc — read it first.

---

## 2. The "Living Comic" NPR pipeline (read before touching materials)

> **Locked look = "Living Comic"** — `ART_DIRECTION.md` is the authoritative recipe (hex, ramps, lighting, outline weights). The mechanics below are verified against the build (2026-09-25).

- **Cel ramps.** `_mkGrad(arr)` builds a Nearest-filtered `DataTexture` of width `arr.length/3`, so 2- and 4-band ramps both work. `_toonGrad` `80,240` (2-band, the default); `_clothGrad` / `_skinGrad` warm 2-band; `_metalGrad` and the Empire-elite `_lacquerGrad` 4-band (a hard bright top band = cel specular); `_groundGrad` `80,<litCap>` (litCap written by `applyEnv` into triplet 2). `gradForKind(kind)` picks the ramp from a part's `matKind`.
- **`toonify(std, kind)`** — builds a `MeshToonMaterial` from a `MeshStandardMaterial`, copying `color`, **`map`** and `emissive`; skin gets the Fresnel rim (`attachSkinRim` — `onBeforeCompile` string-injects `uRimColor/uRimStr/uRimPow`; one source of **GLSL inside JS strings**, see §8). A source material tagged `userData.nature` keeps `userData.baseCol` and is multiplied by the live nature gain (see Colour below).
- **Ink = inverted hull.** `addHull(mesh, t)` adds a child mesh sharing the geometry, drawn `BackSide` with `makeOutlineMat(t)`: world objects use `WORLD_OUTLINE` 0.09 (bold); anything carrying `userData.part` (characters, units, weapons) is multiplied by `CHAR_OUTLINE` 0.8 (thin); both use the shared dark `INK` `0x11121a`. The hull vertex shader:
  - offsets each vertex **in world units** along its **smoothed outline direction** (attribute `oNormal`), so line weight ignores object/part scale;
  - `oNormal` comes from **`ensureOutlineNormals(geo)`** (called by `addHull`): vertices sharing a position get the sum of their *distinct* normals, scaled by 1/min(cos) — a mitre, capped at 2× — so every face is pushed out by the full thickness and hard corners close. Results are cached by geometry *content* (order-sensitive hash), so identical soldier parts are solved once; cache cap 4000 entries;
  - **clamps the shell above the ground** (`OUTLINE_GROUND_Y` 0.02 — above the grass at y=0, below road-decal tops at 0.06): the hull's bottom face flattens into an ink rim around every footprint — the **contact line** where objects meet the ground;
  - falls back to the plain `normal` when `oNormal` is missing (WebGL feeds (0,0,0)) — so a geometry swapped in without `ensureOutlineNormals` still draws, just without closed corners or the contact line. The editor's resize paths call it on the new geometry.
  - Cost: ~33 ms once at boot, ~1 ms per soldier (re)build, nothing per frame (measured headless).
- **`stylizeScene()` runs ONCE at init** (swaps toon materials in + adds hulls; skips flat Plane/Circle/Ring). **Runtime builders call `stylizeOne(obj)` themselves** — houses, props, covers, obstacles, bolts and casings do; the character builder handles its own per-part materials/outlines. A new runtime builder that skips it gets flat, outline-less meshes.
- **Colour pipeline (colour truth).** Output is **`LinearEncoding`**. r128 never converts input colours, so the former `sRGBEncoding` output brightened every authored hex and every canvas texture twice (the washed-out look; shadows lifted to grey). Now the lit band shows the authored colour. `ENV_DEFAULT` lights are the Living Comic "option C" balance — sun 0.8, hemi 0.32, amb 0.12 — which buildings and characters use as-is.
- **Nature gain.** `ENV_DEFAULT.nature.gain` (1.56) brightens only the environment so it stays lush: `applyEnv` sets it on the ground (`ground.material.color.setScalar`) and on every material tagged `userData.nature` with a stored `baseCol` (tree bark + foliage in `BK.buildTree`, hedge foliage + earth bank in `BK.buildCover`, the sea); `toonify` bakes the current gain into nature materials created later. Editor World panel: "Nature brightness".
- **Ground**: flat `MeshBasic` sea (skipped by `stylizeScene`) + a sand `beach` + the grass plane (`GROUND_W×GROUND_D` = 372×252). `generateGrassTexture()` → a 512×512 `CanvasTexture` from `ENV.ground` colours, edge-wrapped to tile, `RepeatWrapping`, toonified with `_groundGrad`. Editor-tunable via `applyEnv`.
- **Lighting / atmosphere** live in **`ENV_DEFAULT`** — `applyEnv` overrides the init-time light literals, so tune there (or in the editor World panel).
- **Halftone** is a CSS multiply-blend overlay (`htOpacity` 0 = off by default, `htSize` 5), not a render pass.

---

## 3. File layout (top → bottom, approximate)

Split across three files now (§1a); each is comment-sectioned. The list below is the historical top-to-bottom order of systems — **file ownership is per `MODULE_MAP.md` / §11**, not this ordering:

1. **CSS** (in `<head>`): HUD, menu (`#title`), editor (`#editor`, `#edTop`, `#edPalette`), Character-Lab panel (`.dp-*` classes), modals, the parchment theme.
2. **HTML body**: canvas mount, HUD overlays, main menu, **editor DOM**, **Character-Lab DOM** (`#charlab`, the `dp-btn` rows), save/export modals.
3. **`<script>`**:
   - **`T`** — the master tuning config (movement, camera, health, economy, grenade, AI ranges…). One object; tweak gameplay here.
   - **`WEAPONS[]`** and **`UNIT_TYPES[]`** — the arsenal + classes (each `WEAPONS` entry is commented with its role; see GAME_DESIGN §7).
   - Scene / camera / renderer / lights; base materials; the **toon pipeline** (`_toonGrad`/`toonify`/`makeOutlineMat`/Fresnel/`stylizeScene`).
   - **The character system** (see §5): `FACTION_PALETTES`, `assembleChar`, `applyFaction`, `buildEntity`, `buildCharacter`, `CHAR_DEFAULT`/`CHAR_MALE`/`CHAR_FEMALE`, `CPARTS`, `poseArms`, `animateSoldier`, plus `makeSword`/`makeAxe`/`makeShield`.
   - **Object factories**: `addBox` (crates/walls → `obstacles[]`), `addPath` (flat decals → `paths[]`), `makeBuilding` (→ `buildings[]`), `makeBarracks` (→ `barracksList[]`), `makeLabel`.
   - **Map data** `DM_Romania` + `buildMap`/`clearMap`/`serializeMap`.
   - **The SIM core** (headless pure-math hit/LOS/collision) — see §4.
   - **Combat**: `fireOnce`, `castShot`, `damageEnemy`, `damagePlayer`, abilities (banzai/bayonet/shield/molotov), grenades/charge system, **FX** (`addImpact`/`addMuzzle`/`addExplosion`/`addCasing` + the single `fx[]` update loop) and **WebAudio** (`SFX`, `sfxShot`/`sfxKill`/`sfxStep`/`sfxClank`/`sfxThunk`).
   - **Player state `ps`**, input handlers (`keydown`/`wheel`/`mouse`), the main **`update(dt)`**, the camera-framing block.
   - **HUD** updates, **progression** (`Prog`, rank tiers, career chip), **shop**, **tutorial**.
   - **The Character Lab** — its own section (see §5.4).
   - **The MAP EDITOR** — its own clearly-marked section near the end (see §6).
   - **`loop()`** — the rAF driver: `update(dt)` in game mode, `updateEditor(dt)` in editor mode, `clRebuild`/lab spin in Lab mode, render.

**Grep note:** HUD text and CSS strings live **outside** the `<script>` — so when grepping to confirm a change, grep the **full HTML**, but for JS syntax checks extract the **last `<script>`** (see §8).

---

## 4. The SIM core (source of truth for combat)

- **Hit detection, line-of-sight, and movement collision run through a headless, pure-math core**, *not* `THREE.Raycaster`. Key pieces: `resolveShot(...)`, `SIM.statics` (mirrored colliders), `rayAabb(...)`, `hasLOS(...)`, `resolveBoxes(pos, r)`.
- `syncSimWorld()` mirrors the THREE scene's colliders into `SIM` before resolving a shot.
- The `THREE.Raycaster` is used **only** for cosmetic things: the aim dot and **editor object-picking**. So editor-only props (e.g. `paths[]`) never affect gameplay shots even though they're raycastable.
- `FIXED_DT = 1/60`; there's an embedded input-command structure. **This was built as Stage-1 multiplayer prep** (deterministic simulation). Networking itself is not implemented yet.
- `resolveBoxes(pos, r)` (AABB push-out) is the player-movement collision — and is **reused by the editor's Tab playtest**.

---

## 5. The character system (the headline system — read fully)

Originally an "Option 1" procedural rigid-primitive rig. It has since been refactored into a **single data-driven builder** that everything routes through.

### 5.1 One builder: `buildEntity(def, opts)`
- **`buildEntity(def, opts)`** is the universal constructor. It walks a **flat `def.parts` array** (each part = `{id, type, parent, mat, tag, ow, geom params, pos/scale/rot, noOutline}`) and assembles the mesh hierarchy, applying materials, per-part **inverted-hull outlines** (via `makeOutlineMat`, unless `noOutline`), and colour options.
  - `type`: `sphere`/`cyl`/`box`/`cone`/`lathe`/`group`/etc. `parent`: another part's `id` (or a joint name) to nest under.
  - `opts`: `{faction, skin, hair, eyes, base, accent}`.
- **`buildCharacter(color, accent, def)`** is now a **thin compatibility shim**: `return buildEntity(def || CHAR_DEFAULT, {base:color, accent:accent})`. Old call sites keep working.
- **Gameplay spawns** go through **`buildSoldier(faction, sex, kit)`** (game): the player is `buildSoldier('Allies')`, enemies are dressed per faction from `SOLDIER_KIT`, the Empire elite uses `ELITE_KIT` + `applyVillain`. **[verified 2026-09-25]**
- **The Character Lab** calls **`buildEntity(clAssembled(), {faction, skin, hair, eyes})`** — a full composed recipe.
- The wardrobe + faction system shows on gameplay units and in the Character Lab alike (same `buildEntity` + `assembleChar` + `applyFaction` plumbing).

### 5.2 The material-**role** system (how faction recolouring works)
Parts declare a `mat` (material key). Buildable materials in the `mats` object: `base`, `primary`, `trim`, `skin`, `hair`, `eyes`, `steel`, `dark`, `leather` (fixed dark brown `0x2e2117`, faction-neutral), `accent`, `coat`, `pad`.
- **`matKind`**: `skin`→'skin', `steel`→'metal', `base/primary/trim/leather/coat/pad`→'cloth', else 'default'.
- **Role tag**: parts with `mat` in `{base, primary, trim, coat, pad}` get a `userData.role`; `leather` does **not** (stays brown across factions).
- **`FACTION_PALETTES`** (Soviet / Allies / Empire) each define `BaseGear`, `FactionPrimary`, `HighlightTrim`, `CoatAccent`, `PadColor` (values in GAME_DESIGN §7.5).
- **`applyFaction(group, faction)`** walks the group and recolours every role-tagged mesh: `base→BaseGear`, `primary→FactionPrimary`, `coat→CoatAccent`, `pad→PadColor`, `trim`/else→`HighlightTrim`. **Living Comic change (locked):** character **outlines stay at the shared dark `INK`** — `applyFaction` must **not** tint them to `FactionPrimary` (allegiance reads from the uniform albedo, not the line). **Exception:** the Empire **elite villain** sets its outline to Empire purple (`0x7B2FD6`) so its silhouette reads against near-black lacquered armour (ART_DIRECTION §7).
- **Design rule learned:** when a colour must diverge across all three factions and no existing role fits, add a **new palette slot + role** (this is exactly why `PadColor`/`pad` was added for pauldrons). The `coat`/`CoatAccent` slot is currently **unused** (the greatcoat was re-roled to base/primary) but left in place — harmless, available for future per-region accents.

### 5.3 Base bodies + `CPARTS` + `assembleChar`
- **`CHAR_DEFAULT`, `CHAR_MALE`, `CHAR_FEMALE`** are `def` objects (`{parts:[...], colors:{...}}`). Male vs female differ in proportions (female is slighter; different head centre/eyes). Both carry **eyes, subtle eyebrows (mat `hair`), and a small nose (mat `skin`, no outline)** baked in.
- **`CPARTS`** is the wardrobe registry: `{ headgear:{none,brodie,m1,cap}, torso:{none,tunic,greatcoat}, legs:{none,trousers}, feet:{none,boots}, webbing:{none,webbing}, hair:{default,none,crop,swept,sidepart,bun,ponytail,mohawk}, face:{none,stubble,mustache,goatee,beard} }`. Each style is a **function `(sex) => [parts]`** sized per body via a `const f=(sex==='f')` ternary pattern.
- **`assembleChar(sel)`** composes a full recipe: clones the base body (`CHAR_MALE`/`CHAR_FEMALE`), applies a **hide filter**, then appends each equipped slot's parts. Hide/skip rules:
  - torso≠none → hide the base `band`; legs≠none → hide base `shorts`.
  - headgear≠none → hide base `hair`+`ponytail`; **and** the **hair slot is skipped** (a helmet covers hair).
  - hair≠`default` → hide base `hair`+`ponytail` (so a chosen style, or `none`=bald, replaces the default).
  - the **face slot is never skipped** (facial hair shows under a helmet).
  - Units/recipes with no hair selection keep the default base hair (gameplay spawns are unaffected).

### 5.4 The Character-Lab UI
State: `charLabSex`, `charLabFaction`, `charLabSkin/Hair/Eyes`, `charLabDef` (the editable base def, for proportions), and **`charLabSlots`** = `{headgear, torso, legs, feet, webbing, hair:'default', face:'none'}`.
- `clAssembled()` mirrors `assembleChar` but reads `charLabSlots`; `clRebuild()` rebuilds `charLabPreview` via `buildEntity`.
- `clCycleSlot(slot, ids)` advances a slot; `clBuildPanel()` refreshes the button labels; per-slot label setters (`_hg/_tn/_lg/_ft/_wb/_hr/_fc`).
- Proportions sliders edit `charLabDef.parts` geometry; colour pickers set skin/hair/eyes; `clSetFaction` re-applies faction; Export/Import serialise the recipe as JSON; `clReset`/`clResetProps`/`clResetColors`; Spin toggle.
- Buttons are wired in `clBindUI` via a `_b(id, fn)` helper.

### 5.5 Rig animation — ANIM v2 (see `ANIMATION_PLAN.md`)
- **`updateSoldierAnim(g, st, dt)`** (game) drives every soldier: planted feet with leg IK to a procedural ankle, stride sized from real leg reach, hips following movement within a twist budget, gun-first holds from `GUN_KIT` sockets (core) with reach-aware two-hand IK, per-gun reloads, shield / katana on IK, hit flinch, greatcoat panels. **Cosmetic only**: the player fires from `playerFireOrigin()`, bots from a fixed chest point. **[verified]**
- Actions: `animHit`, `animThrow`, `deathStart` + `animDeath` (the toy-soldier fall; the root never tilts or sinks). Held models: `setHeldGun`. **[verified]**
- Tuning lives in `ANIM` (game); the editor's **Anim Lab** (Character Lab) loops any action with live sliders. Legacy: `applyRifleHold` / `ANIM.aim` serve only the Character Lab pose tool.

---

## 6. The map editor (overhauled into a sandbox)

Its own section near the bottom of the script. State of note: `edTool`, `edMulti[]` (multi-selection), `edSel`/`edGrab`, `edSelBoxes[]` (a `BoxHelper` pool), `edSnap` (1 or 5), `edMarquee`, `edPathStart`, `edPreview`, `edTargets` (curated placeable list), plus the gizmo/handle state (`_edGizmo`, `_edHandles`).

Features (all live):
- **Grid snapping** — `edToggleSnap()` (button or `G`); `edGround()` rounds placement/drag to the snap unit (5 m).
- **45° rotation** — `Shift + Wheel` → `edRotateSel(dir)` (per-kind; persists in save/export). Resupply supports rotation too.
- **Translate gizmo** — for a single selection, three axis arrows (X red, Y green = up/down, Z blue) with plane-based drag (`_edEnsureGizmo`/`edUpdateGizmo`/`edMoveYBy`); ground-drag still works.
- **Functional resize handles** — 8 corner cubes (`_edHandles`) are raycastable and drag-resize the footprint about the centre (`_edResizeFootprint` → per-kind `edResize`/`edReshapeRamp`/`edResizeResupply`/`edScaleSel`).
- **Path/Road decals** — `addPath()` makes a thin grey box just above the grass with **no collision** (`paths[]`, not `obstacles[]`); the `path` tool is **drag-to-draw**.
- **Marquee select** — drag empty ground → `edMarqueePick()` projects each object's world anchor to screen space and tests the box (deduped by object reference).
- **Group duplicate** — `Ctrl + D` → `edDuplicate()` clones the whole selection offset and keeps it selected.
- **Tab playtest** — `edEnterPreview`/`edPreviewUpdate` drops the player at the camera target with a low over-the-shoulder cam, WASD + mouse-look + Shift-run, real `resolveBoxes` collision; `Tab`/`Esc` returns.
- **Mousedown pick priority:** gizmo > handle > object > marquee. Gizmo/handles hidden in preview/editorPlay/backToMenu.
- **Persistence**: `serializeMap`/`buildMap` round-trip obstacles (with rotation + Y), **paths**, buildings, barracks, spawn, resupply, and the kit pieces — `houses` (legacy makers by `bt`, or `{k:'kit', spec}`), `trees`, `props`, `covers`. Save/Load via `MapStore` (`vmap:<name>`), prefabs via `PrefabStore` (`vprefab:<name>`), both through `KVStore`; Export dumps JSON. **Reset** rebuilds `DM_Cabana`, which now carries the village.
- **World panel** drives `applyEnv` live, incl. **Nature brightness** (`nature.gain`).

---

## 7. Persistence keys

Everything persists through **`KVStore`** (core) — an async key-value API shaped like `window.storage` (`get(k)` → `{key,value}|null`, `set`, `delete`, `list(prefix)` → `{keys,prefix}`). It picks a backend **once at load** — `window.storage` (claude.ai artifact runtime) → `localStorage` (`file://` in Brave; probed read-only) → memory — and **never rejects** (failures resolve to `null`, so un-awaited saves can't raise unhandled rejections; a `localStorage` write that fails, e.g. quota, is held in memory for the session). `KVStore.persist` is false only on the memory backend.

| Key | Store (file) | Notes |
|---|---|---|
| `shamble_prog` | `ProgStore` (game) | rank/level/prestige; falls back to legacy `vanguard_prog` |
| `shamble_tutorial_seen` | `TutStore` (game) | falls back to `vanguard_tutorial_seen`; ungated, so a dismissal is remembered for the session even on the memory backend |
| `vmap:<name>` | `MapStore` (editor) | saved maps; note reads "(saved to browser storage)" when persistent |
| `vprefab:<name>` | `PrefabStore` (editor) | saved prefabs |
| `shamble_charlab` | Character Lab export (editor) | last exported recipe (write-only backup) |

- Under `file://`, Chromium/Brave give **all local files one shared storage area** — `index.html` and `editor.html` share one rank.
- **Key-rename rule:** renaming a key requires fallback-read logic so existing saves survive.
- Tested: 26-assertion Node harness over the real store code across all three backends plus a failing backend (see §8).

---

## 8. Edit / verify workflow (how to safely modify this file — follow it every time)

1. **Targeted `str_replace`** for single edits. For multiple related edits, batch them in a small **Python heredoc** that, for each change, asserts the old substring's `count == 1` (use a counted helper for intended multi-hits) *then* replaces — this catches ambiguous/duplicate matches early. Match whitespace exactly. Always `cp` the file to a `/tmp` backup first.
2. **Verify EVERY change — the four-check battery (run ALL four after any multi-file edit; this is the standing gate):**
   1. **Per-file `node --check`** — the AUTHORITATIVE syntax gate. Check each `.js` file, and extract **each HTML page's inline `<script>`** (`re.findall(r'<script\b[^>]*>(.*?)</script>', html, re.S)[-1]` → temp `.js`) and check that too.
   2. **Concatenation `node --check`** — `cat` the files in **load order** into one temp file and check it: `core [+ game] [+ editor] [+ page-inline]`, for **both** pages. Catches cross-file **`const`/`let`/`class` redeclaration collisions** that per-file checks cannot see.
   3. **Symbol-conservation diff** — the union of top-level symbols across all files must equal the **pre-edit baseline**: **no drops** (accidentally deleted code) and **no cross-file duplicates** (same symbol defined twice). Moving a symbol between files is net-zero; adding N functions shows exactly +N.
   4. **Player-build resolvability (the mis-bucket catcher)** — reconstruct `index.html`'s symbol set (**core ∪ game ∪ index-inline**), strip the dead `mode==='editor'`/`'charlab'` branches, and confirm every **unconditional** reference on `index.html`'s live paths (`loop`, `bootGame`, `deploy`, the tail) resolves within that set — **nothing may resolve only to `editor.js`.** A per-line “is there a `mode===` on this line” scan is **NOT** enough (`loop()` is a dense one-liner with `mode===` elsewhere on it — exactly how the `placeCursor` mis-bucket slipped past a weaker scan).
   - Also **grep presence markers on the FULL html** (HUD/CSS strings live outside the script). And **screenshots of BOTH pages are the behavioural gate** — the automated checks prove loadability/resolvability, not runtime behaviour.
   - ⚠️ **Brace/paren/bracket counting is UNRELIABLE** — core embeds **GLSL inside JS strings** (the outline `ShaderMaterial` + Fresnel `onBeforeCompile`), throwing off naive `{}` counts (a standing `+4` `{` imbalance, a false positive). Use it only as a *delta* check (net 0 across an edit = balanced). Trust `node --check`, not the absolute count.
3. **Logic tests** — regex-extract real functions' source into a tiny **Node harness with a minimal THREE stub** and assert behaviour. The stub that works here: geometry constructors as regular functions (so `new` works); a `Color` stub whose `.set` unwraps `{value}`; an `Object3D` stub with `children`/`traverse`/`userData`/`position|rotation|scale.set`/`quaternion`. Extract via a brace-matching helper over `FACTION_PALETTES`, `applyFaction`, `buildEntity`, `CHAR_MALE/FEMALE`, `CPARTS`, `assembleChar`, `poseArms`, etc. Good for rig math, `assembleChar` hide/skip logic, snap rounding, faction colour resolution.
4. **Python pitfalls:** never append a `//` comment in the middle of a dense one-liner — it silently swallows the rest of the line (valid syntax, missing code; check 3 catches the dropped symbol). `re.sub` *replacement* strings choke on `\u` and `%` escapes (and inserted JS often contains `%`/`\u`). Use **literal substrings + `str.replace`**, not regex replacement. Only use **BMP** unicode literals or `\uXXXX` BMP escapes — **never** astral/surrogate-pair escapes (they crash the write and zero the file). Safe glyphs seen in use: `⚒ × ↔ Δ · − ⊕ ⟳`. ROADMAP writes: plain string concatenation, not `%`-formatting.
5. **Identify the owning file first** (core / game / editor per `MODULE_MAP.md`), edit **that** file, and check **both HTML pages** (`index.html` + `editor.html`): a change to a shared file affects both builds. Commit on the change-set branch after the gate passes (`node tools/gate.mjs`, START_HERE §5). Update `ROADMAP.md` when meaningful work lands.
6. **Seeing the render:** use the headless previewer (8b) for a before/after of every visual change; Mustafa's Brave screenshots are the final gate and the only view of HUD/DOM/editor UI. The `visualize`/`show_widget` tools are unreliable for this project — never rely on them; the only delivery is the file itself.

### 8b. The headless toolkit (`tools/`; never shipped)

Setup: `cd tools && npm ci` (versions pinned in `tools/package-lock.json`; notes in `tools/README.md`). Run every tool from the repo root; on Linux, run `preview.js` renders under `xvfb-run -a`. `gate.mjs` wraps `battery.js` into the four-check gate; `pageshot.js` renders the real pages (`--headed` for the GPU).
- **`preview.js <buildDir> <cmd>`** — loads three r128 (headless WebGL1 via ANGLE, real 2D canvas for the procedural textures, stubbed DOM) + the real `shamble-core.js` + `shamble-game.js`, runs `bootGame()`. Commands: `shot <out.png> <x> <z> [fov]` (gameplay camera: target + (sin yaw·camBack, camHeight, cos yaw·camBack), sun follow applied), `menu`, `top <out> <cx> <cz> <span>`, `eval <snippet.js>` (run code in the booted game, print JSON), `measure`, `time`. Its colour math reproduces the real pipeline (hand-checked). **Limits:** WebGL1 vs Brave's WebGL2; no HUD/DOM/CSS/halftone.
- **`battery.js <dir>`** — checks 3–4 on an acorn AST: top-level symbol inventory + cross-file/intra-file dupes; every player-build reference to an editor-only symbol classified guarded (mode branch / `&&` short-circuit / `typeof` guard / `switch(mode)`) / dead (unreachable from load-time code + index DOM handlers) / FLAGGED. Mutation-tested: re-creating the `placeCursor` incident is flagged at `loop()`.
- **`sim.js`** — `preview.js <dir> eval sim.js`: real `deploy()` + `update(1/60)` for 90 s with 6 bots; reports stall % (alive, moved < 0.3 m in 3 s), stalls within 3 m of village pieces, harvester state. Random AI → compare 3+ runs.
- **`aabbs.js` + `village.py`** — export the live SIM boxes + objective zones; design the left half of a layout, mirror it, validate clearances (roads, colliders, objectives, spawns, every spawn/objective route, harvester routes, bounds, overlaps) with auto-nudge.

---

## 9. File inventory

- **`shamble-core.js` / `shamble-game.js` / `shamble-editor.js`** — the three shared-global source files (engine / gameplay + dispatch / authoring). See `MODULE_MAP.md` + §1a.
- **`index.html`** — the **player build** (core + game). **`editor.html`** — the **authoring build** (core + game + editor). Both hand-thin: manifest comment, script tags, `bootGame()` [`bootEditor()`] + menu tail.
- **Docs (`docs/`, synced into the Advisor's Project Knowledge):** `STATUS.md`, `START_HERE.md` (read first), `MODULE_MAP.md` (read second), `TECHNICAL.md` (this), `GAME_DESIGN.md`, `ART_DIRECTION.md`, `ROADMAP.md`, `UNITS.md`, `DESIGN_NOTES.md`, `adr/`, `reports/`.
- **Tools (`tools/`, never shipped):** `gate.mjs`, `battery.js`, `preview.js`, `pageshot.js`, `sim.js`, `aabbs.js`, `village.py` (§8b, `tools/README.md`).
- **Retired:** `shamble-frontlines.html` (pre-split single file; removed from Project Knowledge). The title was once `VANGUARD` — it survives only in legacy persistence-key fallbacks. Also outdated: `echelon.html`, old prototypes, `sim-core.js`, any `journal.txt` catalogue.

---

## 10. Known fracture points & future cleanup (honest list)

- **Bots have no pathfinding or stuck recovery** — they steer straight at targets and slide along colliders. Collision-bearing map pieces must keep bot routes clear (`village.py` validates; `sim.js` A/B-tests). **[verified]**
- **Draw calls in the village areas** are ~550–770 per frame incl. the shadow pass (was ~200–420): houses' many small detail meshes (+ hulls) dominate, trees are 15–32%. No mesh merging yet; do it if weaker machines drop frames. **[verified, headless count]**
- **Placeholder architecture:** the base buildings (HQ / barracks / refinery / power plant) are plain boxes; the brick-flint farmhouse reads as candy stripes; `BK.PRESETS` has one region (`normandy`).
- **Rotated objects use a loose AABB collider** — rotation is visual; a rotated barracks keeps an axis-aligned interior test. **[assumed]**
- **Dead/legacy code:** `switchWeapon`/`cycleWeapon`/`buyWeapon` are inert (class primaries are locked); a dormant `'jav'` javelin charge path exists. **[jav path verified; the rest assumed]**
- **The `coat`/`CoatAccent` role is unused** by the current wardrobe (palettes still define it). **[assumed]**
- **UI theme is mixed** — HUD/menu use the parchment theme; shop/career/tutorial modals may still be dark-olive. **[assumed; DOM isn't visible to the previewer]**
- **Kill-vignette hitch is still present** — the staggered-readback attempt (`_vigQueue` / `vigStep`) is in the build but didn't remove it; root cause open. Profile first. **[Mustafa, 2026-09-25]**
- **Juggernaut:** livery hardcoded red + gold in `spawnVehicle` (ignores the faction palette); a stale "STUB" comment there predates the working enter/drive code; shell impacts are visual only (no splash). **[verified]**
- **The SIM core is multiplayer prep; networking is unbuilt** (Phase 6).
- **Brace-count false positive** (§8) — expected; don't chase it.

---

## 11. Quick navigation cheat-sheet (grep targets)

Grep the file that owns the system (per `MODULE_MAP.md`). Targets grouped by file:

**`shamble-core.js`** (engine / shared truth):
- Tuning & arsenal: `const T =` · `const WEAPONS` · `const UNIT_TYPES` · `const VEHICLE_TYPES`
- Toon look: `function stylizeScene` · `function toonify` · `function makeOutlineMat` · `function addHull` · `function ensureOutlineNormals` · `_outlineDirs` · `OUTLINE_GROUND_Y` · `_toonGrad` · `_mkGrad` · `gradForKind` · `INK` · `onBeforeCompile` · `generateGrassTexture`
- Colour / persistence: `outputEncoding` · `_natureGain` · `userData.nature` · `const KVStore`
- Lighting/atmosphere: `const ENV_DEFAULT` · `function applyEnv` · `scene.fog` · `HemisphereLight`
- Character builder: `function buildEntity` · `function buildCharacter` · `const CHAR_DEFAULT` · `const CHAR_MALE` · `const CHAR_FEMALE` · `const CPARTS` · `function assembleChar`
- Faction/roles: `const FACTION_PALETTES` · `function applyFaction` · `PadColor` · `matKind`
- Rig / weapon props: `function poseArms` · `makeSword` · `makeShield` · `makeAxe` · `makeAMKRifle`
- Building kit: `const BK` · `function makeTimberHouse` · `function makeThatchCottage` · `function makeBrickFlintFarmhouse` · `function makeManorHouse` · `_bkWorldBoxes`
- Map schema: `function parseGridToMapDef` · `cabanaRepublicMap` · `DM_Cabana` (+ `.houses`/`.trees`/`.props`/`.covers`) · `DM_Romania` · `BK.buildCover`
- SIM math (fenced at bottom): `function resolveShot` · `function hasLOS` · `function rayAabb` · `const SIM` · `FIXED_DT`

**`shamble-game.js`** (gameplay + shared dispatch):
- Combat/abilities: `function fireOnce` · `function castShot` · `function damagePlayer` · `function damageEnemy` · `equipPrimary`/`equipSpecial` · `startCharge`/`releaseCharge`
- Kit trees/props (inside `bootGame`): `BK.TREES` · `BK.buildTree` · `BK.PROPS` · `BK.buildProp`
- Persistence users: `const ProgStore` · `const TutStore`
- FX/audio: `function addImpact` · `function addExplosion` · `for(let i=fx.length-1` · `const SFX`
- Vehicles / movement: `function buildJuggernaut` · `function resolveObstacles`
- Economy/rules/progression: `function applyRules` · `RULES_` · `const Prog`
- HUD & reticle: `function placeCursor` (combat reticle/hitmarker/reload-ring)
- Map build: `function buildMap` · `function clearMap` · `function serializeMap` · `function dedupeMapDef`
- World-registry arrays: `let obstacles` · `let buildings` · `let enemies` (etc.)
- **Shared dispatch/boot:** `function loop()` · `function update(dt)` · `function backToMenu` · `function deploy` · `function setMenuCam` · `function bootGame`

**`shamble-editor.js`** (authoring — `editor.html` only):
- Editor core: `function enterEditor` · `function updateEditor` · `function editorPlay` · `function edSetSel` · `function edDuplicate` · `_edEnsureGizmo` · `_edResizeFootprint` · `edTargets`
- Character Lab: `function clAssembled` · `function clRebuild` · `function clCycleSlot` · `function clBuildPanel` · `charLabSlots`
- Panels / overlays / boot: `function edBuildWorld` · `function edBuildRules` · `ovHideAll` · `function edLeaveToMenu` · `function bootEditor`

**Per page (HTML):** the inline `<script>` is just `bootGame();` [`bootEditor();`] + the menu tail (`setMenuCam(); loop();` + resize). HUD text + CSS live in each page's `<head>`/`<body>` — **outside** the script; grep the full HTML for those.
