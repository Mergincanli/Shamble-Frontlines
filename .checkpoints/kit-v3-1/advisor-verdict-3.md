# Advisor verdict 3, kit-v3-1 (2026-10-27): routine Bash as an allow list. APPROVE, inside the approved plan
General agent reading main's advisor.md. Scope question from build step 1 (3f43a7a). No plan revision round needed.

**Why it is inside the plan.** The plan already decided this: R2 and ROUTINE-CODE-EXEC say routines run no program
that didn't come from main in phase 0, and WRITE-GUARD says everything they must never do is denied. The deny list
doesn't deliver either. I ran decide() on 3f43a7a as night-shift, and all of these went through: `./.gate/x.sh`,
`/tmp/x`, `python3.12 -c 1`, `NODE_OPTIONS=--require=./.gate/x.js node tools/gate.mjs`, `PATH=.gate git status`,
`env -S "node -e 1"`, `make -f …`, `echo CLAUDE.md | xargs -0 rm`, `sort -o CLAUDE.md …`, `find . -fprint CLAUDE.md`,
`echo x > /etc/foo`, `cat ~/.git-credentials`, `printenv`. The traces list the last four kinds as things routines must
never do (night trace #41 and #50). (a)–(c) is how the approved rules get enforced, not a new rule. It only narrows,
binds routines only, touches the same files and needs the same evidence. A refusal never stalls a run.
Not for Mustafa: it is a technical choice inside the vision. The change-set is already a rules change he reconfirms at
merge, and losing tools at night is plan risk 5, which is already accepted.

**Needed commands (traces[0] night, traces[1] PR Advisor).** With the conditions below, every needed command still
works except one form: night #10's `kill -TERM <pid>` for stopping the saver. TaskStop replaces it, which is already
the plan's replacement (ledger L1). Everything else that fails was already refused by approved decisions:
R2 (night #17–20 and #22, PR #11–16), PR-CODE (a) and SETUP (PR #5 `node --version`, #6, #9, #10), M2 (PR #8 `ln -s`),
MERGE-ROUTE (PR #27) and DAY-COST (PR #19, where `git -C` becomes `cd`). The PR #28 cleanup isn't needed.
(c) must allow TZ (night #0). (a) must include `date`, `ls`, `cat`, `head`, `tail`, `grep`, `wc` and `mkdir`
(night #0, #3, #14, #27; PR #0, #5, #7, #20).

**Conditions**
1. The list holds the programs the needed commands use: git, gh, node, python3, npm, the read tools above, and mkdir,
   rm, cp, mv, touch and tee with their target checks. It adds the builtins cd, echo, printf, test/[, true, false and
   pwd, plus strictly read-only helpers. No program on the list may write a file or start a program through any
   option. Where one has such an option (`sort -o`, uniq's output operand, `find -exec/-delete/-fprint*`, `rg --pre`,
   `xxd -r`, `date -s`), that option is refused and tested, or the program stays off the list. Off the list: sed, awk,
   xargs, pagers and editors, make, tar, ln, chmod, kill and pkill, the builtins hash, enable, set and shopt, and env or
   printenv on their own.
2. Programs are called by bare name only. A program named by a path (`./x`, `/usr/bin/x`, `.gate/x.sh`) is refused.
3. (b) checks only the operands a tool writes: redirects, cp's destination, mv's source and destination, and the
   operands of rm, touch, mkdir and tee. Today `sed -n 1,5p docs/STATUS.md` (PR Advisor) and `cp CLAUDE.md .gate/x` are
   refused as writes. Writes go inside the project (minus protected paths) or the scratchpad, plus /dev/null and file
   descriptors. The PR Advisor keeps its narrower limit (.checkpoints/ and .gate/).
4. (c) is load-bearing for (a): PATH, NODE_OPTIONS, BASH_ENV, LD_*, GIT_*, PYTHON* and npm_config_* all change what an
   allowed name runs. Only named variables pass (TZ, and at most LC_ALL, LANG and NO_COLOR), and the same list applies
   to VAR= prefixes, export, `declare -x`, `set -a` and env's options. Bare env, printenv, `export -p` and set are
   refused, because they print the environment.
5. The read tools get the Read tool's credential-path refusals on their file operands (~/.git-credentials,
   ~/.config/gh/, .git/config, /proc/, .netrc, .npmrc). ~ is expanded, and an operand holding `$` or a glob that could
   reach outside the project is refused.
6. Every refusal names what to use instead: kill → TaskStop; curl → WebFetch; sed -i → Edit; sed -n or awk → Read,
   head, tail or grep; a script path → main's tracked script through node. NIGHT_SHIFT.md and CHECKPOINTS.md name
   TaskStop as the only way to stop the saver.
7. Evidence: every probe above becomes a refused row. Each listed program's write and exec options get rows. Every
   needed command in both traces still passes. The mutation runs cover the allow-list check, and the security-auditor's
   final review covers the list.
8. Record this verdict in one line in plan.md L2 ("Both routines") and in the report's design additions, next to the
   launcher split.

**Side findings, not caused by (a)–(c), for the final review:** `node --check` on an edited file is refused (it only
parses, and the gate covers syntax). The Chromium install has no routine form (night #13): TEST-RUN part A should
confirm the night environment's /opt/pw-browsers holds the right revision, or else the setup script installs it.
`npm audit` is refused, which is acceptable in phase 0 because dependency changes touch gate files (daytime only).

**Decisions needed:** none. **For Mustafa:** nothing new.
