# docs-keeper: the docs pass for ci-after-3a (worktree .gate/ci3, head 832268e, main c1da1bd in)

## Plan
1. DECISIONS.md: 8 entries (one per message of his in this chat), each placed by its UTC send time among the
   existing entries, newest first. Quoted text copied by a node script (`docs-keeper-decisions.mjs` here) from each
   saved file's text after the `---` line, its bytes and sha256 checked against the saved file's header before
   insertion. Headers written for the file: his label time (Amsterdam) and the UTC send time; "What it answers, in
   the chat's words (not his)" from the saved headers / question files; records (mode `default`, the write at
   18:02:02 UTC, the three sha256s, the runs) under the ask-2 entry, marked "Records, not his words".
   Positions (UTC): 18:00:43 and 16:41:57 above 13:55:08; 13:54:38 between 13:55:08 and 12:14:23; 11:07:23 between
   11:38:37 and 11:07:07; 00:21:41, 00:20:12, 00:07:29 between 00:22:09 and 2026-11-24 23:53:15; 22:50:34 between
   22:59:49 and 22:44:50.
2. STATUS.md (<= 120): last-updated; Last merged (#67 c1da1bd, #69 311c048, #68 3ab9eae: read with git log on main);
   In flight: #67 and #69 rows moved by script to the archive's "In flight" (merged); the ci-after-3a row added; Next
   item 2; Waiting on: the ci-after-3a merge; Follow-ups: a ci-after-3a bullet, the 3a bullet's two ids marked.
3. STATUS_ARCHIVE.md: dated marks on 3A-GEN-CHECK (done), 3A-RULES-STEP (CI half done), FIREFOX (#61 block), "Firefox in
   CI" (#64 block), 3A-OTHER-BROWSERS (Firefox half on Linux), 3B-OTHER-BROWSERS (still open: PM-0002-12 is a drawing
   test), CUT-PATH and TOKEN-WORDS pointers; new block "From ci-after-3a" (E1 done; FF-DRAW, FF-SPEC-COMMENTS,
   FF-FILEURL-PREF, FF-STOP, FF-SANDBOX, CUT-PATH evidence, TOKEN-WORDS, FF-ORDER).
4. REPO_MAP.md: rows for toolchain.yml, playwright.config.ts, crates/devtools/vectors/, the report; run the check.
5. ROADMAP.md: entry at the top of "Recent work"; fold the oldest (docs-after-54) into "Build history" by script.

## Notes (a line after each file)
- DECISIONS.md done: 8 entries inserted by docs-keeper-decisions.mjs (224 lines added, 0 removed; 3176 -> 3400); --check: all 8 quotes equal the saved bytes and sha256; placed by UTC send time (lines 130, 174, 223, 480, 545, 569, 580, 714).
- STATUS.md done: 120 lines (was 120); 8 lines changed, 8 in; #67 and #69 In flight rows (old lines 25, 26) moved by docs-keeper-move-rows.mjs byte for byte to the archive (lines 238-239, sha256 e2274b6b... and 96163e72...); last-updated, Last merged, the ci-after-3a row, Next item 2, the 3a follow-up bullet and a ci-after-3a bullet (one long line, like the #69 bullet). Left alone (other lanes): Waiting on 3b go, #67 go, Next item 3, the #69 open questions line, the main CI line (09faefa7).
- STATUS_ARCHIVE.md done: 76 lines added, 0 removed (2062 -> 2138). In flight: a "Moved from STATUS" note with the #67/#69 rows (by script); Follow-ups: pointers on 3A-GEN-CHECK (done), 3A-RULES-STEP (CI half), 3A-OTHER-BROWSERS (Firefox half), 3B-OTHER-BROWSERS (open; presses.spec opens the page, verified), FIREFOX (#61 block), Firefox in CI (#64 block), CUT-PATH, TOKEN-WORDS; new block "From ci-after-3a" (E1, FF-DRAW, FF-SPEC-COMMENTS, FF-FILEURL-PREF, FF-STOP, FF-SANDBOX, CUT-PATH-N1, TOKEN-WORDS, FF-ORDER); 3a risk 3 pointer; a Health row (runs read with gh).
- REPO_MAP.md done: 4 rows corrected (crates/devtools/vectors/ "CI does not run it yet" -> "CI runs both checks inside the network cut since ci-after-3a"; playwright.config.ts "Two projects" -> "Three projects", firefox runs tests/rules/ only; tests/rules/ "(the runner's Chrome in CI)" -> Chrome and Firefox; toolchain.yml: release tests of six crates, generator checks, devtools wasm32 line, libEGL line, Firefox file check, version and firefox project) and 1 row added (docs/reports/2026-11-25-ci-after-3a.md). repo-map-check: "repo map OK: 198 entries cover 109 required paths" (exit 0, run in the worktree). Not added: a row for docs/reports/2026-11-25-guard-env-fix.md (#69, another lane; the check does not require it).
- ROADMAP.md done (docs-keeper-roadmap.mjs): ci-after-3a entry (8 lines) on top of Recent work; the oldest entry (docs-after-54, 9 lines, sha256 a80f1407...) moved byte for byte into Build history after night-shift-stints; Recent work back to 12 entries.
- Drift: START_HERE, TECHNICAL, MODULE_MAP, ART_DIRECTION, ANIMATION_PLAN hold no claim this change made untrue (grep for "Chromium only", "Two projects", the old release line, Firefox). No ADR accepted.
- Process: one early call ran `cd <worktree> && git log` (git not first), before the interim rule was applied here; every later git/gh call started its own command from the main folder with explicit ids.
- After the advisor tool's review: DECISIONS re-run (ask-2 records now give the 300 s stop and the chat's reading with its basis; "readable at commit 832268e"; --check 8/8 OK; +227 lines); FF-SPEC-COMMENTS reworded (the comments still read true; update with FF-DRAW); CUT-PATH-N1 names E10's limits; STATUS: 3b's merged Waiting-on bullet moved by script to the archive, a Waiting-on bullet for the ci-after-3a go and FF-DRAW-WHEN added (120 lines); old and new text in docs-keeper-old-new.md. Checked: .gate/egl absent, .gate/pw-firefox-1572 present; 77d28ee^2 = 311c048. Final: STATUS -12 +12 (120), DECISIONS +227 (3403), STATUS_ARCHIVE +85 (2147), ROADMAP -10 +19 (377, a move plus an entry), REPO_MAP -4 +5 (226); repo-map-check OK (198 entries, 109 required paths).
