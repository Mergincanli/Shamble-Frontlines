#!/usr/bin/env python3
"""Makes the float vectors (PM-0004-22): f32-inputs.bin and f32-expected.txt beside this script.

For every float operation ADR 0004 part 7 allows, 10,000 fixed inputs and the results an exact reference gives: each
result is worked out in rational arithmetic and rounded once to f32 (nearest, ties to even), independent of any
float unit's own rounding. So `sqrt` "equals a correctly rounded reference" by construction, and the native build
and the WebAssembly build must each equal the reference, hence each other.

  python3 -B crates/devtools/vectors/make_f32_vectors.py                 writes the two files
  python3 -B crates/devtools/vectors/make_f32_vectors.py --check         compares them byte for byte, writes nothing
  python3 -B crates/devtools/vectors/make_f32_vectors.py --full <op>     prints every result of one operation

Inputs: 10,000 cases of three f32 bit patterns (a, b, c), little-endian: every ordered pair of the edge values
first, then seeded values. No input is a NaN (part 7: a NaN never enters the simulation); a NaN result is written as
0x7FC00000, because only which cases give a NaN must agree, not a NaN's bits. Expected: one line per operation with
the bytes of one result and FNV-1a 64 over all its results in order, each little-endian.
Python's standard library only. crates/devtools/src/rules/f32_ops.rs runs the same operations in the same order.
"""
import hashlib
import math
import pathlib
import struct
import sys
from fractions import Fraction

HERE = pathlib.Path(__file__).resolve().parent
CASES = 10_000
NAN = 0x7FC00000
INF = 0x7F800000


def value(bits):
    """An f32 bit pattern as a Python float (every f32 is exactly a double)."""
    return struct.unpack("<f", struct.pack("<I", bits))[0]


def bits_of(double):
    return struct.unpack("<I", struct.pack("<f", double))[0]


def rounded(q, negative=None):
    """The exact number `q` rounded once to f32, as bits. A zero takes its sign from `negative`."""
    negative = q < 0 if negative is None or q != 0 else negative
    q = abs(Fraction(q))
    if q == 0:
        return 0x80000000 if negative else 0
    e = max(q.numerator.bit_length() - q.denominator.bit_length() - 1, -126)
    if q >= Fraction(2) ** (e + 1) and e >= -126:
        e += 1
    scaled = q / Fraction(2) ** (e - 23)
    m, rest = divmod(scaled.numerator, scaled.denominator)
    if 2 * rest > scaled.denominator or (2 * rest == scaled.denominator and m & 1):
        m += 1
    if m == 1 << 24:
        m, e = m >> 1, e + 1
    bits = INF if e > 127 else m if m < 1 << 23 else ((e + 127) << 23) | (m - (1 << 23))
    return bits | (0x80000000 if negative else 0)


def arithmetic(double, exact):
    """IEEE's special results (NaN, an infinity, a signed zero) are the double's; any other result is `exact`'s."""
    if math.isnan(double):
        return NAN
    if math.isinf(double) or double == 0:
        return bits_of(double)
    return rounded(exact(), double < 0)


