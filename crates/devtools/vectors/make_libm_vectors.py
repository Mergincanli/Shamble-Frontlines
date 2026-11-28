#!/usr/bin/env python3
"""Makes the inputs of the maths routines' vectors (PM-0004-22): libm-inputs.bin beside this script.

The simulation calls two routines of the pinned libm crate, the exponential and the sine-and-cosine, each through one
function of crates/sim/src/fmath.rs (exp_of, sincos_of) that hands on the routine's own f64 result. PM-0004-22 asks of
each libm function the simulation uses at least 10,000 fixed inputs whose outputs are the same bits natively, in
WebAssembly under Node and in each browser engine. The inputs are written here. The outputs are libm's own: it is not
correctly rounded (its own words: "less than 1 ulp"), so they are not worked out here, and no part of libm is in this
file. The native build records them (crates/devtools/tests/libm_vectors.rs writes libm-expected.txt only when it is
run with SHAMBLE_WRITE_LIBM=1: one digest a routine, under a header naming libm's version and these inputs' byte
count and sha256), and every target checks them: the native test, and the test-only module in Node and in each browser
(crates/devtools/src/rules/libm_ops.rs). Their error against a correctly rounded reference is reported, not pinned
(rust-reviewer's measure of 2026-11-28, block 17's B17-2).

  python3 -I -B crates/devtools/vectors/make_libm_vectors.py           writes libm-inputs.bin
  python3 -I -B crates/devtools/vectors/make_libm_vectors.py --check   compares it byte for byte, writes nothing

The inputs, f64 little-endian, as share_per_tick and facing_of form them (rust-reviewer's block 17, B17-1 and Q5;
none is a NaN):
- the exponential's 33,646, each the x of exp(x) = exp(-(rate x dt)) with dt = 1 / 30: the easing over its times to a
  tenth T = k / 1000 on (0, 5] (its rate ln 10 / T); the push over strengths k / 100 on [0, 100]; the push toward zero,
  strengths 2^(-e/4) down to 2^-1074; the easing from T = 5 down by quarter octaves; 10,000 seeded (5,000 uniform on
  [-750, 0], 5,000 of a uniform mantissa and a whole exponent from 2^-60 to 2^10, negated); and the routine's branch
  edges (2^-28, 0.5 ln 2, 1.5 ln 2, ln 2, the over- and underflow thresholds 708.396... and 745.133..., each with
  its two neighbours and in both signs where both reach the routine), 0, -0 and -infinity;
- then the sine and cosine's 101,017, each the radians the routine is handed: the 0.001-degree grid on [0, 90); 10,000
  seeded headings on [0, 360) through the quarter turns; the edges (0, -0, the thresholds 0x3e46a09e, 0x3fe921fb/fc
  and 0x3ff921fb with neighbours, 45 degrees and the doubles just under 45 and 90, the rests of the doubles just under
  90, 180, 270 and 360, the smallest subnormal and normal); and 1,000 seeded rests on (89.9999991, 90) degrees, where
  the reduction takes its medium path.
The two sets are also the routines' counts in libm_ops.rs (EXP_CASES, SINCOS_CASES). Python's standard library only,
and no platform maths: every input is a decimal literal, a hex literal, an ldexp or one IEEE add, multiply or divide
of such, so every machine writes the same bytes.
"""
import hashlib
import math
import random
import struct
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
NAME = "libm-inputs.bin"
EXP_CASES = 33_646
SINCOS_CASES = 101_017
LN_10 = float.fromhex("0x1.26bb1bbb55516p+1")  # core::f64::consts::LN_10
LN_2 = float.fromhex("0x1.62e42fefa39efp-1")  # core::f64::consts::LN_2
DT = 1.0 / 30.0  # the tick's length, as the simulation works it out (rules.rs, DT_S)
K = math.pi / 180.0  # fmath.rs's DEG_TO_RAD
# 2^0, 2^-1/4, 2^-1/2 and 2^-3/4, each the double nearest (decimal at 60 digits, once).
QUARTERS = [1.0, float.fromhex("0x1.ae89f995ad3adp-1"), float.fromhex("0x1.6a09e667f3bcdp-1"), float.fromhex("0x1.306fe0a31b715p-1")]


