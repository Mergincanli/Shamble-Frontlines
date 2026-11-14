# U4r review round 2: security-auditor seat (notes)

Snapshot under review: wip/u4r.patch sha256 21ca46be23704131c77cc20981a4bfe866811b33f9ca853279f7ddbfb5d58bce (checked, matches).
Packet u4r-r2-packet.md b1526bde... (checked); tests file 637249a5... (checked); runner u4r-run.sh 6efb9157... (checked).
Read copies: .gate/u4r-snap2 (2fbec77 + patch 21ca46be: cite here), .gate/u4r-snap1 (572c4f69, compare).

## Plan
1. Ruling "Round 2", my line: FP-1 (the boundary, both halves), FP-3 (manifest and argv, the bounds), FP-5 (h), SEC-U4R-5/-6 in check.mjs.
2. Confirm SEC-U4R-1..7 fixed / deferred as ruled, each with file:line in snap2.
3. Review changed lines only (diff snap1 -> snap2 in my areas).
4. Check the builder report's claims for my areas against code and the c6-*.log logs (plant logs for FP-1, FP-3; check.mjs self-test for SEC-U4R-5/-6).
5. Run only if a finding needs it (runner, log under .gate/u4r/review2-security/).
6. Result: verdict, sha, round1 states, new findings.

## Progress
