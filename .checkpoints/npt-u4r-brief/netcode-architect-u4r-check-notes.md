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
- B1-B5 done (reads; facts recorded in the check file's sections).
- B6 done (check §1: N6, N7, N8, N11, N13, N16).
- B7 done (check §2: N4, N5, N14 + confirmations).
- B8 done (check §3: N1, N2, N3, N10, N12 + confirmations; N1 wording fixed).
- B9 done (check §4: N9, N15; §5 outside scope + confirmed; verdict at top).
Verdict: REVISE, 0 Critical, 0 High, 8 Medium, 8 Low.
Medium: N1 RTT rule's sample must be the server end's (draft :427-429 vs plan :755-757); N2 Q10 early-input bite cannot fire as worded at P0 (fires only if Join+Input in one pump call with the tick due then) - spec timing or doctored-record unit test; N4 "6 runs" second reading (marks.json:6389, 3 recordings x 2) unweighed in Q12; N5 O1 claims need a recheck vs U4e code at <base> (room/server empty at df8975b) - R33 + 2 git greps; N6 replay pacing + room tick base unstated -> late_count risk; N7 "server hashes" = World state hash, not mirror; N8 wallclock bite must reach a sim input not via RTT/lateness, R-independent; PASS = missed; N9 -18 heal counts Snapshot deltas only, never FullState; note resyncs per burst.
Low: N3 comparand run vs receipt + Loopback RTT source (U4h carry); N10 overbroad "every room decision"; N11 ack wording; N12 tie rule/observed instant/Loopback column/df8975b None; N13 late-command vs redundancy; N14 re-run cost 9 runs branch; N15 -18 row details; N16 out-of-ring = addition.
Not settled (day): R2, R4, N5 greps, whether U4e client sends Resync on undecodable delta.
DONE
