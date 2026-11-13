# Checkpoint: foundation-2-3-docs
- **Goal:** step 2.3's first docs pass (Mustafa's words of 2026-11-13): his go for #48 and his 2026-11-13 decision in
  DECISIONS.md by script, with dated pointers; STATUS and ROADMAP true for main 4d21c1a; the 2.2 report's fixes M1, L1,
  L2, L3 and L5. Its own PR, docs only; ready, then stop for his go. After it merges (a new checkpoint): plan step 2.3.
  · plan: `.checkpoints/foundation-2-3-docs/plan.md`
- **Branch:** claude/sweet-mayer-grsukw (for cs/foundation-2-3-docs) · **Base:** main
  4d21c1a7a4d39613bffce5d5f9bbc92f2f88a300
- **Last update:** 2026-11-13 20:12 Amsterdam

## Steps done
0. The model check: Opus 5.5 (get_session: session_context.model and last_served_model are claude-opus-5-5; effort
   max). The advisor tool is on; the `/advisor fable` output said this conversation keeps Opus 5.5 behind it until
   `/clear` or `/compact`. Orientation: CLAUDE.md, CHECKPOINTS.md, STATUS, DECISIONS (2026-11-10 to 11-13 and
   "Queued"), 4d21c1a's message, the 2.2 report, PR #48's description, the advisor agent's verdict on 9b20b67 (the 2.2
   session's record, list_events) and 2.2's notes at 2f0fef3.
1. The checkpoint and plan revision 1 saved (30483c3); the draft PR opened: #52
   (https://github.com/Mergincanli/Shamble-Frontlines/pull/52); the plan names #52 (sed, `#N` to `#52`, 5 places).

## Current step
Step 1 of the plan: the advisor agent's plan review (the saver running).

## Next steps
1. The advisor agent's plan review against the plan's commit SHA (plan §12, Q1 to Q7); record the verdict and the SHA
   here; fold its conditions into revision 2.
2. Then the plan's steps 2 to 8.

## Advisor tool consultations (his words of 2026-11-11; in this conversation Opus 5.5 answers the tool)
1. 2026-11-13 ~20:00 Amsterdam, moment 1, before committing to the docs-pass approach: is the approach right? Changed:
   saved first (checkpoint, plan, draft PR) before any tracked edit; every pointer quotes all four limits; the grep
   widened (ROADMAP:93 and STATUS:38 are sites; every hit listed as added or left); the decision cut by anchors (nested
   quotes); 2.2's appendix check replaced by a hash check plus the body-quote check; KIT-FREEZE-2 quoted, not
   paraphrased; Q1 to Q4 put to the advisor agent; agent briefs per CHECKPOINTS §4; the final head approved as the
   commit that deletes the checkpoint; #40's row only from what can be read.

## Refusals (guard, deny list, classifier) and the route taken
- None so far.

## Open findings
- None.

## Key numbers
- main 4d21c1a; CI on it: gate run 43461710700 and toolchain run 43461710673, both success (the API, 2026-11-13).
- 2.2's `decisions-entries.mjs --check` on main: "OK: 13 entries (12 with quotes)", exit 0 (from a scratch copy in
  `.gate/fdn23-docs/s22/`).
- PR #40: draft, head b4d1f0a, updated 2026-11-13 18:57 UTC (the API).

## Decisions taken
- The slug `foundation-2-3-docs`; the docs pass is its own PR from this branch (his words).

## Running work
- Saver: not started yet.
