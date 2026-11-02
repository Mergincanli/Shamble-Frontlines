import re
lines = open("orig-0002.md").read().split("\n")
r = [l for l in lines if l.startswith("| PM-0002-25 |")][0]
new = ("| PM-0002-78 | What the Worker costs as a whole (part 6: reported, no limit until these numbers exist) | reported, no pass number: the Worker's busy time per tick (all its work: the host's message handling, its Wasm calls and the block's post; median, p95, p99) at N = 48, online at P2 and in a local match; its JS heap and each instance's Wasm memory at minutes 5 and 10, the heap read after a forced garbage collection and set beside ADR 0003's PM-0003-24 flat rule (at most 0.5 MB of growth from minute 5 to 10), for reading, not as a pass; passes when every cell holds a number | "
"ML: Chrome | BX at N = 48: a 10-minute match in local mode and a 10-minute online session at P2; busy time from `performance.now()` at the start and end of each of the Worker's tasks (its message handlers and its timer callback); the heap through the Chrome DevTools Protocol on the Worker's target (proposed tool, not researched), Wasm memory from `memory.buffer.byteLength` | 1 run per mode | "
"the draft, part 6 (the Worker as a whole has no research budget; this record sets any limit from these numbers) and spike item 6; GLOSSARY, the client's sim share (lines 214–218: the step only); ADR 0003's PM-0003-24; the perf review of 2026-11-02, finding 13 |")
AFTER = r + "\n" + new
