# Advisor report: editor-hotfix (2026-10-16, branch cs/editor-hotfix, base main 67cf55b)

## Goal
Make today's map editor usable for Mustafa's three bugs, before editor v2:
- trees can't be deleted;
- rotation does nothing;
- lag after a few placements.

Small, safe fixes only, as in the approved plan (H1–H7, plus E1–E5, all approved as recommended).

## Outcome
- **All items are built:**
  - H1–H7 and E1–E5;
  - three review fixes;
  - one small leak fix: selection-box materials are now disposed with their geometry.
- **Evidence:** 42 base-vs-head checks on the real editor.html with real mouse and keyboard. All 42 PASS, with
  0 regressions and 0 page errors in 24 runs.
- **Gate:** PASS, all 8 checks; symbols +2 −0 (`_edCanRot`, `_edToolKind`).
- **Reviews:** art-director, verifier and security-auditor all gave PASS WITH NOTES. The notes are applied or listed
  below.
- **Commits:** 6b08837 (the hotfix), b7f2b32 (review fixes).
- **Waits on:** the Advisor's APPROVE and Mustafa's Brave check. Not merged.
- **Lag:** there is no code for it here. PR #8 removed the cause, and PR #10 cleans old saves on Load (step zero of the
  plan).

## What changed (shamble-editor.js; shamble-game.js editor branch only; both pages identically)
| # | Fix |
|---|---|
| H1 | `_edCanRot`: trees, platforms and spawns get no rotation controls, ring or empty undo steps. The Rotation row reads the first selected item that can turn. |
| H2 | The inspector captures its selection, so a value typed before clicking another object goes to the right object (Rotation, X, Z). `edRotateSelTo` and `edMoveSelBy` gain an optional `sels` argument. |
| H3 | Dragging Rotation, X or Z updates live (only while the scrubber has blurred the field); typing still applies on Enter, Tab or blur. |
| H4 | Enter applies and leaves the field, so Delete works after it; Escape restores the value the field had on focus. |
| H5 | `selectstart` no longer blocks INPUT, TEXTAREA or contentEditable targets (Ctrl+A works). dragstart and dblclick stay blocked. |
| H6 | Shift+wheel uses deltaY, or deltaX when deltaY is 0, so it turns both ways; a 0 event does nothing and no longer zooms in. |
| H7 | Delete on the spawn or resupply alone does nothing (no empty undo step). |
| E1 | What a click or drag places is selected: the mirror twin, orchard and poplar rows, paths and hedges too. |
| E1a | No gizmo, ring or handles while a placement tool is armed (they don't respond then). |
| E1b | With a tool armed, clicking an object of the same kind (within its footprint + 0.5 m) selects it instead of planting a copy. New `_edToolKind` maps a tool to the kind it places. |
| E1c | Arrows keep panning while placing; they nudge only in Select. |
| E2 | Escape steps back: stop placing (selection kept), then clear the selection, then nothing. It never leaves the editor; the Menu button does. Switching to Select (Q, E, R, T, Escape) keeps the selection. Key repeat is ignored. |
| E4 | A Delete button in the inspector header ('Delete', 'Delete 2'), hidden for spawn and resupply, using the existing destructive style. |
| E5 | While placing, the status bar reads 'Placing Oak — click to place · Del deletes it · Esc / Q: stop placing' ('drag to place' for drag tools). The shortcut card gains 'Esc stop placing' and 'Q select'. The Menu tooltip no longer says Esc. |

## Evidence (base main → head; real editor.html, trusted Playwright input, headless SwiftShader)
| Check | main | this branch |
|---|---|---|
| 10 rotatable kinds: +45°, Shift+wheel, ring drag | (reference) | same yaw to 0.01°, same saved rot, same undo |
| Tree: +45°, Shift+wheel, R-ring | Rotation row shown, yaw stays 0, 3 empty undo steps | no Rotation row, no ring, 0 undo steps |
| [tree, prop] selection: type 90 | row reads 0 (the tree), cart goes to 135° | row reads 45 (the cart), cart goes to 90° |
| Type 90 in crate A's Rotation, then click crate B | A 0°, B 90° | A 90°, B 0° |
| Same with X (−14) / Z (6) | B moves | A moves, B stays |
| Hold a drag on Rotation / X | object moves only on release | object follows every sample while held |
| Type + Enter, then Delete | the field eats the key | object deleted |
| Click Rotation, Ctrl+A, type 90, Enter | field and yaw unchanged | both 90 |
| Shift+wheel deltaX +100 / −100 | −45 / −45 | +45 / −45 |
| Shift+wheel (0, 0) | −45° and an undo step | nothing |
| Shift+wheel deltaX with nothing selected | zooms in both ways | no zoom |
| Delete with only the spawn selected | empty undo step | nothing |
| Place a tree, press Delete | nothing selected, tree stays | tree gone |
| Place a tree, click it with the tool armed, Delete | copy stacked, both stay | no copy, spot empty |
| Mirror X: place a crate, Delete | both stay | both gone, 1 undo step |
| Orchard (25 trees) / poplar row (9), Delete | all stay | all removed |
| Escape after placing, ×3 | leaves the editor (undo history wiped) | Select → clear → nothing; never leaves |
| Hold Escape (key repeat) | leaves the editor | stays; repeats ignored |
| Ctrl+Z after the Escapes | nothing to undo | undoes both placements |
| Place a crate, press R, one Shift+wheel notch | zooms, crate stays 0° | ring shows, crate turns 45° |
| Inspector header with the Delete button | (no button) | 37 px in every state (no layout jump) |
| Page errors over 24 runs | 0 | 0 |

Harnesses and outputs are kept in the session scratchpad (`hfxev/`).

## Reviews
- **art-director: PASS WITH NOTES (approve).** No locked look value was touched, no CSS was added, and both pages
  changed identically.
  - F1: the Delete button didn't look destructive. Fixed: it uses the existing `.edBtnDel` style.
  - F2: the fields jumped 12 px when the button appeared. Fixed with negative margins; the header measures 37 px with
    the button, with the spawn and with nothing selected.
  - F3: hint wording. Fixed: drag tools say 'drag to place', and an em dash follows the label.
  - F4 (deferred): the Tool field shows the raw tool id ('Place · treeOak'), as on main. It also suggests emphasising
    the placing hint.
  - F5 (deferred, plan §5): the placement ghost and the new selection box are the same yellow.
- **verifier: PASS WITH NOTES.** It re-ran the gate and four decisive checks itself, all PASS.
  - `_edToolKind` was not in the plan's symbol list. E1b needs it to know which kind a tool places. It is reported here
    as a deviation.
  - Under heavy CPU load, one drag can split into 2 undo steps when a frame stalls for more than 1.2 s. That is the
    existing undo-grouping window. It is worth a look in Brave.
- **security-auditor: PASS WITH NOTES.**
  - Dynamic DOM uses `createElement` and `textContent` only.
  - `edTool` comes from a closed set of static ids.
  - The `selectstart` change frees only text fields.
  - The Escape branch is strictly narrower than before.
  - Check 4: 51/51 references guarded.
  - L1: build no CSS selector from `edTool`. Fixed.
  - L2: Escape no longer leaves the editor. Accepted (that is E2).

## Decisions and deviations
- **Symbols +2, not +1:** `_edToolKind` was added for E1b.
- **Escape inside the editor never goes to the menu** (E2 with "do nothing" at the end, as approved). The Menu button
  is the way out.
- **STATUS / ROADMAP are not edited here,** to avoid a conflict with PR #11 (which edits both). They follow in the next
  docs pass.
- **Not built:**
  - real tree rotation (map format, E3);
  - prefab dedupe;
  - geometry and material disposal on delete;
  - undo speed;
  - the Tool-field label.

## Not verified
- **Brave on Windows or a real GPU:** all runs are headless SwiftShader.
- **Windows Shift+wheel as deltaX:** only simulated with real wheel events that carry deltaX.
- **OS-timed key auto-repeat:** Playwright sends repeat:true events, not real auto-repeat.
- **E1b on other kinds:** tested on trees only, not props, buildings or platforms.
- **Escape with panels open:** not tested with the prefab stamp, the Building Maker or the Maps panel open.
- **index.html:** markup only, which is identical; no editor interaction there.

## For Mustafa in Brave (editor.html)
1. **Trees:** a tree has no rotation controls. Dragging Rotation° on a crate turns it while you drag, and one Ctrl+Z
   undoes the whole drag.
2. **Typed values:** type 90 into Rotation° on crate A, then click crate B: A turns and B doesn't. Do the same with X.
3. **Shift+wheel:** up and down turn the crate both ways. With nothing selected, Shift+wheel doesn't zoom. If you have
   a horizontal wheel or trackpad, try that too.
4. **Fields:** type a value + Enter, then press Delete: the object is deleted. Click Rotation°, press Ctrl+A, type 45,
   press Enter: it works.
5. **Delete after placing:**
   - Place a tree and press Delete.
   - Place a tree, click it, press Delete: the spot is empty.
   - Mirror on, place a crate, press Delete: the twin is gone too.
   - Paint an orchard and press Delete.
6. **Placing:**
   - While placing, the arrow keys pan the view.
   - Place a crate and press R: the ring shows.
   - While placing there is no gizmo.
7. **Escape:** press it three times after placing. You stay in the editor, and Ctrl+Z still works. The Menu button
   still leaves.
8. **Inspector:** the Delete button turns red on hover. It is hidden for the spawn. The fields don't jump.
9. **Status bar:** it says how to stop placing; it says 'drag to place' for an orchard. The shortcut card shows Esc and
   Q.
10. **Game:** play a match. The HUD, menus and look are unchanged.

## Proposed next step
1. Merge after APPROVE and the Brave check.
2. Then merge PR #10 and re-save old maps (step zero).
