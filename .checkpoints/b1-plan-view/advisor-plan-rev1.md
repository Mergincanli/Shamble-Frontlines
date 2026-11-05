Task: the advisor's review of B.1's plan revision 1 (b1-plan-view, plan-rev1.md at a9e0b72): APPROVE, REVISE or ESCALATE.

Plan: (1) read sources (advisor.md from main, plan, his words, EDITOR_V2, STATUS queue, LOOP_V2 tags, B3 report);
(2) check his words against STATUS and the queue files; (3) check scope, risks, evidence, reviewers; (4) answer P-TAG,
PLACE, OVERLAP, SCALE; (5) check post-#38 fixes; (6) verdict.

## Block 1: sources read
- advisor.md (origin/main), plan-rev1.md, mustafa-words.md: read.
- EDITOR_V2 (66 lines), LOOP_V2 tags (:7-9, five tags incl. [IDEA] since 2026-11-04), STATUS :1-60 and :427-487: read.
DONE block 1.

## Block 2: his words against their sources
- N1: mustafa-words.md matches A3's queue files at 4f268da byte for byte (diff of both passages: identical). STATUS
  :455-487 holds the same words, hard-wrapped with "> " quote marks. Fine.
- N2: the 2026-11-03 source line is " 1. EDITOR_V2: add "Plan view: draw to build". Two views ...". Its first words
  ("EDITOR_V2: add") are his instruction, not the text to record. The plan does not say which span is "his B.1
  paragraph" in EDITOR_V2, nor how E1 matches "byte for byte" once EDITOR_V2's 120-column hard wrap splits a line.
  -> condition (name the span; state E1's normalisation).
DONE block 2.

## Block 3: scope, risks, overlaps (findings)
- N3 (risk 2 misread): his "Built with Editor v2 (slice 5)" names SLICE-ORDER's v2 playable slice 5, "Octopon-3 and
  Editor v2" (ADR 0002:34-35; ADR 0001:1716; STATUS :518), not EDITOR_V2 §6's build slice 5 "Verticality". No
  conflict; the plan's risk 2 must say so, and nothing should be "flagged" against §6.
- N4 (§6 pointer decides): the plan's §6 pointer says "the slices stay the build order of the architecture". That is
  the session's reading of how his tool order meets §6 (his (2) play instantly vs slice 7 "Validation and Test"). Keep
  the pointer neutral: the architecture plan (§6 step 2) maps his order onto the slices, for his approval.
- N5 (play instantly, missed tension): "keep editing while the match runs" meets EDITOR_V2 §2 rule 4 (static merging
  when you press Test) and OQ-0005-CD4 (ADR 0005:670-680: under both options map geometry and navigation data apply at
  the next match; R4-PHYS-REC2 keeps the static world for the match). Not a conflict to settle now (his words don't
  say whether edits reach the running sim), but risk 3 must name both, and per B3's precedent the register row
  (docs/adr/README.md:118) gets a "Mustafa (2026-11-05) bears on it" clause. Risk 3's §5 point is right (the Test
  mode is the editor's; ADR 0002 PM-0002-75 keeps the editor out of the player build).
- N6 (interiors): the plan view's "building-kit house with floors and a roof" and "with floors" sit near §3's
  `[LATER]` "Enterable structures (interiors)". His words do not move interiors; no pointer that claims they do; one
  risk line for the architecture plan. (VISION_V2 §6.2 already names interiors: pre-existing, not B.1's.)
- N7 (SCALE grep): no doc states "1 unit = 1 metre" (my grep of docs/, docs/adr/), but the drafts already work in
  metres (ADR 0005:401 m/s; ADR 0004:261 "a map 2 km across"; ADR 0003:200 and :832 "world units"). The follow-up
  should say it confirms rather than changes them, and name the owners: units in ADR 0004/0005, world-space tiling in
  ADR 0003.
- N8: EDITOR_V2 §5's two lines are DOCS-ALIGN (STATUS :599-600, the Advisor's doc lines): the plan leaves them alone.
  Right.
- N9: P-TAG (a) right: the tags are statuses, and "Principle" and "lower priority" are both Mustafa's (status
  DECIDED); B3 added [IDEA] only because "idea" is a different status. Note the primitives line also holds his
  "Principles:" (plural): it stays inside that Decided line.
- N10: no new scope otherwise; no engine feature added by the session (his decisions are design inputs; the trace rule
  counts EDITOR_V2 as a need source, docs/adr/README.md:294-297, J3).
DONE block 3.

## Block 4: order, evidence, other docs (findings)
- N11 (his order): the plan moves his eighth line, the Principle, to the top of the doc, out of his order. Keep all
  eight lines in his order in the dated block (Principle last); the top gets a one-line pointer, not a copy (E1 "once").
- N12 (dates): the plan view (2026-11-03) as a sub-item inside the "Decided 2026-11-05" block would carry the wrong
  date. It needs its own date in the item.
- N13 (line refs): STATUS :599 cites `docs/EDITOR_V2.md:51` and `:52` (DOCS-ALIGN, §5); inserting the block in §3
  shifts them. research/r1-simulation-core.md:3631 cites EDITOR_V2.md:39 (a dated research record). Section numbers
  §1-§6 are cited 148 times in docs: no renumbering.
