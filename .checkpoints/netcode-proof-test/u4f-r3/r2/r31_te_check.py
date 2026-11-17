# Read-only (the test-engineer's round-2 check of u4f-brief.md rev 3.1): prints counts only, writes nothing.
# Run: python3 -I .checkpoints/netcode-proof-test/u4f-r3/r2/r31_te_check.py
import hashlib
import re

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/"
raw = open(N + "u4f-brief.md", "rb").read()
brief = raw.decode("utf-8")
B = brief.split("\n")
print("brief sha256", hashlib.sha256(raw).hexdigest(), "newlines", raw.count(b"\n"))

df = open(N + "advisor-u4df.md", encoding="utf-8").read()
print("P24 key", df.count('word for word: "Write files with the'), "bare", df.count("Write files with the"))
dl = df.split("\n")
for i, l in enumerate(dl, 1):
    if "Write files with the" in l:
        print("  df line", i, "key here:", 'word for word: "Write files with the' in l)

# 2.10's table: sum of the delta column and of neg rows, measured build (f30 bites only, f38 conditional apart)
tot = 0
neg = 0
rows = 0
for l in B:
    m = re.match(r"^\| (f\d+) \|(.*)\| (\w[\w, ]*|) \| (neg|) \| ([^|]+) \|$", l)
    if not m:
        continue
    rows += 1
    fid, delta = m.group(1), m.group(5).strip()
    d = int(re.match(r"\d+", delta).group(0))
    if "bites only" in delta or "conditional" in delta:
        print("  apart:", fid, delta)
        continue
    tot += d
    if m.group(4) == "neg":
        neg += d
print("2.10 rows", rows, "measured delta sum", tot, "neg sum", neg)

# 5.3's table rows
pre = [l for l in B if re.match(r"^\| p\d+(-p\d+)? \|", l)]
print("5.3 rows", len(pre), [l.split("|")[1].strip() for l in pre])

# Q-range mentions
for pat in ["Q1 to Q17", "Q1 to Q18", "Q1-Q18", "Q1-Q17", "U4F-Q1 to Q17"]:
    print(pat, [i for i, l in enumerate(B, 1) if pat in l])

# 7-word runs of his words (history ranges as r31_check.py, plus :2212 reported apart as the session's text)
H = open(N + "history.md", encoding="utf-8").read().split("\n")
def toks(s):
    return re.findall(r"[a-z0-9]+", s.lower())
def grams_of(ranges):
    g = {}
    for a, b in ranges:
        w = []
        for i in range(a, b + 1):
            w.extend(toks(H[i - 1]))
        for i in range(len(w) - 6):
            g.setdefault(tuple(w[i:i + 7]), a)
    return g
HIS = [(292, 293), (2040, 2058), (2091, 2111), (2116, 2129), (2134, 2146), (2151, 2161), (2166, 2175),
       (2180, 2187), (2190, 2190), (2200, 2200), (2205, 2205), (2230, 2230)]
SESSION = [(2212, 2212)]
start = next(i for i, l in enumerate(B) if l.startswith("3. **The placeholders, by script"))
end = next(i for i, l in enumerate(B) if i > start and l.startswith("4. **The advisor's rulings"))
for label, rg in (("his", HIS), ("session :2212", SESSION)):
    g = grams_of(rg)
    stream = []
    for n, line in enumerate(B, 1):
        if start + 1 <= n <= end:
            stream.append(("|", n))
            continue
        for t in toks(line):
            stream.append((t, n))
    hits = set()
    for i in range(len(stream) - 6):
        w = stream[i:i + 7]
        if any(t == "|" for t, _ in w):
            continue
        k = tuple(t for t, _ in w)
        if k in g:
            hits.add(w[0][1])
    print(label, "7-word runs outside 12 item 3 at lines", sorted(hits))
