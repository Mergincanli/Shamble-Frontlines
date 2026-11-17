"""DOCS-54's copy script (the plan: docs-54/docs-54-plan-draft.md, sha256 62b5ae67..., approved with conditions C1-C9
by the advisor's review, docs-54/advisor-plan-review-full.json, 78f6b5a5...). It copies Mustafa's typed words from this
session's transcript, the primary source (C2), into a worktree's docs/DECISIONS.md, as new entries right under
"## Decisions, newest first", newest first by the transcript's stamps.

- Only his typed text goes inside "> " (C1). The session's text (the questions he answered, the three lines of his yes,
  the ask his "go" answered) goes in fenced blocks under a list line saying it is the session's text, not his words.
- Each contiguous copied run is its own block; each cut is named in place, in an italic list line outside any quote,
  and in the header (C6).
- The transform, exactly: a line X becomes "> X", a blank line becomes ">" (C2). The check is its exact inverse: every
  block, read back, gives the source's lines byte for byte (sha256 printed for both).
- Also checked: each run against history.md at 110c303, the copy checked (C7; the one known difference is named: the
  transcript's first line of E6 ends in a space that history.md:2151 lacks; the copy keeps the space); E3's three lines
  against origin/main .claude/NIGHT_SHIFT.md:45-47 and 4e3d1565... (sha256 taken without the final newline, C2); E10
  against merge commit 4460ef5's message, its only source (C4).
- Before writing, it prints the in/out table: every part, the exact text inside "> " and the text outside it (C1).
- It writes only when the target equals main's docs/DECISIONS.md (git blob 7a1ccea3... at 82c605e) and holds no entry
  of its own yet, so a second run exits 1.

Run from the main folder's root (reads the transcript, git objects and the target; writes only the target and, with
--table, a word-for-word copy of its own printed output):
  python3 -I .checkpoints/netcode-proof-test/docs-54/copy_words_54.py --check [--table <file>]
  python3 -I .checkpoints/netcode-proof-test/docs-54/copy_words_54.py --write <worktree> [--table <file>]
Exit codes: 0 done (or --check passed) · 1 a check failed, or the target is not main's file · 2 usage."""
import hashlib
import json
import os
import subprocess
import sys
import textwrap

REPO = "/home/user/Shamble-Frontlines"
T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
SESSION = "session_0173vVo6igCNyQYBWPvhH36A"
HIST_REV = "110c3031a5145c675f25d320068df0368476771c"
HIST_PATH = ".checkpoints/netcode-proof-test/history.md"
HIST_BLOB = "39f8c7a5a34a0219ee512c170d2e97355e6ca89b"
MAIN = "82c605e2e8a7f284e6925718368d3fe569f89ae9"
DEC_PATH = "docs/DECISIONS.md"
DEC_BLOB = "7a1ccea3ac0103efc82d05d4902cdeb4eede3561"
NS_PATH = ".claude/NIGHT_SHIFT.md"
NS_BLOB = "c1a95f792375ed88236bd2e1116dcf96f491cf3c"
NS_SHA = "4e3d15658c0afaf40d922cdce9559a8dc2d1079ea04a8fc96e327798b3639223"
E10_COMMIT = "4460ef5b8d6dc8566b29c15ded75c6f0c8be386b"
ANCHOR = "## Decisions, newest first\n"
OLD_FIRST = "- **Taken by Mustafa, 2026-11-13, 22:27:05 UTC: his status request, in foundation 2.3's session**"
HIST_AT = f"`.checkpoints/netcode-proof-test/history.md` at {HIST_REV} (PR #40's branch)"
OUT = []
FAILS = []


def say(line=""):
    OUT.append(line)
    print(line)


def check(ok, what):
    say(("ok   " if ok else "FAIL ") + what)
    if not ok:
        FAILS.append(what)
    return ok


def sha(s):
    return hashlib.sha256(s.encode("utf-8")).hexdigest()


def git(*args):
    return subprocess.run(["git", *args], cwd=REPO, capture_output=True, check=True).stdout.decode("utf-8")


def quote(lines):
    """The transform (C2): X -> "> X", a blank line -> ">"."""
    return [">" if x == "" else "> " + x for x in lines]


