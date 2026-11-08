"""Write one worktree's uncommitted work (tracked diff plus untracked files) as a patch, leaving out .checkpoints/,
and copy its build notes into the main folder. Usage: python3 -I worktree_patch.py <worktree> <out.patch> <notes-prefix>
(throwaway merge helper for the U3c builds; no cd, so the guard sees every write)."""
import os
import shutil
import subprocess
import sys

MAIN = "/home/user/Shamble-Frontlines"
wt, out, prefix = sys.argv[1], sys.argv[2], sys.argv[3]


def git(*args):
    return subprocess.run(["git", *args], cwd=wt, capture_output=True, text=True, check=False).stdout


head = git("rev-parse", "HEAD").strip()
patch = git("diff", "HEAD", "--binary", "--", ".", ":(exclude).checkpoints")
for f in git("ls-files", "--others", "--exclude-standard").splitlines():
    if f.startswith(".checkpoints/"):
        continue
    r = subprocess.run(["git", "diff", "--no-index", "--binary", "/dev/null", f], cwd=wt, capture_output=True, text=True)
    patch += r.stdout
with open(out, "w", encoding="utf-8") as fh:
    fh.write(patch)
notes = os.path.join(wt, ".checkpoints/netcode-proof-test")
copied = []
for n in sorted(os.listdir(notes)) if os.path.isdir(notes) else []:
    if n.startswith(prefix):
        shutil.copy2(os.path.join(notes, n), os.path.join(MAIN, ".checkpoints/netcode-proof-test", n))
        copied.append(n)
files = sorted({l[6:] for l in patch.splitlines() if l.startswith("+++ b/")})
print(f"base {head[:7]}; patch {len(patch)} B; {len(files)} files; notes copied {copied}")
for f in files:
    print("  ", f)
