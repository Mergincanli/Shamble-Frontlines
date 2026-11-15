"""Save the advisor's C62H-BOUND ruling (workflow wf_7885e774-38d, agent advisor-c62h-bound-r2: the test-engineer's read
ruled, texts v2 checked, exact replacements, the final size, the paste's blocks) word for word from the journal, as JSON,
into advisor-c62h-bound-r2-full.json. Nothing is the main session's. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/c62h_bound_r2_full.py"""
import hashlib
import json
import os

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_7885e774-38d/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-c62h-bound-r2-full.json"
assert not os.path.exists(OUT), OUT
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
label = {r["key"]: r["label"] for r in rows if r["type"] == "started"}
res = {label[r["key"]]: r["result"] for r in rows if r["type"] == "result"}
assert set(res) == {"advisor-c62h-bound-r2"}, sorted(res)
r = res["advisor-c62h-bound-r2"]
text = json.dumps(r, indent=2, ensure_ascii=False) + "\n"
assert json.loads(text) == r
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
print(f"{OUT}: {text.count(chr(10))} lines, sha256 {hashlib.sha256(text.encode()).hexdigest()[:16]}; verdict "
      f"{r['verdict']}; {len(r['te_rulings'])} rulings, {len(r['replacements'])} replacements, "
      f"{len(r['other_changes'])} other changes")
