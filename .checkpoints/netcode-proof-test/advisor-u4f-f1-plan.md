# Advisor plan review: U4f-F1 (u4f-f1-plan.md rev 1), 2026-11-19, from main's advisor.md

Read only. Sources: `.checkpoints/netcode-proof-test/u4f-f1-plan.md` (rev 1, 55 lines), my -20 ruling
(`advisor-u4f-20.md`: R20-Q1 :114-140, ADV-U4F20-1 :162), sim/src/world.rs, room/src/room.rs, room/src/input.rs,
ADR 0001 part 5, the tests and pins that read refusal counts, SERIES.md, history.md:2234-2282 (TEST-FIRST).

## Plan
1. R20-Q1 against the plan's fix shape: world.rs `run_command` and every `edge` use; room.rs `run_one`; input.rs:285-300;
   ADR 0001:225-237.
2. 1-edge path unchanged; what T2 must show; which tests or pins read the sim's refusal counts.
3. Lane, size, reviewers, builder setup, the re-run and its expected verdicts.
4. Rule U4FF1-Q1 to Q3.
5. Anything for Mustafa (pass mark, judge, marks.json, money, main, the plan's stops).
6. Verdict, findings, rulings, conditions, the line to Mustafa.
