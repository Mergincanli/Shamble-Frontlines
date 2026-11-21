"""Append Mustafa's words of 2026-11-15 17:20 Amsterdam (KL-CHANGE-YES (a): yes to kl-change-v2.diff on top of
kept-logs-final-text.md; build it as set out; the ready report's three items; nothing real packed or deleted before
the PR is on main and this branch has taken main in) to history.md under his decisions, word for word: the whole text
of his chat message in this session's transcript that contains "My words of 2026-11-15, 17:20 (Amsterdam time)"
(its first line, about the compact and the usage, is his too and is kept). Only the heading and the label are the
script's. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/mustafa_kl_change_yes_words.py"""
import hashlib
import json

T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
H = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/history.md"
KEY = "My words of 2026-11-15, 17:20 (Amsterdam time)"
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
            if KEY in t:
                found.append((r.get("timestamp"), t))
assert len(found) == 1, f"expected one message, found {len(found)}"
ts, words = found[0]
assert words.startswith("You have just been /compact-ed") and "KL-CHANGE-YES: (a)." in words
with open(H, encoding="utf-8") as fh:
    text = fh.read()
head = ("## Decision: Mustafa, 2026-11-15 17:20 Amsterdam (16:20 UTC), KL-CHANGE-YES (a): yes to kl-change-v2.diff; "
        "build the kept-logs rule as set out")
assert head not in text, "already recorded"
add = ["", head, f"- **Mustafa, typed into this chat (transcript {ts}), verbatim:**", "",
       *["  " + l.rstrip(" ") if l.strip() else "" for l in words.split("\n")], ""]
new = text.rstrip("\n") + "\n" + "\n".join(add)
for l in words.split("\n"):
    assert l.rstrip(" ") in new
with open(H, "w", encoding="utf-8") as fh:
    fh.write(new)
with open(H, "rb") as fh:
    data = fh.read()
first = text.rstrip("\n").count("\n") + 3
print(f"{H}: +{len(add)} lines (heading at line {first}, last line {data.decode().rstrip(chr(10)).count(chr(10)) + 1}), "
      f"sha256 {hashlib.sha256(data).hexdigest()[:12]}; message at {ts}, {len(words)} chars")
