# docs-check-r1-sync: adversarial fact check of docs-keeper's review round 1 sync (foundation 2.2, PR #48)

Agent: adversarial fact checker (workflow step). Worktree: .gate/foundation-toolchain-wt, branch
claude/laughing-pasteur-fwya24, HEAD 8632249. Edits: none outside this notes file.

## Scope
docs-keeper's uncommitted edits (`git diff`) to docs/STATUS_ARCHIVE.md ("From foundation 2.2", :551-802) and
docs/STATUS.md (:5, :29, :35-39, :42, :95-99). Sources: the report docs/reports/2026-11-12-foundation-2-2.md (findings
table :218-248, follow-ups :250-269, Reviews :137-166, Decisions needed :271-305, Appendix A :315-486), the ruling
advisor-ruling-d1.md, review-r1-findings.json, review-r1-verdicts.json, skeptic-r1-D*.md, and the repo files cited.

## Plan (units, each saved as done)
1. Mechanical checks: STATUS line count, repo-map-check exit, git status --short, OVERFLOW-WASM grep, name sets
   report vs archive vs STATUS (by script), quotes of Mustafa vs Appendix A, line lengths.
2. Every [verified 2026-11-13] citation re-read: toolchain.yml, gate.yml, Cargo.toml, package.json, ADR 0002 lines,
   CLAUDE.md :119/:130, advisor.md lines, routine-guard.mjs:1486, plan.md Part 3.
3. Each reworded/new archive item vs the report row + the reviewer finding + the skeptic verdict: G0, OVERFLOW-XCRATE-CI,
   REQUIRED-CHECK, SETUP-NODE-CACHE, WASM-ARTIFACT, CUT-CARRY, CUT-ESCAPE, SETPRIV-HARDEN, CANCEL-ON-MAIN,
   LINT-TABLE-PEDANTIC, FEATURE-UNIFY, ROOT-WORKSPACE, CLIENT-NET-RULE, OVERFLOW-PER-CRATE, EMPTY-SLOTS, OQ-0002-8-TEST.
4. The archive intro (codes, provenance tags) and STATUS's #48 row, Waiting on Mustafa, Next item 2, follow-ups.
5. Decisions of Mustafa's: nothing recorded as given (K1, Q2.2-a, OVERFLOW-ALL all still asked).

## Progress
- plan written (this file).
- Unit 1 (mechanical) DONE: STATUS.md 120 lines; repo-map-check "repo map OK: 120 entries cover 107 required paths",
  exit 0; git status --short: the two docs + docs-keeper-r1-sync.md (M) + this file (??), nothing else; OVERFLOW-WASM in
  docs/ only at STATUS_ARCHIVE.md:614 ("it replaces OVERFLOW-WASM"), rest in .checkpoints notes; names by script
  (scratchpad names.mjs): report 29 + 5 = archive = STATUS, no extras, no dups; STATUS order = report; archive order
  differs only by NODE-26's older position (after SAVER-SKIP-CI, pre-existing). Lines: archive 680/681/800 >120 are old
  lines; STATUS :29 (#48 table row) is 1436 chars (a table row; docs-keeper's "every new line <= 120" is not true of it).
- Unit 2 (repo citations) DONE, all hold on the branch: toolchain.yml :3, :16-18, :21-22, :46 (sed, so M2's exit 2 under
  bash -e), :47-52 (exit 1 branches), :48-50, :68-73, :69 (setpriv exact), :74 (id; ip -o link), :78, :82-87;
  gate.yml :15-17; setup-node 820762786026 = 8207627 (v7.0.0) at toolchain.yml:37, gate.yml:29 and :60; Cargo.toml :2-4,
  :25-29; root package.json: no packageManager, no devEngines; ADR 0002 :207-209, :212-217, :345-346, :424, :433-436,
  :569 (G0 exit 2 on any setup failure), :571 (G2 Rust lint), :574 (G5 same-feature check), :578 (G9), :581 (G12),
  :642, :1082-1084 (#4533), :1095-1102 (answered by the Advisor 2026-11-03, (a)), :1303 (PM-0002-37, 1 run), :1305,
  :1308, :1310 (PM-0002-44, no phase), :1319 (test in sim), :1323 (PM-0002-57), :1330 (PM-0002-64, 2.5, 1-minute),
  :1421-1422 (foundation checklist item 2); CLAUDE.md :119, :130; advisor.md :157, :163, :224, :273-276, :294, :323;
  routine-guard.mjs:1486; register docs/adr/README.md :95 (Q2.2-a, "His go for this row: PR #48's merge") and :234
  (OVERFLOW-ALL: never his decision, go = #48's merge, engine-architect rewords part 4/PM-0002-53). Kit grep: other
  gate+smoke hits are comments (guard :6), a deny message (guard :990), the gate skill's --no-smoke, or gate.yml itself.
- Unit 3 (items vs findings + verdicts) DONE for: G0 (IE-4, D8, D17: holds; IE-4's 2nd mutation "install step removed
  under RUSTUP_AUTO_INSTALL=0" kept by D8 fix_ok 3 but listed only as a fact, not a mutation, in report :227 and archive
  :607-613: Low), REQUIRED-CHECK (IE-1, SA-4, D2: holds, incl. :294, guard :6/:990 as "comments"), SETUP-NODE-CACHE
  (SA-3, D6: holds), CUT-CARRY (IE-3, D7, D17: holds), CUT-ESCAPE (SA-1, D4: holds; no git version, right: local runs
  were git 2.43.0, SA notes :41, D4 notes :30; work-folder route is "reasoning only" in D4 fix_ok (1), unmarked: Low),
  SETPRIV-HARDEN (SA-2, D5: holds), CANCEL-ON-MAIN (IE-5, D9: holds), LINT-TABLE-PEDANTIC (RR-2, D10: holds),
  FEATURE-UNIFY (RR-3, D11: holds), ROOT-WORKSPACE (EA-1, D12: holds, §4 per D12), EMPTY-SLOTS (EA-3, D14: the archive
  uses D14's suggested text; "a foundation step" is D14's reading, plan.md:269 says "the step that adds wasm-bindgen":
  Low), OQ-0002-8-TEST (EA-4, D15; ADR :1099-1102 Advisor's answer; :222-229 reasons: holds), CLIENT-NET-RULE (EA-5,
  D16: holds but archive :766 names only "PM-0002-22's counter" for both page rules; the Wasm rule is PM-0002-2's
  counter (ADR :1265; D16 notes :36; PM-0002-64 keeps both): Low). All 17 verdicts hold at reviewer severity.
