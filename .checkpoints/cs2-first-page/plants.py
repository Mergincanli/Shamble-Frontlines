#!/usr/bin/env python3
"""Third version (2026-11-19, after round 2): the patterns of the browser plants are anchored on the failing line's
own mark (test-engineer's NT2: Playwright prints the neighbouring lines too, and a passing test's title); plant 7b
names the new limit; plants 14, 14b and 15 cover round 2's fixes (a Worker that cannot load, a Worker's error counted
twice, a refusal by the page's policy that no test saw); the tree must end as it started (it may hold a candidate
diff that is not committed).
The planted defects of change-set 2 (plan section 9), second version (2026-11-19), after test-engineer's review:
each plant now names the line its check must print (`must`), so a plant that is caught by a timeout or by another
test's failure does not count; plants 1, 3, 8 and 8.5 are the sharper ones it asked for; new plants cover the
build's own checks, the growth through the host, a post on every other frame and an allocation in the listener.
The planted defects of change-set 2 (plan section 9). Run inside a throwaway worktree that has its own build
folder and its own node_modules; nothing is committed. Each defect is applied, its named check must FAIL, and the
defect is taken back; at the end the clean tree must pass the same checks (the control).
  python3 -I plants.py <absolute path of the wasm-bindgen tool>"""
import os, re, subprocess, sys, time

TOOL = sys.argv[1]
ENV = dict(os.environ, SHAMBLE_WASM_BINDGEN=TOOL, SHAMBLE_DIST=os.path.abspath("packages/client/dist"),
           SHAMBLE_CHROMIUM="/opt/pw-browsers/chromium", CI="")
ENV.pop("CI")
BUILD = "node packages/client/scripts/build.mjs"
def e2e(spec): return f"node node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=chromium --trace off {spec}"
BOUNDARY = "cd packages/sim-worker && node ../../node_modules/vitest/vitest.mjs run --dir boundary"
CLIENT = "cd packages/client && node ../../node_modules/vitest/vitest.mjs run"

def at(pattern): return r"^\s*>\s*\d+ \|.*" + pattern  # the line Playwright marks as the one that failed
def sh(cmd):
    p = subprocess.run(cmd, shell=True, env=ENV, capture_output=True, text=True)
    return p.returncode, p.stdout + p.stderr

LAYOUT_OLD = """    Section { name: "header", stride: 8, fields: &[
        Field { name: "tick",        ty: ElementType::U32, offset: 0 },
        Field { name: "layout_hash", ty: ElementType::U32, offset: 4 },
    ] },"""
LAYOUT_NEW = """    Section { name: "header", stride: 12, fields: &[
        Field { name: "tick",        ty: ElementType::U32, offset: 0 },
        Field { name: "layout_hash", ty: ElementType::U32, offset: 4 },
        Field { name: "soldiers",    ty: ElementType::U32, offset: 8 },
    ] },"""
