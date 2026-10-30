# Resolutions of the critic's 12 contradictions (step 3a, proposals for the advisor's review)

- **Task:** step 3a of `plan-amendments-2.md` (advisor conditions 1–4 applied). Author: engine-architect, 2026-10-30.
- **Status:** proposals. Nothing here is applied; the advisor reviews each item (step 3b, verdict to
  `advisor-resolutions-verdict.md`), and the owners apply only approved items (step 3c).
- **Inputs:** the six topic files as they stand at commit 8dee746 (line numbers below are of that state), the critic's
  list `engine-architect-consistency-result.json`, VISION_V2 §3, §4 and §6, and the advisor's table "Who decides each
  contradiction" (`advisor-amendments-2-verdict.md`). No page was fetched; no new source is used.
- **Outcomes:** **resolved** (one position holds, from findings the files already hold), **aligned** (one shared open
  question, the same words in every affected topic, one decider and a decision code) or **escalated** (a question for
  Mustafa or the Advisor, with options and a recommendation).
- **Numbers:** every changed number comes from a topic file (its id is given), is arithmetic on those numbers (shown
  under "Arithmetic"), or is a pass mark marked "proposed" with its reason.
- **Glossary:** terms such as "the stress scene" or "the server tick budget" are defined in `docs/research/GLOSSARY.md`.
  In the "after" texts, `(GLOSSARY)` marks a first use, which the owner turns into a link.
- **How the before/after texts work:** "before" quotes the file at the line given, exactly; "after" is the replacement
  for exactly that text. Text outside the quote stays. Where an item depends on an escalated decision, the "after" text
  holds under every option (advisor condition 2).
- **Order of work** (advisor condition 2): #10, #11 and #9 with their glossary terms, then #1; #19 before #21 and #22;
  #5 with #6; then #17, #23, #29. The items below are listed by rank.

## Summary

(Filled in when all 12 are written.)

## Where shared open questions go

An aligned or escalated item becomes one shared open question with the same words in every affected topic. Each topic
gets one new subsection for them, **"### Shared open questions (the same words in every topic)"**, at the end of its
open questions, placed before:
- R1: `### Pointers between R1's units` (r1:2872);
- R2: `### Pointers between the R2 units` (r2:3215);
- R3: `### Pointers to other topics` (r3:1398);
- R4: `### Wave-1 items R4 changes or answers (said openly, not silently)` (r4:1110);
- R5: `### Recommendations that hinge on a source not read` (r5:1889);
- R6: `### Recommendations that hinge on a source not read` (r6:915).

Each shared question is headed by its decision code in bold (for example **MODES-3 (Mustafa).**), so a search for the
code finds every copy (advisor condition 11). A topic's own earlier question on the same point stays and gains one
closing sentence, "Shared as <code> (below).". The critic's other 29 items go in a second new subsection after it,
`### Cross-topic open questions (consistency critic, 2026-10-29)`, with ids `R<n>-XT-OQ<k>` (see `cross-topic-oqs.md`).


| Rank | What clashes | Outcome | Decider and code |
|---|---|---|---|

## 1. What the tick budget covers, and its shares

## 5. Whether the client predicts the player's own abilities

## 6. The spread random stream: in the sim state or server-only

## 9. The stress scene

## 10. Mode sizes: totals or per team

**Outcome: escalated. Decision code MODES-3; decider: Mustafa** (the advisor's table: "Mustafa now (MODES-3 below)").

**Both sides.**
- Totals: r1:2587-2588 (R1-TICK-OQ7) "the largest mode (48 soldiers, 47 of them bots)"; r1:2098-2099 (R1-DET-REC1) "a
  bots-only 24v24 match (48 players) and a 12v12v12 match (36 players)"; r5:1471 (R5-TEST-REC5) "24v24 = 48 and
  12v12v12 = 36"; r6:442 "6v6v6 (start, three teams, upper end) | 18"; r2:1768-1769 (R2's worked estimate) "Two teams,
  4v4 (VISION_V2 §3 start, upper end)" and "Three teams, 6v6v6 (start, upper end)".
- Per team: r2:2820 (R2-ROOM-REC7) "mode files for 2×2, 2×8, 3×6, 3×18, 12v12v12 and 24v24" (3×18 = 54); r2:2809
  (R2-ROOM-REC6) "3 teams × 16 slots".
- Three-team test sizes: r2:2447 (R2-INT-REC3), r4:954 (R4-LOS-REC1 mark 4) and r4:969 (R4-LOS-REC2) "24v24 and
  16v16v16".
- Existing questions: R2-BUD-OQ6 ("totals or per team? … Decides: Mustafa"), R2-XU-OQ13, R3-BUDGET-OQ4.

**Why it is escalated.** The topics' findings cannot settle how VISION_V2 §3 is read; only its owner can (R2-BUD-OQ6
already names Mustafa). It sets the largest mode's N, and with it the stress scene (resolution 9), the offline scale
test (R1-TICK-OQ7), the soak runs (R5-TEST-REC5) and the cost rows (R6).

