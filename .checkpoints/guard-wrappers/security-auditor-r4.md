# security-auditor round 4: guard-wrappers, diff 9ad6bc9..83bc2417 (.claude/hooks), what changed only

Plan (blocks): E1 read the diff and the code around it (NON_RUNNERS, unwrap callers, rule U); E2 probe the new
unwrap check (R3-1/R3-2 closure, span/reps logic, throws); E3 probe the ADV-R4-1 block (behind a program not stripped);
E4 differential 9ad6bc9 -> head (throws, deny/ask -> pass), session and agent; E5 rows and mutants, suite rerun;
E6 answer.

Notes (public: ids, severities and class words only; forms in the scratchpad detail file):
