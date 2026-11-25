# Plan: guard-env-fix (SEC-U7B-1) · risk tier 2 · for the advisor agent's review

- **Asked by:** Mustafa's brief to "the Guard fix chat", 2026-11-25 14:55 Amsterdam (pasted in this session's chat).
- **Finding:** SEC-U7B-1 in `.checkpoints/netcode-proof-test/security-u7-0b-answer.md` and `q-guard-env.md`, on
  `claude/netcode-proof-test-plan-hcky5g` (read with `git show`, never checked out).
- **Branch:** `claude/confident-gates-tbnq38` (this cloud session may push only there; it stands in for
  `cs/guard-env-fix`). **Base:** main 3ab9eaec (fast-forwarded; no `.claude/` or `CLAUDE.md` change since 09faefa).

## 1. Goal
`unwrap()` (`.claude/hooks/routine-guard.mjs:295-316`) strips wrappers before the day guard reads a command. For
`env`, `nice` and `stdbuf` it drops an option word but not that option's value, so the value is taken for the program
and git or gh behind it get no check and no question in a day session or a subagent. Make unwrap() read those three
wrappers' options with their values, and make any option it does not know (or cannot follow) count as unreadable: an
ask in a day session, a refusal for a subagent. Routines already refuse every wrapper (R-PROG in `plainForm`, before
`unwrapRole()` runs); tests prove that for the new forms, with no code change on that path.

Evidence of the gap, by calling `unwrap()` alone (no command run): `probe-unwrap-inscope.mjs` beside this file, its
output `probe-before.txt`: `env -C /d git status` → program `/d`; `env -u X` → `X`; `env -S x` → `x`;
`env --chdir /d` → `/d`; `nice --adjustment 5` → `5`; `nice --adj=5` → `--adj=5`; `stdbuf -o L` → `L`;
`stdbuf --output L` → `L`; `env x.y=1` → `x.y=1`. One-word forms (`env -C/d`, `--chdir=/d`) reach git, but in a folder
git() does not check.

