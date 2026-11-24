# security-auditor: DEP-HOLD answer for cs3a's one batch of manifest changes (2026-11-24)

Plan: hash the four files; read both diffs and the four files; check (a) lockfile entries, (b) what sim-wasm's release
Wasm compiles (features, resolver 3, profiles), (c) root Cargo.toml overflow overrides, (d) ADR 0002 part 3 and the
routes to a player, (e) @shamble/types imports, (f) CI's offline steps (read only), (g) anything else; answer last.
Rules kept: no build, install, fetch or writing command; only this file is written. No agent memory exists here
(no .claude/agent-memory*/security-auditor folder), so nothing earlier to load.

Start hashes (sha256sum, matched deps-sha256.txt): devtools/Cargo.toml 8812561a...874e · Cargo.lock 8f232221...3c5b ·
sim-worker/package.json c2286286...b06 · package-lock.json 30ccbd5e...e474. Head 50656d6, branch cs/3a-rules-boundaries.
git status: the four files plus .checkpoints/.../plants.py (a scratch file, not a manifest) modified; nothing else.

## Findings (one line each, as found)
- (a) Cargo.lock: +5/-0, all inside the devtools entry (Cargo.lock:21-28): sim (path, :65-67), sim-wasm (path, :69-76),
  wasm-bindgen 0.2.130 (registry, checksum a0181c14..., :147-157); no version, source or checksum line changed; the 16
  third-party entries are the same. package-lock.json: +1/-1 at :1259 (the key only); no version, resolved or integrity
  line changed; the file parses (83 entries). Verdict: no outside package enters or changes. Sure: high.
- (b) Player Wasm = `cargo build --locked --release --target wasm32-unknown-unknown -p sim-wasm` (build.mjs:64-68),
  bindings from release/sim_wasm.wasm (:42, :71). devtools is a dependent, not a dependency (sim-wasm/Cargo.toml:12-15;
  Cargo.lock:69-76 unchanged). Resolver 3 (Cargo.toml:4) unifies features over the packages one command selects; even
  with devtools selected it asks wasm-bindgen's default set (devtools/Cargo.toml:17 -> Cargo.toml:16), the set sim-wasm
  asks (:15); sim and sim-wasm have no [features]. No profile line changed (root unmodified; a member's [profile] would
  be ignored). pkg/sim_wasm_bg.wasm read today: c4935aca...c31 (the builder's "before"). Caveat: this tree now also
  holds steps 4-5's unbuilt sim code (crates/sim/src/rng.rs, lib.rs), which is in sim-wasm's graph -> condition 1.
- (c) No crate joins. Target-side third-party crates in a library or test: cfg-if, wasm-bindgen, wasm-bindgen-shared,
  unicode-ident (sim-wasm, now devtools), thiserror, ts-rs (protocol's tests): each named (Cargo.toml:38-54). The rest
  are proc-macros or their host deps (rustversion, wasm-bindgen-macro(-support), bumpalo, proc-macro2, quote, syn x2,
  thiserror-impl, ts-rs-macros): "*" by the rule (:36-37). devtools is a member: :29-30 covers it. Sure: high.
- (d) Allowed: ADR 0002 :194 devtools "anything"; the bans (:207-212) are wasm-bindgen in room/client-endpoint and
  sim-wasm -> navbake/server; sim has no dependencies, so :217-222's feature comparison has nothing to compare yet.
  No route to a player today: nothing builds devtools for wasm32 (toolchain.yml:160; build.mjs:66); Vite emits what
  the Worker's graph reaches (worker.ts:4) and there is no packages/client/public; serve.mjs serves its own folder only
  (:67-98, :174). Gaps for step 7: bundleProblems (build-checks.mjs:161-178) does not check which .wasm files an out
  folder holds; a path cdylib's file has no hash in its name (Cargo; from memory, not read today), so a devtools-graph
  build of sim-wasm in the same target/ and profile writes the same release/sim_wasm.wasm that build.mjs:42 reads
  (latent: identical units today) -> condition 2.
- (e) No Worker source imports @shamble/types (worker.ts:4-6, host.ts:1-5, index.ts:4-6); only the test
  boundary/sim-wasm.test.mjs:6. The page imports it itself (client/src/app/worker-link.ts:2) under its own dependency
  (client/package.json:13; lock :1236-1240). Install unchanged: a workspace linked through the root's workspace edges
  (package.json:9-11; lock :361-364), no dev flag before or after; no workspace has an install script. npm ci checks
  presence and versions only, so it shows the lock installs, not that the hand edit is npm's own form (it is, by my
  reading of npm's rule: a workspace entry copies its package.json lists; from memory). Sure: high / medium-high.
- (f) Reasoning only: the fetch (toolchain.yml:129-130) gets the same packages; :155-156 now also compile devtools
  natively as cdylib+rlib from fetched crates (sim-wasm already does, sim-wasm/Cargo.toml:10); --locked holds (lock
  written by cargo update --offline, the entry lists exactly the three); :160 names no devtools; :161 needs only the
  cached tarballs; build-checks.mjs:15-25 still finds exactly one `name = "wasm-bindgen"` (Cargo.lock:148). All work.
- (g) Low, follow-up (not one of the four files): the root Cargo.toml:14 comment "in sim-wasm only" is no longer
  true; a manifest, so it belongs in the next batch. The later cargo-deny wasm-bindgen rule must allow devtools too.
  devtools/Cargo.toml:9's "never part of what players get" is a promise step 7 must make true (condition 2).
  Not grown by this batch: no cargo-deny or cargo-audit set-up exists yet (no deny.toml; CI runs neither).
- During the review (13:15-13:24 UTC) the session wrote steps 4-5 (crates/sim/src/lib.rs, rng.rs; devtools src, tests,
  vectors) and made two checkpoint commits (54235a9e, a8afdc3a): .checkpoints only, no code, manifest or CI change
  since 50656d6 (git diff --quiet). No other manifest, lockfile, build.rs, .cargo or .npmrc differs (checked twice).

## Conditions
1. Medium, none of the four files changes. Show (b) on the batch alone: build the player's Wasm from the head's code
   plus only these four files (e.g. a clean worktree under .gate/ with the four copied in, hashes checked), with a
   clean target folder, and compare pkg/sim_wasm_bg.wasm with c4935aca...c31, 28,987 bytes (if the folder differs
   from the "before" build's, build "before" there too). This tree's steps 4-5 sim code is in sim-wasm's graph and may
   change the Wasm. Re-hash the four files after `npm ci --offline --ignore-scripts` and after that build: unchanged.
2. Medium, none of the four files changes; holds at step 7. The test module is built in its own cargo call
   (`-p devtools --lib`) with its own --target-dir, never in the player's `-p sim-wasm` call or target/; it and its
   glue go only to an ignored folder outside packages/client/, packages/sim-worker/pkg/ and every dist* folder,
   imported by no page or Worker source, served only by the spec's own routing (never serve.mjs); and the build
   refuses an out folder holding any .wasm but the one equal to pkg/sim_wasm_bg.wasm, shown failing on a planted copy.

End hashes (13:24 UTC, sha256sum -c against deps-sha256.txt): all four OK, the same four values as at the start.

## Answer
The dependency change may be committed and built: yes with conditions (1 and 2 above; neither changes one of the four
files). Follow-up, Low: root Cargo.toml:14's "in sim-wasm only" in the next manifest batch.
Not verified: nothing built or run (no cargo tree/metadata: Cargo's cache tracker may write); Cargo's cdylib naming and
npm's lock rules from memory, not read today; no advisory database consulted (none set up; no package changes).
