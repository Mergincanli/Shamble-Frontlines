"""Copy T4 (advisor-u4df.md, "### T4.", its code block) into the card's "Rules in force", after the U4j round-2
ruling's B6 bullet, by line range, never retyped. Usage: python3 -I t4_card.py [--dry]."""
import sys

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
RULING = f"{N}/advisor-u4df.md"
CARD = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test.md"
ANCHOR = "    agents get the Edit tool, or are told to rewrite their notes whole with Write from the start.\n"

ruling = open(RULING, encoding="utf-8").read().splitlines(keepends=True)
heads = [i for i, l in enumerate(ruling) if l.startswith("### T4.")]
assert len(heads) == 1, heads
start = next(i for i in range(heads[0] + 1, len(ruling)) if ruling[i].startswith("```"))
end = next(i for i in range(start + 1, len(ruling)) if ruling[i].startswith("```"))
block = ruling[start + 1:end]
assert block and block[0].startswith("- **The advisor's U4d-F ruling"), block[:1]
print(f"T4 block: ruling lines {start + 2}-{end} ({len(block)} lines)")

card = open(CARD, encoding="utf-8").read()
assert card.count(ANCHOR) == 1, card.count(ANCHOR)
assert "The advisor's U4d-F ruling (advisor-u4df.md, R1, R4.1 and R6)" not in card, "already applied"
new = card.replace(ANCHOR, ANCHOR + "".join(block), 1)
for line in "".join(block).splitlines():
    assert len(line) <= 120, (len(line), line)
assert "".join(block) in new
if "--dry" in sys.argv:
    print("dry run: OK")
else:
    open(CARD, "w", encoding="utf-8").write(new)
    print("applied")
