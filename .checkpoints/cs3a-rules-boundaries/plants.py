#!/usr/bin/env python3
"""Planted defects for change-set 3a: each plant breaks one thing in a source file, runs the check that should catch
it, and puts the file back. A plant counts as caught only when the run fails AND its output holds the plant's `must`
pattern on a line that marks a failure (vitest's FAIL line of the named test, cargo's panic text, or Playwright's
code frame line with its '>' mark), never because some other check failed.

Run from the repository's root, after a clean build:  python3 -B .checkpoints/cs3a-rules-boundaries/plants.py [ids...]
It refuses to start on a tree whose tracked source differs from what it would restore to (it restores by content).
"""
import pathlib
import re
import subprocess
import sys
import time

ROOT = pathlib.Path.cwd()
WB = str(ROOT / ".gate/wb-tool/x/wasm-bindgen")
OUT = ROOT / ".gate/cs3a"
VITEST = ["node", "node_modules/vitest/vitest.mjs", "run"]
CLIENT = VITEST + ["--root", "packages/client"]
WORKER = VITEST + ["--root", "packages/sim-worker", "--dir", "packages/sim-worker/src"]
SCAN = ["cargo", "test", "-p", "sim", "--locked", "--test", "rules_scan"]
BUILD = ["node", "packages/client/scripts/build.mjs", "--out-dir", str(ROOT / ".gate/dist-plant")]
E2E = ["node", "node_modules/@playwright/test/cli.js", "test", "-c", "packages/e2e/playwright.config.ts", "--project=chromium",
       "--workers=1", "--retries=0", "--reporter=list", "--global-timeout=300000", "--trace", "off"]


def vitest_fail(title):
    """vitest prints ' FAIL  <file> > <title>' (and a line with a cross) for a failed test only."""
    return r"(FAIL|×).*" + re.escape(title)


def frame(source):
    """Playwright's code frame marks only the failing line with '>'."""
    return r"^\s*>\s*\d+ \|.*" + re.escape(source)


