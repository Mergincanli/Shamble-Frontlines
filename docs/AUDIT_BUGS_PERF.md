# Bug & optimization audit — 2026-09-26

Process: deep dive (evidence) → list → review the list → plan → review the plan → execute (one change-set at a time,
four-check battery + a targeted test per fix). Every item below cites how it was found.

## 1. Deep dive — what was run
| Probe | Result |
|---|---|
| Static per-frame scan (acorn call graph from `loop`/`update`; allocation / DOM / traversal sites) | 210 functions reachable per frame, 104 with sites |
| Runtime profile (scripted 60 s combat: auto-fire, grenades, abilities, class switches) | `update()` 0.49 ms/frame CPU; soldier anim 0.18 ms for 7 |
| V8 sampling heap profiler incl. collected garbage (20 s combat) | **~8 MB/s of garbage** (≈140 KB/frame) |
| V8 optimization status (natives syntax, after 30 s) | `update`, `updateAim`, `placeCursor`, `syncSimWorld`, `updateTechBox` **never reach TurboFan** |
| Kill-frame inclusive timing (every game function wrapped) + code read of the vignette | see B1–B3 |
| Fuzz: every class + ability, reloads, grenade, bow, Juggernaut spawn/enter/fire/exit, death/respawn, bot kills/respawns | **0 errors** |
| Soak: 3 min, class switches, forced GC | heap 33.1 → 34.2 MB, scene +63 objects (bounded gun-model caches) — **no leak** |
| Draw calls (main + shadow) at heavy views | village 303 · manor 238 · centre 170 · base 140; ≤ 12k triangles |
| Suspected: hit flash blacks out the elite visor | **not a bug** (visor is `MeshBasicMaterial`, no emissive) |

## 2. The list
| ID | Issue | Evidence | Impact |
|---|---|---|---|
| **B1** | **Kill-vignette hitch.** Each of 3 shots: full scene render → `readRenderTargetPixels` (synchronous GPU stall) → 2 × ~0.5 MB allocations → JS Y-flip loop → `putImageData` → `comicPost` (`getImageData` = second stall, JS Sobel filter, `putImageData`). Plus `void vigEl.offsetWidth` forces a full layout on the kill frame. The earlier stagger spread the stalls over 3 frames; it removed none. | code read (`vigShot`, `comicPost`, `triggerVignette`) | **High** — every kill |
| **B2** | New canvas + texture built for **every comic word** on a kill | kill-frame timing: `makeComicTex` 1.1 ms | Medium |
| **B3** | Damage-number popup is the costliest kill-frame call | kill-frame timing: `spawnDamageNumber` 4.2 ms (headless) | Medium — every hit |
| **P1** | **GC churn ~8 MB/s** → periodic minor GCs = micro-stutter. By allocator: `updateOcclusion` 17 %, `updateSoldierAnim` 17 %, `_occTest` 13 %, `resolveShot` 13 %, `Math.hypot` 12.5 % (allocating builtin, 15+ hot call sites), `resolveObstacles` 7.5 %, `Quaternion.invert` (anim) 5 % | sampling heap profiler | Medium |
| **P2** | `update()` and 4 others stay in V8's baseline tier (never optimized) → every float operation boxed | natives optimization status | Medium (feeds P1) |
| **P3** | ~30 DOM writes per frame in `update()` (`display` / `opacity` / `textContent`), most unconditional → style/layout work each frame | static scan (L2420–2722) | Low–medium (browser only) |
| **P4** | FX create a new Mesh + Geometry + Material per event (`scuffTick`, `spawnDust`, `spawnShock`, `spawnScorch`, grenades) → GPU buffer churn | static scan | Low–medium (explosions) |
| **P5** | Draw calls ~300/frame in the village (driver overhead on weak machines) | renderer.info | Medium on weak PCs |
| **P6** | Each soldier caches up to 5 gun models (hidden) | soak +63 objects, bounded | Low |
| **K1** | Juggernaut livery hardcoded red + gold (ignores faction); stale "STUB" comment in `spawnVehicle` | known, grep | Low (visual) |
| **K2** | Dead/dormant code: javelin charge path, legacy weapon-buy helpers | known | Low |
| **K3** | Bots: no pathfinding / stuck recovery | known | Design-level |
| **K4** | Rotated objects keep an axis-aligned collider | known [assumed] | Low |
| **K5** | Mixed UI theme (parchment vs dark-olive modals) | known [assumed] | Cosmetic |

