#!/usr/bin/env python3
"""Build FIX D3E-2+D3E-3 for PM-0003-2 (docs/adr/0003-rendering.md): BEFORE copied from disk, AFTER by exact
sub-string replacements; checks 7 cells and that each replaced piece occurs once. Usage: mk_pm2.py <repo> <out>"""
import sys, pathlib
repo, out = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
L = (repo / 'docs/adr/0003-rendering.md').read_text(encoding='utf8').split('\n')
rows = [l for l in L if l.startswith('| PM-0003-2 |')]
assert len(rows) == 1, len(rows)
before = rows[0]
reps = [
 # D3E-3: the count closes at LS's end, before any teardown (destroy() resolves lost with "destroyed")
 ("and counts as a device error each `uncapturederror` event of the WebGPU device, its `lost` promise resolving, and "
  "each `webglcontextlost` event on the canvas (proposed, not researched: a WebGPU validation error reaches the page "
  "through the device's error event rather than as a thrown error, and whether",
  "and counts as a device error, until LS ends, each `uncapturederror` event of the WebGPU device, its `lost` promise "
  "resolving, and each `webglcontextlost` event on the canvas (the count closes before any teardown, since the "
  "device's `destroy()` also resolves `lost`, with reason \"destroyed\": WebGPU §22.1; the hook is proposed; as read "
  "at WebGPU §22 (basis), a validation error is not thrown but dispatched to the device, to an open error scope of "
  "its kind or else, if the browser chooses, as the device's `uncapturederror` event (§22.2–22.3); whether"),
 # D3E-2: the plant sits outside any error scope; a miss is told apart (plant, hook or browser)
 ("a planted bind group on the renderer's device that leaves out one entry of its layout, which must count as a "
  "device error whatever the console shows |",
  "a planted bind group on the renderer's device that leaves out one entry of its layout, made where a spy on "
  "`pushErrorScope` and `popErrorScope` shows no error scope open (an error made inside an open scope of its kind "
  "goes to that scope and fires no event: WebGPU §22.3) and with a bare `uncapturederror` listener of the bite's own "
  "beside the hook, which must count as a device error whatever the console shows; if the hook counts none by LS's "
  "end, the same plant inside the bite's own `pushErrorScope('validation')` tells why: no error popped, the plant is "
  "wrong; an error popped and the bare listener hit, the hook missed it; an error popped and the bare listener "
  "silent, the browser fired no event, which the spec allows (it fires one \"If the user agent chooses\" and may "
  "limit how many, §22.3–22.4); in none of the three does the pass count |"),
 # basis: the source read
 ("the device events proposed (the graphics review's finding 14) |",
  "the device events proposed (the graphics review's finding 14); the error and loss model: WebGPU, W3C Candidate "
  "Recommendation Draft of 15 September 2026, §22.1–22.4 and `destroy()` (read by the verifier, fixcheck-d3e, "
  "2026-11-03) |"),
]
after = before
for a, b in reps:
    assert after.count(a) == 1, ('piece count', after.count(a), a[:60])
    after = after.replace(a, b, 1)
def cells(r): return r.strip().strip('|').split(' | ')
assert len(cells(before)) == 7 and after.count('|') == before.count('|'), (len(cells(before)), after.count('|'), before.count('|'))
assert len(cells(after)) == 7
assert L.count(before) == 1
block = f"### FIX D3E-2+D3E-3\nFILE: docs/adr/0003-rendering.md\nBEFORE:\n{before}\nAFTER:\n{after}\nEND\n"
out.write_text(block, encoding='utf8')
print('cells', len(cells(after)), 'pipes', after.count('|'), 'len before', len(before), 'after', len(after))
