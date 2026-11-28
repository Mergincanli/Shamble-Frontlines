# Read-only: 2.10's table sums in u4f-brief.md rev 3.1 (prints only).
import re

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/"
B = open(N + "u4f-brief.md", encoding="utf-8").read().split("\n")
tot = neg = rows = 0
for l in B:
    if not re.match(r"^\| f\d+ \|", l):
        continue
    cells = [c.strip() for c in l.strip().strip("|").split("|")]
    fid, delta, isneg = cells[0], cells[-1], cells[-2]
    rows += 1
    d = int(re.match(r"\d+", delta).group(0))
    if "bites only" in delta or "conditional" in delta:
        print("apart:", fid, delta, isneg)
        continue
    tot += d
    if isneg == "neg":
        neg += d
print("rows", rows, "measured sum", tot, "neg sum", neg)
