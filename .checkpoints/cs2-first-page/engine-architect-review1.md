# engine-architect, review round 1 of cs2-first-page (head 30a5a13), started 2026-11-19T00:29Z

Plan: read plan.md (0, 4.1-4.4, 5, 8) and the resume card's departures; read ADR 0002 parts 3/5/6/7, ADR 0004 parts 4/5/8/9, build-plan card :298-314 and Part 15.1/15.2; then read picture.ts + render/ + app/ + overlay/ + harness/ imports, the sim-worker package (host, worker, pacing, block-pool, read-block, messages), worker-link.ts, boot.ts, sim-wasm lib.rs, client-endpoint, sim, manifests and build scripts; run cargo test/clippy --locked and the vitest unit tests; write each finding here as it comes; answer Q1-Q6.

## Findings (as made)


### F1 (Medium) app/boot.ts:5 reaches render/ around the interface file (00:35Z)
`import { createThreePicture } from "../render/three-picture.ts"`: the one value import from app/ into render/. picture.ts:3-4
says "the app, the overlay and the harness reach the picture through this file alone" and plan 4.3 says the hand-over
at cs5 "moves no import"; today lane 2 renaming its own file or factory at cs6 forces an edit of boot.ts (lane 1's), or
lane 1 freezes a file name inside a folder it no longer owns. Everything else holds: three is imported only in
render/three-picture.ts:1 and render/empty-scene.ts:1; overlay/report.ts:4 takes picture types only (type import);
harness/ never touches render/. Smallest fix: one named entry for the folder, fixed by the interface file: picture.ts
declares the factory's type (e.g. `export type CreatePicture = () => Picture`) and names `render/index.ts`'s
`createPicture` as render/'s only entry; boot.ts imports that; lane 2 may reorganise everything else in render/.

### F2 (Medium) a page-side failure leaves the Worker running and counts one error per block (00:38Z)
worker-link.ts:89-95 and :111-112 with boot.ts:40-43: after a layout or length failure the link stops posting input
(so no buffer ever returns), but nothing tells the Worker; it keeps stepping at 30 Hz, BlockPool.take() misses on every
tick and allocates a fresh block-size buffer (block-pool.ts:41-42), the page drops it, and #takeBlock returns the
failure again for every block (#layoutChecked stays false), so boot.ts notes one error per block: 30 a second, the
readout's "errors" climbs for one cause, and from cs4 on (blocks of the 48-soldier size) the failed match churns
block-size garbage until reload. Smallest fix: note the failure once (onMessage returns "" for blocks once failure is
set) and stop the Worker from the page (boot.ts owns the Worker object: worker.terminate() on the first problem), or a
"stop" message if the Worker must report first.

### F3 (Low) the interface does not state how long a block is valid (picture.ts:19-25)
PictureFrame hands raw ArrayBuffers; the link transfers a block back to the Worker on the first frame after two newer
blocks arrived (worker-link.ts:97, :113-115), so a view the picture keeps past its draw call reads a detached buffer:
length 0, undefined/NaN, no throw. Nothing reads blocks yet (three-picture.ts:33-36), so no wrong result today, but
this is the contract lane 2 codes against after cs5. Fix: one sentence in picture.ts: the blocks are valid during
this draw call only; keep no view on them past it.

### Picture.loop: right (checked 00:39Z)
three r187 starts its own rAF loop on the first render() (node_modules/three/src/renderers/common/Renderer.js:1827) and
resets renderer.info at the start of each of its frames (Animation.js:83, autoReset); Renderer.js:2380-2381 itself says
to run the app's loop "with this method and not manually with requestAnimationFrame() for best compatibility". So the
frame callback on setAnimationLoop is the right cut. Consequence to record: the frame clock that carries lane 1's
input post now runs inside render/ (lane 2's after cs5); picture.ts:39-44 should pin it: frame runs exactly once per
display frame from start() to stop(), never withheld (warm-up and quality changes included).

### F4 (Low) viewTime: wrong clock and unit for the contract it stands for
messages.ts:17 / worker-link.ts:111-115 / boot.ts:97-99: viewTime is the rAF time on the page's own time origin
(picture.ts:40 says "performance.now()"), while doneAt is on the shared clock (timeOrigin + now, messages.ts:28-31) and
the Worker's now() is on the Worker's own time origin (host.ts:19-21); ADR 0002 part 5 means "the view time of the frame
the page drew", which the protocol already types as ticks plus a fraction (packages/types/src/generated/protocol/
ViewTime.ts: tick, fraction in 1/65,536). The Worker ignores it (host.ts:85-88). Fix: drop it until the change-set that
uses it (cs3 rewrites this message anyway), or define it now as the drawn tick and fraction; never page milliseconds.

### F5 (Low) constants copied by hand across the boundary
pacing.ts:7 MAX_TICKS_PER_WAKE = 7 is floor(0.25 x 30) from sim TICK_HZ (world.rs:7); the host gets tickHz from Rust
(worker.ts:34) but the cap ignores it, so a 60 Hz rebuild (ADR 0004 part 4's margin) silently caps at 0.117 s, not 0.25 s.
Also worker-link.ts:101 reads the tick at literal offset 0 though BLOCK_LAYOUT.header.fields.tick.offset is generated
for that (layout-check.ts uses the generated offset); MODE_LOCAL (messages.ts:7 vs sim-wasm lib.rs:15), the pad cap
(flags.ts:18 vs block.rs:17) and the seed length (worker.ts:19 vs SEED_BYTES) fail loudly at create, so they are
nits. Fix: derive the cap from tickHz in one place (or assert it against tick_hz() in the boundary test); read the
tick through BLOCK_LAYOUT.

### F6 (Low, record) the block is fixed-size for a Sim's life: a decision the ADR does not make (00:40Z)
host.ts:56-59 reads block_ptr/block_len once and sizes the pool once; worker-link.ts:95 refuses any other length; the
step returns only the due time. ADR 0002 part 5 says the step "returns where they lie and when the next tick is due".
Right for a header-only block and cheaper (no multi-value return, no extra call), but it binds cs4 and cs8: every
section, the event section included, must be written at a fixed capacity with its count in the header (and an
overflow rule). Record: report + ADR 0002 part 5 pointer at the drafts' next revision.

### F7 (Low) a render-quality policy in the app shell: boot.ts:48
Math.min(window.devicePixelRatio, 2) decides the pixel ratio in lane 1's app; Part 15.2 gives "set quality" to the
picture and render/ holds the quality tiers after cs5. Fix: pass the raw devicePixelRatio; the picture caps it.

### F8 (Low) the Worker-to-page half of the contract is parsed by hand
messages.ts:72-92 guards PageToWorker; worker-link.ts:68-87 checks WorkerToPage ad hoc (message["stats"] as WorkerStats,
:82) with if-chains, so a new WorkerToPage kind in messages.ts compiles without any page change and fails only at run
time ("a message the page does not know"). Fix: isWorkerToPage beside isPageToWorker, and a switch on the narrowed
type in onMessage (ADR 0002 part 7's exhaustive-switch rule then covers it once the 2.3 lint exists).

### F9 (Low, cs3 note) the endpoint's state lives in the bindings crate
sim-wasm lib.rs:55-60, :68-83: Sim composes World, BlockWriter, LocalSchedule and newest_input, with the checks in
Sim::create. ADR 0002 part 3: sim-wasm is "the wasm-bindgen bindings the Worker loads, and nothing else"; the headless
native clients of part 8 cannot reuse this composition without depending on wasm-bindgen. Small now (about 40 lines);
when cs3 brings room and the loopback, the composition belongs in client-endpoint (writer, schedule, newest input) and
room (the authority's World), with sim-wasm a wrapper.

### F10 (Low) test-only surface in the player's Wasm
newest_input() (lib.rs:121-125) is called only by tests (native and boundary); input() has no other observable
effect yet. Acceptable as a test seam for one change-set; record, and drop it when the input reaches a command (cs3).
Also dead in shipped TS: Ring.clear (stats.ts:61-64, test-only), BlockPool.free (block-pool.ts:33, test-only), and
the percentile re-export to the page (index.ts:4; the page uses Ring's method only). Nits.

### F11 (Low) production has no failure line at all
boot.ts:24-25 hides #notice before picture.start; in a production build (no readout) a renderer that cannot start,
a Wasm trap or a layout mismatch ("the page and the Wasm are from different builds", worker-link.ts:92) leaves the
page silent apart from the console. Not needed for the dev zip; needed by the first build players can open (2.6).
Fix when it is due: show #notice with the failure's plain text (textContent) in every build.

### F12 (Low, decide and record) the Worker's readings ship in production
The worker chunk is byte-identical in both flavours (worker-Ca-EDn0E.js in packages/client/dist and
.gate/cs2-first-page/dist-prod): the copy-time and lateness rings, two performance.now() per tick and the
report-request answer run in the players' build. Cheap, and pacing counters are product monitoring candidates (ADR 0004
part 5 counts overruns and drops). Decide: product counters (say so in the report) or harness (gate the copy timing
and the rings behind the build constant in the Worker too, as Part 0 asks of development tools).

### F13 (Low, nit) ownership and small couplings
WorkerLink's hand-off state is public and writable (worker-link.ts:32-42: ready, blockBytes, tickHz, blocks, tick,
posts, failure, stats, lastPause, pauses): getters or a readonly view. The schedule's anchor has two writers: Sim::new
takes now_ms (lib.rs:80) and the Pacer's constructor replaces it at once with rebase(now()) (pacing.ts:59). Folder
cycle by types: app/boot.ts:4 -> overlay/report.ts -> app/worker-link.ts (report.ts:2, PauseRecord). sim-worker
declares @shamble/types as a dependency (package.json) used only by boundary/sim-wasm.test.mjs.

### F14 (Low) the per-tick call count leaves rebase out (00:42Z)
boundary/sim-wasm.test.mjs:89, :94 (and the stand-in in host.test.ts) count input, step, block_ptr and block_len but
not rebase, so "exactly two calls per tick" (PM-0002-3) would still pass if the host rebased every tick. Fix: count
every export the host can call and expect rebase 1 (the Pacer's constructor) in a run with no pause or drop.

### Q5 note: the head's build constant is not proven; a fix is in the working tree
At 30a5a13 boot.ts:12-13 read the flags as import.meta.env["VITE_SHAMBLE_DEVTOOLS"] (bracket form). Vite 8.3.7 defines
env keys only in dotted form (node_modules/vite/dist/node/chunks/node.js:17065) and otherwise replaces the whole
import.meta.env with an object literal (:17115), so the bracket form folds to a literal only if the minifier folds
the lookup. The builds I scanned (packages/client/dist 00:31:32, .gate/cs2-first-page/dist-prod 00:31:35) were made
from the main session's uncommitted switch to Vite define (boot.ts and vite.config.mjs, modified 00:31:32), not from
30a5a13: on those, the page's dev-only code is gone from production (no overlay/report chunks, no __shambleReport,
no runHeader; my scan and production-check.txt agree) and the glue is only in the worker chunk. So: commit the
define change and re-run the production check on the final head.

## Answers (written 00:43Z)
Verdict: findings to fix before the merge (F1, F2 Medium); the rest Low.
Q1: picture.ts is a good first cut (three-free types only); three only in render/ (2 files); overlay/harness never
reach render/; one hole, boot.ts:5 (F1); add the block-lifetime sentence (F3); loop joins the member list. Forward
note for cs4: hand the picture read-only accessors over the block built by lane 1 from the generated layout, not raw
buffers, so lane 2 never computes offsets and the lifetime rule lives in one place.
Q2: holds (lib.rs:128-133; host.ts:62-65, :137-146; worker-link.ts:97-99, :111-117; read-block.ts:17; pacing.ts:92-106;
glue pkg/sim_wasm.js:37-39, :87-90; only worker-*.js holds glue). Gaps F2, F4, F5, F14.
Q3: all departures right. ADR 0002 part 5 pointers: rebase and tick_hz (calls its API list does not name, not per
tick); place and length read once (F6). Report + Part 15.2 member list: Picture.loop with its cadence pinned. Report
only: Option, SEED_BYTES, input(sequence), *const u8, host.ts, isPageToWorker, build-checks/tool modules, .mjs
boundary test (not type-checked; the .d.ts meets HostSim only where worker.ts is compiled), report.ts, newest_input
(drop at cs3).
Q4: holds (cargo tree: sim and client-endpoint have no dependencies; sim-wasm = client-endpoint + sim + wasm-bindgen;
no DOM name in sim-worker; only worker.ts imports the glue and no test imports worker.ts; unit tests pass). Nits F13.
Q5: page side behind one constant; proven only for the uncommitted define change (F-note); Worker readings ship (F12);
missing in production: a failure line (F11).
Q6: no engine feature "for later"; small leftovers F4, F10.
Runs: cargo test --workspace --locked (client-endpoint 17, sim 9, sim-wasm 6, protocol 2, room 1, devtools 1: pass);
clippy -D warnings native and wasm32: clean; vitest sim-worker --dir src 30 pass, client 43 pass (00:36 UTC).
Not checked: boundary tests, tsc, build.mjs, browser tests (not run, as asked); a build of 30a5a13 itself; lockfiles;
serve.mjs's security; e2e spec logic; three-picture's backend detection; Rust safety beyond the boundary.
