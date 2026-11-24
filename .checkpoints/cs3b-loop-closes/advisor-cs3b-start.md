# Advisor: read before 3b's first edit (2026-11-24)

Asked for by Mustafa's message of 2026-11-24, 22:44:50 UTC, point 4 (file:
`.checkpoints/cs3b-loop-closes/mustafa-2026-11-24T224450-merged-65-next-3b.md`). Worktree `.gate/ci-wasm-wt`, branch
`cs/3b-loop-closes` = main 28254a73 + notes commit 51daf6c9 (only `.checkpoints/` files; checked with git diff).
Read-only; this file is my only write. Working from main's `.claude/agents/advisor.md` (last changed 97e2e3bb, 2026-11-13).

## Plan
1. Read his message whole; the resume card; the plan (draft 3) sections 5, 8, 9, 12, 13; the two designs' blocks; my own
   review of 2026-11-19.
2. Item (a): plan's decoder-test numbers vs ADR 0001 row PM-0001-34 (line ~1626); 3b's message types; cost in lines and
   run time (debug `cargo test --workspace --locked`, run twice in CI per toolchain.yml; release); counting allocator
   (`crates/sim/tests/state_hash_alloc.rs`); seeded inputs vs the rng rules (`crates/sim/src/rng.rs`).
3. Item (b): the build plan's two places that say the proof test chooses command timing (quote exactly); OQ-0001-6 at
   ADR lines ~252 and ~1156; does any 3b step depend on the choice.
4. Item (c): link simulator in the plan, its parts and ladder place; does the ladder need a "tell him before" line.
5. Section 5 vs main 28254a73: 3a's changes (scan refuses mul_add, per-processor code, outside-src source; test-only
   module built to `.gate/rules-module/`; feature compare; dependency hold); plan lines assuming a CI edit by this chat;
   3A-FROM-F64, 3A-HOOK-TEST, 3A-MANIFEST-COMMENT.
6. Size: 3,800 / 7,600 / ladder from 4,900 vs the three items.
7. Compaction: card enough to rebuild from?
8. "Decisions of 21 and 22 November": search main for any record.

## Findings (as I go)
