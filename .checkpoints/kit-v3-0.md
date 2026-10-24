# Checkpoint: kit-v3-0
- **Goal:** v2 phase 0, step 1 (brief §2): model policy, checkpoints, efficiency rules, night shift v2, crew v2. Plan:
  `.checkpoints/kit-v3-0/plan.md`.
- **Branch:** claude/bold-faraday-i49gug (stand-in for cs/kit-v3-0) · **Base:** main 48ffb51
- **Last update:** 2026-10-24 23:50 Amsterdam

## Steps done
1. docs/VISION_V2.md added, byte-identical to the attachment (SHA-256 22664025…3ab). Commit: this checkpoint's commit.
2. Plan written (`.checkpoints/kit-v3-0/plan.md`).

## Current step
3. Plan review: advisor (general agent reading main's advisor.md) plus a brief-trace critic.

## Next steps
4. Apply the verdict; commit the brief copy if approved.
5. Build: CLAUDE.md, CHECKPOINTS.md, checkpoint.mjs + test, settings.json, NIGHT_SHIFT.md, 6 new agents, 7 updated
   agents, 2 skills, security guidance line, REPO_MAP, research README. Checkpoint after each.
6. Evidence: gate, script test + mutation, trace table, contradiction sweep, REPO_MAP check.
7. Security review of the final diff; fix or defer.
8. Report; delete `.checkpoints/kit-v3-0*`; PR draft → ready; advisor PR review; merge (Mustafa's go given in chat).
9. After merge: routines (brief §3); R7 on this branch restarted from main.

## Open findings
- PR #25 (docs-after-kit-v2-2) is open since 2026-10-17 and edits STATUS and ROADMAP: kit-v3-0 leaves both alone.

## Key numbers
- none yet

## Decisions taken
- Pin `model: claude-opus-5-5` in agent files (claude-code-guide: frontmatter accepts full IDs; the `opus` alias
  follows the latest Opus; Opus 5.5's default effort is medium, so every agent states its effort).

## Running work
- R7 (legacy inventory) runs alongside in a local worktree, local branch `r7-legacy-inventory`; pushed after this
  PR merges.
