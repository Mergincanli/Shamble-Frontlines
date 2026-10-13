# Shamble Frontlines — START HERE (engineer onboarding)

> **Read this first in every chat.** Written 2026-10-02 from the live build, at the end of the animation-overhaul +
> bug/optimization-audit chat. **[verified]** = confirmed against the code or measured; **[assumed]** = carried over,
> grep before trusting. The code is the source of truth; every doc can drift.

---

## 0. What this is (30 seconds)

Browser-based **top-down team shooter** in **Three.js r128** (UMD global), built as **classic multi-`<script>` files
sharing globals**, played by **double-clicking an HTML file off disk** (`file://`) in Brave. PvE vs 6 AI bots, four
classes (Conscript, Peacekeeper, Imperial Warrior, Officer), the Juggernaut walker, a two-currency base-assault economy,
win by destroying the enemy HQ. Fusion of Foxhole (camera), C&C Renegade (economy), Red Alert 3 (look, three factions),
Gears of War (gun feel). Visual style: **"Living Comic"** (`ART_DIRECTION.md`). UI style: **"war-room dossier"**
(`ui_theme_mockup.html`, being transplanted into the game).

## 1. Onboarding sequence (do this in order)

1. **`STATUS.md`** (where things stand), then **this file**, then **`MODULE_MAP.md`** (who owns what, load order).
2. **Work in the git repo.** The five code files sit at the repo root: `shamble-core.js`, `shamble-game.js`,
   `shamble-editor.js`, `index.html`, `editor.html`. One change-set per branch `cs/<slug>`, cut from an up-to-date
   `main` (`CLAUDE.md`, `/changeset`).
