Task: security-auditor-review2 (round 2, the last planned round, cs1 "Types", foundation step 2.4): only what changed in my lane since round 1 (82142fa -> 65a7b13: 1a2ff13 fix pass + 65a7b13 follow-up), and whether my round-1 Lows S1-S3 are closed. Branch claude/wonderful-hamilton-jdmlnf, HEAD 554ebe1 (checkpoint commits; code = 65a7b13's), base main 22c6afa. Started 2026-11-18. I edit nothing but this file (and my agent memory).

Rules: cargo --locked (and --offline), no cargo update, no npm scripts/npx, nothing installed; no SHAMBLE_WRITE_TYPES in the main folder; trials only in `.gate/cs1-types/sa-plant` (git worktree add --detach, HEAD) with CARGO_TARGET_DIR=.gate/cs1-types/sa-target (my own, never main's target); put every file back, delete planted files/links one by one, `git worktree remove` without --force; never open spikes/ or uploads.

## Plan (blocks, in order)
- B0 Read: diff 82142fa..65a7b13 -- crates packages/types/src/layout-check.ts; fix-pass-1.md (done); plant.sh + plant-results.txt runs 2f, 6a-6e.
- B1 S1: read both write_generated + repository_root; trial in sa-plant: generated folder as link, parent as link, link inside, dangling link named like an output; plus canonicalize/TOCTOU and create_dir edge cases. Judge "refuse write mode when CI is set" not taken.
- B2 S2: read the `*/` line check; trial with planted docs: a doc line exactly `*/`, a block doc comment, #[doc = "..."], type-level doc, a field doc ending in `*/`.
- B3 S3: read layout-check.ts doc.
- B4 repository_root guard (point 4): env var effects, log/path leaks.
- B5 Other changed lines in my lane: scratch loop (AlreadyExists only, absolute base), entries_of, "no file of its own in generated/".
- B6 Verdict, findings ranked, memory update, DONE.

## Notes
