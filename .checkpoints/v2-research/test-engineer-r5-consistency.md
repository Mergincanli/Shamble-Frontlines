Task: step 3c owner apply for R5 (docs/research/r5-toolchain-quality.md): approved contradiction resolutions, glossary terms, shared cross-topic OQs (prompt test-engineer-r5-consistency.txt, 2026-10-30)
Original file copy (before edits): scratchpad r5c/r5-before.md (line refs below are of the original, commit 8948522 state = 8dee746 content).
Heading note: resolution shared questions go under "### Shared open questions (the same words in every topic)" (resolutions.md); the 14 XT items under the prompt's heading "### Shared open questions (consistency critic, 2026-10-30)" (prompt step 3; cross-topic-oqs.md names "Cross-topic open questions (consistency critic, 2026-10-29)": prompt wins, logged).
## Step 1: resolutions (all 14 before-quotes matched exactly once; none OBJECT)
- C1.19 R5-TEST-REC5 r5:1472-1473 applied · nit
- C1.20 R5-TEST-REC5 spike r5:1481 applied · nit
- C10.17 R5-TEST-REC5 r5:1471 applied · nit (interim wording, MODES-3)
- C17.5 R5-GATE-OQ4 r5:1855-1856 applied · nit
- C19.1 R5-PREV-REC2 heading r5:1599-1600 applied · recommendation
- C19.2 R5-PREV-OQ2 r5:1831 applied · nit
- C19.3 R5-PREV-OQ4 r5:1836-1837 applied (REVISE text) · nit
- C21.1 R5-PREV-REC5 r5:1633-1634 applied (REVISE text) · recommendation
- C21.2 R5-PREV-OQ1 r5:1829 applied (REVISE text) · nit
- C22.1 R5-PREV-REC4 r5:1616-1620 applied (carries rank-41 fix: R1-BND-REC2 mark 5) · recommendation
- C22.2 R5-PREV-REC4 r5:1620-1621 applied · nit
- C22.3 R5-PREV-REC4 marks 2,3 r5:1626-1628 applied · recommendation
- C22.4 R5-XQ-OQ3 r5:1885-1887 applied · nit
- C29.4 R5-RUSTLINT-OQ5 r5:1778-1780 applied · nit
- rank 41 nit: applied through C22.1 · nit
- Not applied (wait for BASELINE-WHEN): C17.b3. C9: no R5 change. C11: no R5 change.
- Shared question inserts pending: C10.18 MODES-3, C17.6 BASELINE-WHEN, C19.4 PREV-DELETE, C21.3 PREVIEW-WALL
## Step 2: glossary renames (TSV: 0 exact hits in R5; 52 in-context hits judged)
- g1 R5-LAYOUT-REC5 Serves (r5:1225) "tick time" -> "sim step time" (Sim step time / client's sim share) · nit
- g2 R5-RUSTLINT-REC4 spike (r5:1359) "the tick cost of the checks" -> "the checks' cost in sim step time" (Sim step time) · nit
- g3 R5-TEST-REC1 benchmark row (r5:1396) "tick and frame cost" -> "sim step time and frame cost"; "REC5's tick p99" -> "REC5's server tick time p99" (Sim step time; Server tick time) · nit
- g4 R5-PREV-REC4 heading (r5:1615) "a preview-only header set" -> "the preview header set" (C22.1 carries the content change) · nit
- g5 R5-PREV-REC4 reason (r5:1622) "Cloudflare's `_headers`" -> "Cloudflare Pages' `_headers`" (F18 is the Pages page; The client host) · nit
- g6 R5-TSLINT-OQ5 (r5:1761) "R6's security baseline" -> "the security baseline" · nit
- g7 R5-GATE-OQ4 (r5:1855) "R6's security baseline" -> "the security baseline" (after C17.5) · nit
- g8 pointers to R6 (r5:1952) "For R6's security baseline" -> "For the security baseline (R6; ...)" · nit
- g9 R5-PREV-OQ3 (r5:1834) "an EU server" -> "the staging game server" · nit
- Kept (judged not carrying the term's meaning): r5:36 characters (text length); 522 N players (player count); 524/1469 48 (scale step; design point 48+); 822-886, 1117, 1123-1124, 1599, 1631, 1829 Cloudflare (product/account/URL names); 850, 1124, 1631 Cloudflare Access (product); 858 Deploy Preview (Netlify product in its finding); 985, 1855 "cargo-audit and cargo-deny as CI gates" (names R1-RR-OQ7's proposal); 1085, 1565 always (not isolation); 1702 server-only (a feature on a dependency); 1879 "always, or behind a flag?" (the question's options, now settled in its own entry); 1953 dependency audits (the baseline's subject); 2144, 2213-2219, 2269 Sources tables (row numbers, URLs). 1478 "48-bot room" (design point; not flagged; kept). 1609 "preview deployments" (host deployments, not the link).
- Time statistics rule: no R5 time mark is a tick-time mark without a statistic (R5 not in the entry's list); no change.
## Step 3: shared questions inserted (before "### Recommendations that hinge on a source not read")
- C10.18 MODES-3 inserted (word for word) · nit
- C17.6 BASELINE-WHEN inserted · nit
- C19.4 PREV-DELETE inserted · nit
- C21.3 PREVIEW-WALL inserted (with the advisor's added clause) · nit
- R5-XT-OQ1..OQ14 inserted (ranks 3, 4, 8, 15, 18, 20, 24, 25, 31, 32, 33, 34, 36, 40), critic's words, under "### Shared open questions (consistency critic, 2026-10-30)" · nit each (14)
- pending: pointers from named recommendations; GLOSSARY links; header; part 7
- Session correction (mid-task): XT subsection renamed to "### Cross-topic open questions (consistency critic, 2026-10-29)", right after the shared subsection, as cross-topic-oqs.md says · nit
- Pointers "Cross-topic open questions: ... (below)." added at the end of R5-TEST-REC1 (XT-OQ2, OQ13), R5-LAYOUT-REC2 (OQ3, OQ7), R5-LAYOUT-REC3 (OQ9), R5-LAYOUT-REC5 (OQ8), R5-TEST-REC4 (OQ10), R5-CI-REC1 (OQ14), R5-PREV-REC1 (OQ6, OQ12), R5-PREV-REC4 (OQ5), R5-GATE-REC1 (OQ7), R5-GATE-REC3 (OQ12) · nit each (10). XT-OQ1, OQ4, OQ11 name no R5 recommendation: no pointer.
## Step 2b: links
- 15 "(GLOSSARY...)" markers from the approved texts turned into links to their entries (GLOSSARY.md#anchor; anchors checked against the headings) · nit (1 unit)
- 11 first-use links by wrapping the term itself (words unchanged): preview (r5:27 table), N (TL15), Cloudflare Pages (R5-PREV-F10), the advisories check (R5-GATE-F3), sim step time (R5-LAYOUT-REC5), 48+ (R5-TEST-REC1 table), server tick time and p99 (benchmark row), the access wall (R5-PREV-REC3 spike), the staging game server (R5-PREV-REC5), the security baseline (R5-TSLINT-OQ5), goal (MODES-3 text) · nit each (11). Script's remaining "no" first uses are not term uses: r5:1170 "Start with", r5:531 "sprint mean".
## Step 4: header and part 7
- Header line "Glossary and cross-topic resolutions applied 2026-10-30 (see GLOSSARY.md and part 7)" with link · nit
- Totals: 45 -> 63 open questions (shared 4, cross-topic 14) · nit (count)
- Part 7 subsection "### Consistency resolutions (2026-10-30)" at the end of "## Verifier spot-check" · nit
- Pointer wording fix (CI-REC1) and XT intro wording · nit
## Check after edits: glossary_check.py --skip-records: R5 exact 0, in context 45 (new ones are inside approved word-for-word texts or kept as judged); header links GLOSSARY.md: True. repo-map-check OK.
## Counts: recommendation 4 (C19.1, C21.1, C22.1, C22.3); number 0; source 0; nit 64 (10 resolution texts, rank-41 via C22.1, 4 shared inserts, 14 XT inserts, 10 pointers, 9 renames, 1 marker-link unit, 11 first-use links, header line, totals, part 7, heading correction, wording fix). OBJECT: none. Not applied: C17.b3 (waits for BASELINE-WHEN).
DONE
## Step 3d fixes (coordinator, 2026-10-30; lines of f67245e)
- 1 r5:401-402 wave-1 list: added "; since 2026-10-30 the mark is the hash-map ban" (linked) · nit
- 2 r5:1477 R5-TEST-REC5: "items 1 and 4 of the test-mode list:" (linked) before the list · nit
- 3a r5:1561 R5-CI-REC5: "R1-ECS-REC2's no-HashMap-iteration check" -> "the check of R1-ECS-REC2 mark 2, the hash-map ban" · nit
- 3b r5:2087 pointer: "(no HashMap iteration in the sim)" -> "(the hash-map ban in the sim crate)" · nit
- 4 r5:58 TL;DR "48-bot soaks" -> "bot soaks up to the largest mode's N" · nit (value already changed in REC5 by C10.17)
- 4 r5:1429 R5-TEST-REC2 spike "10-minute 48-bot match" -> "10-minute bot match at the largest mode's N (48, or 54 under MODES-3 (b))" · recommendation
- 4 r5:1487 R5-TEST-REC5 spike "30-minute 48-bot room" -> "30-minute room of bots at the largest mode's N (48, or 54 under MODES-3 (b))" · recommendation
- 4 r5:1426 "§3 (the 48-player scale)" kept: the design point of VISION_V2 §3, not the largest mode
- part 7: "Step 3d fixes (2026-10-30):" lines added · nit
DONE (step 3d fixes)
- final-check nit: r5:1426 "§3 (the 48-player scale)" -> "§3 (the design point (48 players))", linked; part-7 line updated · nit. DONE
