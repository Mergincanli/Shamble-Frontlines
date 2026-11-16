"""Append Mustafa's "yes" of 2026-11-16 (22:09 Amsterdam, 21:09 UTC) to NS-LINES (a) to history.md under his decisions,
word for word: his whole chat message (the user message whose text is exactly "yes", the one typed after the
assistant message that asked NS-LINES), and the three NIGHT_SHIFT.md lines he said yes to, copied from that
assistant message's first fenced block, with their sha256 (the freeze for this change: any change to them goes back
to him). Only the heading and the labels are the script's. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/mustafa_ns_lines_yes_words.py"""
import hashlib
import json

T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
H = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/history.md"
ASK = "**NS-LINES:** yes or no to the three lines above, exactly as written."
rows = []
with open(T, encoding="utf-8") as fh:
    for line in fh:
        try:
            rows.append(json.loads(line))
        except json.JSONDecodeError:
            continue


def texts(r):
    c = (r.get("message") or {}).get("content")
    if isinstance(c, str):
        return [c]
    if isinstance(c, list):
        return [b.get("text", "") for b in c if isinstance(b, dict) and b.get("type") == "text"]
    return []


asks = [(i, t) for i, r in enumerate(rows) if r.get("type") == "assistant" for t in texts(r) if ASK in t]
assert len(asks) == 1, f"expected one NS-LINES question, found {len(asks)}"
ai, ask = asks[0]
yes = [(rows[i].get("timestamp"), t) for i in range(ai + 1, len(rows)) if rows[i].get("type") == "user"
       for t in texts(rows[i]) if t.strip()]
assert yes and yes[0][1] == "yes", f"the first user text after the question is {yes[0][1]!r}" if yes else "no reply"
ts, words = yes[0]
block = ask.split("```\n", 2)[1].rstrip("\n")
lines = block.split("\n")
assert len(lines) == 3 and lines[0].startswith("   While the run waits on agents"), lines
sha = hashlib.sha256(block.encode("utf-8")).hexdigest()
with open(H, encoding="utf-8") as fh:
    text = fh.read()
head = "## Decision: Mustafa, 2026-11-16 about 21:52 Amsterdam, NS-LINES (a) (the three NIGHT_SHIFT.md lines)"
assert head not in text, "already recorded"
add = ["", head, f"- **Mustafa, typed into this chat (transcript {ts}), verbatim:** \"{words}\"",
       f"- **What he said yes to** (the assistant's question at {rows[ai].get('timestamp')}: \"{ASK}\"): these three "
       f"lines, copied by this script from that message's first fenced block, sha256 {sha}:", "", "```",
       *lines, "```", ""]
new = text.rstrip("\n") + "\n" + "\n".join(add)
with open(H, "w", encoding="utf-8") as fh:
    fh.write(new)
with open(H, "rb") as fh:
    data = fh.read()
first = text.rstrip("\n").count("\n") + 3
print(f"{H}: +{len(add)} lines (heading at line {first}); his yes at {ts}; the lines' sha256 {sha}; "
      f"history.md sha256 {hashlib.sha256(data).hexdigest()[:12]}")
