"""Write U4r round 2's packet config, u4r-r2-packet.json, for review_packet.py: facts and sources only (Mustafa's
USAGE-SAVINGS (a): no conclusion of the builder or the main session; builder notes, briefs, the card and summaries are
never named). Round 2 reviews only what changed (the advisor's ruling on round 1, "Round 2"): the files that differ
between the read copy of snapshot 572c4f69 (.gate/u4r-snap1) and that of snapshot 21ca46be (.gate/u4r-snap2), found by
comparing the two folders here, not typed; FP-0's two runner diffs; the ruling; round 1's findings; Mustafa's words of
2026-11-14 (his verbatim lines only, from history.md by line range, the label and closing lines asserted); and the
carries the ruling names (their numbers read from the ruling's text here).
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4r_r2_packet_config.py"""
import filecmp
import hashlib
import json
import os
import re

M = "/home/user/Shamble-Frontlines"
N = ".checkpoints/netcode-proof-test"
S1, S2 = ".gate/u4r-snap1/spikes/netcode-proof", ".gate/u4r-snap2/spikes/netcode-proof"
WT = ".claude/worktrees/agent-a11e2354169f39acf"
OLD, NEW = ("572c4f69435da2ad6f62d54d4bcbc47b04d6329b5493e0faf78fcecfdc4a09c2",
            "21ca46be23704131c77cc20981a4bfe866811b33f9ca853279f7ddbfb5d58bce")


def sha(path):
    with open(os.path.join(M, path), "rb") as fh:
        return hashlib.sha256(fh.read()).hexdigest()


assert sha(f"{N}/wip/u4r.patch") == NEW, "wip/u4r.patch is not snapshot 21ca46be"
RUNNER = sha(f"{N}/u4r-run.sh")
assert RUNNER.startswith("6efb9157"), RUNNER

# The files that differ between the two read copies (every file under each spikes/netcode-proof, .git excluded).
def files(root):
    out = set()
    for d, dirs, fs in os.walk(os.path.join(M, root)):
        dirs[:] = [x for x in dirs if x not in (".git", "target", "node_modules")]
        out |= {os.path.relpath(os.path.join(d, f), os.path.join(M, root)) for f in fs}
    return out


f1, f2 = files(S1), files(S2)
changed = sorted(f for f in f1 | f2 if f not in f1 or f not in f2
                 or not filecmp.cmp(os.path.join(M, S1, f), os.path.join(M, S2, f), shallow=False))
assert len(changed) == 14, changed
SIDE = {"check.test.mjs", "crates/server/tests/schedule.rs"}
diffs = []
for f in changed:
    a = f"{S1}/{f}" if f in f1 else "/dev/null"
    b = f"{S2}/{f}" if f in f2 else "/dev/null"
    diffs.append({"label": f"spikes/netcode-proof/{f}", "a": a, "b": b, "side": f in SIDE})
HELPER = f"{WT}/{N}/u4r-fp5-helpers.test.mjs"
assert os.path.isfile(os.path.join(M, HELPER)), HELPER
diffs.append({"label": "FP-5's helper test (a scratch file in the builder worktree's notes folder, never committed; "
              "the ruling's FP-5 EVIDENCE)", "a": "/dev/null", "b": HELPER, "side": True})

