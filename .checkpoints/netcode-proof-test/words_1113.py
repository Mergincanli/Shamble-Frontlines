"""Record Mustafa's words of 2026-11-13 (his message after his /compact, 11:31:11 UTC) in history.md among his
decisions, word for word: the text is read from the session's own transcript record (its uuid, timestamp, first and
last words asserted), never retyped. Only the line breaks change: each of his lines starts a line, blank lines are
dropped and long lines wrap at 120 characters, as in words_1112.py. Check: the entry's quoted text with whitespace
collapsed equals his message with whitespace collapsed. The main session's own lines under it (Read as; Done) follow
his words, marked as the main session's.
Placement: appended at the end of history.md under its own heading "## Decision: ...", as "## Decisions taken"'s
heading line says, so no line of history.md moves.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/words_1113.py [--apply]"""
import json
import re
import sys
import textwrap

T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
UUID = "dc3e6139-e829-4e9c-819a-ba4828c3fb96"
H = ".checkpoints/netcode-proof-test/history.md"
rec = None
with open(T, encoding="utf-8") as fh:
    for line in fh:
        if UUID not in line:
            continue
        r = json.loads(line)
        if r.get("uuid") == UUID:
            assert rec is None, "two records with the uuid"
            rec = r
assert rec and rec["type"] == "user" and rec["timestamp"] == "2026-11-13T11:31:11.746Z", rec
text = rec["message"]["content"]
assert isinstance(text, str), type(text)
assert text.startswith("You were just compacted to cut usage."), text[:40]
assert text.endswith("Record these words in history.md under my decisions."), text[-60:]
assert text.count("My words of 2026-11-13.") == 1


def wrap(s, first, rest):
    return textwrap.wrap(s, width=120, initial_indent=first, subsequent_indent=rest, break_long_words=False,
                         break_on_hyphens=False)


label = '- **Mustafa, 2026-11-13 about 11:31 UTC, after his /compact (verbatim):** "'
src = [l.strip() for l in text.split("\n") if l.strip()]
entry = []
for k, s in enumerate(src):
    if k == len(src) - 1:
        s += '"'
    m = re.match(r"^(- |\d+\. )", s)
    rest = "  " + " " * len(m.group(1)) if m else "  "
    w = wrap(label + s, "", "  ") if k == 0 else wrap(s, "  ", rest)
    # A wrapped line must not start like a list item his line did not start with.
    for x in w[1:]:
        assert not re.match(r"^\s*(- |\d+\. )", x), x
    entry += w
body = "\n".join(entry)
assert body.startswith(label) and body.endswith('"')
assert " ".join(body[len(label):-1].split()) == " ".join(text.split()), "the words differ"
mine = [
    ("  - ", "    ", "Read as (the main session):"),
    ("    - ", "      ", "CHECK-IN: while any agent or workflow of this session runs, one scheduled check-in into "
     "this session comes due about every two hours; while none runs, it is paused, so there is never more than one. "
     "A check-in only looks whether a cut stopped something (the spend limit's cut; a container restart stops runs "
     "the same way, so it is looked for too), restores and resumes it by the card as SPEND-LIMIT (a) says (the "
     "notice's exact text, the time of the cut and the time of the resume on the card's block \"Cuts by the spend "
     "limit\"), and ends: it starts no new work. A check-in that comes due while the limit still holds is expected "
     "not to run; the next one comes due two hours later, so the work should resume within about two hours of the "
     "reset (not yet seen on a real cut)."),
    ("  - ", "    ", "Done: the scheduled task \"npt check-in (every 2 h while agents run)\" "
     "(trig_01EKqNqwZrQTVJ6caAyRrug5), bound to this session, created 11:34:38 UTC, schedule `34 */2 * * *` (UTC: "
     "12:34, 14:34, and so on; the server set the minute from the time it was made). Its prompt holds three steps: "
     "look whether a cut stopped something; if nothing did, one line and end; if something did, restore and resume "
     "it by the card, the cut recorded, the saver restarted if it stopped, the card saved, then end. It is paused "
     "while no agent runs and enabled before each launch (the card's saver line says so). The schedule could be "
     "made, so there is nothing to tell him about it."),
]
for first, rest, s in mine:
    entry += wrap(s, first, rest)
assert all(len(l) <= 120 for l in entry), [l for l in entry if len(l) > 120]
HEAD = "## Decision: Mustafa, 2026-11-13 about 11:31 UTC, after his /compact (CHECK-IN)"
POINTER = ('## Decisions taken (to 2026-11-11, newest first; later ones are appended at the end, each under '
           '"## Decision: …")')
with open(H, encoding="utf-8") as fh:
    hist = fh.read().split("\n")
assert hist.count(POINTER) == 1, "the heading's pointer is not as words_1112.py left it"
assert label not in "\n".join(hist), "already recorded"
assert HEAD not in hist
assert hist[-1] == "", "history.md ends without a newline"
n_before = len(hist)
new = hist[:-1] + ["", HEAD] + entry + [""]
assert new[:n_before - 1] == hist[:-1], "a line of history.md moved"
print("\n".join([HEAD] + entry))
print(f"{len(entry) + 2} lines appended after line {n_before - 1}; his words {len(src)} lines of his, checked equal; "
      "no earlier line moved")
if "--apply" in sys.argv:
    with open(H, "w", encoding="utf-8") as fh:
        fh.write("\n".join(new))
    print("applied")
else:
    print("dry run: nothing written")
