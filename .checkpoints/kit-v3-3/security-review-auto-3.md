Automated security review of commit bbcb96b (step 4c), 2026-11-05 ~03:00 UTC: two findings, both in the day-session cd
tracking of .claude/hooks/routine-guard.mjs (lineSetsCdpath and dayCd). For the fix round after the step 8 reviews.

7. MEDIUM, a wrapper or quotes hide a variable setter. lineSetsCdpath flags a line whose raw text contains CDPATH or
   `${!`, or whose unwrapped program is a known setter. A name split by quotes (for example CD''PATH) behind `builtin`
   or `command -p` / `command --` (which `unwrap` does not strip) passes both tests. Fix: strip the wrapper options in
   `unwrap` (`command -p/-v/-V/--`, `builtin --`), and treat `command` or `builtin` with anything other than a plain
   program name as a possible setter; or fail closed, marking the line when any word with quotes removed could name
   CDPATH.
8. MEDIUM, `shopt -s cdable_vars`. With cdable_vars on, `cd x` goes to the folder held in the variable x when no
   folder x exists, so a bare relative target can land in the main folder while the guard follows it as ./x. Fix: flag
   any line that runs shopt or mentions cdable_vars; better, in dayCd, count a bare relative target as lost when it
   is not an existing folder under the tracked one (this covers cdable_vars, and any CDPATH or variable form, by
   construction).
Note: each round of fixes to the shell cd tracking has found another bash feature that moves the folder. The fix
round should prefer rules that fail toward the ask by construction (an unknown or non-existent target is lost) over
lists of features.
DONE
