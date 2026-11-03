# fixcheck-d4: 1.99.1's core::f32::math::div_euclid steps (trunc(x / rhs); -1/+1 only when x % rhs < 0), in exact f32 arithmetic.
from fractions import Fraction as F
import math, struct
def f32(v):  # round a Fraction to the nearest f32 (ties to even), via exact search
    x = float(v)  # nearest f64 (one rounding), then to f32 by struct; double rounding cannot bite here, checked below
    y = struct.unpack('f', struct.pack('f', x))[0]
    return F(y)
a = F(3) + F(1, 2**22); b = F(1) + F(1, 2**23)
assert f32(a) == a and f32(b) == b
exact = a / b
print('exact a/b - 3 =', float(exact - 3), '; midpoint 3 - 2^-23 =', float(-F(1, 2**23)), '; above midpoint:', exact > 3 - F(1, 2**23))
q = f32(exact); print('f32(a/b) =', float(q))
t = F(math.trunc(q))
n = math.trunc(exact); r = a - n * b  # fmod is exact
print('trunc =', t, ' a % b =', float(r))
print('div_euclid =', t - 1 if r < 0 else t, ' rem_euclid =', float(r + abs(b) if r < 0 else r), ' correct quotient =', math.floor(exact))
