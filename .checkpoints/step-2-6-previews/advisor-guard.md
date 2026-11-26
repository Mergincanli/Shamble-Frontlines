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
