# Read-only: every run of 8 consecutive words (punctuation ignored) that stands both in the advisor's ruling
# advisor-u4f-brief.md and anywhere in history.md, with the history line, so each hit can be read by hand. Writes nothing.
# Run: python3 -I /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-r3/adv_words_ruling.py
import re

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/"
R = open(N + "advisor-u4f-brief.md", encoding="utf-8").read().split("\n")
H = open(N + "history.md", encoding="utf-8").read().split("\n")
K = 8


def toks(s):
    return re.findall(r"[a-z0-9]+", s.lower())


grams = {}
w = []
for n, line in enumerate(H, 1):
    for t in toks(line):
        w.append((t, n))
for i in range(len(w) - K + 1):
    grams.setdefault(tuple(t for t, _ in w[i:i + K]), w[i][1])
s = []
for n, line in enumerate(R, 1):
    for t in toks(line):
        s.append((t, n))
hits = {}
for i in range(len(s) - K + 1):
    g = tuple(t for t, _ in s[i:i + K])
    if g in grams:
        hits.setdefault(grams[g], set()).add(s[i][1])
for h in sorted(hits):
    print("history line", h, "<- ruling lines", sorted(hits[h])[:6], "|", H[h - 1][:110])
print("history lines hit:", len(hits))
