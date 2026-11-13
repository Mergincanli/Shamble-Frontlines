"""U4r's launch step on the builder brief rev 3.2 (u4r-brief.md section 12 items 1, 2, 3 and 9; the advisor's check,
advisor-u4r-brief.md, condition L4 (:428-429) and L2's last sentence (:423-424); the fold ruling, advisor-u4r-fold.md,
FQ4 (:101-112), N7a's note (:89-90) and FQ6 (:151-159)). It changes the brief only:
- item 1: P6, `<base>` = the spike branch's head when this runs (`git rev-parse HEAD`), with main's 4d21c1a an ancestor
  of it (MAIN-48: main is merged before the fill), spikes/netcode-proof equal to 4e91830's and HEAD's brief equal to rev
  3.2, so the counts are D1's, read from L3's counted run (.gate/l3-manifests/counted.log); filled where P6 stands
  outside item 3's list (section 1 item 1, section 4 rule 5, section 12 item 1), in u4d2_restamp.py's form;
- item 2: P7, u4r-run.sh, its sha256 and `bash -n` asserted; filled where it stands (section 1 item 4, section 12
  item 2);
- item 3: P1 to P5 and P8 to P16 by item 3's own rules, each rule read from item 3's own text (the anchors and the
  line ranges; `\\"` read as a quotation mark), every closed file's sha256 asserted equal to section 0's stamp first;
  each placeholder gone from its home, item 3's list keeping the names as the record;
- item 9 (a): section 0's plan and carries stamps re-set (plan.md and history asserted unchanged), with one sentence
  saying so; (b) every cite of the brief into the plan or the carries whose line L2 (8bcbf36) moved, re-pointed: a cite
  wholly inside an unchanged block (difflib over the two versions) shifts by that block's offset, a cite touching a
  rewritten block is mapped by its text (MANUAL: the range's first and last line asserted in both versions; each key
  used once); section 10 is left whole (FQ4) and its moved cites are listed, and the change lists' moved cites at the
  top are re-pointed too (FQ4's "every other cite"; the main session's decision 1); (c) section 9's re-walk for L3
  (4e91830): R11's verdict as the head shows it, its cites read through `git show`, and section 9's tally line with
  it; every other cite of the four files outside section 10 asserted at `<base>`;
- the launch's record: one sentence at the end of the title block's rev 3.2 paragraph (the main session's words).
Checks: the sha256 of the brief and every source first; each anchor and each old phrase found once (white space read as
one space); a nearest-file classifier over the brief that every moved plan or carries cite is in the inventory; the
words check (rev 3.2's words with only these changes made equal the result's); no `<<P` and a digit outside item 3's
list; no changed line over 120 characters (table rows exempt); the top equal to rev 3.2's but for the launch's record
and the change lists' two re-pointed cites; section 10 whole. A table row is edited in its line; any other unit is
rewrapped within 120 from its first changed line to its end only where a line would pass 120.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4r_fill.py [--apply]
Without --apply it writes nothing: it prints one line per change (brief line, what, source lines), the main session's
words, then the unified diff."""
import difflib
import hashlib
import re
import subprocess
import sys
import textwrap

N = ".checkpoints/netcode-proof-test"
APPLY = "--apply" in sys.argv
BRIEF, PLAN, CARRIES = f"{N}/u4r-brief.md", f"{N}/u4-build-plan.md", f"{N}/u4s-carries.md"
PLANMD, HIST, RUNNER = f"{N}/plan.md", f"{N}/history.md", f"{N}/u4r-run.sh"
CHECK, FOLD = f"{N}/advisor-u4r-brief.md", f"{N}/advisor-u4r-fold.md"
COUNTED = ".gate/l3-manifests/counted.log"
BRANCH = "claude/netcode-proof-test-plan-hcky5g"
SP = "spikes/netcode-proof"
L2C, L3C, TEM3C, U4D2C = "8bcbf36", "4e91830", "c49773f", "cc8595d"
MAIN48 = "4d21c1a7a4d39613bffce5d5f9bbc92f2f88a300"     # main's head merged into the branch at MAIN-48 (7685b2b, PR #48)
HASHES = {
    BRIEF: "362e771eb9e7166ef2751e3b57cf285c198e345e2cecf1e288b8e40a9ac4b241",   # rev 3.2 (a6b3410)
    PLAN: "1e631c55f4c4777edf0357b349fc5c8566d7410daffded926a06f82ff02f47d5",    # as L2 left it (8bcbf36)
    CARRIES: "e8675a02a98e98a5551309388839ffdbb9e341fbc06dd280788d09b604d12ac1",  # as L2 left it (8bcbf36)
    PLANMD: "3ca4e12a39c9f48396b00d9b738cf7b9a3b4aba9269cfad4519eabfc174f83c1",   # section 0's stamp
    RUNNER: "349b51adbed43a889ba9b88eeb89d573aadc3b4f035e186cc8b56de18f4e0831",   # P7
    CHECK: "35f281bfc11c453fb9a5054ecc6e433d4bf9c6509e668f0a28128959feb6ca37",
    FOLD: "d441704072d5a4b63172757df50da9aec860084faa98226f836d5bec738b445e",
}
# section 0's closed records, by its names: each asserted equal to its stamp there
CLOSED = {
    "advisor": ("advisor-u4.md", "7b3e76ca09f3781ba799cc9c0d44885cc7c2f1d6eca2aaa116184246cfab78ff"),
    "wave1": ("advisor-u4-wave1.md", "a45575aeb3b4db8858457e19ac05f6ca2067ca3b0d854f2f2a071c12bedbe4c2"),
    "close": ("advisor-u4d-close.md", "cbc7e1c9210b6e2c664eeea624fc2b5d90fc1c985e57e330d70c70cca8811b33"),
    "j-ruling": ("advisor-u4j.md", "a98dfdb4a4a307de0aca6aaf6f67f8977fe0f7a2535549ccebbc9b927e8d5df2"),
    "j-r2": ("advisor-u4j-r2.md", "7efa9410323f09236c2f8f3ca09b91045b2e9217d64bd1864c98786e85deead1"),
    "df-ruling": ("advisor-u4df.md", "842ec092a4b473d6d91be2147c7b5dc39cd8fb435dac2ff0d58c88b3ce791151"),
    "e-ruling": ("advisor-u4e-brief.md", "3485a6c65b85fd683c454f79499a5657cfde8c570fcbb5f9531c62d6a93d692b"),
    "e-r1": ("advisor-u4e-r1.md", "7c129309703969b83d0d1a4c93cba576564b7627b972bdee6a9d8e380002bf44"),
    "e-r2": ("advisor-u4e-r2.md", "14b0b1802ef8c14f8bf1cb6abf105ada0c427f2c96d3cff92d709ca4245bda5b"),
    "d2-ruling": ("advisor-u4d2-brief.md", "155f3376b47c05011bb18b3c1881242d528a7bd743d28cb2a0b58cc6921379b3"),
    "d2-r1": ("advisor-u4d2-r1.md", "9978b43b763958f9066b708a05319dfd5b03fa61dbc1b920b0375465298d9b6f"),
    "d2-r2": ("advisor-u4d2-r2.md", "e776f0ba7c4894ef72f05fb12ebfa1c955a61bd6018b200404f010f5b310f8d4"),
    "m9": ("advisor-u4d2-m9.md", "f31f7876c009129531067c8c29bcb793ebff48f52cc4b981215691bb468eba73"),
    "te-m3": ("advisor-te-m3.md", "5ecc3e233ea22b64a8573d3c62a54f2b1a0206a726f72cfa81f10a7cf7055c48"),
    "te-m3-check": ("advisor-te-m3-check.md", "1ffd02945dc605e63d22d8733633dda388cd3db84db9430bc36a5328b6760445"),
    "u4r-check": ("advisor-u4r-brief.md", "35f281bfc11c453fb9a5054ecc6e433d4bf9c6509e668f0a28128959feb6ca37"),
}
OLDSTAMP = {PLAN: ("1,092", "01ede63d"), CARRIES: ("644", "8b7f165e")}   # section 0's, before L2
REV32 = "a6b3410"                                                        # rev 3.2's commit
HIST_STAMP = "88bc195fe94b3ba18733ccc7acf4c853e647fa1e8d283d82f671c85810705975"  # section 0's, history at rev 3.2
P1_ENTRIES = ["40", "49", "58", "59", "60", "62", "63", "69", "73", "74", "75", "76", "77", "82", "83", "84", "85"]


def stop(msg):
    sys.exit(f"{msg}: stop")


def sha_b(b):
    return hashlib.sha256(b).hexdigest()


def read_b(p):
    with open(p, "rb") as fh:
        return fh.read()


def git_b(*a):
    r = subprocess.run(["git", *a], capture_output=True)
    if r.returncode:
        stop(f"git {' '.join(a)}: exit {r.returncode}: {r.stderr.decode().strip()}")
    return r.stdout


def git(*a):
    return git_b(*a).decode()


def git_rc(*a):
    return subprocess.run(["git", *a], capture_output=True).returncode


def at(D, k):
    """Line k (1-based) of the list D."""
    return D[k - 1]


