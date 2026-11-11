Task: test-engineer ROUND 2 review of foundation step 2.1a, PR #50, pinned 76544aa (diff 3c98ad5...76544aa).

## Plan (blocks)
B1. Read own round-1 notes (test-engineer-review.md) for TE-1..TE-n.
B2. Diff stat + the gate.mjs fix (3f29cb8) code read.
B3. rows-test-f1.mjs + two recorded runs (red on 2bfab53, green on 3f29cb8).
B4. Re-run green rows into .gate/rows-f1-te-r2.
B5. TE-1: base-* evidence raw with exit codes.
B6. E3: e3-run-f1.mjs, e3-results-f1.json, e3-compare.txt; legacy column exercises new check?
B7. Docs check 12 blocks: mutate a copy, re-run.
B8. Unreached code in the fix.
B9. Verdict written.

## Notes
B1 DONE (round-1 notes read; TE-1 = Medium revert-case raw evidence; TE-3 = rows gaps incl. base.prefix never asserted).
B2 DONE (fix read: tools/gate.mjs 2bfab53->7a8480c, +readdirSync, check after base.prefix, before the smokeOnly exit; base null under --smoke-only so the check never runs there (stated); regex ^(index|editor).html|shamble-.*.js$ /i on readdir of join(ROOT, base.prefix); existsSync(was) false -> []).
B3 DONE (rows-test-f1 read + both runs). Red run (f1 expectations on 2bfab53): h,i,j,l FAIL exit 0, rest PASS, exit 1: meaningful, the 4 move rows are the only diffs. Green: 13+2 ALL PASS, h/i/j/l exit 2 with the moved message; k (no move, root page) exit 0 = the negative case; m clean revert exit 0. base.prefix asserted '' (a,b) and 'legacy/' (k,m): TE-3's prefix gap closed; smoke CLI c now asserts /two places/ (TE-3 part closed); row h's no-smoke now moot under f1 (the gate stops first).
 - finding (Low): row i "decoy" copies the CLEAN index.html/editor.html to the root and writes broken-core.js unreferenced: the comment says "root pages wired to a broken script", they are not. Irrelevant to the presence check (names only); the wired decoy is the evidence-f1/decoy-* runs. Comment only.
 - finding (Low): row m always has an EMPTY legacy/ folder (mkdirSync legacy for every row), so the existsSync(was)=false branch is not reached by the rows; check B5 whether the base-revert worktree run (no legacy/ folder) reaches it.
 - note: the msg is asserted on stderr's first line only; i asserts /still holds .*index\.html/ (both names), j exact case, l the full revert sentence.
Scratch check: .gate/fdn-scratch-head tools tree == 76544aa's tools tree file by file; five files == 76544aa == 44446a5; 89286ed exists, its legacy/ tree == 44446a5's five blobs, diff 44446a5..89286ed = 5 pure renames.
Next: B4 re-run green into .gate/rows-f1-te-r2.
B4 DONE (re-run 2026-11-11 by test-engineer: rows-test-f1.mjs .gate/fdn-scratch-head .gate/rows-f1-te-r2 f1 89286ed, 1 m 22 s, 13+2 ALL PASS exit 0, identical line by line to rows-f1-on-3f29cb8.txt except smoke 'moved 1.4 m' vs 1.6 m (random walk distance, not asserted). No flake.)
Next: B5 base-* evidence.
B5 DONE (TE-1). evidence-f1/base-revert-root-head-vs-legacy-base.{json,exit,stderr}: exit 0, base prefix "legacy/", build ".", pass true, stderr empty; base-2-1b-legacy-head-vs-root-base.*: exit 0, prefix "", build "legacy/", pass true; base-revert-text.txt shows the "(files in legacy/)" header (also closes TE-6); main's gate on the legacy/ base exit 2 "not found at base 7c22abf" (the reason for the split). Raw, exit codes in their own files, no pipes. TE-1 CLOSED. Plus my own reproduction on the reproducible base 89286ed (7c22abf is local-only): own copy .gate/foundation-restructure/te-r2-rev (fixed gate, root layout, NO legacy/ folder) `gate.mjs 89286ed --no-smoke`: header "(files in legacy/) | head = ... (files at the root)", exit 0. This reaches existsSync(was)=false, so B3's Low on the unreached branch is withdrawn (the revert worktree run and mine reach it).
B8 probes (same copy): legacy/Shamble-Game.JS left after a revert: exit 2 "legacy/ still holds Shamble-Game.JS" (findBuild ignores it: case-sensitive on Linux, so only the new check catches it); renamed to legacy/shamble-game.js.bak: exit 0 (not a page/engine name: correct). 76544aa's root names matching the regex: exactly the five (no false positive waiting for 2.1b). The 2.1b-direction run (prefix "") lists the real worktree root and passes: no false positive on the real root.
 - note: the check lists the working tree, so an untracked local copy of a page at the root also stops the gate after 2.1b; fails closed, CI sees tracked files only. Not a finding.
 - note: only the top level of the old folder is listed (legacy/sub/index.html not caught); findBuild ignores subfolders too, so nothing loads it. Not a finding.
