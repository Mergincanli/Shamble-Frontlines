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
- B1-B6 done (key: TE 1 High/14 Med/9 Low; SA 3 High/8 Med/6 Low; SC-1 phase-locked pings; SC-2 = TE-1 holds; SC-3: SA-1, SA-3 hold).
- B7 done. Facts read at 11fd190:
  - room/src/input.rs:80-125 offer order: Repeat (:81-84) -> newest_seq set (:89) -> PressRefused (:90-106) -> Late (:107-113: tick <= ran_through, OR equal to a buffered tick :110) -> Full (:114-116). -58 sites CMD_SEQ :11, PRESS :13, after16 :82, after8 :98. newest_received :68-71.
  - room.rs:791 offers a datagram's commands oldest first; client lib.rs:442 push_front, :443 truncate 8, :464 every unacked command in each Input.
  - room.rs:363-369 canonical; :783-787 NeverSent; :1322 newest_received in header; :1024-1032 overflow inside forced resync -> Kick; :103 overflow_resyncs.
  - client lib.rs:485-488 steps at 1/tick_hz of host clock (SC-1 holds). plan.md:618-624 RQ8 order (NA-4 basis holds).
  - l1 lib.rs:149-156, :214-218: P2 = 50 ms +-10 each way.
  - pump.rs:16, :24, :26; :458-471 ping limit; :500-516 pong; :510 hold clamp uncounted.
  - scenario.rs:1165-1186 client_counts fields; r8 :2120, r9 :2175.
  - marks.json:4909-4943; :4988-5047; :5049-5059; :5061-5113; :5107-5108; :5118. SERIES.md:33-34, :93, :101, :144-145, :155-159, :170, :240, :342.
  - plan :335; :351-352; :360-390; :434-435; :453-476; :971-974. ADR 0001:226-255. carries :513-534, :343-354, :169-179, :166-168, :180-183, :364-381.
  - c-u4r-runs.mjs:131-132 FLOOR 4 GiB; :138-153 MB = 10^6 B, 56 MB; :176-178 judge fit MiB.
- New finding NA-13 Medium: room/src/input.rs:110 (equal-to-buffered stamp is Late under B). Not carried.
- B8 done (disposition: TE 17 fixed / 6 question / 1 run / 0 declined; SA 10 / 7 / 0 / 0; SC-1 fixed; NA-3, -4, -5, -7 changed; NA-13 new. Questions Q1-Q16. D1-D15.)
- Counts rev 2: measured +100, bites +115, neg 60; conditional +3 Q6 (b), +1 Q9 (a), +1 Q11 (iii) (c), +1 Q13 (a), +5 T-6. Bites 17 (+1 Q15 (b)). Judged runs 65 (+1 Q15 (b), +13 Q10 (c)). Pre-runs 14 (+4 Q11 (iv)). Carries 14; WORDS 15; recheck 33; open items 17; questions 16; D 15.
- Disk in MiB: -20 373.9; -21 2,617.0; -22 320.4; -47 none; sets 3,311.3; pre-runs about 190 (14) to 250 (18); cap runs about 95; total about 3,596 to 3,656 vs 3,593 above floor (7,689 - 4,096). Judge -21: 6,426.7 MiB < 10,240.
- B9 W1 done (header, change list, counts, sections 0-1).
- B9 W2 done: section 2 (2.1-2.13) is rev 2 in the file; marker line now says sections 3-12 and findings come from rev 1 at 828e2f4. f-table verified: measured sum 100, neg 60.
- Next: B9 W3 (sections 3-5). Section 5 items to write: 5.1 run list + conditional rows; 5.2 table with TE-3 note fixed, -22 row TE-22/SA-16; 5.3 pre-runs table (14 + 4) with lengths (TE-13), both late readings (TE-4), 7 bitten runs (TE-6, D12, D13), forged A repeated at seed 1 (TE-24, D11), -20 P3 B (TE-3); 5.4 judge calls; 5.5 disk MiB table; 5.6 unchanged. Section 3: row 39 f31. Section 4: SA-17 logging rule.
