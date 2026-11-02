# The final fix round: findings to batch per author (the session's tracking file, 2026-11-02)

Collected from fixcheck-a, -b, -c, the skeptics and the step-10 reviews; one round per author at the end.

## From verifier fixcheck-b (verifier-fixcheck-b-verdicts.md; PASS WITH NOTES, 2026-11-02 about 09:25)
- test-engineer: finding 1 (PM-0002-17, -42: the `enum` bite required again; skeptic-4 first), finding 3 (PM-0002-45
  cannot fail G7 in the foundation; skeptic-4 first), finding 9 (PM-0002-23: "(2 × 1000 / 30 = 66.67 ms)"), finding 12
  (PM-0005-42 "faction definition" vs CD-13's client definition). Findings 2, 4, 6: fixed in round 3 (81d2a65); fixcheck-c
  checks them.
- engine-architect (0005 frame): finding 7 (0005:1759 "Revisit when" lost its verb), finding 11 (0005:5 author line
  reads as if graphics-engineer wrote the pass marks), finding 13 (0005:1693 "until the spikes have measured it" vs
  ADR 0002's "until the Advisor sets the first limit").
- engine-architect (0002): finding 8 (OQ-0002-8's ids line: add R5-XT-OQ6; "no answer on one side" fits R4-XT-OQ5 only).
- rust-reviewer: finding 10 (0004:339–340 "no Rust navigation crate read promises determinism"); finding 5 (0004:805–807,
  "listed once in README": true once the synthesis adds the fragments row; skeptic-4 first).
- The synthesis (sent 09:30, queued): README rows 69–70, 72, 82, 109; OQ-0002-8 row; fragments row; SPIKE_PLAN:30, spike 4
  table (PM-0004-73, -74), :176.
- For the report: R5-TSLINT-F12's construct list (frozen in r5 as "general knowledge, unverified") is now confirmed at the
  source (typescriptlang.org/tsconfig, "Erasable Syntax Only"); the six pending source checks are confirmed; of the five
  claims on unread sources, four confirmed and 0004 row 138 supported (Unsafe Code Guidelines glossary).
