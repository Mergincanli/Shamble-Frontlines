# Checkpoint: main-wall
- **Goal:** MAIN-WALL (Mustafa, 2026-11-25 20:35, item 2; order of 2026-11-26 15:05, item 4): a rule so that only his
  account can update main, measured on a throwaway repository first; day sessions, the PR Advisor and his revert route
  must still merge; his account must not skip gate and smoke; plan first, his clicks listed once; the measurement runs
  in a separate chat holding only the throwaway repository, or by his clicks (never a second repository in this chat:
  his 13:20 item 1, 15:05 item 6); the plan says where GUARD-KIT's parts ride (his 15:05 order: with R4-1's fix).
  The gap's detail stays out of pushed notes (his 20:35 item 3).
- **Branch:** claude/confident-gates-tbnq38 (stands in for cs/main-wall) · **Base:** main 4a62873e
- **Sources (read with git show, never checked out):** claude/optimistic-albattani-mdoonz at 7aadfc17,
  .checkpoints/step-2-6-previews/: questions-2026-11-25.md item 6, security-auditor-plan.md, questions-2026-11-26-guard.md,
  build-notes.md B15/B16 (read at 7aadfc17; the branch has moved since)
- **Last update:** 2026-11-26 afternoon Amsterdam

## Steps done
1. guard-wrappers merged as #73 on his go (main 4a62873e); lane branch fast-forwarded to main · no commit

2. infra-engineer's design (candidate W1, 10+2 tests, 12 clicks) · kept in .gate/main-wall/ (gap detail: not pushed)
3. Plan written (.checkpoints/main-wall/plan.md, no gap detail); the test chat's brief in the scratchpad (not pushed)

4. The advisor agent: REVISE, text only (C1-C7, brief fixes, THROWAWAY-PUBLIC for him) · notes advisor-plan.md
5. Plan revised (C1-C7); brief fixed and his test card written (.gate/main-wall/, not pushed)
6. His THROWAWAY-PUBLIC (a): neutral names (sandbox-1126, ruleset-1, ruleset-2) and early deletion in the plan, brief and card

7. security-auditor on the design, brief and card: REVISE, text only (SA-MW-1..12; decisions T4-BASELINE and
   GUARD-SETTINGS for him) · notes security-auditor-plan.md; all applied (plan pushed; brief and card in .gate/main-wall/)

8. The advisor agent's re-review: APPROVE with four conditions (applied as written: the baseline marked as his choice
   with clicks 6b and 6c, SA-MW-9 in the TECHNICAL note, security-auditor named in part B, a per-run branch name);
   T4-BASELINE's wording shortened · notes advisor-plan.md

