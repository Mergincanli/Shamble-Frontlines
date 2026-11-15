"""Write the night plan check's two results (workflow wf_6e5acbfd-5c3: the verifier night-plan-verify and the advisor
advisor-night-plan) word for word from the journal into night-plan-verify-full.md and advisor-night-plan-full.md.
Only the headings and the field labels are the script's. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/night_plan_check_full.py"""
import hashlib
import json
import os

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_6e5acbfd-5c3/journal.jsonl")
N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
OUTV, OUTA = f"{N}/night-plan-verify-full.md", f"{N}/advisor-night-plan-full.md"
assert not os.path.exists(OUTV) and not os.path.exists(OUTA)
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
label = {r["key"]: r["label"] for r in rows if r["type"] == "started"}
res = {label[r["key"]]: r["result"] for r in rows if r["type"] == "result"}
assert set(res) == {"night-plan-verify", "advisor-night-plan"}, sorted(res)
v, a = res["night-plan-verify"], res["advisor-night-plan"]
assert isinstance(v, dict) and isinstance(a, dict)
NOTE = "Written by script, word for word, from the workflow's journal. Nothing below is the main session's."


def write(path, lines, values):
    text = "\n".join(lines)
    assert all(x in text for x in values)
    with open(path, "w", encoding="utf-8") as fh:
        fh.write(text)
    with open(path, "rb") as fh:
        print(f"{path}: {text.count(chr(10)) + 1} lines, sha256 {hashlib.sha256(fh.read()).hexdigest()}")


out = ["# Verifier: the references of night-2026-11-16.md (workflow wf_6e5acbfd-5c3), 2026-11-15", "",
       NOTE + " Its notes: night-plan-verify.md.", "", f"- **Verdict:** {v['verdict']}",
       f"- **References checked:** {v['refs_checked']}", "", "## Summary", "", v["summary"], "", "## Wrong references", ""]
vals = [v["verdict"], str(v["refs_checked"]), v["summary"]]
for w in v["wrong"]:
    out += [f"- **Where in the plan:** {w['where_in_plan']}", f"  - **Cited:** {w['cited']}",
            f"  - **Finding:** {w['finding']}", f"  - **Correct:** {w['correct']}"]
    vals += [w[x] for x in ("where_in_plan", "cited", "finding", "correct")]
if not v["wrong"]:
    out.append("(none)")
out += ["", "## Spot checks of the scout's map", ""]
for s in v["spot_checks"]:
    out.append(f"- **{s['entry']}** · **holds:** {'yes' if s['ok'] else 'NO'} · {s['note']}")
    vals += [s["entry"], s["note"]]
if not v["spot_checks"]:
    out.append("(none)")
out.append("")
write(OUTV, out, vals)

out = ["# Advisor: the check of night-2026-11-16.md (workflow wf_6e5acbfd-5c3), 2026-11-15", "",
       NOTE + " Its notes: advisor-night-plan.md.", "", f"- **Verdict:** {a['verdict']}", "", "## Summary", "",
       a["summary"], "", "## Findings", ""]
vals = [a["verdict"], a["summary"]]
for f in a["findings"]:
    out += [f"### {f['id']} ({f['severity']})", "", f"- **Where:** {f['where']}", f"- **What:** {f['what']}",
            f"- **Fix:** {f['fix']}", ""]
    vals += [f[x] for x in ("id", "severity", "where", "what", "fix")]
if not a["findings"]:
    out += ["(none)", ""]
out += ["## Conditions", ""] + [f"{i}. {c}" for i, c in enumerate(a["conditions"], 1)]
if not a["conditions"]:
    out.append("(none)")
vals += a["conditions"]
out += ["", "## Approved under his rule of 2026-11-09?", "", a["approved_under_rule"], "", "## To Mustafa", "",
        a["to_mustafa"], ""]
vals += [a["approved_under_rule"], a["to_mustafa"]]
write(OUTA, out, vals)