def nlines(b):
    """A file's line count, in the brief's form (1,092)."""
    return f"{b.count(bytes([10])):,}"


def norm(t):
    return " ".join(t.split())


def ws(t):
    return r"\s+".join(re.escape(w) for w in t.split())


LOG, MAIN = [], []

# ---------------------------------------------------------------- 0. the sources, the head, the counted run
for p, h in HASHES.items():
    got = sha_b(read_b(p))
    if got != h:
        stop(f"{p}: sha256 {got}, expected {h}")
for name, (f, h) in CLOSED.items():
    got = sha_b(read_b(f"{N}/{f}"))
    if got != h:
        stop(f"{name} ({f}): sha256 {got}, expected {h}")
if git("rev-parse", "--abbrev-ref", "HEAD").strip() != BRANCH:
    stop("not on the spike branch")
BASE = git("rev-parse", "HEAD").strip()
assert re.fullmatch(r"[0-9a-f]{40}", BASE), BASE
B7 = BASE[:7]
# MAIN-48 (history's last entry): main is merged before the fill, so `<base>` holds main's 4d21c1a
if git("rev-parse", "--verify", f"{MAIN48}^{{commit}}").strip() != MAIN48:
    stop(f"main's {MAIN48} is not a commit here")
if git_rc("merge-base", "--is-ancestor", MAIN48, BASE):
    stop(f"main's 4d21c1a is not an ancestor of HEAD {B7} (MAIN-48: main is merged before the fill)")
if git_rc("diff", "--quiet", L3C, BASE, "--", SP):
    stop(f"spikes/netcode-proof at HEAD {B7} differs from 4e91830's (section 12 item 1: a counted run gives the counts)")
for anc, desc in ((U4D2C, TEM3C), (TEM3C, L3C), (L2C, BASE), (L3C, BASE)):
    if git_rc("merge-base", "--is-ancestor", anc, desc):
        stop(f"{anc} is not an ancestor of {desc}")
if sha_b(git_b("show", f"{BASE}:{BRIEF}")) != HASHES[BRIEF]:
    stop("HEAD's brief is not rev 3.2 (the working file's)")
# the files outside spikes/ and .checkpoints/ that the brief cites by line: unchanged since rev 3.2's commit (a merge
# of main before this runs must leave them so)
for p in ("docs/adr/0001-multiplayer-netcode.md", "docs/design/netcode-proof-messages.md"):
    if git_rc("diff", "--quiet", REV32, BASE, "--", p):
        stop(f"{p} changed since rev 3.2's commit: its cites need a re-check")
LOG.append(f"<base> = HEAD {BASE} on {BRANCH}; `git diff --quiet 4e91830 {B7} -- {SP}` exit 0; HEAD's brief = the "
           f"working file = rev 3.2 ({HASHES[BRIEF][:12]}); cc8595d < c49773f < 4e91830 < HEAD; main's {MAIN48} "
           "(MAIN-48) < HEAD")
# L2: the plan and the carries, before and after; plan.md and history unchanged
if set(git("diff", "--name-only", f"{L2C}~1", L2C).split()) != {PLAN, CARRIES}:
    stop("8bcbf36 changed more than the plan and the carries")
OLD, NEW = {}, {}
for p in (PLAN, CARRIES):
    ob = git_b("show", f"{L2C}~1:{p}")
    n_, pre = OLDSTAMP[p]
    if not sha_b(ob).startswith(pre) or nlines(ob) != n_:
        stop(f"{p} at 8bcbf36~1 is not section 0's stamp ({n_} lines, {pre}…)")
    if git_b("show", f"{L2C}:{p}") != read_b(p) or git_b("show", f"{BASE}:{p}") != read_b(p):
        stop(f"{p}: the working file is not 8bcbf36's or HEAD's")
    OLD[p], NEW[p] = ob.decode().split("\n"), read_b(p).decode().split("\n")
for p in (PLANMD, HIST):
    if git_b("show", f"{BASE}:{p}") != read_b(p):
        stop(f"{p}: the working file is not HEAD's")
# history is living ("Decisions taken" newest first, entries appended): at rev 3.2's commit it is section 0's stamp,
# and since then it may only have gained lines at its end, so no line of it the brief could cite moved
HIST_OLD, HIST_NOW = git_b("show", f"{REV32}:{HIST}"), read_b(HIST)
if sha_b(HIST_OLD) != HIST_STAMP or nlines(HIST_OLD) != "2,011":
    stop("history at rev 3.2's commit is not section 0's stamp")
if not HIST_NOW.startswith(HIST_OLD):
    stop("history changed above its end since rev 3.2 (its cites need a re-check by hand)")
HIST_COMMITS = git("log", "--format=%h", f"{REV32}..{BASE}", "--", HIST).split()
assert (HIST_NOW == HIST_OLD) == (not HIST_COMMITS), HIST_COMMITS
LOG.append(f"sources: the plan {sha_b(read_b(PLAN))[:12]} and the carries {sha_b(read_b(CARRIES))[:12]} (8bcbf36's, "
           f"= HEAD's); at 8bcbf36~1 01ede63d… (1,092 lines) and 8b7f165e… (644 lines), section 0's stamps; plan.md "
           f"unchanged (section 0's 3ca4e12a…); history {sha_b(HIST_NOW)[:12]} ({nlines(HIST_NOW)} lines) = rev 3.2's "
           f"88bc195f… (2,011 lines) + {HIST_NOW.count(bytes([10])) - HIST_OLD.count(bytes([10]))} lines at its end "
           f"({', '.join(HIST_COMMITS) or 'none'}); the {len(CLOSED)} closed records of section 0 and the check and "
           "fold ruling, each sha256 equal")
# L3: four files, five added lines, Cargo.lock's two edges
L3FILES = {f"{SP}/Cargo.toml", f"{SP}/Cargo.lock", f"{SP}/crates/lab/Cargo.toml", f"{SP}/crates/bots/Cargo.toml"}
if set(git("diff", "--name-only", f"{L3C}~1", L3C).split()) != L3FILES:
    stop("4e91830 is not the four manifest files")
if set(git("diff", "--name-only", TEM3C, L3C, "--", "spikes/").split()) != L3FILES:
    stop("`git diff --stat c49773f 4e91830 -- spikes/` does not name the four files")
NUM = [x.split("\t") for x in git("diff", "--numstat", f"{L3C}~1", L3C).strip().split("\n")]
assert sum(int(a) for a, _, _ in NUM) == 5 and all(d == "0" for _, d, _ in NUM), NUM
LOCK_ADDED = [x[1:] for x in git("diff", "-U0", f"{L3C}~1", L3C, "--", f"{SP}/Cargo.lock").split("\n")
              if x.startswith("+") and not x.startswith("+++")]
assert LOCK_ADDED == [' "npt-sim",', ' "npt-bots",'], LOCK_ADDED
# the counted run (L3's, in the main folder): its HEAD, its 11 PASS lines and the counts
CL = open(COUNTED, encoding="utf-8").read().split("\n")


def one(pat, lo=0):
    hits = [(i, m) for i, m in ((i, re.search(pat, x)) for i, x in enumerate(CL)) if m and i >= lo]
    if len(hits) != 1:
        stop(f"{COUNTED}: {pat!r} found {len(hits)} times")
    return hits[0]


i_head, mh = one(r"; HEAD ([0-9a-f]{7,40})$")
assert L3C.startswith(mh.group(1)) or mh.group(1).startswith(L3C), mh.group(1)
i_run, _ = one(r"^=== \S+ UTC run: node check\.mjs$")
i_all, ma = one(r"^check: all (\d+) steps pass$")
STEPS = int(ma.group(1))
PASSES = CL[i_run + 1:i_all]
assert len(PASSES) == STEPS and all(x.startswith("PASS  ") for x in PASSES), PASSES
one(r"^=== \S+ UTC exit 0: node check\.mjs$")
i_end, me = one(r"^--- end (\S+) UTC, exit 0$")
_, mt_ = one(r"measured build: (\d+) tests passed, 0 failed, 0 reported a skip; bites build: (\d+) tests passed, 0 "
             r"failed, 0 reported a skip$")
_, ms_ = one(r"^PASS  self\s.*\s(\d+) of (\d+) tests passed$")
assert ms_.group(1) == ms_.group(2)
_, mk_ = one(r"npt-judge: (\d+) marks read and checked")
_, mr_ = one(r"redact-check: \d+ item\(s\) checked, (\d+) finding\(s\)$")
CNT = dict(measured=int(mt_.group(1)), bites=int(mt_.group(2)), self=int(ms_.group(1)), steps=STEPS,
           marks=int(mk_.group(1)), redact=int(mr_.group(1)))
COUNTS = (f"measured {CNT['measured']}, bites {CNT['bites']}, self {CNT['self']} of {CNT['self']}, {CNT['steps']} "
          f"steps, marks {CNT['marks']}, redact {CNT['redact']}")
