# U4e round 1: rust-reviewer notes

Seat: rust-reviewer (read only). Read copy: .gate/u4e-r1 (97bb9d3 + U4e snapshot). Builder copy:
.claude/worktrees/wf_0fa6595a-3e2-1. Packet: u4e-packet.md, u4e-packet-tests.md. Log prefix: rr.

## Plan
1. Read the packet's judged-against sections (plan §5 U4e, §4.1, §4.2, §4.10, W1-*, advisor-u4e-brief, B2.1, B5,
   carries 21-27, 39, 50, 51, 53-57, 61, 68, 40, 62, 66; the left-out ids).
2. Read the sources in .gate/u4e-r1: crates/room/src (lib, roster, slot, ack, encoder, events, interest, flood,
   priority, input, rewind), clippy.toml files, crates/server/src (pump/decoder, limits, counters, lib.rs),
   crates/client/src (endpoint), crates/sim/src/scene.rs (effect_rows).
3. Focus checks, each a block: (a) integer-only / float lint, wraps, budget sums, overflow; (b) hash-map bans, order,
   canonical sort (carry 21); (c) panics: unwrap/expect/index/slice/debug_assert/as casts, reach from network input;
   (d) allocation per tick at 48 clients, carry 26, quadratic paths; (e) constant-time compare (black_box, fold);
   (f) Send (e8), wasm32 build, pub fields vs accessors, docs, lints, tests that pass for the wrong reason;
   (g) Scene::effect_rows (A19).
4. Optional runs through u4e-run.sh with prefix rr (clippy, wasm32 build) if needed to settle a point.
5. Verdict.

## Findings (a line after each)
