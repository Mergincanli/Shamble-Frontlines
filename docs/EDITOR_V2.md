# EDITOR V2: the map editor, rebuilt (draft spec)

Written by Mustafa and the Advisor, October 2026. Status tags as in LOOP_V2.md: `[DECIDED]`, `[DEFAULT]` (stands
unless Mustafa vetoes), `[OPEN]`, `[LATER]`. Scope: the **map editor only**. The Character Lab and Anim Lab stay as
they are.
Updated on 2026-11-05 with Mustafa's editor decisions: §3's block dated 2026-11-05, and the pointers to it.
Updated on 2026-11-07 with Mustafa's VFX editor item of 2026-11-06, a future tool after the map features: §7.

Principle (Mustafa, 2026-11-05), a lean editor: §3, Decided 2026-11-05.

## 1. Why rebuild
- Today, every edit and every undo rebuilds the whole map. That is slow, it leaks, and objects lose their identity
  between rebuilds. It caused the lost shading and doubled trees fixed this week, and likely today's bugs: placed trees
  can't be deleted, rotation does nothing, and the editor lags after a few placements.
- Map data and scene objects are tangled together, and static merging (built for the game's performance) runs while
  editing, where objects need to stay individually selectable.
- The new loop needs tools the old editor never had: three teams, verticality, objectives as data, validation.

## 2. The five architecture rules `[DEFAULT]`
1. **The document is the truth.** A map is plain, versioned JSON. Every entity has a stable ID. Edits change the
   document; nothing else is saved. (Mustafa's stamps, trace sketches, sticky notes and snapshots bear on it: §3,
   Decided 2026-11-05.)
2. **Every edit is a command** with do and undo: add, delete, transform, set a property, or a batch of these. Undo
   reverts one command; it never rebuilds the map. The history has a fixed size limit.
3. **The scene mirrors the document, one entity at a time.** Adding, changing or removing an entity touches only its
   own 3D object, and whatever is removed is disposed. Shared materials and textures are cached and reused, never
   rebuilt per edit.
4. **Edit mode shows real, unmerged objects.** Static merging happens only when you press Test, and in the game.
   Repeated objects (trees, props) may use instancing to stay fast. (Mustafa's tool order (2) bears on it: §3, Decided
   2026-11-05.)
5. **The document layer knows nothing about THREE or the page**, so it can be tested in Node: commands, undo/redo,
   save/load, validation.

## 3. Features
- `[DECIDED]` **Parity:** everything the current map editor does that Mustafa marks "keep" in the inventory (§6, step 1).
- `[DEFAULT]` New:
  - **Select and transform:** click to select, multi-select, a move/rotate gizmo plus a numeric inspector. Snapping to
    the grid and to 15° / 45° / 90°. Copy, paste, duplicate, delete. (Mustafa decided each object's width, length and
    height, set with drag handles or typed numbers, plus how high it sits, and primitives that snap to gameplay sizes:
    §3, Decided 2026-11-05.)
  - **Verticality:** plateaus, ramps, cliff edges and drop-downs. Height is read through brightness, as in the greybox
    rule. Layers can be shown and hidden.
  - **Three-team symmetry:** build one third of the map and it is mirrored live to the other two at 120°. Bases are
    assigned per team. (Mustafa's plan view and fairness panel work under 120° mirroring: §3, Decided 2026-11-05.)
  - **Objectives as data:** spawns, barracks and terminals, harvest fields (the centre and the safe ones), uplink sites,
    cache spawn points. (Mustafa's tool order (3) and (4), the heatmaps and the fairness panel, read the objectives:
    §3, Decided 2026-11-05.)
  - **Validation panel:** bot reachability (every spawn to every goal), the 6 m gap rule, a symmetry check, and budgets
    (object count, draw calls when merged). (Mustafa's tool order (3) to (5), heatmaps, the fairness panel and
    sightline checks, bear on it: §3, Decided 2026-11-05.)
  - **Test:** play the map with bots instantly, then return to the editor exactly where you were. (Mustafa's tool order
    (2) bears on it: one key drops you in at the cursor with bots, and you can keep editing while the match runs: §3,
    Decided 2026-11-05.)
- `[LATER]` Enterable structures (interiors), terrain sculpting beyond plateaus and ramps, a prefab library v2.
  (On the prefab library: Mustafa decided instances (stamps), which bear on it: §3, Decided 2026-11-05.)
- **Decided 2026-11-05 (Mustafa, after his talk with the Advisor).** His editor decisions, in his order and in his own
  words, with his marks; the plan view (his B.1 of 2026-11-03) sits after the views line, with its own date:
  - `[DECIDED]` Decided, the tool order: (1) greybox now, art later: you build with blocks, and the same layout swaps to
    real art with one switch, like modular kits; (2) play instantly: one key drops you in at the cursor with bots, and
    you can keep editing while the match runs; (3) bot test runs with heatmaps (where fights and deaths happen, how long
    each team needs to reach each objective); (4) a fairness panel (each team's distance and time to every objective
    under 120° mirroring); (5) sightline checks from the game camera, last, as a balancing tool.
  - `[DECIDED]` Decided: all the basic primitives (cube, sphere, cylinder, plane and the like), with one consistent
    scale across the engine. Principles: 1 unit = 1 metre; textures tile in world space (they repeat per metre and never
    stretch); primitives snap to gameplay sizes (exact numbers later, once they can be tested in the editor).
  - `[DECIDED]` Decided: every object has width, length and height, set with drag handles or typed numbers, plus how
    high it sits. The old editor lacked this.
  - `[DECIDED]` Decided: the plan view looks straight down, flat (orthographic, like a blueprint); the 3D view flies
    freely; one key shows the real game camera angle.
  - `[DECIDED]` **Mustafa, 2026-11-03 (his B.1):** "Plan view: draw to build". Two views of the same map document side
    by side: a top-down plan where you draw outlines on a grid (lines, rectangles, hexagons, octagons, free polygons),
    and the 3D scene, which builds itself live from the drawing. A drawn outline is a footprint with a height and a kind
    (wall, building, floor, cover, ramp, zone); the kind decides what it becomes, e.g. a building footprint becomes a
    building-kit house with floors and a roof. Anything can be moved, rescaled and reshaped in either view; both stay in
    sync because both read the same document. Works with three-team mirroring (draw in one third; the others follow at
    120°) and with floors. Precedents: Hammer's 2D views, Doom-style sector editors, SketchUp's push-pull. Built with
    Editor v2 (slice 5); no code now.
  - `[DECIDED]` Decided: trace sketches (a photo of a paper sketch under the plan view, to draw over); sticky notes on
    the map; snapshots, kept lightweight (they store only what changed).
  - `[DECIDED]` Decided, lower priority: a travel-time ruler showing metres and seconds, using the game's own movement
    data for soldiers, vehicles and leapers.
  - `[DECIDED]` Decided: instances (stamps) are an optional, top-level feature with three edit levels: all copies; a
    tagged group (e.g. "room A") for variations and art passes; one copy as an override (like Unity's prefab variants
    and overrides). A stamp can also be a plain, unlinked copy.
  - `[DECIDED]` Principle: a lean, solid editor, planned and built with the same care as the game; no bloat.

## 4. Quality bars `[DEFAULT]`
- Smooth with the full Cabana map loaded, on Mustafa's laptop.
- Memory flat across 1,000 edits and undos (a soak test).
- 1,000 random commands undone and redone return a byte-identical document.
- Every v1 map loads through a migration; saving and loading round-trips exactly.

## 5. Constraints
- It still runs by double-clicking editor.html. The player build (index.html) must never load editor code.
- Written so it ports cleanly if ADR-002 moves the project to modules. The editor is a natural first module.

## 6. Order of work (for Claude Code)
0. **Diagnose today's three bugs** (read-only): the root cause of each, with a reproduction, and whether this week's
   PRs caused it. Hotfix only if the fix is small and safe, so the current editor works in the meantime.
1. **Inventory** the current map editor's features. Mustafa marks each one keep, drop or improve.
2. **Architecture plan:** the document schema v2, commands, scene sync, the migration from v1, and the test plan.
3. **Build in slices**, one change-set each, while the old editor stays available until v2 reaches parity:
   1. The document and commands, tested in Node.
   2. Scene sync, selection and transform.
   3. Placement tools: place, delete, rotate, snap.
   4. Save, load and migration.
   5. Verticality.
   6. Symmetry and objectives.
   7. Validation and Test.

Mustafa's tool order and his other editor decisions (§3, Decided 2026-11-05): the architecture plan (step 2) maps them
onto these slices, for his approval. His "slice 5" in the plan view (2026-11-03) is the v2 playable slice 5, "Octopon-3
and Editor v2" (SLICE-ORDER, ADR 0002:34-35), not build slice 5 here.

## 7. A future tool: the VFX editor (after the map features)
`[LATER]` **A VFX editor**, as a future tool, after Editor v2's map features (Mustafa, 2026-11-06). Decided principles,
design later:
- `[DECIDED]` Essential: the game's punch depends on it. Inspired by Unreal's Niagara (emitters with a stack of modules:
  spawn, velocity, colour over life, size over life, gravity, collision), but without its clutter.
- `[DECIDED]` What you see is what you get: the editor uses the game's own renderer and lighting, and saves the same
  effect data the game loads, triggered by the game's events (hits, blasts, deaths).
- `[DECIDED]` Test in context: trigger a shot or blast in a live preview, from the real game camera, and keep editing
  while it plays.
- `[DECIDED]` Easy to pick up: each effect opens with a few big sliders (size, intensity, colour, duration); the full
  emitter and module stack folds open on request. One clean panel; curves edited inline. Presets to start from (muzzle
  flash, explosion, smoke, sparks).
- `[DECIDED]` Whole impact moments: one asset can bundle particles, sound, camera shake and hit-stop, timed together, so
  punch is authored in one place.
- `[DECIDED]` Faction colours: effects use the same role colour slots as units, so one explosion recolours per faction.
- `[DECIDED]` Cost and quality: each effect shows its cost against a budget, and has quality tiers (Low drops extras but
  stays readable).
- `[DECIDED]` Comic flavour: ink-outlined smoke, impact frames, halftone flashes, onomatopoeia.
- `[DECIDED]` A VFX style guide comes first, from a separate reference study, so the tool fits our look.
- `[DECIDED]` Built to iterate; no bloat.
