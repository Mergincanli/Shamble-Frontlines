"""U4f's launch fill on the builder brief rev 3.3 (u4f-brief.md section 12 items 1, 2, 3 and 10; section 0.2's living
stamps). It changes the brief only:
- `<base>` = `git rev-parse HEAD`, with spikes/netcode-proof equal to 0ad9661's (results/ excluded as D1 states);
- `<<PV1>>` = u4f-run.sh's sha256, with `bash -n` exit 0;
- `<<P1>>` to `<<P36>>` copied word for word by item 3's rules, each copy once, under its name at the end of item 3's
  list, every other place a pointer "Pn (copied in section 12 item 3)" (the advisor's fold check, section 4); a copy is
  its span with every run of white space read as one space, in quotation marks (one that is itself a quotation stands
  as it is); C2's, C28's and 0.5's name-only mentions kept; 0.5's and item 3's sentences changed as its conditions say;
- 0.2's carries and history rows re-stamped, with one sentence after the table saying so.
A table row or heading is edited in its line; any other unit is rewrapped within 120 from its first changed line to
its end only where a line would pass 120. Every check runs before any write; one failure and nothing is written.
Run from the repo root: python3 -I .checkpoints/netcode-proof-test/u4f_fill.py [--apply]"""
import difflib
import hashlib
import re
import subprocess
import sys
import textwrap

N = ".checkpoints/netcode-proof-test"
APPLY = "--apply" in sys.argv
BRIEF, CARR, HIST, RUNNER = f"{N}/u4f-brief.md", f"{N}/u4s-carries.md", f"{N}/history.md", f"{N}/u4f-run.sh"
BRIEF_SHA = "50530a825b5877d73aca479b55f7449bb1cf2e776a5b87e196da86efffa74ccb"
RUNNER_SHA = "70eb3da5b8ce9a8e2eda582536eeab93f96fb7fbe98e2a74a28e0492ccf6ddae"
FIXED = {f"{N}/u4-build-plan.md": "43ff74f2a3ce903f1061c67f53b416ae2498f3890f2116426feeab4373656385",
         f"{N}/plan.md": "3eecd28196a529969e9cce1e6eee9dd35100b796414242b835903557e28bc09b"}
OLD = {CARR: ("57600c9c79bdccfe521dc1181ee9ab9d86087e11c38b788b9b223072c9c01dce", 682),
       HIST: ("3bfe1aa2d5a3f781267956e011079c42b6cc12e61a058af1af9e395076246854", 2232)}
CLOSED = {"advisor-u4df.md": "842ec092", "advisor-u4j-r2.md": "7efa9410", "advisor-u4d2-m9.md": "f31f7876",
          "advisor-u4d2-r2.md": "e776f0ba", "advisor-te-m3.md": "5ecc3e23", "advisor-u4r-c62h-full.md": "18dd3d20",
          "advisor-brawl1-sets-p3-full.md": "1bc5889c", "advisor-u4r-build2-full.md": "f4f5ac92",
          "advisor-u4r-r1-full.md": "7b552e8c"}
PROB, OUT = [], []


def rb(p):
    with open(p, "rb") as fh:
        return fh.read()


def sha(b):
    return hashlib.sha256(b).hexdigest()


def norm(t):
    return " ".join(t.split())


def ws(t):
    return r"\s+".join(re.escape(w) for w in t.split())


def check(ok, msg):
    if not ok:
        PROB.append(msg)
    return ok


def run(*a):
    return subprocess.run(list(a), capture_output=True, text=True)


# ---------------------------------------------------------------- the brief, the base, the runner, the stamps
RAW = rb(BRIEF)
NRAW = RAW.count(b"\n")
if sha(RAW) != BRIEF_SHA or NRAW != 1835:
    sys.exit(f"the brief is not rev 3.3 (sha256 {sha(RAW)}, {NRAW} lines): stop")
