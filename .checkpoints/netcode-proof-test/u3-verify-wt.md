# U3 verifier (skeptic) for the WebTransport scout: notes

Scout notes: .checkpoints/netcode-proof-test/u3-scout-wt.md. Branch claude/netcode-proof-test-plan-hcky5g (head def4c50 at start; task said ad527f3).
Registry root R = /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f

## Plan
1. Read the scout notes and the probe source (u3-scout-wt-probe.rs), plan §4.1/§4.3 E15/§4.4/§4.7/§12.1 U3/§13 risk 2.
2. C1-C16: open each cited line in R, record verdict here one line per claim as I go.
3. Re-run the probe (or my own) for the measurement claims (C9, C10, C11, C15) if build is offline-feasible.
4. Design problems; missed items. End with DONE.

## Verdicts (one line each, saved as found)
