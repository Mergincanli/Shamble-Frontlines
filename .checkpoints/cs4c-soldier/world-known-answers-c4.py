# The second implementation of world.rs's known answers (change-set 4c-2, C4): the canonical state bytes built here
# from rust-reviewer's block 11.2 table, not from the Rust code, and hashed with FNV-1a 64. It first reproduces the
# old known answers (the state before the soldier table: tick, record, seed), which checks this file's FNV-1a and byte
# order, then prints the new ones. Not part of the product: notes.
# Usage: python3 -I -B .checkpoints/cs4c-soldier/world-known-answers-c4.py
import struct

BUTTONS = 16
ROWS = 48


def fnv1a_64(data):
    h = 0xCBF29CE484222325
    for byte in data:
        h ^= byte
        h = (h * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
    return h


def seed(first):
    return bytes((first + k) & 0xFF for k in range(32))


def f32(value):
    return struct.pack("<f", value)


EMPTY_ROW = bytes(60)


def row(generation, flags, team, position, velocity, stamina, facing, held_move, aim_cm, pull):
    """One row in 11.2's order: u16, u8, u8, 3 f32, 3 f32, u64, 2 f32, 2 i8, i16 u16 i16, 2 f32 (60 bytes)."""
    out = struct.pack("<HBB", generation, flags, team)
    out += b"".join(f32(v) for v in position) + b"".join(f32(v) for v in velocity)
    out += struct.pack("<Q", stamina)
    out += b"".join(f32(v) for v in facing)
    out += struct.pack("<bb", *held_move)
    out += struct.pack("<hHh", *aim_cm)
    out += b"".join(f32(v) for v in pull)
    assert len(out) == 60
    return out


# The test bundle's soldier at tick 0 (content's map TEST_TEXT spawn "a" at (0, 0, -5) facing 90; the movement
# TEST_TEXT's 8 s of sprint and 4 s of refill: 24,000 x 12,000 units).
NONE_AIM = (-32768, 0, 0)
SPAWNED = dict(generation=1, flags=0b111, team=0, position=(0.0, 0.0, -5.0), velocity=(0.0, 0.0, 0.0),
               stamina=24_000 * 12_000, facing=(1.0, 0.0), held_move=(0, 0), aim_cm=NONE_AIM, pull=(0.0, 0.0))


def state(tick, buttons, presses, rows, seed_bytes, with_rows=True):
    out = struct.pack("<IH", tick, buttons) + bytes(presses)
    if with_rows:
        out += b"".join(rows)
    return out, out + seed_bytes


PRESSED = (0x8001, list(range(1, 17)))
ZERO = (0, [0] * BUTTONS)

# The old answers (no rows), as world.rs had them before C4.
old = [
    (state(0, *ZERO, [], seed(0), False)[1], 0x9C1E17D40D7A14DD),
    (state(3, *ZERO, [], seed(0), False)[1], 0x111701044A87A676),
    (state(0, *ZERO, [], seed(1), False)[1], 0x6F2DD92911A2615D),
    (state(131_069, *ZERO, [], seed(0), False)[1], 0x7D30B61041B0AF34),
    (state(0xFFFFFFFF, *ZERO, [], seed(0), False)[1], 0xAEA107E621C4C5D9),
    (state(2, *PRESSED, [], seed(0), False)[1], 0xD01FFE7C10646F36),
]
for data, want in old:
    assert fnv1a_64(data) == want, (hex(fnv1a_64(data)), hex(want))
shared_old = state(2, *PRESSED, [], seed(0), False)[0]
assert fnv1a_64(shared_old) == 0x3F6ED6EDBE368DB6
print("the old known answers: reproduced (FNV-1a 64 and the byte order hold)")

soldier = [row(**SPAWNED)] + [EMPTY_ROW] * (ROWS - 1)
moved_row = dict(SPAWNED, held_move=(127, -3), aim_cm=(100, 50, -200))
moved = [row(**moved_row)] + [EMPTY_ROW] * (ROWS - 1)
empty = [EMPTY_ROW] * ROWS


def show(name, shared_and_whole, which="state"):
    shared, whole = shared_and_whole
    value = fnv1a_64(whole if which == "state" else shared)
    print(f"{name} {which} 0x{value >> 48:04x}_{(value >> 32) & 0xFFFF:04x}_{(value >> 16) & 0xFFFF:04x}_{value & 0xFFFF:04x}")


assert len(state(0, *ZERO, soldier, seed(0))[0]) == 2902
show("new(0, 0)", state(0, *ZERO, soldier, seed(0)))
show("new(0, 0) after 3 empty ticks", state(3, *ZERO, soldier, seed(0)))
show("new(1, 0)", state(0, *ZERO, soldier, seed(1)))
show("new(0, 131069)", state(131_069, *ZERO, soldier, seed(0)))
show("new(0, u32::MAX)", state(0xFFFFFFFF, *ZERO, soldier, seed(0)))
show("played: [], [pressed]", state(2, *PRESSED, soldier, seed(0)))
show("played: [], [pressed]", state(2, *PRESSED, soldier, seed(0)), "shared")
show("moved: [moved] (move (127, -3), aim (100, 50, -200))", state(1, *PRESSED, moved, seed(0)))
show("no spawn: new(0, 0)", state(0, *ZERO, empty, seed(0)))
show("new(0, 0)", state(0, *ZERO, soldier, seed(0)), "shared")
