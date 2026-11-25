# Checkpoint: step-2-6-previews (lane 2, Builder V2): step 2.6's plan, the preview links

Branch `claude/optimistic-albattani-mdoonz` (stands in for `cs/step-2-6-previews`; this session may push only it),
restarted at main c1da1bd24faae21794195e2384895460753a41eb (#67's merge, 2026-11-25). Risk tier 2 for the build (a CI
workflow, a deploy token, hosting): the advisor agent and the security-auditor review; his go in a day session.

## Goal (his words of 2026-11-25, 18:15 Amsterdam, item 3; transcript 17:11:56 UTC, uuid 42aa43e4...)
"Then step 2.6's plan, the preview links, as my order of 25 November has it. Plan first, your advisor agent's review,
then back to me for my go; nothing built before it." The plan holds:
a. his WORKERS-NEXT answer: the deploy step small enough to move, two lines on what a move would cost, the 30-day rule
   for old previews;
b. every click he does himself (the Cloudflare project, the deploy token, anything on GitHub), listed once, in order,
   for one sitting;
c. what lane 2 needs from the CI Builder chat (the edit that hands the tested build to the preview job), as a request
   it can act on;
d. the day he can expect the first preview link, said honestly.
Item 4: step 2.6's first pull request also records his 17:05 message (uuid 0ff798eb..., 16:41:51 UTC, 1094 bytes,
sha256 c41b21e8...) in DECISIONS and adds the dated pointer in #67's report (the advisor agent's Low on 35adfbe).
His 18:15 message itself (uuid 42aa43e4..., 1778 bytes, sha256 e2554c4e...) goes in with it.

## Inputs (read, not decided here)
- Build plan `docs/reports/2026-11-17-build-plan-slice-1.md` :1021-1027 (when step 2.6 can run) and Part 15
  (`...-part-15.md` :103, :176): lane 2 owns the preview workflow (a new file), the host's config files, its runbook;
  must not touch gate.yml, toolchain.yml, the gate's files, crates/; the tested build comes by an edit to toolchain.yml
  that lane 1 (now the CI Builder chat) makes on his go.
- ADR 0006 parts 1-3 and 13 (B9), the revised PM-0006-2 and -7; ADR 0002 (the deploy job, seam S3, PM-0002-66).
- #66's report (Appendix B: the workflows that ran; follow-ups PHC-*), #67's report (follow-ups ADR6-*, STEP26-WORKERS,
  PHC-PREVIEW-NOINDEX).
- His words: "I do not want to click for each preview" (2026-11-19, 00:01:25 UTC); STEP4-COUNT (b): only the newest
  push of a pull request is deployed, and deploys are counted per month; WORKERS-NEXT (a) points.

## Steps
1. #67 merged on his go: c1da1bd (normal merge, expected head 35adfbe; checks green; the advisor agent's APPROVE).
   Main folder fast-forwarded to main; the fixed guard is on disk (hooks equal main). Branch pushed (fast-forward).
2. Research by infra-engineer (notes: `step-2-6-previews/infra-engineer-design.md`): current step.
3. The plan (`step-2-6-previews/plan.md`), with an advisor tool consultation before it.
4. The advisor agent's plan review (and the security-auditor's read of the token design).
5. Back to him for his go; nothing built before it.

## Current step
4: the plan (revision 1, 081e7e9) is with three readers at once (started about 17:54 UTC, effort xhigh): the
advisor agent (plan review; notes advisor-plan.md), the security-auditor (the token design and the untrusted build;
notes security-auditor-plan.md), engine-architect (the departure from ADR 0002 part 10; notes engine-architect-plan.md).
Done: 2, infra-engineer's research (D1-D8; its last section overrides); 3, the plan, after an advisor tool consultation
(its points folded in: M1 as a pass mark before any token; ADR 0002 part 10 read; toolchain.yml read: the build lands in
.gate/dist-production at :172 inside the cut step, so an upload step after it has the network; departures listed;
GATE-FOLLOW and file scope put to the advisor agent).

## Rules in force
- Until a second guard fix is merged: git and gh always start their own command, with no other program in front.
- At most 3 agents at a time; Opus; effort xhigh for infra/security design work on the drafts' parts.
