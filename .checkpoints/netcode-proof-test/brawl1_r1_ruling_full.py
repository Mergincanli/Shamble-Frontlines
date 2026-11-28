"""Write the advisor's ruling on BRAWL-1's review round 1 (workflow wf_59e64b31-fa2) word for word from the journal
into advisor-brawl1-r1-full.md, in the form of advisor-u4r-r1-full.md: the summary, the fix pass in order, the pins,
round 2, the merge step, U4g's brief, the size, "For Mustafa", then each finding. Every text field is copied as the
agent returned it; only the headings and the bullet labels are the script's.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/brawl1_r1_ruling_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_59e64b31-fa2/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-brawl1-r1-full.md"
SNAP = "ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
res = [r["result"] for r in rows if r["type"] == "result" and isinstance(r.get("result"), dict)]
assert len(res) == 1, len(res)
v = res[0]
assert SNAP in v["snapshot"], v["snapshot"][:120]
out = ["# Advisor: the ruling on BRAWL-1's review round 1 (workflow wf_59e64b31-fa2), 2026-11-15", "",
       "Written by script, word for word, from the workflow's journal (advisor-r1's StructuredOutput result). The "
       "round: brawl1-review-r1-full.md. Its notes: advisor-brawl1-r1.md. Nothing below is the main session's.", "",
       f"**Snapshot:** {v['snapshot']}", "", "## Summary", "", v["summary"], "", "## The fix pass, in order", ""]
for i, s in enumerate(v["fix_pass"], 1):
    out += [f"### Step {i}: {s['step']}", "", f"- **Findings:** {', '.join(s['findings']) or 'none'}",
            f"- **Words:** {s['words']}", f"- **Evidence:** {s['evidence']}", ""]
out += ["## The pins", "", v["pins"], "", "## Round 2", "", v["round2"], "", "## The merge step", "", v["merge_step"],
        "", "## U4g's brief", "", v["u4g_brief"], "", "## The size", "", v["size"], "", "## For Mustafa", "",
        v["to_mustafa"], "", "## Each finding", ""]
for f in v["each_finding"]:
    out += [f"### {f['id']} ({f['severity']}; {f['decision']})", "", f"- **Home:** {f['home']}",
            f"- **Why:** {f['why']}", ""]
text = "\n".join(out)
for k in ("snapshot", "summary", "pins", "round2", "merge_step", "u4g_brief", "size", "to_mustafa"):
    assert v[k] in text, k
for s in v["fix_pass"]:
    assert all(s[k] in text for k in ("step", "words", "evidence")) and all(x in text for x in s["findings"])
for f in v["each_finding"]:
    assert all(f[k] in text for k in ("id", "home", "why"))
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    digest = hashlib.sha256(fh.read()).hexdigest()
dec = {}
for f in v["each_finding"]:
    dec[f["decision"]] = dec.get(f["decision"], 0) + 1
print(f"{OUT}: {len(out)} lines, sha256 {digest}; {len(v['fix_pass'])} fix-pass steps; decisions {dec}")
for f in v["each_finding"]:
    if f["severity"] in ("Critical", "High", "Medium"):
        print(f"  {f['severity']}: {f['id']} -> {f['decision']}")
