"""Save the advisor's REFS-KIT ruling (workflow wf_06df1520-0b9, agent advisor-refs-clash: the clash between check.mjs's
refs step, plan §4.9, and the kept-logs kit files merged from main) word for word from the journal, as JSON, into
advisor-refs-clash-full.json. Nothing is the main session's. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/refs_clash_full.py"""
import hashlib
import json
import os

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_06df1520-0b9/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-refs-clash-full.json"
assert not os.path.exists(OUT), OUT
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
label = {r["key"]: r["label"] for r in rows if r["type"] == "started"}
res = {label[r["key"]]: r["result"] for r in rows if r["type"] == "result"}
assert set(res) == {"advisor-refs-clash"}, sorted(res)
r = res["advisor-refs-clash"]
text = json.dumps(r, indent=2, ensure_ascii=False) + "\n"
assert json.loads(text) == r
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
f = r["fix"]
print(f"{OUT}: {text.count(chr(10))} lines, sha256 {hashlib.sha256(text.encode()).hexdigest()[:16]}; verdict "
      f"{r['verdict']}; replacements: check.mjs {len(f['check_mjs'])}, check.test.mjs {len(f['check_test_mjs'])}, "
      f"plan.md {len(f['plan_md'])}")
