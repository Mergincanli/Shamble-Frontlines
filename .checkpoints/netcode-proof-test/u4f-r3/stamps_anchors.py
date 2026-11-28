# Read-only: prints sha256 and line counts, and how many times each placeholder anchor occurs (with its line).
# Run from the repository root: python3 -I .checkpoints/netcode-proof-test/u4f-r3/stamps_anchors.py
import hashlib
import pathlib

N = pathlib.Path(".checkpoints/netcode-proof-test")

STAMPS = [
    N / "u4-build-plan.md",
    N / "plan.md",
    N / "u4s-carries.md",
    N / "history.md",
    N / "u4r-run.sh",
    N / "main-run.sh",
    N / "u4r-report.md",
    N / "advisor-u4df.md",
    N / "advisor-u4j-r2.md",
    N / "advisor-u4d2-m9.md",
    N / "advisor-u4d2-r2.md",
    N / "advisor-te-m3.md",
    N / "advisor-u4r-c62h-full.md",
    N / "advisor-brawl1-sets-p3-full.md",
    N / "advisor-u4r-build2-full.md",
    N / "advisor-u4r-r1-full.md",
    N / "advisor-c62h-bound-head-full.json",
    N / "advisor-c62h-bound-merge-stop-full.json",
    N / "advisor-c62h-review-full.md",
    pathlib.Path("spikes/netcode-proof/marks/marks.json"),
    pathlib.Path("spikes/netcode-proof/marks/SERIES.md"),
    pathlib.Path("spikes/netcode-proof/ops/runs/c-u4r-runs.mjs"),
    pathlib.Path("spikes/netcode-proof/ops/runs/c-u4r-r2.mjs"),
]

# (id, file, start anchor, end anchor or None). The start is matched as a substring; its count over the file is
# printed (it must be 1). The end is the first occurrence at or after the start's line.
ANCHORS = [
    ("P1", "u4s-carries.md", "## U4f (clock and commands)", "## U4g (prediction, interpolation and events)"),
    ("P2", "u4s-carries.md", "Each entry: the finding (review, id), then", "never the rule."),
    ("P3", "u4s-carries.md", "57. **U4E-Q6, the room's half", "(carry\n"),
    ("P4", "u4s-carries.md", "74. **NC-M6, the endpoint's lead", "before its marks count.\""),
    ("P5", "u4s-carries.md", "21. **netcode F2 (the room's half):**", "-30's comparer relies on it.\""),
    ("P6", "u4s-carries.md", "27. **this pass (CMD_CATCHUP):**", "tests included.\""),
    ("P7", "u4s-carries.md", "18. **this pass (the tick grep):**", "review keeps the two lists equal).\""),
    ("P8", "u4s-carries.md", "39. **rust fix review nit (u4s-fix-review-rust.md:114-115", "follow it.)"),
    ("P9", "u4s-carries.md", "50. **the event age", "A unit\n"),
    ("P10", "u4s-carries.md", "61. **U4j's hash rules, the room and the client", "files one hash.\""),
    ("P11", "u4s-carries.md", "58. **U4E-Q6's two conditions", "changes -18's heal path.\""),
    ("P12", "u4s-carries.md", "76. **A6 and SEC-U4E-5, ending a session", "reach no room call.\""),
    ("P13", "u4s-carries.md", "77. **U4r's readings of U4e's room and client", "comes to the advisor.\""),
    ("P14", "history.md", "  - Builders: pass or fail comes from the exit code", "before it changes anything."),
    ("P15", "history.md", "A new bar, or a change to one, comes to me under Decisions needed", "on the ruling alone."),
    ("P16", "history.md", "  From now on: every change to marks.json or to the judge gets a reviewer's read", "what covered it."),
    ("P17", "history.md", "write commands in the simple forms that need no approval", "before you send it."),
    ("P18", "history.md", "  2. SIZE-CHECK from now on. My rule of 2026-11-10 was not applied to U4r.", "as the rule says."),
    ("P19", "history.md", "  C62H-KEEP: (a). The check stays as built.", None),
    ("P20", "history.md", "  C62H-BOUND: (a), the exact limit.", "stay in until it is built."),
    ("P21", "history.md", "  - The stop rules of your post stand. The check is never reworded to fit.", None),
    ("P22", "history.md", "      - (a) Yes. I may pack one or two closed, older log folders", "or the c-u2 folders."),
    ("P23", "history.md", "  - **C62HB-Q4:** should the counter-wrap test (PM-0001-58)", "I recommend (b)."),
    ("P24", "advisor-u4df.md", "Write files with the\n", "rewrites its notes file whole with Write.\""),
    ("P25", "advisor-u4df.md", "\"a failure in <crate>, or in a test that", "ends it, and go on.\""),
    ("P26", "advisor-u4j-r2.md", "From now on: ", None),
    ("P27", "advisor-u4d2-m9.md", "Between waves, with no lane running", "never `--target` without `-p`."),
    ("P28", "advisor-u4d2-m9.md", "(1) never pass `--target`", "no session removes it by hand."),
    ("P29", "advisor-u4d2-r2.md", "never run cargo", "may use it as one."),
    ("P30", "advisor-te-m3.md", "After it ends, FX-10's saved output", "The card's \"Rules in force\" says so."),
    ("P31", "advisor-u4r-c62h-full.md", "Until the fix lands, the rule is:", "as both run scripts already do."),
    ("P32", "advisor-brawl1-sets-p3-full.md", "\"With everyone in view, every client queues", "judged B1 runs count.\""),
    ("P33", "advisor-u4r-build2-full.md", "Adds a card follow-up for the next unit whose lane opens npt-server", "No push for it."),
    ("P34", "advisor-u4r-r1-full.md", "Contract half: a STATUS follow-up for the next unit that edits", "a usage error (exit 2)."),
    ("P35", "u4r-report.md", "- **TE-B1-R2-1 with RR2-B1-1**", "- **TE-B1-R2-2**"),
    ("P36", "u4r-report.md", "  - RR-B1-7 (1), the room_tick loop rewrite", None),
]


def sha(p):
    b = p.read_bytes()
    return hashlib.sha256(b).hexdigest(), b.count(b"\n")


for p in STAMPS:
    h, n = sha(p)
    print(f"stamp {p}: {h} {n} lines")

for pid, name, start, end in ANCHORS:
    text = (N / name).read_text(encoding="utf-8")
    count = text.count(start)
    first = text.find(start)
    line = text.count("\n", 0, first) + 1 if first >= 0 else 0
    end_line = "-"
    if end is not None and first >= 0:
        e = text.find(end, first)
        end_line = str(text.count("\n", 0, e) + 1) if e >= 0 else "NOT FOUND"
    print(f"{pid} {name}: start found {count} time(s), at line {line}; end at line {end_line}")
