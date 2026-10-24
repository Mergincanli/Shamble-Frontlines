# Checkpoint: kit-v3-0
- **Goal:** v2 phase 0, step 1 (brief §2): model policy, checkpoints, efficiency rules, night shift v2, crew v2. Plan:
  `.checkpoints/kit-v3-0/plan.md`.
- **Branch:** claude/bold-faraday-i49gug (stand-in for cs/kit-v3-0) · **Base:** main 48ffb51
- **Last update:** 2026-10-25 01:20 Amsterdam

## Steps done
1. docs/VISION_V2.md added, byte-identical to the attachment (SHA-256 22664025…3ab) · 4afb714.
2. Plan written (`.checkpoints/kit-v3-0/plan.md`) · 4afb714.

3. Plan review (workflow wf_f1150920-4af): advisor kit-v3-0 APPROVE with 18 conditions + escalation KV3-MERGE;
   advisor R7 APPROVE with 11 conditions; critic 28 full / 18 partial / 1 missing of 47. Verdicts:
   `.checkpoints/kit-v3-0/plan-review-verdicts.json`; outcome appended to plan.md. Commit b152533 (brief copy too).
4. Night queue #16 item 1 → plain bullet "in progress as R7" (2026-10-25 ~00:35).
5. checkpoint.mjs + test: 10/10 pass, 4 mutants each fail their own test (ddd13b6;
   `.checkpoints/kit-v3-0/evidence-checkpoint-script.md`).
6. `opus` alias proven: an art-director (model: opus) agent reported model-id claude-opus-5-5 (2026-10-25 00:40).

## Current step
7. Build: CLAUDE.md, CHECKPOINTS.md, NIGHT_SHIFT.md, settings.json and the six new agents written (this commit);
   next the seven existing agents, two skills, security guidance, ADR template, REPO_MAP + check, research README.

## Next steps
5. Build (rest): CLAUDE.md, CHECKPOINTS.md, checkpoint.mjs + test, settings.json, NIGHT_SHIFT.md, 6 new agents, 7 updated
   agents, 2 skills, security guidance line, REPO_MAP, research README. Checkpoint after each.
6. Evidence: gate, script test + mutation, trace table, contradiction sweep, REPO_MAP check.
7. Security review of the final diff; fix or defer.
8. Report (plan + verdicts verbatim); delete `.checkpoints/kit-v3-0*`; PR draft → ready; advisor PR review of the final
   head; ask Mustafa for "merge #<n>"; merge with the approved SHA as expected head.
9. After merge: routines (brief §3); R7 on this branch restarted from main.

## Open findings
- PR #25 (docs-after-kit-v2-2) is open since 2026-10-17 and edits STATUS and ROADMAP: kit-v3-0 leaves both alone.

## Key numbers
- none yet

## Decisions taken
- `model: opus` in every agent file (proven to resolve to claude-opus-5-5); every agent states its effort (Opus 5.5
  defaults to medium, claude-code-guide from the model-config docs).
- Merge path: the advisor's ruling (condition 2, escalation KV3-MERGE): Mustafa's fresh "merge #<n>" after APPROVE.

## Running work
- R7 (legacy inventory) runs alongside in a local worktree, local branch `r7-legacy-inventory`; pushed after this
  PR merges.
