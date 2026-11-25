# Checkpoint: cs4-walk
- **Goal:** the plan of change-set 4, "Walk" (the build plan, `docs/reports/2026-11-17-build-plan-slice-1.md`,
  lines 250 and 334-358), with the recording as its first step by his ORDER-3C (c); then the advisor agent's review;
  then stop for his go on the plan. Nothing is built before that go · plan: `.checkpoints/cs4-walk/plan.md`
  (draft 1) · his words: `.checkpoints/cs4-walk/mustafa-2026-11-25-takeover.md`,
  `mustafa-2026-11-25-order-3c-answer.md`, `mustafa-2026-11-25-export-block.md`, and his Export
  `testlines-export.json` (all by script; the three messages go into `docs/DECISIONS.md` with the first pull request)
- **Branch:** `claude/cool-pasteur-oiqk1r` on GitHub (it stands in for `cs/4-walk`). Local: `cs/4-walk` in the
  worktree `.gate/lane1-takeover` (the folder keeps its first name); pushed with
  `git push origin cs/4-walk:claude/cool-pasteur-oiqk1r`, alone in its command; never a plain `git push`. The
  session's start folder is fast-forwarded to main 311c0484 (so the fixed guard loads) and is never switched ·
  **Base:** origin/main 311c0484f7a9697ae33e7a76a8cf0b507a2837c4 (taken in by merge commit 1802856)
- **Last update:** 2026-11-25 19:15 Amsterdam (from the clock)

## Steps done
1. The takeover and ORDER-3C: his answer **(c)** with four conditions (2a no proof thinner; 2b the recording built
   and green before any movement code; 2c every item named for 3c placed by name; 2d the fallback: the recording
   alone as a small change-set before Walk, told to him before anything moves).
2. Main 311c0484 taken in (merge commit 1802856); the start folder fast-forwarded; the fixed guard loads.
3. His Testlines Export saved whole by script: 15,567 bytes, sha256
   9ad1d9f914acc27c56cbfbe7c4e6c7e654ba24fa0e9ae75cb4e3b93103f06498 (`testlines-save.mjs`; the manifest says "not
   usable as it is": step rate 60, one warning). Compared by script with `docs/legacy-tuning.json`
   (`testlines-compare.mjs`, output `testlines-compare.txt`): 83 names, 60 equal, 23 different, 0 without a
   counterpart. No value is carried. STEP-60 asked of him in the chat (17:55 UTC): no answer yet.
4. Three designs, each ending DONE: `netcode-architect-cs4.md`, `engine-architect-cs4.md`, `rust-reviewer-cs4.md`.
   The main session's notes: `testlines-cs4.md`, `rules-cs4.md`.
5. The plan's draft 1 written and pushed (f789b24): Walk as four change-sets, 4a The recording, 4b Content and the
   world, 4c The first soldier, 4d Sights and the proofs that are left; then the link simulator's change-set.

## Current step
Two agents run: the advisor agent's review of draft 1 (`advisor-cs4-plan.md`) and game-designer's check of
`rules-cs4.md` (`game-designer-cs4.md`).

## Next steps
1. Fold game-designer's corrections into the plan's section 10 and its questions MOVE-RULES and ADS-DELAY, and the
   advisor agent's conditions into the plan (a section at its end says where each is met): draft 2.
2. If section 10 or a question changed after the advisor's read: its short second read of what changed.
3. Push; send him the plan as a file; board: one line naming the plan file and its commit, then the state that says
   the chat waits for him. Stop for his go and his letter for WALK-CUT.
4. On his go: 4a's own checkpoint (`cs4a-recording`), the hash report with rust-reviewer, then 4a's steps 1 to 8
   (plan section 4). No manifest change in 4a.
5. For 4a's docs pass: his blocks into `docs/DECISIONS.md` by script; change-set 3b's owed docs pass; the Hornet's
   entry in `docs/UNITS.md` section 0 by script, with the two clashes listed for him (lines 21-23 and 54-55).

## Open findings
- The checkpoint script cannot push from this worktree: notes are committed by path and pushed by hand; no saver
  runs · this chat · open (the plan's section 3, item 6 says so).
- The first run of the comparison script counted an Export value of 0 as equal to a small old value (it rounded to
  the decimals the number printed with): caught against the Export's own "changed from start" list, fixed before
  the result was used; the kept tool gets a planted case for it (plan section 11d) · this chat · fixed in 2c3078e.
- The session's worker restarted once (about 18:00 Amsterdam); nothing was lost.

## Key numbers
- Walk bottom-up from the designs: about 15,000 lines (4a 1,365; 4b 4,860; 4c 6,625; 4d 2,120), nearly half tests;
  at 1.8 times about 26,900; the card: 3,900. The recording alone: 1,365 (change-set 3's plan: 275).
- The collision library: parry3d =0.31.1; 33 new lockfile packages, 31 in the players' file's tree (rust-reviewer's
  paper reading, to be confirmed by `cargo tree` after the hold).
- The time unit: 3,000 units a second (rust-reviewer, block 4).

## Decisions taken
- ORDER-3C (c) · Mustafa · 2026-11-25, 16:59:52 UTC.
- His rules for this chat that are not on main yet: no CI, kit or gate file is written here; never a push to
  another chat's branch; until a second guard fix is merged, git and gh always start their own command, with no
  other program in front of them (this session and every agent).
- The board add-on is active for this chat (since 2026-11-25).

## Running work
- The advisor agent (plan review of draft 1), started 2026-11-25 about 19:12 Amsterdam; notes
  `.checkpoints/cs4-walk/advisor-cs4-plan.md`.
- game-designer (the check of `rules-cs4.md`), started about 18:57 Amsterdam; notes `game-designer-cs4.md`.
- No saver runs: the main session commits the notes by hand at each update.
