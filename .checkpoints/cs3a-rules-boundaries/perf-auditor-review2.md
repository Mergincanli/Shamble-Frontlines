# perf-auditor, cs3a round 2 (fix pass 79666ad9 vs round 1's code head 5c92311; main 1ec6f6c)

PLAN: scope = `git diff 5c92311 79666ad9 -- packages/sim-worker/src packages/client/src crates/sim/src crates/sim-wasm/src`
only. (1) L-1/L-2: read messages.ts block check + comment; re-run round 1's link-bench (real WorkerLink, sampling
interval 1, the spec's window, plain + --clone) at 5c92311 vs 79666ad9 on copies under .gate/perf-auditor-cs3a/;
steady-state heap delta too if cheap. (2) new hot-path bits: the `instanceof ArrayBuffer` per block (bytes + ns),
host.ts catch + trapMessage (cold, code read), panic-message.ts, panic.rs (saturating_sub, private start), rng.rs
(confirm no draw path changed: diff + same words). (3) Wasm bytes 31,805 -> 31,718: sections diff (round 1's
wasm-sections.mjs) on a copy build; explain the -87. Worktree read-only; scratch and builds under .gate/perf-auditor-cs3a/.

## Notes (one line per measurement or finding)
- 15:54 UTC copies: `git archive <sha> -- packages/types packages/client/src packages/sim-worker/{src,package.json}` into
  .gate/perf-auditor-cs3a/r2-base (5c92311) and r2-fix (79666ad9) + node_modules/@shamble symlinks. In scope only
  messages.ts, host.ts, panic-message.ts, worker.ts (+ 2 tests) differ; client/src and types identical. r2-fix's
  messages.ts code = my round 1 trial (head-fix/), comments aside. Load 0.46.
- L-1, round 1's link-bench.mjs (real WorkerLink, the spec's window blocks 97..340, sampling interval 1 B, 2 runs
  each, deterministic): plain messages 5c92311 296.0 -> 79666ad9 280.0 B/block (isWorkerToPage 32.0 -> 16.0) = main's
  280.0 of round 1; structured-clone messages 305.8 -> 289.8 (-16.0; the +9.8 left vs main is the report check, L-2).
  Per frame 112.0 both. FIXED.
- Steady (optimised; heap delta, 512 MB semi-space, 200k blocks): clone 338.25 = 338.25 B per block (3 runs each),
  plain 328.22 = 328.22 (2 each): round 1's numbers; the check allocates nothing once optimised, before and after.
  ns/block in that loop 194..400 both sides (load from other reviewers): no time verdict from it.
- check-bench.mjs (isWorkerToPage on structured-clone block messages, both modules in one process, A/B in turn, 15 reps
  x 1M calls, never-optimised caller; both TurboFanned, status 1010001): medians 5c92311 22.49 / 21.71 / 21.74 ns,
  79666ad9 23.93 / 22.23 / 21.20 ns per call (3 processes): -0.5..+1.4 ns, inside the +-1.5 ns spread; at 30 blocks/s
  under 0.05 us a second. Mostly the call from the interpreter loop.
- "Now also asks instanceof ArrayBuffer": not new. main's worker-link.ts onMessage already tested
  `message["buffer"] instanceof ArrayBuffer && typeof message["doneAt"] === "number"` per block; 3a moved it into
  messages.ts; the fix pass leaves it as it was at 5c92311. doneAt reads per block: main 2, 5c92311 3, 79666ad9 2.
- 15:57 Wasm: copy build of 79666ad9 (r2-fix-rs = git archive of Cargo.toml/.lock, toolchain, crates; build.mjs's cargo
  line + remap flag, --offline, own target folder; the checked wasm-bindgen ca4df773...): clean 7.6 s; sim_wasm_bg.wasm
  31,718 B sha256 383f5303...3963 = the worktree's pkg byte for byte (glue .js/.d.ts equal too). 5c92311's: round 1's
  pkg-head-same 31,805 B, e138cae8...2bc5. Glue sim_wasm.js 9,588 -> 9,397 (-191), sim_wasm.d.ts 4,056 -> 3,880 (-176).
