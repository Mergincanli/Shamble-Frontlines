# perf-auditor, cs3a round 1 (head 5c923119, base origin/main 1ec6f6c)

PLAN: read AUDIT_BUGS_PERF + plan s4 + resume card 4-10; diff crates+packages; (1) host.ts per-wake path vs main: code read +
Node harness on the real SimHost with the built Wasm (ns/wake, bytes/wake, opt status, --trace-deopt), base and head;
(2) worker-link/messages per-block path: code read + Node harness (heap bytes per receive) + logs of allocation.spec;
(3) Ring.mean / FrameTimes.fps cost at 600 samples (Node microbench); (4) Wasm bytes raw/gzip/brotli, section sizes, against
ADR 0002 marks (PM-0002-29); (5) ChaCha8 words/s native release (scratch crate under .gate/perf-auditor-cs3a), copy vs no
copy; (6) build/CI times from logs + warm/clean timings of checkCrates' cargo tree; (7) anything else. Machine: this container.
Scratch and builds: .gate/perf-auditor-cs3a/ only.

## Notes (one line per measurement or finding)
- 14:33 UTC scratch: `git archive origin/main | tar -x` into .gate/perf-auditor-cs3a/main-src/ also copied main's
  CLAUDE.md, .claude/ and .github/ (inert, byte-identical copies, gitignored folder); the guard refused my `rm -rf` of
  them (correct: CLAUDE.md is protected). Left in place; the main session may delete .gate/perf-auditor-cs3a/main-src/.
- Machine: this container, 4 vCPU Intel Xeon 2.30 GHz, 16 GB; Node v22.23.5, rustc/cargo 1.99.1. Other reviewers run
  Playwright at the same time (load ~1 at 14:31 UTC): every timing below is repeated and its spread stated.
- (4) Wasm bytes, this container, my build of main (clean target .gate/perf-auditor-cs3a/target-main, same cargo line as
  main's build.mjs; sha256 c4935aca...2c31 = the card's) vs head's pkg (sha256 e138cae8...2bc5 = CI's): raw 28,987 ->
  31,805 (+2,818, +9.7 %), gzip -9 11,655 -> 12,653 (+998, +8.6 %), Brotli 11 9,955 -> 10,796 (+841, +8.4 %); sizes are
  deterministic (1 run each). By section (wasm-sections.mjs): code 18,409 -> 19,838 (+1,429), data 2,568 -> 2,869 (+301),
  name 6,954 -> 7,997 (+1,043), function/export/element +45. New functions: Text::write_str 363, PanicHookInfo Display
  327, Text::write_char (std default, write_str inlined) 229, set_hook 148, the closure 121 + its vtable shim 121, rest
  < 25 each. Name section = 25.1 % of the file; 2,565 of its bytes (32 %) are v0 crate hashes "[0123456789abcdef]".
  Without the name section: 23,808 raw / 10,874 gzip / 9,283 Brotli (saves 7,997 / 1,779 / 1,513: more than the hook adds).
  ADR 0002 part 4 + PM-0002-29: size is "reported, no pass number"; gated only "every served file under 25 MiB"
  (26,214,400 B): 31,805 is 0.12 % of it. The hook is required by ADR 0002 part 4 / ADR 0004 part 20 (S20). Not a finding.