## 2. Files and symbols
- `.claude/hooks/routine-guard.mjs`:
  - **add** `const WRAP_OPTS` (just above `unwrap`): per wrapper, three patterns, from GNU coreutils 9.4's `--help`
    (this container's version):
    - `env`: flag `-i -0 -v --ignore-environment --null --debug`; value `-u --unset`; unread `-C -S --chdir
      --split-string` (read with their value, but they ask: git would run in a folder, or on words, the guard does not
      follow).
    - `nice`: flag `-N` (digits only: today's accepted obsolete form); value `-n --adjustment`.
    - `stdbuf`: value `-i -o -e --input --output --error`.
  - **change** `unwrap(argv)`: returns `{ argv, unreadable }` instead of an array (`unreadable`: null, or a short name
    of the form, e.g. `env -C`). Its three lines for `env` (:302), `nice` (:309) and `stdbuf` (:310) become one branch:
    - loop while the word starts with `-` (and is not a lone `-`): `--` ends the options; a value option takes the rest
      of its word (`-uX`, `--unset=X`) or the next word (`-u X`, `--unset X`); an unread option does the same and sets
      `unreadable`; a flag is dropped; **any other option word sets `unreadable`** (an abbreviation such as `--ch` or
      `--adj=5`, a short-option cluster such as `-iu`, a signal option, `--null=x`, nice's `--5`) and is dropped alone.
      All three tools stop at the first non-option word (getopt with a leading `+`), as the loop does; nice's repeated
      options (`-n 5 -n 3`) are read in turn.
    - `env` only: a lone `-` after the options is dropped (env reads it as `-i` and stops); then every word holding
      `=` is dropped (env's own rule: any such word is an assignment, so `x.y=1` is not the program).
    - Dropping an unknown word alone and reading on can only add a refusal (a force push behind it is still denied):
      the ask stays, so it never turns an ask into an allow.
  - **change** the one caller, `bash()` (:721, `argv = unwrap(c.argv)`): take `u.argv`; when `u.unreadable` is set, a
    subagent (`input.agent_id`) is refused and the main session gets `dayAsk ??= ask(...)`, exactly as parseShell's
    `unreadable` is handled at :702-709; the scan goes on, so a later deny in the line still wins.
  - Unchanged: `parseShell`, `unwrapRole` (routines), `dayFolder` and `dayShellWrites` (they get the same argv suffix
    as before; `dayFolder`'s prefix arithmetic still holds), every other wrapper branch.
- `.claude/hooks/routine-guard.test.mjs`: rows in `DAY_CASES`, mutants in `DAY_MUTANTS`, rows in the routine table.
- Docs (docs-keeper, at the end): STATUS (the in-flight row; SEC-U7B-1's state; a follow-up for the out-of-scope
  findings in §7, worded without the exact forms), ROADMAP entry, and the report `docs/reports/2026-11-25-guard-env-fix.md`.

## 3. Why ask on every program, not only git and gh
`env -C DIR` moves the folder for any program, so a relative write target (`env -C .claude sed -i ... settings.json`)
resolves where `dayShellWrites` does not look. Asking on the wrapper form, whatever runs behind it, closes that too.
Security's sketch (follow `-C` as an unknown folder) needs changes in `bash()`/`dayFolder` beyond this function; the ask
is one rule, stricter, and costs a question only for a form nobody needs (security's Q3: no `env -C`). The known forms
in use stay silent: `env -u CARGO_NET_OFFLINE cargo ...` and `env CARGO_NET_OFFLINE=false` (security's answer :106,
:211), `stdbuf -oL`, `nice -n N`.

## 4. Tests (call `decide()` with the fake io only; no live command)
- **Every known form, with git behind it**, in `DAY_CASES`, two rows each: `<form> git push --force origin cs/x` →
  deny (proves git was reached), `<form> git status` → allowed (today's allowed stays allowed). Forms: `env -i`,
  `env -`, `env --ignore-environment`, `env -0`, `env --null`, `env -v`, `env --debug`, `env -u X`, `env -uX`,
  `env --unset X`, `env --unset=X`, `env --`, `env X=1`, `env x.y=1`, `env -i -u X Y=1`; `nice`, `nice -n 5`,
  `nice -n5`, `nice -n -5`, `nice --adjustment 5`, `nice --adjustment=5`, `nice -5`, `nice --`, `nice -n 5 -n 3`;
  `stdbuf -i 0`, `-i0`, `--input 0`, `--input=0`, `-o L`, `-oL`, `--output L`, `--output=L`, `-e 0`, `-e0`,
  `--error 0`, `--error=0`, `stdbuf -oL --`; and a chain `env -u X nice -n 5 stdbuf -o L`.
- **Every form the guard does not follow**, two rows each: `<form> git status` → ask (reason names the form), and the
  same from a subagent → deny. Forms: `env -C /d`, `env -C/d`, `env --chdir /d`, `env --chdir=/d`, `env -S s`,
  `env -Ss`, `env --split-string s`, `env --split-string=s`, `env --ch /d`, `env --block-signal`, `env -iu X`,
  `env --null=x`, `nice --adj=5`, `nice --5`, `nice -x 5`, `stdbuf --out L`, `stdbuf -x 0`. Plus
  `env -C /d git push --force origin cs/x` → deny (the deny wins over the ask).
- **Routines:** `env -C /d git status`, `nice --adjustment 5 git status`, `stdbuf -o L git status` → deny for the night
  shift and the PR Advisor (R-PROG's reason), through the existing `roleRows` helper.
- **Mutants** (`DAY_MUTANTS`: each switched off in a copy must make at least one case fail): a value option takes its
  next word; an unknown option is unreadable; env's `-C`/`-S` are unreadable; env's `=` words are dropped; env's lone
  `-`; nice's `-N` flag; the ask wired into `bash()`; the subagent deny.
- **Baseline (measured on main 3ab9eaec's guard, 2026-11-25):** `node --test .claude/hooks/routine-guard.test.mjs` →
  1107 tests, 1107 pass, 0 fail. After: 1107 + the new rows, 0 fail, no existing row changed or removed.
  `settings.test.mjs` and `checkpoint.test.mjs`: the same counts as main.

## 5. Size, tier, change type, evidence, reviewers
- **Size (estimate):** the guard about +30/−4 lines, the tests about +45 (the rows are generated from the form lists).
  I read Mustafa's 150-line cap as these two files; the docs and the report are stated separately in the report. If
  the two files pass 150 added lines, I stop and ask him.
- **Risk tier 2** (a `.claude/` file, the guard). **Change type:** logic (Node harness tests on the real code, with
  assertions) in a kit file. Nothing a player sees, hears or feels: no Brave check.
- **Evidence:** the test counts before/after; the probe's after-output beside `probe-before.txt`; the mutants;
  `node tools/gate.mjs` PASS; the verifier's PASS.
- **Order (his item 5):** this plan → the advisor agent's review → the edits (each asks Mustafa: a kit file) → the
  gate and the kit tests → security-auditor on the final diff → at most one fix pass and a second round on what
  changed → the verifier → docs-keeper → `/advisor-report` → the full commit id to Mustafa, nothing pushed after it.
  Reviewers: the advisor (plan and final head), security-auditor (tier 2), the verifier. test-engineer is not called:
  his order names security-auditor, and the verifier checks the proof. Agents run on Opus 5.5 at their files' effort;
  at most 3 at a time (here, one at a time).
- **Checkpoint:** `.checkpoints/guard-env-fix.md`, saved and pushed after every step.

## 6. Risks
- A form in use today starts to ask: only forms outside the lists above (abbreviations, clusters, signal options,
  `env -C`/`-S`). The full existing suite must stay green unchanged; the known-forms rows cover what sessions use.
- A wrong pattern lets a value through as the program again: each known form has a force-push row that must deny, and
  each rule has a mutant.
- `unwrap()`'s return shape changes: it has one caller (`grep -n "unwrap(" routine-guard.mjs`: :721 only; the tests
  never import it). `dayFolder`'s `c.argv.length - argv.length` still holds: argv is still a suffix of `c.argv`.
- BSD env (macOS) has other options (`-P`, `-L`, `-U`): they are unknown here, so they ask. Never a silent allow.
- Until the merge, the interim rule holds: no session or agent of this chat puts env, stdbuf or nice with options in
  front of git or gh; nothing tries the gap live.

## 7. Found, outside the scope (reported to Mustafa, not fixed)
Two other wrappers, and two keywords that unwrap() strips, have the same class of gap (a word taken for the program in
a day session; routines unaffected through R-PROG), and `unwrapRole()` carries the same nice/stdbuf lines as dead code.
The exact forms are kept out of this public repository until a fix: they are in the session's scratchpad
(`/tmp/claude-0/-home-user-Shamble-Frontlines/016237bc-e7ac-5a74-a70f-32a985385e94/scratchpad/out-of-scope-gaps.md`)
for the advisor's and security-auditor's eyes, and go to Mustafa in the chat.