TEXT = RAW.decode()
L = TEXT.split("\n")
I3 = next(i for i, x in enumerate(L) if x.startswith("3. **The placeholders, by script"))
I4 = next(i for i, x in enumerate(L) if x.startswith("4. **The advisor's rulings on U4F-Q1"))
ITEM3 = norm(" ".join(L[I3:I4]))
OUT.append(f"script {__file__}: sha256 {sha(rb(__file__))}, {rb(__file__).count(b'x'[:0] + bytes([10]))} lines")
OUT.append(f"brief {BRIEF}: sha256 {BRIEF_SHA}, 1,835 lines (rev 3.3); item 3 at :{I3 + 1}-{I4}")

BASE = run("git", "rev-parse", "HEAD").stdout.strip()
check(re.fullmatch(r"[0-9a-f]{40}", BASE), f"git rev-parse HEAD gave {BASE!r}")
SP = "spikes/netcode-proof"
if run("git", "diff", "--quiet", "0ad9661", BASE, "--", SP).returncode == 0:
    OUT.append(f"<base> {BASE}: `git diff --quiet 0ad9661 {BASE} -- {SP}` exit 0, so D1's counts hold")
else:
    rc = run("git", "diff", "--quiet", "0ad9661", BASE, "--", SP, f":(exclude){SP}/results").returncode
    names = run("git", "diff", "--name-only", "0ad9661", BASE, "--", f"{SP}/results").stdout.split()
    if check(rc == 0 and all(x.endswith("raw-logs.sha256") for x in names),
             f"<base> {BASE}: spikes/netcode-proof differs from 0ad9661's beyond new raw-logs.sha256 lists"):
        OUT.append(f"<base> {BASE}: spikes/netcode-proof equals 0ad9661's with results/ excluded (exit 0); under results/ "
                   f"only {len(names)} new raw-logs.sha256 file(s) ({', '.join(names)}): by D1 every count holds but refs "
                   "and redact's items (+1 each per list), which D1's light run reads")

RSHA = sha(rb(RUNNER))
rn = run("bash", "-n", RUNNER).returncode
check(RSHA == RUNNER_SHA and rn == 0, f"{RUNNER}: sha256 {RSHA}, bash -n exit {rn}")
OUT.append(f"<<PV1>> {RUNNER}: sha256 {RSHA} (expected {RUNNER_SHA[:8]}…), `bash -n` exit {rn}")
for p, h in FIXED.items():
    check(sha(rb(p)) == h, f"{p} changed since 0.2's stamp")
    OUT.append(f"stamp {p}: {h[:8]}… unchanged (0.2's)")
for f, h in CLOSED.items():
    got = sha(rb(f"{N}/{f}"))
    check(got.startswith(h) and f"{f} {h}…" in norm(TEXT), f"closed record {f}: sha256 {got}, 0.2's {h}…")
OUT.append(f"closed records: {len(CLOSED)} sha256 equal to 0.2's stamps ({', '.join(f'{f} {h}…' for f, h in CLOSED.items())})")

TXT = {f: rb(f"{N}/{f}").decode() for f in ["u4s-carries.md", "history.md", "u4r-report.md", *CLOSED]}
LINES = {f: t.split("\n") for f, t in TXT.items()}
CL, HL = LINES["u4s-carries.md"], LINES["history.md"]
i90 = next(i for i, x in enumerate(CL) if x.startswith("90. **SA-11, the view-time wait under option B"))
j90 = next(j for j in range(i90 + 1, len(CL)) if re.match(r"\d+\. |## |$", CL[j]))
old_c = "\n".join(CL[:i90] + CL[j90:]).encode()
check(i90 == 597 and sha(old_c) == OLD[CARR][0], "carries: not 0.2's version plus entry 90 after :597 alone")
check(sha(("\n".join(HL[:2232]) + "\n").encode()) == OLD[HIST][0], "history: its lines :1-2232 are not 0.2's version")
h1 = next(i for i, x in enumerate(HL) if x.startswith("## Decision: Mustafa, 2026-11-17 20:20")) + 1
h2 = next(i for i, x in enumerate(HL) if x.startswith("## Archive: the resume card before the TEST-FIRST cut")) + 1
check((h1, h2) == (2234, 2284), f"history: TEST-FIRST at :{h1}, the archive at :{h2}")
NEWC, NEWH = sha(rb(CARR)), sha(rb(HIST))
NC, NH = TXT["u4s-carries.md"].count("\n"), TXT["history.md"].count("\n")
for word, top in (("carries", 597), ("history", 2232)):          # every cite of the brief into them, by its numbers
    nums = [int(x) for m in re.finditer(word + r"((?:[,;]? (?:and )?:\d+(?:-\d+)?)+)", TEXT)
            for x in re.findall(r"\d+", m.group(1))]
    check(max(nums) <= top, f"the brief cites {word} past :{top} (max :{max(nums)})")
    OUT.append(f"living {word}: {len(nums)} cited line numbers, the highest :{max(nums)}, all at or before :{top}")
