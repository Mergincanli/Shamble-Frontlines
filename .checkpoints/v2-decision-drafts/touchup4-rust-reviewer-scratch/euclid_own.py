# Own check: rustc 1.99.1's div_euclid/rem_euclid steps (core/num/f32.rs 2048-2054, 2083-2086; f64.rs 2028-2034,
# as saved by verifier fixcheck-d3e) in IEEE f32/f64 via numpy, against the exact Euclidean result (fractions).
import numpy as np
from fractions import Fraction as F
import math
def div_euclid(x, rhs, t):
    q = np.trunc(t(x) / t(rhs))
    if np.fmod(t(x), t(rhs)) < 0: return q - t(1) if rhs > 0 else q + t(1)
    return q
def rem_euclid(x, rhs, t):
    r = np.fmod(t(x), t(rhs))
    return r + abs(t(rhs)) if r < 0 else r
def exact(x, y):
    X, Y = F(float(x)), F(float(y)); q = math.floor(X / abs(Y)) * (1 if Y > 0 else -1); return q, X - q * Y
for t, a, b in ((np.float32, 3 + 2**-22, 1 + 2**-23), (np.float64, 3 + 2**-51, 1 + 2**-52)):
    a, b = t(a), t(b)
    for x, y in ((a, b), (-a, b), (a, -b), (-a, -b)):
        q, r = exact(x, y)
        print(t.__name__, float(x), float(y), 'steps q', float(div_euclid(x, y, t)), 'r', float(rem_euclid(x, y, t)),
              '| exact q', q, 'r', r, '-> rounded', float(t(float(r))))
x = np.float32(-2**-30)
q, r = exact(x, np.float32(1))
print('f32 rem_euclid(-2^-30, 1):', float(rem_euclid(x, 1.0, np.float32)), '| exact r', r, '=', float(r), 'rounded to f32', float(np.float32(float(r))))
