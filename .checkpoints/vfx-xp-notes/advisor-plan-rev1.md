# Advisor review of the vfx-xp-notes plan, revision 1 (2026-11-07)

**Verdict: APPROVE WITH CONDITIONS (10).** Docs only, symbols +0 −0, his words faithful; build after the conditions.
Nothing blocks the build on Mustafa. Two questions go to him in the report (XP-TERM, LOOK), neither blocking.

Role read from `origin/main:.claude/agents/advisor.md`; the worktree's CLAUDE.md equals main's. Worktree at 3124aeb
(main 4c6db5b plus two notes commits; only `.checkpoints/` differs from main).

## Review plan (written first)
1. His words: mustafa-words.md against the chat events and a79fef1's copy.
2. B.2's final-head review (PR #42 comment), the B.2 report, STATUS.
3. Every file the plan edits or cites, line by line.
4. The seven decisions; faithfulness; facts; other register rows; evidence, reviewers, budget; gaps.

## Rulings on the plan's §8 decisions
- **VFX-PLACE: (a)**, a new EDITOR_V2 §7 at the end. His "after Editor v2's map features" orders the work; a section
  after §6 keeps §1-§6's numbers, and §3's `[LATER]` line is about map features. No "EDITOR_V2 §7" citation exists
  today, so none changes meaning.
- **VFX-TAG: (a)**, `[LATER]` on the tool, `[DECIDED]` on each of the ten lines. His "Decided principles, design
  later" says exactly that; his own "Essential" line carries the weight of the tool. B.2's note did the same.
- **XP-PLACE: (a)**, a sub-bullet block after the trust rule (VISION_V2:70). It is the line his rule speaks to.
- **XP-TERM: (a)** for the build: keep "XP" as he wrote it. What XP means is his call, so the question goes into the
  report's "Decisions needed" (condition 3), not only a STATUS follow-up. The plan's premise needs a fix: "XP" is
  defined in the docs (condition 2).
- **REG: (a), widened.** Keep the two clauses (R2-ROOM-OQ7, OQ-0005-CD7) and add two: OQ-0006-21 (README:160) and
  OQ-0003-16 (README:104). No clause for R6-TEL-OQ2, OQ-0005-AB3 or AB5 (condition 4).
- **LOOK: (a)**, no LOOK_V2 edit, plus a plainly worded question for Mustafa in the report (condition 5). Editing
  LOOK_V2, even with a pointer, would let a docs pass touch the look brief.
- **HEADER: (a)**, the dated EDITOR_V2 line, with STATUS:656's :93 and :94 moved to :94 and :95. Date it honestly
  (condition 6).

## Conditions (each checkable)
1. **His words, exactly.** E1 checks, case-sensitively and with wrapped text joined: his ten VFX lines whole (text
   after the tag); the three kept opening pieces "a VFX editor" (written "A VFX editor": the one capital), "as a future
   tool, after Editor v2's map features" and "Decided principles, design later"; his XP header words "on progression in
   private (rented) matches"; his two XP lines after their tag, with "Open," kept beside `[OPEN]`. His `[DECIDED]` was
   typed without backticks; the backticks are the docs' form, and the report says so.
2. **Fix the XP premise.** Plan §4 and risk 2 say LOOP_V2 has no "XP" and "XP is not a defined term". Wrong:
   LOOP_V2:242 (`[DECIDED]`, §13) "Levelling: XP from kills, assists and objective work while using that unit or
   weapon", the in-match weapon track that resets each match (LOOP_V2:237-238); register :69 (OQ-0005-AB3) quotes it;
   DESIGN_NOTES:23 says "account XP" (superseded line); the legacy career keeps XP and levels (LEGACY_INVENTORY:302,
   GE15), and LOOP_V2:140 calls the account level "the existing career rank". The plan, report and follow-up list
   these candidates and map none.
3. **XP-TERM goes to Mustafa** as a "Decisions needed" entry in the report, non-blocking: "Which XP does your
   private-match rule cut to 25 %?" Options: (a) all account progression (the account level and Service points);
   (b) Service points only; (c) the account level only; (d) also the in-match weapon XP (LOOP_V2 §13). Advisor's
   recommendation: (a), since it sits under the trust rule about progression and the weapon XP resets each match.
   The STATUS follow-up points at that entry.
4. **The register clauses**, each one "Mustafa (2026-11-06) bears on it" clause, quoting him, appended, no ADR edited:
   - R2-ROOM-OQ7 (:183): his XP rule; the clause says neither Service points nor telemetry is answered (XP-TERM open);
   - OQ-0005-CD7 (:122): his "Whole impact moments" and "Test in context"; it must not say the VFX editor is, holds or
     replaces the Feel Lab;
   - add OQ-0006-21 (:160): its sentence "His note in VISION_V2 §5 ... gives why, the trust rule and the precedents" is
     incomplete once §5 holds his XP rule: add "and, since 2026-11-06, his XP rule for private (rented) matches";
   - add OQ-0003-16 (:104, the look floor): his "quality tiers (Low drops extras but stays readable)"; decides nothing.
   - Not R6-TEL-OQ2 (balance rates, not XP), not AB3 or AB5 (would map XP by placement). The report says why.
