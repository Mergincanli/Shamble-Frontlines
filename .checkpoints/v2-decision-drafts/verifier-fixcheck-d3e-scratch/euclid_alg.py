# 1.99.1's core::f64::math::div_euclid / rem_euclid steps, in IEEE double arithmetic (Python floats), on the f64 case.
import math
from fractions import Fraction
def div_euclid(x, rhs):
    q = float(math.trunc(x / rhs))
    if math.fmod(x, rhs) < 0.0:
        return q - 1.0 if rhs > 0.0 else q + 1.0
    return q
def rem_euclid(x, rhs):
    r = math.fmod(x, rhs)
    return r + abs(rhs) if r < 0.0 else r
a = 3 + 2**-51; b = 1 + 2**-52
print('2b+1 == a exactly:', Fraction(2)*Fraction(b)+1 == Fraction(a))
print('a/b =', repr(a / b), ' div_euclid =', div_euclid(a, b), ' rem_euclid =', rem_euclid(a, b))
for (x, y) in [(a, b), (-a, b), (a, -b), (-a, -b)]:
    print((x > 0, y > 0), 'div_euclid', div_euclid(x, y), 'rem_euclid', rem_euclid(x, y))
