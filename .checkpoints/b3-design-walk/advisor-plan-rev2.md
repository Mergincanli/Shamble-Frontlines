# Advisor review: B3 plan, revision 2 (only its changes)

## Review plan
1. Read main's advisor.md (plan review) and CLAUDE.md. Done.
2. Read the checkpoint's "Plan revision 2", Mustafa's answers (mustafa-2026-11-04.md), my rev1 verdict and the folded
   plan. Done.
3. (a) Each new line (L7, L8, L9, L12) against his answers, word for word; anything it adds beyond them.
4. (b) Line 105 and line 112 against B3-Q2 (a); other docs that quote them.
5. (c) The #35 follow-ups (queue/docs-pass-items.md, "For the NEXT docs pass") against B3's scope.
6. (d) The counts, and every place rev1 still points at the four questions.
7. Verdict and conditions.

## Notes as I go
- Worktree HEAD 73d546f = main 4893ab6 + the checkpoint commit. #35's merge commit time 2026-11-04T16:50:39+01:00 =
  15:50:39 UTC: the checkpoint's "15:50 UTC" is right.
- L7: his words byte for byte except "I write" -> "Mustafa writes", "(c)" -> "Shared or per faction: decided". He
  wrote "from tech 1" where my question said "level 1". LOOP_V2 §4 (line 59) and §5 (line 72-73, `[DEFAULT]` Tier 2)
  put the Javelin, the only listed anti-vehicle unit, at Tech 2. "Their lines stay as they are" leaves that clash
  unstated: it needs one untagged note, no new question (his (c) gives it to the unit list).
