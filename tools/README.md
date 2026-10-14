# tools/: the verification toolkit (never shipped)

Nothing here is loaded by `index.html` or `editor.html`. The tools load the real game files into Node or Chromium to
check, render and simulate them. Run every command **from the repo root**; scratch output goes under `.gate/` (gitignored).

## Setup (once per machine)
Node **22** (headless-gl has prebuilt binaries up to Node 22; newer Node versions would need a C++ build).
```
cd tools
npm ci                                    # exact versions from package-lock.json
npx --no -- playwright install chromium   # only for pageshot.js; --no: use the pinned local playwright, never download another
```
`gl`, `@napi-rs/canvas` and `playwright` are optional dependencies: if one fails to install, the gate still works.

## Commands
| Command | What it does | Needs |
|---|---|---|
| `node tools/gate.mjs [baseRef] [--allow-drop a,b] [--json]` | **The gate**: checks 1–4 on the working tree vs a base ref (default: merge-base with `main`). Exit 0 PASS, 1 FAIL. CI runs it on every push and pull request. | acorn |
| `node tools/battery.js .` | Checks 3–4 as raw JSON (symbols per file, duplicates, editor references). | acorn |
| `node tools/preview.js . menu <out.png>` | Renders the real 3D scene headless (WebGL1). Also `shot <out.png> <x> <z>`, `top`, `eval <snippet.js>`, `evalshot`, `measure`, `time`, `list`. No HUD/DOM/CSS. | gl, @napi-rs/canvas, three, pngjs |
| `node tools/preview.js . eval tools/sim.js` | Headless 90 s match: bot stall %, harvesters. Random AI: compare 3+ runs. | as preview.js |
| `node tools/preview.js . eval tools/aabbs.js > .gate/aabbs.json` then `python tools/village.py` | Map layout export, then layout design + validation; writes `.gate/village.json` (paths: see the header of village.py). | as preview.js, Python 3 |
| `node tools/pageshot.js . index.html <prefix> [--headed] <steps…>` | Renders the real pages (DOM + HUD + CSS + WebGL) in Chromium. Steps: `wait:<ms> shot:<name> eval:<js> click:<css> hover:<css> key:<Key> mouse:<x>,<y> size:<w>x<h> motion:reduce`. | playwright + Chromium, three |

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
- **Never edit the gate tools to make a check pass.** After changing `gate.mjs` or `battery.js`, prove they still bite:
  a deleted function, an unguarded editor reference, a syntax error and a cross-file duplicate must each FAIL.
