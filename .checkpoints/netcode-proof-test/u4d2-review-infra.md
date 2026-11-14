# U4d2 review round 1: infra-engineer seat (notes)

Started 2026-11-12 (about 20:40 UTC). Fresh start (no notes file, no r1-inf-* logs existed).
Copy under review: /home/user/Shamble-Frontlines/.claude/worktrees/u4d2-build (uncommitted on 51520b4); read copy .gate/u4d2-r1.
Runner: .checkpoints/netcode-proof-test/u4d2-run.sh; my log prefix r1-inf-. No Edit tool here: this file is rewritten
whole with Write each time (the brief's rule). I edited no copy and planted nothing.

## Plan
1. Read the runner (u4d2-run.sh), the packet's heads, plan bits (§5 U4d2, §4.11, §4.14, §4.15, §7.1, plan.md R-8,
   A1 of advisor-u4d2-brief.md, C-U7-1) for the budget wording.
2. Read check.mjs's new `wasm` step (diff), hash-wasm's Cargo.toml, loader.mjs, lab's hash.rs build calls, the
   step folder handling (where written, kept or cleaned), the build-flag check and read-back.
3. Read the builder's logs: b8-wasm1/2, e1/e2-check, b1-lock*, b2, b5, b8-scan; its record's split by part.
4. Time `node check.mjs --only wasm` twice with -x through the runner (r1-inf-wasm1.log, r1-inf-wasm2.log).
5. Disk: shared target size before/after, the step folder, recordings regenerated; memory (/usr/bin/time if present).
6. .github/ read-only: no job runs check.mjs or spikes/.
7. Reproducibility: toolchain pins (rust-toolchain.toml), wasm32 target, Node version, absolute paths, Windows.
8. Findings, ranked, then the verdict.
Status: steps 1-8 DONE.

