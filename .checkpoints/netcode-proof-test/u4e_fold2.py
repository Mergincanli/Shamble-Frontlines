"""The main session's fold of U4j's merge into the U4e builder brief (rev 3.1 -> rev 3.2), as the advisor's U4j round-2
ruling asks (advisor-u4j-r2.md B2.1 and B5: 3.1 re-copied from u4s-carries.md §U4e by script before the advisor's check
of the brief; R28 redone). 3.1's block is copied from the carries by line range and checked equal after; the old block
is first checked equal to §U4e as it stood at 30528c7. Each other change is an old text found exactly once and its new
text, a prose item refilled to 120 characters with its words checked. Run from the repo root:
    python3 -I .checkpoints/netcode-proof-test/u4e_fold2.py [--apply]"""
import re
import subprocess
import sys
import textwrap

N = ".checkpoints/netcode-proof-test"
APPLY = "--apply" in sys.argv
BRIEF = f"{N}/u4e-brief.md"
with open(BRIEF, encoding="utf-8") as fh:
    B = fh.read().split("\n")
ORIG = list(B)
with open(f"{N}/u4s-carries.md", encoding="utf-8") as fh:
    C = fh.read().split("\n")
OLD_C = subprocess.run(["git", "show", f"30528c7:{N}/u4s-carries.md"], capture_output=True, text=True,
                       check=True).stdout.split("\n")
LOG, NEW = [], []

# 1. 3.1's block: carries :147-247 (§U4e whole, carry 61 added), in place of :147-228 as at 30528c7
assert C[146].startswith("## U4e (") and C[247] == "" and C[248].startswith("## U4r"), (C[146][:20], C[248][:20])
assert OLD_C[146] == C[146] and OLD_C[228] == "" and OLD_C[229].startswith("## U4r")
h = B.index("### 3.1 Word for word")
o = next(i for i in range(h, len(B)) if B[i] == "```")
c = next(i for i in range(o + 1, len(B)) if B[i] == "```")
assert B[o + 1:c] == OLD_C[146:228], "3.1's block is not §U4e as it stood at 30528c7"
B[o + 1:c] = C[146:247]
LOG.append(f"3.1: the block (brief :{o + 2}-{c}) replaced by carries :147-247 ({247 - 146} lines, was {c - o - 1})")

RISKY = re.compile(r"^(\d+[.)](\s|$)|[-*+](\s|$)|#|>|```|~~~|\||=+\s*$)")
NBSP = " "


def refill(text, first, cont):
    t = text
    for _ in range(20):
        w = textwrap.TextWrapper(width=120, initial_indent=first, subsequent_indent=cont, break_long_words=False,
                                 break_on_hyphens=False)
        lines = w.wrap(t)
        bad = [k for k in range(1, len(lines)) if RISKY.match(lines[k][len(cont):])]
        if not bad:
            return [l.replace(NBSP, " ") for l in lines]
        k = bad[0]
        tok = lines[k][len(cont):].split(" ")[0]
        prev = lines[k - 1].rstrip().split(" ")[-1]
        t = t.replace(f"{prev} {tok}", f"{prev}{NBSP}{tok}", 1)
    sys.exit(f"refill: no safe wrap for {text[:60]}")


def item(start):
    """The list item or paragraph whose first line starts with `start`: (first, last, marker indent, marker)."""
    hits = [i for i, l in enumerate(B) if l.startswith(start)]
    assert len(hits) == 1, (start, hits)
    s = hits[0]
    m = re.match(r"^( *)(- |\d+\. )?", B[s])
    ind, mk = m.group(1), m.group(2) or ""
    cont = " " * (len(ind) + len(mk))
    e = s
    while e + 1 < len(B) and B[e + 1].strip() and B[e + 1].startswith(cont) and not B[e + 1].startswith("```") \
            and not re.match(r"^ *(- |\d+\. |#|\|)", B[e + 1][len(cont):] if cont else B[e + 1]):
        e += 1
    return s, e, ind, mk


def item_sub(start, pairs, tag):
    s, e, ind, mk = item(start)
    cont = " " * (len(ind) + len(mk))
    text = " ".join([B[s][len(ind) + len(mk):].strip()] + [l.strip() for l in B[s + 1:e + 1]])
    new = text
    for old, rep in pairs:
        n = new.count(old)
        assert n == 1, (tag, old[:50], n)
        new = new.replace(old, rep)
    lines = refill(new, ind + mk, cont)
    got = " ".join(l.strip() for l in lines)
    assert got[len(mk):].split() == new.split() if mk else got.split() == new.split(), tag
    B[s:e + 1] = lines
    LOG.append(f"{tag}: brief :{s + 1}-{e + 1} ({e - s + 1} lines) -> {len(lines)} lines")
    NEW.extend(rep for _, rep in pairs)


