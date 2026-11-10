# U4c build notes: the link model (npt-l1; C1' and C2)

Builder: a workflow agent in worktree /home/user/Shamble-Frontlines/.claude/worktrees/wf_f54246ff-e0d-2, base
097f89704d9925dee0dc62f7d7373cb13fbbc9a0 (fast-forwarded; `git log -1` checked). Plan: u4-build-plan.md rev 2 (sha256
7ec1b324..., checked) §5 U4c, §2 C1' and C2; advisor-u2.md:72-103 (C1, C2), advisor-u3q.md:160-173 (C1' (a)-(g)),
advisor-u4.md (U4 verdict, U4s ruling, main session's note). Build rules §7.1: CARGO_TARGET_DIR =
/home/user/Shamble-Frontlines/.gate/shared-target, CARGO_NET_OFFLINE=true, touch crates/ before each build, counts as
deltas, bites shown failing by name, df before each build; files: crates/l1 only (no manifest, no Cargo.lock, no
check.mjs); Write/Edit only; commit nothing. No judged L1 run (L-c8's judged re-run waits for U4j's merge).

## Plan
1. Scratch harness (this folder, u4c-build/lc8-scratch: path deps on npt-l1, npt-link, sha2; its own [workspace]):
   replicate the probe's send loop (probe.rs run(): 10,000 pps, 1,000,000 outside bursts, both directions) and print
   the full canonical SHA-256 and SHA-256(v1 tag + datagram entries); on the OLD code it must give U2's full hashes
   (P2 s1 913db8e9..., B1 s1 feabf2f0..., P4 s1 b8a04f2f...: .gate raw logs of c-u2-pm-0001-3-81c6d01e371b). Also
   capture FNV-1a-64 of (v1 tag + datagram entries) for three small mixed scenarios, to pin in a unit test.
2. Model (crates/l1/src/lib.rs): one packet-number space per link and direction; datagram packets and stream packets
   (messages cut into <= 1,200 B pieces, each its own fate; every resend its own fate); immediate acks, one per
   arrival, drawn from their own per-flow ChaCha8 stream, lost only inside bursts, cumulative (an ack covers every
   packet that had arrived before it was sent); RFC 9002 §6.1 loss detection (packet threshold 3, time threshold
   9/8 x max(srtt, latest), kGranularity 1 ms), PTO = srtt + max(4 rttvar, 1 ms) + 25 ms, doubled per consecutive PTO,
   reset by an ack that newly acknowledges, counted from the last ack-eliciting send, armed while stream data is in
   flight; starting state srtt 333 ms, rttvar 166.5 ms (first PTO 1,024 ms); C2 sampling (only the newest packet of
   its link and direction, newly acknowledged; a stream piece samples from its own packet); event-driven: every
   call first processes the whole net's internal events with time <= now, in (time, fate index, kind) order; and
   `L1Net::next_event_ns`. In-order streams; the reorder step unchanged. Fate log v2: datagram entries keep v1's 40 B.
   Crate doc rewritten to C1' (a)-(g) plus the starting state; the E15 sentence gone.
3. Tests (unit, in src/tests.rs, internal access): L-c1..L-c7, L-c9, plus the v1 pin (L-c8's unit half). Bites as
   `bites`-build defects chosen per instance (`L1Net::with_bite` ids `npt-l1/<name>`): no-pto-doubling,
   acks-in-bursts, every-packet-sampled, stream-sample-in-order, one-draw-per-message, separate-spaces,
   events-at-polls, sends-before-events; each with a bites-build test that its L-c check fails; plus in-place plants
   (one at a time, sha256 restored) switching each L-c test to its bite, to show the test failing by name.
4. Existing tests: keep; update the stream test (old RTO model) and the RTT test to the new model.
5. Gate: check.mjs twice (10/10), clippy host + wasm32 both builds, fmt --check; counts as deltas on 275 / 292.

## Progress
- 18:11Z Start: base checked (097f897); plan sha256 7ec1b324 checked; notes created; df 13 GB free.
- 18:20Z Item 1 done (OLD code, release scratch build in the shared target, touch first): the scratch probe loop gives
  U2's full hashes exactly, whole log = v1 tag + datagram entries (every v1 entry is a datagram):
  P2 s1 913db8e914bdb773f6f19ac404c7098f711798790fcd1a861502502764b11ecc (2,000,000 entries);
  B1 s1 feabf2f02d023fa9784b2f9630ec6e2ceeed0bcdd28507c203b0ba40de1c25bf (2,020,000);
  P4 s1 b8a04f2f8ddc0d39624d853730636c50ffb354ddb187938066fabc156c1f602b (2,000,000).
  Pins, FNV-1a-64 of (v1 tag + datagram entries), mixed scenario (3,000 x 1 ms, datagram each way, control up every
  3rd, 1,500 B full state down every 5th, 4 s drain): P2 s1 0xadfbebe4bb1abf64; B1 (burst at 1 s) s1
  0x43749a62922e9b35; P4 s1 0x246e7a241a2a0a18.
- 21:48Z RESUMED after the spend-limit cut (~18:40Z). Checked: HEAD 097f897; plan sha256 7ec1b324 again; df 13 GB.
  Found on disk, unlogged: item 2's model written (crates/l1/src/lib.rs, 1,869 lines, saved 18:27Z: spaces, acks'
  own flow, RFC 9002 loss rules, PTO with backoff, starting state, C2 sampling, events, `next_event_ns`, v2 log, the
  8 new bites, doc rewritten) and item 3's tests (crates/l1/src/tests.rs, 1,183 lines, 18:33Z: L-c1..L-c7, L-c9,
  8 bite tests + the two-thread test). Not yet done: the v1 pin test (L-c8's unit half), item 4 (tests/l1.rs still
  calls the removed `Profile::rto_ns` and expects 4 bites), and no build result is recorded, so I build from here.