OUT.append(f"living carries: 0.2's version ({OLD[CARR][0][:8]}…, 682) plus entry 90 at :{i90 + 1}-{j90} alone; now "
           f"{NEWC[:8]}…, {NC} lines")
OUT.append(f"living history: :1-2232 byte for byte 0.2's ({OLD[HIST][0][:8]}…); appended :2233-{NH} (TEST-FIRST "
           f":{h1}-{h2 - 2}, the archive :{h2}-{NH}); now {NEWH[:8]}…, {NH:,} lines")
ur = sha(TXT["u4r-report.md"].encode())
OUT.append(f"living u4r-report.md: {ur[:8]}…, {TXT['u4r-report.md'].count(chr(10))} lines (0.2: c5dee3fa…, 1,764; by text)")


# ---------------------------------------------------------------- item 3's copies
def got(tag, f, text, a, b, rng, first=None, last=None):
    c = norm(text)
    check((a, b) == rng, f"{tag}: {f} :{a}-{b}, not item 3's :{rng[0]}-{rng[1]}")
    for w in (first, last):
        check(w is None or norm(w) in ITEM3, f"{tag}: {w!r} is not in item 3's own text")
    check(first is None or c.startswith(norm(first)), f"{tag}: the copy does not begin {first!r}")
    check(last is None or c.endswith(norm(last)), f"{tag}: the copy does not end {last!r}")
    return dict(tag=tag, f=f, copy=c, a=a, b=b)


def span(tag, f, a, z, rng, skip=0):
    t = TXT[f]
    if not check(t.count(a) == 1, f"{tag}: start {a!r} found {t.count(a)} times in {f}"):
        return None
    i = t.find(a)
    m = re.compile(ws(z)).search(t, i)
    if not check(m, f"{tag}: end {z!r} not found after the start in {f}"):
        return None
    return got(tag, f, t[i + skip:m.end()], t.count("\n", 0, i) + 1, t.count("\n", 0, m.end()) + 1, rng,
               a[skip:], z)


def lines(tag, f, a, rng, stop=None, k=0, exact=False, before=None, num=None):
    t, LL = TXT[f], LINES[f]
    if not check(t.count(a) == 1, f"{tag}: start {a!r} found {t.count(a)} times in {f}"):
        return None
    s = next(i for i, x in enumerate(LL) if x.startswith(a))
    e = next(j for j in range(s + 1, len(LL)) if re.match(stop, LL[j])) - 1 if stop else s + k
    check(not exact or LL[s] == a, f"{tag}: the line is not {a!r}")
    check(before is None or LL[s - 1] == before, f"{tag}: the line before is not {before!r}")
    nums = [m.group(1) for x in LL[s:e + 1] for m in [re.match(r"(\d+)\. ", x)] if m]
    check(num is None or nums == num, f"{tag}: entry numbers at line start {nums}, not {num}")
    return got(tag, f, "\n".join(LL[s:e + 1]), s + 1, e + 1, rng, a)


C, H, ENT = "u4s-carries.md", "history.md", r"\d+\. |## "
SPECS = [lines("P1", C, "## U4f (clock and commands)", (507, 540), stop=r"## U4g \(prediction, interpolation and "
               "events\\)", num=["46", "80", "86"]),
         span("P2", C, "Each entry: the finding (review, id), then", "never the rule.", (9, 10))]
