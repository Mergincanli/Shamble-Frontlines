Task: step 3c owner apply (R6) — apply approved consistency resolutions, glossary terms and shared cross-topic OQs to docs/research/r6-infrastructure.md (prompt: .checkpoints/v2-research/prompts/infra-engineer-r6-consistency.txt)
Started 2026-10-30. Base: working tree clean at aa1ba8f. Target file: docs/research/r6-infrastructure.md (1144 lines before edits).

## Plan (R6 items from resolutions.md as approved)
- Res 1: C1.21 (A-2, r6:512, recommendation), C1.22 (R6-COST-OQ1, r6:850-851, nit)
- Res 10: C10.19, C10.20, C10.21, C10.22 (nits), C10.23 insert MODES-3
- Res 11: no text change; glossary link at R6-HOST-REC2's first "the reference server"
- Res 17: C17.7, C17.8 (nits), C17.9 insert BASELINE-WHEN; C17.b* not R6 / wait
- Res 19: C19.5, C19.6 (recommendation), C19.7, C19.8 (nits), C19.9 insert PREV-DELETE
- Res 21: C21.4 (recommendation), C21.5, C21.6 (nits), C21.7 insert PREVIEW-WALL, C21.8 (recommendation)
- Res 22: C22.5 (recommendation), C22.6 (nit)
- Res 23: C23.3 (nit). C23.b1-b3 WAIT (BASELINE-WHEN), not applied.
- Res 5, 6, 9, 29: no R6 change.
- Shared coded questions -> new "### Shared open questions (the same words in every topic)" before "### Recommendations that hinge on a source not read" (resolutions.md placement).
- XT items R6-XT-OQ1..OQ9 (ranks 8,15,18,20,26,28,33,34,38) -> "### Shared open questions (consistency critic, 2026-10-30)" (heading per task prompt; cross-topic-oqs.md names it "Cross-topic open questions (consistency critic, 2026-10-29)": flag in return) right after it.
- Glossary exact hits: 414 and 724 "the tick budget"; 609 (C19.5).

