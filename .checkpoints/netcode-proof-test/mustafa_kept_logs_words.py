"""Append Mustafa's words of 2026-11-15 08:20 Amsterdam (KEPT-LOGS (c) and the kit-rule route) to history.md under his
decisions, word for word: the text of his chat message in this session's transcript that starts with
"My words of 2026-11-15, 08:20 (Amsterdam time)". Only the heading and the label are the script's. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/mustafa_kept_logs_words.py"""
import hashlib
import json

T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
H = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/history.md"
START = "My words of 2026-11-15, 08:20 (Amsterdam time)"
found = []
with open(T, encoding="utf-8") as fh:
    for line in fh:
        try:
            r = json.loads(line)
        except json.JSONDecodeError:
            continue
        if r.get("type") != "user":
            continue
        c = (r.get("message") or {}).get("content")
        texts = [c] if isinstance(c, str) else [b.get("text", "") for b in c if isinstance(b, dict) and b.get("type") == "text"] if isinstance(c, list) else []
        for t in texts:
            if t.startswith(START):
                found.append((r.get("timestamp"), t))
assert len(found) >= 1, "message not found"
ts, words = found[-1]
with open(H, encoding="utf-8") as fh:
    text = fh.read()
head = "## Decision: Mustafa, 2026-11-15 08:20 Amsterdam (07:20 UTC), KEPT-LOGS (c) and the kit-rule route"
assert head not in text, "already recorded"
add = ["", head, f"- **Mustafa, typed into this chat (transcript {ts}), verbatim:**", "", *["  " + l if l else "" for l in words.split("\n")], ""]
new = text.rstrip("\n") + "\n" + "\n".join(add)
assert words.split("\n")[0] in new
with open(H, "w", encoding="utf-8") as fh:
    fh.write(new)
with open(H, "rb") as fh:
    print(f"{H}: +{len(add)} lines, sha256 {hashlib.sha256(fh.read()).hexdigest()[:12]}; message at {ts}, {len(words)} chars")
