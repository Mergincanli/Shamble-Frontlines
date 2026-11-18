# test-engineer, round 2 review of cs1-types (code 65a7b13, pushed head 6c3a24a; round 1 reviewed 82142fa; base main 22c6afa)

## Plan
1. Read: `git diff 82142fa..65a7b13 -- . ':!.checkpoints'`; fix-pass-1.md; plant.sh + plant-results.txt (+ run2);
   runs20.sh + runs20-results.txt; .checkpoints/cs1-types.md step 9; CI runs of the branch (gate, toolchain).
2. High 1: (a) the 20-runs record vs file mtimes in the main folder; no folder re-made under .gate/cs1-types/plant/;
   (b) the guard `repository_root` (layout_ts.rs, ts_export.rs): reproduce the cross-tree case with two trees sharing
   one scratch target of my own (te-target), never the main `target`; (c) false-fail cases: cargo test from a subfolder,
   --manifest-path, nextest-like direct binary runs, CI's env -i cut.
3. Mediums 2, 3, 4 and Lows 5-8: closed or not, one line each; Medium 4's not-taken clause: accept or not.
4. My round-1 defects T1, T2, T3, T11, T13 again in te-plant (own target te-target); three new ones aimed at the new
   code (repository_root comparing nothing; write mode's link check on the parent only; doc-comment check with `*/`
   mid-line; entries_of treating a link as a plain file; the generated/ own-files check removed).
5. New planted runs (2e, 2f, 5a, 5b, 6a-6e, 7): does each prove its label, with the binary and folder it claims.
6. Anything the fix pass weakened in previously-right tests.
7. Word per mark: PM-0002-15, -16, -17, -19, -63.
Rules: cargo --locked; no npm scripts/npx; worktree te-plant + CARGO_TARGET_DIR=.gate/cs1-types/te-target; restore
every file; delete planted files one by one; `git worktree remove` without --force; never SHAMBLE_WRITE_TYPES in main.

## Findings (one line each, as found)
- read: HEAD moved during my review (d34cfc5 docs/DECISIONS.md only; 5bf313d etc. checkpoints); code unchanged since 65a7b13 (`git diff --stat 6c3a24a..d34cfc5`: docs/DECISIONS.md alone).
- High 1 record: runs20-results.txt start 13:40:09Z; main's 9 generated files mtime 13:40:18.904 (layout.ts) / 13:40:18.917 (8 protocol), after start-20 (13:40:18.792) and before run-20.txt (13:40:19.236): run 20 wrote the main folder's files. runs 1-19 were checked at the time by `find ! -newer stamp` (sound: GNU find compares ns). `.gate/cs1-types/` holds no `plant/` folder (only plant-logs, plant-target, runs20, tsconfigs); `git worktree list` = main folder alone. Record sound.
- known answers re-derived by a THIRD implementation (my python, scratchpad fnv_check.py): BASE 9adbecc7, empty 811c9dc5, TAG_PAIR 4265359c / bd51cd2f, HEADER 2df78c30 = layout.ts; TAG_PAIR streams without tags are byte-equal (true collision pair), with tags differ; BASE without tags c22356ee = 5a log's left value 3257095918.
- 5a fails at the FIRST known answer (log: layout.rs:271 in the planted file = :273 `layout_hash(BASE) == 0x9adb_ecc7`), not at TAG_PAIR: it shows the function is held, not that the pair bites on its own; no `assert_ne!(layout_hash(TAG_PAIR[0]), layout_hash(TAG_PAIR[1]))` states the property (layout.rs:278-279 pin values only). Low.
- 5b fails at layout.rs:264 "a changed field in the second section must change the hash": the property bites (my T1's class).
- read: ts-rs 12.0.1 format_docs (lib.rs:1188-1212): a single #[doc] holding '\n' (a Rust /** */ block doc) prints as `/**{doc}*/` with the end glued to its last line; `///` lines print as " *" + line. So the line check (ts_export.rs:123-125) false-fails an honest block doc `/** one\n two */` (" two */" holds "*/"), message "holds */" untrue. Low (repo style is ///).
- note (outside my area, for the advisor): every commit of this session carries `Co-Authored-By: Claude Fable 5.2` (git log 1a2ff13..5bf313d); CLAUDE.md: "The main model and every agent stay Opus 5.5".
