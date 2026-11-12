# U4d2 review round 1: rust-reviewer (notes)
Seat: rust-reviewer, read only. Log prefix rr (.gate/u4d2/r1-rr-*.log). Copy: .claude/worktrees/u4d2-build (runner only).
Base copy: .gate/u4d2-prefix (51520b4). Read copy: .gate/u4d2-r1.

## Plan
1. Read the packet (u4d2-packet.md) and its tests side file; the plan's §5 U4d2, plan.md §4.2/§4.5, ADR 0002:252-266,
   advisor-u4d2-brief.md Q1-Q12, the carries routed to U4d2.
2. Read the new source: crates/hash-wasm (lib.rs, Cargo.toml, loader.mjs), crates/lab (hash.rs, probe.rs, main.rs,
   lib.rs, clippy.toml, recordings.json), check.mjs's wasm step, Cargo.toml/Cargo.lock, client-wasm and proto Cargo.toml.
3. The unsafe exception: seven exports, allow placement and reason, the invariant, symbol collisions (glibc `step`,
   Q8), thread_local re-entrancy, panic on wasm32 and native, the memory export.
4. Identical results: widths at the boundary, usize in hashed state or dump words, dump order, hash halves,
   compare --digest-field at the native layout's indexes.
5. Build: §4.5 flags, Q2, Q4 read-back, release profile, size numbers (re-measure).
6. npt-lab: hash.rs (two processes, 16.7 MB reads, manifest SHA-256, carry 52 generator refusal), HashCompare,
   TickHash::Empty, probe.rs carry 66; panics from a bad file/arg, `as` casts, overflow, per-tick allocation, quadratic.
7. Carry 29's libm comment against libm 0.2.16 source (configure.rs:82-101, arch/x86/fma.rs:9-27); carry 41's lines.
8. clippy.toml bans (Q10); clippy on both targets; docs; test code that could pass for the wrong reason.
9. Runs (through the runner only): clippy both targets, hash-wasm tests, lab tests, wasm build size, symbol table.
10. Verdict.

## Findings (a line after each)
