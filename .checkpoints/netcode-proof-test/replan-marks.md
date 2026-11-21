# Re-plan mark table under NPT-TRIM (test-engineer, 2026-11-21)

## Plan (block 0)
Source of the rule: Mustafa's NPT-TRIM, history.md:7561-7593 (read whole). Steps, each saved as a block:
1. List the 81 marks (id, title, unit/run fields) from spikes/netcode-proof/marks/marks.json by node one-liner.
2. Read plan.md §6.2 (:713-758), §7.2 (:815-866), §12.1 (:1244-1255) for unit/run placement.
3. Read u4-build-plan.md rows, u4g-brief §1.3/§2 (:66-125), u4h-brief, u5-brief, u6-brief/u6-facts headings.
4. Grep Slice 1 build plan headings (change-sets 7, 8), SPIKE_PLAN slices 3/6, ADR 0001 mark table (:1580-1700).
5. Grep the card (.checkpoints/netcode-proof-test.md) for judged results per mark.
6. Write blocks: 1 table, 2 counts, 3 U4 kept rows, 4 lag-comp server cost, 5 R1/R3/R5 rows, 6 MOVED status.
Status: steps 1, 2, 5 done (notes below); next: step 3.

## Notes (working; folded into the blocks)
- marks.json: 81 marks; fields id,title,draft,verdict,where,parts,validity,checks,report,bites,stand_in,arm_rule,
  readings,notes (node one-liner). verdict kinds: pass 72, outside 3 (-39,-40,-63), person 3 (-53, PM-0006-12, -18),
  reported 1 (PM-0006-14). No unit/run-status field today.
- plan §6.2 :720-757 placement; §7.2 :818-826 runs; §12.1 :1251 U4 row groups; :1252 U5; :1253 U6; :1254 U7; :1255 U8.
- u4-build-plan.md: U4r :781-834 (-17,-57,-18 L1; -30 precheck, not a judged id, -30 NOT-RUN until U6 :827); U4f
  :836-853 (-47,-20,-22); U4f2 :855-879 (-21); U4g :881-901 (-41,-42,-60,-45,-61,-62,-55,-58; moved in: -17/-18 B1,
  carry 78 recovery, carry 91 ack.rs, carry 92 tick wrap); U4h :903-918 (-48,-49 test-cap,-50,-51,-54).
- JUDGED so far:
  - -3 L1 P1-P4,B1 PASS (results/c-u2-pm-0001-3/81c6d01e371b/verdict.json); L2 and MP parts NOT-RUN.
  - -57 L1 P0,P2,P3,B1 PASS, counted (u4r-report.md:543).
  - -17 L1 P0,P2,P3 PASS at BRAWL-1 re-run sets (advisor-brawl1-sets-p0-full.md:226,233; history.md:3266 P3);
    -17/-18 L1 B1 INVALID as ruled, U4g's (u4r-report.md:545).
  - -2 SS part FAIL at U4r (u4r-report.md:547), PASS on BRAWL-1 sets (advisor-brawl1-sets-p0.md:25) and U4f sets
    (u4f-report.md:599,654); -2 ES FAIL by reliable events 50.05/s vs 10.5, ruled U4h's (u4r-report.md:308).
  - -20 L1 A and B PASS 3/3 on F1's merge, 46ca143 (u4f-report.md:654-655); headline NOT-RUN (L2 parts).
  - -22 PASS (u4f-report.md:685-689). -47 PASS (u4f-report.md:564-573). -21 not run (U4f2).
