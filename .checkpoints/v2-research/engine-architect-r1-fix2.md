Task: wave 1 step 4, second fix round for R1 (simulation core): apply the second verifier pass to docs/research/r1-simulation-core.md (engine-architect, 2026-10-28).

## Progress
- started: no prior notes; reading inputs next.
- Read inputs: verifier second-pass result + notes (42 C, 2 W, 1 U, 11 label only), scope, first fix log, topic file sections.
## Plan (decided)
1. DET-F34b wrong -> correct F34's last sentence with the verifier's quote ("Note that two generators with different stream parameters may be closely correlated."); options Q4 Pcg32 row; REC4 fallback clause: drop "stream number per generator" merit + "for the spike", state warning, fallback's stream scheme -> new R1-DET-OQ11 (what decides it).
2. DATA-REC6 wrong (file://) -> drop file:// clause (plain-HTTP LAN stays, F29); correction note quoting verifier's MDN Secure contexts read; OQ11 last sentence corrected; add page to "Pages the reviewers read" list.
3. ECS-F34 unsupported (absence) -> fix rule 1 strict: Sure label = absence (no quote), drop from ECS-REC1 Rests on into a note; mark 5 parenthetical.
4. Minors: ECS-REC1 clone_from "do not" -> inferred; ECS-F35 "derive writes only clone" -> our reading; DET-REC5 mark + DET-REC6 reason "can" + inferred; TICK-REC2 Serves §6.1 -> §3; BND-REC5 "or CORS" cut, Serves: no §6 item.
5. Dates: BND-F35 and fix-step table F3 (ED 27 -> verifier saw 28 Oct). F10 row: page match note.
6. Part 7: rows for BND-F32, ECS-F35, DET-F33, DET-F34, BND-F34, BND-F35 in unit tables; update DET-F28 / REC4-fallback / DATA-F29 / ECS-F34 rows' final state; counts paragraph; new subsection "Second verifier pass (2026-10-28)" with table + final state + counts.
7. Header: status "verified (every recommendation claim)"; OQ totals 76 -> 77 (DET OQ1-OQ11); TL;DR unchanged unless counts.
Edit method: python with exact-match replacements, asserting one match each.
- Batch 1 done: DET-F34 corrected (correlation quote, wrong closing note replaced), options Q4 Pcg32 row, DET-REC4 fallback clause -> candidate + R1-DET-OQ11 (new, after OQ10). Next: batch 2 (DATA-REC6, DATA-OQ11, sources list).
- Batch 2 done: DATA-REC6 file:// clause cut (plain-HTTP LAN kept) + reflow; DATA-OQ11 last sentence corrected (verifier's MDN Secure contexts quote, attributed); BND-F35 ED date note + reflow; fix-step table F3 (date) and F10 (page match) rows; 'Pages the reviewers read' + MDN Secure contexts (title/date only, no URL in verifier notes). Next: batch 3 (ECS-F34/F35, ECS-REC1, DET-REC5/6, TICK-REC2 Serves, BND-REC5).
- Batch 3 done: ECS-F34 Sure label (absence, not quotable; no rec rests on it); ECS-F35 head ('clones field by field') + Sure (derive leaves clone_from default = our reading); options ECS snapshot cell (inferred); ECS-REC1 clone bullet (default clone_from quote, 'does not reuse' inferred; repaired a '- -' wrap slip); ECS-REC1 Rests on (F34 moved to a note); mark 5 parenthetical + reflow. Next: batch 4 (DET-REC5 mark, DET-REC6 reason, TICK-REC2 Serves, BND-REC5).
- Batch 4 done: DET-REC5 mark ('can catch', inferred); DET-REC6 reason ('can expose', inferred); TICK-REC2 Serves §6.1 -> §3; BND-REC5 'or CORS' cut + Serves (no §6 item) + reflow. Next: batch 5 part 7 (rows, counts paragraph, second-pass subsection), then header.
- Batch 5 (part 7) drafting: rows to add (ECS-F35; DET-F33, F34a, F34b; BND-F32, F34, F35; DATA REC6 file://), final-state updates (ECS-F34, DET-F28, REC4 fallback, DATA-F29), counts paragraph, new subsection with A/B/C/D table + rec table + E/F + ranked findings + counts.
