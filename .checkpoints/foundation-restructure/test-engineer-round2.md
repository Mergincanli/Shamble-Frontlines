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
