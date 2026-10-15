# Map editor v1: feature inventory

**Purpose.** This is EDITOR_V2 §6 step 1. Mustafa marks every row **keep**, **drop** or **improve** in the last column ("Mark"). v2 reaches parity when it covers every row marked keep or improve (EDITOR_V2 §3 names "keep"; an improve row is kept in a better form).

**Date:** 2026-10-15. **Code:** main at 67cf55b (`shamble-editor.js`, `shamble-game.js`, `editor.html`). All line numbers are main's. In the tables, `editor.js` means `shamble-editor.js` and `game.js` means `shamble-game.js`.

**How each state was checked:**
- **[S]**: real mouse and keyboard input (Playwright) on main. Headless Chromium with SwiftShader (software GPU), 1600×900, one run each.
- **[C]**: code reading only.
- **[D0]**: taken from the editor bug diagnosis (step 0).

The harnesses and logs are kept in the session scratchpad. Two independent sweeps went into this file: the first inventory, then a completeness review that corrected it. Every correction from the review is applied.

**The three reported bugs** are tagged **BUG-1**, **BUG-2**, **BUG-3** in the rows that touch them. "NEW-n" marks a bug found while building this inventory.
- **BUG-1**: placed trees can't be deleted. Rows C4, C15, F1, O1.
- **BUG-2**: rotation does nothing. Rows E3, G1, G2, G3, A12, O1.
- **BUG-3**: the editor lags after a few placements. Rows O1, N3, F1.
- Diagnosis summary ([D0]): all three predate PR #8. PR #8 removed the in-session tree and prop stacking. PR #10 removes stacked copies from old saves on Load. The rest goes to the planned editor-hotfix change-set.

**Old build vs main.** `shamble-editor.js` and `editor.html` are byte-identical in the older build (159bec4) and main. PR #8 changed 11 lines of `shamble-game.js`: runtime builders now call `stylizeOne`, and `clearMap` now clears trees and props (game.js:558-559). So the states below also applied before PR #8, with one exception: before PR #8, every undo, redo, Reset and Load added another copy of every tree and prop, which made BUG-1 and BUG-3 much worse [D0].

## A. Entry, layout, chrome
| ID | Feature | What it does | How to use | Where | State | v2 note | Mark |
|---|---|---|---|---|---|---|---|
| A1 | Open editor | Hides the HUD and bots, shows the editor, resets the camera position and height, clears the selection and undo history, turns the overlays off, sets the tool to Select. It **keeps** Ortho, grid size, rotation step and Mirror from the last visit (it never resets them) | Menu tab "Map editor" | editor.html:1045; editor.js:990, 224‑239 | works [S]; the kept settings are [C] | v2 should restore the camera and selection on re-entry | |
| A2 | Title + map name | Shows "SHAMBLE ▸ EDITOR ‹name›"; the name is set on Save or Load | — | editor.html:1053; editor.js:965, 1076 | partial [S]: shows "Cabana Republic" at boot, but the exported JSON name is always "DM_Romania" (game.js:636) | Store the name in the document | |
| A4 | Library collapse | Clicking the active library rail button hides the library column; clicking again restores it | Rail | editor.js:1173‑1176, 947 | works [S] | | |
| A5 | Inspector column toggle | Shows or hides the right column | Top-bar panel icon | editor.html:1079; editor.js:948, 1212 | works [S] | | |
| A6 | Status bar | Cursor x,z (snapped), map size, selected count, tool, hint, zoom | Passive | editor.html:1221‑1227; editor.js:676, 236, 839 | partial [S]: Zoom is always "100%" (never updated); map size is set only on entry; Tool shows internal ids ("Place · propCart"); the hint says "⇧-drag marquee", but a plain drag already draws a marquee | | |
| A7 | Tooltips | One floating tooltip for every `[data-tip]` (350 ms delay); fills in tips for cards that have none | Hover | editor.js:1197‑1208 | works [S]. **Eight** tips are wrong: Reset ("Clear the map back to empty"; it reloads Cabana), Play ("(Tab)"; Tab is the walk preview), and six cards that say "click; ⇧‑wheel to rotate": Manor, Hay Bales, Farm Cart, Log Pile, Water Trough, Vegetable Patch (editor.html:1146, 1178, 1179, 1181, 1183, 1184). Placing selects nothing, so ⇧‑wheel zooms instead (see C24) | | |
| A8 | Icon injection | Inline SVG icons from the IC table | Automatic | editor.js:1106‑1165 | works [S]; the `path` key is defined twice (1128 and 1132), so the first icon is dead | | |
| A9 | Shortcut card | Static cheat-sheet under the library | — | editor.html:1208 | partial [C]: lists "⇧Drag marquee" (wrong); missing Q/E/R/T, O, Esc, arrow pan, middle-drag pan, Backspace, the Backquote dev panel, Shift-fine nudge (0.25 m), Alt-fine (5° rotate), Ctrl+Shift+Z. The Snap button's own tip does mention Alt (editor.html:1068) | Generate it from the real keymap | |
| A10 | Card animation | Staggered fade-in when switching tabs | Tab click | editor.js:1193; editor.html:497 | works [C] | Cosmetic | |
| A11 | Dev panel over the editor | Backquote opens the game's dev panel in the editor too, because it is handled before the editor branch. The MAP slider resizes the arena (see hidden item 7). The camera sliders do nothing, because the editor rewrites the camera every frame. "⚒ Character Lab" switches mode, but **the editor UI stays on screen**, and leaving goes to the main menu, not back to the editor | ` (Backquote) | game.js:3004, 1922; editor.js:432‑443, 997, 672 | partial [S]: the panel opens in editor mode; after Character Lab the mode is "charlab" and `#editor` is still displayed. Camera sliders are [C] | Keep or drop; v2 should not reuse the game's dev panel | |
| A12 | Text selection in fields | The game blocks text selection, drag and double-click on the whole page. So Ctrl+A doesn't select the text in any editor field (inspector numbers, Search, map name, Seed). Per [D0], double-click still selects text. Drag-select was not checked | Ctrl+A in a field | game.js:3038 | broken [D0]: click the Rotation° field, Ctrl+A, type 90, Enter: the field and the object stay at −25.72; with the blocker bypassed they become 90. Part of **BUG-2** | Limit the blocker to the canvas | |
| A13 | Shortcuts while a dropdown has focus | The shortcut guard skips every `<input>` and `<textarea>` (game.js:3006), but not `<select>`. With a dropdown focused (Array direction, Building Maker pickers), editor shortcuts still fire: ArrowDown nudges the selection instead of changing the option, and Delete deletes the selection | Keys while a dropdown has focus | game.js:3006 | broken [S]: the option stayed "+X", crate z 10→11, crates 15→14; the Maker pickers are [C] | Guard every form control | |

