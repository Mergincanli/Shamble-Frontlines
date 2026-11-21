"""Save the two results of wf_11d3eb2e-5f0 (DOCS-54's plan: docs-keeper's draft, the advisor's review) word for word
from the journal, as JSON, into docs-54/<name>-full.json. Nothing is the main session's. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/docs_54_plan_full.py"""
import hashlib
import json
import os

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_11d3eb2e-5f0/journal.jsonl")
D = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/docs-54"
OUT = {"docs-keeper: DOCS-54 plan": "docs-keeper-draft-full.json", "advisor: DOCS-54 plan": "advisor-plan-review-full.json"}
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
label = {r["key"]: r["label"] for r in rows if r["type"] == "started"}
res = {label[r["key"]]: r["result"] for r in rows if r["type"] == "result"}
assert set(res) == set(OUT), sorted(res)
for name in OUT.values():
    assert not os.path.exists(f"{D}/{name}"), name
for lab, name in OUT.items():
    text = json.dumps(res[lab], indent=2, ensure_ascii=False) + "\n"
    assert json.loads(text) == res[lab]
    with open(f"{D}/{name}", "w", encoding="utf-8") as fh:
        fh.write(text)
    print(f"{name}: {text.count(chr(10))} lines, sha256 {hashlib.sha256(text.encode()).hexdigest()[:16]}")
print("verdict:", res["advisor: DOCS-54 plan"]["verdict"], "plan", res["advisor: DOCS-54 plan"]["plan_sha256"][:16])
