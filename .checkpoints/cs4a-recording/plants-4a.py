#!/usr/bin/env python3
"""Change-set 4a's planted defects: each new check is shown red on the defect it exists to catch, then green again.

From the worktree's root, with no reviewer or other run in it (the script edits crates/sim/src/world.rs and, for one
plant, makes the recordings again; it puts both back whatever happens and says at its end whether it did):
    python3 .checkpoints/cs4a-recording/plants-4a.py <commit id> [letters, e.g. A C]
It writes one line per check to .checkpoints/cs4a-recording/plants-4a.txt and the commands' output to
.gate/plants-4a.log. The commit id is the caller's `git rev-parse HEAD`, printed first; the caller also shows
`git status --porcelain -- crates packages` empty before and after. One run at a time (the browser tests use one
port). Not part of the product: notes.

The plants (PM-0004-21's three injections are C, D and E; each is shown at its tick with the field that differs):
  A  the simulation changed at one tick                      -> every target names step 900
  B  a counter outside the state that survives between runs  -> the two-matches tests (and, being that crude, solo runs)
  G  state that leaks only between interleaved matches       -> ONLY the two-matches tests; the solo checks stay green
  C  a hash map's iteration order folded into the state      -> two processes differ at step 600, field named
  D  a draw from outside the simulation (the process's own)  -> two processes differ at step 600, field named
  E  one value differs in WebAssembly only (a stand-in for a maths call that differs between targets: the search for
     a real sine input was not run, and no float is in the state yet)  -> Node at step 600, field named
  F  a usize in the canonical bytes                          -> the natively made hashes against WebAssembly, step 1
"""
import os, subprocess, sys

ROOT = os.getcwd()
WORLD = "crates/sim/src/world.rs"
FOLDER = "crates/devtools/recordings"
REC, HASHES = f"{FOLDER}/first-loop.rec", f"{FOLDER}/first-loop.hashes"
LOG = open(".gate/plants-4a.log", "w")
OUT = []
ENV = dict(os.environ, SHAMBLE_WASM_BINDGEN=f"{ROOT}/.gate/wb-tool/wasm-bindgen",
           SHAMBLE_DIST=f"{ROOT}/.gate/dist-development", SHAMBLE_CHROMIUM="/opt/pw-browsers/chromium")
STEP = "            self.player = *newest;\n        }\n"
VISIT = '        sink.u32("tick", *tick);\n'
ATOMIC = "std::sync::atomic::AtomicU32"
ORDER = "std::sync::atomic::Ordering::Relaxed"

def run(*command):
    done = subprocess.run(command, capture_output=True, text=True, env=ENV)
    text = done.stdout + done.stderr
    LOG.write(f"\n$ {' '.join(command)}\n{text}\n[exit {done.returncode}]\n"); LOG.flush()
    return done.returncode, text

def native(test, *more): return ("cargo", "test", "-p", "devtools", "--locked", "--test", test, *more)
def tool(*arguments): return ("cargo", "run", "-q", "-p", "devtools", "--locked", "--bin", "replay", "--", *arguments)
BUILD = ("node", "packages/client/scripts/build.mjs", "--out-dir", f"{ROOT}/.gate/dist-development")
NODE = ("node", "node_modules/vitest/vitest.mjs", "run", "--root", "packages/sim-worker", "--dir",
        "packages/sim-worker/boundary", "replay", "--reporter=verbose")
CHROMIUM = ("node", "node_modules/@playwright/test/cli.js", "test", "-c", "packages/e2e/playwright.config.ts",
            "--project=chromium", "--workers=1", "--retries=0", "--trace", "off", "tests/replay")
SOLO, TURNS = "first-loop replays on the players' file", "two matches on one module instance"

def note(letter, kind, ok, what, detail):
    words = {"defect": ("CAUGHT", "NOT CAUGHT"), "control": ("HELD", "DID NOT HOLD")}[kind]
    line = f"{letter} {words[0] if ok else words[1]} ({kind}): {what}: {detail}"
    OUT.append((kind, ok, line)); print(line, flush=True)

def caught(letter, what, command, must_contain):
    code, text = run(*command)
    found = [needle for needle in must_contain if needle in text]
    note(letter, "defect", code != 0 and len(found) == len(must_contain), what, f"exit {code}, found {found} of {list(must_contain)}")
    return text

