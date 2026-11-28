#!/usr/bin/env python3
"""Move the card's 2026-11-20 22:39-02:37 cut entry to history.md word for word, put the 2026-11-21 16:12-17:41 cut
in its place, and point to the moved entry from the card's "Cuts until" line. Run once; it refuses a second run."""
import sys

D = "/home/user/Shamble-Frontlines/.checkpoints"
CARD = D + "/netcode-proof-test.md"
HIST = D + "/netcode-proof-test/history.md"

card = open(CARD, encoding="utf-8").read().split("\n")
start = [i for i, l in enumerate(card) if l.startswith("- **2026-11-20 22:39:36 to 2026-11-21 02:37:53 UTC")]
if len(start) != 1:
    sys.exit("refused: the 2026-11-20 22:39 entry is not on the card exactly once")
s = start[0]
e = s + 1
while e < len(card) and card[e].startswith("  "):
    e += 1
block = card[s:e]
if any("2026-11-21 16:12:40" in l for l in card):
    sys.exit("refused: the new entry is already on the card")

hist = open(HIST, encoding="utf-8").read()
if not hist.endswith("\n"):
    sys.exit("refused: history.md does not end with a newline")
n0 = hist.count("\n")
head = ("## Moved from the card 2026-11-21 ~17:4x UTC (card_trim_1121d.py): the 2026-11-20 22:39:36-02:37:53 cut's "
        "entry, word for word")
add = [head] + block
with open(HIST, "a", encoding="utf-8") as f:
    f.write("\n".join(add) + "\n")
first, last = n0 + 2, n0 + 1 + len(block)

new = [
    "- **2026-11-21 16:12:40 to 17:41:37 UTC (a spend-limit cut: U5's builder in Block 2's fmt check, Block 1 done; the",
    "  U6 facts scan; my guard rev 4 step):** the notice, word for word (transcript line 68468, 16:14:55.013; the same text",
    "  at 16:34:46 as the 16:34 check-in tried; the facts scan's at 16:12:40.551, the builder's at 16:12:46.047): \"You've",
    "  hit your org's monthly spend limit · ask your admin for a higher limit · your 5-hour limit resets 5:40pm (UTC)\".",
    "  Both agents' tasks ended on it (rate_limit, HTTP 429); the saver ran on (byihocm9i; last save 43cf1f7, 16:13:37).",
    "  Resumed at 17:41:37 (his message): no cargo or rustc running; the builder's copy as u5-build.md says (Block 1 DONE,",
    "  no plant left), its last fmt check exit 0; the 8 files the harness flagged were only touched by u5-run.sh:73.",
]
card[s:e] = new
idx = [i for i, l in enumerate(card) if l.startswith("- Cuts until 2026-11-17:")]
if len(idx) != 1:
    sys.exit("refused after the history append: no single 'Cuts until' line (history.md already appended)")
j = idx[0]
k = j + 1
while k < len(card) and card[k].startswith("  "):
    k += 1
last_line = card[k - 1]
if not last_line.endswith("."):
    sys.exit("refused after the history append: the 'Cuts until' paragraph does not end with a period")
card[k - 1] = last_line[:-1] + ";"
card.insert(k, "  2026-11-20's 22:39:36-02:37:53 one at history.md:%d-%d (1121d)." % (first, last))
open(CARD, "w", encoding="utf-8").write("\n".join(card))
print("moved %d lines to history.md:%d-%d; card now %d lines" % (len(block), first, last, len(card) - 1))
