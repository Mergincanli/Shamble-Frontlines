# Checkpoint: build-plan-slice-1
- **Goal:** the plan for foundation steps 2.4 and 2.5 and Slice 1 (move and shoot), by Mustafa's task of 2026-11-14
  (`.checkpoints/build-plan-slice-1/mustafa-2026-11-14-task.md`, items 1 to 10) and his addition of the same day
  (`mustafa-2026-11-14-addition.md`: Blender is the route for the models; items 11 to 13). The plan only: no build, no
  edit outside this checkpoint, no docs pull request. · plan: `.checkpoints/build-plan-slice-1/plan.md` (not written
  yet)
- **Branch:** claude/wonderful-hamilton-jdmlnf (stands in for cs/build-plan-slice-1) · **Base:** main
  4460ef5b8d6dc8566b29c15ded75c6f0c8be386b
- **Last update:** 2026-11-14 15:39 Amsterdam

## Steps done
1. Orientation · the session's model read with get_session at 2026-11-14 13:58 UTC: `session_context.model` and
   `configured_model` both `claude-fable-5-2` (the record holds no `last_served_model` field); effort xhigh; the branch
   is at main 4460ef5; main, claude/sweet-mayer-grsukw and claude/netcode-proof-test-plan-hcky5g fetched (read only).
2. Reading · the main session's own reads are in `reading.md` (B1 to B10 done). His two messages copied by script from
   the transcript into the notes folder (`extract2.mjs`; the task: user message of 13:58:05 UTC, 7538 bytes; the
   addition: queued at 14:12:13 UTC, 2340 bytes).
3. Pass-mark digests by agents (each Opus 5.5 by its agent file, effort xhigh passed by the caller): `marks-0002.md`
   (engine-architect, 82 ids), `marks-0004.md` (rust-reviewer, 75 ids), `marks-0003.md` (graphics-engineer, 47 ids),
   `marks-0001.md` (netcode-architect: the local path, what the proof test may change, the slice 1 and 60 Hz marks).
   All four ended DONE and handed back between 14:07 and 14:14 UTC (the notes files' times).

4. `marks-0005.md` (test-engineer, 46 ids; DONE, handed back about 14:36 UTC) read with the other four digests.
5. Plan revision 1 written at `.checkpoints/build-plan-slice-1/plan.md`: Parts 0 to 10, 12, 13, the decisions and
   "For Mustafa"; Part 3's tables counted by `count-marks.mjs` (201 marks expected, 201 placed, none missing, none
   twice; the asset spike's 7 in Part 12).

## Current step
6. Part 11 (the model-file rules) waits for graphics-engineer's `asset-rules.md` (effort max; reading official
   documentation; still running at 14:38 UTC). Then the advisor agent's plan review.

## Next steps
1. Read `asset-rules.md` when its agent ends; write Part 11 from it.
2. The advisor agent's plan review (Opus 5.5, effort max; no review rounds, no verifier, by his words); fold what it
   finds; record its verdict in Part 14.
3. Stop for his go, with the summary he asked for.

## Open findings
- The PR Advisor's merge places are `docs/`, `tools/`, `README.md` and `FIRST_PROMPTS.md` only
  (`.claude/hooks/routine-guard.mjs:84`): every change-set that touches `crates/` or `packages/` merges on his go from
  a day session.
- The 2.3 plan (revision 2.4, on its branch) was written before his decision of 2026-11-14: its later parts wait for
  spike reports that are no longer built as separate tests (2.3-B "after the boundary spike", 2.3-S "after the sim
  core spike", 2.3-E). The plan must say how those gate checks follow steps 2.4 and 2.5 instead; his words of
  2026-11-12: "The guard, the gate and CI work stays with Opus 5.5."
- CI today runs no Wasm and no browser for v2 (`toolchain.yml`): marks that say "in CI" for a Wasm or browser run are
  shown locally until a CI job exists.
- The reference machines (minimum laptop, recommended desktop, Mac) are not named; nothing now triggers the naming
  (it was tied to the look proof test).

## Key numbers
- Pass marks of the six tests named in his decision, by the digests' counts: ADR 0002 82 rows (with PM-0006-28 to
  -31), ADR 0004 75, ADR 0003 47; ADR 0005 46 (running).

## Decisions taken
- His decision of 2026-11-14 (word for word in `mustafa-2026-11-14-task.md`; recorded in docs/DECISIONS.md by step
  2.3's first docs change-set, built by the 2.3 chat): steps 2.4, 2.5 and Slice 1 may be planned now and built after
  his go on the plan. Its Blender sentences are replaced by his addition of 15:03 (`mustafa-2026-11-14-addition.md`).
- His rules for this chat (same block): main model Fable 5.2, builder agents Fable 5.2, every other agent Opus 5.5,
  reviewers, verifiers and designers of game and engine code at effort max; the advisor tool off; never push to the
  two other chats' branches, never touch spikes/; no kit, gate or CI edit without asking him with its exact text.

## Running work
- saver: background task b33kcams4 · output
  `/tmp/claude-0/-home-user-Shamble-Frontlines/f707c56b-26b4-5848-b1e1-237a99a6a881/tasks/b33kcams4.output` · started
  13:59 UTC (14:59 Amsterdam) · stops by 15:49 UTC · last check 14:09 UTC: ok ("saved and pushed 76cc17a").
- agents: test-engineer digest-0005 and graphics-engineer asset-rules, launched after 14:14 UTC; notes in
  `.checkpoints/build-plan-slice-1/`.
