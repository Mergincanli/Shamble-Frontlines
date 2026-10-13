# Shamble Frontlines — Roadmap

> **Living plan + condensed history — kept deliberately lean.** The code — `shamble-core.js` → `shamble-game.js` → `shamble-editor.js`, loaded by `index.html` / `editor.html` — is the source of truth; this file is orientation only. Read **`START_HERE.md`** first, then `MODULE_MAP.md`; deep references: `GAME_DESIGN.md` (why), `TECHNICAL.md` (how), `ART_DIRECTION.md` (the locked *Living Comic* look), `UNITS.md` (planned roster).
>
> **Hygiene (please follow):** add **terse** entries (a few lines each) under "Recent work", newest first; keep ~8–12 there and fold the oldest into "Build history" when it overflows. Do **not** paste change-set detail, harness output or verify logs here — those belong in the chat.

---

## The game (30 seconds)
Browser-based **top-down team shooter** in **Three.js r128** (UMD global; **classic multi-script** — no ES modules / build step). A fusion of **Foxhole** (overhead mouse-aim cam), **C&C Renegade** (buy classes at a terminal, power-plant pricing, destroy-the-HQ win), **Red Alert 3** (bright toybox palette, rock-paper-scissors units, 3 factions), and **Gears of War** (weighty gun feel + a mechanic per gun). PvE vs 6 bots today; the deterministic headless SIM core is multiplayer groundwork.

## Design pillars
Readability first (overhead cam) · class identity over loadout freedom · rock-paper-scissors counters · gun feel = weight + a hook · base-assault economy with momentum swings · positive/vibrant RA3 energy · faction identity is *worn*, not just tinted.

## Economy (live values: `ECO` / `RULES_DEFAULT` / `T` — verified 2026-09-25)
- **Pay** (personal credits): kill +25 · refinery trickle 1/s (0.25 if your refinery is dead) · each held silo +1/s · harvester dock +600.
- **Tech** (shared team meter, era T1→T4): tech box held to expiry +10 · harvester dock +5 · kill +1 / headshot +2 · era costs [60, 90, 120].
- **Levers:** a downed power plant halves that team's tech and raises the player's prices (`priceMult`). Silos (3.5 s capture) and tech boxes (7 s channel, 120 s active) give mobile, comeback-friendly income.
- **Shop:** the special weapon (Torque Bow, 500 × `priceMult`), frag grenades, resupply (50). Class primaries are locked; the Officer class costs 1250, the Juggernaut 2500.

## Phases
- **0 — Economy spine + eras** — DONE
- **1 — Tier-gated arsenal + player level** — DONE (data-driven era/minLevel gates; content still placeholder)
- **2 — Harvester / supply convoy** — DONE
- **3 — Tech boxes** — DONE
- **4 — Economy-as-weapon (power plant + silos)** — DONE
- **5 — Unit-type system** — IN PROGRESS: 5.1 first-tier infantry done · 5.2 ground vehicles started (Juggernaut built + drivable) · aircraft pending
- **6 — Multiplayer** — NOT STARTED (SIM core is deterministic groundwork)

## Build history (condensed, by system)
- **World / economy:** two-pool economy + era state machine; power-plant pricing; 2 capturable silos; rotating tech boxes; per-team AI harvester convoy; HUD + hold-TAB team scoreboard with real kill attribution; persistent rank/prestige (`Prog`).
- **Map:** data-driven ASCII grid "Cabana Republic" on a rectangular island (water / beach / grass), now populated with a symmetric Normandy village (see Recent work).
- **Arsenal / units:** 4 classes — Conscript, Peacekeeper, Imperial Warrior, Officer (tier 2) — each a locked primary + one ability (molotov / riot shield / banzai katana / bayonet charge); R/P/S triangle; shop extras: frag grenade + Torque Bow special; enemies are the same unit types plus the Empire elite.
- **Vehicles:** Juggernaut — the first vehicle: shop-bought at era T2 (2500), pilot-driven walker with an independent turret and triple artillery (130 dmg, 130 m, hits buildings); idles when unpiloted (`buildJuggernaut` / `enterJug` / `tickPilot` / `exitJug`).
- **Character system:** one data-driven builder `buildEntity`; Character Lab (male/female bodies, live faction recolour, wardrobe slots, 8 hairstyles, 5 facial-hair styles, proportion sliders, JSON export/import); material-**role** faction system; gameplay soldiers faction-dressed (`buildSoldier` + `SOLDIER_KIT`); procedural animation (locomotion → carry/ADS aim layer → 2-bone arm IK).
- **Map editor (sandbox):** grid/angle snap, gizmos + resize handles, path decals, marquee + group ops, Tab playtest, dockable World panel, undo/redo, numeric inspector, mirror/array/prefab stamps, analysis overlays, verticality; **dark-token UI overhaul**; **Building Maker** (`BK` kit-of-parts: footprint/roof/skins/dormers + seeded `BK.roll`; one region preset so far: `normandy`).
- **Architecture:** timber house, thatched cottage, brick-and-flint farmhouse, Norman manor (round tower + conical roof); trees (apple/poplar/oak), props (8 kinds), covers (hedge/drystone/fence).
- **Art direction:** "Lush Comic" → relocked as **"Living Comic"** (2026-07-08, ART_DIRECTION §9 steps 1–5: 2-band ramps, differential dark ink, Living Comic env, brightened primaries, Empire elite villain, halftone default-off).
- **Structure:** file split (2026-07-12) — single ~4,680-line HTML → core/game/editor + two pages; player menu is Play + Career only.
- **Performance / hygiene:** pooled FX, GPU resources disposed on map rebuild, player-following shadow frustum; kill vignette captured on the GPU (`vigShot` corner render + `drawImage`); hot paths use `hyp2`/`hyp3` and change-only DOM writes (`_sd`/`_so`/`_st`); ~6 MB/s residual GC churn (measured — `AUDIT_BUGS_PERF.md`).

