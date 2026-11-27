# Advisor: review of pushed head 266f42e7 (PR #72, step 2.6, risk tier 2)

Scope: the delta 9778a89c..266f42e7 (his answers of 2026-11-26 12:21:32 UTC recorded and folded in; the start block for
the new chat). Main's advisor.md read from origin/main. Main folder at 266f42e7, clean tree (`git rev-parse HEAD`,
`git status --short --branch`), so files were read there. Read only.

## What changed (git diff --stat 9778a89c 266f42e7)
docs/DECISIONS.md (+20), docs/STATUS.md (4 lines), the report, the checkpoint (1 line), build-notes (1 line),
scripts/add_1320.py (new), start-block-new-chat.md (new). Nothing under .github/, hosting/, .claude/ or the kit.

## 1. DECISIONS.md: his message byte for byte
- Python on the file: the quoted block under the top entry, with "> " stripped, is 1097 bytes, sha256
  bf833cc877e0696ad202be3e13ec9c88138ad0f27eba03a28cf23db5a56c0534 (matches the header).
- The session transcript holds exactly one entry with uuid 50bdbeb1-7142-429f-88fe-01b5ecd7ce6e: type user, not a
  sidechain, timestamp 2026-11-26T12:21:32.010Z, text identical to the quoted block (python: equal, same sha256).
- The script add_1320.py runs no git and writes only docs/DECISIONS.md (not a protected file): fits GIT-IN-NODE (a).
- The header's bold summary matches his items 1 to 6; the context is marked "Records, not his words". Nothing added
  as his. Placed as the top entry of "Decisions, newest first". PASS.

## 2. STATUS and the report
- STATUS: 120 lines (wc -l). Changed lines: the stamp (5), #72's row (26), Waiting on Mustafa lane 2 (50), Open
  risks Preview hosting (118). All lane 2's own. His five answers stated as given; item 6 in #72's row; TAG-CHECK's
  reason in Open risks. PASS.
- Report: Outcome carries item 6; TAG-CHECK (a) "untested" with the reason (branch main only, 0 tags allowed);
  Decisions needed "None open" with the five answers; For Mustafa: start the new chat. PASS, with L3 below.
- The colleague's Lows: H1 fixed (report: "Left, by the advisor agent's review (D1)"); H2 fixed (Risk 2 "must still
  hold when it comes"); H3 fixed (stamp no longer names 8c8ad0d0); H4 fixed (the rating now attaches to "The guard
  item", a bullet of its own); N1 fixed (checkpoint :101 and build-notes :422 now give CI-run creation times; checked
  by gh api: 80a9b6a8's pull_request runs 00:20:58Z (its push runs 00:20:56Z), 84a3ea44's first run 00:44:05Z).

## 3. The start block
- The five sha256 in item 5 match the files at head (sha256sum on preview.yml, preview-close.yml, wall-test.yml,
  hosting/package.json, hosting/package-lock.json): all five equal.
- Item 6's token-sitting order (a)-(e), the dates in their own fields 90 days apart and the 14-day reminder match the
  checkpoint :109-115 and the runbook :88-97, :127. 6b matches MAIN-WALL-AGAIN (a) and MAIN-WALL (b) (DECISIONS :213-216).
- The wall's detail in 6c (the environment page's settings) is already in DECISIONS, the runbook and the report; the
  file is under .checkpoints/ and is deleted before ready. No new exposure.
- The guard probe. routine-guard.mjs on origin/main: decide() sends every Bash call straight to bash() (no
  day-session early return); bash() parses `HEAD~1` cleanly (a ~ inside a word is not an unreadable form for day
  sessions); git(): no option before the subcommand, merge not in GIT_NEVER, the role-only lines skipped;
  gitArgsCheck('merge', ['--ff-only','HEAD~1']): GIT_LONG_DANGER has no merge entry, and `--ff-only` passes whether
  git's completion list holds it (exact) or comes back empty (no prefix match), so it returns ['HEAD~1'], no deny;
  then `if (!role) return mainFolderMove(...)` (dayShellWrites runs earlier in bash(); for a main session it can only
  add an ask, never a deny, so it cannot hide the prompt; not traced line by line). mainFolderMove (merge
  branch): t = 'HEAD~1'; it asks unless `isOriginMain(t)`, i.e. unless HEAD~1's commit equals
  refs/remotes/origin/main's commit (an unresolvable origin/main also asks). In the main folder it returns ask(reason);
  for a subagent, deny. So it asks on any branch EXCEPT when the main folder sits exactly one commit ahead of
  origin/main (e.g. after one checkpoint commit on a branch made from main): then it passes silently and would read as
  "the ask did not reach me" (a false alarm on the safe side). Harmless if it runs: HEAD~1 is an ancestor, so
  `--ff-only` says "Already up to date" and changes nothing; with no HEAD~1 (depth-1 clone) git fails, still harmless,
  and the guard still asks (commitOf null). Sound as a first act, with L1.
