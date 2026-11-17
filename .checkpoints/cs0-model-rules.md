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
- **Last update:** 2026-11-17 20:25 Amsterdam

## Steps done
1. His go received (user message of 2026-11-17T19:10:44Z, saved by script, 10,544 bytes). The session's model read
   with get_session at the start of the turn: `session_context.model` and `last_served_model` both `claude-fable-5-2`.
2. The three Art briefs he attached copied byte for byte into `.checkpoints/build-plan-slice-1/inputs/` (the
   Conscript's first blockout, 2026-11-14 16:35; its addition of 2026-11-15; the SMG brief of 2026-11-15 17:35). The
   two pictures his block names (Sheet 16, his SMG line drawing) did not arrive: the upload folder holds five text
   files, among them `Advisor_log_2.md` (104 KB, not read by the main session, not copied: an agent digests it).
3. `git fetch origin main`: main is c94c81b (PRs #53, #54, #55): `.claude/CHECKPOINTS.md` (+20: §11, kept logs),
   `.claude/NIGHT_SHIFT.md` (+3), `.claude/scripts/kept-logs.mjs` and its test (new), docs: DECISIONS (+218), STATUS,
   ROADMAP, STATUS_ARCHIVE, REPO_MAP. No game file.

## Current step
4. Take main in (see "Decisions taken": the route), then tell him in three lines whether anything in the plan no
   longer holds.

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
- The two pictures of his block did not arrive (Sheet 16 `s16_engine_soldier.jpg`; his SMG line drawing).
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
- saver: not running (no agents yet)
