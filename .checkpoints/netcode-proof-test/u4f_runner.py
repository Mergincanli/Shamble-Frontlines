"""U4f's lane runner (u4f-brief.md section 1 item 4 and section 12 item 2): make u4f-run.sh from u4r-run.sh
(sha256 e9437ae2..., 117 lines) with only the lane's names changed (the header, the log folder .gate/u4f/, the usage
line), each change counted, and write the unified diff beside it as u4f-run.sh.diff. Refuses to overwrite. Run from the
repo root:  python3 -I .checkpoints/netcode-proof-test/u4f_runner.py"""
import difflib
import hashlib

N = ".checkpoints/netcode-proof-test/"
SRC, DST, DIFF = N + "u4r-run.sh", N + "u4f-run.sh", N + "u4f-run.sh.diff"
SRC_SHA = "e9437ae200d234725b392bc5090e2b802d9938bb8ba5b12c23123c3d607e97d7"
# (old, new, expected count): the header's two names, the log folder (comment and the check's two lines), the usage.
CHANGES = [
    ("# U4r's runner (u4r-brief.md section 4;", "# U4f's runner (u4f-brief.md section 4;", 1),
    ("in the U4r builder's worktree", "in the U4f builder's worktree", 1),
    ("/home/user/Shamble-Frontlines/.gate/u4r/", "/home/user/Shamble-Frontlines/.gate/u4f/", 1),
    ("bash u4r-run.sh", "bash u4f-run.sh", 1),
    ('"$M"/.gate/u4r/*) ;;', '"$M"/.gate/u4f/*) ;;', 1),
    ("the log must be under $M/.gate/u4r/", "the log must be under $M/.gate/u4f/", 1),
]

raw = open(SRC, "rb").read()
assert hashlib.sha256(raw).hexdigest() == SRC_SHA, "u4r-run.sh is not the brief's e9437ae2..."
text = raw.decode("utf-8")
out = text
for old, new, count in CHANGES:
    assert out.count(old) == count, (old, out.count(old))
    out = out.replace(old, new)
left = [ln for ln in out.splitlines() if ".gate/u4r" in ln or "u4r-run.sh" in ln or "U4r builder" in ln]
assert not left, left
try:
    open(DST, "x", encoding="utf-8").write(out)
    open(DIFF, "x", encoding="utf-8").write("".join(difflib.unified_diff(
        text.splitlines(keepends=True), out.splitlines(keepends=True), fromfile="u4r-run.sh", tofile="u4f-run.sh")))
except FileExistsError:
    raise SystemExit("u4f-run.sh or its .diff exists already: not overwritten")
changed = sum(1 for a, b in zip(text.splitlines(), out.splitlines()) if a != b)
print(f"u4f-run.sh: {out.count(chr(10))} lines, {changed} lines changed, sha256 "
      f"{hashlib.sha256(out.encode()).hexdigest()}")
