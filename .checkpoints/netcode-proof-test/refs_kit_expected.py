"""REFS-KIT (the advisor's ruling, advisor-refs-clash-full.json): build the expected files by script, so the main
session's Edit-tool edits can be compared byte for byte. Takes HEAD's spikes/netcode-proof/check.mjs,
spikes/netcode-proof/check.test.mjs and .checkpoints/netcode-proof-test/plan.md, applies the ruling's six replacements
(fix.check_mjs, fix.check_test_mjs, fix.plan_md; each old text must occur exactly once), and writes the results under the
session scratchpad's refs-kit/expected/, printing each sha256. Also writes the three planted bugs' copies of the expected
check.mjs (P1 broad skip, P2 whole-file skip, P3 no exception) next to it. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/refs_kit_expected.py"""
import hashlib
import json
import os
import subprocess

REPO = "/home/user/Shamble-Frontlines"
OUT = "/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/scratchpad/refs-kit"
assert not os.path.exists(OUT), "already ran"
R = json.load(open(f"{REPO}/.checkpoints/netcode-proof-test/advisor-refs-clash-full.json", encoding="utf-8"))["fix"]
FILES = {"spikes/netcode-proof/check.mjs": R["check_mjs"], "spikes/netcode-proof/check.test.mjs": R["check_test_mjs"],
         ".checkpoints/netcode-proof-test/plan.md": R["plan_md"]}
sha = lambda b: hashlib.sha256(b).hexdigest()
for path, reps in FILES.items():
    text = subprocess.run(["git", "show", f"HEAD:{path}"], cwd=REPO, capture_output=True, check=True).stdout.decode("utf-8")
    for r in reps:
        assert text.count(r["old"]) == 1, f"{path}: an old text matches {text.count(r['old'])} times: {r['old'][:60]!r}"
        text = text.replace(r["old"], r["new"])
    for r in reps:
        assert text.count(r["new"]) == 1, f"{path}: a new text is not there once"
    os.makedirs(os.path.dirname(f"{OUT}/expected/{path}"), exist_ok=True)
    with open(f"{OUT}/expected/{path}", "w", encoding="utf-8", newline="") as fh:
        fh.write(text)
    print(f"expected {path}: {text.count(chr(10))} lines, sha256 {sha(text.encode())}")
    if path.endswith("check.mjs"):
        line = "    if (KEPT_LOGS_FILES.includes(f)) text = text.replaceAll(KEPT_LOGS_RESULTS, \"\");\n"
        assert text.count(line) == 1
        plants = {"P1": text.replace("    if (KEPT_LOGS_FILES.includes(f)) text",
                                     "    if (f.startsWith(\".claude/scripts/\")) text"),
                  "P2": text.replace(line, "    if (KEPT_LOGS_FILES.includes(f)) continue;\n"),
                  "P3": text.replace(line, "")}
        for p, body in plants.items():
            assert body != text and body.count("\n") in (text.count("\n"), text.count("\n") - 1), p
            os.makedirs(f"{OUT}/{p}", exist_ok=True)
            with open(f"{OUT}/{p}/check.mjs", "w", encoding="utf-8", newline="") as fh:
                fh.write(body)
            print(f"plant {p}: sha256 {sha(body.encode())}")