Merged: A3 (tool rail) is now part of E10, because the rail buttons and the Q/E/R/T keys are the same mode switch.

## B. Camera and view
| ID | Feature | What it does | How to use | Where | State | v2 note | Mark |
|---|---|---|---|---|---|---|---|
| B1 | Pan | Moves the camera at 0.9 × camera height **per second** (81 m/s at the default height 90), as long as the frame rate stays at or above 20 fps | Hold W/A/S/D | editor.js:667‑670 | works [C] for the formula (editor.js:667, `edCamH*0.9*dt`). Each frame's time is capped at 0.05 s (game.js:2809), so the pan slows below 20 fps; the first sweep's "4.05 m per frame" is that cap. Headless runs counted 4 to 15 frames in each window of about 1.3 s (about 3 to 12 fps): a 1 s Ctrl+D hold moved 32.3 m, about 8 capped frames (B9). Real-GPU speed is [assumed] | | |
| B2 | Arrow pan | Arrows pan when nothing is selected | Arrows | editor.js:669 | works [S]; not documented anywhere | | |
| B3 | Middle-drag pan | Pans with the middle mouse button | Middle mouse drag | editor.js:1017‑1019 | works [S] | | |
| B4 | Zoom | Height 28 to 140 m, 6 m per notch | Wheel | editor.js:633; game.js:3043 | works [S] | | |
| B5 | Pan clamp | ±(largest arena half-size + 30), clamped to 90..230 | — | editor.js:671 | works [S] (±216 on Cabana) | | |
| B6 | Ortho top-down | Separate orthographic camera; picking uses it too | Ortho button / O | editor.js:635‑642; game.js:2809 | works [S] | | |
| B7 | Ghost footprint | Yellow wire box under the cursor, always at rotation 0 | Automatic | editor.js:61‑64, 676‑677, 1045‑1048 | partial [S]: no ghost for house, cottage, farmhouse, power plants, enemy spawns, prefabs, Building Maker. [C]: also no hover ghost for the drag tools (Path, Hedgerow, Drystone Wall, Fence, Orchard, Windbreak) until you press; their outline appears only while dragging | Real preview that can be rotated before placing | |
| B8 | Fixed camera tilt | Height H, offset z+0.18H | — | editor.js:672 | works [C] | Verticality needs orbit and tilt | |
| B9 | Ctrl shortcuts also pan | The key handler marks every key as held before it checks Ctrl, and panning reads W/A/S/D without checking Ctrl. Ctrl+D pans right while it duplicates, Ctrl+A pans left, and Ctrl+S pans down **and** opens the browser's Save-page dialog (not blocked) | Ctrl+D / Ctrl+A / Ctrl+S | game.js:3006; editor.js:668 | broken [S]: 1 s holds moved Ctrl+D +32.3 m on X, Ctrl+A −11.4 m, Ctrl+S +15.4 m on Z. A 0.1 s tap at 60 fps is about 8 m (estimated, not measured). editor.js:668 has no Ctrl check [C] | Ignore pan keys while Ctrl or Cmd is held | |

