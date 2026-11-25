# Compare the DECISIONS quoted block with the transcript message 30f4ab9b-..., joined text blocks.
import json, hashlib, subprocess, sys
UUID = "30f4ab9b-ffde-4959-beef-3d3e5c015d43"
T = "/root/.claude/projects/-home-user/a7e7f150-3195-52bd-a549-73b27426198b.jsonl"
msg = None; n = 0
with open(T, encoding="utf-8") as f:
    for line in f:
        try: o = json.loads(line)
        except Exception: continue
        if o.get("uuid") == UUID:
            n += 1; msg = o
print("matches for uuid:", n, "type:", msg.get("type"), "role:", msg.get("message", {}).get("role"), "ts:", msg.get("timestamp"))
c = msg["message"]["content"]
if isinstance(c, str):
    parts = [c]
else:
    print("block types:", [b.get("type") for b in c])
    parts = [b["text"] for b in c if b.get("type") == "text"]
for sep_name, sep in (("''", ""), ("'\\n'", "\n"), ("'\\n\\n'", "\n\n")):
    t = sep.join(parts)
    b = t.encode("utf-8")
    print("join", sep_name, "bytes", len(b), "sha256", hashlib.sha256(b).hexdigest())
t = "".join(parts) if len(parts) == 1 else None
# DECISIONS block at head
dec = subprocess.run(["git", "show", "1c45ed7:docs/DECISIONS.md"], capture_output=True, text=True, cwd="/home/user/Shamble-Frontlines").stdout.split("\n")
start = next(i for i, l in enumerate(dec) if "message 30f4ab9b" in l)
# quoted block: consecutive lines starting with '>' after start
i = start
while not dec[i].startswith(">"): i += 1
q = []
while i < len(dec) and dec[i].startswith(">"):
    q.append(dec[i]); i += 1
unq = [l[2:] if l.startswith("> ") else l[1:] for l in q]
block = "\n".join(unq)
bb = block.encode("utf-8")
print("DECISIONS quoted lines:", len(q), "bytes", len(bb), "sha256", hashlib.sha256(bb).hexdigest())
for p in parts:
    print("equal to a single text block:", p == block, "| equal after rstrip:", p.rstrip("\n") == block)
print("joined('') equal:", "".join(parts) == block, "| joined('\\n') equal:", "\n".join(parts) == block)
