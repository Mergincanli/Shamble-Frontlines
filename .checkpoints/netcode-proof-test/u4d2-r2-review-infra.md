# U4d2 review round 2: infra-engineer seat (notes)

Started 2026-11-13. Fresh start (no notes file, no r2-inf-* logs existed). No agent memory exists for this seat.
Copy under review: /home/user/Shamble-Frontlines/.claude/worktrees/u4d2-build (uncommitted on 51520b4, after the fix
pass); read copy .gate/u4d2-r2; round 1's tree .gate/u4d2-r1. Runner: .checkpoints/netcode-proof-test/u4d2-run.sh; log
prefix r2-inf-. I edit no copy and plant nothing. This file is rewritten whole with Write after each finding.
Resumed after a cut (the plan below was on disk, status "step 1 started"; no r2-inf-* log existed at resume). The VM
itself restarted at about 00:53 UTC (uptime 13 min at 01:05:56), which matters for timing run 1 (step 4).
Scratch (text and small scripts, this folder): u4d2-r2-review-infra-probe.mjs, -holders.mjs, -hang.mjs.

## Plan
1. Check the read copy's hashes (fx-hashes-end.sha256: 18 of 18) and the build copy's status; read the runner.
2. FX-7: which calls get the limit, the FAIL line, spawnSync's timeout semantics, whether a stopped call leaves a child
   holding fd 9 (the lane lock); probe outside any copy; the self-test's reach.
3. FX-10: the file's place, its replacement per run, its size on a large failure, the self-test, stale files.
4. Time `node check.mjs --only wasm` twice with -x through the runner; read the step's printed split.
5. T5's INF-1 words against my round-1 INF-1, and Block 1's / M9's cold route.
6. Bounds check of the fix pass for my areas (files changed, no CI or .github change, no new target dir).
7. Findings, ranked; verdict.

Status: steps 1-7 DONE.

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
  stopped (the wasm step's limit on each call but its four cargo builds)" into problems (a FAIL, in the 40 tail lines).
- Which calls (every child call of wasmStep goes through `call`; grep of `call(`): no limit: the four WASM_BUILDS
  (:2120-2140, cmd "cargo": "build npt-lab", "build npt-lab-bites", "build npt-hash-wasm.wasm", "build
  npt-hash-wasm-bites.wasm"). 300 s: "npt-lab hash" (:2319), "Wasm <rec>" x3 (:2330), "compare" (:2338), "Wasm dump
  <rec>" and "compare field" (:2344-2345, only at a mismatch), "h2 <vector>" x4 (:2352), "h2 vectors" (:2355), "npt-lab
  bites" (:2365), "hash-map native legs" (:2374), "hash-map Wasm 1/2" (:2377), the two hash-map compares (:2381),
  "std-sin native legs", "std-sin Wasm", "std-sin compare" (:2432-2436). In process, no child: io.reset/mkdir/copy/
  exists/read, io.libc. Matches Block 3's list. The discriminator is the command, not the label: a future `cargo` call
  in the step would get no limit silently (a note, not a finding of this change).
- Self-test (check.test.mjs:2784-2797; packet tests file :440-453): each fake call's limit (0 for kinds starting
  "build ", else 300000), exactly four zero-limit calls; a Wasm leg and the native legs' call returning `timedOut` each
  FAIL naming "<label>: still running after 300 s ..." within main's tail. It proves `call`'s wiring and the line. It
  cannot see run() (not exported): dropping `...(timeout ? { timeout } : {})` from spawnSync, or a wrong code in
  `timedOut`, leaves all 71 self-tests green. The builder checked spawnSync's ETIMEDOUT shape by hand (u4d2-build.md:
  438-439), not run(). Its plant (fx-7-plant.log: `if (false && r.timedOut)`, "not ok 69", 70 of 71) bites the message
  only. Within Block 8's ruled proof ("one self-test"): INF2-2 (Low).
