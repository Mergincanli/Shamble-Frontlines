# Checkpoint: docs-after-v2-research
- **Goal:** STATUS and ROADMAP (and, if the plan is approved that way, the research README) catch up with PR #28's
  merge and record the 2026-10-31 decisions: Mustafa's MODES-3 (a), PREVIEW-WALL (a), SEC-SPREAD (a), ABL-PREDICT
  later; the Advisor's BASELINE-WHEN (b) · plan: `.checkpoints/docs-after-v2-research/plan.md` (to be written; the
  advisor reviews it before any edit)
- **Branch:** cs/docs-after-v2-research · **Base:** origin/main b6effde (PR #28's merge commit)
- **Last update:** 2026-10-31 14:40 Amsterdam

## Steps done
1. PR #28 merged on Mustafa's "merge #28" in this session's chat (2026-10-31), given after the advisor's APPROVE of
   854d42f121ab67744040da41a4b2e974e200020c (PR comment 6542571633), with `gate` and `smoke` green on that SHA: merge
   commit b6effde (13:26:30 UTC), expected head passed; the session stopped watching the PR.
2. The decisions saved word for word: `docs-after-v2-research/decisions-2026-10-31.md` · this commit.

## Current step
A read-only touchpoint sweep (workflow) over docs/, .claude/, CLAUDE.md and issue #16, then the plan.

## Next steps
1. Write `plan.md` from the sweep (precedent: docs-after-r7's plan, `git show 86ece5c:.checkpoints/docs-after-r7/plan.md`);
   the advisor reviews it: APPROVE → build; REVISE → fix; ESCALATE → Mustafa.
2. On APPROVE: docs-keeper edits; gate; repo-map check; a verification workflow; draft PR (the PR body is the report);
   the advisor reviews the head; fold this checkpoint into the PR body and delete it (CHECKPOINTS.md §9); ready;
   Mustafa's "merge #<n>".

## Open findings
- The advisor's #28 review (PR comment 6542571633), non-blocking: (1) STATUS still says #28 is not merged → this
  change-set; (2) R2-BUD-REC6's reason (`docs/research/r2-netcode.md:2402-2403`) holds only at N = 54 → the synthesis;
  (3) the report's "three decisions stay open" leaves out ABL-PREDICT and SEC-SPREAD → a dated record, STATUS lists
  them; (4) plan condition 9 (the synthesis verifies every ADR claim and number, R3–R6's included) → the synthesis plan.
- `CLAUDE.md:7` and `.claude/agents/netcode-architect.md:17` read VISION_V2 §3 per team ("two teams of 2–8 and three
  teams of 6–18"): rules files, so not in this docs change-set (kit-v3-1 lands first if they collide, BRANCH-2 (a)).
- `docs/VISION_V2.md:43` (§3) is where the ambiguity starts; the file is "written by Mustafa and the Advisor", so the
  plan asks whether the session may add Mustafa's answer there.
- STATUS "Targets" (lines 76–78) predates VISION_V2: "12v12v12 to 16v16v16", "Milestones: 2–8 players → 16 → full scale".
- `docs/research/GLOSSARY.md:334-336` ("The largest mode, and its N"): "This glossary picks neither; a mark gives both
  values until Mustafa answers."

## Key numbers
- At N = 48 the 60 Hz server tick on a re-flood tick is 8.28 ms against the 8.33 ms limit (`GLOSSARY.md:66`): it fits by
  0.05 ms.

## Decisions
- The 2026-10-31 decisions: `docs-after-v2-research/decisions-2026-10-31.md` (verbatim).
