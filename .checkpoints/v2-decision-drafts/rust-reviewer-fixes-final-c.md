# rust-reviewer: follow-up c on ADR 0004 (RR-11, part 9), from the doc on disk (2026-11-02, about 20:45 Amsterdam)

BEFORE copied by script from docs/adr/0004-simulation-core.md as it is in the working tree (whole lines).

### FIX RR-11 (part 9: no usize in serialized or hashed bytes)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
   each tick the sim writes a canonical serialization of its whole state (fixed field order, floats by `to_bits`, the
   streams' state, the tick counter), field by field and never as raw struct bytes (padding bytes are uninitialised,
   R1-RR-OQ6), and hashes it with a fixed, unseeded hash function: the state hash (which function is picked in the sim
AFTER:
   each tick the sim writes a canonical serialization of its whole state (fixed field order, floats by `to_bits`, the
   streams' state, the tick counter), field by field and never as raw struct bytes (padding bytes are uninitialised,
   R1-RR-OQ6), with every integer, every length and every index at a fixed width and little-endian, and no `usize`
   in the serialized or hashed bytes (proposed: `usize` is 32 bits on wasm32 and 64 bits on x86-64, so a length
   written as `usize`, or a hasher fed through `std::hash::Hasher`, whose derived `Hash` writes slice lengths with
   `write_usize`, would differ by target at tick 0; PM-0004-21 checks it). It hashes those bytes as one byte string
   with a fixed, unseeded hash function: the state hash (which function is picked in the sim
END
