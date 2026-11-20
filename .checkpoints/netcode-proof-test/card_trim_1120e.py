"""Move four finished blocks from the card (.checkpoints/netcode-proof-test.md) to history.md's end, word for word,
after U4f-F1's report and -20's L1 record were written (the card at 209 lines, over TEST-FIRST rule 3's 200): the
first -20 and -22 set records (u4f-report.md items 5-6 hold them), U4f-F1's plan line (u4f-f1-plan.md and its report),
the long 'U4f-F1's round CLOSED' block (its round, pass, merge, the -20 re-run and condition 10: u4f-f1-report.md holds
them) and the 01:01 restart's record (F1's report names it). Pointers and a short new running-work line replace them on
the card (written by Edit). Each block is found by its first line, once, and runs while the next lines start with two
spaces (as card_trim_1118.py). Refuses to run twice. Run from the repo root:
python3 -I .checkpoints/netcode-proof-test/card_trim_1120e.py"""
import hashlib

C = ".checkpoints/netcode-proof-test.md"
H = ".checkpoints/netcode-proof-test/history.md"
STARTS = [
    "- **-20 and -22 DONE, all three sets run**",
    "- **U4f-F1** (its own step before U4f2",
    "- **U4f-F1's round CLOSED**",
    "- **2026-11-20 about 01:01 to 01:05 UTC",
]
HEAD = "## Card blocks moved 2026-11-20 after U4f-F1's report and -20's L1 record (card_trim_1120e.py; from 'Now: U4f', 'Running work' and 'Cuts')"

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
