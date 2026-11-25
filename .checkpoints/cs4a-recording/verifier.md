# verifier: change-set 4a "The recording" (draft PR #70, lane 1)

Started 2026-11-25 22:08 UTC. Worktree `.gate/lane1-takeover`, branch cs/4-walk (pushed as claude/cool-pasteur-oiqk1r).
Source final at fb5d0ff; base main 07d673b (merged in by 21dff0c); 4a's own base on this branch 7670c7b.
Change type: logic + netcode (deterministic replay), small performance reading; risk tier 1; no browser check.
Read-only on the working copy; my builds under `.gate/verifier-*`; no git add/commit/push; cargo only --locked.

## Plan
0. Scope: `git diff --stat fb5d0ff HEAD -- crates packages` empty; `git diff --name-status 07d673b...HEAD`: no
   manifest, lockfile, CI, kit or gate file; symbol/file diff against the plan's list.
1. Legacy gate: `node tools/gate.mjs` (smoke if its tools are here, else --no-smoke and say so); CI on PR #70 via gh.
2. Rust: fmt --check; clippy -D warnings --locked; cargo test --workspace --locked (expect 140 passed in debug);
   release line for six crates (expect 139); make_recordings --check.
3. Replays: native recordings / two_processes / two_instances; vitest boundary (expect 3 files, 24 tests);
   Chromium spec tests/replay (expect "first-loop.rec: 1800 steps, every shared hash as in the file").
4. The players' file: size 54,514, sha256 198477...f708; exports against main's gain exactly `sim_shared_hash`.
5. Size: `git diff --shortstat 7670c7b HEAD -- crates packages ':!crates/devtools/recordings'
   ':!packages/e2e/playwright.config.ts'` (claimed 3,300 insertions; stop line 3,400).
