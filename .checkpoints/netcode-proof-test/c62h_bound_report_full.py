"""Save the C62H-BOUND report workflow's two results (wf_a1cd7d1b-556: 'draft: C62H-BOUND report' and 'verify:
C62H-BOUND report') word for word from the journal, as JSON, into c62h-bound-report-draft-full.json and
c62h-bound-report-verify-full.json. Nothing is the main session's. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/c62h_bound_report_full.py"""
import hashlib
import json
import os

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_a1cd7d1b-556/journal.jsonl")
N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
label = {r["key"]: r["label"] for r in rows if r["type"] == "started"}
res = {label[r["key"]]: r["result"] for r in rows if r["type"] == "result"}
assert set(res) == {"draft: C62H-BOUND report", "verify: C62H-BOUND report"}, sorted(res)
for lab, name in (("draft: C62H-BOUND report", "c62h-bound-report-draft-full.json"),
                  ("verify: C62H-BOUND report", "c62h-bound-report-verify-full.json")):
    out = f"{N}/{name}"
    assert not os.path.exists(out), out
    text = json.dumps(res[lab], indent=2, ensure_ascii=False) + "\n"
    assert json.loads(text) == res[lab]
    with open(out, "w", encoding="utf-8") as fh:
        fh.write(text)
    print(f"{name}: {text.count(chr(10))} lines, sha256 {hashlib.sha256(text.encode()).hexdigest()[:16]}"
          + (f"; verdict {res[lab]['verdict']}" if "verdict" in res[lab] else ""))
