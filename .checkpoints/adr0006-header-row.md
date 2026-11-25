# Checkpoint: adr0006-header-row (lane 2, Builder V2)

Branch `claude/optimistic-albattani-mdoonz` (stands in for `cs/adr0006-header-row`; the session may push only this
branch). Base: main 09faefa7f7cd1bca939bd2f98237e65e622d4737 (#66's merge, 2026-11-25). Risk tier 1 (an ADR draft is a
decision doc: his go). No Brave check (nothing a player sees, hears or feels).

## Goal
Carry out his ONE-OFF-HEADER (a), with its two additions, and STEP4-COUNT (b) (2026-11-25, 00:55 Amsterdam; in
`docs/DECISIONS.md`) in ADR 0006, as one small pull request of its own; record his message of 2026-11-25, 11:07:07 UTC
(his go for #66 and his order) by script; correct the status board's #66 row (his words: "The status page's row still
calls #66 a draft: correct that in your next pull request.").

## Steps
1. Merge #66 on his go: done, merge commit 09faefa7f7cd1bca939bd2f98237e65e622d4737 (normal merge, expected head
   d7937de0aad4b86748c7c4574bee5a6921eeb67c; gate, smoke, toolchain green; the advisor agent's APPROVE in this chat;
   the PR Advisor's APPROVE comment 7155309465 naming the SHA). Branch fast-forwarded to 09faefa and pushed.
2. Orientation: done (ADR 0006 part 1 :128-151, the marks -2 :1370, -7 :1375, -9 :1377, "When a mark fails"
   :1404-1418, the hosting checklist :1439-1460, Rollback :1556-1558, "Revisit when" :795-797, the trade-off bullet
   :725-732, the options row :609; GLOSSARY §5 :652-662; his record rules RECORD-FILES, OLD-LINES, REC-OLD).
3. Advisor tool consultation before the plan: done (2026-11-25, after the orientation; its points are folded into
   the plan: judge "no noindex" on production's own address only; pass conditions a separate question; Workers
   paragraph from sources read now; infra-engineer writes notes only; before/after table; one branch; the clean-up go).
4. The plan: `.checkpoints/adr0006-header-row/plan.md` (current step).
5. The advisor agent's plan review (xhigh).
6. infra-engineer's draft text and sources (notes only), placed by the main session.
7. Records: his message by script; the docs pass (lane 2's lines only).
8. Reviews on the final diff: test-engineer (the marks), the verifier (the sources); one round, one fix pass.
9. Report, PR (draft), the advisor agent on the final head, ready, his go.

## Current step
10: the fold. Round 2 done: infra-engineer APPROVE (G1, G2 -> ADR6-WORDING); the verifier PASS WITH NOTES (the
head equals plan revision 3 word for word). Docs pass f18cd11 (docs-keeper; checked by script). Report written to
docs/reports/2026-11-25-adr0006-header-row.md (324 lines; appendix by appendix.py). Next: this save, then the fold
(notes deleted with the report's commit), the advisor agent on the final head, ready, the SHA to him.
Earlier, step 9: review round 2 on what changed (infra-engineer and the verifier, both resumed with their context) and docs-keeper's
pass (STATUS, STATUS_ARCHIVE, ROADMAP, REPO_MAP, SPIKE_PLAN), started about 11:57 UTC; draft PR #67 opened
(https://github.com/Mergincanli/Shamble-Frontlines/pull/67) at a4152ad. Then: the report (draft in `report-draft.md`),
the fold, the advisor agent on the final head, ready, his go.
Round 1 (on 1c45ed7): infra-engineer REVISE (F1 High: "where Pages sets no cap" unsourced; F3-F5 Low; F2 Medium ->
follow-up); test-engineer APPROVE (C1: PASS-WORDS its own entry; F2, F4, F5, F6 Low adopted; F1, F3, F7 follow-ups);
the verifier FAIL on plan conformity only (E7 12 lines vs 8, opener, Q1 per link; E3 bracket). Plan revision 3
(c440de8) -> the advisor agent APPROVE with six conditions (R3-10, R3-11 its words; PASS-WORDS covers E9, E10, E11;
the report's top names Cloudflare's "Start new projects with Workers" and Pages' changed retention; round 2 adds
R3-10, R3-11, S23). Fix pass ab15d6d; gate PASS (--no-smoke); ADR 1694 -> 1719 lines (+23 at the marks table:
PM-0006-2 now :1393, "When a mark fails" :1427, the hosting checklist :1462).
Done before round 1:
- 5: the advisor agent APPROVE with nine conditions on revision 1 (notes `advisor-plan.md`); plan revision 2 folds
  them in (E8 dropped, E5 replaced by E10, E9 and E10 its exact words, the dates fixed: dashboard 2026-11-18, docs
  2026-11-24).
- 6: infra-engineer's research (22 sources, Q1-Q8; notes `infra-engineer-workers.md`): Workers docs silent on
  production's own version links; host noindex on a Preview's workers.dev link only; Pages docs silent on whether
  direct uploads count; Cloudflare's Pages overview: "Start new projects with Workers"; reading does not favour a
  move. Its optional decision WORKERS-NEXT goes to the report's "Decisions needed" (no edit here).
- 7: ADR edits dceb4e2 (E1, E2, E4, E6, E9, E10) and 1c45ed7 (E3, E7; the main session reworded the paragraph for
  plain words, 11 lines); his 11:07:07 UTC message in DECISIONS, 0bead83 (16 lines, 741 bytes, sha256 ae5eb4f6...).
  Gate PASS with --no-smoke (local smoke cannot start: tools' Playwright wants chromium_headless_shell-1194, the
  container has 1243; CI runs smoke). The ADR grew 1694 -> 1712 lines (+16 before the marks table).

## Next steps
- The plan, then the advisor agent's review.
- The questions waiting on him go into `.checkpoints/adr0006-header-row/questions-2026-11-25.md`, pushed, named in
  the message with the commit (his rule of 2026-11-25, 01:35 Amsterdam, item 6).

## Key numbers (from the merged report, Appendix A)
- Production's one-off links 0354374e (round-a, 2026-11-18 17:02) and de1c3e5e (restore-production, 18:13): `noindex`
  on all 7 paths, written by no rule of the production build (the host added it).
- Production's own address: no `noindex` (round-a, restore-production); the planted `noindex` caught there
  (bite-production, run 44366750977: both production links fail on all 7 paths, and on nothing else).
- The build count: "no count shown" in the dashboard before and after the ten uploads, both on 2026-11-18
  (Appendix A :706-757); Cloudflare's docs read 2026-11-24 (:1384-1388) and again 2026-11-25 (Q8): silent.

## Decisions
- His, in DECISIONS: ONE-OFF-HEADER (a) with two additions; STEP4-COUNT (b); his go for #66 (11:07:07 UTC).
- The advisor agent's settlements (no question to him): CHECK-CLOSE by his STEP4-COUNT (b) (E9, E10, tagged as its
  reading); step 2.6's plan starts after this merge (he may name a second branch if he wants it sooner); TR-END's
  condition is met, so the clean-up timing is his (probe off, then archive, then delete).
- For him with the final head: his go (covers E9 and E10, or remove them); his word on step 4 under E10.

## Open findings
- none yet.