6. Plants: own worktree `.gate/verifier-4a` at HEAD, own target dir; plant A native half (tick 899 presses flip,
   red naming step 900); plant G native half (script's text for G: two-matches red, recordings solo green). Then
   remove my edits.
7. His words: `node .checkpoints/cs4a-recording/decisions-4a.mjs check`.
8. Spot-checks: three hash-report claims (known answers via rr_known_answers.py, a timing, ...); two fold findings
   marked fixed (R1 World::visit names every field; S1 Replay::new refuses a seed without TESTSEED).

## Results (one line per check as it finishes)
- 22:09 note: my first four read-only shell calls put `cd <worktree> &&` in front of git; every later git/gh call
  starts its own command. Working copy before my runs: clean except this file (`git status --porcelain`).
- 0a `git diff --stat fb5d0ff HEAD -- crates packages`: empty (source final at fb5d0ff). PASS
- 0b `git diff --name-status 07d673b...HEAD`: 89 paths: .checkpoints/ (cs4-walk, cs4a-recording), crates/devtools,
  room, sim, sim-wasm, docs/DECISIONS.md + docs/UNITS.md, packages/e2e (node-names.d.ts, replay spec),
  packages/sim-worker. No Cargo.toml/Cargo.lock/package*.json, no .github/.claude/CLAUDE.md, no tools/ gate file. PASS
- 0c main's #71 (c1da1bd..07d673b) touched only packages/e2e/playwright.config.ts in crates/packages; nothing between
  c1da1bd and 7670c7b touched crates/packages.
- 2a `cargo fmt --all -- --check`: exit 0, no output. PASS
- 2b `cargo clippy --workspace --all-targets --locked -- -D warnings`: exit 0, 0 warning lines. PASS
- 2c `cargo test --workspace --locked` (debug): exit 0, 140 passed, 0 failed, 0 ignored, 21 test binaries (claimed 140). PASS
- 2d `cargo test --release -p sim -p protocol -p room -p client-endpoint -p sim-wasm -p devtools --locked`: exit 0,
  139 passed, 0 failed (room's unit tests 16 in release vs 17 in debug) (claimed 139). PASS
- 2e `cargo run -q -p devtools --locked --bin make_recordings -- --check`: exit 0, "the recordings are what their
  scripts make". PASS
  (2a-2e ran on the worktree's own target, cached by the main session's local checks; a clean build in my own
  target dir follows in my copy .gate/verifier-4a.)
- 22:13 course correction from the main session (check 4): the digest in the brief (198477...f708) was the file at
  07903d9; the claimed digest on fb5d0ff is now a4d8184c15736ec2cf2c347871afddcff289dd0a364f0d2016c87dd9866cd094,
  54,514 bytes, both builds. To do: measure it myself; if cheap, build the file from 07903d9 in my own copy and say
  what differs (`cmp -l`, sections).
- 4a players' file `stat`/`sha256sum packages/sim-worker/pkg/sim_wasm_bg.wasm` (built by the main session 22:04):
  54,514 bytes, sha256 a4d8184c15736ec2cf2c347871afddcff289dd0a364f0d2016c87dd9866cd094; the same bytes in
  .gate/dist-development/assets/ and .gate/dist-production/assets/ (sim_wasm_bg-BpePgaon.wasm). Matches the
  corrected claim, not the brief's 198477...f708 (which the main session says was 07903d9's). Own rebuild: below.
- 4b exports (WebAssembly.Module.exports, Node 22.23.5): 19 exports incl. sim_shared_hash; 3 imports, all from the
  glue. The boundary pin (sim-wasm.test.mjs) diff vs main: Sim.prototype gains `shared_hash`, per-tick map gains
  ["shared_hash()", "sim_shared_hash"]; the raw Wasm export list is not pinned there. Main's file: to build.
- 3b `node node_modules/vitest/vitest.mjs run --root packages/sim-worker --dir packages/sim-worker/boundary`: exit 0,
  3 files, 24 tests passed (claimed 3 / 24). Replay tests by name: manifest, first-loop on the players' file,
  two matches on one module instance, test module release and dev to the committed hashes + vector state. PASS
  Reading printed: FNV-1a 64 in Wasm (Node 22, release module): 4096 B 4,887 ns; 16384 B 19,824 ns.
- 3c Chromium: `SHAMBLE_DIST=$PWD/.gate/dist-development SHAMBLE_CHROMIUM=/opt/pw-browsers/chromium node
  node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=chromium --workers=1
  --retries=0 --trace off tests/replay` (port 4173 free first; main session's log ended "exit 0" at 22:07): exit 0,
  1 passed (395 ms), printed "first-loop.rec: 1800 steps, every shared hash as in the file, in ... HeadlessChrome/158.0.0.0". PASS
- after 2a-3c: `git status --porcelain -- crates packages` empty; the only other changes are docs-keeper's (docs/,
  docs-keeper files).
- 5 `git diff --shortstat 7670c7b HEAD -- crates packages ':!crates/devtools/recordings'
  ':!packages/e2e/playwright.config.ts'`: 28 files, 3,300 insertions, 102 deletions (claimed 3,300; stop line 3,400).
  Cross-check `git diff --shortstat 07d673b...HEAD -- crates packages ':!crates/devtools/recordings'`: the same
  28 / 3,300 / 102, so the excluded playwright.config.ts change (19/1) is main's #71 alone. PASS
- 7 `node .checkpoints/cs4a-recording/decisions-4a.mjs check`: exit 0 in the working copy and in my clean copy at
  HEAD: five messages (890, 1256, 994, 2728, 5779 bytes) word for word in docs/DECISIONS.md; UNITS.md: Hornet
  sentence word for word, the two lines unchanged. Extra: against main both docs only gain lines (146/0, 10/0); the
  entries cite fb5d0ff, and the five saved files are unchanged since then and were never modified after being added
  (`git log --diff-filter=M` empty). PASS. Not verifiable by me: that the saved files equal his chat messages.
- 4c own builds (copies at HEAD, 07903d9, 07d673b under .gate/; each fresh target; `npm ci --ignore-scripts` in each
  copy; `SHAMBLE_WASM_BINDGEN=<.gate/wb-tool/wasm-bindgen> node packages/client/scripts/build.mjs`, HEAD full dev
  build, the other two `--wasm-only --production`; all exit 0):
    HEAD     54,514 B  a4d8184c15736ec2cf2c347871afddcff289dd0a364f0d2016c87dd9866cd094 = the main session's file; glue
             sha 485359a7... identical; test-only module release c37919d8... and dev 11312c11... identical too.
    07903d9  54,514 B  198477964866d717fff105ebe4b64e5fb8aee1c861e06d4e460e2bf1a7fff708 (= the brief's digest: it was
             round 1's file, as the main session said).
    07d673b  53,813 B  5ebae15400f9435dfd31108fe380e96253695b8bed4a7e639ce94bc180b41262 (main; +701 B for 4a).
  `cmp -l` 07903d9 vs final: exactly 1 byte, file offset 42247 (1-based): 0x9B -> 0x9C. My parser
  (verifier-wasm-diff.mjs): data segment 0, a panic Location {file "crates/sim/src/world.rs", line 155 -> 156,
  column 14}; world.rs line 156 at fb5d0ff is `.expect("the tick counter is at u32::MAX and cannot advance")`; the
  shift is round 2's `#[cfg(test)]` added at line 21. The main session's guess holds.
- 4d exports main vs final (WebAssembly.Module.exports): 18 -> 19, added exactly ["sim_shared_hash"], removed none;
  imports identical (3, all from the glue); glue: Sim.prototype gains only `shared_hash`. PASS
- 2f clean native build: `cargo test --workspace --locked --target-dir .gate/verifier-target` in my copy at HEAD
  (fresh target, 22 crates compiled): exit 0, 140 passed, 0 failed. PASS
- 3a native replays, by name, debug and release (from 2c/2d logs): recordings.rs every_recording_replays_to_its_hashes,
  every_committed_file_is_what_its_script_makes, the_check_names_each_file_...; two_processes.rs
  two_more_processes_give_the_same_hashes_as_each_other_and_as_the_files (children replay every script + a
  36,000-tick match, compared with each other and the files; fails as not valid if hash-map fingerprints are
  equal); two_instances.rs two_matches_stepped_in_turn_each_give_their_own_hashes. All ok in both profiles. PASS
- 6 plants, in my copy .gate/verifier-4a (HEAD), target .gate/verifier-target; each plant inserted after the anchor
  `self.player = *newest; }` in World::authority_step by a python one-liner asserting a unique anchor (the plant
  script's own text), removed by copying HEAD's world.rs back (`cmp` equal):
  A `if self.tick == 899 { self.player.presses[0] ^= 1; }`: `cargo test -p devtools --locked --test recordings
    every_recording_replays`: exit 101, "first-loop, Shared ... Difference { step: 900, expected tick 900 ... }". CAUGHT
  G (static STEPS counter, reset at tick 0, flips presses[1] when it disagrees with the tick): `--test
    two_instances`: exit 101, panicked at two_instances.rs:32:13, difference at step 3. CAUGHT. Controls:
    `--test recordings every_recording_replays -- --test-threads=1`: exit 0 (HELD); `--test two_processes`: exit 0
    (HELD, extra).
  After removal: copy's crates/ = HEAD's (`diff -rq` empty); recordings, two_instances, two_processes green (exit 0);
  working copy `git status --porcelain -- crates packages` empty. PASS (2 of 15 defect checks and 1 of 4 controls
  re-run by me; the rest of plants-4a.txt not re-run, by the brief).
- 1 legacy gate, in my copy at HEAD (tools/node_modules by `npm ci --ignore-scripts` there; this worktree has none):
  `node tools/gate.mjs` (base merge-base(HEAD, origin/main) = 07d673b): [1] 5/5 parse, [2] PASS, [3] core 138 / game
  714 / editor 353, +0 -0, duplicates 0, [4] 51 guarded 0 dead 0 flagged, [5b] [5c] [5d] PASS; [5a] smoke FAIL to
  start: Playwright 1.56.1 wants /opt/pw-browsers/chromium_headless_shell-1194, the machine has -1243 (installing
  needs npx, refused by the brief). `--no-smoke`: GATE PASS (5a skipped). 4a touches nothing in legacy/.
  => checks 1-4, 5b-5d PASS; 5a NOT VERIFIED locally (CI's smoke job: below).
- 22:20 the branch moved while I ran: local HEAD 6fddbf6, pushed head (PR #70) 21c1046. `git diff --name-status
  9b5c96a HEAD`: only .checkpoints/ and docs/ (REPO_MAP, ROADMAP, STATUS, STATUS_ARCHIVE, docs/reports/2026-11-25-
  cs4a-recording.md); `git diff --stat fb5d0ff HEAD -- crates packages` still empty; DECISIONS.md and UNITS.md not
  touched after 9b5c96a. My results stand for the final source.
- 1b CI on 21c1046 (`gh api .../commits/21c1046.../check-runs`, REST; GraphQL refused): gate x2 success, smoke x2
  success (22:17:54-22:19:04 UTC), toolchain x2 in progress at 22:20. To re-read.
- 8a fold R1 (`World::visit` names every field): code world.rs:163-174 `let Self { tick: _, player: _, seed } = self;`
  and visit_shared :177-188 names all three. By running: a field `extra: u8` added to World and new() in my copy:
  `cargo check -p sim --locked`: exit 101, E0027 "pattern does not mention field `extra`" at world.rs:168 (visit) and
  :182 (visit_shared); restored (`cmp` equal). FIXED, CONFIRMED
- 8b fold S1 (seed checked in Replay::new and checkReplayable; Header's Debug without the seed): replay.rs:34-39 check()
  refuses a seed not starting with TEST_SEED_MARK (*b"TESTSEED", recording.rs:39), Replay::new calls it first (:90);
  recording.rs:59-69 Header's Debug by hand, no seed, `finish_non_exhaustive`; Header derives no Debug (:49);
  recording.ts:134 checkReplayable refuses. By running: `--exact replay::tests::what_this_build_cannot_replay_is_
  refused_by_name_before_any_call` green; with the seed condition made `false && ...` in my copy: exit 101, left None,
  right "this build cannot replay this recording's seed: a replay takes a public test seed only" (so Sim::create
  alone would take any seed); restored. TS: `vitest run --dir src --reporter=verbose recording` (sim-worker): 5 of 5
  passed incl. the refusal test (recording.test.ts:150-155). FIXED, CONFIRMED
- 8c hash report item 4 (13 of 13 known answers): `python3 -I rr_known_answers.py crates/devtools/recordings/
  first-loop.vector.txt`: 13 ok, ALL OK; each of its 8 world.rs constants is in the final world.rs tests (:403-423),
  which pass in 2c/2d. CONFIRMED
- 8d hash report items 1 and 8 (cost: about 1.25 ns a byte, native and Wasm alike; 20 us at 16 KiB; p99 about 40 us):
  native `cargo test --release -p devtools --locked --test hash_cost -- --nocapture` x3 on the idle machine:
  16,384 B mean 20.93 / 20.59 / 20.49 us (p99 41.1 / 37.0 / 34.6 us); 4,096 B 5.17 / 5.16 / 5.03 us => 1.25-1.28
  ns/B. Wasm (boundary replay test's reading, Node 22.23.5, release module) x3: 16,384 B 19.82 / 21.02 / 21.74 us;
  4,096 B 4.89 / 5.13 / 5.17 us => 1.21-1.33 ns/B. Report: native 20.2-21.1 (p99 31.7-40.9), Wasm 19.2-21.0. My
  native range sits inside; my Wasm top (21.7) is 3 % above its range (one-run means, a few % noise; a different
  harness from its bench). CONFIRMED within noise.
- 8e hash report items 2, 9, 7 (detection), my own script verifier-fnv-claims.py (seeded, not the reviewer's code):
  one-byte changes: 0 of 13,770 (every position x 255 values of the real 54-byte state from the vector line +
  TESTSEED seed) and 0 of 6,120 (3,076-byte state, 24 positions x 255) hash like the original; two sign flips: low 8
  bits unchanged in 3,000 of 3,000, 9th bit changed in 1,479 (report 1,512); word-fed FNV, two top-bit flips in
  different words: whole hash unchanged in 10,000 of 10,000. The 4 standard FNV-1a 64 values also from my own node
  BigInt code. CONFIRMED (that RFC 9923 section 1.4 prints them: not opened, no fetch).
- 1c CI on 21c1046 (re-read 22:24): gate x2 success, smoke x2 success, toolchain (pull_request run 45712016082)
  success 22:17:58-22:23:01; toolchain (push run 45712006790) still in progress. Its log (`gh api .../actions/jobs/
  139548551493/logs`): cargo test result lines sum 419 passed / 0 failed (= 140 + 140 + 139), sim_wasm_bg.wasm
  54,514 B sha256 a4d8184c...d094 (= mine), boundary 3 files / 24 tests, Chromium 157 "first-loop.rec: 1800 steps,
  every shared hash as in the file", 17 browser tests in 3.2 min. The final source (fb5d0ff) is inside 21c1046.
- 0d symbol diff against the plan (plan.md section 4 table; symbols in netcode-architect-cs4.md block 1; changes
  from the round-1/2 folds), `git diff -U0 07d673b HEAD` on pub items:
  + sim: StateSink (re-exported), World::visit, visit_shared, write_shared_state (step 1); Fnv1a64 pub(crate) with
    start/write/finish (fold R8/E1, buffer-free hash). + room/LocalMatch/Sim::shared_hash(&self) (step 2); Wasm
    export +sim_shared_hash only. + devtools modules compare, files, recording, replay, scripts, sha256; bins
    make_recordings, replay; test-module exports replay_hashes, replay_state_at, fnv1a_64_rounds (steps 3-5; sha256.rs
    and files.rs are not in the plan's table but implement the design's 1.1/1.4 digests and the folder check, and
    size-notes says so). + TS recording.ts (step 7); e2e node-names.d.ts gains readFileSync(path) and readdirSync.
  ~ World::state_hash/shared_hash lose the scratch argument (fold R8/E1); write_state and STATE_BYTES now
    #[cfg(test)] (round 2). - nothing else dropped. 3B-HOSTILE-ALLOC's any_bytes tests untouched (moved by the ladder,
    told to him in STOP-4A). No live code calls shared_hash: only the glue in the built Worker holds the method, no
    Worker or page source calls it. Matches the plan with its folds: yes.
- 0e .checkpoints/ files are in the branch (expected for a draft); before the PR is marked ready they must go.
