# C3's reference for facing_of's test (rust-reviewer-read.md 11.7 row 3): the correctly rounded f32 of sin and cos
# of the degree times pi / 180, in Python decimal at 60 digits (the method of rust-reviewer's rr-trig-ref.py, B7-2),
# and beside it the same of the route the simulation takes (whole quarter turns out, the rest times the f64 constant
# rounded to a double, then the true sin and cos of that double), to see where the constant's own rounding shows.
# Not part of the product: notes. Usage: python3 -I -B .checkpoints/cs4c-soldier/facing-ref-c3.py
import struct
from decimal import Decimal, getcontext

getcontext().prec = 60
PI = Decimal("3.14159265358979323846264338327950288419716939937510582097494459230781640628620899")
K = struct.unpack("<d", bytes.fromhex("399d52a246df913f"))[0]  # bits 3f91df46a2529d39, little-endian


def sin_cos(x):
    x2 = x * x
    s, t, n = Decimal(0), x, 1
    while abs(t) > Decimal("1e-58"):
        s += t
        t = -t * x2 / ((n + 1) * (n + 2))
        n += 2
    c, t, n = Decimal(0), Decimal(1), 0
    while abs(t) > Decimal("1e-58"):
        c += t
        t = -t * x2 / ((n + 1) * (n + 2))
        n += 2
    return s, c


def f32_bits(v):
    return struct.unpack("<I", struct.pack("<f", v))[0]


def f32_of_bits(b):
    return struct.unpack("<f", struct.pack("<I", b))[0]


def nearest_f32(d):
    """The correctly rounded f32 (ties to even) of a Decimal, as bits; zero as +0."""
    if d == 0:
        return 0
    sign = 0x80000000 if d < 0 else 0
    a = abs(d)
    b = f32_bits(float(a))
    best = None
    for c in (b - 1, b, b + 1):
        if c < 0:
            continue
        e = abs(Decimal(f32_of_bits(c)) - a)
        if best is None or e < best[0] or (e == best[0] and c % 2 == 0):
            best = (e, c)
    return sign | best[1]


def double_product(a, b):
    """a * b rounded to a double (Python's float product is IEEE round to nearest)."""
    return a * b


def full(deg):
    s, c = sin_cos(Decimal(deg) * PI / 180)
    return nearest_f32(s), nearest_f32(c)


def route(deg):
    quarter = 0 if deg < 90 else 1 if deg < 180 else 2 if deg < 270 else 3
    rest = (deg - 90.0 * quarter) + 0.0
    s, c = sin_cos(Decimal(double_product(rest, K)))
    s, c = nearest_f32(s), nearest_f32(c)
    neg = lambda b: b ^ 0x80000000 if b else 0
    return [(s, c), (c, neg(s)), (neg(s), neg(c)), (neg(c), s)][quarter]


below_360 = struct.unpack("<d", struct.pack("<Q", struct.unpack("<Q", struct.pack("<d", 360.0))[0] - 1))[0]
for deg in (45.0, 120.0, 240.0, below_360):
    a, b = full(deg), route(deg)
    print(f"{deg!r}: true x {a[0]:08x} z {a[1]:08x} | route x {b[0]:08x} z {b[1]:08x} | x as f32 {f32_of_bits(a[0])!r} vs {f32_of_bits(b[0])!r}")
