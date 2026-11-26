# security-auditor round 4: guard-wrappers, diff 9ad6bc9..83bc2417 (.claude/hooks), what changed only

Plan (blocks): E1 read the diff and the code around it (NON_RUNNERS, unwrap callers, rule U); E2 probe the new
unwrap check (R3-1/R3-2 closure, span/reps logic, throws); E3 probe the ADV-R4-1 block (behind a program not stripped);
E4 differential 9ad6bc9 -> head (throws, deny/ask -> pass), session and agent; E5 rows and mutants, suite rerun;
E6 answer.

Notes (public: ids, severities and class words only; forms in the scratchpad detail file):
- E1 done: diff read. Copies of base and head under .gate/guard-wrappers/sa-r4-*-guard.mjs; head copy hash equals the
  branch file.
- E2a done: every round-3 case file (4 files, 75 lines) rerun on main, 9ad6bc9 and head: head asks in a session and
  refuses an agent on every line; R3-1 and R3-2 closed for those spellings.
- E2b/E3 done: 109 more probe lines (3 files) on main, 9ad6bc9 and head. Nothing looser, no throw.
- R4-1 Critical, open: ADV-R4-1 not closed for every spelling (the xargs half, this diff's own class), plus a rule U
  sibling for git/gh (older, present on 9ad6bc9). Silent for a session and an agent. Not covered by L1's sudo line
  (also shows with another front program). Detail only in the scratchpad.
- Older, stated limit L16 (not this diff): disguised-shell-text spellings stay silent on main, 9ad6bc9 and head.
- E4 done: differential 9ad6bc9 -> head, 860,160 lines (session and agent): 0 throws, 0 looser, 147,077 stricter on
  each side. The 172,523 agent-silent lines on head were tallied by class (the read-only bucket uses head's own
  reader, so it is a tally, not independent proof); no unexplained class.
- E5 done: suite 1551/1551 rerun on the head-equal copy (hash checked); the 8 new mutants are in-suite.
- R4-2 Low, new in this diff: a cost class in the new check; fails closed (hook timeout blocks with onFailure, which
  needs Claude Code v2.1.295 or later).
- Source (high confidence): R4-1's mechanism is documented in GNU coreutils' manual, read from the local info copy
  (gnu.org returned 503); the citation stays in the scratchpad detail until the fix is merged.
- Read-only bucket hand-check: its 1,783 distinct shapes reduce to 13 middles and 34 option sets; each middle is a
  stripped wrapper or the value of an xargs option that takes the next word (findutils' option string); none runs
  input words.
- Memory updated (R3-WAY (a), timeout fails closed, L16 classification).
- Verdict: REVISE (R4-1 Critical open, the diff's own class); round 4 is the last, so it goes to Mustafa. The diff
  itself: nothing looser than 9ad6bc9, no throw, every round-3 spelling closed.
DONE
