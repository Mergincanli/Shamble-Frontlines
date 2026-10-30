# Step 3d, R3+R5+R6: the verifier's report (verbatim)

Workflow run wf_1359fb87-b9e (prompts `prompts/verifier-r356-3d.txt`, then `prompts/verifier-r356-3d-skeptic.txt`), launched 2026-10-30 20:18 UTC, returned about 20:34 UTC. Notes: `verifier-r356-3d.md`; hit judgments `verifier-r356-3d-hits.tsv`.

VERDICT: PASS WITH NOTES

Every approved change for R3, R5 and R6 is in its file in the approved words. No approved change is missing, no owner logged an OBJECT, and no look row changed beyond a glossary term. A few small items go back to the owners and two go to the advisor (listed under Problems). Nothing here needs new research.

**Gate:** skipped, as the task says (the main session runs it). What I checked instead:
- **Approved changes present:** R3 7 of 7, R5 18 of 18, R6 27 of 27. The four "(b)-only" changes (C17.b3, C23.b1–b3) are correctly not applied.
- **C17.5 (R5):** the mechanical check reports its "after" text "not found verbatim". That is expected: the glossary rename that is supposed to follow it (R6's security baseline → the security baseline) was applied on top, as the glossary allows.
- **Shared questions:** all 11 copies match resolutions.md word for word after stripping links: MODES-3 in R3, R5 and R6; BASELINE-WHEN, PREV-DELETE and PREVIEW-WALL in R5 and R6. PREVIEW-WALL includes the advisor's added clause about unreleased art.
- **Cross-topic questions:** all 31 match cross-topic-oqs.md word for word (R3 8, R5 14, R6 9). Placement and headings are as approved.
- **Pointers from recommendations:** R3 5 of 5, R5 10 of 10, R6 5 of 6.
- **Open-question totals in the headers:** R3 47 = 38 + 1 + 8; R5 63 = 45 + 4 + 14; R6 36 = 23 + 4 + 9. All correct.
- **"Retire (exact)" phrases:** 0 in all three files, outside quotes and dated records. I re-ran the session's script and grepped myself.
- **"In context" hits:** 210 judged plus 1 pattern hit, no sampling. 206 right, 5 wrong.
- **Glossary links:** all three headers link GLOSSARY.md. The 6 first uses the script flags as unlinked are all false positives: preview.js, "can mean", "Start with", "sprint mean", "slots" meaning VM slots, and TL13's "mean".

**Symbol diff:** not applicable (research files only). The quotes check agrees: every quoted text in its list is new (none left the files), and each is in the approved texts, a glossary term name or a heading.

**Evidence**
- Commands run (read-only):
  - `git diff 7c6e0d7 41f9cf9` on each of the three files.
  - `python3 -I` comparisons of the files against resolutions.md and cross-topic-oqs.md.
  - `scripts/glossary_check.py --skip-records --first-uses`, output to the scratchpad only. It found the same 210 rows as the session's hit list.
- Arithmetic redone:
  - Draw calls: 48 × 4 × 2 = 384, + 202 + 200 = 786; 54 × 4 × 2 = 432, + 202 + 200 = 834.
  - C1.21: two rooms at 50 % each fill one vCPU.
- Cross-references the owners added outside the approved texts, checked against resolutions.md and the R1 and R2 working trees:
  - R3 (r3:594–597), "R1 names the reference server and R3-BUDGET-REC1's minimum laptop": holds (C9.3, C11.2).
  - R6 pointer to R2, "R2-BUD-REC6 now names it the reference server": holds (C11.8).
  - R6 pointer to R1, "one per vCPU of the reference server since resolution 11": holds (C11.5).
- Changed recommendations: each already has a basis claim confirmed in its part-7 verifier table, so none needed a new fetch.
  - R3: R3-BUDGET-REC2, R3-CHAR-REC1 to REC4, R3-MEASURE-REC2. Confirmed bases: CHAR-F1, F2, F4, F7, F9, F12; BUDGET-F1; LOOK-F5; MEASURE-F3.
  - R5: R5-PREV-REC2, REC4, REC5, R5-TEST-REC5, plus three renames. Confirmed bases: PREV-F4, F12, F13, F14, F18, F20; TEST-F14; RUSTLINT-F13.
  - R6: R6-HOST-REC1, R6-HOST-REC3, A-2 in R6-COST-REC1. Confirmed bases: F1, F3, F5, F12, F15, F17.
- Two renames checked against their findings: R5-PREV-F18 is the Pages headers page, and R6-HOST-F5 is the Workers pricing page. Both renames hold.
- Artifacts:
  - Notes (end in DONE): /home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r356-3d.md
  - Hit judgments: /home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r356-3d-hits.tsv
  - Fetched page: /tmp/claude-0/-home-user-Shamble-Frontlines/ed3ff506-9a45-5a14-adc3-75c262311890/scratchpad/3d-r356/f1/page.html

**Research spot-check** (one new claim, from the R6 TL;DR's reordering)

| Claim | Source | Verdict | Location and exact words |
|---|---|---|---|
| R6 TL;DR (r6:46): Cloudflare Pages "charges nothing for static traffic" (now reads as Pages' claim; its finding F5 is about Workers) | developers.cloudflare.com/pages/functions/pricing/ ("Last updated Sep 8, 2026") | requests part: confirmed | section "Static asset requests": "On both free and paid plans, requests to static assets are free and unlimited." |
| Same claim, egress and bandwidth part | same page | unsupported | the page has no egress or bandwidth words |

Counts: 1 confirmed, 0 wrong, 1 unsupported. The page is not among R6's sources. If R6 cites it, the "price inferred" wording in C19.3 and C19.8 could change in a later logged change.

**Labels** (no "Rests on:" finding id changed, and no key claim was newly checked):
- R3: "spot-checked (45 of 77 key claims)"
- R5: "spot-checked (61 of 115 key claims)"
- R6: "spot-checked (21 of 42 key claims)"

**Problems**

Back to the owners:
1. **R6, r6:46 (TL;DR):** "Cloudflare Pages (the client host), or Workers static assets …, charges nothing for static traffic". C19.6 scopes the free traffic to Workers (F5). Fix: scope the sentence the same way, or cite the Pages pricing page above (a new source, so it is a fix-pass decision).
2. **R6, r6:709 (R6-COST-REC1's spike):** "rooms per VM measured … at 48, 36 and 18 players" assumes MODES-3 reading (a): 18 is 6v6v6 counted as totals, and the 54 of reading (b) is missing. MODES-3 asks every such mark to give both values or name the test-mode list. (Hit judged wrong.)
3. **R6, r6:799 (R6-TEL-REC1's spike):** "3 modes (2 teams, 3 teams, 48 players)" uses a bare 48 for the full-size mode. Fix: "the largest mode's N (48 or 54)" or "24v24". Low stakes. (Hit judged wrong.)
4. **R5, r5:401–402 (wave-1 cited list):** it restates R1-ECS-REC2 mark 2 in its old words ("zero std HashMap or HashSet iteration in the sim's outcome code"). C29.3 changed that mark to R5-RUSTLINT-REC2's type ban. Fix: add "the hash-map ban" beside it. I found this one by reading; the script cannot search for it.

To the advisor:
5. **R3, r3:1019 and r3:1026 (R3-LOOK-REC3):** "48 soldiers on screen" and "12, 24 and 48" use 48 to mean the largest mode, exactly as R3-CHAR-REC1 did before step 3c. But this is a look row, and condition 5 keeps look rows word for word. Decide whether to leave it or add the term outside the row. (Two hits judged wrong.)
6. **R3 time marks with no statistic:** under the glossary's "Time statistics" rule, these now read as p99, a change nobody logged:
   - R3-BUDGET-REC2's CPU, animation, sim and GPU rows (r3:1124–1131)
   - R3-CHAR-REC1's 2.0 ms (r3:1045)
   - R3-CHAR-REC2's 1.0 ms (r3:1061)
   - R3-CHAR-REC4's 2.0 ms GPU (r3:1087)
   - R3-AUDIO-REC3's 0.2 ms per frame (r3:1266)

   The frame-time rows above them are p95 and median. The glossary's "Used today" says R3's marks already name a statistic, which is true only of the frame-time rows. Decide: give these marks a statistic (for example p95, like the frame rows), or accept p99.

Nits:
- **R6-OBS-REC4:** has no pointer to R6-XT-OQ5, which names it in its "Decided by" line.
- **R6-HOST-REC1:** its "Rests on:" list lacks R5-PREV-F12, F13, F14 and F18, which C19.6's reason now cites (the owner flagged this too).
- **r6:1062:** quotes R1-ECS-OQ4's wording from before step 3c ("one match per core"). The new term is added beside it.
- **R3-BUDGET-REC2:** the hull-outline figures "about 1,060" and the "about 1,100" Low cap hold only at N = 48. At N = 54: 834 + 216 + 80 = 1,130. R3-BUDGET-OQ4 covers N, and this is look-adjacent arithmetic, so the owner was right not to change it.
- **R3-CHAR-REC1:** its 2.0 ms full-rig mark is proposed and does not depend on N. It still holds, but the time per soldier drops from 41.7 µs at 48 to 37 µs at 54.

Is what remains only nits? No. Items 1–4 are small wording fixes for the owners and items 5–6 are advisor calls; nothing needs new research.

**Not verified**
- **R1 and R2 files:** the cross-references I checked against them use their uncommitted working trees, since step 3c is still running there.
- **Earlier checks:** no source claim already confirmed in a part-7 verifier table was re-fetched.
- **Revised table:** resolution 1's revised table was not recomputed. It belongs to R1, R2 and R4, not to this unit.

**Host audit**
- WebFetch: 1 fetch, developers.cloudflare.com, final host developers.cloudflare.com.
- curl: 1 fetch, developers.cloudflare.com, HTTP 200, final URL https://developers.cloudflare.com/pages/functions/pricing/.
- Both hosts are on the allow list. No other host was requested.