- N14 (E2 by physical line is noisy): a pointer added to a wrapped bullet re-wraps it. E2 should compare per bullet,
  after joining wrapped lines.
- N15 (E3): today's raw counts in EDITOR_V2: [DECIDED] 2, [DEFAULT] 4, [OPEN] 1, [LATER] 2, [IDEA] 0. The expected
  counts after should be written before the build.
- N16 (B.2 shares the quote): STATUS :455-470's quote holds B.1 and B.2 and his two lead lines. Removing B.1 must keep
  B.2 and the lead lines word for word (E1 covers them).
- N17 (#38 facts, checked with the GitHub API): merged_at 2026-11-05T21:25:34Z, merge commit af8393e (parents 12d7961,
  8b7cfb3), head 8b7cfb3; check runs on af8393e: gate success, smoke success; comment 6669362242 by Mergincanli,
  21:00:35Z, "APPROVE, head 8b7cfb3...". Its Low findings: "No automated review of 0c481e1 arrived, and the report
  doesn't say so"; "The report's E3 count of 176 files is the first scan's; the final scans covered 178 and 193".
  The plan's two A3-report notes match. STATUS :744 and ROADMAP :93 still say "a draft": the plan's fixes are right.
- N18 (dangling paths): STATUS cites `.checkpoints/kit-v3-3/` 18 times; those notes were deleted at 8b7cfb3 and live
  at its parent 6c6c91a (264 files). One note line in STATUS, not 18 rewrites.
- N19 (review budget): plan §6 "Below High it becomes a STATUS follow-up" would leave a Medium misquote in the doc.
  The budget limits review rounds, not fixes: the one fix pass fixes in-scope findings at any severity.
