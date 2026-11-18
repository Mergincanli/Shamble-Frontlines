# verifier-cs1: notes (change-set 1 "Types", PR #58)

Started 2026-11-18. Branch claude/wonderful-hamilton-jdmlnf, head cdaef8e, code commit 65a7b13, base origin/main 22c6afa
(local `main` is stale at 44446a5, 356 behind origin/main; every diff below uses origin/main).

## Plan
A. Gate and hand checks, each with exit code: gate.mjs --no-smoke; repo-map-check; cargo fmt; clippy host + wasm32;
   cargo test workspace; release test sim+protocol; wasm32 build of the five crates; cargo tree protocol (host, wasm32);
   tsc both scratch configs; vitest by path; the three netcode rules; CI gate + toolchain newest runs and their heads.
B. Read plant.sh and runs20.sh first (does each run prove its label?); run plant.sh -> .gate/cs1-types/vf-plant-results.txt
   (want "runs: 30, not as wanted: 0", exit 0), diff against plant-results.txt; run runs20.sh ->
   .gate/cs1-types/vf-runs20-results.txt (20 runs, all rewritten yes, one distinct set, sha256 07cd492f...4bc8, exit 0),
   then git status --short must be clean.
C. Symbol list: plan-draft.md s4 + plan.md item 5(g) vs every pub symbol and test in crates/protocol/src,
   crates/client-endpoint/src, packages/types/src (not generated/); compare with the report's deviations.
D. Report claims vs records: git log times/SHAs, run numbers, Cargo.lock nine entries, test counts, hashes, set hash,
   numstat size, reviewer verdicts/counts, pass marks vs ADR 0002, Mustafa quotes byte for byte, appendix texts,
   25 moved lines.
