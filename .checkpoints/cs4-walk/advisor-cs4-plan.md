# Advisor: plan review of change-set 4 "Walk" (plan.md draft 1)

## My plan (first lines, so a cut-off run can continue)
1. Read plan.md whole; then his words (takeover, order-3c-answer, export-block, question), then the three designs' size tables.
2. Check the four ORDER-3C (c) conditions against the plan text and his words.
3. Recompute the sizes: plan sums vs the designs' own tables; note each number that does not follow.
4. Point 7 (MUL-ADD built-file check, first picture after a pause, 1.8x with stop line).
5. Point 8 a-g (Testlines): Export saved by script, compare by script, --check, not-taken list, 60-steps warning, one home.
6. The ten questions: whose each is; rule on mine; check the reporting rule; missing decisions.
7. Section 6.2 choices and DEPART-4.
8. Lanes and files vs Part 15; no kit/gate/CI file; CI Builder requests are requests only.
9. Evidence, reviewers, risk tier, browser checklists, checkpoint/agent rules, honesty about the unverified.
10. Hornet entry vs docs/UNITS.md section 0.
11. Verdict with numbered conditions (severity + file:line).

## Findings (a line per finding as found)
- F1 (High): plan s0 line 40-41 claims every recording mark is met on the movement recording before 4c merges; but s9 4d holds PM-0004-28 (a recorded-match mark, ADR 0004:1149, proving 4c's camera never reaches the sim) and PM-0004-25 (ADR 0004:1146, PM-0004-19's suite with simd on), and 4c's ladder (plan:220-221) moves PM-0004-17's interleaved run on movement to 4d. Contradicts its own rule 'met in the part that merges the code it proves' (plan:296-297) and his ORDER-3C (c) cond. a / his 2026-11-14 rule.
- F2 (High?): his queued Slice 1 note (DECISIONS.md:81-82) says the camera panel ships 'from its first playable build'; build plan card :334 'the first playable build, with your camera panel'. Plan s6.3/6 (plan:224-227) delivers the early-look zip (a playable build) BEFORE the panel, reading 'first playable build' as 'first judged/merged'. A reading of his words not put to him as a decision.
- F3 (Medium): DEPART-3 (a), accepted by him (cs3-plan at 80a51e42 :312-313), reads 'three limits are code, not data, until the content crate exists in change-set 4 (ADR 0001 line 221)'. The content crate enters in 4b, and the plan says nothing of the three limits; engine-architect-cs4.md:938-941 (N60) proposes they stay code until cs7 'a departure for the list'. DEPART-4 (plan:444-474) omits it.