**The question for Mustafa (MODES-3).** VISION_V2 §3 says "two teams with 2–8 players, and three teams with 6–18
players" to start. Are these totals or sizes per team?
- (a) Totals: 1v1 to 4v4, and 2v2v2 to 6v6v6. The largest mode is then the goal 24v24, N = 48; the three-team goal
  12v12v12 has 36.
- (b) Per team: 2v2 to 8v8, and 6v6v6 to 18v18v18. The largest mode is then 18v18v18, N = 54.
- **Recommendation: (a).** Under (b) the "start small" modes would be larger (54) than both goals (48 and 36), and 54
  is "above the 48 design point" (R2-BUD-OQ6). A run above 48 stays possible as a labelled headroom run (the design
  point and headroom, GLOSSARY), as R1-TICK-REC2 P1 and R4-NAV-REC2 already run N = 64. This is also the advisor's
  recommendation.

**What changes now, under every option** (basis: VISION_V2 §3; R2-BUD-OQ6; LOOP_V2 §2 as quoted at r1:311 and r2:466,
"12v12v12 to 16v16v16, possibly more"). The topics stop fixing N at 48 where they mean the largest mode, name the
test-mode list (GLOSSARY) instead of their own lists, and carry one shared question.

*Arithmetic for the lists:* under (a), "2–8 players" in two teams is 1 to 4 per team and "6–18 players" in three teams
is 2 to 6 per team; under (b), 2 to 8 and 6 to 18 per team; 3 × 18 = 54; 16v16v16 = 3 × 16 = 48.

*Shared question, to insert in R1, R2, R3, R4, R5 and R6 (the "Shared open questions" subsection), word for word:*

> **MODES-3 (Mustafa). How are VISION_V2 §3's starting sizes read?** VISION_V2 §3 says "two teams with 2–8 players,
> and three teams with 6–18 players" to start, and "The goal: 12v12v12 or 24v24". (a) Totals: 1v1 to 4v4, and 2v2v2
> to 6v6v6; the largest mode is then the goal 24v24, N = 48. (b) Per team: 2v2 to 8v8, and 6v6v6 to 18v18v18; the
> largest mode is then 18v18v18, N = 54. The answer sets the largest mode's N (GLOSSARY), which the stress scene, the
> offline scale test (R1-TICK-OQ7), the soak runs (R5-TEST-REC5) and the cost rows (R6-COST) use, and items 3 and 4 of
> the test-mode list (GLOSSARY). Until it is answered, a mark that needs the largest mode's N gives both values. Also
> asked as R2-BUD-OQ6, R2-XU-OQ13 and R3-BUDGET-OQ4. Decided by: Mustafa.

*Changes per file (before → after; text outside the quote unchanged).*

R1:
- R10-1, r1:2098-2099 (R1-DET-REC1, spike):
  - before: `(a) a bots-only 24v24 match (48 players) and a 12v12v12 match (36 players), the two
  goals of VISION_V2 §3, each run 10 minutes`
  - after: `(a) a bots-only 24v24 match (48 players) and a 12v12v12 match (36 players), the two
  goals of VISION_V2 §3, plus the largest mode where it is neither (GLOSSARY; MODES-3: 18v18v18 under reading (b)),
  each run 10 minutes`
- R10-2, r1:2588 (R1-TICK-OQ7):
  - before: `does not say whether the largest mode (48 soldiers, 47 of them bots) must run offline on the minimum laptop.`
  - after: `does not say whether the largest mode (GLOSSARY; 48 soldiers under MODES-3 (a), 54 under (b), all but the
  player bots) must run offline on the minimum laptop.`
- R10-3: insert the shared question MODES-3.

R2:
- R10-4, r2:1768 (worked estimate, results table):
  - before: `| Two teams, 4v4 (VISION_V2 §3 start, upper end) |`
  - after: `| Two teams, 4v4 (VISION_V2 §3 start, upper end, as totals: MODES-3 (a)) |`
- R10-5, r2:1769:
  - before: `| Three teams, 6v6v6 (start, upper end) |`
  - after: `| Three teams, 6v6v6 (start, upper end, as totals: MODES-3 (a)) |`
- R10-6, r2:2447 (R2-INT-REC3, spike):
  - before: `at 24v24 and 16v16v16, all players in one fight`
  - after: `at 24v24 and 16v16v16 (the test-mode list, GLOSSARY) and, under MODES-3 (b), 18v18v18, all players in one
  fight`
- R10-7, r2:2809 (R2-ROOM-REC6, spike):
  - before: `(1) 3 teams × 16 slots, 30 humans`
  - after: `(1) the largest mode's N in three teams (GLOSSARY; 3 teams × 16 slots under MODES-3 (a), 3 × 18 under (b)),
  30 humans`
