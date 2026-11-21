"""Save the C62H-BOUND report's verifier round 2 (wf_bfde2f03-00d: 'verify r2: C62H-BOUND report') word for word from
the journal, as JSON, into c62h-bound-report-verify-r2-full.json. Nothing is the main session's. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/c62h_bound_report_r2_full.py"""
import hashlib
import json
import os

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_bfde2f03-00d/journal.jsonl")
N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
label = {r["key"]: r["label"] for r in rows if r["type"] == "started"}
res = {label[r["key"]]: r["result"] for r in rows if r["type"] == "result"}
assert set(res) == {"verify r2: C62H-BOUND report"}, sorted(res)
out = f"{N}/c62h-bound-report-verify-r2-full.json"
assert not os.path.exists(out), out
text = json.dumps(res["verify r2: C62H-BOUND report"], indent=2, ensure_ascii=False) + "\n"
assert json.loads(text) == res["verify r2: C62H-BOUND report"]
with open(out, "w", encoding="utf-8") as fh:
    fh.write(text)
print(f"c62h-bound-report-verify-r2-full.json: {text.count(chr(10))} lines, sha256 "
      f"{hashlib.sha256(text.encode()).hexdigest()[:16]}; verdict {res['verify r2: C62H-BOUND report']['verdict']}")
