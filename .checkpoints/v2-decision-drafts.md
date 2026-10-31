# Checkpoint: v2-decision-drafts
- **Goal:** the v2 decision drafts (`docs/ENGINE_OVERVIEW.md`, `docs/adr/0001`–`0006`, the spike plan, the loose ends)
  from the drafts brief (`v2-decision-drafts/inputs/DRAFTS_BRIEF.md`), then a draft PR, the report and a stop for Mustafa
  and the Advisor · plan: `v2-decision-drafts/plan.md` (revision 2: the advisor's APPROVE of revision 1 with 19 conditions, folded; verdict verbatim in `advisor-verdicts.md` §1)
- **Branch:** claude/laughing-lovelace-ox61se (the session's branch, standing in for cs/v2-decision-drafts) ·
  **Base:** origin/main 9e7db28 (PR #32's merge)
- **Last update:** 2026-10-31 19:00 Amsterdam

## Steps done
1. Step 0 (brief §0): `docs/LOOK_V2.md` added byte for byte (sha256 52b4fc06…) · 6617e25; the brief copied to `inputs/`;
   plan revision 1 written · d6c71e4.
2. Plan review: the advisor's APPROVE with 19 conditions (about 18:50); folded into plan revision 2 (no second review
   needed, per the verdict); `origin/main` fetched to 9e7db28 (condition 1); the id list rebuilt (REC 165: the two R3
   rows were references, condition 6) · this commit.

## Current step
Step 2: the decision map (engine-architect, notes only; prompt `prompts/engine-architect-decision-map.txt`), still
running; then its addendum (condition 7) by the same agent.

## Next steps
1. When the map returns: run `map_coverage.py` on its id table; send the addendum (plan step 2, condition 7) to the same
   agent; then launch the drafting units (step 3, ≤ 5 at once; prompts from the scratchpad drafts, saved to `prompts/`).
2. Pass marks per draft as each lands (step 4); assembly (step 5); synthesis (step 6); verification (step 7; no
   fetching verifier after 22:00, no new agent after 23:00).

## Open findings
- The research has no asset-pipeline findings (ASSET-RESEARCH, in the plan).

## Key numbers
- Research ids defined in r1–r6 at 9e7db28 (`inputs/research-ids.tsv`, built by script, rebuilt after the review): 533
  rows, 521 unique ids: REC 165, OQ 241, RR 7, XU 30, XT 72, 6 shared codes (18 copies). Matches the research report's
  165 recommendations and 278 own open questions (241 + 30 + 7).
- Findings checked by each topic's verifier (`inputs/findings-checked.tsv`, script-built guide): R1 172/185, R2 136/152,
  R3 44/92, R4 49/62, R5 49/117, R6 18/42.

## Decisions taken
- 2026-10-31: the drafts brief (Mustafa and the Advisor) and LOOK_V2 (attachments of the session's first message).
- 2026-10-31 about 18:50, the advisor: plan APPROVE with 19 conditions; rulings ASSET-RESEARCH (a), TICK-60-HEADROOM
  ruled now (base 8.33 ms, p99 ≤ 6.25 ms; confirmed at the stop), GLOSSARY-8 yes (three entries and a note),
  CHECKPOINT-AT-STOP keep.

## Running work
- advisor plan review: done (APPROVE), notes `advisor-plan-review.md` (DONE).
- engine-architect decision map: background agent, launched 18:37, running; notes `engine-architect-decision-map-notes.md`,
  outputs `engine-architect-decision-map.md` and `-ids.tsv`.
- checkpoint saver: `checkpoint.mjs watch` in the background since 18:22 (110 min).
