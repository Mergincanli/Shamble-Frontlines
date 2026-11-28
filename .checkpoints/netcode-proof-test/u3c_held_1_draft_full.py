"""Save the three results of wf_5140889b-c32 (U3C-HELD-1's plan draft: the netcode-architect's cause and fix, the
test-engineer's evidence and disk, the drafter's return) word for word from the journal, as JSON, into
u3c-held-1/<name>-full.json. Nothing is the main session's. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u3c_held_1_draft_full.py"""
import hashlib
import json
import os

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_5140889b-c32/journal.jsonl")
D = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u3c-held-1"
OUT = {
    "netcode-architect: U3C-HELD-1 cause and fix": "netcode-architect-design-full.json",
    "test-engineer: U3C-HELD-1 evidence and disk": "test-engineer-evidence-full.json",
    "drafter: U3C-HELD-1 plan": "drafter-full.json",
}
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
label = {r["key"]: r["label"] for r in rows if r["type"] == "started"}
res = {label[r["key"]]: r["result"] for r in rows if r["type"] == "result"}
assert set(res) == set(OUT), sorted(res)
for lab, name in OUT.items():
    out = f"{D}/{name}"
    assert not os.path.exists(out), out
for lab, name in OUT.items():
    text = json.dumps(res[lab], indent=2, ensure_ascii=False) + "\n"
    assert json.loads(text) == res[lab]
    with open(f"{D}/{name}", "w", encoding="utf-8") as fh:
        fh.write(text)
    print(f"{name}: {text.count(chr(10))} lines, sha256 {hashlib.sha256(text.encode()).hexdigest()[:16]}")
