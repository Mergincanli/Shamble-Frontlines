#!/usr/bin/env python3
"""Verifier's own reader for DOCS-54's DECISIONS.md entries (read only; prints only).
Parses the new entries in the worktree, inverts "> " exactly, compares with the transcript records
(by the uuid each header names), with history.md at 110c303 and with commit 4460ef5's block, and
prints sha256 of each quote block (lines joined by "\n", no final newline)."""
import hashlib, json, re, subprocess

WT = "/home/user/Shamble-Frontlines/.gate/docs-after-54-wt"
MAIN = "/home/user/Shamble-Frontlines"
TR = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
UUID = r"([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})"

def sh(args, cwd):
    return subprocess.run(args, cwd=cwd, capture_output=True, check=True).stdout.decode("utf-8")

def h(s):
    return hashlib.sha256(s.encode("utf-8")).hexdigest()

FAIL = []
def check(ok, msg):
    print("   [%s] %s" % ("ok" if ok else "FAIL", msg))
    if not ok:
        FAIL.append(msg)

dec = open(WT + "/docs/DECISIONS.md", encoding="utf-8").read().split("\n")
start = dec.index("## Decisions, newest first") + 1
end = next(i for i in range(start, len(dec)) if dec[i].startswith("- **Taken by Mustafa, 2026-11-13, 22:27:05"))
new = dec[start:end]
print("new lines in section:", len(new), "file lines", start + 1, "-", end)
main_dec = sh(["git", "show", "origin/main:docs/DECISIONS.md"], WT).split("\n")
ms = main_dec.index("## Decisions, newest first") + 1
check(main_dec[ms].startswith("- **Taken by Mustafa, 2026-11-13, 22:27:05"), "main's first entry is the 2026-11-13 22:27:05 one")
check(dec[end:] == main_dec[ms:], "everything after the insertion equals main's")

entries = []
for l in new:
    if l.startswith("- **Taken by Mustafa,"):
        entries.append([])
    entries[-1].append(l)
print("entries:", len(entries))

hist = sh(["git", "show", "110c303:.checkpoints/netcode-proof-test/history.md"], MAIN).split("\n")
print("history.md blob at 110c303:", sh(["git", "rev-parse", "110c303:.checkpoints/netcode-proof-test/history.md"], MAIN).strip(), "lines", len(hist))

def hl(a, b):  # history lines a..b, 1-based inclusive, the two-space list indent removed
    out = []
    for x in hist[a - 1:b]:
        if x == "":
            out.append("")
        else:
            assert x.startswith("  "), (a, x)
            out.append(x[2:])
    return "\n".join(out)

def hq(n):  # the text inside the outer quote marks at the end of history line n
    m = re.search(r'verbatim:\*\* "(.*)"$', hist[n - 1])
    return m.group(1) if m else None

def unq(lines):
    out = []
    for l in lines:
        if l == ">":
            out.append("")
        else:
            assert l.startswith("> "), l
            out.append(l[2:])
    return "\n".join(out)

parsed = []
want = set()
for e in entries:
    hdr, kinds, j, cur = [], [], 0, None
    while j < len(e) and not (e[j] == ">" or e[j].startswith("> ")):
        hdr.append(e[j]); j += 1
    while j < len(e):
        l = e[j]
        if l == ">" or l.startswith("> "):
            if not cur or cur[0] != "Q":
                cur = ["Q", []]; kinds.append(cur)
            cur[1].append(l)
        elif l == "```text":
            k = j + 1
            while e[k] != "```":
                k += 1
            cur = ["F", e[j + 1:k]]; kinds.append(cur); j = k
        elif l.startswith("- _"):
            cur = ["I", [l]]; kinds.append(cur)
        elif l.startswith("  ") and cur and cur[0] == "I":
            cur[1].append(l)
        else:
            cur = ["X", [l]]; kinds.append(cur)
        j += 1
    ids = re.findall(r"(?:message|record)\s+" + UUID, " ".join(hdr))
    iids = [x for b in kinds if b[0] == "I" for x in re.findall(r"message\s+" + UUID, " ".join(b[1]))]
    want.update(ids + iids)
    parsed.append((hdr, kinds, ids, iids))

recs = {}
with open(TR, encoding="utf-8") as f:
    for n, line in enumerate(f, 1):
        if not any(u in line for u in want):
            continue
        d = json.loads(line)
        if d.get("uuid") in want:
            if d.get("type") == "attachment":
                t = d["attachment"].get("prompt"); who = "attachment:" + d["attachment"].get("type", "")
            else:
                c = d["message"]["content"]
                t = c if isinstance(c, str) else "\n".join(x.get("text", "") for x in c if x.get("type") == "text")
                who = d["message"]["role"]
            recs[d["uuid"]] = dict(line=n, ts=d.get("timestamp"), who=who, side=d.get("isSidechain"), text=t)

