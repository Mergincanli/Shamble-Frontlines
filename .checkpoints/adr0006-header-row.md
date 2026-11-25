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
4: writing the plan.

## Next steps
- The plan, then the advisor agent's review.
- The questions waiting on him go into `.checkpoints/adr0006-header-row/questions-2026-11-25.md`, pushed, named in
  the message with the commit (his rule of 2026-11-25, 01:35 Amsterdam, item 6).

## Key numbers (from the merged report, Appendix A)
- Production's one-off links 0354374e (round-a, 2026-11-18 17:02) and de1c3e5e (restore-production, 18:13): `noindex`
  on all 7 paths, written by no rule of the production build (the host added it).
- Production's own address: no `noindex` (round-a, restore-production); the planted `noindex` caught there
  (bite-production, run 44366750977: both production links fail on all 7 paths, and on nothing else).
- The build count: "no count shown" before and after the ten uploads (2026-11-18), and again on 2026-11-24.

## Decisions
- His, in DECISIONS: ONE-OFF-HEADER (a) with two additions; STEP4-COUNT (b); his go for #66 (11:07:07 UTC).
- Open for him (asked in this turn's message): where step 2.6's plan is written while the revision waits; the
  clean-up order (probe off first; the archive and the Cloudflare deletions after his go on the revised row).

## Open findings
- none yet.
