"""Move three finished blocks from the card (.checkpoints/netcode-proof-test.md) to history.md's end, word for word
(TEST-FIRST rule 3: the card stays a handoff of at most 200 lines; everything older goes to the history file). Each
block is found by its first line, once, and runs to the line before the next line starting "- " or "## " (or a blank
line). Refuses to run twice. Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/card_trim_1118.py"""
import hashlib

C = ".checkpoints/netcode-proof-test.md"
H = ".checkpoints/netcode-proof-test/history.md"
STARTS = [
    "- **Launch DONE (11:3x-11:49 UTC; each step's figures in this card at a64c216):**",
    "- **2026-11-17:** the advisor's rulings on U4F-Q1 to Q18",
    "- **2026-11-17:** no second review round or second model by default from now on",
    "- The last workflow: wf_b2a20e74-6aa",
]
HEAD = "## Card blocks moved 2026-11-18 (card_trim_1118.py; from the card's 'Now: U4f', 'Choices taken without him' and 'Running work')"

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
