"""Move four finished blocks from the card (.checkpoints/netcode-proof-test.md) to history.md's end, word for word
(TEST-FIRST rule 3: the card stays at most 200 lines; everything older goes to the history file): the three choices of
2026-11-18 under "Choices taken without him" (each already named in its day's message) and MAIN-60's done line under
"Running work". A pointer on the card names their new place (written by Edit). Each block is found by its first line,
once, and runs while the next lines start with two spaces (as card_trim_1118.py). Refuses to run twice. Run from the
repo root:  python3 -I .checkpoints/netcode-proof-test/card_trim_1119f.py"""
import hashlib

C = ".checkpoints/netcode-proof-test.md"
H = ".checkpoints/netcode-proof-test/history.md"
STARTS = [
    "- **2026-11-18 (named in that day's message):** D3-1's fix",
    "- **2026-11-18 (named 2026-11-19):** the fill's form",
    "- **2026-11-18 (named that day, 13:45 UTC; the worktrees 2026-11-19):**",
    "- **MAIN-60 DONE** (its record at history.md's end",
]
HEAD = ("## Card blocks moved 2026-11-19 during MAIN-61's review (card_trim_1119f.py; from the card's 'Choices taken "
        "without him' and 'Running work')")

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
