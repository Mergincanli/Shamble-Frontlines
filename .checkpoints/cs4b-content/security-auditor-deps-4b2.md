# security-auditor: dependency check of 4b-2 step 6 (parry3d =0.31.1 manifest batch)

Brief: `.checkpoints/cs4b-content/deps-4b2-brief.md`. Worktree `.gate/lane1-takeover`, branch `cs/4b-world`, from main
b9aad01. Batch: `.gate/cs4b/batch-4b2/` (inside the worktree, gitignored `.gate/`), five files. No build. No network.
No agent memory folder exists in this environment (checked `.claude/agent-memory*`, `~/.claude`): earlier decisions come
from this morning's notes (`security-auditor-deps-4b1.md`, `security-auditor-review-4b1*.md`).

## Plan
1. Digests: `sha256sum -c` the five files against `deps-4b2.sha256`; compare the three unchanged manifests with step 0's
   digests in `deps-batch.sha256`; check `deps-4b2.diff` is the batch against b9aad01 (re-diff myself).
2. Lockfile: parse the batch lockfile and step 0's (`.gate/cs4b/batch/`), list the 33 registry packages added against
   b9aad01's lockfile, compare name/version/checksum/source with step 0's lockfile and with rust-reviewer-step0.md;
   confirm parry3d 0.31.1, safe_arch 1.2.0, thiserror 2.0.21 exact. Verify checksums against the cached .crate files.
