# Verifier notes: CI-WASM pull request #64 (head 6d0b5b12945f3ca6b66bdb23036b6c7f90fc9299)

Started 2026-11-23T21:46:03Z (clock). Worktree `.gate/ci-wasm-wt` (local `cs/ci-wasm`), base origin/main f5030a3.
Scratch: `.gate/ci-wasm/verifier/` (under the worktree). Main-folder never switched; no git -C/-c; GET-only gh api.
Times below: from the clock where stated; else bounds from the scratch files' own times (`ls --time-style`), read at
22:06:26Z. Correction (22:06:54Z, clock): the first stamps written from C3 on were estimates, not clock reads
(21:58Z to 22:30Z, partly after the real time); they are replaced by these bounds.

## Plan (one block per claim; a line after each block)
1. Gate: `node tools/gate.mjs --no-smoke` on head; `git diff --stat origin/main HEAD -- legacy tools` empty;
   GitHub gate/smoke/toolchain on 6d0b5b1, push + pull_request (poll until complete).
2. Workflow sha256 at head; main's file + ci-edit.diff + ci-edit-2.diff rebuilt in scratch (patch), cmp; the three
   diffs' sha256; each CI commit's file list and resulting file sha256; P-GH planted file = 2a654f3 + ci-plant-p-gh.diff;
   `git log origin/main..HEAD -- .github` = the four; `.claude/`, every CLAUDE.md, `tools/` equal main's.
3. His messages files vs the transcript, byte for byte (typed + queued commands); permissionMode on 17:15:07,
   17:22:42, 19:58:23, 20:47:59; commit times vs yes times; `node .checkpoints/ci-wasm/decisions-3.mjs --check`.
4. stop-rule-check.py on toolchain logs of 2a654f3, d6e4fb6, ce5a7e7, 6d0b5b1 (push + pr each); two logs by hand vs
   plan section 9; quote GitHub's check output from one log.
5. cb476b1's two runs: the /opt line; remote-tracking reflog + commit times 17:25 to 18:40:57.
6. Plants P-BROWSER, P-GH, P-BOUNDARY: each fails only at its own step with its expected line; take-backs; tree vs
   280f31c outside .checkpoints/docs; vs 2a654f3 only 280f31c's two files.
7. Kit test files on a main worktree (`.gate/verifier-main`) and on head: same counts.
8. build-checks.mjs pattern: harness on the real module (refused / allowed names, the cut's env -i list); CI log lines.
9. THE TABLE: ten rows vs main's and head's file; numstat +97/-0.
10. Size: numstat outside docs and .checkpoints vs 155 planned / 310 stop.

## Log
- ~21:48Z (file times) C1 PASS. `git diff --stat origin/main HEAD -- legacy tools` empty; `.claude`, `tools`,
  CLAUDE.md: no diff. Local gate: no tools/node_modules on this container, so CI's own install line was mirrored into
  scratch (`npm ci --prefix .gate/ci-wasm/verifier/tools-deps --omit=optional --ignore-scripts`, lockfile sha256
  8bfdf372...61f7 equal to tools/'s) and run with NODE_PATH (21:47:29-30Z, clock): "GATE PASS (5a smoke skipped:
  --no-smoke)", exit 0, checks 1-4, 5b-5d PASS (symbols +0 -0). GitHub on 6d0b5b1 (read 21:47:37Z, clock): runs
  45272654640 gate/push (jobs gate 138036214374, smoke 138036213902), 45272669452 gate/pr (gate 138036262016, smoke
  138036261648), 45272654783 toolchain/push (job 138036214520), 45272669543 toolchain/pr (job 138036262546): all
  completed success; check-runs API: 6 of 6 success. PR #64 open, draft, head 6d0b5b1, base main f5030a3; remote
  branch = 6d0b5b1. Head logs fetched to verifier/logs/ (push 8f42622c...5567, pr d18f83ed...fd3e).
  Added ~22:05Z: CI's own logs: gate push (base 8d2ce32) and pr (base origin/main) "GATE PASS (5a smoke skipped:
  --no-smoke)"; smoke push and pr "[5a] smoke PASS index.html 8/8 steps, 0 errors ... editor.html 5/5 steps, 0 errors".
