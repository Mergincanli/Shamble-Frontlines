# Checkpoint: cs4c-soldier
- **Goal:** change-set 4c, "The first soldier" (the walk plan's section 6, `.checkpoints/cs4-walk/plan.md`): he plays,
  walking, sprinting, jumping, stepping up and aiming with the mouse, with the follow camera and its tuning panel.
  **Its first building step is 4b's old step 12**, the page's scene with the map's reading field by field (his
  STEP12-GO (a), `docs/DECISIONS.md:183`). 4c starts as 4b did (his words of 2026-11-27, 23:00:39 UTC, item 4): its
  plan sheet (the re-read against that day's main), the advisor agent's review, then the build.
- **Branch:** local `cs/4c-soldier` in the worktree `/home/user/Shamble-Frontlines/.gate/cs4b-2` (the folder keeps
  4b-2's name); pushed with `git push -u origin cs/4c-soldier:claude/ecstatic-wozniak-5cbdzs`, alone in its command
  (this session may push only there; that branch stands in for `cs/4c-soldier`). The session's start folder
  `/home/user/Shamble-Frontlines` is never switched.
- **Base:** origin/main dae6d107fdbe942567104bcf1c9a2cbc6b3a1e0f (pull request #79's merge commit), taken in by
  fast-forward from b7145273.
- **Last update:** 2026-11-28 00:18 Amsterdam (from the clock)

## Steps done
1. **4b-2 merged and main taken in** (2026-11-27). Pull request #79 merged by merge commit dae6d107 at 23:12:04 UTC,
   at head b7145273fdaff2b2ee6eccafaf5e358840bc3572 (his go of 22:50:52 UTC; his Advisor opened and merged it).
   b7145273 checked to be an ancestor of origin/main before the fast-forward; the fast-forward changed no file.
   Main's checks on dae6d107, read once at about 23:13 UTC (his item 3: once): gate success; smoke and toolchain in
   progress. The advisor agent's file on main last changed on 2026-11-13 (8523d50), before this session began
   (2026-11-27 13:54 UTC), and this branch does not change it: the `advisor` agent runs as it is.
2. **This card and the notes.** Ten walk notes put back from 59f6cc7 by `git show`, byte for byte (blob ids
   compared; the guard refuses `git checkout` and `git restore` here). His three messages saved by script
   (`chat-words.mjs`, never retyped) into `.checkpoints/cs4-walk/`: `mustafa-2026-11-27-go-merge-79.md` ("you have my
   go", 22:50:52 UTC), `mustafa-2026-11-27-may-merge.md` (22:52:26 UTC), `mustafa-2026-11-27-design-notes.md` (his
   design notes of 22:15 Amsterdam, 22:58:25 UTC, 7,117 bytes). His message of 23:00:39 UTC is not saved in the
   repository until he answers BOARD-WORDS (the question file).

## Current step
The questions to him: `.checkpoints/cs4c-soldier/question-2026-11-28-before-4c.md` (the four the plan lists before
4c, SIZE-4C, BOARD-WORDS, PASTE-BREAKS, DESIGN-CLASH). Then the plan sheet.

## Next steps
1. **The plan sheet** `.checkpoints/cs4c-soldier/reread-sheet.md`, revision 1, against main dae6d107: step 0 (the first
   docs commit) and step S (the scene, 4b's old step 12) with their exact symbols, for the advisor agent's APPROVE
   now; steps 1 to 7 held for three reads against today's code (engine-architect: the page, the block's layout, the
   camera; netcode-architect: the command path, the snapshot and block, the recordings; rust-reviewer: the movement
   on today's collision world, the float helpers, 4B-RAY-WASM and 4B-RAY-COUNTS), each written to this folder. A
   column per step names which of his questions it waits for. Building a step out of the plan's order while his
   answers are out (the command path or the block before the movement) is put to the advisor agent, never done on
   my own reading.
2. **Step 0, the first docs commit** (after the advisor agent's APPROVE; it waits for PASTE-BREAKS and BOARD-WORDS,
   since nothing in the scene needs it): his go for 4b-2's merge, his word on merging and his design notes into
   `docs/DECISIONS.md` by script; LOOP_V2's new section 14 and its pointers placed by docs-keeper from the saved
   message (`chat-words.mjs part`, never typed); the register line in `docs/adr/README.md`. STEP12-GO is in the
   register already (since 4b-2's docs pass), so it is not written again.
3. **Step S, the scene**, then the steps his answers free.

## What leaves first if the count runs high (his SIZE-4B point 2, applied to 4c; confirmed in the sheet)
- The plan's own ladder: at 13,150 written lines the far-corner map (PM-0004-39) moves to 4d, and he is told before
  it moves. Nothing else moves; at the stop line the chat stops and he decides.
- The stop line with the scene inside is his (SIZE-4C). Until he answers, the scene is held to the lowest line any
  option gives it (the plan's rule, its planned size times 1.25: about 1,420), so no answer is taken for him.

## Open findings
- None yet for 4c.
- Carried from 4b-2 (STATUS "Follow-ups"), placed by the sheet: 4B-QUERY-RECORDING, 4B-RAY-WASM, 4B-RAY-COUNTS,
  4B-WORLD-ROOM, 4B-LIMIT-IMPORT, 4B-TYPES-READER, 4B-MEMBER-KINDS; the advisor agent's acceptance of
  security-auditor's two developer-machine risks, recorded as a dated note.

## Key numbers
- Measured written lines against the designs' count: 4a 2.4 times, 4b-1 2.17, 4b-2 2.7.
- 4c by the designs' count: 7,315 (steps 1 to 7, the plan's section 6) and about 630 for the scene with the map's
  reading (230 / 260 by 4b's sheet, and 60 to 80 more with as many in tests) = about 7,950. At the measured rates:
  about 17,250 to 21,450 written lines. The plan's stop line for 4c: 16,450, set before the scene moved in.
- Ladder rung: 13,150 (the far-corner map). Two thirds of the stop line: told to him in one sentence.

## Decisions taken
- His go for 4b-2's merge (22:50:52 UTC) and his word on merging (22:52:26 UTC): saved as above.
- His message of 23:00:39 UTC: MERGE-ROUTE (a), his Advisor merged #79; and how 4c starts (this card's Goal). Its
  saved copy waits for BOARD-WORDS.
- Standing rules: git and gh always start their own command, with no other program in front of them; every file a
  shell command writes is spelled out with its full path (or written with the Write and Edit tools); no CI, kit or
  gate file is written here; no value of his is typed by hand; times come from the clock; commit and push at least
  every 25 minutes, the agents' work too. The board add-on is active for this chat.

## HAND-OVER: where another session picks this up (kept current)
- **Where:** branch `claude/ecstatic-wozniak-5cbdzs` on origin; local `cs/4c-soldier` in
  `/home/user/Shamble-Frontlines/.gate/cs4b-2`. A session of another account works on its own branch from this
  branch's head and never pushes here (his Lanes rule). SEAT-OUT from him: stop at a whole point and report the last
  commit's id.
- **State:** nothing of 4c is built; no file outside `.checkpoints/` differs from main. No agent runs; no saver runs
  (the main session commits and pushes the notes by hand).
- **Next:** this card's "Next steps", in that order.

## Running work
- No agent runs.
