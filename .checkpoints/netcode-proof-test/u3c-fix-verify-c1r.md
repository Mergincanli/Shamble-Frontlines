# U3c fix pass (c1): adversarial verification of the re-reviewer's gaps (R1, G-F10, G-NC4, G-NC2, G-PLAN, G-FH, G-NC8, G-PRE-WAKE, G-PRE-PANIC, G-METHOD) and regression R1

Scope: `git diff 95597ef..81203da -- spikes/`; claims from u3c-fix-review-c1.md.
Rules: edit nothing but this file; scratch in the session scratchpad (own copies from `git archive`);
CARGO_TARGET_DIR=.gate/u3c-review-target (fresh mtimes before each run, check test counts); no iptables, no #[ignore]d tests.

## Plan
1. Read delay.rs at 81203da: Pump::poll (up loop, down block), current(), due(), move_up, Pump::drop, poll_read.
2. Read pre-fix pump at 95597ef for R1's comparison.
3. Read delay_tests.rs: (c) Return.owed, quiet P2 held bounds, NC-U3c-8 check.
4. Grep u3c-plan.md for the carry corrections (G-PLAN, G-NC2), C3 text.
5. Reproduce R1 (probe 1 style) and F10 cheaply in a scratch copy if time allows.
6. Verdict per item.

## Progress