E. decisions.mjs --check; docs-keeper-blame-check.mjs (0 lines neither lane's nor moved); STATUS <= 120 lines.
F. git diff --name-only origin/main...HEAD: allowed paths only.

## Log (one line per check: command -> result)
- Head moved by the saver to fec166b (1b4fb40, fec166b: .checkpoints only); `git diff --stat 65a7b13 HEAD -- . ':!.checkpoints' ':!docs'` empty: code = 65a7b13.
- Observation (outside A-F): all 61 branch commits carry "Co-Authored-By: Claude Fable 5.2"; CLAUDE.md: main model and agents stay Opus 5.5. Not judged; for the caller.
- B read: plant.sh 30 runs, own worktree .gate/cs1-types/plant + own target plant-target; runs20.sh main folder, mtime-vs-stamp rewrite check, set sha256 incl. paths. Notes: plant.sh `edit` exit not checked (a failed plant shows as "not as wanted" for fail runs; 3a's pass would be hollow but 3d catches it); run 4 prints failed tests but does not assert "only generated_layout_is_current"; 6a/6b use the newest client_endpoint binary, last built by 5b's planted source (guard code unchanged, comment says "clean runs").
- A1 `node tools/gate.mjs --no-smoke` -> exit 0, GATE PASS: [1] 5/5, [2] PASS, [3] +0 -0 (core 138/game 714/editor 353), [4] 51 guarded 0 flagged, [5a] SKIP, [5b-d] PASS. Base 22c6afa (origin/main).
- A2 `node .claude/scripts/repo-map-check.mjs` -> exit 0 ("139 entries cover 109 required paths").
- B1 `bash .checkpoints/cs1-types/plant.sh > .gate/cs1-types/vf-plant-results.txt 2>&1` -> exit 0, 14:39:51-14:40:06 UTC, "runs: 30, not as wanted: 0", worktree removed (no force), status clean. Real compiles in logs (Compiling protocol/client-endpoint per plant). diff vs committed plant-results.txt (timings normalized, line 1 aside) -> identical. Run 4: only generated_layout_is_current failed (4 passed 1 failed); LAYOUT_HASH 0x2df78c30 -> 0x5d0e9efb. 17a/17b: TS1294. 3d: TS2353 at index.test.ts(41,5).
- A3 `cargo fmt --all --check` -> exit 0.
- A4 `cargo clippy --workspace --all-targets --locked -- -D warnings` -> exit 0, 0 warnings; A5 same `--target wasm32-unknown-unknown` -> exit 0, 0 warnings.
- A6 `cargo test --workspace --locked` -> exit 0: client-endpoint 5 (fnv1a_matches_rfc_9923, layout_rules_hold, hash_covers_every_value, typescript_carries_the_hash, generated_layout_is_current), protocol 2 (generated_types_are_current, overflow_checks_are_on). Export tests' copy guard passed in main folder (binaries are this copy's).
- A7 `cargo test --release -p sim -p protocol --locked` -> exit 0 (protocol 2, sim 1).
- A8 `cargo build --locked --target wasm32-unknown-unknown -p sim -p protocol -p room -p client-endpoint -p sim-wasm` -> exit 0.
- A9 `cargo tree --locked -p protocol -e normal` -> exit 0, `protocol` alone; with `--target wasm32-unknown-unknown` -> exit 0, `protocol` alone. Extra: `-i ts-rs --workspace -e normal` -> nothing to print; `-e dev` shows ts-rs 12.0.1.
- A10 `tsc -p .gate/cs1-types/tsconfig.json` -> exit 0 (13 files); `-p .gate/cs1-types/tsconfig.shipped.json` -> exit 0 (11 files, no tests). tsc 6.0.3.
- A11 Vitest by path in packages/types -> exit 0, 2 files, 7 tests (v5.1.0).
- A12 rules: no root .cargo/config.toml (exit 0); no "type" in root package.json (exit 0); spike search over 32 non-md files -> prints nothing (xargs 123).
- A13 CI (gh): newest branch runs on cdaef8e: gate 876 (push) + 877 (pull_request) success; toolchain 174 (push) + 175 (pull_request) success. fec166b/1b4fb40 carry [skip ci] (checkpoint-only). Main on 22c6afa: gate 866, toolchain 165 success. fe49935: gate 874/875, toolchain 172/173 success (= report).
- D (miss 1): 566d1d1 and da22f96 author+commit 12:04:33Z (git log), da22f96's parent = 566d1d1; GitHub activity API: push 55397c8->da22f96 at 12:04:36Z (report: 12:04:37, checkpoint line 247 "the push at 12:04:37": session clock; 1 s; Low note at most). Run 44300471741 = toolchain #168, step 7 12:04:55->12:05:07 (API); log: Compiling proc-macro2 12:04:55.98 ... ts-rs 12:04:59.22, tests ran, "The operation was canceled" 12:05:07.49 (after the build: confirmed). Cancel sent 12:05:01 and chat 12:06:24: session-only record (checkpoint 247-249), not checkable on GitHub.
- B2 `bash .checkpoints/cs1-types/runs20.sh > .gate/cs1-types/vf-runs20-results.txt 2>&1` -> exit 0, 14:43:39-14:43:49 UTC: 20 runs, 9 files each, all rewritten: yes; distinct sets 1; set sha256 07cd492ff01b609f8d984208765b770d3aae115354b9b0949a18a43a44ed4bc8 = record; "git: no difference"; `git status --short` after: only .checkpoints notes (advisor-final.md, verifier.md), no code/generated change. Its "worktrees: 2": the advisor agent's .gate/cs1-types/adv appeared meanwhile; evidence holds (mtime check + copy guard).
- C public Rust: protocol `pub mod messages` + 8 structs (Join 3 fields, JoinAccepted 9, Input 2, SnapshotAck 2, Command 8, AimPoint 3, ViewTime 2, Snapshot 5), types/widths = plan s4 exactly, derives Debug/Clone/PartialEq/Eq + cfg_attr(test, TS). client-endpoint `pub mod layout`: ElementType{U8,U32} + name() + size() [named add], Field{name,ty,offset}, Section{name,stride,fields}, BLOCK_LAYOUT (header stride 8, tick u32@0, layout_hash u32@4), LAYOUT_HASH, layout_hash(). Derives Debug/Clone/Copy/PartialEq/Eq on the 3 [named add]. Test-only modules ts_export, layout_ts = 5(g). stub_builds (client-endpoint) and PACKAGE removed = 5(g); package.json description changed = 5(g).
- C Rust tests (7): generated_types_are_current, overflow_checks_are_on (kept), layout_rules_hold, hash_covers_every_value, fnv1a_matches_rfc_9923, generated_layout_is_current, typescript_carries_the_hash = plan s5 list exactly, no rename.
- C TS exports (not generated): 8 `export type` messages, BLOCK_LAYOUT + LAYOUT_HASH, checkLayoutHash, type LayoutCheck (readonly union = plan). `header` -> BLOCK_LAYOUT = deviation 4 (named).
- C Vitest (7) vs plan's 5 behaviours: +"a block of zero bytes fails" (fix pass, TE 6) and "values of the eight message types build..." (82142fa's "the package exports the eight message types", renamed in fix pass for TE Medium 3). Named elsewhere in report (PM-0002-16 "a block of zero bytes"; planted 3 "the package's own test"; round 1) but not under "Added beside the plan's symbol list": Low.
- C private, not named in the report's private list: fnv1a(), FNV_OFFSET_BASIS, FNV_PRIME: described by plan s4 (basis/prime/test name) -> not a finding.
- C independent hash (node, from the doc comment): block 0x2df78c30, planted 0x5d0e9efb, BASE 0x9adbecc7, TAG_PAIR 0x4265359c / 0xbd51cd2f, FNV "a" 0xe40c292c, "foobar" 0xbf9cf968: all equal the code/record. TAG_PAIR streams without tags equal byte for byte (checked by hand).
- D "Checks by hand on the final code (65a7b13), from an empty build folder": records put cargo clean 13:33-13:37 (fix-pass-1.md:14; EA review2:35 oldest file 13:37:24), before plant run2 on 1a2ff13 (13:39:07) and 65a7b13 (13:39:47, 9 test-only lines). No record of a second clean. My reruns on identical code: all exit 0 -> substance holds; qualifier imprecise: Low.
