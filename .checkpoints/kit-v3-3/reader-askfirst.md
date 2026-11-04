Task: A3 (kit-v3-3) reader for Mustafa's item 1 "ask before kit and gate edits in day sessions" + item 4 (the worktree rule) + the `node .checkpoints/*` allow-list entry (plan stage, notes only).

## Plan (blocks, in order)
B1. Read routine-guard.mjs protectedRel(): list the kit and gate paths.
B2. Read guard.mjs (day-session guard), settings.json (hooks, permissions allow/ask/deny, allowed WebFetch domains), settings.test.mjs (what it asserts).
B3. Official docs: hooks (PreToolUse permissionDecision "ask"), permissions (ask rules, modes), settings; when settings/hooks load (start vs file change); quote with URLs.
B4. How "ask" behaves per mode (default, acceptEdits, plan, auto, bypassPermissions); cloud auto mode; Mustafa's PC.
B5. Edge cases: subagents/workflow agents, routines, commit/merge.
B6. Allow-list entry: STATUS kit-v3-2 item 5; allow list; recommendation.
B7. Item 4: every checkout/switch instruction in CLAUDE.md, CHECKPOINTS.md, changeset skill, advisor.md, NIGHT_SHIFT.md; wording, place, guard enforcement, tests.
B8. Write the answer: options, design, tests, questions.

## Notes
(first start: no block was finished; widened 2026-11-04 to item 4, plan rewritten)
