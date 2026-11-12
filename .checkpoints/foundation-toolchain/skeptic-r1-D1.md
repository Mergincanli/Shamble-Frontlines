# Skeptic, review round 1, D1 (= RR-1, High): head 4d78ad6

Plan: (1) re-read the head's Cargo.toml, the crate manifests, ADR 0002 part 4 and PM-0002-53, the report's E1 and
Outcome wording; (2) re-run the reviewer's script into my own folder (.gate/skeptic-D1/repro) and then an independent
variant of mine (.gate/skeptic-D1/own: different functions, a sim integration test, cargo -v flags, a wasm32 cdylib run
in Node so the Wasm claim rests on behaviour, not on a string search); (3) read the cited rustc 1.99.1 and Cargo book
sources; (4) test fix A (release and bench, room, sim, protocol, wasm, a non-member crate) and look for what it breaks or
leaves open; (5) judge holds and severity on CLAUDE.md's scale. Nothing in the worktree changes but this file.

## Checks
- C1 toolchain: rustc 1.99.1 (58cf8f9c3 2026-10-13), cargo 1.99.1, LLVM 23.1.1; targets x86_64 and wasm32-unknown-unknown
  installed; Node 22.23.4. Head 4d78ad6: Cargo.toml lines 21-26 are the two package blocks only; no crate depends on
  another (all six manifests have no [dependencies]); sim and protocol hold one private #[cfg(test)] overflow test each.
- C2 the reviewer's script re-run into .gate/skeptic-D1/repro: test profile 5 of 5 panic; release with the head's blocks:
  plain, inline, generic, Tick::next FAILED (did not panic), only #[inline(never)] panics; fix A: 5 of 5 panic. Same as
  the reviewer's record.
- C3 my own variant (.gate/skeptic-D1/setup-own.sh into .gate/skeptic-D1/own; catch_unwind, values printed, not only
  "did not panic"). Test profile: room 8 of 8 panic, sim's integration test 8 of 8. Release with the head's blocks, from
  room: counter_step(u16::MAX) returned 0, sub_plain(0,1) 255, neg_plain(i8::MIN) -128, shl_plain(1,40) 256 (masked),
  Fx(i32::MAX)*Fx(2) (a Mul impl, no attribute) -2, sum_all([u32::MAX,1]) (generic) 0: six silent wraps. with_call (plain
  but calls a non-inlined helper, so not a leaf) panicked; div_plain(i8::MIN,-1) panicked (division overflow is never an
  optional check: the control). sim's integration test in release: 8 of 8 panic, so a test inside the sim package cannot
  see the hole (RR-1's claim holds). The hole is shape-dependent: small leaf, #[inline], trait-impl and generic code.
- C4 flags (cargo build --release -v after a clean): sim (lib) gets -C opt-level=3 -C overflow-checks=on; room (lib) and
  sim_wasm get -C opt-level=3 with no overflow flag (rustc's default: off without debug assertions). Holds.
- C5 wasm32, behaviour not strings (.gate/skeptic-D1/node/run-wasm.mjs, a sim-wasm cdylib with four exports into sim,
  run in Node 22): dev build (the debug client) traps (RuntimeError: unreachable) on all four overflows; release build
  (20,548 bytes raw) returns 0 for counter_step(65535), 0 for the generic sum, -2 for the Fx Mul, and traps only for
  with_call (code-generated in sim). Its file keeps "attempt to add with overflow" (with_call's) but has no "attempt to
  multiply with overflow". So a debug client and a release client disagree today on the bug path, and a debug client and
  a release server (room, C3) too: ADR 0002 part 4's "never disagree on overflow" does not hold with part 4's mechanism.
- C6 fix A on my variant (own): release, room 8 of 8 panic and sim's integration test 8 of 8; `cargo test --profile
  bench -p room -p sim -p protocol`: room 8 of 8, sim's and protocol's overflow tests pass; a custom profile with
  inherits = "release": room 8 of 8. Release wasm32 under fix A (20,825 bytes raw): all four exports trap. Flags under
  fix A: sim and room get -C overflow-checks=on; a path crate outside the workspace (extdep) gets none.
- C7 fix A on a pristine copy of the head (.gate/skeptic-D1/pristineA), CI's own lines: `cargo test --workspace
  --locked` 6 of 6 unit tests pass; K1b `cargo test --release -p sim -p protocol --locked` 2 of 2; the wasm32 build of
  the five crates OK; clippy -D warnings clean; Cargo.lock byte-identical. Planted: fix A without its release line, K1b
  red in both sim and protocol ("did not panic as expected"); restored, green. So K1b still guards fix A's line, but it
  cannot tell fix A from the head's blocks (no cross-crate test): the CI proof needs a release test outside sim and
  protocol, i.e. a change to the K1b line (a .github edit).
- C8 what fix A leaves: a crate outside the workspace (extdep) panics in the test profile and returns 0 in release under
  fix A ("*" keeps the release default, as RR-1 says). Third-party integer arithmetic still differs between dev and
  release builds; that is outside D1 (sim's own code), a wording limit for any rewording of part 4's "never disagree".
- C9 sources, read at tag 1.99.1 (raw.githubusercontent.com, saved in .gate/skeptic-D1/src-dl/): rustc_codegen_ssa
  mir/block.rs:747-753, line 751 `if !bx.sess().overflow_checks() && msg.is_optional_overflow_check() { const_cond =
  Some(expected); }` (its comment says it arises "only" from #[rustc_inherit_overflow_checks] fns; C3 shows user crates
  hit it too); rustc_mir_transform cross_crate_inline.rs:117-118 "Assert terminators are similar to calls ... so those
  are just treated as statements", :55-56 #[inline] always inlinable, #[inline(never)] never, :79 and :88 no inference
  when incremental or at opt-level 0 (so release, not dev); rustc_mir_build builder/mod.rs:764-766, the asserts are
  emitted only when the defining crate checks overflow (why "*" leaves third-party code unchecked even inside a checked
  crate). Cargo book 1.99.1 profiles: "the code may be generated in the crate where it is instantiated, and thus may use
  the optimization settings of that crate"; `[profile.dev.package."*"]` "For any non-workspace member". All as cited.
- C10 reach when code exists (.gate/skeptic-D1/own2): tick_all, a non-generic sim loop that calls counter_step, called
  from room in release, returned [2, 0, 4] (wrapped); its generic twin too; both panic in the test profile. After MIR
  inlining it has no calls left, so it is cross-crate inlinable as well (rustc_session options.rs:2422, the threshold
  defaults to 100 MIR statements). Only a function with a call that survives optimization (with_call's #[inline(never)]
  helper, C3) or above the threshold is generated in sim and keeps its checks. So which sim code is checked in a release
  build depends on optimizer heuristics, not on anything visible in the source: the finding's opening phrase ("where the
  sim actually runs") is loose (large sim functions with real calls keep their checks), but its body is precise, and the
  hole's reach once code exists is wide (operators on sim types, accessors, counters, short loops, generics).
- C11 timing: reach is zero at 4d78ad6 (no crate depends on sim, sim has no public item); the first cross-crate sim call
  comes with the engine code Fable 5.2 builds after 2.2 (checkpoint, decisions of 2026-11-12). Holds.
