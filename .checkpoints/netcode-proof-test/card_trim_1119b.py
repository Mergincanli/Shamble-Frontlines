"""Move builders 1's and 2's finished blocks from the card (.checkpoints/netcode-proof-test.md) to history.md's end, word
for word, before MAIN-60 (TEST-FIRST rule 3: the card stays at most 200 lines; everything older goes to the history
file; builder 3's block and the stop ruling now carry U4f's state). Each block is found by its first line, once, and
runs while the next lines start with two spaces (as card_trim_1118.py). Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/card_trim_1119b.py"""
import hashlib

C = ".checkpoints/netcode-proof-test.md"
H = ".checkpoints/netcode-proof-test/history.md"
STARTS = [
    "- **Builder 1 (11:46-13:13 UTC): a PARTIAL build, by its own stop**",
    "- **Builder 2 (13:45-17:59 UTC; cut 14:06:57 to about 16:38): PARTIAL, by its own working room",
]
HEAD = "## Card blocks moved 2026-11-19 before MAIN-60 (card_trim_1119b.py; from the card's 'Now: U4f')"

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
