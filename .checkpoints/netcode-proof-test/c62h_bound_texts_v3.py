"""C62H-BOUND texts v3: c62h-bound-texts-v2.md with the advisor's four replacements (advisor-c62h-bound-r2-full.json,
"replacements"), applied by script. Each old string must occur exactly once in its block; afterwards every row block
must still occur exactly once inside series-placement, the README sentence once inside readme-placement, and the check
and bite objects once inside marks-json-placement. Writes c62h-bound-texts-v3.md in v2's form and prints every block's
sha256 (the hashes the freeze records at his yes). Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/c62h_bound_texts_v3.py"""
import hashlib
import json
import os
import re

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
V2, V3 = f"{N}/c62h-bound-texts-v2.md", f"{N}/c62h-bound-texts-v3.md"
assert not os.path.exists(V3), "already ran"
text = open(V2, encoding="utf-8").read()
names = re.findall(r"^<!-- BEGIN (\S+) -->$", text, re.M)
blocks = {}
for n in names:
    m = re.search(rf"^<!-- BEGIN {re.escape(n)} -->\n```\n(.*?)\n```\n<!-- END {re.escape(n)} -->$", text, re.M | re.S)
    blocks[n] = m.group(1)
assert len(blocks) == 11, sorted(blocks)
R = json.load(open(f"{N}/advisor-c62h-bound-r2-full.json", encoding="utf-8"))["replacements"]
assert len(R) == 4
for r in R:
    b = blocks[r["block"]]
    assert b.count(r["old"]) == 1, f"{r['block']}: the old string matches {b.count(r['old'])} times"
    blocks[r["block"]] = b.replace(r["old"], r["new"])
    print(f"applied to {r['block']}: {r['old'][:50]!r} -> {r['new'][:70]!r}")
for row in [n for n in blocks if n.startswith("series-row-")]:
    assert blocks["series-placement"].count(blocks[row]) == 1, f"{row} is not once inside series-placement"
assert blocks["readme-placement"].count(blocks["readme-sentence"]) == 1
for n in ("check-objects", "bite-object"):
    assert blocks["marks-json-placement"].count(blocks[n]) == 1, n
changed = {r["block"] for r in R}
out = ["# C62H-BOUND: the exact texts, v3 (v2 with the advisor's four replacements of its ruling r2, applied by script)",
       "", "Written by c62h_bound_texts_v3.py from c62h-bound-texts-v2.md and advisor-c62h-bound-r2-full.json. Each block "
       "is word for word; nothing here is retyped. The freeze records these blocks' sha256 at Mustafa's yes.", ""]
for n in names:
    out += [f"<!-- BEGIN {n} -->", "```", blocks[n], "```", f"<!-- END {n} -->", ""]
    print(f"{n}: sha256 {hashlib.sha256(blocks[n].encode()).hexdigest()}{'  (changed in v3)' if n in changed else ''}")
with open(V3, "w", encoding="utf-8") as fh:
    fh.write("\n".join(out))
print(f"{V3}: written")
