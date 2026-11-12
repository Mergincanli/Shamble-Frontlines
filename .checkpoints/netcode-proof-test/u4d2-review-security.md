# U4d2 review round 1: security-auditor (read only)

Seat: the advisor's U4d2 brief check, A12 (the third unsafe exception; check.mjs's unsafe and rustflags scans).
Read copy: /home/user/Shamble-Frontlines/.gate/u4d2-r1 (51520b4 + snapshot). Builder's copy:
/home/user/Shamble-Frontlines/.claude/worktrees/u4d2-build. Logs: /home/user/Shamble-Frontlines/.gate/u4d2/r1-sec-*.log.

## Plan
1. Threat model (below), then read the packet: heads, the diffs for hash-wasm, lab's hash.rs, check.mjs; the
   judged-against sections (§5 U4d2, §4.2, §4.5, conditions 7 and 8, Q3, the U4d2 brief ruling A1-A15, Q1-Q12, L1-L5).
2. The unsafe scan: read check.mjs's scan code; list the bypasses it misses (macro bodies, `unsafe extern`,
   `#[unsafe(export_name)]`, `#[unsafe(link_section)]`, `cfg_attr`, multi-line attributes, raw idents, strings and
   comments, `include!`, build.rs, paths outside the scanned folders). Prove the cheapest by a scratch-free read or a
   run of the scan function in node on a scratch input under the notes folder (no edits to the copies).
3. The signature scan (Q7) and the loader's refusals (moduleProblems, exportProblems), the chunked streaming and the
   lengths the loader trusts.
4. The build-flag check (A8; condition 7 (a); Q4): rustflags sources, parsing, fail-closed, the read-back.
5. The features comparison (PM-0004-74; Q6; P1's plant).
6. Supply chain and process: Cargo.lock, child processes, the wasm-step scratch folder and stale files.
7. Findings with severity; the coverage table; the verdict.

## Threat model
- T1 a change slips `unsafe` past the scan into a build (any crate in the workspace, not only hash-wasm).
- T2 a build flag that changes float results (fast-math-like `-C` flags, target-feature, `-Z` options, a linker
  arg) reaches the native or wasm build without the check seeing it.
- T3 a dependency feature differs between the native and the wasm build of npt-sim (or a dependency of it).
- T4 a check passes for the wrong reason: a stale file in the scratch folder, a comparison of a thing with itself, a
  parse that defaults to "clean" on an error, a test that asserts a constant.
- T5 the loader trusts a hostile .wasm (imports, extra exports, memory export) or a hostile recording length.
- T6 process: child processes through a shell, paths from the environment, writes outside .gate/.
- Out of scope here: the server's network input (no network code in U4d2).

## Findings (a line after each)
