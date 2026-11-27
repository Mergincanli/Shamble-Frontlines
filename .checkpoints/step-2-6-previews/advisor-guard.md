# advisor (general agent reading main's advisor.md): the guard process problem, step 2.6, PR #72, head 84a3ea44
2026-11-26. Read only. No guard-bypass detail beyond B15/B16.

## Plan (blocks)
G1 facts: B15/B16 claims checked against files, git, the session record (get_session), the platform doc.
G2 counts: process-git-calls.md (53 = 31 + 22), the replay's arithmetic.
G3 the 21:00:05 move: what it brought (all protected and gate paths).
G4 session scripts that run git or write protected files inside an interpreter.
G5 decisions for Mustafa (GUARD-OFF, GUARD-KIT, GIT-IN-NODE, others).
G6 verdict, stop or wait, not verified.

## G1 facts (checked)
- Two repositories: get_session lists two sources (Shamble-Frontlines, shamble-preview-check); mode auto. HOLDS.
- add_repo calls in the transcript at 2026-11-15 20:01:46.769Z (and 20:01:54, 20:02:25). Last real hook attachment:
  2026-11-15 20:02:36.203Z (PreToolUse:Bash). Later "hook" matches are only text quoting it. HOLDS.
- New corroboration, not in B16: the transcript folder moved from ~/.claude/projects/-home-user-Shamble-Frontlines
  (last write 2026-11-15 20:07) to ~/.claude/projects/-home-user (from 20:08): the session's project root became
  /home/user, so the repository's .claude/settings.json stopped being the project's settings.
- "No hook trace" is weak alone: most allowed calls left no trace even while the guard ran (only 3 PreToolUse
  traces 16:40-20:02 on 11-15). What carries it: 84 replayed asks with no trace, the folder move, the platform page.
- Platform page (read_documentation session.permissions), exact words: "only a session with one repository reads it"
  (B16 :367 paraphrases as "only [by] a session with one repository": quote it exactly). The same page: "Cloud
  sessions run file edits without asking in every mode" and the modes are Accept edits, Plan, Auto (no Manual).
- 17:15:56 message (transcript, assistant): "Guard: yes. The hooks load the guard from my main folder at every call,
  and that folder now equals main with the fix." He asked that exact question: DECISIONS :307 (18:15 item 2), :354
  (17:05 item 4). Wrong answer to a direct question of his = High (CLAUDE.md: "evidence that does not hold").
- 21:00:05 call (transcript): cd + git status + git merge --ff-only origin/claude/optimistic-albattani-mdoonz;
  result at 21:00:05.360 (0.21 s); "behind 31". HOLDS.
- Classifier refusal 2026-11-25 15:30:10 ("Destructive or Unwanted Actions", result 15:30:18): HOLDS.

