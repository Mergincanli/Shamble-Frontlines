"""Refresh the checkpoint's resume card at Mustafa's HOLD (2026-11-11 about 16:15 UTC): move the card's body (from the
line after "## Resume card" to the line before "## Steps done") verbatim to the end of history.md, under "Running work,
U4 wave 1 (2)", and put the new card text in its place.
  python3 -I card_refresh2.py <repo root> <new card text file>
It checks that every moved line is in history.md afterwards."""
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

heads = [i for i, l in enumerate(cp) if l.startswith("## Resume card")]
ends = [i for i, l in enumerate(cp) if l == "## Steps done"]
assert len(heads) == 1 and len(ends) == 1 and heads[0] < ends[0], (heads, ends)
assert "## Running work, U4 wave 1 (2)" not in "\n".join(hi), "already moved"

moved = cp[heads[0] + 1:ends[0]]
while moved and moved[-1] == "":
    moved.pop()
while hi and hi[-1] == "":
    hi.pop()
hi += ["", "## Running work, U4 wave 1 (2): the resume card of 2026-11-11 morning (moved at Mustafa's HOLD, about 16:20 "
       "UTC, by card_refresh2.py)", "_The card's body as it stood at that moment, verbatim._", ""] + moved + [""]
cp = cp[:heads[0] + 1] + card + [""] + cp[ends[0]:]

assert "\n".join(moved) in "\n".join(hi)
for path, doc in [(hi_path, hi), (cp_path, cp)]:
    with open(path, "w", encoding="utf-8") as fh:
        fh.write("\n".join(doc))
print(f"moved {len(moved)} lines to history.md; the new card has {len(card)} lines")
