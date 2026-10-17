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
## Risks
Ranked, one line each.
## Proposed next step
One change-set, one sentence.
## Decisions needed
One entry per decision, self-contained: code · the question in plain words · why it
matters · options · recommendation. No code that isn't explained here.
## For Mustafa
Numbered actions: Brave checks (screen, position, state, steps) and merges to approve,
or "nothing".
