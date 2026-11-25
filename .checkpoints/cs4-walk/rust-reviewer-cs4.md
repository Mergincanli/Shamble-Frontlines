Task: rust-reviewer design notes for change-set 4 "Walk" (plan stage, paper only): collision crate, MUL-ADD built-file check, movement on parry3d, integer time unit, marks at cs4, sizes.

## Plan (blocks, in order)
0. Read: memory, build plan (Walk card, Part 2 rows, 3.1, 3.2, 3.4, 3.5, 7), ADR 0004 (parts 3, 4, 7-10, marks), research R1/R4, 3a report + STATUS_ARCHIVE 3a block, crates/ code, build-checks.mjs, manifests, lockfile, toolchain, my cs3b notes. Write a short digest block.
1. Collision library (parry3d version, features, full dependency tree wasm32 + native, build scripts, proc macros, licences, Wasm size estimate, build-check overflow override, scan test treatment, security-auditor list; content-crate candidates: serde, serde_json, schema, validation; JSON double bits).
2. MUL-ADD built-file check (what 3a found: the 16 pinned Wasm values; what in a .wasm can be FMA; how to find it in a stripped file; where it runs; planted defect; control; native settings; mul_add / fma hits in parry3d, nalgebra, simba, num-traits, etc.).
3. Movement on parry3d queries (shape, queries, walk/sprint, step-up, jump, tunnelling, ramps, ledges, soldier pairs, static world byte-identical, far corner, zero allocation; float-op allow-list audit; enhanced-determinism vs PM-0004-20).
4. Integer time unit (rounding report design, candidates, recommendation, what Walk needs).
5. Marks at cs4 (each: test, where, planted defect, what stays later).
6. Sizes per step (code / test lines, plus or minus), risks, not-yet-tried list.

## Notes lines (findings as found)