for tag, key, rng in (("P3", "57. **U4E-Q6, the room's half", (223, 228)), ("P4", "74. **NC-M6, the endpoint's lead",
                      (345, 356)), ("P5", "21. **netcode F2 (the room's half):**", (148, 149)),
                      ("P6", "27. **this pass (CMD_CATCHUP):**", (166, 168)),
                      ("P7", "18. **this pass (the tick grep):**", (129, 132)),
                      ("P8", "39. **rust fix review nit (u4s-fix-review-rust.md:114-115", (169, 179)),
                      ("P9", "50. **the event age", (180, 183)),
                      ("P10", "61. **U4j's hash rules, the room and the client", (229, 247)),
                      ("P11", "58. **U4E-Q6's two conditions", (269, 273)),
                      ("P12", "76. **A6 and SEC-U4E-5, ending a session", (366, 373)),
                      ("P13", "77. **U4r's readings of U4e's room and client", (374, 383))):
    SPECS.append(lines(tag, C, key, rng, stop=ENT, num=[key.split(".")[0]]))
SPECS += [lines("P14", H, "  - Builders: pass or fail comes from the exit code", (292, 293), k=1,
                before="  USAGE-SAVINGS: (a), all three, with these limits."),
          span("P15", H, "A new bar, or a change to one, comes to me under Decisions needed", "on the ruling alone.",
               (2125, 2125)),
          lines("P16", H, "  From now on: every change to marks.json or to the judge", (2127, 2127)),
          span("P17", H, "write commands in the simple forms that need no approval", "before you send it.", (2187, 2187)),
          span("P18", H, "  2. SIZE-CHECK from now on.", "as the rule says.", (2048, 2051)),
          lines("P19", H, "  C62H-KEEP: (a). The check stays as built.", (2141, 2141), exact=True),
          span("P20", H, "  C62H-BOUND: (a), the exact limit.", "stay in until it is built.", (2143, 2146)),
          lines("P21", H, "  - The stop rules of your post stand. The check is never reworded to fit.", (2175, 2175),
                exact=True),
          lines("P22", H, "      - (a) Yes. I may pack one or two closed", (2212, 2212)),
          lines("P23", H, "  - **C62HB-Q4:** should the counter-wrap test (PM-0001-58)", (2216, 2216)),
          span("P24", "advisor-u4df.md", 'word for word: "Write files with the', "whole with Write.", (462, 464),
               skip=len('word for word: "')),
          span("P25", "advisor-u4df.md", "a failure in <crate>", "and go on.", (465, 468)),
          span("P27", "advisor-u4d2-m9.md", "Between waves, with no lane running", "never `--target` without `-p`.",
               (252, 256)),
          span("P28", "advisor-u4d2-m9.md", "(1) never pass `--target`", "no session removes it by hand.", (259, 264)),
          span("P29", "advisor-u4d2-r2.md", "never run cargo", "may use it as one.", (304, 308)),
          span("P30", "advisor-te-m3.md", "After it ends, FX-10's saved output", "says so.", (127, 128)),
          span("P31", "advisor-u4r-c62h-full.md", "Until the fix lands, the rule is:", "as both run scripts already do.",
               (20, 20)),
          span("P33", "advisor-u4r-build2-full.md", "Adds a card follow-up", "No push for it.", (28, 28)),
          span("P34", "advisor-u4r-r1-full.md", "Contract half:", "a usage error (exit 2).", (122, 122)),
          lines("P35", "u4r-report.md", "- **TE-B1-R2-1 with RR2-B1-1**", (1138, 1142), stop=r"- \*\*TE-B1-R2-2\*\*"),
          lines("P36", "u4r-report.md", "  - RR-B1-7 (1), the room_tick loop rewrite", (1163, 1163))]
J = TXT["advisor-u4j-r2.md"]                                     # P26: the words after "From now on: " to the line's end
if check(J.count("From now on: ") == 1 and norm("From now on: ") in ITEM3, "P26: its key not found once"):
    i = J.find("From now on: ") + len("From now on: ")
    SPECS.append(got("P26", "advisor-u4j-r2.md", J[i:J.find("\n", i)], J.count("\n", 0, i) + 1,
                     J.count("\n", 0, i) + 1, (298, 298)))
q = '"With everyone in view, every client queues'                # P32: b1-p3 :158, the quoted line whole
B1 = LINES["advisor-brawl1-sets-p3-full.md"]
if check(TXT["advisor-brawl1-sets-p3-full.md"].count(q) == 1 and B1[157].startswith(q) and B1[157].endswith('"'),
         "P32: b1-p3 :158 is not the quoted line"):
    SPECS.append(got("P32", "advisor-brawl1-sets-p3-full.md", B1[157], 158, 158, (158, 158)))
