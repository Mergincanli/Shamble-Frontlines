Task: npt-u4r-brief step 3 -- write rev 2 of the U4r builder brief (one revision pass for netcode-architect's check N1-N16). Notes only.

## Plan (blocks, in order)
B1. Read the check whole (netcode-architect-u4r-check.md) + its notes.
B2. Read rev 1 (u4r-brief-draft.md) whole + drafter notes (test-engineer-u4r.md).
B3. Read the approved plan (night-2026-11-10.md at 2dc1f31, "Rules for every item", "Item 4").
B4. Verify each finding's cite at df8975b (.gate/npt-spike), decide accept/reject; notes line per finding.
B5. Write rev2-parts/00-changes.md (change list).
B6. rev2-parts/01.md: header + sections 1-2.
B7. rev2-parts/02.md: sections 3-6.
B8. rev2-parts/03.md: section 7.
B9. rev2-parts/04.md: section 8.
B10. rev2-parts/05.md: section 9 (recheck table).
B11. rev2-parts/06.md: sections 10-11.
B12. Assemble rev 2 into u4r-brief-draft.md in one Write (from the parts); recount; DONE.

## Notes
- B1 done (check read whole: 16 findings; scope -30 pre-check, O1, RTT argument, -18 fresh-delta).
- B2 done: rev 1 read whole (785 lines; sections 1-11; R1-R32; Q1-Q12; run items N1-N9). Drafter notes read (DONE, findings became Q1-Q12).
- Extra X1: rev 1's §11 run items are named N1-N9, colliding with the check's finding ids N1-N16. Rename run items D1-Dn; update cites at rev 1 lines 290, 452, 562, 591, 598-601, 609, 618, 663, 667, 766-784 (line 259's "arch N2" is carry 20's own words: keep).
- B3 done: plan item 4 = draft, check, test-engineer revises once (rev 2 + change list). Rules: no build/run/web; "needs a run (day)" with exact command.
- B4 verified at df8975b (all 16 ACCEPTED; corrections noted):
  - N1 ACCEPT: l1/src/lib.rs:456-458 `open[end]` client 0 server 1, `rtt: [Rtt; 2]`; :784-785 rtt_ns returns `rtt[self.end]`; plan :755-757 "the server's L1 link holds its first sample". Client end's Join ack ready at t0 (P0) -> rule vacuous if client end logged.
  - N2 ACCEPT, with a correction (X3): at df8975b the ack's sample time is at_ns = arrival_ns + ack_back_ns (l1 :515), at P0 arrival = send instant (delay 0: :418-420, :602, :686), packets delivered only after the send instant (:727-728) but acks applied when at_ns <= now (:527) at any send or poll (:582, :661, :714). So the server end's JoinAccepted sample is applicable from t1 itself (the send instant), not t2 as the check says; link trait :79-80 "as of the last call that took now_ns". So (a') fires only if, after U4c, the value the pump reads at t1's tick does not yet reflect the sample: unknowable at df8975b (predates U4c). Recommend (b') at U4r; (a') in the re-run only if R4/R15 at <base> show it can fire.
  - N3 ACCEPT: plan :755-758 (comparand "before the room reads that client's RTT for any command" vs logged "first command run"); §4.9 :341-344 (wt/L1 link rtt_ns; ws pings). Fix: log first command's receipt instant at U4r (one field); U4h carry names the comparand and the Loopback leg's session class. Also U4r's leg glue classes Loopback sessions with the link's rtt_ns (plan :749-750 assumes it).
  - N4 ACCEPT: advisor-u4.md:346 "re-run the pre-check pair after U4h merges (6 runs)"; marks.json:6389 "3 recordings ... so all 6 runs are measured", parts min 6 (:6344, :6353). Add: a reading "3 clean pairs on one recording" repeats a deterministic run (same recording, P0 draws no delay or loss: l1 :115-117), so not a candidate.
  - N5 ACCEPT: room/src/lib.rs:1-5, server/src/lib.rs:1-5 docs only at df8975b; R33 + run item D10. Add `-e rewind_at` to the second grep.
  - N6 ACCEPT: plan :741-743 silent on pacing; §4.15 :463-466 per-tick TickInput, no send instants, and the recording header (scene, seed, TICK_HZ, pool sizes, capacity) has NO start tick: correct the check's "recording header's start tick" to the recording run's npt-lab header `tick_start` (R30 asks whether `Recording` carries one). ADR 0001:1160-1162 option B; :1312-1315 late invalidates. Extra X2: Loopback delivers in the send instant (link/src/lib.rs:154-163 arrival_ns = now; :187), L1 only after it (l1 :30-31, :727-728) -> admits may land at different instants per leg; builder states whether an admit changes the World's hashed state (spawns are TickInput's: §4.15 :465) and logs each client's admit tick per leg. New test r6 (pacing: clean short replay, late_count 0 both legs).
  - N7 ACCEPT: plan :590 "the canonical hash (FNV-1a-64, spike-local)"; mirror hash §4.6 :300-302 depends on baseline/priority; marks.json:6361. Extra X4: rev 1's 2.1 log bullet lists no per-tick World hash in the room log, which the comparer needs (plan :634 "the comparer reads both legs' room logs"); add it; R30 asks its name.
  - N8 ACCEPT: plan :759; marks.json:6377 "a wall-clock read on one transport's command path" supports the fix; bucket on injected now (plan :694, :704).
  - N9 ACCEPT: §4.7 :321-326; marks.json:4269 "decodes deltas again", :4298 bite, :4307 note; catalogue :90 (Resync: client that "has lost its state"), :150 (FullState coded from per-type defaults). Recheck R34 + D11 (client Resync; event cap).
  - N10 ACCEPT: §4.7 :321-324 Kick(overflow); §4.5 :283-293. Reword 8.2 item 3; records per leg: admits (48), drops, kicks.
  - N11 ACCEPT (wording): plan :753-755 parenthetical "U4c's immediate acks".
  - N12 ACCEPT: (a) link trait :79-80; (b) RQ8 order (draft 2.1, plan.md:617-623) pump step 2 before tick step 3; (c) Loopback column "Some(0) from the start"; (d) l1 :349-353 None at df8975b.
  - N13 ACCEPT + extra: held out of every Input incl. redundant copies until its tick ran, THEN sent (delayed, never dropped: a command never sent is a miss, which the late counter may not count); records the delayed tick and late_count (expected 1); R17 asks how option B counts a late command (X5).
  - N14 ACCEPT: rev 1 :563-564 "plus 300 s if re-recorded" undercounts: 9 runs = 2,700 s when the replay-first pair is VOID.
  - N15 ACCEPT: (a) marks.json:4307 plain words; (b) offsets' draw and spacing stated by the builder; (c) §4.5 :289-290 per client and burst.
  - N16 ACCEPT: list the out-of-ring logging as an addition (with its reason).
- Q12 costs: (i) one recording: 6 runs 1,800 s; if replay-first shows replacements 9 runs 2,700 s. (ii) -30's shape: 10 runs + 2 recordings 3,600 s; with replacements on seed 4: 12 runs + 3 recordings 4,500 s. Q10 (a') in the re-run adds 2 runs (600 s).
- B4 done. Next B5 (change list).
