# Read-only (rev 3.1 of u4f-brief.md): prints the brief's sha256 and line count; the count of P24's start key in
# df-ruling (advisor-u4df.md) and of its bare first words; each <<Pn>> placeholder's count and any unknown id; and every
# run of 7 consecutive words (punctuation ignored) from Mustafa's lines in history.md that also stands in the brief
# outside section 12 item 3 (the fill anchors). Writes nothing.
# Run from the repository root: python3 -I .checkpoints/netcode-proof-test/u4f-r3/r31_check.py
import hashlib
import re

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/"
raw = open(N + "u4f-brief.md", "rb").read()
brief = raw.decode("utf-8")
B = brief.split("\n")
print("brief sha256", hashlib.sha256(raw).hexdigest(), "lines", raw.count(b"\n"))

df = open(N + "advisor-u4df.md", encoding="utf-8").read()
print("P24 key count:", df.count('word for word: "Write files with the'),
      "bare words count:", df.count("Write files with the"))

for k in range(1, 37):
    c = brief.count(f"<<P{k}>>")
    if c == 0:
        print(f"P{k} NOT USED")
print("placeholder uses:", {f"P{k}": brief.count(f"<<P{k}>>") for k in range(1, 37)})
print("unknown <<P ids:", sorted(set(re.findall(r"<<P(\d+)>>", brief)) - {str(k) for k in range(1, 37)}))
print("PV1", brief.count("<<PV1>>"), "base", brief.count("<base>"), "@@", brief.count("@@"),
      "r3 markers", brief.count("<!-- r3"))

# Section 12 item 3's span, by its own text.
start = next(i for i, l in enumerate(B) if l.startswith("3. **The placeholders, by script"))
end = next(i for i, l in enumerate(B) if i > start and l.startswith("4. **The advisor's rulings"))
print("section 12 item 3: lines", start + 1, "to", end)

H = open(N + "history.md", encoding="utf-8").read().split("\n")
RANGES = [(292, 293), (2040, 2058), (2091, 2111), (2116, 2129), (2134, 2146), (2151, 2161), (2166, 2175),
          (2180, 2187), (2190, 2190), (2200, 2200), (2205, 2205), (2230, 2230)]


def toks(s):
    return re.findall(r"[a-z0-9]+", s.lower())


grams = {}
for a, b in RANGES:
    w = []
    for i in range(a, b + 1):
        w.extend(toks(H[i - 1]))
    for i in range(len(w) - 6):
        grams.setdefault(tuple(w[i:i + 7]), a)

stream = []
for n, line in enumerate(B, 1):
    if start + 1 <= n <= end:
        stream.append(("|", n))
        continue
    for t in toks(line):
        stream.append((t, n))
seen = set()
for i in range(len(stream) - 6):
    w = stream[i:i + 7]
    if any(t == "|" for t, _ in w):
        continue
    g = tuple(t for t, _ in w)
    if g in grams and (w[0][1], g) not in seen:
        seen.add((w[0][1], g))
        print("7-word run, brief line", w[0][1], "history from", grams[g], ":", " ".join(g))
print("7-word runs outside section 12 item 3:", len(seen))
