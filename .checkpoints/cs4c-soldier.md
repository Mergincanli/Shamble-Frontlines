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
- **Last update:** 2026-11-28 13:50 Amsterdam (from the clock; an earlier save said 01:08, a time written without
  reading the clock: it was 00:57; an edit at 02:04 wrote "02:06", also unread, corrected here)

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
**4c-2, the soldier (steps 1 to 7): its plan with the advisor agent** (`advisor-steps1to7-brief.md`), built on top of
4c-1's final head by his item 6 b. **4c-1 is closed:** final head **946fef73a07550840d55e12f45932451b2dccd5c**
(pushed 14:35 Amsterdam; CI green: gate and smoke run 46150338098, toolchain run 46150338072), the advisor agent
APPROVE for his Brave check and go (`advisor-final-4c1.md`, copied from `.gate/`), the zip sent to him just before
14:55 (the clock read right after it; `shamble-v2-04c1-the-scene.zip`, 342,469 bytes, sha256 13c5f6ef...; built from the clean final head, its 12 files
checked after unpacking, the page's 6 browser tests passing on the unpacked copy). The notes came back in 4c-2's
first commit (a revert of 946fef73).
The advisor agent's conditions, kept here: **merge** by merge commit only, 946fef73 as the expected head (96f6e6c9
and d1f1288f must stay reachable); the pull request's own checks green on 946fef73; his go typed by him after the
APPROVE, in the chat of the session that merges (nothing on GitHub counts); his Advisor's branch takes no push (a new
commit is a new head: its review, then his go again). **The one-branch way:** until main holds 4c-1's merge, 4c-2 is
counted, gated and reviewed against 946fef73; once merged, main comes in once, in this worktree, and 4c-2's pull
request is not marked ready before that; a scene change he asks for after his Brave check goes into 4c-2's pull
request, reviewed as scene code by graphics-engineer and art-director at least; he is told that if he holds back his
go on 4c-1, the scene and step 0's doc edits land only inside 4c-2's pull request. Its Lows: the pull request's text
says "in the builder chat" for his go (it is the chat of the session that merges: told to him for his Advisor, no
push); this section was stale (refreshed here).
Earlier (kept for the record): **Step S, the scene: the build** (by the sheet's section D, revision 2). The branch rule (condition 2): until he
answers SIZE-4C only step 0 and step S go on the branch; afterwards notes and prototypes only.
- **The code is committed (bc7720f3, c1de41e5); `step-s-wip.patch` is stale and is not applied by anyone** (it was
  the build's saved state before bc7720f3; it goes with the notes at 4c-1's end). How the build went, kept for the
  record: written by 01:24 Amsterdam: the interface file
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
(The sheet, revision 2, and step S are done; the old items 1 to 3 are in "Steps done" and "Current step".)
1. **His answers** (the question file's eleven; the Testlines read and the CI Builder's request in the chat). On each:
   step 0 when PASTE-BREAKS, BOARD-WORDS and DESIGN-CLASH are in (condition 8; BOARD-WORDS (b)'s method written first,
   condition 7); step S's six reviews by SIZE-4C; STREET-GREY before 4c-1 merges.
2. **The dependency batch: answered twice** (APPROVE on the first version; reworded for FACING-SPAWN (a) and APPROVE
   again on `deps-4c-v2.diff`, `security-auditor-deps-4c-v2.md`; `dependency-check.md` rewritten from it, the five
   digests checked again with `sha256sum` against the scratch worktree). The batch is applied only in the step that
   first needs it, after SIZE-4C, in one commit with F1's tightened test, F2's layering test and the maths list's four
   entries with their vectors, and the scan's stricter libm rule (item 3, corrected by rust-reviewer's RR1 and RR4).
3. **The advisor agent's item 10, as notes and prototypes only (condition 2):** rust-reviewer's block 6 (the bodies
   3a, 3b and 2 on block 5's "not run" list: ramp sides with the set-back, a ceiling on a made map, landing, the push
   with two soldiers; what the 2.5D form does with a shape whose bottom is above the ground; its yes or no on the
   record rule); engine-architect's block 5 (GROUND against map.rs:39, map.rs:325 and the height scene.ts draws the
   ground at; FACING-SPAWN's options, any pick that narrows a map file going to him); netcode-architect's written yes
   on the record rule when a slot is free; the sheet's revision 3 (item 6 applied with its confirmations outstanding;
   the recount of steps 1 to 7 with both the 2.5D and the 3D sweep counts until block 6 answers; section G per step).
   The advisor agent is asked to approve steps 1 to 7 only after his answers on BODY-SHAPE, SWEEP-ROUTE and the slope.

## What leaves first if the count runs high (his SIZE-4B point 2, applied to 4c; confirmed in the sheet)
- **For 4c-2 (the advisor agent's Q3, 2026-11-28):** at about 14,960 written lines the far-corner map (PM-0004-39,
  step 6c) moves to 4d, and he is told before it moves (first in the next report). Nothing else moves. His stop line
  for 4c-2 is 18,700: if the count reaches it before the early look is ready, two lines to him and the build goes on
  to the early look; past it, nothing after the early look starts without his word (his SIZE-4C (b)). Until 4c-1's
  merge is on main, 4c-2 is counted against 946fef73; afterwards from the merge-base with main (Q4, condition 8).
- Earlier, for 4c as one (kept): the plan's own ladder: at 13,150 written lines the far-corner map (PM-0004-39) moves
  to 4d, and he is told before it moves. Nothing else moves; at the stop line the chat stops and he decides.
- The stop line with the scene inside is his (SIZE-4C). Until he answers, the scene is held to about 1,665 written
  lines (the advisor agent's item 2: the plan's rule on the recount, 740 x 1.8 x 1.25); one sentence to him before
  the count passes 1,420; the two-thirds sentence at about 1,110; at 1,665 the build stops. What leaves the scene first,
  said to him before it moves: the overview camera (only under SIZE-4C (a)), then the readout's per-file headers.

