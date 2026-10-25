# Checkpoint: kit-v3-0
- **Goal:** v2 phase 0, step 1 (brief §2): model policy, checkpoints, efficiency rules, night shift v2, crew v2. Plan:
  `.checkpoints/kit-v3-0/plan.md`.
- **Branch:** claude/bold-faraday-i49gug (stand-in for cs/kit-v3-0) · **Base:** main 48ffb51
- **Last update:** 2026-10-25 11:25 Amsterdam (CET since the clock change)

## Steps done
1. docs/VISION_V2.md added, byte-identical to the attachment (SHA-256 22664025…3ab) · 4afb714.
2. Plan written (`.checkpoints/kit-v3-0/plan.md`) · 4afb714.

3. Plan review (workflow wf_f1150920-4af): advisor kit-v3-0 APPROVE with 18 conditions + escalation KV3-MERGE;
   advisor R7 APPROVE with 11 conditions; critic 28 full / 18 partial / 1 missing of 47. Verdicts:
   `.checkpoints/kit-v3-0/plan-review-verdicts.json`; outcome appended to plan.md. Commit b152533 (brief copy too).
4. Night queue #16 item 1 → plain bullet "in progress as R7" (about 2026-10-25 00:00).
5. checkpoint.mjs + test: 10/10 pass, 4 mutants each fail their own test (ddd13b6;
   `.checkpoints/kit-v3-0/evidence-checkpoint-script.md`).
6. `opus` alias proven: an art-director (model: opus) agent reported model-id claude-opus-5-5 (2026-10-25, about 00:00).

7. Build: CLAUDE.md, CHECKPOINTS.md, NIGHT_SHIFT.md, settings.json, six new agents · 30ff4a4; seven existing agents,
   two skills, security guidance, ADR template, REPO_MAP + repo-map-check (89/89; dropped and stale entries fail),
   research README · e644c2a.
8. Gate PASS on all 8 checks vs main 48ffb51, symbols +0 -0, smoke 8/8 + 5/5 (`evidence-gate.txt`; Chromium for the
   pinned Playwright 1.56.1 installed into the scratchpad, PLAYWRIGHT_BROWSERS_PATH).

## Current step
9. Final-diff review workflow wf_29950217-697: requirement trace, contradiction sweeps A and B with a skeptic per
   finding, security-auditor. Cut off at about 01:00 by the org's monthly spend limit (reset 02:20 UTC): 17 of 30
   agents done, skeptics B4-B16 failed. Resumed 11:25 with resumeFromRunId (task wov1cm37a). The R7 workflow was cut
   off the same way and resumed (task wllinx1zp).

## Next steps
10. Fix what the review confirms; gate again.
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
