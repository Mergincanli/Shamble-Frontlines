# Art-director review: vfx-xp-notes (docs only), worktree e967196 vs main 4c6db5b

## Plan
1. Compare EDITOR_V2 §7 with Mustafa's words (mustafa-words.md) word for word; flag any added look wording.
2. Read plan-rev1 (LOOK) and advisor-plan-rev1 (LOOK, C5); read STATUS "From vfx-xp-notes" follow-ups.
3. Check LOOK_V2 §6/§7 (esp. :61-63) and ART_DIRECTION for tensions with his comic-flavour list and faction colours.
4. Check VISION_V2 §5 "team colours never change", DESIGN_NOTES, LOOK_V2 faction colours vs "role colour slots".
5. Check the rest of the diff (VISION_V2, ROADMAP, STATUS, report) for any implied look choice.
6. Verdict + numbered findings.

## Progress
- Step 1 done: §7 (docs/EDITOR_V2.md:115-134) matches his ten bullets word for word; only additions are the `[LATER]`
  header line, the `[DECIDED]` tags and the dated header line :7. No look wording added.
- Step 2 done: plan-rev1 (LOOK, risk 1), advisor C5, STATUS:503-519 read.
- Step 3 done: LOOK_V2:60-63, :49-54, :27, :71; ADR 0003:247 (5.h), :278-284 (5.l), :537, :796, :913-919 (OQ-0003-15),
  :1340; README:97, :103, :104, :106. ART_DIRECTION:14, :72, :106-121, :158-172.
- Step 4 done: VISION_V2:66, :82; DESIGN_NOTES (every version); ADR 0005:381, :512-521, :1846-1855; LEGACY_INVENTORY CC3.
- Step 5 done: README and ROADMAP diff; nothing there decides a look item.

## Checked, no finding
- EDITOR_V2:7 and :115-134: his ten lines word for word; only the `[LATER]`/`[DECIDED]` tags, the heading and the
  dated header line are added. No look wording added, nothing moved into LOOK_V2 (unchanged: `git diff 4c6db5b e967196
  -- docs/LOOK_V2.md` is empty), no ADR draft edited.
- Register OQ-0003-16 clause (README:104) quotes him and decides nothing; it does not say which LOOK_V2 items Low drops.
- ART_DIRECTION (legacy Living Comic): halftone dots OFF by default (:14, §5 :106-121) is a legacy screen overlay; the
  legacy build is frozen and his VFX editor is a v2 tool, so nothing there needs handling in this pass.
- LOOK_V2 §6.1 (:50-51, units, faction colours and effects brightest) and §6.4 (:54, muzzle flashes emissive) sit well
  with faction-coloured effects; LOOK_V2 §7 :61 ("Punchy weapons through VFX, sound and feedback") fits his "Whole
  impact moments".

## Verdict: PASS WITH NOTES (1 Medium, 5 Low; nothing Critical or High)
The pass decides no look item: his words stand alone, LOOK_V2 and the drafts are untouched, and the LOOK question
leaves the call with the brief's owners. One open look question his words now bear on is not linked (finding 1).

## Findings
1. **Medium: his "onomatopoeia" bears on the open question OQ-0003-15, which the pass does not link.**
   - Evidence: the register's OQ-0003-15 (`docs/adr/README.md:103`; ADR 0003:913-919) asks keep, change or drop for
     the legacy touches LOOK_V2 does not name, comic words first (CL12, GW3), decided by Mustafa in the look proof test;
     ADR 0003:796 says "Mustafa's call: LOOK_V2 §7 names halftone flashes and line boil, not comic words" and "LOOK_V2
     does not mention onomatopoeia"; ADR 0003:1340 (look checklist 29) shows comic words beside legacy. His `[DECIDED]`
     line EDITOR_V2:132 now names "onomatopoeia". The pass added "bears on it" clauses to four rows (README:104, :122,
     :160, :183) but not :103, and the LOOK follow-up (STATUS:511-515) names only LOOK_V2:63.
   - Why it matters: the look test's checklist would weigh comic words without his newer words beside them, or a
     reader may take EDITOR_V2's `[DECIDED]` as already answering OQ-0003-15 ("keep").
   - Fix: a fifth "Mustafa (2026-11-06) bears on it" clause on README:103, quoting "Comic flavour: ink-outlined smoke,
     impact frames, halftone flashes, onomatopoeia." (EDITOR_V2 §7) and saying it decides nothing: the tool is
     `[LATER]` and whether comic words stay is still his call in the look proof test. Name OQ-0003-15 in the STATUS
     LOOK follow-up and in the report's LOOK entry. No ADR draft edited.
