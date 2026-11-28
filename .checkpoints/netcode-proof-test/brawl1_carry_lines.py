"""Append to u4r-report.md's BRAWL-1 section, word for word, the carry line EVQ-LOAD (advisor-brawl1-sets-p3-full.md
line 158) and the corrected TICK16-WORDING (advisor-brawl1-sets-p0-full.md line 135). Both files' sha256 and both
lines' bounds are asserted; only the headings and the source lines are the script's. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/brawl1_carry_lines.py"""
import hashlib

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
SRC = [(f"{N}/advisor-brawl1-sets-p3-full.md", "1bc5889c9b05443f5bf62716fa14eb4f817f7a941418cb1fcf8acb5f6dc3dcfb", 158,
        '"With everyone in view', 'judged B1 runs count."', "EVQ-LOAD, the carry line for U4f's and U4g's briefs"),
       (f"{N}/advisor-brawl1-sets-p0-full.md", "4b16dab5890a1d93b586f6c9a5d113d16bfee112f9b6880baff6c951d66a9cf1", 135,
        "\"tick16's run is valid", 'was acknowledged."', "TICK16-WORDING, corrected (it replaces the line at :562-564)")]
REPORT = f"{N}/u4r-report.md"
add = []
for path, digest, n, start, end, head in SRC:
    with open(path, "rb") as fh:
        assert hashlib.sha256(fh.read()).hexdigest() == digest, path
    with open(path, encoding="utf-8") as fh:
        line = fh.read().split("\n")[n - 1]
    assert line.startswith(start) and line.endswith(end), (path, line[:60], line[-60:])
    name = path.rsplit("/", 1)[1]
    add += ["", f"### {head}", "", f"_Copied by brawl1_carry_lines.py, word for word, from {name} line {n} (sha256 "
            f"{digest[:8]}...)._", "", line]
with open(REPORT, encoding="utf-8") as fh:
    text = fh.read()
assert "### EVQ-LOAD, the carry line" not in text, "already appended"
new = text.rstrip("\n") + "\n" + "\n".join(add) + "\n"
with open(REPORT, "w", encoding="utf-8") as fh:
    fh.write(new)
with open(REPORT, "rb") as fh:
    print(f"{REPORT}: +{len(add)} lines, sha256 {hashlib.sha256(fh.read()).hexdigest()}")