COPY = {d["tag"]: d for d in SPECS if d}
if "P25" in COPY:                                                # U4F-Q5 ruled (a): the clause kept, its two substitutions
    c = COPY["P25"]["copy"]
    check(c.count("a failure in <crate>,") == 1 and c.count("<crate>'s files") == 1, "P25: its two <crate> not found")
    c = c.replace("<crate>", "npt-room, npt-client, npt-bots or npt-lab (and npt-proto, npt-server and npt-sim, which "
                  "U4F-Q1 opens)", 1).replace("<crate>'s files", "their files")
    check("<crate>" not in c, "P25: a <crate> left")
    COPY["P25"]["copy"] = c
check(sorted(COPY, key=lambda t: int(t[1:])) == [f"P{k}" for k in range(1, 37)], f"copies made: {sorted(COPY)}")

# ---------------------------------------------------------------- the fills (the advisor's fold check, section 4)
# each copy once, under its name at the end of item 3's list; every other `<<Pn>>` a pointer; <base> and PV1 values;
# the three name-only mentions kept, each named by its text
TAGS = sorted(COPY, key=lambda t: int(t[1:]))
QUOTED = {t: (d["copy"] if d["copy"][0] == d["copy"][-1] == '"' else f'"{d["copy"]}"') for t, d in COPY.items()}
POINT = "{} (copied in section 12 item 3)"
REP = {f"`<<{t}>>`": POINT.format(t) for t in COPY}
REP.update({"<<PV1>>": RSHA, "<base>": BASE})
PAT = re.compile("|".join(re.escape(k) for k in sorted(REP, key=len, reverse=True)))
EXEMPT = ["| C2 | Placeholders `<<P1>>` to `<<P36>>`", "`<base>` stays a placeholder", "Values: `<base>` (the spike",
          "branch's head at launch) and `<<PV1>>` (u4f-run.sh's sha256)."]
EX = []
for key in EXEMPT:
    hits = [i for i, x in enumerate(L) if key in x]
    check(len(hits) == 1, f"exempt text {key!r} on {len(hits)} lines")
    EX += hits
WHERE = {}
NL = list(L)
for i, x in enumerate(L):
    if I3 <= i < I4 or i in EX:
        continue
    for m in PAT.finditer(x):
        WHERE.setdefault(m.group(0), []).append(i + 1)
    NL[i] = PAT.sub(lambda m: REP[m.group(0)], x)
for t in TAGS:
    d, w, at = COPY[t], COPY[t]["copy"].split(), WHERE.get(f"`<<{t}>>`", [])
    OUT.append(f"{t}: {d['f']} :{d['a']}" + (f"-{d['b']}" if d["b"] != d["a"] else "") + f"; first \"{' '.join(w[:6])}\" … "
               f"last \"{' '.join(w[-6:])}\"; {len(d['copy']):,} characters, once in item 3's list; {len(at)} pointer(s), "
               f"brief :{', :'.join(map(str, at))}")
for k in ("<<PV1>>", "<base>"):
    OUT.append(f"{k} = {REP[k]}: {len(WHERE.get(k, []))} place(s), brief :{', :'.join(map(str, WHERE.get(k, [])))}")
OUT.append("kept as they are (name-only mentions): " + "; ".join(f":{i + 1} {key!r}" for i, key in zip(EX, EXEMPT)))
FORCE = set()


def phrase(old, new, tag):
    """old (found once, white space read as one space) -> new in NL, its lines kept 1:1, the unit rewrapped."""
    global NL
    t = "\n".join(NL)
    ms = list(re.finditer(ws(old), t))
    if not check(len(ms) == 1, f"{tag}: the old phrase found {len(ms)} times"):
        return
    m = ms[0]
    a, b = t.count("\n", 0, m.start()), t.count("\n", 0, m.end())
    NL = (t[:m.start()] + new + "\n" * (b - a) + t[m.end():]).split("\n")
    FORCE.update(range(a, b + 1))
    OUT.append(f"{tag} :{a + 1}-{b + 1}: {old!r} -> {new!r}")


