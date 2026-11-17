"""DOCS-54's final-head review: save the two results of workflow wf_1ea0a6c1-69d word for word from its journal (the
agents' return values, unchanged, pretty-printed as JSON), one file per agent, and print each file's sha256 and line
count. A word-for-word copy, so a script writes it. Run from the repo root:
  python3 -I .checkpoints/netcode-proof-test/docs-54/docs54_final_full.py"""
import hashlib
import json

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_1ea0a6c1-69d/journal.jsonl")
OUT = {
    "advisor:pr55-aea6f5b": ".checkpoints/netcode-proof-test/docs-54/advisor-docs54-final-full.json",
    "verifier:docs-54-fix-pass": ".checkpoints/netcode-proof-test/docs-54/verifier-docs54-final-full.json",
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
    with open(OUT[label], "w", encoding="utf-8") as fh:
        fh.write(text)
    back = open(OUT[label], encoding="utf-8").read()
    assert json.loads(back) == results[key]
    print(f"{OUT[label]}: {back.count(chr(10))} lines, sha256 {hashlib.sha256(back.encode()).hexdigest()}")