LOG.append(f"counted run {COUNTED} (sha256 {sha_b(read_b(COUNTED))[:12]}): HEAD {mh.group(1)} (:{i_head + 1}), "
           f"{STEPS} PASS lines (:{i_run + 2}-{i_all}), \"check: all {STEPS} steps pass\" (:{i_all + 1}), exit 0 at "
           f"{me.group(1)} UTC (:{i_end + 1}): {COUNTS}")
# P7: the runner
if subprocess.run(["bash", "-n", RUNNER], capture_output=True).returncode:
    stop("bash -n u4r-run.sh failed")
LOG.append(f"P7: {RUNNER} sha256 {HASHES[RUNNER]}, `bash -n` exit 0")

# ---------------------------------------------------------------- the brief, its sections
L = read_b(BRIEF).decode().split("\n")
assert len(L) == 1683 and L[-1] == "", ("the brief is not 1,682 lines", len(L))
ORIG = list(L)
OTEXT = "\n".join(ORIG)


def head(prefix, D=None):
    D = ORIG if D is None else D
    h = [i for i, x in enumerate(D) if x.startswith(prefix)]
    assert len(h) == 1, (prefix, len(h))
    return h[0]


H0, H1 = head("## 0. How to read the citations"), head("## 1. Start")
H9, H10 = head("## 9. Recheck after wave 1 and wave 2"), head("## 10. Questions for the advisor")
H11, H12 = head("## 11. Needs a run (day)"), head("## 12. Left for the main session at launch")
I3, I4 = head("3. **The placeholders, by script"), head("4. **The advisor's rulings on U4R-Q1")
assert H0 < H1 < H9 < H10 < H11 < H12 < I3 < I4
S0 = norm(" ".join(ORIG[H0:H1]))
for n_, f_, pre in re.findall(r"`([a-z0-9-]+)` (advisor-[\w-]+\.md) \(([0-9a-f]{8})…\)", S0):
    assert n_ in CLOSED and CLOSED[n_][0] == f_ and CLOSED[n_][1].startswith(pre), (n_, f_, pre)
assert len(re.findall(r"`([a-z0-9-]+)` (advisor-[\w-]+\.md) \(([0-9a-f]{8})…\)", S0)) == len(CLOSED)
STAMP_PLAN = "`plan` is .checkpoints/netcode-proof-test/u4-build-plan.md (1,092 lines, 01ede63d…)"
STAMP_CARR = "`carries` is u4s-carries.md (644 lines, 8b7f165e…)"
STAMP_PMD = "`plan.md` is the proof test's plan (rev 3.3, 1,518 lines, 3ca4e12a…)"
STAMP_HIST = "`history` is history.md (2,011 lines, 88bc195f…)."
for s in (STAMP_PLAN, STAMP_CARR, STAMP_PMD, STAMP_HIST):
    assert S0.count(s) == 1, s
assert nlines(read_b(PLANMD)) == "1,518"

# ---------------------------------------------------------------- item 3's rules, read from item 3
R3 = " ".join(x.strip() for x in ORIG[I3:I4])
PARTS = re.split(r" - (?=`<<P\d+>>`)", R3)
BUL = {re.match(r"`<<(P\d+)>>`", b).group(1): b for b in PARTS[1:]}
assert list(BUL) == ["P1", "P2", "P3", "P4", "P5", "P8", "P9", "P10", "P11", "P12", "P13", "P16", "P6"], list(BUL)
Q = r'"((?:[^"\\]|\\")*)"'


def unq(s):
    return s.replace('\\"', '"')


def src_file(key):
    if key in CLOSED:
        return CLOSED[key][0]
    hit = [f for f, _ in CLOSED.values() if f == key]
    assert len(hit) == 1, key
    return hit[0]


def src_lines(f):
    with open(f"{N}/{f}", encoding="utf-8") as fh:
        return fh.read().split("\n")


COPY = {}  # tag: (kind, copy (a string, or a list of lines), its source)
# P1: carries, from its heading to the line before the next "## "; the entry numbers asserted
m = re.fullmatch(r"`<<P1>>` carries: from the line beginning " + Q + r" \(found once\) to the line before the next line "
                 r"beginning " + Q + r"; the script asserts the copy's entry numbers at line start are exactly (.+)\.",
                 BUL["P1"])
assert m, BUL["P1"][:80]
h1, h2 = unq(m.group(1)), unq(m.group(2))
nums = re.findall(r"\d+", m.group(3))
assert nums == P1_ENTRIES, nums
C = NEW[CARRIES]
st = [i for i, x in enumerate(C) if x.startswith(h1)]
assert len(st) == 1, ("P1", len(st))
en = next(i for i in range(st[0] + 1, len(C)) if C[i].startswith(h2))
p1 = C[st[0]:en]
got = [re.match(r"(\d+)\. ", x).group(1) for x in p1 if re.match(r"\d+\. ", x)]
assert got == nums, ("P1", got)
COPY["P1"] = ("block", p1, f"carries :{st[0] + 1}-{en} (entries {', '.join(got)}; carry 74 as L2 amended it)")
# P2: history's line by its first words and its one continuation line; the line before it asserted
m = re.fullmatch(r"`<<P2>>` history: the line beginning " + Q + r" \(found once\) and the one continuation line after "
                 r"it \(ending " + Q + r"\); the script asserts the line before it is " + Q + r"\.", BUL["P2"])
assert m, BUL["P2"][:80]
p2_first, p2_end, p2_prev = (unq(m.group(k)) for k in (1, 2, 3))
HL = src_lines("history.md")
st = [i for i, x in enumerate(HL) if x.startswith(p2_first)]
assert len(st) == 1, ("P2", len(st))
i2 = st[0]
assert norm(HL[i2 - 1]) == norm(p2_prev), ("P2: the line before", HL[i2 - 1])
ind = len(HL[i2]) - len(HL[i2].lstrip(" "))
assert len(HL[i2 + 1]) - len(HL[i2 + 1].lstrip(" ")) > ind and not HL[i2 + 1].lstrip().startswith("- "), "P2"
assert HL[i2 + 2].lstrip().startswith("- "), "P2: more than one continuation line"
src2 = norm(HL[i2] + " " + HL[i2 + 1])
assert src2.endswith(p2_end) and src2.count(p2_end) == 1, ("P2", src2[-40:])
mk2 = re.match(r"^- ", src2)
assert mk2 and norm(p2_first).startswith("- "), "P2"
p2 = src2[2:]                       # the list marker is history's markup, not a word (u4d2_fold.py's P2, U4d2's launch)
COPY["P2"] = ("inline", p2, f"history :{i2 + 1}-{i2 + 2} (its list marker left out; the line before, :{i2}, asserted)")
MAIN.append("P2: the copy starts at its words, history's list marker \"- \" left out, as U4d2's launch copied it "
            "(u4d2_fold.py's P2)")


def ranged(tag, pat):
    """A ruling's span: the bullet's file and lines, and its from/to words, each found once in the joined lines."""
    mm = re.fullmatch(rf"`<<{tag}>>` (\S+) :(\d+)-(\d+): " + pat, BUL[tag])
    assert mm, (tag, BUL[tag][:90])
    f = src_file(mm.group(1))
    a, z = int(mm.group(2)), int(mm.group(3))
    D = src_lines(f)
    j = " ".join(x.strip() for x in D[a - 1:z])
    first, last = unq(mm.group(4)), unq(mm.group(5))
    assert j.count(first) == 1 and j.count(last) == 1 and j.index(first) < j.index(last), (tag, first, last)
    copy = j[j.index(first):j.index(last) + len(last)]
    return mm, f, a, z, copy


FT = r"from " + Q + r" to " + Q
_, f3, a, z, p3 = ranged("P3", r"the sentence inside T4 item 1's quotation marks, " + FT + r"\.")
COPY["P3"] = ("inline", p3, f"df-ruling ({f3}) :{a}-{z}")
m4, f4, a, z, p4 = ranged("P4", r"the clause inside T4 item 2's quotation marks, " + FT + r", with the first `([^`]+)` "
                          r"replaced by " + Q + r" and " + Q + r" by " + Q + r"; the script asserts the replacements and "
                          r"no `([^`]+)` left\.")
tok, rep1, tok2, rep2, tok3 = m4.group(6), unq(m4.group(7)), unq(m4.group(8)), unq(m4.group(9)), m4.group(10)
assert tok == tok3 == "<crate>" and tok2 == "<crate>'s files" and rep2 == "their files", (tok, tok2, rep2)
assert "or in npt-log" in rep1 and "or in npt-server" in rep1, ("P4: both clauses (U4R-Q13 (a), U4R-Q6 (a))", rep1)
assert p4.count(tok) == 2 and p4.count(tok2) == 1, ("P4", p4.count(tok), p4.count(tok2))
p4 = p4.replace(tok2, rep2, 1)
assert p4.count(tok) == 1 and p4.index(tok) < p4.index(rep2)
p4 = p4.replace(tok, rep1, 1)
assert tok not in p4 and p4.count(rep1) == 1 and p4.count(rep2) == 1, "P4: a <crate> left"
COPY["P4"] = ("inline", p4, f"df-ruling ({f4}) :{a}-{z}, the first `<crate>` -> item 3's string (both clauses: "
              "npt-log, npt-server) and \"<crate>'s files\" -> \"their files\"")
