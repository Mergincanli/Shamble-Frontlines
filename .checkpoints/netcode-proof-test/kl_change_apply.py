"""KL-CHANGE (Mustafa's KL-YES of 2026-11-15 16:35, history.md:2131-2146): pack walks the set folder once more after
the read-back equals the list and before the folder is removed; files or sizes that differ keep the folder and exit 1,
as a mismatch does; a test and planted bugs for it.
This script takes the three kit texts exactly as posted (kept-logs-final-text.md, sha256 97c81656..., its BEGIN/END
blocks), writes them to the session scratchpad as before/, applies the change by exact replacements (each must match
once) into after/, writes the planted-bug copies M12 and M13 into mutants/, and writes the unified diff that is posted
to Mustafa into kl-change.diff (the notes folder). Nothing under .claude/ or anywhere in the repository but that diff
is written. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/kl_change_apply.py"""
import difflib
import hashlib
import os

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
SP = "/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/scratchpad/kl-change"
DIFF = f"{N}/kl-change.diff"
assert not os.path.exists(DIFF) and not os.path.exists(SP), "already ran"
with open(f"{N}/kept-logs-final-text.md", "rb") as fh:
    raw = fh.read()
assert hashlib.sha256(raw).hexdigest().startswith("97c8165655869d9a"), "not the posted text"
text = raw.decode("utf-8")


def block(name, fence):
    a = text.index(f"<!-- BEGIN {name} -->\n{fence}\n") + len(f"<!-- BEGIN {name} -->\n{fence}\n")
    b = text.index(f"```\n<!-- END {name} -->", a)
    return text[a:b]


before = {"checkpoints-11.txt": block("checkpoints-11", "```"), "kept-logs.mjs": block("kept-logs.mjs", "```js"),
          "kept-logs.test.mjs": block("kept-logs.test.mjs", "```js")}

R = [
    ("checkpoints-11.txt",
     "  `.gate/netcode-proof-test/kept-logs/`, reads every file's hash back from it, and removes the folder only if every\n"
     "  hash equals the list. If one differs, or the folder no longer matches its list, the folder stays: tell Mustafa, and\n"
     "  pack nothing more until he answers.\n",
     "  `.gate/netcode-proof-test/kept-logs/`, reads every file's hash back from it, walks the folder once more, and removes\n"
     "  the folder only if every hash equals the list and the folder still holds exactly the list's files and sizes. If one\n"
     "  differs, or the folder no longer matches its list, the folder stays: tell Mustafa, and pack nothing more until he\n"
     "  answers.\n"),
    ("kept-logs.mjs",
     "//        and hashes it; and removes the folder only if every path, size and sha256 equals the list. If one differs,\n"
     "//        the folder stays, and so does the unfinished archive (<archive>.part), and the script exits 1: tell Mustafa.\n",
     "//        and hashes it; walks the folder once more; and removes the folder only if every path, size and sha256 read\n"
     "//        back equals the list and the folder still holds exactly its files and sizes. If one differs, the folder\n"
     "//        stays, and so does the unfinished archive (<archive>.part), and the script exits 1: tell Mustafa.\n"),
    ("kept-logs.mjs",
     "// Every file under the set folder as { path, bytes }, sorted by path. Anything but plain files and folders refuses.\n"
     "function walk(dir, set) {\n",
     "// Every file under the set folder as { path, bytes }, sorted by path. Anything but plain files and folders goes to\n"
     "// `fail`: a refusal (exit 2) before any work; on pack's second walk, a mismatch (exit 1).\n"
     "function walk(dir, set, fail = refuse) {\n"),
    ("kept-logs.mjs",
     "        refuse(`${set}/${path}: not a plain name of at most 99 characters`);\n",
     "        fail(`${set}/${path}: not a plain name of at most 99 characters`);\n"),
    ("kept-logs.mjs",
     "      else refuse(`${set}/${path} is not a plain file under 8 GiB (a symbolic link, a device or a socket stays)`);\n",
     "      else fail(`${set}/${path} is not a plain file under 8 GiB (a symbolic link, a device or a socket stays)`);\n"),
    ("kept-logs.mjs",
     "  if (!files.length) refuse(`${set} holds no files`);\n"
     "  return files.sort((a, b) => (a.path < b.path ? -1 : a.path > b.path ? 1 : 0));\n"
     "}\n",
     "  if (!files.length) fail(`${set} holds no files`);\n"
     "  return files.sort((a, b) => (a.path < b.path ? -1 : a.path > b.path ? 1 : 0));\n"
     "}\n"
     "// True when a walk's files or sizes are not exactly the list's.\n"
     "const differs = (files, rows) => files.length !== rows.length ||\n"
     "  files.some((f, i) => f.path !== rows[i].path || f.bytes !== rows[i].bytes);\n"),
    ("kept-logs.mjs",
     "  if (now.length !== rows.length || now.some((f, i) => f.path !== rows[i].path || f.bytes !== rows[i].bytes)) {\n",
     "  if (differs(now, rows)) {\n"),
    ("kept-logs.mjs",
     "        `(${got.length} entries): ${bad.map((r) => r.path).join(', ')}`);\n"
     "    }\n"
     "    renameSync(part, archive);\n",
     "        `(${got.length} entries): ${bad.map((r) => r.path).join(', ')}`);\n"
     "    }\n"
     "    // The second walk, the last check before the folder goes: it must still hold exactly the list's files and sizes.\n"
     "    // Anything else is a mismatch: the folder and the unfinished archive stay, and the script exits 1.\n"
     "    if (differs(walk(dir, set, (msg) => { throw new Mismatch(msg); }), rows)) {\n"
     "      throw new Mismatch(`the folder changed while it was packed: its files or sizes now differ from ${listRel}`);\n"
     "    }\n"
     "    renameSync(part, archive);\n"),
    ("kept-logs.test.mjs",
     "  assert.ok(existsSync(join(repo, `${ARCHIVE}.part`)), 'and the next pack leaves it');\n"
     "});\n",
     "  assert.ok(existsSync(join(repo, `${ARCHIVE}.part`)), 'and the next pack leaves it');\n"
     "});\n"
     + r"""
test('a folder that changes after the read-back, before it would go: the second walk fails (1) on a late file and on ' +
  'a late link; the folder and the unfinished archive stay', (t) => {
  // For the child process only: when pack opens the unfinished archive to read it back, `late` runs in the repository.
  // The archive holds exactly the list, so the read-back passes and only the second walk can see the change.
  const packWith = (late) => {
    const { root, repo, set } = fixture(t);
    assert.equal(list(repo).status, 0);
    commit(repo, `${RES}/raw-logs.sha256`, 'raw-log list');
    const hook = join(root, 'late.mjs');
    writeFileSync(hook, "import fs from 'node:fs';\nimport { syncBuiltinESMExports } from 'node:module';\n" +
      'const open = fs.createReadStream;\n' +
      `fs.createReadStream = (path, ...rest) => { if (String(path).endsWith('.tar.gz.part')) { ${late}; } ` +
      'return open(path, ...rest); };\nsyncBuiltinESMExports();\n');
    const env = { ...process.env };
    delete env.SHAMBLE_ROLE;
    const p = spawnSync(process.execPath, ['--import', pathToFileURL(hook).href, SCRIPT, 'pack', SET, '--results', RES],
      { cwd: repo, encoding: 'utf8', env });
    assert.equal(p.status, 1, p.stderr + p.stdout);
    assert.ok(existsSync(join(set, 'marks.json')), 'the folder stays');
    assert.equal(existsSync(join(repo, ARCHIVE)), false);
    assert.ok(existsSync(join(repo, `${ARCHIVE}.part`)), 'the unfinished archive stays for Mustafa to see');
    return { p, set };
  };
  const a = packWith(`fs.writeFileSync(${JSON.stringify(`${RUNS}/${SET}/late.log`)}, 'late')`);
  assert.match(a.p.stderr, /MISMATCH[\s\S]*changed while it was packed[\s\S]*the folder stays/);
  assert.equal(readFileSync(join(a.set, 'late.log'), 'utf8'), 'late', 'the late file stays too');
  const b = packWith(`fs.symlinkSync('marks.json', ${JSON.stringify(`${RUNS}/${SET}/late-link`)})`);
  assert.match(b.p.stderr, /MISMATCH[\s\S]*late-link is not a plain file[\s\S]*the folder stays/);
  assert.ok(lstatSync(join(b.set, 'late-link')).isSymbolicLink(), 'the link stays, never followed');
});
"""),
]
after = dict(before)
for name, old, new in R:
    assert after[name].count(old) == 1, (name, old[:70], after[name].count(old))
    after[name] = after[name].replace(old, new)

