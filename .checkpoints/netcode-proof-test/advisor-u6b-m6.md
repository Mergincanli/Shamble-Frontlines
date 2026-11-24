# Advisor: U6b's merge, step M6 (a failing test at M5), 2026-11-24

Instructions: main's `.claude/agents/advisor.md` (`git show origin/main:.claude/agents/advisor.md`), read at the start.
My earlier ruling: advisor-u6b-fix.md (APPROVE WITH CONDITIONS C1-C2, for the head whose spikes/ and docs/ diff
against 21765e3 equals wip/u6b-fix.patch e29c003c…ac64af1c). Risk tier 1 (spikes/). Read only; no cargo run by me.

## 1. Plan
1. Inputs: the patch's hash; the applied tree's binary.rs (main folder) against the builder's (byte-equal claim);
   m5-check.log's header and steps; failed-steps/test.log at the failure (:2219-2249) and the run's timing lines;
   fx-check1/2.log's test lines; u6a-r2-test.md :60-115 (TR2-U6A-4); u6a-report.md :66; u6b-report.md (its M5 line,
   its carries); u6b-review-test.md N7; my own T-4/F-2; m6-probe.log.
2. Item 1 (cause): what the bitten start's time is made of (the two 1 s sleeps, then U6b's start: the certificate and
   both listeners), the earlier readings of it (U6a's 2,660 ms; U6b's builder runs if printed), what the failed run
   shows (the cap hit, not a wrong OPEN), and what it does not (the start's actual time on this run).
3. Item 2 (fix): the lines in binary.rs that may change; the other caps near their margins (T-4/F-2's 5 s, N7, any
   other 5 s cap in the bites build); in this edit or carried.
4. Item 3 (evidence before merge): the probe's form, when, and a plant for the new cap.
5. Item 4 (M5 again): the disk rule from M5's own numbers.
6. Item 5 (my approval's terms): the head, what I read before M7-M9.
7. Item 6 (the carry gap): where TR2-U6A-4 is recorded and closed; the report's text.
8. Item 7 (Mustafa): the scope line and the stop, a test-margin fix in the plan or not.
9. Findings, verdict, conditions, Decisions needed, For Mustafa, DONE.

## 2. Items