- 21:49-21:52Z (file times; this line is in 1564cd0) C2 PASS. Head file (worktree and `git show HEAD:` while HEAD was
  6d0b5b1) sha256 56d37fc6...9811, mode 100644 (blob b5bb7cb); diffs: ci-edit.diff 240c7db7...2673, ci-edit-2.diff
  b4081e48...03ec, ci-plant-p-gh.diff e4f21ab3...b1ac (worktree and HEAD). Rebuild in scratch: main's file +
  ci-edit.diff (`patch -F0`, exit 0, no fuzz/offset) = d1dbcb53...975c; + ci-edit-2.diff = 56d37fc6...9811; `cmp` with
  head file: equal. Commits: cb476b1 M toolchain.yml only, 93/0, file d1dbcb53; 2a654f3 only, 10/6, 56d37fc6; 47d7024
  only, 2/0, 84b6b3fd (= 2a654f3's file + ci-plant-p-gh.diff, rebuilt and cmp-equal; the two lines printf
  planted.tar.gz + verify-asset on it); ce5a7e7 only, 0/2, 56d37fc6. `git rev-list --full-history origin/main..HEAD
  -- .github` = 4, no merges in range; `.claude`, `tools`, CLAUDE.md (the only one) equal main's; gate.yml blob
  1f3f0509 unchanged.
- 21:52-21:54Z (file times) C3 PASS (for the 8 files from this chat; the 7 thread files: not verifiable here). Own
  script `verifier/words-compare.mjs` (by uuid, body after "\n---\n" minus the final "\n"): 8 of 8 byte-equal with the
  transcript entry, header bytes and sha256 equal: 163046 typed auto 4353; 163316 queued 120; 171507 typed **auto**
  1385; 172242 typed **default** 75; 184057 typed default 1324; 195823 typed **default** 884; 204759 typed **default**
  1006; 212459 queued 744. Queue-operation entries: every human enqueue is one of these 8 (no other message of his).
  The 7 thread files (142503..151101): self-consistent with their own headers, last changed at cc78f39 (15:12Z, before
  this chat). Writes, from the transcript's own tool calls (`verifier/github-writes.mjs`; all 64 calls naming .github
  read: only 4 cp's write there; no Write/Edit targets .github): cp at 17:23:24.626 (result 17:23:43.732) after yes
  17:22:42.519; 19:58:54.759 (20:00:41.601) after 19:58:23.131; 20:48:26.695 (20:48:40.739) after 20:47:59.800;
  21:01:13.963 (21:01:33.525) take-back, his yes in advance 20:47:59. Commits: cb476b1 17:24:14, 2a654f3 20:00:55,
  47d7024 20:48:50, ce5a7e7 21:01:41: each after its yes; nothing written at 17:15:07 (auto). Guard: each cp got a
  PreToolUse "ask" and the result's permissionDecision {accept, user_temporary, once} (write 1 at 17:23:43.732Z: the
  card is right; the main session's 21:49:27 one-off script printed 18:41:12 config/cliArg for write 1, a false match
  on its own 18:41:12 search command: Low, not in the report template line 77). `decisions-3.mjs --check`: every line
  yes, exit 0; DECISIONS sha256 b9190191...b857 and `git status` unchanged. The report draft names `default`
  (report-template.md:72-74), as his rule of 17:15:07 asks.
