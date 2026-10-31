# Step 3c, last part: engine-architect's report on the GLOSSARY.md "Used today" refresh and OBJECT-1's change (sent 20:23 UTC, returned about 20:37 UTC) (verbatim)

All 34 "Used today" lists in GLOSSARY.md are refreshed. Only those lists, item 3 and the new status line changed (a masked diff confirms it). Nothing is committed.

**Item 3 done.** The three fields now read:
- **Retire (exact):** "per core", "server core", "1 CPU core": every use today is for server work (the old ids stay in brackets).
- **Retire (in context):** "one core", for a server time mark or a share of the reference server.
- **Keeps:** "… and where a source is quoted; and a share of the machine a legacy measurement ran on (R2-INT-F20)."

The search script now finds 0 exact hits in all six topics.

**Ids:** 483 listed (an id in two entries counts twice), 175 added, 104 dropped.

**Rows left out as not uses:** 138 of 945. Entries with more than 5: start and goal modes 82, Preview 30 (Vite's preview server, `preview.js`, the grenade preview), Time statistics 7 ("mean" as a verb), Player count 6. Old ids the script misses but that still use the term were kept (plural "previews", "24v24" mode labels, "the proposed reference server").

**Not settled:** the "How to read an entry" text still says lists include retired synonyms. The status line still says nothing is applied to a topic yet. Item 4 kept both unchanged.

Files:
- /home/user/Shamble-Frontlines/docs/research/GLOSSARY.md
- /home/user/Shamble-Frontlines/.checkpoints/v2-research/engine-architect-r14-usedin.md

## Addendum: the four stale header lines (reply to `prompts/engine-architect-r14-glossary-meta.txt`, about 20:40 UTC, verbatim)

All four header lines in /home/user/Shamble-Frontlines/docs/research/GLOSSARY.md now read as you gave them, matched by words and rewrapped:
1. "The owners applied the approved terms to the six topics in step 3c (2026-10-30; each topic's part 7 lists them)."
2. "the ids whose text uses the term, and how. The lists were refreshed after step 3c from the session's search script (advisor condition 9; commit 09de883)."
3. "…skips them as it skips quotes (approved by the advisor, 2026-10-30, step 3b)"
4. "Their full wording is in each affected topic's "Shared open questions (the same words in every topic)" subsection."

The body is unchanged. Nothing is committed.

## Addendum: R3's line under "Time statistics" (reply to `prompts/engine-architect-glossary-r3-timestats.txt`, about 20:47 UTC, verbatim)

Done; only that field changed, not committed. It now reads: "…R3-MEASURE-REC1 (median, p95, p99 and max; R3's frame marks name p95 and the median), R3-BUDGET-REC2's CPU, animation, sim and GPU rows, R3-CHAR-REC1, R3-CHAR-REC2, R3-CHAR-REC4, R3-AUDIO-REC1 and R3-AUDIO-REC3 (sub-budgets at p99, written out in step 3d by the advisor's ruling)."

## Addendum: the step-3d glossary nits (reply to `prompts/engine-architect-glossary-3d.txt`, about 10:16 UTC on 2026-10-31, verbatim)

All five changed, not committed; a word diff shows only these:
1. Time statistics, definition: "…A one-off deadline (a single event, such as a rebuild, a bake or a recovery) is a plain limit, not a statistic."
2. §1: "…and the interest marks (0.70 ms) scaled by 1.125…"; "…10.93 ms (sums of rounded parts; the exact sums are 0.01 ms lower: 4.11 (5.11), 9.92 (10.92))".
3. Both now say "for the server tick".
4. "(P1: the other 25 % of the tick period, inside the server tick budget, is the server-only share)". It was in "The server tick budget" entry, so I changed it there.
5. "…advisor's ruling; R3-AUDIO-REC1's by the same rule, found by R3's owner)".
