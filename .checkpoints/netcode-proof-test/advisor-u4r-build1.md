# Advisor: rulings on U4r build copy 1's nine points (+ point 10 follow-ups)

Task: rule on u4r-build1-questions.md's nine points; point 10 records four earlier Lows as follow-ups.
Sources: the questions, the builder's notes (u4r-build.md), the snapshot .gate/u4r-snap1 (2fbec77 + wip/u4r.patch),
the logs .gate/u4r/, the brief u4r-brief.md rev 3.2, u4s-carries.md, plan.md / u4-build-plan.md, advisor-u4r-brief.md,
history.md, the fact check u4r-build1-facts.md.

## Plan
1. Read the advisor file from main (done), the questions (done), the notes (done).
2. Read the brief's cited sections: section 0 (:182-211), 2.2 / R16 (:126-130), section 4 lane and rules (:960-1010),
   section 5 item 4 (:1084-1092), r4 (:601-608), carry 62 (h) (:643, :662-663, :893, :1176-1178), span_ms (:407, :930,
   :1254), 7.1 (:1140-1170), U4R-Q2 (:1543-1545), U4R-Q14 (:1724-1770), D13 (:1834-1841).
3. Read the carries 62, 82, 83, 84; advisor-u4r-brief.md (L1-L6); d2-r1 :209-212; m9 :117-127; u4-build-plan :69, :800-802.
4. history.md: Mustafa's decisions to quote (PM-0001-2 floor, review panels, Low never triggers a push, LEFTOVER-DELETE).
5. Check the code in the snapshot for each point (hash.rs legs; room.rs find_relevant; mirror.rs; logs.rs refused_as_lost;
   scenario.rs flags, ROOM_HISTORY_MS; check.mjs wasm step).
6. Rule each point 1-9; point 10; further findings.
7. StructuredOutput.

## Progress (one line per item)
- [done] 1.
