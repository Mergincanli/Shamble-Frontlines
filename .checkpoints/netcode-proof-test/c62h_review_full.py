"""Write C62H-REVIEW's two results (workflow wf_4ce9b8cd-ad2: the test-engineer's read c62h-read-test, then the
advisor's ruling advisor-c62h) word for word from the journal into c62h-review-test-full.md and
advisor-c62h-review-full.md. Only the headings and the field labels are the script's.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/c62h_review_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_4ce9b8cd-ad2/journal.jsonl")
N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
label = {r["key"]: r["label"] for r in rows if r["type"] == "started"}
res = {label[r["key"]]: r["result"] for r in rows if r["type"] == "result"}
assert set(res) == {"c62h-read-test", "advisor-c62h"}, sorted(res)
rd, ru = res["c62h-read-test"], res["advisor-c62h"]
assert isinstance(rd, dict) and isinstance(ru, dict)
NOTE = "Written by script, word for word, from the workflow's journal. Nothing below is the main session's."


def write(path, lines, values):
    text = "\n".join(lines)
    assert all(x in text for x in values)
    with open(path, "w", encoding="utf-8") as fh:
        fh.write(text)
    with open(path, "rb") as fh:
        print(f"{path}: {text.count(chr(10)) + 1} lines, sha256 {hashlib.sha256(fh.read()).hexdigest()}")


out = ["# Test-engineer: the read of commit 5e563b9's 15 marks.json lines (C62H-REVIEW, workflow wf_4ce9b8cd-ad2), "
       "2026-11-15", "", NOTE + " Its notes: c62h-review-test.md. The questions are Mustafa's, in history.md, "
       "\"## Decision: Mustafa, 2026-11-15 13:35 Amsterdam\".", "",
       f"- **Seat:** {rd['seat']}", f"- **marks.json at HEAD, sha256:** {rd['marks_sha256']}", ""]
vals = [rd["seat"], rd["marks_sha256"]]
for k, h in [("q1_word_for_word", "Question 1"), ("q2_can_it_fail", "Question 2"), ("q3_bound_off_p0", "Question 3")]:
    q = rd[k]
    out += [f"## {h}", "", f"- **Answer:** {q['answer']}", "", q["text"], "", "**Evidence:**", "", q["evidence"], ""]
    vals += [q["answer"], q["text"], q["evidence"]]
    if "what_could_hide" in q:
        out += ["**What a client bug of that kind could hide:**", "", q["what_could_hide"], ""]
        vals.append(q["what_could_hide"])
out += ["## Findings", ""]
for f in rd["findings"]:
    out += [f"### {f['id']} ({f['severity']})", "", f"- **Where:** {f['where']}", f"- **What is wrong:** {f['wrong']}",
            f"- **Evidence:** {f['evidence']}", f"- **Fix:** {f['fix']}", ""]
    vals += [f[x] for x in ("id", "severity", "where", "wrong", "evidence", "fix")]
if not rd["findings"]:
    out += ["(none)", ""]
out += ["## Proposed bars (proposals only: to the advisor, then to Mustafa)", ""]
for b in rd["proposed_bars"]:
    out += [f"### {b['id']}", "", f"- **Wording:** {b['wording']}", f"- **Why:** {b['why']}", ""]
    vals += [b[x] for x in ("id", "wording", "why")]
if not rd["proposed_bars"]:
    out += ["(none)", ""]
write(f"{N}/c62h-review-test-full.md", out, vals)

out = ["# Advisor: the ruling on C62H-REVIEW's read (workflow wf_4ce9b8cd-ad2), 2026-11-15", "",
       NOTE + " Its notes: advisor-c62h-review.md.", "", "## Summary", "", ru["summary"], "",
       "## The read's three answers", "", ru["answers"], "", "## Each finding", ""]
vals = [ru["summary"], ru["answers"]]
for f in ru["each_finding"]:
    out += [f"- **{f['id']}** ({f['severity']}): {f['decision']}; home: {f['home']}. {f['why']}"]
    vals += [f[x] for x in ("id", "severity", "decision", "home", "why")]
if not ru["each_finding"]:
    out += ["(none)"]
out += ["", "## The proposed bars", ""]
for b in ru["bars"]:
    out += [f"- **{b['id']}**: to Mustafa: {'yes' if b['to_mustafa'] else 'no'}. {b['why']}"]
    vals += [b["id"], b["why"]]
if not ru["bars"]:
    out += ["(none)"]
out += ["", "## Decisions for Mustafa", ""]
for c in ru["decisions_for_mustafa"]:
    out += [f"### {c['code']}", "", f"- **Question:** {c['question']}", f"- **Why it matters:** {c['why_it_matters']}",
            f"- **Options:** {c['options']}", f"- **Recommendation:** {c['recommendation']}", ""]
    vals += [c[x] for x in ("code", "question", "why_it_matters", "options", "recommendation")]
if not ru["decisions_for_mustafa"]:
    out += ["(none)", ""]
for k, h in [("units_ahead", "Units ahead that change marks.json or the judge"), ("report_words", "The report words"),
             ("to_mustafa", "For Mustafa")]:
    out += [f"## {h}", "", ru[k], ""]
    vals.append(ru[k])
write(f"{N}/advisor-c62h-review-full.md", out, vals)
