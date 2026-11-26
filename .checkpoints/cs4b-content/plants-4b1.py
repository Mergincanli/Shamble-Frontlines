#!/usr/bin/env python3
"""Change-set 4b-1's planted defects: each new check is shown red on the defect it exists to catch, then green again.

From the worktree's root, with no reviewer or other run in it (the script edits source files, one at a time, and puts
each back whatever happens; it says at its end whether every file is byte for byte as at the start):
    python3 .checkpoints/cs4b-content/plants-4b1.py <commit id> [letters, e.g. A C]
It writes one line per check to .checkpoints/cs4b-content/plants-4b1.txt and the commands' output to
.gate/plants-4b1.log. The commit id is the caller's `git rev-parse HEAD`, printed first; the caller also shows
`git status --porcelain -- crates packages` empty before and after (this script runs no git). A plant counts as
caught only when the command fails AND its output holds the check's own words for that defect; "the build failed" is
never enough. Not part of the product: notes.
"""
import hashlib, os, subprocess, sys

ROOT = os.getcwd()
LOG = open(".gate/plants-4b1.log", "w")
OUT = []
ENV = dict(os.environ, SHAMBLE_WASM_BINDGEN=f"{ROOT}/.gate/wb-tool/wasm-bindgen")
JSON, NUMBER, WRITE, READ = (f"crates/content/src/{name}.rs" for name in ("json", "number", "write", "read"))
BUNDLE, MAP, TIME = (f"crates/content/src/{name}.rs" for name in ("bundle", "map", "time"))
FILES, SIM_TIME = "crates/devtools/src/files.rs", "crates/sim/src/time.rs"
HELPER = "packages/sim-worker/boundary/plain-file.ts"

def run(*command):
    done = subprocess.run(command, capture_output=True, text=True, env=ENV)
    text = done.stdout + done.stderr
    LOG.write(f"\n$ {' '.join(command)}\n{text}\n[exit {done.returncode}]\n"); LOG.flush()
    return done.returncode, text

def test(name, *more): return ("cargo", "test", "-p", "devtools", "--locked", "--test", name, *more)
CLASSES = test("content_proofs", "the_classes_hold")
CORPUS = test("content_proofs", "the_reader_and_the_writer")
TREES = test("content_proofs", "the_canonical_writer")
FUZZ = test("content_fuzz", "fifty_thousand")
RECORDINGS = test("recordings", "the_plain_read")
TIME_UNIT = test("time_unit")
AIM = test("aim_reach")
UNIT = ("cargo", "test", "-p", "content", "--locked")
NODE = ("node", "node_modules/vitest/vitest.mjs", "run", "--root", "packages/sim-worker", "--dir", "packages/sim-worker/boundary")
WASM = ("node", "packages/client/scripts/build.mjs", "--wasm-only")