Next: B6 E3.
B6 DONE (E3). Own read-only comparer .checkpoints/foundation-restructure/te-r2-e3-compare.mjs: 31 ids same order; 93 runs; 0 diffs in exit, failing ids, build, base prefix, verdict; 0 diffs in every non-5a check result line (5b's Node/allow-list string included); newleg: base prefix '' and build legacy/ in 31 of 31 = a move on every entry, so the new check ran (its pass path: root clean) in every legacy/ column run. Worktrees now: fdn-new HEAD 3f29cb8, fdn-newleg 7c22abf (parent 3f29cb8, gate blob 7a8480c), fdn-old 44446a5, all clean except tools/node_modules. E3 does not reach the stop path (by design: rows h-l and the decoy do). Next: B7 docs check.
B7 DONE (docs check, 12 blocks). Own harness .checkpoints/foundation-restructure/te-r2-docs-mutate.py on copies of DECISIONS + report at 76544aa (.gate/foundation-restructure/dc/): 9 of 9 as expected: C0 exit 0; N1 byte in A3.5, N2 A3.4 last line dropped, N3 A3.4 lines reordered, N4 byte in A3.3 in the appendix only, N5 A3.5 removed whole (fires 'each source block inserted exactly once'), N6 trailing space in A3.3, R1/R2 round-1 M1/M2 re-run: all exit 1. Still bites.
B7b DONE: independent: A3.3/A3.4/A3.5 fenced blocks in the report at 76544aa equal the capture notes' blocks byte for byte (mustafa-2026-11-11-f1.md 16 lines, -prompts.md 8, -prompts-correction.md 8; te-r2-notes-vs-appendix.py, read-only). The notes vs the chat events: not checkable by me (TE-5 stands as a follow-up).
B8 DONE (unreached code). Every branch of the new lines reached by a recorded or my run: condition false (rows a,b? no: a/b have prefix '' and build '.'/'legacy/': b is a MOVE row with a clean root -> pass path; a and k no move -> skipped); existsSync true + empty list (b, m, E3 newleg x31, 2.1b-direction worktree run); existsSync false (revert worktree run, my 89286ed run, my shamble-frontlines-repo/ base 64c89a1 run: exit 0, header "(files in shamble-frontlines-repo/)" = A-1's path, no false stop); die with 'it' (h, j, l, my probe) and 'them' (i, decoy); name() root (h,i,j) and legacy/ (l, my probe). Not reached: the stop on a shamble-frontlines-repo/ leftover (A-1, accepted Low, old bases only). --smoke-only never runs the check (base null): stated known limit; CI's gate job runs it on every PR and push.
 - TE-2 (BASE_PREFIXES order not discriminated) unchanged: follow-up, already in STATUS per the checkpoint. TE-3 closed. TE-4, TE-5 follow-ups. TE-6 closed (base-revert-text.txt).

## B9 Verdict (written)
VERDICT: APPROVE. No Critical, High or Medium open. TE-1 (Medium) closed; TE-3 and TE-6 closed.
Table:
- Fix stops a page/engine file left in the old folder after a move (both directions) | rows h,i,j,l exit 2 on 7a8480c, exit 0 on 2bfab53 (red first); my re-run 13+2 ALL PASS | none
- No false stop without a move, or on a clean move/revert | rows k, m, b; E3 newleg 31/31 a move, all identical; 2.1b-direction run on a real worktree root | none
- Letter case caught | row j (Index.html), my legacy/Shamble-Game.JS probe | Windows itself not run (his required step)
- Base prefix read right both ways (TE-1) | base-*.json/.exit raw: prefix "legacy/" and "", exit 0; my run on reproducible 89286ed | 7c22abf local only: reproduced on 89286ed
- Fix changes no other check | E3 93 runs: exit, failing ids, build, prefix, every non-5a result line identical to 2026-11-10 | 5a wording not compared (random distance), ids equal
- Decoy: gate job catches it, --smoke-only does not | decoy-* on 277eb96 / 2bfab53 / 7a8480c / --smoke-only | stated limit
- DECISIONS = appendix, 12 blocks | my 9 mutations on copies all as expected; appendix A3.3-A3.5 = capture notes | notes vs chat not checkable (TE-5)
Findings:
1. Low: rows-test-f1.mjs row i's comment says the root pages are "wired to a broken script"; the row copies the clean pages and writes an unreferenced broken-core.js. The check is name-based, so the row still proves the stop; the wired decoy is evidence-f1/decoy-*. Comment only (the file is deleted before ready).
2. Low (carried): TE-2, TE-4, TE-5 stay follow-ups.
DONE
