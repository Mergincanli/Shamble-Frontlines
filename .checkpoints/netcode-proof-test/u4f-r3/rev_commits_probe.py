"""Read-only probe: for every commit that touched u4f-brief.md, print its short hash, time, the brief's sha256 and line
count at that commit, so the round-2 checks get the exact rev 3 text (the draft's sha256 efedb237...) and rev 3.1's
(738dc680...). Writes nothing. Run from the repo root:
  python3 -I .checkpoints/netcode-proof-test/u4f-r3/rev_commits_probe.py"""
import hashlib
import subprocess

PATH = ".checkpoints/netcode-proof-test/u4f-brief.md"
log = subprocess.run(["git", "log", "--format=%h %cI", "-12", "--", PATH], capture_output=True, text=True, check=True)
for line in log.stdout.splitlines():
    h, when = line.split(" ", 1)
    blob = subprocess.run(["git", "show", f"{h}:{PATH}"], capture_output=True, check=True).stdout
    print(h, when, hashlib.sha256(blob).hexdigest()[:16], blob.count(b"\n"), "lines")
