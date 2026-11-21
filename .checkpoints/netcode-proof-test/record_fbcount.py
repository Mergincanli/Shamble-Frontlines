"""Append Mustafa's F-B-COUNT answer (typed into this chat; it reached the session at 2026-11-21 19:42:15 UTC, 20:42
Amsterdam) to history.md's end, word for word: the text is copied from the transcript's queued-message entry (uuid
db7781ba-863b-41bb-9adb-734fb9e728dd, origin human), never retyped. The main session's reading follows, marked as such.
Refuses to run twice. Run from the repo root:
python3 -I .checkpoints/netcode-proof-test/record_fbcount.py <transcript.jsonl>"""
import hashlib
import json
import sys

H = ".checkpoints/netcode-proof-test/history.md"
UUID = "db7781ba-863b-41bb-9adb-734fb9e728dd"
HEAD = ("## Decision: Mustafa, 2026-11-21 20:42 Amsterdam (it reached this chat at 19:42:15 UTC), in this chat: "
        "F-B-COUNT (a) (word for word)")

words = None
for line in open(sys.argv[1], encoding="utf-8", errors="strict"):
    if UUID not in line:
        continue
    ev = json.loads(line)
    if ev.get("uuid") != UUID:
        continue
    att = ev["attachment"]
    assert att["type"] == "queued_command", att["type"]
    assert att["origin"] == {"kind": "human"}, att["origin"]
    assert att["humanTurn"] is True
    assert att["timestamp"] == "2026-11-21T19:42:15.771Z", att["timestamp"]
    words = att["prompt"]
assert words is not None, "the entry was not found"
assert words.startswith("F-B-COUNT: (a). ") and "```" not in words

hist = open(H, encoding="utf-8").read()
assert HEAD not in hist, "already recorded"
block = (
    "\n" + HEAD + "\n"
    "- **Mustafa, typed into this chat (transcript 2026-11-21T19:42:15.771Z, a queued message during a turn, uuid "
    + UUID + ", origin human), verbatim** (the whole message, unchanged, in the fence below):\n\n"
    "```text\n" + words + "\n```\n\n"
    "- **Read as** (the main session's reading, not his words): F-B-COUNT's option (a) as the advisor's paste-ready text "
    "words it (advisor-u4f2-r1.md:357-361), which his words restate: kind 7's presses are left out of its `sent_count` "
    "and `unrejected_count` when the server's World shows the forger's soldier dead at the run of that command (d) or "
    "of the one before it (d-1); an item whose d-1 never ran is not left out; the number left out, "
    "`dead_left_out_count`, is written on every `forged_kind` record of every run (0 for the other kinds), and -21's "
    "set report prints it beside the check for every run. FP-2 builds it as the ruling specifies (:404); then R5's "
    "forged lines, R2 and R3 whole, the lag-switch A line again (advisor-u4f2-parta.md), round 2 (netcode, test, rust; "
    "xhigh; the netcode seat reads FP-2's window against these words: :313), R6, the merge, the r2 rows and -21's set. "
    "The reading joins A1's on the readings list, and -21's report names it in a line of its own with who reviewed it "
    "(:366-367). It narrows ADR 0001:1611's \"every forged command\" and what SERIES.md:158's `sent_count` counts for "
    "kind 7 (his, :376); no pass mark, judge rule, marks.json text or existing SERIES.md row text changes; one new "
    "SERIES.md row, `dead_left_out_count`, in U4f2's own subsection (:369-370).\n"
)
new = hist.rstrip("\n") + "\n" + block
open(H, "w", encoding="utf-8").write(new)
first = hist.rstrip("\n").count("\n") + 3
print(f"history.md now {new.count(chr(10))} lines; the block at :{first}-{new.count(chr(10))}; "
      f"his words sha256 {hashlib.sha256(words.encode()).hexdigest()[:16]}; block sha256 "
      f"{hashlib.sha256(block.encode()).hexdigest()[:16]}")
