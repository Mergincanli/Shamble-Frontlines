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
- B11 final rewrite: NA-12 line in the change list (wording "older than 255 ticks"), tally NA 5 changed
- DONE

## Notes
- B1-B6 done (key: TE 1 High/14 Med/9 Low; SA 3 High/8 Med/6 Low; SC-1 phase-locked pings; SC-2 = TE-1 holds; SC-3: SA-1, SA-3 hold).
- B7 done (facts: input.rs:80-125 order, :89, :110; room.rs:791, :1322, :1024-1032; client lib.rs:442-443, :464, :485-488, :671-688; plan.md:618-624; l1 lib.rs:149-156, :214-218; pump.rs:458-471, :500-516, :510; scenario.rs:1165-1186, :2120, :2175; marks.json -21 checks/bites; SERIES rows; plan :335, :351-352, :360-390, :434-435, :453-476, :971-974; ADR 0001:226-255; carries 80/74/39/27/50/76/77; c-u4r-runs.mjs:131-178).
- New finding NA-13 Medium: room/src/input.rs:110: under option B a command whose stamp equals a buffered command's is `Late`, so a forged equal-tick command with a new sequence is counted as honest late, never kind 2; the no-newer-tick bite under B can pass as late. Proof: unit test f13 (ii). Not carried.
- B8 done. Tally: TE 24: 17 fixed, 6 questions (TE-1, -2 Q9; TE-5 Q11 (v); TE-7 Q15; TE-8 Q11 (iii); TE-11 Q16), 1 needs a run (TE-24 D11), 0 declined. SA 17: 10 fixed, 7 questions (SA-2, -9 Q9; SA-3 Q13; SA-5 Q14; SA-6 Q11 (v); SA-7 Q15; SA-11 Q8), 0 runs, 0 declined. SC-1 fixed (accepted). NA: 5 changed (NA-3, -4, -5, -7; NA-12 wording only), 1 new (NA-13).
- B9 done (W1-W4). B10 done (Grep): CARRY 14; WORDS 15; questions 16; D rows 15; recheck rows 33; section 9 rows 17. Tests measured +100, bites +115, neg 60; conditional +3 Q6 (b), +1 Q9 (a), +1 Q11 (iii) (c), +1 Q13 (a), +5 T-6. Bites 17 (+1 Q15 (b)). Judged runs 65 = 36 + 12 twins + 17 bites (+1 Q15 (b), +13 Q10 (c)). Pre-runs 14 (+4 Q11 (iv), +1 Q15 (c)). Disk MiB: sets 3,311.3; pre-runs 190/250/261; cap 95; total 3,596 to 3,667 vs 3,593 above the 4,096 MiB floor: no margin. Judge -21 about 6,427 MiB < 10,240.
- B11 done: rev 2 final at u4f-brief-draft.md, 1,117 lines, sections 0-12 and findings table present, no marker line left.
- Questions (U4F-Q1..Q16) with recs: Q1 lane (a); Q2 run budget (a); Q3 option-A depth (c); Q4 logs keying (b); Q5 TE2-5 stop clause (a); Q6 ClockSync stamp (b), new option (d) hold-at-tick; Q7 controller (b)+(i); Q8 lead bound (b) at 400 ms (12 ticks), (ii) carry to U4h yes; Q9 order (a) K = 16; Q10 bite options (a); Q11 (i) (a), (ii) P0, (iii) (c), (iv) yes, (v) cheat's own racing rule; Q12 cap (b); Q13 refused command and ack (b) keep today's rule, twin script never makes refused-whole commands; Q14 counters (b) report all, hold clamp uncounted with why; Q15 lag switch (c) report + unjudged 400 ms hold; Q16 carry 80 (b) bound (b) ACK_RING_TICKS.
- Flag: 2.3 takes carry 80 (a)'s total-key route (the carry allows either) with reasons (SA-1, TE-21): a brief requirement the advisor may overrule.
- Limits: no build, test, run or web (night rules); NA-4/SC-1's bias (about ±6.7 ms) and NA-5's disk are arithmetic estimates; Rust's sort behaviour on a non-total order not read (D14); D5 still lacks the BRAWL-1 P3 folder name; client lib.rs:810-839, logs.rs:495, contract.rs:1-25, bots :404 cited from the scout's map, not read; sim world.rs refusals/clamp lines are the security-auditor's cites, not re-read.

DONE
