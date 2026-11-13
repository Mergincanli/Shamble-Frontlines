# U4d2 review round 2: security-auditor (read only; on what changed: FX-1 to FX-12)

Seat: the advisor's U4d2 round-1 ruling, Block 8, batch 1, security-auditor. Copy run in: 
/home/user/Shamble-Frontlines/.claude/worktrees/u4d2-build (only through u4d2-run.sh; never edited). Read copy:
/home/user/Shamble-Frontlines/.gate/u4d2-r2. Round 1's tree: /home/user/Shamble-Frontlines/.gate/u4d2-r1. Logs:
/home/user/Shamble-Frontlines/.gate/u4d2/r2-sec-*.log. Probe scripts (text): this notes folder, sec-r2-*.mjs, each
importing the copy's check.mjs or loader.mjs and feeding it in-memory inputs (no edit of any copy). Built modules from
round 1's probes: .gate/u4d2/sec-probe/ (sec_probe_tf2.wasm, sec_probe.wasm).

## Plan
1. Read the packet (done: heads, diffs, Blocks 1-8, T1-T5, carries) and the tests side file; read my round-1 notes
   and probes (sec-probe-scan.mjs, sec-probe-imports.mjs, sec-probe-custom.mjs, sec-probe-wasm.mjs).
2. Verify the read copy equals the builder's copy (sha256sum -c fx-hashes-end.sha256 in .gate/u4d2-r2).
3. FX-1: buildInputProblems on every round-1 name-rule case (`-C llvm-args=...`, `--cfg feature="bites"`,
   `-C overflow-checks=off`, `-C target-cpu=haswell`, `-Ctarget-feature=+avx512f`, `-Ctarget_feature=+fma`,
   `--codegen=target-feature=+sse2,+fma`), RR-M2's pair (CARGO_ENCODED_RUSTFLAGS + RUSTFLAGS), each env name form
   (CARGO_TARGET_<triple>_RUSTFLAGS, CARGO_BUILD_RUSTFLAGS, CARGO_ENCODED_RUSTFLAGS with 0x1f), config `rustflags` in
   [build], [target.<triple>], [target.'cfg(...)'], array and string forms, CARGO_BUILD_TARGET (incl. empty string),
   `build.target` (string, array, dotted key, inline table, `target = ` in another table); a clean env passes. Then the
   live step `--only deps` with r1-rr-superset.log's first environment (must FAIL naming both), and clean `--only deps`
   (must PASS). Edges: whitespace-only values, empty strings, CARGO_BUILD_TARGET="" (set but empty).
4. FX-2: moduleProblems on sec_probe_tf2.wasm (refused, naming the feature) and on both clean built modules in the
   shared target (accepted). targetFeatures against hostile bytes: a length past the end, a prefix other than + - =,
   a name that is no UTF-8, two sections, a LEB of 6 bytes, count larger than entries, trailing bytes, an empty
   section, `=simd128`, `-simd128` (allowed), a feature named with different case or a NUL byte.
5. FX-4: sec-probe-imports' modules (constructor.constructor, constructor.keys, __proto__.valueOf, env.now) each
   refused by name; and a module with a clean target_features section AND an import (import refusal on its own).
6. FX-6: V4's two forms (build.rs at the crate root; `use std::include as inc;`) refused by name; near forms:
   `include` in a `use` list (`use std::{include as inc}`), `core::include`, raw identifier `r#include`, `include`
   inside a macro_rules body, a build script declared in Cargo.toml (`build = "x.rs"`, `package.build`), a build.rs in
   a subfolder, `[package] build = true`, a `build-dependencies` table, and a custom-build target in metadata;
   `include ! (`, `include\n!`; lawful `include_str!` still passes.
7. FX-3: the self-test case and its fixture; an `unsafe {}` in a listed export's body refused at its line (live scan
   on in-memory text).
8. FX-5: the kind refusals (begin as a global, memory as a function) on hand-built modules.
9. SEC-U4D2-3's comment beside EXPORT_INTEGERS; C-U7-2's words (T3) against what stays open.
10. Self-tests: node --test check.test.mjs through the runner (counts), and the hash-wasm loader tests if any.
11. Findings with severity, coverage table, verdict.

## Blocks
