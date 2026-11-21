"""Move one finished block from the card (.checkpoints/netcode-proof-test.md) to history.md's end, word for word,
after U4f2's build ended and its first review round began (the card at 209 lines, over TEST-FIRST rule 3's 200): the
'U4f2's BUILD RUNNING' bullet (the build's record, its four floor stops and part-cleans, end checks A3 and B, and the
round's first two verdicts; u4f2-build.md and the seats' files hold the detail). A short current bullet replaces it
on the card (written by Edit). The block is found by its first line, once, and runs while the next lines start with
two spaces (as card_trim_1120f.py). Refuses to run twice. Run from the repo root:
python3 -I .checkpoints/netcode-proof-test/card_trim_1120g.py"""
import hashlib

C = ".checkpoints/netcode-proof-test.md"
H = ".checkpoints/netcode-proof-test/history.md"
STARTS = [
    "- **U4f2's BUILD RUNNING** from 14:16 UTC",
]
HEAD = "## Card block moved 2026-11-20 about 18:00 UTC after U4f2's build (card_trim_1120g.py; from 'Running work')"

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
at = first + 1
for b in moved:
    print("  moved:", b[0][:90], f"({len(b)} lines, history.md:{at}-{at + len(b) - 1})")
    at += len(b)