logs = sorted(x for x in os.listdir(os.path.join(M, ".gate/u4r")) if x.startswith("c6-"))
plants = [x for x in logs if x.startswith("c6-plant-")]
heads = [
    f"The snapshot reviewed in round 1: `{N}/wip/u4r.patch` sha256 {OLD} (26 files on 2fbec77; committed at b084998). "
    "Its read copy `.gate/u4r-snap1`: a detached worktree at 2fbec77 with that patch applied.",
    f"The snapshot under review: `{N}/wip/u4r.patch` sha256 {NEW} (26 files on 2fbec77; committed at 450af1a). Its "
    "read copy `.gate/u4r-snap2`: a detached worktree at 2fbec77 with that patch applied; each of its 26 files equals "
    f"the builder worktree's (`{WT}`, HEAD 2fbec77, the work uncommitted) by cmp.",
    f"The diffs below compare `{S1}` with `{S2}` file by file: the {len(changed)} files that differ, found by comparing "
    "the two folders. The fix pass added no file under spikes/ and removed none.",
    "Counts at snapshot 572c4f69 (round 1's head): measured build 526 tests passed, bites build 603, self 76 of 76; "
    "check.mjs 11 steps.",
    "The fix pass's runs, all through the lane's runner `.checkpoints/netcode-proof-test/u4r-run.sh` (sha256 "
    f"{RUNNER}, after FP-0) in the builder worktree, logs under `.gate/u4r/`: " + ", ".join(logs) + ".",
    "Among them, the check.mjs runs: c6-end1.log, c6-end2.log, c6-end3.log; FP-8's runs: c6-det-u4.log and "
    "c6-regen-precheck.log (round 1's det-u4.log and regen-precheck.log are older); the plant runs: "
    + ", ".join(plants) + ".",
    "FP-0's two runner diffs are copied below: the runners are outside the patch, and the main session edited them "
    "(each diff is `git diff` of the runner against its committed version).",
    f"Round 1's packet, with the approved plan's lines, marks.json's and SERIES.md's rows, the catalogue and the carries "
    f"as round 1 had them: `{N}/u4r-packet.md` and `{N}/u4r-packet-tests.md`.",
]

with open(os.path.join(M, N, "history.md"), encoding="utf-8") as fh:
    hist = fh.read().split("\n")
first = next(i for i, l in enumerate(hist) if l.startswith('- **Mustafa, 2026-11-14 about 11:07 UTC, after his '
                                                              '/compact (verbatim):** "'))
last = next(i for i in range(first, len(hist)) if hist[i].endswith('Record these words in history.md under my '
                                                                   'decisions."'))
assert hist[last + 1] == "  - Read as (the main session):", hist[last + 1]
docs = [
    {"label": "FP-0: u4r-run.sh against its committed version", "path": f"{N}/u4r-run.sh.inf-u4r-4.diff"},
    {"label": "FP-0: main-run.sh against its committed version", "path": f"{N}/main-run.sh.inf-u4r-4.diff"},
    {"label": "The advisor's ruling on round 1: the fix pass, round 2 and each finding", "path":
     f"{N}/advisor-u4r-r1-full.md"},
    {"label": "Round 1's findings, the six seats' results word for word", "path": f"{N}/u4r-review-r1-full.md"},
    {"label": "Mustafa's words of 2026-11-14 (his SIZE-U4R answer (a) sets the fix pass's scope)", "path":
     f"{N}/history.md", "lines": [first + 1, last + 1], "words_of_mustafa": True},
]
with open(os.path.join(M, N, "advisor-u4r-r1-full.md"), encoding="utf-8") as fh:
    nums = sorted({int(n) for n in re.findall(r"\b[Cc]arr(?:y|ies) (\d+)", fh.read())})
assert nums, "no carry named"
cfg = {
    "title": "U4r round 2 (only what changed since snapshot 572c4f69: the fix pass FP-1 to FP-9 of the advisor's "
             "ruling on round 1, and FP-0's two runner diffs)",
    "out": f"{N}/u4r-r2-packet.md",
    "side_out": f"{N}/u4r-r2-packet-tests.md",
    "heads": heads,
    "diffs": diffs,
    "docs": docs,
    "carries": {"path": f"{N}/u4s-carries.md",
                "include": "^(" + "|".join(str(n) for n in nums) + r")\. \*\*"},
}
out = os.path.join(M, N, "u4r-r2-packet.json")
with open(out, "w", encoding="utf-8") as fh:
    json.dump(cfg, fh, indent=1, ensure_ascii=False)
    fh.write("\n")
print(f"{out}: {len(heads)} heads, {len(diffs)} diffs ({sum(d['side'] for d in diffs)} side), {len(docs)} docs "
      f"(his words: history.md lines {first + 1}-{last + 1}), carries named by the ruling: {nums}")
