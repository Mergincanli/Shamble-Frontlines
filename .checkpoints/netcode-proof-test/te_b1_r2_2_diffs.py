"""TE-B1-R2-2 (advisor-brawl1-r2-full.md; due after BRAWL-1's last re-run set, before U4g's first run): save each
runner's one-line change as a .diff beside it, in INF2-4's form (`git diff` of the runner against ec90bc3, the close
commit before the edit). Asserted per runner: one removed line holding the porcelain form and one added line holding
the ruled ls-files form, nothing else. Run from the repo root:
python3 -I .checkpoints/netcode-proof-test/te_b1_r2_2_diffs.py"""
import hashlib
import subprocess

ROOT = "/home/user/Shamble-Frontlines"
N = ".checkpoints/netcode-proof-test"
BASE = "ec90bc3"
NEW = "untracked $(git ls-files -z --others --exclude-standard -- . | xargs -0 -r sha256sum | sha256sum | cut -d' ' -f1)"
OLD = "untracked $(git status --porcelain --untracked-files=all -- . | sha256sum | cut -d' ' -f1)"
for runner in ("u4r-run.sh", "main-run.sh"):
    path = f"{N}/{runner}"
    d = subprocess.run(["git", "diff", BASE, "--", path], cwd=ROOT, capture_output=True, text=True, check=True).stdout
    added = [l[1:] for l in d.split("\n") if l.startswith("+") and not l.startswith("+++")]
    removed = [l[1:] for l in d.split("\n") if l.startswith("-") and not l.startswith("---")]
    assert len(added) == 1 and len(removed) == 1 and NEW in added[0] and OLD in removed[0], (runner, added, removed)
    assert added[0].replace(NEW, OLD) == removed[0], runner
    out = f"{ROOT}/{path}.te-b1-r2-2.diff"
    with open(out, "w", encoding="utf-8") as fh:
        fh.write(d)
    with open(out, "rb") as fh:
        dsha = hashlib.sha256(fh.read()).hexdigest()
    with open(f"{ROOT}/{path}", "rb") as fh:
        rsha = hashlib.sha256(fh.read()).hexdigest()
    print(f"{runner}.te-b1-r2-2.diff sha256 {dsha[:12]}; {runner} now sha256 {rsha}")