# history.md ranges per entry (from the headers), in entry order
HRANGES = {
    "10:43:48": [("q", 2205)], "07:00:54": [("q", 2200)], "21:09:27": [("q", 2190)],
    "20:48:10": [(2180, 2187)], "19:46:36": [(2166, 2170)], "18:42:53": [],
    "16:21:11": [(2151, 2161)], "15:34:42": [(2134, 2139)], "12:34:01": [(2116, 2116), (2129, 2129)],
    "12:01:41": [(2091, 2107)], "09:02:29": [],
}

mainns = sh(["git", "show", "origin/main:.claude/NIGHT_SHIFT.md"], WT).split("\n")
ns3 = "\n".join(mainns[44:47])
print("NIGHT_SHIFT.md:45-47 on origin/main sha256:", h(ns3))

commit = sh(["git", "log", "-1", "--format=%B", "4460ef5"], WT).split("\n")
cq = [l for l in commit if l == ">" or l.startswith("> ")]
# check the commit's "> " lines are one contiguous run
ci = [i for i, l in enumerate(commit) if l == ">" or l.startswith("> ")]
check(ci == list(range(ci[0], ci[-1] + 1)), "4460ef5's '> ' lines are one contiguous block (%d lines)" % len(ci))

table = []
for hdr, kinds, ids, iids in parsed:
    title = hdr[0]
    stamp = re.search(r"(\d\d:\d\d:\d\d) UTC", title).group(1)
    print("\n=== ", title[:110])
    for k, b in kinds:
        if k == "X":
            check(False, "unclassified line outside quote/fence/italic: %r" % b[0])
    qs = [unq(b) for k, b in kinds if k == "Q"]
    for k, b in kinds:
        if k == "Q":
            back = ["> " + x if x else ">" for x in unq(b).split("\n")]
            check(back == b, "exact inverse transform holds for a %d-line block" % len(b))
    his = recs.get(ids[0]) if ids else None
    if stamp == "09:02:29":
        check(len(qs) == 1 and qs[0] == unq(cq), "E10 quote == 4460ef5's '> ' block (sha %s)" % h(qs[0]))
        check(kinds[0][1] == cq, "E10 lines == commit lines byte for byte")
        table.append(("E10", h(qs[0])))
        continue
    check(his is not None and his["who"] in ("user", "attachment:queued_command") and his["side"] is False,
          "his record: line %s %s %s sidechain=%s" % (his and his["line"], his and his["ts"], his and his["who"], his and his["side"]))
    check(his["ts"].startswith("2026-11-%s" % title.split("2026-11-")[1][:2]) and stamp in his["ts"], "header stamp matches record stamp")
    for u in iids:
        r = recs[u]
        check(r["who"] == "assistant", "session-text id %s is the assistant's (line %d, %s)" % (u, r["line"], r["ts"]))
        for k, b in kinds:
            if k == "F":
                pass
    # every fenced block must be found in one of the cited assistant messages (or its quoted text)
    for k, b in kinds:
        if k == "F":
            fb = "\n".join(b)
            found = [u for u in iids if fb in recs[u]["text"]]
            check(bool(found), "fenced block (%d lines, sha %s) found verbatim in assistant msg %s" % (len(b), h(fb)[:16], found))
            check(fb not in his["text"], "fenced block is not his text")
            if len(b) == 3:
                check(fb == ns3, "E3 three lines == origin/main NIGHT_SHIFT.md:45-47 (sha %s)" % h(fb))
    # quotes vs his message
    full = his["text"]
    if len(qs) == 1 and qs[0] == full:
        check(True, "quote == his whole message (sha %s)" % h(full))
    else:
        pos = 0
        for q in qs:
            p = full.find(q, pos)
            al = p >= 0 and (p == 0 or full[p - 1] == "\n") and (p + len(q) == len(full) or full[p + len(q)] == "\n")
            check(al, "partial block is a line-aligned run of his message, in order (sha %s)" % h(q))
            pos = p + len(q)
        rest = full
        for q in qs:
            rest = rest.replace(q, "\u2588", 1)
        print("   cut (his text not quoted):")
        for x in rest.split("\n"):
            if x.strip() and x != "\u2588":
                print("     |", x[:150])
    # history.md
    hr = HRANGES[stamp]
    for q, r in zip(qs, hr):
        if r[0] == "q":
            src = hq(r[1])
            check(src == q, "history.md:%d's quoted text == quote" % r[1])
        else:
            src = hl(*r)
            if src == q:
                check(True, "history.md:%d-%d == quote" % r)
            else:
                d = [(i + r[0], a, b) for i, (a, b) in enumerate(zip(src.split("\n"), q.split("\n"))) if a != b]
                print("   history.md:%d-%d differs from quote at:" % r, [(i, repr(a[-20:]), repr(b[-20:])) for i, a, b in d])
                check(len(d) == 1 and d[0][2] == d[0][1] + " " and r == (2151, 2161) and d[0][0] == 2151,
                      "the only difference is the named E6 trailing space at :2151")
    if not hr:
        print("   (no history.md source)")
    for q in qs:
        table.append((stamp, h(q)))

print("\nHASHES (stamp, sha256 of quote):")
for t in table:
    print("  ", *t)
print("\nFAILS:", len(FAIL))
for x in FAIL:
    print("  ", x)
