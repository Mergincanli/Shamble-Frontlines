"""Write workflow wf_bbfeece0-4c7's results word for word from its journal: the advisor's ruling on the BRAWL-1
builder's points into advisor-brawl1-points-full.md, and review round 1's four seats into brawl1-review-r1-full.md (per
seat: its verdict and snapshot, its seat text, then each finding with Where, Wrong, Evidence and Fix). Every text field
is copied as the agent returned it; only the headings and the bullet labels are the script's. Results are matched to
agents by the label of their journal's "started" row. Asserted: all five results present, each naming snapshot
ee2973aa..., and every text field present in the output.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/brawl1_r1_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_bbfeece0-4c7/journal.jsonl")
N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
OUT_A = f"{N}/advisor-brawl1-points-full.md"
OUT_R = f"{N}/brawl1-review-r1-full.md"
SNAP = "ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7"
SEATS = [("test", "test-engineer"), ("netcode", "netcode-architect"), ("rust", "rust-reviewer"),
         ("arch", "engine-architect")]
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
label = {r["key"]: r["label"] for r in rows if r["type"] == "started"}
res = {}
for r in rows:
    if r["type"] == "result" and isinstance(r.get("result"), dict):
        res[label[r["key"]]] = r["result"]
need = ["advisor-points"] + [f"review-{k}" for k, _ in SEATS]
assert all(n in res for n in need), sorted(res)


def save(path, out):
    text = "\n".join(out)
    with open(path, "w", encoding="utf-8") as fh:
        fh.write(text)
    with open(path, "rb") as fh:
        return text, hashlib.sha256(fh.read()).hexdigest()


a = res["advisor-points"]
assert SNAP in a["snapshot"], a["snapshot"][:120]  # the field may say more than the sha256: kept whole below
out = ["# Advisor: the ruling on BRAWL-1's builder points (workflow wf_bbfeece0-4c7), 2026-11-15", "",
       "Written by script, word for word, from the workflow's journal (advisor-points' StructuredOutput result). The "
       "points: brawl1-points.md. Its notes: advisor-brawl1-points.md. Nothing below is the main session's.", "",
       f"**Snapshot:** {a['snapshot']}", "", "## Summary", "", a["summary"], "", "## Rulings", ""]
for x in a["rulings"]:
    out += [f"### {x['point']}", "", f"- **Ruling:** {x['ruling']}", f"- **Why:** {x['why']}",
            f"- **Changes the diff under review:** {'yes' if x['changes_diff'] else 'no'}",
            f"- **The change:** {x['change']}", ""]
out += ["## Further findings", ""]
if not a["further_findings"]:
    out += ["None.", ""]
for f in a["further_findings"]:
    out += [f"### {f['id']} ({f['severity']})", "", f"- **Where:** {f['where']}", f"- **Wrong:** {f['wrong']}",
            f"- **Fix:** {f['fix']}", ""]
out += ["## For Mustafa", "", a["to_mustafa"], ""]
text, dig_a = save(OUT_A, out)
assert a["summary"] in text and a["to_mustafa"] in text and a["snapshot"] in text
for x in a["rulings"]:
    assert all(x[k] in text for k in ("point", "ruling", "why", "change"))
for f in a["further_findings"]:
    assert all(f[k] in text for k in ("id", "where", "wrong", "fix"))
print(f"{OUT_A}: {len(out)} lines, sha256 {dig_a}; {len(a['rulings'])} rulings "
      f"({sum(1 for x in a['rulings'] if x['changes_diff'])} change the diff); "
      f"further findings {[f['id'] + ' ' + f['severity'] for f in a['further_findings']]}")

out = ["# BRAWL-1 review round 1: the four seats' results (workflow wf_bbfeece0-4c7), 2026-11-15", "",
       "Written by script, word for word, from the workflow's journal. Snapshot reviewed: wip/brawl1.patch ee2973aa...; "
       "packet brawl1-packet.md 16eab462.... Each seat's notes: brawl1-review-<seat>.md. Nothing below is the main "
       "session's.", ""]
sev = {}
for k, t in SEATS:
    v = res[f"review-{k}"]
    assert SNAP in v["snapshot"], (k, v["snapshot"][:120])
    out += [f"## review-{k} ({v['verdict']}; snapshot {SNAP[:12]})", "", f"**Seat:** {v['seat']}", ""]
    if v["snapshot"] != SNAP:
        out += [f"**Its snapshot field, whole:** {v['snapshot']}", ""]
    if not v["findings"]:
        out += ["**Findings:** none.", ""]
    for f in v["findings"]:
        sev[f["severity"]] = sev.get(f["severity"], 0) + 1
        out += [f"### {f['id']} ({f['severity']})", "", f"- **Where:** {f['where']}", f"- **Wrong:** {f['wrong']}",
                f"- **Evidence:** {f['evidence']}", f"- **Fix:** {f['fix']}", ""]
text, dig_r = save(OUT_R, out)
for k, _ in SEATS:
    v = res[f"review-{k}"]
    assert v["seat"] in text and v["snapshot"] in text
    for f in v["findings"]:
        assert all(f[x] in text for x in ("id", "where", "wrong", "evidence", "fix"))
print(f"{OUT_R}: {len(out)} lines, sha256 {dig_r}; verdicts "
      f"{[(k, res['review-' + k]['verdict']) for k, _ in SEATS]}; findings {sev}")
for k, _ in SEATS:
    for f in res[f"review-{k}"]["findings"]:
        if f["severity"] in ("Critical", "High", "Medium"):
            print(f"  {f['severity']}: {f['id']} ({k})")