## 3. Review of the list
- **Root causes, not symptoms:** B1 is one root cause (CPU↔GPU round trips + CPU image filtering), not "slow vignette". P1 and P2 are the same family (allocation from boxed doubles + allocating builtins + per-call objects) — fix the named allocators first, then re-measure P2.
- **Grouped by frame:** B1 + B2 + B3 all land on the kill frame → one change-set ("kill-frame budget").
- **Measurable vs browser-only:** P1/P2/P4 are measurable headless (allocation rate, opt status). B1's GPU copy and P3's layout cost only show in Brave → each gets a fallback + Mustafa's perf-overlay check.
- **Out of scope for a bug run (design work, tracked in ROADMAP):** K3 pathfinding, K4 rotated colliders, K5 UI theme.
- **Deliberately deferred:** P5 draw-call merging — merging must stay per-house because the occlusion fade and the editor pick per mesh; it deserves its own change-set with before/after renders. P6 is bounded; accepted. K2 dead code: remove only when it next gets in the way (risk > value now).
- **Checked and cleared:** runtime errors (fuzz 0), leaks (soak), elite visor.

## 4. The plan
| CS | Scope | Verification |
|---|---|---|
| **CS1 Kill frame** | B1: capture each shot by rendering into a corner of the main canvas and `drawImage` it into the panel canvas (GPU copy, no readback; the old path stays as a fallback), comic look via a CSS filter instead of the JS Sobel pass, restart the CSS animation without a forced layout. B2: cache comic-word textures. B3: read `spawnDamageNumber`, pool its DOM nodes. | harness: kill-frame CPU before/after with stubbed panel canvases; battery; **Brave perf overlay during kills** |
| **CS2 GC churn** | P1: `Math.hypot` → `Math.sqrt` in hot paths; remove per-call objects/arrays in `updateOcclusion` / `_occTest` / `resolveShot` / `resolveObstacles` (read each first); anim quaternion inversions. Then P2: re-check opt status; if `update()` is still baseline, split it into sub-functions. | sampling profiler (target < 2 MB/s), opt status, sim A/B, anim metrics unchanged |
| **CS3 DOM writes** | P3: cache last values, write only on change | static re-scan; Brave |
| **CS4 FX pooling** | P4: shared geometries + pooled meshes for scuff / dust / shock / scorch | soak + allocation rate |
| **CS5 Juggernaut** | K1: faction livery via the palette; fix the stale comment | render + Brave |

## 5. Review of the plan
- **Coverage:** every list item is in a CS or deferred with a reason (§3). ✔
- **Order:** visible bug first (CS1), then stutter (CS2), then cheap wins (CS3, CS4), cosmetic last (CS5). ✔
- **Conflicts:** CS2's possible `update()` split touches the lines CS3 edits → do CS3's guards inside the same pass or before the split. ✔ (CS2 splits only after measuring.)
- **Risk:** CS1's GPU-copy path can't be exercised headless → keep the readback path as an automatic fallback; Brave is the gate. CS2 edits hot gameplay code → sim A/B + the anim metrics + fuzz after each step. ✔
- **Rollback:** each CS is snapshotted before editing. ✔

