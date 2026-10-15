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
- **Storage (2026-09-25):** `KVStore` adapter (window.storage → localStorage → memory); stores no longer silently in-memory under `file://`; the tutorial no longer re-opens on every deploy.
- **Village / colour / ink passes (2026-09-25):** `DM_Cabana` populated with a mirrored Normandy village, validated against roads, colliders, objectives, spawns and routes (`village.py`); `LinearEncoding` output with option-C lights (sun 0.8, hemi 0.32, amb 0.12) and `ENV.nature.gain` 1.56; contact-line ink via `ensureOutlineNormals` (smoothed + mitred hulls, ground clamp).
- **Doc pass (2026-09-25):** every doc re-verified against the live build; START_HERE and PROJECT_INSTRUCTIONS rewritten for the current workflow; drift fixed across all docs.
- **Structure:** file split (2026-07-12) — single ~4,680-line HTML → core/game/editor + two pages; player menu is Play + Career only.
- **Headless toolkit (2026-09-25):** `preview.js` renders the real build (WebGL1); `battery.js` implements checks 3–4 on an AST (mutation-tested against the `placeCursor` incident); `sim.js` runs a headless 90 s match.
- **Performance / hygiene:** pooled FX, GPU resources disposed on map rebuild, player-following shadow frustum; kill vignette captured on the GPU (`vigShot` corner render + `drawImage`); hot paths use `hyp2`/`hyp3` and change-only DOM writes (`_sd`/`_so`/`_st`); ~6 MB/s residual GC churn (measured — `AUDIT_BUGS_PERF.md`); an early kill-vignette attempt (readbacks queued via `_vigQueue`/`vigStep()`) did not remove the hitch and was superseded by the GPU capture.

## Current state (verified against the build, 2026-09-25)
- **Look:** Living Comic with **colour-true output** (authored hex = on-screen), option-C lighting on buildings/characters, a **nature gain** keeping grass/foliage/hedges/sea lush, and **contact ink** (every object inked where it meets the ground). Confirmed by Mustafa in Brave.
- **Map:** Cabana Republic carries the village (11 houses, 70 trees, 36 props, 10 hedges/fences); the centre stays open as the main combat space.
- **Persistence:** `KVStore` — rank, tutorial flag, maps, prefabs survive reloads under `file://`. Confirmed in Brave.
- **Tooling:** headless previewer + battery + match simulator + `gate.mjs`, all in `tools/`, with GitHub Actions CI (see START_HERE §5, `tools/README.md`).
- **Animation:** ANIM v2 — weapon-first IK holds, planted feet with ankles, per-gun reloads (see Recent work). Katana + shield moved onto the IK system (awaiting Brave check).
- **Performance:** kill-vignette hitch fixed (GPU capture); GC churn ~6 MB/s after the audit; details in `AUDIT_BUGS_PERF.md`.

