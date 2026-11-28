"""C62H-BOUND freeze (the advisor's C9; Mustafa's C62H-BOUND-YES of 2026-11-15 20:50 Amsterdam, history.md:2163-2175:
"Record the sha256 of each exact text now"). Reads c62h-bound-texts-v3.md, checks that the posted file
c62h-bound-paste.md is the one he said yes to (sha256 cfac7502...) and holds each frozen block word for word, and writes
c62h-bound-freeze.txt: the sha256 of each exact text (the two checks, the planted bug, the four SERIES rows, the README
sentence; also the plain words), of texts v3 and of the paste. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/c62h_bound_freeze.py"""
import hashlib
import os
import re

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
OUT = f"{N}/c62h-bound-freeze.txt"
assert not os.path.exists(OUT), "already ran"
sha = lambda b: hashlib.sha256(b).hexdigest()
v3raw = open(f"{N}/c62h-bound-texts-v3.md", "rb").read()
paste = open(f"{N}/c62h-bound-paste.md", "rb").read()
assert sha(paste).startswith("cfac7502"), "the paste is not the one he said yes to"
v3 = v3raw.decode("utf-8")
B = {m.group(1): m.group(2) for m in re.finditer(r"^<!-- BEGIN (\S+) -->\n```\n(.*?)\n```\n<!-- END \1 -->$", v3, re.M | re.S)}
FROZEN = ["check-objects", "bite-object", "series-row-snap_lost_unexplained_count",
          "series-row-snap_whole_despite_link_loss_count", "series-row-tick.s.snap_link_lost_count",
          "series-row-tick.s.snap_incomplete", "readme-sentence", "plain-words"]
lines = ["# C62H-BOUND freeze: the exact texts Mustafa said yes to (C62H-BOUND-YES (a), 2026-11-15 20:50 Amsterdam,",
         "# history.md:2163-2175), by c62h_bound_freeze.py. Any change to one of these texts after his yes goes back to",
         "# him before the judged runs. Before the runs, a script shows the built marks.json, SERIES.md and README.md",
         "# hold exactly these texts.",
         f"c62h-bound-paste.md (as posted) {sha(paste)}",
         f"c62h-bound-texts-v3.md {sha(v3raw)}"]
for n in FROZEN:
    assert B[n] in paste.decode("utf-8"), f"{n} is not word for word in the posted paste"
    lines.append(f"{n} {sha(B[n].encode())}")
text = "\n".join(lines) + "\n"
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
print(text, end="")
print(f"{OUT}: sha256 {sha(text.encode())}")