phrase("the main session copies each by script at launch (section 12 item 3),", "the main session copies each by script "
       "at launch, once, under its name in section 12 item 3's list, every other place pointing there (TEST-FIRST rule "
       "3),", "0.5's sentence (condition 3)")
phrase("the placeholder gone from its every place in the brief,", "the placeholder gone from its every place in the brief "
       "(its copy once in this list, a pointer elsewhere; C2's, C28's and 0.5's name-only mentions kept),",
       "item 3's sentence (condition 4)")

# 0.2's living rows, re-stamped, and the sentence after the table
ROWS = {"| `carries` |": (OLD[CARR][0], "| 682 |", NEWC, f"| {NC} |"),
        "| `history` |": (OLD[HIST][0], "| 2,232 |", NEWH, f"| {NH:,} |")}
for pre, (oh, ol, nh, nn) in ROWS.items():
    i = next(i for i, x in enumerate(L) if x.startswith(pre))
    check(TEXT.count(oh) == 1 and L[i].count(oh) == 1 and L[i].count(ol) == 1, f"0.2's row {pre} not as stamped")
    NL[i] = L[i].replace(oh, nh).replace(ol, nn)
    OUT.append(f"0.2's row {pre} :{i + 1}: {oh[:8]}… {ol} -> {nh[:8]}… {nn}")
SENT = (f"At launch (u4f_fill.py, at {BASE}), the carries and history rows above were re-stamped: the carries gained "
        f"only entry 90 (:{i90 + 1}-{j90}, U4h's section, after :597) and history only lines at its end (:2233-{NH:,}: "
        f"Mustafa's TEST-FIRST words at :{h1}-{h2 - 2} and the old card's archive at :{h2}-{NH:,}), so every cite of "
        "this brief into them still points at the same text (carries :1-597 and history :1-2,232 asserted byte for byte "
        "as 0.2's versions); plan and plan.md are unchanged.")
OUT.append("0.2's sentence: " + SENT)

# ---------------------------------------------------------------- the units: rewrap only where a line passes 120
MARK = re.compile(r"^( *)(- |\d+\. )")
STRUCT = re.compile(r"^(- |\d+\. |#|\||\*\*)")
SUB = re.compile(r"^\((?:[ivx]+|[a-h]|\d+)\) ")
RISKY = re.compile(r"^(\d+[.)](\s|$)|[-*+](\s|$)|#|>|```|~~~|\||=+\s*$|\*\*|\((?:[ivx]+|[a-h]|\d+)\)(\s|$))")
GLUE = "\x01"


def ind(x):
    return len(x) - len(x.lstrip(" "))


def refill(t, first, cont):
    for _ in range(200):
        out = textwrap.TextWrapper(width=120, initial_indent=first, subsequent_indent=cont, break_long_words=False,
                                   break_on_hyphens=False).wrap(t)
        bad = [k for k in range(1, len(out)) if RISKY.match(out[k][len(cont):])]
        if not bad:
            return [x.replace(GLUE, " ") for x in out]
        k = bad[0]
        t = t.replace(f"{out[k - 1].split(' ')[-1]} {out[k][len(cont):].split(' ')[0]}",
                      f"{out[k - 1].split(' ')[-1]}{GLUE}{out[k][len(cont):].split(' ')[0]}", 1)
    sys.exit(f"refill: no safe wrap for {t[:60]!r}: stop")


