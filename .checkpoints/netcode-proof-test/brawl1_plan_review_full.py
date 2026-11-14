"""Write the advisor's plan review of BRAWL-1's lane (workflow wf_533068a9-b2b) word for word from the journal into
advisor-brawl1-plan-full.md: the verdict, the conditions, the answers to Q1-Q8, the findings, the size, the KEPT-LOGS
question and "For Mustafa". Every text field is copied as the agent returned it; only the headings and labels are the
script's.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/brawl1_plan_review_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_533068a9-b2b/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-brawl1-plan-full.md"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
res = [r["result"] for r in rows if r["type"] == "result" and isinstance(r.get("result"), dict)]
assert len(res) == 1, len(res)
v = res[0]
out = ["# Advisor: the plan review of BRAWL-1's lane (workflow wf_533068a9-b2b), 2026-11-14", "",
       "Written by script, word for word, from the workflow's journal (advisor-brawl1-plan's StructuredOutput result). The "
       "plan: brawl1-plan.md. Its notes: advisor-brawl1-plan.md. Nothing below is the main session's.", "",
       f"## Verdict: {v['verdict']}", "", "## Conditions", ""]
out += [f"{i}. {c}" for i, c in enumerate(v["conditions"], 1)] + ["", "## The choices (the plan's section 13)", ""]
for c in v["choices"]:
    out += [f"- **{c['q']}** {c['answer']} **Why:** {c['why']}"]
out += ["", "## Findings", ""]
for f in v["findings"]:
    out += [f"### {f['id']} ({f['severity']})", "", f"- **Where:** {f['where']}", f"- **Wrong:** {f['wrong']}",
            f"- **Fix:** {f['fix']}", ""]
if not v["findings"]:
    out += ["None.", ""]
out += ["## Size", "", v["size"], "", "## The KEPT-LOGS question", "", v["kept_logs_question"], "", "## For Mustafa", "",
        v["to_mustafa"], ""]
text = "\n".join(out)
assert all(c in text for c in v["conditions"]) and v["to_mustafa"] in text and v["size"] in text
assert v["kept_logs_question"] in text
assert all(c["answer"] in text and c["why"] in text for c in v["choices"])
assert all(f["fix"] in text and f["wrong"] in text for f in v["findings"])
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    digest = hashlib.sha256(fh.read()).hexdigest()
print(f"{OUT}: {len(out)} lines, sha256 {digest}; verdict {v['verdict']}; {len(v['conditions'])} conditions; "
      f"{len(v['choices'])} choices; findings {[(f['id'], f['severity']) for f in v['findings']]}")
