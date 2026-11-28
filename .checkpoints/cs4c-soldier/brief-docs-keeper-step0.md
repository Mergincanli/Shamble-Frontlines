# docs-keeper's brief for 4c's step 0, part 2 (ready to send; the main session's notes)

You are docs-keeper. Change-set 4c, step 0 (the plan sheet `.checkpoints/cs4c-soldier/reread-sheet.md`, section C), in
the worktree `/home/user/Shamble-Frontlines/.gate/cs4b-2` (branch `cs/4c-soldier`). Mustafa's design notes of
2026-11-27 (saved by script: `.checkpoints/cs4-walk/mustafa-2026-11-27-design-notes.md`) say, item 2: "docs/LOOP_V2.md
gets a new section 14 after section 13, with exactly the text under item 8. docs-keeper places that text and writes no
design of its own." Item 3: the dated pointer at each line of LOOP_V2's sections 1, 3, 9 and 11 on the harvest fields;
"Keep the old text for the history. The header's "Updated" line gets one sentence that names §14. Change nothing else
in the design docs and rewrite no ADR. The register (docs/adr/README.md) gets one dated line that names §14." His
answers of 2026-11-28 (`.checkpoints/cs4-walk/mustafa-2026-11-28-answers.md`): PASTE-BREAKS (a) (the paste's broken
list lines mended, no word changed), DESIGN-CLASH (a) at both lines, with his own pointer at LOOP_V2's "uplinks replace
silos" line. Part 1 (his words into `docs/DECISIONS.md`) is committed (af789ef3).

## Do, in this order, from the worktree's root
1. `node .checkpoints/cs4c-soldier/place-4c.mjs`: it places section 14 (`section14-mended.md`) and every pointer by
   script (his words are never typed), and refuses unless both design docs still equal main's.
2. Write the header's one sentence yourself, at the end of `docs/LOOP_V2.md` line 5 ("Updated on 2026-11-04 with
   Mustafa's design walk: the lines dated 2026-11-04, and §13."): exactly one sentence that names §14 and the date
   2026-11-27, no design of your own (for example that §14 holds his design notes of 2026-11-27 on resource points and
   repair bots, with the dated pointers at the lines they replace).
3. Write the register's one dated line in `docs/adr/README.md` that names LOOP_V2's §14, in the register's own form
   (its dated pointer lines near lines 55 to 62 are the nearest kind); no ADR text rewritten.
4. Run the checks and keep their output: `node .checkpoints/cs4c-soldier/section14-4c.mjs check`,
   `node .checkpoints/cs4c-soldier/loop-check-4c.mjs`, `node .checkpoints/cs4c-soldier/decisions-4c.mjs check`;
   `git diff --stat` must show only `docs/LOOP_V2.md`, `docs/EDITOR_V2.md` and `docs/adr/README.md` changed (and
   nothing under `.checkpoints/` but your notes). Every check must pass; if one fails, stop and say why.
5. Nothing else: no ROADMAP, STATUS or REPO_MAP edit in this run (the docs pass comes later, after the reviews).

## Rules
Write only those three docs and your notes file `.checkpoints/cs4c-soldier/docs-keeper-step0.md` (your plan, what you
did, each check's output, DONE). Never commit. Shell: git and gh always start their own command, with no other program
in front of them, one git per shell call; spell out every file a command writes with its full path, or use the Write
and Edit tools. Return at most 8 lines: the header sentence and the register line as written, and each check's result.