_, f5, a, z, p5 = ranged("P5", FT + r"\.")
COPY["P5"] = ("inline", p5, f"j-r2 ({f5}) :{a}-{z}")
m = re.fullmatch(r"`<<P8>>` (\S+) :(\d+): the words after " + Q + r" to the line's end\.", BUL["P8"])
assert m, BUL["P8"]
D8 = src_lines(src_file(m.group(1)))
ln8, lead8 = int(m.group(2)), unq(m.group(3))
assert at(D8, ln8).count(lead8) == 1 and sum(x.count(lead8) for x in D8) == 1, ("P8", lead8)
p8 = at(D8, ln8)[at(D8, ln8).index(lead8) + len(lead8):].strip()
COPY["P8"] = ("inline", p8, f"j-r2 ({src_file(m.group(1))}) :{ln8}, after {lead8!r}")
_, f9, a, z, p9 = ranged("P9", r"the text inside W1-TARGET item 3's quotation marks, " + FT + r"\.")
COPY["P9"] = ("inline", p9, f"m9 ({f9}) :{a}-{z}")
for tag, key in (("P10", "m9"), ("P11", "d2-r2"), ("P12", "d2-r1")):
    _, f_, a, z, c_ = ranged(tag, FT + r"\.")
    COPY[tag] = ("inline", c_, f"{key} ({f_}) :{a}-{z}")
# P13 to P15: carries entries 6, 14 and 20, each from its line to the line before the next entry or "## "
m = re.fullmatch(r"`<<P13>>`, `<<P14>>`, `<<P15>>`: each from the line beginning " + Q + ", " + Q + " and " + Q +
                 r" \(each found once\) to the line before the next line that begins with an entry number or \"## \"\.",
                 BUL["P13"])
assert m, BUL["P13"][:90]
for tag, anc in zip(("P13", "P14", "P15"), (unq(m.group(k)) for k in (1, 2, 3))):
    s_ = [i for i, x in enumerate(C) if x.startswith(anc)]
    assert len(s_) == 1, (tag, anc, len(s_))
    e_ = next(i for i in range(s_[0] + 1, len(C)) if re.match(r"\d+\. ", C[i]) or C[i].startswith("## "))
    COPY[tag] = ("block", C[s_[0]:e_], f"carries :{s_[0] + 1}-{e_} (entry {anc.split('.')[0]})")
m16, f16, a, z, p16 = ranged("P16", r"the words inside TE-Q3's quotation marks, " + FT +
                             r" \(each found once; sha256 ([0-9a-f]{8})… asserted first\)\.")
assert CLOSED["te-m3"][1].startswith(m16.group(6)) and f16 == CLOSED["te-m3"][0], m16.group(6)
COPY["P16"] = ("inline", p16, f"te-m3 ({f16}) :{a}-{z}")
assert BUL["P6"] == "`<<P6>>` (item 1); `<<P7>>` (item 2): values, filled wherever they stand.", BUL["P6"]

# ---------------------------------------------------------------- item 9 (b): the moved plan and carries cites
OPS = {k: difflib.SequenceMatcher(a=OLD[p], b=NEW[p], autojunk=False).get_opcodes()
       for k, p in (("plan", PLAN), ("carries", CARRIES))}
FILEOF = {"plan": PLAN, "carries": CARRIES}
# the rewritten blocks the fold ruling names (FQ4 :104-107, old lines, both readings of "plan :837-840"): a cite
# touching one is mapped by its text even where difflib finds its line unchanged
FORCE = {"plan": set(range(496, 502)) | set(range(755, 758)) | set(range(837, 841)), "carries": set(range(340, 347))}


def shift_of(k, x):
    for tag, a1, a2, b1, b2 in OPS[k]:
        if a1 < x <= a2:
            return b1 - a1 if tag == "equal" else None
    raise AssertionError((k, x))


def moved(k, a, b):
    return any(x in FORCE[k] or shift_of(k, x) != 0 for x in range(a, b + 1))


# (rev 3.2's line of the cite, file, old range): (new range, words on its first line, words on its last line), each
# asserted in the old version's line and in the new one's ("" as the last words: a blank line before the next "## ")
MANUAL = {
    (45, "plan", "500-501"): ("500-501", "The determinism step", "pre-check only."),        # C28's, in the change lists
    (224, "plan", "494-497"): ("494-497", "- **Sources:** U4d's three synthetic recordings",
                               "which hold the room's real Rewinds"),
    (273, "plan", "750-803"): ("750-803", "### U4r, the L1 runner and its judged runs", "- **Reviewers:** netcode-architect"),
    (276, "plan", "392-504"): ("392-504", "### 4.11 Determinism and hygiene outside npt-sim", "Never committed as files."),
    (276, "plan", "835-839"): ("835-840", "- **Builds:** `HitHistory`'s sizing", "with fixtures."),
    (319, "plan", "750-803"): ("750-803", "### U4r, the L1 runner and its judged runs", "- **Reviewers:** netcode-architect"),
    (325, "plan", "835-839"): ("835-840", "- **Builds:** `HitHistory`'s sizing", "with fixtures."),
    (332, "plan", "751-757"): ("751-757", "- **Builds:** npt-lab: `SimClock`", "as U2's c-u2-pm-0001-3.mjs."),
    (390, "plan", "753-755"): ("753-755", "`scenario` (with `--record`) and `replay`", "condition 9);"),
    (395, "plan", "494-501"): ("494-501", "- **Sources:**", "pre-check only."),
    (395, "plan", "755-756"): ("755-756", "runner recordings (§4.15", "-57's pairing (§4.6)"),
    (405, "plan", "500-501"): ("500-501", "The determinism step", "pre-check only."),
    (486, "plan", "756"): ("756-757", "npt-bots' library: the brawl `Behaviour`", "the open-loop scripts"),
    (488, "plan", "837"): ("837", "the TC shooter's `Behaviour`", "the TC shooter's `Behaviour`"),
    (525, "plan", "756-757"): ("757", "scripts. Run", "as U2's c-u2-pm-0001-3.mjs."),
    (1219, "plan", "498-500"): ("499-500", "Replayed at P0", "keeps them inside the tolerance"),
    (229, "carries", "548-560"): ("554-577", "## U4h (lag compensation and aim)", ""),
    (1244, "carries", "548-560"): ("554-577", "## U4h (lag compensation and aim)", ""),
}
# a cite that shifts in an unchanged block and is also pinned by its text (the main session's decision 1: the change
# lists' two moved plan cites): (rev 3.2's line, file, old range): (words on its first line, words on its last line),
# each asserted in the old version's lines and in the new one's
PIN = {
    (160, "plan", "885-886"): ("- **Reviewers count toward the 3-agent limit**",
                               "a review round starts only when enough of the three slots are free."),
    (166, "plan", "917-918"): ("6. The main folder keeps its own cache", "with `CARGO_TARGET_DIR` unset."),
}
# every cite of the brief into the plan or the carries whose line moved or which touches a rewritten block, outside
# section 10 (the change lists at the top included: FQ4's "every other cite", the main session's decision 1): (rev
# 3.2's line of its first cite, file, its words as the brief has them); every `:n` in the words is that file's
CITES = [
    (45, "plan", "(plan §4.15 :500-501)"),                                                  # C28: holds, by its text
    (160, "plan", "(plan :885-886), for the advisor to confirm."),                          # C22 (:1673 has its own)
    (166, "plan", "plan §7.1 item 6 (:917-918)"),                                           # C24
    (224, "plan", "| plan :494-497 (`Recording::synthetic` in npt-sim's library) |"),       # section 0's table, 3rd
    (226, "plan", "| plan :873-880 (U4d2's lane widened) |"),                              # cell only (the 1st is
    (229, "carries", "| §U4h | carries :548-560 |"),                                       # rev 2's, a record)
    (249, "plan", "W1-TARGET's lock (section 4; plan :909-921;"),
    (250, "plan", "one at a time (Lanes, plan :881-882)."),
    (251, "plan", "in the main folder (plan :766; §7.2 :927-928; section 7)."),
    (273, "plan", "plan §5 U4r (:750-803) and what it builds to"),
    (276, "plan", "§4.11 to §4.15 (:392-504)"),
    (276, "plan", "U4h's Builds (:835-839); Lanes (:850-888); §6 (:890-906); §7 (:908-931);"),
    (289, "carries", "the recordings section (carries :461-489)"),
    (311, "plan", "binding on proto, room, client and server code (plan §7.1 item 8, :920-921)"),
    (319, "plan", "U4r builds plan §5 U4r (:750-803) exactly"),
    (325, "plan", "pong delayer (U4h, :835-839); npt-bots' npt-net glue and npt-load (U6, :896-897); -30's marked C "
                  "part (U6, :894-895)"),
    (326, "plan", "(its own small unit after U4r, :903-904:"),
    (332, "plan", "Each item from plan :751-757, built to the section named."),
    (390, "plan", "(§4.1 :198; plan :753-755)"),
    (393, "plan", "never committed (plan §7.2 :927-928)."),
    (395, "plan", "**The three runner recordings at U4r** (§4.15 :494-501; plan :755-756;"),
    (405, "plan", 'serves the pre-check only" (§4.15 :500-501).'),
    (486, "plan", "the two kinds U4r builds (plan :756):"),
    (488, "plan", "The TC shooter is U4h's (:837);"),
    (525, "plan", "As c-u2 does (plan :756-757):"),
    (533, "plan", "(`CARGO_TARGET_DIR` unset: :917-918)."),
    (626, "plan", "in the bites build (§7.1 item 3, :913-914)"),
    (664, "carries", "(T4 item 3; carry 67, carries :462-479)"),
    (671, "plan", "### 2.7 Public items (A19; plan §7.2 :929-930)"),
    (678, "plan", "(check.mjs does not: plan :930)."),
    (743, "plan", "plan §7.1 :909-921; the advisor's W1-TARGET"),
    (763, "plan", "(plan :912; the runner does it)."),
    (765, "plan", "**Counts are deltas** (plan :913-914, :883-884),"),
    (776, "plan", "never clean anything (plan :919)."),
    (777, "plan", "No copy cleans the shared folder (plan :919)."),
    (782, "plan", "before any further claim (plan :915)."),
    (783, "plan", "serializes builds: expected (plan :916)."),
    (784, "plan", "The main folder's cache is not yours (plan :917-918)."),
    (786, "plan", "**Your files, and only these** (Lanes :881;"),
    (803, "plan", "session's runs write it: §7.2 :927-928)."),
    (811, "plan", "the main session merges (plan :924-926)."),
    (839, "plan", "binds proto, room, client and server code (plan §7.1 item 8, :920-921):"),
    (918, "plan", "committed under results/ (plan :766; §7.2 :927-928), by the scripts of 2.3"),
    (923, "plan", "as U4h's judged runs are numbered (plan :841-842)."),
    (1119, "plan", "§7.2 :927's \"one set at a time\""),
    (1144, "plan", "never committed (plan §7.2 :928)."),
    (1219, "plan", "and §4.15 :498-500's reason for a P0 recording"),
    (1228, "plan", "on the real transports (plan :894-895),"),
    (1244, "carries", "u4s-carries.md's §U4h, carries :548-560: entry 88"),
    (1268, "plan", "| R11 | crates/lab/Cargo.toml as U4d2 left it (plan :873-880) |"),
    (1669, "plan", "engine-architect (public items: plan :929-930),"),
    (1673, "plan", "reviewers included (plan :885-886), so in two batches"),
    (1675, "plan", "`CARGO_TARGET_DIR` unset (plan §7.1 item 6, :917-918),"),
]
# a cite the classifier below gives to the plan that is not the plan's (its nearest file name is the row's plan cite)
FALSEPOS = {(1263, ":752-758"): "l1/src/lib.rs's BITE_IDS, R6's row"}
TOK = re.compile(r"(?<![\w.]):(\d+)(?:-(\d+))?")
USED, USEDPIN, INVPOS, ITEMS = set(), set(), set(), []