## Log (type)
- C1.21 applied, A-2 (part 4, R6-COST assumptions): density to measure, not budgeted; 4-rooms row is the case to budget unless R2-ROOM-REC2 marks 1-2 pass. recommendation
- C1.22 applied, R6-COST-OQ1 "Decided by" + tick rate + transport runtime. nit
- C10.19, C10.20 applied, egress table rows (as totals: MODES-3 (a)). nit
- C10.21 applied, egress table closing paragraph (MODES-3 (b) rows). nit
- C10.22 applied, Start scenario heading. nit
- C17.7 applied, R6-SEC-OQ1 heading (BASELINE-WHEN pointer). nit
- C17.8 applied, R6-SEC-OQ1 dependency audits item (R5-GATE-REC1 advisories check). nit
- C19.5 applied, R6-HOST-REC1 heading (Pages first, Workers alternative). recommendation
- C19.6 applied, R6-HOST-REC1 reason (why Pages first). recommendation
- C19.7 applied, R6-HOST-REC1 spike (25 MiB mark on Workers only if used). nit
- C19.8 applied, Start scenario client line (Pages' Free plan, price inferred). nit
- C21.4 applied, R6-HOST-REC1 body (access wall = PREVIEW-WALL; no preview talks to a game server until answered). recommendation
- C21.5 applied, R6-HOST-OQ5 (Shared as PREVIEW-WALL). nit
- C21.6 applied, R6-SEC-OQ1 items met on the way (PREVIEW-WALL). nit
- C21.8 applied, R6-HOST-REC3 (previews do not use the staging server until PREVIEW-WALL). recommendation
- C22.5 applied, R6-HOST-REC1 heading (isolation flag; Mustafa's preview carries production's header set). recommendation
- C22.6 applied, R6-HOST-REC1 spike (isolation flag). nit
- C23.3 applied, R6-HOST-REC4 (BASELINE-WHEN pointer). nit
- C23.b1-b3 NOT applied (wait for BASELINE-WHEN). Checked: every applied "after" text found in the file (chk.py, whitespace-normalized); every "before" matched once in the pre-edit file.
- Coordinator correction received: subsections named per resolutions.md / cross-topic-oqs.md ("### Shared open questions (the same words in every topic)" and "### Cross-topic open questions (consistency critic, 2026-10-29)"); prompt's name not used. nit
- C10.23 inserted MODES-3 in new "### Shared open questions (the same words in every topic)" before "### Recommendations that hinge on a source not read". nit
- C17.9 inserted BASELINE-WHEN (same subsection). nit
- C19.9 inserted PREV-DELETE (same subsection). nit
- C21.7 inserted PREVIEW-WALL (same subsection; with the step-3b addition on unreleased art). nit
- R6-XT-OQ1..OQ9 inserted (ranks 8,15,18,20,26,28,33,34,38; critic's words, verbatim from cross-topic-oqs.md) in "### Cross-topic open questions (consistency critic, 2026-10-29)" right after. nit (9 entries)
- Verified: the four shared texts match resolutions.md word for word (whitespace-normalized).
- Pointers added (a '*Cross-topic open questions:*' line before '*Serves:*'): R6-HOST-REC2 (XT-OQ1), R6-HOST-REC3 (XT-OQ2, OQ4), R6-COST-REC1 (XT-OQ1, OQ7), R6-OBS-REC3 (XT-OQ1, OQ6, OQ8), R6-TEL-REC1 (XT-OQ5). XT-OQ3 names only R6-SEC-OQ1 (an OQ, already points to BASELINE-WHEN) and XT-OQ9 names no R6 recommendation: no pointer. nit (5 lines)
## Glossary step (done)
- Server tick budget (Retire exact "the tick budget"): Q2 table G2 risk cell; R6-OBS-REC2 Serves; also R6-HOST-REC2 Rests on "the per-tick budget" -> "the server tick budget". nit (3)
- Server tick time (in context "tick p99"/"tick time" for the whole tick): Q2 G2 cell, K3, R6-HOST-REC2 spike, R6-OBS-REC1 spike, R6-OBS-REC2 heading ("server tick time histogram"), R6-TEL-REC1 spike -> "server tick p99"/"server tick time". Same meaning (whole room tick), no value change. nit (6)
- Security baseline (in context "the deferred baseline"): option C2 and R6-HOST-REC4 -> "the deferred security baseline". nit (2)
- Client host (in context "Cloudflare" alone): TL;DR -> "Cloudflare Pages (the client host), or Workers static assets as the alternative"; pointer to R5 -> "the client host (Cloudflare Pages)"; R6-COST prices line -> "Cloudflare Workers static hosting R6-HOST-F5" (F5 is the Workers page). nit (3)
- Measuring unit / reference server: pointers to R1 and R2 keep their quotes and add resolution 11's wording beside them. nit (2)
- Links (GLOSSARY.md#anchor) at first uses: preview (header), client host (TL;DR), start/goal modes (TL;DR), security baseline (TL;DR), game-server certificate (TL;DR + REC4 spike; NOT REC4 heading, to keep C23.b1's "before" intact), server tick time + time statistics + server tick budget (Q2 G2), team size (egress table header), design point (egress table), measuring unit (CCX23 4-rooms row), staging game server (M1 row), reference server (R6-HOST-REC2), transport runtime (A-2, R6-COST-OQ1), advisories check (C17.8 text and BASELINE-WHEN), largest mode, stress scene (linked on its own words), test-mode list (MODES-3), access wall, isolation flag, client host (approved texts' "(GLOSSARY" markers converted). nit
- Header: totals 23 -> 36 OQs (23 own + 4 shared + 9 XT); line "Glossary and cross-topic resolutions applied 2026-10-30 (see GLOSSARY.md and part 7)" with a link. nit
- Part 7: "### Consistency resolutions (2026-10-30)" added before "## 8. Sources". nit
- Time-statistics rule: R6's own time marks name a statistic (p99) or are counts/deadlines/single runs; "a room process starts in under 2 s (R2's mark)" restates R2-ROOM-REC2 mark 5 and follows R2's text: no change in R6 (for the verifier). No recommendation change from the rule in R6.
- Check script (glossary_check.py --skip-records): R6 exact 0; in context 88, all judged kept (Cloudflare as product or company name, "48"/"players" for the 24v24 room and R2's proposal, "the goal" for goal modes, product names "Cloudflare Access"/"access policy" in findings, "dependency audits" as the baseline subject, "always" unrelated, "50 %" with a named whole, "characters" per line). First uses not linked: Team size at 565 ("slots" = room slots on a VM, another meaning; linked at the egress table), Time statistics at 383 ("mean" of TL13's stall %, not a time mark; linked at Q2 G2). Script reports "header links GLOSSARY.md: False" because it reads only text before the first "## " and R6's header is the "## 1. Header" section; the link is in that section.
- Part 7 glossary bullet amended to name the R6-COST prices line change. nit
- Checks after all edits: b-only "before" quotes for C23.b1, C23.b2 (parts 1 and 3) and C23.b3 still match once; C23.b2's part 2 ("... rejects a shared key.") no longer matches as written because approved C23.3 replaced its final period with "; when the baseline is researched is BASELINE-WHEN (below)." -> the session must adjust that before-quote when BASELINE-WHEN (b) is applied. repo-map-check OK.
- Observation for the verifier (not changed, rule 1): C19.6's reason now cites R5-PREV-F12, F13, F18 and F14, which R6-HOST-REC1's "Rests on:" line does not list.

## Totals
recommendation 6 (C1.21, C19.5, C19.6, C21.4, C21.8, C22.5); number 0; source 0; nit 51 (13 resolution nits, 4 shared + 9 cross-topic questions, 5 pointers, 16 glossary wording changes, link pass, header, part 7, subsection naming). OBJECT: none.
DONE
