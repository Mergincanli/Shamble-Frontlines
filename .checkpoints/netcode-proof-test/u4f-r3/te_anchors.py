# Read-only: prints anchor counts, spans and first/last words for U4f brief rev 3's placeholders.
# test-engineer's own check (independent of stamps_anchors.py). Writes nothing.
import hashlib, re, os

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/"
F = {
    "carries": "u4s-carries.md", "history": "history.md", "df-ruling": "advisor-u4df.md",
    "j-r2": "advisor-u4j-r2.md", "m9": "advisor-u4d2-m9.md", "d2-r2": "advisor-u4d2-r2.md",
    "te-m3": "advisor-te-m3.md", "c62h": "advisor-u4r-c62h-full.md",
    "b1-p3": "advisor-brawl1-sets-p3-full.md", "r-b2": "advisor-u4r-build2-full.md",
    "r-r1": "advisor-u4r-r1-full.md", "u4r-report": "u4r-report.md", "plan": "u4-build-plan.md",
    "plan.md": "plan.md",
}
STAMP = {"df-ruling": "842ec092", "j-r2": "7efa9410", "m9": "f31f7876", "d2-r2": "e776f0ba",
         "te-m3": "5ecc3e23", "c62h": "18dd3d20", "b1-p3": "1bc5889c", "r-b2": "f4f5ac92",
         "r-r1": "7b552e8c", "u4r-report": "c5dee3fa",
         "plan": "43ff74f2a3ce903f1061c67f53b416ae2498f3890f2116426feeab4373656385",
         "plan.md": "3eecd28196a529969e9cce1e6eee9dd35100b796414242b835903557e28bc09b",
         "carries": "57600c9c79bdccfe521dc1181ee9ab9d86087e11c38b788b9b223072c9c01dce",
         "history": "3bfe1aa2d5a3f781267956e011079c42b6cc12e61a058af1af9e395076246854"}

def load(k):
    p = N + F[k]
    b = open(p, "rb").read()
    return b.decode("utf-8"), hashlib.sha256(b).hexdigest()

def norm(s):
    return re.sub(r"\s+", " ", s)

def words(s, n=6):
    w = norm(s).strip().split(" ")
    return " ".join(w[:n]), " ".join(w[-n:]), len(w)

print("== stamps")
for k in F:
    t, h = load(k)
    ok = h.startswith(STAMP[k]) if k in STAMP else None
    print(k, F[k], h[:16], "lines", t.count("\n"), "stamp-ok" if ok else ("NO-STAMP" if ok is None else "STAMP-DIFFERS"))

def lineno(t, idx):
    return t.count("\n", 0, idx) + 1

def span_text(k, start, end):
    t, _ = load(k)
    nt = norm(t)
    c = nt.count(start)
    i = nt.find(start)
    j = nt.find(end, i) if i >= 0 else -1
    ce = nt.count(end)
    if i < 0 or j < 0:
        return c, ce, None
    return c, ce, nt[i:j + len(end)]

def raw_lines(k, start, end):
    # line numbers in the raw file for start (first raw-line match by its first 30 chars) and end
    t, _ = load(k)
    L = t.split("\n")
    s0 = start.strip()[:30]
    e0 = end.strip()[-25:]
    sl = [i + 1 for i, l in enumerate(L) if s0 in l]
    el = [i + 1 for i, l in enumerate(L) if e0 in l]
    return sl, el

def from_to(pid, k, start, end, stated):
    c, ce, s = span_text(k, start, end)
    sl, el = raw_lines(k, start, end)
    if s is None:
        print(pid, k, "start count", c, "end count", ce, "NOT FOUND", "raw start lines", sl, "raw end lines", el)
        return
    a, b, n = words(s)
    print(pid, k, "start count", c, "end count(file)", ce, "raw start lines", sl[:5], "raw end lines", el[:5],
          "stated", stated, "| words", n, "| first:", a, "| last:", b)

def line_begin(pid, k, prefix, stated, extra=0, before=None):
    t, _ = load(k)
    L = t.split("\n")
    hits = [i for i, l in enumerate(L) if l.startswith(prefix)]
    anyc = t.count(prefix.strip())
    if len(hits) != 1:
        print(pid, k, "line-begin hits", [h + 1 for h in hits], "anywhere", anyc, "stated", stated)
        return
    i = hits[0]
    s = "\n".join(L[i:i + 1 + extra])
    a, b, n = words(s)
    bf = ""
    if before is not None:
        bf = " | line before equals: " + str(L[i - 1] == before)
    print(pid, k, "line-begin hits", [i + 1], "anywhere", anyc, "stated", stated, "| span lines", i + 1, "-",
          i + 1 + extra, "| words", n, "| first:", a, "| last:", b, bf)

def entry(pid, k, prefix, stated, num):
    t, _ = load(k)
    L = t.split("\n")
    hits = [i for i, l in enumerate(L) if l.startswith(prefix)]
    if len(hits) != 1:
        print(pid, k, "entry hits", [h + 1 for h in hits], "stated", stated)
        return
    i = hits[0]
    j = i + 1
    while j < len(L) and not (re.match(r"^\d+\. ", L[j]) or L[j].startswith("## ")):
        j += 1
    s = "\n".join(L[i:j])
    a, b, n = words(s)
    en = re.match(r"^(\d+)\.", L[i]).group(1)
    print(pid, k, "entry hits", [i + 1], "span", i + 1, "-", j, "(stated", stated + ")", "entry no", en,
          "ok" if en == num else "NUMBER-DIFFERS", "| words", n, "| first:", a, "| last:", b,
          "| trailing blank lines in span:", sum(1 for x in L[i:j] if x.strip() == ""))

