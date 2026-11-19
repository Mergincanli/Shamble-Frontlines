"""Apply u4f-merge-edits.md's edits word for word (M8 of advisor-u4f-r1.md): each edit's "Old:" fenced block is replaced
by its "New:" fenced block in its file, in the drafted order. Each old text must occur exactly once at the moment it is
applied, or the script stops before writing anything. Edit 8 holds no blocks (a record). Refuses to run twice (the
first new text must not be present yet). Run from the repo root:
python3 -I .checkpoints/netcode-proof-test/apply_u4f_merge_edits.py"""
import hashlib
import re

N = ".checkpoints/netcode-proof-test/"
SRC = N + "u4f-merge-edits.md"
FILES = {"plan": N + "plan.md", "build": N + "u4-build-plan.md", "carries": N + "u4s-carries.md"}
TARGET = {"1": "build", "2": "build", "3": "build", "4": "build", "5": "build", "6a": "build", "6b": "build",
          "7a": "plan", "7b": "plan", "7c": "plan", "9": "carries", "10": "build"}
ORDER = ["1", "2", "3", "4", "5", "6a", "6b", "7a", "7b", "7c", "9", "10"]

lines = open(SRC, encoding="utf-8").read().split("\n")
blocks, cur, i = {}, None, 0
while i < len(lines):
    m = re.match(r"^#{2,3} Edit (\d+[a-z]?)\b", lines[i])
    if m:
        cur = m.group(1)
    if lines[i] in ("Old:", "New:") and cur is not None and i + 1 < len(lines) and lines[i + 1] == "```text":
        j = i + 2
        body = []
        while lines[j] != "```":
            body.append(lines[j])
            j += 1
        blocks.setdefault(cur, {})[lines[i][:-1].lower()] = "\n".join(body)
        i = j
    i += 1
assert sorted(blocks) == sorted(ORDER), sorted(blocks)
texts = {k: open(v, encoding="utf-8").read() for k, v in FILES.items()}
assert blocks["1"]["new"] not in texts["build"], "already applied"
for e in ORDER:
    f = TARGET[e]
    old, new = blocks[e]["old"], blocks[e]["new"]
    n = texts[f].count(old)
    assert n == 1, (e, f, n)
    texts[f] = texts[f].replace(old, new, 1)
for k, v in FILES.items():
    before = open(v, encoding="utf-8").read()
    if before != texts[k]:
        open(v, "w", encoding="utf-8").write(texts[k])
        print(f"{v}: {before.count(chr(10))} -> {texts[k].count(chr(10))} lines; sha256 "
              f"{hashlib.sha256(texts[k].encode()).hexdigest()[:16]}")
print("applied, in order:", ", ".join(ORDER))
