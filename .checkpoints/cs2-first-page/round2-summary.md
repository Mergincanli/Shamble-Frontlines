# Round 2 of the review (on what round 1's fixes changed: git diff 30a5a13..d257bb6), and the fix pass prepared for it

Written 2026-11-19 10:59 UTC (from the clock) by the main session. Six reviewers, each Opus 5.5 at effort max; their
notes are beside this file (`*-review2.md`). netcode-architect read 29378dd in round 1 with nothing above Low; the
lines that changed after it on its ground (`host.ts`'s rounded-up sleep) were read by perf-auditor and test-engineer.
**No Critical, no High. Three Mediums, each with a small fix. A third round is not due** (the budget: only for an open
Critical or High).

| Reviewer | Round-1 findings | New in round 2 |
|---|---|---|
| security-auditor | S1 to S7 closed (20 broken-off downloads leave 0 files open; 300 blocks of 1 MiB after a failure leave 0 MiB held) | Lows: N1 the refused-variables list misses two names; N2 no browser test fails on a policy refusal; N3 no `Cross-Origin-Resource-Policy`; carried: the DECISIONS pointer (docs pass) |
| rust-reviewer | F1, F2, F3, F6, F7 closed; F4, F5 open as planned (docs pass; the CI pull request). "APPROVE on my ground" | Lows: N1 (= security N1), N2 the list also refuses the build-folder variable that `--target-dir` already overrides, N3 one doc line says more than is true, N4 the pattern has no test |
| perf-auditor | P1 closed (wakes per tick 1.64 to 1.001; lateness p50 3.70 to 0.83 ms), A1 closed for the fully blind case, L1, L5 closed | **Medium N1:** the allocation test's per-block window straddles the point where Chromium optimises the listener (about block 470), so its verdict rested on where that falls. Lows: N2 the same in part per frame on a fast machine, N3 one line for the readings (up to 1 ms of each tick start is the whole-millisecond timer, by design) |
| engine-architect | F1, F2, F3, F4, F5, F7, F11, F14 closed; no wrong-way crossing | **Medium N1:** a Worker whose script does not load gives an error event with no message; the fix pass's own `message.replace` then throws, the Worker is not stopped and the page says nothing (a regression of d257bb6; measured). Lows N2 (a Worker's runtime error counted twice), N3 to N6 |
| graphics-engineer | F1 to F5 closed; F6 partly (nothing calls `stop()` yet) | Lows N1 to N6. Its reading of three 0.187.0: nothing on the WebGPU path needs what the page's policy refuses |
| test-engineer | F1 to F7 closed | **Medium NT1:** the failure routes have no test. Lows NT2 to NT8. Three conditions for the report (below) |

## The fix pass, prepared and proven in the scratch worktree only (`round2-fixes.diff`, 9 files, +60 -20)
**Not in the change-set:** SIZE-STOP-2 is open with Mustafa and no code changes until he answers. With it the size would
be 4,470 added lines (now 4,430; stop line 4,400).

| Finding | Change |
|---|---|
| perf N1 (Medium) | `allocation.spec.ts`: the window starts at block 90 and must end before block 420; per block the floor is 430 and the limit 610; the console line prints the window's blocks |
| engine N1 (Medium), N2 | `boot.ts`: the Worker's error event is cancelled and its missing message replaced; `error-hook.ts`: `String(message)` |
| test NT1 (Medium) | `skeleton.spec.ts`: two tests, a Worker that cannot be loaded and one that throws as it starts: the match stops once and the page says so |
| security N2, graphics N2 | `error-hook.ts`: a refusal by the content security policy is counted as an error (so the readout and every browser test show it) |
| security N1, rust N1, N2 | `build.mjs`: `RUSTUP_TOOLCHAIN` and `RUSTC_WORKSPACE_WRAPPER` refused; the two build-folder variables no longer refused; the comment says what is not covered |
| security N3 | `serve.mjs`: `Cross-Origin-Resource-Policy: same-origin`, and its line in the test |
| rust N3, graphics N3, engine N4 | three comments (the `mode` doc line; "programs" are shader stages; the picture reports its renderer's errors) |

Checks in the scratch worktree (`.gate/cs2-plants`, code = d257bb6 plus this diff), 09:38 to 10:58 UTC: `cargo fmt --check`;
clippy; sim-wasm's tests 6; tsc on the page; unit tests (client 48, sim-worker 30, boundary 7); the development and the
production build (its own check passes); a build with `CARGO_TARGET_DIR` set passes and one with `RUSTUP_TOOLCHAIN` or
`RUSTC_WORKSPACE_WRAPPER` set is refused by name; browser tests 10 of 10 three times, then 12 of 12 twice with the two
new tests; the two new tests fail without the fix (the notice never shows; the count is 2) and pass with it; the
real built page with the Worker's script answered 404: before, errors 2, last error "Cannot read properties of undefined
(reading 'replace')", nothing shown; after, errors 1, "The match stopped: the Worker failed" on the page; the stress run
test-engineer asked for (`allocation.spec skeleton.spec --repeat-each=5 --workers=2`, load 5.6): 30 passed, 27 to 30
frames in the allocation window (the check wants more than 20), 491 to 518 bytes per block. The allocation trial:
`round2-n1-trial.md`. Not tried: plant 7 with the new window; anything on a real GPU.

## Follow-ups from round 2 (Low; none triggers a push)
- For the CI pull request: the browser assertions that depend on the machine's speed (test-engineer's table: the frame
  count in the allocation window, 27 to 30 against 20 under load here; the hidden-tab waits); rust N4 (a test of the
  refused-variables pattern); rust F5 (the builder's registry path inside the Wasm).
- For change-set 3: engine N3 (one owner for "the match is over"), N5, N6; graphics N1 (a test of the lost-context
  route), N4 (each renderer error prints twice), N5 (after a lost device the page says nothing outside the readout:
  with ADR 0003's loss handling), N6; test NT3 (the error text's cleaning has no test), NT4 (the hand-off time read
  before the copy has no test), NT5, NT7; perf N2; the seven Lows of netcode-architect.
- Notes only: test NT2 (anchor `plants.py`'s patterns on the failing line's mark), NT8 (plants 5 and 8 ran differently
  from the plan's wording: the report describes them as they ran).

## test-engineer's three conditions for the report (each High if broken: "evidence that does not hold")
1. Either the perf N1 change lands, or the report says plant 7b's per-block catch at the head depends on where the
   browser optimises the listener (perf-auditor read 335 against the limit of 390 in one window: a miss).
2. The report does not claim the failure routes are tested, and scopes engine N1's case, unless its fix and test land.
3. Plants 5 and 8 are described as they ran: plant 5 edits the recorded version (a stand-in can never reach the
   version check: the checksum refuses it first; the version mismatch is unit-tested); the 7-tick cap is checked by
   the unit and boundary tests, not by the browser test (plant 8b is caught by the drop count).