def once_in(text, phrase, tag):
    ms = list(re.finditer(ws(phrase), text))
    flat = norm(text).count(norm(phrase))
    if len(ms) != 1 or flat != 1:
        stop(f"{tag}: {phrase!r} found {len(ms)} times ({flat} flattened)")
    return ms[0]


def rng(s):
    a, _, b = s.partition("-")
    return int(a), int(b or a)


def map_cite(line, k, tokm):
    a, b = int(tokm.group(1)), int(tokm.group(2) or tokm.group(1))
    key = (line, k, tokm.group(0)[1:])
    O, Nn = OLD[FILEOF[k]], NEW[FILEOF[k]]
    if key in MANUAL:
        assert key not in USED, key
        USED.add(key)
        new, fw, lw = MANUAL[key]
        c, d = rng(new)
        for D, x, y, v in ((O, a, b, "8bcbf36~1"), (Nn, c, d, "now")):
            assert fw in at(D, x), (key, v, x, fw)
            if lw:
                assert lw in at(D, y), (key, v, y, lw)
            else:
                assert at(D, y) == "" and at(D, y + 1).startswith("## "), (key, v, y)
        how = f"by its text (MANUAL: {k} :{a} {fw!r}" + (f", :{b} {lw!r}" if lw else f", :{b} blank before \"## \"") + \
              f"; now :{c} and :{d})"
        return ":" + new, how
    shifts = {shift_of(k, x) for x in range(a, b + 1)}
    if len(shifts) != 1 or None in shifts or any(x in FORCE[k] for x in range(a, b + 1)):
        stop(f"brief :{line}: {k} :{a}-{b} touches a rewritten block and has no MANUAL key")
    s = shifts.pop()
    assert O[a - 1:b] == Nn[a - 1 + s:b + s], key
    new = f":{a + s}" if a == b else f":{a + s}-{b + s}"
    if key in PIN:
        assert key not in USEDPIN, key
        USEDPIN.add(key)
        fw, lw = PIN[key]
        for D, x, y, v in ((O, a, b, "8bcbf36~1"), (Nn, a + s, b + s, "now")):
            assert fw in at(D, x) and lw in at(D, y), (key, v, x, y)
        return new, (f"shifted {s:+d} (an unchanged block), by its text (PIN: {k} :{a} {fw!r}, :{b} {lw!r}; now :{a + s} "
                     f"and :{b + s})")
    return new, f"shifted {s:+d} (an unchanged block)"


for line, k, words in CITES:
    m = once_in(OTEXT, words, f"cite at :{line}")
    toks = list(TOK.finditer(m.group(0)))
    assert toks, (line, words)
    first_line = OTEXT.count("\n", 0, m.start() + toks[0].start()) + 1
    assert first_line == line, ("cite inventory", line, first_line)
    out, pos, parts = [], 0, []
    for t in toks:
        tl = OTEXT.count("\n", 0, m.start() + t.start()) + 1
        INVPOS.add(m.start() + t.start())
        new, how = map_cite(tl, k, t)
        parts.append(f"{k} {t.group(0)} -> {new} ({how})")
        out.append(m.group(0)[pos:t.start()] + new)
        pos = t.end()
    out.append(m.group(0)[pos:])
    newwords = norm("".join(out))
    tag = f"cite :{line}"
    if newwords == norm(words):
        LOG.append(f"brief :{line}: held, unchanged: " + "; ".join(parts))
        continue
    ITEMS.append(dict(tag=tag, old=words, new=newwords, src="; ".join(parts)))
assert USED == set(MANUAL), ("MANUAL keys not used", set(MANUAL) - USED)
assert USEDPIN == set(PIN), ("PIN keys not used", set(PIN) - USEDPIN)

# the classifier: every cite token outside section 10 (the change lists at the top included) whose nearest file name is
# the plan or the carries, and whose lines moved or touch a rewritten block, must be in the inventory above
MARKER = re.compile(r"(plan\.md|plan|carries|u4s-carries\.md|history|[\w/-]+(?:\.[\w-]+)*\.(?:rs|mjs|json|md|toml|sh|"
                    r"lock|js|py|log)|advisor-[\w-]+\.md|advisor|wave1|close|j-ruling|j-r2|df-ruling|e-ruling|e-r1|e-r2|"
                    r"d2-ruling|d2-r1|d2-r2|m9|te-m3-check|te-m3|u4r-check|catalogue|README|c-u2|ADR 0001)(?=[\s:(§,]|$)"
                    r"|(§[\dU][\w.]*)")


def file_of(pos):
    seg = OTEXT[max(0, pos - 400):pos]
    ms = list(MARKER.finditer(seg))
    if not ms:
        return "?"
    mm = ms[-1]
    if mm.group(2):
        w = re.search(r"(plan\.md|README|catalogue|plan)$", seg[:mm.start()].rstrip())
        return w.group(1) if w else "plan"
    return {"u4s-carries.md": "carries"}.get(mm.group(1), mm.group(1))


T0 = [i for i in range(H0, H1) if ORIG[i].startswith("| ")]
TOP_INV, LEFT_S10, FP_USED = [], [], set()
for t in TOK.finditer(OTEXT):
    a, b = int(t.group(1)), int(t.group(2) or t.group(1))
    ln0 = OTEXT.count("\n", 0, t.start())
    k = {"plan": "plan", "?": "plan", "carries": "carries"}.get(file_of(t.start()))
    if not k or b > len(OLD[FILEOF[k]]) - 1 or not moved(k, a, b):
        continue
    if ln0 in T0 and OTEXT.count("|", OTEXT.rfind("\n", 0, t.start()) + 1, t.start()) < 2:
        continue                                        # section 0's table, first cell: rev 2's cite, a record
    what = f":{ln0 + 1} {k} {t.group(0)}"
    if H10 <= ln0 < H11:
        LEFT_S10.append(what)
    elif (ln0 + 1, t.group(0)) in FALSEPOS:
        FP_USED.add((ln0 + 1, t.group(0)))
    elif t.start() not in INVPOS:
        stop(f"brief :{ln0 + 1}: {k} cite {t.group(0)} moved and is not in the inventory")
    elif ln0 < H0:
        TOP_INV.append(what)                            # the change lists' moved cites, in the inventory (decision 1)
