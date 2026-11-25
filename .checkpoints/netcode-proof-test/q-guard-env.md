# GUARD-ENV: a question for Mustafa (the main session, 2026-11-25 11:3x UTC)

**The question, in plain words:** the guard misses a git or gh command when certain options come in front of it. Who
fixes that, and when?

**What was found (security-auditor, U7 Block 0b's read, 2026-11-25; security-u7-0b-answer.md, SEC-U7B-1; Critical
on the severity scale, because it is a way around the guard):** in main's guard, the step that strips wrappers off a
command (`.claude/hooks/routine-guard.mjs:296-312`) drops an option word but not that option's value for `env`
(`-C DIR`, `-u NAME`, `-S`, `--chdir`), `stdbuf` (`-o MODE`) and `nice` (`--adjustment N`). The value is then read as
the program, so a command such as `env -C <folder> git switch <branch>` gets no git check and no question in a day
session or an agent. Routines are not affected (their shell runs only git, gh, npm ci, node, python and date).

**Checked by the main session:** read only, the code at :302 (env), :309 (nice) and :310 (stdbuf) does what the
finding says. Nothing was run to try it, and nothing will be.

**Until it is fixed:** no session or agent of this chat puts env, stdbuf or nice options in front of git or gh
(brief 3 in launch-0850.md carries the line from 11:3x UTC; every later brief carries it). U7's commands need none of
them (security's Q3 answer: no `env -C`, no `git -C`).

**Worth knowing for the timing:** the gap's exact form is already in this public repository's pushed notes
(security-u7-0b-answer.md, pushed with c843b66; this file, fc0a318), and history is never rewritten. That weighs for
(a) soon over (c).

**Options:**
- (a) A kit session fixes it soon, as a small tier-2 change-set on its own branch (plan first, the advisor and the
  security reviewer, your go in a day session). This chat sends the finding file over and keeps the interim rule.
- (b) This chat fixes it, on a new branch you allow (this session may push only its netcode branch), between R1's
  steps. Lean mode would need your word for it.
- (c) After R1: the interim rule holds until then.

**My recommendation:** (a). The fix is a few lines in one kit file and needs the kit's own reviews; it does not touch
R1's path, and the interim rule keeps this chat safe meanwhile.

**Answered 2026-11-25 13:55 UTC: (a)** (history.md:7964, word for word there): a small separate chat fixes it soon as
its own change-set, with the security reviewer and his go. This chat fixes nothing and keeps the interim rule in every
brief; security-u7-0b-answer.md stays where it is on this branch (the other chat reads it there); pushed notes get no
more detail about the gap than they hold now.
