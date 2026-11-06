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
- **Done:** step 0, the sources read and copied (`mustafa-words.md`); plan revision 1 (`plan-rev1.md`), committed
  8ee1b5b [skip ci]; draft PR #42 opened and subscribed.
- **Key numbers (VISION_V2 at ee736ec, before):** 88 lines; tags `[DECIDED]` 10, `[DEFAULT]` 3, `[OPEN]` 4,
  `[LATER]` 4, `[IDEA]` 0 (the header's tag line counts one of each of the first four). Citations of the lines B.2
  changes: §7's faction question is the owner named in ADR 0005:686, README:120 and r1-simulation-core.md:2806 (kept,
  pointer only); "VISION_V2 §5" citations are about guest play and login (line 58, unchanged).
- **Done:** step 1, the advisor's review (`advisor-plan-rev1.md`): APPROVE WITH CONDITIONS (9). PLACE (a), TAG (a),
  HANDOVER (b) with C3-C4, REG (a) with C8, DK-41 (b) narrowed (C5: plan premise false, #40's STATUS still calls #41
  "waiting for his go"; one clause for #41's merge in STATUS:3), HEADER (a). C1 the pointer line names the open
  question, not what the drafts "keep"; C2 the trust rule answers nothing else (R2-ROOM-OQ7, R6-TEL-OQ2, rented
  official servers); C6 STATUS:446 cites README:294-298; C7 ROADMAP:99 "taken by B.2 (PR #42)"; C9 E4 adds
  line-number citations into VISION_V2 (r1-simulation-core.md:3301 and :3535 cite VISION_V2.md:54, already stale:
  report it, leave the research file).
- **Found (not B.2's to record):** #40's branch (head 1b5136e) records Mustafa's answers of 2026-11-06 (its plan §0.4,
  "his words in short", commit 411824a at 09:33 UTC): UNDER-WAY (a), and U0 committed at 09:40 UTC (f6886bf), so the
  kit freeze lifted and A4 may start after B.2. Its STATUS edits (:39-41, :165, :431, :462) carry this; B.2 leaves the
  A4 lines' substance to #40 (as DK-41) and tells Mustafa in chat that the earlier ask to answer UNDER-WAY was moot.
- **Current step:** 2, the build in the main session (VISION_V2, the register clause), then E1-E4.
- **Agents:** Opus 5.5 at high, at most 3 at a time.

## Next steps
1. Open the draft PR for the branch, subscribe to it, and run the advisor on plan revision 1 (it reads main's
   `.claude/agents/advisor.md`); save its verdict as `advisor-plan-rev1.md`.
2. On APPROVE: build in the main session (VISION_V2, the register clause), E1-E4 scripts in the notes folder, the gate.
3. docs-keeper's pass (STATUS, ROADMAP, the B.1 report's correction); E5-E7.
4. One review round (the verifier, game-designer, infra-engineer), one fix pass, then the report and the notes'
   deletion in the last commit.
