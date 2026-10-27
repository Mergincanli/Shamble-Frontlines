# Amendments to the approved R1–R6 plan (2026-10-27, this session)

`plan.md` is the approved plan as written at 11:41 UTC (advisor APPROVE with 14 conditions, applied). These changes come
after it. Items 1–7 are the Advisor's instructions, relayed by Mustafa in this session's chat on 2026-10-27; item 8 is
this session's proposal. The advisor agent checks them in this session (verdict: `advisor-amendments-verdict.md`).

## The Advisor's words (verbatim, this session's chat)
> - kit-v3-1 is shelved: leave its branch untouched and don't open its files. Record it in STATUS as paused.
> - Until it returns, research runs in the daytime only, and the night shift takes only items that need no web
>   access. If a night run stops at a permission prompt, list it in the morning report.
> - Turn the PR Advisor routine off for now; the advisor agent reviews PRs in your daytime session.
> - R7-MARKS: (a), leave them pending until the look brief exists.
> - MANAGED: (b). Mustafa is not an admin of the university's Claude plan.
> - Start R1–R6 as planned (wave 1: simulation core and netcode) on cs/v2-research. In R6, cover hosting, costs,
>   monitoring and telemetry; the security baseline waits until later.

## What changes in the plan
1. **kit-v3-1 shelved.** "Until kit-v3-1 is merged" (Branch, day and night; condition 2) now reads "until kit-v3-1
   returns and merges": research is launched or resumed only between 08:30 and 00:10 Amsterdam time, a run still going
   at 00:10 is stopped, and nothing about research goes into issue #16. This session never touches `cs/kit-v3-1` and
   opens none of its files, so the plan's kit-v3-1 worktree line does not apply here. The write-guard stand-in
   (condition 3) stays in force, since WRITE-GUARD has not landed.
2. **R6 scope:** hosting (previews and EU game servers), costs, monitoring (logs, metrics, crash reports) and balance
   telemetry (VISION_V2 §6.6). The security baseline (content security policy, dependency audits, anti-cheat basics)
   is deferred by the Advisor: security-auditor's R6 helper draft is dropped, and `r6-infrastructure.md` lists the
   security baseline under open questions as "deferred by the Advisor, 2026-10-27". About 36 agents instead of 37.
   R2's protocol review by security-auditor stays (it is R2's, not R6's baseline).
3. **PR reviews:** the advisor agent reviews this research PR in the daytime session (the reviewers line already
   names "advisor (the PR)"). The PR Advisor routine is to be off; an agent may not switch it (the API refused:
   the routine was not created by an agent), so Mustafa switches it off himself.
4. **STATUS and ROADMAP (condition 11):** Mustafa asked for kit-v3-1 to be recorded in STATUS as paused now, so a
   small docs-only change-set (docs-after-r7, its own PR) adds the earlier entries now: the v2 pivot, kit-v3-0, PR #25,
   R7, kit-v3-1 paused, the 2026-10-27 decisions. Condition 11's "unless … has already added them" then applies: this
   research PR's docs-keeper step adds only the research rows.
5. **R7-MARKS (a):** the research treats the inventory's marks as working recommendations; rows marked "look brief
   pending" stay pending until the look brief exists (R3 above all).
6. **Night shift:** only items that need no web access until kit-v3-1 returns; a run that stops at a permission prompt
   lists it in its morning report. Recorded in issue #16 and STATUS by docs-after-r7; no other effect on this plan.
7. **MANAGED (b):** a kit-v3-1 decision; recorded in STATUS only.
8. **Orchestration of wave 1 (proposal):** this container has 4 CPUs, and a workflow runs at most min(16, CPUs − 2)
   = 2 agents at a time, so the 11 wave-1 units would run in six rounds and could cross the 00:10 cut-off. The session
   therefore launches each stage's agents as background agents, all at once (11 units; then the 2 assemblers; then the
   verifier on R1, the verifier on R2, rust-reviewer and security-auditor; then the fixers; then any second verifier
   pass). The units, prompts, notes paths, model, effort and stages are the plan's; only the launcher changes. A
   cut-off agent is restarted with the same prompt and carries on from its notes (as the plan already says for wave 2),
   and the session counts what came back at every stage (CHECKPOINTS §7).
