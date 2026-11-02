Task: ADR 0005 final fix round (step 10 reviews), engine-architect frame + Content data + asset pipeline sections; output engine-architect-0005-fixes-final.md
- 11:30 read EA-6, FB-7/11/13, FC-8, PA-11 (GE fixed GPU part 4 at 319d3b6, lines 1583-1588), AD-6 route; infra-engineer drafting 0006 part 12 archive (scratchpad ie-final/fixes_b.py); new route from coordinator: ADR 0004 part 1 pool capacities (b815c85) -> CD-2 reports each pool's worst case
- 11:45 planned FIX set: FB-11(l5), EA-6+pools frame (97-99), pools frame part 1 (123-126), CD-2 pools (286-290), EA-6 CD-8 (388-389), spike item 2 (670), AP-3 (1392, 1401-1402), PA-11 AP-5 (1415), FB-13 (1716), GE-1 follow-ons (1773-1774, 1780, 1952-1953), FB-7 (1782), FC-8 (1819). New route: infra 8fa7173 (archive in records' bucket; OQ-0006-18 unreleased art -> OQ-0005-AP1).
- 11:55 routes received: infra 8fa7173 (EA-6 archive, OQ-0005-AP1 visibility/OQ-0006-18); ADR 0002 RR-16 (CD-3 derives out of shipped builds)
- 12:05 all routes in; drafting generator at scratchpad/ea0005-final/gen.py (BEFOREs by line range from 319d3b6 copy). Extra: Appendix A rows R1-DATA-REC2/REC3/REC6 + counts (REC3 -> changed: adopted 5, changed 6); frame Harder 1778-1780; OQ-0005-AP1 1489-1497 + table row 1861
- 12:15 fixes file written (21 FIX + 6 NOTE); checking preview
- EA-6: FIX CD-8 (388-389) + I-6 line + App. A REC6 row; routed README I-6 row (synthesis).
- FB-7, FB-11, FB-13, FC-8: FIX (one-line corrections).
- PA-11: GE fixed GPU part 4 in 319d3b6; FIX AP-5 points to the ceiling.
- AD-6 routed: FIX AP-3 example list + Need.
- Pools route (ADR 0004 b815c85): FIX CD-2 new bullet, frame part 1, I-1, spike item 2, Harder, App. A REC2; routed test-engineer PM-0005-4/-5 leg; rust-reviewer optional.
- RR-16 route: FIX CD-3, frame part 1, App. A REC3 -> changed (counts adopted 5, changed 6).
- IE-12/UNRELEASED-ART route: FIX OQ-0005-AP1 + list row; routed README row.
- GE-1 follow-ons: FIX Easier, Harder, joined spike item 2.
- 21 FIX + 6 NOTE in engine-architect-0005-fixes-final.md; uniqueness, non-overlap and sequential apply checked on HEAD doc (scratchpad ea0005-final/gen.py, applycheck.py).
DONE
