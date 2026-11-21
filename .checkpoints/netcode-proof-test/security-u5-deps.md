Task: security-auditor DEP-HOLD dependency read for U5's early start (netcode proof test, PR #40, lane claude/netcode-proof-test-plan-hcky5g), Block 0 manifests and locks, libfuzzer-sys 0.4.13, arbitrary 1.5.0, cargo-fuzz 0.13.2, cargo-deny 0.22.0, cargo-audit 0.23.0, nightly-2026-11-01.

## Plan (blocks, in order)
- B0: context: advisor-u5-early.md condition 3 (:219-226), N3/N4 (:28-32); u5-brief.md Block 0 (:101-120); u4s-carries.md entry 107 (U5-Q7 term 3); u5-deps-facts.md.
- B1: item 1, verify the 8 sha256 hashes; read every manifest and both locks; diff against main's versions; added lines, what they pull in, anything beyond Block 0; root Cargo.toml comment.
- B2: item 2, libfuzzer-sys 0.4.13: build.rs (env vars, network, writes outside OUT_DIR, spawns), fuzz_target! expansion (unsafe, no_mangle, extern "C"), license, own deps.
- B3: item 3, arbitrary 1.5.0: unsafe, build script, license, deps.
- B4: item 4, tools: cargo-fuzz 0.13.2, cargo-deny 0.22.0, cargo-audit 0.23.0 (.crate Cargo.lock, build scripts, native code, run-time network, license).
- B5: item 5, nightly-2026-11-01 --profile minimal.
- B6: item 6, RustSec advisories (advisory-db) for the five crates at those versions; what could not be checked.
- B7: verdicts, conditions, dependency-check.md lines; DONE.

## Notes (findings and block completions)
