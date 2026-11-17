# rust-reviewer: U3C-HELD-1 review (round 1, final diff)

Branch claude/netcode-proof-test-plan-hcky5g, PR #40 (draft). Read only; no builds, no cargo.

## Plan
1. Read the diff (`git diff -- spikes/netcode-proof`) and the u3c-plan.md correction (`git log -p -3`).
2. Read the plan draft, the advisor review (C1-C10, HELD-Q1-Q12) in full, the design and evidence notes.
3. Read delay.rs and delay_tests.rs around the changed items in full context: cfgs of every new item and statement,
   Life's new fields and Default, the check's signature change and every caller, lock and borrow order at the two
   growth sites, integer arithmetic, names and doc comments.
4. Read build.md and the logs: light.log (fmt, clippy, doc), l1 / l1-1-94, e3a / e3b / e3c, p1-p5, e5.
5. Determinism and build-config differences (measured vs bites, wasm32).
6. Findings with file:line, severity, fix; verdict.

## Findings (a line after each)
