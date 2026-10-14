# tools/: the verification toolkit (never shipped)

Nothing here is loaded by `index.html` or `editor.html`. The tools load the real game files into Node or Chromium to
check, render and simulate them. Run every command **from the repo root**; scratch output goes under `.gate/` (gitignored).

## Setup (once per machine)
Node **22**, 22.13 or newer (headless-gl has prebuilt binaries up to Node 22; eslint-scope needs 22.13+).
```
cd tools
npm ci                                    # exact versions from package-lock.json
npx --no -- playwright install chromium   # for the gate's smoke test (5a) and pageshot.js; --no: never download another playwright
```
`gl`, `@napi-rs/canvas` and `playwright` are optional dependencies. Without Playwright and its Chromium the gate's 5a
smoke FAILS with this setup command (it never skips silently); `--no-smoke` runs everything else.

## Commands
| Command | What it does | Needs |
|---|---|---|
| `node tools/gate.mjs [baseRef] [--allow-drop a,b] [--no-smoke \| --smoke-only] [--json]` | **The gate**: checks 1–4 and 5a–5d on the working tree vs a base ref (default: merge-base with `main` or `origin/main`, whichever is newer). About 30 s with the smoke test, 2 s with `--no-smoke`. Exit 0 PASS, 1 FAIL, 2 setup error. CI runs it on every push and pull request (two jobs: `gate` with `--no-smoke`, `smoke` with `--smoke-only`). | acorn, eslint-scope; 5a: playwright + Chromium |
| `node tools/smoke.mjs [--json]` | Check 5a alone: the real pages in headless Chromium (index: menu → Deploy → tutorial → W + two shots, ~5 s of play → Esc → Menu; editor: Map editor → Maps → close → Menu). FAILS on any page error, console error or request that leaves the repo; warnings are listed only. | playwright + Chromium, three |
| `node tools/battery.js .` | Checks 3–4 as raw JSON (symbols per file, duplicates, editor references). | acorn |
| `node tools/preview.js . menu <out.png>` | Renders the real 3D scene headless (WebGL1). Also `shot <out.png> <x> <z>`, `top`, `eval <snippet.js>`, `evalshot`, `measure`, `time`, `list`. No HUD/DOM/CSS. | gl, @napi-rs/canvas, three, pngjs |
| `node tools/preview.js . eval tools/sim.js` | Headless 90 s match: bot stall %, harvesters. Random AI: compare 3+ runs. | as preview.js |
| `node tools/preview.js . eval tools/aabbs.js > .gate/aabbs.json` then `python tools/village.py` | Map layout export, then layout design + validation; writes `.gate/village.json` (paths: see the header of village.py). | as preview.js, Python 3 |
| `node tools/pageshot.js . index.html <prefix> [--headed] <steps…>` | Renders the real pages (DOM + HUD + CSS + WebGL) in Chromium. Steps: `wait:<ms> shot:<name> eval:<js> click:<css> hover:<css> key:<Key> mouse:<x>,<y> size:<w>x<h> motion:reduce`. | playwright + Chromium, three |

**Check 5 in short** (the gate's own header has the details):
- **5a smoke**: the pages actually run (load-time crashes, TDZ, a missing script, an error in deploy or the editor).
- **5b names**: every bare name resolves (eslint-scope over each page's scripts and inline `on…="…"` handlers) to a
  local, a top-level declaration of that page's build, an ECMAScript built-in, or `tools/gate-globals.json`. A new
  browser API means one line in that list. `typeof x` operands are exempt; index.html's references to editor-only
  symbols are check 4's (it proves they are guarded). Handlers resolve against the page's globals only: write
  `document.body`, not `body`.
- **5c layering**: `shamble-core.js` references nothing declared in game, editor or the inline scripts, not even
  behind `typeof`.
- **5d script order**: each page loads exactly three r128 (cdnjs) → core → game [→ editor] → one inline script, with no
  `async`, `defer`, `nomodule` or `type="module"`.

**pageshot.js renderer:** headless runs on SwiftShader (CPU). `--headed` opens a real window on the GPU. Every run
prints the WebGL renderer string, and every `shot` line repeats it, so you can tell an RTX render from a SwiftShader one.
Frame times and fonts only count in Brave (DEV panel, backtick).

## Check a new machine (about 10 minutes, from the repo root)
```
cd tools && npm ci && npx --no -- playwright install chromium && cd ..
node tools/gate.mjs                                    # must end with GATE PASS
node tools/pageshot.js . index.html .gate/gpu --headed size:1600x900 wait:2500 shot:menu "eval:(deploy(),0)" wait:1500 "eval:(closeTutorial(),setPause(true),0)" wait:600 shot:pause
                                                       # the renderer line must name your GPU, not SwiftShader
node tools/preview.js . menu .gate/preview_menu.png    # tells you whether headless-gl works natively here
node --version                                         # 22.x
```
Then, in Claude Code at the repo root, ask it to put a syntax error into `shamble-game.js` with the Edit tool and revert
it: the syntax hook must answer "SYNTAX GATE FAILED". Paste the outputs to the Advisor.

## Platform notes
- **Windows:** no prefix needed. If `gl` fails to install natively, the gate is unaffected; report it and use WSL for preview.js.
- **Linux:** preview.js and `pageshot.js --headed` need a display: prefix them with `xvfb-run -a`.
- **Never edit the gate tools to make a check pass.** After changing `gate.mjs`, `battery.js` or `smoke.mjs`, prove they
  still bite: a deleted function, an unguarded editor reference, a syntax error and a cross-file duplicate must each
  FAIL, and so must the check-5 mutations in `docs/reports/2026-10-14-gate-check-5.md` (a function hidden inside
  another, a TDZ at load, swapped script tags, core calling game, a throw in `deploy()`).
