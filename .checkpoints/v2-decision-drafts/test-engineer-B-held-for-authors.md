# test-engineer B's final batch: two blocks held for the drafts' authors (the session, 2026-11-02)

These two blocks add open questions to the drafts' "## Open questions" sections, which are the drafts' authors' text (graphics-engineer for ADR 0003, infra-engineer for ADR 0006). The session holds them and asks each author to adopt or adapt them, keeping the numbers OQ-0003-20 and OQ-0006-20, which unit B's pass-mark rows cite.

### FIX PA-7 (new OQ-0003-20, HEAT-SOAK: the Advisor)
FILE: docs/adr/0003-rendering.md
BEFORE:
  fight view through a colour-blindness simulation, and Mustafa judges it; (c) faction shape cues, left to a later
  design session. Decider: Mustafa. What decides it: his judgement in the look proof test. Research ids: none.
AFTER:
  fight view through a colour-blindness simulation, and Mustafa judges it; (c) faction shape cues, left to a later
  design session. Decider: Mustafa. What decides it: his judgement in the look proof test. Research ids: none.
- **OQ-0003-20. Is the minimum laptop's budget judged after 15 minutes of play, or on a cold laptop?**
  Why it matters: a match runs about 20 minutes (LOOP_V2 §8), while one H run takes about 30 s at 30 fps (900 frames;
  about 110 s in the tail form); in a thin laptop the CPU and the integrated GPU share one power and heat budget, so a
  cold run can measure a short boost the same laptop no longer holds at minute 15 and pass a row it then fails
  (engineering experience, not researched: the chosen laptop's boost behaviour is unknown). Options: (a) the soaked
  set S gates PM-0003-18 to PM-0003-21 on ML, and the cold set is reported beside it; (b) the cold set only, R3's 3
  runs as written; (c) both must pass. The pass marks hold to (a) until it is answered. Recommendation: (a), the
  perf review's question HEAT-SOAK. Decider: the Advisor. What decides it: the Advisor's reading of VISION_V2 §4's
  "30+ fps on Low" over a whole match; the bench's cold and soaked sets, once measured, show how far apart they lie.
  Research ids: R3-BUDGET-REC2 (lines 1162–1164, its 3 runs; it stays adopted, no new disposition).
END

### FIX IE-13 (new OQ-0006-20: the spike's bill, Mustafa)
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
    and logging rows about 3 h), 23 × 0.1386 = 3.19 EUR, before ADR 0001's 3-run rows (perhaps 15 to 30 RS-hours,
    about 2 to 4 EUR) and the CX33 hours of SH, LG and MV; so a 5 EUR cap over everything would send the plan back to
    the draft at its planning check.
  - *Options:* (a) 5 EUR for this record's own operations runs (R6's scope), with ADR 0001's runs on their own line
    of the run plan; (b) one cap set from the run plan, written as arithmetic at R6-HOST-F15's prices before the first
    session; (c) 5 EUR over everything, as written. Until it is answered, PM-0006-18's planning check writes the run
    plan's cost and the plan goes to Mustafa before any VM is created. The test review recommends (b): both records'
    runs bill to one Hetzner project.
  - *Decider:* Mustafa (money).
  - *What decides it:* the run plan's hours (PM-0006-14 writes it before the first session) at the hourly prices.
  - *Carries:* R6-COST-REC1's mark (r6 lines 716–717; it stays adopted, no new disposition); R6-HOST-F15 (r6 lines
    150–159).
END