- Probe (u4d2-r2-review-infra-probe.mjs; no cargo, no shared lock; the lock is the probe file's own, opened read-only as
  the runner opens .lane.lock; output in the Bash tool only), Node v22.23.4:
  - spawnSync(sh -c 'sleep 41.3 & wait', timeout 1500): returned after 1504 ms, status null, signal SIGTERM,
    error.code ETIMEDOUT (what run() reads). The grandchild `sleep 41.3` SURVIVED (pgrep found it after both probes; I
    killed it): the timeout signals the direct child only (libuv docs: uv_process_kill "Sends the specified signal to
    the given process handle").
  - No child of Node held the lock fd: the holders 400 ms after the call were the probing bash alone, and `flock -n -x`
    succeeded (exit 0) once bash closed its fd 9, the orphan still alive. Node's own fd 9 carries O_CLOEXEC
    (/proc/self/fdinfo/9 flags 02100000 in node, 0100000 in bash); a spawnSync child of node lists fds 0-3 only, a
    child of bash lists fd 9. Source: node v22.x src/node.cc calls uv_disable_stdio_inheritance() ("Make inherited
    handles noninheritable"); libuv's core.c sets CLOEXEC on fds from 0, the first 16 unconditionally (fd 9 is inside
    them); libuv docs: "child processes spawned by this process don't accidentally inherit these handles",
    "best-effort". Read 2026-11-13. High confidence for this Node; the mechanism read from source.
  - With `detached: true` and a SIGKILL to the group on ETIMEDOUT, the grandchild was gone too (a candidate form;
    `detached` is not documented for spawnSync in Node 22's docs, though it worked here: medium confidence).
- Live, in the real setting (r2-inf-holders.log, -x, wasm PASS 41.3 s, with u4d2-r2-review-infra-holders.mjs sampling
  /proc every 500 ms, 82 samples): the lane lock's holders were the runner's bash and its toucher subshell (164), node
  check.mjs (82), the toucher's `sleep 2` (81) and one `find`. Without it: cargo (30), rustc (57 across the npt-*
  crates), npt-lab (24) and npt-lab-bites (38) (the legs' children included), the loader's node (30).
- Answer: a stopped call can leave a child running (npt-lab's Leg children, hash.rs:641-650, this binary re-run with
  piped stdin/stdout, read at :884; one stuck in compute never sees the EOF its parent's death gives; Rust std docs,
  Child: "There is no implementation of Drop for child processes ... it will continue to run"), but no child of
  check.mjs holds the lane lock, so the lock is released when check.mjs and the runner exit. Node docs (child_process,
  read 2026-11-13): after a timeout spawnSync "won't return until the process has completely exited"; a child that
  handles SIGTERM keeps it waiting. npt-lab (Rust, no handler) and the loader's node die on SIGTERM: not a path here.
- By count (u4d2-r2-review-infra-hang.mjs through the runner, -s, r2-inf-hang.log, exit 0): wasmStep with a fake runner
  in which every call that replays ticks hangs (a hang in npt-sim's replay: both legs, every loader leg on a recording,
  every compare that replays by replay_to) and the rest succeed: 23 calls, 19 with the limit, 4 without; 12 calls each
  wait out 300 s (3 x the native legs, 6 x the loader on a recording, 3 x a replaying compare) = 60 min of the
  exclusive lock, 12 problem lines naming the limit, the step FAILs. Each of the 3 native-legs calls can leave its two
  legs running (both replay the same sim), so up to 6 orphans busy on this 4-vCPU machine, unseen by the runner's
  wait (pgrep -x cargo / rustc only). INF2-1 (Medium).

## Step 3: FX-10 (done)
- Place: FAILED_STEP_DIR = join(ROOT, ".gate", "netcode-proof-test", "failed-steps") (check.mjs:2514; ROOT = HERE/../..,
  :33), so per checkout: <checkout>/.gate/netcode-proof-test/failed-steps/<step>.log. Gitignored (`git check-ignore -v`:
  ".gitignore:2:.gate/"). Self-test pins the path (check.test.mjs:2485).
- Replacement: main :2530-2531 removes `<step>.log` (rmSync force) before each step that runs, writes it only when that
  step fails (:2539-2545; a write failure is said, not thrown), and prints "the step's whole output (<n> lines):
  <path>" after the 40 lines. So a run's FAIL line always names a file of that run. Steps not run (`--only`) keep their
  old file: the self-test pins this (an older deps.log stays, :2503-2505). Live: fx-1-env.log:11, :17, :23 name
  deps.log; fx-2-tf.log:34 wasm.log (20 lines). The build copy's folder is empty now (the counted runs passed every
  step, and my --only wasm runs passed).
- Size: one file per step, replaced, so no growth across runs. Typical: the wasm step's failure 4,994 B (u4d2-build.md:
  465); a whole bites-build workspace test run 754 lines, 44,592 B at U4j (.gate/te2-5/run1.log), so a failed test
  step (both builds' output: testBothBuilds :1973-1981) is of the order of 100 KB today (estimate). Bound: spawnSync's
  maxBuffer 256 MiB per stream (Node docs: "Largest amount of data in bytes allowed on stdout or stderr. If exceeded,
  the child process is terminated and any output is truncated"), so a runaway printer gives a truncated file of at most
  a few hundred MiB under .gate/ (8 GB free now, the runner's floor 4 GB). Not a finding.
- Self-test (check.test.mjs:2484-2506): a temp dir (mkdtempSync, removed by t.after), a recorded failing test step whose
  panic sits above the last 40 lines; asserts the panic is not printed, the file equals the whole output, the path line
  is printed, the last line is "check: 1 step(s) failed", and a later passing run removes test.log and keeps deps.log.
  It would fail if the write, the removal or the path line went (the builder's plant: fx-10-plant.log, "not ok 66").
  Sound.
- Stale file: a file holds the step's output only (no time, head or command), and `--only` leaves other steps' files,
  so a reader browsing the folder later (not following a FAIL line) can take an older run's file as current; mtime is
  the only clue. INF2-5 (Low).

## Step 4: timing (done)
- r2-inf-wasm1.log (-x, lock 01:03:57, exit 0 at 01:05:11): "PASS  wasm  73.7 s", "check: all 1 steps pass"; 27000/0
  both pairs, h2 equal, hash-map native caught (4313a0b54a6656c5, b506e10c83446415), native-Wasm caught, Wasm-Wasm not
  exercised (f5e8d0e818b168c5): not catchable on wasm32; std-sin caught at SS-s1 tick 2, vehicle[8].pos.y. Split:
  builds 35.9, 6.0, 3.8, 1.4 (47.1); the rest 25.9 (npt-lab hash 4.1, Wasm 3.5 + 1.1 + 0.1, hash-map 3.1 + 3.4 + 3.3 +
  0.7 + 0.3, std-sin 2.9 + 3.3 + 0.1, the rest 0.0).
  - Why the first build took 35.9 s: no dependency was rebuilt (release/deps rlibs written 01:03-01:06: 19, all npt-*
    workspace crates, measured and bites; wasm32: npt_sim twice; none between 00:30 and 01:03), so not a cold target.
    The VM had restarted 11 min before (uptime 13 min at 01:05:56), buff/cache 1.3 GB of 16 GB after the run,
    /proc/pressure/io "full avg300=17.43" (cpu "some avg300=0.15"): the build waited on reading the release
    dependencies and the toolchain from a cold page cache (medium-high confidence on the cause). Run 1 is "warm for
    dependencies, page cache cold (the first run after a machine restart)".
- r2-inf-wasm2.log (-x, lock 01:06:28, exit 0 at 01:07:09): "PASS  wasm  40.4 s"; same outcomes (hash-map digests vary
  per run, as in round 1). Split: builds 5.6, 5.8, 1.4, 1.5 (14.3); the rest 26.1 (npt-lab hash 4.2, Wasm 3.5 + 1.2 +
  0.1, h2 0.1, hash-map 2.8 + 3.4 + 3.4 + 0.7 + 0.3, std-sin 2.9 + 3.4 + 0.1). io pressure avg10 0.00 after.
- r2-inf-holders.log (-x, with the sampler; not a timing run): PASS 41.3 s.
- Warm figures: fix pass fx-check1 41.8, fx-check2 40.5; round 1 b8 42.9, 43.5, e1 46.4, e2 46.5, mine 45.4, 43.8;
  now 40.4 (and 41.3 with the sampler): 40.4-46.5 s. FX-7 costs nothing measurable. Every non-build call takes 0.0-4.2
  s against its 300 s limit (71x margin at the largest).

## Step 5: T5's INF-1 words and the cold route (done)
- My round-1 INF-1 asked: every figure labelled "warm for dependencies, workspace rebuilt by the touch loop"; the
  first-build figures (b3-lab1 16.04 s, b2-hw-test3 3.57 s) and the bound; why no cold run (§7.1 items 1 and 7).
  T5's bullet has each: the label, 16.04 s and 3.57 s, "the infra seat's bound for a cold step is about 80 s", the
  reason (the shared target always held other lanes' release dependencies; the one-folder rule), "against a 5-minute
  budget", and the route (the first run after the between-waves clean, W1-TARGET item 3, its seconds from the card).
  It meets my finding.
- INF2-3 (Low): my bound was "about 80 s on this machine", made with a warm page cache. Run 1 shows the page cache alone
  adds about 30 s (73.7 against 40.4, no dependency rebuilt). If T5's "<the fix pass's and round 2's wasm step
  seconds>" takes 73.7 s unlabelled under "Every figure here is warm for dependencies", the label stays true, but Block
  1's "the budget verdict stands on the warm figures: 40.0 to 46.5 s" sits beside a 73.7. Fill: "41.8 and 40.5 s (the
  fix pass), 40.4 s (round 2), and 73.7 s for round 2's first run after a machine restart (page cache cold; no
  dependency rebuilt: r2-inf-wasm1.log)"; the bound: "about 80 s with a warm page cache; a first run after a restart
  adds about 30 s" (far under 300 s either way: high confidence). The card's cold figure names the machine's uptime.
- INF2-4 (Low): M9's route runs `cargo clean --target-dir .gate/shared-target`, then main-run.sh. The lock file
  `.gate/shared-target/.lane.lock` (inode 655904) lives inside the folder the clean deletes (cargo-clean docs, read
  2026-11-13: "With no options, cargo clean will delete the entire target directory"). main-run.sh:20-21 (and
  u4d2-run.sh's same two lines) make no `mkdir -p` of the lock's folder and do not stop when the exec or the flock
  fails (`set -u` only, no `set -e`, no `|| exit`). After the clean, the exec redirection fails, flock fails on a bad
  fd, and the run goes on with no lock while its log says "lock ... held exclusively". (Bash's default behaviour, a
  failed `exec` redirection returning 1 and the script going on, is by reading: the permission prompt refused my bash
  test; medium confidence.) With no lane running it changes no figure; a lane starting meanwhile would make a fresh
  lock file and overlap the cold run unseen. Fix before M9's cold run (the main session's scripts, not U4d2's lane):
  `mkdir -p "$(dirname "$LOCK")"` before the exec and `flock ... 9 || exit 5`, or `mkdir -p .gate/shared-target` after
  the clean.

## Step 6: bounds (done)
- Files the fix pass changed (the packet's diffs): Cargo.toml's libm comment, check.mjs, check.test.mjs,
  crates/hash-wasm/clippy.toml (new) and loader.mjs, crates/lab/src/hash.rs and lib.rs: inside Block 8's list.
- `git diff --stat 51520b4 -- .github` and `git status --short -- .github` in the build copy: empty (no CI change).
- Every fx-*.log runner header names CARGO_TARGET_DIR the shared target (20 of 20); no `--target-dir` in any fx-*.log
  command. The security seat's sec-r2-home/ (36 KB) holds four config.toml cases (one folder named "target"), not a
  build folder. Shared target 7.0G after my runs (7.0G after fx-check2): my runs rebuilt only workspace rlibs in place;
  df 8 GB free.

## Findings (ranked)
- INF2-1 Medium, not before the merge (to the advisor: a carry for U4r's change to the wasm step, or a follow-up):
  FX-7's limit is per call, and spawnSync's timeout signals only the direct child. A hang in npt-sim's replay, the
  likely root of any hang since every leg replays the same sim, makes 12 calls each wait out 300 s (60 min of the
  exclusive lock, against Block 3's "that is the step's whole budget"), and each of the three native-legs calls can
  leave its two legs running in compute after the step, up to 6 orphans busy on 4 vCPUs that the runner's wait does
  not see; later timed runs and the load-sensitive npt-net test (TE-M3) would run on a loaded machine unseen. The lane
  lock itself is not held by them (Node marks fd 9 close-on-exec). Proof: r2-inf-hang.log (the count) and the probe
  (orphan survives; group kill clears it). Fix: one deadline for the step's non-build calls (each call given what is
  left of 300 s, or the rest skipped after the first stop) and, on a stop, SIGKILL to the call's process group (spawn
  detached; process.kill(-pid)); one self-test for the deadline.
- INF2-2 Low, follow-up: run()'s timeout wiring is untested (the self-test uses a fake runner; the plant bites the
  message only). Fix: export run (or a small wrapper) and one self-test: process.execPath -e "setTimeout(()=>{},1e4)"
  with a 200 ms limit gives timedOut true, status null.
- INF2-3 Low, in M8's fill (no push of its own): T5's figures: label 73.7 s as the first run after a restart (page
  cache cold, no dependency rebuilt), and the bound "about 80 s with a warm page cache".
- INF2-4 Low, before M9's cold run (main session's scripts): the clean deletes the lock's folder; the runners neither
  recreate it nor stop on a failed lock.
- INF2-5 Low, follow-up: FX-10's files carry no time, head or command, and `--only` keeps other steps' older files; a
  first line "<UTC time> <HEAD> check.mjs <argv>" would let a later reader date them.

## Verdict
APPROVE WITH FINDINGS. FX-7 meets its ruled form (300 s on each of the 19 non-cargo calls, none on the four builds, a
named FAIL, one self-test) and no stopped call can leave the lane lock held; FX-10 meets its form (the checkout's
.gate/, removed at each run of the step, written on failure, named in the FAIL lines, bounded in size). The step's
warm time is 40.4 s now (40.4-46.5 s over ten runs); a first run after a machine restart took 73.7 s with no
dependency rebuilt. T5's INF-1 words meet my round-1 finding. One Medium (INF2-1) goes to the advisor; four Lows.
