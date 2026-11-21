"""Print sha256's first 16 hex and the line count of each file U5's brief reads (u5-brief.md:37-43, "Read, never
retyped") and the U5-EARLY ruling, for U5's early fill (advisor-u5-early.md condition 8). Reads only. Run from the
repo root: python3 -I .checkpoints/netcode-proof-test/stamps_u5_early.py"""
import hashlib

N = ".checkpoints/netcode-proof-test/"
S = "spikes/netcode-proof/"
FILES = [
    N + "plan.md", N + "u4-build-plan.md", N + "u4s-carries.md", S + "marks/marks.json", S + "marks/SERIES.md",
    S + "marks/README.md", "docs/design/netcode-proof-messages.md", S + "Cargo.toml", S + "deny.toml", S + "check.mjs",
    N + "u0-probes.md", ".claude/CHECKPOINTS.md", N + "advisor-u5-brief.md", N + "u5-alloc-bounds.md",
    N + "advisor-u5-early.md",
]
NL = b"\n"
for f in FILES:
    b = open(f, "rb").read()
    n = b.count(NL)
    print(f"{f}: {hashlib.sha256(b).hexdigest()[:16]}, {n} lines")
