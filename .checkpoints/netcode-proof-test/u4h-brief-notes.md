# U4h brief (rev 1) — netcode-architect notes
(No Edit tool in this run: the notes file is rewritten whole with Write after each block.)

## Plan
1. Read TEST-FIRST rule 3 (history.md:2234-2282) and SIZE-CHECK (history.md:2048-2051).
2. Read the form: u4g-brief.md (rev 2.1), u4f2-brief.md (rev 1.2); advisor-u4g-brief.md (ADV-U4GB-1, -4, U4G-Q1 at :360-374).
3. Read u4-build-plan.md: §5 U4h, §4.9, §4.14, T14, RQ12, :152, :153, HitHistory lines, TC B1 seed 3 lines, lanes.
4. Read plan.md and marks.json rows -7, -48, -49, -50, -51, -54, -56.
5. Read u4s-carries.md U4h section + every carry naming U4h; hunt for unregistered routes to U4h.
6. u4f2-brief §6, u4f-brief build rules by pointer.
7. Crates: what exists, what U4h adds.
8. results/u4f/*/runs.txt: per-run size and time.
9. Size, disk, U4g / U4g2 dependency.
10. Write the brief (200-300 lines), questions, Mustafa paste-ready items.
STATUS: ALL DONE. u4h-brief.md rev 1 written, 296 lines (read at head 33dabe8). Next: the advisor's check.

## Findings (one line each)
- TEST-FIRST rule 3 (history.md:2264-2267). SIZE-CHECK :2048-2051 + reading :2067-2075.
- ADV-U4GB-15: carry 59's pointer to carry 93 IS in (u4s-carries.md:284-285).
- U4h NEEDS from U4g: Interp's published movers + render time, the ClientSettings send + room clamp. Not Predictor/CueLog. Under U4G-Q1 (b) U4h follows U4g2.
- plan.md's -7 line is now :724 (u4-build-plan.md:125, :152 cite :696): Low cite drift.
- JUDGE: four `planned` figures (judge/src/figures.rs:841-860); no judge commit under TEST-FIRST; rule 2 -> U4H-JUDGE (his).
- SERIES.md:86 pong gap sign reversed against ADR 0001:1647 and -50's check -> U4H-Q7.
- Unregistered routes found: N2-L3 (b) (u4d-round2-netcode.md:81-82; advisor-u4d-close.md:305-306); INF-U4R-6 + INF-R2-2 (u4r-report.md:154-157, :283; advisor-u4r-r2-full.md:45); pump_report's RTT (advisor-u4e-r1.md:662-664).
- Already taken: N2-1, EA-U4R-1/-2, RR4-L1 (BRAWL-1 Part 1); npt-server tests (U4f).
- Design questions U4H-Q1..Q12 written (margin; cap/span as run params; no-RTT rule; -50 twin = estimate; -50 scene/bite/ws; -54 types; pong sign; TC aim for false-hit denominator; cover item; det-u4 route; bite sites; JDG-ONLY-1 order).
- Size: whole about 2,800-5,200 (mid ~4,000; 1.5x: 4,200-7,800) -> passes M line; U4H-SIZE (his), lean (b): U4h 2,050-3,650 (stop past 6,000), U4h2 800-1,600 (planned ~1,200, stop past 2,400).
- Disk: ~10.1 GB free after part-clean; check.mjs run fills 6.3-7.1 GiB dev + 0.26 release; floor 4,096 MiB -> part-clean before each counted run; sets' logs ~0.6-2.7 GiB.
