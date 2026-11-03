# Advisor report: map-dedupe-on-load (2026-10-14, branch cs/map-dedupe-on-load, base cs/editor-undo-ground 499653a)

## Goal
When Mustafa loads a map saved on an older build, the stacked exact copies of trees and props are dropped. The old
rebuild bug left those copies on every Load, Reset, Ctrl+Z and Ctrl+Y. The editor says how many it removed, and the
stored map changes only if he saves.

## Outcome
DONE as in the revised, approved plan, plus two small review fixes. Both reviews: PASS WITH NOTES. Waiting on the
Advisor's APPROVE and Mustafa's Brave check; merges only after PR #8.

## What changed (symbols +2 −0, matches the plan)
- `shamble-game.js`: `dedupeMapDef(def)` above `serializeMap`.
  - Pure; never mutates its input. The copy is made with `{...def}`.
  - Filters only `trees` / `props` arrays. The key is the whole entry as JSON; props are read with `rot||0`, as buildMap
    reads them.
  - Keeps the first copy, in order. Seedless and non-object entries are kept, and so is an entry whose key can't be built
    (fail open). Other kinds are untouched.
- `shamble-editor.js`, the Maps → Load handler:
  - It builds from the deduped copy, then calls the new `edFlashHint`.
  - The note reads "Removed exact duplicates: N trees, M props. Save to keep." and names only non-zero counts, with
    singular / plural.
  - `edFlashHint` shows it for 8 s via `textContent`.
  - It reads the static hint once and cancels any pending restore on every call.
- No auto-save, and no change to stored data.

## Evidence (fixtures made by the real bug on the old build ef019e0; head e1de243)
- Fixtures:
  - stacked = two rebuilds: 280 trees / 144 props;
  - edited = a map edited on a fresh boot, then Loaded and Saved on the old build: 139 / 71;
  - boot = 70 / 36.
- **Node harness on the real code, 29 assertions, all PASS** (`tools/preview.js eval`):
  - Stacked → 70 / 36, equal to the boot serialization; every other key identical; input not mutated; building it gives
    70 / 36 live.
  - Edited → **71 / 37**: the honest residue. The deleted tree and prop come back once, and the old spots of the moved
    tree and the rotated prop stay. Intended was 69 / 35.
  - Kept: x or z off by 0.1, a different seed or kind, a different prop rot, `seed==null`, a tree and a prop with the
    same fields, and exact duplicates in obstacles / covers / houses / paths / buildings / enemySpawns.
  - Dropped: a seed-0 pair; props with rot missing / 0 / null / −0 count as one.
  - Romania and DM_Cabana: deep-equal to the input. No keys are added. `trees:null` and a string pass through.
  - Hostile saves (strings, `[null,1,"x",[]]`, a fake `{length:1e9}`, kind `__proto__` / `constructor`, own `__proto__`
    keys at the top level and in entries, non-object roots): no throw; `Object.prototype` unchanged.
  - A 20,000-deep entry, where Node's JSON.stringify throws: no throw, and the entry is kept (fail open).
  - Time: median 0.25 ms, p95 0.57 ms on 424 entries.
- **Real page (pageshot, editor.html, storage written through KVStore in the same run), head vs PR #8's code:**

| Step | PR #8 code | Head |
|---|---|---|
| Load the stacked save: live trees / props | 280 / 144 | 70 / 36 |
| Status bar after that Load | default hint | "Removed exact duplicates: 210 trees, 108 props. Save to keep." |
| Stored stacked save afterwards | 280 / 144 | 280 / 144 (untouched) |
| Save under a new name: stored copy | 280 / 144 | 70 / 36 |
| Load that copy within 8 s of the note | n/a | original hint back at once |
| Two stacked Loads 3.5 s apart | n/a | note still on 5 s after the 2nd; original hint 9 s after |

- Canvas:
  - Default editor view, stacked Load on PR #8's code vs head: 0 px different.
  - Orchard view (trees and props in frame): 0 px in tree and prop areas. Every differing pixel sits on the two house
    roofs, which also differ clean-vs-clean between builds. That is the known unseeded house textures
    (house-textures-seeded).
- Note width:
  - 324 px against the static hint's 394 px, so it never wraps where the hint doesn't.
  - Checked at 960 px with a live Cursor readout: no wrap.
  - At 800 px both wrap. That is the existing hint's behaviour.
- Shots: `head_stacked.png`, `base_stacked.png`, `orch-*.png`, `status_*.png` in the session scratchpad (mdl/pg).
- Gate: PASS, all 8 checks; symbols +2 (`dedupeMapDef`, `edFlashHint`).

## Reviews
Workflow: security-auditor and art-director, each finding checked by a skeptic.
- **security-auditor: PASS WITH NOTES.**
  - Prototype safety, `textContent` only, no storage writes on Load and KVStore-only persistence all re-checked.
  - The harness was re-run: pass.
  - DoS: 240,000 entries in 254 ms.
  - **Fixed: SEC-MDL-1 (low).** A very deeply nested field made `JSON.stringify` throw under Node, which failed the Load.
    The verifier refuted this for Chromium, but the fix is one line: that entry is now kept.
  - **Deferred: SEC-MDL-2 (info, older issue).** `envMerge` / `rulesMerge` use `Object.assign`, so a stored `__proto__`
    key swaps the prototype of that local env/rules sub-object; `Object.prototype` stays clean. It belongs with the
    server-side map-def validation (ADR-001) and is logged in STATUS.
- **art-director: PASS WITH NOTES.**
  - The map looks the same as a clean load, and the note reads as good news, not an error.
  - **Fixed: AD-1 (low).** The 86-character note wrapped and was cut off at 960 px or narrower. It is now shorter than
    the static hint (see Evidence).
  - Not taken: AD-2 (the note uses the hint slot's dim colour; the verifier found nothing to fix, optional later) and
    AD-3 (Save overwrites by default, which is existing behaviour; the Brave step already says "new name").

## Decisions and deviations
- The note wording changed from the plan's example to a shorter one (AD-1). It still says "exact duplicates" and never
  "cleaned".
- Prefab dedupe is deferred, as in the plan.

## Risks and open questions
1. The residue: on an old stacked save, Cabana trees / props that the map had deleted or moved come back once. They are
   single entries, so dedupe cannot tell them from deliberate ones. Mustafa removes them by hand if he sees any.
2. Merge order: this PR is based on cs/editor-undo-ground. After #8 merges and this PR is retargeted, I merge main in and
   push, so CI gates it against main.
3. Not verified: Brave, a real GPU, and Mustafa's own saved maps (they live in his browser).

## For Mustafa in Brave (editor.html)
1. Open Maps. If it says "(this session only…)", there are no stored old saves: tell me and skip this check.
2. Load a map saved before today's fixes. The status bar names the removed duplicates, or shows nothing. The map looks
   as before (screenshot). Look for trees or props you had deleted or moved coming back.
3. Save under a **new** name, then Load that one: no note.

## Proposed next step
three-teams (LOOP_V2 §11.1): the plan, for approval.
