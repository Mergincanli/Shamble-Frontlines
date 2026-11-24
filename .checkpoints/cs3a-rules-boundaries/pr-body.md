**Draft until Mustafa's browser check.** Change-set 3a, "Rules and boundaries": the first half of change-set 3 (the answer SIZE-3 (b), two change-sets, 3a then 3b, one of the seven recommended answers he confirmed on 2026-11-23 by choosing "Yes to all seven"). Built by the builder chat he calls Builder V3 on his words of 2026-11-24, 11:03:53 UTC. The branch `claude/loving-hopper-6p0mg2` stands in for `cs/3a-rules-boundaries`. Risk tier 1 (game code, tests, build scripts) plus `docs/DECISIONS.md`, a decision doc: it merges only on his go in that chat, for the head the advisor agent approved, after his browser check.

The report: `docs/reports/2026-11-24-cs3a-rules-boundaries.md`.

## What it is
The simulation's rules go under tests before the game loop rests on them, and the edges between the Rust simulation, its WebAssembly wrapper, the Worker and the page are tidied. A player sees almost nothing new: two readout lines changed ("frames" is a count over time; no more "-0.00"), and a simulation error's own text now reaches the page.

## In it (eight planned steps)
1. Follow-ups from change-set 2: one function checks every message from the Worker; one owner of "the match is over"; the readout's two lines; the small server's edge cases.
2. A scan of the simulation's source: no float method outside the allowed list, no hash map without its written escape, no clock, no per-processor code; 43 planted breaches and 12 controls inside its own test.
3. One batch of manifest changes under the dependency hold (security-auditor answered on the exact files first). No outside package entered.
4. Our own ChaCha8 random generator, equal to the eight published test vectors and to a second implementation's 40,096 words.
5. Float test vectors: 38 operations on 10,048 fixed inputs against an exact reference, natively and in WebAssembly.
6. A panic hook that keeps the simulation's error text, and one guard around every WebAssembly call in the Worker.
7. A test-only WebAssembly module, built outside every package, that runs these tests in Node and in the browser; no page, Worker or production build holds it.
8. The build refuses a crate without overflow checks and a feature set that differs between builds, and writes no builder path into the WebAssembly: one commit gives the same bytes here and on GitHub's runner.

## One rule changed: the fused multiply-add
`f32::mul_add` does not give the same bits natively and in WebAssembly on the pinned Rust toolchain (measured: every result below the smallest normal float). Mustafa decided on 2026-11-24, 15:58:25 UTC: "MUL-ADD: (a). The fused multiply-add stays banned in the simulation; it writes a * b + c." The scan refuses it through a one-name list of its own; 48 hard inputs joined the vectors; natively every case must be exact; the 16 WebAssembly values are pinned as a known toolchain defect, so any change fails the test. No word of ADR 0004 changes here.

## Evidence
- CI on the code head 8619d539: `gate` and `toolchain` green, every count as written down before the push (Rust tests in debug and release, 121 tests under Node, 17 boundary tests on the built WebAssembly, 14 browser tests). One earlier head (a1b2e802) was red on an assertion of the builder's own, fixed forward and never re-run; the report says why.
- 96 planted defects, each run against the check that should catch it: 96 caught.
- The players' WebAssembly: 31,718 bytes, sha256 383f5303...3963, here and on the runner.
- Size: 3,561 added lines against about 2,080 planned; his stop line of 4,100 was not reached; nothing was cut.

## Reviews
Three rounds (rust-reviewer, engine-architect, test-engineer, security-auditor, perf-auditor; the third round security-auditor alone on one fix), the verifier, and the advisor agent on the head: their verdicts and every finding's fate are in the report.

## Not verified
A processor without the fused multiply-add instruction (emulated only). Firefox, WebKit and Safari for the vectors. Windows for the build scripts.

## Not in it
Any CI, kit or gate file; any outside package; any word of the design drafts (`docs/adr/`, `docs/research/`); the game loop (3b); Firefox in CI (the small CI pull request after this one).

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01WLuqnvpAh9XTitx8ehKJcQ
