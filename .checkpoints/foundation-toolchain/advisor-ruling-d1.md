# Advisor ruling D1 (RR-1: overflow checks do not reach cross-crate sim code) - notes

Seat: advisor agent, day session, run as a general agent reading main's .claude/agents/advisor.md (2aa5e9c).
Worktree: .gate/foundation-toolchain-wt, branch claude/laughing-pasteur-fwya24.
Head named in the brief: 4d78ad61e652f2e35ddf51c49557e725420248c8. Worktree HEAD at start: 04814cf
(4d78ad6 is its ancestor; 4d78ad6..04814cf touches only .checkpoints/ files, so the reviewed content is the same).
Scratch copies only under /home/user/Shamble-Frontlines/.gate/advisor-d1/.

## Plan
1. Read: role file (done), root Cargo.toml, ADR 0002 part 4 (:247-278) and PM-0002-53 (:1319), the plan (Part 2,
   Part 7, As built), the report draft, RR-1 in review-r1-findings.json, skeptic-r1-D1.md (C1-C15), D1 in
   review-r1-verdicts.json, the reproduction script.
2. Check the mechanism myself in a scratch copy: (a) the per-package override misses cross-crate inline/generic code
   (re-run the reviewer's script or an equivalent); (b) route A (release overflow-checks = true + "*" = false) covers
   workspace members and leaves dependencies off; (c) whether "*" also matches workspace members (Cargo docs:
   "*" = all non-workspace members? verify against cargo's own behaviour); (d) bench/custom profile inheritance;
   (e) generic sim code instantiated in a dependency crate (e.g. a third-party generic calling into sim) - edge case.
3. Weigh routes A/B/C against ADR part 4's words, the plan's pinned mechanism, the review budget and the escalation
   categories (ADR = decision doc; rule change?).
4. Answer questions 1-6, verdict, conditions.

## Checks (one line each, as done)
