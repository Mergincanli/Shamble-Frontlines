# U4c review, round 1 (test-engineer)

Started 2026-11-10 10:15Z. Copy: /home/user/Shamble-Frontlines/.claude/worktrees/wf_f54246ff-e0d-2 (branch
worktree-wf_f54246ff-e0d-2, base 097f897). Build uncommitted: l1/src/lib.rs (M), l1/src/tests.rs (new),
l1/tests/l1.rs (M). No notes file from a cut run existed: fresh start.
Rules: plan §7.1 (shared target .gate/shared-target, CARGO_NET_OFFLINE, touch crates/ before and every 2 s during a
counted run, wait for no other cargo/rustc, df before each build). Plants in place, one at a time, restore, sha256.
No commit. Only crates/l1 touched, and only to plant and restore. This agent has no Edit tool: notes are rewritten
whole with Write; plants go through a plant script written with Write into this notes folder (u4c-test-plant.py:
exact-match replace with occurrence count, a stderr marker in L1Net::new as the positive control that the planted
build ran, wait for no foreign cargo/rustc, touch before and every 2 s, run `cargo test --locked -p npt-l1
--no-fail-fast [--features bites] -- --nocapture`, restore the bytes, sha256 check; logs in u4c-review-test/<name>.log;
backups of the three files in the session scratchpad). Precedent: u4s-bites.py, u3d-test-bites.py. Never sed,
redirects, heredocs or python3 -c. Counted check.mjs runs: u4c-test-check.py (20 s of no cargo first, touch loop,
foreign processes recorded). Flake repeats: u4c-test-repeat.py.

