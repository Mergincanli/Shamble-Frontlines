Change-set 3b, "The loop closes": the second half of change-set 3. Mustafa's browser check is in (2026-11-25, Brave and Firefox, both on WebGPU: every asked number 0; the record: `docs/reports/2026-11-25-cs3b-browser-check.md`). Built by the builder chat he calls Builder V3 on his words of 2026-11-24, 23:24:50 UTC (in `docs/DECISIONS.md`).

The report: `docs/reports/2026-11-25-cs3b-the-loop-closes.md`.

## What it is
One player, one machine: a key pressed on the page goes to a local game server inside the page through the real network messages and comes back, 30 times a second. A player sees a row of six boxes (W, A, S, D, Space, Shift) that light when the server has applied the key, and two new readout lines ("messages", "presses"). Nothing moves yet: the soldier comes with change-set 4.

## In it (the plan's steps 1 to 6)
1. The first wire form of five messages, decoders that never panic and allocate nothing before a message is whole, a transport interface and a loopback (`crates/protocol`).
2. The world applies the player's command and starts at a given tick (`crates/sim`).
3. The room's one slot with its checks and its queue; the client endpoint's side of the exchange, with the press pacer (`crates/room`, `crates/client-endpoint`).
4. Local play's one step through those messages, behind thin bindings (`crates/sim-wasm`), and the Worker's and the page's messages.
5. The page alive: the keyboard, the tally of the server's echo, the key diagram, three page-side checks of that echo.
6. One batch of manifest changes under the dependency hold (path dependencies between our own crates; security-auditor answered on the exact files first). No outside package entered.

## Not in it
The link simulator and the first recording: they moved to a change-set 3c by the plan's size ladder (he was told before anything moved). Any CI, kit or gate file. Any word of the design drafts (`docs/adr/`, `docs/research/`). A bound on how late a command may run (security-auditor's Medium, deferred in writing: no room behind a network transport without it; change-set 7).

## Evidence
- CI on the final code 6abf3d2a: `gate`, `smoke` and `toolchain` green, every count as written down before the push: 114 Rust tests in debug, twice, and the release runs; 139 tests under Node; 18 boundary tests on the built WebAssembly; 16 browser tests in 3.4 of 5 minutes.
- The decoder mark (ADR 0001, PM-0001-34) has started, not passed: per message type 10 million random and 10 million mutated inputs in release, 0 panics, no memory above the caps, a planted unchecked index and a planted over-allocation both found.
- The presses mark (ADR 0002, PM-0002-12), no part of it called passed; what the runs show, in Chromium only: by the mark's method for presses (10,000 presses under late and repeated key messages, 0 lost and 0 doubled, button by button; five planted forms of its two bites all turn it red); for releases as far as the server's echo can show them, not by the row's method (RELEASE-COUNT below); "reorders most neighbours" holds only on a page that draws fast (on CI's slow page only the repeated copies arrive out of order).
- 53 planted defects, each run against the check that should catch it.
- The players' WebAssembly: 53,813 bytes, sha256 5ebae154...1262, here and on the runner (31,718 on main).
- Size: 6,909 added lines against about 3,800 planned; his stop line of 7,600 was not reached.

## Reviews
Two rounds by six reviewers (test-engineer, security-auditor, perf-auditor, engine-architect, netcode-architect, rust-reviewer): all PASS in round 2, nothing Critical or High open. The verifier on the final code: PASS. The advisor agent: the code approved as it stands; its approval of the last commit is given to Mustafa in the chat with the commit's full id.

## Not verified
Firefox, WebKit and Brave in CI (his check covered Brave and Firefox by hand, on real hardware and on WebGPU; the headless runs fall back to WebGL 2). The key-to-tick mark (a reading only). Anything over a link that delays, drops or reorders (3c).

## Decided by him on 2026-11-25
RELEASE-COUNT: (a), accept as built (`docs/DECISIONS.md`, 11:38:37 UTC). The messages carry press counters only, and releases are worked out from the presses and the held keys; no release counter is added in change-set 3c. His two conditions: no report calls any part of the presses mark passed, and the two drafts are made to agree at their next revision, which comes to him before it is written (the follow-up 3B-RELEASE-DRAFTS).

## What his go accepts as still open
- The bound on how late a command may run (3B-LATE-BOUND: security-auditor's Medium, deferred to change-set 7; no room on a network without it).
- OVERFLOW-PANIC-RULE, which fell due with this first code in `room`: the code keeps the rule (plain arithmetic is denied outside tests; the lint check is clean), CI runs no lint step (that stage belongs to the gate step 2.3, whose chat is stopped), so until then every change-set that touches `room`, `protocol` or a server crate runs the lint by hand and says so in its report; the drafts' wording stays with their owners.
- Two limits against the press-pacing option he chose, each his decision when it comes due: at most 255 presses of one button wait their turn (3B-PACER-255); presses are lost in steps of 256 after an outage in which one button is pressed more than 255 times unconfirmed (3B-OUTAGE-256).

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01WLuqnvpAh9XTitx8ehKJcQ
