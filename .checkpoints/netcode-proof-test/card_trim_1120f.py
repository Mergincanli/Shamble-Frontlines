"""Move five finished blocks from the card (.checkpoints/netcode-proof-test.md) to history.md's end, word for word,
during U4f2's build (the card at 230 lines, over TEST-FIRST rule 3's 200): 'U4f-F1 CLOSED' (u4f-f1-report.md and
advisor-u4f-f1-close.md hold it), U4g's brief from rev 1 to rev 2.1 (u4g-brief.md and advisor-u4g-brief.md hold it),
U4f2's brief check (advisor-u4f2-brief.md and u4f2-brief.md rev 1.2 hold it; U4F2-SIZE is answered), and the two cuts
of 2026-11-20 (04:22 restart; 06:16-10:57 idle usage cut). Pointers replace them on the card (written by Edit). Each
block is found by its first line, once, and runs while the next lines start with two spaces (as card_trim_1120e.py).
Refuses to run twice. Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/card_trim_1120f.py"""
import hashlib

C = ".checkpoints/netcode-proof-test.md"
H = ".checkpoints/netcode-proof-test/history.md"
STARTS = [
    "- **U4f-F1 CLOSED** (04:3x UTC)",
    "- **U4g's brief, draft rev 1 DONE**",
    "- **U4f2's brief: the advisor's check APPROVE WITH CONDITIONS**",
    "- **2026-11-20 between about 06:16 and 10:57:41 UTC",
    "- **2026-11-20 about 04:22:05 to 04:24 UTC",
]
HEAD = "## Card blocks moved 2026-11-20 16:2x UTC during U4f2's build (card_trim_1120f.py; from 'Running work' and 'Cuts')"

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
