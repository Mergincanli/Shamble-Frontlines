# game-designer's scratch: first differing tick per run between two bit traces (runs split by "S" lines,
# ticks "T speed x z feet" as f32 hex bits). Prints run index, run length, first differing tick, both states
# decoded, the largest planar gap over the run, and the feet at the first difference.
import struct, sys

def runs(path):
    out, cur = [], None
    for line in open(path):
        p = line.split()
        if not p:
            continue
        if p[0] == 'S':
            cur = []
            out.append(cur)
        elif p[0] == 'T':
            f = lambda h: struct.unpack('>f', bytes.fromhex(h))[0]
            cur.append((p[1], f(p[2]), f(p[3]), f(p[4])))
    return out

a, b = runs(sys.argv[1]), runs(sys.argv[2])
print('runs', len(a), len(b))
for i, (ra, rb) in enumerate(zip(a, b)):
    if ra == rb:
        continue
    first = next((k for k in range(min(len(ra), len(rb))) if ra[k] != rb[k]), min(len(ra), len(rb)))
    gap = max(((x1 - x2) ** 2 + (z1 - z2) ** 2) ** 0.5 for (_, x1, z1, _), (_, x2, z2, _) in zip(ra, rb))
    sa = ra[first] if first < len(ra) else None
    sb = rb[first] if first < len(rb) else None
    prev = ra[first - 1] if first > 0 else None
    print(f'run {i} len {len(ra)}/{len(rb)} first-diff tick {first} prev {prev} base {sa} sweep {sb} max-planar-gap {gap:.4f}')
