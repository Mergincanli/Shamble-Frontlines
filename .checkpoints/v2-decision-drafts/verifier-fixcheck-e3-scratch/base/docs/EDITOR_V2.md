# EDITOR V2: the map editor, rebuilt (draft spec)

Written by Mustafa and the Advisor, October 2026. Status tags as in LOOP_V2.md: `[DECIDED]`, `[DEFAULT]` (stands
unless Mustafa vetoes), `[OPEN]`, `[LATER]`. Scope: the **map editor only**. The Character Lab and Anim Lab stay as
they are.

## 1. Why rebuild
- Today, every edit and every undo rebuilds the whole map. That is slow, it leaks, and objects lose their identity
  between rebuilds. It caused the lost shading and doubled trees fixed this week, and likely today's bugs: placed trees
  can't be deleted, rotation does nothing, and the editor lags after a few placements.
- Map data and scene objects are tangled together, and static merging (built for the game's performance) runs while
  editing, where objects need to stay individually selectable.
- The new loop needs tools the old editor never had: three teams, verticality, objectives as data, validation.

## 2. The five architecture rules `[DEFAULT]`
1. **The document is the truth.** A map is plain, versioned JSON. Every entity has a stable ID. Edits change the
   document; nothing else is saved.
2. **Every edit is a command** with do and undo: add, delete, transform, set a property, or a batch of these. Undo
   reverts one command; it never rebuilds the map. The history has a fixed size limit.
3. **The scene mirrors the document, one entity at a time.** Adding, changing or removing an entity touches only its
   own 3D object, and whatever is removed is disposed. Shared materials and textures are cached and reused, never
   rebuilt per edit.
4. **Edit mode shows real, unmerged objects.** Static merging happens only when you press Test, and in the game.
   Repeated objects (trees, props) may use instancing to stay fast.
5. **The document layer knows nothing about THREE or the page**, so it can be tested in Node: commands, undo/redo,
   save/load, validation.

## 3. Features
- `[DECIDED]` **Parity:** everything the current map editor does that Mustafa marks "keep" in the inventory (§6, step 1).
- `[DEFAULT]` New:
  - **Select and transform:** click to select, multi-select, a move/rotate gizmo plus a numeric inspector. Snapping to
    the grid and to 15° / 45° / 90°. Copy, paste, duplicate, delete.
  - **Verticality:** plateaus, ramps, cliff edges and drop-downs. Height is read through brightness, as in the greybox
    rule. Layers can be shown and hidden.
  - **Three-team symmetry:** build one third of the map and it is mirrored live to the other two at 120°. Bases are
    assigned per team.
  - **Objectives as data:** spawns, barracks and terminals, harvest fields (the centre and the safe ones), uplink sites,
    cache spawn points.
  - **Validation panel:** bot reachability (every spawn to every goal), the 6 m gap rule, a symmetry check, and budgets
    (object count, draw calls when merged).
  - **Test:** play the map with bots instantly, then return to the editor exactly where you were.
- `[LATER]` Enterable structures (interiors), terrain sculpting beyond plateaus and ramps, a prefab library v2.

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
