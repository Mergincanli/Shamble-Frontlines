# Exact Euclidean division reference for euclid_test.raw (Fractions; round-to-nearest-even to the format)
import sys, struct
from fractions import Fraction as F
from math import floor
def f32(h): return struct.unpack('>f', bytes.fromhex(h))[0]
def f64(h): return struct.unpack('>d', bytes.fromhex(h))[0]
def round_to(x, kind):  # correctly round an exact Fraction to f32/f64 (ties to even)
    if kind == 64:
        return float(x)  # Fraction.__float__ is correctly rounded
    d = float(x)  # f64 nearest, then exact rounding to f32 via neighbours
    c = struct.unpack('>f', struct.pack('>f', d))[0]
    # find the f32 neighbours of x exactly
    bits = struct.unpack('>I', struct.pack('>f', c))[0]
    cands = []
    for k in (-2, -1, 0, 1, 2):
        bb = (bits + k) & 0xffffffff
        v = struct.unpack('>f', struct.pack('>I', bb))[0]
        if v == v and abs(v) != float('inf'): cands.append((abs(F(v) - x), bb & 1, v))
    cands.sort(); return cands[0][2]
stats = {}
ex = {}
for line in open(sys.argv[1]):
    p = line.split()
    if p[0] not in ('S32', 'S64', 'MIRROR'): continue
    if p[0] == 'MIRROR': p = ['S32'] + p[2:]
    kind = 32 if p[0] == 'S32' else 64
    cv = f32 if kind == 32 else f64
    a, b, q, r = (cv(h) for h in p[1:5])
    A, B = F(a), F(b)
    n = floor(A / abs(B)) * (1 if B > 0 else -1)   # Euclidean: a = n*b + R, 0 <= R < |b|
    R = A - n * B
    assert 0 <= R < abs(B)
    nq, nr = round_to(F(n), kind), round_to(R, kind)
    s = stats.setdefault(p[0] if p[0] != 'MIRROR' else 'S32', [0, 0, 0])
    s[0] += 1
    if q != nq:
        s[1] += 1
        ex.setdefault((kind, 'q'), []).append((p[1], p[2], q, nq))
    if r != nr:
        s[2] += 1
        ex.setdefault((kind, 'r'), []).append((p[1], p[2], r, nr))
for k, (t, bq, br) in sorted(stats.items()):
    print(f"{k}: {t} pairs; div_euclid != exact: {bq} ({100*bq/t:.3f}%); rem_euclid != correctly rounded exact remainder: {br}")
for k, v in sorted(ex.items()):
    print(k, 'first 5:', v[:5])
