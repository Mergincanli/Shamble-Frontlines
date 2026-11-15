"""Write the KEPT-LOGS kit rule's plan review (workflow wf_0f69bce5-688: the drafter kept-logs-draft, then the advisor
advisor-kept-logs) word for word from the journal into advisor-kept-logs-rule-full.md. Only the headings are the
script's. Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/kept_logs_rule_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_0f69bce5-688/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-kept-logs-rule-full.md"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
label = {r["key"]: r["label"] for r in rows if r["type"] == "started"}
res = {label[r["key"]]: r["result"] for r in rows if r["type"] == "result"}
assert set(res) == {"kept-logs-draft", "advisor-kept-logs"}, sorted(res)
d, v = res["kept-logs-draft"], res["advisor-kept-logs"]
assert isinstance(d, str) and isinstance(v, dict)
keys = [("verdict", "Verdict"), ("summary", "Summary"), ("conditions", "Conditions"),
        ("final_text_for_mustafa", "The final text for Mustafa"), ("pr_reviewers", "The PR's reviewers"),
        ("to_mustafa", "For Mustafa")]
out = ["# Advisor: the plan review of the KEPT-LOGS kit rule's drafted lines (workflow wf_0f69bce5-688), 2026-11-15", "",
       "Written by script, word for word, from the workflow's journal. The draft: kept-logs-rule-draft.md; the advisor's "
       "notes: advisor-kept-logs-rule.md. Nothing below is the main session's.", ""]
for k, h in keys:
    out += [f"## {h}", "", v[k], ""]
out += ["## The drafter's result", "", d, ""]
text = "\n".join(out)
assert all(v[k] in text for k, _ in keys) and d in text
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    print(f"{OUT}: {text.count(chr(10)) + 1} lines, sha256 {hashlib.sha256(fh.read()).hexdigest()}")
