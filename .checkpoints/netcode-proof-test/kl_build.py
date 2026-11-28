"""KEPT-LOGS build, step 1 (the advisor's C7; KL-CHANGE's build_plan_changes 3): rebuild the three kit files and the two
map rows by script from the text Mustafa said yes to (kept-logs-final-text.md, sha256 97c81656...) plus the KL-CHANGE
diff he said yes to (kl-change-v2.diff, sha256 c80dbbb3...; his KL-CHANGE-YES, history.md:2148-2161). Each hunk is
applied as one exact replacement that must match once, at the line its header names.
Writes the expected files, in the repository's layout, under the session scratchpad's kl-build/expected/:
  .claude/CHECKPOINTS.md              main's 229 lines (4460ef5), one blank line, then §11
  .claude/scripts/kept-logs.mjs       the script
  .claude/scripts/kept-logs.test.mjs  the tests
  docs/REPO_MAP.md                    main's, with the two rows after the checkpoint.test.mjs row
Checks: §11 19 lines, the script 323, the tests 326, widest line at most 120 characters; the rebuilt files equal the
checked scratch set kl-change-v2/after/ byte for byte. Writes nothing in the repository. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/kl_build.py"""
import hashlib
import os
import re
import subprocess

REPO = "/home/user/Shamble-Frontlines"
N = f"{REPO}/.checkpoints/netcode-proof-test"
SCR = "/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/scratchpad"
OUT = f"{SCR}/kl-build/expected"
V2 = f"{SCR}/kl-change-v2/after"
BASE = "4460ef5b8d6dc8566b29c15ded75c6f0c8be386b"
TEXT_SHA = "97c8165655869d9a7747d38df6899cd48797da8f02688ae8c1ed34168f1d0092"
DIFF_SHA = "c80dbbb346459d46f145d09ee711c79edce63666a99f9cc172c93ea87ab25ebe"
assert not os.path.exists(f"{SCR}/kl-build"), "already ran"
sha = lambda b: hashlib.sha256(b).hexdigest()

raw = open(f"{N}/kept-logs-final-text.md", "rb").read()
assert sha(raw) == TEXT_SHA, "kept-logs-final-text.md is not the text he said yes to"
text = raw.decode("utf-8")
draw = open(f"{N}/kl-change-v2.diff", "rb").read()
assert sha(draw) == DIFF_SHA, "kl-change-v2.diff is not the diff he said yes to"
dlines = draw.decode("utf-8").splitlines(keepends=True)


def block(name, fence):
    a = text.index(f"<!-- BEGIN {name} -->\n{fence}\n") + len(f"<!-- BEGIN {name} -->\n{fence}\n")
    assert text.count(f"<!-- BEGIN {name} -->") == 1
    return text[a:text.index(f"```\n<!-- END {name} -->", a)]


posted = {"checkpoints-11": block("checkpoints-11", "```"), "kept-logs.mjs": block("kept-logs.mjs", "```js"),
          "kept-logs.test.mjs": block("kept-logs.test.mjs", "```js")}
rows = block("repo-map-rows", "```")
PATHS = {".claude/CHECKPOINTS.md (§11)": "checkpoints-11", ".claude/scripts/kept-logs.mjs": "kept-logs.mjs",
         ".claude/scripts/kept-logs.test.mjs": "kept-logs.test.mjs"}

# Parse the diff: per file, its hunks, each consuming exactly the line counts its header names.
hunks, i, cur = {k: [] for k in posted}, 0, None
HEAD = re.compile(r"^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@\n$")
while i < len(dlines):
    l = dlines[i]
    if l.startswith("--- a/"):
        m = re.match(r"^--- a/(.+) \(as posted\)\n$", l)
        n2 = re.match(r"^\+\+\+ b/(.+) \(with KL-CHANGE\)\n$", dlines[i + 1])
        assert m and n2 and m.group(1) == n2.group(1) and m.group(1) in PATHS, (l, dlines[i + 1])
        cur, i = PATHS[m.group(1)], i + 2
        continue
    m = HEAD.match(l)
    assert m and cur, f"diff line {i + 1} is not a hunk header: {l!r}"
    a, b = int(m.group(1)), int(m.group(2) or 1)
    c, d = int(m.group(3)), int(m.group(4) or 1)
    old, new, i = [], [], i + 1
    while len(old) < b or len(new) < d:
        l = dlines[i]
        tag, body = l[0], l[1:]
        assert tag in " -+", f"diff line {i + 1}: {l!r}"
        if tag in " -":
            old.append(body)
        if tag in " +":
            new.append(body)
        i += 1
    assert len(old) == b and len(new) == d
    hunks[cur].append((a, "".join(old), "".join(new)))

built = {}
for name, src in posted.items():
    out = src
    for a, old, new in hunks[name]:
        assert old, "an empty old side"
        assert src.count(old) == 1 and src[:src.index(old)].count("\n") == a - 1, f"{name}: hunk @{a} not once at its line"
        assert out.count(old) == 1, f"{name}: hunk @{a} does not match once in the text being built"
        out = out.replace(old, new)
    built[name] = out
    print(f"{name}: {len(hunks[name])} hunks applied, each matched once at its header's line")

want = {"checkpoints-11": 19, "kept-logs.mjs": 323, "kept-logs.test.mjs": 326}
for name, body in built.items():
    lines = body.split("\n")
    assert body.endswith("\n") and body.count("\n") == want[name], (name, body.count("\n"))
    assert max(len(x) for x in lines) <= 120, name
    v2 = open(f"{V2}/{name if name != 'checkpoints-11' else 'checkpoints-11.txt'}", "rb").read()
    assert v2 == body.encode("utf-8"), f"{name} differs from the checked scratch set kl-change-v2/after/"
    print(f"{name}: {body.count(chr(10))} lines, widest {max(len(x) for x in lines)}, equal to kl-change-v2/after/")

show = lambda p: subprocess.run(["git", "show", f"{BASE}:{p}"], cwd=REPO, capture_output=True, check=True).stdout
cp_main = show(".claude/CHECKPOINTS.md").decode("utf-8")
assert cp_main.endswith("\n") and cp_main.count("\n") == 229, cp_main.count("\n")
map_main = show("docs/REPO_MAP.md").decode("utf-8")
ROW = "| `.claude/scripts/checkpoint.test.mjs` | Its tests: `node --test .claude/scripts/checkpoint.test.mjs` |\n"
assert map_main.count(ROW) == 1 and rows.count("\n") == 2
files = {".claude/CHECKPOINTS.md": cp_main + "\n" + built["checkpoints-11"],
         ".claude/scripts/kept-logs.mjs": built["kept-logs.mjs"],
         ".claude/scripts/kept-logs.test.mjs": built["kept-logs.test.mjs"],
         "docs/REPO_MAP.md": map_main.replace(ROW, ROW + rows)}
assert files[".claude/CHECKPOINTS.md"].count("\n") == 249
for p, body in files.items():
    os.makedirs(os.path.dirname(f"{OUT}/{p}"), exist_ok=True)
    with open(f"{OUT}/{p}", "w", encoding="utf-8", newline="") as fh:
        fh.write(body)
    with open(f"{OUT}/{p}", "rb") as fh:
        data = fh.read()
    assert data == body.encode("utf-8")
    print(f"expected {p}: {body.count(chr(10))} lines, {len(data)} bytes, sha256 {sha(data)}")
print(f"written under {OUT}; base {BASE}")
