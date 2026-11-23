# Verifier notes: CI-WASM pull request #64 (head 6d0b5b12945f3ca6b66bdb23036b6c7f90fc9299)

Started 2026-11-23T21:46:03Z (clock). Worktree `.gate/ci-wasm-wt` (local `cs/ci-wasm`), base origin/main f5030a3.
Scratch: `.gate/ci-wasm/verifier/` (under the worktree). Main-folder never switched; no git -C/-c; GET-only gh api.

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
- 21:48Z C1 PASS. `git diff --stat origin/main HEAD -- legacy tools` empty; `.claude`, `tools`, CLAUDE.md: no diff.
  Local gate: no tools/node_modules on this container, so CI's own install line was mirrored into scratch
  (`npm ci --prefix .gate/ci-wasm/verifier/tools-deps --omit=optional --ignore-scripts`, lockfile sha256 8bfdf372...61f7
  equal to tools/'s) and run with NODE_PATH: "GATE PASS (5a smoke skipped: --no-smoke)", exit 0, checks 1-4, 5b-5d
  PASS (symbols +0 -0). GitHub on 6d0b5b1 (read 21:47:37Z): runs 45272654640 gate/push (jobs gate 138036214374,
  smoke 138036213902), 45272669452 gate/pr (gate 138036262016, smoke 138036261648), 45272654783 toolchain/push (job
  138036214520), 45272669543 toolchain/pr (job 138036262546): all completed success; check-runs API: 6 of 6 success.
  PR #64 open, draft, head 6d0b5b1, base main f5030a3; remote branch = 6d0b5b1. Head logs fetched to
  verifier/logs/ (push 8f42622c...5567, pr d18f83ed...fd3e).
- 21:50Z C2 PASS. Head file (worktree and `git show HEAD:`) sha256 56d37fc6...9811, mode 100644 (blob b5bb7cb);
  diffs: ci-edit.diff 240c7db7...2673, ci-edit-2.diff b4081e48...03ec, ci-plant-p-gh.diff e4f21ab3...b1ac (worktree
  and HEAD). Rebuild in scratch: main's file + ci-edit.diff (`patch -F0`, exit 0, no fuzz/offset) = d1dbcb53...975c;
  + ci-edit-2.diff = 56d37fc6...9811; `cmp` with head file: equal. Commits: cb476b1 M toolchain.yml only, 93/0, file
  d1dbcb53; 2a654f3 only, 10/6, 56d37fc6; 47d7024 only, 2/0, 84b6b3fd (= 2a654f3's file + ci-plant-p-gh.diff, rebuilt
  and cmp-equal; the two lines printf planted.tar.gz + verify-asset on it); ce5a7e7 only, 0/2, 56d37fc6.
  `git rev-list --full-history origin/main..HEAD -- .github` = 4, no merges in range; `.claude`, `tools`, CLAUDE.md
  (the only one) equal main's; gate.yml blob 1f3f0509 unchanged.