def root(a):
    if a < 0:
        return NAN
    if a == 0 or math.isinf(a):
        return bits_of(a)
    q = Fraction(a)
    shift = 2 * (q.denominator.bit_length() // 2 + 40)
    n = q * (1 << shift)
    assert n.denominator == 1
    r = math.isqrt(n.numerator)
    # A square root of an f32 is never half-way between two f32: a hair more than the cut-off root decides.
    return rounded(Fraction(2 * r + (r * r != n.numerator), 2 << (shift // 2)))


def whole(a, pick):
    """floor, ceil, trunc or round: an integer result, with the input's sign when it is zero."""
    return bits_of(a) if math.isinf(a) else rounded(Fraction(pick(Fraction(a))), math.copysign(1, a) < 0)


def away(q):
    """`round`: to the nearest whole number, a half away from zero."""
    n = math.floor(abs(q) + Fraction(1, 2))
    return -n if q < 0 else n


def fract(a):
    """Rust's `fract` is `self - self.trunc()`, and the whole part keeps the input's sign as a zero too."""
    if math.isinf(a):
        return NAN
    part = math.copysign(math.trunc(a), a)
    return arithmetic(a - part, lambda: Fraction(a) - Fraction(part))


def to_int(a, low, high):
    return min(max(low if a == -math.inf else high if a == math.inf else int(a), low), high)


def signed(bits, width):
    return bits - (1 << width) if bits >> (width - 1) else bits


def total_order(bits):
    return -(bits & 0x7FFFFFFF) - 1 if bits >> 31 else bits


F = lambda x: Fraction(x)
# (name, bytes of one result, the reference): x, y, z are the inputs' bits and a, b, c their values.
OPS = [
    ("add", 4, lambda x, y, z, a, b, c: arithmetic(a + b, lambda: F(a) + F(b))),
    ("sub", 4, lambda x, y, z, a, b, c: arithmetic(a - b, lambda: F(a) - F(b))),
    ("mul", 4, lambda x, y, z, a, b, c: arithmetic(a * b, lambda: F(a) * F(b))),
    ("div", 4, lambda x, y, z, a, b, c: NAN if a == b == 0 else INF | ((x ^ y) & 0x80000000) if b == 0 else arithmetic(a / b, lambda: F(a) / F(b))),
    ("neg", 4, lambda x, y, z, a, b, c: x ^ 0x80000000),
    ("lt", 1, lambda x, y, z, a, b, c: a < b),
    ("le", 1, lambda x, y, z, a, b, c: a <= b),
    ("gt", 1, lambda x, y, z, a, b, c: a > b),
    ("ge", 1, lambda x, y, z, a, b, c: a >= b),
    ("eq", 1, lambda x, y, z, a, b, c: a == b),
    ("ne", 1, lambda x, y, z, a, b, c: a != b),
    ("total_cmp", 1, lambda x, y, z, a, b, c: ((total_order(x) > total_order(y)) - (total_order(x) < total_order(y))) & 0xFF),
    ("sqrt", 4, lambda x, y, z, a, b, c: root(a)),
    # Half of the inputs are negative, where `sqrt` is a NaN: this one gives a root for every case.
    ("sqrt_abs", 4, lambda x, y, z, a, b, c: root(abs(a))),
    ("floor", 4, lambda x, y, z, a, b, c: whole(a, math.floor)),
    ("ceil", 4, lambda x, y, z, a, b, c: whole(a, math.ceil)),
    ("round", 4, lambda x, y, z, a, b, c: whole(a, away)),
    ("trunc", 4, lambda x, y, z, a, b, c: whole(a, int)),
    ("fract", 4, lambda x, y, z, a, b, c: fract(a)),
    ("mul_add", 4, lambda x, y, z, a, b, c: arithmetic(a * b + c, lambda: F(a) * F(b) + F(c))),
    ("abs", 4, lambda x, y, z, a, b, c: x & 0x7FFFFFFF),
    ("copysign", 4, lambda x, y, z, a, b, c: (x & 0x7FFFFFFF) | (y & 0x80000000)),
    ("to_i8", 1, lambda x, y, z, a, b, c: to_int(a, -(1 << 7), (1 << 7) - 1)),
    ("to_u8", 1, lambda x, y, z, a, b, c: to_int(a, 0, (1 << 8) - 1)),
    ("to_i16", 2, lambda x, y, z, a, b, c: to_int(a, -(1 << 15), (1 << 15) - 1)),
    ("to_u16", 2, lambda x, y, z, a, b, c: to_int(a, 0, (1 << 16) - 1)),
    ("to_i32", 4, lambda x, y, z, a, b, c: to_int(a, -(1 << 31), (1 << 31) - 1)),
    ("to_u32", 4, lambda x, y, z, a, b, c: to_int(a, 0, (1 << 32) - 1)),
    ("to_i64", 8, lambda x, y, z, a, b, c: to_int(a, -(1 << 63), (1 << 63) - 1)),
    ("to_u64", 8, lambda x, y, z, a, b, c: to_int(a, 0, (1 << 64) - 1)),
    ("from_i32", 4, lambda x, y, z, a, b, c: rounded(signed(x, 32))),
    ("from_u32", 4, lambda x, y, z, a, b, c: rounded(x)),
    ("from_i64", 4, lambda x, y, z, a, b, c: rounded(signed(x | (y << 32), 64))),
    ("from_u64", 4, lambda x, y, z, a, b, c: rounded(x | (y << 32))),
    ("to_f64", 8, lambda x, y, z, a, b, c: struct.unpack("<Q", struct.pack("<d", a))[0]),
    # The product of two f32 is exact as an f64, so this is one rounding of an f64 that no f32 holds.
    ("from_f64", 4, lambda x, y, z, a, b, c: arithmetic(a * b, lambda: F(a) * F(b))),
    ("is_finite", 1, lambda x, y, z, a, b, c: not math.isinf(a)),
    ("is_nan", 1, lambda x, y, z, a, b, c: math.isnan(a * b)),
]

# The edge values, each with both signs: zero, the smallest and the largest subnormal, the smallest normal, the
# halves `round` turns on, the last f32 below 0.5, the places where f32 stops holding halves (2^23) and odd numbers
# (2^24), the integer types' ends (2^7 to 2^64 and their neighbours), the largest f32 and infinity.
EDGES = [0x00000000, 0x00000001, 0x007FFFFF, 0x00800000, 0x3EFFFFFF, 0x3F000000, 0x3F800000, 0x3FC00000, 0x40200000,
         0x42FE0000, 0x43000000, 0x437F0000, 0x43800000, 0x46FFFE00, 0x47000000, 0x477FFF00, 0x47800000, 0x4AFFFFFF,
         0x4B000000, 0x4B7FFFFF, 0x4B800000, 0x4EFFFFFF, 0x4F000000, 0x4F7FFFFF, 0x4F800000, 0x5EFFFFFF, 0x5F000000,
         0x5F7FFFFF, 0x5F800000, 0x7F7FFFFF, 0x7F800000]
EDGES = [bits | sign for bits in EDGES for sign in (0, 0x80000000)]


def inputs():
    """The 10,000 cases: every ordered pair of the edge values, then seeded ones (splitmix64, so no library's)."""
    state = 0x2026_1124_0000_0001

    def draw():
        nonlocal state
        state = (state + 0x9E3779B97F4A7C15) & (2**64 - 1)
        z = state
        z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & (2**64 - 1)
        z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & (2**64 - 1)
        return z ^ (z >> 31)

    def any_f32():
        bits = draw() & 0xFFFFFFFF
        return bits if bits & 0x7FFFFFFF <= INF else bits & 0xFF7FFFFF  # a NaN pattern becomes a large number

    def game():  # what a match holds: up to a few thousand, in steps a f32 keeps, and sometimes a whole number
        r = draw()
        q = Fraction((r >> 8) % (1 << 26), 1 << (r % 4) * 4) * (-1 if r >> 63 else 1)
        return rounded(q, bool(r >> 63))

    cases = [(a, b, EDGES[(i * 7 + j * 3) % len(EDGES)]) for i, a in enumerate(EDGES) for j, b in enumerate(EDGES)]
    while len(cases) < CASES:
        pick = any_f32 if len(cases) % 2 else game
        cases.append((pick(), pick(), pick()))
    return cases[:CASES]


def fnv1a_64(data):
    h = 0xCBF29CE484222325
    for byte in data:
        h = ((h ^ byte) * 0x100000001B3) & (2**64 - 1)
    return h


def results(op, cases):
    name, width, reference = op
    return [int(reference(x, y, z, value(x), value(y), value(z))) & ((1 << 8 * width) - 1) for x, y, z in cases]


def self_check():
    """Answers known by hand: the reference is wrong if one of these is."""
    one, two, half = 0x3F800000, 0x40000000, 0x3F000000
    op = {name: (lambda x, y=one, z=one, r=reference: r(x, y, z, value(x), value(y), value(z))) for name, _, reference in OPS}
    assert rounded(Fraction(1, 3)) == 0x3EAAAAAB and rounded(Fraction(1, 10)) == 0x3DCCCCCD and rounded(16777217) == 0x4B800000
    assert rounded(16777219) == 0x4B800002 and rounded(Fraction(1, 2**150)) == 0 and rounded(Fraction(3, 2**151)) == 1
    assert rounded(2**128) == INF and rounded(Fraction(-1, 2**200)) == 0x80000000
    assert op["sqrt"](two) == 0x3FB504F3 and op["sqrt"](0x80000000) == 0x80000000 and op["sqrt"](0xBF800000) == NAN
    assert op["sqrt"](0x00000001) == 0x1A3504F3 and op["sqrt"](0x40800000) == two
    assert op["add"](0x3DCCCCCD, 0x3E4CCCCD) == 0x3E99999A and op["add"](one, 0xBF800000) == 0 and op["add"](INF, 0xFF800000) == NAN
    assert op["mul"](INF, 0) == NAN and op["mul"](0x80000000, one) == 0x80000000 and op["div"](one, 0x80000000) == 0xFF800000
    assert op["div"](one, 0x40400000) == 0x3EAAAAAB and op["mul_add"](0x7F7FFFFF, two, 0xFF7FFFFF) == 0x7F7FFFFF
    assert op["round"](0x40200000) == 0x40400000 and op["round"](0xBF000000) == 0xBF800000 and op["round"](0x3EFFFFFF) == 0
    assert op["floor"](0xBF000000) == 0xBF800000 and op["ceil"](0xBF000000) == 0x80000000 and op["trunc"](0xBFC00000) == 0xBF800000
    assert op["fract"](0xC0200000) == 0xBF000000 and op["fract"](0xC0000000) == 0 and op["fract"](INF) == NAN and op["fract"](0x80000000) == 0
    assert op["to_i8"](0x43000000) == 127 and op["to_u8"](0xBF800000) == 0 and op["to_i32"](0xFF800000) == -(1 << 31)
    assert op["from_i32"](0xFFFFFFFF) == 0xBF800000 and op["from_u32"](0xFFFFFFFF) == 0x4F800000
    assert op["total_cmp"](0x80000000, 0) == 0xFF and op["eq"](0x80000000, 0) and op["to_f64"](one) == 0x3FF0000000000000


def main():
    self_check()
    cases = inputs()
    assert len(cases) == CASES and all(bits & 0x7FFFFFFF <= INF for case in cases for bits in case)
    if "--full" in sys.argv[1:]:
        op = next(op for op in OPS if op[0] == sys.argv[sys.argv.index("--full") + 1])
        for index, (case, result) in enumerate(zip(cases, results(op, cases))):
            print(index, *(f"{bits:08x}" for bits in case), f"{result:0{2 * op[1]}x}")
        return 0
    data = b"".join(struct.pack("<3I", *case) for case in cases)
    lines = [f"# f32-inputs.bin: {len(data)} bytes, sha256 {hashlib.sha256(data).hexdigest()}",
             "# operation, bytes of one result, FNV-1a 64 of all results in order (made by make_f32_vectors.py)"]
    for op in OPS:
        out = results(op, cases)
        lines.append(f"{op[0]} {op[1]} {fnv1a_64(b''.join(result.to_bytes(op[1], 'little') for result in out)):016x}")
    files = {"f32-inputs.bin": data, "f32-expected.txt": ("\n".join(lines) + "\n").encode()}
    if "--check" in sys.argv[1:]:
        differing = [name for name, content in files.items() if not (HERE / name).is_file() or (HERE / name).read_bytes() != content]
        print("differs or is missing: " + ", ".join(differing) if differing else f"both files are byte for byte what this script makes ({len(OPS)} operations, {CASES} cases)")
        return 1 if differing else 0
    for name, content in files.items():
        (HERE / name).write_bytes(content)
    print(f"{len(OPS)} operations, {CASES} cases, {len(EDGES)} edge values; f32-inputs.bin {len(files['f32-inputs.bin'])} bytes")
    return 0


if __name__ == "__main__":
    sys.exit(main())
