# test-engineer, review round 1: kit-hold-ci (PR #60), head 4877599a9b85cfcc10b147a69e44a5276df5837e, base main 9b4b518

One question: is it proven? Every result below comes from a command I ran myself unless marked "by reading".
Scratch: .gate/kit-hold-ci/test-engineer/ (gitignored) and the scratchpad. I edit nothing in the repository,
never run `save`/`watch` of any copy in this working tree, never commit or push.

## Plan (blocks; a restart continues after the last complete block)
0. Memory: none for test-engineer (.claude/agent-memory-local/ has only security-auditor). Read the record, his
   three messages, ci-evidence.md, infra-engineer-review1.md (mechanics only, not redone).
1. Read the diff at 4877599 vs origin/main (.claude/ and .github/), the script's hold code and tests 28-33 whole.
2. A: run the test file at head; main's 27 tests against the head script; the new tests against main's script.
3. D: run the file 5 times; Node and git versions; read the tests for can't-fail assertions, unreached steps,
   order dependence, watch timing, temp folder, git version sensitivity.
4. B: broken copies from the head script, one small change each (the 15 listed + any I add); for each: which
   test fails and at which line, or SURVIVED.
5. C: every behaviour the comments and the rule line claim, mapped to a test or "no test"; severity each.
6. E: judge ci-evidence.md; optionally repeat the plant locally in .gate/test-engineer-plant (own target dir,
   --locked --offline; worktree removed with `git worktree remove`, no force).
7. F: numstat; routine-guard / settings / kept-logs counts at head and at main; gate --no-smoke.
8. Verdict, table, C list, flakes, F numbers, follow-up lines.

## Results
(appended as each block completes)

### Block 0-1 (done 18:08 UTC): setup and reading
- Memory: none for test-engineer. Remote branch head 6544c83; `git diff --name-only 4877599 origin/<branch>`: only
  `.checkpoints/` paths. Working tree `.claude/` and `.github/` equal 4877599 (`git diff --quiet 4877599`).
- Env: no GIT_DIR / GIT_WORK_TREE / GIT_INDEX_FILE (only GIT_CONFIG_KEY_0..2 = credential.interactive, two
  url.insteadOf), so the scripts' child git calls act only on their scratch repos.
- Versions: node v22.23.4, git 2.43.0, Linux 6.18.44; os.tmpdir() = /tmp.
- Copies (sha256): head.checkpoint.mjs fc5a4f02..., head.test.mjs 96e8e71b..., main.checkpoint.mjs 6bf8696d...,
  main.test.mjs 64783b21... (all in .gate/kit-hold-ci/test-engineer/).

