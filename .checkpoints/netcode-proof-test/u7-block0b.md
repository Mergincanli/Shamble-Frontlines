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
- **To tell:** Mustafa in this chat's next message (about 06:0x UTC) and in the day message msg-2026-11-26.md; the
  advisor at its next check (NB2's head read or U7's round, whichever comes first).
- **The rule from now on** (the advisor tool, 05:4x): a command naming .gate/shared-target goes out only if it equals
  the card's Tools line character for character but for the profile word (`--profile dev`, `--profile release`,
  `--doc`); never `--target <triple>`; the wasm half only as advisor-u4d2-m9.md's six lines give it (`--target-dir
  <shared>/wasm32-unknown-unknown --profile ...`). On another exit 3: stop, `df`, and pick from that list only.

## Step 1, redone (05:3x UTC)
- The same two edits by Edit, byte for byte as the first try: Cargo.toml +3 (the comment's two lines and
  `wasm-bindgen = "=0.2.130"` after `http-body-util`); crates/client-wasm/Cargo.toml +6 -1 (`wasm-bindgen.workspace =
  true` in place of the "joins at U7" comment, then `[dev-dependencies]` npt-room, npt-server, npt-sim).
- `git diff --stat`: those two files only, 9 insertions, 1 deletion.

## Order of the remaining steps (a choice: the advisor tool's, 05:4x; for the day message)
2, 3, a deps-only check, 4, 6, 7, then 5 (the cold after-run, about 8.5 GB) and 8, so the disk-heavy run comes last and
the CLI's own build folder is cleaned before it. Each step checks the same things as written; none reads another's output.

## Step 2 (05:43 UTC): PASS
- .gate/u7-0b/step2-metadata-2.log (a new name; step2-metadata.log keeps the 05:27:03 stop): free 13,265 MiB, HEAD
  1a56082, diff 715bd28fca7de9489e7972659308bebb612a2d70a3725df4352aae99165e77bc = the first try's header (so the redo
  is byte for byte the same), untracked e3b0c442...; `cargo metadata --format-version 1 --offline` exit 0, no network.

## Step 3: PASS
- `git diff --numstat -- spikes/netcode-proof/Cargo.lock`: `4	0`. `git diff -U0`: one hunk at :911, the four lines
  ` "npt-room",` ` "npt-server",` ` "npt-sim",` ` "wasm-bindgen",` after ` "npt-client",` in npt-client-wasm's block
  (:907-915); no package, version, checksum or source line. Packages 252 (= step 0). sha256 55604a5fae45e50ee0fb262d743bc9e967b7caa3b23482de3b61de4cb28b9f5d
  (the read's 8a2ebb34... line is void: step 0 read 3394b451..., not the model).
- The deps-only check (.gate/u7-0b/step3b-deps.log, 05:43:41-43): PASS 2.6 s, "check: all 1 steps pass", header diff
  0b5b2289... (now with the lock).

## Step 4 (05:43:52-53 UTC): PASS (.gate/u7-0b/step4-trees.log, five calls, each exit 0)
- `-e features -i wasm-bindgen`: wasm-bindgen's feature lines exactly "default" and "std"; dev-edge dependents getrandom
  (0.2.17 and 0.4.3), js-sys, web-time (SEC-U7D-2's expected lines).
- `-e normal,features -i wasm-bindgen`: one package dependent, npt-client-wasm; features default and std only.
- `-e normal --prefix none`: 20 names = npt-client-wasm, today's nine (npt-client, npt-link, npt-proto, npt-sim, libm,
  rand_chacha, ppv-lite86, zerocopy, rand_core) and wasm-bindgen, cfg-if, wasm-bindgen-macro, wasm-bindgen-macro-support,
  wasm-bindgen-shared, bumpalo, proc-macro2, quote, syn 3.0.9, unicode-ident; no js-sys, web-sys, wasm-bindgen-futures,
  serde, serde_json, getrandom or tokio.
- `-e dev --depth 1`: npt-room, npt-server, npt-sim. `--workspace --all-features --target all -e features -i
  wasm-bindgen --depth 1`: wasm-bindgen features default and std only.

## Step 6 (05:44:41-48 UTC): PASS (.gate/u7-0b/step6-deny-audit.log)
- cargo-deny 0.22.0; cargo-audit 0.23.0 (it prints "cargo-audit-audit 0.23.0").
- `cargo deny --locked check advisories bans sources`: exit 0, "advisories ok, bans ok, sources ok"; warnings only the
  seven duplicate names U6c's run had (u6c-block0.md:63, :75-76): base64, getrandom, pem, syn, synstructure,
  windows-link, windows-sys. No new one.
- `cargo audit --file Cargo.lock`: 1567 advisories loaded, 252 crate dependencies, 0 vulnerabilities, "1 allowed
  warning found": RUSTSEC-2026-0441 (pin-utils, unmaintained). No new one.
- Databases: both fetched by the run (FETCH_HEAD 05:44:47 UTC ~/.cargo/advisory-db, 05:44:45 UTC
  ~/.cargo/advisory-dbs/advisory-db-3157b0e258782691); newest commit in each 2026-11-24T22:47:44+01:00 (`git log -1
  --format=%cI` run inside each folder by `env -C`).

## Step 7, offline (05:46:32 UTC): cannot run offline
- Through the runner (CARGO_NET_OFFLINE=true, as the read's 0b header says), with its own CARGO_TARGET_DIR and --root
  under .gate/u7-0b/: exit 101, "error: could not find `wasm-bindgen-cli` in registry `crates-io` with version
  `=0.2.130`" (.gate/u7-0b/step7-cli-offline.log). The CLI's .crate is not in ~/.cargo/registry/cache, so item (10)'s
  install (and its .crate sha256 check) needs one download. Nothing was built or downloaded.
- **Not run on my own reading** (the advisor tool, 05:5x): the read's header says offline and C10 sends a difference
  back to security. The question goes to security-auditor first in the 08:50 window: u7-0b-security-request.md (Q1 the
  install with the network on, Q2 step 8's commit before step 7, Q3 the `env -C` form used for the database dates).
  Blocks R1-R4 need the crate, not the CLI; the CLI is first used by build.mjs in T5 (ii), after R3 (u7-brief.md:59-63,
  :129-132).

## The redact step on the new lock (u7-brief.md:258-262, F26's line): PASS
- `node spikes/netcode-proof/ops/operator/redact-check.mjs spikes/netcode-proof` (05:5x UTC): "redact-check: 371
  item(s) checked, 0 finding(s)" (M5's count was 371 too).

## Step 5 (05:50:34-06:09:08 UTC): PASS
- One runner call: `cargo check --workspace --all-targets --locked :: node check.mjs --only test,deps` ->
  .gate/u7-0b/step5-after.log; header free 13,263 MiB, HEAD ced7e99, diff 0b5b2289... (= steps 3b, 4 and 6),
  untracked e3b0c442...
- `cargo check`: Finished in 29.00 s, exit 0; 0 lines with "warning" in its section (awk over the section, grep -c).
- check.mjs: PASS test 1082.5 s (measured 861 passed, 0 failed, 0 skipped; bites 958 passed, 0 failed, 0 skipped);
  PASS deps 2.6 s; "check: all 2 steps pass"; exit 0. The PASS lines equal before.log's with the timings removed
  (`diff` of the two, each through `sed -E 's/ +[0-9.]+ s / /'`: no difference).
- Free after: 4,250 MiB (df -m).
