"""Move two finished blocks from the card (.checkpoints/netcode-proof-test.md) to history.md's end, word for word,
after MAIN-60 closed (TEST-FIRST rule 3: the card stays at most 200 lines; everything older goes to the history file):
MAIN-60's build record and R3 (iii)'s hand runs. A short pointer stays on the card. Each block is found by its first
line, once, and runs while the next lines start with two spaces (as card_trim_1118.py). Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/card_trim_1119c.py"""
import hashlib

C = ".checkpoints/netcode-proof-test.md"
H = ".checkpoints/netcode-proof-test/history.md"
STARTS = [
    "- **MAIN-60 BUILT (23:39-23:43 UTC; advisor-main-60.md's C1-C6 met):**",
    "- **R3 (iii)'s four hand runs DONE**",
]
HEAD = "## Card blocks moved 2026-11-19 after MAIN-60 closed (card_trim_1119c.py; from the card's 'Running work')"

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
open(H, "w", encoding="utf-8").write(hist.rstrip("\n") + "\n" + block)
open(C, "w", encoding="utf-8").write("\n".join(new_card))
print(f"card: {len(card)} -> {len(new_card)} lines; moved {len(drop)} lines in {len(moved)} blocks; "
      f"history.md now {(hist.rstrip(chr(10)) + chr(10) + block).count(chr(10))} lines; "
      f"moved text sha256 {hashlib.sha256(block.encode()).hexdigest()[:16]}")
for b in moved:
    print("  moved:", b[0][:90], f"({len(b)} lines)")
