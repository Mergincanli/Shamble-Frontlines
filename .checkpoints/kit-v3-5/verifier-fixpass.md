# Verifier: kit-v3-5 fix-pass re-check (head 258afb0; not a round)

Ruling: `advisor-fix-pass.md` (e621d45). Base for the fix pass: 9bacf47 (the kit files' last change before 08423ce).
Scratch: the session scratchpad's `vf6/`.

## Plan
1. Ruling vs fix pass, item by item (word diff 9bacf47..08423ce on the 3 kit files).
2. Kit suites at head (1107 / 127 / 27) + red claim (new tests on 9bacf47's guard: 4 rows).
3. Own mutants on the new guard lines (scratch copy).
4. Gate vs origin/main --no-smoke (copy with deps) + repo-map-check.
5. Docs lines: V3, V6, W1-W3, SA5-3, SA5-5, PX-1 follow-up; PX1-ROUTE quote byte-equal; long-line count (chars).
6. sha256 vs kb8/SHA256SUMS.
7. Scope since f9728c9.

## Results
- 6 PASS. sha256 at the worktree, at 08423ce and at 258afb0 equal kb8/SHA256SUMS: guard bb680283, tests bfd12367,
  advisor.md da27814e (= kb8/role-next.md); 08423ce..258afb0 touches no kit file.
- 7 PASS. `git diff --name-only f9728c9..258afb0`: the 3 kit files, docs/STATUS.md, docs/ROADMAP.md, 8 under
  .checkpoints/. PR #46 head on GitHub = 258afb0 (ls-remote refs/pull/46/head); main 44446a5.
- 1 advisor.md PASS. Word diff 9bacf47..08423ce: only the tier-2 bracket (:80-86). Bracket text, whitespace joined, equals
  the ruling's quote byte for byte (659 bytes; vf6/ruling-bracket.txt vs vf6/applied2.txt). No line over 120.
- 1 guard PASS. Word diff: two hunks only. npmCheck (:561-565): shrinkwrap added to the diff list; then
  `git ls-files --others -- tools/npm-shrinkwrap.json`, refused unless code 0 and empty (fails closed on git error).
  mergeCheck (:1498, :1504): `cf < 1` refused before the read; any line whose first tab field is empty refused.
  Real-git probe (vf6/gitprobe): clean -> allowed; untracked -> listed; untracked and excluded by .git/info/exclude ->
  still listed (no --exclude-standard); staged or committed and differing -> diff 1; all refuse as intended.
- 1 tests PASS WITH NOTE. TE1 (fake gh answers /files? only with --paginate and the exact @tsv filter; mutant
  '.[].filename'), TE2 (docsx/a.md, tools.mjs, README.md.js, readme.md), TE3 (docs/ADR/0001-x.md + one mixed-case row for
  each of the 10 DECISION_DOCS names, checked against routine-guard.mjs:87-88), TE4 (tools/Node_Modules/acorn/dist/
  acorn.js), V2 (agent memory row), TE5 (docs/CLAUDE~1.MD), TE7 (default prFiles 'docs/a.md\t\n'), SA5-2 rows (empty
  list; nameless line) + 2 mutants, SA5-1 rows (untracked; differs) + 2 mutants: all present. V1: the row
  'docs/a.md\t\n.claude/settings.json\t\n' (@tsv, each second field empty) is my round-1 probe row: without
  .filter(Boolean) the first hit is '' (falsy) and the rules file merges, so the in-suite mutant 'empty fields dropped'
  fails this row. SA5-2's "blank line inside the list": the row's nameless line is the last line, not inside (see 3).
- 2 PASS. Archive copy of 258afb0 (vf6/tree): routine-guard 1107/1107, settings 127/127, checkpoint 27/27; in-suite
  mutation tests pass (no surviving listed mutant).
- 2 red PASS. New tests + guard.mjs of head beside 9bacf47's routine-guard.mjs (vf6/red): 7 of 1107 fail = exactly the
  4 rows (night untracked shrinkwrap, night shrinkwrap differs, empty list, nameless line) + the 3 "mutation runs"
  tests (new mutant targets absent from the old guard). Matches build.md's claim.
- 4 PASS. vf6/gaterepo (clone at 258afb0, origin/main 44446a5, `npm ci --prefix tools --omit=optional --ignore-scripts
  --no-audit --no-fund`, 9 packages): GATE PASS, 1-4 and 5b-5d PASS, symbols +0 -0 (core 138 / game 714 / editor
  353), 5a skipped. repo-map-check OK, 114 entries / 101 paths (clone and worktree).
- 1 rule-diff PASS. Rows 18-20 (rule-diff.md:36-38) state the new code and bracket exactly (`cf < 1`, empty first field;
  shrinkwrap + untracked; the bracket's before/after text matches the word diff); row 15's V5 note present (:28).
- 5 V3 PASS. STATUS:146 (was :145) cites `docs/adr/README.md:104`, `:105`, `:123`, `:161`, `:184`; at head those lines
  hold OQ-0003-15, OQ-0003-16, OQ-0005-CD7, OQ-0006-21, R2-ROOM-OQ7 in that order (register unchanged since f9728c9).
- 5 V6 PASS. STATUS:1018-1021 (the foundation line) equals advisor-plan-rev1.md:461-464's E4 foundation wording, with
  "after the gate step or by his explicit word".
- 5 W1-W3 PASS. STATUS:1024-1026: W1 Medium, W2 Medium, W3 Low = security-auditor.md:70-75; deferred under KIT-FREEZE-2.
- 5 SA5-5 PASS. STATUS:1029-1030; its quote "the fixed list in your plan and nothing more" is in his answers file
  (SA5-RIDE line) verbatim; CLAUDE.md:20-22 is the phase-0 line naming ENGINE_OVERVIEW as a decision draft.
- 5 SA5-3 PASS WITH NOTE (Low). STATUS:1027-1028 is not the ruling's quoted line (advisor-fix-pass.md:68-69): it drops
  "(his direction of 2026-11-10: no building for Windows name aliases)" and adds the checkout effect (security-auditor
  SA5-3, :24-26). Defensible: his 2026-11-10 words (answers file, item 2) name short names, "::$DATA" and trailing dots
  and spaces, not reserved names, so the ruling's attribution would have put a direction in his mouth. But build.md
  does not record the deviation; the advisor should confirm the wording.
- 5 PX-1 PASS. STATUS:659-667 (PX-1 at :660): Critical on main before kit-v3-5 if confirmed, Medium after it, the fix as its own kit
  change-set (= the ruling). His quote "(a). A STATUS follow-up, as its own change-set after the foundation's gate
  step." equals the text block's PX1-ROUTE answer after its label, whitespace joined (no wrap inside a word).
  The file's text block (746 chars, sha256 0bf169cb1fb56402) equals the transcript's attachment entry e5811e7c...
  `.attachment.prompt` byte for byte (timestamp 2026-11-10T15:00:44.797Z) (vf6/tx.py).
- 5 long lines PASS. No added line over 120 characters outside tables in docs/ or advisor.md, f9728c9..258afb0 and vs
  main. Correction of my round 1: STATUS:710 (now :720) and ROADMAP:110 (now :111) are 120 characters (121 and 122
  bytes, a "−"); I counted bytes. V4 needed no change.
- 3 PASS WITH NOTES (vf6/runmut.mjs, results vf6/mutants.txt; a kill needs a failing row, not only a "mutation runs" test).
  Round-1 survivors the ruling targets, now killed: X1 (7 rows), X10 (readme.md), X11 (docsx/a.md, tools.mjs), X12
  (README.md.js), X13 (docs/ADR/0001-x.md), X14 and my X14b-d (look_v2, units, Decisions, Editor_V2), X16
  (docs/CLAUDE~1.MD), X21 (tools/Node_Modules/acorn/...). Still surviving, outside the ruling: X5 (TE's Low, not ruled),
  X8, X15, X17, X23 (TE's equivalent / fail-closed set).
  Mine on the new lines: killed N1 (cf < 0), N3 (empty test on the second field), N9 (manifest result ignored), N11
  (cf < 1 removed). Surviving: N2 cf === 0 (equivalent: a non-integer cf already fails the count check). Low test gaps
  (code right, real-git probe in 1): N4 `!l.trim()` (only a tab-only line refused; "\tdocs/old.md" would pass) and N5
  (only the last line checked): the nameless-line row is "\t" as the last line, not inside the list as the ruling says
  (routine-guard.test.mjs:350). N6 (git error on ls-files read as clean: fail-open), N7 (`--exclude-standard` added:
  a gitignored shrinkwrap missed) and N8 (ls-files on another path): the fake answers any `ls-files --others` the same
  (routine-guard.test.mjs:77), so the read's args and its exit code are unpinned (the same class as TE1, on a Low's fix).
  Note: the X1 full-suite run hung after reporting (a defunct child); killed after 20 min. Head's suite does not hang.

## Verdict: PASS WITH NOTES (no Critical, High or Medium; three Lows; one round-1 count corrected)