### Block A (done 18:11 UTC)
- `node --test .claude/scripts/checkpoint.test.mjs` at head: 33 pass, 0 fail, 21.8 s.
- main's test file (433 lines, 27 tests) with CHECKPOINT_SCRIPT=head script: 27 pass, 0 fail.
- head's test file with CHECKPOINT_SCRIPT=main's script: 27 pass, 6 fail (#28-#33). Failing assertion lines:
  #28 :453 (status 0, not 1), #29 :510, #30 :535, #31 :550, #32 :587 (watch line), #33 :620 (step 2; step 1
  "not held when equal to main" passes trivially on main's script: it has no hold at all).
- The test file's only removed line vs main is the import line (`symlinkSync` added); the first 27 tests' bodies
  are untouched (+224 -1, the -1 is line 8's import).
- Verdict A: HOLDS.

### Block D, part 1 (done 18:17 UTC): five runs
- `node --test .claude/scripts/checkpoint.test.mjs` x5, one at a time, idle machine (4 cores, load 0.06):
  33/33 pass every run; walls 22.2, 22.6, 23.3, 23.9, 23.5 s. No flake.
- Watch tests' durations over the 5 runs (ms): #9 1500-1579, #10 352-390, #15 941-998, #16 790-831,
  #32 1006-1035. Deadlines are 15 000 ms: margin about 10x. #32 waits for "saved (locally|and pushed)" and then
  asserts the hold's line, so a slow machine fails it (never passes it falsely).
- Tools: runner.mjs (runs a test file against N script copies, writes TAP per run), failat.mjs (failing test +
  the assertion's line from the stack), build-mutants.mjs + mutants.txt (49 one-change copies of the head script,
  every find matched exactly once, all pass `node --check`).

### Block B (done 18:33 UTC): 49 broken copies of the head script, head test file, 3 runs in parallel
Built by build-mutants.mjs from head.checkpoint.mjs (fc5a4f02...), one find/replace each (mutants.txt), all
`node --check` clean. Each line: copy, what it breaks, then the failing tests with the failing assertion's line.
The brief's list:
- M01 hold removed: #28:453 #29:510 #30:535 #31:550 #32:587 #33:620 (6 fail)
- M02 hold skipped when background: #32:587
- M02b hold moved below the background branch: SURVIVED. Equivalent for publishing: when background && code
  ahead both versions skip the push; only the log line differs (the held files are not named). No gap.
- M03 record read from the working tree: #28:471 (step 3)
- M04 renames followed (--no-renames dropped): #31:565 #32:587
- M05 any record releases: #28:461 #29:510 · M05b path with any hash: #28:461 · M05d hash of the blob id:
  #28:462 #29:513 · M05c record matched as a substring: SURVIVED (no claim beyond the line's text; harmless)
- M06 names emptied: 6 fail · M06a Cargo.toml dropped: #28:454 #30:535 · M06b Cargo.lock dropped: #28:454
  #31:550 #32:587 #33:620 · M06c package.json: #29:510 · M06d package-lock.json: #29:510 #31:565 ·
  M06e npm-shrinkwrap.json: #29:510 · M06h root folder only: #28:454 #29:510 #33:632
- M07a "deleted" releases any existing entry (before the plain check): #31:550 (only through the link case)
- M07b "deleted" releases an existing PLAIN file: SURVIVED  <-- gap
- M08a link as plain (plain check only): #31:550 · M08b link as plain (both regexes): #31:550 #33:632
- M09 comparison with main dropped: #33:610 (step 1)
- M10 any plain file main also has is released: #33:620 (step 2)
- M11 comparison moved before the plain check (single move): SURVIVED, EQUIVALENT at head (blobAt re-checks the
  plain mode on both sides, so a link never matches). The session's broken-main-before-plain was cut from an older
  version where it bit. The double change M11b (moved + any mode): #33:632 (step 3).
- M12 main read as a pattern (main side only, push side literal): SURVIVED  <-- gap (new)
- M13 push side read as a pattern (inPush = plain[1]): #33:653 (step 4) · M13b both sides as pattern: #33:653
- M14 local main instead of origin/main: SURVIVED (known: advisor)
- M15 base always origin/main's merge-base: #29:513 #31:565 #32:587 #33:632
Beyond the list:
- M06f left anchor dropped, M06g right anchor dropped: SURVIVED (each alone; over-hold, safe direction; test 29's
  `not-package.json.txt` needs BOTH anchors gone to be held, so it pins neither alone)
- M06i notes folder excluded from the names: SURVIVED (a manifest created inside .checkpoints/ not held; "in any
  folder")
- M07c removed file held whatever the record: #29:523 · M07d removed released w/o record: #29:520 #31:565 #32:587 ·
  M07e removed released when main lacks it too (KIT-REMOVED): #29:520 #31:565 #32:587
- M11c comparison reads any mode/type on both sides (after plain check): SURVIVED (known: plain file vs link on main)
- M15b base always the remote branch: #30:536 · M15c origin/main tip instead of merge-base: SURVIVED (over-hold
  only)
- M16 names unreadable -> released: SURVIVED · M17 entry unreadable -> released: SURVIVED · M18 content unreadable
  -> released: SURVIVED (fail-closed paths; no test makes git fail there)
- M19 names read without -z: SURVIVED (a path git quotes, e.g. a non-ASCII folder, would be missed: "in any folder")
- M20 save exits 0 when held: #28:453 #29:510 #30:535 #31:550 #33:620
- M21 hold computed on HEAD before the notes commit: #28:462 #29:513 · M24 record without the slug: #28:462 #29:513
- M22 `inPush &&` dropped: SURVIVED (matters only for a ':' path that is a link while the pattern finds a plain
  file; contrived) · M23 trim dropped: SURVIVED (no claim) · M26 mode compared too: SURVIVED (known: mode-only)
Raw TAP: scratchpad B/<id>.txt.

### Block B2 / C evidence (done 18:41 UTC): are the survivors real? (extra.test.mjs, scratch, 10 tests)
- Old step 4 (test file at 94c197f, before the fix) vs head script: 33 pass; vs M12: 33 pass. So neither the old
  nor the new step 4 pins main's literal read at this head. The old step 4 bit on a main-side pattern read only
  while the push side was itself a pattern read (old code: plain[1]); fix2 made the push side literal and rewrote
  step 4 for the push side. Main's side has had no test since fix2.
- .gate/kit-hold-ci/test-engineer/extra.test.mjs (10 short tests, own fixture copy): head 10/10 pass; each
  broken copy fails exactly its own test and no other:
  X1 M07b "deleted" releases an existing plain lockfile · X2 M12 main read as a pattern (push adds `:/Cargo.lock`
  with the content of main's ROOT Cargo.lock; main has no ':' folder) · X3 M06i manifest inside .checkpoints/ ·
  X4 M19 non-ASCII folder `crates/ü/Cargo.toml` · X5 M16 no origin/main + unpushed branch ("could not be read") ·
  X6 M06f and M06g look-alikes `package.json.txt`, `old-package.json` · X7 M14 unpushed local main ·
  X8 M11c plain file with the bytes of main's link · X9 M26 mode-only change released (reading (b)) ·
  X10 M15c unpushed branch vs main's newer tip (over-hold).
- So M07b, M12, M06i, M19, M16, M06f/g, M14, M11c, M26, M15c are real gaps (not equivalent copies). M02b, M03b,
  M11 are equivalent; M05c, M22, M23 carry no claim; M17, M18 cannot be reached without making git fail.

### Block D, part 2 (done 18:47 UTC)
- Each new test alone (`--test-name-pattern`): 1/1 pass for each of #28-#33: no order dependence.
- Under load: 8 more full runs, 4 at a time on 4 cores: 33/33 each; slowest watch tests: #9 1612 ms, #32 1163 ms.
  Totals at head: 14 full runs, 0 failures; plus 49 broken-copy runs where every test outside the hold passed.
- Temp folder: 0 leftover /tmp/checkpoint-test-* after all runs, failing ones included.
- t.after order (after-order.test.mjs): FIFO. So in every watch test (#9, #15, #16, #32) the fixture's rmSync runs
  BEFORE child.kill: the watch child is alive while its repo is deleted. Never seen to fail (63 runs); the child's
  next git call is ~600 ms after the line the test waits for. Pre-existing pattern from main's tests; #32 copies it.
- No assertion that cannot fail found; every step reaches the hold (M03 kills at :471 = step 3 reaches the record
  read; M13 at :653 = step 4 reaches the push-side read). Weak spots: test 29's look-alike `not-package.json.txt`
  pins neither anchor alone; test 33 step 3's link case is guarded twice in the code (M11 equivalent); step 4 pins
  the push side only.
- The kit tests run nowhere in CI: neither workflow nor tools/gate.mjs runs any `.claude/**/*.test.mjs` (grep).
  All test evidence for the kit is local (node 22.23.4, git 2.43.0 here).

### Block F (done 18:52 UTC)
- `git diff --numstat origin/main -- .claude .github` at 4877599 (and equal for the working tree): 10+0
  CHECKPOINTS.md, 47+1 checkpoint.mjs, 224+1 checkpoint.test.mjs, 12+0 toolchain.yml = 293 added, 2 removed. MATCHES.
- routine-guard 1107/1107 pass; settings 127/127; kept-logs 10 tests: 9 pass, 1 skipped (#10 needs
  KEPT_LOGS_DRY_RUN), 0 fail. MATCH the record. Their inputs (.claude/hooks/**, settings.json, kept-logs.*) are
  byte-identical to main (`git diff --quiet origin/main 4877599 -- ...`) and none reads a changed file's content, so
  main's counts are the same by construction (not re-run in a main worktree).
- `node tools/gate.mjs --no-smoke`: GATE PASS (5a skipped). It reads legacy/ only: no evidence about this PR's files.
- `sha256sum` (GNU coreutils 9.4) prints byte for byte the "<hash>  <path>" lines the tests build in Node.
- Correction to F's first line (coordinator's note ~18:53: compare commits, not the working tree): re-run as
  `git diff --numstat origin/main 4877599 -- .claude .github`: 10+0, 47+1, 224+1, 12+0 = 293 added, 2 removed.
  The working tree equals 4877599 for .claude .github legacy tools (`git diff --quiet 4877599 -- ...`); the docs
  pass touches docs/ only, so every test and gate run above measured the head.

### Block E plan (local plant, started 18:57 UTC)
Worktree .gate/test-engineer-plant (detached at HEAD), CARGO_TARGET_DIR .gate/test-engineer-plant-target, every
cargo call `--locked --offline`; no commit: the plant is made in the worktree's own index (`git update-index
--chmod=+x`) plus the file's bit, which is what a checkout of the CI plant commit gives. Steps: (1) clean: plain
test, write mode, compare -> empty; (2) the plant: plain test passes, write mode, compare -> " M ...Join.ts";
(3) a content change: the plain test fails first (ci-evidence's "cannot be reached"); (4) restore, worktree
removed with `git worktree remove` (no force), my target folder deleted.
