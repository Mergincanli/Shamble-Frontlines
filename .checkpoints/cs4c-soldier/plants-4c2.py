#!/usr/bin/env python3
"""Change-set 4c-2's planted defects (the soldier, steps 1 to 7): each new check is shown red on the defect it exists
to catch, then green again. 4c-1's script (`plants-4c.py`) in its form. The plants grow commit by commit (step 1's
build order, `step1-build-order.md`): each commit's plants are run before that commit is made (the advisor tool's
check of 2026-11-28), and the whole set at step 1's end.

From the worktree's root, with no other run in it (the script edits source files, one plant at a time, and puts each
back whatever happens; it says at its end whether every file is byte for byte as at the start):
    python3 -I -B .checkpoints/cs4c-soldier/plants-4c2.py <commit id> [names, e.g. R1 R2]
    python3 -I -B .checkpoints/cs4c-soldier/plants-4c2.py --anchors
It writes one line per check to .checkpoints/cs4c-soldier/plants-4c2.txt (a whole run only) and the commands' output
to /home/user/Shamble-Frontlines/.gate/cs4b-2/.gate/plants-4c2.log. The commit id is the caller's `git rev-parse
HEAD` (the tree it ran on, which may hold the commit-to-be's files); the caller also shows `git status --porcelain --
crates packages content` before and after (this script runs no git). A plant counts as caught only when the command
fails AND its output holds the check's own words for that defect. Not part of the product: notes.
"""
import hashlib, os, signal, subprocess, sys

ROOT = "/home/user/Shamble-Frontlines/.gate/cs4b-2"
os.chdir(ROOT)
LOG = open("/home/user/Shamble-Frontlines/.gate/cs4b-2/.gate/plants-4c2.log", "a")
OUT = []
ENV = dict(os.environ, SHAMBLE_WASM_BINDGEN=f"{ROOT}/.gate/wb-tool/wasm-bindgen")
SOLDIERS = "crates/sim/src/soldiers.rs"

def run(*command, env=None, cwd=None, timeout=None):
    # Its own process group, so that a timeout ends every child too (4c-1's test-engineer note). A run stopped by its
    # timeout reads as exit 124 with a note.
    process = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, env=env or ENV, cwd=cwd or ROOT, start_new_session=True)
    try:
        text, _ = process.communicate(timeout=timeout)
        code = process.returncode
    except subprocess.TimeoutExpired:
        os.killpg(process.pid, signal.SIGKILL)
        text, _ = process.communicate()
        code, text = 124, (text or "") + f"\nTIMED OUT after {timeout} s: the run's whole process group was ended\n"
    LOG.write(f"\n$ {' '.join(command)}\n{text}\n[exit {code}]\n"); LOG.flush()
    return code, text

def cargo_test(package, *filters, lib=True):
    return ("cargo", "test", "-p", package, "--locked", *(["--lib"] if lib else []), *filters)

SOLDIER_TESTS = cargo_test("sim", "soldiers::tests")

# name, kind, what the defect is, the edits (file, text there once, its replacement), the command, the words its output must hold
PLANTS = [
    # C1: the soldier table (11.7 row 1, PM-0004-15).
    ("R1", "defect", "a spawn that does not raise the slot's generation: an id despawned before resolves again",
     [(SOLDIERS, "        let generation = row.generation.wrapping_add(1);\n", "        let generation = row.generation;\n")],
     SOLDIER_TESTS, ["test soldiers::tests::a_stale_id_never_resolves ... FAILED"]),
    ("R2", "defect", "a despawn that leaves the row occupied: the despawned id still resolves",
     [(SOLDIERS, "        *row = Soldier {\n            generation: row.generation,\n            ..Soldier::EMPTY\n        };\n        true",
                 "        *row = Soldier {\n            generation: row.generation,\n            flags: OCCUPIED,\n            ..Soldier::EMPTY\n        };\n        true")],
     SOLDIER_TESTS, ["test soldiers::tests::a_stale_id_never_resolves ... FAILED"]),
    ("RC1", "control", "the clean tree: the soldier table's tests",
     [], SOLDIER_TESTS, ["test result: ok. 5 passed"]),
]

def note(name, kind, ok, what, detail):
    words = {"defect": ("CAUGHT", "NOT CAUGHT"), "control": ("HELD", "DID NOT HOLD")}[kind]
    line = f"{name} {words[0] if ok else words[1]} ({kind}): {what}: {detail}"
    OUT.append((kind, ok, line)); print(line, flush=True)

def digest(path): return hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.exists(path) else "absent"

def execute(command):
    return run(*command)

def planted(name, kind, what, edits, command, words):
    originals = {path: open(path).read() for path, _, _ in edits}
    try:
        for path, old, new in edits:
            text = open(path).read()
            count = text.count(old)
            if count != 1: return note(name, kind, False, what, f"the text to replace stands {count} times in {path}")
            open(path, "w").write(text.replace(old, new, 1))
        code, text = execute(command)
        found = [word for word in words if word in text]
        ok = (code == 0 if kind == "control" else code != 0) and len(found) == len(words)
        note(name, kind, ok, what, f"exit {code}, found {len(found)} of {len(words)} wanted texts" + ("" if ok else f"; missing {[w for w in words if w not in found]}"))
    finally:
        for path, original in originals.items(): open(path, "w").write(original)

def anchors():
    """Reads only: every plant's text to replace stands exactly once in its file (a plant whose anchor moved is no plant)."""
    wrong = []
    for name, _, _, edits, _, _ in PLANTS:
        texts = {}
        for path, old, new in edits:
            text = texts.get(path, open(path).read())
            if text.count(old) != 1: wrong.append((name, path, text.count(old)))
            texts[path] = text.replace(old, new, 1)
    for name, path, count in wrong: print(f"{name}: its text stands {count} times in {path}")
    print(f"ANCHORS: {sum(len(plant[3]) for plant in PLANTS)} edits of {len(PLANTS)} plants read, {len(wrong)} wrong")

def main():
    if sys.argv[1] == "--anchors": return anchors()
    commit, names = sys.argv[1], set(sys.argv[2:])
    print(f"plants on {commit}", flush=True)
    touched = sorted({path for plant in PLANTS for path, _, _ in plant[3]})
    before = {path: digest(path) for path in touched}
    for name, kind, what, edits, command, words in PLANTS:
        if names and name not in names: continue
        planted(name, kind, what, edits, command, words)
    back = {path: digest(path) for path in touched} == before
    defects = [ok for kind, ok, _ in OUT if kind == "defect"]
    controls = [ok for kind, ok, _ in OUT if kind == "control"]
    lines = [line for _, _, line in OUT]
    lines.append(f"DEFECT CHECKS: {sum(defects)} caught, {len(defects) - sum(defects)} not caught. CONTROLS: {sum(controls)} held, {len(controls) - sum(controls)} did not hold.")
    lines.append(f"EVERY FILE PUT BACK (sha256 of {len(touched)} files as at the start): {'yes' if back else 'NO'}")
    if not names:
        open(f"{ROOT}/.checkpoints/cs4c-soldier/plants-4c2.txt", "w").write(f"plants on {commit}\n" + "\n".join(lines) + "\n")
    print("\n".join(lines[-2:]))

main()