assert FP_USED == set(FALSEPOS), FP_USED
# the change lists' moved plan cites are exactly C28's (held by its text), C22's and C24's (re-pointed)
assert TOP_INV == [":45 plan :500-501", ":160 plan :885-886", ":166 plan :917-918"], TOP_INV
LOG.append(f"cites: {len(CITES)} inventory entries ({sum(len(list(TOK.finditer(w))) for _, _, w in CITES)} cite "
           f"tokens), each found once; {len(MANUAL)} MANUAL keys and {len(PIN)} PIN keys each used once; the "
           "classifier finds no other moved plan or carries cite outside section 10 (its one false positive: brief "
           ":1263's :752-758, l1/src/lib.rs's BITE_IDS)")
LOG.append("the change lists at the top (FQ4 :103-105, \"every other cite\"; the main session's decision 1): their moved "
           "plan cites are in the inventory above, :160's and :166's re-pointed and :45's held by its text: "
           + ", ".join(TOP_INV))
LOG.append(f"left as written, section 10 whole (FQ4 :102-104): {len(LEFT_S10)} moved cites: " + ", ".join(LEFT_S10))

# ---------------------------------------------------------------- item 9 (c): L3's files, R11 and the tally
LABO, LABN = (git("show", f"{r}:{SP}/crates/lab/Cargo.toml").split("\n") for r in (f"{L3C}~1", BASE))
BOTO, BOTN = (git("show", f"{r}:{SP}/crates/bots/Cargo.toml").split("\n") for r in (f"{L3C}~1", BASE))
WSN = git("show", f"{BASE}:{SP}/Cargo.toml").split("\n")


def line_of(D, text):
    h = [i + 1 for i, x in enumerate(D) if x == text]
    assert len(h) == 1, (text, h)
    return h[0]


def deps(D):
    s = line_of(D, "[dependencies]")
    e = s
    while e < len(D) and at(D, e + 1).strip():
        e += 1
    return s, e


def map1(O, Nn, x):
    for tag, a1, a2, b1, b2 in difflib.SequenceMatcher(a=O, b=Nn, autojunk=False).get_opcodes():
        if tag == "equal" and a1 < x <= a2:
            return x + b1 - a1
    raise AssertionError(x)


# rev 3.2's R11 cites, read at 4e91830~1: npt-lab's dependencies :23-34 with no npt-bots, npt-bots' :20-25 with no
# npt-sim, npt-net at :24
assert deps(LABO) == (23, 34) and "npt-bots.workspace = true" not in LABO, deps(LABO)
assert deps(BOTO) == (20, 25) and "npt-sim.workspace = true" not in BOTO and at(BOTO, 24) == "npt-net.workspace = true"
lab_bots, bots_sim = line_of(LABN, "npt-bots.workspace = true"), line_of(BOTN, "npt-sim.workspace = true")
ws_bots = line_of(WSN, 'npt-bots = { path = "crates/bots" }')
bots_net = line_of(BOTN, "npt-net.workspace = true")
assert map1(BOTO, BOTN, 24) == bots_net
(ld0, ld1), (bd0, bd1) = deps(LABN), deps(BOTN)
assert ld0 <= lab_bots <= ld1 and bd0 <= bots_sim <= bd1
F3 = dict(lab_bots=lab_bots, bots_sim=bots_sim, ws_bots=ws_bots, bots_net=bots_net, lab_deps=(ld0, ld1),
          bots_deps=(bd0, bd1))
assert F3 == dict(lab_bots=32, bots_sim=22, ws_bots=42, bots_net=25, lab_deps=(23, 35), bots_deps=(20, 26)), F3
# the brief's other cites of L3's four files outside section 10 and the change lists: written at the head by the
# fold (F6-1, F6-2), asserted at <base>
for words, D, ln_, text in (("no features: bots/Cargo.toml:22)", BOTN, 22, "npt-sim.workspace = true"),
                            ("npt-net and npt-log (:20-26)", BOTN, 20, "[dependencies]"),
                            ("npt-lab on npt-bots (lab/Cargo.toml:32)", LABN, 32, "npt-bots.workspace = true"),
                            ("`iter_over_hash_type` (Cargo.toml:107)", WSN, 107, 'iter_over_hash_type = "deny"')):
    once_in(OTEXT, words, "an L3 cite")
    assert at(D, ln_) == text, (words, at(D, ln_))
assert at(BOTN, 26) == "npt-log.workspace = true"
L3TOK = re.compile(r"(?<![\w/-])(?:crates/)?(?:(?:lab|bots)/)?Cargo\.(?:toml|lock):\d+(?:-\d+)?")
L3LEFT = [f":{OTEXT.count(chr(10), 0, t.start()) + 1} {t.group(0)}" for t in L3TOK.finditer(OTEXT)
          if OTEXT.count(chr(10), 0, t.start()) < H0 or H10 <= OTEXT.count(chr(10), 0, t.start()) < H11]
LOG.append("left as written (records), L3's file cites in the change lists and section 10 (each read at 4e91830~1): "
           + ", ".join(L3LEFT))
LOG.append(f"L3's files at <base> ({B7}): lab/Cargo.toml:{lab_bots} npt-bots, deps :{ld0}-{ld1} (were :23-34); "
           f"bots/Cargo.toml:{bots_sim} npt-sim, deps :{bd0}-{bd1} (were :20-25), npt-net :{bots_net} (was :24); "
           f"Cargo.toml:{ws_bots} the workspace table's npt-bots line; Cargo.lock +2 lines {LOCK_ADDED}; 2.2's "
           "bots/Cargo.toml:22, :20-26 and lab/Cargo.toml:32 and r1's Cargo.toml:107 hold (written at the head by F6-1, "
           "F6-2)")
ROWS = {}
for i in range(H9, H10):
    mm = re.match(r"^\| (R\d+)(?: \(new\))? \|", ORIG[i])
    if mm:
        cells = re.split(r"(?<!\\)\|", ORIG[i])[1:-1]
        ROWS[mm.group(1)] = (i, cells[-1].strip())
OLD_R11 = ROWS["R11"][1]
assert OLD_R11 == ("**holds:** lab's and bots' manifests unchanged since df8975b (D5): npt-lab names no npt-bots "
                   "(lab/Cargo.toml:23-34), npt-bots no npt-sim (bots/Cargo.toml:20-25). New fact for U4R-Q1: npt-bots "
                   "names npt-net (:24) (C26)"), OLD_R11
NEW_R11 = (f"**changed:** since L3 ({L3C}, the main session's commit: U4R-Q1 (b)), npt-lab names npt-bots "
           f"(lab/Cargo.toml:{lab_bots}, in its dependencies :{ld0}-{ld1}) and npt-bots names npt-sim, no features "
           f"(bots/Cargo.toml:{bots_sim}, in its dependencies :{bd0}-{bd1}), and the workspace table gains the "
           f"`npt-bots` line that the `.workspace = true` form needs (Cargo.toml:{ws_bots}); Cargo.lock gains those two "
           "edges only. "
           f"npt-bots still names npt-net (bots/Cargo.toml:{bots_net}) (C26). 2.2 and D5 say so; re-walked at launch "
           "(section 12 item 9)")
# the main session's decision 2: R11's npt-net cite names its file (a bare ":25" after Cargo.toml and Cargo.lock reads
# as Cargo.toml's)
assert NEW_R11.count(f"npt-bots still names npt-net (bots/Cargo.toml:{bots_net}) (C26)") == 1 and "(:" not in NEW_R11
assert at(BOTN, bots_net) == "npt-net.workspace = true"
ITEMS.append(dict(tag="R11", old=OLD_R11, new=NEW_R11, src=f"git show {L3C}~1 and {B7} of crates/lab/Cargo.toml, "
                  "crates/bots/Cargo.toml and Cargo.toml (the facts above); the table's \"**changed:**\" form"))
MAIN.append("R11's last cell (section 9; item 9's re-walk for L3): " + NEW_R11)


def tally(st):
    hold = [r for r in (f"R{k}" for k in range(1, 35)) if st[r].startswith("**holds")]
    chg = [r for r in (f"R{k}" for k in range(1, 35)) if st[r].startswith("**changed")]
    assert len(hold) + len(chg) == 34, (hold, chg)
    return hold, chg


ST = {r: c for r, (_, c) in ROWS.items()}
hold, chg = tally(ST)
t0 = [i for i in range(H9, H10) if ORIG[i].startswith("Of R1-R34: ")]
assert len(t0) == 1 and ORIG[t0[0] + 2] == "", t0
OLD_TALLY = norm(ORIG[t0[0]] + " " + ORIG[t0[0] + 1])
assert OLD_TALLY == (f"Of R1-R34: {len(hold)} hold ({', '.join(hold)}) and {len(chg)} changed ({', '.join(chg)}); ten "
                     "rows added (R35-R44)."), OLD_TALLY
ST["R11"] = NEW_R11
hold, chg = tally(ST)
assert "R11" in chg and (len(hold), len(chg)) == (17, 17)
NEW_TALLY = (f"Of R1-R34: {len(hold)} hold ({', '.join(hold)}) and {len(chg)} changed ({', '.join(chg)}; R11 at "
             "launch, for L3's manifests: section 12 item 9); ten rows added (R35-R44).")
