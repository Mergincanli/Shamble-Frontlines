# infra-engineer, follow-up batch c (ADR 0006): OQ-0006-20 and spike item 2's bill cap (the session, about 20:42 Amsterdam)

Every BEFORE was copied by script (whole lines, by line range) from docs/adr/0006-infrastructure-operations.md as it
is on disk now (the working tree with test-engineer's uncommitted pass-mark batch; my text untouched) and occurs
exactly once there; the blocks share no line and none touches "### Pass marks" (lines 1075-1251 now).
OQ-0006-20 is test-engineer's held block, adopted with its number and three changes: R6-COST-REC1 is "changed" in
Appendix A, not "adopted", so its Carries line says the disposition stays "changed"; the sum behind "about 23 hours"
is shown; the load generators are counted as spike 2 now sizes them. Its arithmetic checks: 14.3 + 3.3 + 2.2 + 3 =
22.8, about 23; 23 × 0.1386 = 3.19 EUR.

### FIX IE-13 (OQ-0006-20, test-engineer's held block, adapted) (Open questions: OQ-0006-20, the netcode proof test's bill cap; lines 1001-1002 on disk now)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  - *Carries:* no research id (the test review's finding 2 and its question 2; R6-COST-OQ1 stays with the netcode
    proof test).
AFTER:
  - *Carries:* no research id (the test review's finding 2 and its question 2; R6-COST-OQ1 stays with the netcode
    proof test).
- **OQ-0006-20. What may the netcode proof test's VMs cost?**
  - *Why it matters:* R6 sized "the spike's own bill (a few hours of one or two VMs) under 5 EUR" (r6 lines 716–717),
    but the pass marks now plan far more: on RS alone about 23 hours for this record's rows (the infra review's
    arithmetic on the rows' run plans: PM-0006-14 about 14.3 h, PM-0006-13 3 × 1.1 h, PM-0006-17 about 2.2 h, the soak
    and logging rows about 3 h; 14.3 + 3.3 + 2.2 + 3 ≈ 23), 23 × 0.1386 = 3.19 EUR, before ADR 0001's 3-run rows
    (perhaps 15 to 30 RS-hours, about 2 to 4 EUR) and the CX33 hours of SH, MV and the load generators (as many as
    spike 2's calibration run asks); so a 5 EUR cap over everything would send the plan back to the draft at its
    planning check.
  - *Options:* (a) 5 EUR for this record's own operations runs (R6's scope), with ADR 0001's runs on their own line
    of the run plan; (b) one cap set from the run plan, written as arithmetic at R6-HOST-F15's hourly prices (0.1386
    EUR a CCX23-hour, 0.0144 a CX33-hour) before the first session; (c) 5 EUR over everything, as written before.
    Until it is answered, PM-0006-18's planning check writes the run plan's cost, and the plan goes to Mustafa before
    any VM is created. The test review recommends (b): both records' runs bill to one Hetzner project (OQ-0006-17).
  - *Decider:* Mustafa (money).
  - *What decides it:* the run plan's hours (PM-0006-14 writes it before the first session) at the hourly prices.
  - *Carries:* no research id of its own: R6-COST-REC1's bill mark (r6 lines 716–717; its disposition stays
    "changed", Appendix A) and R6-HOST-F15's prices (r6 lines 150–159).
END

### FIX IE-13 (Spike 2: the bill under the cap OQ-0006-20 sets; lines 1056-1056 on disk now)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  spike's own hosting bill under 5 EUR (R6-COST-REC1's mark);
AFTER:
  spike's own hosting bill under the cap OQ-0006-20 sets (R6-COST-REC1's mark gives 5 EUR for "a few hours of one or
  two VMs", r6 lines 716–717);
END

### FIX IE-13 (Appendix A, R6-COST-REC1: the bill mark's cap; lines 1391-1391 on disk now)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
| R6-COST-REC1 | changed: the goal budget stated at both densities, with the bound the marks allow | part 8: about 361 or 238 EUR at 8 rooms per VM, about 534 EUR (fixed fleet) at the 4 rooms per VM that A-2 names the case to budget, about 967 EUR at the 2 rooms per VM the marks' ceilings allow, until the netcode proof test measures it, and that figure provisional until slice 6 (OQ-0006-19); the start budget unchanged; a tick bench line added |
AFTER:
| R6-COST-REC1 | changed: the goal budget stated at both densities, with the bound the marks allow | part 8: about 361 or 238 EUR at 8 rooms per VM, about 534 EUR (fixed fleet) at the 4 rooms per VM that A-2 names the case to budget, about 967 EUR at the 2 rooms per VM the marks' ceilings allow, until the netcode proof test measures it, and that figure provisional until slice 6 (OQ-0006-19); the start budget unchanged; a tick bench line added; its 5 EUR bill mark, sized for "a few hours of one or two VMs", becomes the cap OQ-0006-20 sets (spike 2; PM-0006-18) |
END
