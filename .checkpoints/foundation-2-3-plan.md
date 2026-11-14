# Checkpoint: foundation-2-3-plan
- **Goal:** plan foundation step 2.3 (the gate v2 and CI, the end of the kit's freeze). Mustafa's task: docs/DECISIONS.md,
  the entry "Taken by Mustafa, 2026-11-13, 18:47:55 UTC", from "Then, after the docs pass is merged: plan step 2.3." to
  its end (items 1 to 9), plus his message of 2026-11-14 starting the plan (three additions; review by the advisor agent
  and the advisor tool only). No build; no edit outside this checkpoint. His rules of 2026-11-14 (merge commit 4460ef5's
  message, items 2 and 3): consult the advisor tool before a step is saved as done; in a report, write about my own
  process only what a record shows, with its time. · plan: `.checkpoints/foundation-2-3-plan/plan.md` (to write)
- **Branch:** claude/sweet-mayer-grsukw (stands in for cs/foundation-2-3-plan; restarted from main by fast-forward
  bfd719b..4460ef5, pushed) · **Base:** main 4460ef5b8d6dc8566b29c15ded75c6f0c8be386b
- **Last update:** 2026-11-14 11:45 Amsterdam

## Steps done
1. Orientation: the session's model read from get_session (session_context.model and last_served_model:
   claude-opus-5-5); the advisor tool's model read from the transcript (17 calls, every one answered by
   claude-fable-5-2, the latest at 2026-11-14 09:07:35 UTC); the branch fast-forwarded to main 4460ef5 and pushed; this
   checkpoint opened.

## Current step
2. Reading: the whole 18:47:55 entry with its "Read first" list, today's main, the old plan's part 3 at 26e0d82, ADR 0002
   parts 7 to 9 and PM-0002-39 to PM-0002-62, the "Findings for 2.3" of the 2.1a, 2.1b, 2.2 and 2.3-docs reports,
   "Queued", STATUS's follow-ups, the archive's security items.

## Next steps
1. Finish the reading (notes in `.checkpoints/foundation-2-3-plan/reading.md`, one block per source).
2. Advisor tool, moment 1 (before committing to the plan's approach); record it below.
3. Write the plan (`.checkpoints/foundation-2-3-plan/plan.md`): items 1 to 9 and the three additions.
4. The advisor agent's plan review (from main's `.claude/agents/advisor.md`); fix on REVISE.
5. Advisor tool, moment 5, before the plan is saved as done; then the end message and stop for his go.

## Open findings
- (none yet)

## Key numbers
- Advisor-tool consultations in this task: 0 so far (the docs pass's 17 are counted in its report and the merge).

## Decisions taken
- Step 2.3 may be planned now; its parts that need no proof test's result may be built after his go on the plan
  (Mustafa, 2026-11-13, 18:47:55 UTC; docs/DECISIONS.md).
- The plan gets the advisor agent's review and the advisor tool at its five moments; no review rounds, no verifier
  (Mustafa, 2026-11-14, his message starting the plan).

## Advisor-tool consultations (one line each)
1. 2026-11-14 ~11:00 UTC (12:00 Amsterdam), moment 1, after the reading notes and approach.md: is the approach (the
   cut, the order, the needs-result reading, who decides what) right, and what is missing? Changed: a mark counts as
   met only when every clause holds, X named with its trigger; item 9 gets the exact-text mechanism (patch at a
   checkpoint SHA, his yes naming it, byte check) and prompt counts per CS; PIN-GUARD's scope becomes his decision, the
   config file names fixed in the plan; the reading completed (root manifests, crates/ and packages/, recursive greps
   over .claude/, .github/, tools/README.md, W1-W3); the exception lines carry his limits, the advisor-tool text is his
   bullets by script, the moment-5 line marked wider than his words, SD-N1 out of CS-B, the settings allow rule put to
   him beside his allow-list words; three facts settled by read-only checks (hook input's permission mode; which
   workflow file a pull_request run uses and conflicting PRs; whether the sim core spike's open questions touch ADR 0004
   part 7's allow-list); the summary says each CS gets its own plan and plan review.

## Refusals (guard, deny list, classifier)
- (none yet in this task)

## Running work
- saver: background task b6ean8gz0 · output /tmp/claude-0/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3/tasks/b6ean8gz0.output ·
  started 10:44 UTC (11:44 Amsterdam) · stops by ~12:34 UTC (110 minutes) · last check 11:00 UTC: ok (first line
  "watching .checkpoints/ on claude/sweet-mayer-grsukw every 10 min for 110 min").
- agents (2 of 3, read-only, no notes of their own: they cannot write; results saved to reading.md on return):
  claude-code-guide on the hook input's permission mode; web-fetch on GitHub's pull_request workflow source and
  conflicting PRs (launched ~11:02 UTC).
