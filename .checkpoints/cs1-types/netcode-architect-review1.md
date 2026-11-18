# netcode-architect, round 1 review of cs1-types (code at 82142fa, base main 22c6afa)

Read-only review. No source edits. Never `spikes/`, never /root/.claude/uploads/.

## Plan
1. Read crates/protocol/src/messages.rs (+ lib.rs, Cargo.toml) and 2-3 generated TS files.
2. Read plan-draft.md section 4 + O1, plan.md "After the advisor agent's review" item 6, rust-reviewer-review1.md "For later steps".
3. Read ADR 0001 part 5 (input, time, aim point), part 6 (snapshots), message table (~line 680), OQ-0001-6/7,
   PM-0001-17, PM-0001-58; parts 4 and 7 for the layout-hash question. ADR 0004 tick type; ADR 0005 content ids/hashes;
   R2 R2-XU-OQ4.
4. Check each type field by field (missing / extra / misleading); provisional list; pre-emption; decoder caps; layout q.
5. Verdict + ranked findings with file:line and the draft's line.

## Findings (one line each, as found)
