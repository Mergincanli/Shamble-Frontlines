#!/usr/bin/env python3
"""The planted defects of change-set 2 (plan section 9). Run inside a throwaway worktree that has its own build
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

def sh(cmd):
    p = subprocess.run(cmd, shell=True, env=ENV, capture_output=True, text=True)
    return p.returncode, p.stdout + p.stderr

PLANTS = [
 dict(n=1, what="the page calls WebAssembly.instantiate (PM-0002-2's own bite)", prep=[BUILD], check=e2e("skeleton.spec"), show=r"__wasmCalls|Expected: 0|Received:",
      edits=[("packages/client/src/app/boot.ts", "  const flags = readFlags(location.search, DEVTOOLS);",
              "  void WebAssembly.instantiate(new Uint8Array([0, 97, 115, 109, 1, 0, 0, 0])).catch(() => {});\n  const flags = readFlags(location.search, DEVTOOLS);")]),
 dict(n=2, what="a block posted without its transfer list (PM-0002-6)", prep=[BUILD], check=e2e("skeleton.spec"), show=r"undetached|Expected: 0|Received:",
      edits=[("packages/sim-worker/src/host.ts", "doneAt: this.#env.timeOrigin + doneAt }, [buffer]);", "doneAt: this.#env.timeOrigin + doneAt }, []);")]),
 dict(n=3, what="a host that keeps its view of memory.buffer (PM-0002-6)", prep=[], check=BOUNDARY, show=r"FAIL|after the memory grows|passed|failed",
      edits=[("packages/sim-worker/src/read-block.ts", "  new Uint8Array(into).set(new Uint8Array(memory.buffer, ptr, len));",
              "  kept ??= new Uint8Array(memory.buffer, ptr, len);\n  new Uint8Array(into).set(kept);"),
             ("packages/sim-worker/src/read-block.ts", "export function copyBlock(", "let kept: Uint8Array | undefined;\nexport function copyBlock(")]),
 dict(n=4, what="a header field added in Rust without regenerating (PM-0002-16): the export test", prep=[], check="cargo test -p client-endpoint --locked", show=r"generated_layout_is_current|test result|panicked",
      edits=[("crates/client-endpoint/src/layout.rs", '''    Section { name: "header", stride: 8, fields: &[
        Field { name: "tick",        ty: ElementType::U32, offset: 0 },
        Field { name: "layout_hash", ty: ElementType::U32, offset: 4 },
    ] },''', '''    Section { name: "header", stride: 12, fields: &[
        Field { name: "tick",        ty: ElementType::U32, offset: 0 },
        Field { name: "layout_hash", ty: ElementType::U32, offset: 4 },
        Field { name: "soldiers",    ty: ElementType::U32, offset: 8 },
    ] },''')]),
 dict(n=4.5, what="the same field: the page's check refuses the block Rust now writes", prep=[BUILD + " --wasm-only"], check=BOUNDARY, show=r"FAIL|layout-hash|passed|failed",
      edits=[("crates/client-endpoint/src/layout.rs", '''    Section { name: "header", stride: 8, fields: &[
        Field { name: "tick",        ty: ElementType::U32, offset: 0 },
        Field { name: "layout_hash", ty: ElementType::U32, offset: 4 },
    ] },''', '''    Section { name: "header", stride: 12, fields: &[
        Field { name: "tick",        ty: ElementType::U32, offset: 0 },
        Field { name: "layout_hash", ty: ElementType::U32, offset: 4 },
        Field { name: "soldiers",    ty: ElementType::U32, offset: 8 },
    ] },''')]),
 dict(n=5, what="a tool one patch off: the recorded tool version is 0.2.129 while Cargo.lock holds 0.2.130 (PM-0002-18)", prep=[], check=BUILD + " --wasm-only", show=r"^build:",
      edits=[("packages/client/scripts/wasm-bindgen-tool.mjs", 'version: "0.2.130",', 'version: "0.2.129",')]),
 dict(n=6, what="a misspelt field name in the writer's lookup", prep=[], check="cargo build -p client-endpoint --locked", show=r"error|no field of this name|evaluation",
      edits=[("crates/client-endpoint/src/block.rs", 'layout::field("header", "tick")', 'layout::field("header", "tik")')]),
 dict(n=7, what="a 64-byte typed array allocated per frame in the page's hand-off code (PM-0002-7's bite)", prep=[BUILD], check=e2e("allocation.spec"), show=r"allocation in|Expected|Received|passed|failed",
      edits=[("packages/client/src/app/worker-link.ts", "    const returned = this.#returned;\n", "    const returned = this.#returned;\n    this.scratch = new Uint8Array(64);\n"),
             ("packages/client/src/app/worker-link.ts", "  pauses = 0;\n", "  pauses = 0;\n  scratch: Uint8Array | null = null;\n")]),
 dict(n=8, what="a Worker that does not pause for a hidden tab (PM-0004-70's bite)", prep=[BUILD], check=e2e("skeleton.spec"), show=r"hidden tab|Expected|Received|passed|failed",
      edits=[("packages/sim-worker/src/pacing.ts", "  pause(): void {\n    this.#paused = true;\n  }", "  pause(): void {\n    this.#paused = false;\n  }")]),
 dict(n=8.5, what="a Worker that runs the hidden gap on the tab's return instead of dropping it", prep=[BUILD], check=e2e("skeleton.spec"), show=r"hidden tab|Expected|Received|passed|failed",
      edits=[("packages/sim-worker/src/pacing.ts", "      this.#pauses += 1;\n      this.#nextDueMs = this.#sim.rebase(now);", "      this.#pauses += 1;"),
             ("packages/sim-worker/src/pacing.ts", "    while (ran < MAX_TICKS_PER_WAKE && this.#nextDueMs <= now) {", "    while (this.#nextDueMs <= now) {")]),
 dict(n=9, what="a small server that lets a path out of its folder", prep=[], check=CLIENT, show=r"FAIL|leaves the folder|leads outside|passed|failed",
      edits=[("packages/client/serve.mjs", "  if (fromRoot === '' || fromRoot.startsWith('..') || path.isAbsolute(fromRoot)) return { status: 404 };", "  if (fromRoot === '') return { status: 404 };"),
             ("packages/client/serve.mjs", "  if (realFromRoot.startsWith('..') || path.isAbsolute(realFromRoot) || !info.isFile()) return { status: 404 };", "  if (!info.isFile()) return { status: 404 };")]),
 dict(n=10, what="an isolation flag that sets COOP but not COEP (PM-0002-20's own bite)", prep=[BUILD], check=e2e("isolation.spec"), show=r"flag on|Expected|Received|passed|failed",
      edits=[("packages/client/vite.config.mjs", '    ? { "Cross-Origin-Opener-Policy": "same-origin", "Cross-Origin-Embedder-Policy": "require-corp" }', '    ? { "Cross-Origin-Opener-Policy": "same-origin" }')]),
]

def stamp(): return time.strftime("%Y-%m-%d %H:%M:%S UTC", time.gmtime())
def lines(out, show): return [l.rstrip()[:200] for l in out.splitlines() if re.search(show, l)][:8]

print(f"# planted defects, started {stamp()}")
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
    verdict = "FAILED as it must" if code != 0 else "PASSED: THE DEFECT WAS NOT CAUGHT"
    bad += code == 0
    print(f"\n## {plant['n']}. {plant['what']}\ncheck: {plant['check']}\nexit {code}: {verdict} {prep_fail}")
    for l in lines(out, plant["show"]): print(f"    {l}")
assert sh("git status --short -- crates packages")[1].strip() == "", "the tree is not clean after the plants"
print(f"\n# control: the clean tree, {stamp()}")
for cmd in [BUILD, "cargo test --workspace --locked", BOUNDARY, CLIENT, e2e("")]:
    code, out = sh(cmd)
    bad += code != 0
    tail = [l.strip()[:160] for l in out.splitlines() if re.search(r"passed|failed|test result: .* [1-9][0-9]* passed|^build:|Tests ", l)][-3:]
    print(f"exit {code}: {cmd}\n    " + " | ".join(tail))
print(f"\n# finished {stamp()}: {'every defect was caught and the clean tree passes' if bad == 0 else str(bad) + ' PROBLEMS'}")
sys.exit(1 if bad else 0)
