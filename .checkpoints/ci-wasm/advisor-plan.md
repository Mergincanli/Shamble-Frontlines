Task: advisor plan review of the CI-WASM pull request (part "plan"), plan.md revision 1. Verdict: APPROVE / REVISE / ESCALATE with conditions.

## Plan (blocks, in order)
B1. Read plan.md (revision 1) whole.
B2. Read the card .checkpoints/ci-wasm.md.
B3. Read brief-context.md.
B4. His words: docs/DECISIONS.md entries (2026-11-18 22:34:37 CI-WASM (a), TOOL-SOURCE (a); 20:20:09 WRITE-MODE, REC-OLD; 2026-11-19 13:36:37; GATE-FOLLOW (b)) + the mustafa-*.md files.
B5. The CI text: ci-edit.diff, proposed toolchain.yml, ci-plant-p-gh.diff, main's toolchain.yml.
B6. infra-engineer-ci-plan.md (block summaries, End, parts a plan claim rests on).
B7. test-engineer-followups.md (block summaries, End, parts a plan claim rests on).
B8. decisions-3.mjs + browser-check-record.mjs + scratch outputs (lead texts vs messages).
B9. docs/reports/2026-11-18-kit-hold-ci.md (how text, stop rule, plant were handled).
B10. Checks: advisor.md unchanged on branch; PYC-AGAIN (git log, .git/info/exclude); repo state.
B11. Judgement: the 8 questions, conditions, findings, six questions.
B12. Answer written; DONE.

## Notes
- B1 done (plan.md rev 1 read, 290 lines). To verify later: quotes "stays with the CI pull request" (16:08:04), "My browser reports are recorded with the next docs pass", "is accepted as you wrote it down, with your lesson" (13:36:37), CI-WASM (a) quote, "Until the CI-WASM pull request is merged...", "The game has to work in Chrome, ... Firefox", PYCACHE "With change-set 3"; his size rule (planned size + stop line); filename times (1516, 1638) vs plan's 14:28:25 / 16:08:04. Next: B2 card.
- B2 done (card read). Notes: card says "this chat is Fable 5.2" as main model; CLAUDE.md says main model stays Opus 5.5 (Fable only via advisor tool) -> grep DECISIONS for a later word of his on Fable (item 8). Card size rule: "at twice the plan, stop and tell him in three lines" (source? grep). Card's VISION note text at lines 137-143 = the text to compare with decisions/VISION script. Filenames 1516/1638 vs 14:28:25/16:08:04: check headers. Next: B3 brief-context.
- B3 done (brief-context read). Notes: smoke job already downloads Playwright 1.56.1's Chromium (precedent, legacy tools/); brief quotes 13:36:37 "The game has to work in Chrome, the Chromium-based browsers and Firefox. Safari and Mac come later." (check). Next: B4 his words.
- B4 done (his words). Verified exact in DECISIONS/main or script files: CI-WASM (a) quote; TOOL-SOURCE "(a), for this one file only: <file>... by the route of your plan's section 7, item 5a. ... every other file, version or checksum comes back to me."; 13:36:37 "The game has to work in Chrome, the Chromium-based browsers and Firefox. Safari and Mac come later." and "is accepted as you wrote it down, with your lesson"; WRITE-MODE "read the chat's mode right before each kit write and stop unless it is Accept edits"; REC-OLD "(a), with one limit ... never reworded: it gets a dated pointer"; GATE-FOLLOW (b),(i) (2026-11-17 19:10:44, DECISIONS:515) "You ask me for each gate and CI edit with its exact text; for those edits I set my words of 2026-11-12 (... CI work stays with Opus 5.5.) aside" -> Fable main model OK for CI work (card's Fable line is his: DECISIONS:537, :782). go-61 (14:28:25): "My browser reports are recorded with the next docs pass, not by a push to this head."; "Until the CI-WASM pull request is merged, those tests ran in your container only."; "With change-set 3, one line in .gitignore for __pycache__/, so that a save cannot pick up compiled Python files again." 16:08:04 notes verified. Filenames use HIS labels (15:16, 16:38 Amsterdam) though transcript times are 14:28:25/16:08:04 UTC (=15:28/17:08 Amsterdam): entries must quote label as label, not as time. No personal paths in the four reports. Note: TOOL-SOURCE names change-set 2's route 5a; the CI route is new -> CI-TEXT's question should say its yes covers the CI route (check 5a in B5/B6). Next: B5 CI text.
