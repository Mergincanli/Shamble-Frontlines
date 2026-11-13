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
    set (check.mjs:2289-2293 replays every manifest entry on the Wasm leg; check.mjs is in the lane only for 83-85),
    nor that regenerating a generator-0 (runner) entry at check time means re-running the whole L1 scenario
    (§4.15 :502-504; hash.rs:292-311 refuses generator != 2 today; replay.rs:50-51 generator 0 = a runner's): about
    3 x 9,000 / R s more (R = 90: 300 s, the whole budget).
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
- B4 done (rows at the head 83c7801, spikes/ = cc8595d): hold as the brief states: R2/R38 (l1 lib.rs:42-44 doc, :118
  INITIAL_RTT_NS, :633-635 smoothed_ns, :64 doc, :1748-1750 rtt_ns reads `smoothed`); R4/D12 (no first-sample
  accessor; send_ack :1418-1465 sends at arrival with delay 0 at P0; advance :947-959; poll needs gate_ns < now
  :1667-1681); R13 (Room::new :431-438 six args; with_bite :510; BITES 7 ids room/lib.rs:38-46); R15/R37 (pump.rs
  RttSource :183-190, new :224-247, rtt_ns :298-303, pump reads no RTT, pings only when Joined :346); R16 (input stamps
  newest + lead and its own seq: client :390-400; StepOut :189-202); R17 (offer: forward compare :84-89 before late
  :111-117; take :136-158; CMD_CATCHUP = REDUNDANCY_CAP :56-58); R18 (getters :195-297 at the cited lines); R19/R36
  (rewinds_refused :133-135 field; count in fill_input :860-864; late :846); R33/D10 (git grep at HEAD: only pump.rs
  and lib.rs:14); R34/D11 (no Resync in client/src; PROVISIONAL_CAP 1_024 events.rs:40); R43 (with_lead :291-295,
  default 1 :262; with_starts :298-302; with_event_start room.rs:488); R44 (join returns (slot, replaced) :637); R30
  (World::new :701; with_bite :944; state_hash seam.rs:551; entity_records :554); check.mjs cites :39, :879, :914,
  :1560, :1864, :1891, :1935, :1973, :2020, :2062, :2099, :2120, :2212, :2254, :2468-2503, :2520, :2548 all hold.
  New facts found:
  - F-j (HIGH, carry 74's lead): ClientEndpoint stamps `newest + lead` with newest the newest snapshot tick it holds
    (client :336, :390-392), which is already one downlink delay plus up to one client step old; so the command must
    cover downlink + step wait + uplink: lead >= ceil(max RTT / tick) + 1. Carry 74's formula covers one way only
    (P0 1, P2/B1 3, P3 4). Print-only model (uniform jitter as l1 lib.rs:790-812; snapshots each tick; client steps at
    a fixed phase; 5 phases x 20,000 steps): late share at P2 L=3 0.977, L=4 0.023, L=5 0; at P3 L=4 0.982, L=5
    0.558, L=6 0. So under carry 74 every P2, P3 and B1 run has ~98 % of commands late: the 1 % rule sends every one
    to the advisor; -17 P2/P3/B1 and -18 B1 cannot count; SS P2 s1 and ES P3 s2 recordings hold mostly reruns.
    The brief must raise it (a new question), it cannot reword carry 74.
  - F-k (Medium, -18's heal "decode" undefined): under no-fallback the client files the sentinel 0 for each refused
    delta (client :196, :352, :486); a heal record that takes any filed hash heals the bitten run. The brief's 2.1 says
    "first decode" but never "a non-zero hash".
  - F-l (Medium, the brawl's view time): rewind_at refuses a fraction > 0 unless view_tick + 1 is in the ring
    (rewind.rs:8-11); the ring holds ticks <= T - 1 when tick T fills its input (room.rs:812-819). A brawl that
    stamps (newest, fraction > 0) at lead 1 (P0) gets every firing command refused: rewinds_refused > 0 on both legs
    of the pre-check, VOID by carry 40 forever. The brief never states the view time the brawl stamps.
  - F-m (Medium, U4R-Q6 (a) is incomplete): Decoder has no bite selection of its own (pump.rs: only its Quant;
    `Decoder::new` :224-231); npt_server::BITES is a fixed [&str; 3] of Schedule's BITE_IDS (schedule.rs:245-251). A
    per-instance pump bite (§4.11 :399-401) needs a new pub `Decoder::with_bite` (A19) and a change to BITES' shape in
    U4b's file, plus the site's allow (server/clippy.toml:6-17): not "nothing else".
  - F-n (Low): the lab's I/O step must call `Decoder::accept` after an Ok join (pump.rs:265-269; without it the
    session stays Joining and every datagram counts `before_join`) and `reject` on Err; 2.1 names neither.
  - F-o (Low): the late-command bite's one Input holds T..T+L (L+1 commands); REDUNDANCY_CAP 8 (input.rs:56-58;
    proto input.rs:42-50) drops the oldest beyond 8, so L = 8 would drop T itself (a miss, not late): L <= 7 or T
    sent alone first.
  - F-p (Low): the Loopback leg has no next_event_ns; 2.1/7.2 never say what instants the RQ8 loop visits there.
  - F-q (Low): the pre-check never says it scripts each recorded tick's spawns, teleports and attributes into both
    rooms (Room::script, room.rs:617-630), or that SS P0 s4 holds none.
  - F-r (Low): commands once per client step (plan.md E12: 30 inputs a second), never per 60 Hz frame: a second
    command stamped for an already-buffered tick counts late (input.rs:111-117).
  - F-s (Low): warm-up and after-phase lengths are not stated (carry 62 (d)/(e); carry 75's 32,768 events), nor in
    7.3's arithmetic.
