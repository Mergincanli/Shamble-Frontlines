# Checkpoint: build-plan-slice-1
- **Goal:** the plan for foundation steps 2.4 and 2.5 and Slice 1 (move and shoot), by Mustafa's task of 2026-11-14
  (`.checkpoints/build-plan-slice-1/mustafa-2026-11-14-task.md`, items 1 to 10) and his addition of the same day
  (`mustafa-2026-11-14-addition.md`: Blender is the route for the models; items 11 to 13). The plan only: no build, no
  edit outside this checkpoint, no docs pull request. · plan: `.checkpoints/build-plan-slice-1/plan.md` (revision 2)
- **Branch:** claude/wonderful-hamilton-jdmlnf (stands in for cs/build-plan-slice-1) · **Base:** main
  4460ef5b8d6dc8566b29c15ded75c6f0c8be386b
- **Last update:** 2026-11-14 16:30 Amsterdam

## Steps done
1. Orientation · the session's model read with get_session at 2026-11-14 13:58 UTC: `session_context.model` and
   `configured_model` both `claude-fable-5-2` (the record holds no `last_served_model` field); effort xhigh; the branch
   is at main 4460ef5; main, claude/sweet-mayer-grsukw and claude/netcode-proof-test-plan-hcky5g fetched (read only).
2. Reading · the main session's own reads are in `reading.md` (B1 to B10 done). His two messages copied by script from
   the transcript into the notes folder (`extract2.mjs`; the task: user message of 13:58:05 UTC, 7538 bytes; the
   addition: queued at 14:12:13 UTC, 2340 bytes).
3. Pass-mark digests by agents (each Opus 5.5 by its agent file, effort xhigh passed by the caller): `marks-0002.md`
   (engine-architect, 82 ids), `marks-0004.md` (rust-reviewer, 75 ids), `marks-0003.md` (graphics-engineer, 47 ids),
   `marks-0001.md` (netcode-architect), `marks-0005.md` (test-engineer, 46 ids). All ended DONE (the files' times:
   14:07 to 14:36 UTC).
4. The model-file rules, a first reading: graphics-engineer, effort max, from the drafts, the glTF specification, the
   Blender exporter's documentation and source and the three.js loader's source: `asset-rules.md` (its time 14:44 UTC)
   with its notes `graphics-engineer-asset-rules.md`.
5. Plan revision 1 · saved by the background saver (aa5dbc4 and later) · Parts 0 to 14, the decisions, "For Mustafa";
   Part 3's tables counted by `count-marks.mjs` (201 marks expected, 201 placed, none missing, none twice; the asset
   spike's 7 in Part 12).
6. The advisor agent's plan review of revision 1 (Opus 5.5 by its agent file, effort max passed; started after 14:43
   UTC; its notes `advisor-plan-r1.md` last written 15:08 UTC, DONE): **REVISE**, 3 High, 11 Medium, Lows.
7. Mustafa sent three Art sheets into the chat during the fix pass ("The Art director chat told me to send this to
   you"): typed out in `art-sheets-2026-11-14.md`.
8. Plan revision 2 · ee41298 · every finding of the review folded in (Part 14's table says where), the Art sheets
   taken in (Part 4, FIRST-SOLDIER, Part 11 rule 12); the count unchanged (201 of 201; 7).

9. The advisor agent's re-read of revision 2 at ee41298 (asked after 15:18 UTC; its notes `advisor-plan-r2.md` last
   written 15:24 UTC, DONE): **APPROVE**, with eleven Lows to fold without a further review; all eleven folded (Part
   14, "The re-read of revision 2"); the count unchanged (201 of 201; 7).

## Current step
10. Done: the plan is approved by the advisor agent and saved; stopped for his go.

## Next steps
1. Stopped for his go on the plan and his answers to its twelve decisions (SPLIT-25, FIRST-SOLDIER, GATE-FOLLOW,
   MACHINES, BRAVE-ROUTE, PREVIEW-START, STAND-IN-LOOK, COMPARE, MODEL-PUBLIC, RULES-WHO, SCOPE-RELOAD, START-RULES).
2. On his go, and once his decision of 2026-11-14 is on main (the 2.3 chat's docs change-set): change-set 0's own
   plan (the model-file rules), then the advisor agent's review of it. Its pull request also carries this plan to
   main as a report, with his messages in its appendix.

## Open findings
- The advisor agent's review of revision 1: REVISE; all folded into revision 2; its re-read: APPROVE (closed).
- The Art sheets' text and numbers are in `art-sheets-2026-11-14.md` on this public branch since ee41298 (15:18
  UTC), typed before the question whether unreleased art may be seen (OQ-0006-18) was weighed; told to him in the
  closing message and in decision MODEL-PUBLIC. The images are not in the repository.
- The PR Advisor's merge places are `docs/`, `tools/`, `README.md` and `FIRST_PROMPTS.md` only
  (`.claude/hooks/routine-guard.mjs:84`): every change-set that touches `crates/` or `packages/` merges on his go from
  a day session.
- The 2.3 plan (revision 2.4, on its branch) was written before his decision of 2026-11-14: its later parts need his
  word to follow this chat's change-sets (decision GATE-FOLLOW; the network step as its own sub-question).
- The route that brings a built zip to his laptop is not proven (Part 8; decision BRAVE-ROUTE).
- The reference machines are not named (decision MACHINES, from his standing words on timing).
- The Art sheet's line rule (units thicker than buildings) differs from `docs/LOOK_V2.md` §2 on main; the carry of his
  look decisions settles it; change-set 6 waits for it.

## Key numbers
- Pass marks of the six tests named in his decision: 201 (ADR 0002: 36; ADR 0006: 4; ADR 0004: 74; ADR 0003: 47; ADR
  0005: 37; ADR 0001: 3), all placed; the asset spike's 7 placed in Part 12.
- The cut: 14 units (change-sets 0 to 12 and M); planned 28,200 hand-written lines; 188 to 199 hours of session work.
- Decisions asked of him: 12.

## Decisions taken
- His decision of 2026-11-14 (word for word in `mustafa-2026-11-14-task.md`; recorded in docs/DECISIONS.md by step
  2.3's first docs change-set, built by the 2.3 chat): steps 2.4, 2.5 and Slice 1 may be planned now and built after
  his go on the plan. Its Blender sentences are replaced by his addition of 15:03 (`mustafa-2026-11-14-addition.md`).
- His rules for this chat (same block): main model Fable 5.2, builder agents Fable 5.2, every other agent Opus 5.5,
  reviewers, verifiers and designers of game and engine code at effort max; the advisor tool off; never push to the
  two other chats' branches, never touch spikes/; no kit, gate or CI edit without asking him with its exact text.
- The advisor agent's ruling (its review of revision 1): the scan test in the sim crate may stand in for the 2.3
  chat's float ban until that lint merges, on three conditions (plan, Part 7).

## Running work
- saver: stopped with TaskStop after the save of 15:26 UTC (background task b33kcams4; started 13:59 UTC; its log:
  saves every 10 minutes from 14:09 UTC, each "saved and pushed"). Start it again with the same command when agents run.
- agents: none running.
- The plan was sent to Mustafa as a file through the session's file tool after the save of 15:26 UTC (it also tries
  the route of the plan's Part 8).
