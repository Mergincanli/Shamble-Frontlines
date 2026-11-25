# Checkpoint: cs4-walk
- **Goal:** the plan of change-set 4, "Walk" (the build plan, `docs/reports/2026-11-17-build-plan-slice-1.md`,
  lines 250 and 334-358), with the recording as its first step by his ORDER-3C (c); then the advisor agent's review;
  then stop for his go on the plan. Nothing is built before that go · plan: `.checkpoints/cs4-walk/plan.md` (not
  written yet) · his words: `.checkpoints/cs4-walk/mustafa-2026-11-25-takeover.md` (the takeover block; to
  `docs/DECISIONS.md` with the first pull request) and `mustafa-2026-11-25-order-3c-answer.md` (ORDER-3C (c), its
  four conditions, the plan's asks, the Hornet entry for `docs/UNITS.md`)
- **Branch:** `claude/cool-pasteur-oiqk1r` on GitHub (it stands in for `cs/4-walk`). Local: `cs/4-walk` in the
  worktree `.gate/lane1-takeover` (the folder keeps its first name); pushed with
  `git push origin cs/4-walk:claude/cool-pasteur-oiqk1r`, alone in its command; never a plain `git push`. The
  session's start folder is fast-forwarded to main 311c0484 (so the fixed guard loads) and is never switched ·
  **Base:** origin/main 311c0484f7a9697ae33e7a76a8cf0b507a2837c4 (taken in by merge commit 1802856)
- **Last update:** 2026-11-25 19:05 Amsterdam

## Steps done
1. The takeover (2026-11-25): checks told to him in the first answer; ORDER-3C asked
   (`question-2026-11-25-order-3c.md`, at ec9b3a6d under its first folder name) and answered by him: **(c)**, with
   four conditions (2a no proof thinner than the build plan has it; 2b the recording built and green before any
   movement code; 2c every item named for 3c placed in one of the two parts, by name; 2d the fallback: the
   recording alone as a small change-set before Walk, told to him before anything moves).
2. Main 311c0484 (the merge of #69, the guard fix) taken in: merge commit 1802856, pushed. The start folder
   fast-forwarded to it (`git merge --ff-only origin/main`, the form CLAUDE.md allows there); its guard file equals
   main's (sha256 710dd81e...f568), its tests pass there (1,247 of 1,247), a probe was refused by the hook after.
3. His answer saved by script (2,728 bytes, sha256 18fd8bb4...f428, sent 2026-11-25T16:59:52.682Z); the eight-line
   answer given in the chat, with the ask for his Testlines Export's JSON (his point 8b).
4. A read-only copy of his Testlines sandbox (branch `claude/sleepy-edison-ygmwfe` at be608cad; 3c07269 when his
   Advisor read it) is in the session's scratch folder, outside the repository: README and `src/params.ts` read for
   names, units and meaning only; no code and no value is taken from it.

## Current step
The designs for the plan, by agents writing notes into `.checkpoints/cs4-walk/` (at most 3 at a time): the
recording, the command and the snapshot (netcode-architect); the content crate, the block, the page and the cut
(engine-architect); the collision library, our own movement, the built-file check for the fused multiply-add, the
time unit (rust-reviewer); then the movement rules from the old game and SLICE-FEEL (game-designer). The main
session reads the code and writes the Testlines part (his point 8) meanwhile.

## Next steps
1. Read each agent's notes when it ends with DONE; where two designs differ, choose and say so in the plan.
2. Write `.checkpoints/cs4-walk/plan.md` in the shape of change-set 3's plan (`cs3-plan.md` at 80a51e42): in one
   screen; what he sees; before any edit; the steps with files and lines (code and tests apart), sized bottom-up,
   then at 1.8 times, with a stop line; one stop or two, and the earliest point he can play; the 3c items placed by
   name; the built-file check for the fused multiply-add (MUL-ADD); 3B-RESUME-FLASH as his decision; SLICE-FEEL
   priced and placed; lane 2's list (issue #59) row by row; his Testlines values (point 8, a to g); 3B-ONE-PER-TICK;
   the pass marks placed at Walk, each with its test and its planted defect; risks; not in it; decisions needed;
   his browser checklist.
3. The advisor agent's review of the plan (from main's `.claude/agents/advisor.md`); fold its conditions in.
4. Push; send him the plan as a file; board: one line, then the state that says the chat waits for him. Stop.
5. For the first pull request's docs pass (not now): his two blocks into `docs/DECISIONS.md` by script; change-set
   3b's owed docs pass (STATUS says so); the Hornet entry in `docs/UNITS.md` section 0 by script from his answer
   (the unit decided, the numbers a sketch; a clash is listed for him, never settled).

## Open findings
- The checkpoint script cannot push from this worktree (it pushes the local branch's own name; this chat may push
  only to `claude/cool-pasteur-oiqk1r`): notes are committed by path and pushed by hand, and the background saver
  cannot run as it stands · this chat, 2026-11-25 · open, for the plan's "before any edit".
- The session's worker restarted once (about 18:55 Amsterdam); nothing was lost (the notes were on disk).

## Key numbers
- 3c's parts as change-set 3's plan sized them: the recording 275 lines; the link simulator 595 lines.
- Walk's card: 3,900 planned lines (the build plan, line 250), before SLICE-FEEL's additions.

## Decisions taken
- ORDER-3C (c) · Mustafa · 2026-11-25, 16:59:52 UTC.
- His rules for this chat that are not on main yet: no CI, kit or gate file is written here; never a push to
  another chat's branch; until a second guard fix is merged, git and gh always start their own command, with no
  other program in front of them (this session and every agent).
- The board add-on is active for this chat (since 2026-11-25).

## Running work
- None yet (the agents are started next; each is listed here with its notes file when it runs).
