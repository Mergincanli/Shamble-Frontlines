# U5-EARLY, Block 0: the facts for security-auditor's dependency read (main session, 2026-11-21 ~15:1x UTC)

W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-a60a432d4564f4cc7 (U5's early builder's copy, fast-forwarded
to the lane head bd88fa756c6cc6287d261572571b26f2eaa34d32; spikes/netcode-proof tree 2b7647ff… before Block 0).
The builder's own notes: W/.checkpoints/netcode-proof-test/u5-build.md and u5-tools.md (2c6cca66…).

## The Block-0 files (the builder's return; paths under W/spikes/netcode-proof/, sha256)
- Cargo.toml 3bf4d76008175aa7e779d2c1f504b09c5979a5e0e49ddcded8380b7045383b04 (the crates/alloc member,
  `exclude = ["fuzz"]`, an `npt-alloc` path entry; the unsafe-exception comment rewritten as final with the ADR 0002:269
  cite: four exceptions, npt-alloc the fourth at deny)
- Cargo.lock 31c2c7ea9fdf04f78391b0e6d90165f14b74483c87f235c5daf4ad5f8993a7a8 (an offline resolve through the runner:
  the new `npt-alloc` package entry, plus `npt-alloc` in npt-proto's and npt-sim's dependency lists; nothing else)
- crates/alloc/Cargo.toml fb5324f1ac53251ef03accb89a9182d62506eb98d074aed6165c00cc49efebf1 (no dependency, no build
  script, no `build` or `include` key)
- crates/alloc/src/lib.rs e64d0d9efcaaeeace5ff6c38416dd035f89b801f77b090b6121e33a9d042d297 (a 2-line placeholder;
  not a manifest)
- crates/sim/Cargo.toml b5385fcdaa5dcc2344c41177b5afec923cea9d2004512245fc6d776a2f0c329a (dev-dependency npt-alloc)
- crates/proto/Cargo.toml 670914131fb08661aafdf8299bea21c18a15e2e7a33f754c295b1bbd61ea1b12 (dev-dependency npt-alloc)
- crates/net/Cargo.toml afd0952b15865881f763b1cba9d0feac715d2cf104dc4b2c6d13d3ab87f7b265 (`fuzz = []`, no dependency)
- fuzz/Cargo.toml 7606244c186ebb95c486c2ef8c100740383cdcdf81f9e985b9495c1e328a369d (crate npt-fuzz outside the
  workspace; libfuzzer-sys "0.4"; seven path crates; rand_chacha and rand_core at the workspace's versions; 17 bins)
- fuzz/Cargo.lock: 31c2c7ea… (a copy of the workspace lock) before the fetch; after the fetch below
  **3117b9305e1ebbf8ed4d802ab3689a6f4296067275e66d84fd8d7b9e66bbf1fd**

## The networked steps (main session, condition 3; by --manifest-path, never by cd)
1. `cargo +1.99.1 fetch --manifest-path W/spikes/netcode-proof/fuzz/Cargo.toml` (~15:0x UTC): "Locking 2 packages to
   highest Rust 1.99.1 compatible versions / Adding arbitrary v1.5.0 / Adding libfuzzer-sys v0.4.13 / Downloaded
   libfuzzer-sys v0.4.13 / Downloaded arbitrary v1.5.0". No build script ran.
2. The two locks compared (u5_lock_compare.py, read-only): workspace 243 packages, fuzz 130, kept in both 127; added 3
   (arbitrary 1.5.0 registry, libfuzzer-sys 0.4.13 registry, npt-fuzz 0.0.0 path); dropped 116 (every package outside
   the fuzz crate's graph: npt-lab, npt-server, npt-log, serde, wtransport, web-transport-quinn and their trees); **0
   kept packages changed source or checksum; 0 new versions of a package the workspace lock holds.**
3. `cargo +1.99.1 tree --locked --offline --depth 1 --manifest-path W/spikes/netcode-proof/fuzz/Cargo.toml`: exit 0
   (the lock is complete and every source is on disk).
4. Version lookups (`cargo +1.99.1 info`; each downloaded its .crate into the registry cache, nothing built or
   installed): cargo-fuzz 0.13.2 (MIT OR Apache-2.0; rust-version unknown); cargo-deny 0.22.0 (MIT OR Apache-2.0;
   rust-version 1.95.0); cargo-audit 0.23.0 (Apache-2.0 OR MIT; rust-version 1.88; default feature binary-scanning).
5. Not yet: the nightly (`nightly-2026-11-01` = 1.101.0-nightly f9ceb4a4d 2026-10-31, u0-probes.md:9) and every
   install; both wait for dependency-check.md (condition 3).
