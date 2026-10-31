Task: step 3c owner apply (R2): apply approved consistency resolutions, glossary terms and shared cross-topic OQs to docs/research/r2-netcode.md (prompt: .checkpoints/v2-research/prompts/netcode-architect-r2-consistency.txt)

## Plan (2026-10-30, before edits)
- Inputs read: prompt, plan-amendments-2.md, advisor-resolutions-verdict.md, resolutions.md (all 12; R2 changes: C1.7-C1.14 incl. C1.10b, C5.8-C5.10, C6.5-C6.7, C9.9-C9.11, C10.4-C10.11, C11.8-C11.12, C17.3-C17.4, C23.1-C23.2; NOT C17.b*/C23.b*), GLOSSARY.md, glossary-hits-split.tsv (R2: 20 exact, 193 context), cross-topic-oqs.md (R2: XT-OQ1-OQ15), glossary_check.py.
- Part 7 = "## Verifier spot-check"; add "### Consistency resolutions (2026-10-30)" at its end (before "## Sources").
- Course correction from the session (received mid-task): subsections are "### Shared open questions (the same words in every topic)" (MODES-3, ABL-PREDICT, SEC-SPREAD, BASELINE-WHEN for R2) then "### Cross-topic open questions (consistency critic, 2026-10-29)" (R2-XT-OQ1-15), both before "### Pointers between the R2 units". Not the prompt's "Shared open questions (consistency critic, 2026-10-30)". Type: nit (logged).
- Method: python apply script in scratchpad (whitespace-normalized unique match, idempotent), stage 1 resolutions, stage 2 glossary, stage 3 OQs, stage 4 header/part 7.

## Step 1: resolutions (applied 2026-10-30, script scratchpad r2c/apply1.py; every before quote matched exactly once)
- C1.7 SYNC-REC1 mark 4 (r2:2016-2017): recommendation (the share now has its value, 25 % of the tick period).
- C1.8 SYNC-REC9 mark 1: recommendation (listed).
- C1.9 BUD-REC6 heading and reason (as revised, item g): recommendation (listed).
- C1.10 BUD-REC6 spike (as revised, item h): recommendation (listed).
- C1.10b BUD-REC6 spike, new sentence: recommendation (inside C1.8-C1.11).
- C1.11 INT-REC3 spike: recommendation (listed).
- C1.12 ROOM-REC2 mark 1: nit (same 50 %, term only).
- C1.13 TRN-REC1 mark 2, sentence added: recommendation (how the transport runtime counts).
- C1.14 XU-OQ8 closing sentence: nit.
- C5.8 SYNC-REC3 list item added: recommendation (the client-not-predicted list gains abilities; same scope as before, now explicit).
- C5.9 SYNC-OQ10 closing sentence: nit.
- C6.5 SYNC-REC3 stream name: nit (server-only -> authority-only, offline Worker named; no change online).
- C6.6 SYNC-OQ11 sentence outside the dated bracket: nit.
- C9.9 SYNC-REC5 mark 3: recommendation (listed; 48+12 -> largest mode's N + 15).
- C9.10 INT-REC4 spike: recommendation (worst case -> the stress scene).
- C9.11 BUD-OQ5 closing sentence: nit.
- C10.4, C10.5 worked estimate row labels: nit.
- C10.6 INT-REC3 spike (+18v18v18 under (b)): recommendation.
- C10.7 ROOM-REC6 spike: recommendation.
- C10.8 ROOM-REC7 spike (test-mode list): recommendation.
- C10.9 BUD-OQ6, C10.10 XU-OQ13 closing sentences: nit.
- C11.8 BUD-REC6 spike (one vCPU of the CCX23): nit (unit name only, resolution 11 "nothing here changes a number").
- C11.9 TRN-REC1 mark 2 (1 CPU core -> 1 vCPU): nit (unit name, resolution 11).
- C11.10 ROOM-REC2 reference host: nit.
- C11.11 XU-OQ9 closing sentence: nit.
- C11.12 table E (r2:1721) and pointer (r2:3244), core -> vCPU: nit.
- C17.3 TRN-REC1 mark 5 cross-reference: nit (no policy change, resolution 17 "now").
- C23.1 TRN-REC5 production line, C23.2 TRN-OQ4 closing: nit (no position change). NOT applied: C17.b2, C23.b* (wait for BASELINE-WHEN).
- fmt: C5.8's list item indentation restored to 2 spaces (the regex kept the file's own indent): nit.

