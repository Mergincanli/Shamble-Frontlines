# Read-only: counts the brief's placeholders, leftover markers and lines, prints its sha256, and compares the brief
# with Mustafa's lines in history.md (P14-P23's spans) for any run of 7 words copied word for word.
# Run from the repository root: python3 -I .checkpoints/netcode-proof-test/u4f-r3/brief_check.py
import hashlib
import pathlib
import re

N = pathlib.Path(".checkpoints/netcode-proof-test")
brief = (N / "u4f-brief.md").read_text(encoding="utf-8")
raw = (N / "u4f-brief.md").read_bytes()
print("brief sha256", hashlib.sha256(raw).hexdigest(), raw.count(b"\n"), "lines")
for k in range(1, 37):
    print(f"P{k}", brief.count(f"<<P{k}>>"))
print("PV1", brief.count("<<PV1>>"), "base", brief.count("<base>"))
print("@@ left", brief.count("@@"), "markers left", brief.count("<!-- r3"))
print("unknown <<P ids", sorted(set(re.findall(r"<<P(\d+)>>", brief)) - {str(k) for k in range(1, 37)}))

hist = (N / "history.md").read_text(encoding="utf-8").splitlines()
# His words of 2026-11-14 and from 2026-11-15 on, by line range (1-based, inclusive).
ranges = [(292, 293), (2040, 2058), (2091, 2111), (2116, 2129), (2134, 2146), (2153, 2161), (2166, 2175),
          (2180, 2187), (2190, 2190), (2200, 2200), (2205, 2205), (2230, 2230)]
words = lambda s: re.findall(r"[A-Za-z0-9']+", s.lower())
bw = words(brief)
grams = {tuple(bw[i:i + 7]) for i in range(len(bw) - 6)}
hits = []
for a, b in ranges:
    w = words(" ".join(hist[a - 1:b]))
    for i in range(len(w) - 6):
        g = tuple(w[i:i + 7])
        if g in grams:
            hits.append((a, " ".join(g)))
seen = set()
for a, g in hits:
    if g not in seen:
        seen.add(g)
        print("7-word match with history from line", a, ":", g)
print("7-word matches:", len(seen))