## Plan
1. sha256 of the three changed files (and every file under crates/l1) before anything; compare with the builder's.
2. Read plan §5 U4c (L-c1..L-c9 and its bites), §4.1, §7.1; the builder notes; the two reviewers' notes.
3. Step 1 of the task: check.mjs unchanged, record ten step lines and counts (expected: measured 285, bites 312).
4. Step 2: plant each §5 U4c bite in place (8 model bites), run, record the failure by name, restore, sha256. Add
   bites the plan missed (candidates from reading + reviewers' findings).
5. Step 3: each test can fail, none vacuous, numbers vs their sources (RFC 9002 constants, 1,200 B, PTO 1,024 ms,
   L-c2 bands, L-c8 pins; the lc8-scratch SHA-256 vs U2's committed runs.txt).
6. Step 4: check.mjs again after a fresh touch; sha256 of every file = step 1.
7. Findings list + verdict; DONE.

## Progress
- 10:15Z Plan step 1: sha256 before anything = the builder's: lib.rs 1647e75d8b5bea59..., src/tests.rs
  c21a362f460a7fcd..., tests/l1.rs 639ab9c5e1e73982..., Cargo.toml bf80e546..., clippy.toml 3ffe7dc6.... Full
  hashes of all 158 spike files: scratchpad u4c-sha-before.txt. git status: only lib.rs M, tests/l1.rs M, src/tests.rs
  new (+ builder notes, lc8-scratch).
- 10:16-10:20Z Task step 1, check.mjs run R1 (no other cargo/rustc at start; touch first and every 2 s; df 12 GB):
  all 10 PASS. fmt 12.7 s; clippy 44.6 s; clippy-wasm32 10.4 s; doc 20.9 s; test 163.2 s measured 285 passed 0
  failed 0 skip, bites 312 passed 0 failed 0 skip; deps 1.7 s; refs 0.6 s (562 tracked files); marks 3.5 s (81
  marks, drafts 626df678); self 1.2 s 48/48; redact 0.8 s 10/10 (158 items, 0 findings). Counts = 275+10 / 292+20.
- Reviewers' candidates a run can settle: rust M "L-c5 cannot fail on a wrong next_event_ns" (plant next_event_ns
  -> None / no timers / no inbox); netcode F3 (C2 stricter than RFC A.7: plant A.7's rule, does L-c7 tell them apart).
- Read: lib.rs whole; tests.rs whole; tests/l1.rs diff; lc8-scratch main.rs; plan §5 U4c :552-583, §4.1, §7.1;
  advisor-u3q.md:160-173 (C1'), advisor-u2.md:72-103 (C1, C2).
- 10:30Z Controls (marker only): measured lib 13 + tests/l1.rs 15 + doc 0, all pass, marker 74 lines, "Compiling
  npt-l1 (this copy's path)", 16 s; bites lib 23 + 20 + 0, all pass, marker 102. No foreign cargo in any run below.
  Names (from the control logs) = the builder's: 8 new lc tests + v1s_tag_... + next_event_ns_... measured; + 8
  bite_*_fails_lcN + two-thread + the_model_bites_are_chosen_only_here in bites. L-c2 PTOs seeds 1-10: 139.229
  145.974 154.005 175.217 149.907 155.599 167.569 158.779 149.389 145.595 (mean 154.13).
- 10:30-10:38Z The 8 named bites, planted DIRECTLY in the measured model (not through with_model_bite; the builder used
  the bites build), except stream-sample-in-order (bites-only state: the L-c7 #[test] switched to with_model_bite).
  Each restored, sha256 = build after each. Failing tests by name (lib unit tests unless said):
  - separate-spaces (both `let space =` lines -> 1): lc1 "L-c1: 12 probe timeouts fired (the first at 3144.168 ms)";
    also lc3 "link 0: its message did not arrive inside the burst with its acknowledgement lost" and lc9 "link 2's
    message was not declared lost and resent 1 ms after its send". 3 failed, 10 passed; tests/l1.rs 15 pass.
  - no-pto-doubling (pto_ns returns base): lc3 only, "probe timeout 2 at 6278.775 ms, not at 6428.162 ms (the PTO,
    149.387 ms, doubles at each expiry)".
  - acks-in-bursts (ack `lost = false`): lc3 only, "link 0: no probe timeout fired".
  - every-packet-sampled (every_sampled = true): lc7 only, "link 0 took 4000 RTT samples where the newest-packet rule
    gives 145 (first difference at Some(1))".
  - stream-sample-in-order (bites build, lc7's #[test] on with_model_bite): lc7 only, "link 1's samples after 4 s are
    [(4236.0, 226.0), (4236.0, 100.0)] ms". 22 passed 1 failed; tests/l1.rs 20 pass.
  - one-draw-per-message (shared = first): lc4 only, "its middle packet alone was not lost".
  - EXTRA one-draw-per-message with the planted loss kept (`Some(x) => x || forced`): lc4 only, "0 of 600 messages
    (0.0000) had pieces both lost and delivered, not about 0.1425": the statistical half bites on its own.
  - events-at-polls (both `if bitten!(EventsAtPolls)` -> true): lc5 "the fate logs differ (93267 and 93387 bytes)";
    also lc1, lc2, lc3, lc4, lc6, lc7, lc9 and 3 tests/l1.rs tests (reorder step x2, half the latency). 8+3 failed.
  - sends-before-events (both `if bitten!(SendsBeforeEvents)` -> true): lc6 only, "the PTO due at 4166.215 ms did not
    resend message 0 in its instant".
  All 8 = the builder's messages word for word: the bites-build bite code is what the measured defect does.
- 10:40-10:44Z Bites the plan missed (measured build, all npt-l1 tests, marker present, our path compiled, no foreign
  cargo, restored + sha256 each):
  - next-event-none (next_event_ns always None): lib 13/13 PASS incl. lc5; only tests/l1.rs
    next_event_ns_names_the_next_arrival_... fails ("the datagram's arrival"). L-c5 cannot see it.
  - next-event-no-timers (next_event_ns blind to loss/PTO timers): ALL 28 PASS.
  - next-event-no-ready (blind to the inbox's hand-out instants, the P0 gate+1 case): ALL 28 PASS.
    -> confirms rust-reviewer M: L-c5's event mode calls at every whole ms until 6 s (tests.rs:895, next_send =
    (t/MS+1)*MS whatever the script sends) and once at 10 s (tests.rs:911), and compares arrival stamps, not the
    instant each item is handed out; advance() applies events at their own time, so any next_event_ns passes.
  - no-packet-threshold (PACKET_THRESHOLD 1,000,000): ALL 28 PASS. packet-threshold-2: ALL 28 PASS.
    -> C1' (b)'s kPacketThreshold is pinned by no test: L-c1's 33 ms spacing makes the time threshold win always
    (3 later packets = 100 ms + RTT > 9/8 RTT). The stream test's comment (tests/l1.rs:385-386) names the packet
    threshold too, unpinned.
  - time-threshold-srtt-only (latest RTT dropped from max): ALL 28 PASS (L-c7 link 1 has srtt = latest).
  - acks-lost-at-profile-rate (acks also lost at the profile's loss outside bursts): ALL 28 PASS: Q15's "lost inside
    bursts only" has no negative test (L-c1..3 run lossless; L-c4's link 1 checks hand-outs from the log).
  - acks-share-datagram-flow (acks draw from the datagram flow): tests/l1.rs 3 FAIL: v1s_tag_... ("P2: the datagram
    entries are not v1's (0x167f703a471bb105)"), fates_belong_to_their_flow_..., a_datagram_refused_.... L-c8's unit
    half bites.
  - event-order-link-first (rank (time, link, fate index, ...)): ALL 28 PASS: L-c6's link 2/3 check (tests.rs:971-972,
    1038-1046) sends link 2 before link 3, so fate order = link order.
  - pto-from-oldest-unacked (PTO from the oldest unacked stream packet, not the last ack-eliciting send): lc1 FAILS
    "1 probe timeouts fired (the first at 6645.003 ms)": caught, by one event in 36 messages.
  - rfc-a7-sampling (sample whenever the ack newly acks its largest: RFC 9002 A.7): lc7 FAILS "link 0 took 504 RTT
    samples where the newest-packet rule gives 145": L-c7 pins C2's strict wording; at 30 Hz P2 (L-c2) the two rules
    differ by <= 0.3 ms of PTO in 5 of 10 seeds (139.229 146.236 154.005 175.217 149.909 155.577 167.388 158.852
    149.389 145.595): netcode F3's "rare at 30 Hz" holds; at 1 kHz P4 it is 3.5x fewer samples.
- 10:44Z L-c8 SHA check: the builder's lc8-scratch (own [workspace], path deps on this copy; `cargo +1.99.1 build
  --release --locked`, shared target, touch loop), on the NEW model: SHA-256(v1 tag + datagram entries) P2 s1
  913db8e914bdb773f6f1..., B1 s1 feabf2f02d023fa9784b..., P4 s1 b8a04f2f8ddc0d39624d... = the 16-hex prefixes in the
  committed results/c-u2-pm-0001-3/81c6d01e371b/runs.txt:16,28,34 (2,000,000 / 2,020,000 / 2,000,000 datagram
  entries of 3,959,634 / 3,979,629 / 3,899,978). Pins: 0xadfbebe4bb1abf64, 0x43749a62922e9b35, 0x246e7a241a2a0a18 =
  tests/l1.rs. 3 s per probe run. Not re-derived on the OLD code by me (provenance: builder notes 18:20Z item 1).
  Scratch files' sha256 unchanged by the run (main.rs f0cc015d, Cargo.toml 954ed45f, Cargo.lock 0f305fab).
- 10:47-10:49Z check.mjs R2: all 10 PASS, 285 / 312, but another lane's `cargo test -p npt-sim --test te_probe` was
  running at its start (my pgrep showed it; the command did not wait): NOT counted.
- 10:50:55-10:52:03Z check.mjs R3 (u4c-test-check.py: 68 s wait for 20 s idle; touch loop; no foreign cargo/rustc
  seen during it; df 12 GB): test step FAILED: measured build `-p npt-net --lib`: "159 passed; 1 failed; 12
  ignored" (the failing test's name fell outside check.mjs's 40-line tail, check.mjs:1529); 9 other steps PASS.
  npt-net uses only npt-l1's Profile and ReorderStep (net/Cargo.toml:19-20; src/l2/*, tcp_model.rs:72), both
  unchanged by U4c (base lib.rs:75-160 = new); npt-net has wall-clock L2 tests (l2/socket_tests.rs, delay_tests.rs,
  wt_tests.rs, delay_clock.rs). -> a pre-existing flake candidate, not U4c's; root-causing by repeats.
- 10:53-10:54Z repeat net 1/6 of `-p npt-net --lib`: 160 passed 0 failed (27.25 s); repeats 2-6 aborted (other lanes'
  cargo never quiet for 20 s in 9 min). 11:05Z relaunched (label net2, 5 runs, 3 s quiet) in the background.
- Next: net2 results; a counted final check.mjs (step 4); final sha256 vs u4c-sha-before.txt; findings.
