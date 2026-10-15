# Advisor report: editor v2 steps 0 and 1 (2026-10-15, branch cs/docs-editor-v2, base main 67cf55b)

## Goal
- **EDITOR_V2 §6 step 0:** diagnose Mustafa's three editor bugs, read-only, and say whether PR #8 or PR #10 caused them.
  If the fix is small and safe, plan a hotfix.
- **Step 1:** inventory today's map editor so Mustafa can mark each feature keep, drop or improve.
- **Also:** add `docs/EDITOR_V2.md` (the Advisor's draft) to the repo.

This change-set is docs only.

## What changed (docs only)
- `docs/EDITOR_V2.md`: the Advisor's draft, byte-identical to the attachment.
- `docs/EDITOR_V1_INVENTORY.md`: 107 features in 15 groups.
  - Each row gives what the feature does, how to use it, where it is (file:line), its state and a v2 note, followed by
    an empty **Mark** column.
  - Each state is tagged [S] (real input in headless Chromium), [C] (code reading) or [D0] (from the bug diagnosis).
  - A list of ten hidden, legacy and dead-code items follows the tables.
- This report.

## Step 0: the three bugs
**How the diagnosis was done:**
- One agent per bug drove the real editor.html with real mouse and keyboard on three builds:
  - OLD (159bec4, before PR #8);
  - MAIN (b5f4ecf, the same engine files as 67cf55b);
  - PR #10.
- A separate skeptic per bug then re-ran the decisive steps with its own scripts. All three agreed.

**Finding: none of the three bugs comes from PR #8 or PR #10.** All of them were in the code before PR #8.

| Bug | Main causes | Who fixes it |
|---|---|---|
| **Lag** | Before PR #8, every undo, redo, Reset and Load added a full copy of every tree and prop: 4 undo+redo took 70 trees to 3,850, and JS per frame went from 6 ms to 82–3,246 ms. On main it appears only when a map saved before PR #8 is loaded (constant 27–32 ms per frame with a 16× save). Placing objects never slows the editor (6–10 ms per frame from 0 to 20 objects). | PR #8 (merged) stops new copies; PR #10 removes old ones on Load |
| **Trees can't be deleted** | (a) stacked identical copies, so Delete removes only one; (b) focus left in an inspector field after typing + Enter, so Delete edits the text; (c) placing doesn't select, and the tool stays armed, so clicking the new tree plants a second one on the same spot | (a) PR #10; (b) and (c) the planned hotfix |
| **Rotation does nothing** | Trees show rotation controls but can't rotate; dragging Rotation° turns the object only on release; a typed value goes to the next object you click (X and Z too); Ctrl+A is blocked in fields; Shift+wheel turns only −45° when the browser sends it as deltaX (believed to be Chromium on Windows; not tested on Windows); stacked copies (only one copy turns) | the planned hotfix; PR #10 for the stacked copies |

- **Most likely reason Mustafa's lag grows as he works:** it matches OLD. First check: his checkout is at main 67cf55b or
  newer.
- **The hotfix plan is written and revised after two adversarial reviews** (code: SOUND WITH CHANGES; UX: SOUND WITH
  CHANGES). It has two parts:
  - **Core:** H1–H7, about 25 lines. No fake tree rotation; the inspector edits the object it was opened for; live drag;
    Enter and Escape in fields; Ctrl+A; Shift+wheel both ways; no fake delete.
  - **Placement package:** E1–E5, about 25 lines, for Mustafa to decide. Select what you placed, with three guards;
    Escape steps back instead of leaving the editor; tree rotation later; a Delete button in the inspector; a hint on
    how to stop placing.
- **Status:** the plan waits for approval. No code was written.

## Step 1: the inventory
**How it was built:**
- Two independent sweeps: the first inventory, then a completeness critic. The critic found 18 wrong rows, 9 missing
  features, 4 hidden-list fixes and 5 duplicates.
- docs-keeper merged them. Three skeptic rounds then checked the merge and more than 60 rows against the code (13, then
  6, then 2 findings, all applied).
- **Structure:** 15 tables, every row with 8 cells, and pandoc GFM renders them all.

**Notable new findings (NEW-1 to NEW-11 in the doc):**
- Eight tooltips are wrong; six of them promise "⇧-wheel to rotate", which can't work because nothing is selected after
  placing.
- Switching tool or mode (Q/E/R/T, or the Select, Move, Rotate, Scale and Measure buttons) clears the selection (NEW-5).
- The Zoom readout is always 100 %.
- Ctrl+S opens the browser's Save-page dialog (code reading).
- A list of ten hidden, legacy and dead-code items, among them unused `wall` branches and a dead Building Maker respawn path.

## Not verified
- Nothing was run in Brave on Windows. All frame times are headless SwiftShader, so JS time per frame is the metric.
- Which build and which saved maps Mustafa used when he saw the bugs.
- The rows tagged [C] (listed at the end of the inventory).

## For Mustafa
1. **Inventory:** mark each row in `docs/EDITOR_V1_INVENTORY.md` keep, drop or improve. This is v2's parity list.
2. **Hotfix:** approve the core fixes H1–H7, and decide E1–E5. The plan file is in the session.
3. **Checkout:** check that your checkout is current (main 67cf55b or newer). After PR #10 merges, load and re-save each
   old map once.

## Proposed next step
1. The editor hotfix, if approved.
2. Editor v2 step 2, the architecture plan, after the playable greybox loop, as scheduled.
