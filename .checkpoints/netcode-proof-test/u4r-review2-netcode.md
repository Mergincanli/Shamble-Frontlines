# U4r review, round 2: netcode-architect seat (batch 1)

Snapshot under review: wip/u4r.patch sha256 21ca46be23704131c77cc20981a4bfe866811b33f9ca853279f7ddbfb5d58bce
(checked by sha256sum at start). Packet u4r-r2-packet.md sha256 b1526bde... and u4r-r2-packet-tests.md sha256
637249a5... (both checked). Runner u4r-run.sh sha256 6efb9157... (checked).
Read copies: .gate/u4r-snap2 (2fbec77 + new patch), .gate/u4r-snap1 (round 1, 572c4f69).
(This file is rewritten whole with Write each time.)

## Plan
1. Ruling "Round 2" whole (done): my list = FP-1's session ending, FP-4's verdict causes, FP-5 (g), NC-U4R-2's refusal.
   Plus confirm NC-U4R-1..7 fixed/deferred as ruled (1,2,3,6,7 fix now; 4,5 follow-up).
2. Read packet (diff 572c4f69 -> new), tests file.
3. FP-1: scenario.rs Server::io closed branch, module text, new test + plant log.
4. FP-4: precheck.rs verdict + doc, new unit test + plant log.
5. FP-5 (g): c-u4r-runs.mjs one() overflow resync flag, late-command whole-line check; helper test output.
6. NC-U4R-2: scenario::parse refusal + parse case.
7. NC-U4R-7: bots lib.rs docs. NC-U4R-4/-5: report's follow-up list.
8. Report's claims vs c6-*.log for my items.
9. Verdict.

## Progress
- [done] 1.

## Findings (round 2)
(none yet)
