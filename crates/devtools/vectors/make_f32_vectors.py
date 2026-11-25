#!/usr/bin/env python3
"""Makes the float vectors (PM-0004-22): f32-inputs.bin and f32-expected.txt beside this script.

For every `f32` operation ADR 0004 part 7 allows, fixed inputs and the results an exact reference gives: each
result is worked out in rational arithmetic and rounded once to f32 (nearest, ties to even), independent of any
float unit's own rounding. So `sqrt` "equals a correctly rounded reference" by construction, and the native build
and the WebAssembly build must each equal the reference, hence each other. `f32` only: `f64` arithmetic, casts
between `f64` and the integers, and casts to `usize` and `isize` (as wide as the target) are not in these vectors.

  python3 -I -B crates/devtools/vectors/make_f32_vectors.py                 writes the two files
  python3 -I -B crates/devtools/vectors/make_f32_vectors.py --check         compares them byte for byte, writes nothing
  python3 -I -B crates/devtools/vectors/make_f32_vectors.py --full <op>     prints every result of one operation

Inputs: 10,072 cases of three f32 bit patterns (a, b, c), little-endian: every ordered pair of the edge values
first, then seeded values up to case 9,999, then 48 constructed cases for `mul_add` and 24 for the cast from f64. No
input is a NaN (part 7: a NaN never enters the simulation); a NaN result is written as 0x7FC00000, because only which
cases give a NaN must agree, not a NaN's bits. Expected: one line per operation with the bytes of one result and FNV-1a 64 over all its results in order,
each little-endian; then the `pin` lines.

The constructed cases are `mul_add`'s hard ones: c's last bit is 1 and a * b is half a unit of c's last place, less a
hair. The exact sum rounds back to c. A sum worked out in f64 first loses the hair, lands on the half and rounds
away from c. None of the 10,000 other cases is of this kind (checked below), so without these a `mul_add` made of
two roundings passes.

The last 24 cases are the cast from f64 to f32 where it is hardest (the follow-up 3A-FROM-F64): an f64 exactly
half-way between two f32, which goes to the one whose last bit is 0, at one and below the smallest f32; f64 from
the largest f32 up to twice 2^127, where the cast turns to infinity exactly at the half-way point; and, as the last
four, the first f64 above half the smallest f32 and the half-way point below the smallest normal f32. They come after
every other case, so no earlier case changes its number.

The `pin` lines (MUL-ADD): the cases where `f32::mul_add` in WebAssembly does NOT give the exact result, each with the
value measured there: Rust 1.99.1, wasm32-unknown-unknown, release and dev, on 2026-11-24. The toolchain's software
`fmaf` corrects a sum that lands on a half only where the result is a normal f32; below the smallest normal f32 it
returns the two-roundings value. Natively, on a processor with FMA, every case is exact. So `mul_add` is not the
same on every target, the simulation's scan refuses it meanwhile (crates/sim/tests/rules_scan.rs), and the pins are
a known defect of the toolchain, kept so that any change shows; they are evidence of nothing else.
Python's standard library only. crates/devtools/src/rules/f32_ops.rs runs the same operations in the same order.
"""
import hashlib
import math
import pathlib
import struct
import sys
from fractions import Fraction

HERE = pathlib.Path(__file__).resolve().parent
SEEDED = 10_000  # the edge values' pairs and the seeded cases
FUSED = 10_048  # those, then `mul_add`'s constructed cases
CASES = 10_072  # those, then the cast from f64's constructed cases
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


def narrowed(bits):
    """An f64 bit pattern rounded once to f32. Two f32 patterns side by side never make a NaN or an infinite f64."""
    double = struct.unpack("<d", struct.pack("<Q", bits))[0]
    assert math.isfinite(double)
    return rounded(Fraction(double), bits >> 63 == 1)


def through_f64(x, y, z):
    """a * b + c with two roundings: to f64 (where the product of two f32 is exact), then to f32."""
    double = value(x) * value(y) + value(z)
    return arithmetic(double, lambda: Fraction(double))


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
    # Two inputs' bits side by side as one f64: all 53 bits of it, and far outside f32's range on both sides.
    ("from_f64", 4, lambda x, y, z, a, b, c: narrowed(x | (y << 32))),
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


