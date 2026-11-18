# The release record of change-set 2's dependency change (.claude/CHECKPOINTS.md, section 3)

Written by the main session on 2026-11-18 23:05:50 UTC (from the clock), after security-auditor's answer and before the manifests were
committed. The answer: `.checkpoints/cs2-first-page/security-auditor-deps.md` (security-auditor, Opus 5.5 at effort
max, 2026-11-18; the nine files hashed to these values when it started, 22:39 UTC, and when it ended, 23:04 UTC, by
its own lines): "the dependency change may be committed and built: yes with conditions"; none of its four conditions
changes these nine files. What it read: `deps-manifests.diff`, `deps-lockfiles.diff` and `deps-lock-compare.txt`
beside this file. Mustafa's answers that this content rests on (RNG-CRATE (c): no random-number crate; WB-HEALTH (a):
wasm-bindgen pinned to exactly 0.2.130): `mustafa-2026-11-18-2326-five-answers.md`.

One line per file, as `sha256sum` prints it from the repository's root:

bb922d81c97080164f9050238c1f8fa91ccd4a6e9015505feb869d78f370cf27  Cargo.toml
75739f3b45f70bbc9f7d9c28fef61a6d68a17d3ff342aed472179a927d4e7705  Cargo.lock
9966d554ff23ccc742caeb19574028a08a7046f7e72f0ce4b9dde59e6dad3058  crates/sim-wasm/Cargo.toml
90fd46b681fec81c7e4615b2d4074739f547bc562d4bbf7a30b6be9b2ef7f9ad  package.json
212bf01fbee136cc9d69f23747d0b13a3fe3d2adec7b53c59cab08b6ef67eae5  package-lock.json
e4433f6b9fb3997145064cf519326fd1ae7d23ce720e3835c382d82fad83dfcd  packages/client/package.json
5baa670b42f7f116828db481c9ab0b003f96cdc4c10e02f84656a42333bf7ff5  packages/e2e/package.json
72884f0ffdabde0cea61976ddd65b5a45dacb214ba5ead09dce4b01d887d5a6e  packages/sim-worker/package.json
80994f0db606a4fb096832c20c5cea6df886e6feee09f7a76c73389b9bc2a0d6  packages/types/package.json
