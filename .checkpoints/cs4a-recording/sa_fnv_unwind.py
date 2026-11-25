# security-auditor, cs4a round 1: why no state hash over bytes a client is not sent may reach that client.
# FNV-1a 64 (state_hash.rs): h = basis; for each byte b: h = (h ^ b) * prime mod 2^64. The prime is odd, so each step
# can be undone: h_before = (h_after * inverse(prime)) ^ b. Given the hash and every byte but a few, the few come back.
# Run: python3 -I sa_fnv_unwind.py  (no input, no network; prints what it recovered and the work it took)
import random
import time

BASIS = 0xCBF29CE484222325
PRIME = 0x00000100000001B3
MASK = (1 << 64) - 1
INV = pow(PRIME, -1, 1 << 64)


def forward(h, data):
    for b in data:
        h = ((h ^ b) * PRIME) & MASK
    return h


def backward(h, data):
    for b in reversed(data):
        h = ((h * INV) & MASK) ^ b
    return h


def fnv(data):
    return forward(BASIS, data)


def recover(hash_value, before, after, n):
    """The n unknown bytes between known `before` and `after`, by meeting in the middle."""
    start = forward(BASIS, before)
    end = backward(hash_value, after)
    half = n // 2
    table = {}
    for x in range(1 << (8 * half)):
        table.setdefault(forward(start, x.to_bytes(half, "little")), []).append(x)
    found = []
    for y in range(1 << (8 * (n - half))):
        mid = backward(end, y.to_bytes(n - half, "little"))
        for x in table.get(mid, []):
            found.append(x.to_bytes(half, "little") + y.to_bytes(n - half, "little"))
    return found


rng = random.Random(4)
# A 4c-like shared state: 22 known bytes, then a hidden enemy's u32 (credits), then 40 more known bytes.
before = bytes(rng.randrange(256) for _ in range(22))
secret = rng.randrange(1 << 32).to_bytes(4, "little")
after = bytes(rng.randrange(256) for _ in range(40))
h = fnv(before + secret + after)
t = time.perf_counter()
got = recover(h, before, after, 4)
print(f"4 hidden bytes {secret.hex()}: recovered {[g.hex() for g in got]} in {time.perf_counter() - t:.2f} s "
      f"(2 x 65,536 steps)")
# Two hidden bytes (a u16 cooldown): 256 x 2 steps.
secret2 = rng.randrange(1 << 16).to_bytes(2, "little")
h2 = fnv(before + secret2 + after)
got2 = recover(h2, before, after, 2)
print(f"2 hidden bytes {secret2.hex()}: recovered {[g.hex() for g in got2]}")
# Control: the inverse is right (unwinding the whole input from the hash gives the basis back).
assert backward(h, before + secret + after) == BASIS
print("control: unwinding every byte from the hash gives the offset basis back")