## Step 2: glossary terms (applied 2026-10-30, script scratchpad r2c/step2.py; all 39 replacements matched once)
- Links: every "(GLOSSARY" marker of the step-1 texts turned into [GLOSSARY](GLOSSARY.md#anchor) (C1.7, C1.8, C1.9 x2, C1.10, C1.11, C1.12, C1.14, C6.5, C6.6, C9.9, C9.10, C9.11, C10.6, C10.7, C10.8, C11.8, C11.9, C11.10, C11.11, C17.3): nit. C1.13's link moved to the first use one line up (TRN-REC1 mark 2): nit. Some step-1 lines rewrapped (same words): nit.
- Table E (r2:1721-1722): "Whole tick" -> "Server tick time", "of the interval" -> "of the tick period"; evidence cell: "weighed and set aside on 2026-10-30 for the server tick budget's 50 % (R2-BUD-REC6)": nit (option table; no value of a recommendation changes).
- BUD-REC6 heading: "A server CPU budget per tick for the whole tick" -> "A server tick budget for the whole server tick"; reason "25 % for the whole tick" -> "whole server tick": nit (exact phrase "whole tick" retired; C1.9's text otherwise as approved).
- Time statistics ("no statistic means p99"): BUD-REC6 spike encode "<= 50 µs per client per snapshot at p99"; INT-REC1 (c) "0.2 ms per tick at p99"; INT-REC2 (b) "0.1 µs per enemy pair at p99", re-flood "1 ms at p99"; INT-REC4 "0.5 ms per tick at p99": recommendation x4 (BUD-REC6, INT-REC1, INT-REC2, INT-REC4).
- TRN-REC1 mark 2 "the transport layer" -> "the transport runtime": nit. Mark 4 "tick p99" -> "server tick p99": nit. Mark 5 (C17.3 text) "An audit gate" -> "An advisories check ... that gates pull requests", "R6's security baseline" -> "the security baseline": nit (R2's position unchanged).
- TRN-REC5: links for preview and game-server certificate: nit.
- TRN-REC6 mark 2 "Simulation tick duration at p99" -> "Server tick time at p99" (GLOSSARY lists this mark under server tick time): recommendation (scope named: the whole tick with its sends).
- ROOM-REC2 (C11.10 text) "reference host: the reference server" -> "on the reference server"; mark 2 "on the reference host" -> "on the reference server"; mark 4 "tick p99" -> "server tick p99": nit.
- ROOM-OQ1 "tick p99" -> "server tick p99"; ROOM-OQ2 and ROOM-OQ11 "R6's (deferred) security baseline" -> "the security baseline": nit.
- XU-OQ8: lead "The units gave shares of different wholes (their figures before 2026-10-30, in the glossary's terms; settled below)"; "the whole tick ... of the tick interval" -> "the server tick time ... of the tick period"; "room's tick" -> "server tick time"; "the transport layer on at most 1 CPU core" -> "the transport runtime on at most 1 vCPU": nit.
- Pointers to R1: spread "a server-only stream" -> "an authority-only stream since the fix pass": nit. Pointers to R6: "R6's security baseline" -> "The security baseline (R6)", "the dependency audit that keeps ... current" -> "the policy of the advisories check that keeps ... current (...; R5-GATE-REC1 schedules the check)": nit.
- OBJECT (not applied): r2:866 R2-INT-F20 "4-9 % of one core": the legacy JavaScript hasLOS measurement on unrecorded hardware (a browser, so client hardware: GLOSSARY Keeps "Core, for client hardware"); "one vCPU" (of the reference server) would misstate the finding. Left as is; the check script will count 1 exact hit. Proposal: the glossary's measuring-unit entry adds r2:866 under Keeps.
- Left as is (in context, other meaning): bare "48" (R2's design point, VISION_V2 "design for 48+"; resolutions changed the ones that mean the largest mode), "players" (all player counts, none a team size), "always", "characters" (display-name characters), "vehicles" in the worked estimate (its own assumption), "a room's tick" (r2:1900, the tick loop), Source's "server-only seed" (r2:2076, 2893), "dependency audits" (the baseline's subject), "a publicly trusted certificate" (r2:1875 and TRN-REC5 name the per-hostname form).
- Step 2b (scratchpad r2c/step2b.py, 12 links at first uses, all nit): design point (header table row 1), prediction scope (SYNC-F12), start modes (budgets intro "to start"), time statistics (legacy CM4 row "mean"), team size (rooms intro), live projectile (option E2), N (option A1 "N²"), authority (transport option A), reference server (SYNC-REC1 mark 4), big vehicle (SYNC-REC4 Serves), sim step time (BUD-REC6 spike), state hash (TRN-REC4 mark 2). Script first-use false hits left (other meaning): r2:52 "mean" (verb), r2:400 "start time", r2:447 legacy grenade "preview", r2:452 legacy "slots", r2:1524 TF2's N; and two lines where the link sits on the line before (advisories check r2:2596, reference server r2:2283).
- Check after step 2: R2 exact 1 (the OBJECT at r2:866), in context 182.

## Step 3: shared and cross-topic questions (applied 2026-10-30, scratchpad r2c/step3.py, step3b.py)
- Inserted before "### Pointers between the R2 units": "### Shared open questions (the same words in every topic)" with ABL-PREDICT (C5.10), SEC-SPREAD (C6.7), MODES-3 (C10.11), BASELINE-WHEN (C17.4; also serves resolution 23), texts taken programmatically from resolutions.md, GLOSSARY markers turned into links: nit x4 (+1 intro line, nit).
- Then "### Cross-topic open questions (consistency critic, 2026-10-29)" with R2-XT-OQ1..OQ15 (ranks 2, 4, 7, 8, 12, 13, 14, 16, 18, 20, 27, 28, 30, 32, 39), texts from cross-topic-oqs.md (rank 13 keeps "r6:757 at commit 8dee746"; rank 18 carries BASELINE-WHEN): nit x15 (+1 intro line).
- Pointers "*Cross-topic open question(s):* R2-XT-OQk (Open questions)." added to: SYNC-REC3 (7), SYNC-REC4 (2), SYNC-REC6 (3), SYNC-REC7 (13), SYNC-REC8 (1), SYNC-REC9 (7), BUD-REC1 (14), BUD-REC2 (3, 11), BUD-REC4 (5, 14), INT-REC1 (14), INT-REC2 (15), TRN-REC5 (10), ROOM-REC1 (11), ROOM-REC2 (4), ROOM-REC8 (4): nit x15.
- "See also R2-XT-OQk (below)." added to the named R2 questions: BUD-OQ8 (3), ROOM-OQ9 (4), BUD-OQ1 (5), ROOM-OQ2 (9, outside its dated bracket), ROOM-OQ11 (9), SYNC-OQ5 (13), XU-OQ12 (14), INT-OQ1 (15): nit x8. Also the pointer to R1 on trap recovery (12) and the worked estimate (8, 11): nit x2.
- Reclassified (stricter, so the verifier checks them): C11.9 -> recommendation (TRN-REC1 mark 2's cap moves from "1 CPU core" to "1 vCPU", which may be half a core, R6-HOST-F23, though resolution 11 calls it a rename); C6.5 -> recommendation (SYNC-REC3 now names the offline Worker as the stream's holder).

## Step 4: header and part 7 (scratchpad r2c/step4.py)
- Header: line "Glossary and cross-topic resolutions applied 2026-10-30 (see GLOSSARY.md and part 7)" after Status; unit-table row for the shared and XT questions; totals sentence (+19 open questions, 91 in all): nit x3. Status label unchanged.
- Part 7: "### Consistency resolutions (2026-10-30)" at its end, before "## Sources" (applied ids, glossary terms, p99 marks, questions added, the OBJECT, counts, which approved texts carry a later rename): nit.
- Check after all steps (glossary_check.py --skip-records): R2 exact 1 (r2:870 now, the OBJECT), in context 185 (+3 from the approved shared texts: MODES-3 "the goal 24v24, N = 48", BASELINE-WHEN "dependency audits"); header links GLOSSARY.md: True; first uses 25, unlinked 7, all documented false hits (other meaning, or the link on the line before).
- Verbatim check: approved after texts present word for word (links normalized) except C1.9, C1.10, C1.10b, C1.13, C11.9, C11.10, C17.3, C23.1, each differing only by a logged step-2 rename, link or p99.

## Totals
- recommendation 20: C1.7, C1.8, C1.9, C1.10, C1.10b, C1.11, C1.13, C5.8, C6.5, C9.9, C9.10, C10.6, C10.7, C10.8, C11.9; p99 marks BUD-REC6, INT-REC1, INT-REC2, INT-REC4; TRN-REC6 mark 2.
- nit 94 (step 1: 17, step 2: 14, step 2b: 12, step 3: 46, step 4: 4, course-correction heading: 1).
- number 0, source 0. OBJECT 1 (r2:866/870 R2-INT-F20 "one core").
- Not applied: C17.b2, C23.b* (BASELINE-WHEN). Note: C23.2's approved text carries "r6:424-425" without "at commit 8dee746" (applied as approved; the advisor's commit nit covered only the cross-topic texts).

DONE

## Advisor ruling OBJECT-1, applied 2026-10-30 (advisor-object-1-verdict.md: OBJECT upheld, option (b); nit approved)
- R2-INT-F20 (r2:870): "4–9 % of one core" -> "4–9 % of one core of the measuring machine": nit.
- R2-TRN-OQ4 (C23.2's text, r2:3051): "r6:424-425" -> "r6:424-425 at commit 8dee746": nit.
- Part 7 "Consistency resolutions (2026-10-30)": OBJECT line now says upheld by the advisor (option (b)) and names both nits: nit.
- Totals now: recommendation 20, nit 97, number 0, source 0. GLOSSARY.md not touched (engine-architect's change).
DONE

## Step 3d fix round (coordinator message, 2026-10-31; verifier-r2-3d-result.md, verifier-r2-3d-skeptic-result.md)
(scratchpad r2c/fix3d.py, fix3d-p7.py; every replacement matched once; line numbers of f67245e)
- r2:2017 SYNC-REC1 m1 "48 clients" -> "As many clients as the largest mode's N (GLOSSARY link: 48, or 54 under MODES-3 (b); ...)": recommendation.
- r2:2022 SYNC-REC1 m4 -> "the largest mode's N clients (48, or 54 ...)": recommendation.
- r2:2103 SYNC-REC3 m2 "48-player brawl" -> "a brawl of the largest mode's N players (...)": recommendation.
- r2:2186 SYNC-REC5 m4 "48 candidates" -> "the largest mode's N candidates (...)": recommendation.
- r2:2292 SYNC-REC9 m1: keeps "48 clients" (BUD-REC6 quotes it), adds "(the largest mode's N, or 54 clients under MODES-3 (b); ...)": recommendation.
- r2:2319 BUD-REC1 spike -> "at the largest mode's N players (...)": recommendation.
- r2:2338 BUD-REC2 spike -> "at the design point, N = 48 (GLOSSARY link)": nit (the 160 kbps mark rests on the 48-player estimate; value unchanged under (b)).
- r2:2358 BUD-REC3 spike "48-player soak" -> "soak at the largest mode's N (...)": recommendation.
- r2:2439 INT-REC1 (c), r2:2499 INT-REC4, r2:2588 TRN-REC1 m2, r2:2718 TRN-REC5 m2, r2:2773 ROOM-REC2 m1, r2:2777 ROOM-REC2 m3 -> the largest mode's N (48, or 54 ...): recommendation x6.
- r2:2952 SYNC-OQ13, r2:2993 BUD-OQ9 -> the largest mode's N (...): recommendation x2 (open questions; logged so because N changes under (b), as the coordinator's rule says).
- Skeptic: r2:2401 "fit inside the server tick budget's 50 % of the tick period", r2:2402 "25 % of the tick period for the server tick", r2:2404 "a p99 server tick time at half the period": nit x3; r2:2413 "each within its 25 % share of the tick period": recommendation.
- ROOM-REC2 mark 5 "every room start-up ... under 2 s" (rooms draft gives no statistic; a few-room spike has no meaningful p99; a start-up is a wait a player sees): recommendation.
- r2:2289 Serves "(12v12v12 or 24v24; `CLAUDE.md`: design for 48+ players)": nit. r2:3896 part 7 statement corrected (bare 48s; CLAUDE.md's "Design for 48+ players"): nit.
- R2-XU-OQ9: "Before 2026-10-30 (answered below)", past tense: nit.
- Part 7: totals (37 recommendation, 106 nits) and "Step 3d fixes (2026-10-31):" list: nit x2.
- Not changed (outside the list; same slip as r2:2289, older text): "VISION_V2 §3 (48+ players)" r2:2355, "VISION_V2 §3 (48+)" r2:2409, "the 48+ design point of VISION_V2 §3" r2:2627, "the §3 scale (48+ players)" r2:2737, and XU-OQ18's quotes of them.
- Check after: R2 exact 0, in context 199 (new hits are "48" inside "(48, or 54 under MODES-3 (b))" beside the marking words, and "players" in "the largest mode's N players").
- Round totals: recommendation 17, nit 9. File totals: recommendation 37, nit 106, number 0, source 0.
DONE
