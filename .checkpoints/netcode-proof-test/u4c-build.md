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
