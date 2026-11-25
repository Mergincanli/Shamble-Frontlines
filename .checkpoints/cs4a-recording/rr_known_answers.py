# rust-reviewer scratch: an independent FNV-1a 64 and canonical-bytes writer, from ADR 0004 part 9's format as
# world.rs documents it (tick u32 LE, buttons u16 LE, 16 press bytes, then the 32-byte seed). Checks the RFC
# vectors, world.rs's known answers and the committed vector line. Run: python3 -I <this file> <vector.txt path>
import sys

MASK = (1 << 64) - 1
BASIS = 0xCBF29CE484222325
PRIME = 0x100000001B3


def fnv1a_64(data: bytes) -> int:
    h = BASIS
    for b in data:
        h = ((h ^ b) * PRIME) & MASK
    return h


def state(tick, buttons, presses, seed, shared_only=False):
    out = tick.to_bytes(4, "little") + buttons.to_bytes(2, "little") + bytes(presses)
    return out if shared_only else out + bytes(seed)


def counting(first):
    return [(first + i) & 0xFF for i in range(32)]


NONE = (0, [0] * 16)
PRESSED = (0x8001, list(range(1, 17)))
checks = [
    ("rfc empty", fnv1a_64(b""), 0xCBF29CE484222325),
    ("rfc a", fnv1a_64(b"a"), 0xAF63DC4C8601EC8C),
    ("rfc foobar", fnv1a_64(b"foobar"), 0x85944171F73967E8),
    ("rfc Hello!..", fnv1a_64(b"Hello!\x01\xff\xed"), 0xBD51EA7094EE6FA1),
    ("new world", fnv1a_64(state(0, *NONE, counting(0))), 0x9C1E17D40D7A14DD),
    ("3 steps", fnv1a_64(state(3, *NONE, counting(0))), 0x111701044A87A676),
    ("seed 1..", fnv1a_64(state(0, *NONE, counting(1))), 0x6F2DD92911A2615D),
    ("start 131069", fnv1a_64(state(131069, *NONE, counting(0))), 0x7D30B61041B0AF34),
    ("start u32::MAX", fnv1a_64(state(0xFFFFFFFF, *NONE, counting(0))), 0xAEA107E621C4C5D9),
    ("played whole", fnv1a_64(state(2, *PRESSED, counting(0))), 0xD01FFE7C10646F36),
    ("played shared", fnv1a_64(state(2, *PRESSED, counting(0), True)), 0x3F6ED6EDBE368DB6),
    ("shared new world", fnv1a_64(state(0, *NONE, counting(0), True)), 0xF84841652498787D),
]
bad = 0
for name, got, want in checks:
    ok = got == want
    bad += not ok
    print(f"{'ok ' if ok else 'BAD'} {name:18} got {got:016x} want {want:016x}")

if len(sys.argv) > 1:
    for line in open(sys.argv[1], encoding="utf8"):
        if " shared bytes " in line:
            parts = line.split()
            raw = bytes.fromhex(parts[parts.index("bytes") + 1])
            want = int(parts[parts.index("fnv1a64") + 1], 16)
            got = fnv1a_64(raw)
            ok = got == want
            bad += not ok
            print(f"{'ok ' if ok else 'BAD'} vector line ({len(raw)} bytes) got {got:016x} want {want:016x}")
print("ALL OK" if bad == 0 else f"{bad} BAD")
