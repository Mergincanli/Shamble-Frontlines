"""INF2-4's proof files (advisor-u4d2-m9.md, Block 2, P2 and P4): each runner's diff against its committed version,
saved beside it (main-run.sh.inf2-4.diff, u4d2-run.sh.inf2-4.diff); and for each runner and each fail-closed case (a)
`T=` pointed at a missing folder, (b) `LOCK=` in a missing folder, (c) the flock line's fd 9 changed to 99, a copy
<r>.inf2-4-<c>.sh with exactly one line changed (asserted), and its diff <r>.inf2-4-<c>.diff.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/inf2_4_proof.py"""
import difflib
import subprocess

N = ".checkpoints/netcode-proof-test"
CASES = {
    "main-run": {
        "a": ("T=$M/.gate/shared-target", "T=$M/.gate/no-such-target"),
        "b": ("LOCK=$T/.lane.lock", "LOCK=$M/.gate/no-such-dir/.lane.lock"),
        "c": ('flock -x 9 || { echo "STOP: cannot take the lane lock $LOCK"; exit 5; }',
              'flock -x 99 || { echo "STOP: cannot take the lane lock $LOCK"; exit 5; }'),
    },
    "u4d2-run": {
        "a": ("T=$M/.gate/shared-target", "T=$M/.gate/no-such-target"),
        "b": ("LOCK=$T/.lane.lock", "LOCK=$M/.gate/no-such-dir/.lane.lock"),
        "c": ('flock "$mode" 9 || { echo "STOP: cannot take the lane lock $LOCK"; exit 5; }',
              'flock "$mode" 99 || { echo "STOP: cannot take the lane lock $LOCK"; exit 5; }'),
    },
}
for r, cases in CASES.items():
    path = f"{N}/{r}.sh"
    d = subprocess.run(["git", "diff", "--", path], capture_output=True, text=True, check=True).stdout
    assert d, f"{path}: no diff against its committed version"
    with open(f"{path}.inf2-4.diff", "w", encoding="utf-8") as fh:
        fh.write(d)
    added = sum(1 for l in d.split("\n") if l.startswith("+") and not l.startswith("+++"))
    removed = sum(1 for l in d.split("\n") if l.startswith("-") and not l.startswith("---"))
    print(f"P2 {path}.inf2-4.diff: -{removed} +{added}")
    src = open(path, encoding="utf-8").read().split("\n")
    for c, (old, new) in cases.items():
        at = [i for i, l in enumerate(src) if l == old]
        assert len(at) == 1, (r, c, old, at)
        copy = list(src)
        copy[at[0]] = new
        diffs = [i for i, (x, y) in enumerate(zip(src, copy)) if x != y]
        assert diffs == at and len(copy) == len(src), (r, c, diffs)
        cpath = f"{N}/{r}.inf2-4-{c}.sh"
        with open(cpath, "w", encoding="utf-8") as fh:
            fh.write("\n".join(copy))
        ud = "".join(difflib.unified_diff([l + "\n" for l in src], [l + "\n" for l in copy], fromfile=path,
                                          tofile=cpath))
        with open(f"{N}/{r}.inf2-4-{c}.diff", "w", encoding="utf-8") as fh:
            fh.write(ud)
        print(f"P4 copy {cpath}: line {at[0] + 1} changed, exactly one ({old!r} -> {new!r})")
