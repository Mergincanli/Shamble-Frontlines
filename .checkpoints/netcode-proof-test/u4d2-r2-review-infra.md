# U4d2 review round 2: infra-engineer seat (notes)

Started 2026-11-13. Fresh start (no notes file, no r2-inf-* logs existed). No agent memory exists for this seat.
Copy under review: /home/user/Shamble-Frontlines/.claude/worktrees/u4d2-build (uncommitted on 51520b4, after the fix
pass); read copy .gate/u4d2-r2; round 1's tree .gate/u4d2-r1. Runner: .checkpoints/netcode-proof-test/u4d2-run.sh; log
prefix r2-inf-. I edit no copy and plant nothing. This file is rewritten whole with Write after each finding.
Resumed after a cut (the plan below was on disk, status "step 1 started"; no r2-inf-* log existed at resume). The VM
itself restarted at about 00:53 UTC (uptime 13 min at 01:05:56), which matters for timing run 1 (step 4).

## Plan
1. Check the read copy's hashes (fx-hashes-end.sha256: 18 of 18) and the build copy's status; read the runner.
2. FX-7: which calls get the limit, the FAIL line, spawnSync's timeout semantics, whether a stopped call leaves a child
   holding fd 9 (the lane lock); probe outside any copy; the self-test's reach.
3. FX-10: the file's place, its replacement per run, its size on a large failure, the self-test, stale files.
4. Time `node check.mjs --only wasm` twice with -x through the runner; read the step's printed split.
5. T5's INF-1 words against my round-1 INF-1, and Block 1's / M9's cold route.
6. Bounds check of the fix pass for my areas (files changed, no CI or .github change, no new target dir).
7. Findings, ranked; verdict.

Status: steps 1, 2 and 4 DONE; step 5 drafted; step 3 next, then 6 and 7.

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
  only. Within Block 8's ruled proof ("one self-test"): INF2-2 (Low) below.
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
  piped stdin/stdout; one stuck in compute ignores the EOF its parent's death gives), but no child of check.mjs holds
  the lane lock, so the lock is released when check.mjs and the runner exit. The orphan keeps a CPU busy and the
  runner's wait does not see it (pgrep -x cargo / rustc only), so a later timed run on this 4-vCPU machine could be
  slowed unseen: INF2-3 (Low) below. Node docs (child_process, read 2026-11-13): after a timeout spawnSync "won't
  return until the process has completely exited"; a child that handles SIGTERM and does not exit keeps it waiting.
  npt-lab (Rust, no handler) and the loader's node (no handler) die on SIGTERM, so not a path here.

## Step 4: timing (done)
- r2-inf-wasm1.log (-x, lock 01:03:57, exit 0 at 01:05:11): "PASS  wasm  73.7 s", "check: all 1 steps pass"; 27000/0
  both pairs, h2 equal, hash-map native caught (4313a0b54a6656c5, b506e10c83446415), native-Wasm caught, Wasm-Wasm not
  exercised (f5e8d0e818b168c5): not catchable on wasm32; std-sin caught at SS-s1 tick 2, vehicle[8].pos.y. Split:
  builds 35.9, 6.0, 3.8, 1.4 (47.1); the rest 25.9 (npt-lab hash 4.1, Wasm 3.5 + 1.1 + 0.1, hash-map 3.1 + 3.4 + 3.3 +
  0.7 + 0.3, std-sin 2.9 + 3.3 + 0.1, the rest 0.0).
  - Why the first build took 35.9 s: no dependency was rebuilt (release/deps rlibs written 01:03-01:06: 19, all npt-*
    workspace crates, measured and bites; wasm32: npt_sim twice; none between 00:30 and 01:03), so this is not a cold
    target. The VM had restarted 11 min before (uptime 13 min at 01:05:56), buff/cache 1.3 GB of 16 GB after the run,
    /proc/pressure/io "full avg300=17.43" (cpu "some avg300=0.15"): the build waited on reading the release
    dependencies and the toolchain from a cold page cache. So run 1 is "warm for dependencies, page cache cold (the
    first run after a machine restart)".
