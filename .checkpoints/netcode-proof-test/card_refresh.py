"""Refresh the checkpoint's resume card (2026-11-11): move the card's blocks of 2026-11-10 (the wave-1 list, NOW, SAVE,
SAVE POINT and the next three actions of then: lines 11-184) verbatim to the end of history.md, under "Running work,
U4 wave 1", and put the new card text in their place.
  python3 -I card_refresh.py <repo root> <new card text file>
It checks the block's first and last lines before moving, and that every moved line is in history.md afterwards."""
import sys

root, card_path = sys.argv[1], sys.argv[2]
cp_path = f"{root}/.checkpoints/netcode-proof-test.md"
hi_path = f"{root}/.checkpoints/netcode-proof-test/history.md"

with open(cp_path, encoding="utf-8") as fh:
    cp = fh.read().split("\n")
with open(hi_path, encoding="utf-8") as fh:
    hi = fh.read().split("\n")
with open(card_path, encoding="utf-8") as fh:
    card = fh.read().rstrip("\n").split("\n")

first, last = 11, 184
assert cp[first - 1].startswith("**The current step: U4 wave 1**"), cp[first - 1]
assert cp[last - 1].startswith("   - The brief itself goes to the advisor after wave 1 merges"), cp[last - 1]
assert cp[last] == "" and cp[last + 1].startswith("**Open questions for Mustafa:**"), cp[last + 1]
assert "## Running work, U4 wave 1" not in "\n".join(hi), "already moved"

moved = cp[first - 1:last]
while hi and hi[-1] == "":
    hi.pop()
hi += ["", "## Running work, U4 wave 1: the resume card's blocks of 2026-11-10 (moved 2026-11-11 by card_refresh.py)",
       "_Lines 11-184 of the checkpoint at 6486e6e, verbatim._", ""] + moved + [""]
cp = cp[:first - 1] + card + cp[last:]

text = "\n".join(hi)
assert "\n".join(moved) in text
for path, doc in [(hi_path, hi), (cp_path, cp)]:
    with open(path, "w", encoding="utf-8") as fh:
        fh.write("\n".join(doc))
print(f"moved {len(moved)} lines to history.md; the card has {len(card)} lines")
