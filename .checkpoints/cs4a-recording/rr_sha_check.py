"""rust-reviewer r1 scratch: checks rr_sha_diff's native output against hashlib, and the wasm32 numbers given.
Usage: python3 -I rr_sha_check.py <native-output-file> <wasm-fold-hex> <wasm-long-hex>"""
import hashlib
import sys


def message(length):
    return bytes(((i * 7 + i // 251) & 0xFF) for i in range(length))


def fold(digests):
    h = 0xCBF29CE484222325
    for d in digests:
        for b in d:
            h = ((h ^ b) * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
    return h


lines = open(sys.argv[1]).read().split("\n")
digests, bad, count = [], 0, 0
native_fold = native_long = None
for line in lines:
    if not line:
        continue
    key, value = line.split(" ")
    if key == "fold":
        native_fold = value
        continue
    if key == "long":
        native_long = value
        continue
    want = hashlib.sha256(message(int(key))).digest()
    digests.append(want)
    count += 1
    if want.hex() != value:
        bad += 1
        print("DIFFERS at length", key)
expect_fold = "%016x" % fold(digests)
long_digest = hashlib.sha256()
long_digest.update(b"a" * ((1 << 29) + 1))
expect_long = long_digest.hexdigest()[:16]
print(f"lengths compared {count}, differing {bad}")
print(f"fold: hashlib {expect_fold}, native {native_fold}, wasm32 {sys.argv[2]}")
print(f"2^29+1 bytes of 'a' (bits = 2^32 + 8): hashlib {expect_long}, native {native_long}, wasm32 {sys.argv[3]}")
ok = bad == 0 and expect_fold == native_fold == sys.argv[2] and expect_long == native_long == sys.argv[3]
print("ALL EQUAL" if ok else "MISMATCH")