- (6) clean build of main's players' Wasm here: 6.9 s wall (9.1 s user), 1 run, load ~0.1.
- (1) host.ts per wake, Node harness on the real SimHost + real Wasm (main: my build of main; head: the worktree pkg);
  .gate/perf-auditor-cs3a/host-bench-root.mjs: one tick per wake, the harness loop never optimised (%NeverOptimizeFunction)
  so each wake is the root of its own optimised code, as the Worker's timer callback is; the posted message escapes.
  Time, 5 processes x 5 reps x 300k wakes each side, medians: main 193.4/196.9/186.4/186.2/184.3 ns (median 186.4),
  head 195.5/189.0/189.6/190.6/175.9 ns (median 189.6): +3 ns, inside the +-10 ns spread between processes. At 30 Hz
  even +20 ns would be 0.6 us of Worker time a second.
  Bytes (heap delta, 512 MB semi-space, gc() before, 3 windows x 200k wakes, 5 processes each): main 410.81 x4, 421.56
  x1; head 410.81 x2, 421.56 x3: the same two JIT modes to the hundredth of a byte, so the guard adds 0 bytes a wake.
  After 20 hide/show toggles (resume through #call at head): 410.81 B/wake in 4 of 4 processes each side.
  Deopt (--trace-opt --trace-deopt, toggle run): the same events at main and head, once at the first hide/show:
  onMessage + isPageToWorker "wrong map" (the new message shape), #wake + Pacer.wake "code dependencies" (head also
  #runTicks); all re-optimised, no loop. #call never appears as its own compile: inlined into #wake. No new deopt.
  Side note (not 3a): a harness that inlines the wake into its own loop shows modes of 276/332/406 B/wake (head hit
  406 in 4 of 8 processes, main 0 of 8): where TurboFan roots the compile, not the guard (root harness above equal).
  Pre-existing per tick on the Worker (main too): copyBlock's two Uint8Array views 192 B + the block message ~84 B.
- (2) page per block: runner (CI logs) main-era 13 runs of #64 = 487..523, mean 505.5, SD 10.2; 3a heads 6 runs = 507,
  511, 525, 528, 539, 539, mean 524.8, SD 13.6: +19.3 B/block (Welch t 3.1, p ~0.02). Container: main 506 (1 run),
  3a 516/523/525/531/537. Node harness link-bench.mjs (real WorkerLink, main vs head; events made before the window,
  so it isolates the link's own JS; the feed loop never optimised; 30 blocks : 60 frames : 3.75 reports; window =
  the spec's blocks 97..340 from a cold start; sampling interval 1 B, deterministic, 2 runs each): main 280.0 B/block,
  head 296.0 (+16.0): isWorkerToPage 32.0 + onMessage 16.0 at head vs onMessage 32.0 at main. Cause: `doneAt` is a
  double field; each read in unoptimised code boxes a fresh 16-byte HeapNumber; head reads it 3 times per block
  (messages.ts:142 twice: typeof + Number.isFinite; worker-link.ts:147 once), main twice (worker-link main:88-89).
  Trial in scratch (head-fix/: messages.ts:142 -> `value["buffer"] instanceof ArrayBuffer && Number.isFinite(value["doneAt"])`):
  280.0 B/block = main's, 2 of 2 runs. Steady state (after 100k blocks, optimised; heap delta): 328.22 B per block incl.
  2 frames + reports, main = head, 3 runs each: the check allocates nothing once optimised. No deopt in the link path
  (--trace-deopt, both). Time per block with 2 frames: main median 194.5 ns (165..198), head 209.1 (183..253), 5 each:
  +15 ns, 0.5 us a second at 30 blocks/s.
  FINDING L-1 (Low): messages.ts:142 reads doneAt twice; +16 B a block while the listener is unoptimised (the spec's
  window), which explains the shift of the readings. Window still holds: runner max 539 vs limit 610 (71 B room, ~5 SD);
  a planted 64-byte typed array (~+215) still lands far above. Fix: drop the redundant typeof (Number.isFinite refuses
  non-numbers without coercion). Ride this change-set's fix pass (one line; a Low never triggers a push alone).
- (3) ring-bench.mjs (real Ring, a full ring of 600, Node, 3 runs): head's fps() = mean() 0.34 us a call (3 of 3 runs
  0.34); main's fps() = percentile(50) 4.90..5.09 us and ~241 B a call (copy + sort; main-src 5.69). At 4 readouts a
  second: 1.4 us/s at head vs ~20 us/s at main: 3a makes fps() ~15x cheaper and drops one of the readout's five sorts.
  A running sum would save 0.3 us four times a second and add drift on wrap: not worth it. Dev builds only. Not a finding.
- (5) ChaCha8 (rngbench/: the shipped crates/sim/src/rng.rs via #[path], and a copy whose next_u32 reads the cached
  word in place, `match &self.block`; same words asserted; release, overflow checks on as the workspace). Native x86-64
  here, 2 runs x 7 reps, medians: one stream 4.67 / 4.37 ns a word as shipped vs 4.13 / 3.94 in place; 48 streams
  drawn in turn 5.12 / 4.76 vs 4.37 / 4.24 (195..229 M words/s shipped). Wasm (bare wasm32 cdylib in Node 22): one
  stream 8.95 / 8.46 vs 9.83 / 7.16 (noisy); 48 streams 8.71 / 7.85 vs 8.36 / 6.27 ns a word. The copy is ~0.4..0.8 ns
  a word native, up to ~1.5 in Wasm (10..20 %). At ADR 0004's scale (N = 48, a few draws a shot; even 1,000 draws a
  tick) that is ~9 us a tick in Wasm, 0.07 % of the 13.33 ms sim share; the copy ~1.5 us. Not a finding now: with
  PM-0004-27's cost read in change-set 8 (rust-reviewer's L4), fix there (read in place; same words).
- (6) CI, the runner's job logs (ci-steps.py + phase stamps; 1 job each): job log 134.1 s (a1c59e8), 134.3 (50656d6)
  before; 164.5 (df72a58), 137.3 (5c92311) after. Build phase ("== the page as players get it" -> "development page
  in"): 8.2 / 6.9 s before -> 25.7 / 22.0 s after. The test-only module's two clean cargo builds + bindgen: 16.2 s
  (df72a58: release 7.51 + dev 8.44) and 14.0 s (5c92311: 6.56 + 7.26); in its own target folder syn and the
  wasm-bindgen macros compile again for each profile. checkCrates: the whole gap from the build's start to cargo's
  first line is 0.25 s on the runner (node + tool check + --version + the 4 cargo tree), twice a job. Browser tests
  12 -> 14: 64.7 / 62.3 s -> 69.0 / 59.4 s (Playwright's own lines; runner noise +-5 s). Net: about +14..16 s a job
  (+11 %), inside the runner's +-15 s spread between jobs; the job stays under 3 minutes.
