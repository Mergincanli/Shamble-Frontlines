---
name: changeset
description: 'Run one SHAMBLE FRONTLINES change-set end to end: branch, plan, approval, implement, gate, review, commit, report. Use when Mustafa starts a task with /changeset.'
argument-hint: '<goal in one sentence>'
disable-model-invocation: true
---
# Change-set: $ARGUMENTS

Follow the protocol in CLAUDE.md exactly. Do not skip or merge steps.

1. **Branch.** Create `cs/<short-slug>` from `origin/main` (`git fetch origin main`, then
   `git switch -c cs/<short-slug> origin/main`, or a worktree: CLAUDE.md, Never). If the working tree is dirty, stop and
   ask.
   Start the checkpoint `.checkpoints/<short-slug>.md` and keep it current after every step (`.claude/CHECKPOINTS.md`).
2. **Understand.** Read docs/STATUS.md, docs/START_HERE.md and docs/MODULE_MAP.md; grep the owning file(s). Use Explore
   subagents for wide searches so your context stays clean.
3. **Plan (plan mode), then STOP.** Present:
   - Goal and the player-facing reason · what is out of scope
   - Systems and files touched · exact symbols to add, change or remove
   - Risks (layering, load order, hot paths, art direction, saves) and how you'll contain them
   - Change type(s) and the evidence you'll produce · risk tier (CLAUDE.md) · reviewers you'll call
   - What Mustafa must check in Brave (if anything visual)
   Wait for explicit approval. Revise and re-present if asked.
4. **Implement** the smallest correct change in the main session. Keep edits reviewable; no drive-by refactors.
5. **Gate.** Run /gate after every step. On the final diff, delegate to the verifier agent with the plan and base ref,
   and again after fixes. Fix and re-run until PASS. The gate and the verifier sit outside the review budget: their
   re-runs are not rounds, and a verifier FAIL counts as High (CLAUDE.md's severity scale).
6. **Review.** Call the reviewers that match the change type and its risk tier (security-auditor on every risk-tier-2
   change), in parallel (at most 3 at a time, `.claude/CHECKPOINTS.md` §4), on the final diff, within the review budget
   (CLAUDE.md: at most two rounds, the second on what changed; a third only for an open Critical or High). Every agent
   runs Opus at the effort `CLAUDE.md` "Models and effort" gives and labels each finding on CLAUDE.md's severity scale.
   Address each finding, or defer it in writing with a reason: below the third-round bar a finding becomes a STATUS
   follow-up, and a Low never triggers a push on its own.
7. **Commit** on the branch: what changed, why, and the evidence summary. Do not merge visual work before Brave approval.
8. **Report.** Run /advisor-report (it folds the checkpoint in). Then ask docs-keeper to update STATUS, ROADMAP and the
   repository map from that report. Delete the checkpoint in the last commit before the PR is marked ready.