## C. Library and placement
| ID | Feature | What it does | How to use | Where | State | v2 note | Mark |
|---|---|---|---|---|---|---|---|
| C1 | Tabs | Recent / Terrain / Buildings / Vegetation / Props / Markers / Prefabs | Click a tab | editor.html:1096‑1104; editor.js:1191‑1193 | works [S] (items per tab: 0/3/16/5/11/8/0) | | |
| C2 | Search | Filters cards across all tabs by card name or group name | Type in "Search library…" | editor.html:1105; editor.js:1181‑1190 | works [S] ("oak" → Oak; "hq" → both HQs) | | |
| C3 | Recent | Last 8 tools used, newest first | Recent tab | editor.js:457, 847 | works [S]; not saved between sessions | | |
| C4 | Armed tool after placing | Placing does not select the new item, and the tool stays armed | — | editor.js:1036 | works as coded [S]. This is the core of **BUG-1**: Delete right after placing does nothing (2→2), and clicking a tree with the tree tool armed plants a stacked copy (+1 per click) | Auto-select what was placed; Esc disarms the tool | |
| C5 | Placement height | Everything lands on the y=0 ground plane | — | editor.js:576 | works [C]; you can't place on top of a platform | Verticality | |
| C6 | Path / Road | Drag an axis-aligned strip; a click makes 6×6 | Terrain tab, drag | editor.html:1114; editor.js:1033, 1045, 1055 | works [S] (12×6 drawn, 6×6 on click) | Not mirrored live (H1) | |
| C7 | Platform | 10×10×2.5 | Click | editor.html:1115; editor.js:581 | works [S] | | |
| C8 | Ramp | 12×8, rising 0→2.5 | Click | editor.html:1116; editor.js:582 | works [S] | | |
| C9 | Friendly HQ / Refinery / Barracks / Power Plant | Team building, team "friend" | Click | editor.html:1122‑1125; editor.js:588‑595 | works [S] | Two teams only; needs three | |
| C10 | Hostile HQ / Refinery / Barracks / Power Plant | Same, team "hostile" | Click | editor.html:1129‑1132 | works [S] | Same | |
| C11 | Enterable Barracks | Walk-in barracks with the armory inside | Click | editor.html:1136; editor.js:610 | works [S]; placed as "neutral", and the Team buttons offer only Friendly/Hostile | | |
| C12 | Half-Timbered House / Thatched Cottage / Brick & Flint Farm | Village houses | Click | editor.html:1140‑1142; editor.js:583‑585 | works [S]; no ghost | | |
| C13 | Norman Manor | Landmark building | Click | editor.html:1146; editor.js:586 | works [S] | | |
| C14 | Bocage Hedgerow / Drystone Wall / Post & Rail Fence | Cover line at any angle; a click makes 4 m | Drag | editor.html:1150‑1152; editor.js:1046, 1056 | works [S] (hedge 16.1 m at −0.52 rad) | Not mirrored live | |
| C15 | Apple Tree / Lombardy Poplar / Oak | One tree with a random seed | Click | editor.html:1158‑1160; editor.js:599‑601 | works [S]; see **BUG-1** (C4, F1) and **BUG-2** (G2) | | |
| C16 | Apple Orchard | Rectangle filled with a grid at about 4.5 m (at most 9×9); preview dots, corner posts, live tree count | Drag | editor.html:1164; editor.js:19‑25, 1047, 1057 | works [S] (12 trees) | | |
| C17 | Poplar Windbreak | Row of poplars every 3 m (at most 15) | Drag | editor.html:1165; editor.js:39‑44 | works [S] (9 trees) | | |
| C18 | Crate S / M / L | 2 / 3 / 4 m crates | Click | editor.html:1171‑1173; editor.js:578‑580 | works [S] | | |
| C19 | 8 farm props | Haystack, Hay Bales, Cart, Well, Log Pile, Barrels, Trough, Veg Patch | Click | editor.html:1177‑1184; editor.js:602‑609 | works [S] (all 8 placed) | | |
| C20 | Player Spawn | Moves the single spawn point | Click | editor.html:1190; editor.js:611 | works [S]; only one; Delete is ignored | Per-team spawns | |
| C21 | Resupply | Moves the single terminal | Click | editor.html:1191; editor.js:612 | works [S] | | |
| C22 | Soviet / Allies / Empire spawn flags | Bot respawn points (game.js:700) | Click | editor.html:1195‑1197; editor.js:596‑598 | works [S] | Map factions to teams | |
| C24 | Placement rotation | Every placed object, and the ghost, starts at rotation 0. There is no rotate-before-place. Because placing selects nothing (C4), Shift+wheel right after placing zooms | Pick a card, click the map, Shift+wheel | editor.js:577‑614, 676; game.js:3043 | works as coded [S]: Farm Cart placed, then Shift+wheel zoomed 90→96 and the cart stayed at 0° | Rotate the ghost before placing | |

