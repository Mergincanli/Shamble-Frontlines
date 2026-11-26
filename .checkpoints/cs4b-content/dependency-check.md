# Dependency check: cs4b-content, step 1 (the content crate's path-only batch)

security-auditor's answer on exactly this content: `.checkpoints/cs4b-content/security-auditor-deps-4b1.md` (APPROVE,
2026-11-26; it names the same three digests). No package from a registry, git or another source is added, no locked
version moves, no build script. The collision library's batch is NOT covered: it is step 6's and gets its own answer.

0ea3ed50699e37398df809cd6308178f2a0b7a701d32d513e208712002d3bbb0  Cargo.lock
1c20979e0ae907ad818068f3be19060d24d637028e6916deaa0c0f82409474f0  crates/content/Cargo.toml
c3b458d55796508790abc9fa7bb454e866fadc99df31b09e4262ed5a01946ec6  crates/devtools/Cargo.toml
