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
- **Last update:** 2026-11-19 00:01 Amsterdam (2026-11-18 23:01 UTC)

## Steps done
1. His item 1: main b763ba3e merged into the branch (plain merge, no conflict: no file changed on both sides since
   4460ef5b), merge commit aa02297; pushed through the checkpoint script with this file.

2. His item 2 done (2026-11-18 about 23:05 UTC): Part 15 read whole (348 lines; his six answers at its top: LANE-SPLIT,
   RECORD-FILES, VFX-ROUTE, VFX-FIX, VFX-MOMENTS, GUN-FILES, each (a) with additions); DECISIONS.md RECORD-FILES (each
   pull request writes only its own lines, in its docs pass, after taking main in; never rewords, re-wraps or moves
   another lane's lines) and OLD-LINES (lines about merged pull requests may be moved by script, byte for byte, by
   either lane's docs pass, each listed in its report); REC-OLD in b763ba3e's message (a docs pass may correct a line
   that describes code when that code changed, old and new text in its report; a line recording what happened or
   what he decided is never reworded: it gets a dated pointer). Lane 2's units by 15.2: the hosting check, step 2.6,
   the look carry, the comparisons, 6 Look, the VFX proof, M, 10 Body, 11 Feel.

## Current step
3. The needs list: one agent (engine-architect, model opus, its file's effort xhigh) launched about 23:07 UTC, notes
   `.checkpoints/lane2-needs/engine-architect-needs.md`, draft `.checkpoints/lane2-needs/needs-list-draft.md`; the
   saver `watch lane2-needs` started (background task bvaxw5o8t). Then one advisor-tool consultation checks the list;
   then one comment on issue #59 (naming lane 2, ending with the Claude Code footer), its link to him.

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

## Running work
- agent engine-architect (needs list), launched 2026-11-18 about 23:07 UTC; notes and draft in `.checkpoints/lane2-needs/`.
- saver bvaxw5o8t (`watch lane2-needs`), started about 23:07 UTC.
