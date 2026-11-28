"""Write U4r review round 2's results word for word from the workflow's journal (wf_5e07787a-ae9) into
u4r-review-r2-full.md, in round 1's form (u4r-review-r1-full.md): per seat, its verdict and snapshot, its seat text, the
state of each of its round-1 findings, then each new finding with Where, Wrong, Evidence and Fix. Every text field is
copied as the agent returned it; only the headings and the bullet labels are the script's. Asserted: five results,
each naming snapshot 21ca46be..., and the seats in the workflow's order.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4r_review_r2_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_5e07787a-ae9/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-review-r2-full.md"
SNAP = "21ca46be23704131c77cc20981a4bfe866811b33f9ca853279f7ddbfb5d58bce"
KEYS = ["netcode", "security", "test", "rust", "infra"]
TYPES = ["netcode-architect", "security-auditor", "test-engineer", "rust-reviewer", "infra-engineer"]
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
res = [r["result"] for r in rows if r["type"] == "result"]
assert len(res) == 5, len(res)
for v, t in zip(res, TYPES):
    assert isinstance(v, dict) and v["snapshot"].startswith(SNAP), v.get("snapshot")
    assert v["seat"].startswith(t), (t, v["seat"][:60])
out = ["# U4r review round 2: the five seats' results (workflow wf_5e07787a-ae9), 2026-11-14", "",
       "Written by script, word for word, from the workflow's journal. Snapshot reviewed: wip/u4r.patch 21ca46be...; "
       "packet u4r-r2-packet.md b1526bde.... Each seat's notes: u4r-review2-<seat>.md. Nothing below is the main "
       "session's.", ""]
for k, v in zip(KEYS, res):
    out += [f"## review2-{k} ({v['verdict']}; snapshot {v['snapshot'][:12]})", "", f"**Seat:** {v['seat']}", ""]
    if v["snapshot"] != SNAP:
        # One seat wrote more than the sha256 into its snapshot field: kept whole, word for word.
        out += [f"**Its snapshot field, whole:** {v['snapshot']}", ""]
    out += ["**Its round-1 findings:**", ""]
    for x in v["round1"]:
        line = f"- **{x['id']}:** {x['state']}. **Where:** {x['where']}"
        if x.get("note"):
            line += f" **Note:** {x['note']}"
        out.append(line)
    out.append("")
    if not v["findings"]:
        out += ["**New findings:** none.", ""]
    for f in v["findings"]:
        out += [f"### {f['id']} ({f['severity']})", "", f"- **Where:** {f['where']}", f"- **Wrong:** {f['wrong']}",
                f"- **Evidence:** {f['evidence']}", f"- **Fix:** {f['fix']}", ""]
text = "\n".join(out)
for v in res:
    for f in v["findings"]:
        for field in ("where", "wrong", "evidence", "fix"):
            assert f[field] in text
    assert v["seat"] in text and v["snapshot"] in text
    for x in v["round1"]:
        assert x["where"] in text and x.get("note", "") in text
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    digest = hashlib.sha256(fh.read()).hexdigest()
sev = {}
for v in res:
    for f in v["findings"]:
        sev[f["severity"]] = sev.get(f["severity"], 0) + 1
print(f"{OUT}: {len(out)} lines, sha256 {digest}; verdicts {[v['verdict'] for v in res]}; findings {sev}; "
      f"medium or higher: {[f['id'] for v in res for f in v['findings'] if f['severity'] in ('Critical', 'High', 'Medium')]}")
