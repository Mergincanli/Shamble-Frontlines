# Advisor check: U7's build brief, rev 1 (2026-11-23)
Brief: u7-brief.md, 358 lines, sha256 c9e1fb33d078438bbe79a3cfd412defb3dbbfcb11b721d2583e80c00a13ebfab (confirmed).
Drafter: netcode-architect (notes netcode-u7-brief.md). Main's advisor.md read (git show origin/main:.claude/agents/advisor.md).

## Plan
1. Read the rules in force word for word: history.md:7561-7593 (NPT-TRIM), :7595-7607 (2026-11-22 answers),
   :2234-2282 (TEST-FIRST); the resume card's Rules, Running work and U7 line.
2. Read U7's rows: replan.md (:18, :45-46, :63-64, :77-79, :89-94, :101-102), replan-units.md §4 (:123-149) and §7,
   replan-marks.md (:31, :48, :49, :56, :75, :96); plan.md where cited (:485-506, :1123-1126, :1214-1223, :1255).
3. Read the sibling briefs' sections the brief leans on and their checks (u6a/u6b/u6c/u8 rev 1.1, advisor-u6a/u6b/u6c/u8),
   security-r1-deps.md (:232-255), security-u6c-deps.md, u6a-fix-pass.md's last section (O2 carry).
4. Grep the spike's code for each fact the brief states in section 0 and section 9 (client lib.rs, clock.rs, bots, lab
   scenario.rs, log lib.rs, client-wasm, Cargo.lock), plus U8's patch lines and U6a's patch lines.
5. Rule Q-U7-1..4, contradictions (a)-(i), size and R1's date, Block 0's security reads, and the rest (carries,
   tests, plants, bans, files, seats, not-verified, marks).
6. Write verdict, paste-ready conditions, answers.

## Notes (one line per finding, as I go)
- N1 Rules read word for word: NPT-TRIM history.md:7561-7593 (item 3 :7572; review seats :7590: "security only for
  code that faces the internet in a run"; one advisor check :7591); answers :7595-7607 (R1-DAY :7604, dates :7605);
  TEST-FIRST :2234-2282 (rule 2 :2261-2262). Card read (Rules, Now, Running work, U7 line :167-176).
- N2 O2 (u6a-review-netcode.md:155-158; u6a-fix-pass.md:79): "one home both the binary and the clients or page reach,
  the `mode_hash` too, with a test that they are equal". Code: `Join` carries version + content_hash only
  (proto/src/msg.rs:74-83), checked by the server's pump (server/src/pump.rs:484-497); `mode_hash` is JoinAccepted's
  (msg.rs:104), set [0;32] by U6a's binary (wip/u6a-build.patch:992), read by no client (grep: no hit in crates/client).
  Q-U7-2's lean names version and content hash only: it must take mode_hash and the equality test too. Medium (a carry
  met short; caught here, so no wire risk).
