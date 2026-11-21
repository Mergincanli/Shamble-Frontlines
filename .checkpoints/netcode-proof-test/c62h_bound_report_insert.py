"""Append the C62H-BOUND report section to u4r-report.md, word for word from c62h-bound-report-draft.md (the advisor's
P3 ruling, order item 4: "a section of u4r-report.md, as BRAWL-1's was"). It goes at the file's end, after BRAWL-1's
section, with one blank line before it, so the lines the section cites (:997, :1008, :1035, :1167) stay where they are.
The draft's bytes are bound to the verified version: its sha256 is the one argument (exit 1 on any difference).
Before writing: the report's sha256 is the (c) script's "after" (777ee782...), the section is not there yet, and the
four cited lines are as the draft names them. After: the old report is an exact prefix of the new one, the new text is
old + one blank line + the draft, each cited line unchanged, and the section heading is there once.
Run from the repository root:  python3 -I .checkpoints/netcode-proof-test/c62h_bound_report_insert.py <draft-sha256>"""
import hashlib
import sys

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
R = f"{N}/u4r-report.md"
D = f"{N}/c62h-bound-report-draft.md"
R_BEFORE = "777ee782611c5eaca4f679e83bd5ccaacde7b00887d2ac475243cc1753bf7f66"
HEADING = "## C62H-BOUND (every lost tick on L1 P2, P3 and B1 explained by the room's or the link's record)"


def stop(msg):
    print(f"c62h_bound_report_insert: STOP, nothing written: {msg}")
    sys.exit(1)


if len(sys.argv) != 2 or len(sys.argv[1]) != 64:
    stop("give the verified draft's full sha256 as the one argument")
raw_r = open(R, "rb").read()
raw_d = open(D, "rb").read()
if hashlib.sha256(raw_r).hexdigest() != R_BEFORE:
    stop(f"u4r-report.md is sha256 {hashlib.sha256(raw_r).hexdigest()}, not the expected {R_BEFORE}")
if hashlib.sha256(raw_d).hexdigest() != sys.argv[1]:
    stop(f"the draft is sha256 {hashlib.sha256(raw_d).hexdigest()}, not the verified {sys.argv[1]}")
rep = raw_r.decode("utf-8")
dra = raw_d.decode("utf-8")
if not rep.endswith("\n") or rep.endswith("\n\n"):
    stop("u4r-report.md does not end with exactly one newline")
if not dra.startswith(HEADING + "\n") or not dra.endswith("\n") or dra.endswith("\n\n"):
    stop("the draft does not start with the section heading or does not end with exactly one newline")
if HEADING in rep or "## C62H-BOUND" in rep:
    stop("the section is already in u4r-report.md")
old = rep.split("\n")
cited = {997: "Bounded by C62H-BOUND", 1008: "Bounded by C62H-BOUND", 1035: "-17 L1 B1 stays with U4g's judged B1 runs",
         1167: "-17 L1 B1"}
for n, frag in cited.items():
    if frag not in old[n - 1]:
        stop(f"u4r-report.md:{n} does not carry {frag!r}: {old[n - 1][:100]!r}")
new = rep + "\n" + dra
if not new.startswith(rep) or new[len(rep):] != "\n" + dra:
    stop("the new text is not the old report + one blank line + the draft")
nl = new.split("\n")
if [nl[n - 1] for n in cited] != [old[n - 1] for n in cited]:
    stop("a cited line moved")
if new.count(HEADING) != 1:
    stop("the heading is not there exactly once")
if len(nl) != len(old) + 1 + len(dra.split("\n")) - 1:
    stop(f"line count {len(nl)} is not old {len(old)} + 1 + the draft's {len(dra.split(chr(10))) - 1}")
out = new.encode("utf-8")
open(R, "wb").write(out)
print(f"u4r-report.md: {len(old) - 1} lines, sha256 {R_BEFORE} -> {len(nl) - 1} lines, sha256 "
      f"{hashlib.sha256(out).hexdigest()} (the section from line {len(old) + 1}; draft sha256 {sys.argv[1]})")