SECOND = ("    if (differs(walk(dir, set, (msg) => { throw new Mismatch(msg); }), rows)) {\n"
          "      throw new Mismatch(`the folder changed while it was packed: its files or sizes now differ from ${listRel}`);\n"
          "    }\n")
THROWER = "walk(dir, set, (msg) => { throw new Mismatch(msg); })"
mutants = {"M12": after["kept-logs.mjs"].replace(SECOND, ""), "M13": after["kept-logs.mjs"].replace(THROWER, "walk(dir, set)")}
assert after["kept-logs.mjs"].count(SECOND) == 1 and after["kept-logs.mjs"].count(THROWER) == 1
for d, files in (("before", before), ("after", after)):
    os.makedirs(f"{SP}/{d}")
    for name, body in files.items():
        with open(f"{SP}/{d}/{name}", "w", encoding="utf-8") as fh:
            fh.write(body)
for m, body in mutants.items():
    os.makedirs(f"{SP}/mutants/{m}")
    with open(f"{SP}/mutants/{m}/kept-logs.mjs", "w", encoding="utf-8") as fh:
        fh.write(body)

out = []
for name, path in (("checkpoints-11.txt", ".claude/CHECKPOINTS.md (§11)"), ("kept-logs.mjs", ".claude/scripts/kept-logs.mjs"),
                   ("kept-logs.test.mjs", ".claude/scripts/kept-logs.test.mjs")):
    out += difflib.unified_diff(before[name].splitlines(keepends=True), after[name].splitlines(keepends=True),
                                fromfile=f"a/{path} (as posted)", tofile=f"b/{path} (with KL-CHANGE)", n=1)
with open(DIFF, "w", encoding="utf-8") as fh:
    fh.write("".join(out))
for name in before:
    b, a = before[name], after[name]
    width = max(len(l) for l in a.split("\n"))
    print(f"{name}: {b.count(chr(10))} -> {a.count(chr(10))} lines; widest after {width}; "
          f"after sha256 {hashlib.sha256(a.encode()).hexdigest()[:16]}")
with open(DIFF, "rb") as fh:
    d = fh.read()
print(f"{DIFF}: {d.count(b'\n')} lines, sha256 {hashlib.sha256(d).hexdigest()[:16]}")
