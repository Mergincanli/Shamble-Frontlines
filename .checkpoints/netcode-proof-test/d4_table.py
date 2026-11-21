"""D4's tables, print only: R (ticks per second) for each of the 15 timed runs in a runner log, the slowest kept per
scene and profile beside r5's (u4r-build.md:1460-1466), and 7.3's wall-time estimate redone for the set as it stands
today. Each judged run is 300 + 18,000 + about 35 after-phase ticks (u4r-build.md:1468-1469). P0 has six bites now
(whole-as-lost joined -17's bites at 5e563b9). The pre-check is a recording plus 3 pairs x 2 legs, each 9,000 ticks,
at P0's R (an upper bound, as in r5).
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/d4_table.py [log]   (default .gate/u4r-d4/d4.log)"""
import re
import sys

LOG = sys.argv[1] if len(sys.argv) > 1 else "/home/user/Shamble-Frontlines/.gate/u4r-d4/d4.log"
R5 = {("SS", "P0"): 79.6, ("SS", "P2"): 63.1, ("SS", "P3"): 60.1, ("SS", "B1"): 67.1, ("ES", "P3"): 161.6}
JUDGED_TICKS = 300 + 18_000 + 35
PRE_TICKS = 9_000

text = open(LOG, encoding="utf-8").read()
blocks = re.split(r"^=== \S+ UTC run: ", text, flags=re.M)[1:]
rows = []
for b in blocks:
    m = re.match(r"bash -c time \S+ scenario --run (r5-(\w+)-(\w+)-s(\d))", b)
    if not m:
        continue
    run, scene, prof, seed = m.group(1), m.group(2).upper(), m.group(3), int(m.group(4))
    t = re.search(rf"^{re.escape(run)} {scene} {prof} seed {seed}: (\d+) ticks \((\d+) after\)", b, flags=re.M)
    real = re.search(r"^real\s+(\d+)m([\d.]+)s", b, flags=re.M)
    code = re.search(r"^=== \S+ UTC exit (\d+): ", b, flags=re.M)
    assert t and real and code, f"incomplete block for {run}"
    secs = int(real.group(1)) * 60 + float(real.group(2))
    rows.append((scene, prof, seed, int(t.group(1)), int(t.group(2)), secs, int(t.group(1)) / secs, int(code.group(1))))
assert len(rows) == 15, f"{len(rows)} runs found, 15 expected"
assert all(r[7] == 0 for r in rows), "a run did not exit 0"

print("| Scene, profile | Seed 1 | Seed 2 | Seed 3 | Slowest kept (ticks per second) | r5's slowest |")
print("|---|---|---|---|---|---|")
slow = {}
for key in R5:
    rs = sorted((r for r in rows if (r[0], r[1]) == key), key=lambda r: r[2])
    cells = [f"{r[3]:,} / {r[5]:.2f} s = {r[6]:.1f}" for r in rs]
    slow[key] = min(r[6] for r in rs)
    print(f"| {key[0]} {key[1]} | {' | '.join(cells)} | **{slow[key]:.1f}** | {R5[key]} |")

p0, p2, p3, b1 = (slow[("SS", p)] for p in ("P0", "P2", "P3", "B1"))
items = [
    ("-17 measured (also -57's), P0 x 3 seeds", 3, 3 * JUDGED_TICKS / p0),
    ("P2 x 3", 3, 3 * JUDGED_TICKS / p2),
    ("P3 x 3", 3, 3 * JUDGED_TICKS / p3),
    ("B1 x 3", 3, 3 * JUDGED_TICKS / b1),
    ("-17's bites (tick16, client-rounding, whole-as-lost) and -57's (pos-4cm, yaw-12bit, pitch-8bit), P0", 6,
     6 * JUDGED_TICKS / p0),
    ("-30 pre-check's recording (SS P0 seed 4, 9,000 ticks, --logs 0), at P0's R (upper bound)", 1, PRE_TICKS / p0),
    ("-30 pre-check, 3 pairs x 2 legs x 9,000 ticks, at P0's R (upper bound)", 6, 6 * PRE_TICKS / p0),
]
print()
print("| Item | Runs | Wall s at the slowest R |")
print("|---|---|---|")
for name, n, s in items:
    print(f"| {name} | {n} | {s:,.1f} |")
total = sum(s for _, _, s in items)
print(f"| **Total** | | **{total:,.1f} s = {total / 60:.1f} min = {total / 3600:.2f} h** |")
sets = {"pre-check": items[5][2] + items[6][2], "P3": items[2][2], "B1": items[3][2], "P2": items[1][2],
        "P0": items[0][2] + items[4][2]}
print()
print("Per set (runs only; builds and the judge not included): " +
      "; ".join(f"{k} {v:,.0f} s ({v / 60:.1f} min)" for k, v in sets.items()))
print(f"Longest single judged run at the slowest R: {JUDGED_TICKS / min(p0, p2, p3, b1):,.1f} s "
      f"(RUN_LIMIT_MS 3,600 s is {3600 / (JUDGED_TICKS / min(p0, p2, p3, b1)):.1f} times it)")