- R10-8, r2:2820 (R2-ROOM-REC7, spike):
  - before: `(1) mode files for 2×2, 2×8, 3×6, 3×18, 12v12v12 and 24v24 load`
  - after: `(1) mode files for every mode of the test-mode list (GLOSSARY; under MODES-3 (a): 1v1, 4v4, 2v2v2, 6v6v6,
  12v12v12, 16v16v16 and 24v24; under (b): 2v2, 8v8, 6v6v6, 18v18v18, 12v12v12, 16v16v16 and 24v24; plus the
  N = 64 headroom mode) load`
- R10-9, r2:2938 (R2-BUD-OQ6), at the end of the entry:
  - before: `point but inside the estimate's 64-player row. Decides: Mustafa.`
  - after: `point but inside the estimate's 64-player row. Decides: Mustafa. Shared as MODES-3 (below).`
- R10-10, r2:3173 (R2-XU-OQ13), at the end of the entry:
  - before: `(R2-ROOM-REC6). Decided by: Mustafa (R2-BUD-OQ6).`
  - after: `(R2-ROOM-REC6). Decided by: Mustafa (R2-BUD-OQ6). Shared as MODES-3 (below).`
- R10-11: insert the shared question MODES-3.

R3:
- R10-12, r3:1360-1361 (R3-BUDGET-OQ4), at the end of the entry:
  - before: `OQ7 decides the N of the offline scale test. The draw-call and triangle rows scale with N.`
  - after: `OQ7 decides the N of the offline scale test. The draw-call and triangle rows scale with N. The largest mode's
  N (GLOSSARY) is 48 or 54 by the answer to MODES-3 (below).`
- R10-13: insert the shared question MODES-3.

R4:
- R10-14, r4:954 (R4-LOS-REC1 mark 4):
  - before: `R2-INT-REC3's full pass (24v24 and 16v16v16, three points)`
  - after: `R2-INT-REC3's full pass (24v24 and 16v16v16 from the test-mode list, GLOSSARY, and 18v18v18 under MODES-3
  (b); three points)`
- R10-15, r4:969 (R4-LOS-REC2, spike):
  - before: `at 24v24 and 16v16v16 with everyone in one fight`
  - after: `at 24v24 and 16v16v16 (the test-mode list, GLOSSARY) and, under MODES-3 (b), 18v18v18, with everyone in one
  fight`
- R10-16: insert the shared question MODES-3.

R5:
- R10-17, r5:1471 (R5-TEST-REC5):
  - before: `(24v24 = 48 and 12v12v12 = 36, plus LOOP_V2 §11's N = 12, 24, 36, 48 scale steps)`
  - after: `(24v24 = 48 and 12v12v12 = 36, plus the largest mode where it is neither (GLOSSARY; MODES-3: 18v18v18 = 54
  under reading (b)), plus LOOP_V2 §11's N = 12, 24, 36, 48 scale steps)`
- R10-18: insert the shared question MODES-3.

R6:
- R10-19, r6:441:
  - before: `| 4v4 (start, two teams, upper end) |`
  - after: `| 4v4 (start, two teams, upper end, as totals: MODES-3 (a)) |`
- R10-20, r6:442:
  - before: `| 6v6v6 (start, three teams, upper end) |`
  - after: `| 6v6v6 (start, three teams, upper end, as totals: MODES-3 (a)) |`
- R10-21, r6:447, after the table's closing paragraph:
  - before: `client (R2). Ingress is free on Hetzner (R6-HOST-F17).`
  - after: `client (R2). Ingress is free on Hetzner (R6-HOST-F17). Under MODES-3 (b) the start modes' upper ends are 8v8
  (16 players) and 18v18v18 (54); their rows follow from R2's formula (r2:1760-1762) once Mustafa answers.`
- R10-22, r6:464 (Start scenario):
  - before: `**Start (VISION_V2 §3: two teams of 2–8, three teams of 6–18).**`
  - after: `**Start (VISION_V2 §3: two teams of 2–8, three teams of 6–18, read as totals: MODES-3 (a); under (b) a start
  room holds up to 54).**`
- R10-23: insert the shared question MODES-3.

**Basis ids:** R2-BUD-OQ6, R2-XU-OQ13, R3-BUDGET-OQ4, R1-TICK-OQ7, R2-ROOM-REC6, R2-ROOM-REC7, R2-INT-REC3,
R4-LOS-REC1, R4-LOS-REC2, R1-DET-REC1, R5-TEST-REC5, R6 egress table; VISION_V2 §3; LOOP_V2 §2 as quoted at r1:311 and
r2:466. **New numbers:** 54 (3 × 18, arithmetic), per-team ranges under (a) (arithmetic above); no other.
**Changes that depend on this item elsewhere:** resolution 9 (the stress scene's N) and resolution 1 (P1's N).


## 11. One core or one vCPU as the measuring unit

## 17. Who owns the dependency audit in CI

## 19. Previews on Cloudflare Workers or Pages

## 21. A login wall on previews

## 22. Isolation headers on every preview, or behind a flag

## 23. One wildcard certificate on every game server

## 29. HashMap in the sim: lookups allowed, or the type banned
