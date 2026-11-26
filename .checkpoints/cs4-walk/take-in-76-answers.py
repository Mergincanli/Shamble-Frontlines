#!/usr/bin/env python3
"""Builder V4, after the take-in of main 2671595 under pull request #75: his answers of 2026-11-26, 18:16:35 UTC, into
docs/STATUS.md and docs/STATUS_ARCHIVE.md (docs/DECISIONS.md is written by decisions-take-in-76.mjs, run before this).

From the worktree's root:  python3 .checkpoints/cs4-walk/take-in-76-answers.py
His words are read from the saved message (.checkpoints/cs4-walk/mustafa-2026-11-26-take-in-76.md), never typed.
STATUS keeps its 120 lines: one line changes, one bullet is laid out again with two more ids.
"""
import textwrap

saved = open(".checkpoints/cs4-walk/mustafa-2026-11-26-take-in-76.md").read().split("\n---\n", 1)[1].split("\n")
def his(start):
    line = [l for l in saved if l.startswith(start)]
    assert len(line) == 1, start
    return line[0][len(start):].strip()
sight, rate = his("4. SIGHT-FLAG:"), his("5. DUR-1:")
WHEN = "`docs/DECISIONS.md`, 2026-11-26, 18:16:35 UTC"

# The archive: two follow-ups in change-set 4b-1's block, each with his answer word for word; and the block's header
# names the pull request.
p = "docs/STATUS_ARCHIVE.md"
a = open(p).read()
closed = "  - Closed by 4b-1, at its merge: 4A-S3-READ's read side"
assert a.count(closed) == 1 and a.count("standing in for `cs/4b-content`; no pull request yet;") == 1
items = [
    f"- 4B-SIGHT-MARK (his SIGHT-FLAG, answered; {WHEN}; his words: \"{sight}\"): Slice 2's plan adds the \"blocks sight\" mark to the map kind, with the sight check that reads it. Until then every shape blocks movement, shots and sight alike; 4b-2's collision world is planned to keep, for every solid piece, the shape it came from (engine-architect's review of 4b-1), so that the mark needs no rebuild of the world.",
    f"- 4B-FIRE-RATE (his DUR-1, answered; {WHEN}; his words: \"{rate}\"): for change-set 8 (the guns): a gun's interval is its table value, with the timer's leftover carried (ADR 0004 part 4); no value is rewritten to match the old game's rate at 60 frames a second.",
]
wrapped = "\n".join(textwrap.fill(i, width=118, initial_indent="  ", subsequent_indent="    ", break_long_words=False, break_on_hyphens=False) for i in items)
a = a.replace(closed, wrapped + "\n" + closed)
a = a.replace("standing in for `cs/4b-content`; no pull request yet;", "standing in for `cs/4b-content`; pull request #75;")
archive_path, archive_text = p, a

# STATUS: the waiting bullet says what he answered; the follow-ups bullet names the two new ids.
p = "docs/STATUS.md"
s = open(p).read().split("\n")
before = len(s)
at = next(i for i, l in enumerate(s) if l.startswith("- 4b-1 (#75), reviewed (two rounds"))
s[at] = f"- 4b-1 (#75), reviewed (its report): his go. Answered ({WHEN}): SIGHT-FLAG (a), DUR-1 (C), STATUS-LEN (a)."
first = next(i for i, l in enumerate(s) if l.startswith("- **From change-set 4b-1**"))
last = next(i for i in range(first, len(s)) if s[i] == "")
text = " ".join(l.strip() for l in s[first:last])
old_head = "- **From change-set 4b-1** (the archive's block \"From change-set 4b-1\"; the 4B- ids are its docs pass's):"
old_tail = " And 14 more ids there (4B-CI-CONTENT is the CI Builder's)."
assert text.startswith(old_head) and text.endswith(old_tail)
reach = text[len(old_head):-len(old_tail)]          # 4B-AIM-REACH's sentences, word for word
new = "- **From change-set 4b-1** (the archive's block of that name; ids by its docs pass):" + reach + " And 16 more ids there, among them 4B-SIGHT-MARK (Slice 2's plan adds the mark), 4B-FIRE-RATE, 4B-CI-CONTENT (the CI Builder)."
lines = textwrap.wrap(new, width=128, subsequent_indent="  ", break_long_words=False, break_on_hyphens=False)
# The bullet may take fewer lines than before, never more: the file's count may only go down.
assert len(lines) <= last - first, (len(lines), last - first)
s[first:last] = lines
open(p, "w").write("\n".join(s))
open(archive_path, "w").write(archive_text)
print(f"STATUS: {before - 1} lines before, {len(s) - 1} now; his words: SIGHT-FLAG: {sight!r}; DUR-1: {rate!r}")
