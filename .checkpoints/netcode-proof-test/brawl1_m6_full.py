"""Write the advisor's M6 review of BRAWL-1's pushed head (workflow wf_8067a67a-6fe) word for word from the journal into
advisor-brawl1-m6-full.md: the verdict, its summary, each check, each finding, the go for M7 and "For Mustafa". Every
text field is copied as the agent returned it; only the headings and the bullet labels are the script's.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/brawl1_m6_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_8067a67a-6fe/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-brawl1-m6-full.md"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
res = [r["result"] for r in rows if r["type"] == "result" and isinstance(r.get("result"), dict)]
assert len(res) == 1, len(res)
v = res[0]
out = ["# Advisor: M6, the review of BRAWL-1's pushed head (workflow wf_8067a67a-6fe), 2026-11-15", "",
       "Written by script, word for word, from the workflow's journal (advisor-m6's StructuredOutput result). Its notes: "
       "advisor-brawl1-m6.md. Nothing below is the main session's.", "",
       f"**Head:** {v['head']}", "", f"## Verdict: {v['verdict']}", "", v["summary"], "", "## The checks", ""]
for i, c in enumerate(v["checks"], 1):
    out += [f"{i}. **{c['check']}** {c['result']}"]
out += ["", "## Findings", ""]
if not v["findings"]:
    out += ["None.", ""]
for f in v["findings"]:
    out += [f"### {f['id']} ({f['severity']})", "", f"- **Where:** {f['where']}", f"- **Wrong:** {f['wrong']}",
            f"- **Fix:** {f['fix']}", ""]
out += ["## M7", "", v["m7"], "", "## For Mustafa", "", v["to_mustafa"], ""]
text = "\n".join(out)
for k in ("head", "summary", "m7", "to_mustafa"):
    assert v[k] in text, k
for c in v["checks"]:
    assert c["check"] in text and c["result"] in text
for f in v["findings"]:
    assert all(f[k] in text for k in ("id", "where", "wrong", "fix"))
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    digest = hashlib.sha256(fh.read()).hexdigest()
print(f"{OUT}: {text.count(chr(10)) + 1} lines, sha256 {digest}; verdict {v['verdict']}; {len(v['checks'])} checks; "
      f"findings {[f['id'] + ' ' + f['severity'] for f in v['findings']]}")
