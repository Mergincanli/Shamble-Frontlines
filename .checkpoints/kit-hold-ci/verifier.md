# Verifier notes: kit-hold-ci (PR #60)

Branch `claude/wonderful-hamilton-jdmlnf`, code head 4877599a9b85cfcc10b147a69e44a5276df5837e, base main 9b4b518.
Working-tree HEAD at start: 308286d (later commits touch only `.checkpoints/`, to be confirmed).
Scratch: `.gate/kit-hold-ci/verifier/` and the session scratchpad. I edit nothing else; I never run `save`/`watch`.

## Plan
1. Gate: `node tools/gate.mjs --no-smoke`; `node .claude/scripts/repo-map-check.mjs`.
2. Kit tests with counts: checkpoint.test (33), routine-guard.test (1107), settings.test (127), kept-logs.test (10/9/0);
   `git diff --stat origin/main -- .claude/hooks .claude/scripts/kept-logs*`.
3. Cross-run: new tests vs main's script (expect 27 pass, 6 fail = tests 28-33); main's tests vs new script (expect 27 pass).
4. Texts: sha256 of Mustafa's three files; kit-edit.diff at 3ebe654a (262 lines, sha 70756f3d...) applied to main = as-shown/;
   kit-change.diff = diff -u as-shown -> head, adds only, count 75 (6/67/2); kit-fix2.diff at f7e232fb (45 lines, sha 9197f070...)
   = git diff 415bd73 4877599 -- .claude/; toolchain.yml diff = report blocks 1-3; numstat 47/224/10/12 = 293/2.
5. Plant: `git diff 94c197f 415bd73 -- . ':!.checkpoints'` empty; generated files mode 100644 at HEAD.
6. Replay: replay-miss.sh with new script (exit 1, names 3 files, nothing pushed) and main's (exit 0, pushes).
7. Probe: probe-colon-folder.mjs on new script (held) and on 415bd73's (RELEASED).
8. CI claims: WebFetch public run/job pages; else consistency only.
9. Draft sentences: "What went wrong on the way", "What changed", "Evidence", "Not covered, and not verified" vs card, notes, git,
   Mustafa's three files word for word.

## Results (one block per check, written as each finishes)

### Check 1: gate and map (DONE, PASS)
- `node tools/gate.mjs --no-smoke` (working tree at 308286d; no uncommitted change outside `.checkpoints/`): GATE PASS, exit 0.
  [1] 5/5 parse · [2] both pages concatenate · [3] core 138 / game 714 / editor 353, +0 -0, duplicates 0/0 · [4] 51 refs,
  51 guarded, 0 dead, 0 flagged · [5a] SKIP · [5b] 0 problems both pages · [5c] 0 references up · [5d] order OK. Output:
  `.gate/kit-hold-ci/verifier/gate.out`.
- Smoke cannot start here, confirmed: the full gate gives "[5a] smoke FAIL Chromium for Playwright could not start:
  Executable doesn't exist at /opt/pw-browsers/chromium_headless_shell-1194/..."; tools/node_modules playwright-core
  1.56.1; /opt/pw-browsers holds chromium-1243 and chromium_headless_shell-1243 only. CI's `smoke` job is the check.
- `node .claude/scripts/repo-map-check.mjs`: "repo map OK: 139 entries cover 109 required paths", exit 0.
- `git diff --stat 4877599 HEAD -- . ':!.checkpoints'`: empty (later commits touch only `.checkpoints/`).