def constructed():
    """`mul_add`'s hard cases (the docstring above), each with the four placements of its signs."""
    sign = 0x80000000

    def scaled(bits, by):  # times 2^by
        exponent = (bits >> 23 & 0xFF) + by
        assert 1 <= exponent <= 254
        return bits & 0x807FFFFF | exponent << 23

    def signs(a, b, c):
        return [(a, b, c), (a ^ sign, b, c), (a ^ sign, b, c ^ sign), (a, b, c ^ sign)]

    cases = []
    for by in (0, -100, -20, 20, 100):  # around one, and the same numbers times a power of two
        cases += signs(scaled(0x33800001, by), 0x3F7FFFFE, scaled(0x3F800001, by))
    cases += signs(0x71800001, 0x40FFFFFE, 0x7F7FFFFF)  # at the largest f32, where one side of the half is infinity
    # Results below the smallest normal f32 (four values of c), then the first normal ones (two).
    for c in (0x007FFFFF, 0x00400001, 0x00012345, 0x00000041, 0x00800001, 0x00FFFFFF):
        cases += signs(0x01000001, 0x32FFFFFE, c)
    return cases


def narrowing():
    """The cast from f64's hard cases: each f64's low and high half as a and b, c zero, then the same f64 negated."""
    doubles = [
        (0x3FF0000010000000, 0x3F800000),  # 1 + 2^-24, half-way: to the even 1
        (0x3FF0000030000000, 0x3F800002),  # 1 + 3 * 2^-24, half-way: to the even 1 + 2^-22
        (0x3690000000000000, 0x00000000),  # 2^-150, half the smallest f32: to the even 0
        (0x36A8000000000000, 0x00000002),  # 3 * 2^-150, half-way: to the even second-smallest f32
        (0x47EFFFFFE0000000, 0x7F7FFFFF),  # the largest f32 itself
        (0x47EFFFFFE8000000, 0x7F7FFFFF),  # a quarter of a step above it: still the largest f32
        (0x47EFFFFFEFFFFFFF, 0x7F7FFFFF),  # the last f64 below the half-way point: still the largest f32
        (0x47EFFFFFF0000000, INF),  # half-way to 2^128, which no f32 holds: to the even side, infinity
        (0x47EFFFFFF0000001, INF),  # the first f64 above the half-way point
        (0x47F0000000000000, INF),  # 2^128
        (0x3690000000000001, 0x00000001),  # the first f64 above 2^-150: up to the smallest f32
        (0x380FFFFFE0000000, 0x00800000),  # half-way between the largest subnormal and the smallest normal: even
    ]
    cases, expected = [], []
    for double, result in doubles:
        for sign in (0, 1):
            cases.append((double & 0xFFFFFFFF, double >> 32 | sign << 31, 0))
            expected.append(result | sign << 31)
    return cases, expected


# Measured, never worked out: what `f32::mul_add` gives in WebAssembly on the pinned toolchain where that is not the
# exact result (the docstring's `pin` lines). Every one is a case whose result lies below the smallest normal f32.
WASM_MUL_ADD = {
    (0x01000001, 0x32FFFFFE, 0x007FFFFF): 0x00800000, (0x81000001, 0x32FFFFFE, 0x007FFFFF): 0x007FFFFE,
    (0x81000001, 0x32FFFFFE, 0x807FFFFF): 0x80800000, (0x01000001, 0x32FFFFFE, 0x807FFFFF): 0x807FFFFE,
    (0x01000001, 0x32FFFFFE, 0x00400001): 0x00400002, (0x81000001, 0x32FFFFFE, 0x00400001): 0x00400000,
    (0x81000001, 0x32FFFFFE, 0x80400001): 0x80400002, (0x01000001, 0x32FFFFFE, 0x80400001): 0x80400000,
    (0x01000001, 0x32FFFFFE, 0x00012345): 0x00012346, (0x81000001, 0x32FFFFFE, 0x00012345): 0x00012344,
    (0x81000001, 0x32FFFFFE, 0x80012345): 0x80012346, (0x01000001, 0x32FFFFFE, 0x80012345): 0x80012344,
    (0x01000001, 0x32FFFFFE, 0x00000041): 0x00000042, (0x81000001, 0x32FFFFFE, 0x00000041): 0x00000040,
    (0x81000001, 0x32FFFFFE, 0x80000041): 0x80000042, (0x01000001, 0x32FFFFFE, 0x80000041): 0x80000040,
}


