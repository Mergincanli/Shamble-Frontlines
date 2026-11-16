Task: night item 7 (npt-u4f-brief), 2026-11-16, Agents item 3: netcode-architect revises U4f's build brief once, for both checks (test-engineer, security-auditor) and SC-1, into rev 2 at u4f-brief-draft.md with a change list at its top.

(No Edit tool in this agent: this notes file and the draft are rewritten whole with Write at each save.)

## Plan (blocks, in order)
- B1 read the approved plan (git show 7812127...) -- input, not saved
- B2 read rev 1 draft (u4f-brief-draft.md, 837 lines)
- B3 read drafter's notes (netcode-architect-u4f.md, NA-1..NA-12)
- B4 read test-engineer check (TE-1..TE-24, NA verdicts, needs-a-run list)
- B5 read security-auditor check (T1-T29, SA-1..SA-17, NA verdicts, needs-a-run list)
- B6 read night checkpoint npt-u4f-brief.md open findings (SC-1, SC-2)
- B7 targeted reads at 11fd190 in the worktree, only where a finding needs a cite settled
- B8 disposition table: every TE-n, SA-n, SC-1, NA-n changed -> fixed / U4F-Q / Dn / declined (saved here)
- B9 write rev 2 in four cumulative writes: W1 header + change list + counts + sections 0-1; W2 + section 2; W3 + sections 3-5; W4 + sections 6-12 + findings
- B10 counts and budgets cross-check (one unit throughout)
- DONE

