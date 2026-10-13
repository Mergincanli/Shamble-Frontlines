---
name: advisor-report
description: 'Write the end-of-change-set report for Mustafa and his Advisor (a separate Claude): compact, evidence-first, paste-ready. Use at the end of every change-set or workflow run, and whenever Mustafa asks what happened.'
argument-hint: '[change-set slug]'
---
Write `docs/reports/<YYYY-MM-DD>-<slug>.md` and print it. At most ~60 lines. Plain language, decimals with periods.
Facts only: every claim points to evidence. Say clearly what was NOT verified.

# Advisor report: <slug> (<date>, branch cs/<slug>, base <ref>)
## Goal
One or two lines: what and why (player-facing).
## Outcome
DONE / PARTIAL / BLOCKED, plus one sentence.
## What changed
Files with one line each. Symbols added / removed (from the gate) and whether that matches the approved plan.
## Evidence
Gate result per check · tests and assertions · render pairs (paths) · sim runs (all numbers) · measurements (before → after).
## Reviews
Each reviewer's verdict and its top findings; for each finding: fixed / deferred (why).
## Decisions and deviations
Anything decided during implementation, or done differently from the plan, and why.
## Risks and open questions
Ranked. Questions for Mustafa phrased as a recommendation he can veto.
## For Mustafa in Brave
Exact screenshots to take (screen, position, state), or "nothing visual".
## Proposed next step
One change-set, one sentence.
