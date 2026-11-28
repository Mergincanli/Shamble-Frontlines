# C3's reference for rules.rs's share test (rust-reviewer-read.md 11.7 row 2): for each rate the test names, the
# correctly rounded f32 (ties to even) of 1 - exp(-x), where x is the double the simulation works out (the rate, then
# times the tick's length 1 / 30 as a double; the easing's rate is LN_10 over its time, LN_10 being the double
# core::f64::consts::LN_10). Python decimal at 60 digits; x as an exact double. Not part of the product: notes.
# Usage: python3 -I -B .checkpoints/cs4c-soldier/share-ref-c3.py
import struct
from decimal import Decimal, getcontext

getcontext().prec = 60
LN_10 = struct.unpack("<d", struct.pack("<Q", 0x40026BB1BBB55516))[0]
DT = 1.0 / 30.0


def f32_bits(v):
    return struct.unpack("<I", struct.pack("<f", v))[0]


def f32_of_bits(b):
    return struct.unpack("<f", struct.pack("<I", b))[0]


def nearest_f32(d):
    if d == 0:
        return 0
    b = f32_bits(float(d))
    best = None
    for c in (b - 1, b, b + 1):
        e = abs(Decimal(f32_of_bits(c)) - d)
        if best is None or e < best[0] or (e == best[0] and c % 2 == 0):
            best = (e, c)
    return best[1]


def share(rate):
    x = rate * DT  # the double the simulation multiplies out
    return nearest_f32(1 - (-Decimal(x)).exp())


assert LN_10 == 2.302585092994046
print("easing: time to a tenth -> rate LN_10 / time -> bits")
for time in (0.01, 0.05, 0.1, 0.25, 0.5, 1.0, 2.0, 5.0):
    print(f"  {time!r}: {share(LN_10 / time):08x}")
print("push: strength -> bits")
for strength in (0.5, 1.0, 10.0, 50.0, 100.0):
    print(f"  {strength!r}: {share(strength):08x}")