- r2-inf-wasm2.log (-x, lock 01:06:28, exit 0 at 01:07:09): "PASS  wasm  40.4 s"; same outcomes (hash-map digests
  vary per run, as in round 1). Split: builds 5.6, 5.8, 1.4, 1.5 (14.3); the rest 26.1 (npt-lab hash 4.2, Wasm 3.5 +
  1.2 + 0.1, h2 0.1, hash-map 2.8 + 3.4 + 3.4 + 0.7 + 0.3, std-sin 2.9 + 3.4 + 0.1). io pressure avg10 0.00 after.
- r2-inf-holders.log (-x, with the sampler; not a timing run): PASS 41.3 s.
- Earlier warm figures: fix pass fx-check1 41.8, fx-check2 40.5; round 1 b8 42.9, 43.5, e1 46.4, e2 46.5, mine 45.4,
  43.8. Warm figures now 40.4-46.5 s over ten runs (with run 3, 41.3). The FX-7 limits cost nothing measurable (40.4 vs
  40.5-41.8 in the fix pass). Only the four builds would change with a cold target; every other call is 0.0-4.2 s
  against its 300 s limit (71x margin at the largest).

## Step 5: T5's INF-1 words and the cold route (drafted)
- My round-1 INF-1 asked: every figure labelled "warm for dependencies, workspace rebuilt by the touch loop"; the
  first-build figures (b3-lab1 16.04 s, b2-hw-test3 3.57 s) and the bound; why no cold run (§7.1 items 1 and 7).
  T5's bullet has each: the label (word for word in substance), 16.04 s and 3.57 s, "the infra seat's bound for a cold
  step is about 80 s", the reason (the shared target always held other lanes' release dependencies; the one-folder
  rule), and the route (the first run after the between-waves clean, W1-TARGET item 3, its seconds from the card).
  Block 1 adds "against 300 s" through T5's "against a 5-minute budget". It meets my finding.
- Two gaps, both Low:
  - INF2-4: my bound was "about 80 s on this machine", made with a warm page cache. Run 1 shows the page cache alone
    adds about 33 s (73.7 against 40.4, no dependency rebuilt). If T5's "<the fix pass's and round 2's wasm step
    seconds>" takes 73.7 s under "Every figure here is warm for dependencies", the label stays true but Block 1's
    "the budget verdict stands on the warm figures: 40.0 to 46.5 s" would sit beside a 73.7. Fill: "40.5 and 41.8 (the
    fix pass), 40.4 (round 2), and 73.7 s for round 2's first run after a machine restart (page cache cold; no
    dependency rebuilt: r2-inf-wasm1.log)"; and the bound "about 80 s with a warm page cache; a first run after a
    restart adds about 30 s" (still far under 300 s: high confidence). The card's cold figure should name whether
    the machine had just restarted (uptime), so U4r's Evidence decision reads the right case.
  - INF2-5: M9's route runs `cargo clean --target-dir .gate/shared-target` and then main-run.sh. The lock file
    `.gate/shared-target/.lane.lock` (inode 655904, listed in the folder) lives inside the folder the clean deletes
    (cargo-clean docs, read 2026-11-13: "With no options, cargo clean will delete the entire target directory").
    main-run.sh:20-21 and u4d2-run.sh `exec 9>"$LOCK"; flock -x 9` make no `mkdir -p` of the lock's folder and do not
    stop when either fails (`set -u` only, no `set -e`, no `|| exit`): after the clean, the exec redirection fails, the
    flock fails on a bad fd, and the run goes on with no lock while its log says "lock ... held exclusively". (I did
    not run a bash test of the failed exec: the permission prompt refused it; bash's default, non-POSIX behaviour is
    that `exec` with a redirection error returns 1 and the script continues: medium confidence, by reading.) With no
    lane running it changes no figure; a lane starting meanwhile would make a fresh lock file and overlap the cold run
    unseen. Fix before M9's cold run (main session's scripts, not U4d2's lane): `mkdir -p "$(dirname "$LOCK")"` before
    the exec and `flock ... 9 || exit 5` in both runners, or run the clean and then `mkdir -p .gate/shared-target`.
