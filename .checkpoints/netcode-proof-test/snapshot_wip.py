"""Snapshot the U3c build agents' uncommitted work (notes + diffs, untracked files included) into the main folder."""
import os
import shutil
import subprocess

MAIN = "/home/user/Shamble-Frontlines"
OUT = os.path.join(MAIN, ".checkpoints/netcode-proof-test/wip")
OLD = {"agent-a487e419ee806d37b", "agent-a79d7290e0e6a6e50", "agent-a9099feea96fafb39", "agent-adf1deee6caf1481f"}

def git(cwd, *args):
    return subprocess.run(["git", *args], cwd=cwd, capture_output=True, text=True).stdout

os.makedirs(OUT, exist_ok=True)
lines = git(MAIN, "worktree", "list", "--porcelain").splitlines()
paths = [l.split(" ", 1)[1] for l in lines if l.startswith("worktree ")]
for p in paths:
    name = os.path.basename(p)
    if "/.claude/worktrees/" not in p or name in OLD:
        continue
    head = git(p, "rev-parse", "HEAD").strip()
    patch = git(p, "diff", "HEAD", "--binary")
    for f in git(p, "ls-files", "--others", "--exclude-standard").splitlines():
        r = subprocess.run(["git", "diff", "--no-index", "--binary", "/dev/null", f], cwd=p, capture_output=True, text=True)
        patch += r.stdout
    dest = os.path.join(OUT, f"{name}.patch")
    with open(dest, "w", encoding="utf-8") as fh:
        fh.write(f"# WIP snapshot of {p} on base {head}; apply with: git apply <this file> (in a worktree at that base)\n")
        fh.write(patch)
    notes = os.path.join(p, ".checkpoints/netcode-proof-test")
    copied = []
    if os.path.isdir(notes):
        for n in sorted(os.listdir(notes)):
            if n.startswith("u3c-build"):
                shutil.copy2(os.path.join(notes, n), os.path.join(MAIN, ".checkpoints/netcode-proof-test", n))
                copied.append(n)
    print(f"{name}: base {head[:7]}, patch {len(patch)} B, notes copied {copied}")
