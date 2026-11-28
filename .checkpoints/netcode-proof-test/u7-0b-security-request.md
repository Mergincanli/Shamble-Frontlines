# U7's Block 0b: three questions for security-auditor (the main session, 2026-11-25 05:5x UTC; to send at 08:50)
Your read: security-u7-deps.md (Block 0b CLEAR WITH CONDITIONS; its binding check :281-316; C10: only a difference
from items (7)-(10) comes back to you). The evidence so far: u7-block0b.md (steps 0-4 and 6 PASS; step 5 running).
Answer in `.checkpoints/netcode-proof-test/security-u7-0b-answer.md`, each question by number, ending with DONE.

## What passed (details and logs in u7-block0b.md)
- Step 1: the two manifests exactly as item (7) (+3 in Cargo.toml, +6 -1 in crates/client-wasm/Cargo.toml).
- Step 2: `cargo metadata --format-version 1 --offline` exit 0, no network.
- Step 3: `4	0`; the four lines in npt-client-wasm's block only; 252 packages; sha256 55604a5f...9f5d (your 8a2ebb34 line void:
  step 0 read 3394b451..., U6c's merged lock, not your model).
- Step 4: all five trees as written (features default and std only; one shipped dependent; the 20 names; the three
  dev-dependencies).
- Step 6: cargo-deny 0.22.0 "advisories ok, bans ok, sources ok", the same seven duplicate names as U6c's run, no new
  one; cargo-audit 0.23.0, 252 crate dependencies, 0 vulnerabilities, one allowed warning RUSTSEC-2026-0441. Both
  databases fetched 05:44 UTC today; newest commit in each 2026-11-24T22:47:44+01:00.
- The redact step on the new lock (u7-brief.md:258-262, F26's line): `redact-check: 371 item(s) checked, 0 finding(s)`.
- Step 5 (cargo check 0 warnings; check.mjs test and deps equal to the before-run's 861/958): running; its result
  goes into u7-block0b.md before you read it, or comes in the launch note.

## Q1. Step 7's install needs one download. May it run with the network on?
- Your 0b header (:281-282) says every cargo call runs through the runner with CARGO_NET_OFFLINE=true. Item (10) gives
  the install (`cargo install wasm-bindgen-cli --version 0.2.130 --locked`, default features, CARGO_TARGET_DIR and
  `--root` under .gate/, `--bin wasm-bindgen`) and the .crate's sha256 c37f6904...fc7a in ~/.cargo/registry/cache.
- Offline it cannot run: exit 101, "error: could not find `wasm-bindgen-cli` in registry `crates-io` with version
  `=0.2.130`" (.gate/u7-0b/step7-cli-offline.log). The .crate is not in the registry cache; nothing was downloaded
  or built.
- Options: (a) the same command once with the network on (CARGO_NET_OFFLINE unset for that one call, the same flags,
  through the runner, its own target and root under .gate/u7-0b/); then the .crate's sha256 and `--version` checked,
  and on a mismatch the root removed and you told; (b) as (a), but the .crate fetched and its sha256 checked before
  any build (name the command; a fetch outside cargo is another network path); (c) no install in this container:
  build.mjs's runs wait for RS (U8). My lean: (a), since the read cleared this exact install, and cargo itself checks
  the download against the index's checksum before building; tell me if the order of the sha256 check matters to you.

## Q2. May step 8's commit go before step 7?
- Step 8: dependency-check.md's section, then the save that pushes the two manifests and the lock (DEP-HOLD).
- Why ask: U7's R-half builder starts from a base that holds 0b, in the 08:50-09:20 UTC window (new agents only in
  the first 30 minutes after a usage reset). Blocks R1-R4 need the wasm-bindgen crate (in the lock), not the CLI; the
  CLI is first used by build.mjs (u7-brief.md:59-63) in Block T5 (ii), after R3 (:129-132).
- Options: (a) yes: the section records steps 0-6 and the redact line now, and the CLI's lines (version, .crate
  sha256) as an addendum after Q1's install; (b) no: the commit waits for step 7. My lean: (a).

## Q3. A command form for the database dates
- Step 6 asks for `git log -1 --format=%cI` run inside ~/.cargo/advisory-db and cargo-deny's folder (this repo's hook
  refuses `git -C`). I ran `env -C <that folder> git log -1 --format=%cI` on the two database folders (read only, not
  this repository). U6c's record used the fetch times instead.
- Options: (a) `env -C` is fine for those two read-only calls; (b) record the fetch times only, as U6c did. Until you
  answer, I use neither form on this repository or its worktrees.
