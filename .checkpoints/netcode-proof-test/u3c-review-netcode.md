# U3c skeptic round — netcode-architect review notes

Scope: git diff 2c68bf9..989118e -- spikes/ (U3c), against u3c-plan.md rev 2 [C1]-[C16], advisor-u3c-plan.md, advisor-u3q.md S0-S14.
Focus: DelayStream vs S0-S14; ws_gate vs fixes.rs:777 and W22; TickWriter newest-only; per-return record S12 times; SERIES rows; loopback tests + mechanism test (c) vs rev 2 1.4-1.5; kernel cross-check classification.
Rules: no network changes; CARGO_TARGET_DIR=.gate/u3c-review-target; edit no source.

## Plan
1. Read u3c-plan.md, advisor-u3c-plan.md, advisor-u3q.md S0-S14.
2. Read delay.rs, delay_clock.rs, ws_gate.rs, fixes.rs, ws_config.rs.
3. Read tests (delay_tests, ws_gate_tests), SERIES.md.
4. Kernel check compare.py + verdict.
5. Run tests.

## Findings (one line each, as found)
