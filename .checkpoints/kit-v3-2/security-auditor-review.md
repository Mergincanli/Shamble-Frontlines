security-auditor review of kit-v3-2 (A2) final diff, PR #37
Plan (blocks):
1. Scratch rule: CHECKPOINTS.md §4 / CLAUDE.md vs checkpoint.mjs secret check, size, binaries, .gitignore
2. Saver-check rule (§5) vs checkpoint.mjs watch behaviour and routine-guard.mjs
3. Reworded rules vs guard and routine rules (NIGHT_SHIFT, agents, skills, CLAUDE.md)
Block 1 (scratch rule) DONE.
- F1 minor: paste block CHECKPOINTS.md:107-118 moves agent scratch/harnesses from gitignored .gate/ to the pushed public
  notes folder but carries no "public repo, no secrets" line; that rule lives only at CHECKPOINTS.md:16 (agents don't see
  it). Saver scan (checkpoint.mjs:27-41,100-112) is pattern-based: misses opaque bearer tokens, cookies, passwords,
  compressed/binary content. No size/binary check (git add -A :151) - deferred by plan rev2:66-67, accepted, not flagged.
  .gitignore *.log and .env* also apply inside .checkpoints (git add -A respects them).
Block 2 (saver check) DONE.
- F2 minor: CHECKPOINTS.md:135-137 remedy "push with save" for every NOT pushed/push refused line. watch withholds code
  on purpose (checkpoint.mjs:10-11,189-192); save pushes all local commits (:2-3,:194) and scans only .checkpoints/
  (:103). push refused cannot be fixed by save and §5:144 says left alone.
- F3 minor: §3 CHECKPOINTS.md:62 and CLAUDE.md:155 tell every session to check/restart the saver; header :6 says the rules
  hold for the PR Advisor; the exception is only at :142. Guard refuses (routine-guard.mjs:458) - wasted call, no harm.
  Night shift restart is allowed by guard (:469-476: tracked, equal to main, protectedInBranch). Read of the task output
  file is allowed in routines (:281, credentialDeny only).
Block 3 (rewordings vs guard/routine rules) DONE: no weakening found. verifier.md/perf-auditor.md harness move to notes:
  routines refuse both .gate/ and untracked notes scripts (routine-guard.mjs:466-471); day sessions lose the settings.json
  allow for node .gate/* so a notes harness asks first (stricter). NIGHT_SHIFT rule 7 only adds pointers.
DONE
