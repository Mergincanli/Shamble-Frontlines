"""Write BRAWL-1 review round 2's results word for word from the workflow's journal (wf_19a5485a-e64) into
brawl1-review-r2-full.md, in round 1's form: per seat, its verdict and snapshot, its seat text, the state of each of its
round-1 findings, then each new finding with Where, Wrong, Evidence and Fix. Every text field is copied as the agent
returned it; only the headings and the bullet labels are the script's. Results are matched to agents by the label of
their journal's "started" row. Asserted: both results present, each naming snapshot c2561b6c..., every text field in
the output. Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/brawl1_r2_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_19a5485a-e64/journal.jsonl")
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/brawl1-review-r2-full.md"
SNAP = "c2561b6ca54c8cde994fcdad5954c63d332df73f2db038c12aba3d71a0daa49f"
SEATS = [("test", "test-engineer"), ("rust", "rust-reviewer")]
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
label = {r["key"]: r["label"] for r in rows if r["type"] == "started"}
res = {}
for r in rows:
    if r["type"] == "result" and isinstance(r.get("result"), dict):
        res[label[r["key"]]] = r["result"]
assert all(f"review2-{k}" in res for k, _ in SEATS), sorted(res)
out = ["# BRAWL-1 review round 2: the two seats' results (workflow wf_19a5485a-e64), 2026-11-15", "",
       "Written by script, word for word, from the workflow's journal. Snapshot reviewed: wip/brawl1.patch c2561b6c...; "
       "packet brawl1-r2-packet.md e50a22f7.... Each seat's notes: brawl1-review2-<seat>.md. Nothing below is the main "
       "session's.", ""]
sev = {}
for k, t in SEATS:
    v = res[f"review2-{k}"]
    assert SNAP in v["snapshot"], (k, v["snapshot"][:120])
    out += [f"## review2-{k} ({v['verdict']}; snapshot {SNAP[:12]})", "", f"**Seat:** {v['seat']}", ""]
    if v["snapshot"] != SNAP:
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
        sev[f["severity"]] = sev.get(f["severity"], 0) + 1
        out += [f"### {f['id']} ({f['severity']})", "", f"- **Where:** {f['where']}", f"- **Wrong:** {f['wrong']}",
                f"- **Evidence:** {f['evidence']}", f"- **Fix:** {f['fix']}", ""]
text = "\n".join(out)
for k, _ in SEATS:
    v = res[f"review2-{k}"]
    assert v["seat"] in text and v["snapshot"] in text
    for x in v["round1"]:
        assert x["where"] in text and x.get("note", "") in text
    for f in v["findings"]:
        assert all(f[x] in text for x in ("id", "where", "wrong", "evidence", "fix"))
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(text)
with open(OUT, "rb") as fh:
    digest = hashlib.sha256(fh.read()).hexdigest()
print(f"{OUT}: {len(out)} entries, sha256 {digest}; verdicts {[(k, res['review2-' + k]['verdict']) for k, _ in SEATS]}; "
      f"findings {sev}")
for k, _ in SEATS:
    for x in res[f"review2-{k}"]["round1"]:
        if x["state"] not in ("fixed as ruled", "deferred as ruled"):
            print(f"  round-1 {x['id']} ({k}): {x['state']}")
    for f in res[f"review2-{k}"]["findings"]:
        if f["severity"] in ("Critical", "High", "Medium"):
            print(f"  {f['severity']}: {f['id']} ({k})")
