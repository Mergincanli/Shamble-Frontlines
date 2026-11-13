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