# id, what is planted, [(file, old, new)], commands (each a list; a build first for the browser ones), must
PLANTS = [
    ("f8-unknown", "the link takes a message it does not know as nothing",
     [("packages/client/src/app/worker-link.ts",
       'if (!isWorkerToPage(data)) return this.#fail("the Worker sent a message the page does not know");',
       'if (!isWorkerToPage(data)) return "";')],
     [CLIENT + ["src/app/worker-link.test.ts"]], vitest_fail("a block of the wrong length, an error from the Worker and an unknown message")),
    ("f8-stats-field", "the statistics check leaves its first field out",
     [("packages/sim-worker/src/messages.ts", "for (let i = 0; i < STATS_KINDS.length; i += 1) {", "for (let i = 1; i < STATS_KINDS.length; i += 1) {")],
     [WORKER + ["messages.test.ts"]], vitest_fail("the statistics are refused when one field is missing")),
    ("f8-done-at", "a block whose time is not a number is taken",
     [("packages/sim-worker/src/messages.ts", ' && Number.isFinite(value["doneAt"]);', ";")],
     [WORKER + ["messages.test.ts"]], vitest_fail("anything else from the Worker is refused")),
    ("n3-no-terminate", "the end of the match does not stop the Worker",
     [("packages/client/src/app/worker-link.ts", "    this.#port.terminate();\n", "")],
     [CLIENT + ["src/app/worker-link.test.ts"]], vitest_fail("the match ends once, whoever ends it first")),
    ("n3-ends-twice", "a second cause ends the match again",
     [("packages/client/src/app/worker-link.ts", '    if (this.#failure !== "") return false;\n', "")],
     [CLIENT + ["src/app/worker-link.test.ts"]], vitest_fail("the match ends once, whoever ends it first")),
    ("n3-posts-after", "the link still posts the tab's visibility after the end",
     [("packages/client/src/app/worker-link.ts", '    if (this.#failure !== "") return;\n    this.#port.postMessage({ type: "visibility", hidden }, []);',
       '    this.#port.postMessage({ type: "visibility", hidden }, []);')],
     [CLIENT + ["src/app/worker-link.test.ts"]], vitest_fail("the match ends once, whoever ends it first")),
    ("f13-writable", "a field of the link is writable from outside again",
     [("packages/client/src/app/worker-link.ts", "  get posts(): number {\n    return this.#posts;\n  }\n",
       "  get posts(): number {\n    return this.#posts;\n  }\n\n  set posts(value: number) {\n    this.#posts = value;\n  }\n")],
     [CLIENT + ["src/app/worker-link.test.ts"]], vitest_fail("what the link knows can be read and not written from outside")),
    ("fps-median", "frames a second from the usual gap again",
     [("packages/client/src/harness/frame-times.ts", "const mean = this.#deltas.mean();", "const mean = this.#deltas.percentile(50);")],
     [CLIENT + ["src/harness/frame-times.test.ts"]], vitest_fail("frames a second is a count over time")),
    ("mean-capacity", "the ring's mean divides by its capacity",
     [("packages/sim-worker/src/stats.ts", "sum / this.#count;", "sum / this.#values.length;")],
     [WORKER + ["stats.test.ts"]], vitest_fail("the mean is over the samples held, and only those")),
    ("neg-zero", "a value that rounds to zero keeps its minus sign",
     [("packages/client/src/overlay/report.ts", 'return Number(text) === 0 ? text.replace("-", "") : text;', "return text;")],
     [CLIENT + ["src/overlay/report.test.ts"]], vitest_fail("a value that rounds to zero prints as zero")),
    ("nt3-control", "the error text keeps its control characters",
     [("packages/client/src/app/error-hook.ts", '.replace(/[\\u0000-\\u001f\\u007f]+/g, " ")', "")],
     [CLIENT + ["src/app/error-hook.test.ts"]], vitest_fail("the newest error's text is kept as one line of bounded length")),
    ("nt3-length", "the error text is not cut",
     [("packages/client/src/app/error-hook.ts", ".slice(0, 300)", "")],
     [CLIENT + ["src/app/error-hook.test.ts"]], vitest_fail("the newest error's text is kept as one line of bounded length")),
    ("nt4-after-copy", "the hand-off's time is read after the copy",
     [("packages/sim-worker/src/host.ts", "this.#env.post({ type: \"block\", buffer, doneAt: this.#env.timeOrigin + before }, [buffer]);",
       "this.#env.post({ type: \"block\", buffer, doneAt: this.#env.timeOrigin + this.#env.now() }, [buffer]);")],
     [WORKER + ["host.test.ts"]], vitest_fail("the hand-off's time is read when the step returned, before the copy")),
    ("favicon", "the icon request falls through to the files",
     [("packages/client/serve.mjs", "if ((request.url ?? '').split(/[?#]/, 1)[0] === '/favicon.ico') {", "if (false) {")],
     [CLIENT + ["serve.test.mjs"]], vitest_fail("the browser's own request for an icon")),
    ("l4-refusal-policy", "a refusal goes out without the policy header",
     [("packages/client/serve.mjs", "  response.writeHead(status, {\n    ...COMMON_HEADERS,",
       "  response.writeHead(status, {\n    'X-Content-Type-Options': 'nosniff',\n    'Cache-Control': 'no-store',\n    'Cross-Origin-Resource-Policy': 'same-origin',")],
     [CLIENT + ["serve.test.mjs"]], vitest_fail("every method but GET and HEAD")),
    ("tel-lexical", "a path that leaves the folder reaches the file system",
     [("packages/client/serve.mjs", "if (fromRoot === '' || fromRoot.startsWith('..') || path.isAbsolute(fromRoot)) return { status: 404 };",
       "if (fromRoot === '' || path.isAbsolute(fromRoot)) return { status: 404 };")],
     [CLIENT + ["serve.test.mjs"]], vitest_fail("a path that leaves the folder, or names the folder itself, before the file system is asked")),
    ("tel-empty", "the folder itself reaches the file system",
     [("packages/client/serve.mjs", "if (fromRoot === '' || fromRoot.startsWith('..') || path.isAbsolute(fromRoot)) return { status: 404 };",
       "if (fromRoot.startsWith('..') || path.isAbsolute(fromRoot)) return { status: 404 };")],
     [CLIENT + ["serve.test.mjs"]], vitest_fail("a path that leaves the folder, or names the folder itself, before the file system is asked")),
    ("tel-slash", "an address without a leading slash is looked up",
     [("packages/client/serve.mjs", "  if (!rawPath.startsWith('/')) return { status: 400 };\n", "")],
     [CLIENT + ["serve.test.mjs"]], vitest_fail("an address that does not start with a slash")),
    ("tel-not-a-file", "a folder with a served ending is answered as a file",
     [("packages/client/serve.mjs", " || !info.isFile()) return { status: 404 };", ") return { status: 404 };")],
     [CLIENT + ["serve.test.mjs"]], vitest_fail("a folder whose name has a served ending")),
    ("n6-prefix", "the page's chunks are told from the Worker's by a name prefix again",
     [("packages/client/scripts/build-checks.mjs", "const page = scripts.filter(({ name }) => name !== workerEntry);",
       "const page = scripts.filter(({ name }) => !/(^|\\/)worker-[^/]*\\.js$/.test(name));")],
     [CLIENT + ["scripts/build-checks.test.mjs"]], vitest_fail("the glue stays in the Worker's chunk")),
    ("n6-not-entry", "a chunk that is no entry counts as the Worker's",
     [("packages/client/scripts/build-checks.mjs", "chunk.isEntry === true && ", "")],
     [CLIENT + ["scripts/build-checks.test.mjs"]], vitest_fail("the Worker's chunk is the one entry chunk the build made")),
    ("alloc-every-call", "the state hash frees and takes its buffer on every call",
     [("crates/sim/src/world.rs", "        out.clear();\n        out.reserve(STATE_BYTES);", "        out.clear();\n        out.shrink_to_fit();\n        out.reserve(STATE_BYTES);")],
     [["cargo", "test", "-p", "sim", "--locked", "--test", "state_hash_alloc"]], r"a later call took memory"),
    # --- step 2: the rule scan on the simulation crate itself, and the planted counter
    ("scan-hash", "a std hash map enters the simulation crate",
     [("crates/sim/src/world.rs", "use crate::state_hash::fnv1a_64;\n", "use crate::state_hash::fnv1a_64;\n#[allow(unused_imports)]\nuse std::collections::HashMap;\n")],
     [SCAN], r"world\.rs:6: `HashMap`"),
    ("scan-float", "a float method outside the list enters the simulation crate",
     [("crates/sim/src/world.rs", "/// The length of a match seed in bytes.", "pub fn planted(x: f32) -> f32 {\n    x.sin()\n}\n\n/// The length of a match seed in bytes.")],
     [SCAN], r"world\.rs:\d+: `sin`"),
    ("scan-clock", "a clock enters the simulation crate",
     [("crates/sim/src/world.rs", "/// The length of a match seed in bytes.", "pub fn planted() -> bool {\n    Instant::now().elapsed().is_zero()\n}\nuse std::time::Instant;\n\n/// The length of a match seed in bytes.")],
     [SCAN], r"world\.rs:\d+: `Instant`"),
    ("scan-method-form", "the tick's bytes are taken in the method form again",
     [("crates/sim/src/world.rs", "u32::to_le_bytes(*tick)", "tick.to_le_bytes()")],
     [SCAN], r"world\.rs:\d+: `to_le_bytes`"),
    ("overflow-off", "the release profile's overflow checks are turned off (by a cargo setting on the command line; no file is edited)",
     [], [["cargo", "test", "--release", "-p", "sim", "--locked", "--lib", "--config", "profile.release.overflow-checks=false"]],
     r"overflow_checks_are_on - should panic \.\.\. FAILED"),
    # --- in the browser: a build of the planted tree, then one spec
    ("nt5-stale", "the Worker runs on while hidden and the page asks for no report while hidden",
     [("packages/sim-worker/src/host.ts", "        if (data.hidden) {\n          this.#pacer.pause();\n          this.#clearTimer();\n        } else if",
       "        if (data.hidden) {\n          // planted: nothing\n        } else if"),
      ("packages/client/src/app/boot.ts", "      link.requestReport();\n", "      if (!document.hidden) link.requestReport();\n")],
     [BUILD, E2E + ["-g", "a hidden tab pauses the match"]], frame("expect(stillHidden.tick).toBe(atRead.tick);")),
    ("te-104", "a resume runs 90 ticks at once and counts no drop",
     [("packages/sim-worker/src/pacing.ts", "      this.#pauses += 1;\n      this.#nextDueMs = this.#sim.rebase(now);",
       "      this.#pauses += 1;\n      for (let i = 0; i < 90; i += 1) {\n        this.#sim.step();\n        this.#ticks += 1;\n      }\n      this.#nextDueMs = this.#sim.rebase(now);")],
     [BUILD, E2E + ["-g", "a hidden tab pauses the match"]], frame("- hidden).toBeLessThanOrEqual(most);")),
    ("two-workers", "the page starts a second Worker beside the first",
     [("packages/client/src/app/boot.ts", "  const worker = new SimWorker();\n", "  const worker = new SimWorker();\n  new SimWorker();\n")],
     [BUILD, E2E + ["-g", "the first page on the default backend"]], frame("expect(page.workers()).toHaveLength(1);")),
]


