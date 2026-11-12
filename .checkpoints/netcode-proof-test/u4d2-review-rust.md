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

## Progress
- Read: packet heads, judged-against (§5 U4d2, §4.1, §4.11, §4.14, §4.15, Lanes, §7.1, plan.md §4.1/§4.2/§4.5/§4.7,
  conditions 2/7/8/10, Q3, A1-A15, Q1-Q12, L1-L5, ADR 0002:252-266/501-506, ADR 0004 -21/-22/-74, carries).
- Read source: hash-wasm/src/lib.rs (537), loader.mjs (239), lab/src/hash.rs (1412), lab lib.rs diff, main.rs diff,
  probe.rs diff, clippy.toml, recordings.json, std-sin-scan.txt; check.mjs's whole diff; check.test.mjs's hash-wasm test.

## Runs (mine, through the runner; result = exit code and the runner's summary lines)
- r1-rr-clippy.log (shared, -k), exit 0: clippy -D warnings host measured and bites (hash-wasm, lab, all targets) and
  wasm32 measured and bites (hash-wasm): each exit 0; `cargo fmt --check` exit 0; a pedantic pass (warn only, exit 0):
  in U4d2's code only hash-wasm lib.rs:150 `(h >> shift) as u32` (intended low-32 truncation); the other casts are U2's
  probe.rs lines (159, 171, 180, 361, 459), not this change.
- r1-rr-tests.log (shared, -k), exit 0: hash-wasm measured 5 passed, bites 7 passed; npt-lab lib 5 passed both; npt-lab
  bin 18 passed, 1 ignored (h7 (ii)) both. Release wasm32 rebuild from the copy's source: 114,282 B, sha256 48b5ec5f…,
  byte-equal to the wasm step's copy (.gate/netcode-proof-test/wasm-step/bin/npt-hash-wasm.wasm).
- Wasm module facts (my scratch u4d2-review-rust-wasminfo.mjs, run by the runner): exports exactly memory + the seven
  functions; imports none; custom sections name 10,873 B, producers 67 B, target_features 132 B; target_features
  +bulk-memory, +bulk-memory-opt, +call-indirect-overlong, +multivalue, +mutable-globals, +nontrapping-fptoint,
  +reference-types, +sign-ext (no simd128, no relaxed-simd). Bites module: same exports, 130,368 B.
- Sizes: measured 114,282 raw, GNU gzip -9 39,902 (gzip -6 40,006; Node zlib level 9 40,475); bites 130,368 raw, gzip
  -9 44,188 (Node zlib 9: 44,741). The builder's figures are GNU gzip -9's.
- Native: libnpt_hash_wasm.so (release) dynamic symbols defined: exactly the seven (nm -D). glibc 2.39's libc.so.6 has
  `step@GLIBC_2.2.5` as W (single @: a non-default compat version); none of the other six names in libc, libm,
  libgcc_s, libpthread, libdl, ld.so. The native test binary defines the seven statically (nm), none dynamic (nm -D).
  Q8's reading holds.
- Carry 66: the builder's e3-h7.log / b4-lab5.log show h7 (ii) passed on the release build (P2, B1, P4 seed 1 equal
  U2's 913db8e9…, feabf2f0…, b8a04f2f…) and probe runs printing them.

## Facts checked
- libm 0.2.16 (registry, read only): configure.rs:82-90 emit_intrinsics_cfg (off unless unstable-intrinsics),
  :94-101 emit_arch_cfg (on with `arch`, the default feature: Cargo.toml:40-41); src/math/arch/mod.rs:12-19: wasm32's
  module only with intrinsics_enabled, else with sse2 the x86 module exporting sqrt, sqrtf, fma, fmaf; arch/x86.rs:8-19
  sqrtf by `sqrtss`, :21-32 sqrt by `sqrtsd`; arch/x86/fma.rs:10-26 fma's run-time dispatch (FMA, FMA4, else generic
  fallback), :28- fmaf the same. So carry 29's facts hold but one: sqrtf is `sqrtss`, not `sqrtsd`.
- npt-sim calls libm cosf, sinf, atan2f, roundf (world.rs:449, 999, 1224, 1370-1371, 2264); no fma or mul_add.
- sim/src/hash.rs:21-25 `wide` writes low word first, so compare's `[lo, hi]` digest reconstruction matches; the fold
  is hash_map_order(64) = bite_order_digest (world.rs:617-624, 971-973, 2019-2021).
- Every other LEAF crate has a clippy.toml with the clock ban (link, l1, log, sim, proto, room, client); hash-wasm none.

## Findings (a line after each)
- RR-L1 (Low) Cargo.toml:38: the carry-29 comment says "sqrt and sqrtf by x86-64's `sqrtsd`"; libm's sqrtf is
  `sqrtss` (libm-0.2.16 src/math/arch/x86.rs:13), sqrt `sqrtsd` (:26). The carry's own text has it too. Not before merge.
- RR-M1 (Medium) check.test.mjs:1916 and the P3 plant (u4d2-build.md:197): the stray-unsafe proofs plant `unsafe {}` in a
  new helper, where the crate's deny makes rustc refuse it too; the case only the scan guards (an `unsafe` block inside a
  listed export's body, which that export's own #[allow(unsafe_code)] lets rustc accept) has no self-test and no plant.
  The scan's code catches it today (hashWasmProblems: only the no_mangle's `unsafe` index is allowed). Fix: one
  self-test case (an `unsafe {}` inside an export's body, refused by file:line) and the live plant once (rustc exit 0,
  deps FAIL). Before merge: yes (cheap; it is the exception's own test).
- RR-L2 (Low) crates/hash-wasm: no clippy.toml, so as the eighth LEAF crate (check.mjs LEAF) it is the only one without
  plan.md §4.2's clippy clock ban ("Clock reads are banned in them by clippy"); disallowed_types/methods are denied with
  empty lists. No wrong result can follow (no imports; Instant::now traps on wasm32-unknown-unknown). Fix: a clippy.toml
  as npt-sim's. Not before merge.
