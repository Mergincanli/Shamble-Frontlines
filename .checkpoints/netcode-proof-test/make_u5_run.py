"""Make U5's runner u5-run.sh from u4f-run.sh (advisor-u5-early.md condition 10 and section 6: "the runner copy made as
u4f-run.sh was", u4f-brief.md:306-309), changing only the lane's names: the header's runner name and builder, the log
folder .gate/u4f/ -> .gate/u5/, the usage line. Each change is asserted by count; the unified diff is written beside it
as u5-run.sh.diff; the new file's sha256 is printed. Refuses to overwrite. Run from the repo root:
python3 -I .checkpoints/netcode-proof-test/make_u5_run.py"""
import difflib
import hashlib
import os

N = ".checkpoints/netcode-proof-test/"
SRC, DST, DIFF = N + "u4f-run.sh", N + "u5-run.sh", N + "u5-run.sh.diff"
assert not os.path.exists(DST), "u5-run.sh exists"
src = open(SRC, encoding="utf-8").read()
assert hashlib.sha256(src.encode()).hexdigest().startswith("70eb3da5b8ce9a8e"), "u4f-run.sh is not the recorded one"
CHANGES = [
    ("# U4f's runner (", "# U5's runner, copied from u4f-run.sh for U5-EARLY (advisor-u5-early.md condition 10) (", 1),
    ("in the U4f builder's worktree", "in the U5 builder's worktree", 1),
    ("/home/user/Shamble-Frontlines/.gate/u4f/;", "/home/user/Shamble-Frontlines/.gate/u5/;", 1),
    ("# Usage: bash u4f-run.sh ", "# Usage: bash u5-run.sh ", 1),
    ('"$M"/.gate/u4f/*) ;;', '"$M"/.gate/u5/*) ;;', 1),
    ("the log must be under $M/.gate/u4f/", "the log must be under $M/.gate/u5/", 1),
]
out = src
for old, new, count in CHANGES:
    assert out.count(old) == count, (old, out.count(old))
    out = out.replace(old, new)
assert "u4f" not in out.replace("u4f-run.sh", "").replace("u4f-brief.md", ""), "a u4f name is left"
open(DST, "w", encoding="utf-8").write(out)
os.chmod(DST, os.stat(SRC).st_mode)
diff = "".join(difflib.unified_diff(src.splitlines(True), out.splitlines(True), SRC, DST))
open(DIFF, "w", encoding="utf-8").write(diff)
print(f"u5-run.sh {hashlib.sha256(out.encode()).hexdigest()}; {out.count(chr(10))} lines; diff {DIFF} "
      f"({diff.count(chr(10))} lines, {len(CHANGES)} changes)")