## G2 counts
- process-git-calls.md: A rows :28-58 = 31, B rows :59-80 = 22, C rows :81-83 = 3. 53 HOLDS as counted. It is a
  floor: agents not scanned; git inside scripts (kit or session) not counted; B's "git && git" (3 calls) depends on
  reading "no other program in front" as covering another git (the checkpoint's practical rule does).
- Replay (scratchpad process-guard-replay.md :3): 1,582 of 1,649, 84 ask, 3 deny, 67 GitHub writes not replayed. HOLDS.
- build-notes.md :375 WRONG: "Main-folder asks: 10 = 4 in the test repository, 3 ..., 2 ..., the 15:30:10 ..., and
  21:00:05" sums to 11. The test-repository asks are 3 (2026-11-15 20:09:44, 20:58:17; 2026-11-18 11:10:29).
- build-notes.md :376-377: the two plain merges (11-18 22:57:55, 11-24 22:49:38) ran in the main folder with no cd;
  his words (DECISIONS :1750) name "a plain merge of main into your branch", not the folder. Same kind of move as
  21:00:05 (not an allowed form); B16 must not present them as covered by his instruction.

## G3 the 21:00:05 move
- 4fda80a4..e3b3642b (31 commits): toolchain.yml, docs/DECISIONS, REPO_MAP, ROADMAP, STATUS, STATUS_ARCHIVE, a new
  report, packages/e2e/playwright.config.ts. No .claude/, CLAUDE.md, AGENTS.md, gate file. toolchain.yml in
  e3b3642b equals main 07d673b0's (diff empty). HOLDS, with one wording fix: e3b3642b is the session's own merge
  commit (its conflict resolution of the docs), so "brought main's reviewed commits" should read "main's reviewed
  commits plus this branch's own merge commit (docs only)".
- At head 84a3ea44: .claude, CLAUDE.md, tools/gate.mjs, tools/smoke.mjs equal origin/main (diff empty).

## G4 scripts and commits
- Non-merge commits with this session's trailer touching protected paths since 2026-11-15 20:01: only 9ded5a87.
  HOLDS for non-merge commits on local refs; e3b3642b (merge) also lists toolchain.yml (main's side).
- place_u2.py :3 says "The guard asks Mustafa before this write": false in this session (and Python writes are
  never seen by the guard); the checkpoint already admits the write went unseen.
- verify71.py :6 runs `git show` via subprocess (read-only): a git call inside an interpreter (GIT-IN-NODE).
- guard_offline.mjs / guard_replay.mjs import .claude/hooks/routine-guard.mjs from the root (read only by design).

## G4b the three checks (after the advisor tool)
- Live asks while the guard WAS loaded (one repository, auto mode): 2026-11-15 16:40:32 (result 16:40:55, 23 s),
  16:41:41 (result 16:42:01, 20 s), 20:02:36 (result 20:02:39, 3.5 s); each hook printed permissionDecision "ask".
  The transcript has no permission-prompt record type at all, so whether he saw them is unknown; 3.5 s is unlikely
  to have waited for a person. Mode: "auto" on every one of 147 mode-bearing entries, 2026-11-15 15:33 to now.
  So a one-repository chat in auto mode may load the guard and still settle its asks without him: the security-
  auditor's Critical branch (round2-security-auditor.md :68-69) is not ruled out; it is the open question.
- GitHub tool writes since 2026-11-15 20:01:46 (deduplicated by call id): actions_run_trigger 26,
  update_pull_request 14, create_pull_request 13, add_issue_comment 3, merge_pull_request 2 (#66 11-25 11:10, #67
  11-25 17:12) = 58 (B16 says 67: not reconciled; counting method differs). No create_or_update_file, push_files or
  delete_file: no file reached GitHub outside git. "Only 9ded5a87" holds for file content.
- The two `git switch -C` refusals (2026-11-18 18:23:02, 19:00:12): transcript cwd /home/user/shamble-preview-check
  (the test repository). B16 HOLDS.
- His decisions after 17:15:56: none quotes the guard, but the plan his go of 19:36:53 approved says (plan.md
  :113-114) "In front of these today stand only the kit's rules: tier 2 for .github/ and hosting/, and the guard's
  ask before a .github/ edit", and MAIN-WALL (b) sits on that paragraph. In this chat that ask did not exist.
- Context: get_session shows 898,042 of 1,000,000 tokens used, autocompact threshold 901,600: this chat is at the
  compaction edge. Weighs for a handover from the checkpoint.
- Platform fit: the security-auditor's "run in Manual or Accept-edits mode" (:73) does not fit the cloud page
  (no Manual; "Cloud sessions run file edits without asking in every mode"). Keep it out of the decisions.

## G5 decisions for Mustafa (final wording in the handback)
- GUARD-OFF (draft kept, improved): (a) new one-repository chat on this branch, after this chat's report and docs
  pass; its first act a harmless guard ask while he watches: he sees a prompt and answers no; if no prompt reaches
  him, that chat does no risky step and works as (b). (b) stay here, in-chat asks. Recommend (a).
- MAIN-WALL-AGAIN (new): does MAIN-WALL (b) stand now that the guard's ask did not stand in front in this chat?
  Recommend: it stands only to the token sitting, where his own rule already re-asks him if the wall is not merged.
- GUARD-KIT (draft kept, sharpened): tier-2, plan first, the Guard fix chat: show the guard is live at start, find
  whether a hook ask reaches him in auto mode (11-15 evidence), and a rule: no second repository in a chat that
  writes this repository. Recommend yes, queued after the wall.
- GIT-IN-NODE: (a) main's tracked kit scripts (checkpoint saver, gate, map check) allowed, session-written scripts
  never run git or write protected files; (b) nothing runs git inside another program; (c) all allowed. Recommend (a).

## G6 verdict
ESCALATE, with REVISE conditions on B15/B16. Stop now: U6, any .github/ or kit write, any main-folder move beyond the
three forms, the token sitting, the merge, until GUARD-OFF is answered. The correction of "Guard: yes" goes to him
now (High: a direct question of his, DECISIONS :307 and :354, answered wrongly), not with the report. The report,
the docs pass and notes may go on, each git call alone.