BOOT = "packages/client/src/app/boot.ts"; LINK = "packages/client/src/app/worker-link.ts"; HOST = "packages/sim-worker/src/host.ts"
PACING = "packages/sim-worker/src/pacing.ts"; READ = "packages/sim-worker/src/read-block.ts"; SERVE = "packages/client/serve.mjs"
PLANTS = [
 dict(n="1", what="the page calls into WebAssembly after it has started (PM-0002-2's own bite)", prep=[BUILD], check=e2e("skeleton.spec"), must=at(r"__wasmCalls\)\)\.toBe\(0\)"),
      edits=[(BOOT, "  link.start(flags.padBytes);", "  link.start(flags.padBytes);\n  try {\n    WebAssembly.validate(new Uint8Array(8));\n  } catch {\n    // counted by the test's stand-in\n  }")]),
 dict(n="2", what="a block posted without its transfer list (PM-0002-6)", prep=[BUILD], check=e2e("skeleton.spec"), must=at(r"undetached\)\.toBe\(0\)"),
      edits=[(HOST, "doneAt: this.#env.timeOrigin + before }, [buffer]);", "doneAt: this.#env.timeOrigin + before }, []);")]),
 dict(n="3", what="a copy that keeps its view of memory.buffer (PM-0002-6): the copy test alone", prep=[], check=BOUNDARY + " -t 'after the memory grows'", must=r"after the memory grows",
      edits=[(READ, "  new Uint8Array(into).set(new Uint8Array(memory.buffer, ptr, len));", "  kept ??= new Uint8Array(memory.buffer, ptr, len);\n  new Uint8Array(into).set(kept);"),
             (READ, "export function copyBlock(", "let kept: Uint8Array | undefined;\nexport function copyBlock(")]),
 dict(n="3b", what="a host that keeps its own view and copies around copyBlock (PM-0002-6): the host on the real Wasm", prep=[], check=BOUNDARY + " -t 'the host on the real Wasm'", must=r"the host on the real Wasm",
      edits=[(HOST, "    copyBlock(this.#env.memory, this.#blockPtr, this.#blockBytes, buffer);", "    this.kept ??= new Uint8Array(this.#env.memory.buffer, this.#blockPtr, this.#blockBytes);\n    new Uint8Array(buffer).set(this.kept);"),
             (HOST, "  #failed = false;\n", "  #failed = false;\n  kept: Uint8Array | undefined;\n")]),
 dict(n="4", what="a header field added in Rust without regenerating (PM-0002-16): the export test", prep=[], check="cargo test -p client-endpoint --locked", must=r"generated_layout_is_current \.\.\. FAILED",
      edits=[("crates/client-endpoint/src/layout.rs", LAYOUT_OLD, LAYOUT_NEW)]),
 dict(n="4b", what="the same field: the page's check refuses the block Rust now writes", prep=[BUILD + " --wasm-only"], check=BOUNDARY, must=r"layout-hash",
      edits=[("crates/client-endpoint/src/layout.rs", LAYOUT_OLD, LAYOUT_NEW)]),
 dict(n="5", what="a tool one patch off: the recorded tool version is 0.2.129 while Cargo.lock holds 0.2.130 (PM-0002-18)", prep=[], check=BUILD + " --wasm-only", must=r"^build: Cargo\.lock records wasm-bindgen 0\.2\.130 and wasm-bindgen-tool\.mjs the tool 0\.2\.129",
      edits=[("packages/client/scripts/wasm-bindgen-tool.mjs", 'version: "0.2.130",', 'version: "0.2.129",')]),
 dict(n="5b", what="the tool's recorded sha256 changed by one digit: the file is not run", prep=[], check=BUILD + " --wasm-only", must=r"^build: the wasm-bindgen tool at .* has sha256 .* not the recorded one: it is not run",
      edits=[("packages/client/scripts/wasm-bindgen-tool.mjs", 'sha256: "ca4df773', 'sha256: "da4df773')]),
 dict(n="6", what="a misspelt field name in the writer's lookup", prep=[], check="cargo build -p client-endpoint --locked", must=r"the header has a tick field",
      edits=[("crates/client-endpoint/src/block.rs", 'layout::field("header", "tick")', 'layout::field("header", "tik")')]),
 dict(n="7", what="a 64-byte typed array allocated per frame in the page's hand-off code (PM-0002-7's bite)", prep=[BUILD], check=e2e("allocation.spec"), must=at(r"expect\(perFrame\)\.toBeLessThan\(170\)"),
      edits=[(LINK, "    const returned = this.#returned;\n", "    const returned = this.#returned;\n    this.scratch = new Uint8Array(64);\n"),
             (LINK, "  pauses = 0;\n", "  pauses = 0;\n  scratch: Uint8Array | null = null;\n")]),
 dict(n="7b", what="a 64-byte typed array allocated per block where the message event is unpacked", prep=[BUILD], check=e2e("allocation.spec"), must=at(r"expect\(perBlock\)\.toBeLessThan\(610\)"),
      edits=[(LINK, "    return this.onMessage(event.data);", "    this.scratch = new Uint8Array(64);\n    return this.onMessage(event.data);"),
             (LINK, "  pauses = 0;\n", "  pauses = 0;\n  scratch: Uint8Array | null = null;\n")]),
 dict(n="8", what="a Worker that ignores the hidden tab and runs on (PM-0004-70's bite)", prep=[BUILD], check=e2e("skeleton.spec"), must=at(r"stillHidden\.worker\?\.ticks\)\.toBe\(hidden\)"),
      edits=[(HOST, "        if (data.hidden) {\n          this.#pacer.pause();\n          this.#clearTimer();\n        } else if", "        if (data.hidden) {\n          // ignored\n        } else if")]),
 dict(n="8b", what="a Worker that runs the hidden gap on the tab's return instead of dropping it (the 7-tick cap kept)", prep=[BUILD], check=e2e("skeleton.spec"), must=at(r"worker\?\.drops\)\.toBe\(0\)"),
      edits=[(PACING, "      this.#pauses += 1;\n      this.#nextDueMs = this.#sim.rebase(now);", "      this.#pauses += 1;")]),
 dict(n="9", what="a small server that lets a path out of its folder", prep=[], check=CLIENT, must=r"leaves the folder|leads outside",
      edits=[(SERVE, "  if (fromRoot === '' || fromRoot.startsWith('..') || path.isAbsolute(fromRoot)) return { status: 404 };", "  if (fromRoot === '') return { status: 404 };"),
             (SERVE, "  if (realFromRoot.startsWith('..') || path.isAbsolute(realFromRoot) || !info.isFile()) return { status: 404 };", "  if (!info.isFile()) return { status: 404 };")]),
 dict(n="9b", what="a small server that leaves the file of a broken-off download open", prep=[], check=CLIENT, must=r"leaves no file open",
      edits=[(SERVE, "  pipeline(createReadStream(found.file), response, () => {});", "  createReadStream(found.file).pipe(response);")]),
 dict(n="10", what="an isolation flag that sets COOP but not COEP (PM-0002-20's own bite)", prep=[BUILD], check=e2e("isolation.spec"), must=r"✘.*with the flag on .*crossOriginIsolated is true",
      edits=[("packages/client/vite.config.mjs", '    ? { "Cross-Origin-Opener-Policy": "same-origin", "Cross-Origin-Embedder-Policy": "require-corp" }', '    ? { "Cross-Origin-Opener-Policy": "same-origin" }')]),
 dict(n="11", what="the page posts its input on every other frame only", prep=[BUILD], check=e2e("skeleton.spec"), must=at(r"frames - posts\)\.toBeLessThanOrEqual\(1\)"),
      edits=[(BOOT, "    link.frame(sequence);", "    if (sequence % 2 === 0) link.frame(sequence);")]),
 dict(n="12", what="the build's switch is not folded in: development code in a production build", prep=[], check=BUILD + " --production --out-dir .gate/dist-prod", must=r"development code in a production build",
      edits=[(BOOT, "  if (__SHAMBLE_DEVTOOLS__ && frames !== null) {", "  if ((globalThis as { __never?: boolean }).__never !== true && frames !== null) {"),
             (BOOT, "  const frames = __SHAMBLE_DEVTOOLS__ ? new FrameTimes() : null;", "  const frames = new FrameTimes();")]),
 dict(n="14", what="a Worker whose script cannot be loaded: its error event has no message, and the page passes that on", prep=[BUILD], check=e2e("skeleton.spec"), must=at(r"The match stopped: "),
      edits=[(BOOT, '    stop(event.message || "the Worker failed");', "    stop(event.message);")]),
 dict(n="14b", what="a Worker's error is not cancelled, so the window counts it a second time", prep=[BUILD], check=e2e("skeleton.spec"), must=at(r"errors\)\.toBe\(1\)"),
      edits=[(BOOT, "    event.preventDefault();\n", "")]),
 dict(n="15", what="the page does something its content security policy refuses (an inline style)", prep=[BUILD], check=e2e("skeleton.spec"), must=at(r"later\.errors\)\.toBe\(0\)"),
      edits=[(BOOT, "  link.start(flags.padBytes);", '  link.start(flags.padBytes);\n  document.head.append(Object.assign(document.createElement("style"), { textContent: "body{}" }));')]),
 dict(n="13", what="the page imports the Wasm's glue itself", prep=[], check=BUILD, must=r"the Wasm's glue outside the Worker's chunk",
      edits=[(BOOT, 'import { WorkerLink } from "./worker-link.ts";', 'import { WorkerLink } from "./worker-link.ts";\nimport { tick_hz } from "../../../sim-worker/pkg/sim_wasm.js";'),
             (BOOT, "  link.start(flags.padBytes);", "  link.start(flags.padBytes);\n  console.info(tick_hz);")]),
]

