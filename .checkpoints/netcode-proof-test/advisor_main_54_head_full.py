"""Save the advisor's check of the MAIN-54 head (wf_910a6598-efe: 'advisor: MAIN-54 head') word for word from the
journal, as JSON, into advisor-main-54-head-full.json. Nothing is the main session's. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/advisor_main_54_head_full.py"""
import hashlib
import json
import os

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_910a6598-efe/journal.jsonl")
N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
LAB = "advisor: MAIN-54 head"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
label = {r["key"]: r["label"] for r in rows if r["type"] == "started"}
res = {label[r["key"]]: r["result"] for r in rows if r["type"] == "result"}
assert set(res) == {LAB}, sorted(res)
out = f"{N}/advisor-main-54-head-full.json"
assert not os.path.exists(out), out
text = json.dumps(res[LAB], indent=2, ensure_ascii=False) + "\n"
assert json.loads(text) == res[LAB]
with open(out, "w", encoding="utf-8") as fh:
    fh.write(text)
print(f"advisor-main-54-head-full.json: {text.count(chr(10))} lines, sha256 "
      f"{hashlib.sha256(text.encode()).hexdigest()[:16]}; verdict {res[LAB]['verdict']}; head {res[LAB]['head_sha']}")
