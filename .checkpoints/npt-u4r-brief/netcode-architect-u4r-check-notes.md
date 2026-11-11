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
- B2 partial: plan :717-772 read (U4r), :282-347 (§4.5-4.9), :462-487 (§4.15), :540-714 (U4b..U4e). Confirmed :690-691 = U4e rewind_at "(the integer half; the tolerance and cap rules land in U4h)"; :799-800 = U4h builds. :678-679 = U4d2 bites text; :787 = U4g Predictor text (confirmed).
- Candidate concern (check later): pre-check "server hash" = World hash? links differ in acks -> encoder baseline; does any link-dependent room decision reach the sim (join/spawn tick, Kick(overflow), Resync)? start alignment of recording tick vs joins on each leg unspecified -> late_count risk.
