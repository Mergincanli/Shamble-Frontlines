# rust-reviewer block 17 scratch: the exp and sincos inputs as share_per_tick and facing_of form them (f64 LE).
import struct, random, math, sys
OUT = sys.argv[1]
LN_10 = float.fromhex("0x1.26bb1bbb55516p+1")
assert LN_10 == math.log(10)
DT = 1.0 / 30.0
K = math.pi / 180.0
assert struct.unpack("<Q", struct.pack("<d", K))[0] == 0x3f91df46a2529d39
def d(bits): return struct.unpack("<d", struct.pack("<Q", bits))[0]
def b(x): return struct.unpack("<Q", struct.pack("<d", x))[0]
def nb(x): return d(b(x) - 1)  # positive x only
def na(x): return d(b(x) + 1)
rng = random.Random(0x4C2C3)
ex = []
ex += [-((LN_10 / 0.164) * DT), -(8.0 * DT)]                         # his values
ex += [-((LN_10 / (k / 1000)) * DT) for k in range(1, 5001)]         # easing T = k/1000 in (0, 5]
ex += [-((k / 100) * DT) for k in range(0, 10001)]                   # push s = k/100 in [0, 100]
ex += [-((2.0 ** (-e / 4)) * DT) for e in range(0, 4297)]            # push toward zero, to 2^-1074
ex += [-((LN_10 / (5.0 * 2.0 ** (-e / 4))) * DT) for e in range(0, 4300)]  # easing T from 5 down
ex += [-rng.uniform(0.0, 750.0) for _ in range(5000)]                # seeded
ex += [-(2.0 ** rng.uniform(-60.0, 10.0)) for _ in range(5000)]
base = [d(0x3e30000000000000), d(0x3fd62e4300000000), d(0x3ff0a2b200000000), d(0x4086232b00000000),
        708.39641853226410622, 745.13321910194110842, 0.5 * math.log(2), 1.5 * math.log(2), math.log(2)]
edges = [0.0, -0.0, float("-inf")]
for e in base:
    edges += [-nb(e), -e, -na(e)]
    if e < 1.1:
        edges += [nb(e), e, na(e)]
ex += edges
assert not any(math.isnan(x) for x in ex)
open(OUT + "/exp-in.bin", "wb").write(b"".join(struct.pack("<d", x) for x in ex))
def rest(deg):
    q = 0.0 if deg < 90 else 90.0 if deg < 180 else 180.0 if deg < 270 else 270.0
    return ((deg - q) + 0.0) * K
sc = [(k / 1000) * K for k in range(0, 90000)]
sc += [rest(rng.uniform(0.0, 360.0)) for _ in range(10000)]
sc += [0.0, -0.0, d(0x3e46a09dffffffff), d(0x3e46a09e00000000), d(0x3fe921fbffffffff), d(0x3fe921fc00000000),
       nb(d(0x3ff921fb00000000)), d(0x3ff921fb00000000), 45.0 * K, nb(45.0) * K, nb(90.0) * K,
       rest(nb(90.0)), rest(nb(180.0)), rest(nb(270.0)), rest(nb(360.0)), d(1), d(0x0010000000000000)]
sc += [rest(rng.uniform(89.9999991, 90.0)) for _ in range(1000)]  # rem_pio2 medium band
open(OUT + "/sincos-in.bin", "wb").write(b"".join(struct.pack("<d", x) for x in sc))
print(len(ex), len(sc))
