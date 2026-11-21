#!/usr/bin/env python3
"""File the advisor's U6 brief rulings (advisor-u6-brief.md:179-224: U6-Q2, Q4, Q13, Q15 to Q18, Q21) as register
entries 112-119 in u4s-carries.md's U6 section, word for word, each with its line cite. Run once; refuses a second run."""
import sys

D = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/"
ADV = D + "advisor-u6-brief.md"
REG = D + "u4s-carries.md"

adv = open(ADV, encoding="utf-8").read().split("\n")
# 1-based lines 179..224 hold the eight rulings, each a bullet starting "- **U6-Q".
lines = adv[178:224]
if not lines[0].startswith("- **U6-Q2 ") or not lines[-1].startswith("  throwing or rejoining"):
    sys.exit("refused: advisor-u6-brief.md:179-224 is not the rulings block it was")
bullets = []
for off, l in enumerate(lines):
    n = 179 + off
    if l.startswith("- **U6-Q"):
        bullets.append([n, n, [l[2:]]])
    elif l.startswith("  ") and bullets:
        bullets[-1][1] = n
        bullets[-1][2].append(l[2:])
    else:
        sys.exit("refused: unexpected line %d: %r" % (n, l))
names = [b[2][0].split(" (")[0].replace("**", "") for b in bullets]
if names != ["U6-Q2", "U6-Q4", "U6-Q13", "U6-Q15", "U6-Q16", "U6-Q17", "U6-Q18", "U6-Q21"]:
    sys.exit("refused: the rulings are %r" % names)

reg = open(REG, encoding="utf-8").read()
if "112. **" in reg:
    sys.exit("refused: entry 112 already exists")
anchor = "     what counts).\"\n\n## For the advisor\n"
if reg.count(anchor) != 1:
    sys.exit("refused: entry 111's end and the next heading are not where they were")
out = []
for i, (a, b, text) in enumerate(bullets):
    num = 112 + i
    out.append("%d. **%s, the advisor's ruling on U6's brief (advisor-u6-brief.md:%d-%d; filed 2026-11-21 with U6's brief rev 2,"
               % (num, names[i], a, b))
    out.append("     its condition 8), word for word:**")
    for t in text:
        out.append("     " + t)
block = "\n".join(out) + "\n"
reg = reg.replace(anchor, "     what counts).\"\n" + block + "\n## For the advisor\n")
open(REG, "w", encoding="utf-8").write(reg)
print("filed entries 112-%d (%d lines)" % (111 + len(bullets), len(out)))