Merged: C23 (Generators panel, holds only the Building Maker) is now part of J1. There is no C23 row.

## D. Selection
| ID | Feature | What it does | How to use | Where | State | v2 note | Mark |
|---|---|---|---|---|---|---|---|
| D1 | Click select | Picks the top object, draws a yellow box, fills the inspector | Q + click | editor.js:1028‑1031, 73‑75 | works [S] | | |
| D2 | Deselect | Clears the selection | Click empty ground | editor.js:1054 | works [S] | | |
| D3 | Marquee | Picks objects whose centre falls in the box; replaces the selection; skips spawn and resupply | Drag on empty ground | editor.js:1032, 1044, 178‑182 | works [S] (3 of 4 crates); holding Shift changes nothing | Pick by bounds | |
| D4 | Add to selection | — | Shift+click | — | missing [S] (Shift+click on a second crate still leaves 1 selected) | New in v2 | |
| D5 | Multi-select colour | Boxes turn blue when more than one is selected | — | editor.js:74 | works [C] | | |

## E. Transform
| ID | Feature | What it does | How to use | Where | State | v2 note | Mark |
|---|---|---|---|---|---|---|---|
| E1 | Drag move | On the first move, the grabbed object's **centre jumps to the cursor** (snapped when Grid is on); the rest of the selection follows by the same amount. The grab offset is not kept | Press on an object, drag | editor.js:144, 1031, 1043 | partial [S]: a platform centred at x=30, pressed at x=34 and moved to 35, ended with its centre at 35 (31 if the offset were kept). **NEW-1:** pressing the centre of a selected object grabs the move gizmo's Y arrow (only about 5 px on screen) and lifts it (crate y 0.75 → 23.46 m) | Keep the grab offset | |
| E2 | Nudge | 1 m (or the grid size); Shift = 0.25 m | Arrows with a selection | editor.js:222; game.js:3006 | works [S] | | |
| E3 | Wheel rotate | Rotates every selected object by the step; Alt+Shift = 5°. With nothing selected, Shift+wheel zooms | Shift+Wheel with a selection | editor.js:158, 147‑157; game.js:3043 | partial [S]. **NEW-7 (BUG-2 family):** with a horizontal-scroll event (deltaX, which is what Chromium on Windows sends for Shift+wheel) both directions rotate −45° (+100 → −45, −100 → −45); Brave not checked. Trees, platforms, enemy-spawn flags and the player spawn have no branch in `edRotateOne`, so they never turn, yet each try adds an undo step (platform measured [D0]: undo 41 → 42; enemy spawn and player spawn are [C]) | | |
| E4 | Rotation step | 15 / 45 / 90, default 45 | Snap button | editor.html:1068; editor.js:574 | works [S] | | |
| E5 | Grid snap | Off / 0.5 / 1 / 2 / 5 m for placement, drag and the cursor readout | Grid button / G | editor.html:1067; editor.js:458, 576 | works [S] ((3.3, 5.1) → (4, 6) at 2 m) | Not saved between sessions | |
| E6 | Move gizmo X/Z | Drag the red or blue arrow | E | editor.js:85‑98, 1023, 1040 | works [S] (+5.9 m) | | |
| E7 | Move gizmo Y | Raises or lowers, clamped to 0..60 m | E, green arrow | editor.js:99‑105 | partial [S]. Too sensitive (+28 m from a 90 px drag). No effect on houses, covers, trees, props or enemy spawns, but still adds an undo step. **NEW-6:** height is not saved for platforms, paths, barracks, spawn or resupply, so the next undo or save drops it (platform 4.25 → 1.25) | Proper verticality | |
| E8 | Rotate ring | Press the object or the ring and drag | R | editor.js:115‑123, 1042 | works [S] (−114°); pressing R first clears the selection | | |
| E9 | Scale handles | Resizes the footprint of crate, platform, path, ramp and resupply; uniform scale for team buildings and barracks | T, drag | editor.js:79‑80, 107‑113, 1041 | partial [S]: crate 2→5.7 works. **NEW-8:** a short drag on the HQ jumps from 1× to 4× (the maximum). No effect on trees, props, houses or covers | | |
| E10 | Mode switch (Q/E/R/T keys and the tool rail) | **Q and E are the same mode**: both set gizmo mode "move" and tool "select"; only the lit rail button differs. R shows the rotate ring, T the scale handles. The Select rail button also sets "move" (editor.js:1178), so Select always shows the move gizmo (the root of NEW-1). The rail also holds Place library, Generators and Measure; every tool-rail click (Select, Move, Rotate, Scale, Measure) and every mode key clears the selection (setEdTool, editor.js:457). This is **NEW-5** (merged from A3). The Place library and Generators buttons don't, except that Place clears it when the Maker's Place mode was on (editor.js:830) [C] | Q/E/R/T keys, or the rail (Select / Move / Rotate / Scale, Place, Generators, Measure) | game.js:3006; editor.js:124, 457, 1171‑1180; editor.html:1084‑1094 | works [S]: after both Q and E, gizmoMode is "move" and the move gizmo is visible. NEW-5 stands | A real Select mode without a gizmo, or drop one of Q/E; don't drop the selection on a mode change | |
| E11 | Multi-object rotate | With several objects selected, Shift+wheel, the inspector Rotation° and ±step buttons, and an R-drag turn **each object about its own centre**. The group does not turn around a shared pivot. The ring only appears for a single selection | Select several, then Shift+wheel, inspector, or R + press an object | editor.js:158, 855, 1042, 119 | works as coded [S]: crates at x=−20 and −10 both turned 45° and stayed in place | v2 gizmo needs a group pivot | |

