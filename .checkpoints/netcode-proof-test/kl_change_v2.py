"""KL-CHANGE v2: the advisor's condition C1 (advisor-kl-change-full.md, conditions 1-3) on top of KL-CHANGE v1.
1. Re-extracts the three posted kit texts (kept-logs-final-text.md, sha256 97c81656...) and asserts they equal the
   scratch before/ copies byte for byte; asserts v1's after/ copies are the ones checked (their sha256 prefixes).
2. Applies C1 to v1's test file: the new test's two name lines, and three lines after the late-link assertion. Every
   C1 line is asserted to appear word for word in the advisor's condition text in the workflow's journal.
3. Writes kl-change-v2/ in the scratchpad (before, after, mutants M4, M12, M13, M14; M14's exact replacement also
   asserted against the condition text) and kl-change-v2.diff in the notes folder, and checks that the v2 diff differs
   from kl-change.diff only by C1's lines and the hunk headers they move.
Refuses to run twice.  Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/kl_change_v2.py"""
import difflib
import hashlib
import json
import os
import shutil

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
S1 = "/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/scratchpad/kl-change"
S2 = S1 + "-v2"
DIFF1, DIFF2 = f"{N}/kl-change.diff", f"{N}/kl-change-v2.diff"
J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_d7fbca53-76d/journal.jsonl")
assert not os.path.exists(DIFF2) and not os.path.exists(S2), "already ran"
sha = lambda s: hashlib.sha256(s.encode()).hexdigest()

with open(f"{N}/kept-logs-final-text.md", "rb") as fh:
    raw = fh.read()
assert hashlib.sha256(raw).hexdigest().startswith("97c8165655869d9a")
text = raw.decode("utf-8")


def block(name, fence):
    a = text.index(f"<!-- BEGIN {name} -->\n{fence}\n") + len(f"<!-- BEGIN {name} -->\n{fence}\n")
    return text[a:text.index(f"```\n<!-- END {name} -->", a)]


NAMES = ("checkpoints-11.txt", "kept-logs.mjs", "kept-logs.test.mjs")
posted = dict(zip(NAMES, (block("checkpoints-11", "```"), block("kept-logs.mjs", "```js"),
                          block("kept-logs.test.mjs", "```js"))))
read = lambda p: open(p, encoding="utf-8").read()
for n in NAMES:
    assert read(f"{S1}/before/{n}") == posted[n], f"before/{n} is not the posted block"
v1 = {n: read(f"{S1}/after/{n}") for n in NAMES}
for n, p in zip(NAMES, ("2be7e0a7632c7612", "c9109ab42a0c84c9", "b661911365aafc58")):
    assert sha(v1[n]).startswith(p), f"after/{n} is not the checked v1"

with open(J, encoding="utf-8") as fh:
    rows = [json.loads(l) for l in fh if l.strip()]
cond = [r["result"] for r in rows if r["type"] == "result"][0]["conditions"][0]
OLD_NAME = ("test('a folder that changes after the read-back, before it would go: the second walk fails (1) on a late file and on ' +\n"
            "  'a late link; the folder and the unfinished archive stay', (t) => {\n")
NEW_NAME = ["test('a folder that changes after the read-back, before it would go: the second walk fails (1) on a late file, a ' +",
            "  'late link and a grown file; the folder and the unfinished archive stay', (t) => {"]
AFTER = "  assert.ok(lstatSync(join(b.set, 'late-link')).isSymbolicLink(), 'the link stays, never followed');\n"
ADD = ["  const c = packWith(`fs.appendFileSync(${JSON.stringify(`${RUNS}/${SET}/marks.json`)}, 'late')`);",
       r"  assert.match(c.p.stderr, /MISMATCH[\s\S]*changed while it was packed[\s\S]*the folder stays/);",
       r"  assert.equal(readFileSync(join(c.set, 'marks.json'), 'utf8'), '{}\nlate', 'the grown file stays as it is');"]
