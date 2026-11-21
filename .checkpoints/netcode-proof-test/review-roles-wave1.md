# Wave 1's review panels: findings rated Medium or higher that only one role found (Mustafa's ask, 2026-11-11)

From the notes, without agents (his words, verbatim in history.md: "a small table with, for each reviewer role, how
many findings rated Medium or higher only that role found in wave 1's units (U4s, U4a, U4b, U4c, U4d, U4j). The review
panels stay as they are until I decide.").

**Method.** Each reviewer's own rating, as written in its notes. A finding counts as one role's only when no other
role's finding, at any severity, is the same issue: the advisor's joins (`=` in advisor-u4-wave1.md W1-U4D; the map
"Every finding, mapped" in advisor-u4j.md, where findings sent to the same fix are one issue), the reviewers' own
`(= …)` and "with" notes (u4ab-review-test.md), and the carries' joint labels (u4s-carries.md entry 1). Partial
overlaps count as shared. U4j's round 2 is running and is not in the table. The night shift's skeptic check of U4s
(cs/npt-u4s-skeptic) is not a review round of the unit and is not counted. "—" means the role was not on that panel.

| Role | U4s | U4a + U4b (one joint review) | U4c | U4d | U4j (round 1) | Total |
|---|---|---|---|---|---|---|
| netcode-architect | 2 | 0 | 0 | 5 | 1 | **8** |
| engine-architect | 1 | — | — | 2 | — | **3** |
| rust-reviewer | 0 | 0 | 0 | 0 | — | **0** |
| test-engineer | — | 0 | 3 | 3 | 1 | **7** |
| security-auditor | — | 0 | — | 0 | 0 | **0** |

**The findings behind each count** (file and the reviewer's id):
- **netcode-architect (8):** U4s F1 (the Command's client-claimed ticks beside the accepted one) and F2 (the room's
  order left open), u4s-review-netcode.md; U4d round 1 H1 (High: vehicle parts as world transforms), N-M1 (SS's
  magazine runs dry), N-M4 (vehicles and projectiles not in the fight), N-M5 (effect rows change every tick), per
  W1-U4D; U4d round 2 N2-M2 (ES's vehicles 40-82 m out), u4d-round2-netcode.md; U4j round 1 finding 3 (README's `hs`
  writer rules, FX-12), u4j-review-netcode.md.
- **engine-architect (3):** U4s F1 (-57's bites act on floats), u4s-review-arch.md; U4d A-M1 (the client cannot
  build `HitVolumes` from records) and A-M2 (no reset of the predicting World's own soldier), per W1-U4D.
- **test-engineer (7):** U4c M2 (kPacketThreshold pinned by no test), and O1 and O2, both outside U4c's files (an
  unreproduced npt-net test failure; check.mjs hiding a failing test's name), u4c-review-test.md; U4d T-M1 (ES misses
  two PM-0001-2 bands), T-M2 (the 1 MB cap is 2^20 B), T-M3 (D-9 cannot see a wrong blend), per W1-U4D; U4j round 1
  F9 (FX-9, the tolerance and band boundaries untested), u4j-review-test.md.
- **rust-reviewer (0)** and **security-auditor (0):** every Medium or higher they rated was also found by another role.

**Shared findings, for scale** (Medium or higher by at least one of the roles that found them):
- U4s: arch F2 = netcode F7 (the givens). U4a: netcode U4a-N1 = test T-A1 = rust F-A3 = security SEC-7; rust F-A1 =
  security SEC-U4a-1 = test T-A2 (S1, the soldier's layout). U4b: no finding rated Medium or higher.
- U4c: test M1 = rust's Medium test gap (L-c5 cannot fail on a wrong next_event_ns).
- U4d round 1: N-M2 = R-M2 = A-C (the press counters); N-M3 = T-M4 (FLAG_FIRED); R-M1 = T-M5 (`load_movers`'s pools).
  Round 2: N2-M1 (the ES drift; the builder's open point B, measured by the test-engineer's probe X2); T2-M1 = A2-L3.
- U4j round 1: ten issues found by two or three roles (FX-1 to FX-8, FX-10, FX-11), FX-1 (the FAR BETTER cap, High)
  by all three.

**Borderline, not counted:** U4a-N2 (netcode, rated "Low/Medium"; the advisor's W1-N2 ruling); the parts of U4j's
netcode 5 (ii-iv), security F9 (FU-2) and test F11 (FX-5 (c)) that no other role reached, each counted as shared
because the rest of the finding was.
