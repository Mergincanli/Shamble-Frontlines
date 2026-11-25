# U7's Block 0b: the binding check (the main session, 2026-11-25, the hour U6c merged)
Rules: security-u7-deps.md items (7)-(10) and its "Block 0b: the binding check" steps 0-8 (:281-316), with the
advisor's C6 and C10 (advisor-u7-brief.md:167-173, :197-220): three dev-dependencies, a four-line lock diff; only a
difference from (7)-(10) goes back to security. Every cargo call through main-run.sh (CARGO_NET_OFFLINE=true, the lane
lock, the shared target), from spikes/netcode-proof. The approved dev part-clean ran first (05:1x UTC: 6.4 GiB, 5,403
files; free 4,536 -> 10,447 MiB), so the before-run is cold.

## Step 0 (05:2x UTC): PASS
- `sha256sum Cargo.lock` 3394b451b06492758621445c250f44e93572e0b33a0269e68b7c9021b0f54873 = U6c's merged lock as
  dependency-check.md's U6c section records it. It is not the read's model (ea6efb00...9111), so the read's after-lock
  hash (8a2ebb34...) is void and only the four-line rule binds (item (8)).
- `grep -c '^\[\[package\]\]' Cargo.lock` 252; web-sys and wasm-bindgen-futures 0; `^name = "wasm-bindgen"$` 1.
- npt-client-wasm's block holds the one line ` "npt-client",`.
- crates/client-wasm/Cargo.toml sha256 c68877f33877359caf2a30965b94015f6a3f8f3f9542d48273cd6c5d7d0fab20 (= the read's
  c68877f3...ab20: U6c did not change it); Cargo.toml 7992c8e10da940a2f610be7223bcf0d6d3e4c2e31c7b78a61edbe2e88925ca03.

## Step 5's before-run
- `node check.mjs --only test,deps` before step 1 (.gate/u7-0b/before.log).