- Wasm sections (round 1's wasm-sections.mjs, 5c92311 -> 79666ad9): raw 31,805 -> 31,718 (-87), gzip -9 12,653 ->
  12,642 (-11), Brotli 11 10,796 -> 10,755 (-41). name -74 (panic_const_sub_overflow's name), data -17 (its 33-byte
  "attempt to subtract with overflow" gone; the rest not split further), function -1 entry, code +5 (Text::write_str
  363 -> 384 = +21 for saturating_sub + the checked add; panic_const_sub_overflow -16 gone). Every other function the
  same size; export section 311 = 311. FITS what changed.
- `private`: the Wasm still exports `start` (exports listed via WebAssembly.Module.exports, both builds); only the
  glue's JS `export function start()` went (sim_wasm.js -191 B, .d.ts -176). 0 bytes in the Wasm. Side note, not perf:
  init() returns the raw instance exports (glue line 191; worker.ts:21 keeps them), so `wasm.start()` stays callable;
  panic.rs's "the host cannot call it" is true of the glue's named exports only (harmless if called: re-sets the hook).
- rng.rs: not in the players' Wasm at all (no chacha/ChaCha/next_u32/rng name in either name section). Diff: a const
  assert (compile time, no code), docs, one test. No draw path changed.
- Runner (toolchain job logs, 1 job each: 45408571090 = 5c92311, 45425663247 = 79666ad9): Wasm 31,718 / 12,642 /
  10,755 + sha 383f5303 = mine. Production page JS 823,255 -> 823,228 (-27; gzip -5, Brotli -164); Worker chunk
  8,833 -> 8,853 (+20; gzip 3,446 -> 3,477, Brotli 3,118 -> 3,149: trapMessage, the room check, worker.ts's empty
  fallback memory; once per page load, 0.2 %). Development page JS 826,237 -> 826,210. Not a finding.
- allocation.spec on the runner: 525 (5c92311) -> 528 (79666ad9) B per block, 1 reading each (the gate workflow does
  not run it). Sampling interval 64 gives ~+-13 B a run (round 1: SD 10.2 / 13.6): one reading cannot see a -16 (or,
  in Chrome, likely -12) shift. Inconclusive by design, not a contradiction.
- 16:02 L-1 in Chromium's own V8 (browser-link-bench.mjs + browser-link-page.js: the same feed and window as
  link-bench, the repo's .ts type-stripped by a local server on a free port, messages by structuredClone; CDP sampling,
  interval 1, collected objects counted = the spec's method; headless Chromium 153.0.8010.12 from /opt/pw-browsers,
  not the runner's 157; 2 runs each, alternating): isWorkerToPage's own bytes 24.0 -> 12.0 per block (2 runs each,
  exact: 2 -> 1 boxed doubles, 12 B each with pointer compression; Node's are 16 B); with callees 32.1/31.4 ->
  19.4/19.4; under receive 181.9/180.1 -> 168.1/169.9 (-12.0 mean). isWorkerStats 7.4 B/block both (= 60 B a report,
  5 x 12: L-2's "a few bytes each" holds). Round 1's runner shift +19.3 B/block = 12 + 7.4 here: after the fix the
  expected spec reading is main + ~7.4 (~513); the one reading, 528, is ~1 SD over: noise.
- 16:03 Worker per wake (round 1's host-bench-root.mjs, real SimHost + each head's own Wasm copied into r2-base/r2-fix
  pkg): bytes (heap delta, 512 MB semi-space, 3 windows x 200k wakes, 3 processes each): 5c92311 421.56 x2 / 410.81
  x1, 79666ad9 410.81 x2 / 421.56 x1 = round 1's two JIT modes, 0 B/wake from the fix pass. Time (5 reps x 300k, 3
  processes): medians 197.4 / 187.5 / 169.1 vs 186.0 / 191.8 / 185.8 ns: no difference beyond +-15 ns process spread.
  --trace-opt --trace-deopt over 300k wakes: the same 46 events each side, 0 deopts, 0 bailouts.
- host.ts:137-141 catch -> trapMessage (panic-message.ts:30-34): runs once per match at most (#failed is set, every later
  #call returns at its first line); try body unchanged. worker.ts:57-59 fallback object: only when start() fails, once.
  Cold; allocation there (DataView, 2-item array, TextDecoder, the sentence) does not reach a tick or a frame.
- rng.rs at 79666ad9 on the copy: `cargo test -p sim --lib rng` 6 of 6 pass (the published TC1..TC8 keystream among
  them). Non-test code without comments, 5c92311 vs 79666ad9: the one const assert, nothing else.
- L-2: messages.ts:84-88 now says the report check "builds no list of its own; reading a report's fractional numbers
  still costs a few bytes each", "four times a second, in development builds". Checked: requestReport only behind
  __SHAMBLE_DEVTOOLS__ (boot.ts:80, 115), READOUT_EVERY_MS 250 (boot.ts:18); 60 B a report in Chromium. FIXED.
- Side note (not perf, Low at most, for rust-reviewer / engine-architect): panic.rs:73-74 "It is no export of the glue
  (`private`): the host cannot call it". The glue's named export is gone, but the Wasm still exports `start` and the
  glue's typings keep `readonly start: () => void` in InitOutput (pkg/sim_wasm.d.ts:70); init() returns those raw
  exports (glue line 191, worker.ts:21). A call would only re-set the same hook. Wording, not a cost.

## Verdict (round 2, code 79666ad9; branch head 94b7318f has no code change since)
VERDICT: OK. 0 Critical, 0 High, 0 Medium, 0 Low in perf (+1 Low wording side note outside perf). L-1 FIXED (per block
in the spec's window: Node 296.0 -> 280.0 = main, Chromium 153 isWorkerToPage 24.0 -> 12.0; 0 B steady both).
L-2 FIXED (doc true: 60 B a report in Chromium, dev builds only). Wasm -87 B raw / -41 Brotli, explained section by
section; Worker chunk +20 B raw once per load; per wake 0 B, same JIT events. Not measured: the spec on the runner more
than once per head; Chrome 157 (153 here); Brave/real GPU; the real postMessage/MessageEvent; the trap path's time.