M14 = ("    if (differs(walk(dir, set, (msg) => { throw new Mismatch(msg); }), rows)) {",
       "    if (walk(dir, set, (msg) => { throw new Mismatch(msg); }).length !== rows.length) {")
for line in NEW_NAME + ADD + [AFTER.rstrip("\n"), *M14]:
    assert line.strip() in cond, f"not word for word in the condition: {line[:70]}"

v2 = dict(v1)
t = v2["kept-logs.test.mjs"]
assert t.count(OLD_NAME) == 1 and t.count(AFTER) == 1
t = t.replace(OLD_NAME, "\n".join(NEW_NAME) + "\n").replace(AFTER, AFTER + "\n".join(ADD) + "\n")
v2["kept-logs.test.mjs"] = t
script = v2["kept-logs.mjs"]
SECOND = ("    if (differs(walk(dir, set, (msg) => { throw new Mismatch(msg); }), rows)) {\n"
          "      throw new Mismatch(`the folder changed while it was packed: its files or sizes now differ from ${listRel}`);\n"
          "    }\n")
THROWER = "walk(dir, set, (msg) => { throw new Mismatch(msg); })"
for old in (SECOND, THROWER, M14[0] + "\n", "  if (differs(now, rows)) {\n"):
    assert script.count(old) == 1, old[:60]
mutants = {"M4": script.replace("  if (differs(now, rows)) {\n", "  if (false) {\n"),
           "M12": script.replace(SECOND, ""), "M13": script.replace(THROWER, "walk(dir, set)"),
           "M14": script.replace(M14[0] + "\n", M14[1] + "\n")}
for d, files in (("before", posted), ("after", v2)):
    os.makedirs(f"{S2}/{d}")
    for n, body in files.items():
        with open(f"{S2}/{d}/{n}", "w", encoding="utf-8") as fh:
            fh.write(body)
for m, body in mutants.items():
    os.makedirs(f"{S2}/mutants/{m}")
    with open(f"{S2}/mutants/{m}/kept-logs.mjs", "w", encoding="utf-8") as fh:
        fh.write(body)

out = []
for n, path in zip(NAMES, (".claude/CHECKPOINTS.md (§11)", ".claude/scripts/kept-logs.mjs",
                           ".claude/scripts/kept-logs.test.mjs")):
    out += difflib.unified_diff(posted[n].splitlines(keepends=True), v2[n].splitlines(keepends=True),
                                fromfile=f"a/{path} (as posted)", tofile=f"b/{path} (with KL-CHANGE)", n=1)
d2 = "".join(out)
with open(DIFF2, "w", encoding="utf-8") as fh:
    fh.write(d2)
d1 = read(DIFF1)
gone, came = [], []
for l in difflib.ndiff(d1.splitlines(), d2.splitlines()):
    if l.startswith("- "):
        gone.append(l[2:])
    elif l.startswith("+ "):
        came.append(l[2:])
ok_gone = {"+" + x for x in OLD_NAME.rstrip("\n").split("\n")}
ok_came = {"+" + x for x in NEW_NAME + ADD}
assert all(x.startswith("@@") or x in ok_gone for x in gone), gone
assert all(x.startswith("@@") or x in ok_came for x in came), came
assert ok_came <= set(came), "a C1 line is missing from the v2 diff"
print(f"v2 diff vs v1 diff: removed {len(gone)} lines (hunk headers and the two old name lines), added {len(came)} "
      "(hunk headers, the two new name lines, the three new lines): only C1")
for n in NAMES:
    print(f"{n}: {posted[n].count(chr(10))} -> {v2[n].count(chr(10))} lines; widest "
          f"{max(len(l) for l in v2[n].split(chr(10)))}; sha256 {sha(v2[n])[:16]}")
print(f"{DIFF2}: {d2.count(chr(10))} lines, sha256 {sha(d2)[:16]}")
