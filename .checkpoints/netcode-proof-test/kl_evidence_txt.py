"""PR #53, the advisor's C1 (KLH-1): the five .log evidence files are ignored by .gitignore:7 (*.log), so they were never
committed. Copy them word for word, from .gate/kept-logs-rule/ (the originals), into the notes folder's kl-evidence/
under .txt names that .gitignore does not match: evidence-0.txt, evidence-1.txt, compare-head.txt, gate.txt,
gate-no-smoke.txt. Each copy is checked equal to its original and to the earlier kl-evidence/*.log copy. Prints bytes and
sha256. Refuses to run twice.  Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/kl_evidence_txt.py"""
import hashlib
import os

SRC = "/home/user/Shamble-Frontlines/.gate/kept-logs-rule"
DST = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/kl-evidence"
for n in ("evidence-0", "evidence-1", "compare-head", "gate", "gate-no-smoke"):
    out = f"{DST}/{n}.txt"
    assert not os.path.exists(out), out
    data = open(f"{SRC}/{n}.log", "rb").read()
    assert data == open(f"{DST}/{n}.log", "rb").read(), f"{n}.log: the original and the earlier copy differ"
    with open(out, "wb") as fh:
        fh.write(data)
    assert open(out, "rb").read() == data
    print(f"{n}.txt: {len(data)} bytes, sha256 {hashlib.sha256(data).hexdigest()}")
