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
- **Last update:** 2026-11-28 02:44 Amsterdam (from the clock; an earlier save said 01:08, a time written without
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
**Step S, the scene: the build** (by the sheet's section D, revision 2). The branch rule (condition 2): until he
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
2. **The dependency batch: answered** (APPROVE, 2026-11-28; `dependency-check.md` written by its form, the five
   digests checked again with `sha256sum` against the scratch worktree). The batch is applied only in the step that
   first needs it, after SIZE-4C, in one commit with F1's test and F2's layering test; FACING-SPAWN is settled before,
   since its option (a) would change the root comment the auditor answered on (then a new answer). For rust-reviewer
   when exp is first called: the answer's item 3 (one named allowance in `rules_scan.rs`, the exact paths in
   `maths-allowed.mjs`, PM-0004-22's vectors, the feature pin, one exhaustive native run).
3. **The advisor agent's item 10, as notes and prototypes only (condition 2):** rust-reviewer's block 6 (the bodies
   3a, 3b and 2 on block 5's "not run" list: ramp sides with the set-back, a ceiling on a made map, landing, the push
   with two soldiers; what the 2.5D form does with a shape whose bottom is above the ground; its yes or no on the
   record rule); engine-architect's block 5 (GROUND against map.rs:39, map.rs:325 and the height scene.ts draws the
   ground at; FACING-SPAWN's options, any pick that narrows a map file going to him); netcode-architect's written yes
   on the record rule when a slot is free; the sheet's revision 3 (item 6 applied with its confirmations outstanding;
   the recount of steps 1 to 7 with both the 2.5D and the 3D sweep counts until block 6 answers; section G per step).
   The advisor agent is asked to approve steps 1 to 7 only after his answers on BODY-SHAPE, SWEEP-ROUTE and the slope.

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
- Step S as built: 1,288 written lines (bc7720f3 1,277, c1de41e5 11; `count-4c.mjs` at 3ab59bd7 against dae6d107),
  about 1.74 times its recount of 740.
- Steps 1 to 7 recounted (the sheet's F2, revision 3.1 with the blocks of 02:25 to 02:33 Amsterdam): about 8,190 to
  8,290 by the designs' count (the plan 7,315); by the plan's rule (2.25 times) a stop line of about 18,420 to 18,640;
  each SIZE-4C option's line, two thirds and rung are in F2.
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
- **State (02:10 Amsterdam, from the clock):** step S is built and proven on the branch: code commits bc7720f3 (the
  scene) and c1de41e5 (the volume check). **CI on 8bee8365 (the head holding both): gate, smoke and toolchain green**
  (the toolchain 01:00:03 to 01:10:04 UTC). Step 0 is not written (it waits for his answers). The dependency batch for
  the later steps (libm in `sim`, `content` in the tests of `room` and `client-endpoint`) is a patch in the notes, NOT
  applied: `deps-4c.diff` with its digests `deps-4c.sha256`, made in the scratch worktree `.gate/deps-4c`.
  **security-auditor: APPROVE** on exactly those five digests (`security-auditor-deps-4c.md`), with two conditions on
  the commit that applies it (F1 Medium: devtools' `dependency_rules.rs` updated and tightened in the same commit; F2
  Low: `tests/layering.rs` lands in the same commit), both written in `dependency-check.md`. No saver runs: the main
  session commits and pushes the notes by hand, at least every 25 minutes, and holds notes pushes while a code head's
  toolchain run is going.
- **Next:** wait for his answers (the question file's eleven, and the two asks in the chat); meanwhile only notes and
  prototypes (condition 2): the advisor agent's item 10 before steps 1 to 7.

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
    are pushed before the answer (02:46): a patch in the notes is no manifest or lockfile, which is all DEP-HOLD names,
    and the advisor agent's item 8 keeps the batch a patch so that pushes are never held; nothing applies it before
    the answer (the first version's files were held back by the advisor tool's caution, which is not needed for a
    patch). Running from
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
