# Checkpoint: cs0-model-rules
- **Goal:** change-set 0 of the build plan (`.checkpoints/build-plan-slice-1/plan.md`, revision 2, approved by the
  advisor agent, his go of 2026-11-17 20:10 Amsterdam in `.checkpoints/build-plan-slice-1/mustafa-2026-11-17-go.md`).
  Docs only, no game code. It does four things:
  1. confirms and records the rules a model file must follow (plan Part 11; OQ-0005-AS3 and AS4 in ADR 0005), with
     the names of colour areas and joints fixed (his words: "the names of your change-set 0 win" over the Art
     briefs' working names), the export part's own pass marks placed at change-set M, and LOOK_V2 §8's line "Spline
     or Blender" (his words of 2026-11-03);
  2. records his decisions in `docs/DECISIONS.md` by script from this chat's transcript (his CHANGE 1): his block of
     2026-11-14 with its decision, his addition of 15:03 that day, his message of 2026-11-17 20:10 (his go, his
     answers to the twelve decisions, four changes; its change 2 holds a decision "to record word for word");
  3. changes STATUS and ROADMAP only where his decision changes what they say (PR #55 already made them true for
     main on 2026-11-17);
  4. carries the plan to main as a report (`docs/reports/`), with his messages in its appendix, and deletes both
     checkpoints (`build-plan-slice-1*`, `cs0-model-rules*`) in its last commit before the PR is marked ready.
- **Branch:** claude/wonderful-hamilton-jdmlnf (stands in for cs/cs0-model-rules) · **Base:** main
  c94c81b440c77f7979e0f944a5341319f9a57d1b (to take in first; the branch still sits on 4460ef5 plus checkpoint commits)
- **Last update:** 2026-11-17 23:17 CET (from the clock)

## Steps done
1. His go received (user message of 2026-11-17T19:10:44Z, saved by script, 10,544 bytes). The session's model read
   with get_session at the start of the turn: `session_context.model` and `last_served_model` both `claude-fable-5-2`.
2. The three Art briefs he attached copied byte for byte into `.checkpoints/build-plan-slice-1/inputs/` (the
   Conscript's first blockout, 2026-11-14 16:35; its addition of 2026-11-15; the SMG brief of 2026-11-15 17:35). The
   two pictures his block names (Sheet 16, his SMG line drawing) did not arrive with it. `Advisor_log_2.md` came along
   by mistake (his words of 2026-11-17, 20:20): it is not an input, it is not read and no part of it goes in the repo.
3. `git fetch origin main`: main is c94c81b (PRs #53, #54, #55): `.claude/CHECKPOINTS.md` (+20: §11, kept logs),
   `.claude/NIGHT_SHIFT.md` (+3), `.claude/scripts/kept-logs.mjs` and its test (new), docs: DECISIONS (+218), STATUS,
   ROADMAP, STATUS_ARCHIVE, REPO_MAP. No game file.

4. Main taken in: a plain `git merge origin/main` in the main folder (the guard lets it through when the branch
   changed no kit file; it changed only `.checkpoints/`). Told him in three lines: nothing in the plan breaks from
   #53, #54, #55; STATUS's Next is corrected by this change-set; the changes come from his own message.
5. Change-set 0's plan written (`.checkpoints/cs0-model-rules/plan.md`); `decisions-insert.mjs` written (not run).
6. The Advisor chat's log: one read-only agent was started on it at 19:16:35 UTC (the transcript's record) and
   returned a digest into this chat; it wrote no file. Five lines of that digest stood in this card from commit
   8ea40cd (19:44:56 UTC). On his words of 20:20 Amsterdam they are removed here (this update) and nothing of the
   digest is used; the lines stay in the branch's history (no history is rewritten: CLAUDE.md, "Never").
7. The advisor agent's review of change-set 0's plan (effort max; notes `cs0-model-rules/advisor-plan-r1.md`, DONE):
   **REVISE**: 1 High, 10 Medium, 5 Low. It needs to see 1 to 11 fixed.
8. Shamble VFX cloned read-only to /home/user/mergincanli/shamblevfx (head 92ec2dc) for Part 15. A guard refusal on
   the way: `git -C <dir> ...` ("no option before the subcommand"); route: cd into the folder, then git.

10. His two later messages saved by script: 19:59:06 UTC (his label 20:20; `cs0-model-rules/mustafa-2026-11-17-2020.md`,
    683 bytes) and 20:03:57 UTC (his label 20:58, sent while the turn ran; `mustafa-2026-11-17-2058.md`, 2,028 bytes).
    The 20:58 message brings a fourth Art text: the Art lead's correction of 2026-11-15, 18:40 (the gun poses,
    restated from the sockets), saved byte for byte as `build-plan-slice-1/inputs/ART_BRIEF_CORRECTION_2026-11-15_1840_gun_poses.md`
    (5,878 bytes: the Advisor's check above its line, the Art lead's text below; a second attached copy, 3,244 bytes,
    equals the text below the line). For change-set 0: only its rule and the socket names (plan, revision 2, point 17).
11. When his message of 19:59 UTC arrived, the harness stopped the two running agents and the saver (their last
    writes: 19:54 to 19:55 UTC; the saver's last line "[killed]"). The saver was restarted (task brf1n425m) and both
    agents were started again from their notes at about 20:05 UTC, each with its first brief copied by script from
    the transcript (`cs0-model-rules/engine-architect-brief.md`, `part-15/graphics-engineer-brief.md`).
12. Both record scripts dry-run on a scratch copy of the docs (20:07 UTC): `decisions-insert.mjs` (three entries equal
    to his words byte for byte, the appendix A1 to A10) and `pointers-insert.mjs` (DECISIONS 4 pointers, SPIKE_PLAN 3,
    ADR 0002 1; no line over 120). Not yet run on the repo's docs.

13. Drafts written in the notes while the joint-list ruling runs (20:16 UTC): the report's "since the plan" section
    (`cs0-model-rules/since-the-plan.md`; 44 quoted spans checked against his saved messages by `quotes-check.mjs`,
    all found), `report-build.mjs` (the plan copied from commit 68addf9 below that section, checked byte for byte).
14. Part 15, "The second lane": the Shamble VFX reading is done (`part-15/shamble-vfx-digest.md`, 97 lines,
    graphics-engineer, second run finished about 20:12 UTC); Part 15 is drafted
    (`part-15/part-15-the-second-lane.md`, 209 lines, six questions for him) and is with the advisor agent (effort
    max; notes `part-15/advisor-part-15-r1.md`). It is sent to him as a file after that review. It is not carried by
    change-set 0's pull request: it goes to main once he has answered it (a docs change of its own or change-set 1's
    docs pass); if `.checkpoints/part-15*` is deleted before that, the report and STATUS name the commit that holds it.

15. engine-architect's ruling on the joints is in (`cs0-model-rules/joint-list.md`, 98 lines; its second run ended
    before 20:25 UTC): 20 joints (the build plan's 16 plus four coat joints), `root` is the node above the skeleton
    and not a joint (against graphics-engineer's draft: the advisor agent rules), the sockets' axes, the hands'
    rule, the origin and facing check, the brief's 17 parts on the joints. Folded into the plan as points 19 to 22.

18. Part 15: the advisor agent's re-check came back APPROVE with four Lows, folded in; sent to him as a file
    (`part-15/part-15-the-second-lane.md`, 257 lines, six questions).
19. His message of 20:39:23 UTC (his label 21:33; `cs0-model-rules/mustafa-2026-11-17-2133.md`, 1,754 bytes): SIZE-0
    (a), one pull request, new stop line 1,200 written lines (copied lines do not count), his records and the plan's
    carry in commits of their own, the split rule if round 1 returns a Critical or High on the rules or the joint
    list that one fix pass cannot close; LOG-HISTORY (a), the history stays; A10 is the Art lead's text alone (the
    notes' copy replaced by the 3,244-byte one); his SMG line drawing arrived with it (one image, not copied).
20. The advisor agent's re-read of the plan: APPROVE with conditions (`cs0-model-rules/advisor-plan-r2.md`): its
    findings 18 to 22 fixed in the plan (points 23, 24, one files table) and in the scripts; ROOT ruled for
    engine-architect. graphics-engineer's update of the research draft is done (395 lines; mark 8 counts draws;
    points P1 to P3 for engine-architect in its notes). Both record scripts dry-run clean again (20:57 UTC): five
    entries equal byte for byte, appendix A1 to A11, six pointers in DECISIONS, three in SPIKE_PLAN, one in ADR 0002.

21. The docs build, his commit order (plan point 23). Done and pushed by 21:04 UTC:
    - commit 1, his records: 3747bb3 (five entries in DECISIONS by script, six pointers there, three in SPIKE_PLAN,
      one in ADR 0002; dated answers under OQ-0002-6 and OQ-0006-18, PM-0004-34's row, the register's rows and
      counts, 98 open becomes 96; the appendix A1 to A11) and its fix 4a1b0d7 (two Art texts arrived with CR LF line
      ends and the repository stores LF only: the appendix now gives the stored text's sha256 and, for A4 and A6, the
      attached file's too; `appendix-verify.mjs` on HEAD: 11 blocks, 0 mismatches);
    - commit 2, the plan's carry: 212f55b (`docs/reports/2026-11-17-build-plan-slice-1.md`, 1,421 lines: "since the
      plan" plus the plan, equal to commit 68addf9 below the mark);
    - commit 3, first part: the two open questions answered in part in ADR 0005 and the register, LOOK_V2 section
      8's line, one pointer in SPIKE_PLAN.

## (earlier current step, done)
22. Commit 3, second part: waits for two agents. engine-architect writes the docs version of the ruling
    (`cs0-model-rules/soldier-joint-list.draft.md`: citations repointed, graphics-engineer's points P1 to P3 ruled
    on, a consistency check of the rules draft). test-engineer writes the ten pass-mark rows PM-0005-47 to -56 for
    ADR 0005 (`cs0-model-rules/adr0005-marks.draft.md`; notes `test-engineer-adr0005-marks.md`) and lists what the
    research draft's part 5 must change. Then: graphics-engineer's file takes those changes, both research files
    move to `docs/research/` with the index rows, the rows go into ADR 0005. Then commit 4 (docs-keeper), the
    reviews on the final diff (engine-architect, test-engineer, the verifier), commit 5 (the report, the deletion
    of the checkpoints), the advisor agent on the final head, the pull request.

23. (21:05 UTC) The draft pull request is open: #56 (https://github.com/Mergincanli/Shamble-Frontlines/pull/56).
    Checks run here: `node tools/gate.mjs --no-smoke` PASS (checks 1 to 4, 5b to 5d); the gate's browser smoke test
    (5a) cannot start in this container (the gate's pinned Playwright looks for Chromium build 1194, the container
    has 1243, and no other browser is installed here): CI's `smoke` job runs it. `repo-map-check.mjs`: OK before the
    new files. `cd tools && npm ci` was run once for the gate's dependencies (nothing tracked changed).

24. (21:07 UTC) engine-architect's docs version of the ruling is done (`cs0-model-rules/soldier-joint-list.draft.md`,
    223 lines): citations repointed to the appendix and the report (its citations of the plan checked against the
    report: Part 11's rules 19, 20, 21 exist), P1 ruled (no node in a model file may be named `root`), P2 and P3
    taken. It found five mismatches in the rules draft (M1 to M5): sent to graphics-engineer, which is fixing them
    (second pass on the draft). test-engineer is still writing the mark rows.
    The size so far (`size-count.mjs`): 413 written lines in the pushed commits (2,201 added under docs/, of which
    1,788 copied: his words 123, the appendix's blocks 390, the plan 1,275). Still to come: the rules about 395, the
    joint list about 223, the mark rows, the report, docs-keeper's lines: about 1,150 in all, against his stop line
    of 1,200. The two research files have hard limits (400 and 225 lines); fixes tighten, they do not add.

25. (21:29 UTC) test-engineer wrote the ten mark rows (`cs0-model-rules/adr0005-marks.draft.md`: rows, its rulings,
    the section's texts, 21 changes for the rules draft, 3 for the joint list). Pasted into ADR 0005 by script and
    pushed: 65c8fe8 (PM-0005-47 to -56; the table now has 56 rows; SPIKE_PLAN's asset-spike lines point at them).
    The first part of commit 3 was 5f356ef. Size now: 479 written lines.
    Its three questions: O1-AT-M (ask Mustafa the draw-cap question in change-set M's plan, before M builds: taken,
    it is the plan already); MARKS-AFTER-EXPORT (a change that loosens a mark needs his word, the Blender recipe
    changes first: taken, written into ADR 0005's "When a mark fails"); RIFLE-ROUTE (may a script-written gun file
    enter the game: it is Part 15's question GUN-FILES).
    Running: graphics-engineer applies test-engineer's 21 changes to the rules draft; engine-architect its 3 to the
    joint list; docs-keeper's pass (STATUS, ROADMAP, the repository map, the reports index; at most 30 added lines;
    its brief: `cs0-model-rules/docs-keeper-brief.md`).
    His message of 21:15:31 UTC pasted the Art lead's correction once more: equal to the saved A10 (3,243 bytes
    without the last line end); nothing changes.

26. (21:39 UTC) Commit 3's third part is pushed: cacea55 (`docs/research/asset-export-rules.md`, 397 lines, with
    test-engineer's 21 changes and the five fixes M1 to M5; `docs/research/soldier-joint-list.md`, 224 lines; two
    rows in the research index). Written lines now: 1,103, of which a few are docs-keeper's in-progress lines
    (stop line 1,200; still to come: the rest of docs-keeper's pass, at most 30 lines in all, and the report, at
    most 60).
    Review round 1 on the final diff has started: the verifier (its brief: `cs0-model-rules/verifier-brief.md`;
    notes `verifier-cs0.md`) and test-engineer (the marks as pasted; the rules' part 5 against its rows).
    engine-architect's review follows when a slot is free. docs-keeper is still on its pass.
    One question passed on for the advisor agent's review of the final head: mark 10's line 6 (PM-0005-56) checks
    the loaded scene graph where its ruling said "on both backends ... the file's height".

27. (21:47 UTC) Review round 1, test-engineer: REVISE, 2 Medium and 4 Low, all word changes in rows (notes: block 2
    of `test-engineer-adr0005-marks.md`). All six applied and pushed: f61f43c (PM-0005-55's probe under a root
    scaled 1.2; planted defects for PM-0005-53's position checks and for check lines of -47, -48, -51, -56 that had
    none; "a copy of the passing file, entry or test set-up"; PM-0005-56's scale line within 0.5 mm; two wordings in
    the rules file). No line added. Its confirmations: the rows sit verbatim in ADR 0005, "When a mark fails" changed
    only by its three changes, the rules file holds all 21 items, SPIKE_PLAN's two lines are right.
    Mustafa asked where Part 15's six questions are (21:4x UTC): answered in chat with the file's place and the six
    questions in short.
    Running: the verifier; engine-architect's review; docs-keeper (29 lines so far by its notes).

28. (21:55 UTC) Pushed: 94aaae8 (commits 4 and 5: docs-keeper's pass on STATUS, ROADMAP and the repository map, 26
    lines added, STATUS still 120 lines; this change-set's report `docs/reports/2026-11-17-cs0-model-rules.md`, 67
    lines, the verifier's and the advisor agent's verdicts still pending in it; engine-architect's review APPROVE
    with 5 Low, four applied, the fifth ("the model's entry") deferred to change-set M's plan).
    docs-keeper left the reports index alone (it has no list of reports) and could not move lines to the status
    archive (outside its four files): a follow-up for the docs pass at #56's merge.
    Size: 1,195 written lines (`size-count.mjs`), 5 under his stop line of 1,200. A verifier fix that adds more than
    5 lines stops the change-set: tell him in three lines.
    The saver was stopped by its time limit at about 21:53 UTC and restarted (task b97h3rkxv).
    Still running: the verifier (first pass). Then: its short second look at f61f43c and 94aaae8; the advisor agent
    on the final head; the report completed; the checkpoints deleted in the last commit; the pull request marked
    ready; stop for his go.

29. His message of 22:10:34 UTC (his label 23:06; saved by script, `part-15/mustafa-2026-11-17-2306.md`, 2,574 bytes,
    commit 023e6f3): all six questions of Part 15 answered (a), with details (the copy of the VFX runtime is never
    edited by hand and the re-copy script can check; no hit-stop now; every gun file through Blender); "Lines that a
    finding of the verifier or of the advisor agent asks for do not count toward the 1,200. New scope still does.";
    "Yes, watch it" for #56 (subscribed to its activity). His answers are NOT recorded in DECISIONS by change-set 0
    (new scope): Part 15 with his answers goes to main in a docs change of its own, before change-set 1. STATUS's
    waiting line says so (4f20643).
30. The verifier's first pass: PASS at cacea55, 2 Medium and 6 Low (`cs0-model-rules/verifier-cs0.md`). All fixed and
    pushed: d6b6b30 (seven pointers carry his rule's last sentence, by `pointers-fix1.mjs`; `pointers-insert.mjs`'s
    own check fixed; the plan report's "since the plan": his sentence on the Art list, two times, measured sizes; the
    research files' line citations renumbered to this branch by `renumber-cites.py`, run once; spot-check counts in
    the research index) and a65d26f (the report tightened). Its spot-check: 71 of 72 key claims.
    Size: 1,206 written lines, of which 6 were asked for by the verifier's findings and do not count: 1,200 counted.
31. (22:17 UTC) The container restarted during the push of a65d26f (exit 137, a TLS error). Nothing lost: two local
    commits (a65d26f and the saver's last autosave 4857f19) pushed on the second try; the saver restarted (task
    bm929m5yy). `tools/node_modules` survived.

## Current step
32. Round 2, on what changed: the verifier's second look at the commits after cacea55, and the advisor agent on the
    final head. Then: the report completed (Outcome, the two verdicts, CI), the checkpoints deleted
    (`.checkpoints/build-plan-slice-1*`, `cs0-model-rules*`, `part-15*`) in the last commit, the advisor agent's
    word on that last head, the pull request marked ready, stop for his go.

## Next steps
1. Change-set 0's own short plan at `.checkpoints/cs0-model-rules/plan.md`; the advisor agent's review of it (effort
   max; APPROVE before any edit outside `.checkpoints/`).
2. Build, docs only:
   - `docs/research/` file for the export and slot-naming part (from `.checkpoints/build-plan-slice-1/asset-rules.md`
     and the plan's Part 11, plus the Art briefs' conventions: sockets `socket_grip`, `socket_support`,
     `socket_muzzle`, `socket_sight`, `socket_eject`; guns: +Z to the muzzle, origin at the grip socket; the soldier
     hangs on one root so it scales with one number; no skin colour on the first stand-in: hands and face are Dark);
   - ADR 0005: OQ-0005-AS3 and AS4 marked answered as rules with their proof still to come; the export part's marks;
   - `docs/LOOK_V2.md` §8 "Spline or Blender" (only that line; "no plain tubes or cylinders" at line 30 is NOT
     edited here: his words say the carry of his look decisions removes it);
   - `docs/DECISIONS.md` entries by script (three messages); STATUS and ROADMAP where his decision changes them; the
     queue line for the camera panel stays;
   - `docs/reports/2026-11-17-build-plan-slice-1.md` (the plan, with Part 14) and its appendix (his messages, the Art
     briefs as received); REPO_MAP rows.
3. Evidence and reviews: the verifier spot-checks the research claims against their sources and his words byte for
   byte; reviewers at effort max: graphics-engineer (rules), engine-architect (names, joints), docs-keeper pass;
   then the advisor agent on the final head. At most 3 agents at a time; start the saver when agents run.
4. Report (`/advisor-report`), fold and delete the checkpoints, PR (draft, then ready), stop for his go on the merge.
5. Then, not holding change-set 0: Part 15 "The second lane" (his change 2 and change 4), sent to him as a file:
   his proposal: the second chat (branch claude/optimistic-albattani-mdoonz, Opus 5.5 with Fable 5.2 on its advisor
   tool) takes the preview hosting check and step 2.6, the carry of his look decisions into LOOK_V2, the
   comparisons that stay (Part 3.2), and the picture track from 6 on (6 Look, 10 Body, 11 Feel, M); this chat keeps
   0 to 5, 7, 8, 9 and 12. Part 15 must say: agree or not with reasons; per unit of the second lane its folders and
   files, what it needs merged first, what it must not touch, how the two chats' pull requests take turns; one owner
   for every CI and workflow file; which chat builds the gun models; how change-set 11 uses Shamble VFX
   (github.com/Mergincanli/ShambleVFX: editor, runtime, export folder loaded by event name; his lean: load its
   exported effects through its runtime, build no effect twice), what must be proven (same picture as in the editor,
   cost inside the frame budget, how the runtime's code reaches this repo), what it changes in the sizes; and which
   line of `docs/EDITOR_V2.md` §7 needs a dated pointer.

## Open findings
- The chat is 79 % full (get_session at the start of this turn: 794,185 of 1,000,000 tokens; automatic compaction at
  901,600). Asked him to compact; work goes on. Heavy reading goes to agents; the main session keeps outputs small.
- The pictures: Sheet 16 arrived with his message of 19:59:06 UTC (his label 20:20; saved byte for byte in
  `cs0-model-rules/mustafa-2026-11-17-2020.md`): one image, seen in the chat, not copied into the repo. Its numbers
  match the soldier brief's where checked (1.80 m; shoulders 0.49; waist 0.33; hem 0.61 by 0.46; markings 0.022 and
  0.03). His SMG line drawing still did not arrive: the 19:59 message carried one image only and the 20:58 message none.
- His words of 20:20: stamp times from the clock, never ahead of it (the card of commit cb0fd7b9, 20:13:23, said
  20:25). From now on "Last update" is written from `TZ=Europe/Amsterdam date`.
- Refusals and routes: (1) 19:2x UTC, the guard refused `git -C <dir> ...`; route: cd into the folder, then git.
  (2) about 19:58 UTC, a Bash call was denied (a dry run of the two insert scripts on a scratch copy of the docs, one
  long command that began with `rm -rf` of a scratch folder); route: the same dry run as short commands in a new
  scratch folder, nothing removed.
- Plan changes his answers bring (to fold into the plan's record in the report, not silently): GATE-FOLLOW (b) with
  (i): this chat asks him for each gate and CI edit with its exact text; MACHINES: desktop RTX 2070 with i7-9700K,
  laptop class Intel Iris Xe or the AMD equivalent, the actual machines named before change-set 6, the look built
  for the desktop on High, never lowered on High to reach the laptop's floor; BRAVE-ROUTE "A, if not possible: B";
  COMPARE (b): the character-controller comparison (PM-0004-34) runs only if our own movement misses its marks;
  MODEL-PUBLIC (a), the sheet notes may stay; RULES-WHO (a); SCOPE-RELOAD: a simplified active reload joins
  change-set 8 ("hitting it will make the reload end faster"); START-RULES (b): the jump's reach 3.07 m (the
  formula's value), soldiers push each other softly and never block; SPLIT-25 (a); FIRST-SOLDIER (a); PREVIEW-START
  (a), started in the second builder chat; STAND-IN-LOOK (a). The 2.3 chat stays stopped: its lint and format
  settings will not be on main before change-set 2 (plan Part 7's case).
- His change 3: the soldier brief replaces the sizes typed into `art-sheets-2026-11-14.md` (1.80 m to the top of the
  hat, a placeholder); line widths: LOOK_V2 stands, the Art sheet's widths are a proposal, both switchable at
  change-set 6; the rifle file `AMK9_v4_stage1.glb` stays with him until asked for; poses: sockets win over hand
  points; the muzzle is where flash and tracer start, a shot's start in the simulation is data of its own; in the
  Art list "(never a plain tube)" is struck.

## Key numbers
- none yet

## Decisions taken
- His go on the plan as written (revision 2, 68addf9), 2026-11-17 20:10 Amsterdam: change-set 0 now, then the plan's
  order, each as its own pull request that stops for his go on the merge. "This go covers no merge."
- No game code before change-set 0 is on main (his change 1).
- Taking main in: the branch differs from 4460ef5 only in `.checkpoints/`. Route to decide at the current step:
  CLAUDE.md allows `git merge --ff-only origin/main` in the main folder and forbids other merges there; the branch
  cannot fast-forward (checkpoint commits). His words "Take main in first" cover the act.

## Running work
- saver: background task bm929m5yy, started about 22:17 UTC (after the container restart).
- agents: the verifier (second look) and the advisor agent (final head), started after this update.
