#!/usr/bin/env python3
"""Change-set 4a's planted defects: each new check is shown red on the defect it exists to catch, then green again.

From the worktree's root:  python3 .checkpoints/cs4a-recording/plants-4a.py [letters, e.g. A C]
It writes one line per plant to .checkpoints/cs4a-recording/plants-4a.txt and the commands' output to
.gate/plants-4a.log. Each plant is a few lines put into crates/sim/src/world.rs; the file is put back from the copy
taken at the start whatever happens, and the last step shows every check green on the clean tree. One run at a time
(the browser tests use one port). Not part of the product: notes.
"""
import os, subprocess, sys

ROOT = os.getcwd()
WORLD = "crates/sim/src/world.rs"
REC = "crates/devtools/recordings/first-loop.rec"
LOG = open(".gate/plants-4a.log", "w")
OUT = []
ENV = dict(os.environ, SHAMBLE_WASM_BINDGEN=f"{ROOT}/.gate/wb-tool/wasm-bindgen",
           SHAMBLE_DIST=f"{ROOT}/.gate/dist-development", SHAMBLE_CHROMIUM="/opt/pw-browsers/chromium")
STEP = "            self.player = *newest;\n        }\n"
VISIT = '        sink.u32("tick", *tick);\n'

def run(*command):
    done = subprocess.run(command, capture_output=True, text=True, env=ENV)
    text = done.stdout + done.stderr
    LOG.write(f"\n$ {' '.join(command)}\n{text}\n[exit {done.returncode}]\n"); LOG.flush()
    return done.returncode, text

NATIVE = lambda test, name="": ("cargo", "test", "-p", "devtools", "--locked", "--test", test, name)
BUILD = ("node", "packages/client/scripts/build.mjs", "--out-dir", f"{ROOT}/.gate/dist-development")
NODE = ("node", "node_modules/vitest/vitest.mjs", "run", "--root", "packages/sim-worker", "--dir",
        "packages/sim-worker/boundary", "replay")
CHROMIUM = ("node", "node_modules/@playwright/test/cli.js", "test", "-c", "packages/e2e/playwright.config.ts",
            "--project=chromium", "--workers=1", "--retries=0", "--trace", "off", "tests/replay")

def note(letter, what, ok, detail):
    line = f"{letter} {'CAUGHT' if ok else 'NOT CAUGHT'}: {what}: {detail}"
    OUT.append(line); print(line, flush=True)

def expect(letter, what, command, must_contain):
    code, text = run(*command)
    found = [needle for needle in must_contain if needle in text]
    note(letter, what, code != 0 and len(found) == len(must_contain),
         f"exit {code}, found {found} of {list(must_contain)}")

def planted(original, anchor, code):
    assert original.count(anchor) == 1, anchor
    open(WORLD, "w").write(original.replace(anchor, anchor + code))

