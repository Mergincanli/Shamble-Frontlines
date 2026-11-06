Task: netcode-architect round-2 review of the netcode proof test plan (revision 2), .checkpoints/netcode-proof-test/plan.md

## Plan (blocks, in order)
B1. Read review1-findings.md (what round 1 raised, esp. netcode-architect's findings) and plan 0.1 map.
B2. Read my digest (netcode-architect-digest.md) for each mark's setup + order constraints.
B3. Read plan sections 4 (crates: 4.1, 4.4, 4.7), L1, 10.2, 10.3, R-25 parts.
B4. Read plan sections 6.2, 7.2 (run mapping, hours) and check against digest.
B5. Read plan 8.1 (ports, OQ-0006-16) and infra digest on ports.
B6. Read plan readings R-5..R-26 and section 11 item 3.
B7. Check ADR 0001 lines: 220-275, 419-473, 654-682, 1444-1445, 1631-1681; ADR 0004 part 5 (S17); find R2-XU-OQ7 rule.
B8. Write findings, verdict, picks.

## Notes
- B1 done: round 1 had no netcode-architect review (security-auditor F1-F19, engine-architect EA-1..14, verifier F1-F16). Relevant to my lens: EA-1/2/3 (room/server split, Link, lab), verifier F1 (-14 sessions), F4 (-36 order), F7 (ports), F8 (R-5 per stream), F9 (readings that change marks), F10 (R-6 L2 rate), SA F13 (WS delay in accept path). Next B2 (digest).
