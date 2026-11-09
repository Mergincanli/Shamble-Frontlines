# kit-v3-4 fix pass (builder notes), 2026-11-09

Carries out `advisor-fix-pass.md` section 1 exactly. Worktree head c3cf647. No commit, no push (the session does).

## Plan
1. Kit texts as scratch files (agents may not edit kit files): `scratchpad/kb2/advisor-new.txt` (from the worktree's
   `.claude/agents/advisor.md`) and `scratchpad/kb2/checkpoints-new.txt` (from `.claude/CHECKPOINTS.md`), plus
   `advisor.diff` and `checkpoints.diff`. advisor.md: SA-2, IE-1, IE-2 (IE-2-SCOPE), IE-3, IE-4, IE-5, SA-3, SA-4,
   IE-6, IE-7, SA-6, SA-7. CHECKPOINTS.md §10: IE-8, IE-9, IE-10 (his quote untouched).
2. Docs: L1 (STATUS:528, :539; vfx report :73, :78, :83, :88); STATUS follow-ups F1-F4 in a new "From kit-v3-4" block.
3. Notes: kit-draft.md §8 rows (M1: §10; L2: tier-1 Check; one row per sentence this pass adds or changes).
4. Checks: diffs, line lengths, his words byte for byte, repo-map check.

## Progress
- Step 1 done: `scratchpad/kb2/advisor-new.txt` (301 -> 324 lines, +23; 16 replacements, each anchor asserted once)
  and `checkpoints-new.txt` (220 -> 229, +9; 3 replacements in §10); `advisor.diff`, `checkpoints.diff` made with
  `diff -u` against the worktree's files; `git apply --check` OK for both. Builder scripts: `scratchpad/kb2-tools/`.
  Placement: IE-1, IE-2, SA-3 as sub-bullets of "Your check of a revert PR"; IE-4 a new "Other PRs" bullet after
  "Merge"; IE-5 in "Who"; IE-6 in "Main breaks"; SA-6 after "Count the rounds". IE-8 keeps the addendum's "stays
  until its work is saved" as "A copy with work to keep stays until that work is saved".
- Step 2 done: L1 (STATUS:528 :184, :539 :104; vfx report :73, :78, :83 "at 69ea88e" with the paragraphs re-wrapped,
  :88 :184); STATUS's new "From kit-v3-4" block (F1-F4) after the vfx-xp-notes block (+21 lines, all after :560, so
  F3's line list holds).
- Step 3 done: kit-draft.md §8 rows 56-63 (M1, L2) and 64-82 (the fix pass), and a note under the table.
- Step 4 done: no added line over 120 characters (scratch files, docs, notes; tables aside); advisor.md's four A4
  quotes and §10's quote unchanged (diffed) and found in his source text; §1-§9 of CHECKPOINTS.md unchanged; repo
  map OK (114 entries, 101 paths).
- Observations: `spikes/netcode-proof/` (IE-9, IE-10's paths) is on `origin/claude/netcode-proof-test-plan-hcky5g`,
  not on main yet (its `.cargo/config.toml` read there: target-dir `../../.gate/netcode-proof-test/target`). SA-3's
  trigger test names a red run or Mustafa's own words; the third "main breaks" case (a kit change stops sessions)
  passes it only through his report.