def build(S, notes):
    """S: L with the fills made, line for line; the result's lines, each unit rewrapped only where a line passes 120."""
    out, i = [], 0
    while i < len(L):
        x = L[i]
        if not x.strip() or x.lstrip().startswith(("|", "#")):
            if S[i] != x and x.startswith("#") and len(S[i]) > 120:
                notes.append(f"heading :{i + 1} in its line, now {len(S[i]):,} characters")
            out.append(S[i])
            i += 1
            continue
        m = MARK.match(x)
        cont = len(m.group(0)) if m else ind(x)
        e = i
        while e + 1 < len(L) and L[e + 1].strip() and ind(L[e + 1]) == cont and not STRUCT.match(L[e + 1].lstrip()) \
                and not SUB.match(L[e + 1].lstrip(" ")):
            e += 1
        ch = [k for k in range(i, e + 1) if S[k] != L[k]]
        if ch and (any(len(S[k]) > 120 for k in range(ch[0], e + 1)) or FORCE & set(range(i, e + 1))):
            f = ch[0]
            pre = (m.group(0) if m else " " * cont) if f == i else " " * cont
            new = refill(" ".join([S[f][len(pre):].strip()] + [y.strip() for y in S[f + 1:e + 1] if y.strip()]), pre,
                         " " * cont)
            notes.append(f"unit :{i + 1}-{e + 1} rewrapped from :{f + 1} ({e - f + 1} -> {len(new)} lines)")
            out += S[i:f] + new
        else:
            out += S[i:e + 1]
        i = e + 1
    k = next(k for k, x in enumerate(out) if x.startswith("| `history` |"))
    out[k + 1:k + 1] = [""] + refill(SENT, "", "")
    return out


def item4(D):
    return next(k for k, x in enumerate(D) if x.startswith("4. **The advisor's rulings on U4F-Q1"))


NOTES, BLOCK = [], []
for t in TAGS:                                   # the copies, at the end of item 3's list, each under its name
    b = refill(f"{t}'s copy: {QUOTED[t]}", "   - ", "     ")
    check(b[0].startswith(f"   - {t}'s copy: {QUOTED[t].split()[0]}"), f"{t}: its opening quotation mark left its line")
    BLOCK += b
ROWED = [i for i, x in enumerate(L) if x.startswith("|") and NL[i] != x and not x.startswith(tuple(ROWS))]
R, RD = build(NL, NOTES), build([L[i] if i in ROWED else y for i, y in enumerate(NL)], [])
R[item4(R):item4(R)] = BLOCK
RD[item4(RD):item4(RD)] = [f"   - {t}'s copy: «{t}»" for t in TAGS]
k = next(k for k, x in enumerate(NL) if x.startswith("| `history` |"))
NL[k + 1:k + 1] = ["", SENT]
NL[item4(NL):item4(NL)] = [f"   - {t}'s copy: {QUOTED[t]}" for t in TAGS]
OUT.append(f"item 3's list gains {len(TAGS)} copies, {len(BLOCK)} lines, before item 4")

# ---------------------------------------------------------------- the checks on the result
FINAL = "\n".join(R)
check(norm(FINAL) == norm("\n".join(NL)), "words check: the result is not rev 3.3 with only these fills")
J3 = next(k for k, x in enumerate(R) if x.startswith("3. **The placeholders, by script"))
J4 = item4(R)
IN3 = norm(" ".join(R[J3:J4]))
check(IN3 == norm(ITEM3.replace(norm("the placeholder gone from its every place in the brief,"), "the placeholder gone "
      "from its every place in the brief (its copy once in this list, a pointer elsewhere; C2's, C28's and 0.5's "
      "name-only mentions kept),") + " " + " ".join(BLOCK)), "item 3's list: more changed than condition 4 and the copies")
check(all(re.search(rf"\bP{n}\b", ITEM3) for n in range(1, 37)), "item 3's record lost a name")   # P4-P12: "P3 to P13"
NF = norm(FINAL)
once = [t for t in TAGS if NF.count(norm(QUOTED[t])) == 1 and IN3.count(norm(f"{t}'s copy: {QUOTED[t]}")) == 1]
check(once == TAGS, f"copies not once, in item 3's list under their names: {sorted(set(TAGS) - set(once))}")
ptr = re.findall(r"\b(P\d+) \(copied in section 12 item 3\)", NF)
check(ptr and all(p in once for p in ptr), "a pointer names a Pn whose copy is not in item 3's list")
check(len(ptr) == sum(len(v) for k, v in WHERE.items() if k.startswith("`")), "pointers: not one per placeholder place")
rest = norm("\n".join(R[:J3] + R[J4:]))
for key in EXEMPT:                                                # the exempt texts, each once, then nothing else left
    check(rest.count(norm(key)) == 1, f"exempt text {key!r} not once in the result")
    rest = rest.replace(norm(key), " ", 1)