## 6. Execution log
**CS1 Kill frame — done (awaiting Brave).** Vignette shots now render into a corner of the main canvas and `drawImage` into
the panel (GPU copy; the readback path is the automatic fallback). Capture moved *before* the main render (it paints over the
corner). The shadow map is reused while the hit-stop freezes the scene (vignette + frozen frames skip the 2048² shadow pass).
No forced reflow on the kill frame. Comic-word textures pre-warmed at load (a kill now builds 0). Panel look: CSS
`contrast/saturate` instead of the JS edge filter (the shot already carries the game's ink). Headless: correct source rect,
viewport/scissor restored, capture-before-render and hit-stop shadow reuse verified; the removed CPU work alone was ~43 ms per
shot in the software renderer. **B3 deferred after reading the code:** one small div per hit with a 40-node cap — pooling would
need a two-frame transition reset per reuse; not worth the risk for the cost.

**CS2 GC churn — done.** All 25 `Math.hypot` calls in core + game → `hyp2`/`hyp3` (AST rewrite, arguments untouched):
allocation **8.0 → ~6.3 MB/s** (A/B, noise ±0.5). Found why `update()` never optimized: V8 *const-tracking let* deopts (a
top-level `let` reassigned for the first time — first kill, first vehicle… — invalidates every optimized function that baked
it in). A type-preserving one-time reassignment of all 119 top-level lets at load removes them (**0 const-tracking deopts**,
allocation-neutral; a first sentinel version was measured +2 MB/s and replaced). Remaining `update()` invalidations are
"code dependencies" from objects gaining fields at runtime (normal JIT warm-up; pre-declaring those fields is a later cleanup).
**Stopped deliberately:** the residual (~6 MB/s ≈ one ~1 ms young-GC every ~3 s) comes from boxed float returns in the SIM ray
primitives and three.js `Matrix4.invert` — changing the shared SIM ray API is more risk than this gain. Anim metrics, fuzz
(0 errors), soak (flat) and a headless match unchanged.

**CS1 + CS2 confirmed in Brave by Mustafa (2026-09-26).**

**CS3 DOM writes — done.** The 18 per-frame `display` / `opacity` / `textContent` writes in `update()` go through
`_sd` / `_so` / `_st` (AST rewrite; element and value expressions verbatim). They compare against the element's current
value, so writes made elsewhere can't desync them. Measured in combat: ~3 helper calls per frame, **0.01 real writes per
frame** — ~99.7 % of those writes were identical values, now skipped.

**CS5 Juggernaut — done.** Built in the player's faction livery (hull = `FactionPrimary`, accents = `HighlightTrim`); new
`PLAYER_FACTION` is the single source for the player's uniform and vehicles; the stale "STUB" comment is replaced. The boot
test caught a temporal-dead-zone error in the first placement (the player is built while the script loads) — the constant
now sits at the top of the file. Render: Allies blue.

**CS4 FX pooling — deferred (after reading the code).** The unpooled spawners (scuff, dust, shock rings, scorch) fire only on
explosions, a handful of objects per event; each FX kind scales its mesh differently in the update loop, so sharing geometry
needs per-kind visual checks. Upgrade B "Comic Impact" rebuilds these exact effects → pool them there.

**Run complete.** Open, with reasons: P5 draw-call merging (per house, occlusion-aware — own change-set), CS4 (→ Comic Impact),
residual GC churn (SIM ray API boxing — not worth the risk now), runtime-added object fields (pre-declare to settle the JIT
sooner), K2 dead code, K3 pathfinding, K4 rotated colliders, K5 UI theme.

## 7. Phase 2 — "do all of it" (Mustafa, 2026-09-26)
Every open item from §6, re-listed, planned in order of risk × certainty, reviewed against the code before each step.

| CS | Item | Plan | Status |
|---|---|---|---|
| **CS6** | FX pooling (was CS4) | Scuff / scorch / dust / shock rings join the existing `FXPool`: shared unit geometries, one material per slot; dust scales from its base radius (`r0`). `FXPool` gains an optional per-kind cap (acquire → `null` when full); scuffs capped at 40 (they peaked at 103). | **Done** — every slot returns to its pool; visuals A/B-identical; fuzz 0 |
| **CS7** | Damage numbers (was B3) | Fixed pool of 40 DOM nodes; reuse restarts with the transition off, fade re-enabled next frame; re-attaches if the layer is emptied. | **Done** — 60 spawns through 40 distinct nodes, no errors |
| **CS8** | Runtime-added fields | Pre-declare in the literals: enemies `_aimT, _tx, _tz, _fOn`; player `lastAttacker, _tx, _tz, _stepT` (defaults match how the code reads them). | **Done** — fuzz 0, soak flat |
| **CS9** | Dead code (K2) | Reference scan of all 546 functions (code + both pages + string literals) and the javelin path traced end to end (no weapon has `thrown`, so `startCharge('jav')` could never fire). Removed by exact AST ranges: the javelin path (registry, spear builder, throw, per-frame loop, 5 branches) + 11 unreferenced functions (`buildCharacter` shim, `makeSoldier`, `aimColliders`, `_jugFireBarrel`, `toggleShield`, `buyShield`, `throwGrenade`, `doMelee`, `showXpPop`, `levelUpRifle`); kept `makeInputCommand` (multiplayer groundwork). | **Done** — check 3 shows exactly the 14 intended drops; re-scan finds no cascade; fuzz 0, match clean |
| **CS10** | Rotated colliders (K4) | Recon: **movement already used oriented boxes** (`resolveObstacles`; the "[assumed]" doc note was wrong for movement); **shots + line of sight did not** — the SIM got each rotated obstacle's loose world AABB. New core `rayStatic`: oriented-box ray test (ray into the box frame, normal back to world, same transform as movement) used by `resolveShot`, `hasLOS` and game's other static ray; `syncSimWorld` gives 45° obstacles their box. The shipped map has none (0/24 obstacles, 0/11 houses) — this fixes editor-made maps. | **Done** — isolated 45° crate: corner shot old *hit empty air* → new passes; face shot old 3.586 (loose box) → new 4.586 (true edge); LOS across the corner old blocked → new clear; movement unchanged |
| **CS11** | Residual GC + `update()` tier | **Hypotheses tested, not assumed:** boxed float *returns* — disproved (`resolveShot` allocates the same ~1.5 KB/call when every ray misses and returns an integer); non-uniform SIM slot shapes — disproved (uniform slots: no change, reverted). **Bisect + V8 traces → root cause:** V8's cumulative inlining budget runs out in big hot functions, so helper calls stay out-of-line and every float argument/return crossing them is boxed. **Fixed the biggest allocator:** the camera-occlusion test (52+ calls/frame) now uses an inline slab test — 20,000/20,000 random cases identical to the old test. **`update()` tier:** its optimized code rests on ~2,600 map / field-constness / property-cell dependencies; rarely-written config/state fields invalidate it during play (normal JIT warm-up at scale; ~0.5 ms/frame even unoptimized) — documented, no change. | **Done** — allocation **6.3 → ~4.9 MB/s** (2 runs; −39 % since the audit began); occlusion gone from the top-10 allocators. **Optional next:** split `updateSoldierAnim` into smaller functions (expected −1.5 MB/s) — held because the animation is Brave-confirmed and the gain is small |
| **CS12** | Draw calls (P5) | New core `mergeStaticGroup`: after a static kit piece (house, cover, prop, tree) is inked, its meshes fold into one mesh per (material content, outline weight, shadow flags, uv, raycast), transforms baked, one hull each. **First attempt merged nothing** — measured: `toonify` gives every mesh its own material, so grouping by reference never matched → grouped by material *content* (`_matSig`; custom shader hooks never shared). Pieces stay groups; collider bodies stay unmerged; transparent parts skipped. | **Done** — meshes 1,611 → 370 (houses −60 %, covers −79 %, props −82 %, trees −75 %); draw calls village **398 → 202**, manor **284 → 162**; renders pixel-identical (0.00 % differ); colliders identical; editor pick intact. Trade-offs: merged pieces cull as a unit (rendered triangles up to +12 %, still < 18k); +150 ms once per map build |
| **CS13** | Bot pathfinding (K3) | NAV module (game): 2 m grid built with the bots' own collision (`resolveObstacles`, cell centre tested at the bot radius 0.7 + half a cell since bot-poplar-wedge, 2026-10-14; was 0.75) — warmed at boot, rebuilt warm at deploy (~40 ms), invalidated by `clearMap`; A* (8-neighbour, octile, no corner cutting) on typed arrays, zero allocation per search (0.26 ms each, ≤ 2 per frame); string-pulled waypoints; `navSteer` walks straight when clear and follows the route only when not; stuck detection repaths. Wired into the combat approach and the tech-box, harvester and siege branches. | **Done** — synthetic 30 m wall: straight steering stuck 25 s → routed in 9.9 s; headless match stall rate old 11.0 / 11.2 / 7.4 % → new 9.5 / 3.4 / 5.8 % (mean 9.9 → 6.2 %) |
| **CS14** | UI theme (K5) | From Mustafa's screenshots (the previewer can't render HTML/CSS): paper = main menu + pause; dark olive = armory, career, scoreboard, victory, HUD, dev — Career opened as a dark box over paper screens, the end screen was a third style, and the menu blurb still described the removed Javelin. Decision: two deliberate families sharing one font + palette — **paper** for menu-context screens (title, pause, career, end of round), **dark field-kit** for overlays over the live game (armory, scoreboard, HUD, dev: dark stays readable over the bright world). Career and the end screen restyled to paper with the existing tokens (`--paper`, `--olive`, `--red`, `--hostile`); blurb rewritten for classes. CSS-only, both pages. | **Done** — awaiting Mustafa's Brave screenshots (DOM isn't visible headless) |

**Review of the plan:** order = small/certain first (CS6–9), then correctness (CS10), then measurable perf (CS11–12), then the
two feature-sized items (CS13 pathfinding, CS14 UI). CS10 and CS11 both touch the SIM ray code → CS10 first, CS11 builds on it.
CS12 must keep per-house meshes (occlusion + editor). Every CS keeps the four-check battery + fuzz + a targeted test.

**Found along the way:** `update()` still stays in V8's baseline tier after a natural 30 s warm-up; its remaining
invalidations are "code dependencies" → folded into CS11(b).

**Audit complete (2026-09-26):** CS1–CS14 done. Kept on purpose, with reasons above: `makeInputCommand` (multiplayer groundwork), the residual ~4.9 MB/s GC churn (optional anim split), `update()` staying in V8's baseline tier (warm-up at scale).
