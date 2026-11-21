"""Append Mustafa's message of 2026-11-20 21:37:56 UTC (KEEP-PROGRESSING) to history.md's end, word for word, with the
main session's reading marked as such. Refuses to run twice. Run from the repo root:
python3 -I .checkpoints/netcode-proof-test/record_keep_progressing.py"""
import hashlib

H = ".checkpoints/netcode-proof-test/history.md"
HEAD = "## Decision: Mustafa, 2026-11-20 22:37 Amsterdam (it reached this chat at 21:37:56 UTC), KEEP-PROGRESSING"
WORDS = ("continue the work, the advisor will be answering the question in 2 days, until then keep progressing.\n"
         "Things that are very important which needs an answer: you can ask me")

hist = open(H, encoding="utf-8").read()
assert HEAD not in hist, "already recorded"
block = (
    "\n" + HEAD + "\n"
    "- **Mustafa, typed into this chat (transcript 2026-11-20T21:37:56.993Z, uuid 2d25342b-64a2-4e2d-9fb2-fe6faf675f5e, "
    "origin human), verbatim** (the whole message, unchanged, in the fence below):\n\n"
    "```text\n" + WORDS + "\n```\n\n"
    "- **Read as** (the main session's reading, not his words): \"the question\" is F-B-COUNT (advisor-u4f2-r1.md:347-367, "
    "sent in msg-2026-11-21.md), and U4G-Q1 waits with it; \"the advisor\" is his Advisor chat, whose answer counts once "
    "he pastes it (LANE-RULES' ACCOUNTS); until then all plan rev 3.3 work that does not wait on that answer goes on "
    "(U4f2's part A; U4h's brief; what the advisor agent rules may start). A very important question that needs his "
    "answer may go to him in the chat when it arises, beside TEST-FIRST's one decision message a day; everything else "
    "still goes in that day's message. No pass mark, judge, marks.json text, money, main or stop changes.\n"
)
new = hist.rstrip("\n") + "\n" + block
open(H, "w", encoding="utf-8").write(new)
first = hist.rstrip("\n").count("\n") + 3
print(f"history.md now {new.count(chr(10))} lines; the block at :{first}-{new.count(chr(10))}; "
      f"his words sha256 {hashlib.sha256(WORDS.encode()).hexdigest()[:16]}; block sha256 "
      f"{hashlib.sha256(block.encode()).hexdigest()[:16]}")