print("== P1")
t, _ = load("carries")
L = t.split("\n")
s1 = [i + 1 for i, l in enumerate(L) if l == "## U4f (clock and commands)"]
s2 = [i + 1 for i, l in enumerate(L) if l == "## U4g (prediction, interpolation and events)"]
print("P1 start lines", s1, "U4g lines", s2)
if len(s1) == 1 and len(s2) == 1:
    body = L[s1[0] - 1:s2[0] - 1]
    nums = [re.match(r"^(\d+)\.", l).group(1) for l in body if re.match(r"^\d+\.", l)]
    print("P1 entry numbers at line start", nums, "span", s1[0], "-", s2[0] - 1)

print("== P2")
from_to("P2", "carries", "Each entry: the finding (review, id), then", "never the rule.", ":9-10")
print("== P3-P13")
entry("P3", "carries", "57. **U4E-Q6, the room's half", ":223-228", "57")
entry("P4", "carries", "74. **NC-M6, the endpoint's lead", ":345-356", "74")
entry("P5", "carries", "21. **netcode F2 (the room's half):**", ":148-149", "21")
entry("P6", "carries", "27. **this pass (CMD_CATCHUP):**", ":166-168", "27")
entry("P7", "carries", "18. **this pass (the tick grep):**", ":129-132", "18")
entry("P8", "carries", "39. **rust fix review nit (u4s-fix-review-rust.md:114-115", ":169-179", "39")
entry("P9", "carries", "50. **the event age", ":180-183", "50")
entry("P10", "carries", "61. **U4j's hash rules, the room and the client", ":229-247", "61")
entry("P11", "carries", "58. **U4E-Q6's two conditions", ":269-273", "58")
entry("P12", "carries", "76. **A6 and SEC-U4E-5, ending a session", ":366-373", "76")
entry("P13", "carries", "77. **U4r's readings of U4e's room and client", ":374-383", "77")
print("== history")
line_begin("P14", "history", "  - Builders: pass or fail comes from the exit code", ":292-293", extra=1,
           before="  USAGE-SAVINGS: (a), all three, with these limits.")
from_to("P15", "history", "A new bar, or a change to one, comes to me under Decisions needed", "on the ruling alone.", ":2125")
line_begin("P16", "history", "  From now on: every change to marks.json or to the judge", ":2127")
from_to("P17", "history", "write commands in the simple forms that need no approval", "before you send it.", ":2187")
from_to("P18", "history", "2. SIZE-CHECK from now on.", "as the rule says.", ":2048-2051")
line_begin("P19", "history", "  C62H-KEEP: (a). The check stays as built.", ":2141")
from_to("P20", "history", "C62H-BOUND: (a), the exact limit.", "stay in until it is built.", ":2143-2146")
line_begin("P21", "history", "  - The stop rules of your post stand. The check is never reworded to fit.", ":2175")
line_begin("P22", "history", "      - (a) Yes. I may pack one or two closed", ":2212")
line_begin("P23", "history", "  - **C62HB-Q4:** should the counter-wrap test (PM-0001-58)", ":2216")
print("== closed records")
from_to("P24", "df-ruling", "Write files with the", "whole with Write.", ":462-464")
from_to("P25", "df-ruling", "a failure in <crate>", "and go on.", ":465-468")
t, _ = load("j-r2")
L = t.split("\n")
h = [i + 1 for i, l in enumerate(L) if "From now on: " in l]
print("P26 j-r2 lines with 'From now on: '", h)
if h:
    l = L[h[0] - 1]
    rest = l.split("From now on: ", 1)[1]
    a, b, n = words(rest)
    print("P26 words", n, "| first:", a, "| last:", b)
from_to("P27", "m9", "Between waves, with no lane running", "never `--target` without `-p`.", ":252-256")
from_to("P28", "m9", "(1) never pass `--target`", "no session removes it by hand.", ":259-264")
from_to("P29", "d2-r2", "never run cargo", "may use it as one.", ":304-308")
from_to("P30", "te-m3", "After it ends, FX-10's saved output", "says so.", ":127-128")
from_to("P31", "c62h", "Until the fix lands, the rule is:", "as both run scripts already do.", ":20")
t, _ = load("b1-p3")
L = t.split("\n")
print("P32 b1-p3 line 158:", words(L[157])[0], "...", words(L[157])[1], "| words", words(L[157])[2])
print("P32 lines containing 'EVQ-LOAD'", [i + 1 for i, l in enumerate(L) if "EVQ-LOAD" in l])
from_to("P33", "r-b2", "Adds a card follow-up", "No push for it.", ":28")
from_to("P34", "r-r1", "Contract half:", "a usage error (exit 2).", ":122")
print("== u4r-report")
t, _ = load("u4r-report")
L = t.split("\n")
a1 = [i + 1 for i, l in enumerate(L) if l.startswith("- **TE-B1-R2-1 with RR2-B1-1**")]
a2 = [i + 1 for i, l in enumerate(L) if l.startswith("- **TE-B1-R2-2**")]
print("P35 start", a1, "end-before", a2)
if len(a1) == 1 and len(a2) >= 1:
    nxt = [x for x in a2 if x > a1[0]]
    s = "\n".join(L[a1[0] - 1:nxt[0] - 1])
    print("P35 span", a1[0], "-", nxt[0] - 1, "words", words(s)[2], "| first:", words(s)[0], "| last:", words(s)[1])
line_begin("P36", "u4r-report", "  - RR-B1-7 (1), the room_tick loop rewrite", ":1163")