def unquote(lines):
    """Its exact inverse; anything else is an error."""
    out = []
    for q in lines:
        if q == ">":
            out.append("")
        elif q.startswith("> "):
            out.append(q[2:])
        else:
            raise ValueError(f"not a quote line: {q!r}")
    return out


def wrap(text):
    return textwrap.wrap(text, width=120, subsequent_indent="  ", break_long_words=False, break_on_hyphens=False)


# His typed messages: transcript line, record uuid, stamp, where the text sits in the record.
HIS = {
    "E1": (48998, "90173405-9d31-4247-8d81-4a46d2902961", "2026-11-17T10:43:48.016Z", "content"),
    "E2": (47621, "50b71aa0-f12b-4f33-b170-1fe80a5ee722", "2026-11-17T07:00:54.917Z", "content"),
    "E3": (45793, "dcec7506-366c-4214-a0a5-1c6ccdae9a8e", "2026-11-16T21:09:27.492Z", "content"),
    "E4": (45719, "8840e0b5-3b64-4365-9f3d-b4967eb136b0", "2026-11-16T20:48:10.683Z", "queued_command"),
    "E5": (43019, "63954cb4-7747-4dc6-86a8-6e72c97e81e9", "2026-11-15T19:46:36.849Z", "content"),
    "GO53": (42437, "420bc186-1912-44ee-9d6e-652637b753e2", "2026-11-15T18:42:53.995Z", "content"),
    "E6": (42064, "513dc802-6ed8-4f23-b620-02df87f8d996", "2026-11-15T16:21:11.362Z", "content"),
    "E7": (41516, "37f2d715-08cd-4120-9f49-5e2f4c99c274", "2026-11-15T15:34:42.893Z", "content"),
    "E8": (40306, "14631e5d-679d-477c-a17a-64d8fbd4acb5", "2026-11-15T12:34:01.309Z", "content"),
    "E9": (40222, "6aff71a8-5f98-485b-a8c2-d0de1670163c", "2026-11-15T12:01:41.136Z", "content"),
}
# The session's messages whose lines are shown, never inside "> ".
SESS = {
    "S-E1a": (48664, "a5b19b89-9a18-4259-9f64-4cbdc7d7abfd", "2026-11-17T08:54:58.700Z"),
    "S-E1b": (48994, "dc526fb2-bca6-4e03-8f03-f17d983d1f95", "2026-11-17T10:35:38.816Z"),
    "S-E2": (47614, "345259cd-922b-4a94-ba72-d498af6889fe", "2026-11-17T06:34:50.318Z"),
    "S-E3": (45789, "85c33559-9cfa-4834-b019-02a57b82b7e6", "2026-11-16T20:50:16.246Z"),
    "S-GO53": (42414, "d95dd5a3-457b-4893-969b-2d627afdc934", "2026-11-15T18:04:58.542Z"),
}
# His long messages: the history.md range that holds the whole message (two-space indent, blank lines blank) and the
# runs copied to main (history.md lines, inclusive).
LONG = {
    "E9": (2091, 2111, [(2091, 2107)]),
    "E8": (2116, 2129, [(2116, 2116), (2129, 2129)]),
    "E7": (2134, 2146, [(2134, 2139)]),
    "E6": (2151, 2161, [(2151, 2161)]),
    "E5": (2166, 2175, [(2166, 2170)]),
    "E4": (2180, 2187, [(2180, 2187)]),
}
# His short messages: the history.md line that quotes the whole message inside quote marks, behind an exact prefix.
SHORT = {
    "E1": (2205, '- **Mustafa, typed into this chat (transcript 2026-11-17T10:43:48.016Z), verbatim:** "', '"'),
    "E2": (2200, '- **Mustafa, typed into this chat (transcript 2026-11-17T07:00:54.917Z), verbatim:** "', '"'),
    "E3": (2190, '- **Mustafa, typed into this chat (transcript 2026-11-16T21:09:27.492Z), verbatim:** "', '"'),
}


def load_records():
    want = {v[0]: k for k, v in {**HIS, **SESS}.items()}
    recs = {}
    with open(T, encoding="utf-8") as fh:
        for n, line in enumerate(fh, 1):
            if n in want:
                recs[want[n]] = json.loads(line, strict=False)
    return recs


