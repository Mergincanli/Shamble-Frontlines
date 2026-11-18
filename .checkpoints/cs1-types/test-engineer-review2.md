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
