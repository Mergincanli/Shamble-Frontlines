# Read-only (rev 3.2 of u4f-brief.md): prints 2.10's test table sums (all, negative, conditional rows apart), the
# recheck rows of section 8, the 9.1 rows, the U4F-Q headings of section 10, the 5.3 rows, and 5.5's disk total from
# its rows. Writes nothing. Run: python3 -I .checkpoints/netcode-proof-test/u4f-r3/r32_sums.py
import re

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/"
B = open(N + "u4f-brief.md", encoding="utf-8").read().split("\n")


def span(start_pred, end_pred):
    s = next(i for i, l in enumerate(B) if start_pred(l))
    e = next(i for i, l in enumerate(B) if i > s and end_pred(l))
    return B[s:e]


t = span(lambda l: l.startswith("### 2.10 "), lambda l: l.startswith("### 2.11 "))
tot = neg = cond = 0
ids = []
for l in t:
    m = re.match(r"^\| (f\d+) \|", l)
    if not m:
        continue
    cells = [c.strip() for c in l.strip().strip("|").split("|")]
    ids.append(m.group(1))
    d = cells[-1]
    isneg = cells[-2] == "neg"
    if "conditional" in d:
        cond += 1
        continue
    if "bites only" in d:
        continue
    v = int(re.match(r"\d+", d).group(0))
    tot += v
    if isneg:
        neg += v
print("2.10 rows:", len(ids), ids[0], "to", ids[-1], "| measured sum", tot, "| negatives", neg,
      "| conditional rows", cond)

r8 = span(lambda l: l.startswith("## 8. "), lambda l: l.startswith("**Tally:**"))
rows = [l for l in r8 if re.match(r"^\| \d+ \|", l)]
print("section 8 rows:", len(rows), "last", rows[-1].split("|")[1].strip())

r91 = span(lambda l: l.startswith("### 9.1 "), lambda l: l.startswith("### 9.2 "))
print("9.1 rows:", len([l for l in r91 if l.startswith("| ") and not l.startswith("| Id") and not l.startswith("|---")]))

qs = [l for l in B if re.match(r"^\*\*U4F-Q\d+,", l)]
print("questions:", len(qs))

r53 = span(lambda l: l.startswith("### 5.3 "), lambda l: l.startswith("### 5.4 "))
print("5.3 rows:", [l.split("|")[1].strip() for l in r53 if re.match(r"^\| p\d", l)])

r55 = span(lambda l: l.startswith("### 5.5 "), lambda l: l.startswith("**Room"))
vals = {}
for l in r55:
    m = re.match(r"^\| (-20|-21|-22) \| \d+ \| ([\d,.]+) ", l)
    if m:
        vals[m.group(1)] = float(m.group(2).replace(",", ""))
pre, cap = 195.0, 95.0
low = sum(vals.values()) + pre + cap
print("5.5 rows:", vals, "| total with 15 pre-runs", round(low, 1), "| with 21", round(low - pre + 315.0, 1))
print("products: 7 x 53.4 =", round(7 * 53.4, 1), "| 49 x 53.4 + 6.2 =", round(49 * 53.4 + 6.2, 1),
      "| 3 x 53.4 + 160.2 =", round(3 * 53.4 + 160.2, 1))
print("D2 room: 5468 - 4096 =", 5468 - 4096, "| + 380 =", 5468 - 4096 + 380, "| short of 3,607 by",
      3607 - (5468 - 4096 + 380), "| C62H-BOUND sets at 73 %:", round((741 + 298) * 0.73))
