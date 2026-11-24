# test-engineer, part a (MOVED), one review round on b22c30cd
2026-11-24. Reviewer notes; text only. Question: is it proven?

## Plan
1. Run: cargo test -p npt-judge --locked --no-fail-fast; clippy -D warnings; fmt --check; --check-marks. Paste lines.
2. marks.json diff: script base vs head (strip `moved`, undo -7/-56 edits, compare); count labels; check `to`s against
   moved-block.md and moved-table.mjs; MARK-NOTES sentences word for word; rough twin wording (cond. 8).
3. Conditions 1-9 against code and named tests.
4. Hunt unproven behaviour; plant defects in a scratch copy under .gate/ (never in worktree source).
5. Contract text (marks/README.md 2.1, 2.2, 2.3, 2.6, 3, 5, 6; fixtures/README.md) vs code.
6. The three changed existing assertions: forced? anything stale?

## Progress
- (lines appended per block / finding)
