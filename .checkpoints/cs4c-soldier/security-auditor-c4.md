# security-auditor: C4 read (fb2030ed, parent 3ac717b4) against F2, ruling 8 (a)-(d), F9

Read only: notes here, scratch under .gate/. No checkout in the worktree.

## Plan
1. maths-allowed.mjs at fb2030ed: 21 entries vs maths-refusals-c4.txt; each why vs libm 0.2.16 source (rem_pio2_large,
   the header's "own Rust path" claim, C::round's second caller face_cm). Rule on the 6 parent-covered entries.
2. build-checks.test.mjs departure: exact carve-out pair; planted twin; condition-1 block; anything lifted wrongly.
2b. support::env / float_traits / int_traits: what else lives there (non-exact methods, fma); version pin note;
   rem_pio2_large vs F2.
3. room on_datagram: refused_move, refused_aim, NONE tie, JOIN_SLOT, no panic / no per-tick alloc, `handed` cfg(test),
   any_bytes fuzz (run it).
4. collision.rs bounds: WorldError::Bounds, face_cm, aim_box, holds_cm, world past reach.
5. sim-wasm local_constants, CreateError messages (no seed byte), constructor check order.
6. devtools replay.rs Lines: seed never printed.
7. git show --stat: dependencies / dynamic text / storage unchanged.
8. Verdict.

## Notes