## Read so far (sources, with what each shows)
- u4d2-run.sh: touches every file under crates/ at start and every 2 s (so every cargo call rebuilds the workspace's
  npt-* crates; deps stay cached); -x exclusive flock on fd 9 (inherited by every child); waits for no cargo/rustc;
  4 GB free floor; offline. main-run.sh (the main folder's) does the same, on the same shared target.
- check.mjs (build copy) :2009-2362 wasmStep; :2016 WASM_STEP_DIR = <checkout>/.gate/netcode-proof-test/wasm-step;
  :2124-2127 io.reset = rmSync(dir, recursive) then mkdir at the start of every run (kept after the run, cleaned at the
  next); :2031-2050 four release builds (npt-lab x86-64 measured + bites; npt-hash-wasm wasm32 measured + bites);
  :2183-2192 artifact path read from cargo's JSON, copied to bin/<copy> (no .exe suffix: "npt-lab", "npt-lab-bites");
  :1767-1772 run() = spawnSync, maxBuffer 256 MiB, no timeout; :2132-2135 libc from process.report's glibc field;
  :2348 std-sin's failure names the libc only. The step prints its seconds per call; nothing fails it over 5 min
  (the plan says "printed, budget 5 minutes": a review target, as the brief reads it).
- check.test.mjs:2392-2413: the wasm step's self-tests use a fake io (no rmSync, no real folder).
- .gitignore: `.gate/` ignored (step folder never committed). rust-toolchain.toml: channel 1.99.1, minimal, clippy +
  rustfmt, targets wasm32-unknown-unknown. .cargo/config.toml: target-dir ../../.gate/netcode-proof-test/target
  (the runner's CARGO_TARGET_DIR overrides; the step reads the artifact from cargo's JSON, so either works).
- Bites features forward to npt-* crates only (crates/*/Cargo.toml [features]), so the bites builds share every
  non-workspace dependency with the measured builds: a cold run pays the dependency compile once per target.
- Builder logs: b8-wasm1.log 18:20:20-18:21:03 UTC, step 42.9 s; b8-wasm2.log 18:21:35-18:22:19, 43.5 s. Split (run 1):
  builds 6.2 + 6.6 + 1.5 + 1.6 = 15.9 s; npt-lab hash 4.4; Wasm legs 3.5 + 1.2 + 0.1; hash-map 3.0 + 3.3 + 3.5 + 0.7
  + 0.4; std-sin 3.2 + 3.5 + 0.1; the rest 0.0. Runner provenance empty (check.mjs keeps cargo's output in memory).
- Builder record u4d2-build.md:171-178: run 1 "the release deps were already in the shared target from my by-hand
  runs" -> no cold figure was taken; both of its runs are warm for deps (and its release builds were already in the
  shared target: b2-hw-test3 16:40, b3-lab1 16:47), so neither meets the brief's "cold (its first run: the release
  builds not yet in the shared target)" (u4d2-brief.md:747-748).
- First release builds in this lane: b2-hw-test3.log 16:40:46 wasm32 npt-hash-wasm "Finished in 3.57s" (compiled only
  npt-sim + npt-hash-wasm: wasm32 deps already there); b3-lab1.log 16:47:56 npt-lab release "Finished in 16.04s"
  (compiled serde_core, zmij, serde_json, serde, serde_derive, memchr + 13 workspace crates; tokio/quinn/ring/rustls
  already there).
- A cold bound on this machine (4 vCPU Xeon 2.1 GHz, 15 GB, no swap): .gate/rr3-unit.txt (2026-11-07, a fresh
  rr3-target): 74 crates incl. ring, rustls, tokio, quinn-proto, tungstenite, release, "Finished in 20.33s";
  .gate/netcode-proof-test/u3vf-diag1.txt the same tree 21.27 s. The rust seat's r1-rr-superset-build.log (fresh
  .gate/u4d2-rr-target, wasm32 release of npt-hash-wasm, zerocopy/libm/... compiled, other flags): "Finished in
  6.05s". (Observation, not a finding of the change: that seat's run used `--target-dir` off the shared target,
  outside §7.1 item 1; I cite its figure only as a bound.)
- Disk (20:38 UTC, and again after my two runs): shared target 6.9G both times (debug 6.4G, release 264M, wasm32 159M
  of which release 40M, doc 21M); step folder 40M (recordings 33M, bin 2.6M, measured 2.6M, hash-map 1.3M,
  std-sin 1.0M); the builder's own scratch 120M (not the step's). df: 7.4G free before and after (7483M), so about
  3.4 GB above the runner's 4 GB floor. Wasm sizes 114282 B measured, 130368 B bites (= the builder's).
- glibc: `readelf -Ws --dyn-syms /lib/x86_64-linux-gnu/libm.so.6`: sinf@@GLIBC_2.2.5 is IFUNC (chosen at load by CPU
  features); this CPU has fma. std-sin-scan.txt records rustc, ldd's glibc 2.39 and Node v22.23.4, not the CPU.
- Node v22.23.4 here. loader.mjs: sync `new WebAssembly.Module`, empty imports, Object.hasOwn; nothing past Node 16.
  The wasm32 features rustc 1.99.1 enables (deps read-back: bulk-memory, multivalue, mutable-globals,
  nontrapping-fptoint, reference-types, sign-ext) are all in Node 22's V8. No Node pin for check.mjs (README:26 names
  Node 22 for redact-check only).
- No wasm-opt, wasm-bindgen or wasm-pack on PATH or in ~/.cargo/bin; rustup 1.29.3 with 1.99.1 + wasm32 installed.
  Playwright's Chromium is absent here: .gate/u4e-merge/gate-full.log:7 "Executable doesn't exist at
  /opt/pw-browsers/chromium_headless_shell-1194/...".
- .github (read only): one workflow, gate.yml: jobs gate (node tools/gate.mjs --no-smoke) and smoke (gate.mjs
  --smoke-only), ubuntu-24.04, Node 22, timeouts 10 and 15 min, contents: read; no cargo, no rust, no check.mjs, no
  spikes/ path. `git diff --stat 51520b4 -- .github` in the build copy: empty. plan.md:328 and :1401 (R-8: "approved;
  no CI job"); plan.md:668 puts "the native-against-Wasm hash in Node and local Chromium" in this container.
- Repo visibility: public (`gh api repos/<owner>/<repo>` .private false).
- Web sources (read 2026-11-12):
  - docs.github.com/en/actions/reference/runners/github-hosted-runners: public repos ubuntu-24.04: 4 CPU, 16 GB RAM,
    14 GB SSD; private: 2 CPU, 8 GB, 14 GB; standard runners "free and unlimited on public repositories". High.
  - docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching: "By default, the limit is 10 GB per
    repository"; entries not accessed in over 7 days removed; beyond 10 GB billed. High.
  - github.com/Swatinem/rust-cache README: "the workspace crates themselves are not cached"; caches ./target's
    dependency artifacts; v2. High.
  - raw.githubusercontent.com/libuv/libuv/v1.x/src/win/process.c: search_path tries a name as-is only if it has an
    extension (or with UV_PROCESS_WINDOWS_FILE_PATH_EXACT_NAME), else appends .com then .exe. High (source).
  - raw.githubusercontent.com/nodejs/node/v22.x/src/spawn_sync.cc: never sets EXACT_NAME. High (source).

## Runs (mine)
- r1-inf-wasm1.log (-x): lock 20:40:44, exit 0 at 20:41:30 (46 s): "PASS  wasm  45.4 s", "check: all 1 steps pass";
  27000/0 both pairs; h2 equal; hash-map native caught, Wasm 1-2 not exercised (not catchable on wasm32); std-sin
  caught at SS-s1 tick 2 vehicle[8].pos.y. Seconds: builds 7.2, 6.9, 1.4, 1.7 (17.2 = 38 %); clean half 4.6 + 3.5 +
  1.1 + 0.1 + 0.0 = 9.3; h2 0.0; bites half 18.5. Peak memory (ps sampled every 0.3 s): sum of cargo/rustc/node/npt-lab
  RSS 877 MiB, one process 275 MiB (rustc).
- r1-inf-wasm2.log (-x): lock 20:41:43, exit 0 at 20:42:27 (44 s): "PASS  wasm  43.8 s"; same outcomes; builds 6.5,
  6.6, 1.4, 1.5 (16.0); clean 9.2; bites 18.2. Peak 875 MiB sum, 274 MiB one rustc.
- Six warm figures now: 42.9, 43.5 (builder alone), 46.4, 46.5 (inside e1/e2), 45.4, 43.8 (mine): 42.9-46.5 s.
- Full run cost: e1-check.log 18:28:19-18:36:19 (8.0 min; pre-builds 49 s; check.mjs 431 s: test 294.7, doc 69.5,
  wasm 46.4 = 10.8 %); e2 7.5 min. The merged U4e head's 10-step run: .gate/u4e-merge/m5-counted.log 17:56:34-18:06:01
  = 9.5 min (test 481.8 s), before any wasm step.

## Cold, by bound (not measured: §7.1 items 1 and 7 allow no empty target)
- Extra over warm = the dependency compile, once per target: npt-lab's release tree about 20-21 s for the 74-crate
  transport tree (rr3-unit.txt, u3vf-diag1.txt) plus the serde family (b3-lab1: 16.0 s against 6.5-7.2 s warm, so
  about +9 s), i.e. about +30 s; wasm32 about +4.7 s (6.05 s fresh against 1.4 s). Cold step about 45 + 30 + 5 = 80 s
  on this machine (medium confidence on the figure; high that it is far under 300 s: even 3x the dependency compile
  stays under 3 min). A fresh machine adds rustup's toolchain install and the crates.io fetch at the first cargo call
  (inside the step's first build under `--only wasm`, inside fmt in a full run), and cannot run offline until
  `cargo fetch --locked` has run once.
- "Warm" here is the CI-cached case, not a developer re-run: the touch loop rebuilds every workspace crate, as a CI
  job with rust-cache would (workspace crates not cached). A no-touch re-run would be about 45 - 17 = 28 s.
- The run that would measure cold, which the rules do not allow: one `node check.mjs --only wasm` with
  CARGO_TARGET_DIR on a new empty folder under .gate/ (deleted after by its owner), exclusive lock, nothing else
  running; it needs a written exception to §7.1 item 1 (Mustafa's shared-target rule). Or the first run of a future
  CI job (R-8 out of scope today).

## Findings (ranked)
- INF-1 Medium, before the merge (report text only): no cold figure exists; both runs the builder could label cold
  are warm for deps (u4d2-build.md:171-172 says so; the release builds were already in the shared target from
  b2-hw-test3 16:40 and b3-lab1 16:47), so the brief's section 5 item 4 / 2.4 (a) "cold (its first run: the release
  builds not yet in the shared target)" is not met as worded. The report must label every figure "warm for
  dependencies, workspace rebuilt by the touch loop", give the first-build figures (b3-lab1 16.04 s, b2-hw-test3
  3.57 s) and the bound above, and say a cold run was not possible under §7.1 items 1 and 7. The budget verdict
  holds either way.
- INF-2 Low, follow-up: on Windows the step cannot run its native legs: check.mjs:2032-2033 copy the executable to
  bin/npt-lab and bin/npt-lab-bites, dropping cargo's ".exe"; Node 22's spawnSync on Windows (libuv search_path, no
  EXACT_NAME flag) only tries the name with .com/.exe appended, so ENOENT, and the step FAILs ("no comparison was
  written"), never a false pass. plan.md:668 puts this check in this container, so the reach is small. Fix: keep
  extname(path) in the copy's name; proof: a check.test.mjs case whose `executable` ends in .exe.
- INF-3 Low, follow-up: the std-sin catch is a property of glibc's IFUNC-chosen sinf on this CPU (fma) as well as of
  glibc 2.39; std-sin-scan.txt records libc and rustc, not the CPU features, and check.mjs:2348 names only the libc.
  Another machine (other CPU features, other glibc, macOS, Windows) can false-FAIL with a message that points at libc
  alone. Fix: record the CPU model and its fma/avx2 flags in the scan's record and in the failure line, and say "re-run
  the scan on this machine". CI's ubuntu-24.04 has glibc 2.39 too (likely the same catch; medium).
- INF-4 Low, follow-up: nothing in the step has a timeout (run() at check.mjs:1767-1768 passes none; hash.rs:656-668
  blocks on read_line; the loader is synchronous). A hung leg keeps the runner's exclusive lock (fd 9, inherited by
  every child) and stops every other lane's cargo until killed (the runner gives up after 60 min). Fix: a per-call
  timeout in wasmStep's `call` (e.g. 300 s, the budget) so a hang is a named FAIL.
- INF-5 Low, for the main session: a counted run on the merged head is about 9.5 min (m5-counted.log) + about 0.8 min
  wasm step + the six pre-builds (24-49 s) = about 11 min, over the 600 000 ms foreground Bash limit: round 2's and
  the merge's counted runs go in the background (U4e's M5 used >= 1,500,000 ms). Re-time the wasm step from that
  run's printed split: npt-lab's release build now carries U4e's npt-room, npt-server and npt-client code.
- INF-6 Low, for U7's brief (C-U7-1): U7's Chromium leg and its wasm-opt rerun have no runner yet: no CI job (R-8;
  gate.yml), no Chromium in this container (gate-full.log:7), no wasm-opt or wasm-bindgen installed. Each needs a
  pinned tool download (a decision for U7's brief); a CI home would be .github work (tier 2, Mustafa's go). Time cost
  small against the budget (estimate).
- INF-7 Low, follow-up (docs-keeper at U4's close): README.md:16-22 lists check.mjs's steps and the wasm32 clippy
  crates without the wasm step, npt-hash-wasm or its prerequisites (wasm32 target via rust-toolchain.toml, Node 22,
  glibc for the std-sin data, one online `cargo fetch --locked` before offline runs). README was stale before U4d2
  ("15 crates"; 16 at <base>, 17 now).

## Verdict
APPROVE WITH FINDINGS: the step's warm time is 42.9-46.5 s over six runs (mine 45.4 and 43.8), about 15 % of the
5-minute budget; a cold run is bounded at about 80 s on this machine; memory under 0.9 GB; disk 40 MB per checkout,
no growth of the shared target across runs. INF-1 is report text before the merge; the rest are follow-ups.