### Check 2: the kit's tests (DONE, PASS)
- `node --test .claude/scripts/checkpoint.test.mjs`: tests 33, pass 33, fail 0, exit 0 (22.7 s). `.gate/kit-hold-ci/verifier/t-checkpoint.out`
- `node --test .claude/hooks/routine-guard.test.mjs`: tests 1107, pass 1107, fail 0, exit 0.
- `node --test .claude/hooks/settings.test.mjs`: tests 127, pass 127, fail 0, exit 0.
- `node --test .claude/scripts/kept-logs.test.mjs`: tests 10, pass 9, fail 0, skipped 1 (test 10 "dry run on a copy of a
  real run-set folder", SKIP: needs KEPT_LOGS_DRY_RUN), exit 0.
- `git diff --stat origin/main -- .claude/hooks .claude/scripts/kept-logs*`: empty (both test files and what they test
  are main's, so main's counts are the same files' counts).

### Check 3: cross-runs (DONE, PASS)
- Scratch copies by `git show origin/main:...` in `.gate/kit-hold-ci/verifier/main/`; `git hash-object` equals main's blob
  ids (checkpoint.mjs ef8ca2be, checkpoint.test.mjs d82e87e8, CHECKPOINTS.md 552ec517). Both test files honour
  CHECKPOINT_SCRIPT (new :13, main :12).
- New tests vs main's script: tests 33, pass 27, fail 6, exit 1; the fails are exactly 28 to 33; each fails where main's
  script pushes: 28, 29, 30, 31, 33 "expected: 1 actual: 0" (exit), 32 "did not match /saved locally, NOT pushed .* no
  recorded check: Cargo\.lock\. /". `.gate/kit-hold-ci/verifier/x-newtests-mainscript.out`
- Main's tests vs the new script: tests 27, pass 27, fail 0, exit 0. `x-maintests-newscript.out`
- Extra, the draft's "Main" column: main's tests vs main's script: 27 pass, 0 fail. `x-maintests-mainscript.out`

### Check 4: the texts (DONE, PASS)
- His three files (`.gate/kit-hold-ci/verifier/words-sha.mjs`: text after the "---" line, minus the file's last newline):
  1624 merge-58: 2618 bytes, sha256 6979627f...6508220 = header; 1730 kit-text: 1542 bytes, 1f3b95b4...04e227e7 = header;
  1852 kit-fix: 779 bytes, 9c9815fa...40607316 = header. Extra: the 2618-byte text stands byte for byte in merge commit
  9b4b518's message (offset 308 of 3280 bytes; `git log -1 --format=%B 9b4b518`).
- `git show 3ebe654a:.checkpoints/kit-hold-ci/kit-edit.diff`: 262 lines, sha256 70756f3d...2026ce5 (= his 16:39 message;
  = notes' `kit-edit-as-shown.diff`). Split per file (1-74 script, 75-91 rules, 92-262 tests; concatenation cmp-equal),
  `patch -o` on main's three files (git show origin/main): no fuzz, no offset; all three cmp-EQUAL to `.gate/kit-hold-ci/as-shown/`.
- `kit-change.diff` (113 lines, sha256 ca24f0f9..., committed = working copy): byte for byte what `diff -u --label ...`
  gives from as-shown/ to the head's three kit files (head 4877599 files = working tree). Adds only: 0 removed lines;
  75 added: script 6, tests 67, rules 2 (`git diff --no-index --numstat` agrees: 6/0, 67/0, 2/0). The added lines: his
  rule (2 header comment lines, 1 function comment line, 3 code lines reading both sides with --literal-pathspecs),
  the rule line in CHECKPOINTS.md (2 lines), test 33 (67 lines, 4 steps).
- `git show f7e232fb:.checkpoints/kit-hold-ci/kit-fix2.diff`: 45 lines, sha256 9197f070...a7f9 (= his 18:01 message).
  `git diff 415bd73 4877599 -- .claude/`: normalized (file headers and git's @@ function context dropped) both are 45
  lines and `diff` finds no difference: 2 hunks (@@ -130,8 +130,9 @@ script; @@ -632,21 +632,22 @@ tests), every line
  equal. `.gate/kit-hold-ci/fix2/` = head for both files.
- Workflow: `.gate/kit-hold-ci/verifier/ci-text.mjs` rebuilds the file from main's toolchain.yml and the report at
  1bfe462 (= main's copy), section "The exact texts asked for" (4 code blocks; block 4 = part 2, unused): block 3 after
  the one `timeout-minutes: 15` (inside job `toolchain:`), block 1 after the one `cargo test --workspace --locked`,
  block 2 in place of the one `npx --no -- tsc --version` (the script's last line; block 2 starts with that same line).
  Rebuilt 5665 bytes = head 5665 bytes: EQUAL. New lines 2 + 3 + 7 = 12.
- Nothing else: `git diff --stat origin/main HEAD -- . ':!.checkpoints'` (HEAD 308286d, again at 2d1fa70): exactly the
  four files; numstat 10/0 CHECKPOINTS.md, 47/1 checkpoint.mjs, 224/1 checkpoint.test.mjs, 12/0 toolchain.yml = 293
  added, 2 removed; same at 4877599 and with the draft's command (`-- .claude .github`).

### Check 5: the plant leaves no trace (DONE, PASS)
- `git diff 94c197f 415bd73 -- . ':!.checkpoints'`: empty; even the whole trees are equal (94c197f^{tree} = 415bd73^{tree} =
  43b0e2e2). 0c0e271 (parent 94c197f): only `:100644 100755 692b886 692b886 M packages/types/src/generated/protocol/Join.ts`;
  415bd73 (parent 0c0e271): the reverse, same blob.
- Modes: `git ls-files -s packages/types/src/generated`: 9 files, all 100644; `git ls-tree -r` at HEAD (2d1fa70) and at
  4877599: 9 x 100644; no executable file on disk.

### Check 6: the replay of change-set 1's miss (DONE, PASS)
- Read first: `replay-miss.sh` clones the repo twice into the scratchpad (`.../scratchpad/replay/run-<name>-XXXX`), points
  the clone's origin at the scratch bare (`git remote get-url --push origin` = the scratch origin.git), sets the remote
  branch to 55397c8 and main to 22c6afa, switches to 566d1d1, restores da22f96's notes, runs one foreground save.
  The script writes only a temp index in the current repo's git dir (checkpoint.mjs:189). Premise checked: 566d1d1's
  parent is 55397c8 and it changes exactly Cargo.lock, Cargo.toml, crates/protocol/Cargo.toml; cs1 report :18-21 says
  566d1d1 and the checkpoint commit da22f96 on top of it were pushed together at 12:04:36.
- This head's script (18:24:15-18:24:19 UTC): "[final] exit 1; remote branch unchanged (nothing pushed)"; "checkpoint:
  saved locally, NOT pushed ce02d34 on claude/wonderful-hamilton-jdmlnf: the push would publish a dependency change with
  no recorded check: Cargo.lock, Cargo.toml, crates/protocol/Cargo.toml. ..."; scratch remote branch still 55397c8,
  566d1d1 not on it.
- Main's script (scratch copy, blob ef8ca2be = origin/main's; 18:24:25-18:24:28): "[main] exit 0; remote branch MOVED to
  b824ed8 (pushed)"; "checkpoint: saved and pushed b824ed8"; 566d1d1 is now on the scratch remote branch (published).
- Matches `replay-miss-results.txt` (only the new commits' SHAs differ: commit times).

### Check 7: the hole fixed by KIT-FIX (DONE, PASS)
- Probe read first: builds its own scratch repo + bare origin under the system temp folder, deletes it after.
- This head's script: "exit 1; held (not pushed); checkpoint: saved locally, NOT pushed ... no record...".
- `git show 415bd73:.claude/scripts/checkpoint.mjs` (scratch `.gate/kit-hold-ci/verifier/s415/`, blob fae135bb = the same
  blob at 94c197f): "exit 0; RELEASED (pushed); checkpoint: saved and pushed ...".
- Extra: the text he answered on (`as-shown/checkpoint.mjs`): "exit 1; held". So the hole existed only in 8bf4bee..415bd73.

### Finding (for check 9), probe of my own
- Draft "Fail-safe corners": "A true record for a path that begins with ":" never matches (...), so such a file stays
  held." Probe `.gate/kit-hold-ci/verifier/probe-true-record.mjs` on the head script (scratch repo in the temp folder):
  A (root Cargo.lock and ":/Cargo.lock" both "X\n", the true record names both): "exit 0; RELEASED (pushed)";
  B (root "Y\n"): "exit 1; held ... :/Cargo.lock". So a true record DOES match when the path git reads in its place
  (checkpoint.mjs:128, the pattern read) holds the same bytes; it also matches a true "deleted" line when that path is
  absent. Safe direction holds (only the recorded bytes pass). Low, wording.

### Check 8: the CI claims (DONE; pages checked, log lines NOT checked by me)
- WebFetch of the public pages (github.com; api.github.com refused: HTTP 403). Logs need sign-in on every page.
  | run (job) | page shows | claim |
  | 44357176860 (134988309631) | toolchain #184, push 17:18, 94c197f, success, run 55 s, job 51 s | 17:18:27-17:19:18 = 51 s: OK |
  | 44357282401 | toolchain #185, pull_request (PR #60 opened) 17:18, 94c197f, success, 45 s | 17:18:56-17:19:41 = 45 s: OK |
  | 44357616602 (134989872637) | toolchain #186, 0c0e271, failed in 38 s; annotation "Build and test with the network cut off": "Process completed with exit code 1." | failure at that step: OK |
  | 44357633651 (134989932210) | toolchain #187 (PR title), 0c0e271, failed in 33 s; same annotation | step 7 failure: OK (step 7 = Set up job, checkout, setup-node, 3 named steps, then this one) |
  | 44357864275 | toolchain, push 17:22, 415bd73, success, 44 s | 17:22:01-17:22:45: OK |
  | 44357881263 | toolchain #189, pull_request synchronize #60, 415bd73, success, 44 s | 17:22:06-17:22:50: OK |
  | 44357864589 | gate, push 17:22, 415bd73, success, 1 m 9 s (gate 22 s, smoke 50 s) | to 17:23:10: OK |
  | 44357881162 | gate, pull_request synchronize #60, 415bd73, success, 9 m 21 s (smoke 9 m 18 s) | to 17:31:27: OK |
  | 44365431438 | toolchain #190, push 18:03, 4877599, success, 39 s | 18:03:14-18:03:53: OK |
  | 44365446199 | toolchain #191, pull_request synchronize #60, 4877599, success, 42 s | 18:03:18-18:04:00: OK |
  | 44365431235 | gate #892, push 18:03, 4877599, success, 6 m 15 s (smoke 6 m 11 s) | 18:03:14-18:09:29: OK |
  | 44365445961 | gate #893, pull_request synchronize #60, 4877599, success, 55 s | 18:03:18-18:04:13: OK |
- Commits vs runs (`git log --format='%h %ci %s' origin/main..HEAD -- . ':!.checkpoints'`): 8bf4bee and 94c197f 17:18:10,
  0c0e271 17:20:38, 415bd73 17:21:54, 4877599 18:03:10: every run starts after its commit; the take-back commit (17:21:54)
  is after both plant jobs' last times (17:21:20 push log, 17:21:24 PR job end). Consistent.
- Step 7's inner script runs under `bash --norc --noprofile -euo pipefail -c` (toolchain.yml), so the clean runs'
  success alone proves `git status` and both type-check lines exit 0 inside the cut.
- NOT checked by me (sign-in): the quoted log lines ("Cache mode: none" at 17:18:27.9099464Z; "writing the generated
  types again changed:" and " M packages/types/src/generated/protocol/Join.ts" at 17:21:20; the 581-line count; the
  absence of cache lines). They are consistent with the visible annotations and durations.
- Draft table overclaim (Low): row 0c0e271 puts the push AND the pull-request run under "failed at the compare step:
  <the two log lines> (17:21:20)". The record (card step 16, ci-evidence §2) quotes those lines from the push job only;
  for the pull-request job it records only "step 7 ... failure, 17:21:17 to 17:21:21".
