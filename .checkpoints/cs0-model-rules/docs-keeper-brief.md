# Brief: docs-keeper's pass for change-set 0 of the build plan (effort max)

Task name: cs0-docs-keeper. Docs only. You edit `docs/STATUS.md`, `docs/ROADMAP.md`, `docs/REPO_MAP.md` and
`docs/reports/README.md`, and your notes. Nothing else: not DECISIONS, not an ADR, not the research files, not the
reports themselves. You never commit: the main session does.

## The hard limit
This change-set stands close to Mustafa's stop line (1,200 written lines; his words of 2026-11-17, 21:33: "The new
stop line: 1,200 written lines; copied lines do not count."). **Your pass may add at most 30 lines in all four files
together** (count with `git diff --numstat`). Change existing lines where a line has become untrue; add a line only
where nothing existing can carry the fact. If 30 lines cannot hold what is true, stop and say what is left out.

## What happened (the facts, each with its record)
- Pull request: #56, a draft, branch `claude/wonderful-hamilton-jdmlnf` (it stands in for `cs/cs0-model-rules`), base
  main c94c81b. Risk tier 1 (decision docs): Mustafa's go; no Brave check. Its commits so far: 3747bb3 and 4a1b0d7
  (his records), 212f55b (the plan's carry), 5f356ef, 65c8fe8 and the commit that adds the two research files (the
  rules and the joint list). `git log --oneline origin/main..HEAD -- docs` shows them.
- His decision of 2026-11-14 is now in `docs/DECISIONS.md` (the entry "Taken by Mustafa, 2026-11-14, 13:58:05 UTC: the
  game is built now ..."), with his addition of that day, his go of 2026-11-17 on the build plan (19:10:44 UTC: his
  answers to the plan's twelve decisions and four changes) and his two later messages (20:03:57 and 20:39:23 UTC).
  Read those five entries before you write a line about them, and quote him only from them.
- The plan: `docs/reports/2026-11-17-build-plan-slice-1.md` (the build plan for foundation steps 2.4 and 2.5 and
  Slice 1, fourteen change-sets; "Since the plan" on top), with its appendix
  `docs/reports/2026-11-17-build-plan-slice-1-appendix.md` (A1 to A11).
- What STATUS still says that his words changed (find the lines; `docs/STATUS.md` "Phase", "Next", "Waiting",
  "Routines" as they apply): step 2.3's plan waiting for his go as the next thing; "one heavy spike at a time"; who
  records his decision of 2026-11-14 (his CHANGE 1: change-set 0, not step 2.3's docs change-set); the 2.3 chat is
  stopped ("The 2.3 chat stays stopped for now: I pass it nothing until its plan has been checked."); a second
  builder chat exists since 2026-11-15 (branch `claude/optimistic-albattani-mdoonz`, main model Opus 5.5, Fable 5.2
  on its advisor tool; it runs the preview hosting check in a separate test repository); "No game code is built
  before change-set 0 is on main."
- Part 15 of the plan, "The second lane", is proposed and not on main: it was sent to Mustafa as a file on 2026-11-17
  with six questions (LANE-SPLIT, RECORD-FILES, VFX-ROUTE, VFX-FIX, VFX-MOMENTS, GUN-FILES) and goes to main once he
  has answered. Change-set 1 does not start before that. The main session gives you the commit that holds its text.
- The model rules: `docs/research/asset-export-rules.md` (graphics-engineer) and `docs/research/soldier-joint-list.md`
  (engine-architect): rules and checks on paper, no export has gone through them. OQ-0005-AS3 and AS4 are answered in
  part; ten new pass marks, PM-0005-47 to -56, first run at the plan's change-set M.
- His answers recorded where the drafts ask: OQ-0002-6 (the jump's reach), OQ-0006-18 (a committed model may be
  public), OQ-0003-1 further in part (the machine classes), the three "do soldiers block" rows, PM-0004-34's row. The
  register's count of open questions went from 98 to 96.
- Open after this change-set, for STATUS's follow-ups if it keeps such a list (one line each, at most five): the draw
  cap with a gun from its own file (the rules file's open point O1; PM-0005-54 fails with today's rifle: a question
  for Mustafa with change-set M's plan); the fifth material class's name (O2); the proof of the rules by a first real
  Blender export (change-set M); whether the coat keeps the legacy motion (his call at change-set 10's pictures);
  the machine lines in ADR 0001, ADR 0003 and SPIKE_PLAN and ADR 0002's "one change-set per step" are carried later
  (the plan's report, "Since the plan", names each carrier).
- This change-set's report will be `docs/reports/2026-11-17-cs0-model-rules.md` (the main session writes it; list it
  in the map and in the reports index).

## What to do
1. `docs/STATUS.md`: make every line true that the facts above change, in the fewest lines; add the pull request's row
   where STATUS lists them.
2. `docs/ROADMAP.md`: one terse entry for change-set 0, in the file's own form.
3. `docs/REPO_MAP.md`: the new files (two research files, three reports); run
   `node .claude/scripts/repo-map-check.mjs` and show its last line.
4. `docs/reports/README.md`: the three reports, in the index's own form.
5. Do not restate his words loosely: quote from DECISIONS, or point to the entry. Do not call anything decided that
   is proposed (Part 15; the Art lead's texts; the rules' open points).
6. Check: `git diff --numstat -- docs/STATUS.md docs/ROADMAP.md docs/REPO_MAP.md docs/reports/README.md` (added lines
   at most 30); no line over 120 columns outside tables; open nothing under /root/.claude/uploads/.

## Your answer (at most 15 lines)
Per file: what changed, in one line; the numstat; the repo-map check's last line; anything you could not make true
within the limit.

## Saving your progress
A usage limit can cut you off part way, and you would then be started again with this same task. Keep notes in
/home/user/Shamble-Frontlines/.checkpoints/cs0-model-rules/docs-keeper-cs0.md (you may always write there). Look there
first: if the first line names another task, move the file into
/home/user/Shamble-Frontlines/.checkpoints/cs0-model-rules/old/ and start fresh; if the notes end with DONE, write
your answer from them; otherwise go on after the last block they mark done. Trust the blocks they mark done: never
reread, recheck or redo them (a later block may still read a file it needs). Start a new file with one line naming
your task, then your plan: the blocks you will do, in order (a block: a source read, a file reviewed, a test group
run, a section written). Write each finished block to its file at once. Add a notes line for each finding as you find
it (what, where), and one after each finished block (done, what comes next). Redo a block the notes don't mark done,
written over, never appended. Scratch files go in /home/user/Shamble-Frontlines/.checkpoints/cs0-model-rules/: text
and small scripts only; builds and binaries go under .gate/cs0-model-rules/. This folder is pushed to a public
repository: never put a secret, token, environment value, request header, cookie or raw payload in it, whether in a
note, a script or a script's output; the saver's secret check only knows a few token formats and cannot read archives
or binaries. End with DONE, then give your answer from your notes alone, opening no other file. Never commit or push.
