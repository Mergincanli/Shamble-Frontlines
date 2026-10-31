# Checkpoint: docs-after-v2-research
- **Goal:** record PR #28's merge and the 2026-10-31 decisions (two messages, verbatim in
  `docs-after-v2-research/decisions-2026-10-31.md`) in STATUS, ROADMAP, the research folder, VISION_V2 §3 (one line)
  and the Night queue · plan: `docs-after-v2-research/plan.md` (revision 2; the advisor's review pending)
- **Branch:** cs/docs-after-v2-research · **Base:** origin/main b6effde (PR #28's merge commit)
- **Last update:** 2026-10-31 15:00 Amsterdam

## Steps done
1. PR #28 merged on Mustafa's "merge #28" in this session's chat (2026-10-31), after the advisor's APPROVE of
   854d42f121ab67744040da41a4b2e974e200020c (PR comment 6542571633), `gate` and `smoke` green on that SHA: merge commit
   b6effde (13:26:30 UTC), expected head passed; the session stopped watching the PR · 1bd662d (checkpoint).
2. The first message's decisions saved word for word · 1bd662d.
3. Read-only sweep (workflow wf_c793ae7a-de6, 2 agents, effort high; prompts in `prompts/sweep-workflow.js.txt`):
   sweep A 56 findings, sweep B 18; notes `sweep-a-decisions.md`, `sweep-b-merge-state.md`.
4. The second message (VISION-LINE (a), TICK-30, SLICE-ORDER and three conditions) saved word for word.
5. Plan revision 2; the research edits as an exact script (`scripts/apply_research_answers.py`): dry run 34 of 34
   anchors once; preview `research-edits.diff` (+65 −12 lines), table `research-edits-table.md`; docs/ untouched.
6. NS-2 carried out with Mustafa: his new routine "Shamble Frontline Night shift" (trig_01QpV6y6UaG1Na2q3jumRDxh) checked:
   fresh session per run, repository attached, Opus 5.5, same prompt; its 13:51 UTC test run read NIGHT_SHIFT.md from
   main and stopped silently (outside the window). The session's schedule edit was refused by the API ("Agents can only
   update routines they created"): Mustafa sets `CRON_TZ=Europe/Amsterdam 10 0-7 * * *` on its page. The old "Night
   shift" (trig_019LtRqmAdzddyk6K3e3kDJz) was switched off by the session at 13:54:19 UTC (NS-2 (c)); only the new one
   is enabled (read back at 13:54 UTC).

## Current step
The advisor's review of plan revision 2 (APPROVE → build; REVISE → fix; ESCALATE → Mustafa).

## Next steps
1. On APPROVE (with its conditions): `python3 -I .checkpoints/docs-after-v2-research/scripts/apply_research_answers.py
   --apply` (the two ADAPTED texts as the advisor rules); the VISION_V2 line; docs-keeper's part L; the issue #16 edits
   (after the research and docs commits); gate; repo-map check; the verification workflow (V1–V4); fixes.
2. Draft PR (body = report); the advisor reviews the head; fold this checkpoint into the PR body and delete it
   (CHECKPOINTS.md §9); ready; Mustafa's "merge #<n>".

## Open findings
- The advisor's #28 review (PR comment 6542571633), non-blocking: (1) STATUS stale → this change-set; (2) R2-BUD-REC6's
  reason holds only at N = 54 → the drafts (carry-forward 2 in the plan); (3) the report's "three decisions stay open"
  → a dated record, STATUS lists them; (4) plan condition 9 → the drafts' plan.
- Rules files (KIT-ALIGN) and the Advisor's documents (DOCS-ALIGN): listed in the plan, not edited here.
- Two of the six approved (b)-only texts need the advisor's ruling (C17.b2, C23.b2: their quotes changed in step 3c).

## Key numbers
- GLOSSARY.md:62-66, N = 48, 60 Hz: a tick that encodes and re-floods fits the 50 % server tick budget by 0.05 ms
  (8.28 of 8.33 ms), and its server-only share is 0.24 ms over its 25 % (4.41 of 4.17 ms). TICK-30 makes 30 Hz the
  baseline for 48 players.

## Decisions
- `docs-after-v2-research/decisions-2026-10-31.md` (both messages, verbatim).