## Notes
- B1-B6 done (summaries in the previous save; key: TE 1 High/14 Med/9 Low; SA 3 High/8 Med/6 Low; SC-1 phase-locked pings; SC-2 = TE-1 holds; SC-3: SA-1, SA-3 hold).
- B7 done. Facts read at 11fd190:
  - room/src/input.rs:80-125 offer order: Repeat (:81-84) -> newest_seq set (:89) -> PressRefused (:90-106) -> Late (:107-113: tick <= ran_through, OR equal to a buffered tick :110) -> Full (:114-116). -58 sites CMD_SEQ :11, PRESS :13, after16 :82, after8 :98. newest_received :68-71.
  - room.rs:791 offers a datagram's commands oldest first (`iter().rev()`); client lib.rs:442 push_front newest-first, :443 truncate 8, :464 every unacked command in each Input. So an honest reordered datagram's commands are all repeats already received (or outside an 8-window): TE-2 lands in the repeat filter / SA-9's outside-window rule.
  - room.rs:363-369 canonical; :783-787 NeverSent; :1322 newest_received in header; :1024-1032 overflow inside forced resync -> Kick; :103 overflow_resyncs.
  - client lib.rs:485-488: each step at base + steps x 1e9 / tick_hz on the host clock: pings (sent at steps) have a whole-tick period -> SC-1 holds. plan.md:618-624: packets due, then server I/O (ping answers, hold ~0), then tick if due: Pong's tick = last_tick+1 (scenario.rs:459) -> NA-4 basis holds.
  - l1 lib.rs:149-156, :214-218: profile latency/jitter are round trip's; each direction half: P2 = 50 ms +-10 each way (D4's params right).
  - pump.rs:16 PING_INTERVAL_NS 250 ms (server's own); :24, :26; :458-471 ping limit; :500-516 pong, :510 hold clamp uncounted.
  - scenario.rs:1165-1186 client_counts writes commands, steps_without_command, duplicate_stamps (c.duplicates), late, press_refused, buffer_full, rewinds_refused, stale, events_queued, overflow_resyncs; r8 :2120, r9 :2175.
  - marks.json:4909-4943 forged checks; :4988-5047 fast clock, lag switch; :5049-5059 lag report; :5061-5113 bites; :5107-5108 not-older-than's "what"; :5118 notes.
  - SERIES.md:33-34, :93, :101, :144-145, :155-159, :170, :240, :342.
  - plan :335 cap formula; :351-352 tolerance incl. input-buffer wait; :360-390 §4.10; :434-435 SS no reloads; :453-476 §4.14; :971-974 readings PR route.
  - ADR 0001:226-255 (acks "newest one the server received" :228-229; checks :231-239; time :239-251).
  - carries :513-534 (80 (a)-(e)), :343-354 (74), :169-179 (39), :166-168 (27), :180-183 (50), :364-381 (76, 77).
  - c-u4r-runs.mjs:131-132 FLOOR 4 GiB; :138-153 MB = 10^6 B, 56 MB other runs; :176-178 judge fit MiB.
- New finding NA-13 Medium: room/src/input.rs:110: under option B a command whose stamp equals a buffered command's is `Late`, so a forged equal-tick command with a new sequence is counted as honest late, never kind 2; the no-newer-tick bite under B can pass as late. Proof: unit test (f13 second case). Not carried.
- B8 done. Disposition (change list):
  TE fixed 17: TE-3, -4, -6, -9, -10, -12, -13, -14, -15, -16, -17, -18, -19, -20, -21, -22, -23. TE question 6: TE-1 (Q9), TE-2 (Q9), TE-5 (Q11 (v)), TE-7 (Q15), TE-8 (Q11 (iii)), TE-11 (Q16). TE needs a run 1: TE-24 (D11). Declined 0.
  SA fixed 10: SA-1, -4, -8, -10, -12, -13, -14, -15, -16, -17. SA question 7: SA-2 (Q9), SA-3 (Q13), SA-5 (Q14), SA-6 (Q11 (v)), SA-7 (Q15), SA-9 (Q9), SA-11 (Q8). Needs a run 0. Declined 0.
  SC-1 fixed (accepted: 2.1, U4F-Q6, NA-4; D4 amended). NA changed: NA-3, NA-4, NA-5, NA-7; NA-13 new.
  Questions: Q1-Q12 kept (Q1, Q2, Q6, Q7, Q8, Q9, Q11 amended), new Q13 (SA-3 refused command and the ack; twins; rec (b) keep today's rule, twin script never makes a refused-whole command, its seq index skipped), Q14 (SA-5 logged counters, U4E-Q11 size/bucket reading, hold clamp; rec (b) report all, hold clamp uncounted with why, follow-up), Q15 (TE-7/SA-7 lag switch; rec (a) report the arithmetic + D13, marks unchanged), Q16 (TE-11 carry 80 (b) bound on newest move; rec (b) ACK_RING_TICKS; apart from C62H-BOUND). Q8 rec now 400 ms (12 ticks) per SA-11 (catch-up kind under B needs only > 9-L commands inside the bound); Q8 (ii) asks a carry to U4h.
  Needs a run: D1-D10 (D4 amended: phase-locked + bitten arrivals; D2 in MiB), new D11 (TE-24), D12 (TE-5/-6, SA-6 short bitten runs), D13 (TE-7/SA-7 lag-switch bitten), D14 (SA-1 cyclic triple against the unfixed sort, a plant), D15 (TE-3/-4 option-B short runs, both late readings). 15 in all.
- Counts for rev 2 (computed): tests measured +100, bites build +115 (100 + 14 bites-only unit proofs + f30); negative cases 60 (measured); conditional both builds: +3 U4F-Q6 (b), +1 U4F-Q9 (a) store bound, +1 U4F-Q13 (a), +1 U4F-Q11 (iii) (c), +5 T-6. Bites 17 (+1 by Q15 (b)). Judged runs 65 = 36 measured + 12 twins + 17 bite runs (+1 Q15 (b); +13 Q10 (c)). Pre-runs (5.3) 14 (+4 by Q11 (iv)). Carries 14 placeholders; WORDS 15; recheck rows 33; open items 17 rows (15 scout + SA-16 follow-up + SA-11 carry); questions 16; needs-a-run 15.
- Disk in MiB (56 MB = 53.41 MiB): -20 373.9; -21 2,617.0; -22 320.4 (3 x 53.4 + S30 bite 160.2); -47 none; sets 3,311.3; pre-runs about 190 (14) to 250 (18); cap runs about 95; total about 3,596 to 3,656 vs 3,593 above floor (7,689 - 4,096): no margin. Judge -21: 302.2 + 2.3403 x 2,617.0 = 6,426.7 MiB < 10,240.
- Test list rev 2 (Δ, neg): f1 1; f2 3; f3 2 n; f4 1 n; f5 1; f6 1 (lab); f7 1 n; f8 3 n; f9 0; f10 8 n; f11 2 n; f12 4 n; f13 2 n; f14 3 n; f15 11 n; f16 24; f17 1; f18 3; f19 1 n; f20 1 n; f21 1 n; f22 2 n; f23 1 n; f24 1 n; f25 1 n; f26 1 n; f27 1 n; f28 1; f29 1; f30 1 bites-only; f31 2; f32 2; f33 13 n.
- Next: B9 W1.