- 21:51Z First build (touch first; df 13 GB): compiles clean. Fixed: `L1Net::force` planted on links not yet connected
  (index panic): plants now wait at the net and move to their link at its connect. Then 9 of 13 unit tests pass; 4
  fail on the tests' own scenarios, not the model: L-c2's band [140, 170] ms is too tight (60 seeds, P2 after 4 s at
  30 Hz: PTO mean 152.65 ms, min 138.28, max 175.59; analytic 100 + 4 x ~6.9 + 25 = ~152.6; the advisor's "about
  155-160" sits inside the spread, its mean 2-3 % lower); L-c4 sees a spurious resend (3 messages in one instant:
  jitter reorders their pieces, and a later piece's ack declares an earlier one lost by the packet threshold: QUIC's
  real behaviour); L-c6's timer moved after the test read it (the last datagrams' acks came back after 4 s and
  changed the PTO); L-c3 under investigation.
- 21:54Z Item 3 (measured build) green: 13 of 13 lib tests pass. Test fixes, no model change: L-c2 and L-c3 compared
  every handed-out item, datagrams included, with the one control message (now filtered to Control); L-c2's band
  per seed [130, 180] ms plus a mean over seeds 1-10 in [145, 160] (mean 154.1; seeds give 139.2 to 175.2); L-c4's
  hand-out times now checked exactly against the in-order rule computed from the log alone (`piece_arrivals`,
  `handouts`), on both links, with spurious resends allowed; L-c6's warm-up ends at 3 s and the test checks the timer
  is unmoved 1 ns before its instant. Test renamed: lc1_..._is_declared_lost_and_resent (rustfmt's line limit).
- 21:55Z Bites build lib: 22 of 22 (13 + 8 bite tests + the two-thread test); each bite's own check fails by name:
  separate-spaces "L-c1: 12 probe timeouts fired"; no-pto-doubling "L-c3: ... probe timeout 2 at 6278.775 ms, not
  at 6428.162 ms"; acks-in-bursts "L-c3: link 0: no probe timeout fired"; one-draw-per-message "L-c4: its middle
  packet alone was not lost"; events-at-polls "L-c5: the fate logs differ (93267 and 93387 bytes)";
  sends-before-events "L-c6: the PTO due at 4166.215 ms did not resend message 0 in its instant";
  every-packet-sampled "L-c7: link 0 took 4000 RTT samples where the newest-packet rule gives 145";
  stream-sample-in-order "L-c7: link 1's samples after 4 s are [(4236.0, 226.0), ...]".
- 21:56Z Item 4 done (tests/l1.rs): `exchange`'s drain is n x spacing + 4 s (was + 8 x the removed `rto_ns`); the
  stream test rewritten to the new model (burst messages arrive in [3.04, 3.6) s, some outside it wait past the 60 ms
  ceiling, > 1,000 resends logged); the bites list test expects 12 (PM-0001-3's four, then npt-l1's eight). New:
  `v1s_tag_and_the_v2_logs_datagram_entries_hash_as_v1s_log_did` (L-c8's unit half: the three FNV pins from item 1,
  plus entry counts per kind) and `next_event_ns_names_the_next_arrival_then_its_acknowledgement_then_nothing`.
  Measured 15 of 15, bites 20 of 20.
- 21:57Z L-c8 hash pre-check on the NEW model (the item-1 scratch, release, touch first; not npt-lab, no judge: no
  judged run): SHA-256(v1 tag + datagram entries) = U2's committed hashes exactly: P2 s1 913db8e914bdb773...4b11ecc
  (log 3,959,634 entries, 2,000,000 datagrams); B1 s1 feabf2f02d023fa9...0de1c25bf (3,979,629; 2,020,000); P4 s1
  b8a04f2f8ddc0d39...0c1f602b (3,899,978; 2,000,000). Whole v2 logs (for the record): P2 43745683ab36d67b...,
  B1 722f20713e18352e..., P4 432693efb9698c05.... Pins unchanged on the new model (0xadfbebe4bb1abf64,
  0x43749a62922e9b35, 0x246e7a241a2a0a18).
- 22:01Z Clippy -D warnings on npt-l1: host all targets and wasm32, measured and bites builds, all clean; rustdoc -D
  warnings both builds clean; `cargo fmt -p npt-l1` applied, `fmt --all --check` OK. Doc: "What L1 leaves out" adds
  the PTO that finds only already-acknowledged data (counts, sends nothing; quinn sends a PING). Source comments no
  longer cite these notes. Hashes before the in-place plants: src/tests.rs 270ec318..., src/lib.rs e1c17bef...,
  tests/l1.rs 131d817b.... Next: the 8 in-place plants, one at a time, then check.mjs twice.
- 22:02Z The 8 in-place plants DONE, one at a time in src/tests.rs (the L-c test's own `#[test]` built on
  `L1Net::with_bite(seed, id)` in place of `L1Net::new(seed)`; touch; `cargo test -p npt-l1 --features bites --lib
  <test>`; restored with Edit; sha256 270ec318... checked after each). Each run: 1 test, FAILED by name:
  separate-spaces: lc1_... "L-c1: 12 probe timeouts fired (the first at 3144.168 ms)"; no-pto-doubling: lc3_...
  "probe timeout 2 at 6278.775 ms, not at 6428.162 ms"; acks-in-bursts: lc3_... "link 0: no probe timeout fired";
  one-draw-per-message: lc4_... "its middle packet alone was not lost"; events-at-polls: lc5_... "the fate logs
  differ (93267 and 93387 bytes)"; sends-before-events: lc6_... "the PTO due at 4166.215 ms did not resend message 0
  in its instant"; every-packet-sampled: lc7_... "link 0 took 4000 RTT samples where the newest-packet rule gives
  145"; stream-sample-in-order: lc7_... "link 1's samples after 4 s are [(4236.0, 226.0), (4236.0, 100.0)]". After
  the last: tests.rs 270ec318, lib.rs e1c17bef, tests/l1.rs 131d817b (all as before the plants).
- 22:05Z check.mjs run A (touch first, df 13 GB): 10 of 10 PASS, but counts measured 285 / bites 292. Expected:
  measured 275 + 10 = 285 (lib +8: L-c1..L-c7, L-c9; integration +2) and bites 292 + 19 = 311 (+10, +9 bite and
  two-thread tests in lib). 292 = 311 - 42 (npt-l1's bites tests now) + 23 (base npt-l1's): the bites step ran the
  other lane's (base) npt-l1 artifacts from the shared target. Run A is VOID (never reported); touch, rerun.
- 22:07Z Direct check after a touch: `cargo test --locked --workspace --all-features` gives 311, npt-l1 lib 22 and
  tests/l1.rs 20 (confirms the expected bites count). check.mjs run B: counts right (285 / 311) but the test step
  FAILED on npt-net's doctest, E0463 "can't find crate for `npt_l1`" (libnpt_l1-8649204fc5e411b5.rlib rewritten
  mid-run while another lane's `cargo test -p npt-proto` was running in the shared target). Run B VOID; rerun.
- 22:10Z check.mjs run C (22:09:33-22:10:30Z): test step FAILED at `-p npt-proto --test catalogue` (base npt-proto in
  this copy, untouched; it passed in run B): again another lane's artifact (U4a builds npt-proto, U4d npt-sim, both
  in the shared target now). Run C VOID. Next: rerun when no other cargo runs, until two clean runs in a row.
