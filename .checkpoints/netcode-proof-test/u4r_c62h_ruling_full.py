"""Write the advisor's ruling on carry 62 (h)'s record (workflow wf_d16081f4-f98) word for word from the journal into
advisor-u4r-c62h-full.md: the cause, the defect (JDG-ONLY-1), the record, whether carry 62 (h) stands, the next steps and
"For Mustafa". Every text field is copied as the agent returned it; only the headings and labels are the script's.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4r_c62h_ruling_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_d16081f4-f98/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-u4r-c62h-full.md"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
res = [r["result"] for r in rows if r["type"] == "result" and isinstance(r.get("result"), dict)]
assert len(res) == 1, len(res)
v = res[0]
d = v["defect"]
out = ["# Advisor: the ruling on carry 62 (h)'s record under --only PM-0001-17 (workflow wf_d16081f4-f98), 2026-11-14", "",
       "Written by script, word for word, from the workflow's journal (advisor-u4r-c62h's StructuredOutput result). Its "
       "notes: advisor-u4r-c62h.md. Nothing below is the main session's.", "", "## The cause", "", v["cause"], "",
       "## The defect", "", f"- **A defect:** {'yes' if d['is_defect'] else 'no'}"]
for k, label in (("severity", "Severity"), ("reach", "Reach"), ("home", "Home")):
    if d.get(k):
        out.append(f"- **{label}:** {d[k]}")
out += ["", "## The record", "", v["record"], "", f"## Carry 62 (h) stands: {'yes' if v['carry62h_stands'] else 'no'}",
        "", "## Next steps", ""]
out += [f"{i}. {s}" for i, s in enumerate(v["next_steps"], 1)]
out += ["", "## For Mustafa", "", v["to_mustafa"], ""]
text = "\n".join(out)
assert v["cause"] in text and v["record"] in text and v["to_mustafa"] in text and all(s in text for s in v["next_steps"])
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    digest = hashlib.sha256(fh.read()).hexdigest()
print(f"{OUT}: {len(out)} lines, sha256 {digest}; defect {d['is_defect']}; carry 62 (h) stands {v['carry62h_stands']}; "
      f"{len(v['next_steps'])} next steps")
