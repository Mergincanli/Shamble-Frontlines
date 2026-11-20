"""Move two finished blocks from the card (.checkpoints/netcode-proof-test.md) to history.md's end, word for word,
during -20's re-run (the card at 202 lines, over TEST-FIRST rule 3's 200): the advisor's -20 ruling block (its facts
are in u4f-report.md, "After the merge", item 5, and in advisor-u4f-20.md) and the 'U4f-F1 BUILT' line (superseded by
the merge's record). A pointer replaces them on the card (written by Edit). Each block is found by its first line,
once, and runs while the next lines start with two spaces (as card_trim_1118.py). Refuses to run twice. Run from the
repo root:  python3 -I .checkpoints/netcode-proof-test/card_trim_1120d.py"""
import hashlib

C = ".checkpoints/netcode-proof-test.md"
H = ".checkpoints/netcode-proof-test/history.md"
STARTS = [
    "- **The advisor's -20 ruling** (advisor-u4f-20.md, 170 lines, read in full)",
    "- **U4f-F1 BUILT**",
]
HEAD = "## Card blocks moved 2026-11-20 during -20's re-run (card_trim_1120d.py; from 'Now: U4f' and 'Running work')"

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