def inputs():
    """The cases: every ordered pair of the edge values, seeded ones (splitmix64, so no library's), the constructed."""
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
    while len(cases) < SEEDED:
        pick = any_f32 if len(cases) % 2 else game
        cases.append((pick(), pick(), pick()))
    return cases[:SEEDED] + constructed() + narrowing()[0]


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
    # An f64 that no f32 holds: 1 + 2^-24 is a half (to the even 1.0), a hair more goes up; too large; too small.
    assert op["from_f64"](0x10000000, 0x3FF00000) == one and op["from_f64"](0x10000001, 0x3FF00000) == 0x3F800001
    assert op["from_f64"](0, 0x7F800000) == INF and op["from_f64"](0, 0x80100000) == 0x80000000
    # Every constructed case rounds back to its c, and two roundings give something else; no other case does.
    hard = constructed()
    assert len(hard) == len(set(hard)) == FUSED - SEEDED
    assert all(op["mul_add"](*case) == case[2] != through_f64(*case) for case in hard)
    # Each pin is a constructed case below the smallest normal f32, and what was measured is the two-roundings value.
    assert set(WASM_MUL_ADD) <= set(hard) and all(case[2] & 0x7F800000 == 0 for case in WASM_MUL_ADD)
    assert all(measured == through_f64(*case) != case[2] for case, measured in WASM_MUL_ADD.items())
    # Each of the cast's constructed cases gives the result written beside it, worked out by hand.
    casts, expected = narrowing()
    assert len(casts) == len(set(casts)) == CASES - FUSED
    assert [op["from_f64"](*case) for case in casts] == expected


def main():
    self_check()
    cases = inputs()
    assert len(cases) == CASES and all(bits & 0x7FFFFFFF <= INF for case in cases for bits in case)
    fused = next(reference for name, _, reference in OPS if name == "mul_add")
    plain = [case for case in cases[:SEEDED] if fused(*case, *map(value, case)) != through_f64(*case)]
    assert not plain, f"{len(plain)} of the first {SEEDED} cases already tell two roundings from one"
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
    lines += ["# MUL-ADD, a known defect of the toolchain: where mul_add in WebAssembly is not the exact result.",
              "# pin, operation, case, a, b, c, the exact result, the result measured in WebAssembly"]
    lines += [f"pin mul_add {index} {case[0]:08x} {case[1]:08x} {case[2]:08x} {fused(*case, *map(value, case)):08x} {WASM_MUL_ADD[case]:08x}"
              for index, case in enumerate(cases) if case in WASM_MUL_ADD]
    files = {"f32-inputs.bin": data, "f32-expected.txt": ("\n".join(lines) + "\n").encode()}
    if "--check" in sys.argv[1:]:
        differing = [name for name, content in files.items() if not (HERE / name).is_file() or (HERE / name).read_bytes() != content]
        print("differs or is missing: " + ", ".join(differing) if differing else f"both files are byte for byte what this script makes ({len(OPS)} operations, {CASES} cases)")
        return 1 if differing else 0
    for name, content in files.items():
        (HERE / name).write_bytes(content)
    print(f"{len(OPS)} operations, {CASES} cases ({CASES - SEEDED} constructed, {len(WASM_MUL_ADD)} pinned), {len(EDGES)} edge values; f32-inputs.bin {len(files['f32-inputs.bin'])} bytes")
    return 0


if __name__ == "__main__":
    sys.exit(main())