ITEMS.append(dict(tag="tally", old=OLD_TALLY, new=NEW_TALLY, src="the table's R1-R34 verdicts with R11's new one"))
MAIN.append("section 9's tally line: " + NEW_TALLY)

# ---------------------------------------------------------------- item 9 (a): section 0's stamps, and its sentence
plan_n, carr_n = nlines(read_b(PLAN)), nlines(read_b(CARRIES))
ITEMS.append(dict(tag="stamp plan", old="(1,092 lines, 01ede63d…)", new=f"({plan_n} lines, {HASHES[PLAN][:8]}…)",
                  src=f"u4-build-plan.md now ({plan_n} lines, sha256 {HASHES[PLAN][:12]})"))
ITEMS.append(dict(tag="stamp carries", old="u4s-carries.md (644 lines, 8b7f165e…)",
                  new=f"u4s-carries.md ({carr_n} lines, {HASHES[CARRIES][:8]}…)",
                  src=f"u4s-carries.md now ({carr_n} lines, sha256 {HASHES[CARRIES][:12]})"))
if HIST_COMMITS:
    hist_new = f"`history` is history.md ({nlines(HIST_NOW)} lines, {sha_b(HIST_NOW)[:8]}…)."
    SENT0 = ("The plan's, the carries' and history's stamps were re-set at launch (section 12 item 9, by u4r_fill.py): "
             "L2's commit 8bcbf36 changed the plan and the carries, and every cite of this brief into them whose line "
             "moved was re-pointed then, except in section 10 and the change lists above, which stay as written "
             f"(records); history gained lines at its end only ({', '.join(HIST_COMMITS)}), so no line of it moved; "
             "plan.md's stamp holds.")
else:
    hist_new = STAMP_HIST
    SENT0 = ("The plan's and the carries' stamps were re-set at launch (section 12 item 9, by u4r_fill.py) because L2's "
             "commit 8bcbf36 changed both files (plan.md's and history's stamps hold), and every cite of this brief "
             "into them whose line moved was re-pointed then, except in section 10 and the change lists above, which "
             "stay as written (records).")
ITEMS.append(dict(tag="stamp history and sentence", old=STAMP_HIST, new=hist_new + " " + SENT0,
                  src=f"history.md now ({nlines(HIST_NOW)} lines, sha256 {sha_b(HIST_NOW)[:12]}); the sentence is the "
                      "main session's words"))
MAIN.append("section 0's stamps bullet, after the history stamp: " + SENT0)

# ---------------------------------------------------------------- items 1 and 2: P6 and P7 where they stand
P6_1_OLD = ("(P6, filled by the main session at launch: the spike branch's head then; U4r depends on U4c, U4d2, U4e "
            "and U4j, plan :750, all merged by cc8595d).")
P6_1_NEW = (f"(P6, filled by the main session at launch: {BASE}, the spike branch's head then, whose "
            f"spikes/netcode-proof is 4e91830's, L3's manifests (`git diff --quiet 4e91830 {B7} -- spikes/netcode-proof` "
            "exits 0); U4r depends on U4c, U4d2, U4e and U4j, plan :750, all merged by cc8595d; your worktree may start "
            "at a later commit whose spikes/ is `<base>`'s, the main session's fill commit, as at U4d2's launch, whose "
            "builder recorded `git diff --stat <base> HEAD -- spikes/` empty).")
P6_4_OLD = f"The base: {COUNTS} (P6: the counts at `<base>`; at cc8595d,"
P6_4_NEW = f"The base: {COUNTS} (P6: the counts at `<base>`, {B7}, whose spikes/netcode-proof is 4e91830's; at cc8595d,"
P6_12_OLD = "1. **`<base>` and its counts (P6).**"
P6_12_NEW = f"1. **`<base>` and its counts (P6: {B7}; {COUNTS}, its spikes/netcode-proof being 4e91830's; done at launch).**"
once_in(OTEXT, f"the counts are D1's ({COUNTS});", "D1's counts in section 12 item 1")
P7_1_OLD = "(P7: sha256 349b51ad…, made by the main session"
P7_1_NEW = "(P7: sha256 349b51ad…, `bash -n` exit 0 at launch; made by the main session"
P7_12_OLD = "2. **The runner (P7: u4r-run.sh, sha256 349b51ad…).**"
P7_12_NEW = "2. **The runner (P7: u4r-run.sh, sha256 349b51ad…, `bash -n` exit 0; done at launch).**"
assert HASHES[RUNNER].startswith("349b51ad")
for tag, o, n_, src in (("P6 (section 1 item 1)", P6_1_OLD, P6_1_NEW, f"git rev-parse HEAD = {BASE}"),
                        ("P6 (section 4 rule 5)", P6_4_OLD, P6_4_NEW, f"<base> {B7}; the counts from {COUNTED}"),
                        ("P6 (section 12 item 1)", P6_12_OLD, P6_12_NEW, f"<base> {B7}; the counts from {COUNTED}"),
                        ("P7 (section 1 item 4)", P7_1_OLD, P7_1_NEW, f"{RUNNER}: sha256 and bash -n"),
                        ("P7 (section 12 item 2)", P7_12_OLD, P7_12_NEW, f"{RUNNER}: sha256 and bash -n")):
    ITEMS.append(dict(tag=tag, old=o, new=n_, src=src))
    MAIN.append(f"{tag}: {n_}")

# ---------------------------------------------------------------- item 3: the copies into their homes
for tag in ("P2", "P3", "P4", "P5", "P8", "P9", "P10", "P11", "P12", "P16"):
    kind, copy, src = COPY[tag]
    ITEMS.append(dict(tag=tag, old=f'"<<{tag}>>"', new=f'"{copy}"', src=src, copy=copy))
ITEMS.append(dict(tag="P1", kind="block", names=["P1"], src=COPY["P1"][2]))
ITEMS.append(dict(tag="P13-P15", kind="block", names=["P13", "P14", "P15"],
                  src="; ".join(f"{t}: {COPY[t][2]}" for t in ("P13", "P14", "P15"))))

# ---------------------------------------------------------------- the edit engine
MARK = re.compile(r"^( *)(- |\d+\. )")
STRUCT = re.compile(r"^(- |\d+\. |#|\||```|\*\*|<<)")
SUB = re.compile(r"^\((?:[ivx]+|[a-h]|\d+)\) ")
RISKY = re.compile(r"^(\d+[.)](\s|$)|[-*+](\s|$)|#|>|```|~~~|\||=+\s*$|\*\*|<<|\((?:[ivx]+|[a-h]|\d+)\)(\s|$))")
NBSP = " "


def refill(t, first, cont):
    for _ in range(40):
        w = textwrap.TextWrapper(width=120, initial_indent=first, subsequent_indent=cont, break_long_words=False,
                                 break_on_hyphens=False)
        out = w.wrap(t)
        bad = [k for k in range(1, len(out)) if RISKY.match(out[k][len(cont):])]
        if not bad:
            return [x.replace(NBSP, " ") for x in out]
        k = bad[0]
        tk = out[k][len(cont):].split(" ")[0]
        prev = out[k - 1].rstrip().split(" ")[-1]
        assert f"{prev} {tk}" in t, (prev, tk)
        t = t.replace(f"{prev} {tk}", f"{prev}{NBSP}{tk}", 1)
    stop(f"refill: no safe wrap for {t[:60]!r}")


def indent(x):
    return len(x) - len(x.lstrip(" "))


def unit_end(D, f):
    mm = MARK.match(D[f])
    prefix = mm.group(0) if mm else " " * indent(D[f])
    cont = " " * len(prefix)
    e = f
    while e + 1 < len(D) and D[e + 1].strip() and indent(D[e + 1]) == len(cont) \
            and not STRUCT.match(D[e + 1].lstrip(" ")) and not SUB.match(D[e + 1].lstrip(" ")):
        e += 1
    return prefix, cont, e


FLAT, NEWS, GONE, WHERE = [], [], [], {}