- 21:54-22:00Z (file times) C4 (script part). 36 runs on the branch, all attempt 1 (no re-run); failures only cb476b1
  (stop), 45a6f0e, 47d7024, 8d2ce32 (plants). Job ids -> commits by the API: 2a654f3 137974047720/137974099620, d6e4fb6
  137981826960/137981878828, ce5a7e7 138010909634/138010956116, head 138036214520/138036262546. My own re-fetch of 14
  logs == the pre-fetched copies byte for byte. `python3 -B -I .gate/ci-wasm/stop-rule-check.py` (sha256
  bdf8a3af...e253, = the copy committed in 1564cd0) on the 8 clean logs: ALL, 18 of 18 yes each; jobs
  146/138/138/135/138/142/147/135 s; gh 2.104.0; Chrome 157.0.8103.78; size tables 4; install lines exact; Wasm 28991
  bytes sha256 22dad2eb...018b on all 8.
- 21:54-22:00Z C4 PASS. Read by hand against plan s9's sentence: head push 138036214520 and 2a654f3 push 137974047720
  (same line numbers in both): 249 final address release-assets.githubusercontent.com; 250 "size: 11860397"; 251
  archive ": OK"; 270 gh 2.104.0; 276 success line naming release 0.2.130; 310-316 seven members (folder + 6);
  308 install step env "WB_TOOL: /wasm-bindgen-0.2.130/wasm-bindgen"; 317-320 the four install lines in order, nothing
  between ("/" = "mode 755, owner root:root, 4096 bytes, /"); 437/442 uid=1001(runner), "the network is cut: no TCP
  connection to 140.82.114.3:443 (curl exit 7)" (2a654f3: 48.200.82.112); 4 size tables 835-862; 880 "Tests 7 passed
  (7)" between the boundary echo 873 and the type-check echo 884; 884->885 nothing printed, and the cut runs
  `bash -euo pipefail`, so the five tsc lines exited 0; 887 "Google Chrome 157.0.8103.78"; 889 "Running 12 tests using
  1 worker"; 12 ticks; 915 "12 passed (1.1m)"; 0 failed/flaky/retry/x-mark/##[error] lines; 147 s and 146 s; image
  ubuntu-24.04 20261116.133.1; token "Contents: read, Metadata: read". Recorded only: backend "WebGL 2 fallback"
  (SwiftShader) on the default run; Wasm 28991 bytes 22dad2eb...018b.
  GitHub's check, whole (head push log lines 270-276; `gh --version` then `gh release verify-asset`):
    gh version 2.104.0 (2026-11-12)
    https://github.com/cli/cli/releases/tag/v2.104.0
    Calculated digest for wasm-bindgen-0.2.130-x86_64-unknown-linux-musl.tar.gz: sha256:9687f3bb551b9f66f37ed32a40d7913a00f7962db653054e56857bc306c3227f
    Resolved tag 0.2.130 to sha1:c93b8ae848d4d9e6beaeaf9516e18a7cadd818a2
    Loaded attestation from GitHub API
    (empty line)
    ✓ Verification succeeded! wasm-bindgen-0.2.130-x86_64-unknown-linux-musl.tar.gz is present in release 0.2.130
- 21:54-22:00Z C5 PASS. cb476b1 push job 137877549845 (image 20261116.376.1, lines 313-317) and pull request job
  137877597017 (20261116.133.1, lines 325-329), both in step "The tool's one program": "mode 555 ... /opt/wasm-
  bindgen-0.2.130/wasm-bindgen", "mode 755 ... /opt/wasm-bindgen-0.2.130", "mode 777, owner root:root, 4096 bytes,
  /opt", "/opt/wasm-bindgen-0.2.130 or /opt is not owned by root, or others than root may write in it",
  "##[error]Process completed with exit code 1."; jobs done 17:25:24Z / 17:25:33Z. Pushes: local reflog of
  origin/claude/loving-hopper-6p0mg2 and GitHub's activity API agree: 17:24:58 8dfa2f2->cb476b1, next 18:42:48
  cb476b1->d4b7e3b (after 18:40:57.810); 5a0a852 (17:25:32) and 7a46f88 (17:28:29) went up only then. All 22 pushes
  plain "push" (no force_push), by Mergincanli; no push while a run of the previous head was going (no run cancelled).
