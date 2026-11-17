Task: advisor-part-15-to-main-plan (advisor review of the plan to bring Part 15 + Mustafa's six answers to main)

## Plan (blocks, in order)
B1. Read the plan (.checkpoints/part-15-to-main/plan.md).
B2. Read his two messages (merge-56 go; six answers) and the quotes-check script.
B3. Read Part 15's text (revision 2) and compare with ea851a8's copy.
B4. Read main's docs: DECISIONS (2026-11-14 13:58:05; 2026-11-17 19:10:44, 20:03:57, 20:39:23), build-plan report "Since the plan" s2, cs0 report "Proposed next step".
B5. Read asset-export-rules.md AX-R1 (~l.155), ADR 0002 l.190-217, STATUS.md (and ROADMAP pointer lines).
B6. Agent file rules for decision docs / his words (already in context: advisor.md risk tiers).
B7. Checks 1-5, findings.
B8. Answer.

## Notes
- B1 done: plan read (62 lines; rows 1-7; three questions; evidence; size 150, stop 300).
- B2 done: his two messages read. Merge-56 go (22:59:16Z): "Keep it to Part 15's text, my answers by script, the pointers they need and the rules file's line on gun files. It also brings the status page and the roadmap up to date for this merge." Answers (22:10:34Z): six answers + STOP LINE + PR #56 parts.
- F-a (Low, evidence tool): quotes-check.mjs hard-codes sources under .checkpoints/build-plan-slice-1/ and .checkpoints/cs0-model-rules/, which are gone from the tree (deleted in cd61b28); run as is, it throws ENOENT. Fix: point it at the two new message files (--source, or edit N/C), and say so in the report.
- Next: B3 (Part 15 text vs ea851a8).
