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

## Step 5's before-run: PASS (05:08:54-05:26:32 UTC)
- `node check.mjs --only test,deps` before step 1 (.gate/u7-0b/before.log): header free 10,445 MiB, HEAD e88419e, diff
  and untracked both e3b0c442... (the empty hash: no spike change); test PASS 1055.5 s (measured 861 passed, 0 failed,
  0 skipped; bites 958 passed, 0 failed, 0 skipped); deps PASS 2.3 s; "check: all 2 steps pass"; exit 0.

## Step 1, first try (05:2x UTC), and the shared-target incident (my mistake)
- The two manifest edits were made by Edit; step 2's first call stopped at main-run.sh's floor: free 1,919 MiB, "STOP:
  under 4 GB free" (.gate/u7-0b/step2-metadata.log, 05:27:03; the cold before-run used about 8.5 GB).
- To free space, with no lane running: allowed cleans of finished units' own folders (CHECKPOINTS.md §10:
  .gate/u4e/rr-bench/target and target-dbg, .gate/netcode-proof-test/u3-verify-timing-probe/target and
  timer-probe/target), then the release part-clean of the shared target (`--profile release`, under the lane lock).
- **Then the forbidden form:** `cargo clean ... --target-dir .gate/shared-target --target x86_64-unknown-linux-gnu`,
  which advisor-u4d2-m9.md:228-229 names as the folder-level form ("which `--target <triple>` alone also is"). It
  removed the whole shared-target folder, about 9.7 GiB: everything under it, .lane.lock and CACHEDIR.TAG included.
- **Recovery, by that ruling's own method (:247-251):** the two manifest edits reverted by Edit (spikes/ clean against
  HEAD), no lane or agent running; from spikes/netcode-proof `CARGO_TARGET_DIR=<shared> CARGO_NET_OFFLINE=true cargo
  build --locked -p npt-log` (Finished in 5.56 s); `test -f <shared>/CACHEDIR.TAG` OK; `ls -A` shows .rustc_info.json,
  CACHEDIR.TAG, debug. No mkdir. The lock's file comes back at the next main-run.sh call (`exec 9>`, main-run.sh:27);
  a wasm32 folder under it, with its tag, comes back at the next wasm32 build (cargo makes it).
- **Cost:** no logs, notes or commits lost (the shared target holds build output only); every build cache is cold
  again: this check's after-run, U7's R-half builder and U8's builder each start cold (about 20-40 min of build each,
  one lane at a time). Free disk after it: about 13.3 GB.
- **Told:** Mustafa (this chat, and the day message msg-2026-11-26.md); the advisor at its next check (NB2's head read
  or U7's round, whichever comes first).

## Step 1, redone (05:3x UTC)
- The same two edits by Edit, byte for byte as the first try: Cargo.toml +3 (the comment's two lines and
  `wasm-bindgen = "=0.2.130"` after `http-body-util`); crates/client-wasm/Cargo.toml +6 -1 (`wasm-bindgen.workspace =
  true` in place of the "joins at U7" comment, then `[dev-dependencies]` npt-room, npt-server, npt-sim).
- `git diff --stat`: those two files only, 9 insertions, 1 deletion.
