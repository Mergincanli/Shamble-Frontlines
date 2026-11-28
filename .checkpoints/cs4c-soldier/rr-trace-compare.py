# rust-reviewer's scratch for block 10: compares two per-tick bit traces of rr-body-proto-7 (stderr, TRACE_BITS=1),
# run by run ("S" starts a soldier). Usage: python3 -I rr-trace-compare.py <base> <other>
import struct, sys
from collections import defaultdict

def runs(path):
    out, cur = [], None
    for line in open(path):
        p = line.split()
        if p[0] == "S":
            cur = []
            out.append(cur)
        elif p[0] == "T":
            cur.append((float(p[1]), int(p[2], 16), int(p[3], 16), int(p[4], 16)))
    return out

def f(b):
    return struct.unpack("<f", struct.pack("<I", b))[0]

a, b = runs(sys.argv[1]), runs(sys.argv[2])
print(f"runs {len(a)} / {len(b)}")
stats = defaultdict(lambda: [0, 0, 0.0])  # speed -> runs, differing runs, largest difference (m)
examples = []
for i, (ra, rb) in enumerate(zip(a, b)):
    sp = max((t[0] for t in ra), default=0.0)
    st = stats[sp]
    st[0] += 1
    if ra != rb:
        st[1] += 1
        worst = 0.0
        first = None
        for k, (ta, tb) in enumerate(zip(ra, rb)):
            if ta != tb:
                first = k if first is None else first
                worst = max(worst, *(abs(f(x) - f(y)) for x, y in zip(ta[1:], tb[1:])))
        if len(ra) != len(rb):
            worst = max(worst, float("inf")) if False else worst
        st[2] = max(st[2], worst)
        if sp <= 9.6 and len(examples) < 8:
            examples.append((i, sp, first, len(ra), len(rb), worst))
for sp in sorted(stats):
    n, d, w = stats[sp]
    print(f"speed {sp}: {n} runs, {d} differ, largest difference {w:.6f} m")
for e in examples:
    print("differs: run %d speed %s first tick %s ticks %d/%d largest %.6f m" % e)
