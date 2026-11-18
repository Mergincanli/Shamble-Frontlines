# CI evidence of kit-hold-ci (read with the GitHub tools by the main session; times are the logs' own, UTC)

All runs are on branch `claude/wonderful-hamilton-jdmlnf`, pull request #60, 2026-11-18. Workflow files:
`.github/workflows/toolchain.yml` (job `toolchain`) and `.github/workflows/gate.yml` (jobs `gate`, `smoke`).

## 1. The clean run of the new CI lines: head 94c197fb0b9675251bd7d18ba1084f6b81df728c
- `toolchain`, push: run 44357176860, job 134988309631, 17:18:27 to 17:19:18, success.
  https://github.com/Mergincanli/Shamble-Frontlines/actions/runs/44357176860/job/134988309631
- `toolchain`, pull request: run 44357282401, 17:18:56 to 17:19:41, success.
- The push job's log was read whole (581 lines). Lines copied from it:
```
2026-11-18T17:18:27.9098820Z Secret source: Actions
2026-11-18T17:18:27.9099464Z Cache mode: none
2026-11-18T17:18:27.9100040Z Prepare workflow directory
...
2026-11-18T17:18:38.0868597Z ##[group]Run actions/setup-node@820762786026740c76f36085b0efc47a31fe5020
2026-11-18T17:18:38.0868974Z with:
2026-11-18T17:18:38.0869427Z   node-version: 22
2026-11-18T17:18:38.0869674Z   check-latest: false
2026-11-18T17:18:38.0871846Z   token: ***
2026-11-18T17:18:38.0872096Z   package-manager-cache: true
...
2026-11-18T17:18:38.3651481Z Found in cache @ /opt/hostedtoolcache/node/22.23.4/x64
...
2026-11-18T17:19:01.1761653Z uid=1001(runner) gid=65534(nogroup) groups=65534(nogroup)
...
2026-11-18T17:19:01.1900704Z the network is cut: no TCP connection to 20.29.134.23:443 (curl exit 7)
2026-11-18T17:19:01.2205401Z    Compiling proc-macro2 v1.0.107
...
2026-11-18T17:19:05.3378174Z     Finished `test` profile [unoptimized + debuginfo] target(s) in 4.14s
   (the plain test run: every "test result: ok"; protocol's ts_export::generated_types_are_current ... ok;
    client_endpoint's layout_ts::generated_layout_is_current ... ok)
2026-11-18T17:19:05.4239549Z     Finished `test` profile [unoptimized + debuginfo] target(s) in 0.01s
   (the write-mode run: the same tests again, every "test result: ok", the last line at 17:19:05.5077732Z)
   (no line "writing the generated types again changed:" follows)
2026-11-18T17:19:05.5782860Z    Compiling proc-macro2 v1.0.107
...
2026-11-18T17:19:10.0298752Z     Finished `release` profile [optimized] target(s) in 4.46s
...
2026-11-18T17:19:10.1069730Z     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
2026-11-18T17:19:10.9009285Z added 46 packages in 757ms
   (npm test --workspaces: client 1 test, e2e 1 test, net 1 test, sim-worker 1 test, types 7 tests: all passed)
2026-11-18T17:19:13.5983683Z Version 6.0.3
   (1.04 s with no output: the two type-check runs print nothing when they pass)
2026-11-18T17:19:14.6377079Z Post job cleanup.
```
- The log holds no line about a cache being restored or saved (no "Cache restored", "Cache hit", "Cache saved",
  "cache key"): searched by eye in the 581 lines.

## 2. The planted change: head 0c0e27194bead3be4dea394b9d1b6c2c19532e74
`packages/types/src/generated/protocol/Join.ts` committed with mode 100755 (content unchanged).
- `toolchain`, push: run 44357616602, job 134989872637: failed. The end of its log:
```
2026-11-18T17:21:20.0773104Z    Doc-tests sim_wasm
2026-11-18T17:21:20.0838832Z 
2026-11-18T17:21:20.0839298Z running 0 tests
2026-11-18T17:21:20.0839705Z 
2026-11-18T17:21:20.0840002Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
2026-11-18T17:21:20.0840273Z 
2026-11-18T17:21:20.1392095Z writing the generated types again changed:
2026-11-18T17:21:20.1392577Z  M packages/types/src/generated/protocol/Join.ts
2026-11-18T17:21:20.1413830Z ##[error]Process completed with exit code 1.
2026-11-18T17:21:20.1522578Z Post job cleanup.
```
- `toolchain`, pull request: run 44357633651, job 134989932210: step 7 "Build and test with the network cut off"
  concluded failure, 17:21:17 to 17:21:21; the job ended 17:21:24.
- Tried locally before it (16:51 to 16:52 UTC, a worktree of its own with its own build folder): the plain
  `cargo test --workspace --locked` passed, write mode passed, the two compare lines printed the same two lines.

## 3. The take-back: head 415bd73b12cf8e9b2efa76e872956ab237320314 (pushed 17:21:55, after both plant runs had ended)
The code tree equals 94c197f. All four runs success: `toolchain` push 44357864275 (17:22:01 to 17:22:45) and pull
request 44357881263 (17:22:06 to 17:22:50); `gate` push 44357864589 (17:22:01 to 17:23:10) and pull request
44357881162 (17:22:06 to 17:31:27).

## What these runs prove, and what they do not
- Proven: the workflow file with `cache-mode: none` is accepted and the job's log prints "Cache mode: none"; git
  works inside the network cut as the runner's user (the compare lines ran: silent on the clean tree, failing on
  the plant); the two type-check lines exit 0 on the clean tree; the compare step fails when writing the types
  again changes a tracked generated file's mode.
- Not proven by a run: that the compare step fails on a content change (it cannot be reached with one: the plain
  test run one line above fails first; that failure is the test's own proof); that the type-check lines fail on
  a type error (not asked; shown locally in change-set 1's report); what "Cache mode: none" stops beyond what the
  log prints.
