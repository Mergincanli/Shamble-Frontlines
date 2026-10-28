# R1: the owner's second fix round, the agent's report verbatim

Workflow run wf_0ad9f0c0-59d (script `v2-research-fix2-run1.js`, prompt `prompts/engine-architect-r1-fix2.txt`), returned 2026-10-28 (about 20:24 UTC). Notes: `engine-architect-r1-fix2.md`.

I applied every finding from the verifier's second pass to `docs/research/r1-simulation-core.md`. No recommendation now rests on a claim found wrong or unsupported, and I fetched no new pages.

**Changes by type:** 7 recommendation, 2 source, 2 number, 8 nit.

**What changed:**
- **Pcg32 fallback in R1-DET-REC4 (wrong claim R1-DET-F34b):** F34 now quotes the source's warning: "Note that two generators with different stream parameters may be closely correlated." The fallback no longer counts the stream number as a strength, and options Q4 matches.
- **File:// clause in R1-DATA-REC6 (wrong):** cut. The plain-HTTP LAN part stays, since it rests on R1-DATA-F29. R1-DATA-OQ11's matching sentence is corrected.
- **R1-ECS-F34 (unsupported, a claim of absence):** it is labelled as an absence that no quote can confirm, and R1-ECS-REC1 no longer lists it under "Rests on".
- **Minor points:** fixed in R1-ECS-REC1, R1-DET-REC5 and REC6, R1-TICK-REC2's "Serves" line and R1-BND-REC5, plus the draft dates.
- **Part 7:** 8 rows added, and a new subsection "Second verifier pass (2026-10-28)" with the verifier's table and counts.
- **Beyond what the verifier named:** the same "CORP or CORS" wording in options Q6 (G2) and R1-BND-OQ5 is now marked as our reading. The TL;DR was 6 lines, so I shortened it to 5 with the same content.

**Recommendation that became an open question:** the Pcg32 fallback's one-stream-per-purpose scheme in R1-DET-REC4 is now R1-DET-OQ11. That brings the open-question total from 76 to 77.

**Re-sourced claims:** none.

**Verifier findings not applied:** none. The verifier suggested the header label "verified (every recommendation claim; second pass 2026-10-28)". I used the label your rule 4 prescribes and named the second pass in the sentence after it.

**Final header status:** verified (every recommendation claim)

Notes and change log: `/home/user/Shamble-Frontlines/.checkpoints/v2-research/engine-architect-r1-fix2.md` (ends with DONE). Nothing committed or pushed.
