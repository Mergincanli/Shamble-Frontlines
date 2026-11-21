"""Print sha256's first 16 hex and the line count of each file U4g's brief reads (u4g-brief.md:32-40), the U4G-EARLY
ruling and U4f2's patch, for U4g's early fill (advisor-u4g-early.md condition 7); the runner's full sha256. Reads
only. Run from the repo root: python3 -I .checkpoints/netcode-proof-test/stamps_u4g_early.py"""
import hashlib

N = ".checkpoints/netcode-proof-test/"
S = "spikes/netcode-proof/"
FILES = [
    N + "advisor-u4g-brief.md", N + "advisor-u4r-build3-full.md", N + "u4-build-plan.md", N + "u4s-carries.md",
    N + "plan.md", N + "u4f-brief.md", N + "u4f2-brief.md", N + "advisor-u4f-stop.md", N + "u4f-report.md",
    N + "u4r-report.md", N + "advisor-u4f-f1-r1.md", N + "advisor-u4f-build1.md", N + "u4g-brief-carry.md",
    N + "advisor-brawl1-sets-p3-full.md", N + "c62h-bound-brief.md", N + "history.md", S + "marks/marks.json",
    S + "marks/SERIES.md", "docs/adr/0001-multiplayer-netcode.md", N + "advisor-u4g-early.md",
    N + "wip/u4f2-build.patch",
]
NL = b"\n"
for f in FILES:
    b = open(f, "rb").read()
    n = b.count(NL)
    print(f"{f}: {hashlib.sha256(b).hexdigest()[:16]}, {n} lines")
r = open(N + "u4f-run.sh", "rb").read()
print(f"{N}u4f-run.sh: {hashlib.sha256(r).hexdigest()}")
