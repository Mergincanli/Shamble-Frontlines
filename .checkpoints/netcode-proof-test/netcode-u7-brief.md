# netcode-architect: U7 build brief rev 1 (notes)

Task: draft u7-brief.md rev 1 (U7 "a plain browser client") for the one advisor check NPT-TRIM allows.
Branch claude/netcode-proof-test-plan-hcky5g. Started 2026-11-22. No Edit tool: this file is rewritten whole by Write.

## Plan
1. Read sources. 2. Browser facts (W3C, MDN, BCD). 3. Draft u7-brief.md in u6c-brief.md's shape (0-10), lines <=120.
4. Quote check script in u7-brief/; run; count. 5. Return report.

## Progress
- Block 1 (reads) DONE. Block 2 (browser facts) DONE (F25-F29).
- Coordinator message (~20:3x UTC): advisor-u6c-brief.md rulings bind U7 (C9, Q-U6C-3 (a) no alias, C1 (e), section 5
  items 3 and 5, C1 (c)): F30-F33, taken in the brief as fixed.
- Block 3 DONE: u7-brief.md rev 1, 358 lines, sha256 c9e1fb33d078438bbe79a3cfd412defb3dbbfcb11b721d2583e80c00a13ebfab.
- Block 4 DONE: u7-brief/quote_check.py: 29 local quotes OK (0 misses), 0 lines over 120. Four web quotes (W3C
  "[Exposed=(Window,Worker), SecureContext]" and "the total length of the validity period MUST NOT exceed two weeks";
  MDN "This feature is available in Web Workers"; Playwright "at your own risk") checked by eye against the fetches.
- Block 5: report (handback). DONE when sent.

## Findings (one line each)
- F1 NPT-TRIM history.md:7572; R1-DAY Thu 3 Dec :7604; TEST-FIRST rule 3 :2267.
- F2 replan.md:18, :77-79, :89-94, :101-102 (reading 3).
- F3 replan-units.md §4 :123-149; §7 :192-222.
- F4 plan.md +1 drift past :379 (advisor-u6c-brief.md:18-21); brief cites live lines.
- F5 plan.md:643-644 npm pins; U7 needs neither Vite nor three.js.
- F6 C-U7-1..4 NOT in u4s-carries.md; home plan.md:1255. u4s-carries.md:831 ws:// pages -> cut (replan-units.md:19).
- F7 u4s-carries.md:870-871 cert-hash "is U6d's" -> U6c.
- F8 u6c-brief.md:126-128 allow-list takes U7's dist names; CSP plan.md:1123-1126.
- F9 CSP shapes the page (no inline script/style; wasm-unsafe-eval; worker-src self; connect-src).
- F10 C-U6-19 gate line advisor-u6a-brief.md:233-237; U7's line written.
- F11 U6b cond 4 hash JSON; ports; WS tag values homeless -> Q-U7-2.
- F12 token `nptj_` + base64url 16 B (u6a patch :753-790).
- F13 PROTOCOL_VERSION/CONTENT_HASH private in U6a main.rs -> Q-U7-2.
- F14 bind per catalogue :112-115.
- F15 endpoint API facts; C-U7-3/-4 sites.
- F16 client-wasm stub; LEAF lacks it.
- F17 U8 save() writes no header -> Q-U7-1. U8 file refs are PATCH lines (config.mjs file line = patch - 52; lib.mjs
  file line = patch - 261): brief cites wip/u8-build.patch:111-132, :374-404, :398-404.
- F18 rs.yaml dist build taken as proposed (F14).
- F19 U8's `/npt/cert-hash.json` text stale -> fix pass item 3.
- F20 marks: -36, -29, PM-0006-25 (as PM-0006-15), -14 browser side; bites read-once, no-wt-timeout.
- F21 CONTRADICTION replan-units.md:138 vs replan.md:63-64 / replan-marks.md:48 on -28 -> Q-U7-4.
- F22 security-r1-deps.md:245 web/.npmrc vs advisor-u8-brief.md C2 -> C2 rules.
- F23 Chromium 1243 here; tools/ playwright-core 1.56.1 wants 1194; executablePath "at your own risk".
- F24 wss in container via the test context's ignoreHTTPSErrors only.
- F25-F29 browser facts (MDN, W3C, BCD, Playwright), listed in the brief's section 10.
- F30-F33 advisor-u6c-brief.md rulings (C9, C1 (e), Q-U6C-3, growth 1.9-2.3x, lane, F26).
- F34 earlier Rust half: no (moves U6c's merge and Stop B; agent seats full in rounds); T5 (i) early contact instead.
- F35 R3's in-process room test needs client-wasm dev-deps npt-room, npt-sim -> Block 0b lock diff 3 lines (no
  package).
- F36 Reproducible Wasm needed for R1.md's expected dist SHA-256s (registry paths in panic locations): RUSTFLAGS
  remap CARGO_HOME in build.mjs, own target dir, a path-scan test. Not verified on RS (Stop B compares).
- F37 -56 item 2's "negotiated draft" has no writer in any brief; page cannot read it. Flagged for the main session.
- F38 BCD: requireUnreliable unsupported in Chrome (harmless); datagrams.writable deprecated, createWritable Chrome
  159 / Firefox 155 -> feature check.
