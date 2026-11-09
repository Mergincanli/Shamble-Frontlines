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
