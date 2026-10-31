# Step 3c, R6: infra-engineer's report (background agent launched 18:39 UTC, returned about 18:49 UTC) (verbatim)

I've applied every approved R6 item, the glossary terms and the shared open questions to /home/user/Shamble-Frontlines/docs/research/r6-infrastructure.md. There are no OBJECTs. Nothing was fetched and nothing was committed.

**Changes by type:** 6 recommendation, 0 number, 0 source, 51 nit.

**Applied (rank, change id):**
- Rank 1: C1.21 (recommendation), C1.22.
- Rank 10: C10.19–C10.23 (MODES-3 inserted).
- Rank 11: no text change; the glossary is linked at R6-HOST-REC2.
- Rank 17: C17.7–C17.9 (BASELINE-WHEN inserted).
- Rank 19: C19.5 and C19.6 (recommendation), C19.7, C19.8, C19.9 (PREV-DELETE inserted).
- Rank 21: C21.4 (recommendation), C21.5, C21.6, C21.7 (PREVIEW-WALL inserted), C21.8 (recommendation).
- Rank 22: C22.5 (recommendation), C22.6.
- Rank 23: C23.3.

**Not applied:** C23.b1–b3, which wait for BASELINE-WHEN. When (b) is applied, C23.b2's quote "…rejects a shared key." needs adjusting: approved C23.3 replaced its final period.

**Glossary terms:** server tick budget, server tick time, deferred security baseline and client host, plus first-use links for 21 terms. The check script finds 0 exact hits in R6; I judged all 88 in-context hits as kept.

**Questions added:** the four shared ones under "Shared open questions (the same words in every topic)". R6-XT-OQ1–OQ9 are under "Cross-topic open questions (consistency critic, 2026-10-29)", per your correction. Pointers are in REC2, REC3, COST-REC1, OBS-REC3 and TEL-REC1. Header shows 36 open questions; part 7 has the subsection.

**Two notes:**
- The script reports "header links GLOSSARY.md: False" for R6. It reads only above the first `## `, and R6's header is its own "## 1. Header" section, where the link is.
- R6-HOST-REC1's "Rests on" doesn't list the R5 findings that C19.6 now cites.

Notes: /home/user/Shamble-Frontlines/.checkpoints/v2-research/infra-engineer-r6-consistency.md

## Addendum: the step-3d fix round (reply to `prompts/r6-3d-fixes.txt`, about 20:43 UTC, verbatim)

All six step-3d items are fixed in r6-infrastructure.md only (lines as at f67245e). The check script still finds 0 exact retired phrases in R6, and nothing is committed.
1. r6:46 (TL;DR): "free static traffic" now applies to Workers static assets only (R6-HOST-F5), as in C19.6. nit
2. r6:709: the room sizes give both MODES-3 readings ((a) 48, 36, 18; (b) 54, 48, 36). recommendation. Next to it, r6:710's egress mark now says "per 24v24 (48-player) match-hour". nit
3. r6:799: "the largest mode: 48 players under MODES-3 (a), 54 under (b)". recommendation
4. r6:665: "8 rooms of the largest mode (48 under (a), 54 under (b); bots)", with a glossary link. recommendation
5. Added a pointer from R6-OBS-REC4 to R6-XT-OQ5, and R5-PREV-F12, F13, F14 and F18 to R6-HOST-REC1's "Rests on:" list. nit
6. r6:48, r6:417, r6:508 and r6:602 now read "24v24 (48-player)", since their figures come from R2's 24v24 row. nit. r6:744's soak now runs at the largest mode's N (48 or 54). recommendation. r6:212 and r6:256 stay as they were: they repeat the 48 soldiers of the legacy perfbots figure (TL15), stated right beside it.

Part 7 has a "Step 3d fixes (2026-10-30):" line with one entry per change. Notes: /home/user/Shamble-Frontlines/.checkpoints/v2-research/infra-engineer-r6-consistency.md

## Addendum: the final check's open item and nit (reply to `prompts/r6-3d-final-fix.txt`, about 20:53 UTC, verbatim)

Both items are fixed in r6-infrastructure.md (not committed). The record mark is logged as a recommendation change. Each part-7 step-3d line now gives its type. The mark now reads: "a 24v24 record is at most 1 MB uncompressed (A-10), and a record of the largest mode at most 1 MB × N / 48 (arithmetic: 1.125 MB at N = 54 under MODES-3 (b))."

## Addendum: the completeness critic's nits (reply to `prompts/critic-nits-r6.txt`, about 10:55 UTC on 2026-10-31, verbatim)

All four nits are in r6-infrastructure.md: N at r6:807 and 24v24 at r6:417 now link to the glossary. A-2 and A-3 call their 48 a 24v24 room, and A-2 says R2 now states its mark for the largest mode's N. Each has a part-7 line, logged as nits. Not committed.
