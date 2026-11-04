# Checkpoint: kit-v3-2 (A2 on Mustafa's list: CHECKPOINTS.md rules for every session; the TOY-BOX rewording)

- **Branch:** `claude/sleepy-tesla-ofsxsr`. It stands in for `cs/kit-v3-2`: this cloud session may push only there.
  Worktree: `/tmp/claude-0/-home-user-Shamble-Frontlines/263eddf3-7643-5640-b1d5-39be43d3567c/scratchpad/a2`, local
  branch `kit-v3-2`, from main 8bcc65e (PR #36 merged). The push is a fast-forward from a32e4c0. Checkpoint commits
  are made by hand, marked `[skip ci]`.
- **Mustafa's words** (2026-11-03, item A.2), word for word: `.checkpoints/kit-v3-2/mustafa-2026-11-03-list.md`.
- **Current step:** 2, the build (his answers given).
  - Plan revision 1: `.checkpoints/kit-v3-2/plan-rev1.md`; its writer's notes: `plan-rev1-notes.md`.
  - The advisor: APPROVE with 11 conditions, in `.checkpoints/kit-v3-2/advisor-plan-rev1.md`. "Build only after
    Mustafa answers the four decisions (or says 'recommendations')."
  - **Mustafa's answer (2026-11-04 ~20:25 UTC), word for word:** "recommendations” in V3." Read as: all four
    recommended options, given in this session (Planner V3): AGENT-CAP (A), every running agent in a session,
    workflow agents included; SCRATCH-HOME (A), notes folder for text and small scripts, `.gate/<task-slug>/` for
    builds and binaries; SAVER-CHECK (A), at every checkpoint update while agents or a workflow run; LOOK-LINES (B),
    the four "look not decided" lines reworded too, pointers only. The plan was written for these options.
  - From now on this session runs at most 3 agents at a time (AGENT-CAP A), before A2 merges too.
- **Agents:** all Opus 5.5 at high, at most 3 at a time (the plan's own rule, under AGENT-CAP A).

## Next steps
1. Mustafa's four answers, then the plan with the 11 conditions and his answers folded in.
2. The build: C1–C12, M1–M3, T1–T5 (+ condition 8's line), S1–S4, K1, N1 and condition 2's advisor.md clause.
   Then docs-keeper: STATUS, ROADMAP, REPO_MAP, and condition 6's three `.gate/` lines.
3. Evidence E1–E10. Then the review wave: art-director, security-auditor (narrow), the verifier. Then fixes.
4. The report; delete `.checkpoints/kit-v3-2*` in the last commit (not `[skip ci]`); mark the PR ready; the
   advisor's final-diff review; Mustafa's go (a rules change: needs-mustafa).
