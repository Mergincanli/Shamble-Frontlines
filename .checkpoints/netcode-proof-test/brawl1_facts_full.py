"""Write the BRAWL-1 facts workflow's two returns (wf_d24f7a9b-8fe) word for word from the journal into
brawl1-facts-full.md: reader A's answers A1-A9 and risks, reader B's items and other next-lane items. Every text field
is copied as the agents returned it; only the headings and labels are the script's.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/brawl1_facts_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_d24f7a9b-8fe/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/brawl1-facts-full.md"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
res = [r["result"] for r in rows if r["type"] == "result" and isinstance(r.get("result"), dict)]
assert len(res) == 2, len(res)
a = next(r for r in res if "answers" in r)
b = next(r for r in res if "items" in r)
out = ["# BRAWL-1's facts: the two readers' returns (workflow wf_d24f7a9b-8fe), 2026-11-14", "",
       "Written by script, word for word, from the workflow's journal. Their notes: brawl1-facts-a.md and "
       "brawl1-facts-b.md. Nothing below is the main session's.", "", "## Reader A: the brawl, the world, the recordings", ""]
for x in a["answers"]:
    out += [f"### {x['q']}", "", x["summary"], "", f"**Evidence:** {x['evidence']}", ""]
out += ["### Risks for the plan", ""] + [f"- {r}" for r in a["risks_for_the_plan"]] + ["", f"Notes: {a['notes_file']}", ""]
out += ["## Reader B: the N2-1 items", ""]
for x in b["items"]:
    out += [f"### {x['id']}", "", f"- **Ruled:** {x['ruled']}", f"- **Code now:** {x['code_now']}",
            f"- **Change:** {x['change']}", f"- **Open points:** {x['open_points']}", f"- **Test:** {x['test']}",
            f"- **BRAWL-1 interaction:** {x['brawl1_interaction']}", f"- **Size:** {x['size']}", ""]
out += ["### Other next-lane items", ""] + [f"- {r}" for r in b["other_next_lane_items"]] + ["", f"Notes: {b['notes_file']}", ""]
text = "\n".join(out)
assert all(x["summary"] in text and x["evidence"] in text for x in a["answers"])
assert all(x["change"] in text and x["ruled"] in text for x in b["items"])
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    digest = hashlib.sha256(fh.read()).hexdigest()
print(f"{OUT}: {len(out)} lines, {len(text)} chars, sha256 {digest}; A answers {len(a['answers'])}, risks "
      f"{len(a['risks_for_the_plan'])}; B items {[x['id'] for x in b['items']]}; other {len(b['other_next_lane_items'])}")