- Fit with CLAUDE.md: a worktree under .gate/ is the Never bullet's own form; the main folder stays put; pasting the
  block makes the grant his word (Lanes: "once I paste one, it is my instruction"). One account per branch needs the
  new chat on the same account as Builder V2 (M2). The local fetch refspec is `+refs/heads/*:refs/remotes/origin/*`,
  so `git fetch origin <branch>` then `git worktree add .gate/step-2-6 <branch>` creates the tracking branch.

## Findings
- M1 (Medium): his item 1 asks for "the branch and head"; the block gives the code head 84a3ea44 but the branch head
  only as "Builder V2's last push", and gives the new chat no check of it. A file cannot name the commit that carries
  it, so the fix: the block says "the branch head is the SHA Builder V2 names in the chat message that hands you this
  block"; Builder V2 states that SHA in chat; the new chat compares it with
  `git rev-parse origin/claude/optimistic-albattani-mdoonz` before anything else and stops on a mismatch.
- M2 (Medium): neither the block's note to Mustafa nor the report's For Mustafa says which account. My reading of
  Lanes' "One account per branch" for a successor chat on the same branch: it should run on Builder V2's account (his
  personal one). Not a breach; he chooses. Fix: a line in the note to him, "start it on your personal account (the
  branch's account; one account per branch)".
- M3 (Medium): the block tells the new chat to read the checkpoint first, but the checkpoint at 266f42e7 still says
  "STOPPED UNTIL HIS GUARD-OFF ANSWER" and "Waiting for his five answers" (:124-132); it does not record his answers,
  the start block or the hand-over. Fix: update the checkpoint's current step and next steps to his answers of
  12:21:32 UTC and the hand-over (CLAUDE.md Checkpoints 1).
- L1 (Low): the probe passes silently if HEAD~1 is origin/main's commit. Fix: "run it before any commit in your main
  folder" (or note that exception in item 1).
- L2 (Low): item 7 "Never push to another chat's branch" reads against item 2's grant. Fix: "other than PR #72's
  branch (item 2)".
- L3 (Low): the report's GIT-IN-NODE line drops "and never writes a protected file" (his item 4). Fix: add it.

## CI on 266f42e7 (gh api check-runs)
gate success, smoke success, toolchain in progress, push-to-branch failure (wall-test, by design), two skipped.

## Verdict for head 266f42e7
REVISE (narrow): M1-M3 in the start block, the report's For Mustafa and the checkpoint; L1-L3 ride the same push.
The recording itself (DECISIONS byte for byte, STATUS, report, the colleague's Lows) is correct. Every fix is under
`.checkpoints/` or `docs/`, so Builder V2 may push it under his item 6 (no .github/ or kit change). Not mergeable anyway:
draft, wall-test.yml in, no token sitting, no verifier; item 6 holds this chat to no .github/, kit, main-folder move
or merge. No decision open for him.

## Not verified
- The gate by me (no code in this delta; CI gate and smoke green on 266f42e7); toolchain still running.
- Whether a new cloud session may push to another session's claude/ branch. This decides whether item 2 works at all:
  if it may not, the hand-over stalls at the first push (safely, by "Push nowhere else"), and he loses a sitting. The
  platform's GitHub-access page did not settle it. Suggested line for item 2: "if the push is refused, stop and tell
  me; open no new branch or PR". Whether the guard loads in the new chat and whether its ask reaches him in auto mode
  (the probe's job).
- dayShellWrites' exact answer for `git merge` (read: it adds asks for kit writes in a main session; not traced).
- The guard was read, not run (as asked); dayShellWrites' ask on removing wall-test.yml in a worktree not traced.

## Own process slip
One gh call held a `$(echo)` substitution inside its quoted argument (a read, returned a count); every other git/gh
call ran alone with nothing in front or after.
