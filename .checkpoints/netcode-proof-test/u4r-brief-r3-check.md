# U4r brief rev 3: the netcode-architect's check

Started 2026-11-13. Brief: .checkpoints/netcode-proof-test/u4r-brief.md (read only). Branch head 83c7801. No Edit tool
in this seat: the notes are rewritten whole with Write after each block.

## Plan
- B0: read the brief whole (structure, sections, placeholders, recheck rows, questions, D-items).
- B1: (1) u4-build-plan.md §5 U4r as it stands: every test, negative case, bite, judged run, carry; map to the brief.
- B2: (2) placeholders: anchors found once, first and last words (python3 -I print-only).
- B3: (3) rulings since rev 2 naming U4r (advisor-*.md minus *-brief.md, plus advisor-u4e-brief.md / advisor-u4d2-brief.md as named): applied as ruled; open questions answered.
- B4: (4) >= 10 recheck rows vs code at head (R2, R4, R15, R33, R34 + U4e room, pump, client rows).
- B5: (5) O1 (carry 40), -30 pre-check, RTT rule, -18's heal record vs U4e's merged room/server/client.
- B6: (7) build rules: runner u4r-run.sh, W1-TARGET item 3 as amended, lessons, TE2-5, B6, A13.
- B7: (6) missing / untestable / contradicting / scope creep; findings ranked.

## Blocks
- B0 done: brief read whole (1,412 lines; sections 0-12; C1-C26; R1-R44; U4R-Q1..Q13 open set; D1-D12). Stamps at
  head 83c7801: plan 01ede63d, plan.md 3ca4e12a, carries 8b7f165e, history 88bc195f, u4r-run.sh 349b51ad: all equal
  the brief's section 0. All 13 ruling files' sha256 equal section 0's stamps. (The brief says head fa8e335;
  checkpoint commits since touch .checkpoints/ only.)
- B1 done (plan §5 U4r :750-803 vs brief): every Builds item, test, judged run, bite and Evidence line has a home in
  the brief (2.1-2.4, 7.1-7.4). Candidate findings:
  - F-a (recordings' scope): plan §4.15 :500-501 "The determinism step keeps its three; the fourth serves the
    pre-check only", yet brief 2.1 says the four "join crates/lab/recordings.json under carry 82" and section 5 item 4
    times the wasm step "with U4r's recordings". The brief never says whether SS P0 seed 4 enters the wasm step's
    set, nor that regenerating a generator-0 (runner) entry means running the whole L1 scenario at check time
    (hash.rs:292-311 refuses any generator != Recording::GENERATOR today; replay.rs:50-51 generator 0 = a runner's).
  - F-b (carry 49 refusal): no test of the spawn/teleport refusal (not required by the carry; Low).
  - F-c (U4R-Q4 narrows plan :759-760 "each of -21's eight suites"): open question to the advisor, fine.
  - F-d (section 12 item 2): INF2-4's P3 was run at 11:01 UTC with u4d2-run.sh and u4d2-build (card :128-130), so the
    "if u4d2-build is gone" clause is stale.
- B2 done (placeholders, python3 -I print-only, stdin script):
  - P1 carries :255-432, heading found once, entries exactly 40 49 58 59 60 62 63 69 73 74 75 76 77 82 83 84 85.
  - P2 history :292-293, found once; line before :291 "  USAGE-SAVINGS: (a), all three, with these limits." OK.
  - P3 df :462-464 OK by range; its start "Write files with the" occurs twice in the file (:313 and :462): only a
    range-scoped search finds it once (same form as u4d2-brief 7.2 item 3, which worked). Low at most.
  - P4 df :465-468 once, ends "and go on." OK. Its replacement names no npt-server if U4R-Q6 (a) gives pump.rs to U4r
    (only npt-log for Q13 is conditional): F-e, Low.
  - P5 j-r2 :168-171 once OK. P8 j-r2 :298 once OK. P9 m9 :252-256 once OK. P10 m9 :259-264 once OK.
  - P11 d2-r2 :304-308 once OK. P12 d2-r1 :308-310 once OK.
  - P13 carries :72-75 (6.), P14 :116-119 (14.), P15 :135-136 (20.): each found once, spans end before the next entry.
- B3 done (rulings since df8975b; every ruling file postdates it): applied as ruled: W1-N2/U4E-Q2 widened (C2), U4E-Q3
  (U4R-Q6 (a) uses npt_server::BITES), U4E-Q4, U4E-Q5's condition (C6, 7.2), U4E-Q6 conds (carry 58), W1-U4D TUNE-1
  (carry 60 adds reach and spread), TUNE-2/N-M4 (carry 49), close F3 (carry 40's words), close B1.2/B1.3 (carry 59),
  j-ruling R2.4 with e-r1 R5 NC-L10 ("measured"), j-r2 B2.1 (carry 62 (g)), e-r1 R1 (carry 73), R3 (74-76), R4 NC-L1
  (C7), NC-L4 (no drop count), e-ruling U4E-Q13 + "For later briefs" (r4), d2-ruling A13 (2.2), d2-r1 Block 5 (TE-M3,
  P12), d2-r2 Blocks 1, 2 (84, 85; loader sentence in the lane), Block 6 (P11), m9 Blocks 1 and 2 (D3, P9, P10, the
  runner). No open U4R-Q is answered by a ruling (grep of all 12 rulings for wallclock, 63,736, TC B1, pre-check).
  Candidate findings:
  - F-f (df R6): "An agent that slips says so at the top of its notes" is not in the brief's write rule (Low).
  - F-g (r1's bite): no selection form (id or config) and no bites-only clippy allow named; npt-lab's clippy.toml bans
    HashMap (lab/clippy.toml:5-7) and the workspace denies iter_over_hash_type; l1 shows the form (crate-local ids
    with `with_model_bite`, lib.rs:752-764; `#[allow(clippy::disallowed_methods, clippy::disallowed_types)]`
    :769-770). Low.
  - F-h (section 12 item 8): plan §7.1 item 6 (:917-918) puts every judged L1 run on the main folder's own cache with
    CARGO_TARGET_DIR unset; the advisor's drift note (d2-r1 :716-719) is about merge checks only; the brief offers
    the shared target as the main session's choice without a ruling. Low.
  - F-i (TE-M3 under section 5): a check.mjs run in which TE-M3 failed is recorded and does not stop the step group,
    but section 5 needs two runs "each with every step passing"; the brief does not say that run is not one of the
    two (d2-r1 :298-299: a whole run in which all 11 pass). Low.
