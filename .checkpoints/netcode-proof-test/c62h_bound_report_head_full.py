"""Save the advisor's check of the C62H-BOUND report head (wf_f820dccd-fb8: 'advisor: C62H-BOUND report head') word
for word from the journal, as JSON, into advisor-c62h-bound-report-head-full.json. Nothing is the main session's.
Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/c62h_bound_report_head_full.py"""
import hashlib
import json
import os

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_f820dccd-fb8/journal.jsonl")
N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
LAB = "advisor: C62H-BOUND report head"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
label = {r["key"]: r["label"] for r in rows if r["type"] == "started"}
res = {label[r["key"]]: r["result"] for r in rows if r["type"] == "result"}
assert set(res) == {LAB}, sorted(res)
out = f"{N}/advisor-c62h-bound-report-head-full.json"
assert not os.path.exists(out), out
text = json.dumps(res[LAB], indent=2, ensure_ascii=False) + "\n"
assert json.loads(text) == res[LAB]
with open(out, "w", encoding="utf-8") as fh:
    fh.write(text)
print(f"advisor-c62h-bound-report-head-full.json: {text.count(chr(10))} lines, sha256 "
      f"{hashlib.sha256(text.encode()).hexdigest()[:16]}; verdict {res[LAB]['verdict']}; head {res[LAB]['head_sha']}")
