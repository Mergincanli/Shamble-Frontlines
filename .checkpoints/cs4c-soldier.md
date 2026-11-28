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
- **Last update:** 2026-11-28 01:59 Amsterdam (from the clock; an earlier save said 01:08, a time written without
  reading the clock: it was 00:57)

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
3. **The questions, asked** (deb337ad; in the chat at about 00:25 Amsterdam): `.checkpoints/cs4c-soldier/
   question-2026-11-28-before-4c.md`, the four the plan lists before 4c (MOVE-RULES, NO-SOURCE, RESUME-FLASH,
   TUNE-HOME), then SIZE-4C, BOARD-WORDS, PASTE-BREAKS and DESIGN-CLASH. No answer yet.

4. **The plan sheet, revision 1** (78fd901e, 00:59 Amsterdam): `.checkpoints/cs4c-soldier/reread-sheet.md`, from
   the three finished reads; the question file gained SIZE-4C's refinement (recommendation now (b)) and STREET-GREY
   (79cfc4c0), posted on the board.

5. **game-designer's read** (DONE): the sandbox's own step rule (a 0.4 m body, a flat 0.15 m foot, a pull, a 0.2 m
   snap), BODY-SHAPE's options, the push's numbers; the question file corrected from it (5597b00d). It fetched his
   Testlines repository read-only by its public address, outside the one repository this session has attached: told
   to him. rust-reviewer re-runs its prototype with that body (running).
6. **The advisor agent's verdict on the sheet's revision 1** (`advisor-reread-4c.md`, DONE): **step 0 and step S
   APPROVED with conditions 1 to 9; steps 1 to 7 not yet** (its item 10). The sheet is revision 2 with every condition
   applied; the question file gained DEPART-4C, STREET-GREY restated, SIZE-4C's branch consequence and the scene's new
   hold.

7. **rust-reviewer's body re-run** (DONE, its block 5): the sandbox's own body passes every run, with the pull (3a)
   and without it (3b); BODY-SHAPE's evidence is in the question file (04646ca3).

