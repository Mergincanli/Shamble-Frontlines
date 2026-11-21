"""Copy the KEPT-LOGS build's evidence, word for word, from .gate/kept-logs-rule/ (gitignored) into the notes folder's
kl-evidence/ so that it is committed with the checkpoint and can be linked from the pull request: the step logs
(evidence-0.log: quotes, guard, full; evidence-1.log: mutants, dry, kit; compare-head.log: the pushed head against the
rebuilt files), the full run's TAP, the 14 planted bugs' TAP files, the two dry runs' TAP files and the two gate logs.
Not copied: base/ and head/ (the three kit test files' whole TAP, 268 KB each; their counts are in evidence-1.log).
Prints a manifest (bytes, sha256). Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/kl_evidence_copy.py"""
import hashlib
import os

SRC = "/home/user/Shamble-Frontlines/.gate/kept-logs-rule"
DST = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/kl-evidence"
NAMES = (["evidence-0.log", "evidence-1.log", "compare-head.log", "full.tap"] + [f"M{i}.tap" for i in range(1, 15)]
         + ["dry-u4r-precheck-30-faf42aeb034c.tap", "dry-u4r-P3-fd4612050cd6.tap", "gate.log", "gate-no-smoke.log"])
assert not os.path.exists(DST), "already ran"
os.makedirs(DST)
for n in NAMES:
    data = open(f"{SRC}/{n}", "rb").read()
    with open(f"{DST}/{n}", "wb") as fh:
        fh.write(data)
    assert open(f"{DST}/{n}", "rb").read() == data
    print(f"{n}: {len(data)} bytes, sha256 {hashlib.sha256(data).hexdigest()}")
print(f"{len(NAMES)} files copied to {DST}")
