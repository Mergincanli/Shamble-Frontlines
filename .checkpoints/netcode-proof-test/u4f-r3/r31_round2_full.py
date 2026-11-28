"""U4f's brief rev 3.1, round 2 and D4b: save the three results of workflow wf_997ea01f-b4f word for word from its
journal (the agents' return values, unchanged, pretty-printed as JSON), one file per agent; each file is read back and
compared with the journal's value; prints each file's sha256 and line count. A word-for-word copy, so a script writes
it. Run from the repo root:
  python3 -I .checkpoints/netcode-proof-test/u4f-r3/r31_round2_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_997ea01f-b4f/journal.jsonl")
D = ".checkpoints/netcode-proof-test/u4f-r3/"
OUT = {
    "r2-te:u4f-r31": D + "r2-te-r31-full.json",
    "r2-sa:u4f-r31": D + "r2-sa-r31-full.json",
    "d4b:u4f-r31": D + "d4b-r31-full.json",
}
labels, results = {}, {}
for line in open(J, encoding="utf-8"):
    r = json.loads(line)
    if r.get("type") == "started":
        labels[r["key"]] = r["label"]
    elif r.get("type") == "result":
        results[r["key"]] = r["result"]
assert set(results) == set(labels) and sorted(labels.values()) == sorted(OUT), (labels, list(results))
for key, label in labels.items():
    text = json.dumps(results[key], ensure_ascii=False, indent=1) + "\n"
    with open(OUT[label], "x", encoding="utf-8") as fh:
        fh.write(text)
    back = open(OUT[label], encoding="utf-8").read()
    assert json.loads(back) == results[key]
    print(f"{OUT[label]}: {back.count(chr(10))} lines, sha256 {hashlib.sha256(back.encode()).hexdigest()}")
