"""Move two finished blocks from the card (.checkpoints/netcode-proof-test.md) to history.md's end, word for word
(TEST-FIRST rule 3: the card stays at most 200 lines; everything older goes to the history file): the two cut records
of 2026-11-18 (builders 2 and 3) from "Cuts by the spend limit". A pointer on the card names their new place (written
by Edit). Each block is found by its first line, once, and runs while the next lines start with two spaces (as
card_trim_1118.py). Refuses to run twice. Run from the repo root:
python3 -I .checkpoints/netcode-proof-test/card_trim_1119e.py"""
import hashlib

C = ".checkpoints/netcode-proof-test.md"
H = ".checkpoints/netcode-proof-test/history.md"
STARTS = [
    "- **2026-11-18 18:47:03 to about 22:39 UTC (builder 3):**",
    "- **2026-11-18 14:06:57 to 16:38 UTC (builder 2):**",
]
HEAD = "## Card blocks moved 2026-11-19 before builder 4 (card_trim_1119e.py; from the card's 'Cuts by the spend limit')"

card = open(C, encoding="utf-8").read().split("\n")
hist = open(H, encoding="utf-8").read()
assert HEAD not in hist, "already moved"
moved, drop = [], set()
for s in STARTS:
    hits = [i for i, l in enumerate(card) if l.startswith(s)]
    assert len(hits) == 1, (s, hits)
    i = hits[0]
    j = i + 1
    while j < len(card) and card[j].startswith("  "):
        j += 1
    moved.append(card[i:j])
    drop.update(range(i, j))
new_card = [l for k, l in enumerate(card) if k not in drop]
block = "\n" + HEAD + "\n" + "\n".join("\n".join(b) for b in moved) + "\n"
new_hist = hist.rstrip("\n") + "\n" + block
open(H, "w", encoding="utf-8").write(new_hist)
open(C, "w", encoding="utf-8").write("\n".join(new_card))
first = hist.rstrip("\n").count("\n") + 3
print(f"card: {len(card)} -> {len(new_card)} lines; moved {len(drop)} lines in {len(moved)} blocks; "
      f"history.md now {new_hist.count(chr(10))} lines, the block at :{first}-{new_hist.count(chr(10))}; "
      f"moved text sha256 {hashlib.sha256(block.encode()).hexdigest()[:16]}")
for b in moved:
    print("  moved:", b[0][:90], f"({len(b)} lines)")
