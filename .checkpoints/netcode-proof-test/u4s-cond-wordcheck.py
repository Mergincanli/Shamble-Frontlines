# Word-for-word check of the ruling's texts against the spec (netcode-architect, 2026-11-09).
# Usage: python3 -I u4s-cond-wordcheck.py <advisor-u4.md> <spec>
import re
import sys

adv = open(sys.argv[1], encoding="utf-8").read().split("\n")
spec = " ".join(open(sys.argv[2], encoding="utf-8").read().split())


def norm(lines):
    return " ".join(" ".join(lines).split())


ruling_d4 = norm(adv[354:360])  # advisor-u4.md:355-360
m = re.search(r'\*\*New D-4:\*\* "(.*?)"', ruling_d4)
d4 = m.group(1)
print("D-4 found in spec:", d4 in spec, len(d4))

c3 = norm(adv[327:333])  # advisor-u4.md:328-333
c3 = c3.split("**Freeze the seam's shapes for wave 1.** ", 1)[1]
c3 = c3.split(" 4. **The wave-1 briefs", 1)[0]
print("cond 3 found in spec:", ("**Freeze the seam's shapes for wave 1.** " + c3) in spec, len(c3))
