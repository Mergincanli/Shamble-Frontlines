# Checkpoint: the CI Builder paused (session_019YNMUUemXge56777MiUWFz), 2026-11-27
His word (saved by script: `ci-builder-pause/mustafa-2026-11-27T1445-go-78.md`): this account's usage is used up until
Tuesday 1 December, 20:00 (Amsterdam); after the merge, save this checkpoint, push it, stop scheduled tasks, start
nothing new.

## Where things stand
- #78 (CI-TIME) merged at the expected head 312a763d2894a743c64bc186607c3301484c0757 on his go of 14:45 Amsterdam:
  main b8f02b0149cc928964e54e194cd9c4d68f52bd94 (parents b9aad01, 312a763). The toolchain job's limit is now 25 minutes.
- Before it, merged: #71 ci-after-3a (07d673b), #74 ci-preview-handover (3b949c1a).
- Branch `claude/keen-tesla-v807cf` (this chat's one push branch): this commit sits on main b8f02b0, pushed as a
  fast-forward. Worktrees: `.gate/ci3`, `.gate/ci4`, `.gate/ci5` (this one, local branch `cs/wrapup`), and the advisor
  agent's scratch worktree `.gate/ci5/.gate/pr-78`; none holds unpushed work. The main folder is at b9aad01 (fast-forwarded
  on his word of 12:50); `git merge --ff-only origin/main` there loads main's newest guard.
- Scheduled tasks: none active (list_triggers, enabled only: empty). The board: the CI-TIME thread set to done.
- Running: no agent, workflow or background shell.

## Open (for the next session of this chat or the next docs pass)
1. DECISIONS: his word of 2026-11-27, 14:45 Amsterdam (the saved file above) is not yet in `docs/DECISIONS.md`; copy it
   by script (the copy script pattern: `git show 0a4a198:.checkpoints/ci-time/decisions.mjs`).
2. STATUS: the #78 row still reads "draft"; it is merged (b8f02b0); move it as the other merged rows were. CHROME-STOP
   (a) as a follow-up line: decide when Chrome's tests near 4.5 minutes (infra-engineer's Low, `infra-engineer-ci.md` at
   0a4a198).
3. The CI-TIME report's two Lows (the advisor agent's read of 312a763): its "Plan and approval" calls the ready-and-merge
   step met before it happened (now true); "Why (his words)" is a paraphrase (his text is byte-for-byte in DECISIONS).
4. Earlier follow-ups still open, in `docs/STATUS_ARCHIVE.md`: CUT-ESCAPE-FIX (the cut's code as its own throwaway
   user), FF-DRAW (Firefox's drawing tests in CI, with step 2.3, its own question), FF-SPEC-COMMENTS, PH-IE-4 (a re-run
   and the artifact name), PH-KEEP-NAMES, and the merge-time items E1-E4 of #74's docs merge.
5. Nothing waits on him from this chat now.

## Standing rules (his)
git and gh always start their own command, with no other program in front of them. In a shell command every file
written, moved, copied or deleted is spelled out with its full path (or the Write and Edit tools). Every CI write is
asked of him with its exact text unless he approved the text in advance; the mode is read right before the write.
