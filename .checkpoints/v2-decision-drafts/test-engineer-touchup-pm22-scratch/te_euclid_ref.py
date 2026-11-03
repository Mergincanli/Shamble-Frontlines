# test-engineer touch-up PM-0004-22: own exact recomputation of the D3A-1 cases (no fetch).
# Exact Euclidean quotient and remainder by integer arithmetic on the inputs scaled by 2^23 (f32) or 2^52 (f64),
# cross-checked with Fractions; plus an IEEE emulation of rustc 1.97.0's steps (skeptic-6's disassembly, de32.disasm):
# q = trunc(a / b); if fmod(a, b) < 0 then q -= 1 (b > 0) or q += 1 (b < 0).
import math, struct
from fractions import Fraction as F
import numpy as np

def euclid_int(x, y):
    # Euclidean: x = q*y + r, 0 <= r < |y|
    r = x % abs(y)
    q = (x - r) // y
    assert q * y + r == x and 0 <= r < abs(y)
    return q, r

cases = {
    'f32': (np.float32, 23, 0x40400001, 0x3f800001, '<I', '<f'),
    'f64': (np.float64, 52, 0x4008000000000001, 0x3ff0000000000001, '<Q', '<d'),
}
for name, (T, k, abits, bbits, ib, fb) in cases.items():
    a = T(struct.unpack(fb, struct.pack(ib, abits))[0])
    b = T(struct.unpack(fb, struct.pack(ib, bbits))[0])
    A, B = 3 * 2**k + 2, 2**k + 1
    assert F(float(a)) == F(A, 2**k) and F(float(b)) == F(B, 2**k), name
    for sa, sb in [(1, 1), (-1, 1), (1, -1), (-1, -1)]:
        x, y = T(sa) * a, T(sb) * b
        q, r = euclid_int(sa * A, sb * B)
        # Fraction cross-check
        fx, fy = F(float(x)), F(float(y))
        qf = math.floor(fx / fy) if fy > 0 else -math.floor(fx / -fy)
        rf = fx - qf * fy
        assert qf == q and rf == F(r, 2**k) and 0 <= rf < abs(fy)
        # emulate rustc 1.97.0's steps in IEEE arithmetic of the type
        quo = np.trunc(x / y)
        fm = np.fmod(x, y)
        if fm < 0:
            quo = quo + (T(-1) if y > 0 else T(1))
        rem = fm + abs(y) if fm < 0 else fm
        print(f"{name} a={'-' if sa<0 else '+'}a b={'-' if sb<0 else '+'}b x/y={float(x/y)!r} "
              f"exact_q={q} exact_r={r}*2^-{k} (= {float(F(r,2**k))!r}) "
              f"emul197_q={float(quo)!r} emul197_r={float(rem)!r} q_ok={float(quo)==q} r_ok={float(rem)==float(F(r,2**k))}")

# Why the reference must be "rounded once to the type": an exact Euclidean remainder need not be representable.
x, y = np.float32(-2.0**-30), np.float32(1.0)
fm = np.fmod(x, y); rem = fm + abs(y) if fm < 0 else fm
exact = F(float(x)) - math.floor(F(float(x)) / F(float(y))) * F(float(y))
print(f"f32 rem_euclid-like(-2^-30, 1): emul={float(rem)!r} exact={exact} (= 1 - 2^-30) "
      f"exact_rounded_to_f32={float(np.float32(float(exact)))!r} equals_exact={F(float(rem))==exact} "
      f"equals_rounded={float(rem)==float(np.float32(float(exact)))}")
