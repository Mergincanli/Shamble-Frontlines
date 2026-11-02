#!/usr/bin/env python3
# usage: blk.py <heading> <doc 0001|0002> <first-line-substring> <nlines> <subs.py>
# subs.py defines SUBS = [(old, new), ...] applied to BEFORE, each old exactly once; or AFTER = "..." whole.
import sys, os, runpy
D = os.path.dirname(os.path.abspath(__file__))
OUT = "/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/test-engineer-A-fixes-final.md"
PATHS = {"0001": "docs/adr/0001-multiplayer-netcode.md", "0002": "docs/adr/0002-code-structure-build-toolchain.md"}
head, doc, first, n, spec = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4]), sys.argv[5]
orig = open(f"{D}/orig-{doc}.md").read()
work = open(f"{D}/work-{doc}.md").read()
lines = orig.split("\n")
idx = [i for i, l in enumerate(lines) if first in l]
assert len(idx) == 1, f"first-line substring matches {len(idx)} lines"
i = idx[0]
before = "\n".join(lines[i:i+n])
assert orig.count(before + "\n") == 1, "BEFORE not unique in orig"
assert work.count(before + "\n") == 1, "BEFORE not unique in working copy (overlap?)"
g = runpy.run_path(spec)
if "AFTER" in g:
    after = g["AFTER"].strip("\n")
else:
    after = before
    for old, new in g["SUBS"]:
        c = after.count(old)
        assert c == 1, f"sub old occurs {c} times: {old[:80]!r}"
        after = after.replace(old, new)
assert after != before
work = work.replace(before + "\n", after + "\n", 1)
open(f"{D}/work-{doc}.md", "w").write(work)
blk = f"### FIX {head}\nFILE: {PATHS[doc]}\nBEFORE:\n{before}\nAFTER:\n{after}\nEND\n\n"
open(OUT, "a").write(blk)
print("ok", head, "lines", i+1, "to", i+n, "after lines", after.count("\n")+1)