# letter, what the defect is, file, the text it replaces (once), its replacement, the command, the words its output must hold
PLANTS = [
    ("A", "the reader takes a tab as white space", JSON, "Some(b' ' | b'\\n') => self.bump(),", "Some(b' ' | b'\\n' | b'\\t') => self.bump(),",
     CLASSES, ["a tab or a carriage return, case", "and the class says", "Whitespace { byte: 9 }"]),
    ("B", "a key that is there twice is let through", JSON, "                && first.key == second.key\n", "                && first.key == second.key\n                && first.key.is_empty()\n",
     CLASSES, ["a key that is there twice, case", "DuplicateKey"]),
    ("C", "the depth is checked one level too late", JSON, "b'{' | b'[' if depth > MAX_DEPTH => {", "b'{' | b'[' if depth > MAX_DEPTH + 1 => {",
     CLASSES, ["nesting past the cap, case", "TooDeep { cap: 32 }"]),
    ("D", "a byte no content file holds is taken between values", JSON, "Some(byte) if !(0x20..=0x7e).contains(&byte) => {", "Some(byte) if byte == 0 => {",
     CLASSES, ["a forbidden byte between values, case", "ForbiddenByte"]),
    ("E", "the writer always writes 17 digits", NUMBER, 'let scientific = format!("{:e}", value.abs());', 'let scientific = format!("{:.16e}", value.abs());',
     CORPUS, ["the writer writes"]),
    ("F", "the writer starts the exponent form at 1e16", NUMBER, "if (count..=21).contains(&point) {", "if (count..=16).contains(&point) {",
     CORPUS, ["the writer writes"]),
    ("G", "the writer takes the upper of two equally close texts, as the standard library does", NUMBER, "    if places < 1 || decimals != places.checked_sub(1)? {", "    if places < 1 || decimals != places.checked_sub(1)? || places > 0 {",
     CORPUS, ["the writer writes"]),
    ("H", "the reader cuts a number's text short", NUMBER, "    if value.is_finite() {\n        Ok(value)", "    if value.is_finite() {\n        Ok(value as f32 as f64)",
     CORPUS, ["the reader gives"]),
    ("I", "the writer lays an empty array out on two lines", WRITE, 'Value::Array(items) if items.is_empty() => sink.put("[]"),', 'Value::Array(items) if items.is_empty() => sink.put("[\\n]"),',
     TREES, ["the writer writes"]),
    ("J", "a key the kind does not have is let through", READ, "        match unread {\n            None => Ok(()),", "        match unread.filter(|_| false) {\n            None => Ok(()),",
     CLASSES, ["and the class says", "UnknownKey { key: Echo("]),
    ("K", "a range that leaves its lower end out takes it", READ, "            value > self.min\n", "            value >= self.min\n",
     CLASSES, ["a number outside its range, case", "OutOfRange"]),
    ("L", "the bundle's files may come in any order", BUNDLE, "if last.is_some_and(|last| last.as_bytes() >= path.as_bytes()) {", "if last.is_some_and(|last| last.as_bytes() >= path.as_bytes() && false) {",
     CLASSES, ["paths out of order, case", "PathOrder"]),
    ("M", "a file may be one byte over its cap", BUNDLE, "let top = document(text, MAX_FILE_BYTES)?;", "let top = document(text, MAX_FILE_BYTES + 1)?;",
     CLASSES, ["a file past the byte cap, case", "TooLarge"]),
    ("N", "a text that is not in canonical form is taken", BUNDLE, "    match first_difference(&top, bytes) {\n        None => Ok(top),", "    match first_difference(&top, bytes).filter(|_| false) {\n        None => Ok(top),",
     CLASSES, ["a text not in canonical form, case", "NotCanonical"]),
    ("O", "the aim's reach is one centimetre too wide", MAP, "pub const AIM_REACH_CM: i32 = 32_767;", "pub const AIM_REACH_CM: i32 = 32_768;",
     CLASSES, ["a map past the aim's reach, with no spawn or an id twice, case", "BeyondAim"]),
    ("P", "the same reach against the aim point's own number types", MAP, "pub const AIM_REACH_CM: i32 = 32_767;", "pub const AIM_REACH_CM: i32 = 32_768;",
     AIM, ["32768 cm does not fit the aim point's field"]),
    ("Q", "nine millimetres pass for a centimetre", MAP, '.saturating_sub(place(low, "min")?) < grid.thinnest', '.saturating_sub(place(low, "min")?) < grid.thinnest - 1',
     CLASSES, ["a map's box off its grid, too thin or outside the bounds, case", "TooThin"]),
    ("R", "a corner between two steps of its grid is taken", MAP, "    (steps / per_metre == value).then_some(steps as i64)", "    (steps / per_metre == value || steps != 0.0).then_some(steps as i64)",
     CLASSES, ["a map's box off its grid, too thin or outside the bounds, case", "OffGrid"]),
    ("S", "an id may stand twice in a map's list", MAP, "            && first == second\n", "            && first == second\n            && first.is_empty()\n",
     CLASSES, ["a map past the aim's reach, with no spawn or an id twice, case", "DuplicateId"]),
    ("T", "a panic on a path that mutation reaches (a number below zero)", READ, "    if range.holds(value) {\n        return Ok(value);", "    assert!(value >= 0.0, \"planted\");\n    if range.holds(value) {\n        return Ok(value);",
     FUZZ, ["the loader panicked", "seed 4, input "]),
    ("U", "a duration is cut, not rounded", TIME, "let units = (seconds * f64::from(UNITS_PER_SECOND)).round();", "let units = (seconds * f64::from(UNITS_PER_SECOND)).trunc();",
     TIME_UNIT, ["the conversion gives 215 units where the text's digits give 216"]),
    ("V", "the simulation counts another unit than the content crate converts to", SIM_TIME, "pub const UNITS_PER_SECOND: u32 = 3_000;", "pub const UNITS_PER_SECOND: u32 = 6_000;",
     TIME_UNIT, ["the_content_crates_unit_is_the_simulations", "left: 3000", "right: 6000"]),
    ("W", "the plain read opens whatever stands in a file's place", FILES, "    plain(folder, \"\")?;\n    plain(folder, name)?;\n    let path = folder.join(name);", "    plain(folder, \"\")?;\n    let path = folder.join(name);",
     RECORDINGS, ["the_plain_read_refuses_a_link", "link.rec"]),
    ("X", "the Node read opens whatever stands in a file's place", HELPER, "if (!entry.isFile()) throw", "if (entry.isSocket()) throw",
     (*NODE, "plain-file"), ["a link, a folder, a missing file and a name that is a path are refused"]),
]