def bits(x):
    return struct.unpack("<Q", struct.pack("<d", x))[0]


def of_bits(b):
    return struct.unpack("<d", struct.pack("<Q", b))[0]


def below(x):
    """The double just below a positive x."""
    return of_bits(bits(x) - 1)


def above(x):
    """The double just above a positive x."""
    return of_bits(bits(x) + 1)


def quarter_octaves(e):
    """2^(-e/4), as an exact power of two times one of the four constants."""
    return math.ldexp(QUARTERS[e % 4], -(e // 4))


def rest(deg):
    """The radians facing_of hands the routine for a heading: the whole quarter turns out, the rest canonicalised."""
    turned = 0.0 if deg < 90 else 90.0 if deg < 180 else 180.0 if deg < 270 else 270.0
    return ((deg - turned) + 0.0) * K


def inputs():
    assert bits(K) == 0x3F91DF46A2529D39 and bits(LN_10) == 0x40026BB1BBB55516 and bits(LN_2) == 0x3FE62E42FEFA39EF
    rng = random.Random(0x4C2C3)
    ex = [-((LN_10 / (k / 1000)) * DT) for k in range(1, 5001)]
    ex += [-((k / 100) * DT) for k in range(0, 10001)]
    ex += [-(quarter_octaves(e) * DT) for e in range(0, 4297)]
    ex += [-((LN_10 / (5.0 * quarter_octaves(e))) * DT) for e in range(0, 4300)]
    ex += [-rng.uniform(0.0, 750.0) for _ in range(5000)]
    ex += [-math.ldexp(1.0 + rng.random(), rng.randrange(-60, 10)) for _ in range(5000)]
    edges = [0.0, -0.0, float("-inf")]
    for e in [of_bits(0x3E30000000000000), of_bits(0x3FD62E4300000000), of_bits(0x3FF0A2B200000000),
              of_bits(0x4086232B00000000), 708.39641853226410622, 745.13321910194110842, 0.5 * LN_2, 1.5 * LN_2, LN_2]:
        edges += [-below(e), -e, -above(e)]
        if e < 1.1:
            edges += [below(e), e, above(e)]
    ex += edges
    sc = [(k / 1000) * K for k in range(0, 90000)]
    sc += [rest(rng.uniform(0.0, 360.0)) for _ in range(10000)]
    sc += [0.0, -0.0, of_bits(0x3E46A09DFFFFFFFF), of_bits(0x3E46A09E00000000), of_bits(0x3FE921FBFFFFFFFF),
           of_bits(0x3FE921FC00000000), below(of_bits(0x3FF921FB00000000)), of_bits(0x3FF921FB00000000), 45.0 * K,
           below(45.0) * K, below(90.0) * K, rest(below(90.0)), rest(below(180.0)), rest(below(270.0)),
           rest(below(360.0)), of_bits(1), of_bits(0x0010000000000000)]
    sc += [rest(rng.uniform(89.9999991, 90.0)) for _ in range(1000)]
    assert (len(ex), len(sc)) == (EXP_CASES, SINCOS_CASES), (len(ex), len(sc))
    assert not any(math.isnan(x) for x in ex + sc)
    return b"".join(struct.pack("<d", x) for x in ex + sc)


def main():
    made = inputs()
    target = HERE / NAME
    if sys.argv[1:] == ["--check"]:
        found = target.read_bytes() if target.is_file() and not target.is_symlink() else None
        if found != made:
            print(f"{NAME} is not what this script makes (run it without --check to write it)", file=sys.stderr)
            sys.exit(1)
        print(f"{NAME}: {len(made)} bytes, sha256 {hashlib.sha256(made).hexdigest()}, as this script makes it")
    elif sys.argv[1:] == []:
        if target.is_symlink():
            sys.exit(f"{NAME} is a link: not written through")
        target.write_bytes(made)
        print(f"{NAME}: {len(made)} bytes, sha256 {hashlib.sha256(made).hexdigest()}, written")
    else:
        sys.exit("usage: make_libm_vectors.py [--check]")


main()
