# Checkpoint: lane2-needs
- **Goal:** lane 2's first list of needs from the block (build plan Part 15.2, "What each lane needs from the other"):
  every field and every event that lane 2's units 6, M, 10 and 11 read from the simulation; for each, the unit that
  reads it, every tick or on an event, and the source that asks for it (the build plan's card, the old animation code
  that change-set 10 ports, an Art brief); wishes marked as wishes. Posted as ONE comment on the "Lane requests" issue
  #59, naming lane 2; its link to Mustafa in chat. Due before change-set 4's plan is reviewed (lane 1 builds
  change-set 2 now). Keep it light: one agent reads the sources, one advisor-tool consultation checks the list, no
  review round. (Mustafa's words of 2026-11-18, 23:37 Amsterdam, item 3; byte for byte in
  `.checkpoints/preview-hosting-check/mustafa-2026-11-18-lane2.md`, 3318 bytes, sha256 765226cd4657...)
- **Branch:** claude/optimistic-albattani-mdoonz (shared with the preview hosting check, which keeps the first place:
  whenever a step of sitting 3 or of Day 2 can move, it moves first) · **Base:** main b763ba3e, merged in at aa02297
- **Last update:** 2026-11-19 01:05 Amsterdam (00:05 UTC)

## Steps done
1. His item 1: main b763ba3e merged into the branch (plain merge, no conflict: no file changed on both sides since
   4460ef5b), merge commit aa02297; pushed through the checkpoint script with this file.

