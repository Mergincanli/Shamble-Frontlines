import sys
# usage: mk.py ID START END AFTERFILE  (appends a FIX block; BEFORE copied from disk lines START..END)
#        mk.py --note ID REASONFILE
root = "/home/user/Shamble-Frontlines/"
doc = "docs/adr/0006-infrastructure-operations.md"
out = root + ".checkpoints/v2-decision-drafts/r6b-infra-engineer-fixes.md"
if sys.argv[1] == "--note":
    rid, rf = sys.argv[2], sys.argv[3]
    reason = open(rf, encoding="utf-8").read().rstrip("\n")
    with open(out, "a", encoding="utf-8") as f:
        f.write(f"### NOTE {rid}\n{reason}\nEND\n\n")
    sys.exit(0)
rid, s, e, af = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
lines = open(root + doc, encoding="utf-8").read().split("\n")
before = "\n".join(lines[s-1:e])
text = "\n".join(lines)
n = text.count(before)
after = open(af, encoding="utf-8").read().rstrip("\n")
for i, l in enumerate(after.split("\n")):
    if len(l) > 120 and not l.startswith("|"):
        print(f"WARN long line {i+1}: {len(l)}")
if "\nEND\n" in "\n"+after+"\n":
    print("ERROR END line in after"); sys.exit(1)
if n != 1:
    print(f"ERROR before occurs {n} times"); sys.exit(1)
with open(out, "a", encoding="utf-8") as f:
    f.write(f"### FIX {rid}\nFILE: {doc}\nBEFORE:\n{before}\nAFTER:\n{after}\nEND\n\n")
print("ok", rid, s, e)
