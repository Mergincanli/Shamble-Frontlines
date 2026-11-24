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