def run(command):
    env = dict(__import__("os").environ)
    env["SHAMBLE_WASM_BINDGEN"] = WB
    env["SHAMBLE_DIST"] = str(ROOT / ".gate/dist-plant")
    env["SHAMBLE_CHROMIUM"] = "/opt/pw-browsers/chromium"
    done = subprocess.run(command, cwd=ROOT, env=env, capture_output=True, text=True, timeout=600)
    return done.returncode, done.stdout + done.stderr


def main():
    wanted = set(sys.argv[1:])
    OUT.mkdir(parents=True, exist_ok=True)
    rows = []
    for plant_id, what, edits, commands, must in PLANTS:
        if wanted and plant_id not in wanted:
            continue
        originals = {}
        for name, old, new in edits:
            text = originals.get(name) or (ROOT / name).read_text()
            originals.setdefault(name, text)
        try:
            current = dict(originals)
            for name, old, new in edits:
                assert current[name].count(old) == 1, f"{plant_id}: the text to replace is not there exactly once in {name}"
                current[name] = current[name].replace(old, new)
                (ROOT / name).write_text(current[name])
            output, code = "", 0
            for command in commands:
                code, text = run(command)
                output += f"$ {' '.join(command)}\n{text}\n"
                if code != 0:
                    break
        finally:
            for name, text in originals.items():
                (ROOT / name).write_text(text)
        (OUT / f"plant-{plant_id}.out").write_text(output)
        hit = [line for line in output.splitlines() if re.search(must, line)]
        caught = code != 0 and bool(hit)
        rows.append((plant_id, what, "CAUGHT" if caught else "NOT CAUGHT", (hit[0].strip() if hit else f"exit {code}, no line matches")[:150]))
        print(f"{rows[-1][2]:10} {plant_id:18} {rows[-1][3]}", flush=True)
    stamp = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
    print(f"{stamp}: {sum(1 for row in rows if row[2] == 'CAUGHT')} of {len(rows)} caught")
    return 0 if all(row[2] == "CAUGHT" for row in rows) else 1


if __name__ == "__main__":
    sys.exit(main())
