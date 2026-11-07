# kit-v3-4 builder (docs parts F and D): plan and progress

Worktree a4, branch kit-v3-4 at e86d70b. Edits only under `docs/`. No commit, no push (the main session does).
Sources: plan-rev1 parts D and F; advisor-plan-rev1 C10, C11; facts §0, §4, §7; mustafa-2026-11-07-answers.md.

## Plan
F1. VISION_V2:72 (the `[DECIDED]` XP rule line): append a bracketed pointer with his XP-TERM (a) words, wrapped onto
    continuation lines (+3 lines). Line 72's own words unchanged; no tag added; header line not touched.
    Then recompute STATUS follow-ups citing VISION_V2 lines: "From B.2" ADR 0005 ranges (§6 moves +3), R1's 100 ms
    item (:58, before the block, unchanged), :489/:496 "VISION_V2:71-75", :536 "VISION_V2:82".
F2. README:183 R2-ROOM-OQ7: the "Which progression ... stay open" clause replaced by the text given (C11).
F3. STATUS: XP-TERM follow-up answered (his words); LOOK follow-up answered ((a) is his word; the option text is the
    report's); B.2's "Not decided by the note" updated with XP-TERM (a), unrented private matches still open;
    STATUS:107 and ROADMAP:94 "four" -> "five" register rows, OQ-0003-15 (README:103) added.
F4. vfx report: "Corrections after the merge (2026-11-07, kit-v3-4)" after For Mustafa, before Appendix A
    (Low notes 1, 3, 4; his answers XP-TERM (a), LOOK (a)).
D1. SPIKE_PLAN foundation entry: one dated note, his item 4 in his words, pointer to item 6 (ADR 0006), ADR 0002
    part 7's per-edit hook has no format check (0002:472-474). No design.
D2. ADR 0006: dated notes quoting item 6 at part 9, part 10, OQ-0006-14, the Rollback "A bad release" bullet.
    No slice mapping of "first playtests".
D3. README: OQ-0006-14 row (:153) "Mustafa (2026-11-04) bears on it" clause; I-18 (:355) clause on ADR 0003's
    slice-6 field data (0003:1245) vs his "from the first playtests on". No new row.
Checks: quote script (joined text), diff --stat docs only, line lengths, line numbers, repo-map-check, gate --no-smoke.

## Progress
- F1 VISION_V2 pointer (+3 lines at :72-75); F3 STATUS ranges/R1/:85 citations, B.2 'Not decided', XP-TERM and LOOK answered, :107 five rows; ROADMAP:94 five rows. Done.
- F2 README:183 R2-ROOM-OQ7 clause replaced (C11 text). Done.
- F4 vfx report corrections section (notes 1, 3, 4 + answers). D1 SPIKE_PLAN foundation note (:439-444). Done.
- D2 ADR 0006 notes: part 9 (:431-433), part 10 (:461-465), OQ-0006-14 (:1026-1029), Rollback 'A bad release' (:1563-1566); STATUS ADR 0006:474-475 -> 482-483. Done.
- D3 README OQ-0006-14 (:153) and I-18 (:355) clauses. Done.
- Checks: builder-quotes.py 15/15 PASS (FAILS 0); diff only docs/ (7 files); no changed line >120 chars except
  table rows (README :153 :183 :355, STATUS :107) and ROADMAP:94; VISION_V2 tags unchanged (13/4/5/3); cited lines
  spot-checked; repo-map-check OK; gate.mjs 69ea88e --no-smoke PASS (+0 -0). Not committed (main session commits).
- Shifted citations fixed: STATUS ADR 0006:474-475 -> 482-483; VISION_V2:82 -> :85; :71-75 -> :71-78 in follow-ups.
  Left as written: STATUS:107's "VISION_V2:71-75" (the record of what PR #43 built).
- Final line numbers: ADR 0006 notes at :431, :461, :1026, :1564; SPIKE_PLAN note :439-443; vfx report section :71-91.
