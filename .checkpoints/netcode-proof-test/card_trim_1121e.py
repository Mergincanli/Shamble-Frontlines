#!/usr/bin/env python3
"""Move the card's 2026-11-21 16:12:40-17:41:37 cut entry to history.md word for word, and point to it from the card's
"Cuts until" paragraph (room for F-B-COUNT's answer and U4f2's part B). Run once; it refuses a second run."""
import sys

D = "/home/user/Shamble-Frontlines/.checkpoints"
CARD = D + "/netcode-proof-test.md"
HIST = D + "/netcode-proof-test/history.md"

card = open(CARD, encoding="utf-8").read().split("\n")
start = [i for i, l in enumerate(card) if l.startswith("- **2026-11-21 16:12:40 to 17:41:37 UTC")]
if len(start) != 1:
    sys.exit("refused: the 2026-11-21 16:12:40 entry is not on the card exactly once")
s = start[0]
e = s + 1
while e < len(card) and card[e].startswith("  "):
    e += 1
block = card[s:e]
if len(block) != 7:
    sys.exit("refused: the entry is %d lines, not the 7 it was" % len(block))

hist = open(HIST, encoding="utf-8").read()
if not hist.endswith("\n"):
    sys.exit("refused: history.md does not end with a newline")
if "(card_trim_1121e.py)" in hist:
    sys.exit("refused: already moved")
n0 = hist.count("\n")
head = ("## Moved from the card 2026-11-21 ~19:5x UTC (card_trim_1121e.py): the 2026-11-21 16:12:40-17:41:37 cut's "
        "entry, word for word")
add = [head] + block
with open(HIST, "a", encoding="utf-8") as f:
    f.write("\n".join(add) + "\n")
first, last = n0 + 2, n0 + 1 + len(block)

del card[s:e]
idx = [i for i, l in enumerate(card) if l.startswith("- Cuts until 2026-11-17:")]
if len(idx) != 1:
    sys.exit("refused after the history append: no single 'Cuts until' line (history.md already appended)")
j = idx[0]
k = j + 1
while k < len(card) and card[k].startswith("  "):
    k += 1
last_line = card[k - 1]
if not last_line.endswith("(1121d)."):
    sys.exit("refused after the history append: the 'Cuts until' paragraph does not end as it did")
card[k - 1] = last_line[:-1] + ";"
card.insert(k, "  2026-11-21's 16:12:40-17:41:37 one at history.md:%d-%d (1121e)." % (first, last))
open(CARD, "w", encoding="utf-8").write("\n".join(card))
print("moved %d lines to history.md:%d-%d; card now %d lines" % (len(block), first, last, len(card) - 1))