- L8: his words byte for byte, plus two lead-ins ("The 'tech building' is the HQ:", "The KGB agent's 'downed
  enemy':"), which restate his own question. "A destroyed power plant raises that team's prices" is LOOP_V2 §3 line 37
  (`[DEFAULT]`): L8 rightly does not add "prices rise". "The HQ holds the team's tech" sits beside §8 line 99
  ("Destroying an HQ is a big points swing"): what a destroyed HQ does to that tech is not said; note it, decide nothing.
- L9, L12's inserted lines: verbatim.
- Line 112's new text is not given ("the same goes for").
- Stale after rev2 and not in it: UNITS §0's two `[OPEN]` pointers (Factions; Infiltrators), DESIGN_NOTES N3
  ("Whether new classes stay account unlocks is open in LOOP_V2 §8"), rev1 §7's risk "neither is removed until he
  answers", rev1 §1/§3/§7/§11/§12/Counts/For Mustafa "4 open questions", E4's "4".
- ADR 0005 lines 1263 and 1278-1283 (AB5's options (a) and (c): "LOOP_V2 §8: 'sidegrade variants'", "the catalogue
  grows in classes", "unlocking a class") quote or lean on line 105: not in L16's outside-pointer list. Research
  r1-simulation-core.md:1386 also cites "sidegrade variants" (a research record; a note only).
- Register: AB5 (README row 60) is today in R0's "bears on" list; a dated clause on its own row moves it out of R0.
  CD5 (row 114, "Faction variants: full definitions that reference shared ones, or overrides of a base?") is touched
  by B3-Q1's "shares one core with a faction twist on top": bears on, does not answer.
- #35 follow-ups: item 4 done, item 7 no change. Item 5's "B3" sits inside his quoted words in README row 86 and ADR
  0006's OQ-0006-23 (line 1155), and in the merged #35 report line 97: B3 edits no ADR file and no quoted words. The
  PR Advisor's first scheduled run is 18:45 UTC today; the first guarded night starts 2026-11-05 00:10 Amsterdam.
- Counts: no "Open since" in docs today (grep); after rev2's four replacements it is 0.
- `[OPEN]` today: LOOP_V2 4 (tag line 7, the Spy's cost 77, aiming 128, map size 130), UNITS 0, DESIGN_NOTES 1. After
  B3 with rev2: LOOP_V2 6 (plus L2's AB14 sub-bullet and L13's war), UNITS 0 once §0's two pointers lose the tag.

---

## Verdict: APPROVE, with conditions

Revision 2 records his three answers faithfully: every new line holds his words, and the lead-ins only restate the
question he answered. Superseding line 105 and the "specialists" half of line 112 is right under B3-Q2 (a). Folding
PR #35's follow-ups into docs-keeper's step is right. The new counts are right (0 "Open since"). But revision 2 misses
a few things: one clash his answer creates, line 112's new text, and the places rev1 still points at the four
questions. Conditions 1-10 fix them while building; no revision 3 is needed.

**What I checked:** L7, L8, L9 and L12 against `mustafa-2026-11-04.md`, clause by clause. LOOP_V2 lines 37, 55-60,
62-79, 82, 99, 105 and 110-113 in the worktree (73d546f). Every doc that quotes line 105 or 112: DESIGN_NOTES:21,
ADR 0005:1263, 1278-1283 and 1301, and r1-simulation-core.md:1386. Register rows 60 (AB5), 86 and 114 (CD5). The
#35 follow-up list, and where item 5's "B3" lives (README:86, ADR 0006:1155, the #35 report:97). #35's merge time
from git. The "Open since" and `[OPEN]` counts today. **Not checked:** E1-E10 (nothing is built yet), and the routine
proofs (they don't exist yet).

### Rulings on (a)-(d)
- **(a) His words:** faithful.
  - **L7:** his answer, with "I" made "Mustafa" and "(c)" spelled out. "This covers the Javelin and the Officer … and
    the Juggernaut" is right: those are the three units his question named. "Their lines stay as they are until the
    unit list decides them" describes the docs, not the design, so it decides nothing.
  - **The clash L7 leaves unstated:** he wrote "from tech 1", where my question said "level 1". In LOOP_V2, Tech 1 is
    the tier where class abilities unlock (§4). The Javelin, the only listed answer to vehicles, is a Tech 2 unit
    (§4 line 59; §5's `[DEFAULT]` Tier 2). Read either way ("tech 1" or "tier 1"), today's roster breaks his new rule
    until the unit list fixes it. His (c) gives that job to the unit list, so the clash gets one untagged note and no
    new question (condition 1).
  - **L8:** his words, with two lead-ins that restate his question ("The 'tech building' is the HQ:", "The KGB agent's
    'downed enemy':"). It rightly does not add "prices rise": that is §3's own `[DEFAULT]`. One new tension to note,
    with no question: "the HQ holds the team's tech" sits beside §8's "Destroying an HQ is a big points swing", and
    his words don't say what a destroyed HQ does to that tech (condition 3).
  - **L12:** handles line 105 rightly. For line 112, its new text is missing (condition 4).
- **(b) Superseding lines 105 and 112:** right.
  - **Line 105:** "more classes" is exactly what B3-Q2 (a) replaces. The two inserted lines keep everything else in
    it: "never power", "class variants" (his own word for sidegrade variants) and "cosmetics". So replacing the whole
    line loses nothing he still holds.
  - **Line 112:** only "and specialists" is superseded: tier-two specialists are new classes, and new classes now
    come through tech. "Unlocks add variants" and the line's first sentence stand.
  - **Other docs that quote the old words:** ADR 0005 quotes line 105 (AB5's options, line 1263), and DESIGN_NOTES'
    Mastery line repeats "more classes". They need pointers (conditions 5 and 6).
- **(c) Folding #35's follow-ups:** right. STATUS and ROADMAP are docs-keeper's files in step 6 anyway, and one pass
  avoids a duplicate change-set. It comes with limits:
  - No ADR edits, and no changes inside his quoted words.
  - B3 never waits for routine proofs that don't exist yet.
  - The queue file marks what B3 took (condition 9).
- **(d) Counts:** right. "Open since" becomes 0 everywhere, because the four questions are answered and no other
  "Open since" exists in docs/. §8 becomes "Answered by Mustafa (2026-11-04)". But the pointers to those questions
  must also go, and E4 must prove it (conditions 6 and 7).

### Conditions
1. **L7: one note on the anti-vehicle rule.**
   - After "…until the unit list decides them.", add: `Today the Javelin, the only listed answer to vehicles, is a
     Tech 2 unit (§4 and its line above): which unit meets the tech-1 rule is for the unit list.`
   - The report tells Mustafa this in plain words, with no decision asked. He wrote "from tech 1", where the question
     said "level 1". The docs keep his words. Today the Javelin unlocks at Tech 2.
2. **His answers are under the same checks as his walk.**
   - **§3's deviations list gains:** "I" becomes "Mustafa" (L7). The answer letters "(c)", "(i)" and "(ii)" are
     replaced by lead-ins that restate his question ("Shared or per faction: decided", "The 'tech building' is the
     HQ:", "The KGB agent's 'downed enemy':") and are cited as "B3-Qn (x)". A clause after a lead-in keeps its
     lowercase.
   - **E1 and E2 extend to his three answers:** each clause is checked byte for byte, allowing only those
     deviations, and each sits under `[DECIDED]` (L7, L8 twice, L12's second line).
   - **The report's appendix carries B3-Q1 to B3-Q3 as sent** (the question, the options and the recommendation,
     from `advisor-plan-rev1.md` "Decisions needed"), next to his answers. That way "(c)", "(a)", "(i)", "(ii)" and the
     "B3-Qn" citations in LOOP_V2 resolve from git alone. §8 "Answered by Mustafa (2026-11-04)" keeps each question
     with its options and adds his answer word for word.
3. **The HQ tension goes in §8.5** (noted, no question): `"The HQ holds the team's tech" (§5) sits beside §8's
   "Destroying an HQ is a big points swing": what a destroyed HQ does to the tech it holds is not said.`
4. **Line 112's exact new text:** `  level 1. Unlocks add variants, so new players are never short of an answer.`
   The line stays `[DEFAULT]` and its first sentence is unchanged.
   - §12's list (L16) gains, word for word: in §8, "`[DECIDED]` Unlocks give breadth, never raw power: more classes,
     sidegrade variants, cosmetics." and " and specialists".
   - E3's replaced strings go from 7 to 9 (give them S-ids).
   - Counts are updated.
   - Rev1 §7's risk "Two `[DECIDED]` lines that may clash … neither is removed until he answers" is rewritten: line
     105 is replaced under B3-Q2 (a).
5. **Outside pointers (L16) gain ADR 0005.**
   - Lines 1263 and 1278-1283: AB5's options quote LOOP_V2 §8's "sidegrade variants" and weigh unlocking classes
     ("the catalogue grows in classes", "as unlocking a class"). They are now pointers for the drafts' next revision,
     with no ADR edit.
   - The report names r1-simulation-core.md:1386 as a research record left as it is.
   - "8 listed for others" is updated.
6. **The stale pointers are replaced** (all untagged pointers):
   - **UNITS §0, Factions:** `  - Shared or per faction: decided unit by unit in the unit list, with two fixed rules:
     LOOP_V2 §5 (Mustafa, 2026-11-04).`
   - **UNITS §0, Infiltrators:** `  - The tech building is the HQ, and "downed" is a short window on a fresh body:
     LOOP_V2 §5 (Mustafa, 2026-11-04).`
   - **UNITS §0's tag line** drops `[OPEN]`, because §0 then has none.
   - **N3:** `→ *Superseded 2026-11-04: account progression gives class variants and cosmetics only; new classes come
     through the team's tech in the match (LOOP_V2 §8).*`
   - **L15a and N5:** "tech building" becomes "tech building (the HQ)". This is pointer prose, not his quoted words.
   - **The plan text:** every "4 open questions" (§1, §3, §7, §11, §12, Counts, For Mustafa) becomes "0 open; 4
     answered (B3-Q1 to B3-Q3)".
7. **E4 and E5's new expectations:**
   - "Open since 2026-11-04" is 0 in every file under docs/.
   - No "See …open question" pointer is left in LOOP_V2, UNITS or DESIGN_NOTES.
   - The `[OPEN]` count is 6 in LOOP_V2 (today's 4, plus L2's AB14 sub-bullet and L13's war), 0 in UNITS and 1 in
     DESIGN_NOTES (unchanged).
8. **The register.**
   - **AB5 (row 60)** gets: `Mustafa (2026-11-04) bears on it: "Account progression gives class variants and
     cosmetics only. New classes come through the team's tech in the match, so nobody is ever missing a counter."
     (LOOP_V2 §8). How an account-unlocked weapon reaches a match is not said.` AB5 then leaves R0's "bears on" list
     (21 ids).
   - **CD5 (R6)** adds: `B3-Q1 bears on it too: "a role every faction has (anti-vehicle, infiltration) shares one
     core with a faction twist on top" (LOOP_V2 §5); the data shape stays open.`
   - Register edits go from 12 to 13.
   - The verifier checks both clauses. It also checks whether B3-Q3's answers plainly touch any row; a "bears on"
     clause is added only where they do.
9. **#35's follow-ups, with these limits:**
   - **Items 1, 2, 3 and 6** go in STATUS and ROADMAP.
   - **Item 5** is fixed only in STATUS, ROADMAP and B3's own report. Its other places stay in the queue file for the
     drafts' next revision: README row 86, ADR 0006's OQ-0006-23 (both inside his quoted words) and the merged #35
     report.
   - **Items 4 and 7:** nothing to do.
   - **The two routine proofs:** record only what has a record when docs-keeper runs, read from the routine's session
     record. B3 never waits for them, and what isn't recorded yet stays in the queue file. A record added after the
     advisor reviews a head is a new push, so it is reviewed again.
   - **Each fact is checked at its source:** 4893ab6 and 15:50:39 UTC from git; 14:03:10 UTC from the routines-API
     record.
   - **Bookkeeping:** the session marks each item it takes as "taken by B3 (PR #36)" in `queue/docs-pass-items.md`.
     The report gives these items a short section of their own, and E7 lists every file they touch.
10. **Unchanged:** rev1's 11 conditions stand, except condition 10's Night-queue entry (moot: B3 runs in this day
    session) and condition 11's counts (replaced by condition 7). The reviewers and their effort are as in rev1.

The PR will likely be needs-mustafa (rule text in the core loop): his reconfirmation, not a Brave check.

## Decisions needed
None. His three answers settle B3-Q1 to B3-Q3, and every gap they leave is noted for his unit list or routed.

## For Mustafa
1. Nothing to check in Brave: B3 changes docs only.
2. One thing to know, no action needed: you wrote that every faction has an anti-vehicle answer "from tech 1". Today
   the only anti-vehicle unit, the Javelin, unlocks at Tech 2. The docs keep your words and note the gap for your unit
   list.
3. Later: when B3's pull request (#36) is ready and approved, you'll likely be asked to reconfirm its merge. Say
   "merge #36" in a session's chat if the record matches your walk and your answers.
