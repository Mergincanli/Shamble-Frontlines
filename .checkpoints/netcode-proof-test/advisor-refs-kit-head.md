# advisor: REFS-KIT head check (914798a)

Seat: advisor checking a pushed head (not a review round). Rules: origin/main:.claude/agents/advisor.md (read).

## Plan
1. Read the ruling (advisor-refs-clash-full.json): fix.check_mjs (3 replacements), fix.check_test_mjs (1), plan.md lines (2).
2. git show --stat 914798a; compare the spikes/ diff to the ruling's replacements word for word.
3. git show 63cc7c3 -- plan.md: the ruling's two lines word for word.
4. Evidence under .gate/refs-kit/: refs-self.log, fixed.tap, P1..P3.tap, plant copies, gate-no-smoke.log, counted.log (diff hash vs git diff 63cc7c3 914798a -- spikes/).
5. Anything that must change before the merge of main.
6. Verdict.

## Notes