def his_text(code, r):
    n, uuid, stamp, where = HIS[code]
    check(r.get("uuid") == uuid and r.get("timestamp") == stamp and not r.get("isSidechain"),
          f"{code}: transcript line {n} is record {uuid} at {stamp}, not a side chain")
    if where == "content":
        c = (r.get("message") or {}).get("content")
        check(r.get("type") == "user" and isinstance(c, str), f"{code}: a user record whose content is his text")
        return c if isinstance(c, str) else ""
    a = r.get("attachment") or {}
    check(r.get("type") == "attachment" and a.get("type") == "queued_command" and isinstance(a.get("prompt"), str),
          f"{code}: a queued-command record whose prompt is his text")
    return a.get("prompt") or ""


def sess_text(code, r):
    n, uuid, stamp = SESS[code]
    check(r.get("uuid") == uuid and r.get("timestamp") == stamp and not r.get("isSidechain")
          and r.get("type") == "assistant", f"{code}: transcript line {n} is the session's record {uuid} at {stamp}")
    blocks = [b.get("text") for b in (r.get("message") or {}).get("content") or []
              if isinstance(b, dict) and b.get("type") == "text"]
    check(len(blocks) == 1 and isinstance(blocks[0], str), f"{code}: one text block")
    return blocks[0] if blocks and isinstance(blocks[0], str) else ""


def find_run(lines, first, count, code):
    """The index where `count` lines starting with `first` sit; `first` must occur exactly once."""
    idx = [i for i, x in enumerate(lines) if x == first]
    check(len(idx) == 1, f"{code}: the line {first[:50]!r}... occurs exactly once in the session's message")
    return idx[0] if len(idx) == 1 else 0


