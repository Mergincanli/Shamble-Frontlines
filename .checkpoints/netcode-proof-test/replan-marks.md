# Re-plan mark table under NPT-TRIM (test-engineer, 2026-11-21)

## Plan (block 0)
Source of the rule: Mustafa's NPT-TRIM, history.md:7561-7593 (read whole). Steps, each saved as a block:
1. List the 81 marks (id, title, unit/run fields) from spikes/netcode-proof/marks/marks.json by node one-liner.
2. Read plan.md §6.2 (:713-758), §7.2 (:815-866), §12.1 (:1244-1255) for unit/run placement.
3. Read u4-build-plan.md rows, u4g-brief §1.3/§2 (:66-125), u4h-brief, u5-brief, u6-brief/u6-facts headings.
4. Grep Slice 1 build plan headings (change-sets 7, 8), SPIKE_PLAN slices 3/6, ADR 0001 mark table (:1580-1700).
5. Grep the card (.checkpoints/netcode-proof-test.md) for judged results per mark.
6. Write blocks: 1 table, 2 counts, 3 U4 kept rows, 4 lag-comp server cost, 5 R1/R3/R5 rows, 6 MOVED status.
Status: steps 1-2 done (notes below); next: step 3.

## Notes (working; folded into the blocks)
- marks.json: 81 marks; fields id,title,draft,verdict,where,parts,validity,checks,report,bites,stand_in,arm_rule,
  readings,notes (node one-liner). verdict kinds: pass 72, outside 3 (-39,-40,-63), person 3 (-53, PM-0006-12, -18),
  reported 1 (PM-0006-14). No unit/run-status field today.
- plan §6.2 :720-757 placement; §7.2 :818-826 runs; §12.1 :1251 U4 row groups; :1252 U5; :1253 U6; :1254 U7; :1255 U8.
- Card :73: -20's L1 PASSES on F1's fix (46ca143). Card has no per-mark judged ledger; set reports to check.
