"""Save PR #53's review workflow results (wf_a9e04f3e-df7: kl-pr-security, kl-pr-test, advisor-kl-pr-head1) word for
word from the journal, as JSON, into kl-pr-security-full.json, kl-pr-test-full.json and advisor-kl-pr-head1-full.json.
Nothing is the main session's. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/kl_pr53_review_full.py"""
import hashlib
import json
import os

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_a9e04f3e-df7/journal.jsonl")
N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
label = {r["key"]: r["label"] for r in rows if r["type"] == "started"}
res = {label[r["key"]]: r["result"] for r in rows if r["type"] == "result"}
assert set(res) == {"kl-pr-security", "kl-pr-test", "advisor-kl-pr-head1"}, sorted(res)
for lab, name in (("kl-pr-security", "kl-pr-security-full.json"), ("kl-pr-test", "kl-pr-test-full.json"),
                  ("advisor-kl-pr-head1", "advisor-kl-pr-head1-full.json")):
    out = f"{N}/{name}"
    assert not os.path.exists(out), out
    text = json.dumps(res[lab], indent=2, ensure_ascii=False) + "\n"
    assert json.loads(text) == res[lab]
    with open(out, "w", encoding="utf-8") as fh:
        fh.write(text)
    sev = [f["severity"] for f in res[lab].get("findings", [])]
    print(f"{name}: {text.count(chr(10))} lines, sha256 {hashlib.sha256(text.encode()).hexdigest()[:16]}; verdict "
          f"{res[lab]['verdict']}; findings {len(sev)} ({', '.join(f'{s} {sev.count(s)}' for s in dict.fromkeys(sev))})")
