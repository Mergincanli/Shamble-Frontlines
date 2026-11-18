# advisor-final: final review of change-set 1 (Types), PR #58, head cdaef8e (code 65a7b13), base 22c6afa

Started 2026-11-18 14:37 UTC (clock: `date -u` 14:36:39).

## Plan
1. Read my own conditions (advisor-plan.md, advisor-security.md) and list them.
2. Read the report, the diff (minus .checkpoints), the resume card, the round notes, fix-pass-1, plant/runs20, docs-keeper notes.
3. Read DECISIONS entries that bind it (2026-11-18 11:08:25, 2026-11-17 22:10:34, 2026-11-17 19:10:44, 2026-11-14 13:58:05).
4. Check 1: each condition met or not, one line.
5. Check 2: protocol (plan before edits, gate per step, evidence, rounds, report, docs pass, checkpoint delete).
6. Check 3: report vs evidence; Decisions needed entries (DEP-HOLD, CI-TYPES, CI-CUT, LANE-ISSUE); no kit/gate/CI file in diff.
7. Check 4: RECORD-FILES, OLD-LINES; run blame check and decisions.mjs --check.
8. Check 5: scope, lane files, size count (940 vs 600, stop 1,200).
9. Check 6: the three early stamps and the gh read: does the report need more.
10. Check 7: PR description text and state.
11. Optional: own tests in a throwaway worktree (own target dir, --locked), then git worktree remove.

## Findings (a line each, as found)
- By 14:39 UTC (clock; an earlier "14:45" here was ahead of the clock, corrected) Read: my plan + security notes; resume card; report (360 lines); diff file list (no kit/gate/CI file; all modes 100644; no symlink/submodule).
- Size recounted by me (numstat vs 22c6afa, generated + lockfile + docs out): 940 added, 12 removed. Matches report.
- decisions.mjs --check: exit 0 (1,554 bytes equal in DECISIONS and merge commit 22c6afa; header + pointer; every other byte unchanged).
- docs-keeper-blame-check.mjs: exit 0 (34 removed: own 8, moved 24, blank 2). 25 listed moved lines = 24 + own #57 row. New line numbers verified (ROADMAP 90, 92-106; ARCHIVE 1275-1282; STATUS 120 lines).
- Moved lines are about merged PRs: #43, #44 (ROADMAP), STATUS-DIET table (#49's record; reasonable reading; archive note fixes "this file").
- Code read: layout.rs, layout_ts.rs, ts_export.rs, messages.rs, lib.rs x2, TS files. ts-rs only via Config::new().with_out_dir(scratch) (ts_export.rs:110-112); no #[ts(export)], export_to, from_env (to grep).
- Condition 2 (ts-rs only via Config::new() + own folder): grep crates/packages/Cargo.toml for ts(export|export_to|from_env|TS_RS: doc lines only (ts_export.rs:3-4). MET.
- Condition 5: security-auditor-review1.md R1 (ts_export.rs), R2 (both write modes, trials), R5 (A1 confirmed, narrowed). MET.
- 20-runs set: my sha256 of the nine committed files at HEAD == runs20-results.txt's set. Code unchanged 65a7b13..HEAD (fec166b now: two checkpoint commits since cdaef8e, .checkpoints only).
- Round 2: all five APPROVE / nothing >= Medium (rr, te, sa, ea, na notes). te's report conditions (doc check words, 5a "known answers bite", marks) are in the report.
- Plan Lows 5(a)-(i): met per record, (e) --diff-filter=d not seen in a record (to check).
- Low candidate: report miss 1 last bullet + STATUS Follow-ups line drop hold-rule part (d) (committed anyway -> push nothing; a skip mark on a non-head commit or a cancel protects nothing): the exact route of miss 1.
- Low candidate: report's process lines without a time: the two planted runs not all as wanted (run1 12:48, run2 13:39 per file times), the rm refusal (step 7, 12:48-12:50).
- Refusal (guard): `git -C <dir> rev-parse` refused ("no option before the subcommand"); route: cd into the folder, plain git.
- Next: own checks in a worktree (.gate/cs1-types/adv, own target): gate --no-smoke (NODE_PATH to main's tools/node_modules), fmt, clippy, cargo test, write mode + git status, tsc by path, Vitest by path.
- 14:42-14:44 UTC (clock) own checks in worktree .gate/cs1-types/adv at fec166b (code = 65a7b13), target .gate/cs1-types/adv-target, every cargo call --locked --offline:
  gate --no-smoke PASS (1-4, 5b-5d; NODE_PATH = main's tools/node_modules); full gate: smoke cannot start here (tools' Playwright wants chromium 1194, 1243 installed; via root's Playwright 1254): FAIL for that reason only, CI's to run.
  fmt --check 0; clippy -D warnings host 0 and wasm32 0; cargo test --workspace: client-endpoint 5, protocol 2, others 1 each, exit 0;
  SHAMBLE_WRITE_TYPES=1 cargo test --workspace: exit 0, all nine generated files newer than my stamp, git status --porcelain --untracked-files=all empty (PM-0002-15 first clause, here);
  release tests sim 1 + protocol 2; wasm32 build of five crates (fresh rlibs); cargo tree: protocol alone on wasm32, workspace normal+build --target all = 6 workspace crates, -i ts-rs normal/build: nothing;
  pedantic clippy + missing_docs on both crates after cargo clean -p: 0 warnings; Vitest by path in packages/types: 2 files, 7 tests passed.
  tsc 6.0.3 by path with the CI-TYPES flags: TS5112 in my nested worktree (".gate/cs1-types/tsconfig.json is present but will not be loaded if files are specified on commandline. Use '--ignoreConfig'"): tsc walks up to the main session's scratch config. With --ignoreConfig: shipped 0 errors, shipped+tests 0 errors.
  -> FINDING (Low, report appendix CI-TYPES): under TS 6.0 a tsconfig.json in the checkout root or any parent turns both tsc lines into TS5112 errors; 2.3's base tsconfig will add one. Add --ignoreConfig to both lines before he answers (a loud fail, not a silent pass).
- Refusal (guard): `touch` of two source files (relative, after cd) in my worktree to force clippy; route: none taken for touch; `cargo clean -p` in my own target instead.
- Cleanup: cargo clean of my target (removed), git worktree remove (no force), stamp file deleted.