2. His item 2 done (2026-11-18, before 23:00 UTC; transcript): Part 15 read whole (348 lines; his six answers at its top: LANE-SPLIT,
   RECORD-FILES, VFX-ROUTE, VFX-FIX, VFX-MOMENTS, GUN-FILES, each (a) with additions); DECISIONS.md RECORD-FILES (each
   pull request writes only its own lines, in its docs pass, after taking main in; never rewords, re-wraps or moves
   another lane's lines) and OLD-LINES (lines about merged pull requests may be moved by script, byte for byte, by
   either lane's docs pass, each listed in its report); REC-OLD in b763ba3e's message (a docs pass may correct a line
   that describes code when that code changed, old and new text in its report; a line recording what happened or
   what he decided is never reworded: it gets a dated pointer). Lane 2's units by 15.2: the hosting check, step 2.6,
   the look carry, the comparisons, 6 Look, the VFX proof, M, 10 Body, 11 Feel.

## Current step
(none: the task is done; see step 3 below, kept here until he has the link)
3. The needs list: one agent (engine-architect, model opus, its file's effort xhigh) launched 2026-11-18T23:00:26.326Z (transcript), notes
   `.checkpoints/lane2-needs/engine-architect-needs.md`, draft `.checkpoints/lane2-needs/needs-list-draft.md`; the
   saver `watch lane2-needs` started (background task bvaxw5o8t; its first line 23:00:26 UTC). Then one advisor-tool consultation checks the list;
   then one comment on issue #59 (naming lane 2, ending with the Claude Code footer), its link to him.

   - The agent's hand-back (2026-11-18 after 23:15 UTC; notes ending DONE): the draft is ready to post (about 15.6 KB):
     15 fields read every tick (in the block today 1; named by a card 3; implied by a card 4; named only by a draft 3;
     new 4: velocity, on the ground, team or faction, which soldier is the local player), 6 events (named by a card 4,
     by a draft 1, new 1: where a shot ended, with the surface), 5 wishes, 10 gaps or questions. Read by the session
     in full; its Part 15 citations (lines 27, 109, 113-114, 117-121, 124-126, 130-131, 214) checked against the file.
     Next: one advisor-tool consultation, then the post.
   - The advisor-tool consultation (his item 3's one; called 2026-11-18 23:19:10 UTC, transcript time): issue #59
     read first (open, "Lane requests", no comment yet, no form: a request is a comment naming its lane); the lines the
     Status column rests on printed (layout.ts: the header's `tick` and `layout_hash` only; the plan's cards 4, 8, 9,
     10, 11 at lines 336-347 and 400-432; 0001:303-318; 0002:291-294; 0004:545-546; CS1:265-268): all as cited; every
     noun of cards 10 and 11 mapped to a row; added: F14 (projectiles in flight, only if lane 2 draws them) and E1's
     ask for the bow; gaps 4 and 6 labelled "For Mustafa, not a request to lane 1"; a top line asking lane 1 which rows
     change-set 4 carries and which come with 8 and 9; "first list"; E1's own-shot timing made an ask; full paths in
     the legend; totals recounted (16 fields, one conditional; 6 events; 5 wishes; 10 gaps). A grep of the legacy game
     found no grenade bounce or bow-release sound (noted in the comment). The posted text:
     `.checkpoints/lane2-needs/needs-list-comment.md` (sha256 b4bae8e35cd473c9...).
   - **Posted: https://github.com/Mergincanli/Shamble-Frontlines/issues/59#issuecomment-6994614989**, created
     2026-11-18T23:24:14Z by Mergincanli (author_association OWNER), read back: equal to the file, plus a second footer
     line the tool appended. The saver bvaxw5o8t stopped with TaskStop after the post (its output ends "[killed]"; its last save 9c4dbd1 at 23:10:29 UTC).
   - Before the hosting check's closing pull request is marked ready, this checkpoint and `.checkpoints/lane2-needs/`
     must be gone too (CHECKPOINTS.md section 9), with this task's record folded into that report or the chat.

## Next steps
4. His items 4-6 stand as rules: the hosting check to its report and closing pull request, then step 2.6's plan; no
   game code before change-set 5 is merged (then render/, soldier/ and camera/ pass to lane 2); no GLB made by a builder
   chat; with no pull request open here, take main in at the start of each unit and before opening one.

## Open findings
- none

## Key numbers
- main b763ba3e5f34c67fc4f875cfbd9266167fb095f3 (the merge of #60); merge base was 4460ef5b; merge commit aa02297.

## Decisions taken
- His lane-2 words of 2026-11-18, 23:37 (the file above): items 1-6.
- **His answers of 2026-11-19, 00:59** (byte for byte in `.checkpoints/preview-hosting-check/mustafa-2026-11-19-answers.md`,
  2196 bytes, sha256 7a0965c1fe8b...): PROJ-DRAW (a): lane 1 draws grenades and arrows in flight plainly in
  change-set 9; they pass to lane 2 with the plain tracers and flash at change-set 9's merge, and from then on lane 2
  reads the block's projectile section. AIM-STATE (a) as far as the block goes: lane 2 asks for no aiming flag and
  plans Slice 1's poses without one; the rule for the poses is not "fired recently" and is not settled now; "Aiming and
  firing" in the Art lead's sheet is the pose of a gun that is ready, not aiming down sights; his Shamble Testlines
  sandbox (github.com/Mergincanli/Shamble-Testlines: look at it, copy nothing) shows the aiming pose whenever the
  soldier is not sprinting and the sprint pose while he sprints: change-set 10's plan starts from that and brings him
  the idle pose as a proposal with pictures; "A first shot never leaves a gun that points away from its target."
  Whether aiming down sights and the sprint-to-fire delay become game rules in Slice 1 is a separate decision of his,
  for lane 1 ("You hear it from me"). "The list on issue #59 stands as posted. The Advisor read it against Part 15.2
  and checked a sample of its citations." For DECISIONS.md in lane 2's next docs pass (RECORD-FILES: its own lines);
  not posted on #59 (a comment there is never his word; lane 1 hears his decisions from him).

## Running work
- agent engine-architect (needs list), launched 2026-11-18T23:00:26.326Z (transcript): DONE (hand-back after 23:15 UTC).
- saver bvaxw5o8t (`watch lane2-needs`), first line 2026-11-18 23:00:26 UTC; stopped by TaskStop after the post.
