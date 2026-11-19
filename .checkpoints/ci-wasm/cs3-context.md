# Change-set 3 ("The loop closes"): the context for its plan, gathered beside the CI-WASM pull request

_Written by the main session, 2026-11-19, 17:33 UTC (from the clock). The plan is drafted beside the CI-WASM pull
request and built only after that pull request is merged (his CI-WASM (a)). This file moves into change-set 3's own
checkpoint when that opens. It is a list of pointers: the files it names hold the text._

## The card (the build plan, `docs/reports/2026-11-17-build-plan-slice-1.md`)
- The table's row, line 249: "The same page, alive: press keys and a small key diagram lights up, echoed back by the
  local server through the real messages; counters show ticks, messages and zero lost presses"; planned 2,700
  lines, about 15 h; his go on the merge.
- The card, lines 316-333: Rust: `protocol`'s codec (decoders return errors, never panic) and the thin transport
  interface with its loopback backend; `room` (one slot, timers that count ticks, the command queue, the snapshot
  encoder); `client-endpoint` (the join exchange, the command built from the newest input with its press counters,
  the snapshot decoder); the Worker's one step in the draft's order (the command for tick T reaches the local server
  before it runs T); `devtools`: the replay runner (one state hash per tick), the comparer (the first differing
  tick), the link simulator L1 behind the loopback (seeded: delay, jitter, loss, reordering, duplication, a rate
  limit, a packet-fate log); the panic hook that keeps a trap's message. Tests that settle rules first: every float
  operation on ADR 0004's allow-list and `sqrt`, and the first 10,000 outputs of each random stream, bit for bit
  natively and in WebAssembly; a scan test in `sim` that fails on a hash map, a clock call or a float method outside
  the allow-list. A small separate test: the Worker with two Wasm instances beside the one-instance layout. "If this
  change-set nears twice its size, the two-instance test and the table of build options move to a follow-up of
  their own first." Reviewers: netcode-architect, engine-architect, rust-reviewer; then test-engineer,
  security-auditor.
- Its done-when lines and pass marks: the table "Step 2.5", lines 463-476 (ADR 0001's milestone; ADR 0003's second
  half, every tick runs on the newest post; ADR 0004's milestone, the first recording's hashes natively twice and in
  Wasm; PM-0002-64 in Chromium here; ADR 0004's foundation line; PM-0002-49, L1's packet-fate log and its 5 % bounds
  over 1,000,000 packets per profile; PM-0002-53, a planted counter overflow).
- The interim scan test's three conditions (the advisor agent's ruling, lines 949-953): it checks exactly ADR 0004
  part 7's allow-list; it runs in today's CI test job; the report calls it interim and names the ruling it replaces.

## The code as it stands on main f5030a3 (lines)
`crates/sim` (world.rs 166, state_hash.rs 46), `crates/protocol` (messages.rs 134, ts_export.rs 162),
`crates/client-endpoint` (block.rs 107, layout.rs 394, layout_ts.rs 188, schedule.rs 117), `crates/room` (lib.rs 8),
`crates/devtools` (lib.rs 8), `crates/sim-wasm` (lib.rs 259); `packages/sim-worker`, `packages/client`,
`packages/e2e`. Change-set 2's report: `docs/reports/2026-11-19-cs2-first-page.md`.

## His words that bind change-set 3 (whole in `docs/DECISIONS.md`, or in this folder until the docs pass)
- RNG-CRATE (c), 2026-11-18, 22:34:37 UTC: "The library enters in change-set 3 beside its test vectors, and you ask
  me which one with that plan. For that question also read two things: the rand project's upgrade guide for 0.10
  (rust-random.github.io/book/update-0.10.html), which speaks of the output of the like-named generators in
  chacha20, and why chacha20's releases 0.10.0 and 0.10.1 were withdrawn." rust-reviewer's reading:
  `rust-reviewer-cs3-rng.md` in this folder.
- 2026-11-19, 14:28:25 UTC: "With change-set 3, one line in .gitignore for __pycache__/, so that a save cannot pick
  up compiled Python files again."
- 2026-11-19, 16:08:04 UTC: "For change-set 3's browser check: the README asks me to keep the browser's window in
  front and uncovered during the first run, and has one step with the window covered on purpose, so that BR-GAP is
  read and not guessed."
- 2026-11-19, 13:36:37 UTC: "The game has to work in Chrome, the Chromium-based browsers and Firefox." (the zip's
  README keeps its Firefox steps).
- DEPART-2 (a): the hand-off reading at the size of 12 and of 48 soldiers is made in change-sets 4 and 5.
- The dependency rule (`.claude/CHECKPOINTS.md` section 3's hold; security-auditor answers on exactly the manifest
  content before it is committed or built), TOOL-SOURCE ("every other file, version or checksum comes back to me"),
  and until the quality gate step: "every ready report of a pull request that changes a manifest or a lockfile
  quotes each release record and the security answer it names."

## The follow-ups the archive places with change-set 3 (`docs/STATUS_ARCHIVE.md`, the block of PR #61)
NA-N1 to NA-N7 (netcode-architect: the step gets the host's time; the hidden-tab pause from the mode once online
lands, held buttons cleared on hide; who picks the blend fraction; the seed as an authority-only section written
last; the page's per-frame number named the post number; a start tick in the authority's match setup; the Pacer
throws on a due time that is not finite; `checked_sub` or wrap-aware compares on decoded ticks), EA-F8, EA-F9
(the endpoint's state moves from `sim-wasm` into `client-endpoint`), EA-F10, EA-F13, EA-N3, EA-N5, EA-N6 with VF-L7,
VF-L4, VF-L5, GE-N1, GE-N4, GE-N5, GE-N6, TE-NT3, TE-NT4, TE-NT5, TE-NT7, PA-N2, PA-L2, TE-L, SA-ENV, FAVICON. And
from his browser check (the record `docs/reports/2026-11-19-cs2-browser-check.md` once the docs pass is in; until
then `browser-reports-reading.md` here): BR-GAP (with his words above), FPS-LINE, NEG-ZERO, FF-WAKES, BR-LATE,
ADAPTER, TQ-FF, NOT-SHOWN. Whatever the CI-WASM pull request leaves of its own group (TE-NT6, RR-N4, RR-F5,
E2E-TYPES, SA-TREE, SA-DIST) also lands here: see its plan.

## What the plan must hold (CLAUDE.md step 2, and his rules)
Goal; systems and files with their lane (Part 15); exact symbols; new libraries with the dependency rule; build
steps; evidence by kind with exact commands (netcode: deterministic tests with injected latency, jitter and loss);
risks; change type and risk tier; reviewers; what he checks in his browser (the zip's README with the Brave and
Firefox steps and the BR-GAP steps); the planned size (the card says 2,700) and the stop line at twice the plan;
what is not in it; the open questions for him (RNG-CRATE first); where the build plan is out of date; kit, gate and
CI edits with their exact text (the scan test must run in CI's test job; the L1 tests in the plain test job).
Agents for the draft, at most 3 at a time: netcode-architect (the messages, the room, the endpoint, the step's
order, L1), engine-architect (the boundaries, EA-F8 to EA-N6), rust-reviewer (RNG-CRATE: running), test-engineer (the
tests that settle rules; the two-instance test).