3. Set up the **toolkit** once per machine (§5): `cd tools && npm ci`.
4. **Grep the one owning file** for the task (§6). Pull deep references only as needed:
   `TECHNICAL.md` (how) · `GAME_DESIGN.md` (why) · `ART_DIRECTION.md` (the locked look) · `ANIMATION_PLAN.md`
   (animation system) · `AUDIT_BUGS_PERF.md` (what was measured/fixed, and what was kept on purpose) ·
   `ROADMAP.md` (history + plan) · `UNITS.md` (planned roster, not what's built) · `ui_theme_mockup.html` (UI look).
5. **Plan first** (plan mode): your understanding of the relevant systems and your plan; **wait for Mustafa's
   approval** before editing.

## 2. Architecture (compact — full map in `MODULE_MAP.md`)  **[verified]**

| File | Role |
|---|---|
| `shamble-core.js` | Engine + shared truth: config `T`, arsenal (`WEAPONS`, `UNIT_TYPES`, `VEHICLE_TYPES`), render globals, NPR pipeline, character builder + factions, gun kit (`GUN_KIT`), building kit (`BK`), map data (`DM_Cabana`), `KVStore`, `mergeStaticGroup`, `hyp2`/`hyp3`, the fenced SIM math (`rayAabb`, `rayStatic`, `resolveShot`, `hasLOS`). References nothing above it. |
| `shamble-game.js` | All gameplay + world registries + the shared dispatch (`loop`, `update`, `deploy`, `backToMenu`, input) + `bootGame`. Also: ANIM v2, NAV pathfinding, FX pools, `PLAYER_FACTION` (declared at the very top). |
| `shamble-editor.js` | Map editor, Character Lab + **Anim Lab**, `bootEditor`. Loaded by `editor.html` only. |
| `index.html` | Player build: THREE → core → game → inline `bootGame();` + menu tail. |
| `editor.html` | Authoring build: + editor → inline `bootGame(); bootEditor();`. Same head/DOM plus the Map Editor entry. |

- **Layering (hard):** core ← game ← editor. Anything on `index.html`'s live path resolves in core ∪ game, never editor.
- **One character builder:** `buildEntity(def, opts)`; gameplay soldiers via `buildSoldier` + `SOLDIER_KIT` / `ELITE_KIT`.
- **Faction colour = material roles** (`applyFaction` + `FACTION_PALETTES`); add a palette slot + role when a colour must diverge.
- **World registries** (`obstacles`, `houses`, `covers`, `trees`, `props`, `buildings`, `enemies`…) are game globals owned by `buildMap` / `clearMap`.

## 3. Hard constraints — each with its WHY  **[verified]**

- **Classic scripts, no ES modules, no bundler** — browsers block module imports under `file://`; Mustafa double-clicks the HTML.
- **Three.js r128 UMD** — no `CapsuleGeometry` (r142+). **No addons, no post-processing/EffectComposer** — both rules date from the old artifact runtime and are **queued for review with Mustafa**; keep them until he decides.
- **No external assets** — textures are canvas-generated, SFX synthesized (WebAudio), geometry procedural, UI fonts are system fonts. (Open question for Mustafa: embedding one stencil font as data in the page.)
- **Persistence only through `KVStore`** (window.storage → localStorage → memory). Renaming a key needs a fallback read.
- **`LinearEncoding` output** — authored hex = on-screen colour. Switching to sRGB needs every input colour converted.
- **`stylizeScene()` runs once at init** — runtime builders call `stylizeOne()` themselves; static kit pieces then call `mergeStaticGroup()`.
- **Animation is cosmetic** — the player fires from `playerFireOrigin()`, bots from a fixed chest point. Nothing animated may decide a hit.
- **Load order matters inside a file** — code that runs while the script loads must not touch a `const`/`let` declared later (temporal dead zone; see §7).

## 4. Do NOT do this

- Don't append a `//` comment in the middle of a dense one-liner — it silently swallows the rest of the line.
- Don't use `re.sub` replacement strings for JS (they choke on `\u` / `%`), and never write astral `\u{…}` escapes — use literal `str.replace` after `assert count==1`, or AST source ranges (acorn) for multi-site edits.
- Don't bucket code by where it sits — follow the references (the `placeCursor` incident, §7).
- Don't allocate per frame in hot paths: no `Math.hypot` (use `hyp2`/`hyp3`), no per-frame closures/arrays/objects, no destructuring `for…of` in per-frame loops; prefer inline math in tight loops.
- Don't add fields to long-lived objects (enemies, `ps`) at runtime — pre-declare them in the object literal.
- Don't add a new top-level `let` without adding it to the **let warm-up block at the end of `shamble-game.js`** (or accept one deopt on its first reassignment, §7).
- Don't set `renderer.shadowMap.autoUpdate` anywhere else — `loop()` owns it (shadow map reused during the hit-stop).
- Don't dispose pooled FX meshes (`FXPool` slots) or merged/shared materials; don't merge anything that animates or fades per part (tag `userData.noMerge`).
- Don't change the shared SIM ray API (`rayAabb` / `rayStatic` / `resolveShot` / `hasLOS`) casually — shots, line of sight and occlusion all depend on it.
- Don't ship UI text that describes removed mechanics (the menu blurb still mentioned the Javelin until the audit).
- Don't trust a doc claim without grepping — doc drift has been found every session.

## 5. Edit → verify workflow (the standing gate)  **[verified]**

**Edit:** on the change-set's branch, with the Edit tool (unique matches); multi-site mechanical edits through an acorn
AST script, never a blind regex. The syntax hook (`.claude/hooks/syntax-gate.mjs`) runs `node --check` on every JS and
HTML file the moment it is written. Commit on the branch: git is the backup and the delivery (no uploads).

**The gate**, after every edit to the engine files or pages and before every commit, report or merge:
`node tools/gate.mjs [baseRef]` (default base: the merge-base with `main`; `/gate` runs it; `--allow-drop a,b` for
removals the plan names; exit 0 PASS, 1 FAIL). CI (`.github/workflows/gate.yml`) runs it on every push and pull request.
1. `node --check` on each engine file and on each page's inline `<script>`.
2. Concatenation `node --check` in load order for both pages, read from their `<script src>` tags (catches cross-file `const`/`let` collisions).
3. Symbol conservation vs the base ref: no drops, no cross-file or within-file duplicates; prints +added / −dropped / moved, and the diff must equal the plan (`battery.js`).
4. Player-build resolvability: every unguarded reference on `index.html`'s live paths resolves in core ∪ game (`battery.js`; mutation-tested against the `placeCursor` incident).
Proven to bite (Phase 0): a deleted function, an unguarded call into the editor, a syntax error and a cross-file
duplicate each FAIL on the right check. Not covered: layering (core referencing game): review that by hand.
Brace counts are a delta check only (GLSL inside JS strings gives a standing false imbalance).

**Targeted tests** on the real code (harness snippets run through `node tools/preview.js . eval <snippet.js>`): animation metrics (hands on sockets < 1 cm, planted-foot drift, boots ≥ 0 cm above ground, IK stretch ≤ 0.98), fuzz (every class/ability, grenade, bow, Juggernaut, death/respawn — expect 0 errors), 3-minute soak (heap flat after GC), `sim.js` stall % (**random AI: compare ≥ 3 runs per build**), V8 sampling heap profiler for allocation rate, draw-call counts at fixed views.

**Toolkit** (`tools/`, never shipped; full notes in `tools/README.md`). Once per machine, with Node 22:
```
cd tools && npm ci                 # the pinned versions in tools/package-lock.json
npx playwright install chromium    # only needed for pageshot.js
```
Run every tool from the repo root. On Linux, prefix `preview.js` (and `pageshot.js --headed`) with `xvfb-run -a`;
Windows needs no prefix. Scratch output goes under `.gate/` (gitignored).
- **`node tools/preview.js . <cmd>`**: renders the real **3D scene** headless (WebGL1): `shot`, `menu`, `top`, `eval <snippet>`, `evalshot <snippet> <out.png>`, `measure`, `time`. It **cannot draw the HUD/DOM/CSS**. Needs the optional native `gl` package.
- **`node tools/pageshot.js . index.html <prefix> [--headed] <steps…>`**: Playwright + Chromium renders the **real pages including DOM, HUD and CSS** (use it for UI work). Headless = SwiftShader (CPU); `--headed` = the real GPU. Every shot prints the WebGL renderer it came from. The Linux container lacks Arial Black / Courier New / Segoe Print, so fonts differ from Brave there.
- **`node tools/battery.js .`**: checks 3–4 as JSON (`gate.mjs` wraps it). **`sim.js`**: headless 90 s match (`node tools/preview.js . eval tools/sim.js`). **`aabbs.js` + `village.py`**: map layout export + validation (`python tools/village.py`, paths in its header).
- Node flags that paid off: `--expose-gc` (soak), `--allow-natives-syntax` (`%GetOptimizationStatus`), `--trace-deopt` / `--trace-compilation-dependencies` (why a function won't stay optimized).

**Final gate: Mustafa's Brave screenshots** — the only real view of the HUD/DOM and of WebGL2. Perf claims about the browser need his DEV-panel FPS readout.

## 6. Key systems + grep targets

**core** — tuning/arsenal: `const T =`, `const WEAPONS`, `const UNIT_TYPES`, `const VEHICLE_TYPES` · look: `function toonify`, `function addHull`, `function ensureOutlineNormals`, `function makeOutlineMat`, `function stylizeOne`, `INK`, `WORLD_OUTLINE`, `CHAR_OUTLINE`, `OUTLINE_GROUND_Y`, `const ENV_DEFAULT`, `function applyEnv` · characters: `function buildEntity`, `function assembleChar`, `const CPARTS`, `const FACTION_PALETTES`, `function applyFaction`, `function applyVillain` · guns: `const GUN_KIT`, `function buildGunModel` · kit/map: `const BK`, `DM_Cabana` · draw calls: `function mergeStaticGroup`, `function _matSig` · math/SIM: `function hyp2`, `function rayAabb`, `function rayStatic`, `function resolveShot`, `function hasLOS`, `const SIM` · persistence: `const KVStore`.

**game** — `const PLAYER_FACTION` (top of file) · animation: `const ANIM`, `function updateSoldierAnim`, `function _animInit`, `animHit`, `animThrow`, `deathStart`, `animDeath`, `function setHeldGun`, `function playerFireOrigin` · combat: `function fireOnce`, `function castShot`, `function damageEnemy`, `function damagePlayer`, `function playerDie`, `startCharge`/`releaseCharge` · FX: `const FXPool` (optional per-kind caps), `function spawnDamageNumber` (`_dnPool`), `function vigShot` (GPU capture; `vigShotReadback` fallback) · world: `function resolveObstacles`, `function syncSimWorld`, `function updateOcclusion`, `function _occTest`, `function buildMap`, `function clearMap` · bots: `const NAV`, `navBuild`, `navPath`, `navSteer`, `function setEnemyType` · dispatch: `function loop()`, `function update(dt)`, `function deploy`, `function bootGame` · DOM: `_sd`/`_so`/`_st` (write only on change) · dev: `devCycleGun` · the **let warm-up block** at the end of the file.

**editor** — `function enterEditor`, `function updateEditor`, Character Lab `clRebuild`/`clTick`, Anim Lab `alTick`/`AL_ACTIONS`/`alEnsureUI`, `function bootEditor`.

**Pages** — the inline script is only `bootGame();` [`bootEditor();`] + the menu tail. CSS and DOM (menus, HUD) live in each page's `<head>`/`<body>` — grep the full HTML. The UI transplant prefixes its classes and keyframes with **`wr-`**.

## 7. Known fracture points + hard-won lessons

- **Mis-bucketing:** `placeCursor` once sat in the editor file while `loop()` called it every frame → `index.html` died on frame 1. Check 4 exists for this.
- **Temporal dead zone:** `PLAYER_FACTION` was first declared mid-file, but the player is built while the script loads → crash at boot. Now at the top of the file; the boot test caught it.
- **Stacking context:** animating `transform` on an element turns it into a stacking context, so its `::before` with `z-index:-1` (the halftone cel shadow) paints **over** its background. Animate a wrapper (the screen), never the sheet.
- **V8 behaviour (measured):** `Math.hypot` allocates on every call; large hot functions run out of the inlining budget, so floats crossing helper calls get boxed (inline the math in tight loops); a top-level `let` reassigned for the first time deopts every function that baked it in (hence the warm-up block); fields added at runtime change object shapes.
- **Materials:** `toonify` gives every mesh its own material, so merging groups by material *content* (`_matSig`), never by reference.
- **Hit-stop:** while `hitStop > 0`, `loop()` reuses the shadow map; vignette shots render into a canvas corner **before** the main render.
- **Pathfinding grid:** built at boot (cold) and rebuilt at `deploy` (~40 ms warm); `clearMap` invalidates it. A map change without a deploy leaves the bots on the old grid.
- **Headless ≠ Brave:** the software renderer makes CPU/FPS numbers pessimistic and fonts differ; the browser verdict comes from Mustafa.

## 8. Current state + in-flight (do NOT clobber this)

**Built and confirmed in Brave:** Living Comic look (colour-true output, nature gain, contact ink); the Normandy village map; `KVStore` persistence; **ANIM v2** (gun-first IK holds from `GUN_KIT`, planted feet with a procedural ankle, stride sized from leg reach, greatcoat panels, per-gun reloads, flinch, toy-soldier deaths, weapon draw, throws, bayonet thrust, katana + shield on IK); the stable fire origin; the **Anim Lab**; the DEV all-guns cycle (Unlock All → wheel / 2); the **14-change-set audit** (`AUDIT_BUGS_PERF.md`: kill-vignette hitch fixed, GC −39 %, change-only DOM writes, pooled FX + damage numbers, dead code removed, oriented-box shots/LOS, static kit pieces merged by material — village draw calls 398 → 202, bot pathfinding, two UI families); Juggernaut in the faction livery.

**In flight — UI theme transplant** (approved mockup: `ui_theme_mockup.html`): step 1 (main menu → dossier, pause → clipboard, field-manual mode) delivered by the UI chat, **awaiting Mustafa's Brave check** (incl. menu FPS in the DEV panel). Next: step 2 service record + end of round, step 3 armory + scoreboard (dark crate family).

**Open issues found by the UI chat (each its own change-set):**
- The first-deploy tutorial lists controls that no longer exist (V melee, B shield, 1–5 switch) → replace it with the clipboard's field manual (one controls list).
- Editor: opening it throws a `refreshPrefabList` error (declared inside `bootEditor()`, called outside) — a one-line fix.
- The 3D scene renders every frame behind the opaque menu — skip it on the menu.

**Kept on purpose (with reasons in the audit):** `makeInputCommand` (multiplayer groundwork), ~4.9 MB/s residual GC churn (optional `updateSoldierAnim` split), `update()` staying in V8's baseline tier.

**Next on the roadmap:** finish the UI transplant → graphics Upgrade A (comic cast shadows → painted ground) → Upgrade B "Comic Impact" (pool its effects there) → character rework toward Foxhole-like proportions (the mockup's service-record portrait is the style target) → progression design (Identity / Momentum / Mastery; Momentum persistence still undecided) → gun tiers T1–T4.

**Design-only:** progression, gun tiers, the `UNITS.md` roster, aircraft, multiplayer (the deterministic SIM is groundwork).

## 9. Working with Mustafa

- First-year game design student (BUas); plays in **Brave on Windows**; his screenshots are the final gate for anything visual.
- **One change-set at a time, verified before the next. Fix bugs before features.** Root-cause by reading and measuring, not guessing.
- **Think with him on design:** propose, justify, push back with reasons; be honest about limits and trade-offs. He dislikes X-or-Y questions — recommend, then let him veto.
- **Answers:** digestible, scannable, plain language; **decimals use periods**.
- **Delivery:** commits on the change-set branch, an advisor report in `docs/reports/` (`/advisor-report`); terse `ROADMAP.md` entries when meaningful work lands (hygiene rules at the top of ROADMAP).

### Provenance
Rewritten 2026-10-02 from the live build (end of the animation + audit chat) after the original went missing from Project Knowledge. §2–§5 and §7 are verified against the code and this session's measurements; §8's UI-chat items are as reported by that chat. §1 and §5 rewritten 2026-10-13 (Phase 0) for the git + Claude Code workflow; §5's commands were run in the Linux container that day.
