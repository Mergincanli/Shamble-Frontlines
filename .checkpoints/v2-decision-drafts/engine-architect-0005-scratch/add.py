import sys, pathlib
# usage: add.py <doc copy> <out> <id> <a> <b> <after-file>
doc = pathlib.Path(sys.argv[1]).read_text(); lines = doc.split("\n")
out = pathlib.Path(sys.argv[2]); fid = sys.argv[3]; a, b = int(sys.argv[4]), int(sys.argv[5])
after = pathlib.Path(sys.argv[6]).read_text().strip("\n")
before = "\n".join(lines[a-1:b])
assert doc.count(before) == 1, ("not unique", doc.count(before))
if not out.exists():
    out.write_text("# engine-architect, ADR 0005 follow-up batch b (frame and content data): fixes on d57dc5a\n\n"
                   "Routed from game-designer's ability-section batch (d57dc5a; its NOTEs RR-4 and GD-7). BEFORE blocks are whole\n"
                   "lines copied by script from docs/adr/0005 at d57dc5a; each occurs exactly once and no two overlap.\n\n")
with out.open("a") as f:
    f.write(f"### FIX {fid}\nFILE: docs/adr/0005-content-abilities-assets.md\nBEFORE:\n{before}\nAFTER:\n{after}\nEND\n\n")
print("added", fid, a, b)
