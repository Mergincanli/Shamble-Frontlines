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
