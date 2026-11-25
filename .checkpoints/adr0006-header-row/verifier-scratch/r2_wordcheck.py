# Round 2: apply plan section 3 (revision 2 texts) as amended by revision 3 to the base ADR, compare with ab15d6d.
import re, subprocess, difflib, sys
R = "/home/user/Shamble-Frontlines"
def show(ref, p): return subprocess.run(["git", "show", f"{ref}:{p}"], capture_output=True, text=True, cwd=R).stdout
ADR = "docs/adr/0006-infrastructure-operations.md"
base, head = show("09faefa", ADR), show("ab15d6d", ADR)
plan = show("HEAD", ".checkpoints/adr0006-header-row/plan.md")
N = lambda s: re.sub(r"\s+", " ", s).strip()
def after(eid):
    m = re.search(r"\*\*" + eid + r"\.[^\n]*\n(?:.*\n)*?- After: (.*(?:\n  .*)*)", plan)
    return N(m.group(1))[1:-1]  # strip the outer quotes
def item(rid):
    i = plan.index("**" + rid + ".")
    j = plan.find("\n**", i + 3)
    return plan[i:j]
def pair(txt):  # first '"X" becomes "Y"' in a bullet; Y greedy to the bullet's last quote
    for b in re.split(r"\n(?=- )", txt):
        b = N(b)
        m = re.search(r'"(.*?)" becomes "(.*)"', b)
        if m: return m.group(1), m.group(2)
fails = []
def check(name, cond):
    print(("OK   " if cond else "FAIL ") + name)
    if not cond: fails.append(name)
# --- E7 exact, line by line, against R3-1's fenced block
blk = re.search(r"\*\*R3-1\..*?```\n(.*?)```", plan, re.S).group(1).rstrip("\n").split("\n")
hl = head.split("\n")
k = next(i for i, l in enumerate(hl) if l.startswith("   *Workers static assets, weighed again*"))
check(f"E7 block {len(blk)} lines equals head lines {k+1}-{k+len(blk)} byte for byte", hl[k:k+len(blk)] == blk)
check("E7 followed by '2. **Preview privacy.**'", hl[k+len(blk)].startswith("2. **Preview privacy.**"))
check("E7 preceded by the *Need:* line", "*Need:* VISION_V2 §6.1" in hl[k-2] + hl[k-1])
# --- revision 3 pairs
r33 = pair(item("R3-3")); r34 = pair(item("R3-4")); r35 = pair(item("R3-5")); r36 = pair(item("R3-6"))
r310 = pair(item("R3-10")); r311 = pair(item("R3-11"))
for n, p in (("R3-3", r33), ("R3-4", r34), ("R3-5", r35), ("R3-6", r36), ("R3-10", r310), ("R3-11", r311)):
    print("   ", n, "X:", p[0][:70], "| Y:", p[1][:90], "...")
# advisor's exact words for R3-10 / R3-11 equal the plan's
adv = N(show("HEAD", ".checkpoints/adr0006-header-row/advisor-plan.md"))
check("R3-10 text equals the advisor's recorded words", f'"{r310[0]}" becomes "{r310[1]}"' in adv)
check("R3-11 text equals the advisor's recorded words", f'"{r311[0]}" becomes "{r311[1]}"' in adv)
# --- expected texts
e1 = after("E1").replace(r33[0], r33[1]); assert r33[0] in after("E1")
e3 = after("E3")
e3 = e3.replace("(read 2026-11-24, and again for this revision) (PM-0006-7;", "(read 2026-11-24 and 2026-11-25; PM-0006-7;")
e3 = e3.replace("(Mustafa's STEP4-COUNT (b), 2026-11-25).", "(Mustafa's STEP4-COUNT (b), 2026-11-25, 00:55 Amsterdam time).")
for piece in [p.strip(' ,;"') for p in re.split(r"\[[^\]]*\]", N(item("R3-2")).split('- "', 1)[1].split('"\n')[0]) if len(p.strip(' ,;"')) > 8][:3]:
    pass
r32 = N(item("R3-2")); r32q = r32[r32.index('- "') + 3: r32.index('" - It replaces')]
r32pieces = [p.strip(' ,;') for p in re.split(r"\[\.\.\.\]", r32q) if len(p.strip(' ,;')) > 8]
for p in r32pieces: check("R3-2 piece in expected E3: " + p[:60], p in e3)
e4 = after("E4").replace(r35[0], r35[1]); assert r35[0] in after("E4")
e6, e9, e10, e2 = after("E6"), after("E9"), after("E10"), after("E2")
# --- apply to the normalized base
B = N(base)
def rep(old, new, label):
    global B
    c = B.count(old); check(f"{label}: 'before' found once in base (count {c})", c == 1); B = B.replace(old, new)
rep("Pages' Free plan allows 500 builds a month (R6-HOST-F3, r6 lines 72–79), and whether a direct upload counts against them is a spike item (part 14). If uploads count, only the newest push of each pull request deploys (ADR 0002's concurrency cancel) and the month's deploys are counted; a paid plan is Mustafa's after a price read (proposed, not researched).", e3, "E3")
rep(r310[0], r310[1], "R3-10")
need_end = "VISION_V2 §4 (desktop browsers first). 2. **Preview privacy.**"
rep(need_end, "VISION_V2 §4 (desktop browsers first). " + N("\n".join(blk)) + " 2. **Preview privacy.**", "E7 insert")
rep("- whether a direct upload counts against Pages' 500 builds a month (R6-HOST-REC1, r6 lines 633–634);", e6, "E6")
rep(r34[0], r34[1], "E1b R3-4")
e1_before = "previews `X-Robots-Tag: noindex`, production none; the preview built with"
e1_new = e1.split("; the preview built with")[0].split("every `.wasm` as `application/wasm`; ")[1] + "; the preview built with"
rep(e1_before, e1_new, "E1 (+R3-3)")
rep("a `noindex` rule added to production |", "a `noindex` rule added to production (fails on production's own address) |", "E2")
rep(r311[0], r311[1], "R3-11")
e4_before = "anything else recorded as read; if uploads count, part 1's consequence follows (only the newest push per pull request is deployed, and the month's deploys are counted) |"
rep(e4_before, e4.replace("[...] ", "") + " |", "E4 (+R3-5)")
rep("one pair of readings; passes when both are recorded |", e9 + " |", "E9")
rep(r36[0], r36[1], "E11 R3-6")
rep("read the count again. Expected: two numbers recorded (PM-0006-7).", e10.replace("[...] ", ""), "E10")
H = N(head)
check("expected head == actual head (whitespace-normalised, whole file)", B == H)
if B != H:
    a, b = B.split(" "), H.split(" ")
    for op in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes():
        if op[0] != "equal":
            print("  DIFF", op[0], "expected:", " ".join(a[op[1]:op[2]])[:300], "| actual:", " ".join(b[op[3]:op[4]])[:300])
print("RESULT", "all OK" if not fails else f"{len(fails)} fail(s)")
