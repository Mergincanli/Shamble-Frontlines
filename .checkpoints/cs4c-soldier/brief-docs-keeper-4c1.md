# docs-keeper's brief for 4c-1's docs pass (ready to send after the verifier; the main session's notes)

You are docs-keeper. Change-set 4c's first pull request, **4c-1: step 0 and step S, the page's scene**, in the worktree
`/home/user/Shamble-Frontlines/.gate/cs4b-2` (branch `cs/4c-soldier`). Mustafa's SIZE-4C (b) (his answers of
2026-11-28, `.checkpoints/cs4-walk/mustafa-2026-11-28-answers.md`; in the register since step 0): the scene alone first,
then the soldier (steps 1 to 7) as 4c-2 on top of it. The plan sheet: `.checkpoints/cs4c-soldier/reread-sheet.md`
(sections C, D, G and I); the card: `.checkpoints/cs4c-soldier.md`; the review round: `.checkpoints/cs4c-soldier/review-s-round.md`.

## Do
1. **His pointer at `docs/LOOP_V2.md:16`** (security-auditor's note on the review: it sits mid-sentence): run
   `node .checkpoints/cs4c-soldier/place-4c.mjs fix-16` from the worktree's root (it moves his pointer, by script, from
   the line's end to right after "centre.", where section 1's sentence on the centre field ends), then
   `node .checkpoints/cs4c-soldier/loop-check-4c.mjs` and `node .checkpoints/cs4c-soldier/section14-4c.mjs check`;
   both must pass. Write no word of your own there.
2. **ROADMAP:** a terse entry for 4c-1 (the scene and step 0), in the file's own form.
3. **STATUS:** where 4c stands (4c-1 ready for his Brave check on Sunday; 4c-2 next on the same branch); the follow-ups
   this pull request leaves, each with its owner and the change-set it waits for (from `review-s-round.md`, "The fix
   pass": GE-F2 the depth range on a large map; GE-F3 with P-2 the draw calls on the largest map, merge the floor by
   material; GE-F4 the WebGPU pipelines built when the map arrives; AD-4 the readout over a short window, the overview
   leaving at step 4); the follow-ups of 4b-2 that 4c-1 meets (4B-TYPES-READER, 4B-MEMBER-KINDS, 4B-BOOT-CALLS:
   engine-architect's review says met); and **his two open marks of 2026-11-28, in his words** (his item 12 b and c:
   PM-0004-60 "stays open in STATUS until the nightly job runs its 10 million, named with the change-set that brings
   it"; PM-0004-20's 48-soldier run "stays open in STATUS, named with the first build that can show it"), copied from
   his saved message, never retyped (quote them by script or not at all, and say which).
4. **REPO_MAP:** the new and moved files (`packages/client/src/render/scene.ts`, `inventory.ts`; the removed
   `empty-scene.ts`; `packages/sim-worker/src/local-map.ts`; the new tests), and any other line `node
   .claude/scripts/repo-map-check.mjs` asks for.
5. Drift fixes only where a doc's claim is now wrong (START_HERE, MODULE_MAP, TECHNICAL), each with its file:line.
6. Run `node .claude/scripts/repo-map-check.mjs` and the two checks of item 1; keep their output in your notes.

## Rules
Write only docs under `docs/` and your notes file `.checkpoints/cs4c-soldier/docs-keeper-4c1.md` (your plan, what you
changed with file:line, each check's output, DONE). No ADR text rewritten; no word of Mustafa's typed. Never commit.
Shell: git and gh always start their own command, with no other program in front of them, one git per shell call;
spell out every file a command writes with its full path, or use the Write and Edit tools. Return at most 10 lines.