2. **Low: the LOOK question states the tension only as "used sparingly"; the lists differ too, and a no-conflict
   reading is missing.**
   - Evidence: STATUS:511-513 sets his line "against LOOK_V2:63's ... 'used sparingly'". His list has four touches,
     LOOK_V2:63 two; they share only "halftone flashes". "Ink-outlined smoke", "impact frames" and "onomatopoeia" are
     not in LOOK_V2 §7, and LOOK_V2's "ink 'line boil'" is not in his list (no drop is implied by that).
   - The options are right and neutral: (a) §7 stands until his style guide and the look proof test; (b) the brief's
     owners update §7 in their own pass. But one fair reading is absent: his line says what the tool can make, and
     "used sparingly" says how often it is used, so both may stand as written.
   - Fix: in the follow-up and the report entry, state the list difference factually and add that reading under (a)
     (or as (c) "no conflict"); the Advisor's recommendation (a) can stay.
3. **Low: EDITOR_V2 §7 carries `[DECIDED]` comic-flavour words while LOOK_V2:63 is `[DEFAULT]`, with no pointer.**
   - Evidence: EDITOR_V2:132 `[DECIDED]` vs LOOK_V2:63 `[DEFAULT]`. The tags are correct (his "Decided principles",
     and VFX-TAG (a)), but a reader of EDITOR_V2 alone may take §7 as settling the look.
   - Fix (optional, the advisor's call): a bracketed pointer after his style-guide line (EDITOR_V2:133), as B.2 added
     pointers in brackets beside his words, for example "(The look itself is LOOK_V2's; the open look items: STATUS
     follow-up LOOK.)", or leave it to the report and the follow-up.
4. **Low (a note, decide nothing): "role colour slots" sits well with the colour docs; two tensions for the tool's
   design session.**
   - Fit: his "role colour slots" is ADR 0005's "faction colour slots" (AP-3; ADR 0005:512-521; LOOK_V2:71) and
     legacy's role system (LEGACY_INVENTORY CC3: BaseGear, FactionPrimary, HighlightTrim, PadColor; GAME_DESIGN:46).
     VISION_V2:66's "team colours never change" reads as fixed colours; effects that take them keep that rule.
   - Tension (i), per faction vs per team: VISION_V2:82 says "one faction per team" and ADR 0005:381 allows "faction
     per team or 'chosen by host'"; neither says two teams may not share a faction. If they may, effects recoloured
     "per faction" (like units today) would not tell those teams apart.
   - Tension (ii): ADR 0003 5.h (:247) gives "faction colours" and "visual effects" separate material classes, each
     with its own colour gain; a faction-coloured effect sits in both.
   - Fix: one follow-up line (it can join the LOOK follow-up), naming both for the VFX design session and the modes
     design; no doc edit now.
5. **Low (older than this pass, out of scope): VISION_V2:66 cites DESIGN_NOTES for "team colours never change", and
   no version of DESIGN_NOTES has such a line.** Evidence: grep of every commit touching `docs/DESIGN_NOTES.md`; its only
   colour line is :76 (the Soviet primary). Fix: a docs-keeper follow-up to find the line's source or fix the citation.
6. **Low: "ink-outlined smoke" bears on ADR 0003's OQ-0003-7 (the particle technique, not researched).** Outlines on
   soft, transparent particles are a technique question (ADR 0003:275, :522; STATUS:928-929). Fix: mention it in the
   style-guide follow-up (STATUS:514-515) as an input for graphics-engineer when that research runs; no edit now.

## Brave checklist for Mustafa
None: docs only, nothing a player sees.
