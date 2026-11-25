# advisor agent: plan review of adr0006-header-row, revision 1 (2026-11-25)

Reviewed: `.checkpoints/adr0006-header-row/plan.md` at branch claude/optimistic-albattani-mdoonz cbd5ada (main 09faefa).
Verdict: APPROVE with conditions (below). Risk tier 1 (ADR draft = decision doc; DECISIONS records his words): needs-mustafa.

## Checked against the files (main 09faefa)
- ADR 0006 :4 Date line; part 1 :128-151 (build count :140-143); PM-0006-2 :1370 (bites "a `noindex` rule added to
  production"); PM-0006-7 :1375 ("passes when both are recorded"); PM-0006-9 :1377; "When a mark fails" :1404-1418
  (header fail on Pages -> "Workers static assets weighed again"; author infra-engineer); checklist :1439-1460 (intro:
  "He records "achieved" or "not yet" (with a note) for each step, or the numbers it asks for"; step 4 "Expected: two
  numbers recorded"); spike 1 list :1214; Rollback :1556-1558; Revisit :795-797; trade-off :725-732; options row :609.
- Report :278-301: ONE-OFF-HEADER (a) text matches plan :19-21; STEP4-COUNT (a) "both stay open until a count source
  exists", (b) "both go back to the draft with the header row: the question is re-stated as one the dashboard cannot
  answer ...", (c) "he gives step 4 a verdict now". "Both" = step 4 and the build-count mark (:293-296).
- Appendix A: round-a :667-668 (0354374e fail on all 7 paths, noindex, "production's build writes no noindex, so the
  host added it"; all other headers as expected); bite-production :805-811 (both production links fail on all 7 paths,
  own address included, nothing else); restore-production :813-818 (own address passes; de1c3e5e fails 7 paths). E1/E2
  numbers hold.
- Build-count readings: both dashboard readings 2026-11-18 (:706, :750). 2026-11-24: Cloudflare's docs only (:1384);
  his billing message (:1367) is about paid items, not a build count. => E4 "or on 2026-11-24" and E5 "Read on
  2026-11-18 and 2026-11-24: no count shown" are wrong as written (would be High at head).
- DECISIONS :176-177 his ONE-OFF-HEADER/STEP4-COUNT words; OLD-LINES :1256; REC-OLD :1085; RECORD-FILES :1311;
  TR-END :1596 ("I archive the repository, and delete the project after the report is merged.").
- GLOSSARY §5 :652-662: production set = what the build ships; host header outside it. Leaving it alone is fine.
- ADR Date lines: all six ADRs 2026-10-31; six commits to 0006 never touched it.
- STATUS_ARCHIVE :176: #64's archived row state reads "merged: PR #64 ... 1ec6f6c" (precedent for #66's row).
- Line citations of ADR 0006 outside reports: SPIKE_PLAN :467, STATUS_ARCHIVE :390/:577/:1806 - all dated. Plan's claim holds.

## Answers
Q1 CHECK-CLOSE: (b) as put to him reaches the pass words of PM-0006-7 and step 4 (not PM-0006-9's). Plan's CHECK-CLOSE (b)
  = his passed-over STEP4-COUNT (a). No pre-build question. E9/E10 join, as the advisor agent's reading, never his
  decision; his go on the final head must knowingly cover them. Step 4's word (achieved / not yet) is his: asked with
  his go, nothing written in his name. PM-0006-9 unchanged.
Q2 E1: faithful, not looser ("judge ... only" = recorded, not judged). Reason = option (a)'s reason; the finding is the
  check's. Keep "host's header not ours to require" out of the row; don't cite the advisor tool as authority. Name
  "production's own address" (the project's pages.dev host) once.
Q3 E8: drop.
Q4 security-auditor: not needed unless E7 changes part 2's wall text or favours a move.
Q5 R4: order is the advisor's to set: step 2.6's plan after this PR merges ("after it", 11:07); one-line opt-out. Not a question.
Q6 R6: order right (probe off, archive, delete project + Access app). Timing is his TR-END (a); its condition is met;
  no edit here needs a re-reading. "My go" is the wrong owner.

## Notes for reviewers (not conditions)
- test-engineer: the host adds noindex on one-off addresses; on previews the noindex expectation may be met by the host
  alone, so no bite tests a missing preview noindex rule. Pre-existing; note or follow-up only.
- Rollback :1556 "if ... a header mark fails on Pages, move to Workers": left-alone reason should be "the revised mark is
  met by readings already taken, so its trigger is not reached", not "how a move would go".

## Revision 3 (plan.md at c440de8; round 1 on 09faefa..1c45ed7) - verdict: APPROVE with conditions
Read: plan.md R3-1..R3-9; infra-engineer-review.md (F1-F6, S23); test-engineer-review.md; verifier.md (FAIL on plan
conformity only); infra-engineer-workers.md (S2, S5, S10, S16, Q1-Q8); ADR 0006 at 1c45ed7 (:148 "no cap on previews",
PM-0006-8 :1392 "on a custom domain if CF has one", checklist intro :1459).

- E7 at 15 lines: accepted as "a short paragraph". His words set no line count; "at most 8" was the plan's own frame.
  Every clause carries a source, as the evidence rule asks, and it is one paragraph. Cap: 15 lines, no growth in round 2.
  The report states the length and why.
- E1 F6 (custom domain) and E1b: within ONE-OFF-HEADER (a). (a) removes only the one-off addresses from the noindex
  judgement. A custom domain is production's own stable address, so judging it is stricter, and PM-0006-8 already
  names it. "own addresses" is the draft's word, never quoted as his.
- Gap found: part 1 :148's *Why* says "no cap on previews" (R6-HOST-F3, read 2026-10-28). Pages' limits page of
  2026-11-25 (S23) no longer says so and names a retention rule (30 days, unless among the 100 most recent). E7, seven
  lines below, gives Workers' cap of 100. Read together, the weighing tilts toward Pages on a stale premise. Medium.
  Fix: one dated pointer at :148, not a rewrite, and the rest stays in ADR6-PAGES-RETENTION.
- E11 changes the checklist intro, part of the same pass-word change as E9/E10 (the advisor agent's reading): PASS-WORDS
  names all three. Step 4's word stays his.
- Q7 "Start new projects with Workers" plus the Pages retention change: they do not decide a move under (a), and no
  escalation follows. The report's top says both in plain words, so his go is informed.
- Follow-ups R3-7 accepted under the budget (ADR6-ROLLBACK Medium; PHC-PREVIEW-NOINDEX Medium to step 2.6's plan).
- Round 2 (R3-9) accepted. Add the :148 pointer to infra-engineer's re-read and S23 to the verifier's spot-checks.
- Low (rides this push): E1's custom domain is judged but PM-0006-2's method cell reads only the branch alias and the
  hash URL. Add "and a custom domain if CF has one", as PM-0006-8 does.