3. Build-time code: list build.rs and proc-macro crates among the 33 (from cached sources' Cargo.toml), read each
   build.rs for network/clock/env reads; read proc macros' surface.
4. `unsafe` on parry3d's enhanced-determinism path (no std, f32, dim3, alloc): grep parry3d and its deps' sources.
5. What the players' Wasm file gains: features that turn on SIMD, FMA, libm; what step 7's built-file check should hold.
6. devtools: `protocol` to dev-dependencies; grep who reads it.
7. `cargo tree`/`cargo metadata --offline --locked` in a copy at `.gate/security-auditor-4b2-scratch/`: does the lock
   resolve unchanged (no rewrite), which features are on per target (wasm32 and native).
8. Anything step 6 will need that the batch lacks (dev-deps for tests, features, `[lints]`, deny.toml entries).
9. Answer (<= 20 lines).

## Progress
- [ ] 1 digests
- [ ] 2 lockfile
- [ ] 3 build-time code
- [ ] 4 unsafe
- [ ] 5 players' file
- [ ] 6 devtools
- [ ] 7 resolve in a copy
- [ ] 8 missing pieces
- [ ] 9 answer

## Findings (one line each, as made)
- [x] 1 digests. `sha256sum -c deps-4b2.sha256` in `.gate/cs4b/batch-4b2/`: five OK. Step 0's batch (`.gate/cs4b/batch/`)
  re-checks against `deps-batch.sha256`: six OK. Root Cargo.toml (a854d5e3...), sim (1b19f15a...), sim-wasm (02a77a9a...)
  equal step 0's digests byte for byte. The branch over b9aad01 adds only `.checkpoints/` (git diff b9aad01 HEAD --stat),
  the tree's manifests are b9aad01's (git status: only packages/sim-worker/boundary/content.test.mjs, a comment, outside
  the batch). `patch -p1 -i deps-4b2.diff` on a copy of b9aad01's files gives exactly the five digests: the diff is the
  batch's full text and touches no sixth file. Scratch copies: `.gate/security-auditor-4b2-scratch/{base,patched,ws}`.
- [x] 2 lockfile (script: `security-auditor-deps-4b2-lock.py`, python -I, reads only). Lock format 4 in all three.
  b9aad01 23 packages -> batch 56: 33 added, all `registry+https://github.com/rust-lang/crates.io-index` (no git, no
  other registry, no path outside the repo), none removed; sim gains [content, parry3d], sim-wasm gains content.
  Against step 0's lockfile (`.gate/cs4b/batch/Cargo.lock`, f7233053...): same 56 (name, version) pairs, all 49
  registry entries identical in every field (version, source, checksum, dependency list); only path crates differ:
  content drops ts-rs (4b-1 merged content with no dependency), devtools adds protocol. parry3d 0.31.1 (7b626edc...),
  safe_arch 1.2.0 (42c6efa1...), thiserror 2.0.21 (09e52cb8...): exactly step 0's. The 33 = rust-reviewer's list
  (rust-reviewer-step0.md Q5: B1-7's 33 minus ena, slab plus serde, serde_derive), versions as named there.
  Checksums: for all 49 registry entries lock checksum = cached sparse-index `cksum`; for the 47 with a cached .crate
  also = sha256 of the .crate file; serde and serde_derive have no .crate (never compiled: cfg(any()) edges) and match
  the index. None yanked in the cached index (fetched 2026-11-26 ~13:05 UTC). Caveat: index and .crate files came
  through the same session proxy, so this proves consistency, not independent origin; CI's first `--locked` build on a
  GitHub runner re-downloads from crates.io and cargo checks each .crate against these checksums (the independent check).
- [x] 3 build-time code (script `security-auditor-deps-4b2-src.py`): the 31 cached new crates' unpacked sources equal
  their .crate archives byte for byte (no missing, extra or changed file; cargo does not re-check them). No `links` key.
  Build scripts: heapless 0.8.0 (autodetected build.rs; reads TARGET, RUSTC, OUT_DIR, RUSTC_WRAPPER,
  CARGO_ENCODED_RUSTFLAGS; writes OUT_DIR/probe.rs; runs rustc on an ARM `clrex` probe, fails on wasm32/x86_64),
  libm 0.2.16 (cargo's variables + OPT_LEVEL + ENSURE_NO_PANIC; ENSURE_NO_PANIC set would only break the build, since
  `no-panic` is a dev-dependency: fail-closed; emits CFG_* rustc-env that no libm source reads), num-traits 0.2.19
  (autocfg probe `1f64.total_cmp(&2f64)`: runs rustc), serde_core 1.0.229 (writes OUT_DIR/private.rs from the patch
  version, runs `rustc --version`). Host library autocfg 1.5.1: runs RUSTC/RUSTC_WRAPPER, writes/removes probe files in
  OUT_DIR, reads RUSTFLAGS/CARGO_ENCODED_RUSTFLAGS/TARGET/HOST. Proc macro: num-derive 0.5.1 (1003 lines, no fs, net,
  process, env or include_*). serde_derive never builds (cfg(any())). None reads the network, the clock or randomness;
  every spawned process is the compiler cargo names. Matches rust-reviewer-step0.md Q5.
- [x] 4 unsafe in parry3d 0.31.1 (all 25 code sites read): the raw-pointer ones (bvh_refit.rs RefitPtrs Send/Sync and
  derefs; bvh_insert.rs:308-323 NodesPtr) are all under `cfg(feature = "parallel")` (= rayon; off; rayon not in the
  lock): not compiled. Compiled: reference transmutes of `#[repr(C)]` shapes to/from `[Vector; N]` (segment.rs:214,
  triangle.rs:206/212, tetrahedron.rs:356), BvhNode -> BvhNodeSimd (bvh_tree.rs:613, size and align asserted at compile
  time, :408-411), slice views (as_bytes.rs:17 Vector2/3 -> bytes; trimesh.rs:1241, polyline.rs:744 `[[u32;N]]` ->
  `[u32]`), SortedPair Deref (sorted_pair.rs:148 `[T;2]` -> `(T,T)`: relies on tuple layout, which Rust does not
  promise; identical in practice; upstream nit, not ours). None takes an input-dependent offset: no client- or
  content-sized pointer arithmetic. Our crates keep `unsafe_code = "deny"` (root Cargo.toml).
- [x] 7 resolve in a copy (`.gate/security-auditor-4b2-scratch/ws`, the batch's five files copied in, digests OK; the
  1.99.1 toolchain and its wasm32 target are installed, so rustup fetches nothing; no cargo config anywhere).
  `cargo tree --offline --locked` resolves for wasm32 and the host; Cargo.lock's digest is ad0f7680... before and after
  every call (the lock is complete for these manifests). Script `security-auditor-deps-4b2-lists.mjs` (imports the
  branch's build-checks.mjs, runs build.mjs's three tree commands offline): players' Wasm 71, test-only module 71,
  native tests 76 entries, each equal entry for entry to crates/devtools/vectors/sim-feature-lists.json (committed by
  the session in 5cc96c5 at 20:46 UTC while I worked; no manifest in it); the compare with the two exceptions gives no
  problem, without them exactly the five named entries; missingOverflowOverrides gives none. parry3d "std" absent
  (Mustafa: std stays off); wide has no "std"; safe_arch absent from both wasm32 lists.
- Note: HEAD moved under me (a8741fc: the test comment; 5cc96c5: the feature compare plus my notes as they stood). Neither
  touches a manifest or lockfile; the tree's Cargo.toml/Cargo.lock are still b9aad01's.
- [x] 5 players' file. Gains 28 registry crates (+ thiserror 2.0.21, already locked, now linked with std off):
  approx arrayvec bitflags bytemuck byteorder downcast-rs either equivalent foldhash glam glamx hash32 hashbrown
  heapless indexmap libm log num-complex num-traits ordered-float parry3d rstar serde_core simba smallvec
  stable_deref_trait static_assertions wide. Build machine only: num-derive (proc macro), autocfg (num-traits' build
  dependency). Never built: serde, serde_derive. x86 only: safe_arch. Run-time reads in these crates under our
  features: none of clock, env, files, network, randomness from the host (parry's file writers are behind `wavefront`
  and `spade`, off; its `Instant::now` lines are comments; foldhash/hashbrown/indexmap RandomState only on a
  default-hasher map, which parry's enhanced-determinism alias avoids: rust-reviewer Q6/side). No js-sys, web-sys,
  getrandom in the lock, so no new import is expected. Notable features on in the players' Wasm: indexmap "std" and
  "serde" (serde_core compiled; no deserialise path from our code), hashbrown "default-hasher", libm "arch".
  The built-file check (no check of the Wasm's imports or strings exists today: build-checks.mjs exports read) should
  also hold: (a) an exact allow-list of the players' Wasm's imports (module + name), so no crate can add a host call
  (clock, randomness, console) unseen; (b) no builder path in the file (no `/home/`, `/root/`, `\Users\`, CARGO_HOME):
  parry adds many panic-location strings, the remap flag (RR-F5) covers the registry; the check proves it. SIMD
  opcodes cover Wasm's only multiply-add (relaxed-simd madd); libm's software fma/fmaf need the allow-list (MUL-ADD).
- [x] 6 devtools: `protocol` is named only by tests/aim_reach.rs:9 (`use protocol::messages::AimPoint`);
  dependency_rules.rs:80/92 are string fixtures; src/lib.rs:1 is a doc word; src/bin (make_recordings, replay,
  time_report) and src/ name no `protocol::` (binaries cannot see dev-dependencies, and none needs it). The test-only
  Wasm module (`-p devtools --lib`) still links protocol through sim-wasm, so its features do not move (lists equal
  the vector). Narrower and right.
- Licences: all permissive (MIT, Apache-2.0, Zlib, Unlicense-or-MIT). parry3d, simba, approx are Apache-2.0 only; no
  NOTICE file in any of the 31; parry3d's archive carries no LICENSE file. The hosted build must ship the licence
  texts (hosting step; Low, can wait).
- Supply-chain gates: no cargo-audit/cargo-deny in CI yet (ADR 0002 G5 and R5-GATE-REC1 plan them). RustSec not read
  here (no network): from memory I know no advisory against these versions; that is not evidence.
- Overflow overrides, all views (cargo tree --prefix none, offline, locked): native tests (normal+dev, host) 36
  third-party crates, test-only module (wasm32) 33, workspace on wasm32 (normal+dev) 35: none lacks an override; only
  "*" names no crate. 36 `[profile.release.package...]` headers = "*" + 6 earlier + 29 new.
- Batch files: regular files (no symlink), root-owned, pure ASCII (no hidden Unicode), LF; .gitattributes has
  `* text=auto eol=lf` and no filter driver, so committing does not change the bytes. No [patch], [replace], registry,
  git or out-of-repo path in any of the five.
- [x] 8 missing pieces: none for step 6. Step 7's built-file check is dependency-free by design (pure functions in
  build-checks.mjs: rust-reviewer-cs4.md:82); step 9's ts-rs line in content is its own batch (brief; the content
  rule's doc says so). Order is sound; add: after the commit, check the committed blobs (`git show <sha>:<path>`)
  against the five digests; no cargo call without --locked between copy-back and commit.
- Mustafa's words checked at docs/DECISIONS.md:377 (FEATURE-COMPARE (a), conditions as the brief quotes them).

## Findings
- SA-4B2-1 Low (can wait; cheapest in step 6's own commit): "a change of parry3d's version ... needs a new yes from
  me; the library's std stays off" is held by process only (DEP-HOLD, review). The feature compare cannot see a
  change made to every build at once: traced, parry3d "std" on adds glamx/simba/arrayvec std, slab, ena to all three
  lists alike and `thiserror feature "std"` to the players' list; the one uneven entry left (native's
  `thiserror feature "default"`) is excused by exception 2. A parry3d bump also moves all lists alike. Fix: the sim
  half of crates/devtools/tests/dependency_rules.rs also pins the root's parry3d line (version "=0.31.1",
  default-features = false, exactly dim3, f32, alloc, enhanced-determinism) and Cargo.lock's parry3d 0.31.1,
  safe_arch 1.2.0, thiserror 2.0.21, its message naming FEATURE-COMPARE (a) (DECISIONS.md:377). No manifest change.
  (The root comment's "Each bump is its own change-set" omits his yes: wording for the next manifest batch.)
- SA-4B2-2 Low (step 7, same pull request): the designed import rule refuses only an import naming fma
  (rust-reviewer-cs4.md:78). Make it an exact allow-list of the players' Wasm imports (module + name), so a crate
  cannot add a host call (clock, randomness, console, env.*) unseen; and refuse builder paths in the file
  (/home/, /root/, \Users\, the CARGO_HOME path) to prove the RR-F5 remap held. No dependency needed.
- Note (not security): Apache-2.0-only crates in the players' file (parry3d, simba, approx; no NOTICE; parry3d's
  archive has no LICENSE file): the hosted build must carry the licence texts (hosting step follow-up).

## Verdict
APPROVE on the five digests of deps-4b2.sha256 (a854d5e3... Cargo.toml, ad0f7680... Cargo.lock, 1b19f15a... sim,
02a77a9a... sim-wasm, 6b90a2de... devtools). Nothing for Mustafa: no version of the three pinned packages moved.
Not checked: RustSec (no network); origin of the archives beyond the cached index (CI's first --locked fetch from
crates.io checks them); nothing compiled; sources beyond build scripts, proc macro, unsafe and greps.
- [x] 9 answer
DONE
- SA-4B2-1 re-traced from manifests (a std-on resolve cannot run under --locked: it adds slab and ena to the lock):
  wide `std = []`, simba `std = ["wide/std"]`, glamx `std = ["glam/std", "simba/std", "approx?/std"]`: no target-
  conditional entry, so every list moves alike and the compare stays silent. Nuance: the overflow check would trip on
  slab and ena in the players' Wasm, but its message asks for two override blocks, not for his yes; a parry3d bump with
  an unchanged tree trips nothing. The finding stands at Low.