## Next / open
- **Queue (Advisor, updated 2026-10-14):** kit-v1-2 (merged) → bot-poplar-wedge (merged) → editor-undo-ground (approved) → map-dedupe-on-load (plan approved) → tutorial-field-manual → ui-step-2 → scale-test → ui-step-3 → house-textures-seeded (new, low priority) → build-spike → ADR run. Client-side visual/UI work first (it survives the multiplayer refactor); gameplay systems (progression, new units, Spy, squads, three teams) wait for the foundation refactor.
- **Graphics — rest of Upgrade A:** comic cast shadows (fold the sun's shadow into the 2-band ramp) → painted ground (macro colour patches, baked contact shadows, dirt roads instead of grey tiles). Then **Upgrade B "Comic Impact"**: bright hit flash, hit-stop on every kill, weighted shake, cel explosions, comic muzzle stars, impact frames.
- **Constraints review with Mustafa:** no-addons and no-post-processing date from the artifact era — decide keep / amend.
- **Architecture:** real base buildings (HQ / barracks / refinery / power plant are still placeholder boxes); fix the candy-stripe brick farmhouse; more region presets.
- **AI:** bots route around obstacles (grid A*, `navSteer`); richer behaviours (shield / molotov / officer) still to design.
- **Performance (open):** static kit pieces merged by material (village 398 → 202 draw calls); FX pooling with Comic Impact; pre-declare runtime-added object fields; skip the 3D render while the opaque menu covers the screen (it renders every frame behind it today). See `AUDIT_BUGS_PERF.md` §6.
- **Editor bug (open):** Ctrl+Z (undo) changed the ground textures (old, reported by Mustafa); fixed on cs/editor-undo-ground (PR #8, approved), merge on Mustafa's go. (The three-ReferenceError editor bug is done: PR #4.)
- **Animation:** actions (flinch, deaths, weapon draw, throws, bayonet thrust), katana + shield on IK, and the Anim Lab (Character Lab tab) — built, awaiting Brave check. Reference + next steps: `ANIMATION_PLAN.md`. DEV: after Unlock All, mouse wheel / 2 cycle every gun.
- **UI theme (approved mockup):** "war-room dossier" drawn in the Living Comic style — inked sheets with halftone cel shadows, stencil headings, rubber-stamp juice, ease-in-out pen marks (4 variants); paper for menu screens, dark field-kit crate for in-game overlays. Source of truth: `ui_theme_mockup.html` (keep in Project Knowledge). Menu + pause done (awaiting Brave); next: service record + end of round → armory + scoreboard, each gated on Mustafa's screenshots; then the first-deploy tutorial — it lists dead controls (V / B / 1–5), so make it the clipboard's field manual (one controls list).
- **Character rework (later):** move from the blocky look to Foxhole-like human proportions; the service-record portrait in the mockup is the style target.
- **Gameplay (design first):** progression (Identity / Momentum / Mastery — design only), infantry gun tiers T1→T4 (held for progression), Spy unit, richer enemy behaviours (shield / molotov / officer), aircraft, multiplayer.

---

## Recent work
*(Newest first, a few lines each. Older items fold into "Build history" above.)*

**Editor undo ground [2026-10-14, APPROVED, merge on Mustafa's go]** — cs/editor-undo-ground (PR #8; Advisor APPROVE; Mustafa's Ctrl+Z check in Brave passed; `shamble-game.js` only, symbols +0 -0, 7 insertions / 4 deletions): `addBox`, `addPath`, `makeBuilding`, `makeBarracks` and the `buildMap` terminal now call `stylizeOne()` (rebuilds lost toon + ink because `stylizeScene` runs only at boot); `clearMap` now clears `trees[]` and `props[]` (each rebuild stacked another 70 trees / 36 props). Full-map draw calls boot / 1 / 2 rebuilds 1992 / 2406 / 3502 -> 1992 constant; trees / props 70/36 constant vs doubling; heap after 10 rebuilds 119.5 -> 44.5 MB; real-page pixels changed by one Ctrl+Z 57,854 -> 124. Gate PASS (8 checks); art-director and perf-auditor PASS WITH NOTES. Deferred: PERF-2 (skip shared `BK._tex` textures in `_disposeObj3D`, low). Old saved maps may keep duplicates: map-dedupe-on-load is next. Report: `docs/reports/2026-10-14-editor-undo-ground.md`. Waits on Mustafa's go to merge.

**Bot poplar wedge [2026-10-14, MERGED]** — bot-poplar-wedge (PR #7, 159bec4, Advisor APPROVE; `shamble-game.js` only, symbols +0 -0): `navBuild` tests each nav cell centre at 0.7 m + half a cell (was 0.75), `navPath` starts from the nearest open cell when the start is blocked, 3-call `navPath` warm-up at the end of `bootGame`. Poplar pins >5 s 39 -> 0; 240 seeded match pairs: tree-face stall windows 1885 -> 0, mean stall 9.12% -> 8.50%; sim.js 40 runs each: mean 8.82% -> 7.73%; all bot goals reachable on both maps. perf-auditor and regression hunter PASS WITH NOTES. New risks logged in STATUS (editor-made layouts, replan storm, 8000-node A* cap, wall-clock reads vs determinism). Report: `docs/reports/2026-10-14-bot-poplar-wedge.md`.

**Kit v1.2 [2026-10-14, MERGED]** — kit-v1-2 (PR #6, ef019e0, Advisor APPROVE): the Advisor's fix for the finding deferred from gate-check-5. `.claude/settings.json` allow rule `Bash(npx --no -- playwright *)` replaced by `Bash(npx --no -- playwright install chromium)`; other playwright commands (`install --with-deps`, which runs sudo apt; `open`; `codegen`) now ask. One stale START_HERE line fixed (Playwright is needed for the gate's smoke test, not only pageshot). Gate PASS. No game code touched.

**Gate check 5 [2026-10-14, MERGED]** — gate-check-5 (PR #5, 8675367, Advisor APPROVE after his own full-gate run PASS in 26 s and mutations that failed correctly): adds check 5 to `tools/gate.mjs`: 5a headless-Chromium smoke with real input on both pages (`tools/smoke.mjs`), 5b names (eslint-scope: bare names resolve, no assignment to a const, every `THREE.<member>` exists in r128), 5c core layering, 5d script order. Default base is now the newer of main / origin/main; `--no-smoke` / `--smoke-only`; CI has two jobs (gate, smoke). 22/22 mutation proofs, 5/5 clean full-gate runs, CI green. Review found 28 issues, in-scope ones fixed; deferred: no SRI/CSP on the three.js CDN tag (ADR-001), over-broad playwright allow rule (fixed in kit v1.2). Report: `docs/reports/2026-10-14-gate-check-5.md`. Check 5 is live on main and in CI; a fresh `npm ci` in tools/ and Node 22.13+ are needed.

**Editor: three ReferenceErrors fixed [2026-10-14, MERGED]** — editor-prefab-list (PR #4, c458fe7, gate PASS vs 5858864, Advisor APPROVE, Mustafa's Brave check of editor.html passed with no problems): `refreshPrefabList`, `refreshMapList` and `edSavePrefab` hoisted unchanged out of `bootEditor()` to top level in `shamble-editor.js`; the prefab list fills on entry, Maps lists saved maps, "Save as prefab" works. Gate PASS (+3 editor symbols), verifier PASS; headless before/after: 4 ReferenceErrors → none. Found: the gate's default base can use a stale local `main` (fixed in gate-check-5).

**Kit v1.1 + Phase 0 merged [2026-10-14]** — Phase 0 merged (PR #2, 2d30e4a, Advisor APPROVE; CS-1 / PR #1 approved after the fact); stays PARTIAL until Mustafa's Windows run. kit-v1-1 MERGED (PR #3, 5858864, 2026-10-14, Advisor APPROVE incl. the 19-rule deny amendment): Advisor's `.claude/settings.json` applied exactly + standing-permission paragraph in CLAUDE.md; deny rules cover push-to-main by refspec, force forms, `--mirror`/`--delete`, `--upload-pack`/`ext::`, `--output`. No code touched. Mustafa still runs `/permissions` locally. Scan found the editor bug is three bugs. Branch protection on `main` recommended. Order: editor-prefab-list → gate-check-5 → bot-poplar-wedge → Prompts 1–3.

**Phase 0: agentic environment + green baseline [2026-10-13]** — portable toolkit (`tools/package.json` + lockfile, Node 22) and `tools/gate.mjs` (checks 1–4 vs a base ref), proven to fail on four injected breakages; GitHub Actions CI; syntax hook; `pageshot.js --headed`; START_HERE §1/§5 rewritten, multiplayer target updated. No game files touched. Verifier PASS WITH NOTES (two holes fixed). Pending: Windows acceptance run (Prompt 0b) and merge after CS-1. Gate does not check load order or layering; a load smoke test is proposed.

**Repo root move (CS-1) [2026-10-13]**: the GitHub web upload had nested the project in `shamble-frontlines-repo/` and dropped every dot-file (the `.claude/` kit, `.gitignore`, `.gitattributes`); Mustafa pushed those from the zip (826b896). CS-1 moves all 52 files to the root as pure renames (tree identical) and removes GitHub's placeholder README, so Claude Code loads CLAUDE.md and the kit from the repo root.

**UI theme, step 1 [2026-10-02]** — main menu + pause transplanted from `ui_theme_mockup.html`: the menu is the dossier (folder, typed brief, rubber stamps, pen-mark tabs: Deploy / Service record / Field manual, + Map editor on the authoring build); pause is the clipboard; Field manual opens the clipboard from the menu (Aim / Audio reachable before deploying). CSS namespaced `wr-*` in both pages; behaviour in game (`wrBoot` / `wrDeploy` / `wrType`). Fixed the mockup's pen-mark flecks; fits 1280×720. Awaiting Brave.

**Audit phase 2 [2026-09-26]** — every open audit item handled (`AUDIT_BUGS_PERF.md` §7): pooled FX + damage numbers, stable object shapes, dead code removed (javelin path + 11 functions), rotated obstacles block shots/LOS exactly (`rayStatic`), inline occlusion test (GC −39 % overall), static kit pieces merged by material (village 398 → 202 draw calls), bot pathfinding (grid A*, `navSteer`), two UI families (paper menus / dark in-game overlays).

**Bug & optimization run [2026-09-26]** — full audit (per-frame static scan, runtime + V8 heap profiling, fuzz, soak; `AUDIT_BUGS_PERF.md`). Kill-vignette hitch fixed: GPU capture (corner render + `drawImage`, no readbacks), no forced reflow, shadow map reused during the hit-stop, comic textures pre-warmed. GC churn −20 % (`Math.hypot` → `hyp2`/`hyp3`); V8 const-tracking-let deopts removed (one-time warm-up); per-frame DOM writes only on change; Juggernaut in the faction livery (`PLAYER_FACTION`). Kill fix confirmed in Brave.

**Animation overhaul [2026-09-26]** — ANIM v2 (game): every gun has its own model + sockets (`GUN_KIT`, core); holds are placed gun-first with both hands on IK and the support hand reach-aware; feet plant in the world with a procedural ankle (boots stay flat); stride sized from real leg reach; hips follow movement within a twist budget; greatcoat skirt in four thigh-driven panels; per-gun reloads. Player shots fire from a fixed point (`playerFireOrigin`), not the animated muzzle. Confirmed in Brave.
