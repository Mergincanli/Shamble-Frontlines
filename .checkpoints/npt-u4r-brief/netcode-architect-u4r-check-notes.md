Task: netcode-architect check of U4r builder brief rev 1 (npt-u4r-brief, night item 4 step 2): -30 pre-check, O1/carry 40, RTT argument, -18 fresh-delta reading.

Plan (blocks, in order):
B1 read the draft u4r-brief-draft.md (+ drafter notes test-engineer-u4r.md)
B2 read plan item 4 (git show 2dc1f31:night-2026-11-10.md) + u4-build-plan.md:730-800 and §4.15, §4.6, §4.9
B3 read advisor-u4.md cond 2, 3, Q10, :343-347; u4s-carries.md:176-182; advisor-u3q.md:180-186
B4 read code at df8975b: crates/link lib.rs (rtt_ns), crates/room (tolerance, cap, :690-691, :799-800), server, client
B5 read marks.json -18 entry + catalogue netcode-proof-messages.md
B6 write check section 1 (-30)
B7 write check section 2 (O1)
B8 write check section 3 (RTT)
B9 write check section 4 (-18) + outside scope + verdict
Then DONE.

Notes:
- B1 done (draft read: §7.2 at :405-445, §8 at :504-577, -18 row :397, U4R-Q10 :732-741, Q12 :751-758; drafter notes say DONE). Next B2.
- B2 done: plan :717-772 (U4r), :282-347 (§4.5-4.9), :398-436 (§4.12-4.13), :462-487 (§4.15), :540-714 (U4b..U4e); night plan item 4 read. Confirmed :690-691 = U4e rewind_at "(the integer half; the tolerance and cap rules land in U4h)"; :799-800 = U4h builds; :678-679 = U4d2 bites; :787 = U4g Predictor.
- B3 done: advisor-u4 cond 2 (:90-101), cond 3 (:102-111), Q10 (:208), O1 (:343-347); carries :176-182 = draft :234-240 word for word; u4-conditions-check.md:42-45 holds O1; advisor-u3q :180-186 read. marks.json -30 :6325-6390 ("3 recordings of 5 minutes ... all 6 runs are measured", :6389; bite late-command expect invalid rule late-zero :6380-6383). ADR 0001:1156-1162 option B = one per client per tick, repeat last on miss, late dropped; :1313-1315 run counts only if every command ran at its stamped tick and late counter 0.
- B4 done: at df8975b crates/room/src/lib.rs, crates/server/src/lib.rs = doc comment only; crates/client/src/lib.rs = BITES + grid_with_bite only. link lib.rs:203-205 Loopback rtt Some(0); Loopback delivers stamped at send time (:154-165, :184-192). l1: rtt_ns = smoothed Option, None before first sample (:350-367, :784-785) -> 333 ms only after U4c; sample read when the end's clock reaches the ack (:33-34, :512-532); P0 = 0 delay (:115-117); B1 = P2 + 1 s bursts (:135-139), in_burst half-open (:163-167); "never in the instant sent" (:30-31). U2 probe bursts at whole seconds (lab main.rs:89-91).
- B5 done: marks.json -18 :4236-4308 (checks after_link_ns max<=500ms, count>=480; notes :4307 "when it first decodes a delta after the link returns" = plain reading); catalogue: only PeerBurst row :209 (100 % both ways, 10 bursts) - no contradiction. judge eval.rs:1283 quote confirmed.
Findings (draft line -> source):
- N1 Medium: 7.2 :427-429 RTT rule says "its link's first RTT sample" - drops plan :755-757's "the server's L1 link"; client end's first sample (Join's ack) always precedes -> rule passes vacuously.
- N2 Medium: Q10 :732-741 / 8.2 item 3 :538-541 early-input bite cannot make the rule fire as worded at P0: server's first sample = JoinAccepted's ack, ~0 ms, next instant; first command runs >= the instant after receipt; fires only if Join+Input reach the pump in one call and the Input's tick is the tick due that instant. Fix: specify timing or show VOID logic on doctored records.
- N3 Low: 8.4 :576 "re-run needs no new log" + comparand = first command RUN; plan :755-756 "before the room reads that client's RTT for any command"; if U4h reads RTT at receipt, comparand = receipt instant. Also Loopback session's RTT source (rtt_ns vs ws pings, §4.9 :341-344). Carry to U4h.
- N4 Medium: "6 runs" reading (7.2 :439-441, 8.3 :554, Q12) - alternative reading from marks.json:6389 (-30's own shape, 3 recordings x 2 legs, all 6 measured) not weighed; Q12 must offer both.
- N5 Medium: O1 claim "nothing in the room reads the RTT" / "no code replaces or clips" rests on plan only; at df8975b room/server are empty (room lib.rs:1-5, server lib.rs:1-5), so must be rechecked vs U4e's merged code at <base>: add recheck row + git grep command.
- N6 Medium: replay clients' pacing and the room's tick base vs the recording's first stamped tick unspecified -> late_count>0 -> every pair VOID.
- N7 Medium: "per-tick server hashes" undefined; must be World state hash, not mirror hash.
- N8 Medium: wallclock bite path (not RTT, not lateness, not R-dependent); PASS = missed.
- N9 Medium: -18 heal counts Snapshot deltas only, never FullState/Resync; log Resyncs per burst.
- N10 Low: 8.2 item 3 :536 overbroad "every room decision".
- N11 Low: "which it acknowledges at once" wording.
- N12 Low: tie rule / observed sample instant / Loopback leg's logged value.
- N13 Low: late-command bite vs redundancy.
- N14 Low: 8.3 cost omits the VOID first clean pair (9 runs).
- N15 Low: -18 row details (marks.json:4307 plain wording; burst spacing and offset draw; df8975b rtt None).
- N16 Low: out-of-ring logging = addition.
- B6 done (check §1 written: N6, N7, N8, N11, N13, N16). Next B7 (O1: N4, N5, N14, confirmations).
