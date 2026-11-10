# test-engineer: kit-v3-5 (F1), PR #46, head f9728c9 vs main 44446a5

## Plan
1. Read the guard diff and the test diff (mergeAllowed, merge block, total_count, protectedRel part A, CANON, GATE_FILES).
2. Run the three suites myself; record counts.
3. Extra mutants in a scratch copy (te5/): @tsv parse, final-empty-line drop, cf type checks, places' case, others.
4. Fakes vs live gh (read-only): PR #1 object (changed_files), its files with the @tsv filter, a commit's check-runs (total_count).
5. Red-then-green: rebuild the red run on main's guard, check only the assertion loops changed.
6. Gaps: removed file names, copied files, empty list + changed_files 0, @tsv escapes of newline.

## Findings (a line each)
- Counts (mine, f9728c9 worktree): routine-guard 1086/1086, settings 127/127, checkpoint 27/27. Scratch copy of hooks: 1086/1086 (needs guard.mjs beside it).
- SHA-256 of the worktree guard and tests equal kb7/SHA256SUMS (6507f795..., 240ea071...): the evidence is of this code.
- Red reproduced (te5/red = kb7/red; red guard byte-equal to main's; the test copy differs only in the two unmutated assertion loops plus a RED collector): 47 of 1086 fail, 43 rows (39 routine, 4 day). Of 52 new rows, 9 pass on main: 7 expected-false rows (docs/package.json, tools/x.mjs, README+FIRST_PROMPTS, docs/a-b_c.1.md, a docs rename, 2999 files, K5 linux) and 2 already refused there (.github/workflows/x.yml, gate still running). Sound.
- Low: the brief says "46 new merge rows"; the diff has 44 merge rows (41 PRA + 3 DAY) + 3 night + 5 day = 52 = 1086 - 1034. 16 mutants (15 routine + 1 day) is right.
- Live (read-only): PR #1 changed_files 54 (number), @tsv 54 lines, each "new\told" or "name\t", final "\n" only; check-runs on f9728c9 total_count 4 (number) with 4 runs; on 2b8c596 (the PR's current head, a [skip ci] checkpoint) total_count 0, no runs.
- gojq @tsv escapes live: "a\nb\tc\\d\re" -> a\nb\tc\\d\re (backslash sequences), so L1 refuses every escaped name. Confirmed.
- Pagination live: per_page=10 on PR #1 -> exit 1 after page 1 (proxy 403 on repositories/{id} Link). Known and recorded (STATUS:1011, PR #40); fails closed.
- Extra mutants (te5/runmut.mjs, 24, each a full-suite run in te5/m<i>; results te5/mutants.txt). A "fail=1" that is only the in-suite mutation test failing on "its target text occurs exactly once" is NOT a kill (the mutant edited a listed mutant's target), so it counts as surviving.
  - Killed by rows: X4 (no final-line pop), X6 (only fewer lines), X7 (only the old name), X9 (dir places lower-cased; Docs/x.md), X18 (cap removed), X19 (empty segment), X20 (dot only on segment 0), X22 (rules name only at root).
  - Equivalent / fail closed: X2 (no --paginate: >100 files then fails the count), X3 (drop every empty line: @tsv lines are never empty), X8, X15 (L4 already makes the prefix exact), X17, X23, X24.
  - Surviving, real: X1, X10, X11, X12, X13, X14, X16, X21, X5 (see below).
- MEDIUM test:83/57 fakeIo's gh ignores --jq and --paginate; X1 (filter back to '.[].filename') survives, so the C5 rename check rests only on E5's one-time live read. Fix: answer the files path only for the exact args (['api','--paginate',path,'--jq',FILTER]) or assert them in one row.
- MEDIUM guard:94 the place boundary has no row: X11 (dir place without its slash: docsx/a.md, tools.mjs), X12 (file places as prefixes: README.md.js), X10 (file places lower-cased: readme.md) survive. Code is right (all refused by hand). Fix: rows docsx/a.md, tools.mjs, README.md.js, readme.md, all refused.
- MEDIUM guard:98 the decision-doc fold and list are mostly unpinned: X13 (adr test on rel, not low: docs/ADR/0001-x.md) and X14 (LOOK_V2 dropped) survive; 8 of 10 names have no row. Fix: row docs/ADR/0001-x.md; one mixed-case row per DECISION_DOCS name (table-driven).
- MEDIUM test:310 the L3 case-fold row is also refused by L6 (package.json under tools/), so X21 (exact 'node_modules') survives. Fix: tools/Node_Modules/acorn/dist/acorn.js.
- LOW test:312 the short-name row is at the root, where L4 refuses it first; X16 ('~' in L1) survives. Fix: docs/CLAUDE~1.MD.
- LOW guard:1499 X5 (loose != on cf) survives: no row with changed_files as text. GitHub sends a number (live), so reach is nil. Fix: row changedFiles '1' refused.
- LOW guard:1493-1499 an empty list with changed_files 0 goes on (probed through decide()); no row pins either choice. Fix: refuse !Number.isInteger(cf) || cf < 1 and add the row (or a row pinning "goes on").
- LOW test:44,289 the default fake shape is 'docs/a.md\n'; gh prints 'docs/a.md\t\n'. Harmless today (split gives the same name). Fix: defaults in the @tsv shape.
- Probed through decide() (te5/q4.mjs), all refused: removed rules file, copied from a rules file, escaped \n in a name, total_count "2", CRLF, a blank line between pages.
- LOW (wording) the brief's "46 new merge rows": 44 merge rows; 52 new rows in all.
- NOTE PR #46's head is now 2b8c596 ([skip ci], checkpoint only): check-runs total_count 0, so the merge check refuses that head until CI runs on the final head.
- Verdict: PASS WITH NOTES (no Critical or High; four Medium row gaps, code correct on every probed case).
