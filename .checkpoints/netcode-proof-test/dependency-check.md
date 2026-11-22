# DEP-HOLD release, U5's early Block 0 (main's .claude/CHECKPOINTS.md §3; advisor-u5-early.md condition 3)
# security-auditor's answer, read in full (ends DONE): .checkpoints/netcode-proof-test/security-u5-deps.md
# Written 2026-11-21 ~15:3x UTC by the main session; every line below is one the answer cleared at that content.
# The answer's conditions: SEC-U5D-1 (Medium) to the advisor before Block 3; SEC-U5D-4 (Medium) a lock check around
# every cargo-fuzz call (or plain `cargo +nightly-2026-11-01 build --locked`); CUSTOM_LIBFUZZER_PATH and
# CUSTOM_LIBFUZZER_STD_CXX unset, fuzz outputs under .gate/; fuzz_target! only (SEC-U5D-3); cargo-fuzz never
# init/add/--build-std/--careful/--sanitizer=memory, its target dir under .gate/ or the shared target; the nightly only
# by `rustup toolchain install nightly-2026-11-01 --profile minimal`, called as +nightly-2026-11-01; cargo-fuzz by
# `cargo install --locked --version 0.13.2 cargo-fuzz`; cargo-deny 0.22.0 and cargo-audit 0.23.0 for Block 8 only, one
# at a time, build dirs under .gate/, the advisory database never inside the repository or a worktree.
3bf4d76008175aa7e779d2c1f504b09c5979a5e0e49ddcded8380b7045383b04  spikes/netcode-proof/Cargo.toml
31c2c7ea9fdf04f78391b0e6d90165f14b74483c87f235c5daf4ad5f8993a7a8  spikes/netcode-proof/Cargo.lock
fb5324f1ac53251ef03accb89a9182d62506eb98d074aed6165c00cc49efebf1  spikes/netcode-proof/crates/alloc/Cargo.toml
b5385fcdaa5dcc2344c41177b5afec923cea9d2004512245fc6d776a2f0c329a  spikes/netcode-proof/crates/sim/Cargo.toml
670914131fb08661aafdf8299bea21c18a15e2e7a33f754c295b1bbd61ea1b12  spikes/netcode-proof/crates/proto/Cargo.toml
afd0952b15865881f763b1cba9d0feac715d2cf104dc4b2c6d13d3ab87f7b265  spikes/netcode-proof/crates/net/Cargo.toml
7606244c186ebb95c486c2ef8c100740383cdcdf81f9e985b9495c1e328a369d  spikes/netcode-proof/fuzz/Cargo.toml
3117b9305e1ebbf8ed4d802ab3689a6f4296067275e66d84fd8d7b9e66bbf1fd  spikes/netcode-proof/fuzz/Cargo.lock

# DEP-HOLD release, U6a's Block 0 (main's .claude/CHECKPOINTS.md §3; advisor-u6a-brief.md condition 1)
# security-auditor's answers, read in full (each ends DONE): (a) the url 2.5.9 bump:
# .checkpoints/netcode-proof-test/security-r1-deps.md (SEC-R1D-1; binding check :249-253); (b) the manifest edits and
# step 5: .checkpoints/netcode-proof-test/security-u6a-deps.md (SEC-U6AD-1 to -6; binding check :119-137; section 6:
# cargo-deny 0.22.0 and cargo-audit 0.23.0 cleared for this Block 0; section 7: deny.toml's allow-wildcard-paths line,
# committed alone at 6aa78ff). Written 2026-11-22 ~14:2x UTC by the main session after binding checks 1-5 passed:
# cargo-deny "advisories ok, bans ok, sources ok" (exit 0, five duplicate warnings); cargo-audit 0 vulnerabilities, one
# allowed warning RUSTSEC-2026-0441 (evidence: u6a-block0.md). Every line below is at the cleared content.
5ebfbedfdf3015e1928aa140bbda1585ad39012521c11ede62f40dfcbcd0d36a  spikes/netcode-proof/Cargo.toml
c55bab3c93d86c0068f572d9b20d152fe0080a6f29f278617615db04836e6b4d  spikes/netcode-proof/Cargo.lock
e694b595ef02d4e0f01e87df61ed083ba3ae254ab0b46687f6183a11df28a60d  spikes/netcode-proof/crates/server/Cargo.toml
8ba544d368e0e35c509868a233dc0201cf8d4e0c3a19fa217f101724a5759086  spikes/netcode-proof/crates/super/Cargo.toml
90404ee6b66b3884129195ad1d6eaef8a97a023159d4ff2b8f888242d7369ae1  spikes/netcode-proof/deny.toml
