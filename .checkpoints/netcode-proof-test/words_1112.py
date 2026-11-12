"""Record Mustafa's words of 2026-11-12 (his message after his /compact, 15:31:51 UTC) in history.md among his
decisions, word for word: the text is read from the session's own transcript record (its uuid, timestamp, first and
last words asserted), never retyped. Only the line breaks change: each of his lines starts a line, blank lines are
dropped and long lines wrap at 120 characters, as in the entries of "Decisions taken". Check: the entry's quoted text
with whitespace collapsed equals his message with whitespace collapsed. The main session's own lines under it (Read
as; its own addition; the list's additions; Done) follow his words, marked as the main session's.
Placement: appended at the end of history.md under its own heading "## Decision: ...", and the heading "## Decisions
taken" gains a pointer in its own line, so no line of history.md moves (the card, U4d2's brief, the advisor's rulings
and u4d2_fold.py cite history.md by line number). A first run put the entry at the top of "Decisions taken"; this run
takes it out from there (checked equal) before it appends.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/words_1112.py [--apply]"""
import hashlib
import json
import re
import sys
import textwrap

T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
UUID = "e51ffddd-bfc8-4340-9f1d-21179f54fa71"
H = ".checkpoints/netcode-proof-test/history.md"
RP = ".checkpoints/netcode-proof-test/review_packet.py"
rec = None
with open(T, encoding="utf-8") as fh:
    for line in fh:
        if UUID not in line:
            continue
        r = json.loads(line)
        if r.get("uuid") == UUID:
            assert rec is None, "two records with the uuid"
            rec = r
assert rec and rec["type"] == "user" and rec["timestamp"] == "2026-11-12T15:31:51.086Z", rec
text = rec["message"]["content"]
assert isinstance(text, str), type(text)
assert text.startswith("You were just compacted to cut usage.") and text.endswith("Then carry on."), text[-40:]
assert text.count("My words of 2026-11-12.") == 1


def wrap(s, first, rest):
    return textwrap.wrap(s, width=120, initial_indent=first, subsequent_indent=rest, break_long_words=False,
                         break_on_hyphens=False)


label = '- **Mustafa, 2026-11-12 about 15:32 UTC, after his /compact (verbatim):** "'
src = [l.strip() for l in text.split("\n") if l.strip()]
entry = []
for k, s in enumerate(src):
    if k == len(src) - 1:
        s += '"'
    m = re.match(r"^(- |\d+\. )", s)
    rest = "  " + " " * len(m.group(1)) if m else "  "
    if k == 0:
        w = wrap(label + s, "", "  ")
    else:
        w = wrap(s, "  ", rest)
    # A wrapped line must not start like a list item his line did not start with.
    for x in w[1:]:
        assert not re.match(r"^\s*(- |\d+\. )", x), x
    entry += w
body = "\n".join(entry)
assert body.startswith(label) and body.endswith('"')
assert " ".join(body[len(label):-1].split()) == " ".join(text.split()), "the words differ"
with open(RP, "rb") as fh:
    rp_sha = hashlib.sha256(fh.read()).hexdigest()
mine = [
    ("  - ", "    ", "Read as (the main session):"),
    ("    - ", "      ", "SPEND-LIMIT (a): the cap stays. After each cut the main session restores what a cut agent "
     "left (a plant, a half edit) and resumes, as this morning; the card's block \"Cuts by the spend limit\" gives, "
     "for each cut, the notice's exact text, the time of the cut and the time of the resume."),
    ("    - ", "      ", "review_packet.py: the \"advisor-\" exception gives way to ALLOWED, the exact paths that "
     "pass though the pattern forbids them: advisor-u4e-brief.md alone. Every other name the pattern forbids is "
     "refused: the main session's four briefs to the advisor, and advisor-u4d2-brief.md (the advisor's ruling, not "
     "needed by a packet yet). A file goes on the list only if the advisor agent wrote it as its ruling and a packet "
     "needs it; each addition is noted below. The packets already cut stay as they are; no review restarts."),
    ("    - ", "      ", "Main (PR #51, 2aa5e9c): merged into the branch at the next clean stop, no running agent "
     "stopped; docs/DECISIONS.md and docs/STATUS.md keep both sides, as for PR #49."),
    ("  - ", "    ", "The main session's own addition, which only refuses more: the flag for his words "
     "(`words_of_mustafa`) now passes history.md only; before, any flagged path passed the name check."),
    ("  - ", "    ", "Additions to ALLOWED (the exact name; that the advisor agent wrote it as its ruling; the packet "
     "that needed it): none yet."),
    ("  - ", "    ", f"Done: review_packet.py changed (sha256 {rp_sha[:8]}…). packet_name_check.py shows its name "
     "check on 11 paths (logs .gate/packet-name-check/): the new script 11 of 11 as wanted; the script before the "
     "change (153b76ea…, 1829b5f's rule) 5 of 11, the four briefs, advisor-u4d2-brief.md and a flagged report "
     "passing. The card's block \"Cuts by the spend limit\" holds today's three cuts (02:02, 06:41 and 09:24 UTC), "
     "read from the session's transcript. This entry is appended at the end of history.md under its own heading, "
     "and \"Decisions taken\" points here from its heading line, so no cited line of history.md moves."),
]
for first, rest, s in mine:
    entry += wrap(s, first, rest)
assert all(len(l) <= 120 for l in entry), [l for l in entry if len(l) > 120]
OLD_HEAD = "## Decisions taken"
NEW_HEAD = ('## Decisions taken (to 2026-11-11, newest first; later ones are appended at the end, each under '
            '"## Decision: …")')
HEAD = "## Decision: Mustafa, 2026-11-12 about 15:32 UTC, after his /compact (SPEND-LIMIT (a), review_packet.py, main)"
assert len(NEW_HEAD) <= 120 and len(HEAD) <= 120
with open(H, encoding="utf-8") as fh:
    hist = fh.read().split("\n")
heads = [i for i, l in enumerate(hist) if l == OLD_HEAD]
assert len(heads) == 1, heads
i = heads[0] + 1
# The first run's entry at the top of "Decisions taken": taken out, its lines checked equal to this run's, except the
# Done line's tail that this run adds (the first run's entry ends where the 16:45 entry begins).
if hist[i] == entry[0]:
    j = next(k for k in range(i, len(hist)) if hist[k].startswith("- **Mustafa, 2026-11-11 about 16:45 UTC"))
    old = hist[i:j]
    assert old[:-1] == entry[:len(old) - 1] and old[-1].startswith('    spend limit" holds today'), old[-3:]
    hist = hist[:i] + hist[j:]
    print(f"took out the first run's {len(old)} lines at history.md lines {i + 1}-{j}")
assert hist[i].startswith("- **Mustafa, 2026-11-11 about 16:45 UTC (verbatim):**"), hist[i]
assert label not in "\n".join(hist), "already recorded"
assert HEAD not in hist
n_before = len(hist)
hist[heads[0]] = NEW_HEAD
assert hist[-1] == "", "history.md ends without a newline"
new = hist[:-1] + ["", HEAD] + entry + [""]
print("\n".join([NEW_HEAD, "...", "", HEAD] + entry))
print(f"heading line {heads[0] + 1} renamed in place; {len(entry) + 2} lines appended after line {n_before - 1}; "
      f"his words {len(src)} lines of his, checked equal")
if "--apply" in sys.argv:
    with open(H, "w", encoding="utf-8") as fh:
        fh.write("\n".join(new))
    print("applied")
else:
    print("dry run: nothing written")
