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
- B7 done (facts: input.rs:80-125 order, :89, :110; room.rs:791, :1322, :1024-1032; client lib.rs:442-443, :464, :485-488, :671-688; plan.md:618-624; l1 lib.rs:149-156, :214-218; pump.rs:458-471, :500-516, :510; scenario.rs:1165-1186, :2120, :2175; marks.json -21 checks/bites; SERIES rows; plan :335, :351-352, :360-390, :434-435, :453-476, :971-974; ADR 0001:226-255; carries 80/74/39/27/50/76/77; c-u4r-runs.mjs:131-178).
- New finding NA-13 Medium: room/src/input.rs:110 (equal-to-buffered stamp is Late under B). Not carried.
- B8 done (TE 17 fixed / 6 question / 1 run / 0 declined; SA 10 / 7 / 0 / 0; SC-1 fixed; NA-3, -4, -5, -7 changed; NA-13 new. Q1-Q16. D1-D15.)
- Counts rev 2: measured +100, bites +115, neg 60; conditional +3 Q6 (b), +1 Q9 (a), +1 Q11 (iii) (c), +1 Q13 (a), +5 T-6. Bites 17 (+1 Q15 (b)). Judged runs 65 (+1 Q15 (b), +13 Q10 (c)). Pre-runs 14 (+4 Q11 (iv); +1 Q15 (c) -> W4 adds p19 and updates the header counts and disk to 3,596-3,667). Carries 14; WORDS 15; recheck 33; open items 17; questions 16; D 15.
- Disk MiB: sets 3,311.3; pre-runs 190 (14) / 250 (18) / 261 (19); cap 95; total 3,596 to 3,667 vs 3,593. Judge -21 6,427 MiB.
- B9 W1, W2 done. W3 done: sections 3, 4 (SA-17 logging rule; lane with server by Q14 (a)/Q6 (d)), 5 (5.1-5.6 rev 2; 5.3 table p1-p18; 5.5 MiB) are rev 2 in the file; marker line says sections 6-12 and findings come from rev 1.
- Next: B9 W4 (sections 6-12, findings; header counts row for pre-runs and disk updated for Q15 (c); 5.3 p19; 5.5 pre-run row). Q recs: Q6 (b) + new (d) option; Q8 (b) at 400 ms (12 ticks), (ii) carry to U4h yes; Q9 (a) K = 16; Q11 (iii) (c), (v) (b); Q13 (b); Q14 (b); Q15 (c); Q16 (b) ACK_RING_TICKS.
