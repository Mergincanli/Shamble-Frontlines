# rust-reviewer block 17 scratch: libm's exp and sincos (native dump) against correctly rounded references (Python
# decimal, 60 digits), and the shares 1 - exp(x) narrowed to f32 against the correctly rounded f32. Usage: ref.py <dir>
import struct, sys, math
from decimal import Decimal, getcontext
getcontext().prec = 60
D = sys.argv[1]
def f64s(name):
    b = open(f"{D}/{name}", "rb").read()
    return list(struct.unpack(f"<{len(b)//8}d", b))
def bits(x): return struct.unpack("<Q", struct.pack("<d", x))[0]
def f32b(x): return struct.unpack("<I", struct.pack("<f", x))[0]
def f32v(b): return struct.unpack("<f", struct.pack("<I", b))[0]
def nearest_f32(dv):
    c0 = f32b(float(dv))
    best = None
    for c in (c0 - 1, c0, c0 + 1):
        if c < 0: continue
        e = abs(Decimal(f32v(c)) - dv)
        if best is None or e < best[0] or (e == best[0] and c % 2 == 0): best = (e, c)
    return best[1]
def ulps64(got, exact):
    if exact == 0: return 0.0 if got == 0 else float("inf")
    u = math.ulp(float(exact))
    return float(abs(Decimal(got) - exact) / Decimal(u))
def ulps32(gotbits, exact):
    if exact == 0: return 0.0
    r = f32v(nearest_f32(exact))
    u = Decimal(f32v(f32b(r) + 1)) - Decimal(r)
    return float(abs(Decimal(f32v(gotbits)) - exact) / u)

ex_in, ex_out = f64s("exp-in.bin"), f64s("exp-native.bin")
off = 0; worst = (0.0, None)
for x, y in zip(ex_in, ex_out):
    exact = Decimal(0) if x == float("-inf") else Decimal(x).exp()
    cr = float(exact)
    if bits(cr) != bits(y): off += 1
    u = ulps64(y, exact)
    if u > worst[0]: worst = (u, x)
print(f"exp: {len(ex_in)} inputs, {off} off the correctly rounded f64, largest error {worst[0]:.4f} ulp at x = {worst[1]!r}")

# The shares: the first 2 + 5000 + 10001 + 4297 + 4300 inputs are share arguments.
N = 2 + 5000 + 10001 + 4297 + 4300
DT = 1.0 / 30.0
off32 = 0; worst32 = (0.0, None); over1 = []
for i, (x, y) in enumerate(zip(ex_in[:N], ex_out[:N])):
    got = f32b(1.0 - y)
    exact = Decimal(1) - (Decimal(0) if x == float("-inf") else Decimal(x).exp())
    if exact == 0:
        if got != 0: over1.append((i, x, "nonzero for zero"))
        continue
    cr = nearest_f32(exact)
    if got != cr: off32 += 1
    u = ulps32(got, exact)
    if u > worst32[0]: worst32 = (u, x)
    if u >= 1.0: over1.append((i, x, u))
EDGE = max((abs(c[1]) for c in over1), default=0.0)
above = [(x, y) for x, y in zip(ex_in[:N], ex_out[:N]) if abs(x) > EDGE]
oa = 0; wa = 0.0
for x, y in above:
    exact = Decimal(1) - (Decimal(0) if x == float("-inf") else Decimal(x).exp())
    got = f32b(1.0 - y)
    if got != nearest_f32(exact): oa += 1
    wa = max(wa, ulps32(got, exact))
print(f"shares with |x| above {EDGE!r}: {len(above)} cases, {oa} off the correctly rounded f32, largest {wa:.4f} ulp")
for lo, hi, name in ((2, 5002, "easing T = k/1000"), (5002, 15003, "push s = k/100"), (15003, 19300, "push 2^(-e/4)"), (19300, N, "easing T from 5 down")):
    o = 0; w = 0.0
    for x, y in zip(ex_in[lo:hi], ex_out[lo:hi]):
        exact = Decimal(1) - (Decimal(0) if x == float("-inf") else Decimal(x).exp())
        got = f32b(1.0 - y)
        if exact == 0: continue
        if got != nearest_f32(exact): o += 1
        w = max(w, ulps32(got, exact))
    print(f"  {name}: {hi - lo} cases, {o} off, largest {w:.4f} ulp")
print(f"shares: {N} cases, {off32} off the correctly rounded f32, largest {worst32[0]:.4f} ulp at x = {worst32[1]!r}")
print(f"shares at or over 1 ulp: {len(over1)}; the largest |x| among them: {max((abs(c[1]) for c in over1), default=None)!r}")
print(f"  as a push strength (|x| x 30): {max((abs(c[1]) * 30 for c in over1), default=None)!r}")
# his values
for name, x, y in (("ease", ex_in[0], ex_out[0]), ("push", ex_in[1], ex_out[1])):
    exact = Decimal(1) - Decimal(x).exp()
    print(f"  his {name}: x = {x!r}, share bits {f32b(1.0 - y):08x}, reference {nearest_f32(exact):08x}, {ulps32(f32b(1.0 - y), exact):.4f} ulp")

PI = Decimal("3.14159265358979323846264338327950288419716939937510582097494459230781640628620899")
def sin_cos(x):
    x2 = x * x
    s, t, n = Decimal(0), x, 1
    while t != 0 and (s == 0 or abs(t) > abs(s) * Decimal("1e-62")):
        s += t; t = -t * x2 / ((n + 1) * (n + 2)); n += 2
    c, t, n = Decimal(0), Decimal(1), 0
    while t != 0 and abs(t) > Decimal("1e-62"):
        c += t; t = -t * x2 / ((n + 1) * (n + 2)); n += 2
    return s, c
sc_in, sc_out = f64s("sincos-in.bin"), f64s("sincos-native.bin")
offs = offc = 0; ws = (0.0, None); wc = (0.0, None); off32s = 0
for i, x in enumerate(sc_in):
    s, c = sin_cos(Decimal(x))
    gs, gc = sc_out[2 * i], sc_out[2 * i + 1]
    if bits(float(s)) != bits(gs) and not (x == 0.0): offs += 1
    if bits(float(c)) != bits(gc): offc += 1
    us, uc = ulps64(gs, s), ulps64(gc, c)
    if us > ws[0]: ws = (us, x)
    if uc > wc[0]: wc = (uc, x)
    if s != 0 and (f32b(gs) != nearest_f32(abs(s)) and gs > 0): off32s += 1
    if f32b(gc) != nearest_f32(c): off32s += 1
print(f"sincos: {len(sc_in)} inputs; sin {offs} off the correctly rounded f64 (largest {ws[0]:.4f} ulp at {ws[1]!r}), cos {offc} off (largest {wc[0]:.4f} ulp at {wc[1]!r})")
print(f"sincos narrowed to f32: {off32s} of {2 * len(sc_in)} off the correctly rounded f32 of the double's sine and cosine")
