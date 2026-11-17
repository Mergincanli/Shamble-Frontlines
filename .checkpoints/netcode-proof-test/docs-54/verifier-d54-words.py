#!/usr/bin/env python3
"""Verifier's own reader for DOCS-54's DECISIONS.md entries (read only; prints only).
Parses the new entries in the worktree, inverts "> " exactly, compares with the transcript records
(by the uuid each header names), with history.md at 110c303 and with commit 4460ef5's block, and
prints sha256 of each quote block (lines joined by "\n", no final newline)."""
import hashlib, json, re, subprocess, sys

WT = "/home/user/Shamble-Frontlines/.gate/docs-after-54-wt"
MAIN = "/home/user/Shamble-Frontlines"
TR = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"

def sh(args, cwd):
    return subprocess.run(args, cwd=cwd, capture_output=True, check=True).stdout.decode("utf-8")

def h(s):
    return hashlib.sha256(s.encode("utf-8")).hexdigest()

dec = open(WT + "/docs/DECISIONS.md", encoding="utf-8").read().split("\n")
start = dec.index("## Decisions, newest first") + 1
end = next(i for i in range(start, len(dec)) if dec[i].startswith("- **Taken by Mustafa, 2026-11-13, 22:27:05"))
new = dec[start:end]
print("new lines in section:", len(new), "file lines", start + 1, "-", end)
main_dec = sh(["git", "show", "origin/main:docs/DECISIONS.md"], WT).split("\n")
# the new block must be the only insertion at that point
ms = main_dec.index("## Decisions, newest first") + 1
assert main_dec[ms].startswith("- **Taken by Mustafa, 2026-11-13, 22:27:05"), main_dec[ms]
assert dec[end:] == main_dec[ms:], "tail of file differs from main after the insertion"

# split into entries
entries = []
for i, l in enumerate(new):
    if l.startswith("- **Taken by Mustafa,"):
        entries.append([])
    entries[-1].append(l)
print("entries:", len(entries))

# history.md at 110c303
hist = sh(["git", "show", "110c303:.checkpoints/netcode-proof-test/history.md"], MAIN).split("\n")
hist_blob = sh(["git", "rev-parse", "110c303:.checkpoints/netcode-proof-test/history.md"], MAIN).strip()
print("history.md blob at 110c303:", hist_blob, "lines", len(hist))

def H(a, b):  # history lines a..b (1-based, inclusive)
    return hist[a - 1:b]

# transcript: collect records by uuid named in headers
want = set()
parsed = []
for e in entries:
    kinds = []  # list of (kind, lines)
    hdr = []
    j = 0
    while j < len(e) and not (e[j] == ">" or e[j].startswith("> ")):
        hdr.append(e[j]); j += 1
    cur = None
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
        elif l.startswith("- _") or (l.startswith("  ") and cur and cur[0] == "I"):
            if l.startswith("- _"):
                cur = ["I", [l]]; kinds.append(cur)
            else:
                cur[1].append(l)
        else:
            print("UNCLASSIFIED LINE:", repr(l)); cur = ["X", [l]]; kinds.append(cur)
        j += 1
    text = " ".join(hdr)
    ids = re.findall(r"(?:message|record) ([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})", text)
    ids += [x for blk in kinds if blk[0] == "I" for x in re.findall(r"message ([0-9a-f]{8}-[0-9a-f-]{27})", " ".join(blk[1]))]
    want.update(ids)
    parsed.append((hdr, kinds, ids))

recs = {}
with open(TR, encoding="utf-8") as f:
    for n, line in enumerate(f, 1):
        if not any(u in line for u in want):
            continue
        d = json.loads(line)
        if d.get("uuid") in want:
            if d.get("type") == "attachment":
                t = d["attachment"].get("prompt")
            else:
                c = d["message"]["content"]
                t = c if isinstance(c, str) else "\n".join(x.get("text", "") for x in c if x.get("type") == "text")
            recs[d["uuid"]] = (n, d.get("timestamp"), d.get("type"), d.get("message", {}).get("role") if d.get("message") else "attachment", d.get("isSidechain"), t)

def unq(lines):
    out = []
    for l in lines:
        if l == ">":
            out.append("")
        else:
            assert l.startswith("> "), l
            out.append(l[2:])
    return "\n".join(out)

for hdr, kinds, ids in parsed:
    title = hdr[0][:90]
    print("\n=== ", title)
    for u in ids:
        r = recs.get(u)
        print("   id", u, "-> line", r[0] if r else None, r[1] if r else None, r[2] if r else None, r[3] if r else None, "sidechain", r[4] if r else None)
    his = recs.get(ids[0])
    for k, blk in kinds:
        if k == "Q":
            q = unq(blk)
            # exact inverse check
            back = ["> " + x if x else ">" for x in q.split("\n")]
            assert back == blk
            pos = his[5].find(q) if his else -1
            full = his and q == his[5]
            # line-aligned substring?
            aligned = his and pos >= 0 and (pos == 0 or his[5][pos - 1] == "\n") and (pos + len(q) == len(his[5]) or his[5][pos + len(q)] == "\n")
            print("   Q lines", len(blk), "sha", h(q), "== whole msg:", bool(full), "line-aligned substring:", bool(aligned), "at", pos)
        elif k == "F":
            print("   F lines", len(blk), "sha", h("\n".join(blk)))
        elif k == "I":
            print("   I", " ".join(blk)[:200])
        else:
            print("   X", blk)
    if his:
        # what of his message is NOT in quotes
        rest = his[5]
        for k, blk in kinds:
            if k == "Q":
                rest = rest.replace(unq(blk), "█", 1)
        print("   cut from his message (non-quoted remainder, len %d):" % len(rest))
        for x in rest.split("\n"):
            if x.strip() and x != "█":
                print("     |", x[:160])

print("\nrecords json:")
json.dump({u: [v[0], v[1], v[5]] for u, v in recs.items()}, sys.stdout, ensure_ascii=False)
print()
