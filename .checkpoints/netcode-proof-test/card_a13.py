"""Copy the advisor's A13 card item (advisor-u4d2-brief.md, block 8: the text between `(the main session writes it): "`
and the item's last `"`) into the card's "Next steps, in order" as item 4, before U4r's brief (renumbered 5). The words
are read from the ruling's own lines, never retyped; the anchor and the old item's first words are checked unique.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/card_a13.py [--apply]"""
import sys
import textwrap

N = ".checkpoints/netcode-proof-test"
with open(f"{N}/advisor-u4d2-brief.md", encoding="utf-8") as fh:
    src = fh.read().split("\n")
a, b = 583, 589
j = " ".join(l.strip() for l in src[a - 1:b])
start = '(the main session writes it): "'
assert j.count(start) == 1, j.count(start)
text = j[j.index(start) + len(start):j.rindex('"')]
assert text.startswith("**After U4e's merge,") and text.endswith("(§4.11)."), text[:60]
card_path = ".checkpoints/netcode-proof-test.md"
with open(card_path, encoding="utf-8") as fh:
    card = fh.read().split("\n")
old = "4. **U4r's brief:**"
hits = [i for i, l in enumerate(card) if l.startswith(old)]
assert len(hits) == 1, hits
i = hits[0]
item = textwrap.wrap("4. " + text, width=120, subsequent_indent="   ", break_long_words=False,
                     break_on_hyphens=False)
new = card[:i] + item + ["5. " + card[i][3:]] + card[i + 1:]
print("\n".join(item))
print(f"inserted {len(item)} lines before card line {i + 1}; U4r's brief renumbered 5")
if "--apply" in sys.argv:
    with open(card_path, "w", encoding="utf-8") as fh:
        fh.write("\n".join(new))
    print("applied")
