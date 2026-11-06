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
- **Done:** step 2, the build (this commit; Mustafa at 90% usage asked to save): VISION_V2 (header line :6-7; pillar 5
  replaced by his Factions line, HANDOVER (b); pointers on §5's account line and §7's faction question; the note at
  the end of §5 with C1's wording) and one clause appended to README:160 (C8). Quick checks PASS: no changed line over
  120; tags `[DECIDED]` 10→12, `[OPEN]` 4→3, `[LATER]` 4→5, the rest unchanged (E3's expected change); each of his
  six pieces found once in the joined text (E1 quick); README one line changed.
- **Done:** CI on 67978c2: gate and smoke green (both runs). After the usage reset (Mustafa: "Try again", about
  23:20 UTC): `e-checks.py` (E1-E4, read-only) PASS, every condition it covers included (C1, C3, C8, C9; the only
  line-number citations into VISION_V2 outside STATUS and ROADMAP are r1-simulation-core.md:3301 and :3535, left
  unedited); `node tools/gate.mjs ee736ec --no-smoke` PASS, symbols +0 -0 (`gate-build.txt`); repo map OK (113
  entries). tools/node_modules copied from the b1 worktree (same lockfile as main; ignored by git).
- **Done:** step 3, docs-keeper's pass (`docs-keeper.md`; a general agent reading main's docs-keeper.md): STATUS
  (header #39 and #41, Phase, B.1's row merged with the advisor's APPROVE 6682184695, B.2's row, B3's row "taken by
  B.2", C6's line numbers :119, :294-298, :202, :192-193, B.2's queued words removed with a pointer, the "From B.2"
  follow-ups C8 and C9), ROADMAP (v2-next queue, B.2's entry, B.1 MERGED, B3 "taken by B.2"; it also moved the R7
  entry into Build history to keep 8-12 recent entries, its role's rule), the B.1 report's correction after the
  merge. Its checks: gate PASS (+0 -0), repo map OK, e-checks PASS, widths OK.
- **Mustafa's VFX editor item (2026-11-06, 23:27 UTC, "For the next docs pass")** arrived during step 3: saved word
  for word (`mustafa-2026-11-06-vfx.md`, a79fef1) and queued in STATUS's "Queued, in Mustafa's words" (the main
  session; its quote unwraps to his text exactly) as the next docs change-set after B.2, then A4; Phase and ROADMAP's
  queue say so. Not built in B.2 (its scope stays as approved); the advisor's final-head review sees the entry.
- **Done:** step 4, the review round on 26a5455 (general agents reading main's role files): the verifier PASS WITH
  NOTES (V1 Medium: ADR 0005's VISION_V2 line ranges at :27, :48-50 move; V2-V6 Low: the report not yet written, A4's
  freeze tense, the VFX order is the session's reading, the 2026-11-03 list is pointers only, B3's line numbers
  undated); game-designer PASS WITH NOTES (GD-1 Low: rented official private servers vs the trust rule; GD-2 Low: "the
  war around the matches" not in VISION_V2 §7); infra-engineer PASS WITH NOTES (IE-1 Low: the trust rule adds no
  field now, part 12 stands; IE-2 Low = GD-1). Nothing Critical or High: no skeptics, no second round.
- **Done:** step 5, the one fix pass (this commit): STATUS and ROADMAP wording for V3-V6; follow-ups for V1 (with a
  fifth range, ADR 0005:2550, found by the widened E4), IE-1, GD-1/IE-2 and GD-2; e-checks.py's E4 widened to line
  ranges (PASS). V2 is met by the report. For the report: PR #40's two stale #41 lines (its STATUS:31 and :173).
- **Current step:** 6, the report (`docs/reports/2026-11-06-b2-community-servers.md`, the advisor-report skill).
- **Agents:** Opus 5.5 at high, at most 3 at a time.

## Next steps
1. (Done: PR #42 opened and subscribed; the advisor's verdict saved; the build committed.)
2. E1-E4 scripts and the local gate; fix anything they find.
3. docs-keeper's pass (STATUS, ROADMAP, the B.1 report's correction) under conditions C5-C8: #39 and #41 merged in
   STATUS:3 (times from the GitHub API), B.1's row merged, B.2's row, B3's row and ROADMAP:99 "taken by B.2 (PR #42)",
   the five line numbers (:99/:447 → :119, :446 → :294-298, :465 → :202, :466 → :192-193), B.2's queued words
   removed with a pointer, a STATUS follow-up for ADR 0006's OQ-0006-21 to point at VISION_V2 §5; the A4 lines'
   substance left to #40; then E5-E7.
4. One review round (the verifier, game-designer, infra-engineer), one fix pass, then the report and the notes'
   deletion in the last commit.
