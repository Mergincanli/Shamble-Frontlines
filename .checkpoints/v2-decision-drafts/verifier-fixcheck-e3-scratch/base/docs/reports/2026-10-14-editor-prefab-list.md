# Advisor report: editor-prefab-list (2026-10-14, branch cs/editor-prefab-list, base 7c80e56)

## Goal
The map editor stops throwing errors: the prefab list fills on entry, the Maps window lists saved maps, and
"Save as prefab" works.

## Outcome
DONE: all three errors gone in a headless run of editor.html. Waiting on the Advisor's APPROVE and Mustafa's Brave check.
Stacked on PR #3 (kit-v1-1), so merge that first.

## What changed
- `shamble-editor.js` only:
  - `refreshMapList`, `edSavePrefab` and `refreshPrefabList` moved, unchanged, from inside `bootEditor()` to the top
    level directly above it, with a two-line comment saying why they must live there.
  - Symbols: +3 in editor (`edSavePrefab`, `refreshMapList`, `refreshPrefabList`), −0. Matches the plan.
- The Advisor said "fix only this one unless the plan says otherwise". The plan said otherwise (all three, same
  cause), and Mustafa approved it.

## Evidence
- Pure move: the three function texts are byte-identical before and after, and the rest of the file is identical
  apart from the cut and the inserted block (acorn positions). No functions remain inside `bootEditor()`.
- Scope safety: `bootEditor()` had exactly three locals, these three functions, and none of them used anything
  declared inside it. Every name resolves to the same thing before and after.
- Gate vs main 2d30e4a: PASS. 5/5 parse; both concatenations; core 134 / game 691 / editor 349 (+3 −0); 0 duplicates;
  44 editor refs, all guarded, 0 flagged.
- Runtime, the same pageshot script before and after (enter editor → Maps → select a tree, "Save as prefab" →
  leave and re-enter), headless Chromium on SwiftShader, fresh browser storage:

  | Step | Before | After |
  |---|---|---|
  | Enter the editor: prefab list | empty; "refreshPrefabList is not defined" | "No prefabs saved yet." |
  | Open Maps: map list | empty; "refreshMapList is not defined" | "No saved maps yet." |
  | Click "Save as prefab" on a tree | list empty; "edSavePrefab is not defined" | "Prefab 1"; button reads "Saved · Prefab 1" |
  | Re-enter the editor | empty; "refreshPrefabList is not defined" | "Prefab 1" |
  | Page errors | 4 ReferenceErrors | none |

- index.html: boots and deploys with no page errors (the file is unchanged).

## Reviews
- verifier: **PASS**, no problems.
  - It ran the gate against 7c80e56 itself; the diff matches the plan.
  - It confirmed the move is pure with its own acorn script.
  - Scope: every caller (238, 691 and 937 outside `bootEditor()`; 959–985 and 1076 inside) resolves to the single
    top-level declaration.
  - It reproduced both runtime runs independently: base 4 ReferenceErrors, head none.
  - It did not click the rows' Load / Delete / Confirm handlers (textually unchanged), and did not test persistence
    across real browser sessions.
- No other reviewer: nothing visual, hot-path, security-relevant or netcode-related changed.

## Decisions and deviations
- Kept the plan's placement: directly above `bootEditor()`, not next to MapStore/PrefabStore.
- Found on the way: the gate's default base uses the local `main` even when it is behind `origin/main`. This checkout's
  stale `main` gave base 8e58937. That was harmless here (the editor file didn't change between them), but it is a
  footgun. Recommendation: fold it into gate-check-5 (prefer the newer of `main` / `origin/main`).

## Risks and open questions
- None known for the fix itself. Untested: Brave, and maps saved by earlier sessions (the list code is unchanged; only
  where it lives moved).

## For Mustafa in Brave (editor.html)
1. Open the Map editor with the console open (F12): no red errors.
2. Open Maps (folder icon): your saved maps, or "No saved maps yet.".
3. Select any object, then click "⊕ Save as prefab" in the inspector: the button says "Saved · Prefab N".
4. Leave to the menu and re-enter: the prefab shows in the Prefabs list.

## Proposed next step
gate-check-5 (load-time errors, undefined names, layering, script order). It must catch a re-introduced copy of this bug.