## F. Edit operations
| ID | Feature | What it does | How to use | Where | State | v2 note | Mark |
|---|---|---|---|---|---|---|---|
| F1 | Delete | Removes the selection | Del / Backspace | editor.js:615‑628 | works [S] with a selection (tree 1→0). **BUG-1** cases: nothing is selected after placing (C4); a focused inspector field swallows the key [D0]; stacked copies from older saves need several presses [D0] (PR #10 dedupes these on load). **BUG-3:** removed objects are never disposed, only taken out of the scene [C] | Dispose on delete | |
| F2 | Duplicate | Copies the selection +4,+4 m and selects the copies | Ctrl+D | editor.js:185‑216 | partial [S]. **NEW-2:** a platform copy swaps height and depth (10×2.5×10 → 10×10×2.5; arguments at 207 don't match `makePlatform(x,z,w,d,h)`). **NEW-3:** a Building Maker copy becomes a plain timber house (the spec is dropped at 195). Ctrl+D also pans the camera (B9) | | |
| F3 | Copy / Paste | Clipboard keeps positions relative to the centroid; pastes at the cursor | Ctrl+C / Ctrl+V | editor.js:218‑220 | works [S]; same bugs as F2 | | |
| F4 | Array | The count is the **total including the original** (2 to 50, default 3), with a gap (default 6 m) along ±X or ±Z. Selects the originals plus the copies | Inspector "Array ×": count, direction, "Gap m", Make | editor.js:168‑173, 928‑936 | works [S]: the default 3 added 2 copies and left 3 selected | | |
| F5 | Set team | Friendly / Hostile; rebuilds the object (it loses its identity) | Inspector | editor.js:629‑632, 915‑917 | works [S] | Three teams | |
| F6 | Building HP | Number field, −25 / +25 | Inspector | editor.js:857, 923‑927 | works [S] (1600 → 1625) | | |
| F7 | Scale × | Uniform scale 0.3 to 4 for buildings and barracks | Inspector | editor.js:160, 918‑922 | works [S] | | |
| F8 | Size W/H/D | Crates, paths, platforms, resupply | Inspector | editor.js:858‑863, 884‑899, 106 | works [S] (W = 5) | | |
| F9 | Platform height presets | Step 1.2 / Low 2.5 / Mid 4 / High 6 | Inspector | editor.js:900‑904 | works [S] | | |
| F10 | Ramp shape | Run, Width, Low h, High h; High-end presets 2.5 / 4 / 6 | Inspector | editor.js:864‑867, 905‑914 | works [S] | | |

## G. Inspector
| ID | Feature | What it does | How to use | Where | State | v2 note | Mark |
|---|---|---|---|---|---|---|---|
| G1 | X / Z fields | Numeric position | Type + Enter | editor.js:871‑874 | partial [S]. **NEW-4 (BUG-2 family):** a value typed without Enter is applied to the next object you click (typed 33 on A, clicked B → B moved to 33; matches [D0]) | | |
| G2 | Rotation° field + −step / ⊕snap / +step | Numeric rotation | Inspector | editor.js:875‑883, 855‑856, 851 | works [S] for crates, ramps, buildings, houses, props, covers, resupply, and [C] for **paths and barracks**. The field is hidden for platforms, the player spawn and enemy-spawn flags (`_edRotCur`, 851). **BUG-2:** shown for trees but does nothing (`edRotateOne` has no tree branch, 147‑157) and still adds an undo step. **BUG-2 [D0]:** a value typed in Rotation° without Enter goes to the next object you click (same pattern as G1). On a pre-PR #8 save, only one stacked copy turns (O1) | | |
| G3 | Number scrub | Drag horizontally on a field | Drag | editor.js:1166‑1170 | partial [S]: the object only updates on mouse release, not during the drag (part of **BUG-2**) | | |
| G4 | Empty state | Shows the World and Match Rules panels when nothing is selected | — | editor.js:942, 1211 | works [S]; hidden while anything is selected | | |
| G5 | Kind label | "Crate", "Friendly hq", "N selected"… | — | editor.js:850, 869 | partial [S]: every house (village, manor, Building Maker) shows the raw id "house", and enemy-spawn flags show "enemyspawn". `edKindLabel` has no case for them | | |
| G6 | Multi-selection inspector | With 2+ selected: header "N selected". X/Z and Rotation° show the first object and move or turn all of them by the same amount. Array and Save-as-prefab use all of them. Size, team, scale and HP rows are hidden | Select several | editor.js:868‑937 | works [C]. [C/D0]: if the first selected object is a tree, each typed Rotation° value turns the others by the full value again (editor.js:855) | | |

## H. Mirror / symmetry
| ID | Feature | What it does | How to use | Where | State | v2 note | Mark |
|---|---|---|---|---|---|---|---|
| H1 | Live mirror X/Z | Click placements and prefab stamps also place a mirrored copy; swaps friend↔hostile; rotation becomes −rotation | Mirror button (Off/X/Z) | editor.html:1069; editor.js:162‑167, 1035‑1036 | partial [S]: works for clicks (crate, HQ, tree). Not applied to drag tools, paste or duplicate. **NEW-10:** a Z-mirror doesn't flip asymmetric pieces (both barracks doors face +Z). **NEW-3 also hits mirroring:** a Building Maker building's mirrored copy is a plain timber house (no spec; type "timberHouse" at x=−40) | Replace with live 120° three-way symmetry | |
| H2 | Mirror selection | Adds a mirrored copy of the selection | M (only when mirror is on) | game.js:3006; editor.js:167 | works [S] | | |

## I. Prefabs
| ID | Feature | What it does | How to use | Where | State | v2 note | Mark |
|---|---|---|---|---|---|---|---|
| I1 | Save as prefab | Saves the selection relative to its centroid as "Prefab N" (KVStore `vprefab:`) | Inspector button | editor.js:937, 971‑976 | works [S] | | |
| I2 | Stamp prefab | Stamps (mirrored if mirror is on) and selects the result | Prefabs tab, click a row, click the map | editor.js:977‑987, 699‑702 | works [S]; no ghost, no rotation, no rename; F2 bugs apply | Prefab library v2 is [LATER] | |
| I3 | Delete prefab | Deletes after a two-click confirm | ✕ twice | editor.js:985 | works [S] | | |

## J. Building Maker
| ID | Feature | What it does | How to use | Where | State | v2 note | Mark |
|---|---|---|---|---|---|---|---|
| J1 | Open / close, and the Generators panel | The top-bar Maker button and the Generators rail button only **open** the Maker (the Generators panel holds only the Maker; merged from C23). Only the panel header toggles it. The Place-library rail button closes it and turns Place mode off. Clicking the active Generators rail button hides the column, but the Maker stays open: its preview stays in the scene and Place mode stays on | Maker button, Generators rail button (star), panel header | editor.js:831, 841‑846, 1096‑1098, 1172‑1176; editor.html:1210‑1213 | works [S]; the hidden-column case is [C] | | |
| J2 | Region preset | Picks the region style | Select | editor.js:813‑815 | partial [C]: changing it has no visible effect until you press Dice | | |
| J3 | Parts pickers | Controls generated from BK.PARTS | Select | editor.js:816‑821 | works [C] | | |
| J4 | Dimension sliders | Controls generated from BK.DIMS | Slider | editor.js:816‑821 | works [C] | | |
| J5 | Seed + Dice | Rolls a new variation | Seed field / ⚅ Dice | editor.js:808, 822‑825 | works [S] (1 → 4079755702) | | |
| J6 | Live preview | Blue-outlined preview at the camera's look point | Automatic | editor.js:797‑804 | works [S] | | |
| J7 | Place mode | Stamps the current spec at rotation 0 | "+ Place mode", then click the map | editor.js:795, 826‑827 | works [S] (kit building with its spec saved) | Copies lose the spec (NEW-3) | |

## K. Analysis
| ID | Feature | What it does | How to use | Where | State | v2 note | Mark |
|---|---|---|---|---|---|---|---|
| K1 | Spawn distances | Friendly spawn disc and hostile spawn band (red zone "random across band"). Lines and F/H distances to each silo, tech point and the harvester. **Flags any objective that favours one side by more than 15 %** (warning colour, "+N m"). Rebuilds itself when the spawn or the objectives move | Markers → Spawn Distances | editor.js:495‑518 (15 %: 511; auto-refresh: 515, 680) | works [S]; the flag and auto-refresh are [C]. The hostile spawn is a fixed point (0, 21−arenaZ, line 502) and ignores placed spawn flags | Three teams; objectives as data | |
| K2 | Symmetry diff | Off → N‑S → E‑W; marks matched and unmatched pieces | Symmetry toggle | editor.js:521‑546 | works [S] (Cabana N‑S: 9 matched / 27 unmatched); only checks crates, buildings, platforms, ramps, barracks, objectives | 120° check | |
| K3 | Cover / LOS heatmap | Exposure map, computed on toggle | Cover Map toggle | editor.js:549‑572 | works [S] (172 cells, about 4.6 s); cells are at least 12 m, so coarse | | |
| K4 | Measure | Click A, click B: distance, Δ, Manhattan path | Rail ruler | editor.js:476‑492, 679 | works [S] ("50.0 squares…"); the unit reads "squares", not metres | | |

## L. World panel and M. Match Rules
| ID | Feature | What it does | How to use | Where | State | v2 note | Mark |
|---|---|---|---|---|---|---|---|
| L1 | World controls | 22 controls: Ground 9, Lighting 9, Sky & Fog 4; saved per map | Sliders / colour pickers | editor.js:703‑756; editor.html:1230 | works [S] (density 1.25 → 1.8, one undo step, Reset restores) | | |
| L2 | World / Rules buttons | Deselect, then scroll to and flash the panel (not a toggle). When the inspector column is hidden (A5) they deselect and show nothing | Top bar | editor.js:757, 789, 940 | works [S]; the hidden-column case is [C] | | |
| M1 | Match Rules | 9 sliders: silo capture, tech-box ×4, resupply cost, harvester ×3; saved in `rules` | Sliders | editor.js:758‑788 | works [S] (applies live: SILO.captureTime = 14); Reset works | | |

Merged: L3 (env undo) is now a note in N1.

## N. Undo / redo
| ID | Feature | What it does | How to use | Where | State | v2 note | Mark |
|---|---|---|---|---|---|---|---|
| N1 | Undo / redo | Undoes and redoes map edits. It is the same snapshot undo for environment (World panel) changes, which the snapshot already includes (merged from L3) | Ctrl+Z, Ctrl+Y, Ctrl+Shift+Z, buttons | editor.js:833‑838, 1102 | works [S] (env undo 0.42 → 0.8) | | |
| N2 | Snapshot model | Each step is the whole map as JSON (about 12 KB on Cabana), at most 60 steps; same-tag edits within 1.2 s merge | — | editor.js:835 | works [S] (60-step cap holds) | Replace with commands | |
| N3 | Rebuild on undo | Rebuilds the whole map, ground textures included | — | editor.js:836; game.js:612 | partial. **BUG-3:** about 300 ms per undo on Cabana [S]; 280 to 850 ms per undo or redo in the step-0 runs [D0]; each placement adds draw calls (house +12, HQ +7, tree +4, crate +2) [D0]. Selection is lost [S]; unsaved state is lost (E7) | | |

## O. Maps and files
| ID | Feature | What it does | How to use | Where | State | v2 note | Mark |
|---|---|---|---|---|---|---|---|
| O1 | Maps modal | Save by name: the field is prefilled with the title name, max 40 characters, click only (Enter does nothing); overwrites silently. List A–Z, Load (clears history), Delete (two-click confirm). The note reads "(saved to browser storage)" or "(this session only — use Export…)" when storage falls back to memory. **A failed browser-storage write (quota) silently falls back to memory and still says "Saved!"**, so the map is lost on reload | Folder button | editor.js:684‑691, 959‑970, 1074‑1076; editor.html:1249; core.js:48‑52 | works [S] (localStorage under file:// in Chromium; Brave not checked); prefill, Enter, note and quota are [C]. **BUG-1/2/3 on main [D0]:** loading a map saved before PR #8 brings back every stacked copy of each tree and prop it was saved with (tested with 4× and 16× saves: 280 and 1120 trees). This is the only way the lag reproduced on main. On the 4× save, Delete leaves 3 of 4 copies and rotating turns only 1 of 4. PR #10 dedupes on Load | Save errors must be shown | |
| O2 | Export JSON + Copy | Shows the map JSON and copies it | Export button | editor.js:1065, 1068‑1069 | works [S] (1436 chars reached the clipboard); the name is always "DM_Romania" | | |
| O3 | Import JSON | — | — | — | missing [C] | Needed for migration | |
| O4 | Reset | Rebuilds Cabana and adds an undo step | Reset button | editor.js:1066 | works [S]; the tooltip is wrong (A7) | | |
| O5 | What the v1 format saves | Spawn, resupply, crates, paths, platforms, ramps, houses (legacy type or kit spec), covers, trees (no rotation), props, enemy spawns, buildings, barracks, env, rules. Platforms save their box height `h` but **not their elevation `y`** or a rotation. Paths and barracks save no `y` either. Crates, ramps and buildings save `y` | — | game.js:635‑651 | [C]. **Not saved:** version, ids, name, arena size, objectives (silos, tech points and harvester field are fixed to Cabana: game.js:673, 675, 843), non-crate walls | Migration input | |

## P. Test
| ID | Feature | What it does | How to use | Where | State | v2 note | Mark |
|---|---|---|---|---|---|---|---|
| P1 | Walk preview | Puts the player **at the camera's centre, not at the spawn**. W walks toward the mouse, S away, A/D strafe; Shift runs, Space jumps; collisions; kept inside the arena. On exit the player snaps back to the spawn. Wheel and clicks do nothing | Tab; Tab or Esc to exit | editor.js:645‑664 (start 647, exit 650); game.js:3043 | works [S]: camera at (50, −20) put the player at (50, −20); spawn is (−98.9, 89.9); jump reached y 1.29; no bots, no shooting | | |
| P2 | Play | Full round with 6 bots on the edited map | Play button | editor.js:451‑456; game.js:1600‑1602 | partial [S]. **NEW-9:** damage carries back into the editor (a destroyed HQ stays dead and sunk). **NEW-11:** the round reset puts each building back at its *unscaled* half-height: a 2× HQ sinks 4.5 m (bottom −4.5) and a refinery raised to y=7 drops to 2. **No way back to the editor** except Pause → Menu → Map editor, which resets the camera and clears undo [C] | v2 Test must snapshot, restore and return | |
| P3 | Back to menu | Leaves with no prompt; the map stays in memory, but undo history is cleared | Esc / Menu button | game.js:3006, 2739 | works [S] | | |
| P4 | Esc with a modal open | Esc leaves the editor even while the Export or Maps modal is open (unless the cursor is in a text field). The modal is still open when you come back | Esc | game.js:3006; editor.html:1247‑1252 | broken [S]: mode "menu" after Esc; the Export modal is shown again on re-entry | Esc should close the modal first | |

## Hidden, legacy and dead code
1. **`wall` kind branches** (editor.js:69, 100, 109, 130, 148, 186, 527, 616, 849, 850) can never run: barracks walls are re-tagged as the barracks (game.js:536), and `serializeMap` drops non-crate obstacles (game.js:639). editor.js:527 labels non-crate obstacles 'wall' in the symmetry diff. It is dead too: every non-crate obstacle is a barracks wall, and 526-527 skip those [C].
2. **Close (✕) handlers** for World, Rules and Maker (editor.js:1082, 1087, 1097) are dead because CSS hides the buttons (editor.html:520, 528).
3. **The palette "measure" toggle branch** (editor.js:1070) is dead: there is no measure card.
4. **Duplicate `IC.path` key** (editor.js:1128 and 1132).
5. **Static placeholders:** `edStatZoom` ("100%") and `edWMapHint` / `edRMapHint` ("this map") are never updated.
6. **index.html carries a full copy of the editor DOM** (0 lines differ) but never loads the editor script.
7. **Dev panel (Backquote key) MAP slider** calls `setMapSize` (game.js:76, 3086): a square override from 24 to 140 that isn't saved and can't represent Cabana's 186×126.
8. **Undocumented behaviour** (the missing shortcuts are listed in A9 only): the cursor readout shows the tree count during orchard and row drags; the enemy-spawn layer is visible only in the editor (game.js:2809). Space jumps and Esc exits the walk preview (P1).
9. **Also missing:** no confirmation when Esc leaves the editor. Recent tools, grid, rotation step and mirror settings aren't saved between sessions. There is no Ctrl+S handler, but pressing Ctrl+S still pans the camera and opens the browser's Save-page dialog (B9).
10. **Dead `d.spec` branch:** `edSpawnDesc` can rebuild a Building Maker building from its spec, but nothing ever passes one, because `edDescribe` drops it (editor.js:209, 195). Fixing NEW-3 means filling this in.

## Not verified
- Nothing was run in Brave on Windows.
- No frame times on a real GPU (SwiftShader frame times don't mean anything for BUG-3).
- KVStore and the clipboard under Brave's file:// not checked, and whether Brave's file:// storage ever falls back to memory (O1) was not checked.
- Code reading only: A1's kept settings, A9, A10, the A11 camera sliders, the A13 Maker pickers, B1's formula, the B7 drag-tool ghosts, B8, the B9 Ctrl+S Save-page dialog, C5, D5, the E3 enemy-spawn and player-spawn rotation cases, E10's Place-button exception, F1's missing dispose, G2 for paths and barracks, G6, J1 and L2 with the column hidden, J2, J3, J4, the K1 flag and auto-refresh, the O1 prefill, Enter, note and quota behaviour, O3, O5, P2's way back to the editor, and hidden item 1.
- The B9 "about 8 m per 0.1 s tap" figure is estimated, not measured.
- I didn't visually confirm where the overlay labels sit on screen.
- First-run FAILs in the first sweep were partly test-order effects (for example the R key clearing the selection before the test started); each was re-checked in a later run. One page closed during a screenshot and its last 15 checks were re-run.