def main():
    args = sys.argv[1:]
    table = None
    if "--table" in args:
        i = args.index("--table")
        if i + 1 >= len(args):
            print("usage: --table <file>")
            return 2
        table = args[i + 1]
        del args[i:i + 2]
    if args == ["--check"]:
        mode, wt = "check", None
    elif len(args) == 2 and args[0] == "--write":
        mode, wt = "write", args[1]
    else:
        print(__doc__)
        return 2

    say(f"DOCS-54 copy_words_54.py, mode {mode}")
    check(git("rev-parse", "origin/main").strip() == MAIN, f"origin/main is {MAIN}")
    check(git("rev-parse", f"{MAIN}:{DEC_PATH}").strip() == DEC_BLOB, f"main's {DEC_PATH} is blob {DEC_BLOB}")
    check(git("rev-parse", f"{MAIN}:{NS_PATH}").strip() == NS_BLOB, f"main's {NS_PATH} is blob {NS_BLOB}")
    check(git("rev-parse", f"{HIST_REV}:{HIST_PATH}").strip() == HIST_BLOB, f"history.md at 110c303 is blob {HIST_BLOB}")
    hist = git("show", f"{HIST_REV}:{HIST_PATH}").split("\n")

    def H(n):
        return hist[n - 1]

    recs = load_records()
    check(set(recs) == set(HIS) | set(SESS), "every transcript record found at its line")
    his = {c: his_text(c, recs.get(c, {})) for c in HIS}
    sess = {c: sess_text(c, recs.get(c, {})) for c in SESS}
    hashes = []   # (block, source, history.md lines, sha256 source, sha256 copy-back, match)

    # His long messages: the whole message against history.md's range, then the runs.
    runs = {}
    for code, (a, b, rr) in LONG.items():
        tl = his[code].split("\n")
        check(len(tl) == b - a + 1, f"{code}: {len(tl)} transcript lines = history.md:{a}-{b}")
        diffs = []
        for i, x in enumerate(tl):
            want = ("  " + x) if x else ""
            if a + i <= len(hist) and H(a + i) != want:
                diffs.append(a + i)
        if code == "E6":
            ok = (diffs == [2151] and tl[0].endswith(" ") and not tl[0].endswith("  ")
                  and H(2151) == "  " + tl[0][:-1])
            check(ok, "E6: the one known difference, history.md:2151 lacks the transcript's trailing space")
        else:
            check(diffs == [], f"{code}: history.md:{a}-{b} equal the transcript's lines (two-space indent)")
        runs[code] = [tl[x - a:y - a + 1] for x, y in rr]
        for (x, y), run in zip(rr, runs[code]):
            back = unquote(quote(run))
            hist_run = [(H(k)[2:] if H(k) else "") for k in range(x, y + 1)]
            match = back == run and (hist_run == run or (code == "E6" and x == 2151 and
                                     hist_run[1:] == run[1:] and hist_run[0] == run[0][:-1]))
            hashes.append((f"{code} :{x}-{y}", f"transcript {HIS[code][1]}", f"{x}-{y}", sha("\n".join(run)),
                           sha("\n".join(back)), "yes" if match else "NO"))
            check(match, f"{code}: run :{x}-{y} round-trips byte for byte and equals history.md's copy"
                  + (" (but the named space)" if code == "E6" else ""))

    # His short messages: the whole message; history.md quotes it inside quote marks behind an exact prefix.
    for code, (n, pre, post) in SHORT.items():
        t = his[code]
        check("\n" not in t and H(n) == pre + t + post, f"{code}: history.md:{n} is the prefix + his text + the quote mark")
        hashes.append((f"{code} whole", f"transcript {HIS[code][1]}", f"{n} (inside the quotes)", sha(t),
                       sha("\n".join(unquote(quote([t])))), "yes" if H(n) == pre + t + post else "NO"))
    check(his["GO53"] == "go", "GO53: his message is the word go (not in history.md: the transcript is its only source)")
    hashes.append(("GO53 whole", f"transcript {HIS['GO53'][1]}", "none", sha(his["GO53"]),
                   sha("\n".join(unquote(quote([his["GO53"]])))), "yes"))

    # The session's text, shown outside the quotes.
    s1 = sess["S-E1a"].split("\n")
    j = find_run(s1, "**Decisions needed:**", 10, "S-E1a")
    blk = s1[j:j + 10]
    check([("  " + x) if x else "" for x in blk] == [H(k) for k in range(2208, 2218)],
          "S-E1a: history.md:2208-2217 equal the message's Decisions-needed block (two-space indent)")
    s1_docs, s1_ns = blk[7], blk[9]
    check(s1_docs.startswith("- **DOCS-54:**") and s1_ns.startswith("- **NS-TRAILER and NS-FIRST:**"),
          "S-E1a: :2215 is DOCS-54's question, :2217 is NS-TRAILER and NS-FIRST's")
    s2 = sess["S-E1b"].split("\n")
    j = find_run(s2, "**Decisions needed:** unchanged:", 5, "S-E1b")
    blk2 = s2[j:j + 5]
    check([("  " + x) if x else "" for x in blk2] == [H(k) for k in range(2221, 2226)],
          "S-E1b: history.md:2221-2225 equal the message's repeated block (two-space indent)")
    s2_docs, s2_ns = blk2[2], blk2[4]
    check(s2_docs == "- DOCS-54: I recommend (a)." and s2_ns == "- NS-TRAILER and NS-FIRST: I recommend (b) for both.",
          "S-E1b: :2223 and :2225 are DOCS-54's and NS-TRAILER and NS-FIRST's lines")
    pre2201 = "- **What it answered** (the assistant's message at 2026-11-17T06:34:50.318Z, its \"For Mustafa\" line, copied by this script): "
    e2_line = H(2201)[len(pre2201):] if H(2201).startswith(pre2201) else ""
    check(e2_line != "" and sess["S-E2"].split("\n").count(e2_line) == 1,
          "S-E2: history.md:2201 behind its prefix is one whole line of the 06:34:50 message")
    pre2191 = "- **What he said yes to** (the assistant's question at 2026-11-16T20:50:16.246Z: \""
    post2191 = f"\"): these three lines, copied by this script from that message's first fenced block, sha256 {NS_SHA}:"
    q = H(2191)[len(pre2191):-len(post2191)] if H(2191).startswith(pre2191) and H(2191).endswith(post2191) else ""
    check(q != "" and sess["S-E3"].count(q) == 1, "S-E3: history.md:2191's quoted question occurs once in the 20:50:16 message")
    s3 = sess["S-E3"].split("\n")
    fences = [i for i, x in enumerate(s3) if x.startswith("```")]
    three = s3[fences[0] + 1:fences[1]] if len(fences) >= 2 else []
    ns = git("show", f"{MAIN}:{NS_PATH}").split("\n")[44:47]
    check(len(three) == 3 and sha("\n".join(three)) == NS_SHA and three == ns
          and three == [H(k) for k in range(2194, 2197)],
          "E3: the first fenced block's three lines = NIGHT_SHIFT.md:45-47 on main = history.md:2194-2196, sha256 4e3d1565...")
    hashes.append(("E3 three lines (the session's)", f"transcript {SESS['S-E3'][1]}", "2194-2196", sha("\n".join(three)),
                   sha("\n".join(ns)), "yes" if sha("\n".join(three)) == NS_SHA else "NO"))
    go_paras = sess["S-GO53"].split("\n\n")
    go_ask = go_paras[-1]
    check(go_ask.count('type "go"') == 1 and "\n" not in go_ask, "S-GO53: the last paragraph is the one-line ask to type go")

    # E10: the commit's "> " block, re-quoted byte for byte.
    msg = git("log", "-1", "--format=%B", E10_COMMIT).split("\n")
    qi = [i for i, x in enumerate(msg) if x == ">" or x.startswith("> ")]
    contiguous = qi == list(range(qi[0], qi[-1] + 1)) if qi else False
    e10 = msg[qi[0]:qi[-1] + 1] if contiguous else []
    e10_words = unquote(e10) if e10 else []
    check(contiguous and quote(e10_words) == e10 and e10_words[0].startswith("My words of 2026-11-14 for PR #52"),
          "E10: 4460ef5's message holds one contiguous quote block; re-quoting it gives it back byte for byte")
    hashes.append(("E10 whole", f"commit {E10_COMMIT}", "none", sha("\n".join(e10)), sha("\n".join(quote(e10_words))),
                   "yes" if quote(e10_words) == e10 else "NO"))
    for fb in (three, [s1_docs], [s1_ns], [s2_docs], [s2_ns], [e2_line], [q], [go_ask]):
        check(not any(x.startswith("```") for x in fb), "no fence line inside a fenced part")

    def label(code):
        for x in his[code].split("\n"):
            if x.startswith("My words of ") and "(Amsterdam time)" in x:
                return x[:x.index("(Amsterdam time)") + len("(Amsterdam time)")]
        raise ValueError(code)

    def src(code, extra=""):
        n, uuid, stamp, where = HIS[code]
        rec = "its queued-command record" if where == "queued_command" else "message"
        return (f"his message typed into the netcode proof test's day session, {SESSION}, on his university account"
                f"{extra}; the primary source is that session's transcript, {rec} {uuid}, stamped {stamp}")

    def checked(lines_text, tail=""):
        return f"Copied by script from the transcript and checked byte for byte against {HIST_AT}, {lines_text}{tail}"

    # The entries, newest first. Each part: ("hdr", text) | ("his", lines) | ("cut", text) | ("sesslabel", text)
    # | ("sess", lines). "his" parts go inside "> "; every other part stays outside any quote.
    E = []
    E.append(("E1", [
        ("hdr", "- **Taken by Mustafa, 2026-11-17, 10:43:48 UTC: RECS-1043, his answer to the session's recommendations; "
                "for main: DOCS-54 (a), NS-TRAILER (b) and NS-FIRST (b)** (" + src("E1", "; no time label of his own")
                + "). It answers the session's Decisions-needed block as last posed at 08:54:58 UTC and repeated at "
                "10:35:38 UTC; the lines for main are shown under it as the session's text, not his words. DOCS-54 (a) "
                "is carried out by the docs pass that added this entry; NS-TRAILER and NS-FIRST are queued above. "
                "PACK-READ and C62HB-Q4, which it also answered (history.md:2209-2214, :2216, :2222 and :2224), and the "
                "session's reading (:2227) are the netcode test's and stay with PR #40. "
                + checked("line 2205 (his words, inside the quote marks behind its prefix) and lines 2215, 2217, 2223 "
                          "and 2225") + ":"),
        ("his", [his["E1"]]),
        ("sesslabel", "- _The session's text, not his words: the questions it answers for main, as last posed (the "
                      f"assistant's message at {SESS['S-E1a'][2]}, transcript message {SESS['S-E1a'][1]}, two lines of "
                      "its Decisions-needed block):_"),
        ("sess", [s1_docs]),
        ("cut", "- _[history.md:2216 left out: C62HB-Q4, the netcode test's; it stays with PR #40]_"),
        ("sess", [s1_ns]),
        ("sesslabel", "- _The session's text, not his words: the same two as last repeated before his message (the "
                      f"assistant's message at {SESS['S-E1b'][2]}, transcript message {SESS['S-E1b'][1]}):_"),
        ("sess", [s2_docs]),
        ("cut", "- _[history.md:2224 left out: C62HB-Q4, the netcode test's; it stays with PR #40]_"),
        ("sess", [s2_ns]),
    ]))
    E.append(("E2", [
        ("hdr", "- **Taken by Mustafa, 2026-11-17, 07:00:54 UTC: MERGE-54, his go for #54** ("
                + src("E2", "; no time label of his own") + "). It answers the session's \"For Mustafa\" line of "
                "06:34:50 UTC, shown under it as the session's text, not his words. Carried out by the merge 82c605e "
                "(a merge commit, expected head 20f9d59918eea1e3431ae830f72510f83f6424c7; GitHub's merged_at "
                "2026-11-17T07:03:03Z). "
                + checked("line 2200 (inside the quote marks behind its prefix) and line 2201") + ":"),
        ("his", [his["E2"]]),
        ("sesslabel", "- _The session's text, not his words: the line it answers (the assistant's message at "
                      f"{SESS['S-E2'][2]}, transcript message {SESS['S-E2'][1]}, its \"For Mustafa\" line):_"),
        ("sess", [e2_line]),
    ]))
    E.append(("E3", [
        ("hdr", "- **Taken by Mustafa, 2026-11-16, 21:09:27 UTC: NS-LINES (a), his yes to the three night-shift lines** ("
                + src("E3", "; no time label of his own") + "). By his words of 20:48:10 UTC (the entry below), this "
                "yes is also the exception to the kit freeze (KIT-FREEZE-2) for that one change. Carried out by PR #54 "
                "(merged at 82c605e): the three lines stand in `.claude/NIGHT_SHIFT.md` :45-47. It answers the "
                "session's question of 20:50:16 UTC; the question and the three lines are shown under it as the "
                "session's text, not his words. "
                + checked("line 2190 (inside the quote marks behind its prefix) and lines 2191 and 2194-2196",
                          f"; the three lines also equal `.claude/NIGHT_SHIFT.md` :45-47 at main {MAIN} (sha256 "
                          f"{NS_SHA}, taken without the final newline)") + ":"),
        ("his", [his["E3"]]),
        ("sesslabel", "- _The session's text, not his words: the question it answers (the assistant's message at "
                      f"{SESS['S-E3'][2]}, transcript message {SESS['S-E3'][1]}) and the three lines (that message's "
                      "first fenced block):_"),
        ("sess", [q]),
        ("sess", three),
    ]))
    E.append(("E4", [
        ("hdr", "- **Taken by Mustafa, 2026-11-16, 20:48:10 UTC: NS-STATUS (a) and the approval-free command forms** "
                f"(his label: \"{label('E4')}\"; " + src("E4", ", sent while a turn ran") + "). Its NS-STATUS was "
                "carried out by PR #54 (merged at 82c605e); his yes that it names as the exception to the kit freeze "
                "is the entry above. " + checked("lines 2180-2187; whole") + ":"),
        ("his", runs["E4"][0]),
    ]))
    E.append(("E5", [
        ("hdr", "- **Taken by Mustafa, 2026-11-15, 19:46:36 UTC: KL-MERGE (a), merge #53, and the security and test "
                f"findings accepted as left open** (his label: \"{label('E5')}\"; " + src("E5") + "). Carried out by "
                "the merge 53272a1 (PR #53); the follow-ups it accepts as left open (SA-KL-1 to SA-KL-4, KLT-1 to "
                "KLT-8) are queued above for the kit's next change after the freeze. Partial: history.md:2172-2175 "
                "(C62H-BOUND-YES) are the netcode test's and stay with PR #40. " + checked("lines 2166-2170") + ":"),
        ("his", runs["E5"][0]),
        ("cut", "- _[history.md:2172-2175 left out: C62H-BOUND-YES, the netcode test's; it stays with PR #40]_"),
    ]))
    E.append(("GO53", [
        ("hdr", "- **Taken by Mustafa, 2026-11-15, 18:42:53 UTC: his go for the copy of the kept-logs kit files (PR #53)** ("
                + src("GO53", "; no time label of his own") + "). It answers the session's ask of 18:04:58 UTC, "
                "shown under it as the session's text, not his words. Not in history.md: found by the transcript "
                "sweep that DOCS-54's plan review asked for; copied by script from the transcript, its only source:"),
        ("his", [his["GO53"]]),
        ("sesslabel", "- _The session's text, not his words: the ask it answers (the assistant's message at "
                      f"{SESS['S-GO53'][2]}, transcript message {SESS['S-GO53'][1]}, its last paragraph):_"),
        ("sess", [go_ask]),
    ]))
    E.append(("E6", [
        ("hdr", "- **Taken by Mustafa, 2026-11-15, 16:21:11 UTC: KL-CHANGE-YES (a), yes to kl-change-v2.diff; build the "
                f"kept-logs rule as set out** (his label: \"{label('E6')}\"; " + src("E6") + "). Carried out by PR #53 "
                "(merged at 53272a1). " + checked("lines 2151-2161; whole, with one known difference: the "
                                                  "transcript's first line ends in a space that history.md:2151 lacks, "
                                                  "and the copy keeps the transcript's space") + ":"),
        ("his", runs["E6"][0]),
    ]))
    E.append(("E7", [
        ("hdr", "- **Taken by Mustafa, 2026-11-15, 15:34:42 UTC: KL-YES with one change, and the exception to the kit "
                f"freeze (KIT-FREEZE-2) for the kept-logs rule** (his label: \"{label('E7')}\"; " + src("E7") + "). "
                "Carried out by PR #53 (merged at 53272a1): the rule stands in `.claude/CHECKPOINTS.md` §11. Partial: "
                "history.md:2141-2146 (C62H-KEEP and C62H-BOUND) are the netcode test's and stay with PR #40. "
                + checked("lines 2134-2139") + ":"),
        ("his", runs["E7"][0]),
        ("cut", "- _[history.md:2141-2146 left out: C62H-KEEP and C62H-BOUND, the netcode test's; they stay with PR #40]_"),
    ]))
    E.append(("E8", [
        ("hdr", "- **Taken by Mustafa, 2026-11-15, 12:34:01 UTC: the kit rule's exact lines go to his Advisor first** "
                f"(his label: \"{label('E8')}\"; " + src("E8") + "). Partial: history.md:2118-2127 (C62H-REVIEW, and a "
                "reviewer's read for every marks.json or judge change) are the netcode test's and stay with PR #40. "
                + checked("lines 2116 and 2129") + ":"),
        ("his", runs["E8"][0]),
        ("cut", "- _[history.md:2118-2127 left out: C62H-REVIEW, and a reviewer's read for every marks.json or judge "
                "change, the netcode test's; they stay with PR #40]_"),
        ("his", runs["E8"][1]),
    ]))
    E.append(("E9", [
        ("hdr", "- **Taken by Mustafa, 2026-11-15, 12:01:41 UTC: KEPT-LOGS (c) and the kit-rule route** (his label: "
                f"\"{label('E9')}\"; " + src("E9") + "). Carried out by the kept-logs kit rule, PR #53 (merged at "
                "53272a1: `.claude/CHECKPOINTS.md` §11). Partial: history.md:2109 and :2111 (the bot fix's two cleans "
                "and its 1,400-line stop; the report line on commit 5e563b95) are the netcode test's and stay with PR "
                "#40. " + checked("lines 2091-2107") + ":"),
        ("his", runs["E9"][0]),
        ("cut", "- _[history.md:2109 and :2111 left out: the bot fix's two cleans and its 1,400-line stop, and the report "
                "line on commit 5e563b95, the netcode test's; they stay with PR #40]_"),
    ]))
    E.append(("E10", [
        ("hdr", "- **Taken by Mustafa, 2026-11-14, 09:02:29 UTC: his go for #52** (foundation 2.3's first docs pass, PR "
                "#52; given in that change-set's session, session_019YNMUWSsyRgaYWxk4auQ5b, on his personal account, "
                "after the advisor agent's APPROVE of bfd719b; carried out by the merge 4460ef5). The lines below are "
                "merge commit 4460ef5's record of his message (quoted there with \"> \", captured there by script from "
                "that session's transcript, message 14ea7caa-8c5d-47cd-bde5-df0d72d859d1): a record of his words, not "
                f"an instruction found in a commit. Extracted by script from the message of commit {E10_COMMIT} only, "
                "where re-quoting them gives its block back byte for byte. Not checked against that session's chat "
                "record: the account that made this entry cannot read it (the sessions API answers that the event was "
                "not found). The commit's block, as it stands:"),
        ("e10", e10),
    ]))

    # Render, and print the in/out table (C1).
    rendered = []
    say("")
    say("IN/OUT TABLE (C1): every part of every entry, in order; IN = inside \"> \" (his typed words only), OUT = outside")
    for code, parts in E:
        say(f"== {code}")
        for kind, val in parts:
            if kind == "hdr":
                lines = wrap(val)
            elif kind in ("cut", "sesslabel"):
                lines = wrap(val)
            elif kind == "his":
                lines = quote(val)
            elif kind == "e10":
                lines = list(val)
            else:
                lines = ["```text"] + list(val) + ["```"]
            side = "IN " if kind in ("his", "e10") else "OUT"
            say(f"-- {code} {kind} {side} sha256 {sha(chr(10).join(lines))[:16]} ({len(lines)} lines)")
            for x in lines:
                say(f"{side}| {x}")
            rendered.extend(lines)
    new = "\n".join(rendered) + "\n"
    for code, parts in E:
        for kind, val in parts:
            if kind in ("his",):
                check(unquote(quote(val)) == val, f"{code}: the his-part's lines come back byte for byte")

    say("")
    say(f"HASH TABLE (C7; history.md at {HIST_REV}, blob {HIST_BLOB})")
    say("block | source | history.md lines | sha256 source | sha256 copy read back | match")
    for row in hashes:
        say(" | ".join(row))

    old = git("show", f"{MAIN}:{DEC_PATH}")
    check(old.count(ANCHOR) == 1 and old.count(OLD_FIRST) == 1 and
          old.index(OLD_FIRST) == old.index(ANCHOR) + len(ANCHOR), "main's file: the anchor once, the old first entry right under it")
    first_hdr = rendered[0]
    check(first_hdr not in old, "main's file holds no entry of this script yet")
    pos = old.index(ANCHOR) + len(ANCHOR)
    result = old[:pos] + new + old[pos:]
    say(f"new text: {len(rendered)} lines, sha256 {sha(new)}; the file after: {result.count(chr(10))} lines, sha256 {sha(result)}")

    if mode == "write" and not FAILS:
        target = os.path.join(wt, DEC_PATH)
        cur = open(target, "rb").read()
        blob = subprocess.run(["git", "hash-object", target], capture_output=True, check=True).stdout.decode().strip()
        if not check(blob == DEC_BLOB and cur.decode("utf-8") == old and first_hdr not in cur.decode("utf-8"),
                     f"the target {target} is main's file (blob {DEC_BLOB}) and holds no entry of this script"):
            say("REFUSED: nothing written")
        else:
            with open(target, "wb") as fh:
                fh.write(result.encode("utf-8"))
            back = open(target, "rb").read().decode("utf-8")
            check(back == result, "read back: the file is main's with the new text right under the anchor, byte for byte")
            region = back[pos:pos + len(new)].split("\n")[:-1]
            check(region == rendered, "read back: the inserted region equals the rendered parts")
            i = 0
            for code, parts in E:
                for kind, val in parts:
                    n = len(wrap(val)) if kind in ("hdr", "cut", "sesslabel") else (
                        len(val) if kind in ("his", "e10") else len(val) + 2)
                    got = region[i:i + n]
                    if kind == "his":
                        check(unquote(got) == list(val), f"{code}: read back, inverse of the quote = the transcript's lines")
                    i += n
            check(i == len(region), "read back: every part accounted for")
    if table:
        with open(table, "w", encoding="utf-8") as fh:
            fh.write("\n".join(OUT + [("FAILED: " + "; ".join(FAILS)) if FAILS else "ALL CHECKS PASS"]) + "\n")
    print(("FAILED: " + "; ".join(FAILS)) if FAILS else "ALL CHECKS PASS")
    return 1 if FAILS else 0


if __name__ == "__main__":
    sys.exit(main())
