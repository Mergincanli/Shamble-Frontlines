# Step 3c, R5: test-engineer's report (background agent launched 18:39 UTC, returned about 18:48 UTC) (verbatim)

Step 3c for R5 is done. All approved items are applied to `/home/user/Shamble-Frontlines/docs/research/r5-toolchain-quality.md` and to no other file. There is no OBJECT.

**Changes by type:** 4 `recommendation` (C19.1, C21.1, C22.1, C22.3), 0 `number`, 0 `source`, 64 `nit`. The step-3d verifier should check the four recommendation changes.

**Resolutions applied:** every "before" text matched exactly once.
- rank 1: C1.19, C1.20
- rank 10: C10.17, C10.18
- rank 17: C17.5, C17.6
- rank 19: C19.1, C19.2, C19.3 (in the revised words), C19.4
- rank 21: C21.1, C21.2 (in the revised words), C21.3
- rank 22: C22.1–C22.4
- rank 29: C29.4
- rank 41: applied as a nit through C22.1
- Ranks 5, 6, 9, 11 and 23 change nothing in R5.

**Not applied:** C17.b3, which waits for the Advisor's BASELINE-WHEN answer.

**Glossary:** R5 had no "Retire (exact)" hits. I made 9 in-context renames: sim step time, server tick time, the preview header set, Cloudflare Pages, the security baseline (three places) and the staging game server. Each term now links to its entry from its first real use. The header links GLOSSARY.md, has the new line, and its open-question total goes from 45 to 63. Part 7 has the new "Consistency resolutions (2026-10-30)" subsection.

**Shared open questions added:**
- MODES-3, BASELINE-WHEN, PREV-DELETE and PREVIEW-WALL, under "Shared open questions (the same words in every topic)".
- R5-XT-OQ1 to R5-XT-OQ14, under "Cross-topic open questions (consistency critic, 2026-10-29)", per your correction. Ten recommendations now point to them.

**Check script after edits:** R5 has 0 exact hits and 45 in-context hits. The new in-context hits sit inside approved word-for-word texts or were kept on judgement. Dated-record skips rose from 5 to 19 because the new part 7 subsection sits in the skipped verifier part.

Notes, ending in DONE: `/home/user/Shamble-Frontlines/.checkpoints/v2-research/test-engineer-r5-consistency.md`. Nothing is committed.

## Addendum: the step-3d fix round (reply to `prompts/r5-3d-fixes.txt`, about 20:44 UTC, verbatim)

I fixed all four step-3d items in `/home/user/Shamble-Frontlines/docs/research/r5-toolchain-quality.md`, and added one line per change under "Step 3d fixes (2026-10-30):" in part 7. Nothing is committed.

1. r5:401–402: R1's old words were not a quote, so they stay. I added "; since 2026-10-30 the mark is the hash-map ban" with the link (nit).
2. r5:1477: the list now opens with "items 1 and 4 of the test-mode list:", linked. The modes the soak runs are unchanged (nit).
3. r5:1561 now reads "the check of R1-ECS-REC2 mark 2, the hash-map ban", and r5:2087 reads "(the hash-map ban in the sim crate)", both linked (nit each).
4. Bare 48s:
   - r5:58: "48-bot soaks" becomes "bot soaks up to the largest mode's N" (nit, since REC5's value already changed through C10.17).
   - r5:1429 and r5:1487: each match now runs at the largest mode's N, "48, or 54 under MODES-3 (b)" (recommendation each).
   - r5:1426 stays: "the 48-player scale" names VISION_V2 §3's design point, not the largest mode.

The check script still finds 0 exact hits in R5; the in-context count went from 45 to 48. My guess is the three extra hits are the new "(48, or 54 under MODES-3 (b))" values, which sit beside the term. Notes: `/home/user/Shamble-Frontlines/.checkpoints/v2-research/test-engineer-r5-consistency.md`.