- 22:15Z Runs D and E (a background wait-for-idle loop, then touch, then check.mjs): D (22:11:22-22:12:17Z) FAILED
  again at npt-net's doctest, E0463 for `npt_l1` (another lane's check.mjs started 22:13Z beside it); E (from
  22:12:17Z) 10 of 10 PASS, measured 285 / bites 311 (the expected counts). Both on the pre-change code below, so
  neither counts for the end. (My kill of the orphaned run-E process was refused by the classifier; it finished.)
- 22:15Z DESIGN CHANGE (A19: no public item beyond §4.1's list): `L1Net::BITES` back to its U2 form, `[&str; 4]`,
  PM-0001-3's ids only, and `with_bite` unchanged from U2; the 8 link-model bites go to `MODEL_BITE_IDS` and a private
  `L1Net::with_model_bite`, both `cfg(all(test, feature = "bites"))`: npt-lab and the marks cannot reach them (the
  plan's "Bites (unit, per instance)"); their planted code stays in every bites build (each variant is still built by
  its `bitten!` comparison, so no dead code). New bites-build unit test `the_model_bites_are_chosen_only_here`.
  tests/l1.rs's bites list test back to U2's text (4 ids). Expected now: measured 285, bites 312. The plants must be
  redone on this code.
- 22:16Z On the changed code (touch first): npt-l1 measured lib 13 + tests/l1.rs 15 = 28; bites lib 23 + 20 = 43
  (`--list` names the new ones: the 8 lcN tests, the 8 bite_* tests, the two-thread test,
  the_model_bites_are_chosen_only_here; v1s_tag_..., next_event_ns_...). Clippy -D warnings host all targets and
  wasm32, both builds ("Checking npt-l1" each time), rustdoc -D warnings both builds, fmt --all --check: clean.
  Hashes before the second round of plants: src/tests.rs c21a362f..., src/lib.rs 1647e75d..., tests/l1.rs 639ab9c5....
- 22:18Z Plants round 2 DONE on the final code (as round 1, through `L1Net::with_model_bite`; touch before each; each
  run 1 test, FAILED by name, the same 8 messages as round 1, word for word; tests.rs c21a362f checked after each
  restore). After the last: tests.rs c21a362f, lib.rs 1647e75d, tests/l1.rs 639ab9c5. Next: check.mjs twice, waiting
  for no other cargo before each.
- 22:22Z check.mjs run F on the final code (touch first, df 13 GB; 22:19:34-22:22:16Z): all 10 steps PASS; test:
  measured 285, bites 312, 0 failed, 0 skips (both the expected counts: 275 + 10, 292 + 20). The other lane's
  check.mjs (wf_f54246ff-e0d-1) ran beside it from 22:19:57Z; the exact counts say my artifacts ran. Run G started
  22:22:16Z.
- 22:25Z Run G (22:22:21-22:24:59Z): counts right (285 / 312, 0 failed) but the test step FAILED on npt-net's doctest
  in the bites build, E0463 for `npt_l1` (`--extern npt_l1=libnpt_l1-8649204fc5e411b5.rlib`), while the other lane's
  `cargo test --workspace` compiled npt_net and wrote libnpt_l1-af5441632d39f304.rlib (2.2 MB: its base npt-l1) at
  22:24:28Z. Same signature as runs B and D, each time with another lane's cargo building beside it. G VOID; F stands
  alone, so two clean runs in a row are still owed. Next: a background loop that waits for 30 s with no other cargo
  or check.mjs, touches, runs check.mjs, and repeats until two consecutive passes with exactly 285 / 312 (6 tries).
- 06:33Z (2026-11-10) RESUMED after the second spend-limit cut (~22:35Z). Checked: HEAD 097f897; files equal the last
  hashes (tests.rs c21a362f, lib.rs 1647e75d, tests/l1.rs 639ab9c5); no cargo or check.mjs running; df 12 GB free.
  The wait-and-run loop left no log, so no run after G counts. Next: re-read §5 U4c against the code (pub items,
  tests), then check.mjs twice in a row, clippy both builds host and wasm32, fmt --check.
- 06:38Z Re-read §5 U4c, §4.1, advisor-u3q.md:160-173 and advisor-u2.md:72-103 against the code: all items there.
  Pub items vs base: added `L1Net::next_event_ns`; removed `Profile::rto_ns` (U2's RTO model, gone with C1'; no other
  crate uses it). check.mjs run H (no other cargo running; touch first; df 12 GB; 06:33:33-06:37:56Z): all 10 steps
  PASS; measured 285, bites 312, 0 failed, 0 skips (fmt, clippy, clippy-wasm32, doc, test, deps, refs, marks, self
  48/48, redact 10/10). Run I next, at once.
- 06:41Z check.mjs run I (no other cargo at start; touch first; df 12 GB; 06:38:04-06:40:33Z): all 10 steps PASS;
  measured 285, bites 312, 0 failed, 0 skips. H and I: two clean runs in a row, exact counts. Next: the explicit
  clippy and fmt lines and the `--list` names for the report.
- 06:45Z Explicit lines on the final code (touch first; another lane's cargo was running, so the shared lock
  serialized us): `cargo +1.99.1 fmt --all --check` OK; `clippy --locked -p npt-l1 -- -D warnings` host all targets,
  host all targets + bites, wasm32, wasm32 + bites: each "Checking npt-l1 (this copy's path)", clean. `--list`:
  measured lib 13 + tests/l1.rs 15; bites lib 23 + tests/l1.rs 20 (names as at 22:16Z). Found for the report:
  advisor-u4.md:105 says "L1 starts at 333 ms after U4c", but npt-link's `rtt_ns` contract (link/src/lib.rs:79-81:
  `None` before its first sample) is what L1 follows: the 333 ms starting state drives L1's timers only, and
  `rtt_ns` stays `None` until a sample (quinn's `rtt()` would report 333 ms then). Not mine to decide: flagged.
  Final hashes: tests.rs c21a362f, lib.rs 1647e75d, tests/l1.rs 639ab9c5.
- DONE 06:46Z.

## Fix pass, round 1

Fixer: a workflow agent, same copy, base 097f897, after round 1's notes (main folder .checkpoints/netcode-proof-test:
u4c-review-netcode.md, u4c-review-rust.md, u4c-review-test.md). Files: crates/l1 only (src/lib.rs, src/tests.rs,
tests/l1.rs). Rules as §7.1 above, plus: wait for no other cargo/rustc before each counted run, touch every file under
crates/ before and every 2 s during it; plants in place one at a time, restored, sha256-checked; commit nothing.
Start hashes: lib.rs 1647e75d, src/tests.rs c21a362f, tests/l1.rs 639ab9c5 (= the build's).

### Plan
1. Read lib.rs and src/tests.rs whole (event queue, next_event_ns, loss detection, the L-c tests, `fails()`).
2. M1: a test that drives the net only by `next_event_ns` (and the script's real send instants), fails if it returns
   None while an event is pending or names a later instant than one that changes state; covering arrivals, acks,
   loss/PTO timers and the reorder step's held hand-outs (gate + 1). Plants: next_event_ns -> None; blind to timers;
   blind to held hand-outs: each must fail it.
3. M2: pieces sent 1 ms apart over a fixed delay (no jitter, no loss but a forced loss of the first); the first must be
   declared lost by count exactly at the ack of the third later packet (not at the second's, and before the time
   threshold). Plants: PACKET_THRESHOLD 1,000,000 and 2: each must fail it.
4. Lows: L1 latest-RTT term (RTT step-up case; plant srtt-only); L2 no ack Lost outside a burst in a lossy run's v2
   log (plant acks lost at the profile's rate); L3 L-c6 sends link 3 first (plant link-first order); L4 tests/l1.rs
   burst comment: assert or reword (plant no-packet-threshold); N1 `fails()` takes the expected L-c number.
5. Other Mediums in crates/l1 (netcode F1, F2; rust M = M1): F1 needs a ruling (no handshake, plan T12), F2 is in
   crates/lab: list both for the main session; a doc line for F1's "left out" gap if it is a fact, not a ruling.
6. End: clippy (host, wasm32, both builds), fmt --check, check.mjs twice in a row (10 of 10; counts as deltas on
   285 / 312), sha256 of each changed file, evidence per fix in this section.

### Progress
- 15:47Z Start: hashes = the build's (1647e75d, c21a362f, 639ab9c5); df 12 GB free; no cargo/rustc running.
- 15:55Z Item 1 (read) done. Design: Rig records each hand-out's call instant (`at`); `drive()` = an event-driven
  host (calls only at next_event_ns and the script's own send instants; sends, then polls); `woken()` = every
  hand-out in its due instant and every internal event instant (log arrivals, PTOs, losses declared) a call instant.
  L-c5's event mode moves to `drive()` with the script's real send instants (its doc already claimed that); new test
  `a_host_called_only_at_next_event_ns_and_its_sends_misses_nothing` (quiet PTO, loss timer, P0 gate + 1, held and
  overtaken datagrams; completeness before a late poll). M2: `the_packet_threshold_...` (5 one-piece messages 1 ms
  apart, 100 ms fixed RTT). L1: time-threshold trials on a jittered link (10 per seed, step-ups required). L2 in
  tests/l1.rs on `mixed(B1)`'s canonical log. L3: lc6 sends link 3 first. L4: the burst comment reworded, and its
  "the PTO never fires" asserted (no stream resend sent in [2.2 s, 3 s)). N1: fails(r, "L-cN", words of the check).
  F1: a doc line in "What L1 leaves out" (no handshake), its ruling for the main session; F2: crates/lab, not mine.
- 16:02Z Code written (tests.rs: Rig.at, drive, woken, Lc5 struct, L-c5's event mode, host_at_next_event,
  packet_threshold, time_threshold, lc6 link order; tests/l1.rs: the burst comment + its assertion,
  acknowledgements_are_lost_only_inside_bursts; lib.rs: the handshake line). Build 1 (run.sh; this copy's npt-l1
  compiled; no foreign cargo): measured lib 16 of 16, tests/l1.rs 16 of 16. `cargo fmt -p npt-l1` applied.
- 16:06Z N1 done. Plant (tests.rs, bite_no_pto_doubling_fails_lc3 given acks-in-bursts, which trips L-c3's other check)
  with the OLD fails(): bites lib 26 of 26 PASS, the planted test "ok" on "L-c3: link 0: no probe timeout fired"
  (the gap: n1-before.log). New fails(r, "L-cN", words of the check) for all 8 bite tests; same plant: FAILED by name,
  "failed on another check than L-c3's \"doubles at each expiry\": L-c3: link 0: no probe timeout fired"
  (n1-after-plant.log). Restored (grep: 2 no-pto-doubling ids, as before): bites lib 26 of 26 (n1-after-clean.log);
  each bite still fails its own check (L-c5 "the fate logs differ", L-c6 "did not resend message 0 in its instant",
  the other six as the build's). Time-threshold trials: steps up 5, 6, 1 on seeds 1-3 (deterministic).
  Hashes now, the plants' base: lib.rs d6e6e130, tests.rs ef2ecfd5, tests/l1.rs 9c23f9cc.
- 16:10Z M1 done: six plants in lib.rs `Net::next_event`, one at a time, measured build, all npt-l1 tests (run.sh: this
  copy's npt-l1 compiled each time, no foreign cargo/rustc), restored with Edit, sha256 d6e6e130 checked after each:
  - none (always None): new test FAILED "next_event_ns: link 0, end 1: 31 of 33 items handed out when the host stopped
    at 3001.000 ms (next_event_ns said nothing was left)"; L-c5 FAILED "L-c5: link 0: an item that arrived at
    51.757190 ms was handed out at 67.000000 ms, not in its instant, 51.757190 ms"; also tests/l1.rs
    next_event_ns_names_... ("the datagram's arrival"). [m1-none.log]
  - no timers (loss and PTO timers ignored): new test FAILED "... 32 of 33 items handed out when the host stopped at
    3101.000 ms ..." (the loss-timer message); L-c5 FAILED "L-c5: the host was never called at 23 instants with an
    internal event, the first 112.781568 ms"; tests/l1.rs 16 of 16 pass (round 1's gap). [m1-no-timers.log]
  - no ready (hand-out instants ignored): new test FAILED "next_event_ns: link 1: an item that arrived at 20.000000 ms
    was handed out at 30.000000 ms, not in its instant, 20.000001 ms"; L-c5 passes (no same-instant arrivals in its
    script: expected). [m1-no-ready.log]
  - skips (max for min of event and hand-out): new test FAILED, the same message as no-ready. [m1-skips.log]
  - no acks (acknowledgement arrivals ignored): new test FAILED "next_event_ns: the host was never called at 37
    instants with an internal event, the first 101.000000 ms"; L-c5 FAILED "... never called at 876 instants ..., the
    first 102.651679 ms"; tests/l1.rs next_event_ns_names_... ("its acknowledgement's arrival"). [m1-no-acks.log]
  - no held (the reorder step's HeldDue ignored): new test FAILED "next_event_ns: link 2: an item that arrived at
    80.000000 ms was handed out at 84.333333 ms, not in its instant, 80.000000 ms"; L-c5 passes. [m1-no-held.log]
  Clean (build 1, 16:02Z): both pass.
- 16:11Z M2 done (lib.rs PACKET_THRESHOLD, restored, d6e6e130 checked after each): 1,000,000: new test FAILED
  "packet threshold: losses declared at [1112.5] ms (by count [false]), not the first message's alone, by count, at
  the third later packet's acknowledgement, 1,103 ms" [m2-threshold-1m.log]; 2: FAILED "... declared at [1102.0] ms
  (by count [true]) ..." [m2-threshold-2.log]. All other npt-l1 tests pass under both (round 1's gap). Clean: passes.
- 16:12Z L1 done (lib.rs Rtt::loss_delay_ns from the smoothed RTT only): new test FAILED "time threshold: trial 1:
  declared lost Some(110.514853) ms after its send, not 9/8 x max(98.235, 100.619) = 113.196 ms" [l1-srtt-only.log];
  restored d6e6e130; clean passes (steps up 5, 6, 1 on seeds 1-3).
- 16:12Z L2 done (lib.rs send_ack: acknowledgements also lost at the profile's rate, as kind 2): new test FAILED "361
  acknowledgements with the wrong fate, the first (kind, sent ns) Some((2, 69879691))"; every other npt-l1 test passes
  under it [l2-acks-at-rate.log]; restored d6e6e130; clean passes.
- 16:14Z L3: FIRST TRY WRONG, found by its plant. Sending link 3 first on P0 changed nothing: on P0 the second send's
  own call applies the first link's arrival before it sends, so the order follows the sends and the rank is never
  consulted (the link-first plant passed every test, the new check included) [l3-link-first.log]. Fixed: links 2 and 3
  get a fixed 100 ms round trip (`fixed(0)`), both datagrams sent at 20 ms (link 3 first) arrive at 70 ms and one call
  applies both; the check reads the acknowledgements sent at 70 ms. Plant (Event::rank as (time, link, order, key,
  ...)): lc6 FAILED "L-c6: the two arrivals at 70 ms were not acknowledged in fate-log order, link 3's first (links 2
  and 3 at Some(6), Some(7))" [l3-link-first-2.log]; restored d6e6e130; clean passes.
- 16:16Z L4 done (tests/l1.rs comment reworded: "by the loss rules (the unit tests pin the packet and the time
  threshold)", no claim left on the packet threshold alone; its PTO claim asserted: no stream packet resent in
  [2.2 s, 3 s)). Plant (lib.rs set_timer: the PTO from the oldest unacknowledged stream packet, not the last
  ack-eliciting send; round 1 saw it caught by L-c1 alone): streams_lose_nothing_... FAILED "2 stream packets resent
  inside the burst after 2.2 s, the first at 2415856348 ns: a PTO fired" (and L-c1) [l4-pto-oldest.log]; restored
  d6e6e130; clean passes.
- 16:17Z Clean, bites build (after `cargo fmt -p npt-l1`): lib 26 of 26, tests/l1.rs 21 of 21 [clean2-bites.log];
  sends-before-events still fails L-c6 on "did not resend message 0 in its instant". Hashes: lib.rs d6e6e130,
  tests.rs 50cfbcdc, tests/l1.rs 9c23f9cc. tests.rs's only change since the M1, M2, L1 and N1 plant runs (on ef2ecfd5)
  is lc6's (the L3 fix), which none of those runs' failing tests use. Next: fmt --check, check.mjs twice.
- 20:40Z RESUMED (a new fixer agent; the run above was cut after 16:26Z). Found: lib.rs d6e6e130 and tests/l1.rs
  9c23f9cc as last recorded, but src/tests.rs e8014c67, NOT the 50cfbcdc recorded at 16:17Z: an unrecorded change
  after the last line (no copy of 50cfbcdc kept). Two check.mjs logs exist with no notes line (u4c-fix/check-1.log
  16:21-16:24Z, check-2.log 16:24-16:26Z: 10 of 10, measured 289, bites 316); check-1 ended with a foreign cargo
  running (npt-sim zz_probe_es), and neither is known to have run on e8014c67: NEITHER COUNTS. Plan for the rest: read
  the three files' diffs whole; re-check each fix against its finding; redo every plant on the final code (the tests.rs
  change is unknown, so the earlier plant evidence is the record of the method, not of the final file); then fmt
  --check, check.mjs twice in a row, hashes, report.
- 20:50Z Read whole: lib.rs (1,898 lines), src/tests.rs (1,723), tests/l1.rs's diff. lib.rs differs from the build
  only in the handshake sentence (lines 54-57): every line the three reviewers cite sits exactly 3 lines lower now
  (PACKET_THRESHOLD :111 -> :114, detect_lost :1556 -> :1559, the ack loss :1423 -> :1426, Rtt::START :610 -> :613,
  BITES :1818 -> :1821). The fixes in the code match the 15:55Z design. Final-code plant list (run.sh, the earlier
  fixer's script, unchanged: waits for 5 s with no cargo/rustc, df, touch before and every 2 s, foreign processes
  logged; measured runs `cargo test --locked -p npt-l1 --no-fail-fast`, bites runs add `--features bites` and
  `-- --nocapture`): clean measured + clean bites first; M1 x6 (none, no-timers, no-ready, skips, no-acks, no-held);
  M2 x2 (threshold 1,000,000, 2); L1 srtt-only; L2 acks at the profile's rate; L3 rank link-first; L4 PTO from the
  oldest unacked; N1 x2 (another check of the same test; another L-c test); then clean again, both builds. Logs in
  u4c-fix/r2-<name>.log (the earlier fixer's logs kept as they are).
  (Clock note: the two lines above were stamped 20:40Z and 20:50Z by estimate; `date -u` read 20:41:44Z just after
  them. Times below are read from the logs.)
- 20:41-20:42Z Clean, final code (lib.rs d6e6e130, tests.rs e8014c67, tests/l1.rs 9c23f9cc), this copy's npt-l1
  compiled each time: measured lib 16 of 16, tests/l1.rs 16 of 16 [r2-clean-measured.log]; bites lib 26 of 26,
  tests/l1.rs 21 of 21 [r2-clean-bites.log], each of the 8 bites failing its own check (the build's messages, word for
  word); L-c2 PTOs seeds 1-10 = round 1's (139.229 ... 145.595); time-threshold steps up 5, 6, 1 on seeds 1-3. (A
  foreign `cargo clippy`/`cargo check` was seen at the bites run's end, after its tests had run.)
- 20:44-20:48Z M1 on the final code: six plants in lib.rs `Net::next_event`, one at a time, measured build, restored
  with Edit, sha256 d6e6e130 checked after each; no foreign cargo/rustc at any run's end:
  - none (returns None): new test FAILED "next_event_ns: link 0, end 1: 31 of 33 items handed out when the host
    stopped at 3001.000 ms (next_event_ns said nothing was left)"; L-c5 FAILED "L-c5: link 0: an item that arrived at
    51.757190 ms was handed out at 67.000000 ms, not in its instant, 51.757190 ms"; tests/l1.rs next_event_ns_names_...
    FAILED ("the datagram's arrival") [r2-m1-none.log].
  - no timers (Timer events left out): new test FAILED "... 32 of 33 items handed out when the host stopped at
    3101.000 ms ..."; L-c5 FAILED "L-c5: the host was never called at 23 instants with an internal event, the first
    112.781568 ms"; tests/l1.rs 16 of 16 pass [r2-m1-no-timers.log].
  - no ready (hand-out instants left out): new test FAILED "next_event_ns: link 1: an item that arrived at 20.000000
    ms was handed out at 30.000000 ms, not in its instant, 20.000001 ms"; L-c5 passes [r2-m1-no-ready.log].
  - skips (max in place of min): new test FAILED, the same message [r2-m1-skips.log].
  - no acks (Ack events left out): new test FAILED "next_event_ns: the host was never called at 37 instants with an
    internal event, the first 101.000000 ms"; L-c5 FAILED "... never called at 876 instants ..., the first 102.651679
    ms"; tests/l1.rs next_event_ns_names_... FAILED ("its acknowledgement's arrival") [r2-m1-no-acks.log].
  - no held (HeldDue events left out): new test FAILED "next_event_ns: link 2: an item that arrived at 80.000000 ms
    was handed out at 84.333333 ms, not in its instant, 80.000000 ms"; L-c5 passes [r2-m1-no-held.log].
  All six equal the 16:10Z messages. Clean (20:41Z): both tests pass.
- 20:48-20:53Z The rest on the final code, one plant at a time, restored with Edit, sha256 checked after each (lib.rs
  d6e6e130, tests.rs e8014c67); this copy's npt-l1 compiled and no foreign cargo/rustc at each run's end:
  - M2, PACKET_THRESHOLD 1,000,000: the_packet_threshold_... FAILED "packet threshold: losses declared at [1112.5] ms
    (by count [false]), not the first message's alone, by count, at the third later packet's acknowledgement, 1,103
    ms"; every other npt-l1 test passes [r2-m2-threshold-1m.log]. PACKET_THRESHOLD 2: FAILED "... declared at
    [1102.0] ms (by count [true]) ..."; every other test passes [r2-m2-threshold-2.log].
  - L1, `Rtt::loss_delay_ns` from the smoothed RTT only: the_time_threshold_... FAILED "time threshold: trial 1:
    declared lost Some(110.514853) ms after its send, not 9/8 x max(98.235, 100.619) = 113.196 ms"; every other test
    passes [r2-l1-srtt-only.log].
  - L2, `send_ack`: acknowledgements also lost at the profile's rate outside bursts (Outcome::Lost, from the ack's
    own word 0): tests/l1.rs acknowledgements_are_lost_only_inside_bursts FAILED "361 acknowledgements with the wrong
    fate, the first (kind, sent ns) Some((1, 69879691))"; every other test passes [r2-l2-acks-at-rate.log]. (The
    16:12Z plant marked them kind 2; this one is the plainer defect, kind 1; the same 361 acknowledgements.)
  - L3, `Event::rank` as (time, link, order, key, ...): lc6 FAILED "L-c6: the two arrivals at 70 ms were not
    acknowledged in fate-log order, link 3's first (links 2 and 3 at Some(6), Some(7))"; every other test passes
    [r2-l3-link-first.log].
  - L4, `set_timer`: the PTO from the oldest unacknowledged retransmittable packet, not the last ack-eliciting send:
    tests/l1.rs streams_lose_nothing_... FAILED "2 stream packets resent inside the burst after 2.2 s, the first at
    2415856348 ns: a PTO fired"; also lc1 "L-c1: 1 probe timeouts fired (the first at 6645.003 ms)"
    [r2-l4-pto-oldest.log].
  - N1, bites build: (a) bite_no_pto_doubling_fails_lc3 given acks-in-bursts (it trips another L-c3 check): FAILED
    "failed on another check than L-c3's \"doubles at each expiry\": L-c3: link 0: no probe timeout fired"
    [r2-n1-other-check.log]; (b) bite_events_at_polls_fails_lc5 running lc1 on its bite (another L-c test): FAILED
    "failed on another check than L-c5's \"the fate logs differ\": L-c1: message 0: declared lost 166.667 ms after its
    send, outside [113.3, 153.3] ms" [r2-n1-other-test.log]. The old fails() (any "L-c" error) passes both; shown for
    (a) at 16:06Z [n1-before.log]. Every other test passes in both runs (25 of 26 lib, 21 of 21 tests/l1.rs).
  After the last: lib.rs d6e6e130, tests.rs e8014c67, tests/l1.rs 9c23f9cc (= the clean runs' files).
- 20:53-20:54Z Clean again, measured: lib 16 of 16, tests/l1.rs 16 of 16 [r2-clean2-measured.log] (the bites build's
  clean run, 20:41Z, was on these same bytes). `cargo +1.99.1 fmt --all --check`: exit 0, no diff [r2-fmt.log].
  Counted check.mjs runs go through u4c-fix/run2.sh (run.sh plus a 20 s idle wait and, every 2 s during the run, a log
  of each cargo/rustc/clippy/rustdoc process whose working directory is outside this copy, with its
  CARGO_TARGET_DIR: <label>.foreign.log). Expected counts: measured 285 + 4 = 289 (lib +3: the next_event_ns host,
  the packet threshold, the time threshold; tests/l1.rs +1: acknowledgements lost only inside bursts), bites
  312 + 4 = 316.
- 20:55-21:04Z check.mjs r2-check-1 (20:55:14-20:58:54Z, after 20 s with no cargo/rustc anywhere; touch every 2 s):
  all 10 PASS; measured 289, bites 316, 0 failed, 0 skips (= 285 + 4, 312 + 4); fmt 0.3 s, clippy 33.8, wasm32 4.2,
  doc 25.6, test 149.9, deps 0.5, refs 0.9 (562), marks 3.2 (81, drafts 626df678), self 0.9 (48/48), redact 0.7 (10/10,
  158 items, 0 findings). Foreign lanes in the shared target in its last 8 s (from 20:58:46Z: agent-a3b8dfeb01ed52b50
  `cargo build --tests -p npt-log -p npt-judge`, then wf_f54246ff-e0d-1 `cargo test -p npt-sim --test replay` at its
  end) [r2-check-1.log, .foreign.log]. r2-check-2 (21:02:04-21:04:02Z, the same waits): all 10 PASS; measured 289,
  bites 316, 0 failed, 0 skips; fmt 0.4, clippy 3.1, wasm32 0.6, doc 19.5, test 89.8, deps 0.5, refs 0.0 (562),
  marks 2.0 (81, 626df678), self 1.1 (48/48), redact 0.7 (10/10, 158, 0). Foreign in the shared target: 21:02:12-26Z
  agent-a3b8... `cargo test -p npt-judge --test fixtures` and `cargo run -p npt-judge -- --check-marks` (during my
  doc step); 21:03:58-04:00Z wf_f54246ff-e0d-1 `cargo check`/`clippy -p npt-sim` (during my marks step). Neither
  foreign lane built npt-l1 or npt-link in either window. The other lanes (U4j, U4d) keep the shared target busy; a
  third run follows with a 60 s idle wait, to try for a quiet pair.