## Current step
**Step S, the scene: the build** (by the sheet's section D, revision 2). The branch rule (condition 2): until he
answers SIZE-4C only step 0 and step S go on the branch; afterwards notes and prototypes only.
- **Code not yet committed is saved as a patch:** `.checkpoints/cs4c-soldier/step-s-wip.patch` (from `git diff`, the
  new files marked with `git add -N`; the old scene file's removal unstaged so no commit carries half-built code).
  A new seat applies it with `git apply` on this branch's head. Written so far (01:24 Amsterdam): the interface file
  (`PictureContent`, `SceneObject`, `setContent`, `inventory`), `render/scene.ts` (the empty floor moved; the map's
  floor with flat faces turned outward; greys by height; the grid; disposal; the overview), `render/inventory.ts`,
  three-picture.ts's `setContent` and `inventory`, boot's `__shambleScene`, the two markers.
- **First proof, done (the advisor tool's item 1):** both pages built from that code (`.gate/dist-production-4c`,
  `.gate/dist-development-4c`, exit 0 with the build's own checks): the production page holds `inventory(){return
  null}` and neither marker; the development page holds both. So the inventory's cut by the build flag holds, and
  no dynamic import is needed.
- **Since (01:44 Amsterdam): the step's code and tests are written, and every check run so far passes** (the patch
  holds them all, 23 source files):
  - the map's reading in content.ts (`readMap`, `readContent`, the kinds and their constants, `NOT_THE_MAP`,
    `PageContent(deliver, read)`); `LOCAL_MAP` from a module of its own in the Worker's package (recording.ts is
    test-only, so the page takes in nothing of it); boot's hand-off; the readout's content and scene rows;
  - tests: content.test.ts (the map read field by field, closed key sets, members by kind, ramps alone with
    `rises_toward`, nothing read through a prototype, the two fixed sentences, the bundle's files, delivery once),
    content-map.test.mjs (the committed test scene reads as itself), content-ties.test.mjs (the kinds' formats and
    folders tied to the Rust source), scene.test.ts (corners, outward flat faces for a box and a ramp toward each
    side, greys, grid, disposal, overview at three aspects, the inventory reading the buffers), report.test.ts (the
    rows), content-schema.test.mjs (each member's kind, 4B-MEMBER-KINDS, with its planted mismatch), the browser
    specs (the drawn scene equals the built map; the refused bundles leave the empty floor and its counts; a broken
    Worker leaves the content "none");
  - the type checks of the Worker, the page and the specs; the page's 171 unit tests, the Worker's 53, the types' 14;
  - the Chromium specs: 27 of 29 on the first run, the two failures my test's strict "lighter" where the design says
    "never darker" (the yard's 0.55 m and 0.56 m steps share a grey: one grey level spans about 4 cm); fixed in both
    tests; the skeleton spec 5 of 5 after; the whole run took 217 s (CI's limit for it: 5 minutes);
  - the render counts measured: 32 draw calls, 267 triangles, 162 lines, 32 geometries, 3 textures, 6 programs, the
    arithmetic exactly, and the same with the map drawn in after the first frame (ready at 4,463 ms, first frame at
    4,114 ms) and before it (ready at 124 ms, first frame at 10,115 ms), by `counts-order-4c.mjs`;
  - allocation on main's page and the scene's: 100 and 85 bytes per frame, 572 and 553 per block, 18,166 and 17,536
    a second (the sampling profiler's noise; both pass);
  - the after picture: `.gate/scene-after-4c.png` (not committed: a binary).
- **The code commit: bc7720f3** (00:47 UTC, no `[skip ci]`): 1,277 written lines by the count (`count-4c.mjs` against
  dae6d107; under the 1,420 he hears about first, and the 1,665 hold); the gate PASS with smoke
  (`PLAYWRIGHT_BROWSERS_PATH=/home/user/Shamble-Frontlines/.gate/cs4b-2/.gate/pw-1194 node tools/gate.mjs`: the gate's
  Playwright asks for build 1194's headless shell, which `.gate/pw-1194` links to this machine's 1243). CI on bc7720f3:
  gate and smoke green; the toolchain run started 00:47:49 UTC. Notes are held local until it ends (a push cancels it).
- **The full local checks on bc7720f3's tree: ALL LOCAL CHECKS PASSED** (`.gate/local-checks-4c.log`; the browser run
  29 of 29 in 217 s). **CI on bc7720f3: gate, smoke and toolchain green** (runs 46089793885 and 46089793887; the
  toolchain 11 min 2 s).
- **The plants run on acd7d4f** (`plants-4c.txt`): 27 of 28 caught at first; the miss (the +z ramp's face list on a
  -z ramp) was no defect (the same faces under other names). Its replacement (the +x list) passed the scene test
  without a volume check (a one-off run: 8 of 8 passed), so the test now holds each solid's enclosed volume
  (**c1de41e5**, a test-only code commit, gate PASS); the face plants re-run: 3 of 3 caught. So 28 of 28 caught, 3 of 3
  controls held, every file put back.
- **Pushed: d6a9fd49** (about 01:59 Amsterdam). Its head was a `[skip ci]` notes commit, so the commit after it (this card,
  without `[skip ci]`) carries CI onto the head that holds c1de41e5's test.
- **Step S stands, built and proven, waiting for him:** his SIZE-4C answer (whether 4c-1 is its own pull request, so
  its six reviewers, the verifier and his Brave check, or the reviews on 4c's final diff), and STREET-GREY (before
  4c-1 merges). Step 0 waits for PASTE-BREAKS, BOARD-WORDS and DESIGN-CLASH (its checks are written:
  `section14-4c.mjs` tied to the cut text's digest, `loop-check-4c.mjs` byte for byte against main's two design docs
  saved beside it). Only notes and prototypes go on the branch until SIZE-4C is answered (condition 2).

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
- The stop line with the scene inside is his (SIZE-4C). Until he answers, the scene is held to about 1,665 written
  lines (the advisor agent's item 2: the plan's rule on the recount, 740 x 1.8 x 1.25); one sentence to him before
  the count passes 1,420; the two-thirds sentence at about 1,110; at 1,665 the build stops. What leaves the scene first,
  said to him before it moves: the overview camera (only under SIZE-4C (a)), then the readout's per-file headers.

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
- **State:** nothing of 4c is built; no file outside `.checkpoints/` differs from main. Three reads run (see
  "Running work"); no saver runs (the main session commits and pushes the notes by hand, at least every 25 minutes).
  If this seat ends while they run, their files hold whatever blocks they finished: a new read continues after the
  last complete block.
- **Next:** this card's "Next steps", in that order.

## Running work
- Design reads against main dae6d107 (at most three agents at a time), each writing only its own file in
  `.checkpoints/cs4c-soldier/`; the main session commits their files with the notes at each save:
  - engine-architect (`engine-architect-read.md`): blocks 1 (the scene, folded into the sheet's section D), 2
    (SIZE-4C's seam) and 3 (steps 3 to 5) done; block 4 (4B-WORLD-ROOM) running.
  - netcode-architect (`netcode-architect-read.md`): DONE (folded into the sheet's E2, E3's timing, E6, E6b, E6c).
  - rust-reviewer (`rust-reviewer-read.md`): DONE (folded into the sheet's E1: BODY-SHAPE, SWEEP-ROUTE, libm's
    dependency hold, the pins re-made in step 1).
  - engine-architect: DONE (block 4, 4B-WORLD-ROOM, folded into the sheet's E3 to E5).
  - game-designer (`game-designer-read.md`): DONE.
  - the advisor agent (`advisor-reread-4c.md`): DONE (its verdict above).
  - rust-reviewer's re-run (its block 5 in `rust-reviewer-read.md`): DONE.
  - No agent runs now.
- The scene's before picture: `.gate/scene-before-4c.png` (main's page, 3 draw calls), by `shot-4c.mjs`.
