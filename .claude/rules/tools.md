---
paths:
  - "**/tools/**"
---
# Rules for the verification toolkit (never shipped)
- Keep every tool path-portable (resolve dependencies relative to tools/) and runnable on Windows and Linux.
- preview.js runs the real core + game in Node with headless WebGL1 and draws only the 3D scene (no DOM, HUD or CSS).
  pageshot.js renders the real pages in Chromium; headless means SwiftShader (CPU) unless run headed on the GPU.
  Headless timing is pessimistic and fonts differ from Brave: Mustafa's Brave check stays final.
- sim.js uses random AI: compare at least 3 runs per build, never one.
- battery.js is the authority for checks 3–4 and gate.mjs wraps it. After changing any gate tool, prove it still bites:
  a deliberately broken build (dropped function, unguarded editor reference) must FAIL.
