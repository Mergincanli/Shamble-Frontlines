# rust-reviewer's scratch for block 7: checks rr-trig-proto's dump (route A: quarter turns out, libm sincos in f64,
# narrowed; route C: libm sincosf on the f32 angle) against the correctly rounded f32 of sin and cos of the degree
# times pi / 180, worked in Python decimal at 60 digits. Usage: python3 -I rr-trig-ref.py <dump file>
import struct, sys
from decimal import Decimal, getcontext

getcontext().prec = 60
PI = Decimal("3.14159265358979323846264338327950288419716939937510582097494459230781640628620899")

def sin_cos(x):
    # Taylor series, x in [0, pi/2)
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
    # correctly rounded (ties to even) f32 of a positive Decimal
    b = f32_bits(float(d))
    best = None
    for c in (b - 1, b, b + 1):
        if c < 0:
            continue
        e = abs(Decimal(f32_of_bits(c)) - d)
        if best is None or e < best[0] or (e == best[0] and c % 2 == 0):
            best = (e, c)
    return best[1]

off_a = off_c = 0
worst_c = 0
rows = 0
first_a = []
for line in open(sys.argv[1]):
    deg_s, ax, az, cx, cz = line.split()
    deg = Decimal(float(deg_s))  # the exact double the program used
    s, c = sin_cos(deg * PI / 180)
    rs, rc = nearest_f32(s) if s > 0 else 0, nearest_f32(c)
    ax, az, cx, cz = (int(v, 16) for v in (ax, az, cx, cz))
    rows += 1
    if (ax, az) != (rs, rc):
        off_a += 1
        if len(first_a) < 5:
            first_a.append((deg_s, hex(ax), hex(rs), hex(az), hex(rc)))
    if (cx, cz) != (rs, rc):
        off_c += 1
        worst_c = max(worst_c, abs(cx - rs), abs(cz - rc))
print(f"rows {rows}; route A (sincos f64, narrowed) off the correctly rounded f32: {off_a}; route C (sincosf): {off_c}, worst {worst_c} ulp")
for r in first_a:
    print("route A off at", r)
