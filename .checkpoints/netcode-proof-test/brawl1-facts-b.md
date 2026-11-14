# brawl1-facts-b: the other npt-lab lane items (facts for BRAWL-1's plan)

Agent: facts-b (read-only). Branch claude/netcode-proof-test-plan-hcky5g, main folder.

## Plan
1. Read the rulings: advisor-u4r-r2-full.md (N2-1, TE-R2-1, TE-R2-2, RR4-L1, RR4-L2, item 18),
   advisor-u4r-r1-full.md (EA-U4R-1, EA-U4R-2, RR3-L2, FP-7 items), u4r-review-r1-full.md, u4r-review-r2-full.md,
   advisor-u4r-sets-r2-full.md (BRAWL-1 Fix a-d) for the interaction.
2. Per item: B1 quote + source line; B2 code at HEAD (file:line, moved?); B3 exact change + open points; B4 test;
   B5 interaction with BRAWL-1 / recordings; B6 size.
3. Grep notes for "next npt-lab lane" for other items.
4. Summary per item at the end.

## Findings (a line per finding, file:line)
- F0: crates/ unchanged since the merge 71ee13d (`git log -- spikes/netcode-proof/crates` tops at 71ee13d); round 2
  lines (snap2 = merged bytes) equal HEAD; round 1 lines (snap1 572c4f69) moved by the fix pass.
- F1 N2-1: closed_checks scenario.rs:2148-2164 (doc :2148-2149 "FP-1's three checks"); fp1 test :2166-2213 (doc
  :2166-2169 says "and not as a replacement"); the 'no replacement' assert :2209-2212; drop_slot :408-422 ('replaced'
  push only on a live old session :411-419); closed branch :491-500. Room::roster room.rs:568, Room::leave room.rs:690-695
  (roster.leave :691), Roster::is_live roster.rs:80. All pub.
- F2 N2-1 plant fact: under the leave plant only ONE of today's three checks fails (owner.remove and ended.push still
  run): c6-plant-fp1.log:20-23 (.gate/u4r/), u4r-build.md:1649-1654, :2089-2090. With the roster check, two of four would
  fail (client + roster), not "all four" as the ruling's text says: an open point.
- F3 parse: sum refusal scenario.rs:1376-1378, after-max :1379-1381, last-burst check :1442-1454 (comment :1442),
  burst_starts :1295-1310 (plain u64 sums :1306-1307), due_ns :130-132 (`as u64`), :862 `as u32`. fp3 test :2420-2431
  (only --measured-s). All lines as the round-2 rulings name them.
