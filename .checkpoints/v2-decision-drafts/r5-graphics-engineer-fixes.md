### FIX e2-F1-part8
FILE: docs/adr/0003-rendering.md
BEFORE:
     after at least 15 minutes of play (the pass marks' soaked set S); the soaked set gates and the cold set is
     reported beside it (OQ-0003-20, answered by the Advisor, 2026-11-03: "judge after 15 minutes of play").
AFTER:
     after at least 15 minutes of play (the pass marks' soaked set S); the soaked set gates and the cold set is
     reported beside it (OQ-0003-20, answered by the Advisor, 2026-11-03: "judge after 15 minutes of play"), its
     option (a) (the advisor's reading, 2026-11-03).
END

### FIX c3-oq1-entry
FILE: docs/adr/0003-rendering.md
BEFORE:
  Answered in part by Mustafa (2026-11-03): "Test PC: my own, in Brave" (SPIKE-MACHINES). That PC is MP, which the
  pass marks' key defines as what he judges and never a budget result, so it stands in for none of ML, RD or MAC
  (the advisor's reading, 2026-11-03); still open: which laptop (ML) and which desktop (RD), and which Mac (MAC) for
  the rows that name Safari, decided by Mustafa (escalated to him, since the netcode proof test's marks on ML or RD
  cannot run without them: the advisor's reading, 2026-11-03).
AFTER:
  Answered in part by Mustafa (2026-11-03): "minimum: a typical student laptop with integrated graphics;
  recommended: a desktop with a mid-range card (RTX 3060/4060 class); Mac: a MacBook Air, M1 or newer, for Safari.
  All borrowed." (REF-MACHINES). Still open: the actual machines, named by Mustafa before the look proof test. The
  stand-in for ML (REF-MACHINES, Mustafa, 2026-11-03): "Until then, my laptop's integrated graphics (Brave set to
  "Power saving") stands in for the minimum, for rough checks only." Mustafa's "Test PC: my own, in Brave"
  (SPIKE-MACHINES, 2026-11-03, at the stop) is MP. A rough check is reported, never judged against a pass mark.
  MP stays what he judges, never a measured result, and nothing stands in for RD or MAC
  (the advisor's reading, 2026-11-03).
END

### FIX c3-migration-step1
FILE: docs/adr/0003-rendering.md
BEFORE:
1. **Drafts signed off as the basis for the spikes** by Mustafa (2026-11-03, SPIKE-ORDER (b); this record stays
   Proposed, not yet accepted); the two reference machines named (OQ-0003-1, answered in part: still open, Mustafa's).
AFTER:
1. **Drafts signed off as the basis for the spikes** by Mustafa (2026-11-03, SPIKE-ORDER (b); this record stays
   Proposed, not yet accepted); the two reference machines named.
   OQ-0003-1 is answered in part by Mustafa (2026-11-03): "minimum: a typical student laptop with integrated
   graphics; recommended: a desktop with a mid-range card (RTX 3060/4060 class); Mac: a MacBook Air, M1 or newer, for
   Safari. All borrowed." (REF-MACHINES). Still open: the actual machines, named by Mustafa before the look proof
   test. The stand-in for ML (REF-MACHINES, Mustafa, 2026-11-03): "Until then, my laptop's integrated graphics (Brave
   set to "Power saving") stands in for the minimum, for rough checks only." A rough check is reported, never judged
   against a pass mark. MP stays what he judges, never a measured result, and nothing stands in for RD or MAC
   (the advisor's reading, 2026-11-03).
END

### NOTE n1-look-test-needs
FILE: docs/adr/0003-rendering.md
no change: the look proof test's "Needs" (0003:1018, "both reference machines (OQ-0003-1)") stays true under
condition 3: Mustafa names "the actual machines before the look proof test", and the OQ-0003-1 entry and migration
step 1 now carry his classes, the stand-in for ML and the advisor's reading. Left for the drafts' next revision,
outside this round's items: that "Needs" line names no Mac, though the look-test rows PM-0003-1, -2, -3, -5, -6 and
-16 (test-engineer's) name MAC for Safari; his answer lists "Mac: a MacBook Air, M1 or newer, for Safari" among the
machines he names before the look proof test, so nothing in the line is false now.
END

### NOTE n2-other-pointers
FILE: docs/adr/0003-rendering.md
no change: the other places in my text that cite OQ-0003-1 (part 7, 0003:325, "the models are OQ-0003-1"; the M-A
and M-B lines, 0003:671 and 0003:739; the dispositions R1-TICK-OQ6, R1-BND-OQ3, R3-BUDGET-REC1 and R3-BUDGET-OQ1,
0003:1441, 1442, 1460 and 1491, "open: OQ-0003-1") carry no label and no "stands in for none" sentence, and stay true
for a question answered in part, which stays open. Part 7's class text (0003:325-331) is not rewritten (condition
3); its "Mustafa's RTX 5080 is a check above the recommended class and the Brave check, never the recommended
reference" agrees with "not Mustafa's RTX 5080 laptop" (DESIGN-ANSWERS). The keys (test-engineer) and README and
SPIKE_PLAN (engine-architect b) are theirs.
END

