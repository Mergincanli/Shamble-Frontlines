# verifier scratch: three of the hash report's detection claims, run on my own code (not rust-reviewer's).
#   item 2: a change in one byte is always caught
#   item 9: two sign flips (top bit of two bytes) leave the low 8 hash bits unchanged; the 9th bit changes about half
#   item 7: FNV fed 8-byte words: two top-bit flips in different words cancel (the whole hash is unchanged)
# Run: python3 -I verifier-fnv-claims.py <first-loop.vector.txt>
import random
import sys

MASK = (1 << 64) - 1
BASIS, PRIME = 0xCBF29CE484222325, 0x100000001B3


def fnv(data):
    h = BASIS
    for b in data:
        h = ((h ^ b) * PRIME) & MASK
    return h


def fnv_words(data):
    h = BASIS
    for i in range(0, len(data), 8):
        h = ((h ^ int.from_bytes(data[i:i + 8], "little")) * PRIME) & MASK
    return h


rng = random.Random(20261125)

# The real shared state from the committed vector line, and the whole state with a TESTSEED seed after it.
shared = None
for line in open(sys.argv[1], encoding="utf8"):
    if " shared bytes " in line:
        parts = line.split()
        shared = bytes.fromhex(parts[parts.index("bytes") + 1])
assert shared is not None and len(shared) == 22
whole = shared + b"TESTSEED" + bytes(range(24))

# item 2, exhaustive on the real 54-byte state: every position, every other value.
base = fnv(whole)
misses = sum(fnv(whole[:i] + bytes([v]) + whole[i + 1:]) == base for i in range(len(whole)) for v in range(256) if v != whole[i])
print(f"item 2, 54-byte state, all {len(whole) * 255} one-byte changes: {misses} hash like the original")

# item 2, on a 3,076-byte state: 24 random positions, every other value at each (6,120 changed copies).
big = bytes(rng.randrange(256) for _ in range(3076))
base = fnv(big)
positions = rng.sample(range(len(big)), 24)
misses = sum(fnv(big[:i] + bytes([v]) + big[i + 1:]) == base for i in positions for v in range(256) if v != big[i])
print(f"item 2, 3,076-byte state, 24 positions x 255 values: {misses} hash like the original")

# item 9: two sign flips in a 3,076-byte state, 3,000 trials.
low8_same = bit9_changed = 0
for _ in range(3000):
    i, j = rng.sample(range(len(big)), 2)
    flipped = bytearray(big)
    flipped[i] ^= 0x80
    flipped[j] ^= 0x80
    a, b = fnv(big), fnv(bytes(flipped))
    low8_same += (a ^ b) & 0xFF == 0
    bit9_changed += ((a ^ b) >> 8) & 1
print(f"item 9, two sign flips, 3000 trials: low 8 bits unchanged in {low8_same}; 9th bit changed in {bit9_changed}")

# item 7: 8-byte words, a top-bit flip in two different words, 10,000 trials.
same = 0
for _ in range(10000):
    i, j = rng.sample(range(len(big) // 8), 2)
    flipped = bytearray(big)
    flipped[i * 8 + 7] ^= 0x80
    flipped[j * 8 + 7] ^= 0x80
    same += fnv_words(big) == fnv_words(bytes(flipped))
print(f"item 7, word-fed FNV, two top-bit flips in different words, 10000 trials: whole hash unchanged in {same}")
