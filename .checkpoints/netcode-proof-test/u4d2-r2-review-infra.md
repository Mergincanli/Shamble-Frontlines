# U4d2 review round 2: infra-engineer seat (notes)

Started 2026-11-13. Fresh start (no notes file, no r2-inf-* logs existed). No agent memory exists for this seat.
Copy under review: /home/user/Shamble-Frontlines/.claude/worktrees/u4d2-build (uncommitted on 51520b4, after the fix
pass); read copy .gate/u4d2-r2; round 1's tree .gate/u4d2-r1. Runner: .checkpoints/netcode-proof-test/u4d2-run.sh; log
prefix r2-inf-. I edit no copy and plant nothing. This file is rewritten whole with Write after each finding.

## Plan
1. Check the read copy's hashes (fx-hashes-end.sha256: 18 of 18) and the build copy's status; read the runner.
2. FX-7: read check.mjs's run() and wasmStep's `call` in the build copy: which calls go through `call`, which get the
   300 s limit (cmd !== "cargo"), which calls bypass `call` (io.libc, copy, read, etc.), the four cargo builds' labels;
   the FAIL line's words; spawnSync timeout semantics (SIGTERM to the direct child only; grandchildren; killSignal;
   whether a SIGTERM-ignoring child hangs spawnSync); does a stopped call leave a child holding fd 9 (the lane lock)?
   Probe: a scratch script with spawnSync timeout on a `sh -c 'sleep 600 & wait'` style tree to see what survives,
   and whether the inherited flock fd stays held (outside any copy, in my notes folder/scratchpad, no cargo).
   The self-test(s) for FX-7 in check.test.mjs: does it prove the limit wiring (limit passed to the runner per call
   kind) or only the message? Could it pass for the wrong reason?
3. FX-10: FAILED_STEP_DIR = ROOT/.gate/netcode-proof-test/failed-steps; ROOT's value per checkout; rmSync per step
   at start (only the steps run: --only leaves other steps' stale files); write on failure; the size on a large failure
   (maxBuffer 256 MiB, out = stdout+stderr); the self-test (does it use a temp dir, does it clean up, does it prove the
   panic above 40 lines); a stale file's risk for a later reader (no run id, no timestamp, no head in the file).
4. Time `node check.mjs --only wasm` twice with -x through the runner (r2-inf-wasm1.log, r2-inf-wasm2.log); read the
   step's own printed split; compare with round 1 (42.9-46.5 s) and the fix pass's fx-check1/2 logs.
5. T5's INF-1 words against my round-1 INF-1 (labels, first-build figures, bound of about 80 s vs 300 s, cold route
   after the merge via W1-TARGET item 3), and Block 1's route.
6. Bounds check of the fix pass for my areas (files changed, no CI or .github change, no new target dir).
7. Findings, ranked; verdict.

Status: step 1 started.