- 21:54-22:00Z NOTE (moved head): during this check the main session committed 1564cd0 (21:52:09, notes only,
  [skip ci]) in this worktree and pushed it (21:52:13): PR #64's head is now 1564cd0, with 0 workflow runs; outside
  `.checkpoints/` it equals 6d0b5b1. Checks from here use the explicit sha 6d0b5b1, not HEAD.
- 21:54-22:00Z C6 PASS. P-BROWSER 45a6f0e (render-counts.json "WebGL 2 fallback" drawCalls 3->4, one line) -> d6e4fb6
  (blob 011b4ae back; tree outside .checkpoints = 2a654f3's): jobs 137977706424 / 137977761665 fail only at step 10
  (API), after install lines, the cut, "Tests 7 passed (7)" (boundary) and Chrome; tests 8 and 9 x, "Error:
  expect(received).toEqual(expected)", '-   "drawCalls": 4,' / '+   "drawCalls": 3,', "2 failed", "10 passed
  (1.1m)"; 0 retry lines. P-GH 47d7024 -> ce5a7e7 (blob b5bb7cb back): jobs 138003254035 / 138003287607: step 7
  "GitHub's own check of the archive (gh release verify-asset)" failure, steps 8, 9, 10 skipped (API); between "gh
  version 2.104.0" and "##[error]" only "attestation for 0.2.130 does not contain subject sha256:27c24e42...6e10";
  `printf 'not an asset of the release\n' | sha256sum` = 27c24e42e238...6e10 (computed here). P-BOUNDARY 8d2ce32
  (sim-wasm.test.mjs:31 toBe(30)->31) -> 6d0b5b1 (blob ded0fb2c back, = 2a654f3's): jobs 138029769180 / 138029828871
  fail only at step 10: "AssertionError: expected 30 to be 31", "Tests  1 failed | 6 passed (7)", then ##[error] and
  nothing from the script after it. Origin: test-engineer review 1 (Medium, B9); the report draft names it as beyond
  the plan. Trees: `git diff --stat 280f31c 6d0b5b1 -- . ':(exclude).checkpoints' ':(exclude)docs'` empty; vs 2a654f3
  only build-checks.mjs (4 +-) and build-checks.test.mjs (+1), diff sha256 7a851fac... = 280f31c's own diff.
- 22:01-22:02Z (file times) C7 PASS. `git worktree add --detach .gate/verifier-main origin/main` (f5030a3; left in
  place). `node --test` per file (22:00:03-22:01:08Z, clock), TMPDIR = the session scratchpad for both trees (tests
  write only in mkdtemp folders): main routine-guard 1107/1107 pass, settings 127/127, checkpoint 33/33, fail 0; head
  (worktree at 1564cd0, `.claude/` == 6d0b5b1's) 1107/1107, 127/127, 33/33, fail 0; 0 "not ok" lines in all six
  outputs (verifier/kit-*.txt).
- 22:01-22:02Z C8 PASS. 280f31c: pattern + RUSTC_BOOTSTRAP, CARGO_INCREMENTAL, RESOLVER in the CARGO_(...)_.* family;
  the test's refused list + the 3 names. Harness `verifier/build-checks-harness.mjs` on a byte copy of 6d0b5b1's module
  (624d2f12...9062) and workflow (56d37fc6): 31 of 31 ok: the 3 refused alone and together; CARGO_TARGET_DIR,
  CARGO_BUILD_TARGET_DIR pass; the 11 names of the cut's env -i list read from workflow lines 142-145 (HOME PATH USER
  CI LANG PROBE_IP CARGO_HOME RUSTUP_HOME RUSTUP_AUTO_INSTALL npm_config_update_notifier SHAMBLE_WASM_BINDGEN) pass;
  every probe refused by the old pattern still refused; new refusals among probes only CARGO_RESOLVER_*; lookalikes
  (XCARGO_INCREMENTAL, CARGO_INCREMENTALX, RUSTC_BOOTSTRAPX, CARGO_RESOLVER, ...) pass. Runner: head push job
  138036214520 checked out 6d0b5b1 (git log -1), pr job d667882 = merge of 6d0b5b1 into f5030a3; "✓
  scripts/build-checks.test.mjs (18 tests) 36ms", "Tests  49 passed (49)". (18 and 49 equal 2a654f3's counts: the fix
  extends one test's list, it adds no test.)
- 22:02-22:05Z (file times) C9 PASS. `git diff --numstat origin/main 6d0b5b1 -- .github/workflows/toolchain.yml` = 97
  0; main's file (8b581adf...e016, 100 lines) is an ordered subsequence of the final (56d37fc6...9811, 197 lines). THE
  TABLE (infra-engineer-review-1.md:102-170) spot-checked by `verifier/table-spot.mjs`, 11 rows + 3 settings rows, all
  hold: unchanged byte for byte #2 (4-6/4-6), #4 (25/21), #10 (66/50), #30 (127/57), #35 (151-153/80-82), #36
  (158/87), timeout (35/31), permissions (17-18/13-14); new, at the stated final line and absent from main: #7 (42),
  #17 (91-92), #20 (100), #26 (117), #28 (123); env -i (142-145 vs 72-74): 10 names -> 11, extra
  SHAMBLE_WASM_BINDGEN only, none lost. Hunks: -6 +6,6 (comments); -33 +37,14 (WB_* env); -56 +72,56 (three steps
  before "Fetch every dependency"); -74 +144,3 (the env name); -99 +170,28 (in-cut lines appended after main's last
  tsc line): none inside an existing || chain.
- ~22:05Z (file times) C10 PASS. `git diff --numstat origin/main 6d0b5b1 -- . ':(exclude)docs' ':(exclude).checkpoints'`:
  191 added, 17 removed, 11 files (toolchain.yml 97, .gitignore 1, build-checks.mjs 15, build-checks.test.mjs 21,
  build.mjs 2/11, node-names.d.ts 16, playwright.config.ts 4, allocation.spec.ts 4/1, clock.spec.ts 2/1, report.ts 9,
  skeleton.spec.ts 20/4); the same at 1564cd0. Plan: about 155, stop line 310: 191 is +36 (23 %) over the estimate,
  under the stop line; = revision 3's 186 + 4 (ci-edit-2 net) + 1 (280f31c's test line). No manifest or lockfile in
  the range. Docs: 1131 added (DECISIONS 517, VISION_V2 4, the browser-check record 610), counted apart by the plan.
- ~22:05Z FINDING (symbols vs plan, Medium): legacy battery +0 -0 = plan. v2 exported symbols added: CARGO_OVERRIDES
  (moved from build.mjs), refusedBuildSettings(env), node-names.d.ts's Node names: all in plan s6; **drawn(page,
  frames)** in packages/e2e/tests/skeleton/report.ts (+9, 3cddb55): in neither plan s4's file table nor s6, and
  test-engineer-followups.md:279 said "no change to report.ts's interface". Also playwright.config.ts timeout 90_000
  vs the plan's 60_000. Already infra-engineer's F2 (Low) and listed in the report draft (report-template.md:49-50).
  The verifier rule calls an unplanned addition a FAIL; the battery's lists (+0 -0) do not cover v2 code, so it goes
  to the advisor agent to accept or to have the plan record it, before Ready.

## Verdict (22:08Z, clock)
PASS WITH NOTES. C1-C10 all PASS on 6d0b5b1. Notes: Medium, drawn() in report.ts and the 90 s timeout are not in the
plan (the advisor agent to accept or have the plan record it, before Ready); Low, the PR head moved to 1564cd0
([skip ci], 0 runs): the head that goes to Ready and Merge needs its own green gate, smoke and toolchain runs; Low, the
main session's 21:49:27 one-off output for write 1's guard answer is a false match (the card and the report draft are
right). Not verified: the 7 project-thread message files against their transcript (not on this container); vitest and
Playwright suites run here (read from runner logs only); who answered the guard's four prompts (the record says
user_temporary, once); how the tool's program was built; Firefox, WebKit, WebGPU, a real GPU, Brave.
DONE
