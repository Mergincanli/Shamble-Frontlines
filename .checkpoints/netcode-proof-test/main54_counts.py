"""MAIN-54, the advisor's C4: read N (tracked files, `git ls-files`) and M (`git ls-files spikes/netcode-proof`) just
before the light check, and show that `git status --short --ignored -- spikes/netcode-proof` prints nothing.
Read-only; writes nothing. Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/main54_counts.py"""
import subprocess


def git(*args):
    return subprocess.run(["git", *args], capture_output=True, text=True, check=True).stdout


n = len(git("ls-files").splitlines())
m = len(git("ls-files", "spikes/netcode-proof").splitlines())
st = git("status", "--short", "--ignored", "--", "spikes/netcode-proof")
head = git("rev-parse", "HEAD").strip()
print(f"HEAD {head}")
print(f"N (git ls-files) = {n}")
print(f"M (git ls-files spikes/netcode-proof) = {m}")
print(f"git status --short --ignored -- spikes/netcode-proof: {'empty' if st == '' else repr(st)}")
