# Stop B: the two dist builds differ (C1's bar failed, 2026-11-27 22:5x UTC): for the advisor agent before X
The plan's C1 bar: build 2 at Y must equal Y's committed list byte for byte. It does not.

## The facts (logs in .gate/stopb/)
- Build 1 at Y0 (87cb8677), worktree /home/user/Shamble-Frontlines/.gate/stopb-a, CARGO_HOME .gate/stopb-a/.gate/cargo,
  22:28-22:31 (build1.log; list build1-dist.sha256 = Y's committed list).
- Build 2 at Y (652e9513), worktree /home/user/Shamble-Frontlines/.gate/stopb-b, CARGO_HOME .gate/stopb-b/.gate/cargo,
  22:50-22:52 (build2.log; list build2-dist.sha256). Same tools: rustc 1.99.1, wasm-bindgen 0.2.130 (each freshly
  installed), node v22.23.4, npm 10.9.9.
- Six of seven files equal. Line 6 differs, the Wasm:
  - build 1: 2a97394b023c5f52e7c2dcbcbb28204dd9bcf23bc4c0acaaf5f3c9191e4c4551  ./npt_client_wasm_bg.wasm
  - build 2: a94698198f7345a041c7c547dd09e9d2c4a5c083b0e523701dbab9e022f05059  ./npt_client_wasm_bg.wasm
  - U7's run 4 (u7-report.md:141, its builder worktree): 901a6492..., a third value.
- Deterministic at one path: build 2's wasm target cleaned (`cargo clean --locked --target-dir`, 270 files), then
  build.mjs re-run at the same path and CARGO_HOME: a94698198f73... again (build2-again.log).
- No path text in the Wasm: build 2's wasm (143,200 bytes) holds none of "stopb-b", "stopb-a", "/.gate/", "home/user",
  "registry/src", "index.crates.io", "Shamble"; "/rustc/" once (std's own remap). build.mjs:164's home-path scan
  passed both times.
- The two worktree paths have the same length (stopb-a, stopb-b): the difference is in a value derived from the paths,
  not their length.

## Reading (not proven)
build.mjs:69-74 passes `--remap-path-prefix=<CARGO_HOME>=/cargo-home`, `<spike>=/npt` and `<target>=/npt-target`
in CARGO_ENCODED_RUSTFLAGS: the real paths are inside the flags, and cargo's per-crate hashes (`-C metadata`, symbol
disambiguators) can depend on the flags or on path sources. So the Wasm depends on the build's absolute paths even
though no path survives in it. RS builds at /home/npt-build/clone/spikes/netcode-proof with CARGO_HOME
/home/npt-build/.cargo (rs.yaml:174-200): a third path, so RS's Wasm would match neither list. On R1's day
bring-up would record DIST-DIFFERS and no page would be served without his `dist-accept` word (r1.sh:130-131, :875).

## What this blocks
X (and so Stop B's OK), until the advisor agent rules. Y0 and Y are pushed; the list in Y is build 1's.
SEC-SB-5's two-path build was never run before this fill (u8-report.md:76), so this is its first result.
