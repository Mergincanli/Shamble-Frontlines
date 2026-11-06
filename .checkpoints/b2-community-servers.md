# Checkpoint: b2-community-servers (B.2 on Mustafa's list: VISION_V2's community-servers note and B3's hand-over)

- **Branch:** `claude/sleepy-tesla-ofsxsr`, standing in for `cs/b2-community-servers`: this cloud session may push
  only there.
  - Worktree: `/tmp/claude-0/-home-user-Shamble-Frontlines/263eddf3-7643-5640-b1d5-39be43d3567c/scratchpad/b2`.
  - Local branch `b2-community-servers`, from main ee736ec (PR #39 merged). Pushes are fast-forwards
    (`git push origin HEAD:refs/heads/claude/sleepy-tesla-ofsxsr`; checkpoint.mjs refuses the local branch name, so
    notes are committed by hand with [skip ci]).
  - Notes live in `.checkpoints/b2-community-servers/`, committed as they go (Mustafa, 2026-11-05: no draft only in
    /tmp).
- **Mustafa's words**, word for word, in `.checkpoints/b2-community-servers/mustafa-words.md`:
  - B.2 of 2026-11-03 with his two lead lines;
  - COMMUNITY-SERVERS of 2026-11-03 (for reference: ADR 0006 OQ-0006-21 holds it);
  - the LOOP_V2 lines of 2026-11-04 the hand-over quotes, and the three VISION_V2 lines B3 handed over.
- **Scope:** docs only: VISION_V2, one clause in `docs/adr/README.md` (if REG (a)), STATUS, ROADMAP, the B.1 report's
  correction, the B.2 report. Also the docs after PR #39's merge (ee736ec, 2026-11-06 17:09:30 UTC).
- **Other sessions:** PR #40 (the netcode proof test's plan, a draft, session_0173vVo6igCNyQYBWPvhH36A) edits STATUS
  and REPO_MAP and is in conflict with main; it already records #41 in STATUS and asks Mustafa UNDER-WAY (when the
  test counts as under way: the same question as this session's A4-START). PR #41 merged at d8c485a before #39 and
  shifted the register's lines after line 31 by one.
- **Done:** step 0, the sources read and copied (`mustafa-words.md`); plan revision 1 (`plan-rev1.md`).
- **Current step:** 1, the advisor's review of plan revision 1.
- **Agents:** Opus 5.5 at high, at most 3 at a time.

## Next steps
1. Open the draft PR for the branch, subscribe to it, and run the advisor on plan revision 1 (it reads main's
   `.claude/agents/advisor.md`); save its verdict as `advisor-plan-rev1.md`.
2. On APPROVE: build in the main session (VISION_V2, the register clause), E1-E4 scripts in the notes folder, the gate.
3. docs-keeper's pass (STATUS, ROADMAP, the B.1 report's correction); E5-E7.
4. One review round (the verifier, game-designer, infra-engineer), one fix pass, then the report and the notes'
   deletion in the last commit.
