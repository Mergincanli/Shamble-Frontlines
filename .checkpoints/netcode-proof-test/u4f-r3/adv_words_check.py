# Read-only (the advisor's check of u4f-brief.md rev 3.2): every run of 8 consecutive words (punctuation ignored) that
# stands both in the brief, outside section 12 item 3 (the fill anchors), and anywhere in history.md, with the history
# line it comes from, so each hit can be read by hand (his words, or the session's own). Writes nothing.
# Run: python3 -I /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-r3/adv_words_check.py
import re

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/"
B = open(N + "u4f-brief.md", encoding="utf-8").read().split("\n")
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
    g = tuple(t for t, _ in w[i:i + K])
    grams.setdefault(g, w[i][1])

start = next(i for i, l in enumerate(B) if l.startswith("3. **The placeholders, by script"))
end = next(i for i, l in enumerate(B) if i > start and l.startswith("4. **The advisor's rulings"))
stream = []
for n, line in enumerate(B, 1):
    if start + 1 <= n <= end:
        stream.append(("|", n))
        continue
    for t in toks(line):
        stream.append((t, n))
hits = {}
for i in range(len(stream) - K + 1):
    win = stream[i:i + K]
    if any(t == "|" for t, _ in win):
        continue
    g = tuple(t for t, _ in win)
    if g in grams:
        hits.setdefault(grams[g], set()).add(win[0][1])
for h in sorted(hits):
    print("history line", h, "<- brief lines", sorted(hits[h])[:6], "|", H[h - 1][:110])
print("history lines hit:", len(hits))
