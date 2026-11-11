Task: author the E3 mutation table (foundation step 2.1a) -> e3-mutations.json (read-only on repo)

## Plan (blocks)
B1 read sources: phase-0 report 23-27, gate-check-5 report 28-41
B2 read tools/gate.mjs + tools/smoke.mjs (what each check looks at)
B3 find the historical shamble-editor.js for M6
B4 locate exact find strings in the 44446a5 files for every mutation
B5 write e3-mutations.json + uniqueness verifier script; run it
B6 final review, DONE

## Notes
- 5 game files equal 44446a5 in the working tree (checked).
- B1 done: phase-0 report line 24-25 = P1..P4, PC1 (--allow-drop refreshRect), PC2 (pure move -> PASS, listed as moved); check-5 report lines 30-41 = 22 mutations + 3 controls as in the task.
- B2 done: gate.mjs: 1 node --check per file/inline; 2 concatenation per page (also broken src); 3 battery symbols vs base (dropped, moved, cross/intra dupes); 4 battery check4 (unguarded editor-only refs reachable from index build, DOM handler refs); 5b eslint-scope free refs vs page build+builtins+gate-globals.json, const writes, THREE members r128; 5c core free refs to upper top-level names; 5d exact script list per page (flags async/defer/nomodule/type=module; data types skipped). smoke: index menu->Deploy->tutorial->W moves 0.1m->2 clicks->Esc->Menu; editor menu->Map editor->Maps(list fills)->close->Menu; errors on pageerror/console.error/network/WebSocket/file outside repo. Next B3.
- B3 finding: the real bug = refreshPrefabList, refreshMapList, edSavePrefab nested inside bootEditor(); fixed by dbe24f8 (2026-10-14 11:50, on cs/editor-prefab-list, merged to main by PR #4 c458fe7 at 20:58 +0200). main of 2026-10-14 before that = 5858864 (PR #3; also 2d30e4a); blob e37b6d48 identical at b23a133, dbe24f8^, 2d30e4a, 5858864. M6 from_commit = 5858864.
- finding: battery.js symbols = top-level declarations only (battery.js:92). So re-nesting refreshMapList (M7) or the old editor (M6) DROPS top-level symbols vs any base where they are top-level (44446a5 has them top-level at shamble-editor.js:987/1004/1010) -> check 3 FAIL too. The 2026-10-14 run reported only 5a+5b because its default base was merge-base with main = 2d30e4a/5858864 (cs/gate-check-5 was stacked on cs/editor-prefab-list, not yet merged), where the three were still nested.
- finding: gate-globals.json lists no WebSocket/Image/fetch: M4/M5 must use window.WebSocket / new window.Image() so 5b stays green.
- finding: refreshRect (shamble-game.js:1101) is referenced at shamble-game.js:3275 (inside bootGame, runs at boot) and in both pages' inline resize handlers (index.html:1263, editor.html:1266). Deleting only the function: check 3 DROPPED, plus today 5b (not defined) and 5a (ReferenceError in bootGame). PC1 (= P1 + --allow-drop) would then FAIL 5a+5b, not PASS: the Phase 0 report predates check 5.
- finding (plan.md:249-259): E3 runs `node tools/gate.mjs 44446a5 --json` (full gate, smoke on) on OLD and on the head; pass = failing-id sets equal and non-empty per mutation, every control passes in both; differences from the report's grouping are a note, not a fail. Base = 44446a5.
- files have no CR characters (checked with tr|wc).
- B3 done. M6 = from_commit 5858864 (full sha in JSON), path shamble-editor.js at the root (blob e37b6d48). e3-analyze.cjs (battery.analyze in memory, not the gate): old editor 346 symbols vs 353 at 44446a5; dropped _edCanRot, _edToolKind, edFlashHint, edSavePrefab, ovHostileSpawn, refreshMapList, refreshPrefabList (none referenced outside the editor); old editor's unresolved names in editor.html build: refreshPrefabList, refreshMapList, edSavePrefab; check 4 0 flagged. => predicted 3 + 5a + 5b.
- finding: editor in index.html's build leaves 0 unresolved names (M20/M22 predicted 5d only; 5b not tripped by gate logic).
- finding: wrType (game 3015-3018) frees WR, mode, sfxTypeTick (game tops) -> P4 predicted 3 + 5c.
- finding: makeInputCommand (core:1481) is declared and never called anywhere -> host for M21 (no 5a). devReset (game:2214) only runs from the dev panel button (never clicked by the smoke) -> host for M12-M18.
- PC2 choice: _aLerp (game:234, frees none beyond builtins), moved into core just before hyp2 (core:743).
- B4 in progress: finds chosen; next write builder + verifier.
- B4 done: finds chosen (see e3-build.cjs, which builds the JSON from git show 44446a5 and slices wrType, refreshMapList, _aLerp by acorn ranges).
- B5 done: e3-mutations.json written (31 entries: P1-P4, M1-M22, C1-C3, PC1, PC2; 35 edits). Verifier e3-verify.cjs: ALL CHECKS PASS: every find occurs exactly once in its file at 44446a5 and once in the file as edited so far (edits in order); every edit changes its file; ids/order complete; expected ids valid, non-empty for mutations, empty for controls; flags only on PC1; every mutated file still parses (acorn on JS, inline scripts and handlers of pages) except P3, which fails to parse as intended. M6's from_commit resolves and differs from base.
- convention: `expected` = the report's named checks verbatim; where gate.mjs logic predicts a different result on base 44446a5, `notes` has a sentence starting "PREDICTED ON 44446a5: [...]".
- predictions differing from the reports: P1 [3,5a,5b]; P2 [4,5a]; P3 [1,2,3,4,5a,5b]; P4 [3,5c]; M6 [3,5a,5b]; M7 [3,5a,5b]; M19 [5b]; M20 [5d]; PC1 [5a,5b] (NOT a pass: the control cannot pass the full gate as written).
- M6 shape: edits: [{file:"shamble-editor.js", from_commit:"<full sha>"}] (the object the task gave, inside edits).
- C3: index.html:321 has a prefers-reduced-motion rule that kills #title animations; Playwright's default is no-preference, so the pulse runs in the smoke.
- Next: B6 final review (advisor), then DONE.
- C3 check: no animationend/transitionend listener in shamble-game.js, shamble-core.js or index.html, so the pulse cannot stall the Deploy flow.
- B6 done (advisor review: table ships as built).

## Uncertainties (for the answer)
U1 PC1 cannot PASS the full gate on base 44446a5: refreshRect is still referenced (shamble-game.js:3275 in bootGame; index.html:1263, editor.html:1266), so 5a and 5b fail; only check 3 flips to "dropped (allowed)". The plan's rule "every control passes in both" (plan.md:255) cannot hold for PC1 as written: the session decides (judge PC1 on check 3's line, or redefine it).
U2 Base difference: the reports ran under other conditions (Phase 0: checks 1-4 only; check 5: default base = merge-base with main 2d30e4a/5858864, where the three editor functions were nested). Predicted on 44446a5: P1 [3,5a,5b], P2 [4,5a], P3 [1,2,3,4,5a,5b], P4 [3,5c], M6 [3,5a,5b] (7 editor symbols dropped), M7 [3,5a,5b]. `expected` keeps the report's checks; the predictions are in `notes`.
U3 M19/M20: the report pairs them under "5b + 5d" without saying which fails which. Predicted M19 [5b], M20 [5d].
U4 M6 shape: {file, from_commit} placed as the single element of `edits` (the task wording also allows it in place of `edits`). Full sha 58588648e4b65b4c77d4e18255facceb3bcfc31b. M6 may show more 5a errors than the three ReferenceErrors (an old file on today's game).
U5 M5 relies on Chromium emitting a file: request event for an <img> from a file:// page (smoke.mjs flags file: requests outside the root); a console error for the missing file would fail 5a anyway.
U6 M4/M5 use window.WebSocket / new window.Image() (gate-globals.json lists neither); how the 2026-10-14 session wrote them is not recorded.
U7 M8, M9, C1, C2, C3 apply to index.html only (the report names no page); M22 is read as an extra editor tag with type text/ecmascript in index.html; M20 is a closed tag.
U8 Assumed, not verified: M20/M22 stay green on 5a (the pages share their DOM; the editor's top level is declarations only); P2's double bootEditor on editor.html is of unknown effect (index.html fails 5a regardless).
U9 C2 adds a top-level var onE3Probe to index.html's inline script: check 3 should print +added (not a failure).
DONE
