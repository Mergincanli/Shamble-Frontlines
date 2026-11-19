# rust-reviewer, cs3-rng, block 5: an independent ChaCha keystream (djb layout: 64-bit counter, 64-bit IV), written
# from Bernstein's description, checked here against every vector of draft-strombergson-chacha-test-vectors-01
# (8, 12, 20 rounds; 128- and 256-bit keys; two blocks each). A scratch script for the notes, not project code.
# Usage: python3 -I rr-chacha-ref.py <path to draft-strombergson-chacha-test-vectors-01.txt>
import re
import sys

MASK = 0xFFFFFFFF


def rotl(x, n):
    return ((x << n) | (x >> (32 - n))) & MASK


def qr(s, a, b, c, d):
    s[a] = (s[a] + s[b]) & MASK; s[d] = rotl(s[d] ^ s[a], 16)
    s[c] = (s[c] + s[d]) & MASK; s[b] = rotl(s[b] ^ s[c], 12)
    s[a] = (s[a] + s[b]) & MASK; s[d] = rotl(s[d] ^ s[a], 8)
    s[c] = (s[c] + s[d]) & MASK; s[b] = rotl(s[b] ^ s[c], 7)


def block(key, iv8, counter, rounds):
    """One 16-word block. key: 16 or 32 bytes; iv8: 8 bytes; counter: 64-bit block number."""
    if len(key) == 32:
        const = b"expand 32-byte k"
        k = key
    else:
        const = b"expand 16-byte k"
        k = key + key
    w = lambda b, i: int.from_bytes(b[4 * i:4 * i + 4], "little")
    st = [w(const, i) for i in range(4)] + [w(k, i) for i in range(8)]
    st += [counter & MASK, (counter >> 32) & MASK, w(iv8, 0), w(iv8, 1)]
    x = list(st)
    for _ in range(rounds // 2):
        qr(x, 0, 4, 8, 12); qr(x, 1, 5, 9, 13); qr(x, 2, 6, 10, 14); qr(x, 3, 7, 11, 15)
        qr(x, 0, 5, 10, 15); qr(x, 1, 6, 11, 12); qr(x, 2, 7, 8, 13); qr(x, 3, 4, 9, 14)
    return [(x[i] + st[i]) & MASK for i in range(16)]


def words(key, iv8, rounds, start_word, n):
    """n consecutive 32-bit output words from word position start_word (what next_u32 returns, in order)."""
    out = []
    pos = start_word
    while len(out) < n:
        b = block(key, iv8, pos // 16, rounds)
        take = b[pos % 16:]
        out += take[: n - len(out)]
        pos += len(take)
    return out


def parse_draft(text):
    # Drop the page footers and headers first, so a block split by a page break reads as one.
    text = re.sub(r"\n[^\n]*\[Page \d+\]\n", "\n", text)
    text = re.sub(r"\n\f?Internet-Draft[^\n]*\n", "\n", text)
    text = text.replace("\f", "")
    hexbytes = lambda s: bytes(int(h, 16) for h in re.findall(r"0x([0-9a-fA-F]{2})", s))
    cases = []
    for m in re.finditer(r"Key:(.*?)IV:(.*?)Rounds:\s*(\d+)(.*?)Keystream block :(.*?)Keystream block 1:(.*?)(?=\n\s*Key:|\n\s*TC\d|\n\d+\.\s|\Z)", text, re.S):
        key = hexbytes(m.group(1))
        iv = hexbytes(m.group(2))
        rounds = int(m.group(3))
        b0 = hexbytes(m.group(5))
        b1 = hexbytes(m.group(6))
        cases.append((key, iv, rounds, b0[:64], b1[:64]))
    return cases


def main():
    text = open(sys.argv[1], encoding="utf-8").read()
    cases = parse_draft(text)
    ok = bad = 0
    for key, iv, rounds, b0, b1 in cases:
        if len(key) not in (16, 32) or len(iv) != 8 or len(b0) != 64 or len(b1) != 64:
            print("unparsed case", len(key), len(iv), rounds, len(b0), len(b1))
            bad += 1
            continue
        got = b"".join(x.to_bytes(4, "little") for x in block(key, iv, 0, rounds) + block(key, iv, 1, rounds))
        if got == b0 + b1:
            ok += 1
        else:
            bad += 1
            print("MISMATCH", key.hex(), iv.hex(), rounds)
    print(f"draft vectors: {ok} match, {bad} differ or unparsed, of {len(cases)}")
    r8 = [c for c in cases if c[2] == 8 and len(c[0]) == 32]
    print(f"8-round 256-bit-key vectors among them: {len(r8)}")
    # ChaCha8Rng::from_seed([0; 32]) (stream 0, word 0): the first four next_u32 values, by this script.
    print("ChaCha8, key 0, stream 0, words 0..3:", [hex(v) for v in words(bytes(32), bytes(8), 8, 0, 4)])
    # The 64-bit counter's carry: 96 words from block 2^32 - 1 (the place rand's StdRng counter test reads).
    print("ChaCha8, key 0, stream 0, words from block 2^32-1, first 2:",
          [hex(v) for v in words(bytes(32), bytes(8), 8, (2**32 - 1) * 16, 2)])


if __name__ == "__main__":
    main()