def line_sub(old, rep, tag):
    hits = [i for i, l in enumerate(B) if old in l]
    assert len(hits) == 1 and B[hits[0]].count(old) == 1, (tag, hits)
    B[hits[0]] = B[hits[0]].replace(old, rep)
    LOG.append(f"{tag}: brief :{hits[0] + 1}")
    NEW.append(rep)


def insert_after_item(start, lines, tag):
    _, e, _, _ = item(start)
    B[e + 1:e + 1] = lines
    LOG.append(f"{tag}: {len(lines)} lines after brief :{e + 1}")
    NEW.append(" ".join(lines))


W = textwrap.TextWrapper(width=120, break_long_words=False, break_on_hyphens=False)

# 2. the title and the fold note
line_sub("# U4e builder brief: the room and the codec path (rev 3.1)",
         "# U4e builder brief: the room and the codec path (rev 3.2)", "title")
note = W.wrap(
    "**The main session's fold at U4j's merge (2026-11-11 about 20:40 UTC; rev 3.2):** U4j merged (4d2b0ef, d50396a), "
    "and its merge numbered round 1's and round 2's carries (advisor-u4j-r2.md B5, B7.2). So 3.1 is re-copied by "
    "script from u4s-carries.md §U4e (carry 61 added; carries :147-247), with a note for carry 61, carry 40's cite "
    "moved, and the count, the return list's item 6 and section 0's notes updated; the lines that called U4j unmerged "
    "now say what merged (carry 54's note, the build rules, U4E-Q13's option (c)); R27, R28 and R38 are rechecked at "
    "d50396a. Nothing else changed.")
insert_after_item("**The main session's fold before the check", [""] + note, "the rev 3.2 fold note")

# 3. section 0
item_sub("- Every file:line is at the spike branch's head when rev 3 was written",
         [("U4j is in its fix pass and has not merged: this brief relies on nothing of U4j's.",
           "U4j merged after rev 3.1 (4d2b0ef and d50396a; under spikes/ it changed only crates/judge, crates/log and "
           "marks/): rev 3.2's new cites are at d50396a, and the brief relies on U4j through carry 61 and rows R27, "
           "R28 and R38 only.")], "section 0, the heads")
item_sub("- `plan` is .checkpoints/netcode-proof-test/u4-build-plan.md",
         [("u4-build-plan.md (sha256 7375cdcf…, 1,070 lines);",
           "u4-build-plan.md, its cites to the version of sha256 7375cdcf… (1,070 lines; `git show "
           "30528c7:.checkpoints/netcode-proof-test/u4-build-plan.md`): U4j's merge added a row after :199 (§4.1), "
           "three lines to §10's preamble and two to §11, so in the current file (4b8eaab6…, 1,076 lines) a cite from "
           ":200 to :953 is one line lower, and one from :964 on four lines lower;")], "section 0, plan's version")

# 4. 3.1's intro, the count, the notes
item_sub("Quoted from carries :147-228",
         [("Quoted from carries :147-228,", "Quoted from carries :147-247,"),
          ("Checked: the fold re-copied this block by script, and rev 3.1 compared it with carries :147-228 by a "
           "`diff` of the two line ranges: equal.",
           "Checked: the main session re-copied this block by script at U4j's merge (rev 3.2, carry 61 added) and "
           "compared it with carries :147-247 by a `diff` of the two line ranges: equal.")], "3.1's intro")
line_sub("Fifteen carries (21-27, 39, 50, 51 and 53-57). Notes for the builder, which change no word:",
         "Sixteen carries (21-27, 39, 50, 51, 53-57 and 61). Notes for the builder, which change no word:", "the count")
c61 = textwrap.TextWrapper(width=120, subsequent_indent="  ", break_long_words=False, break_on_hyphens=False).wrap(
    "- carry 61: the room's `server_hash` and the client's `decoded_hash` follow marks/README.md §1 as U4j merged it "
    "(d50396a: :50-71, with round 2's words); its unit test (tick t+1's snapshot fed to `ClientEndpoint` before tick "
    "t's, and one datagram received twice) is one of your tests (2.4), named in the report; your report says whether "
    "your client decodes a stale group or drops it (the advisor's lean is the decode: advisor-u4j-r2.md B2.1), and a "
    "client that drops counts each drop per client under a proposed series name with its SERIES.md row for the main "
    "session, as carry 54's refusal count does;")
insert_after_item("- carry 57: the room exposes each overflow resync", c61, "the note for carry 61")
item_sub("- carry 40 (§U4r, carries :231-239)", [("carries :231-239", "carries :250-258")], "carry 40's cite")
item_sub("- carry 54: U4e counts the refusal per client",
         [("(marks/ is U4j's lane, unmerged, and not U4e's: section", "(marks/ is not U4e's lane: section")],
         "carry 54's note")

# 5. the build rules, the return list, U4E-Q13
item_sub("- Never crates/sim, crates/proto, crates/lab",
         [("crates/judge and marks/ (U4j's, unmerged), check.mjs",
           "crates/judge and marks/ (U4j's in wave 1, merged at 4d2b0ef; not U4e's), check.mjs")], "the build rules")
