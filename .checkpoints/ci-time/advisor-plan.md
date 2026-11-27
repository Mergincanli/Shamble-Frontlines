# Advisor plan review: CI-TIME (plan at afdb078), 2026-11-27
Worked from main b9aad01's .claude/agents/advisor.md (worktree .gate/ci5 equals main for kit files; only .checkpoints/ added).

## Checked myself
- toolchain.yml:38 is `    timeout-minutes: 15` (job toolchain); the only other timeouts in the file are the two
  Playwright `--global-timeout=300000` (205 Chrome, 212 Firefox). gate.yml's own 10/15 are untouched.
- sha256 of toolchain.yml d017b83b... and of draft-toolchain.diff cf98135b... match the plan; the diff is the one line.
- His block saved by script matches the task's summary (points 1-6).
- Push route: e8340eb is an ancestor of origin/main; remote claude/keen-tesla-v807cf already at afdb078 (fast-forward).
- Repository is public (REST: visibility public), so the cost claim holds; no tier-3 money effect.
- docs/reports files get a REPO_MAP row (main-wall precedent at line 191); ROADMAP has no CI-TIME line yet.
- tools/node_modules absent in .gate/ci5 (local gate not yet runnable there).

## Verdict: APPROVE with conditions
1. No [skip ci] on the final head (the notes-deletion commit included): gate, smoke, toolchain must complete
   success on that exact SHA.
2. Order: docs, STATUS in final wording ("merges on his go", never "merged"), notes deletion, then the
   security-auditor's read and my head review. No push after his go; his go and the merge are recorded by the next PR.
3. ROADMAP: one line (or the report states why none). STATUS and ROADMAP are on the per-tier checklist.
4. Run node tools/gate.mjs on the final head in a .gate worktree (toolkit via npm ci), or say why not; I run it at head review.
5. Evidence, honest: one-line diff (hash), YAML parse (job timeout 25), toolchain green on head, the 13-minute run of
   5c6738a cited by run id and duration from REST. "Saves a run over 15 minutes": not verified until one happens; say so.
6. Merge path is allowed: his point 4 + standing permission open the draft (REST, draft=true); his point 5 + CLAUDE.md's
   merge rule cover mark-ready and merge. Only after: his go in this chat naming the full SHA; my APPROVE and the
   security-auditor's on that SHA; gate, smoke, toolchain success on it; re-checked right before; ready via the CCR
   ready_for_review route, merge via REST with sha=<approved>; no auto-merge; a refusal goes to him, nothing worked around.
7. DECISIONS: both messages whole, by script, no retyping; nothing else in DECISIONS changes.
8. Final file list: toolchain.yml (one line), DECISIONS, STATUS, ROADMAP, REPO_MAP, the report; check
   `git diff --raw --no-renames origin/main...<head>` (modes 100644, no .checkpoints).

## Docs right-sizing
Right-sized: report + STATUS are protocol step 6; DECISIONS keeps his words; REPO_MAP's row already lists the 5-minute
stops, so adding the job limit keeps it honest. Add nothing more.

## Low
- Checkpoint commits by hand instead of checkpoint.mjs (it pushes to the local name): acceptable deviation, say so in the report.