9. Mustafa (2026-11-26, in this chat): "No, there will be an exception for me where its not required for my account."
   (on today's rule: pull request, gate, smoke). This changes his recorded condition of 2026-11-25 20:35 ("his account
   must not skip gate and smoke"). Asked him EXCEPTION-SCOPE: sessions act as his account, so an exception for his
   account very likely reaches every session too (to measure). No click done by him yet.

10. Mustafa (2026-11-26, in this chat): "EXCEPTION-SCOPE: ok go with D": no exception; the plan stands as approved
    (his condition of 2026-11-25 20:35 stays: his account does not skip gate and smoke).

11. Mustafa (2026-11-26, in this chat, answering PLAN-GO and T4-BASELINE in that order): "a" / "b": his go on the
    plan at 149bec7 (as revised after the advisor agent's APPROVE); T4-BASELINE (b), no baseline run (T4 passes only
    on the record naming the second ruleset's update rule, else "unproven"); brief and card updated to match
12. Click 0 (his screenshots): no deploy keys; 0 collaborators; the Claude app: Administration READ (read: Dependabot
    alerts, administration, commit statuses, merge queues, metadata; read and write: actions, checks, code,
    discussions, issues, pull requests, repository creation, repository hooks, workflows), repository access "All
    repositories" (so click 2 needs nothing); the Claude authorization page. GUARD-SETTINGS not needed (risk 4 covers
    his own account only). Missing: Settings > Actions > General.

13. His clicks 1, 3, 4, 5 done (screenshots): sandbox-1126 public, outside-contributor approval on; the test chat on
    his university account holds only sandbox-1126; ruleset-1 imported (Active, bypass empty, default branch, the
    copied rules); ruleset-2 (Active, Repository admin Exempt, default branch, Restrict updates only). The test chat
    put its setup in its pull request 1 (it cannot push to main), gate and smoke green there; it cannot read rulesets
    or rule records (no tool), so T0 and the rule records come from his screenshots. Still missing: the real
    repository's Settings > Actions > General.

14. Measurement done in mode Exempt: T1 (isolation), T2 (session merges), T4 (outsider refused, graded from GitHub's
    refusal text: no Insights page in his Settings), T5, T7 PASS; T1e, T6, T9, T3a and the second-account merge not run
    at his request (he dislikes the manual steps); results in .gate/main-wall/results-exempt.md (not pushed)

15. Results reviews: the advisor agent APPROVE (conditions: security-auditor first, his go on click 11 in his words,
    click 11 exactly as measured, the other lane's release point, docs quote his words); security-auditor APPROVE
    (C1 click 11 stands alone, C2 click 12 field by field, C3 secrets/variables/environments names checked once before
    click 11 (SA-MW-9), C4 T10 reading rules; SA-MW-R1..R8 Low/Medium) · notes advisor-plan.md, security-auditor-plan.md
    Decisions for him: TOKEN-RELEASE, T1E-BEFORE-DELETE, T4-GRADING; which Claude account this chat is on.

16. Mustafa (2026-11-26, in this chat): CLICK-11-GO (a); T4-GRADING (a); T1E-BEFORE-DELETE (b); TOKEN-RELEASE (c)
    "but we really need to speed up things, its taking too long and certain manual tasks are literally frustrating, to
    the point that I am tempted to take the risk without all of these things."; WHICH-ACCOUNT: "My first personal
    account (I have two) and 1 enterprise (university), so I have 3 accounts working on the project". His secrets
    screenshot was the sandbox's; the real repository's is folded into click 12 (click 11 only tightens, so the order
    swap loosens nothing: recorded as a deviation for the docs).

## Current step
His one visit: click 11, then click 12 (the saved rule page; the real repository's Secrets and variables > Actions);
then the sandbox's fork count and deletion.

## Next steps
1. Plan (.checkpoints/main-wall/plan.md, no gap detail); the advisor agent's review; his go on the plan.
2. Throwaway-repository measurement (separate chat or his clicks); results by his paste.
3. His clicks on the real repository (listed once); checks; docs (with guard-wrappers' six Low follow-ups F2-F7).
4. Then one guard change-set (R4-1's fix plus GUARD-KIT), then the review tracks with SOURCE-GO (a).

## Open findings
- R4-1 (open Critical, class only); interim rule: git and gh always start their own command.

## Decisions taken
- MAIN-WALL next; the wall first · Mustafa · 2026-11-25 20:35
- Order: MAIN-WALL, guard (R4-1 + GUARD-KIT), review tracks + SOURCE-GO (a) · Mustafa · 2026-11-26 15:05
- CLICK-11-GO (a), T4-GRADING (a), T1E-BEFORE-DELETE (b), TOKEN-RELEASE (c) · Mustafa · 2026-11-26
- PLAN-GO (a), T4-BASELINE (b) · Mustafa · 2026-11-26 ("a" / "b")
- EXCEPTION-SCOPE (d): no exception for his account on today's rule · Mustafa · 2026-11-26 ("ok go with D")
- THROWAWAY-PUBLIC (a): public throwaway, neutral names, deleted as soon as the results are reviewed · Mustafa · 2026-11-26 (his answer "a" in this chat)

## Running work
- none
