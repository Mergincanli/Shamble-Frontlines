#!/usr/bin/env python3
"""Makes the random stream's vector file (PM-0004-27): chacha8-words.bin and chacha8.manifest.txt beside this script.

A second implementation of ChaCha with 8 rounds, written from Bernstein's description and independent of the Rust
code it checks (crates/sim/src/rng.rs). Before it writes anything it must reproduce the eight published test vectors
that the Rust file's tests hold (TC1 to TC8 of draft-strombergson-chacha-test-vectors-01, 8 rounds, 256-bit key).

  python3 -B crates/devtools/vectors/make_chacha8_vectors.py           writes the two files
  python3 -B crates/devtools/vectors/make_chacha8_vectors.py --check   compares them byte for byte, writes nothing

Python's standard library only. The layout below is also what crates/devtools/src/rules/stream.rs expects.
"""
import hashlib
import pathlib
import re
import sys

HERE = pathlib.Path(__file__).resolve().parent
RNG_RS = HERE.parents[2] / "crates/sim/src/rng.rs"
MASK = 0xFFFFFFFF
# The key of every section: the published test key of TC8 ("All your base are belong to us!").
KEY = bytes.fromhex("c46ec1b18ce8a878725a37e780dfb7351f68ed2e194c79fbc6aebee1a667975d")
WORDS = 10_000
# (stream id, first word, word count): four streams from their start, at the edges of both halves of the id; then
# 96 words of stream 1 from block 2^32 - 3, across the carry of the block counter's low word into its high word.
SECTIONS = [(0, 0, WORDS), (1, 0, WORDS), (1 << 32, 0, WORDS), ((1 << 64) - 1, 0, WORDS), (1, ((1 << 32) - 3) * 16, 96)]


def rotl(x, n):
    return ((x << n) | (x >> (32 - n))) & MASK


def quarter(s, a, b, c, d):
    s[a] = (s[a] + s[b]) & MASK; s[d] = rotl(s[d] ^ s[a], 16)
    s[c] = (s[c] + s[d]) & MASK; s[b] = rotl(s[b] ^ s[c], 12)
    s[a] = (s[a] + s[b]) & MASK; s[d] = rotl(s[d] ^ s[a], 8)
    s[c] = (s[c] + s[d]) & MASK; s[b] = rotl(s[b] ^ s[c], 7)


def block(key, stream, number):
    """The sixteen words of one block: constants, key, the 64-bit block number, the 64-bit stream id."""
    start = [int.from_bytes(b"expand 32-byte k"[i:i + 4], "little") for i in range(0, 16, 4)]
    start += [int.from_bytes(key[i:i + 4], "little") for i in range(0, 32, 4)]
    start += [number & MASK, number >> 32, stream & MASK, stream >> 32]
    x = list(start)
    for _ in range(4):
        quarter(x, 0, 4, 8, 12); quarter(x, 1, 5, 9, 13); quarter(x, 2, 6, 10, 14); quarter(x, 3, 7, 11, 15)
        quarter(x, 0, 5, 10, 15); quarter(x, 1, 6, 11, 12); quarter(x, 2, 7, 8, 13); quarter(x, 3, 4, 9, 14)
    return [(x[i] + start[i]) & MASK for i in range(16)]


def words(key, stream, first, count):
    """`count` words from word `first`, as little-endian bytes."""
    out = []
    for number in range(first // 16, (first + count + 15) // 16):
        out += block(key, stream, number)
    return b"".join(word.to_bytes(4, "little") for word in out[first % 16:first % 16 + count])


def published():
    """The eight published vectors as the Rust tests hold them: (key, IV, the first two keystream blocks)."""
    table = RNG_RS.read_text(encoding="utf-8").split("const PUBLISHED", 1)[1].split("];", 1)[0]
    cases = re.findall(r'\("([0-9a-f]{64})", "([0-9a-f]{16})", concat!\(((?:\s*"[0-9a-f]{64}",)+)\s*\)\)', table)
    return [(bytes.fromhex(key), bytes.fromhex(iv), bytes.fromhex("".join(re.findall(r'"([0-9a-f]{64})"', stream)))) for key, iv, stream in cases]


def main():
    cases = published()
    assert len(cases) == 8, f"expected the 8 published vectors in {RNG_RS}, found {len(cases)}"
    for number, (key, iv, keystream) in enumerate(cases, 1):
        assert len(keystream) == 128 and words(key, int.from_bytes(iv, "little"), 0, 32) == keystream, f"TC{number}: this script does not give the published keystream"
    parts = [words(KEY, stream, first, count) for stream, first, count in SECTIONS]
    data = b"".join(parts)
    lines = ["chacha8-words.bin: ChaCha with 8 rounds, 32-bit words in drawing order, each little-endian",
             f"made by make_chacha8_vectors.py after it reproduced the 8 published vectors in crates/sim/src/rng.rs",
             f"key {KEY.hex()}", f"bytes {len(data)}  sha256 {hashlib.sha256(data).hexdigest()}"]
    offset = 0
    for (stream, first, count), part in zip(SECTIONS, parts):
        lines.append(f"at byte {offset}: stream {stream}, words {first} to {first + count - 1} ({count})  sha256 {hashlib.sha256(part).hexdigest()}")
        offset += len(part)
    files = {"chacha8-words.bin": data, "chacha8.manifest.txt": ("\n".join(lines) + "\n").encode()}
    if "--check" in sys.argv[1:]:
        differing = [name for name, content in files.items() if not (HERE / name).is_file() or (HERE / name).read_bytes() != content]
        print("differs or is missing: " + ", ".join(differing) if differing else f"both files are byte for byte what this script makes ({len(data)} bytes of words)")
        return 1 if differing else 0
    for name, content in files.items():
        (HERE / name).write_bytes(content)
    print(lines[3])
    return 0


if __name__ == "__main__":
    sys.exit(main())
