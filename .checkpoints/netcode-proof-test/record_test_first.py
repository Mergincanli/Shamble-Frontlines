"""Record Mustafa's message of 2026-11-17 ("My words of 2026-11-17, 20:20 (Amsterdam time)") word for word in
history.md: the last main-chain user message that starts with those words, copied from this session's transcript into
a fenced block, unchanged. Refuses to run twice. Run from the repo root:
  python3 -I .checkpoints/netcode-proof-test/record_test_first.py"""
import hashlib
import json

T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
H = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/history.md"
START = "My words of 2026-11-17, 20:20 (Amsterdam time)"
END = "4. Go on with U4f. You do not wait for my answer to that message."


def text_of(r):
    c = (r.get("message") or {}).get("content")
    if isinstance(c, str):
        return c
    if isinstance(c, list):
        return "".join(b.get("text", "") for b in c if isinstance(b, dict) and b.get("type") == "text")
    return ""


hits = []
with open(T, encoding="utf-8", errors="replace") as fh:
    for line in fh:
        try:
            r = json.loads(line, strict=False)
        except Exception:
            continue
        if isinstance(r, dict) and r.get("type") == "user" and not r.get("isSidechain"):
            t = text_of(r)
            if t.startswith(START):
                hits.append((r.get("timestamp"), r.get("uuid"), t))
assert len(hits) == 1, [h[:2] for h in hits]
ts, uuid, said = hits[0]
said = said.rstrip("\n")
assert said.endswith(END), repr(said[-80:])
assert "```" not in said
with open(H, encoding="utf-8") as fh:
    old = fh.read()
assert START not in old, "already recorded"
assert old.endswith("toolchain run 44133861577).\n"), repr(old[-60:])
heading = (f"## Decision: Mustafa, 2026-11-17 20:20 Amsterdam by his own label (it reached this chat at "
           f"{ts[11:19]} UTC), TEST-FIRST (how this chat works until the netcode proof test closes)")
block = (
    f"\n{heading}\n"
    f"- **Mustafa, typed into this chat (transcript {ts}, uuid {uuid}), verbatim** (the whole message, unchanged, "
    "in the fence below):\n\n"
    "```text\n" + said + "\n```\n\n"
    "- **Named** TEST-FIRST on the card (the main session's name for it, not his words).\n"
)
new = old + block
with open(H, "w", encoding="utf-8") as fh:
    fh.write(new)
print(f"history.md: {old.count(chr(10))} -> {new.count(chr(10))} lines; message sha256 "
      f"{hashlib.sha256(said.encode()).hexdigest()[:16]}, {said.count(chr(10)) + 1} lines; block sha256 "
      f"{hashlib.sha256(block.encode()).hexdigest()[:16]}")
print(heading)
