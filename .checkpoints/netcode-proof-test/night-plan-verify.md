# Verifier (references): night-2026-11-16.md, item 7, pinned 11fd190

Plan: check every reference in N/night-2026-11-16.md at 11fd190 (file:line, sections, carries, rulings, Qs), the SHA on origin, every file to read exists, then spot-check 15 scout-map entries; a line per finding. (No Edit tool here: this file is rewritten whole with Write.)

## Findings
- Setup: HEAD moved during the check (5afc772, ab5cfc1 commit the plan and the card); vs 11fd190 only the card, the plan, advisor-night-plan.md and these notes differ, so every other file is read from the working tree; the card from `git show 11fd190:`.
- SHA: 11fd190e95... is a commit, on origin/claude/netcode-proof-test-plan-hcky5g (git branch -r --contains). OK.
- Header: card at 11fd190 :3362-3366 "Mustafa, 2026-11-09 (verbatim, the adjusted version)": words equal the plan's quote (card :25 names it too). night_1116_rule_words.py exists at 11fd190. OK.
- NIGHT_SHIFT.md step 4: "a step it does not cover ... stops the item under Decisions needed" (:30-33). OK.
- CHECKPOINTS.md §8 :190 "without moving the main folder" via a worktree; §4 :65-85 plan first, save as you go, absolute path :85. OK.
- CLAUDE.md "Models and effort": xhigh for netcode (:174-177). Agent types netcode-architect, test-engineer, security-auditor exist in .claude/agents/. OK.
- u4-build-plan.md:882-883 "Then U4r ... then U4f, U4g and U4h in order". OK.
- u4-build-plan.md:1054-1056 Q21 U4-BRIEFS: U4f gets a short brief (field tables, budgets, run lists with counts) checked before build. OK.
- u4-build-plan.md:805-816 "### U4f, clock and commands" (Builds :806-809, Marks moved :810-815, Reviewers :816). OK.
