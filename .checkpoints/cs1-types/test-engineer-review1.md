# test-engineer, round 1 review of cs1-types (head 9b8c668, code 82142fa, base main 22c6afa)

## Plan
1. Read: the diff (code only), plan.md + plan-draft.md sections 2, 5, 6; ADR 0002 marks PM-0002-15..19, -43, -63,
   migration step 4; build plan card 1, Part 3 rows, Part 7; checkpoint steps 6-7; plant.sh + results; runs20.
2. Per mark: claim (met / shown) vs evidence; name uncovered clauses.
3. Bite analysis: read each Rust and Vitest test; look for tests that cannot fail.
4. Own defects (>= 3 not in plant.sh) in worktree .gate/cs1-types/te-plant (CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/target),
   restore each file, delete planted files one by one, `git worktree remove` without --force.
5. Vitest: run by path; tsc with the scratch configs.
6. CI: what toolchain.yml runs today; the drafted CI edit's commands; what is wrong in the main session's draft.
7. Robustness: threads, --release, dirty/missing generated folder, CRLF/autocrlf, second run, scratch leftovers.

## Findings (one line each, as found)
- baseline (main folder, no write mode): cargo test -p protocol -p client-endpoint --locked exit 0 (2 + 5 passed); tsc -p .gate/cs1-types/tsconfig.json exit 0, -p tsconfig.shipped.json exit 0 (also ran once with --listFilesOnly: 9 generated + 4 src files read); Vitest 2 files, 6 tests passed, exit 0.
- read: .gitattributes `* text=auto eol=lf` covers CRLF/autocrlf checkouts; TypeScript 6.0.3, Vitest 5.1.0; node_modules/@types holds chai, deep-eql, estree only.
- read: toolchain.yml:70 `env -i` clears SHAMBLE_WRITE_TYPES inside the cut, so write mode cannot leak in from the runner env.
- read: ts-rs 12.0.1 `Config::new()` = defaults, no env read (lib.rs:607-609; `from_env` is separate, 618); imports and merges sorted by BTreeMap/BTreeSet (export.rs:340-360, 220): no env var or hash seed can change the bytes. EXPORT_PATHS is per process (export.rs:20); the fresh scratch folder per run avoids its merge path.
- reading: `hash_covers_every_value` last line `assert_eq!(LAYOUT_HASH, layout_hash(BLOCK_LAYOUT))` (layout.rs:229) cannot fail: LAYOUT_HASH is defined as exactly that (layout.rs:61). Low.
- reading: every CHANGED table (layout.rs:206-217) changes the first section; none changes only the second, none swaps or adds sections -> to plant (T1, T2).
- reading: BAD's overlap case (layout.rs:178) has the later field start inside the earlier one only; the other direction is untested -> to plant (T3).
- reading: index.test.ts second test (index.test.ts:13-43): its three runtime expects check the test's own literals; types are erased by Vitest -> cannot fail in CI (no tsc there, toolchain.yml:87 prints the version) -> to plant (T12).
