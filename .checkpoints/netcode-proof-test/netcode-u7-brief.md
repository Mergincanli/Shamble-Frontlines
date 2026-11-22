# netcode-architect: U7 build brief rev 1 (notes)

Task: draft u7-brief.md rev 1 (U7 "a plain browser client") for the one advisor check NPT-TRIM allows.
Branch claude/netcode-proof-test-plan-hcky5g. Started 2026-11-22.

## Plan
1. Read sources: history.md:7561-7607 (NPT-TRIM, answers), :2234-2282 (TEST-FIRST); replan.md :18 :77-78 :89-94 :101;
   replan-units.md §4 :123-149, §7 :192-222; plan.md :495-497 :643-644 :739 :818 :1214-1220 :1254; u4s-carries.md
   C-U7-1..4; marks.json PM-0001-36, -14; replan-marks.md; advisor-u6a-brief.md Q-U6A-6; advisor-u8-brief.md C2;
   security-r1-deps.md :104-160 :249-253; u6c-brief.md (shape); u6a-brief.md §6; u6b-brief.md; u8-build.md F2 F15;
   wip/u8-build.patch browser scripts; spike code crates/client lib.rs clock.rs, client-wasm, hash-wasm, proto.
2. Browser facts from official sources (W3C WebTransport, MDN, Firefox release notes).
3. Draft u7-brief.md in u6c-brief.md's shape (sections 0-10), lines <=120.
4. Quote check script in u7-brief/ ; run it; record count.
5. Return report.

## Findings (one line each, as I go)
