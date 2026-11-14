"""Write the advisor's plan review of U4r-F1 (workflow wf_0471e991-4c8) word for word from the journal into
advisor-u4r-f1-plan-full.md: the verdict, each item, the answers to Q1-Q6, the conditions, the size and "For Mustafa".
Every text field is copied as the agent returned it; only the headings and labels are the script's.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4r_f1_plan_review_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_0471e991-4c8/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-u4r-f1-plan-full.md"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
res = [r["result"] for r in rows if r["type"] == "result" and isinstance(r.get("result"), dict)]
assert len(res) == 1, len(res)
v = res[0]
out = ["# Advisor: the plan review of U4r-F1 (workflow wf_0471e991-4c8), 2026-11-14", "",
       "Written by script, word for word, from the workflow's journal (advisor-u4r-f1-plan's StructuredOutput result). The "
       "plan: u4r-f1-plan.md. Its notes: advisor-u4r-f1-plan.md. Nothing below is the main session's.", "",
       f"## Verdict: {v['verdict']}", "", "## Conditions", ""]
out += [f"{i}. {c}" for i, c in enumerate(v["conditions"], 1)] + ["", "## The choices (the plan's section 8)", ""]
for c in v["choices"]:
    out += [f"- **{c['q']}** {c['answer']} **Why:** {c['why']}"]
out += ["", "## Each item", ""]
for it in v["items"]:
    out += [f"- **{it['item']}:** {it['ruling']}"]
out += ["", "## Size", "", v["size"], "", "## For Mustafa", "", v["to_mustafa"], ""]
text = "\n".join(out)
assert all(c in text for c in v["conditions"]) and v["to_mustafa"] in text and v["size"] in text
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    digest = hashlib.sha256(fh.read()).hexdigest()
print(f"{OUT}: {len(out)} lines, sha256 {digest}; verdict {v['verdict']}; {len(v['conditions'])} conditions; "
      f"{len(v['choices'])} choices")
