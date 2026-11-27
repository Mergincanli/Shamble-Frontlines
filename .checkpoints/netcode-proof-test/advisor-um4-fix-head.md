# Advisor: UM-4 fix pass, head read (fix f1b46682, pushed in 1d4e81a6), 2026-11-27, effort xhigh

Plan: main's advisor.md unchanged since c9bd268 (re-checked against b8f02b01); my ruling advisor-um4-r1.md sections 2-4
and 6. Then the commit: `git show --stat`, `git diff --numstat` and `git diff a7c4b25b f1b46682 -- <file>` per file;
um4-plan.md's "FIX PASS DONE LINES". Then each log under .gate/um4-fix/ printed, with fixed.sha256 against
`git rev-parse f1b46682:<path>` and `git hash-object` of each fixed copy, and counted.log's diff hash against the diff
I print myself. Then the hazard re-read: the victim test against sec-um4-1-probe.md; my own whole-spike search for git
run by name (spikes/netcode-proof, node_modules aside), beside the main session's list; m-wild.log's diff; this
repository's state (`git branch --list side`, `git symbolic-ref HEAD`). Then 3b's seven departures, one ruling each;
the commit message; CI per run with `gh run view`; PR #40's state. Last: verdict, findings on the scale, UM-4's
close, SEC-UM4-1 on PR #40's closing list, Decisions needed, For Mustafa, status line. One simple command per Bash
call, git and gh first; no test, cargo, gate, mutant or plant run; no git variable exported.

## Notes (one line per check)
