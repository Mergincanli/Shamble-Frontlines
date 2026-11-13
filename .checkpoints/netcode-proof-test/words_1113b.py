"""Record Mustafa's words of 2026-11-13 about main's PR #48 (his message of 18:13:55 UTC) in history.md among his
decisions, word for word: the text is read from the session's own transcript record (its uuid, timestamp, first and
last words asserted), never retyped. Only the line breaks change: each of his lines starts a line, blank lines are
dropped and long lines wrap at 120 characters, as in words_1113.py. Check: the entry's quoted text with whitespace
collapsed equals his message with whitespace collapsed. The main session's own lines under it (Read as) follow his
words, marked as the main session's.
Placement: appended at the end of history.md under its own heading "## Decision: ...", as "## Decisions taken"'s
heading line says, so no line of history.md moves.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/words_1113b.py [--apply]"""
import json
import re
import sys
import textwrap

T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
UUID = "3db40f57-1812-4e12-885a-90564c362f85"
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
assert rec and rec["type"] == "user" and rec["timestamp"] == "2026-11-13T18:13:55.069Z", rec
text = rec["message"]["content"]
assert isinstance(text, str), type(text)
assert text.startswith("My words of 2026-11-13. Main moved again: PR #48"), text[:50]
assert text.endswith("Record these words in history.md under my decisions."), text[-60:]
assert "4d21c1a7a4d39613bffce5d5f9bbc92f2f88a300" in text


def wrap(s, first, rest):
    return textwrap.wrap(s, width=120, initial_indent=first, subsequent_indent=rest, break_long_words=False,
                         break_on_hyphens=False)


label = '- **Mustafa, 2026-11-13 about 18:14 UTC (verbatim):** "'
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
    ("    - ", "      ", "MAIN-48: main's PR #48 (foundation step 2.2, the toolchain) is merged as 4d21c1a. Main "
     "(origin/main, 4d21c1a or later) is merged into this branch at the next clean stop: the point where no step is "
     "half done, never by stopping a running agent. Here that is after L4's script returns and its commit, before "
     "L6's launch, so U4r's builder starts on the merged state. docs/STATUS.md's conflict keeps both sides, no line "
     "retyped, within its 120 lines (as at the PR #49 and PR #51 merges). The spike keeps its own workspace "
     "(spikes/netcode-proof/Cargo.toml); main's root workspace and its CI job `toolchain` are main's, and the job "
     "also runs on this branch's pushes. The report to him: two lines, when main is in (the merge commit) and "
     "whether the `toolchain` job is green on this branch."),
]
for first, rest, s in mine:
    entry += wrap(s, first, rest)
assert all(len(l) <= 120 for l in entry), [l for l in entry if len(l) > 120]
HEAD = "## Decision: Mustafa, 2026-11-13 about 18:14 UTC (MAIN-48)"
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