def stamp(): return time.strftime("%Y-%m-%d %H:%M:%S UTC", time.gmtime())
def lines(out, show): return [l.rstrip()[:220] for l in out.splitlines() if re.search(show, l)][:4]

print(f"# planted defects, started {stamp()}")
START = sh("git diff -- crates packages")[1]
print(f"the tree at the start: HEAD {sh('git rev-parse HEAD')[1].strip()}, with {len(START.splitlines())} lines of uncommitted diff in crates/ and packages/")
bad = 0
for plant in PLANTS:
    saved = {}
    for path, old, new in plant["edits"]:
        text = open(path, encoding="utf-8").read()
        saved.setdefault(path, text)
        assert text.count(old) == 1, (plant["n"], path, old[:50])
        open(path, "w", encoding="utf-8").write(text.replace(old, new))
    prep_fail = ""
    for cmd in plant["prep"]:
        code, out = sh(cmd)
        if code != 0: prep_fail = f"(the build itself failed: exit {code}: {' / '.join(lines(out, 'build:|error')[:2])})"
    code, out = sh(plant["check"])
    for path, text in saved.items(): open(path, "w", encoding="utf-8").write(text)
    named = lines(out, plant["must"])
    caught = code != 0 and len(named) > 0
    verdict = "FAILED as it must, at the line that names the defect" if caught else ("PASSED: THE DEFECT WAS NOT CAUGHT" if code == 0 else "failed, BUT NOT AT THE LINE THAT NAMES THE DEFECT")
    bad += not caught
    print(f"\n## {plant['n']}. {plant['what']}\ncheck: {plant['check']}\nexit {code}: {verdict} {prep_fail}", flush=True)
    for l in named: print(f"    {l}")
    if not caught:
        for l in out.splitlines()[-12:]: print(f"    | {l.rstrip()[:200]}")
assert sh("git diff -- crates packages")[1] == START, "the tree is not as it started after the plants"
print(f"\n# control: the clean tree, {stamp()}")
for cmd in [BUILD, "cargo test --workspace --locked", BOUNDARY, CLIENT, e2e(""), BUILD + " --production --out-dir .gate/dist-prod"]:
    code, out = sh(cmd)
    bad += code != 0
    tail = [l.strip()[:160] for l in out.splitlines() if re.search(r"passed|failed|test result: .* [1-9][0-9]* passed|^build:|Tests ", l)][-3:]
    print(f"exit {code}: {cmd}\n    " + " | ".join(tail))
print(f"\n# finished {stamp()}: {'every defect was caught and the clean tree passes' if bad == 0 else str(bad) + ' PROBLEMS'}")
sys.exit(1 if bad else 0)
