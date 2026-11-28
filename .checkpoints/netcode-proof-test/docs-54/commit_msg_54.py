"""DOCS-54: build the build commit's message from its template (.gate/docs-54/commit-msg-template.txt) by copying,
word for word, the HASH TABLE rows of the copy script's write run (docs-54/copy-table-write.txt), and by computing the
sha256 of the scripted moves from the worktree's files (the same values move_lines_54.py printed). Writes only the
message file (.gate/docs-54/commit-msg.txt). Run from the main folder's root:
  python3 -I .checkpoints/netcode-proof-test/docs-54/commit_msg_54.py"""
import hashlib

WT = "/home/user/Shamble-Frontlines/.gate/docs-after-54-wt"
TPL = "/home/user/Shamble-Frontlines/.gate/docs-54/commit-msg-template.txt"
OUT = "/home/user/Shamble-Frontlines/.gate/docs-54/commit-msg.txt"
TABLE = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/docs-54/copy-table-write.txt"
MOVED = {"kit-v3-2": "335312651ef3eb2de9f999139ff8f8070ccafb70feecec29256a4177867e1df0",
         "kit-v3-3": "b8d41e6dcc525d073858961f7f7a6cf922c9e4cc022cca162badcff387b17139",
         "b1-plan-view": "9b9e5cad2d6a06902aef493d519d89b42e21ad31b19f8873551c7f24792d2cf7"}
ARCH = {"#53 lines 88-105": "29cff002e8657c62e61c5edd05b2a2aafe3a161297c816b6bb070ca6c2042ca2",
        "#54 lines 23-27": "d24ba2a3ea9023dee6cc1327aa6018a12d7eec5b67dcf18d177ea3f1850cfc6d"}


def sha(s):
    return hashlib.sha256(s.encode("utf-8")).hexdigest()


t = open(TABLE, encoding="utf-8").read().split("\n")
i = next(k for k, x in enumerate(t) if x.startswith("HASH TABLE"))
rows = []
for x in t[i + 1:]:
    if " | " not in x:
        break
    rows.append(x)
assert rows[0].startswith("block | source") and len(rows) == 14, rows[:2]

R = open(f"{WT}/docs/ROADMAP.md", encoding="utf-8").read().split("\n")
moved = [x[2:] for x in R[84:87]]
assert [sha(x) for x in moved] == list(MOVED.values()), "the moved ROADMAP lines changed"
A = open(f"{WT}/docs/STATUS_ARCHIVE.md", encoding="utf-8").read().split("\n")
k53 = next(k for k, x in enumerate(A) if x.startswith("- **From KEPT-LOGS**"))
k54 = next(k for k, x in enumerate(A) if x.startswith("- **From the night-shift lines**"))
c53 = [x[2:] for x in A[k53 + 5:k53 + 5 + 18]]
c54 = [x[2:] for x in A[k54 + 6:k54 + 6 + 5]]
assert [sha("\n".join(c53)), sha("\n".join(c54))] == list(ARCH.values()), "the archive copies changed"
moves = ["The scripted moves (sha256 of the lines moved or copied, byte for byte):"]
moves += [f"- ROADMAP, {k} (Recent work -> Build history): {v}" for k, v in MOVED.items()]
moves += [f"- the archive's Follow-ups, PR {k}: {v}" for k, v in ARCH.items()]

tpl = open(TPL, encoding="utf-8").read()
assert tpl.count("{HASH_TABLE}") == 1 and tpl.count("{MOVES}") == 1
msg = tpl.replace("{HASH_TABLE}", "\n".join(rows)).replace("{MOVES}", "\n".join(moves))
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(msg)
back = open(OUT, encoding="utf-8").read()
assert back == msg and all(r in back for r in rows)
print(f"{OUT}: {back.count(chr(10))} lines, sha256 {sha(back)}; {len(rows) - 1} hash rows copied word for word")