## Current state (verified against the build, 2026-09-25)
- **Look:** Living Comic with **colour-true output** (authored hex = on-screen), option-C lighting on buildings/characters, a **nature gain** keeping grass/foliage/hedges/sea lush, and **contact ink** (every object inked where it meets the ground). Confirmed by Mustafa in Brave.
- **Map:** Cabana Republic carries the village (11 houses, 70 trees, 36 props, 10 hedges/fences); the centre stays open as the main combat space.
- **Persistence:** `KVStore` — rank, tutorial flag, maps, prefabs survive reloads under `file://`. Confirmed in Brave.
- **Tooling:** headless previewer + battery + match simulator (Project Knowledge; see START_HERE §5).
- **Animation:** ANIM v2 — weapon-first IK holds, planted feet with ankles, per-gun reloads (see Recent work). Katana + shield moved onto the IK system (awaiting Brave check).
- **Performance:** kill-vignette hitch fixed (GPU capture); GC churn ~6 MB/s after the audit; details in `AUDIT_BUGS_PERF.md`.

## Next / open
- **Graphics — rest of Upgrade A:** comic cast shadows (fold the sun's shadow into the 2-band ramp) → painted ground (macro colour patches, baked contact shadows, dirt roads instead of grey tiles). Then **Upgrade B "Comic Impact"**: bright hit flash, hit-stop on every kill, weighted shake, cel explosions, comic muzzle stars, impact frames.
- **Constraints review with Mustafa:** no-addons and no-post-processing date from the artifact era — decide keep / amend.
- **Architecture:** real base buildings (HQ / barracks / refinery / power plant are still placeholder boxes); fix the candy-stripe brick farmhouse; more region presets.
- **AI:** bots route around obstacles (grid A*, `navSteer`); richer behaviours (shield / molotov / officer) still to design.
- **Performance (open):** static kit pieces merged by material (village 398 → 202 draw calls); FX pooling with Comic Impact; pre-declare runtime-added object fields; skip the 3D render while the opaque menu covers the screen (it renders every frame behind it today). See `AUDIT_BUGS_PERF.md` §6.
- **Editor bug (pre-existing):** `refreshPrefabList` is declared inside `bootEditor()` but called from top-level `enterEditor` → ReferenceError on every editor entry (prefab list not refreshed). Fix: hoist it.
- **Animation:** actions (flinch, deaths, weapon draw, throws, bayonet thrust), katana + shield on IK, and the Anim Lab (Character Lab tab) — built, awaiting Brave check. Reference + next steps: `ANIMATION_PLAN.md`. DEV: after Unlock All, mouse wheel / 2 cycle every gun.
- **UI theme (approved mockup):** "war-room dossier" drawn in the Living Comic style — inked sheets with halftone cel shadows, stencil headings, rubber-stamp juice, ease-in-out pen marks (4 variants); paper for menu screens, dark field-kit crate for in-game overlays. Source of truth: `ui_theme_mockup.html` (keep in Project Knowledge). Menu + pause done (awaiting Brave); next: service record + end of round → armory + scoreboard, each gated on Mustafa's screenshots; then the first-deploy tutorial — it lists dead controls (V / B / 1–5), so make it the clipboard's field manual (one controls list).
- **Character rework (later):** move from the blocky look to Foxhole-like human proportions; the service-record portrait in the mockup is the style target.
- **Gameplay (design first):** progression (Identity / Momentum / Mastery — design only), infantry gun tiers T1→T4 (held for progression), Spy unit, richer enemy behaviours (shield / molotov / officer), aircraft, multiplayer.

---

## Recent work
*(Newest first, a few lines each. Older items fold into "Build history" above.)*

**UI theme, step 1 [2026-10-02]** — main menu + pause transplanted from `ui_theme_mockup.html`: the menu is the dossier (folder, typed brief, rubber stamps, pen-mark tabs: Deploy / Service record / Field manual, + Map editor on the authoring build); pause is the clipboard; Field manual opens the clipboard from the menu (Aim / Audio reachable before deploying). CSS namespaced `wr-*` in both pages; behaviour in game (`wrBoot` / `wrDeploy` / `wrType`). Fixed the mockup's pen-mark flecks; fits 1280×720. Awaiting Brave.

**Audit phase 2 [2026-09-26]** — every open audit item handled (`AUDIT_BUGS_PERF.md` §7): pooled FX + damage numbers, stable object shapes, dead code removed (javelin path + 11 functions), rotated obstacles block shots/LOS exactly (`rayStatic`), inline occlusion test (GC −39 % overall), static kit pieces merged by material (village 398 → 202 draw calls), bot pathfinding (grid A*, `navSteer`), two UI families (paper menus / dark in-game overlays).

**Bug & optimization run [2026-09-26]** — full audit (per-frame static scan, runtime + V8 heap profiling, fuzz, soak; `AUDIT_BUGS_PERF.md`). Kill-vignette hitch fixed: GPU capture (corner render + `drawImage`, no readbacks), no forced reflow, shadow map reused during the hit-stop, comic textures pre-warmed. GC churn −20 % (`Math.hypot` → `hyp2`/`hyp3`); V8 const-tracking-let deopts removed (one-time warm-up); per-frame DOM writes only on change; Juggernaut in the faction livery (`PLAYER_FACTION`). Kill fix confirmed in Brave.

**Animation overhaul [2026-09-26]** — ANIM v2 (game): every gun has its own model + sockets (`GUN_KIT`, core); holds are placed gun-first with both hands on IK and the support hand reach-aware; feet plant in the world with a procedural ankle (boots stay flat); stride sized from real leg reach; hips follow movement within a twist budget; greatcoat skirt in four thigh-driven panels; per-gun reloads. Player shots fire from a fixed point (`playerFireOrigin`), not the animated muzzle. Confirmed in Brave.

**Doc pass [2026-09-25]** — every doc re-verified against the live build (values pulled from the running game); START_HERE and PROJECT_INSTRUCTIONS rewritten for the current workflow; drift fixed across all docs.

**Village pass [2026-09-25]** — `DM_Cabana` populated: roadside hamlets on both outer roads, a brick farm per side, the Norman manor on the centre line, orchards, poplar rows, oaks, bocage hedgerows, props. Left half designed + mirrored; every piece validated against roads, colliders, objectives, spawns, bot and harvester routes (`village.py`). Headless match A/B: no bot stalls near any new piece.

**Colour pass [2026-09-25]** — `LinearEncoding` output (sRGB output had brightened every authored colour twice → washed-out look, grey shadows). Option-C lights (sun 0.8, hemi 0.32, amb 0.12); `ENV.nature.gain` 1.56 keeps the environment lush; World panel "Nature brightness" slider.

**Ink: contact lines [2026-09-25]** — outline hulls use smoothed + mitred directions (`ensureOutlineNormals`), world-space thickness and a ground clamp → continuous ink where objects meet the ground; weights unchanged (measured).

**Storage fix [2026-09-25]** — `KVStore` adapter (window.storage → localStorage → memory); stores no longer silently in-memory under `file://`; the tutorial no longer re-opens on every deploy.

**Headless toolkit [2026-09-25]** — `preview.js` renders the real build (WebGL1); `battery.js` implements checks 3–4 on an AST (mutation-tested against the `placeCursor` incident); `sim.js` runs a headless 90 s match.

**Kill-vignette hitch attempt** — the three synchronous GPU readbacks moved off the kill frame (`_vigQueue` drained by `vigStep()` across the hit-stop). Did **not** remove the hitch (Mustafa, 2026-09-25) — root cause still open.
