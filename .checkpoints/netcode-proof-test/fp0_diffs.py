"""BRAWL-1 FP-0 (advisor-brawl1-r1-full.md, step 2): save each runner's one-line change as a .diff beside it, in
INF2-4's form (`git diff` of the runner), word for word from the commit that holds the edit (7402dac, the autosave of
05:41 UTC; the Edits were made at 05:33). Asserted per runner: exactly one added line, equal to the ruled line, and no
removed line. Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/fp0_diffs.py"""
import hashlib
import subprocess

ROOT = "/home/user/Shamble-Frontlines"
N = ".checkpoints/netcode-proof-test"
COMMIT = "7402dac"
LINE = ("  echo \"diff $(git diff HEAD -- . | sha256sum | cut -d' ' -f1); untracked $(git status --porcelain "
        "--untracked-files=all -- . | sha256sum | cut -d' ' -f1)\"")
for runner in ("u4r-run.sh", "main-run.sh"):
    path = f"{N}/{runner}"
    d = subprocess.run(["git", "diff", f"{COMMIT}~1", COMMIT, "--", path], cwd=ROOT, capture_output=True, text=True,
                       check=True).stdout
    added = [l[1:] for l in d.split("\n") if l.startswith("+") and not l.startswith("+++")]
    removed = [l for l in d.split("\n") if l.startswith("-") and not l.startswith("---")]
    assert added == [LINE] and not removed, (runner, added, removed)
    out = f"{ROOT}/{path}.te-b1-7.diff"
    with open(out, "w", encoding="utf-8") as fh:
        fh.write(d)
    with open(out, "rb") as fh:
        dsha = hashlib.sha256(fh.read()).hexdigest()
    with open(f"{ROOT}/{path}", "rb") as fh:
        rsha = hashlib.sha256(fh.read()).hexdigest()
    print(f"{runner}.te-b1-7.diff: {len(d.splitlines())} lines, sha256 {dsha}; {runner} now sha256 {rsha}")
