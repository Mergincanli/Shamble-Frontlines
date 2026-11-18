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
