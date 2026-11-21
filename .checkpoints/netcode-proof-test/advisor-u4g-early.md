# Advisor ruling U4G-EARLY (2026-11-21): may U4g's blocks 1-4 build now, while U4f2 waits on F-B-COUNT?

Stand-in: a general-purpose agent following `git show origin/main:.claude/agents/advisor.md` (read at start).
Seat: day session, plan ruling inside the netcode proof test (PR #40, branch claude/netcode-proof-test-plan-hcky5g).

## Plan (blocks I will write, each saved as soon as it is done)
- B1. Facts checked myself (each fact in the request, with file:line or command output; any that does not hold).
- B2. The overlap: U4f2's patch hunks in crates/room and crates/client, set against what U4g blocks 1-4 must touch
  there (u4g-brief.md blocks 1-4), so the hunk list (iii) rests on evidence.
- B3. (i) Worth the risks? conflicts at the re-base; the base without U4f2's or round 2's room and client edits;
  agent and quiet-machine limits; disk.
- B4. (ii) Whose choice: TEST-FIRST rule 2 or Mustafa's.
- B5. (iii) The hunk list, or the rule that replaces it.
- B6. (iv) Rules the early half needs that u4g-brief.md rev 2.1 lacks (-21's evidence kept clean, and others).
- B7. (v) The one-line wording for his message.
- B8. Verdict and numbered conditions.

## Notes (one line per finding as I go)
