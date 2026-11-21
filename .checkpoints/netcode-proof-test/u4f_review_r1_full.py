"""Write workflow wf_a011d9f2-abf's results word for word from its journal: U4f's review round 1, five seats, into
u4f-review-r1-full.md (per seat: its verdict and notes file, then each finding with Where, What, Proof and Fix, then its
own lines, its runs and what it did not check). Every text field is copied as the agent returned it; only the headings
and the bullet labels are the script's. Results are matched to agents by the label of their journal's "started" row.
Asserted: all five results present and every text field present in the output.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4f_review_r1_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_a011d9f2-abf/journal.jsonl")
N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
OUT = f"{N}/u4f-review-r1-full.md"
SEATS = [("netcode", "netcode-architect"), ("security", "security-auditor"), ("test", "test-engineer"),
         ("rust", "rust-reviewer"), ("arch", "engine-architect")]
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
label = {r["key"]: r["label"] for r in rows if r["type"] == "started"}
res = {}
for r in rows:
    if r["type"] == "result" and isinstance(r.get("result"), dict):
        res[label[r["key"]]] = r["result"]
need = [f"review:{k}" for k, _ in SEATS]
assert all(n in res for n in need), sorted(res)

out = ["# U4f's review round 1: the five seats' results (workflow wf_a011d9f2-abf), 2026-11-19", "",
       "Written by script, word for word, from the workflow's journal (each seat's StructuredOutput result). Tree "
       "reviewed: builder 3's worktree, equal to wip/u4f-build.patch 74ae26aa... (runner hashes diff 54cb6faa..., "
       "untracked e8dc3663...). The brief: u4f-panel-brief.md. Each seat's notes: u4f-review-<seat>.md. Nothing below "
       "is the main session's.", ""]
sev = {}
for k, t in SEATS:
    v = res[f"review:{k}"]
    out += [f"## {k} ({t}): {v['verdict']}", "", f"**Notes file:** {v['notes_file']}", ""]
    if not v["findings"]:
        out += ["**Findings:** none.", ""]
    for f in v["findings"]:
        sev[f["severity"]] = sev.get(f["severity"], 0) + 1
        out += [f"### {f['id']} ({f['severity']})", "", f"- **Where:** {f['where']}", f"- **What:** {f['what']}",
                f"- **Proof:** {f['proof']}", f"- **Fix:** {f['fix']}", ""]
    out += ["### Its own lines", ""] + ([f"- {x}" for x in v["own_lines"]] or ["None."]) + [""]
    out += ["### What it ran", ""] + ([f"- {x}" for x in v["ran"]] or ["Nothing."]) + [""]
    out += ["### What it did not check", "", v["not_checked"], ""]
text = "\n".join(out)
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    dig = hashlib.sha256(fh.read()).hexdigest()
for k, _ in SEATS:
    v = res[f"review:{k}"]
    assert v["notes_file"] in text and v["not_checked"] in text
    assert all(x in text for x in v["own_lines"]) and all(x in text for x in v["ran"])
    for f in v["findings"]:
        assert all(f[x] in text for x in ("id", "where", "what", "proof", "fix"))
print(f"{OUT}: {len(out)} lines, sha256 {dig}; verdicts {[(k, res['review:' + k]['verdict']) for k, _ in SEATS]}; "
      f"findings {sev}")
for k, _ in SEATS:
    for f in res[f"review:{k}"]["findings"]:
        if f["severity"] in ("Critical", "High", "Medium"):
            print(f"  {f['severity']}: {f['id']} ({k})")