## Open findings
- **For step S (4c-1):** G-3 (Low, engine-architect's block 5): no test tells the bounds' bottom from y 0
  (scene.test.ts:149 checks the literal 0); it rides the next code commit with no push of its own, and step S's
  review round addresses or defers it (`review-briefs-step-s.md`).
- **For steps 1 to 7 (each placed in the sheet):** G-1 (Low): map.rs:39, schema.rs:146-147 and aim_reach.rs:45 call
  y 0 "the floor", in step 1's commit; RR1 (Medium): the sim's scan refuses std's spellings only, fixed by the
  stricter libm rule in the commit that applies the batch; the two model holes every body shares (the in-air step's
  "toward" from the tick's start; a top crossed inside one tick counts as landed), one condition each, rust-reviewer's
  claim that neither is a rule of his to be confirmed by game-designer at the step's review; the head bump's details
  (not in the notes); FALL-STEP (his).
- Carried from 4b-2 (STATUS "Follow-ups"), placed by the sheet: 4B-QUERY-RECORDING, 4B-RAY-WASM, 4B-RAY-COUNTS,
  4B-WORLD-ROOM, 4B-LIMIT-IMPORT, 4B-TYPES-READER, 4B-MEMBER-KINDS; the advisor agent's acceptance of
  security-auditor's two developer-machine risks, recorded as a dated note.

## Key numbers
- Step S as built: 1,288 written lines (bc7720f3 1,277, c1de41e5 11; `count-4c.mjs` at 3ab59bd7 against dae6d107),
  about 1.74 times its recount of 740.
- Steps 1 to 7 recounted: **about 8,550 by the designs' count** (the sheet's I4b, after rust-reviewer's block 10;
  F2's 8,295 to 8,395 is superseded), about 18,550 to 23,100 written lines at the measured rates; **his stop line for
  4c-2: 18,700** (SIZE-4C (b)), two thirds about 12,470; the early look (steps 1 to 5, about 7,080 by the designs'
  count) at about 15,360 to 19,110 written lines.
- 4c-1 as built: 1,565 written lines at 0645432f (`count-4c.mjs` against dae6d107: 1,564 at the fix pass 2b2f44c2,
  plus one comment line in 0645432f; the hold 1,665; he was told when it passed 1,420).
- Step 1 by rust-reviewer's block 11 with blocks 12 and 13: **about 1,520 / 1,945 (code / tests), 3,465 in all**
  (block 13 adds J1 to J5 as test row 26, +10 / +80, which step 2's count drops; test 15 per mode; test 21's faces and
  misses); the ridge flag (block 12, a STATUS follow-up, not now) would add 15 / 20.
- Measured written lines against the designs' count: 4a 2.4 times, 4b-1 2.17, 4b-2 2.7.
- 4c by the designs' count: 7,315 (steps 1 to 7, the plan's section 6) and about 630 for the scene with the map's
  reading (230 / 260 by 4b's sheet, and 60 to 80 more with as many in tests) = about 7,950. At the measured rates:
  about 17,250 to 21,450 written lines. The plan's stop line for 4c: 16,450, set before the scene moved in.
- Ladder rung: **about 14,960** written lines (the advisor agent's Q3 ruling of 2026-11-28 on steps 1 to 7: 18,700 /
  1.25; the far-corner map, step 6c, moves to 4d there, and he is told in the next report, before it moves); it was
  13,150, the planned size under the 16,450 line he did not take. Two thirds of his 18,700: about 12,470, told to him in
  one sentence. Past 18,700 at step 5's end, nothing after the early look starts without his word.

## Decisions taken
- **His answers of 2026-11-28, 12:10 Amsterdam** (sent 11:43:45 UTC; saved by script, whole, as
  `.checkpoints/cs4-walk/mustafa-2026-11-28-answers.md`, 4,516 bytes, sha256 3b14fb63...; the transcript holds it twice
  from a restart, the second copy the one answered): MOVE-RULES (a) as starting rules, each his to change after he has
  played, rows 6 and 7 confirmed again at 4d, row 9 open (as the sandbox for now; build nothing that depends on roofs
  being walkable or not); NO-SOURCE (b); RESUME-FLASH (b); TUNE-HOME (b); SIZE-4C (b), the soldier's stop line 18,700
  (reached before the early look: tell him in two lines and build on to it; past the line nothing after the early look
  starts without his word); the one-branch way of his item 6 (finish the scene, tell him its final head with its full
  id and the Brave steps, then build the soldier on top; his Advisor opens the scene's pull request from a branch of
  its own at exactly that head; scene changes after his Brave check go into the soldier's pull request; take main in
  once when the scene's merge is on main); BOARD-WORDS (b); PASTE-BREAKS (a); DESIGN-CLASH (a), with his own pointer at
  LOOP_V2's silo line; STREET-GREY (b); BODY-SHAPE (3a); DEPART-4C (a) on his terms (the library checks our sweeps for
  every shape, the ramp included, before the movement step counts as done; the old and new wording of PM-0004-20 and
  -35 side by side, the new asking no less; PM-0004-60 and PM-0004-20's 48-soldier run stay open in STATUS, each named
  with the change-set that meets it); FALL-STEP (c) by reading, else (a); DIGEST-NOTE (a); TESTLINES yes (read only by
  its public address; numbers and behaviour only, no code copied; not attached); "no aim yet" fine; CHROME-TIME (the CI
  Builder parked until Tuesday 1 December 20:00; never cut or thin a test; a run stopped at the limit is re-run once,
  twice stopped is told in two lines; a head for the advisor agent needs a green run); while he is away: save every 25
  minutes, new questions into the question file with a recommendation, the advisor agent decides what it may; one git
  per shell call. He is away until Sunday 29 November, 12:00.
- His go for 4b-2's merge (22:50:52 UTC) and his word on merging (22:52:26 UTC): saved as above.
- His message of 23:00:39 UTC: MERGE-ROUTE (a), his Advisor merged #79; and how 4c starts (this card's Goal). Saved by
  BOARD-WORDS (b), without its item 3, as `.checkpoints/cs4-walk/mustafa-2026-11-27-merge-route.md` (1,284 bytes shown,
  sha256 91b6783a...; the whole message's digest only in the chat).
- Standing rules: git and gh always start their own command, with no other program in front of them; every file a
  shell command writes is spelled out with its full path (or written with the Write and Edit tools); no CI, kit or
  gate file is written here; no value of his is typed by hand; times come from the clock; commit and push at least
  every 25 minutes, the agents' work too. The board add-on is active for this chat.

## HAND-OVER: where another session picks this up (kept current)
- **Where:** branch `claude/ecstatic-wozniak-5cbdzs` on origin; local `cs/4c-soldier` in
  `/home/user/Shamble-Frontlines/.gate/cs4b-2`. A session of another account works on its own branch from this
  branch's head and never pushes here (his Lanes rule). SEAT-OUT from him: stop at a whole point and report the last
  commit's id.
- **The advisor agent on C2's and C3's departures (`advisor-delta-c2c3.md`): all 8 ACCEPTED; GO for C4 on
  conditions:** C4 not pushed until CI on 9e9a1cc8 is green (a red run makes it HOLD); C4's maths list exactly the
  paths the real build refuses, for both builds, each a whole path (no covering prefix), any path outside the exp and
  sincos families or any compiler_builtins or C-named one reported and not added, `rem_pio2_large`'s reason "linked,
  unreachable on [0, 360), the one target-dependent routine", and security-auditor reads the real list in C4's commit
  (a fix in its own commit before C5); the report names the departures (block 17's Q1, Q2; the recorded digests prove
  sameness, not accuracy; the accuracy figures with their method, from rust-reviewer's 33,648-input scratch set where
  the committed file holds 33,646: said so, or re-measured; libm's cost against std's not measured); step1-build-order.md's
  superseded lines marked (done in this notes commit).
- **C4's predictions, written before the Rust side runs (the advisor tool's "predict, then run"):** world.rs's known
  answers by `world-known-answers-c4.py` (it first reproduces all six old answers and the old shared one, so its
  FNV-1a and byte order hold): new(0, 0) 0xfe5c_1b37_2be6_b04f; after 3 empty ticks 0xed75_6338_8a8f_e700; new(1, 0)
  0x5e87_f5ee_6b99_ae0f; new(0, 131069) 0x356d_7029_9b57_4922; new(0, u32::MAX) 0x7514_161d_05d9_e31b; played ([],
  [pressed]) 0x2d3a_5263_e54c_3740, its shared 0x6ad4_99aa_4def_3e40; moved (move (127, -3), aim (100, 50, -200))
  0x1464_7681_b7aa_69fc; no spawn new(0, 0) 0x4492_32b7_23f1_4ddd; new(0, 0) shared 0xfe9a_6ff7_b575_8ccf. The
  recordings: `first-loop.content.json` must NOT move (C2's freeze; content unchanged in C4); `first-loop.rec` must not
  move if it holds no command field C4 adds (to check in recording.rs before the run); `first-loop.hashes` (every tick
  line), `first-loop.vector.txt` (the rows join the state) and `manifest.txt` move.
- **security-auditor's round 2 on C3's fix 9e9a1cc8: APPROVE** (`security-auditor-c3.md`, "Round 2"): M1, L1, L3, L4
  and its item-2 remark FIXED (its probe re-run on the fixed marking; the three changed test targets passed at
  9e9a1cc8); no new finding. **L5's deferral is accepted on three conditions, which travel to 4c-2's report and
  STATUS:** (1) the report or STATUS names the exact CI line as open (toolchain.yml:165-166 runs the chacha8 and f32
  generators' `--check`, not `make_libm_vectors.py --check`: a request for the CI Builder); (2) local-checks.sh keeps
  running the generator's `--check` before every push until that CI line lands; (3) any change to libm-inputs.bin
  before then goes back to security-auditor. C3 closes when CI on 9e9a1cc8 is green.
- **C4's order of work (written 18:16 Amsterdam, from the clock; the design's pointers are in the 17:52 state
  below):** (1) sim world.rs: `PLAYER_SLOT`, `PLAYER_TEAM`, `MOVE_REACH`, `move_in_reach`; `PlayerCommand` gains
  `move_x`, `move_y: i8` and `aim: AimCm` (NONE with `AimCm::NONE`); `PlayerRecord { buttons, presses }` and `NONE`;
  `World { tick, player: PlayerRecord, soldiers, seed }`; `World::new(setup, &Rules, &[SpawnPoint])` (slot 0 at the
  first point, grounded, stamina full, NONE aim; no soldier when the table is empty); the fold (oldest first: the
  newest's buttons and presses are the record; each command's move into `held_move`, a real aim into `aim_cm`; no row
  touched on an empty slot); `visit` and `visit_shared` with the 48 rows of 60 bytes (2,902 shared bytes);
  `StateSink::f32`; `authority_step(commands)` still takes no rules (C5 adds them). (2) soldiers.rs's row visited in
  11.2's order. (3) collision.rs `bounds_cm`, `holds_cm`, the cm derivation in `build`, `WorldError::Bounds`. (4) content
  map.rs `on_grid` `pub`, GROUND's words. (5) room: `statics`, `Room::new(.., &Rules, statics, &[SpawnPoint])`,
  `Room::statics()`, the fill, `refused_move`, `refused_aim`, `refused()`, the doc, the NONE tie, the join on
  `PLAYER_SLOT`/`PLAYER_TEAM`, queue.rs's literal, any_bytes. (6) protocol `AimPoint::NONE`, messages.rs:117. (7)
  endpoint.rs:258's NONE. (8) sim-wasm `local_constants`, `CreateError::Rules` and `::Soldier`, `LocalMatch::new(..,
  &Rules, statics, &[SpawnPoint])` without `world` and `static_world()`, step_alloc.rs. (9) devtools: replay.rs's
  `Lines` gains `f32`, hash_cost.rs, the recordings made again (the frozen bundle is C2's: no re-freeze; the `.hashes`
  now move, as the state does), the vectors' `vector.txt`. (10) `maths-allowed.mjs`'s entries for what the players'
  build then prints (about 14, B17-6), each with its reason. (11) sim/tests/state_hash_alloc.rs. (12) lib.rs's
  re-exports. (13) the tests of rows 22, 25, 27, 28, 31, 33, 34, 35, 36, 37, 38 (bar the direct feed), 39; world.rs's
  known answers re-made by a second implementation written for it in the notes (no committed script exists: earlier
  change-sets' agents wrote theirs in notes), `world-known-answers-c4.py`, FNV-1a 64 over the canonical bytes.
- **State (19:31 Amsterdam, from the clock): C4 proven and committed after this notes commit (the code head, so CI
  runs on it).** The full local checks on the tree with the plants put back: **ALL LOCAL CHECKS PASSED**
  (`.gate/local-checks-c4.log`: fmt, clippy `-D warnings`, cargo test twice with the generated types unchanged when
  written again, the release tests, the wasm build, the vector generators' checks, libm's included; the packages' tests,
  client 182, Worker 53, types 14, net 1; the boundary tests 41 of 41 in 6 files; the players' file bdbcba92..., 287,113
  bytes in both builds, from C3's 258,488; the browser tests 29 of 29 in 213 s; the recordings are what their scripts
  make). The two files the advisor agent's conditions edited, after that run's package and build steps: the client's
  tests again 182 of 182, build-checks.test.mjs 59 of 59 (its condition 4), the production build's check clean
  (`.gate/build-production-c4b.log`). **The gate PASS with smoke** (`.gate/gate-c4.log`). The tree compared with
  `c4-wip.patch` (`cmp`: the same) and the new file with its copy (the same); both leave the notes in this commit.
  **The count: 4c-2 at 6,170 written lines** against 946fef73 (C1 442, C2 459, C3 with its fix 2,137, C4 3,132, of
  which 865 are first-loop.vector.txt). Next: the code commit, its push, CI on it; then security-auditor on the commit
  (the advisor agent's ruling 8 (d); brief in the scratchpad, `c4-security-brief.txt`, with the advisor agent's five
  points); a fix, if any, in its own commit before C5; the board.
- **State (19:27 Amsterdam, from the clock; first written as 19:28 without reading it, corrected): the advisor agent on the build-check test's departure
  (`advisor-c4-maths-test.md`): APPROVE, not a thinning (the banned rule reads no list; the only entries the new form
  admits are the two exact strings its ruling 8 accepted, each held to its own two lines; with its condition 1 the
  guard is stricter than HEAD's; Mustafa's sentences on thinning tests are about time limits, DECISIONS.md:226 and
  :480, so the ruling is its own: nothing goes to him).** Its five conditions, applied in the worktree after the local
  checks had passed their package-test and page-build steps (they were in the browser step, which reads the built
  pages only): (1) the list-dependent block (libm's expf, exp2, expm1, sin, cos, sincosf and the neighbours exp::expf,
  sincos::sin each refused by libm's rule with exact lines; the two real names meet neither rule and are refused once
  their two entries are taken out); (2) the planted twin's comment says the banned rule reads no list; (3) two false
  sentences corrected (maths-allowed.mjs's "none a prefix that covers another routine": three module entries cover
  their modules, six entries sit below another; the test's "each names a routine's own path"); (4) after the run:
  build-checks.test.mjs with its count, and the gate on C4's head; (5) C::round's why names face_cm's own rounding.
  For security-auditor's C4 brief, its five points (the count of 21 against F2's four; the exact two-string carve-out;
  the banned rule takes no list; the three module entries lift every function of their modules, bounded only by the
  version pin; rem_pio2_large rests on unreachability). Next: the local checks' end; the client package's tests and
  the production page build again on the two edited files; the gate; the patch saved again; C4's commit.
- **State (19:21 Amsterdam, from the clock): the whole plant set on C4's tree: 60 of 60 defects caught, 19 of 19
  controls held, every file put back** (`plants-4c2.txt`, its first line naming 1b08e8ac: the tree was a95ea332's,
  notes only since). The tree then compared with the saved patch (`cmp`: the same) and the new file with its copy
  (the same). The full local checks running on it (`.gate/local-checks-c4.log`); the advisor agent's ruling still
  running.
- **State (19:18 Amsterdam, from the clock): the advisor tool's check of C4 taken; the whole plant set running
  (`.gate/plants-whole-c4.log`; its first line names 1b08e8ac, the head before this card's last notes commit
  a95ea332, which changed notes only); the advisor agent asked to rule on one departure
  (`advisor-c4-maths-test.md`, running).** The advisor tool's points and what was done: (1) build-checks.test.mjs,
  edited but not run: run, RED at its line 770 (`bannedMathsProblems` over every entry's segments: `exp` and `sincos`
  are banned C names, and libm's own entries hold them as segments). Not dropped: the check kept for every entry with
  an exact expected result (`[]` for 21, and exactly two lines of their own name for `libm::math::exp::exp` and
  `libm::math::sincos::sincos`), a check that those two are listed, and a new planted twin holding the plain C exp,
  sincos, sin, cos and expf refused for each under today's whole list; green, 59 of 59. **A departure for the advisor
  agent's ruling** (Mustafa's rule: no test cut or thinned), and for security-auditor's C4 brief. (2) `C::round`'s why
  now names face_cm as its second caller (match setup, finite by the bounds check). (3) `visit`'s doc names the 48
  rows. (4) The TypeScript fixtures that reach the players' module carry one movement document each (the repository's
  content, first-loop's frozen bundle); no Node or browser test pins a hash. (5) **The count** (`count-4c.mjs` on
  `git diff --numstat 946fef73 -- crates packages`, the new file marked with `git add -N` and unmarked after): **4c-2
  at 6,142 written lines** (at C3's fix head 3,038: C4 is 3,104, of which 865 are first-loop.vector.txt, which
  make_recordings writes but the rule's left-out list does not name; without it C4 is 2,239). Under the two-thirds
  line (about 12,470) and the stop line (18,700); C5 (the movement) and steps 2 to 7 still to come, so the count is
  watched at every commit. Next, in order: the advisor agent's ruling; the whole plant set's result; the full local
  checks and the gate on this tree; the tree compared with the patch; this card; C4's commit; CI; security-auditor on
  the commit (condition (d), with the six covered entries and the test's departure named).
- **State (19:12 Amsterdam, from the clock): C4 written whole; its plants run; saved again** (`c4-wip.patch`, 341,240
  bytes, the tracked files; `c4-wip/state_rule.rs`, the one new file, its sha256 equal to the tree's). Since 19:02:
  T-A (`crates/sim/tests/state_rule.rs`: world.rs and soldiers.rs read with the shared tokenizer; every World field
  bound in exactly one of visit and visit_shared, visit_shared drops nothing but `seed: _`, visit_row binds all ten
  Soldier fields; its own planted copies each red), T-B and W-2's standing half (world.rs), row 33's three bundles
  (devtools content_files.rs: the test bundle, the repository's content and first-loop's frozen bundle each make a
  match with his full stamina), aim_reach.rs's NONE line (-(AIM_REACH_CM + 1)). rustfmt on the 12 named files, fmt
  check clean, clippy `-D warnings` clean (one fix: `JOIN_SLOT` by the slot's bytes, clippy's always-true
  comparison). **The maths list's evidence by the build check's own code** (`maths-refusals-c4.txt`, made by the
  scratchpad's `c4-maths-refusals.mjs`: the twins as the production build left them, read by `twinProblems` with the
  loader's two entries only): release twin 4 refusals, dev twin 29, 21 unique paths, the list's 21 equal to them (0
  extra, 0 missing), none outside `libm::math::`; with the whole list 0 refusals in each. **C4's plants**
  (`plants-4c2.py`, 27 defects B1 to B5, W1 to W8, RM1 to RM8, AB1, TA1, TB1, SW1, NT1, NT2, and 8 controls, RC2's count
  now 59): 26 of 27 caught at first; NT2's words were wrong (the wire's NONE moved alone is refused at build time by
  the tie, NT1, before the reach test runs): rewritten to move both NONEs, still tied: CAUGHT. So 27 of 27, 8 of 8
  held, every file put back. Next: the whole plant set, the full local checks, the gate, the tree against the patch,
  C4's commit.
- **State (19:02 Amsterdam, from the clock): C4 builds whole and every Rust test passes; saved again as
  `c4-wip.patch`.** Since 18:45: the room's tests (T2-6, N-1, W-3's stand-and-join half, the starved-tick pin with a
  test-only count `handed` of the slice handed to the world, the world past the reach: the room's 22 unit tests
  pass); any_bytes (honest moves inside the reach, honest aims inside the bounds or NONE, the three lies, a
  per-message identity on `refused_move` and `refused_aim` for intact messages after the join, and `refused()` the
  sum of the seven refusals: refused moves 2,490, refused aims 5,481 in the run); state_hash_alloc.rs and hash_cost.rs
  on the test bundle; replay.rs's state-lines test pinned with his row (spawn (0, 0, -5) bits c0a00000, flags 7,
  stamina 288,000,000 units, facing (1, 0), aim x 32768) and the shared bytes' 60-byte row. **The recordings made
  again, as predicted:** `first-loop.content.json` (f0093b57...) and `first-loop.rec` (30f80fde...) byte for byte the
  same; `first-loop.hashes` (its header unchanged, all 1,800 tick lines moved), `first-loop.vector.txt` (249 to 39,397
  bytes: his row at street_1, (-20, 0, -0.9), facing +x, the frozen movement's stamina 315,000,000) and manifest.txt
  moved. **The maths list (item 10):** the real production build printed 33 refusals, 21 unique paths (release twin 4:
  exp, sincos, rem_pio2, rem_pio2_large; dev twin all 21), all inside B17-5's exp and sincos families, no
  compiler_builtins and no C name: each its own entry with its why (rem_pio2_large's: linked, unreachable on [0, 360),
  its table 66 words on 32-bit and 690 on 64-bit, a debug assertion on 64-bit only), checked against libm 0.2.16's
  source for every "used by"; the production build then passes (players' file 287,113 bytes, from 258,488).
  build-checks.test.mjs pins the 23 routines. A slip: one shell call ran `cd <libm's registry folder> && grep ...`
  (read only, no git); the shell went back to the start folder and the worktree's own `cd` was run alone at once.
  Next: T-A, T-B, W-2's stand half, row 33's three bundles, then the plants, the full local checks, the gate, C4's
  commit.
- **State (18:45 Amsterdam, from the clock): C4 in the worktree, saved again as `c4-wip.patch` (98,224 bytes, 18
  files):** items (3) collision.rs's bounds in whole centimetres (`face_cm`, `aim_box`, `bounds_cm()`, `holds_cm`,
  `WorldError::Bounds`; tie-test parts a, b, d, e and the refusal: 15 collision tests pass), (4) GROUND's words
  (map.rs, schema.rs, aim_reach.rs; the generated map schema written again by `SHAMBLE_WRITE_TYPES=1`: one line), (5)
  the room's code (`statics`, `Room::new(.., &Rules, statics, &[SpawnPoint])`, `statics()`, the fill, `refused_move`,
  `refused_aim`, `refused()`, the doc, the NONE tie, `JOIN_SLOT`, queue.rs's literal), (6) protocol's `AimPoint::NONE`
  and the two sentences (its generated types written again: AimPoint.ts, Command.ts), (7) endpoint.rs's NONE, (8)
  sim-wasm (`local_constants`, `CreateError::Soldier` and `::Rules`, `LocalMatch::new(.., &Rules, statics,
  &[SpawnPoint])`, `world` and `static_world()` gone, its tests, step_alloc.rs), (9) part: replay.rs's `Lines` with
  `f32` and the row prefix. Next: the room's tests (T2-6, N-1, W-3's stand-and-join half, the pin with a test-only
  count of the slice handed, the world past the reach), any_bytes (T2-7), state_hash_alloc.rs, hash_cost.rs, the
  recordings, T-A, T-B, row 33's three bundles, `maths-allowed.mjs`.