def edit(tag, old, new):
    """`old` (found once, white space read as one space) -> `new`, word by word: unchanged words keep their line
    breaks; a table row is edited in its line; any other unit is rewrapped from its first changed line to its end only
    where a line passes 120."""
    global L
    text = "\n".join(L)
    m = once_in(text, old, tag)
    mt = m.group(0)
    wm = list(re.finditer(r"\S+", mt))
    ow, nw = [x.group(0) for x in wm], new.split()
    out, pos, first = [], 0, None
    for op, i1, i2, j1, j2 in difflib.SequenceMatcher(a=ow, b=nw, autojunk=False).get_opcodes():
        ins = " ".join(nw[j1:j2])
        if op == "equal":
            out.append(mt[pos:wm[i2 - 1].end()])
            pos = wm[i2 - 1].end()
            continue
        if first is None:
            first = m.start() + (wm[i1 - 1].end() if (op != "replace" and i1) else wm[i1].start() if i1 < len(wm)
                                 else wm[-1].end())
        if op == "insert":
            out.append(" " + ins if i1 else ins + " ")
        elif op == "replace":
            out.append(mt[pos:wm[i1].start()] + ins)
            pos = wm[i2 - 1].end()
        else:
            pos = wm[i2 - 1].end() if i1 else wm[i2].start()
    out.append(mt[pos:])
    piece = "".join(out)
    nt = text[:m.start()] + piece + text[m.end():]
    want = norm(text[:m.start()] + new + text[m.end():])
    assert norm(piece) == norm(new) and norm(nt) == want, (tag, "the word edit differs")
    f = text.count("\n", 0, first)
    NL = nt.split("\n")
    if L[f].lstrip().startswith("|"):
        assert "\n" not in mt and len(NL) == len(L), (tag, "a row edit runs past its row")
        how = f"the result's row :{f + 1}, edited in its line"
    else:
        prefix, cont, e = unit_end(NL, f)
        if any(len(x) > 120 for x in NL[f:e + 1]):
            body = " ".join([NL[f][len(prefix):].strip()] + [x.strip() for x in NL[f + 1:e + 1]])
            outl = refill(body, prefix, cont)
            assert norm(" ".join(outl)) == norm(" ".join(NL[f:e + 1])), tag
            how = (f"the result's :{f + 1}-{f + len(outl)}, rewrapped within 120 from its first changed line to its "
                   f"unit's end ({e - f + 1} -> {len(outl)} lines)")
            NL[f:e + 1] = outl
        else:
            how = f"the result's :{f + 1}, in place (no line over 120)"
    assert norm("\n".join(NL)) == want, (tag, "a word outside the edit changed")
    L = NL
    FLAT.append((norm(old), norm(new)))
    NEWS.append((tag, new))
    if norm(old) not in norm(new):
        GONE.append((tag, old))
    return how


def fit(lines):
    """A block copy's lines as they are, but an entry with a line over 120 is rewrapped within 120 in its own
    continuation indent (item 3's rule: "rewrapped to 120"; white space only, checked)."""
    out, notes, i = [], [], 0
    while i < len(lines):
        if not re.match(r"\d+\. ", lines[i]):
            out.append(lines[i])
            i += 1
            continue
        j = i + 1
        while j < len(lines) and lines[j].strip() and not re.match(r"\d+\. |## ", lines[j]):
            j += 1
        ent = lines[i:j]
        if any(len(x) > 120 for x in ent):
            cont = " " * (indent(ent[1]) if len(ent) > 1 else len(re.match(r"\d+\. ", ent[0]).group(0)))
            new = refill(" ".join(x.strip() for x in ent), "", cont)
            assert norm(" ".join(new)) == norm(" ".join(ent)) and all(len(x) <= 120 for x in new), ent[0][:30]
            notes.append(f"entry {ent[0].split('.')[0]} rewrapped within 120 in its indent ({len(ent)} -> {len(new)} "
                         "lines, white space only)")
            ent = new
        out += ent
        i = j
    return out, notes


def block(tag, names):
    global L
    k = [i for i, x in enumerate(L) if x == f"<<{names[0]}>>"]
    assert len(k) == 1, (tag, len(k))
    s = k[0]
    for j, nm in enumerate(names):
        assert L[s + j] == f"<<{nm}>>", (tag, nm)
    assert L[s - 1] == "```" and L[s + len(names)] == "```", (tag, "not in its fence")
    new, notes = [], []
    for nm in names:
        lines_, n_ = fit(COPY[nm][1])
        new += lines_
        notes += [f"{nm}'s {x}" for x in n_]
    FLAT.append((norm("``` " + " ".join(f"<<{nm}>>" for nm in names) + " ```"), norm("``` " + " ".join(new) + " ```")))
    NEWS.append((tag, " ".join(new)))              # its names stay in item 3's list: the check below reads the rest
    L[s:s + len(names)] = new
    return (f"its fence's placeholder line{'s' if len(names) > 1 else ''} replaced by {len(new)} lines as they are, the "
            f"result's :{s + 1}-{s + len(new)}" + (f" ({'; '.join(notes)})" if notes else ""))


# each site in rev 3.2 once, none overlapping
SPANS = []
for it in ITEMS:
    if it.get("kind") == "block":
        k = [i for i, x in enumerate(ORIG) if x == f"<<{it['names'][0]}>>"]
        assert len(k) == 1
        s = len("\n".join(ORIG[:k[0]])) + 1
        e = s + len("\n".join(ORIG[k[0]:k[0] + len(it["names"])]))
    else:
        m = once_in(OTEXT, it["old"], it["tag"])
        s, e = m.start(), m.end()
    it["rev32"] = (OTEXT.count("\n", 0, s) + 1, OTEXT.count("\n", 0, max(e - 1, s)) + 1)
    SPANS.append((s, e, it["tag"]))
SPANS.sort()
for (a0, a1, ta), (b0, b1, tb) in zip(SPANS, SPANS[1:]):
    assert a1 <= b0, ("sites overlap in rev 3.2", ta, tb)

# applied in the brief's order
for it in sorted(ITEMS, key=lambda x: x["rev32"]):
    how = block(it["tag"], it["names"]) if it.get("kind") == "block" else edit(it["tag"], it["old"], it["new"])
    a, b = it["rev32"]
    site = f":{a}" if a == b else f":{a}-{b}"
    what = it["tag"] if it["tag"].startswith(("P", "R11", "tally", "stamp")) else "re-pointed"
    LOG.append(f"brief {site} ({what}): {it['src']}; {how}")

# ---------------------------------------------------------------- the checks
final = "\n".join(L)
flat = norm(final)
exp = norm(OTEXT)
for o, n_ in FLAT:
    assert exp.count(o) == 1, ("words check: an old phrase is not once at its turn", o[:60])
    exp = norm(exp.replace(o, n_, 1))
assert exp == flat, "words check: the result's words differ from rev 3.2's with only these changes made"
for tag, new in NEWS:
    assert flat.count(norm(new)) == 1, (tag, "the new words are not in the result once", new[:60])
for tag, old in GONE:
    assert norm(old) not in flat, (tag, "the replaced words are still in the brief", old[:60])
J3, J4 = head("3. **The placeholders, by script", L), head("4. **The advisor's rulings on U4R-Q1", L)
outside = "\n".join(L[:J3] + L[J4:])
assert not re.search(r"<<P\d", outside), ("a placeholder left", re.findall(r"<<P\d+>>", outside))
gen = [i + 1 for i, x in enumerate(L) if "<<P" in x and not J3 <= i < J4]
assert len(gen) == 1 and "`<<P...>>`" in L[gen[0] - 1], gen       # the intro's name of the form, not a placeholder
inlist = " ".join(L[J3:J4])
for tag in ("P1", "P2", "P3", "P4", "P5", "P6", "P7", "P8", "P9", "P10", "P11", "P12", "P13", "P14", "P15", "P16"):
    assert f"`<<{tag}>>`" in inlist, (tag, "item 3's record lost the name")
assert inlist == " ".join(ORIG[I3:I4]), "item 3's list changed"
before = set(ORIG)
long = [k + 1 for k, x in enumerate(L) if len(x) > 120 and x not in before and not x.lstrip().startswith("|")]
assert not long, ("changed lines over 120 characters", [(k, L[k - 1]) for k in long])
for i in range(H10, H11):                                             # section 10 whole
    assert ORIG[i] in L, ("section 10 changed", i + 1)
s10n = head("## 10. Questions for the advisor", L)
assert L[s10n:s10n + (H11 - H10)] == ORIG[H10:H11], "section 10 changed"
assert L[:H0] == ORIG[:H0], "the top (title, records, change lists) changed"
LOG.append(f"words check: {len(FLAT)} changes; rev 3.2's words with only these made equal the result's; each new "
           f"phrase once; each replaced phrase gone; no <<P and a digit outside item 3's list (the intro's `<<P...>>` "
           f"at :{gen[0]} names the form); item 3's list, section 10 and the top unchanged; no changed line over 120 "
           "(rows exempt)")
# where each copy and fill stands now
for tag, new in NEWS:
    mm = re.search(ws(new), final)
    WHERE[tag] = (final.count("\n", 0, mm.start()) + 1, final.count("\n", 0, mm.end()) + 1)
LOG.append("in the result: " + "; ".join(f"{t} :{a}" + (f"-{b}" if b != a else "") for t, (a, b) in WHERE.items()
                                         if t.startswith(("P", "R11", "tally", "stamp"))))
print("\n".join(LOG))
print("the main session's words (no rule or ruling gives them word for word):")
print("\n".join(f"- {x}" for x in MAIN))
print(f"brief sha256 before {sha_b(read_b(BRIEF))}; lines {len(ORIG) - 1} -> {len(L) - 1}; sha256 after "
      f"{sha_b(final.encode())}")
sys.stdout.writelines(difflib.unified_diff([x + "\n" for x in ORIG], [x + "\n" for x in L], BRIEF, BRIEF, n=1))
if APPLY:
    with open(BRIEF, "w", encoding="utf-8") as fh:
        fh.write(final)
    print(f"applied; brief sha256 after {sha_b(read_b(BRIEF))}")
else:
    print("dry run: nothing written")
