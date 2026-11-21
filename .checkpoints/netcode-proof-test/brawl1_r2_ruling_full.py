"""Write the advisor's ruling on BRAWL-1's review round 2 (workflow wf_d32decd1-87b) word for word from the journal into
advisor-brawl1-r2-full.md: the summary, round 3, each finding, copy 2's deviations, the resume-part errors, the merge
step, the report words and "For Mustafa". Every text field is copied as the agent returned it; only the headings and the
bullet labels are the script's. Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/brawl1_r2_ruling_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_d32decd1-87b/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-brawl1-r2-full.md"
SNAP = "c2561b6ca54c8cde994fcdad5954c63d332df73f2db038c12aba3d71a0daa49f"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
res = [r["result"] for r in rows if r["type"] == "result" and isinstance(r.get("result"), dict)]
assert len(res) == 1, len(res)
v = res[0]
assert SNAP in v["snapshot"], v["snapshot"][:120]
out = ["# Advisor: the ruling on BRAWL-1's review round 2 (workflow wf_d32decd1-87b), 2026-11-15", "",
       "Written by script, word for word, from the workflow's journal (advisor-r2's StructuredOutput result). The round: "
       "brawl1-review-r2-full.md. Its notes: advisor-brawl1-r2.md. Nothing below is the main session's.", "",
       f"**Snapshot:** {v['snapshot']}", "", "## Summary", "", v["summary"], "",
       f"**Round 3:** {'opens' if v['round3'] else 'does not open'}", "", "## Each finding", ""]
for f in v["each_finding"]:
    out += [f"### {f['id']} ({f['severity']}; {f['decision']})", "", f"- **Home:** {f['home']}",
            f"- **Why:** {f['why']}", ""]
out += ["## Copy 2's deviations", ""]
for i, d in enumerate(v["deviations"], 1):
    out += [f"{i}. **What:** {d['what']}", f"   - **Ruling:** {d['ruling']}", f"   - **Why:** {d['why']}"]
if not v["deviations"]:
    out += ["None."]
out += ["", "## The main session's resume-part errors", "", v["resume_errors"], "", "## The merge step", "",
        v["merge_step"], "", "## The report words", "", v["report_words"], "", "## For Mustafa", "", v["to_mustafa"], ""]
text = "\n".join(out)
for k in ("snapshot", "summary", "resume_errors", "merge_step", "report_words", "to_mustafa"):
    assert v[k] in text, k
for f in v["each_finding"]:
    assert all(f[k] in text for k in ("id", "home", "why"))
for d in v["deviations"]:
    assert all(d[k] in text for k in ("what", "ruling", "why"))
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    digest = hashlib.sha256(fh.read()).hexdigest()
print(f"{OUT}: {text.count(chr(10)) + 1} lines, sha256 {digest}; round 3 {v['round3']}; "
      f"decisions {[(f['id'], f['decision']) for f in v['each_finding']]}; {len(v['deviations'])} deviations")