def plants(original, wanted):
    if "A" in wanted:  # the simulation changed at one tick: every target names step 900
        planted(original, STEP, "        if self.tick == 899 { self.player.presses[0] ^= 1; }\n")
        expect("A", "the simulation changed at tick 900, natively", NATIVE("recordings", "every_recording_replays"), ["step: 900"])
        code, _ = run(*BUILD)
        expect("A", "the same, in Node on the players' file", NODE, ["expected 900 to be null"])
        expect("A", "the same, in Chromium on the players' file", CHROMIUM, ["Received: 900"] if code == 0 else ["the build failed"])
    if "B" in wanted:  # a counter kept outside the state: two matches in turn differ from each alone
        planted(original, STEP, "        { static PLANTED: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);\n"
                "          let n = PLANTED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);\n"
                "          self.player.presses[1] = self.player.presses[1].wrapping_add(n as u8); }\n")
        expect("B", "a static counter added to the state, natively (two matches in turn)", NATIVE("two_instances"), ["two_instances.rs", "panicked"])
        run(*BUILD)
        expect("B", "the same, in Node (two matches on one module instance)", NODE, ["two matches on one module instance", "FAIL"])
    if "C" in wanted:  # a hash map's iteration order folded into the state at tick 600: two processes differ
        planted(original, STEP, "        if self.tick == 599 { let map: std::collections::HashMap<u32, u32> = (0..32).map(|k| (k, k)).collect();\n"
                "          let mut fold = 0u8; for (i, (k, _)) in map.iter().enumerate() { fold = fold.wrapping_mul(31).wrapping_add((*k as u8) ^ (i as u8)); }\n"
                "          self.player.presses[2] = fold; }\n")
        expect("C", "a hash map's order in the state at tick 600 (two processes)", NATIVE("two_processes"), ["two_processes.rs", "left == right"])
    if "D" in wanted:  # a draw from outside the simulation (the process's own random state) at tick 600
        planted(original, STEP, "        if self.tick == 599 { use std::hash::BuildHasher; let draw = std::hash::RandomState::new().hash_one(7u8);\n"
                "          for (count, byte) in self.player.presses[3..11].iter_mut().zip(draw.to_le_bytes()) { *count = byte; } }\n")
        expect("D", "a draw from outside the simulation at tick 600 (two processes)", NATIVE("two_processes"), ["two_processes.rs", "left == right"])
    if "E" in wanted:  # one target computes one value differently (stands in for a libm call): WebAssembly only
        planted(original, STEP, '        #[cfg(target_arch = "wasm32")]\n        if self.tick == 599 { self.player.presses[4] ^= 1; }\n')
        code, _ = run(*NATIVE("recordings", "every_recording_replays"))
        note("E", "control: the native run stays green with a WebAssembly-only change", code == 0, f"exit {code}")
        run(*BUILD)
        expect("E", "one value differs in WebAssembly only at tick 600, in Node on the players' file", NODE, ["expected 600 to be null"])
        _, native = run("cargo", "run", "-q", "-p", "devtools", "--locked", "--bin", "replay", "--", "state", REC, "600")
        script = ("import('./.gate/rules-module/release/devtools.js').then((g)=>{const fs=require('node:fs');"
                  "g.initSync({module:fs.readFileSync('.gate/rules-module/release/devtools_bg.wasm')});"
                  f"process.stdout.write(g.replay_state_at(fs.readFileSync('{REC}'),600));}})")
        _, wasm = run("node", "-e", script)
        differing = [f"{a.split(' = ')[0]}: native {a.split(' = ')[-1]}, WebAssembly {b.split(' = ')[-1]}"
                     for a, b in zip(native.splitlines(), wasm.splitlines()) if a != b]
        note("E", "the field-by-field difference at tick 600 names the field", len(differing) == 1 and differing[0].startswith("player.presses"), "; ".join(differing)[:230])
    if "F" in wanted:  # a usize in the canonical bytes: 8 bytes natively, 4 in WebAssembly
        planted(original, VISIT, '        sink.bytes("tick.usize", &usize::to_le_bytes(*tick as usize));\n')
        run("cargo", "run", "-q", "-p", "devtools", "--locked", "--bin", "make_recordings")
        run(*BUILD)
        expect("F", "a usize in the canonical bytes: the natively made hashes against WebAssembly", NODE, ["expected 1 to be null"])

def main():
    wanted = [letter.upper() for letter in sys.argv[1:]] or list("ABCDEF")
    original = open(WORLD).read()
    files = {name: open(f"crates/devtools/recordings/{name}", "rb").read() for name in os.listdir("crates/devtools/recordings")}
    try:
        plants(original, wanted)
    finally:
        open(WORLD, "w").write(original)
        for name, data in files.items():
            open(f"crates/devtools/recordings/{name}", "wb").write(data)
    # The clean tree again: every check green.
    green = [run(*BUILD)[0], run("cargo", "test", "-p", "devtools", "--locked")[0], run(*NODE)[0], run(*CHROMIUM)[0],
             run("cargo", "run", "-q", "-p", "devtools", "--locked", "--bin", "make_recordings", "--", "--check")[0]]
    line = f"CLEAN {'GREEN' if green == [0] * 5 else 'NOT GREEN'}: build, native tests, Node, Chromium, make_recordings --check: exits {green}"
    OUT.append(line); print(line, flush=True)
    open(".checkpoints/cs4a-recording/plants-4a.txt", "w").write("\n".join(OUT) + "\n")

main()
