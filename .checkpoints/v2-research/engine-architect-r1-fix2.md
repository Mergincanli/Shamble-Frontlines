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
- Batch 5 done: part 7 unit-table rows added (ECS-F35; DET-F33, F34a, F34b; BND-F32, F34, F35; DATA 'REC6 file://'), final states updated (ECS-F34, DET-F28, REC4 fallback, DATA-F29), counts paragraph updated (+ rows bullet), new subsection 'Second verifier pass (2026-10-28)' (claims table 33 rows, recs table 7 rows, E/F, ranked findings, counts). Next: batch 6 header (status, OQ totals, DET OQ range) + TL;DR check + consistency checks.
- Batch 6 done: header date/status ('verified (every recommendation claim)'), DET id range OQ1-OQ11, totals 76 -> 77 OQs, TL;DR compressed to 5 lines (was 6; same content). Next: consistency checks (ids referenced exist, open questions count), then change log, DONE.
- Consistency pass done: 77 OQ definitions (ABL 10, BND 9, DATA 12, DET 11, ECS 7, TICK 9, RR 7, XU 12), 37 recs, tables 0 column mismatches; no "await the second pass" left; ECS-F34 wording fixed; rows bullet wording; options Q6 G2 and BND-OQ5 "CORP or CORS" marked our reading (same claim as BND-REC5's minor point).

## Change log by type (one line each)
### recommendation (7)
- recommendation: R1-DET-REC4 Pcg32 fallback: stream number dropped as a merit, the "for the spike" deferral dropped, correlation warning quoted (F34); fallback is a candidate; stream scheme -> R1-DET-OQ11.
- recommendation: R1-DATA-REC6 "file://" cut from the secure-context clause (plain-HTTP LAN kept, F29).
- recommendation: R1-ECS-REC1 reason: "derived and default clone_from do not reuse buffers" now quoted + marked inferred (F35); Rests on: F34 moved to a note (absence).
- recommendation: R1-DET-REC5 pass mark: "any two runs ... also catch any order" -> "can catch", inferred.
- recommendation: R1-DET-REC6 reason: "expose" -> "can expose", inferred.
- recommendation: R1-TICK-REC2 Serves: §6.1 -> §3 (24v24, bots fill empty slots).
- recommendation: R1-BND-REC5: "or CORS" cut (F21 quotes only CORP); Serves states no §6 item names isolation.
### source (2)
- source: R1-DET-F34 now quotes the verifier's words from the Lcg64Xsh32 page ("Note that two generators with different stream parameters may be closely correlated."); the wrong closing note replaced. Same page, same id (correction, not a re-source).
- source: MDN Secure contexts (verifier's read, title and date only) cited in R1-DATA-OQ11's correction and listed under "Pages the reviewers read"; no new finding id, no fetch by me.
### number (2)
- number: header totals 76 -> 77 open questions (+R1-DET-OQ11); DET id range OQ1-OQ11.
- number: part 7 second-pass counts (56 claims: 42 C, 2 W, 1 U, 11 label only; final: 2 W fixed, 1 U fixed, 0 re-sourced, 7 recs changed).
### nit
- nit: R1-ECS-F34 Sure label (absence, no quote can confirm; no rec rests on it); mark 5 parenthetical.
- nit: R1-ECS-F35 head ("clones field by field") + Sure ("derive leaves clone_from at default" = our reading); options ECS snapshot cell marked inferred.
- nit: options DET Q4 Pcg32 row (correlation warning, -> OQ11); options BND Q6 G2 and R1-BND-OQ5 "CORP or CORS" marked our reading.
- nit: R1-DATA-OQ11 last sentence corrected (file:// secure per the verifier's MDN read).
- nit: dates: R1-BND-F35 and fix-step sources row F3 (ED dated 28 Oct at the second pass); row F10 (second pass matched quotes on the page).
- nit: new R1-DET-OQ11 (what decides it stated).
- nit: part 7: 8 unit-table rows added (ECS-F35; DET-F33, F34a, F34b; BND-F32, F34, F35; DATA "REC6 file://"); final states updated (ECS-F34, DET-F28, REC4 fallback, DATA-F29); counts paragraph; subsection "Second verifier pass (2026-10-28)" (claims table, recs table, E/F, ranked findings, counts).
- nit: header date/status -> "verified (every recommendation claim)" + second-pass sentence; TL;DR compressed from 6 to 5 lines (same content, README's limit).

## Not applied (and why)
- None of the verifier's findings left unapplied. Wording note: the verifier proposed the header label "verified (every recommendation claim; second pass 2026-10-28)"; the task's rule 4 prescribes "verified (every recommendation claim)", so the second pass is named in the sentence after it instead.
- Re-sourced claims: none (no new page fetched; every correction uses the verifier's quotes).

## Final header status
verified (every recommendation claim)

DONE
