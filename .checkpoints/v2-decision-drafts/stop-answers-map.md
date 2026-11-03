# Stop answers: where each lands (engine-architect, read-only mapping unit)

Inputs: inputs/STOP-ANSWERS-mustafa-2026-11-03.md; plan-r4-stop-answers.md ("Where each answer lands").
Docs mapped at HEAD 4374a1c. Line numbers are at that HEAD. "In plan" = the plan's table already names the place.
One table per answer; each table ends with an END line.

## ROOM-CRASH (OQ-0001-18), the Advisor: "end the match, as proposed, and keep the crash and match records"
Settles OQ-0001-18 as option (a). "Keep the crash and match records" matches what the drafts already write (part 11:
report, match record marked crashed; ADR 0006 part 12: a stub record marked incomplete). It sets no retention period
(see the advisor questions).
| # | Place | Author | Current (short) | Change the answer asks for | In plan |
|---|---|---|---|---|---|
| RC1 | docs/adr/0001-multiplayer-netcode.md:1155-1162 (OQ-0001-18) | netcode-architect | "does the match end (ROOM-CRASH)? ... Decider: the Advisor ... Recommendation: (a)" | Answered by the Advisor (2026-11-03): (a), the match ends; the crash report and the match record are kept. Options stay as the record of what was weighed. | yes |
| RC2 | docs/adr/0001-multiplayer-netcode.md:457-463 (part 11) | netcode-architect | "...marks the match record as crashed and exits the process non-zero ... (ADR 0004 part 20 says the same; whether a match should ever resume instead is OQ-0001-18)" | The closing bracket becomes "a match never resumes: OQ-0001-18, answered (a) by the Advisor, 2026-11-03". | no |
| RC3 | docs/adr/0001-multiplayer-netcode.md:713 (Failure modes, "A panic on the room's tick thread") | netcode-architect | "(part 11; OQ-0001-18)" | "(part 11; OQ-0001-18, answered (a))"; the row's "(proposed: a mark beside PM-0001-26 ...)" now exists as PM-0001-59, so it can name it. | yes ("its failure row") |
| RC4 | docs/adr/0001-multiplayer-netcode.md:1409 (PM-0001-59) | test-engineer | "...the match record is written and marked crashed ...; if OQ-0001-18 picks its option (b), this row is rewritten for the resume" | Drop the conditional tail (b is not picked); the row stands. | no |
| RC5 | docs/adr/0001-multiplayer-netcode.md:1693 (Appendix A, R2-ROOM-REC2) | netcode-architect | "adopted; a panic on the tick thread ends the match and exits the process non-zero (OQ-0001-18)" | "(OQ-0001-18, answered (a))". | no |
| RC6 | docs/adr/0004-simulation-core.md:566 (part 20) | rust-reviewer | "Whether a room may instead resume from a snapshot is the Advisor's question in ADR 0001." | "A room never resumes from a snapshot: the Advisor's answer to ADR 0001's OQ-0001-18 (2026-11-03)." | no |
| RC7 | docs/adr/0006-infrastructure-operations.md:456-458 (part 12) | infra-engineer | "a crashed room leaves a stub record marked incomplete (R6-TEL-REC2)" | No change needed by the words; optional pointer "(kept: OQ-0001-18's answer)". Wording differs from ADR 0001 ("marked as crashed") and PM-0001-59 ("written and marked crashed"): see the advisor questions. | yes ("ADR 0006 where it keeps those records") |
| RC8 | docs/adr/0006-infrastructure-operations.md:419-425 (part 11, retention) | infra-engineer | crash reports at our endpoint 14 days (proposed); match records kept while their content hash is analysed; Sentry's cap open | Only if the Advisor's "keep" means a retention period: none is given, so no change unless the advisor says so. | no |
| RC9 | docs/adr/README.md:61 (decisions table row) | engine-architect | "ROOM-CRASH (OQ-0001-18) ... does the match end (proposed) ... the Advisor" | Mark answered (a) by the Advisor, 2026-11-03 (or move the row out of the open table); open count drops by one (README:16, :23). | yes |
| RC10 | docs/adr/README.md:16, :23 (the drafts table, ADR 0001's "18 (OQ-0001-1 to -18)"; the total "114 numbered, 111 open") | engine-architect | "18 (OQ-0001-1 to -18)"; "111 of them open" | Recount after all answers (see the count note at the end). | partly ("the open count") |
END ROOM-CRASH

