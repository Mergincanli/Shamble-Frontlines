"""Append Mustafa's words of 2026-11-16 21:50 Amsterdam (NS-STATUS (a): a small kit change to .claude/NIGHT_SHIFT.md
as its own pull request after C62H-BOUND's P2 and P3 runs, its exact lines to him first, his yes also KIT-FREEZE-2's
exception for this one change, one review round, then a stop for his go on the merge; and, on the 17:09-20:36 UTC
stall, the approval-free command forms) to history.md under his decisions, word for word: the whole text of his chat
message, which reached this session while a turn ran (the transcript keeps it as an "attachment" record of type
queued_command, and as the matching queue-operation "enqueue" record), starting "My words of 2026-11-16, 21:50
(Amsterdam time)". Only the heading and the label are the script's. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/mustafa_ns_status_words.py"""
import hashlib
import json

T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
H = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/history.md"
START = "My words of 2026-11-16, 21:50 (Amsterdam time)"
found, queued = [], []
with open(T, encoding="utf-8") as fh:
    for line in fh:
        try:
            r = json.loads(line)
        except json.JSONDecodeError:
            continue
        a = r.get("attachment") if r.get("type") == "attachment" else None
        if isinstance(a, dict) and a.get("type") == "queued_command" and str(a.get("prompt", "")).startswith(START):
            found.append((r.get("timestamp"), a["prompt"]))
        if r.get("type") == "queue-operation" and r.get("operation") == "enqueue" \
                and str(r.get("content", "")).startswith(START):
            queued.append(r["content"])
assert len(found) == 1, f"expected one message, found {len(found)}"
ts, words = found[0]
assert queued == [words], "the enqueue record does not hold the same text"
assert "NS-STATUS: (a)." in words and "KIT-FREEZE-2" in words
with open(H, encoding="utf-8") as fh:
    text = fh.read()
head = ("## Decision: Mustafa, 2026-11-16 21:50 Amsterdam (20:50 UTC), NS-STATUS (a) (a NIGHT_SHIFT.md change after "
        "C62H-BOUND's P2 and P3 runs) and the approval-free command forms")
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
