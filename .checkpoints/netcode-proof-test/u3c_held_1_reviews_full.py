"""Save the three reviews of wf_88c44554-26a (U3C-HELD-1's review round, C8) word for word from the journal, as JSON,
into u3c-held-1/<agent>-review-full.json. Nothing is the main session's. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u3c_held_1_reviews_full.py"""
import hashlib
import json
import os

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_88c44554-26a/journal.jsonl")
D = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u3c-held-1"
OUT = {
    "netcode-architect: U3C-HELD-1 review": "netcode-architect-review-full.json",
    "test-engineer: U3C-HELD-1 review": "test-engineer-review-full.json",
    "rust-reviewer: U3C-HELD-1 review": "rust-reviewer-review-full.json",
}
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
    sev = [f["severity"] for f in res[lab]["findings"]]
    print(f"{name}: {text.count(chr(10))} lines, sha256 {hashlib.sha256(text.encode()).hexdigest()[:16]}; "
          f"{res[lab]['verdict']}; findings {len(sev)}: " + ", ".join(f"{s} {sev.count(s)}" for s in
                                                                   ("Critical", "High", "Medium", "Low")))
