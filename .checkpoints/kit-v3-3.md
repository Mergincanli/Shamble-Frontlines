# Checkpoint: kit-v3-3 (A3 on Mustafa's list: the guard's follow-ups)

- **Branch:** `claude/sleepy-tesla-ofsxsr`, standing in for `cs/kit-v3-3`: this cloud session may push only there.
  Worktree: `/tmp/claude-0/-home-user-Shamble-Frontlines/263eddf3-7643-5640-b1d5-39be43d3567c/scratchpad/a3`, local
  branch `kit-v3-3`, from main 12d7961 (PR #37 merged). Pushes are fast-forwards. Checkpoint commits are made by hand
  and marked `[skip ci]`.
- **Mustafa's words**, word for word: `.checkpoints/kit-v3-3/mustafa-words.md`. His three items:
  1. ask before kit and gate edits in day sessions;
  2. the `sk-` key after `%20`, `%2C` or `%3A` (checkpoint.mjs's secret check);
  3. read-only GitHub Actions tools (such as `actions_get`) for pr-advisor.
  security-auditor reviews A3. His two additions (2026-11-04, after PR #37's merge):
  4. the worktree rule from the freeze, into the rules: never switch a session's main folder to another branch, a
     PR's head or an old commit; use a separate worktree (`git worktree add`). Why: switching loads that branch's
     `.claude/settings.json` and hooks into the running session;
  5. the deferred points and everything in the session's scratchpad queue file into the repo, in STATUS's
     follow-ups, alongside A3's plan. The queue files are now in `.checkpoints/kit-v3-3/queue/`: docs-pass-items.md
     (the queue file), a4-spec-2026-11-04.md (A4 in his words), mustafa-2026-11-03-list.md (his list: B.1 and B.2
     still queued) and a2-items.md (where A3's items came from).
- **For the plan to weigh** (STATUS "Open bugs and risks", A3's entry; the advisor's deferred F-1 from PR #37):
  - `git rev-list` refused for routines;
  - python allowed on main's tracked scripts but missing from the docs;
  - the advisor has no Write tool, so in a routine it returns its notes in its report;
  - an allow-list entry for `node .checkpoints/*`;
  - F-1: the trust rule misses a done block whose output was lost with a container.
  Out of scope unless the plan argues otherwise: the kit-v3-1 Windows test follow-up (its own change-set, advisor
  condition C6).
- **Current step:** 2, the build, part 1 (a workflow: E10's doc quotes, plan revision 2 with the conditions, then
  build step 2, the STATUS follow-ups first).
  - Plan revision 1, with the critic's points folded in: `plan-rev1.md` (the draft before the critic:
    `plan-rev1-draft.md`; the critic's notes: `critic.md`; the reviser's: `reviser.md`).
  - The advisor's verdict: APPROVE with conditions C1-C12 (`advisor-plan-rev1.md`, 2026-11-05 ~00:40 UTC). Two
    decisions for Mustafa: WT-NIGHT (the night shift's exception to his worktree rule; recommendation (a)) and
    PC-MODE (information only). C2: WT-NIGHT is built last, after his answer; everything else is built first.
  - Mustafa (2026-11-05, before he went to sleep): "continue with the work (take the lead)".
  - The planning runs: wf_d8f3b3e4-0f7 (stopped once to widen it to items 4 and 5, once by the usage limit, once by
    a container restart; resumed each time from the notes and the journal).
- **Agents:** Opus 5.5 at high, at most 3 at a time.
- **On a usage limit** (Mustafa, 2026-11-05, word for word: "Just wait for the next reset, I can't ask to raise the
  org limit"): wait for the reset the error names, then resume from the notes and the workflow's journal.

## Next steps
1. Plan revision 1; the advisor's plan review (a general agent reading main's advisor.md).
2. If the advisor escalates: the decisions to Mustafa, then wait. If APPROVE: build in this worktree.
3. Gate and evidence; security-auditor (and the verifier) on the final diff; the report; delete the checkpoint;
   ready; the advisor's final-head review; `needs-mustafa`; his go.
