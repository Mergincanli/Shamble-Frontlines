"""FP-0's proof files (the advisor's ruling on U4r round 1, advisor-u4r-r1-full.md item 2, INF-U4R-4): each runner's
diff against its committed version, saved beside it in INF2-4's form (`git diff -- <runner>`, as inf2_4_proof.py
wrote main-run.sh.inf2-4.diff): u4r-run.sh.inf-u4r-4.diff and main-run.sh.inf-u4r-4.diff. Asserted: exactly three
lines change in each runner, the df read, the log's free line and the floor's test, each from the ruled old text to
the ruled new text ("No other runner line changes"); then `bash -n` on each runner prints nothing and exits 0.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/fp0_runner_diffs.py"""
import subprocess

N = ".checkpoints/netcode-proof-test"
OLD_DF = "avail=$(df -BG --output=avail /home/user | tail -1 | tr -dc 0-9)"
NEW_DF = "avail=$(df -B1 --output=avail /home/user | tail -1 | tr -dc 0-9)"
OLD_IF = 'if [ "$avail" -lt 4 ]; then'
NEW_IF = 'if [ "$avail" -lt $((4*1024*1024*1024)) ]; then'
FREE = {
    "u4r-run": ('  echo "pwd $(pwd); free ${avail} GB; HEAD $(git rev-parse --short HEAD); CARGO_TARGET_DIR '
                '$CARGO_TARGET_DIR"',
                '  echo "pwd $(pwd); free $((avail/1048576)) MiB; HEAD $(git rev-parse --short HEAD); CARGO_TARGET_DIR '
                '$CARGO_TARGET_DIR"'),
    "main-run": ('  echo "pwd $(pwd); free ${avail} GB; HEAD $(cd "$M" && git rev-parse --short HEAD)"',
                 '  echo "pwd $(pwd); free $((avail/1048576)) MiB; HEAD $(cd "$M" && git rev-parse --short HEAD)"'),
}
for r, (old_free, new_free) in FREE.items():
    path = f"{N}/{r}.sh"
    d = subprocess.run(["git", "diff", "--", path], capture_output=True, text=True, check=True).stdout
    assert d, f"{path}: no diff against its committed version"
    removed = sorted(l[1:] for l in d.split("\n") if l.startswith("-") and not l.startswith("---"))
    added = sorted(l[1:] for l in d.split("\n") if l.startswith("+") and not l.startswith("+++"))
    assert removed == sorted([OLD_DF, old_free, OLD_IF]), (path, removed)
    assert added == sorted([NEW_DF, new_free, NEW_IF]), (path, added)
    with open(f"{path}.inf-u4r-4.diff", "w", encoding="utf-8") as fh:
        fh.write(d)
    b = subprocess.run(["bash", "-n", path], capture_output=True, text=True)
    assert b.returncode == 0 and b.stdout == "" and b.stderr == "", (path, b.returncode, b.stdout, b.stderr)
    print(f"{path}.inf-u4r-4.diff: -{len(removed)} +{len(added)}, the three ruled lines only; bash -n: exit 0, "
          "printed nothing")
