# Read-only: prints every run of 7 consecutive words from Mustafa's quoted lines in history.md that also stands in
# u4f-brief.md outside section 12 item 3 (the fill anchors, brief lines 1310-1351). Writes nothing.
import re

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/"
H = open(N + "history.md", encoding="utf-8").read().split("\n")
B = open(N + "u4f-brief.md", encoding="utf-8").read().split("\n")

# His lines named by the task and the brief (1-based, inclusive).
RANGES = [(292, 293), (2048, 2051), (2125, 2125), (2127, 2127), (2134, 2146), (2151, 2161), (2166, 2175),
          (2180, 2187), (2200, 2200), (2205, 2205)]
ANCHOR_LINES = range(1310, 1352)


def toks(s):
    return re.findall(r"[a-z0-9]+(?:['.][a-z0-9]+)*", s.lower())


his = []
for a, b in RANGES:
    for i in range(a, b + 1):
        his.extend(toks(H[i - 1]))
grams = set()
for i in range(len(his) - 6):
    grams.add(tuple(his[i:i + 7]))

# The brief outside the anchors, as one token stream with each token's line.
stream = []
for n, line in enumerate(B, 1):
    if n in ANCHOR_LINES:
        stream.append(("|", n))  # a break: no run crosses the anchors
        continue
    for t in toks(line):
        stream.append((t, n))
hits = []
for i in range(len(stream) - 6):
    w = stream[i:i + 7]
    if any(t == "|" for t, _ in w):
        continue
    if tuple(t for t, _ in w) in grams:
        hits.append((w[0][1], " ".join(t for t, _ in w)))
print("his words:", len(his), "7-grams:", len(grams), "hits outside the anchors:", len(hits))
seen = set()
for n, g in hits:
    if (n, g) in seen:
        continue
    seen.add((n, g))
    print(n, "|", g)
