"""C62H-REVIEW's ruling (advisor-c62h-review-full.md), its word-for-word parts, from the workflow's journal
(wf_4ce9b8cd-ad2):
1. TE-C62H-2 (Low, fix now): u4s-carries.md gains one line under carry 62 (h), after the "(h) amended" line that ends
   "U4r's brief 2.5 row (:643) reads this way.": the read's line word for word (between the quotes of its fix), plus
   the ruling's "Mustafa's answer to C62H-KEEP decides whether it stays". Wrapped at 120 columns, indent 4; no word
   changes.
2. TE-C62H-1 (Medium, its report lines now): u4r-report.md's -17 L1 P3 and -17 L1 P2 lines (the BRAWL-1 re-run sets)
   each gain one sub-line: the ruling's report words for that profile, word for word, and its sentence that starts
   "These are the advisor's estimates". Only the labels are the script's. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/c62h_review_apply.py"""
import hashlib
import json
import textwrap

J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_4ce9b8cd-ad2/journal.jsonl")
N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
label = {r["key"]: r["label"] for r in rows if r["type"] == "started"}
res = {label[r["key"]]: r["result"] for r in rows if r["type"] == "result"}
rd, ru = res["c62h-read-test"], res["advisor-c62h"]

# 1. The carries line.
fix = [f for f in rd["findings"] if f["id"] == "TE-C62H-2"][0]["fix"]
a = fix.index("after :316: '") + len("after :316: '")
b = fix.index("' Nothing in marks.json changes.")
line = fix[a:b]
assert line.startswith("**(h) amended again") and line.endswith('"parts": ["L1 P0"]}.'), line
add = "Mustafa's answer to C62H-KEEP decides whether it stays"
assert add in [f for f in ru["each_finding"] if f["id"] == "TE-C62H-2"][0]["home"]
words = f"{line} {add}."
wrapped = textwrap.fill(words, width=120, initial_indent="    ", subsequent_indent="    ", break_long_words=False,
                        break_on_hyphens=False)
assert " ".join(wrapped.split()) == " ".join(words.split())
C = f"{N}/u4s-carries.md"
with open(C, encoding="utf-8") as fh:
    text = fh.read()
anchor = "    PM-0001-17/whole-as-lost; the check's JSON stays word for word. U4r's brief 2.5 row (:643) reads this way.\n"
assert text.count(anchor) == 1 and "(h) amended again" not in text
text = text.replace(anchor, anchor + wrapped + "\n")
with open(C, "w", encoding="utf-8") as fh:
    fh.write(text)

# 2. The report's P3 and P2 lines.
rw = ru["report_words"]
src = {p: [l.strip() for l in rw.split("\n") if l.strip().startswith(f"- {p}: ")] for p in ("P2", "P3")}
est = [l.strip() for l in rw.split("\n") if l.strip().startswith("These are the advisor's estimates")]
assert all(len(v) == 1 for v in src.values()) and len(est) == 1, (src, est)
R = f"{N}/u4r-report.md"
with open(R, encoding="utf-8") as fh:
    rep = fh.read()
assert "Against the profile's loss (C62H-REVIEW" not in rep
for p in ("P3", "P2"):
    head = f"- -17 L1 {p}: "
    i = [k for k, l in enumerate(rep.split("\n")) if l.startswith(head) and "npt-lab counts" in l]
    assert len(i) == 1, (p, i)
    lines = rep.split("\n")
    sub = (f"  - Against the profile's loss (C62H-REVIEW; the advisor's report words, advisor-c62h-review-full.md): "
           f"{src[p][0][2:]} {est[0]}")
    lines.insert(i[0] + 1, sub)
    rep = "\n".join(lines)
with open(R, "w", encoding="utf-8") as fh:
    fh.write(rep)
for f in (C, R):
    with open(f, "rb") as fh:
        print(f"{f}: sha256 {hashlib.sha256(fh.read()).hexdigest()}")
