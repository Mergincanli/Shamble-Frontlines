# U4d2 review round 1: infra-engineer seat (notes)

Started 2026-11-12 (about 20:40 UTC). Fresh start (no notes file, no r1-inf-* logs existed).
Copy under review: /home/user/Shamble-Frontlines/.claude/worktrees/u4d2-build (uncommitted on 51520b4); read copy .gate/u4d2-r1.
Runner: .checkpoints/netcode-proof-test/u4d2-run.sh; my log prefix r1-inf-. No Edit tool here: this file is rewritten
whole with Write each time (the brief's rule).

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

## Read so far (sources, with what each shows)
- u4d2-run.sh: touches every file under crates/ at start and every 2 s (so every cargo call rebuilds the workspace's
  npt-* crates; deps stay cached); -x exclusive flock; waits for no cargo/rustc; 4 GB free floor; offline.
- check.mjs (build copy) :2009-2362 wasmStep; :2016 WASM_STEP_DIR = <checkout>/.gate/netcode-proof-test/wasm-step;
  :2124-2127 io.reset = rmSync(dir, recursive) then mkdir at the start of every run (kept after the run, cleaned at the
  next); :2031-2050 four release builds (npt-lab x86-64 measured + bites; npt-hash-wasm wasm32 measured + bites);
  :2183-2192 artifact path read from cargo's JSON, copied to bin/<copy> (no .exe suffix: "npt-lab", "npt-lab-bites");
  :1767-1772 run() = spawnSync, maxBuffer 256 MiB, no timeout; :2132-2135 libc from process.report's glibc field.
- .gitignore: `.gate/` ignored (step folder never committed). rust-toolchain.toml: channel 1.99.1, minimal, clippy +
  rustfmt, targets wasm32-unknown-unknown. .cargo/config.toml: target-dir ../../.gate/netcode-proof-test/target
  (the runner's CARGO_TARGET_DIR overrides; the step reads the artifact from cargo's JSON, so either works).
- Builder logs: b8-wasm1.log 18:20:20-18:21:03 UTC, step 42.9 s; b8-wasm2.log 18:21:35-18:22:19, 43.5 s. Split (run 1):
  builds 6.2 + 6.6 + 1.5 + 1.6 = 15.9 s; npt-lab hash 4.4; Wasm legs 3.5 + 1.2 + 0.1; hash-map 3.0 + 3.3 + 3.5 + 0.7
  + 0.4; std-sin 3.2 + 3.5 + 0.1; the rest 0.0. Runner provenance empty (check.mjs keeps cargo's output in memory).
- Builder record u4d2-build.md:171-178: run 1 "the release deps were already in the shared target from my by-hand
  runs" -> no cold figure was taken; both of its "cold/warm" runs are warm for deps.
- First release builds in this lane: b2-hw-test3.log 16:40:46 wasm32 npt-hash-wasm "Finished in 3.57s" (compiled only
  npt-sim + npt-hash-wasm: wasm32 deps already there); b3-lab1.log 16:47:56 npt-lab release "Finished in 16.04s"
  (compiled serde_core, zmij, serde_json, serde, serde_derive, memchr + 13 workspace crates; tokio/quinn/ring/rustls
  already there).
- A cold bound on this machine (4 vCPU Xeon 2.1 GHz, 15 GB, no swap): .gate/rr3-unit.txt (2026-11-07, a fresh
  rr3-target): 74 crates incl. ring, rustls, tokio, quinn-proto, tungstenite, release, "Finished in 20.33s";
  .gate/netcode-proof-test/u3vf-diag1.txt the same tree 21.27 s. The rust seat's r1-rr-superset-build.log (fresh
  .gate/u4d2-rr-target, wasm32 release of npt-hash-wasm, zerocopy/libm/... compiled): "Finished in 6.05s".
- Disk now (20:38 UTC): shared target 6.9G (debug 6.4G, release 264M, wasm32 159M of which release 40M, doc 21M);
  step folder 40M (recordings 33M, bin 2.6M, measured 2.6M, hash-map 1.3M, std-sin 1.0M); the builder's own scratch
  120M (not the step's). df: 7.4G free of the volume.
- glibc: `readelf -Ws --dyn-syms /lib/x86_64-linux-gnu/libm.so.6`: sinf@@GLIBC_2.2.5 is IFUNC (chosen at load by CPU
  features); this CPU has fma. std-sin-scan.txt records rustc, ldd's glibc 2.39 and Node v22.23.4, not the CPU.
- Node v22.23.4 here. loader.mjs: sync `new WebAssembly.Module`, empty imports, Object.hasOwn; nothing past Node 16.

## Runs (mine)
(next: r1-inf-wasm1.log, r1-inf-wasm2.log, both -x)

## Findings (a line after each)