- N20: reviewers fit (verifier; game-designer for the balance tools; no engine-architect, as B3's condition 8). Agents
  and model fit. No Night queue entry (day work, as B3). Mustafa's reconfirmation, no Brave check: right.
- N21: no ESCALATE: nothing here needs his answer now; the CD4 and rule-4 tension is for the architecture plan and
  OQ-0005-CD4's deciders later.
DONE block 4.

## Block 5: the verdict

**Verdict: APPROVE, with 14 conditions.** The plan is the right size: docs only, his words copied exactly (checked
against A3's queue files at 4f268da: identical), no new scope, and fitting evidence and reviewers. The conditions fix
four things: the order of his lines, two misread or incomplete risks, how E1 and E2 match wrapped text, and two
line-reference problems the merge of #38 and this edit cause.

### Conditions
1. **His order.** All eight lines of his 2026-11-05 addition go in the dated §3 block, in his order, the Principle
   last. Under the intro paragraph goes only a one-line pointer ("Principle (Mustafa, 2026-11-05): §3, Decided
   2026-11-05"), not a second copy of his words, so E1's "exactly once" holds.
2. **The plan view's own date and span.** The plan view item may sit right after the views line, but it carries its
   own date ("Mustafa, 2026-11-03") so the 2026-11-05 heading does not claim it. The recorded text is his title "Plan
   view: draw to build" plus "Two views of ... no code now." whole. His instruction words ("1. EDITOR_V2: add") are
   left out and listed as the one deviation in the report, as B3 listed its own.
3. **E1 states its matching rule.** Before comparing: join hard-wrapped lines, strip list markers, `> ` quote marks
   and tags, and collapse runs of whitespace; nothing else. Pieces: the plan view text and his eight lines. Each must
   be found exactly once in EDITOR_V2. In STATUS, B.2's words and his two lead lines ("Small change-sets, one at a
   time, plan first (Mustafa, 2026-11-03)." and "B. Docs:") must still be found exactly once after B.1's entry is
   removed. The report gets an appendix with his words copied exactly (as B3's), and E1 runs on it too.
4. **E2 compares bullets, not lines.** Every bullet that already exists is its old text, plus at most one pointer in
   brackets at its end. Headings and the section numbers §1-§6 are unchanged: other docs cite EDITOR_V2's sections
   about 148 times.
5. **E3's expected counts are written before the build.** Today: `[DECIDED]` 2, `[DEFAULT]` 4, `[OPEN]` 1, `[LATER]` 2,
   `[IDEA]` 0. After: `[DECIDED]` rises by the number of tagged items the build adds (state the number first), and
   every other count stays the same. No new tag.
6. **Risk 2, rewritten.** His "(slice 5)" means SLICE-ORDER's v2 playable slice 5, "Octopon-3 and Editor v2" (ADR
   0002:34-35, ADR 0001:1716, STATUS :518). It does not mean EDITOR_V2 §6's build slice 5, "Verticality". There is no
   conflict and nothing to flag.
7. **The §6 pointer stays neutral.** For example: "Mustafa's tool order (Decided 2026-11-05) ranks the tools; the
   architecture plan (step 2) maps it onto these slices, for his approval." Not "the slices stay the build order":
   that reading would settle his (2) "play instantly" against slice 7, "Validation and Test".
8. **Risk 3 and OVERLAP: "keep editing while the match runs".**
   - Add §2 rule 4 ("Static merging happens only when you press Test") to the pointer list.
   - Risk 3 also names OQ-0005-CD4: under both of its options, map geometry and navigation data apply at the next
     match (ADR 0005:670-680), and R4-PHYS-REC2 keeps the static world for the match.
   - As in B3, the register's OQ-0005-CD4 row (`docs/adr/README.md:118`) gains one clause, "Mustafa (2026-11-05)
     bears on it:", followed by his clause. It chooses no option, and no ADR draft is edited.
   - Risk 3's §5 point stands: Test is the editor's own mode, and ADR 0002 PM-0002-75 keeps the editor out of the
     player build.
9. **Interiors.** One risk line: the plan view's "building-kit house with floors and a roof" does not move §3's
   `[LATER]` "Enterable structures (interiors)". No pointer on it.
10. **The SCALE follow-up says what the drafts already do.** No doc states "1 unit = 1 metre", but the drafts already
    work in metres (ADR 0005:401 metres per second; ADR 0004:261 "a map 2 km across"). So the line confirms the drafts
    and changes nothing in them. It names the owners: ADR 0004 and ADR 0005 for the unit, ADR 0003 for textures that
    tile in world space. The same follow-up line names condition 8's OQ-0005-CD4 point.
11. **Line references.**
    - STATUS :599 cites `docs/EDITOR_V2.md:51` and `:52`; update them to the lines' new numbers.
    - `docs/research/r1-simulation-core.md:3631` (EDITOR_V2.md:39) is a dated research record: leave it, and say in
      the report whether it still hits.
    - One STATUS line says that A3's notes, cited at `.checkpoints/kit-v3-3/` (18 references), were deleted in
      8b7cfb3 and can be read at 6c6c91a (`git show 6c6c91a:<path>`). The 18 lines themselves are not rewritten.
12. **The A3 report.** The two Low notes are added as a dated note, "Corrections after the merge (2026-11-05, B.1)".
    The merged lines keep their original words.
13. **The review budget means fewer rounds, not fewer fixes.**
    - There is one review (the verifier and game-designer), then one fix pass. That pass fixes every finding inside
      B.1's scope, whatever its severity; findings outside the scope become STATUS follow-ups.
    - After the fix pass, E1-E3 run again. That rerun is evidence, not a review round.
    - A second review round happens only for an upheld Critical or High finding.
    - Any E1 or E2 failure is fixed whatever its severity: it is this change-set's acceptance test.
14. **The header.** Add one dated line, "Updated on 2026-11-05 with Mustafa's editor decisions", as LOOP_V2:5 has for
    B3. The tag line stays unchanged (P-TAG (a)). B.1 gets no Night queue entry (it is day work). The merge needs his
    reconfirmation; there is no Brave check.

### The four questions
- **P-TAG: (a).** Tag each item `[DECIDED]`, and keep his "Principle:" and "Decided, lower priority" in his own words.
  The tags record status. Both marks are statuses Mustafa decided: "Principle" is a kind of rule, and "lower priority"
  is a priority, so neither is a new status. B3 added `[IDEA]` only because "idea" is a different status (not
  decided). The "Principles:" inside his primitives line stays inside that Decided line.
- **PLACE: a dated block inside §3.** It is a bullet group, not a new numbered section. It holds his eight lines in his
  order, plus the plan view with its own date (conditions 1 and 2). The top of the doc gets only a pointer to the
  Principle.
- **OVERLAP: pointers only.** The old words stay as they are. Pointers go on "Select and transform" (the numeric
  inspector; it may also name "primitives snap to gameplay sizes" next to the grid snapping), "Test", the "Validation
  panel", §2 rule 4 (condition 8) and the `[LATER]` "a prefab library v2". The §6 pointer stays neutral (condition 7).
  There is no pointer on interiors (condition 9).
- **SCALE: as the plan recommends.** EDITOR_V2 records his words, and a STATUS follow-up carries them to the drafts'
  next revision, with condition 10's content. No ADR draft is edited in B.1.

### Checked, and not checked
- Checked:
  - His words against A3's queue files at 4f268da (diff: identical).
  - #38's merge facts and CI on af8393e (GitHub API: gate success, smoke success).
  - The advisor's Low notes (comment 6669362242).
  - SLICE-ORDER's slice 5, OQ-0005-CD4, the trace rule (`docs/adr/README.md:294-297`), DOCS-ALIGN (STATUS :599-600),
    and the line references into EDITOR_V2.
- Not checked: the gate. It has nothing to run until the build exists, and I run it at the final-head review.

## Decisions needed
None. Nothing in B.1 needs Mustafa's answer now. "Keep editing while the match runs" against the drafts'
next-match proposal (OQ-0005-CD4) belongs to Mustafa and the Advisor when that question is decided. It is recorded,
not asked.

## For Mustafa
Nothing now. Later, once the advisor has approved B.1's final head with CI green: reconfirm that your words and marks
sit where you meant them, then say "merge #39" in a session's chat.

DONE
