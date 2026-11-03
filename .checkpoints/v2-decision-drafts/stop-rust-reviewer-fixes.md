# rust-reviewer stop-answer fixes (ADR 0004), 2026-11-03

### FIX O15
FILE: docs/adr/0004-simulation-core.md
BEFORE:
**Author:** rust-reviewer (the "Pass marks" table: test-engineer, plan step 4) · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no
AFTER:
**Author:** rust-reviewer (the "Pass marks" table: test-engineer, plan step 4) ·
**Reviewed by:** (filled in at review) ·
**Signed off by Mustafa:** as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted
END

### FIX S10, H23
FILE: docs/adr/0004-simulation-core.md
BEFORE:
- TICK-30 (STATUS "Decisions pending"): 30 Hz is the baseline; 60 Hz only with
  [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) of at least 25 %, which ADR 0001 owns and measures in
  the netcode proof test.
AFTER:
- TICK-30 (STATUS "Decisions pending"): 30 Hz is the baseline; 60 Hz only with
  [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) of at least 25 %, which ADR 0001 owns and measures
  twice: early, in wave 1's 60 Hz run after the sim core spike (SPIKE-ORDER (b)), and finally at slice 6 with the real
  sim (TICK-60-SETUP, the Advisor, 2026-11-03: "(a), as an early signal only. The final 60 Hz call is measured again
  with the real sim at slice 6."). The full rule is ADR 0001's part 2, with Mustafa's addition (TICK-60-HEADROOM,
  2026-11-03): "60 Hz also needs the goal budget to hold at the rooms per VM measured at 60 Hz".
END

