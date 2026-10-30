# Step 3c, R2: netcode-architect's report (background agent launched 18:39 UTC, cut off by the spend limit about 18:52, resumed 20:11 UTC, returned about 20:19 UTC) (verbatim)

I finished R2 step 3c. Every approved change for R2 is applied, except one glossary rename that I refused (the OBJECT below). The check script now finds 1 exact retired phrase in R2: that same refused place. Nothing was committed.

**Changes by type:** 20 recommendation, 94 nit, 0 number, 0 source.

**Applied, by resolution rank:**
- 1: C1.7–C1.14, including C1.10b
- 5: C5.8–C5.10
- 6: C6.5–C6.7
- 9: C9.9–C9.11
- 10: C10.4–C10.11
- 11: C11.8–C11.12
- 17: C17.3, C17.4
- 23: C23.1, C23.2

C17.b2 and C23.b* were not applied; they wait for BASELINE-WHEN.

**OBJECT (not applied):** R2-INT-F20's "4–9 % of one core" (r2:870). It is a legacy JavaScript measurement made in a browser on unrecorded hardware, so "one vCPU" would misstate the finding. The glossary keeps "core" for client hardware. I propose adding this place to the measuring-unit entry's Keeps.

**Glossary:** all 25 terms R2 uses are linked at their first use. Retired wordings were replaced, and four time marks now say p99 (BUD-REC6, INT-REC1, INT-REC2, INT-REC4).
- C1.9, C11.9, C11.10 and C17.3 carry a later rename, as the order of work says.
- C1.10, C1.10b, C1.13 and C23.1 differ from the approved text only by a link or "at p99".

**Questions added:** ABL-PREDICT, SEC-SPREAD, MODES-3 and BASELINE-WHEN, plus R2-XT-OQ1–OQ15, under your two subsection names. Each has pointers from the recommendations and R2 questions it names.

**Nit for you:** C23.2's approved text cites "r6:424-425" without "at commit 8dee746". I applied it as written.

Files:
- /home/user/Shamble-Frontlines/docs/research/r2-netcode.md
- /home/user/Shamble-Frontlines/.checkpoints/v2-research/netcode-architect-r2-consistency.md (ends with DONE)
