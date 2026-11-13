# U4d2 review round 2: infra-engineer seat (notes)

Started 2026-11-13. Fresh start (no notes file, no r2-inf-* logs existed). No agent memory exists for this seat.
Copy under review: /home/user/Shamble-Frontlines/.claude/worktrees/u4d2-build (uncommitted on 51520b4, after the fix
pass); read copy .gate/u4d2-r2; round 1's tree .gate/u4d2-r1. Runner: .checkpoints/netcode-proof-test/u4d2-run.sh; log
prefix r2-inf-. I edit no copy and plant nothing. This file is rewritten whole with Write after each finding.
Resumed after a cut (the plan below was on disk, status "step 1 started"; no r2-inf-* log existed at resume).

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

Status: steps 1-2 DONE (reading and probe; the live fd check rides timing run 2); step 3 next.

## Step 1 (done)
- `sha256sum -c .gate/u4d2/fx-hashes-end.sha256` from .gate/u4d2-r2's root: 18 of 18 OK; from the build copy's root:
  18 of 18 OK. Build copy HEAD 51520b43; `git status --untracked-files=all`: 9 modified, 10 untracked (the 18 spike
  files plus the builder's record), as the packet says.
- Runner read: `exec 9>"$LOCK"; flock "$mode" 9` (fd 9 inherited by every child of the runner's bash), waits for no
  cargo/rustc (60 min cap, after the flock; the flock itself has no -w, so it waits without limit), 4 GB floor, touch
  loop every 2 s, CARGO_TARGET_DIR shared, offline; `run_one` runs "$@" in the foreground and waits.

## Step 2: FX-7 (done)
- Code (build copy check.mjs): run() :1851-1856 passes `timeout` to spawnSync only when non-zero and returns
  `timedOut: r.error?.code === "ETIMEDOUT"`; wasmStep's `call` :2261-2268: `limit = cmd === "cargo" ? 0 :
  WASM_CALL_LIMIT_MS` (300_000, :2212, not exported); a timeout pushes "<label>: still running after 300 s, so it was
  stopped (the wasm step's limit on each call but its four cargo builds)" into problems (so it is a FAIL and lands in
  the 40 tail lines).
- Which calls (every child call of wasmStep goes through `call`; grep of `call(`): no limit: the four WASM_BUILDS
  (:2120-2140, cmd "cargo", labels "build npt-lab", "build npt-lab-bites", "build npt-hash-wasm.wasm", "build
  npt-hash-wasm-bites.wasm"). 300 s: "npt-lab hash" (:2319), "Wasm <rec>" x3 (:2330), "compare" (:2338), "Wasm dump
  <rec>" and "compare field" (:2344-2345, only at a mismatch), "h2 <vector>" x4 (:2352), "h2 vectors" (:2355), "npt-lab
  bites" (:2365), "hash-map native legs" (:2374), "hash-map Wasm 1/2" (:2377), "hash-map compare native-Wasm" and
  "Wasm-Wasm" (:2381), "std-sin native legs", "std-sin Wasm", "std-sin compare" (:2432-2436). Not child calls (in
  process, no limit needed): io.reset/mkdir/copy/exists/read, io.libc (process.report). Matches Block 3's list (loader
  legs, npt-lab hash legs and compares, vectors, bites). The discriminator is the command, not the label: a future
  `cargo run` call in the step would get no limit silently (note, not a finding of this change).
- Self-test (check.test.mjs:2784-2797, the packet's tests file :440-453): asserts each fake call's limit (0 for kinds
  starting "build ", else 300000) and exactly four zero-limit calls; a Wasm leg and the native legs' call returning
  `timedOut` each FAIL naming "<label>: still running after 300 s ..." within main's tail. This proves `call`'s wiring
  and the line. It cannot see run() (not exported): dropping `...(timeout ? { timeout } : {})` from spawnSync, or a
  wrong error code in `timedOut`, leaves all 71 self-tests green. The builder checked spawnSync's ETIMEDOUT shape by
  hand (`sleep 5`, timeout 1000: status null, SIGTERM, ETIMEDOUT; u4d2-build.md:438-439), not run() itself. The
  builder's plant (fx-7-plant.log: `if (false && r.timedOut)`, "not ok 69", 70 of 71) bites the message only. Within
  Block 8's ruled proof ("one self-test"); a Low gap (INF2-2 below).
- Probe (u4d2-r2-review-infra-probe.mjs, my notes folder; no cargo, no shared lock; the lock is the probe file's own,
  opened read-only as the runner opens .lane.lock; output in the Bash tool only). Node v22.23.4:
  - spawnSync(sh -c 'sleep 41.3 & wait', timeout 1500): returned after 1504 ms, status null, signal SIGTERM,
    error.code ETIMEDOUT (what run() reads). The grandchild `sleep 41.3` SURVIVED (pgrep found pid 1704 after both
    probes; I killed it): spawnSync's timeout signals the direct child only.
  - But no child of Node held the lock fd: the holders 400 ms after the call were the probing bash alone, and
    `flock -n -x` on the file succeeded (exit 0) as soon as bash closed its fd 9, with the orphan still alive. Why:
    Node's own fd 9 carries O_CLOEXEC (/proc/self/fdinfo/9 flags 02100000 in node; 0100000 in bash), so nothing Node
    spawns receives it: `ls -l /proc/self/fd` in a spawnSync child of node lists 0-3 only; the same in a child of bash
    lists fd 9.
  - With `detached: true` and a SIGKILL to the group on ETIMEDOUT (a candidate form), the grandchild was gone too
    (pgrep found only 41.3's, not 41.4's).
  - So, answer for check.mjs's calls: a stopped call can leave a child running (npt-lab's Leg children, hash.rs:641-650,
    are this binary re-run with piped stdin/stdout; one stuck in compute ignores the EOF its parent's death gives), but
    that child does not hold the lane lock: the lock's holders under a check.mjs run are the runner's bash (and its
    toucher subshell) and check.mjs's node only, released when they exit. The orphan keeps a CPU busy and is invisible
    to the runner's wait (pgrep -x cargo / rustc only), so a later timing run on this 4-vCPU machine could be slowed
    unseen. Live check of the holders in the real setting: timing run 2 (step 4).