def held(letter, what, command):
    code, _ = run(*command)
    note(letter, "control", code == 0, what, f"exit {code}")

def node_tests(text):
    """Which of the Node suite's tests passed and failed, from the verbose reporter's lines."""
    marks = {}
    for line in text.splitlines():
        for name in (SOLO, TURNS):
            if name in line and line.lstrip()[:1] in "✓×":
                marks[name] = line.lstrip()[0] == "✓"
    return marks

def planted(original, anchor, code):
    assert original.count(anchor) == 1, anchor
    open(WORLD, "w").write(original.replace(anchor, anchor + code))

def tick_and_field(letter, what):
    """For a defect two processes disagree on: the step, by the tool against the committed hashes; the field, by the
    state at that tick printed in two processes and compared by the tool."""
    caught(letter, f"{what}: the tool names the step", tool("check", REC, HASHES), ["step 600"])
    for name in ("one", "two"):
        _, state = run(*tool("state", REC, "600"))
        open(f".gate/plants-state-{name}.txt", "w").write(state)
    text = caught(letter, f"{what}: the tool names the field", tool("fields", ".gate/plants-state-one.txt", ".gate/plants-state-two.txt"), ["player.presses: expected"])
    OUT.append(("detail", True, "    " + " | ".join(line for line in text.splitlines() if line.startswith("player."))[:300]))

def plants(original, wanted):
    if "A" in wanted:
        planted(original, STEP, "        if self.tick == 899 { self.player.presses[0] ^= 1; }\n")
        caught("A", "the simulation changed at tick 900, natively", native("recordings", "every_recording_replays"), ["step: 900"])
        code, _ = run(*BUILD)
        caught("A", "the same, in Node on the players' file", NODE, ["expected 900 to be null"])
        caught("A", "the same, in Chromium on the players' file", CHROMIUM, ["Received: 900"] if code == 0 else ["the build failed"])
    if "B" in wanted:
        planted(original, STEP, f"        {{ static PLANTED: {ATOMIC} = {ATOMIC}::new(0);\n"
                f"          let n = PLANTED.fetch_add(1, {ORDER});\n"
                "          self.player.presses[1] = self.player.presses[1].wrapping_add(n as u8); }\n")
        caught("B", "a counter outside the state, natively (two matches in turn)", native("two_instances"), ["two_instances.rs", "panicked"])
    if "G" in wanted:
        planted(original, STEP, f"        {{ static STEPS: {ATOMIC} = {ATOMIC}::new(0);\n"
                f"          if self.tick == 0 {{ STEPS.store(0, {ORDER}); }}\n"
                f"          if STEPS.fetch_add(1, {ORDER}) != self.tick {{ self.player.presses[1] ^= 1; }} }}\n")
        caught("G", "state that leaks only between matches in turn, natively", native("two_instances"), ["two_instances.rs", "panicked"])
        held("G", "the solo replay stays green under it (one test at a time)", native("recordings", "every_recording_replays", "--", "--test-threads=1"))
        held("G", "the two-process test stays green under it", native("two_processes"))
        run(*BUILD)
        code, text = run(*NODE)
        marks = node_tests(text)
        note("G", "defect", code != 0 and marks.get(TURNS) is False, "the same in Node: two matches on one module instance", f"exit {code}, that test failed: {marks.get(TURNS) is False}")
        note("G", "control", marks.get(SOLO) is True, "the solo replay on the players' file stays green under it", f"that test passed: {marks.get(SOLO)}")
    if "C" in wanted:
        planted(original, STEP, "        if self.tick == 599 { let map: std::collections::HashMap<u32, u32> = (0..32).map(|k| (k, k)).collect();\n"
                "          let mut order = 0u64; for (k, _) in &map { order = order.wrapping_mul(0x100_0000_01B3) ^ u64::from(*k); }\n"
                "          for (count, byte) in self.player.presses[2..10].iter_mut().zip(order.to_le_bytes()) { *count = byte; } }\n")
        caught("C", "a hash map's order in the state at tick 600 (two processes)", native("two_processes"), ["the two processes differ", "step: 600"])
        tick_and_field("C", "a hash map's order")
    if "D" in wanted:
        planted(original, STEP, "        if self.tick == 599 { use std::hash::BuildHasher; let draw = std::hash::RandomState::new().hash_one(7u8);\n"
                "          for (count, byte) in self.player.presses[3..11].iter_mut().zip(draw.to_le_bytes()) { *count = byte; } }\n")
        caught("D", "a draw from outside the simulation at tick 600 (two processes)", native("two_processes"), ["the two processes differ", "step: 600"])
        tick_and_field("D", "an outside draw")
    if "E" in wanted:
        planted(original, STEP, '        #[cfg(target_arch = "wasm32")]\n        if self.tick == 599 { self.player.presses[4] ^= 1; }\n')
        held("E", "the native run stays green with a WebAssembly-only change", native("recordings", "every_recording_replays"))
        run(*BUILD)
        caught("E", "one value differs in WebAssembly only at tick 600, in Node on the players' file", NODE, ["expected 600 to be null"])
        _, native_state = run(*tool("state", REC, "600"))
        script = ("import('./.gate/rules-module/release/devtools.js').then((g)=>{const fs=require('node:fs');"
                  "g.initSync({module:fs.readFileSync('.gate/rules-module/release/devtools_bg.wasm')});"
                  f"process.stdout.write(g.replay_state_at(fs.readFileSync('{REC}'),600));}})")
        wasm_state = subprocess.run(("node", "-e", script), capture_output=True, text=True, env=ENV).stdout
        open(".gate/plants-state-native.txt", "w").write(native_state)
        # Node's own output only (no warning text), and without the vector line the test module adds after the state.
        open(".gate/plants-state-wasm.txt", "w").write("".join(line + "\n" for line in wasm_state.splitlines() if not line.startswith("tick 600 shared bytes")))
        text = caught("E", "the tool names the field (the native state against the test module's)", tool("fields", ".gate/plants-state-native.txt", ".gate/plants-state-wasm.txt"), ["player.presses: expected"])
        OUT.append(("detail", True, "    " + " | ".join(line for line in text.splitlines() if line.startswith("player."))[:300]))
    if "F" in wanted:
        planted(original, VISIT, '        sink.bytes("tick.usize", &usize::to_le_bytes(*tick as usize));\n')
        run("cargo", "run", "-q", "-p", "devtools", "--locked", "--bin", "make_recordings")
        run(*BUILD)
        caught("F", "a usize in the canonical bytes: the natively made hashes against WebAssembly", NODE, ["expected 1 to be null"])

