"""Append Mustafa's U4F2-SIZE answer to history.md's end, word for word, with the question it answers (the main
session's words, quoted from its chat reply, so the record says what "A" chose). Refuses to run twice. Run from the
repo root:  python3 -I .checkpoints/netcode-proof-test/record_u4f2_size.py"""
import hashlib

H = ".checkpoints/netcode-proof-test/history.md"
HEAD = "## Decision: Mustafa, 2026-11-20 15:10 Amsterdam (it reached this chat at 14:10:29 UTC), U4F2-SIZE (a): build U4f2 whole"
WORDS = "A"
QUESTION = """**U4F2-SIZE** (stopped on you; already in the 2026-11-20 message): how much to build in U4f2, the separate unit for the forged-command test and -21's runs. It matters because U4f2's build, the next step, waits for this answer.
  - (a) Build it whole: about 1,700 to 2,250 added lines, against the 1,500 your answer assumed. The stop at 3,000 lines stays.
  - (b) Leave four small add-ons (about 80 to 215 lines) to a later unit.
  - My lean, and the advisor's: (a)."""

hist = open(H, encoding="utf-8").read()
assert HEAD not in hist, "already recorded"
block = (
    "\n" + HEAD + "\n"
    "- **Mustafa, typed into this chat (transcript 2026-11-20T14:10:29.509Z, uuid 11fc896d-a8c0-4716-9932-1cfb49495518, "
    "origin human), verbatim** (the whole message, unchanged, in the fence below):\n\n"
    "```text\n" + WORDS + "\n```\n\n"
    "- **The question it answers** (the main session's words, its chat reply of about 11:00 UTC, quoted so the record says "
    "what \"A\" chose; not his words):\n\n"
    "```text\n" + QUESTION + "\n```\n\n"
    "- **Read as** (the main session's reading, not his words): U4F2-SIZE (a). U4f2 is built whole by u4f2-brief.md rev 1.2: "
    "about 1,700 to 2,250 added lines (up to about 2,700 at U4f's 1.2), its four riders included (C-U4F2-7 with T-6, "
    "C-U4F2-8, ADV-U4FP-3); the stop past 3,000 added lines at a block end stays (his SIZE-CHECK, history.md:2048-2051); "
    "no pass mark, judge rule, marks.json text or run count changes (his SIZE-U4F (a), history.md:6896-6903).\n"
)
new = hist.rstrip("\n") + "\n" + block
open(H, "w", encoding="utf-8").write(new)
first = hist.rstrip("\n").count("\n") + 3
print(f"history.md now {new.count(chr(10))} lines; the block at :{first}-{new.count(chr(10))}; "
      f"his words sha256 {hashlib.sha256(WORDS.encode()).hexdigest()[:16]}; block sha256 "
      f"{hashlib.sha256(block.encode()).hexdigest()[:16]}")
