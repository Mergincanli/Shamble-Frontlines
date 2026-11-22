# netcode-architect: U7 build brief rev 1 (notes)

Task: draft u7-brief.md rev 1 (U7 "a plain browser client") for the one advisor check NPT-TRIM allows.
Branch claude/netcode-proof-test-plan-hcky5g. Started 2026-11-22. No Edit tool: this file is rewritten whole by Write.

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

## Progress
- Block 1 (reads): history, replan, replan-units, plan.md parts DONE.

## Findings (one line each, as I go)
- F1 NPT-TRIM item 3 (history.md:7572): "U7, cut to a plain browser client: ..." R1-DAY Thu 3 Dec (history.md:7604).
  TEST-FIRST rule 3 brief shape (history.md:2267).
- F2 replan.md:18 (U7 row), :77-79 (Wasm build of client endpoint, C-U7-1/-2 go), :89-94 order, :101-102 reading (3).
- F3 replan-units.md §4 :123-149; §7 :192-222: Sun 29 U7 Rust half + round + headless Chromium both transports;
  risks: Chromium install, Firefox hashes untested, C-U7-3 cap source.
- F4 plan.md line drift: current plan.md has every cited line +1 vs replan-units (U7 row now :1255, not :1254; -36 row
  :740 not :739; R1 row :819 not :818; E7 web :495-504, net.ts :496-497, host.ts :498). Smoke check :1214-1223.
  "JoinAccepted field (plan.md:1254)" in replan-units = the U7 row's C-U7-3 text, now :1255.
- F5 plan.md:643-644: "TypeScript 6.0.x, Vite, WebdriverIO and three.js, each exact ... `npm ci --ignore-scripts`".
  U7 needs neither Vite nor three.js (security B7: vite not needed; three.js goes with render.ts).