- **State (18:29 Amsterdam, from the clock): C3 is closed. CI on 9e9a1cc8 green** (gate and smoke run 46169331291,
  toolchain run 46169331285, finished 17:25:23 UTC), so the advisor agent's first GO condition for C4 is met. **C4 in
  the worktree, not committed, saved as `c4-wip.patch`** (`git diff --binary HEAD -- crates packages content` on
  f22b07b7; no new file yet; 40,535 bytes): items (1), (2) and (12) done (world.rs rewritten: the soldier rows in the
  state, `PlayerCommand`'s move and aim, `PlayerRecord`, the fold, `StateSink::f32` and `row`, 48 rows of 60 bytes;
  soldiers.rs's `occupied_mut`, `rows`, the test-only `from_rows`; lib.rs's re-exports). **sim's 51 unit tests pass
  and every known answer came out as predicted** (`world-known-answers-c4.py`, ten of ten). Item (4) begun: map.rs's
  `on_grid` made `pub` (one rule for content and sim). `cargo build --workspace --all-targets` fails in room (its
  `World::new` call and its `PlayerCommand` literal at room/src/lib.rs:219): item (5). Next: item (3), collision.rs's
  bounds in whole centimetres (netcode-architect-read.md 7.3), then (5) to (11) and (13).
  Earlier state (18:13 Amsterdam, from the clock): C3's fix pass committed after this notes commit (the code head, so CI
  runs on it); its full local checks passed (`.gate/local-checks-c3b.log`: the players' file 58548ac7..., unchanged;
  Node 41 of 41; the browser tests 29 of 29 in 212 s), the gate PASS (`.gate/gate-c3b.log`), the tree byte for byte
  the saved `c3b-wip.patch` (removed in this commit). Next: CI on it, then security-auditor's round 2 on what changed
  (brief in the scratchpad), then C4. Earlier state (18:04 Amsterdam, from the clock; first written as 18:06 without reading it, corrected): C3 is green in CI
  (gate and smoke run 46167247237, toolchain run 46167247231, finished 17:04:20 UTC, 14 min 18 s). security-auditor on C3 (`security-auditor-c3.md`): REVISE on one
  Medium; the batch's approval stands (the five digests exact, no sixth file); items 1 to 5 MET but for M1; its three
  points: (a) a visibility before `mod` is within its ruling, (b) the libm conditions hold with the raw-name rule,
  (c) the recorded vectors raise no concern (its "correctly rounded" condition superseded by block 18).** Findings and
  the fix pass (in the worktree, its local checks running, `.gate/local-checks-c3b.log`): **M1 (Medium)** marking.rs
  read `'\''` to the escaped quote, so `['\'','{']` (no space, as under `#[rustfmt::skip]` or in a macro) opened a
  brace: fixed (an escape's closing quote is looked for after the escaped character), with three bites (two in
  layering, one in the content scan); **L1** three libm bites added (`libm::expf`, `::libm::exp` outside its site,
  `extern crate libm as m`); **L3** a backslash outside comments refused in every crate manifest and the root
  (dependency_rules.rs) and in layering's manifest half, each with a bite; **L4** the raw-name walk now takes every
  crate but sim and devtools (the three of today asserted among them), and the sim half counts a `pub type` alias;
  and its item-2 remark (dependency_rules.rs:335 compared a string with an edited copy of itself): now `maths_lines`
  is a function the main test uses and three planted root texts fail. **L5** (CI runs no `make_libm_vectors.py
  --check`, toolchain.yml:165-166): a `.github` edit, not this chat's: a request for the CI Builder (parked until
  Tuesday 1 December), deferred by name. Plants of the fix pass: P11 and P11b (M1 put back) CAUGHT, P13 CAUGHT, P12
  first NOT CAUGHT (my escaped-name bite also wrote `path = "../content"`, which the content check caught anyway),
  the bite narrowed to the escape alone, then CAUGHT; RC3 to RC5 held. Earlier state (17:52), kept: C3 = d7e260f7c1c903719023ff4424a92a5130a54975, pushed; CI: gate and
  smoke green (run 46167247237), toolchain running (46167247231, started 16:50:02 UTC); security-auditor reading the
  commit (its notes `security-auditor-c3.md`; brief: items 1 to 5 of its conditions and the three flagged points: the
  visibility before `mod`, the raw-name rule, the recorded vectors). 4c-2: 2,953 written lines against 946fef73 (C1
  442, C2 459, C3 2,052). Board posted (200). C4's edits wait for that verdict (a fix to C3 goes in its own commit
  first).** C4's design, all in the notes: rust-reviewer-read.md block 16's C4 row (the symbols and the tests by row),
  11.1 items 0 and 1 (the start and the fold), 11.2 (the 60-byte row, the record, `visit_shared` = 2,902 bytes), 11.6
  (sim, room, protocol, endpoint, sim-wasm, devtools lines), 11.7 rows 22, 25, 27, 28, 31, 33 to 39, 11.8 (the pins
  re-made: world.rs's known answers by the second implementation, first-loop re-frozen and re-made);
  netcode-architect-read.md 9.2 (B4 and W-1 to W-4) and 9.3 (the room's half: the fill, `refused_move`, `refused_aim`,
  `refused()`, the doc, the NONE tie, any_bytes, T2-6, T2-7); engine-architect-read.md block 7 (F1: the room owns
  rules, statics and spawns; `World { tick, player, soldiers, seed }`); B16-2 (`Room.rules` borrowed in C4, owned in
  C5) and B17-6 (C4's maths list: about 14 entries for the dev twin, not 4). Earlier state (17:48), kept: C3 committed and pushed after this notes commit (CI then runs on its
  code head); security-auditor reads it next (F9). The plants' whole run on 4b5f922c plus C3 (`plants-4c2.txt`):
  **29 of 29 defects caught** (C1's 2, C2's 13, C3's P1 to P10 and PV1 to PV4), every file put back; 10 of 12 controls
  held at once, the other two (RC4, PVC1) exited 0 but with wrong expected words of mine (layering has 4 tests now;
  the Node run is 6 files and 41 tests): corrected and re-run by name, both HELD. Notable: PV1 (one bit of every exp
  output flipped on wasm32 only) turned Node red while PV1n, the same plant run natively, stayed green: the per-target
  compare bites. The tree compared with every saved copy before the commit: the same. `c3-wip.patch` and `c3-wip/`
  leave the notes in this commit (C3 is in the code commit after it). Earlier state (17:45), kept: C3's full local checks PASSED (`.gate/local-checks-c3.log`: the Node
  boundary tests 41 of 41, two more for the libm vectors in both profiles; the browser tests 29 of 29 in 214 s; the
  players' file 58548ac7..., 258,488 bytes in both builds, 33 more than C2's d16cb57d..., not traced: rust-reviewer
  measured no libm routine linked, and the built-file check passed) and the gate PASS with smoke
  (`.gate/gate-c3.log`). The plants' whole run on 4b5f922c plus C3 is running; then the tree compared with the
  saved patch, C3's commit, its push (CI), and security-auditor on the commit (brief in the scratchpad).** Earlier
  state (17:36), kept: **C3 is built whole; next its full local checks, its plants run, the
  gate and its commit, then security-auditor on the commit (F9).** Saved as `c3-wip.patch` (tracked files) and
  `c3-wip/` (the new files: fmath.rs, rules.rs, marking.rs, layering.rs, libm_ops.rs, libm_vectors.rs,
  make_libm_vectors.py, libm-expected.txt; `libm-inputs.bin`, 1,077,304 bytes, is what `make_libm_vectors.py` writes,
  sha256 0ee2be59...). **rust-reviewer's blocks 17 and 18, taken:** Q1 YES, `exp_of` and `sincos_of` in fmath.rs hand
  on libm's own f64 results, `share_per_tick` and `facing_of` built on them; the scan's `MATHS_CALLS` moved onto them
  and its raw-name rule (`RAW`: each name only in fmath.rs, its own fn and its one caller, test code included; 10 bites,
  3 controls); layering refuses both names in room's, client-endpoint's and sim-wasm's `src/` (its plant: one call in a
  copy of room's and of sim-wasm's lib.rs); `maths-allowed.mjs:9` reworded (Low); C3's players' file links no libm
  routine (measured by rust-reviewer: release 0 names, dev 0 paths), so the maths list keeps its 2 entries in C3
  (C4: about 14, B17-6, Medium). Q2 (rate) and Q3 (`ads` as a speed) YES. Q4: the share test's words give the domain
  (every easing time in (0, 5], push strengths 0 or at least 1e-7 a second). **Q5, a departure rust-reviewer said YES
  to (block 18):** no Python twin of libm in the repository (libm's sources carry Sun's notice, which a port would have
  to carry; correct rounding is asked of sqrt only): `make_libm_vectors.py` writes the inputs (no platform maths: hex
  and decimal literals, ldexp, single IEEE steps; his two values left out, the grids hold 0.164 s and 8 a second);
  `libm_ops.rs` works one FNV-1a a routine through `exp_of` and `sincos_of` under a header naming `libm =0.2.16` and the
  inputs' byte count and sha256, which `check` verifies first (its condition 2); the native test records the file only
  with `SHAMBLE_WRITE_LIBM=1` (condition 1) and checks it otherwise; Node (`rules.test.mjs`) and each browser
  (`wasm-rules.spec.ts`) check the same file through `wasm.rs`'s `libm_vectors`. Recorded: exp 33646 551ff1fa383a1f22,
  sincos 101017 9e81faac70af97ff. Condition 3: B17-2's accuracy figures go into the report as measured, their method
  in the notes (`rr17-ref.py`, `rr17-dump.rs`, `rr17-make-inputs.py`, copied from `.gate/rr17/`). The generator's
  `--check` runs in local-checks.sh; CI's line is a request for the CI Builder (no CI file is written here). Clippy
  `-D warnings` and fmt clean. Earlier state (17:25), kept: **C3 built but for the vectors (waiting on rust-reviewer's block 17).**
  Saved again as `c3-wip.patch` (tracked files) and `c3-wip/` (the four new files: fmath.rs, rules.rs, marking.rs, layering.rs; the
  new files' homes: `crates/sim/src/`, `crates/sim/tests/support/marking.rs`, `crates/devtools/tests/layering.rs`).
  Added since 17:14: **the libm rule** in `rules_scan.rs` (`MATHS`, `MATHS_FILE`, `MATHS_CALLS` with the two allowed
  pairs; `maths_breach` with `enclosing_fn` and `is_name`; the float rule leaves every `libm` path to it; a fused
  multiply-add by its libm name stays the fused rule's, so 4b-2's `libm::fma` bite still finds one breach; 11 bites and
  3 controls); **the shared narrow marking** `crates/sim/tests/support/marking.rs` (its own tokenizer, raw names kept
  apart from keywords; test-only only a `#![cfg(test)]` file or a top-level `#[cfg(test)] [#[..]] mod NAME {..}`),
  the content panic scan switched to it and the old `in_tests` and `TEST_ITEMS` removed; two content controls became
  bites by the narrow rule (a top-level `#[cfg(test)]` const and fn), F5's `r#fn` bite added. **A finding for
  security-auditor (F9):** its shape (ii) names only `mod`; content's `map.rs:415` is `#[cfg(test)] pub(crate) mod tests`,
  so the marking takes a `pub` with or without its reach before `mod` (a visibility changes nothing of a cfg(test)
  module; the real content tree went red without it); a control covers the form. **layering.rs** (devtools): the
  manifests' half, the sources' half (both `lib.rs` found; 19 bites, 5 controls, T-G's line in a copy of a room and an
  endpoint file), the simulation's half (one handed-on constant). rustfmt run on the five files by full path; fmt
  clean; clippy `-D warnings` clean on the workspace; sim's 46 unit tests, the scan's 9, layering's 3 and the
  dependency rules' 3 pass. Earlier state (17:14), kept: **C2 is green in CI; C3 is being built in the worktree.** C2 =
  **14538bd74c029f3beab4bbb05006be8e8889a38a** (notes 411d8e96 before it): gate and smoke run 46162913451, toolchain run
  46162913429 (finished 16:13:28 UTC, 12 min 2 s). C3 so far (saved as `cs4c-soldier/c3-wip.patch` for the tracked
  files and `cs4c-soldier/c3-wip/` for the new ones, on the head this notes commit makes): the batch applied
  (`git apply` of `deps-4c-v2.diff`, its digest 0edaf8f3... checked; the five files then equal the approved five
  digests exactly); `fmath.rs` (the three helpers, `share_per_tick` by a rate, `facing_of` with the quarter turns,
  `DEG_TO_RAD`; tests: the helpers' signed zeros, the constant's bits, the quarter turns by bits, and 45, 120, 240 and
  the largest double under 360 against the route's correctly rounded reference, `facing-ref-c3.py`); `rules.rs`
  (`Rules`, `Rules::take` with its eight refusals, `HeadroomFeet` with `HEADROOM_FEET` = `SandboxTest`,
  `with_headroom_feet` under `cfg(test)`; tests: each of the 28 fields reaching its own rules and no other, each
  refusal by name with each limit playing, the shares within one f32 step of `share-ref-c3.py`'s decimal reference,
  the stamina's units); `soldiers.rs`'s `SpawnPoint` and `spawn_points` (ranked by id in byte order; W-2's order half
  tested); collision.rs's `narrow` as `pub(crate)`; F1 in `dependency_rules.rs` (sim's three exact lines, the root's
  libm line to the byte, one locked libm version, only `sim` names libm, with plants). sim's 46 unit tests pass and
  the dependency rules' 3. **Open:** rust-reviewer's block 17 (running: the vectors' form, whether the two libm
  allowances may sit on two functions returning the routines' own f64 results, the rate signature, `ads` as a speed,
  the shares' accuracy domain, the vectors' files), sent a finding of mine too: at the largest double under 360 the
  route gives x a688d313 where the true angle's correctly rounded x is a68efa35 (the constant's rounding outweighs a
  sine of 1e-15). Still to build: the libm scan rule, `tests/support/marking.rs` with layering.rs and the content
  panic scan switched to it, the vectors (after block 17), the plants, the checks; then security-auditor on C3's
  commit (F9). Earlier state (17:00), kept: **C2 is committed and pushed, CI running on it.** The code commit follows
  this notes commit (no `[skip ci]` on it, so CI runs on the head). Its proof, on the very tree committed (`git diff
  --binary` compared byte for byte with `c2-wip.patch` after the plants, the checks and the gate): the plants' whole
  run again with V1b (`plants-4c2.txt`: 15 of 15 caught, 5 of 5 held, every file put back); the full local checks
  passed (`.gate/local-checks-c2.log`: the players' file d16cb57d..., 258,455 bytes in both builds, from C1's
  a624ff32..., 257,421; the browser tests 29 of 29 in 211 s); the gate PASS with smoke (`.gate/gate-c2.log`). The count
  against 946fef73: **901 written lines** (C1 442, C2 459; `content/` is outside the count's folders). `c2-wip.patch` is
  removed in this commit (C2 is in the code commit after it); C1's copies (`c1-wip.patch`, `c1-wip/`) are stale too
  and are not applied by anyone. **Next: C3** (`step1-build-order.md` C3; drafts of `fmath.rs`, `rules.rs` and the
  spawn table in the scratchpad, not in the tree: `share_per_tick` takes a rate, so the push's strength goes in as
  written and the easing's rate is ln 10 over its time, a departure from 11.5's signature to name in the report).
  Earlier state (16:50), kept: **C2's plants passed once (14 of 14 caught, 5 of 5 held, every file put
  back: `plants-4c2.txt` on f21bf71e plus C2); the first full local run stopped at the types package's type check (its
  `index.test.ts` built a movement table of 15 fields: fixed, 13 made-up values as in `TEST_TEXT`, its key count 29);
  the ruling's header-level test added (`read.rs`, `the_header_takes_the_newest_version_only`, header at newest 2:
  2 read, 1, 3, 0, 1.5, -1 refused; its plant V1b). Next: the plants' whole run again, the full local checks, the gate,
  the tree compared with the patch once more, then C2's commit.** The patch is now 21 files, written again on
  f21bf71e (apply with `git apply` on f21bf71e; the notes commit changed no source). The advisor tool's checks on C2,
  done: the verdicts file's 38 digests all moved because `case()` draws a whole movement file first for every class,
  maps and bundles too (`content_proofs.rs:561-562`; 28 fields draw more), while each case is still held to its own
  class's verdict at every regeneration and check (`content_proofs.rs:1117`): a per-kind count would differ from the
  new draws alone, so the per-case hold is the check; the nine VERSION-RULE conditions ticked against the diff (the
  schema's `"const": 2` at `movement.schema.json:189`; `testlines.test.mjs`'s remaining `version: 1` is the hand-over
  table's own format version; content.test.ts's and report.test.ts's are the map's or the bundle's; the mixed case is
  map 2 with movement 1). Earlier state (16:43), kept: C2 is written and its tests pass. Saved as
  `cs4c-soldier/c2-wip.patch` (`git diff --binary HEAD -- crates packages content` on 43897d29: 20 files). Done since 16:32:
  the proofs' committed digests (`content-verdicts.txt` written again by `SHAMBLE_WRITE_VERDICTS=1`); the frozen bundle
  re-frozen (`make_recordings --freeze first-loop`: only `movement/soldier.json` differs, version 1 to 2 and the
  thirteen new fields; the map the same) and the recordings made again: **`first-loop.rec` differs in 32 bytes only,
  offsets 51 to 82, the header's bundle sha256 (now f0093b57...); `first-loop.hashes`' 1,800 tick lines are byte for byte
  the same, only its first line changes (it names the `.rec`'s sha256); `first-loop.vector.txt` is byte for byte the
  same.** So G5 holds (no state changed); the build order's "`.hashes` byte-identical" was loose wording: a `.hashes`
  file's header names its `.rec`, which names the bundle. The whole workspace: 246 Rust tests pass, 0 fail (two_instances
  and two_processes too). C2's plants are written in `plants-4c2.py` (V1 to V3, F1, F2, S1 to S4, T1 to T3, controls
  VC1 to VC4; anchors 14 of 14 once). Not plantable: `bundle.rs`'s exact version rule (the bundle's VERSION is 1, so
  the rule's lower end changes nothing today). Earlier state (16:32), kept: step 1's C1 is committed and green, C2 is
  being built. C1
  (71e0f857: the soldier table, the aim's type, the vector helpers; no state change: the players' file a624ff32...
  unchanged, the recordings unchanged): the full local checks and the gate with smoke passed before its commit; its
  plants R1 and R2 caught, RC1 held; CI green (gate and smoke run 46159411516, toolchain run 46159411530, 10 min 49 s);
  442 written lines against 946fef73. Rulings since: the advisor agent's delta verdict (D1 to D3 with G1 to G12),
  G1's two yeses, rust-reviewer's per-commit table (block 16), the VERSION-RULE ruling (the movement kind to
  version 2), game-designer's thirteen rows (block 10). **C2 in the worktree, not committed:** the thirteen rows in
  the table by script (`add-rows-c2.mjs`), the converter's two new row forms (sandbox rows, the `measured.` place)
  with its tests (17 pass), movement.rs at version 2 with 28 fields and its tests, the loader and the bundle taking
  the newest version only, the schema's thirteen descriptions, the generated type and schema written again,
  `soldier.json` written by the converter (28 values: 16 from his Export, 12 from his sandbox), the page at version 2
  with its tests (182 pass), the fuzz's movement versions; still to do: the proofs' committed digests, the frozen
  bundle's re-freeze and the recordings (their `.hashes` byte-identical), the plants, the full local checks. A slip:
  one vitest run was started with `cd ... &&` in front (the worktree's own `cd` put back at once). Earlier state
  (15:49), kept: **F1 closed** (engine-architect's block 7, confirmed by rust-reviewer's block 14; B5 (c) confirmed);
  **game-designer's block 9:** the lowest-id spawn is a build default, not his (the early look's step 1 names
  `street_1` as a stand-in); **security-auditor's third answer:** REVISE on layering.rs's wording only, the batch's
  approval standing (a narrow test-only marking with its plants, in `step1-build-order.md` C3; its F5 and F6, Lows);
  **netcode-architect's block 9** (written whole before the restart, committed after it at f63d7eb0): B4 yes with three
  conditions and tests W-1 to W-4; step 1 carries the wire's move and aim through the room with both checks (a plan
  delta: T2-6 and T2-7 from step 2 to step 1); Q-1 (the step only writes its query counts). **Running:**
  rust-reviewer's block 15 (folds all of it into 11.1, 11.6, 11.7, 11.9; grounded at spawn; the panic scan's leak; the
  spawn ranking). **Next:** the advisor agent on the plan delta (F5's rule: a table that departs from its read comes
  back to it), then the advisor tool on the build order, then C1. Earlier state (15:32), kept: **F1's written answer is
  engine-architect's block 7** (the room owns
  rules, statics and spawns; `World` holds the tick, a `PlayerRecord`, the soldiers and the seed; 17 changes to
  rust-reviewer's 11.6; B5, the bundle's one movement document, blocks step 1's code until confirmed; B4, slot 0's
  soldier at the lowest-id spawn, a default). Q2's re-run is done (rust-reviewer's block 13: the test scene and every
  recorded route the same under (a) and (b); question 13 corrected). **Running (three):** rust-reviewer's block 14
  (confirms F1's Part A, the 17 changes and B5; rewrites 11.6), netcode-architect's block 9 (B4; which step carries the
  wire's move and aim; F5's superseded marks at its :355 and :430), game-designer's block 9 (whether the first
  soldier's spawn is a rule of his). **Next:** security-auditor on layering.rs's non-test wording (its F2); then step
  1's build order (small commits; the batch in the commit that first calls exp and sincos; the pins re-made in every
  commit that changes the hashed state), the advisor tool's check before the first code. Earlier state (15:09), kept:
  **the advisor agent APPROVES steps 1 to 7 with conditions**
  (`advisor-steps1to7.md`): Q1 the plan, with F1 to F6 as conditions; Q2 HEADROOM-FEET's default **(b), his sandbox's
  rule** (its ruling), with test 15 per mode and a re-run of blocks 5, 6, 9, 10 and 12 under (b) before step 1's
  movement code; Q3 the rung at about 14,960; Q4 the plan's order on nine conditions; Q5 the information lines agreed.
  **Before step 1's code:** F1 (where the world's rules, statics and spawns live: engine-architect's ruling, "Not in
  World", against block 11.6) answered in writing by both agents; Q2's re-run. **Before step 1 counts as done:** F2
  (PM-0004-20's and -35's new wording drafted, old and new side by side, keeping the three things it lists), F3 (test
  21's faces and misses, green). Before each of steps 2 to 7: its symbol table, confirmed by its owner (F5). His
  checklist with every expected result (F4); security-auditor on the dependency commit, engine-architect on step 1's
  boundary (F9). The Lows F7 applied (the sheet's 3.05 m ledge and 0.069 m, 6b's line, the card's rung); F8 (the ray
  counts and sweep proof) placed at step 6. Question 13 gains the two lines (the default (b); the flicker under (b)).
  Earlier state (14:55), kept: **4c-1 closed, 4c-2 open.** 4c-1's final head 946fef73 (the advisor
  agent APPROVE; CI green; the zip sent to him); the notes back (the revert commit); next: the advisor agent on steps 1
  to 7 (`advisor-steps1to7-brief.md`: Q1 the plan, Q2 HEADROOM-FEET's default, Q3 the rung, Q4 the order, Q5 the
  information lines), then step 1 by rust-reviewer's block 11 with block 12. His Brave check of 946fef73 and his go are
  his, from Sunday 29 November, 12:00. Earlier state (14:31), kept: **the verifier PASS** (`verifier-4c1.md`: FAIL on the report's first
  draft for its words only, two Mediums and five Lows, each corrected; the re-check PASS with one Low more, corrected;
  every number it checked held, its own full local checks passing on 0645432f's code, 29 Chromium tests in 231 s).
  Its follow-up V-1 and one citation are in be276ef3 (docs). **Next, in order:** this notes commit (the last one of
  4c-1); then (a) the report in `docs/reports/2026-11-28-cs4c1-scene.md` naming this commit as where the notes stand
  whole, with STATUS's row and ROADMAP's entry brought up to date, the checks run again and the gate with smoke; main
  fetched again (dae6d107 at 14:16, unmoved); then (b) the removal only, no `[skip ci]`, pushed as the final head;
  then the freeze until the advisor agent's verdict (`advisor-final-4c1-brief.md`, the head's id and its CI runs in
  the request); the zip built meanwhile from the final head's clean tree (`make-zip-4c1.py`, read out with `git show`
  into `.gate/`); then the zip to him, the final head's full id and the Brave steps, and the board. Earlier state
  (14:12), kept: the report's draft (`report-draft-4c1.md`), the pull request's text
  (`pr-text-4c1.md`) and the verifier's brief (`verifier-brief-4c1.md`) are written; the gate PASS with smoke at
  9283d302 (`.gate/gate-4c1-docs.log`, symbols +0 -0); the verifier next.** The close's order, by the advisor tool's
  check of 14:05: the verifier on the draft; then (a) one commit with the report in `docs/reports/2026-11-28-cs4c1-scene.md`
  and STATUS's and ROADMAP's 4c-1 lines brought up to date (the docs checks, repo-map-check and the 12 b/c check run
  again), main fetched by name first and taken in if it moved, the gate with smoke on that tree; (b) one commit that
  only removes the notes (`.checkpoints/cs4c-soldier.md`, `.checkpoints/cs4c-soldier/`, `.checkpoints/cs4-walk/`), no
  `[skip ci]`, pushed as the head; **then a freeze: no commit at all on the branch until the advisor agent's verdict**
  (in-flight files in `.gate/` or the scratchpad; past 25 minutes, one line to him instead of a push); the zip built
  while CI runs (3b's `make-zip.py` structure: the ids as arguments, SHA256SUMS, the unpack control, the browser spec
  on the unpacked copy; Node 18 or newer as `serve.mjs` says; Windows steps; the readout's commit in step 1); then the
  advisor agent; then he is told the head and the Brave steps. 4c-2's first commit brings the notes back (a revert of
  (b), the guard permitting). For 4c-2: HEADROOM-FEET's switch default goes to the advisor agent explicitly; this chat
  proposes (b), his sandbox's rule, until he answers (his MOVE-RULES (a)). His next chat update owes him two
  corrections: the walk difference is 0.069 m (not "6 cm"), and question 13's sentence about his sandbox was wrong.
  Earlier state (14:03), kept: **4c-1's docs pass is committed (9283d302**, docs-keeper's, its checks
  passing; STATUS's quotes of his item 12 b and c compared byte for byte with his saved message by
  `quote-check-12bc.mjs` in the scratchpad: both exact). CI green on 0645432f (gate, smoke, toolchain 9 min 3 s;
  Chromium 3.6 min). Question 13 corrected at 13:58 (game-designer's block 8: "the sandbox's maps never met it" was
  wrong; the swept wall pass's numbers as information, no question 14) and posted on the board as needs-musti at
  37776da1 before the correction. rust-reviewer's block 11 (step 1's consolidated table, 25 tests) and block 12 (tests
  7 and 8 off the roofs, each plant still caught; the ridge flag as a follow-up; the 3.05 m ledge, 0.069 m) are done.
  **Next:** the report's draft and the pull request's text in the notes, then the verifier on them. Earlier state
  (13:50), kept: **4c-1's code is done and reviewed.** The fix pass is committed (2b2f44c2,
  CI green: gate, smoke, toolchain 9 min 53 s) with one comment reworded (0645432f, CI running); `fix-s-wip.patch` is
  stale (its code is in 2b2f44c2). The review: two rounds, round 2 APPROVE from graphics-engineer and test-engineer
  (`review-s-round.md`); the plant run on 2b2f44c2 caught 45 of 45 with 3 of 3 controls (`plants-4c.txt`). Running:
  docs-keeper's docs pass (ROADMAP, STATUS, REPO_MAP, and his line 16 pointer moved by `place-4c.mjs fix-16`). **Next
  for 4c-1, in order:** STATUS's quotes of his item 12 b and c compared byte for byte with his saved message; the
  verifier on everything (it retakes the after picture); the report with the pull request's title and text; the notes
  out of the head in the final commit; a green CI run on that head; the advisor agent on it; the zip built from a
  clean tree at that head and sent to him at once (SendUserFile); then he is told the final head's full id and the
  Brave steps (`zip-readme-4c1.md`). **For 4c-2:** HEADROOM-FEET is his (question 13); game-designer rules again on
  the swept wall pass with block 10's numbers (question 14 if it is his); rust-reviewer writes step 1's consolidated
  symbol table (block 11); then the advisor agent on steps 1 to 7. Earlier state (13:21), kept: step S's fix pass was
  then in the worktree (saved as `fix-s-wip.patch` at 18454538).
  Step 0 landed (af789ef3 and the next commit). The review round is complete (`review-s-round.md`: six reviews, the
  Medium found four times and TE-1 fixed with the Lows; four Lows and AD-4 deferred). Type checks and the unit tests
  pass on the fix (the page's 176, the Worker's 53); the full local checks run (`.gate/local-checks-fix-s.log`); then
  the whole plant set on the fix (`plants-4c.py`, 17 new plants F1 to F17 beside the 31, a timeout for F1's hang; the
  anchors 0 wrong), the code commit, CI, the verifier. game-designer's Testlines read resumed after the restart (N16
  to N19 done: FALL-STEP (a) by the code, the head bump, the headroom rule). Earlier state, kept: **his answers are in
  (12:10; "Decisions taken"), and he is away until Sunday 29 November, 12:00.** Now: step S's six reviews (three running: graphics-engineer, perf-auditor,
  engine-architect, each writing `review-s-<agent>.md`), step 0 (the register by `decisions-4c.mjs`; LOOP_V2's section
  14, the pointers, the header sentence, EDITOR_V2's pointer and the register line placed by docs-keeper, his item 2),
  then the fix pass, the verifier, the docs pass, the report with the pull request's title and text, the notes out of
  the head, CI green, the advisor agent on that head; then he is told the final head and the Brave steps, and the
  soldier is built on top (his item 6). Earlier state, kept: step S is built and proven on the branch: code commits
  bc7720f3 (the scene) and c1de41e5 (the volume check). **CI on 8bee8365 (the head holding both): gate, smoke and toolchain green**
  (the toolchain 01:00:03 to 01:10:04 UTC). Step 0 is not written (it waits for his answers). The dependency batch for
  the later steps (libm in `sim`, `content` in the tests of `room` and `client-endpoint`) is a patch in the notes, NOT
  applied: `deps-4c.diff` with its digests `deps-4c.sha256`, made in the scratch worktree `.gate/deps-4c`.
  **security-auditor: APPROVE** on exactly those five digests (`security-auditor-deps-4c.md`), with two conditions on
  the commit that applies it (F1 Medium: devtools' `dependency_rules.rs` updated and tightened in the same commit; F2
  Low: `tests/layering.rs` lands in the same commit), both written in `dependency-check.md`. No saver runs: the main
  session commits and pushes the notes by hand, at least every 25 minutes, and holds notes pushes while a code head's
  toolchain run is going.
- **Next:** his answers are in (2026-11-28, 12:10), but for HEADROOM-FEET (question 13, asked while he is away). Finish
  4c-1 by the State line above, then 4c-2 on top of 4c-1's final head (his item 6). The earlier "Next" (waiting for
  his twelve answers) is done.

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
  - security-auditor (`security-auditor-deps-4c.md`): the dependency hold's answer on the batch `deps-4c.diff`, by
    the digests in `deps-4c.sha256` (started just before 02:04 Amsterdam, the clock read right after it): DONE,
    APPROVE with F1 and F2.
  - Started together just after 02:10 Amsterdam (the clock read 02:10 before the launch, 02:14 after it), each in
    its own file: rust-reviewer's **block 6** in `rust-reviewer-read.md` (the bodies 3a, 3b and 2 on block 5's "not
    run" list; the 2.5D question; its yes or no on the record rule); engine-architect's **block 5** in
    `engine-architect-read.md` (GROUND, FACING-SPAWN); netcode-architect's **block 7** in `netcode-architect-read.md`
    (the record rule, the bounds' one home, the zero ray). netcode-architect DONE: the record rule YES ((i), the
    soldier's hashed row holds `held_move: [i8; 2]` and `aim_cm` in 8 bytes; a command-less tick walks on and turns
    toward the held aim; the restore test plants 30 command-less ticks; step 2 moves no first-loop hash on four named
    conditions), the bounds' one home YES (the loader's own grid rule, its "off the grid is refused" withdrawn), the
    zero ray YES (Z1, Z2); step 2 rises to about 340 / 675, step 1 gains about 10 / 100; whether `Room::new` and
    `Endpoint::new` refuse a world with no aim area is engine-architect's call.
    engine-architect DONE (block 5, N27 to N41, 5.1 to 5.8): **GROUND** reconciled (the three statements agree on
    every map that exists, all starting at y 0; G-1 Low: map.rs:39, schema.rs:146-147 and aim_reach.rs:45 call y 0
    "the floor" though map.rs:325 lets it sit higher; G-2 Medium if built as written: netcode-architect's aim
    fallback to the plane y 0; G-3 Low, step S's evidence: no test tells the bounds' bottom from y 0,
    scene.test.ts:149 checks the literal 0) and ruled: the bounds' bottom is solid ground (its words in step 1's
    commit; tests T1 to T3). **FACING-SPAWN:** recommended (a), one `libm::sincos` per spawn at the match's start,
    quarter turns exact; it changes the batch's root comment, so the patch is reworded before it is applied and
    security-auditor answers once more; nothing to him under (a) (FACING-QUARTER only if a reviewer wants (b)). The held
    aim at spawn: the spawn point in whole centimetres. Questions RR1 to RR5 for rust-reviewer (RR1 Medium: the scan
    refuses std's names only, so `libm::sincos(` and `libm::expm1(` pass today) and NA1 to NA3 for netcode-architect,
    sent after rust-reviewer's block 6.
    rust-reviewer DONE (block 6, `rr-body-proto-6.rs`, outputs `-3a`, `-3b`, `-2`, `-2w`; with the new options off it
    reproduces block 5 byte for byte): (3a) / (3b) / (2) pass ramps met from the side 306 / 306 / 306 (body (2) only
    with a one-line wall fix, "2w"; without it 108 of 306 walk into the ramps), landings 240 each, the snap down the
    ramps 6 each (smallest margin 0.094 m), easing from rest 72 each, the push's 16 cases with the sandbox's push-out
    and the plan's sweep (nobody stopped on open ground, none inside a solid, the 0.25 m cap never bit; ties must go by
    soldier id). Two model holes every body shares, each one condition and no rule of his (the in-air step's "toward"
    from the tick's start; a top crossed inside one tick counts as landed). The 2.5D count for step 1 honestly about
    205 / 265 (shapes may have space under them: VISION_V2:90-91, LOOK_V2:32), against 3D's 260 / 260. The record rule
    YES (F17 stands; the row also holds the body's carry-over: `pull: u16` under (3a), `stepped: u16` under (3b)).
    Two open points: CEILING (it says the sandbox has no head rule; but game-designer-cs4.md:29, F18, reads "head bumps
    a floating block", movement.ts:132-138, and MOVE-RULES row 8 already lists it: correction sent to it and to
    game-designer at 02:35) and an unjumped fall from the 0.55 m step gaining the 0.56 m step 2 m away (to
    game-designer). Follow-ups running from about 02:33 Amsterdam: rust-reviewer's block 7 (RR1 to RR5, FACING-SPAWN
    from the maths side, the CEILING correction), netcode-architect's block 8 (NA1 to NA3, G-2), game-designer's new
    block (the unjumped fall; the head bump from its notes).
    rust-reviewer's block 7 DONE (scratch `rr-trig-*`; libm 0.2.16 built offline from the cargo cache under
    `.gate/rr7/`, after the batch's APPROVE): **RR1 confirmed, Medium**: the scan refuses std's spellings only
    (rules_scan.rs:52-62, :386), so `libm::expm1(`, `libm::sincos(`, `libm::sincosf(` and `libm::expf(` pass today, and
    a grouped import with a bare call passes even std's names; security-auditor's planned bite "libm::expm1 still
    refused" would be red, so its item 3 is corrected at its next answer; the fix (libm treated as the collision
    library: one file, never imported, full-path calls only, one allowance per function and routine) lands in the
    commit where exp first joins, about 25 / 40. RR2: `libm::sincos` in f64 then narrowed (0 of 90,000 off;
    `sincosf` off on 49,125 by up to 19,437 ulp); K's bits pinned. RR3: quarter turns exact; the -0.0 trap (write
    `0.0 - v`, add `+ 0.0`). RR4: four libm names in a wasm32 build (`libm::math::sincos`, `rem_pio2`,
    `rem_pio2_large`, `exp`). RR5: the floor a plane beside the tree; `cast_ray` stays shape-only. FACING-SPAWN (a): yes
    from the maths side; the batch's root comment changes, so security-auditor answers once more. CEILING: block 6
    corrected, it is MOVE-RULES row 8's head bump (F18); open in the notes: the speed after the bump, the position fix,
    the shape tested, the headroom formula (F19).
    game-designer's block 5 DONE (ESCALATE on one rule): **FALL-STEP** goes to him (the question file's question 11,
    written at 02:40 Amsterdam): whether an unjumped fall steps up onto a slightly higher top met in the air; its notes
    lean to yes but do not settle it; the old game does (legacy/shamble-game.js:2531-2540); options (a) yes, (b) no
    (the prototype's runs again), (c) he tries it in the sandbox or lets it be read again; recommended (c), else (a);
    PM-0004-30 holds as worded; the 0.56 m test starts on the ground beside the step. The head bump: the sandbox's own
    rule (F18), what it does not in its notes.
    netcode-architect's block 8 DONE: NA1 YES (the aim's fallback meets the floor, the bounds' bottom face, in whole
    centimetres; no hash moves), NA3 YES (FACING-SPAWN (a) changes neither the snapshot's soldier record nor the
    recordings; the general sine and cosine rest on PM-0004-22's vectors), NA2 NO to both options and in their place a
    new protocol meaning, (-32,768, 0, 0) = "no aim yet" (the row holds it from spawn, the spawn facing stays until a
    real aim, never through the bounds' clamp; `Endpoint::new` has no first aim; tests N-1 to N-3), which revises its
    block 7 and needs rust-reviewer's and engine-architect's yes (asked at 02:44). Counts: step 2 about 335 / 690, step
    1 about +25 / +145 (from +10 / +100).
    **FACING-SPAWN (a) picked with its reviewers** (engine-architect, rust-reviewer, netcode-architect). **The batch
    reworded** (`deps-4c-v2.diff`, 85 lines, `deps-4c-v2.sha256`): only the root comment changed (it names `sincos`
    for the spawns' facings), so only Cargo.toml's digest moves (fc436ec3... for 099d224b...); `git apply --check`
    passes; `cargo metadata --offline --locked` exits 0; `dependency-check.md` marked to be rewritten. The two v2 files
    are pushed before the answer (8b3c689b, between 02:44 and 02:48 Amsterdam: no clock was read at that push; an
    earlier save wrote "02:46" here unread): a patch in the notes is no manifest or lockfile, which is all DEP-HOLD names,
    and the advisor agent's item 8 keeps the batch a patch so that pushes are never held; nothing applies it before
    the answer (the first version's files were held back by the advisor tool's caution, which is not needed for a
    patch). rust-reviewer's block 8 DONE: YES to "no aim yet" = (-32,768, 0, 0): the row's `aim_cm: AimCm` (i16, u16,
    i16) becomes required and holds NONE from spawn, beside `facing: [f32; 2]`, `held_move: [i8; 2]` and the body's
    carry; the facing rule compares the integers first and leaves the facing bit for bit on NONE; tests S-1 to S-3
    beside netcode-architect's N-1 to N-3; first-loop's hashes move in step 1 (the row joins the hashed state), and
    step 2 moves none only if NONE lands whole in step 1 (endpoint.rs:258 writing NONE in place of (0, 0, 0)).
    engine-architect's block 6 DONE: "no aim yet" in the block YES, with `own_flags` bit 1 "aim held" (clear while the
    aim is NONE and before the first snapshot that carries the soldier; the aim then written as 0, so NONE never
    crosses to the page as a number), the owner's section showing the held aim from the newest snapshot (the endpoint
    no longer holds one), the row's pitch 0 and the yaw the spawn facing while NONE; the page draws no aim line and the
    readout says "none yet"; the section stays 20 bytes, the block 1,988; about 8 / 20 in steps 3 and 4; it changes
    the meaning netcode-architect's 2.1 (:240) gives the owner's aim, with no wire change (for netcode-architect to
    note at step 2's review). A world with no aim area: NO constructor change (the loader already refuses such a
    playable map, map.rs:13-14; only a map built in code can lack it, and it fails safe), one room test with a planted
    panic, about 0 / 10. So "no aim yet" has all three yeses.
    **security-auditor on the reworded batch DONE: APPROVE** on its five digests (Cargo.toml fc436ec3..., the other
    four unchanged) and the patch (0edaf8f3...); F1 stands (with the renamed-dependency point); F2 stands, its root
    comment half closing with the maths list's four exact entries, the exp and sincos vectors and maths-allowed.mjs:9-10
    rewritten in the same commit; item 3 corrected by RR1 and RR4 (the scan's stricter libm rule lands in the commit
    that applies the batch; content's refusal of facings outside [0, 360) stays, since it keeps rem_pio2_large, the
    one target-dependent routine, unreachable). `dependency-check.md` rewritten from it (02:49 Amsterdam, from the clock). No agent
    runs now. Earlier note, kept: running from
    02:44 Amsterdam: security-auditor on the reworded batch (with RR1's correction of its item 3), rust-reviewer's
    block 8 and engine-architect's block 6 ("no aim yet"; the aim area in the constructors).
- **The sheet's revision 3** (from 02:14 Amsterdam): the record rule applied with its confirmations outstanding, the
  recount F2 with each option's stop line, section G for steps 1 to 7 with his checklist as a draft, section H's table
  of item 10's thirteen points.
- **BOARD-WORDS (b)'s method written and tested** (condition 7, F-S0-1; by 02:21 Amsterdam): `chat-words.mjs less`
  and `decisions-4c.mjs`'s reading of its header; tested in the scratchpad only (the sheet's section C has the
  numbers). His message of 23:59 is still in no file of the repository; it is saved only when he answers BOARD-WORDS.
  **Changed by 02:24 Amsterdam** after the commit security review (Medium): the whole message's bytes and sha256 go
  into no file of the repository (with the shown text published, they would let a reader confirm a guess of the
  left-out line); they go to him in the chat. The local commit bfe447a3 had named the whole length and an 8-character
  digest prefix in the sheet: fixed forward in the next commit, not rewritten (the Never list), and told to him.
- **Step S's six review briefs written** (`review-briefs-step-s.md`), ready to send the moment SIZE-4C starts them.
- The scene's before picture: `.gate/scene-before-4c.png` (main's page, 3 draw calls), by `shot-4c.mjs`.