item_sub("6. Carries 21-27, 39, 50, 51 and 53-57, by number:",
         [("6. Carries 21-27, 39, 50, 51 and 53-57, by number:"[3:], "Carries 21-27, 39, 50, 51, 53-57 and 61, by number:"),
          ("; and, for 3.2, which scene each ES number came",
           "; carry 61 with its client's choice for a stale group (decoded, or dropped and counted); and, for 3.2, "
           "which scene each ES number came")], "the return list, item 6")
item_sub("- Options: (a) an id outside the PM form",
         [("(c) a marks.json entry for it, through U4j's lane (marks/ is U4j's, unmerged), for a bite no mark judges.",
           "(c) a marks.json entry for it, through the main session (marks/ is not U4e's lane), for a bite no mark "
           "judges.")], "U4E-Q13's option (c)")

# 6. the recheck table: R27, R28, R38, and the rows-changed line
line_sub("| marks.json at `<base>` (U4j edits it) |", "| marks.json at `<base>` |", "R27 (i)")
line_sub("U4j's fix pass may edit marks.json; W1-N2 keeps lt-tick-offset's id (wave1 :341-342) |",
         "W1-N2 keeps lt-tick-offset's id (wave1 :341-342). Rechecked at d50396a (U4j's merge): U4j added one id, "
         "`PM-0001-18/ws-budget-x2` (marks.json:4431), the send budget's bite on the L2 B1 ws part; the send budget is "
         "npt-net's WsServer, wired in U6 (plan.md:217, :521), not U4e's, so the ids U4e plants are unchanged |",
         "R27 (ii)")
line_sub("| changed: not at the head (U4j has not merged: marks/SERIES.md has neither row; crates/log's `TickRecord` has "
         "no `hs`). The brief relies on neither: U4e exposes the values under the plan's names (`event.burst_eviction`: "
         "§4.5 :298-299; `tick.s.snap_incomplete`: §4.6 :321), and U4r's logs write them. npt-room and npt-client have "
         "no npt-log dependency (room/Cargo.toml:17-21; client/Cargo.toml:17-22) |",
         "| changed at d50396a (U4j's merge): crates/log's `TickRecord` has `hs` (log/src/lib.rs:121) with "
         "`set_hashes` and `hashes` (:158, :168); SERIES.md has the `tick.s.snap_incomplete` row (:319) and "
         "`event.burst_eviction.evicted`'s (:320); marks/README.md §1's `hs` rules bind U4e through carry 61. U4e still "
         "exposes the values under the plan's names (`event.burst_eviction`: §4.5 :298-299; `tick.s.snap_incomplete`: "
         "§4.6 :321) and U4r's logs write them; npt-room and npt-client still have no npt-log dependency "
         "(room/Cargo.toml:17-21; client/Cargo.toml:17-22). Recheck at launch |", "R28")
line_sub("| new: measured 365, bites 412, self 52 of 52, 10 steps (u4d-report.md:68-70) |",
         "| new: measured 365, bites 412, self 52 of 52, 10 steps (u4d-report.md:68-70); at d50396a (U4j's merge): "
         "measured 396, bites 443, self 52 of 52, 10 steps (u4j-build.md, \"Merge into the spike branch\", M5) |", "R38")
item_sub("Rows changed: R4, R6, R11, R15, R19, R20, R27, R28 (8 of 28)",
         [("Rev 3.1 rewrote R31's route by close B2.",
           "Rev 3.1 rewrote R31's route by close B2. Rev 3.2 rechecked R27, R28 and R38 at d50396a (U4j's merge).")],
         "the rows-changed line")

# checks: 3.1's block equals carries :147-247; every new text present; changed prose lines fit 120 characters
h = B.index("### 3.1 Word for word")
o = next(i for i in range(h, len(B)) if B[i] == "```")
c = next(i for i in range(o + 1, len(B)) if B[i] == "```")
assert B[o + 1:c] == C[146:247], "3.1's block differs from carries :147-247"
flat = " ".join(" ".join(B).split())
for t in NEW:
    assert " ".join(t.split()) in flat, t[:60]
import difflib
diff = list(difflib.unified_diff(ORIG, B, n=0, lineterm=""))[2:]
added = [l[1:] for l in diff if l.startswith("+") and not l.startswith("+++")]
long = [l[:50] for l in added if not l.startswith("|") and len(l) > 120 and l not in C[146:247]]
print(f"brief: +{len(added)} -{len([l for l in diff if l.startswith('-') and not l.startswith('---')])} lines; "
      f"{len(ORIG)} -> {len(B)} lines; over 120: {long}")
print("\n".join(LOG))
if APPLY:
    with open(BRIEF, "w", encoding="utf-8") as fh:
        fh.write("\n".join(B))
    print("APPLIED")
else:
    print("dry run: nothing written")
