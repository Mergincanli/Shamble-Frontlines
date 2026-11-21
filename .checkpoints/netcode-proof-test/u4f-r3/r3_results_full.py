"""U4f's brief rev 3: save the four results of workflow wf_e78b24a2-736 word for word from its journal (the agents'
return values, unchanged), one file per agent: a text result as the exact text (.md), a structured result
pretty-printed as JSON (.json). Each file is read back and compared with the journal's value; prints each file's
sha256 and line count. A word-for-word copy, so a script writes it. Run from the repo root:
  python3 -I .checkpoints/netcode-proof-test/u4f-r3/r3_results_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_e78b24a2-736/journal.jsonl")
D = ".checkpoints/netcode-proof-test/u4f-r3/"
OUT = {
    "draft:u4f-r3": D + "draft-r3-full",
    "check-te:u4f-r3": D + "check-te-r3-full",
    "check-sa:u4f-r3": D + "check-sa-r3-full",
    "revise:u4f-r3": D + "revise-r3-full",
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
    val = results[key]
    if isinstance(val, str):
        path, text = OUT[label] + ".md", val
    else:
        path, text = OUT[label] + ".json", json.dumps(val, ensure_ascii=False, indent=1) + "\n"
    with open(path, "x", encoding="utf-8") as fh:
        fh.write(text)
    back = open(path, encoding="utf-8").read()
    assert (back == val) if isinstance(val, str) else (json.loads(back) == val)
    print(f"{path}: {back.count(chr(10))} lines, sha256 {hashlib.sha256(back.encode()).hexdigest()}")
