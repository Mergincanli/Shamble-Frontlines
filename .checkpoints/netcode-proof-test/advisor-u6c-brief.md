# The advisor's check of U6c's brief, rev 1 (2026-11-22)
Brief: u6c-brief.md rev 1, 505 lines, sha256 13b0fe58777585525188adbd9d61cde4bc3a1d70df2306ae78cc88c6ba26ed5b (checked
equal at the start of this check). Drafter: netcode-architect (netcode-u6c-brief.md). Working by main's
`.claude/agents/advisor.md` (read with `git show origin/main:.claude/agents/advisor.md`). Rules read word for word:
NPT-TRIM history.md:7561-7593; his answers of 2026-11-22 history.md:7595-7607; TEST-FIRST history.md:2234-2282.

## Plan
1. Read the rules, the card, the brief and the drafter's notes. DONE (brief hash checked).
2. Read U8's brief rev 1.1 (C10, C1-C4, Q-U8-4/-5, :55-71, :200-240, :383, :416-423, :466, :505-521) and its check.
3. Read U6a's and U6b's briefs rev 1.1 and their checks (conditions the brief cites; Block 0 shape u6b-brief.md:393-400).
4. Read security-r1-deps.md (instant-acme, the 17, SEC-R1D-4, :249-253) and security-u6b-deps.md.
5. Grep the spike's code for each fact the brief leans on (stubs, socket_tests.rs lines, stats.rs, thread.rs, SERIES.md
   rows, check.mjs, clippy.toml, ws_config.rs, config.rs, lab probe, lock entries).
6. Rule the six contradictions, Q-U6C-1 to -8, size and R1's date, Block 0, and the rest; note each finding here.
7. Verdict, paste-ready conditions, answers. Hand back.

## Notes (one line per finding, as found)
- N0 plan.md gained one line at :379 on 2026-11-22 08:50 (commit 4d71571, C-U4F2-8's text), after replan-units.md
  (0d9ccac, 2026-11-21 23:17) was written: every plan.md cite past :379 in replan-units.md and replan.md is one low
  today (U6's row is :1254, U8's :1256; `/npt/cert-hash.json` is :995-996). This explains contradiction (f)'s row
  number; u6b-brief.md's "plan.md:994-995" and this brief's copies of it carry the same off-by-one.