def note(letter, kind, ok, what, detail):
    words = {"defect": ("CAUGHT", "NOT CAUGHT"), "control": ("HELD", "DID NOT HOLD")}[kind]
    line = f"{letter} {words[0] if ok else words[1]} ({kind}): {what}: {detail}"
    OUT.append((kind, ok, line)); print(line, flush=True)

def digest(path): return hashlib.sha256(open(path, "rb").read()).hexdigest()

def main():
    commit, letters = sys.argv[1], set(sys.argv[2:])
    print(f"plants on {commit}", flush=True)
    touched = sorted({plant[2] for plant in PLANTS} | {JSON})
    before = {path: digest(path) for path in touched}
    for letter, what, path, old, new, command, words in PLANTS:
        if letters and letter not in letters: continue
        original = open(path).read()
        if original.count(old) != 1:
            note(letter, "defect", False, what, f"the text to replace stands {original.count(old)} times in {path}")
            continue
        try:
            open(path, "w").write(original.replace(old, new))
            code, text = run(*command)
            found = [word for word in words if word in text]
            note(letter, "defect", code != 0 and len(found) == len(words), what, f"exit {code}, found {found} of {words}")
        finally:
            open(path, "w").write(original)
    # One verdict that differs in WebAssembly only: the native classes stay green, the Node compare names the class.
    if not letters or "Y" in letters:
        original = open(JSON).read()
        old = "Some(byte @ (b'\\t' | b'\\r')) => return Err(self.error(Kind::Whitespace { byte })),"
        new = "#[cfg(not(target_arch = \"wasm32\"))]\n                " + old + "\n                #[cfg(target_arch = \"wasm32\")]\n                Some(b'\\t' | b'\\r') => self.bump(),"
        try:
            assert original.count(old) == 1
            open(JSON, "w").write(original.replace(old, new))
            code, _ = run(*CLASSES)
            note("Y", "control", code == 0, "a reader that takes tabs in WebAssembly only: the native classes stay green", f"exit {code}")
            build, _ = run(*WASM)
            code, text = run(*NODE, "content")
            words = ["a tab or a carriage return, case", "and the class says"]
            found = [word for word in words if word in text]
            note("Y", "defect", build == 0 and code != 0 and len(found) == len(words), "the same reader, in Node on the test-only module, against the native digests", f"build {build}, exit {code}, found {found} of {words}")
        finally:
            open(JSON, "w").write(original)
    after = {path: digest(path) for path in touched}
    back = after == before
    defects = [ok for kind, ok, _ in OUT if kind == "defect"]
    controls = [ok for kind, ok, _ in OUT if kind == "control"]
    lines = [line for _, _, line in OUT]
    lines.append(f"DEFECT CHECKS: {sum(defects)} caught, {len(defects) - sum(defects)} not caught. CONTROLS: {sum(controls)} held, {len(controls) - sum(controls)} did not hold.")
    lines.append(f"EVERY FILE PUT BACK (sha256 of {len(touched)} files as at the start): {'yes' if back else 'NO'}")
    if not letters:
        # The clean tree again: the module rebuilt, then the checks the plants used, all green.
        exits = [run(*WASM)[0], run(*UNIT)[0], run(*test("content_proofs"))[0], run(*TIME_UNIT)[0], run(*AIM)[0], run(*test("recordings"))[0], run(*NODE, "content", "plain-file")[0]]
        lines.append(f"CLEAN TREE GREEN: the test module's build, the content crate's tests, the proofs, the time unit, the aim tie, the recordings, Node: exits {exits}")
    open(".checkpoints/cs4b-content/plants-4b1.txt", "w").write(f"plants on {commit}\n" + "\n".join(lines) + "\n")
    print("\n".join(lines[-3:]))

main()