def main():
    commit, wanted = sys.argv[1], [letter.upper() for letter in sys.argv[2:]] or list("ABGCDEF")
    print(f"planted defects on commit {commit}", flush=True)
    original = open(WORLD).read()
    files = {name: open(f"{FOLDER}/{name}", "rb").read() for name in os.listdir(FOLDER)}
    try:
        plants(original, wanted)
    finally:
        open(WORLD, "w").write(original)
        for name, data in files.items():
            open(f"{FOLDER}/{name}", "wb").write(data)
    restored = open(WORLD).read() == original and sorted(os.listdir(FOLDER)) == sorted(files) and all(open(f"{FOLDER}/{n}", "rb").read() == d for n, d in files.items())
    # The clean tree again: every check green.
    green = [run(*BUILD)[0], run("cargo", "test", "-p", "devtools", "--locked")[0], run(*NODE)[0], run(*CHROMIUM)[0],
             run("cargo", "run", "-q", "-p", "devtools", "--locked", "--bin", "make_recordings", "--", "--check")[0]]
    count = lambda kind, ok: sum(1 for k, o, _ in OUT if k == kind and o == ok)
    summary = [
        f"planted defects on commit {commit}; plants run: {' '.join(wanted)}",
        *[line for _, _, line in OUT],
        f"DEFECT CHECKS: {count('defect', True)} caught, {count('defect', False)} not caught. CONTROLS: {count('control', True)} held, {count('control', False)} did not hold.",
        f"TREE PUT BACK (world.rs and the recordings byte for byte as at the start): {'yes' if restored else 'NO'}",
        f"CLEAN TREE {'GREEN' if green == [0] * 5 else 'NOT GREEN'}: build, native tests, Node, Chromium, make_recordings --check: exits {green}",
    ]
    print("\n".join(summary[-3:]), flush=True)
    open(".checkpoints/cs4a-recording/plants-4a.txt", "w").write("\n".join(summary) + "\n")

main()