5. **The look.** The STATUS follow-up and a report "Decisions needed" entry state the tension in plain words: his
   "Comic flavour: ink-outlined smoke, impact frames, halftone flashes, onomatopoeia" against LOOK_V2:63's `[DEFAULT]`
   "Comic impact touches for big moments, used sparingly: halftone flashes, ink 'line boil'". Question: does his line
   change LOOK_V2 §7? Options: (a) no, LOOK_V2 §7 stands until his VFX style guide and the look proof test;
   (b) yes, LOOK_V2's owners update §7 in their own pass. Recommendation: (a). This pass edits no LOOK_V2 line, and the
   style-guide research is a follow-up only: not queued, not in the Night queue, needs its own plan.
6. **Dates and lines.** EDITOR_V2 has 112 lines, not 113 (§6 is :96-112). The header line goes at :7 and is dated
   honestly: "Updated on 2026-11-07 with Mustafa's VFX editor of 2026-11-06: §7. The scope above stays the map
   editor." (or the builder's equal wording with both dates). E4 lists every line citation into both files; the moved
   ones are STATUS:656 (fixed) and ADR 0005's VISION_V2 ranges (the existing follow-up, renumbered for this pass).
7. **The need rule (risk 3) in writing.** The STATUS follow-up says: under decision map J3 (README:408, "no feature for
   `[LATER]` or `[OPEN]` items") EDITOR_V2 §7 gives the drafts no feature now; its principles are inputs for the tool's
   design session. It must not cite VISION_V2 §6.3 as the VFX need: §6.3's "effect" is the gameplay ability-and-effect
   system, not visual effects. Today's visual effects already trace to LOOP_V2 §7, LOOK_V2 §3/§7 and §6.9 (ADR 0003
   part 5.k).
8. **The B.2 follow-up and the match record.** "Not decided by the note" is updated to "bears on it", not "answered":
   he set XP rates for private (rented) matches; whether that is the progression the trust rule means waits on
   XP-TERM. The record follow-up says no field is added now (the note is `[LATER]`, J3) and that ADR 0006 part 12
   already has "private flag" and "humans and bots per team" (ADR 0006:474-475).
9. **The B.2 corrections are exact.** Low note 2 names #40's STATUS :3, :31 and :173 at **0588a17** (the head the
   note checked; #40's head is now 10b1a53). Notes 1, 3 and 4 as in the plan; note 1 goes with the removed VFX entry.
10. **The CI line is factual.** Cite run 42313665674 on e223ecc: attempt 1 cancelled after 6 min 34 s in "Install
    Chromium's system libraries" (23:42:39-23:49:13 UTC), attempt 2 passed after 6 min 49 s in that step; the push run
    42313658977 did it in 11 s. Not "stalled twice": one stall and cancel, one slow pass. The checkpoint says it the
    same way. No `.github/` change in this pass.

Optional, builder's choice: a dated clause in VISION_V2's header line for the §5 addition (B.2 added one for its own).

## On the evidence, reviewers and budget
- E1-E7 are right for a docs pass; E1 and E3 widen per conditions 1 and 4 (README gains four clauses, not two;
  E3 adds README's tag counts: unchanged). E7's file list is right; LOOK_V2 stays untouched (E7 checks it).
- Reviewers: the verifier, game-designer (XP rule and the XP-TERM candidates), art-director (the look lines):
  enough. graphics-engineer is not needed: nothing technical is decided and no ADR is edited.
- Budget: one round, one fix pass, skeptics only for High or above: fine. Opus 5.5 at high, at most 3 agents: fine.
- Classification: `needs-mustafa` (his words); no Brave check. His go confirms his words and marks, the section's
  place, and answers XP-TERM and LOOK if he wants (they may stay open).

## Items for Mustafa (in the report's "Decisions needed", non-blocking)
- XP-TERM (condition 3) and LOOK (condition 5).

## What I checked
- His words: get_event 0f5f8a45... (2026-11-06T23:27:12Z) and 634a1bb3... (23:50:53Z) equal mustafa-words.md byte for
  byte; the VFX block equals a79fef1's note and STATUS:502-520.
- PR #42 comment 6698888516 (gh api, read only): the four Low notes; 0588a17 is #40's SHA they name; at 0588a17,
  #40's STATUS :3, :31, :173 still say #41 waits for his go.
- #42: merged_at 2026-11-06T23:57:35Z, merge 4c6db5b (parents ee736ec, e223ecc). #40 and #43 open drafts.
- EDITOR_V2:1-112 (header :1-8, §5 :92-94, §6 :96-112, §3 `[LATER]` :53-54); VISION_V2:60-74 (trust rule :70,
  OQ-0006-21 pointer :72-73), §6 :75-94; LOOK_V2:60-63; LOOP_V2:116-126, :128-147, :225-250; DESIGN_NOTES:23.
- README:69, :104, :120-125, :160, :170, :181, :183, :202, :294-298, :357 (I-20), :380, :408 (J3).
- ADR 0006:472-518 (part 12); ADR 0005:527-538 (CD-14); ADR 0003:270-281 (5.k, 5.l), :920-925 (OQ-0003-16).
- STATUS:3, :33-47, :105, :476-494, :498-520, :656. Line citations into EDITOR_V2: STATUS:656 (:93, :94),
  LEGACY_INVENTORY:324 (:4, unmoved), r1:3631 and the B.1 report :51 (:39, dated records).
- CI runs on e223ecc (gh api): the step times in condition 10.
- Not checked: ROADMAP's text (the verifier's E7); the gate (no file changed yet; E6 runs it).
