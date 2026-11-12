"""Show review_packet.py's name check (Mustafa's words of 2026-11-12, item 2, verbatim in history.md) on one doc per
case: which paths pass and which are refused. Each case's config and packet go under .gate/packet-name-check/ (an
absolute "out"), so no packet in the repo is touched. The script under test is review_packet.py, or the path given.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/packet_name_check.py [script]"""
import json
import os
import subprocess
import sys

S = "/home/user/Shamble-Frontlines/.gate/packet-name-check"
os.makedirs(S, exist_ok=True)
N = ".checkpoints/netcode-proof-test"
cases = [
    ("advisor-u4e-brief.md (the advisor's ruling, on the list)", f"{N}/advisor-u4e-brief.md", False, "pass"),
    ("advisor-u4e-brief-brief.md (the main session's brief)", f"{N}/advisor-u4e-brief-brief.md", False, "refused"),
    ("advisor-u4df-brief.md (the main session's brief)", f"{N}/advisor-u4df-brief.md", False, "refused"),
    ("advisor-u4j-r2-brief.md (the main session's brief)", f"{N}/advisor-u4j-r2-brief.md", False, "refused"),
    ("advisor-u4e-r1-brief.md (the main session's brief)", f"{N}/advisor-u4e-r1-brief.md", False, "refused"),
    ("advisor-u4d2-brief.md (the advisor's ruling, on the list by addition 1)", f"{N}/advisor-u4d2-brief.md", False,
     "pass"),
    ("advisor-u4e-r2-brief.md (the main session's brief)", f"{N}/advisor-u4e-r2-brief.md", False, "refused"),
    ("u4d2-brief.md (a builder brief)", f"{N}/u4d2-brief.md", False, "refused"),
    ("history.md, flagged as Mustafa's words", f"{N}/history.md", True, "pass"),
    ("history.md, not flagged", f"{N}/history.md", False, "refused"),
    ("u4df-report.md flagged as Mustafa's words (a report)", f"{N}/u4df-report.md", True, "refused"),
    ("advisor-u4e-r1.md (a ruling the pattern never matched)", f"{N}/advisor-u4e-r1.md", False, "pass"),
]
script = sys.argv[1] if len(sys.argv) > 1 else f"{N}/review_packet.py"
bad = 0
for i, (label, path, flag, want) in enumerate(cases, 1):
    doc = {"label": label, "path": path, "lines": [1, 1]}
    if flag:
        doc["words_of_mustafa"] = True
    cfg = {"title": f"name check case {i}", "out": f"{S}/case{i}.md", "heads": ["test"], "diffs": [], "docs": [doc]}
    cp = f"{S}/case{i}.json"
    with open(cp, "w", encoding="utf-8") as fh:
        json.dump(cfg, fh)
    r = subprocess.run(["python3", "-I", script, cp], capture_output=True, text=True)
    got = "pass" if r.returncode == 0 else ("refused" if "AssertionError" in r.stderr else f"error {r.returncode}")
    ok = got == want
    bad += not ok
    print(f"{'OK ' if ok else 'BAD'} case {i}: {label}: exit {r.returncode}, {got} (want {want})")
print(f"{script}: {len(cases) - bad} of {len(cases)} as wanted")
raise SystemExit(1 if bad else 0)
