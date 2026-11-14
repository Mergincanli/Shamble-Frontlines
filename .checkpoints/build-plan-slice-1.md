# Checkpoint: build-plan-slice-1
- **Goal:** the plan for foundation steps 2.4 and 2.5 and Slice 1 (move and shoot), by Mustafa's task of 2026-11-14
  (his paste block in this chat: "We start building the game"). The plan only: no build, no edit outside this
  checkpoint, no docs pull request. · plan: `.checkpoints/build-plan-slice-1/plan.md` (not written yet)
- **Branch:** claude/wonderful-hamilton-jdmlnf (stands in for cs/build-plan-slice-1) · **Base:** main
  4460ef5b8d6dc8566b29c15ded75c6f0c8be386b
- **Last update:** 2026-11-14 15:00 Amsterdam

## Steps done
1. Orientation · the session's model read with get_session at 2026-11-14 13:58 UTC: `session_context.model` and
   `configured_model` both `claude-fable-5-2` (the record holds no `last_served_model` field); effort xhigh; the branch
   is at main 4460ef5; main, claude/sweet-mayer-grsukw and claude/netcode-proof-test-plan-hcky5g fetched (read only).

## Current step
2. Reading (CLAUDE.md, CHECKPOINTS.md, REPO_MAP, STATUS and ENGINE_OVERVIEW read; next: ADR 0002's migration plan and
   the milestones it names, SPIKE_PLAN, LOOP_V2, LOOK_V2, DECISIONS, the 2.3 plan on its branch).

## Next steps
1. Finish the reading; notes in `.checkpoints/build-plan-slice-1/reading.md`.
2. Write the plan (his items 1 to 10) at `.checkpoints/build-plan-slice-1/plan.md`.
3. The advisor agent's plan review (Opus 5.5, effort max; no review rounds, no verifier, by his words).
4. Stop for his go.

## Open findings
- none yet

## Key numbers
- none yet

## Decisions taken
- His decision of 2026-11-14 (in his paste block, word for word; recorded in docs/DECISIONS.md by step 2.3's first docs
  change-set, built by the 2.3 chat): steps 2.4, 2.5 and Slice 1 may be planned now and built after his go on the plan.
- His rules for this chat (same block): main model Fable 5.2, builder agents Fable 5.2, every other agent Opus 5.5,
  reviewers, verifiers and designers of game and engine code at effort max; the advisor tool off; never push to the
  two other chats' branches, never touch spikes/; no kit, gate or CI edit without asking him with its exact text.

## Running work
- saver: not started yet