left = re.findall(r"<<P\d+>>|<<PV1>>|<base>", rest)
check(not left, f"<<P, <<PV1>> or <base> left outside item 3 and the exempt lines: {left}")
forms = [k + 1 for k, x in enumerate(R) if not J3 <= k < J4 and re.search(r"<<P(n|)>>", x)]
OUT.append(f"checks: each of the {len(once)} copies once in the brief, in item 3's list under its name; {len(ptr)} "
           f"pointers, each to a copy there; no <<P, <<PV1>> or <base> left outside item 3 but the {len(EX)} exempt lines "
           f"(the form's names `<<P>>` and `<<Pn>>`, no placeholder, stand at the result's :{', :'.join(map(str, forms))})")
old = set(L)
long_ = [k + 1 for k, x in enumerate(R) if len(x) > 120 and x not in old and not x.lstrip().startswith(("|", "#"))]
check(not long_, f"changed lines over 120 characters: {long_}")
# Mustafa's words only where a placeholder copies them: r31_check.py's 7-word method (its ranges, plus TEST-FIRST's
# :2238-2279), on the result's words with every copy masked and section 12 item 3 left out
RANGES = [(292, 293), (2040, 2058), (2091, 2111), (2116, 2129), (2134, 2146), (2151, 2161), (2166, 2175), (2180, 2187),
          (2190, 2190), (2200, 2200), (2205, 2205), (2230, 2230), (2238, 2279)]


def toks(s):
    return re.findall(r"[a-z0-9]+", s.lower())


grams = {tuple(w[j:j + 7]): a for a, b in RANGES for w in [[t for n in range(a, b + 1) for t in toks(HL[n - 1])]]
         for j in range(len(w) - 6)}
check(toks(FINAL) == toks("\n".join(NL)), "the result's words differ from the fills'")
stream = []
for n, y in enumerate(R, 1):                                      # item 3 (the result's :J3+1 to :J4) is a barrier
    stream += [("|", n)] if J3 < n <= J4 else [(t, n) for t in toks(y)]
runs = sorted({(stream[j][1], " ".join(t for t, _ in stream[j:j + 7])) for j in range(len(stream) - 6)
               if "|" not in [t for t, _ in stream[j:j + 7]] and tuple(t for t, _ in stream[j:j + 7]) in grams})
for n, g in runs:
    check(False, f"7-word run of Mustafa's words outside item 3's copies, the result's line {n}: {g}")
OUT.append(f"7-word check (r31_check.py's method, on the result, item 3 left out): {len(runs)} run(s)")
OUT += NOTES
OUT.append(f"brief: {len(L) - 1:,} -> {len(R) - 1:,} lines; sha256 after {sha(FINAL.encode())}")
FULL = "--full" in sys.argv
OUT.append("the unified diff of the result" + ("" if FULL else f": each copy in item 3's list shown as «Pn» (its "
           f"words are the lines above; --full prints the whole diff), and the {len(ROWED)} table rows filled in their "
           f"line alone not shown (each pointer or value replaced in place, no other byte: brief "
           f":{', :'.join(str(i + 1) for i in ROWED)})"))
DIFF = list(difflib.unified_diff([x + "\n" for x in L[:-1]], [x + "\n" for x in (R if FULL else RD)[:-1]], BRIEF,
                                 BRIEF, n=0))
SHOW = FULL or "--diff" in sys.argv
if not SHOW:
    OUT[-1] = (f"the unified diff (not printed; --diff prints it, each copy as «Pn» and the {len(ROWED)} rows filled in "
               f"place left out; --full prints it whole): {sum(x.startswith('@@') for x in DIFF)} hunks")
print("\n".join(OUT))
sys.stdout.writelines(DIFF if SHOW else [])
if PROB:
    print(f"\n{len(PROB)} problem(s); nothing written:\n" + "\n".join(f"- {p}" for p in PROB))
    sys.exit(1)
if APPLY:
    with open(BRIEF, "w", encoding="utf-8", newline="\n") as fh:
        fh.write(FINAL)
    back = rb(BRIEF)
    if back != FINAL.encode():
        sys.exit(f"read back differs (sha256 {sha(back)}): stop")
    print(f"applied; read back equal, sha256 {sha(back)}")
else:
    print("dry run: nothing written")
