"""Copy the advisor's eight register lines (94-101) for U4h, word for word, from advisor-u4h-brief.md's "### The register
lines" text block into u4s-carries.md's U4h section, right after entry 93's block (before entry 28), as the advisor's
U4h brief check asks (its rev-2 verdict, condition 4). Refuses to run twice. Run from the repo root:
python3 -I .checkpoints/netcode-proof-test/add_u4h_register.py"""
import hashlib

A = ".checkpoints/netcode-proof-test/advisor-u4h-brief.md"
R = ".checkpoints/netcode-proof-test/u4s-carries.md"
HEAD = "### The register lines (u4s-carries.md, U4h section, after entry 93; word for word)"

adv = open(A, encoding="utf-8").read().split("\n")
h = [i for i, l in enumerate(adv) if l == HEAD]
assert len(h) == 1, h
assert adv[h[0] + 1] == "```text", adv[h[0] + 1]
end = h[0] + 2
while adv[end] != "```":
    end += 1
lines = adv[h[0] + 2:end]
assert lines[0].startswith("94. ") and any(l.startswith("101. ") for l in lines), (lines[0], lines[-1])

reg = open(R, encoding="utf-8").read().split("\n")
assert not any(l.startswith("94. **N2-L3 (b)") for l in reg), "already added"
s93 = [i for i, l in enumerate(reg) if l.startswith("93. **The brawl's ES hit rule")]
assert len(s93) == 1, s93
nxt = s93[0] + 1
while reg[nxt].startswith("    "):
    nxt += 1
assert reg[nxt].startswith("28. **netcode F6"), reg[nxt]
new = reg[:nxt] + lines + reg[nxt:]
open(R, "w", encoding="utf-8").write("\n".join(new))
block = "\n".join(lines)
print(f"u4s-carries.md: {len(reg)} -> {len(new)} lines; inserted {len(lines)} lines at :{nxt + 1}-{nxt + len(lines)} "
      f"(after entry 93's block, before entry 28); copied from advisor-u4h-brief.md:{h[0] + 3}-{end}; "
      f"block sha256 {hashlib.sha256(block.encode()).hexdigest()[:16]}")
