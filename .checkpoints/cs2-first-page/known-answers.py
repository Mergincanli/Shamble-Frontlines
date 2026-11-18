#!/usr/bin/env python3
"""A second implementation, not the Rust code: the known answers the tests of change-set 2 hold.
FNV-1a, 64 bits (RFC 9923): offset basis 0xcbf29ce484222325, prime 0x100000001b3.
The world's canonical bytes: tick as u32 little-endian, then the 32 seed bytes."""
MASK = (1 << 64) - 1


def fnv1a_64(data: bytes) -> int:
    h = 0xCBF29CE484222325
    for b in data:
        h = ((h ^ b) * 0x100000001B3) & MASK
    return h


def world_bytes(tick: int, seed: bytes) -> bytes:
    assert len(seed) == 32
    return tick.to_bytes(4, "little") + seed


if __name__ == "__main__":
    for s in (b"", b"a", b"foobar"):
        print(f"fnv1a_64({s!r}) = 0x{fnv1a_64(s):016x}")
    seed = bytes(range(32))
    for tick in (0, 3, 0xFFFFFFFF):
        b = world_bytes(tick, seed)
        print(f"world(seed 0..31, tick {tick}): {len(b)} bytes, hash 0x{fnv1a_64(b):016x}")
    other = bytes(range(1, 33))
    print(f"world(seed 1..32, tick 0): hash 0x{fnv1a_64(world_bytes(0, other)):016x}")
