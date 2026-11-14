"""U4r's lane runner, u4r-run.sh, copied from u4d2-run.sh (the advisor's M9 ruling, advisor-u4d2-m9.md Block 2: later
lane runners are copied from u4d2-run.sh with the tag check and the two lock lines unchanged). Only the lane's names
change, each old line found exactly once: the header's first two lines (U4r, its brief, the copy's source), the log
folder `.gate/u4r/` in the header and in the log check, and the usage line. The script asserts that no other line
differs, that every changed line is at most 120 characters, and that the CACHEDIR.TAG check, the two lock lines and
the provenance block are byte-for-byte u4d2-run.sh's; it saves the diff beside the copy (u4r-run.sh.diff).
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4r_runner.py [--apply]"""
import difflib
import sys

N = ".checkpoints/netcode-proof-test"
SRC = f"{N}/u4d2-run.sh"
DST = f"{N}/u4r-run.sh"
CHANGES = [
    ("# U4d2's runner (u4d2-brief.md section 4; W1-TARGET, advisor-u4-wave1.md; the advisor's U4d2 brief check, A2): one",
     "# U4r's runner (u4r-brief.md section 4; W1-TARGET, advisor-u4-wave1.md; u4d2-run.sh copied, advisor-u4d2-m9.md): one"),
    ("# command, or a sequence, in the U4d2 builder's worktree, with the shared target. It takes the shared target's lane lock",
     "# command, or a sequence, in the U4r builder's worktree, with the shared target. It takes the shared target's lane lock"),
    ("# /home/user/Shamble-Frontlines/.gate/u4d2/; each command's exit code and, at the end, the provenance lines",
     "# /home/user/Shamble-Frontlines/.gate/u4r/; each command's exit code and, at the end, the provenance lines"),
    ("# Usage: bash u4d2-run.sh [-x] [-k] <worktree> <log file> <command...> [:: <command...>]...",
     "# Usage: bash u4r-run.sh [-x] [-k] <worktree> <log file> <command...> [:: <command...>]..."),
    ('  "$M"/.gate/u4d2/*) ;;',
     '  "$M"/.gate/u4r/*) ;;'),
    ('  *) echo "STOP: the log must be under $M/.gate/u4d2/ (got: $log)"; exit 2 ;;',
     '  *) echo "STOP: the log must be under $M/.gate/u4r/ (got: $log)"; exit 2 ;;'),
]
KEEP = [
    'if [ ! -f "$T/CACHEDIR.TAG" ]; then',
    '  echo "STOP: $T has no CACHEDIR.TAG (missing, or not cargo\'s own folder); no runner makes it"',
    'exec 9>"$LOCK" || { echo "STOP: cannot open the lane lock $LOCK"; exit 5; }',
    'flock "$mode" 9 || { echo "STOP: cannot take the lane lock $LOCK"; exit 5; }',
    'prov=$(grep -E "^ *(Compiling|Checking|Documenting) npt-" "$log" | sed \'s/^ *//\' | sort | uniq -c)',
]
with open(SRC, encoding="utf-8") as fh:
    src = fh.read().split("\n")
out = list(src)
changed = []
for old, new in CHANGES:
    at = [i for i, l in enumerate(src) if l == old]
    assert len(at) == 1, (old, at)
    assert len(new) <= 120, (len(new), new)
    out[at[0]] = new
    changed.append(at[0])
diffs = [i for i, (a, b) in enumerate(zip(src, out)) if a != b]
assert sorted(diffs) == sorted(changed) and len(out) == len(src), (diffs, changed)
for k in KEEP:
    assert src.count(k) == 1 and out.count(k) == 1 and src.index(k) == out.index(k), k
body = "\n".join(out)
assert ".gate/u4d2/" not in body and "bash u4d2-run.sh" not in body and "U4d2 builder" not in body, "U4d2's lane left"
ud = "".join(difflib.unified_diff([l + "\n" for l in src], [l + "\n" for l in out], fromfile=SRC, tofile=DST))
print(ud, end="")
print(f"{len(changed)} lines changed (lines {', '.join(str(i + 1) for i in sorted(changed))}); the tag check, the two "
      "lock lines and the provenance line unchanged")
if "--apply" in sys.argv:
    with open(DST, "w", encoding="utf-8") as fh:
        fh.write("\n".join(out))
    with open(f"{DST}.diff", "w", encoding="utf-8") as fh:
        fh.write(ud)
    print(f"applied: {DST} and {DST}.diff written")
else:
    print("dry run: nothing written")
