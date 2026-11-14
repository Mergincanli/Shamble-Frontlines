"""Record Mustafa's words of 2026-11-14 (his message of 11:07:23 UTC, after his /compact: SIZE-U4R, SIZE-CHECK, the
/compact wait, main at 4460ef5) in history.md among his decisions, word for word: the text is read from the session's
own transcript record (its uuid, timestamp, first and last words asserted), never retyped. Only the line breaks
change: each of his lines starts a line, blank lines are dropped and long lines wrap at 120 characters, as in
words_1113b.py. Check: the entry's quoted text with whitespace collapsed equals his message with whitespace collapsed.
The main session's own lines under it (Read as) follow his words, marked as the main session's; their short codes
(COMPACT-WAIT, MAIN-52) are the main session's labels, not his.
Placement: appended at the end of history.md under its own heading "## Decision: ...", as "## Decisions taken"'s
heading line says, so no line of history.md moves.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/words_1114.py [--apply]"""
import json
import re
import sys
import textwrap

T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
UUID = "87982fa0-e805-4e36-b108-6847c60f56cb"
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
assert rec and rec["type"] == "user" and rec["timestamp"] == "2026-11-14T11:07:23.652Z", rec
text = rec["message"]["content"]
assert isinstance(text, str), type(text)
assert text.startswith("You were just compacted to cut usage. Read the resume card"), text[:60]
assert text.endswith("Record these words in history.md under my decisions."), text[-60:]
assert "My words of 2026-11-14." in text and "4460ef5b8d6dc8566b29c15ded75c6f0c8be386b" in text
assert "1. SIZE-U4R: (a)." in text and "2. SIZE-CHECK from now on." in text


def wrap(s, first, rest):
    return textwrap.wrap(s, width=120, initial_indent=first, subsequent_indent=rest, break_long_words=False,
                         break_on_hyphens=False)


label = '- **Mustafa, 2026-11-14 about 11:07 UTC, after his /compact (verbatim):** "'
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
    ("    - ", "      ", "SIZE-U4R (a): the fix pass as the advisor's ruling on round 1 rules it "
     "(advisor-u4r-r1-full.md, \"The fix pass, in order\": FP-0 to FP-9, the 10 Mediums and the small Lows in the "
     "same files), not option (b). The question (advisor-u4r-r1-full.md:48-52) goes on the card with this answer; "
     "the card's ruling block had said \"Nothing for Mustafa beyond the report's plain story\", which missed it, and "
     "the main session's reports of the morning said \"Decisions needed: none\". When the fix pass is done (its "
     "snapshot cut), one line to him: how many lines U4r has then, counted as the ruling counted its about 7,600 "
     "(lines added by the snapshot's patch on 2fbec77: into existing files plus the new files' lines)."),
    ("    - ", "      ", "SIZE-CHECK, from now on: at every snapshot of a unit the main session counts the unit's "
     "lines that way and writes them on the card beside its planned size. \"Well past its plan\" is more than twice "
     "the plan. Where the plan gives only a size letter, the plan is what earlier units of that letter came to: M "
     "about 3,000 lines (his figure; U4c 3,130 and U4j 2,860), so an M unit is past at more than 6,000. For another "
     "letter the main session writes on the card, before the unit's first snapshot, what earlier units of that "
     "letter came to, with their sources. When a unit passes that line: stop and tell him in three lines (what "
     "grew, why, and the cheaper option), as his rule of 2026-11-10 says (history.md:398-399). U4r is past: 7,568 "
     "lines at snapshot 572c4f69 (6,282 in 13 new files, 1,286 inserted into 13 files; the ruling's about 7,600) for "
     "M; his SIZE-U4R answer (a) covers its fix pass, and its count still goes on the card at each snapshot."),
    ("    - ", "      ", "COMPACT-WAIT (the main session's label for his item 3): the main session still asks him "
     "to type /compact when the chat is at 35 to 40 % of its size, after refreshing the card (USAGE-SAVINGS (a)), but "
     "never waits for it: the work goes on. The ask's time goes on the card, and a one-time reminder into this "
     "session is set for 20 minutes after the ask, so a turn that ends after the ask cannot stand still. If he has "
     "not typed /compact by then, the card is refreshed and the work carries on; the system then compacts the chat by "
     "itself, and after that the card is read first, as after his /compact."),
    ("    - ", "      ", "MAIN-52 (the main session's label for his item 4): main is 4460ef5 (PR #52 merged: "
     "foundation step 2.3's first docs pass, eight files under docs/, no code and no kit, gate or CI file). It is "
     "not merged into this branch now. Main is merged in when main changes code, CI or rules (the kit, gate or CI "
     "files, a CLAUDE.md, .claude/ or .github/), and once before PR #40 is marked ready; each snapshot's card update "
     "looks at what main's new commits change. The card's Base line says so."),
]
for first, rest, s in mine:
    entry += wrap(s, first, rest)
assert all(len(l) <= 120 for l in entry), [l for l in entry if len(l) > 120]
HEAD = "## Decision: Mustafa, 2026-11-14 about 11:07 UTC, after his /compact (SIZE-U4R, SIZE-CHECK, /compact, main)"
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
