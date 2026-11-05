Task: Advisor ruling on pass 4 of A3 (kit-v3-3, PR #38), 2026-11-05.
Plan (blocks): B1 read my prior rulings (pass3 s5, fix-pass2); B2 read pass-4 reports (SA, verifier, auto-6 #14); B3 read code at 60ae124 (plainForm, bash, unwrapRole, writePath, protectedRel, night commit); B4 check probes / git semantics for P4-S4; B5 rule and write ruling; DONE.

- B1 done: pass3 s5 (5.1 narrowing R-PLAIN = verifier only; 5.4 git-semantics bypass of WT-NIGHT/commit/push -> me; default one by-construction refusal in A3 w/ verifier, else split); fix-pass2 rule: routines are the wall (refuse by construction), day is a seatbelt. Next B2.
- B2 done: SA FAIL (S4 crit, S1/S2/S3/S5 high, S6 med, S7/S8 low; fixes proposed); verifier PASS WITH NOTES (V1 obs, V2/V3/V4 low; C6 claim TRUE, 4 rows byte-identical; nuance 'no expansion in a redirect target' still reachable via quoted target, stricter); auto-6 #14 MEDIUM day clone-dir shift. Next B3 code.
- B3 part: code read. protectedRel maps only .claude/worktrees; commit check + protectedHit use protectedRel(worktreeRel(f)) -> .gate/w/... unprotected (S4 confirmed by reading :1052-1053, :1082); switchTargetCheck uses protectedRel(f) (tree path). writePath :403 / writeTarget :484 project-relative only; kitPath :97 is worktree-aware (git common dir). bash() :656 'if (!argv.length) continue' before role redirect loop :666 (S3 confirmed). unwrapRole :463 WRAPPERS.has(b) basename; xvfb-run -e/-f skipped :467 (S1,S5 confirmed). plainForm word() clears atCmd on any first word incl. keywords (S2).
- FINDING A4-1 (HIGH, routine, on main 12d7961 too; mine): a failed cd then ';' : guard reads the next redirect in the cd target, bash writes in the start folder. adv4-cd.out / adv4-cd-main.out (real bash): night and pr-advisor 'cd .gate/nonexist; git status > .claude/settings.json' null -> live settings.json overwritten (not JSON); && form harmless (bash exit 1). Routine texts use no cd (NIGHT_SHIFT, CHECKPOINTS, main advisor.md grep), git -C refused (:969). => exclude cd from the routine program list.
- B3 done. Push check: night pushes only the current branch (:1006-1009) then protectedInBranch(vcwd); git -C refused (:969), so a routine reaches .gate/w only through its cwd (cd in a prior call, or the harness). Commit/push hit = f (tree path) returned by find, so the union keeps every existing reason. kitPath :97 falls back to project-relative when git fails (fakeIo safe).
- B4 done (window drift): NOT a drift on main or A3: main and A3 CLAUDE.md:116/:121 say 07:30; NIGHT_SHIFT.md:3 "00:10 to 07:10" = the cron's start times (STATUS:617, WINDOW-0730 (a) STATUS:543), :17 and :66 window/stop 07:30, REPO_MAP:75 07:30. The 08:30 is in the session's own primary checkout /home/user/Shamble-Frontlines, branch claude/sleepy-tesla-ofsxsr, CLAUDE.md from 3f07223 (kit-v3-0): that session runs an old CLAUDE.md. No edit; tell the session.
- Decisions so far: inside A3 (fix pass 4, narrow); R-PROG = R-PLAIN program clause (list git gh npm node python python3 date; no cd, no wrappers, keywords, paths, quoted program word, redirect before program, command without program) closes S1 S2 S3 S5 A4-1; S3 also bash() reorder (second layer, PLAIN_BEHIND mutant); S4 union protectedRel(f)||protectedRel(worktreeRel) at commit check + protectedHit, kitPath at writePath + writeTarget; check = verifier alone + automated review; C6 accept + quoted-glob-target row; follow-ups FU-P4a (S6), FU-P4b (S8+V4), FU-P4c (#14); S7/V2 + V3 in final docs pass. Next B5 write ruling.

## Ruling (B5)

**Verdict: PROCEED with a narrow fix pass 4 inside A3, checked by the verifier alone; then the final docs pass, the
report and my final-head review.** No question for Mustafa. Every pass-4 finding at High or above is fixed now (his
budget allows a further round only for those); everything below High becomes a STATUS follow-up, not code. The routine
fixes are two rules, both by construction, no new shell syntax:
- **R-PROG** (a clause of R-PLAIN): every command in a routine line starts with a routine program by its bare name.
  It closes P4-S1, P4-S2, P4-S3, P4-S5 and one more High of the same kind that I found (A4-1, below).
- **The worktree path rule** (P4-S4): a routine's kit-path checks also read a path by its place in its own worktree.

What I read: main's advisor.md; mustafa-words.md (scope stop) and the A4 queue note; my pass-3 ruling (sections 1, 2,
4, 5) and my fix-pass-2 ruling; security-auditor-pass4.md and its sap4 outputs (wt, wrap, xvfb); verifier-pass4.md;
security-review-auto-6.md; builder-pass3-routine-a.md; routine-guard.mjs at 60ae124 (protectedRel :32-43, kitPath
:97-108, writePath :403-415, WRAPPERS :266, unwrapRole :438-470, writeTarget :484-497, plainForm :522-608, bash()
:610-688, routineProgram :898-908, git() :966-1060 with the push check :1000-1012, worktreeRel :1068-1070,
protectedHit :1078-1084, switchTargetCheck :1104-1145); NIGHT_SHIFT.md:1-66; REPO_MAP.md:75; CLAUDE.md on main and on
the session's own checkout. What I ran: one probe, adv4-cd.mjs, against the guard at 60ae124 and at main 12d7961 with
real bash 5.2.21 and git 2.43.0 (adv4-cd.out, adv4-cd-main.out). What I did not do: run the suites, the mutants or the
gate (the verifier did: 1084/1084, gate with smoke PASS); I took the auditor's sap4 outputs as written, after checking
each one against the code lines it names.

### 1. Where: inside A3, now (fix pass 4)
- **It fits his scope stop.** He asked that routines refuse any line that is not plain. R-PROG only narrows what
  "plain" means; it teaches the guard no shell syntax. P4-S4 is git's path semantics, and its fix is one rule at four
  callers. My pass-3 items 5.1 and 5.4 named exactly these two cases: narrowing R-PLAIN needs only the verifier's
  check, and a git bypass that one by-construction refusal closes stays in A3.
- **It fits his review budget.** "A third only for a Critical or High finding": these are one Critical and five High.
  Nothing below High is built.
- **A split costs more and gains nothing.** R-PROG lives in plainForm, which exists only on A3's branch. A routine
  change-set off main would have to bring R-PLAIN with it, which means the whole routine half of fix pass 3, then a
  rebase of A3 and two reconfirms instead of one. Main's exposure until the merge stays low, as in pass 3, section 1:
  the Night queue has no unchecked item, every bypass needs the routine agent itself to type the line, branch
  protection stops a hidden push to main, and a kit change on a `cs/` branch still meets the PR review and Mustafa's
  go. A3 is strictly stricter than main on every routine line (the verifier: 1260 + 6024 role decisions, 0 looser),
  so merging it sooner is the fastest way to close the gaps on main.

### 2. Finding by finding
| Finding | Severity (ruled) | Ruling | Shape of the fix |
|---|---|---|---|
| P4-S4: a worktree under `.gate/` (or the scratchpad) lets a routine write, commit and push a `.claude/` or `.github/` change; the paths are read project-relative (`.gate/w/.claude/settings.json`), and protectedRel maps only `.claude/worktrees/<n>/` back | **Critical**, routine, on main | **Fix now, by construction** | The worktree path rule (section 3.1). Commit check and protectedHit: a name is protected when `protectedRel(f) \|\| protectedRel(worktreeRel(f, top, vcwd, io))`; f is the name git prints, the path the commit or push publishes. writePath and writeTarget (routines) also refuse when `kitPath(p, cwd, io)` names a kit path. |
| P4-S3: a command with no program word (`>CLAUDE.md`, `TZ=x >.claude/settings.json`) skips the routine redirect checks (`if (!argv.length) continue` comes first) | **High**, routine, on main | **Fix now** | R-PROG refuses a command with no program word and a redirect before the program. Second layer, one moved block: in bash(), the role redirect and read loops run before `if (!argv.length) continue`. |
| P4-S1: a wrapper or keyword name written as a path (`docs/do git status`, `./docs/timeout 5 …`) is stripped by its basename and bash runs the file; a routine can make the file executable with `git add --chmod=+x` and a worktree | **High**, routine, on main | **Fix now** | R-PROG: the program word must be one of the seven names exactly, so a path, a wrapper and a keyword are all refused. |
| P4-S2: after `if`, `then` or `do`, a quoted `"TZ=./x"` in the program position passes; bash runs `./TZ=./x` | **High**, routine, a gap in fix pass 3's own narrowing | **Fix now** | R-PROG: shell keywords are not routine programs, and the program word is unquoted. |
| P4-S5: `xvfb-run -e/-f <file>` writes the file unseen | **High**, routine, on main | **Fix now** | R-PROG: xvfb-run is not a routine program. No routine needs it: tools/preview.js and tools/pageshot.js are refused to routines already (DENIED_SCRIPTS). |
| **A4-1 (new, mine):** a failed cd followed by `;`. The guard reads the next redirect in the cd's target folder, and bash writes in the start folder. | **High**, routine, on main | **Fix now, in R-PROG** | adv4-cd.out (60ae124) and adv4-cd-main.out (12d7961), real bash: night-shift and pr-advisor `cd .gate/nonexist; git status > .claude/settings.json` gets no deny, and the live settings.json is overwritten with git's output. The `&&` form is harmless: cd fails, and bash exits 1. Shape: `cd` is not a routine program. No routine text uses cd: I grepped NIGHT_SHIFT.md, CHECKPOINTS.md and main's advisor.md. git -C is refused already (:969). |
| P4-S6: D-ASK does not see here-document bodies (a backslash-newline that moves the end line; `$(...)` in an unquoted body) | Medium, day | **Follow-up FU-P4a** | Below High (his budget). Text in section 6. |
| Automated review 6, finding 14: the day clone target after two unknown value options or a bundle such as `-qb` | Medium, day | **Follow-up FU-P4c** | Text in section 6. |
| P4-S8 = P4-V4: D-ASK's prompt cost (every day line with a comment or continuation asks; an agent's is refused) | Low, day, by design | **Follow-up FU-P4b** | Text in section 6. |
| P4-S7 = P4-V2: STATUS's R-PLAIN line says `<&` before a number is allowed, and it leaves out the narrowings | Low, docs | **Final docs pass, now** | The new R-PLAIN line (section 5) replaces it. |
| P4-V3: STATUS.md:157 should say `NIGHT_SHIFT.md:27-30` | Low, docs | **Final docs pass, now** | The docs-keeper re-verifies the line at the final head: step 7 grows, but lines 1-43 do not move. |
| P4-V1: three reasons of fix-pass-2 rows changed (kind kept) | Observation | **Report line** | The report's movers list names the three PASS2 rows. |
| Night-window drift (docs-keeper) | — | **No drift on main; no edit** | Section 6.6. |

### 3. The fixes, exactly
#### 3.1 P4-S4, the worktree path rule (routines)
- **One helper, two callers:** `const kitName = (f, top, vcwd, io) => protectedRel(f) ||
  protectedRel(worktreeRel(f, top, vcwd, io));`. The night commit check (:1052-1053) and protectedHit (:1082) use it
  in place of `protectedRel(worktreeRel(…))`. protectedHit serves the night push check, checkpoint save and watch
  (protectedInBranch). The hit they report stays f, the tree path, so every existing reason is unchanged.
  switchTargetCheck already reads protectedRel(f) and stays as it is.
- **writePath (Write, Edit, NotebookEdit, MultiEdit) and writeTarget (shell redirects), routines:** after the
  project-relative check and before the scratchpad allowance, `const k = kitPath(p, cwd, io)` (writeTarget: `vcwd`).
  kitPath is the day side's worktree-aware test: the path's own worktree top, when its git common dir is this
  repository's. A non-null k is refused. A worktree in the scratchpad is covered the same way.
- **Refusal texts:**
  - Commit (unchanged): `routine (night-shift): the commit would change <tree path> (a rules, CI or gate file)`.
  - Push, checkpoint save and watch (unchanged): `routine (night-shift): the branch changes <tree path> (a rules, CI
    or gate file); it can't be pushed at night`.
  - writePath: `routine (<role>): <p> is <k> in a worktree of this repository (a rules, CI or gate file); those change
    only in a daytime kit change-set`.
  - writeTarget: `routine (<role>): <what> on <p> (<k> in a worktree of this repository: a rules, CI or gate file) is
    not allowed`.
- **Not changed:** git rm, git mv and git add keep their project-relative check, because the commit and push checks
  are the wall for anything staged. Commits made without `git commit` (cherry-pick and the like) are not on the
  routines' git list (GIT_ROUTINE), and the push check would see them anyway.

#### 3.2 R-PROG, the routines' program word (a clause of R-PLAIN, in plainForm, on the raw text)
- **Rule:** in a routine line, every simple command is this, in order:
  1. Zero or more settings `NAME=value`, where NAME is unquoted and one of TZ, LC_ALL, LANG, LANGUAGE or NO_COLOR.
     The value may be quoted.
  2. The program word, unquoted and exactly one of `git`, `gh`, `npm`, `node`, `python`, `python3` or `date`.
  3. Its arguments and redirects. A redirect, including a number before `>` or `<`, may come only after the program
     word.
  A command with any word or redirect but no program word is refused. An empty command (after a trailing `;` or a
  newline) is not a command. Commands are what the separators split: a newline, `;`, `&`, `&&`, `||`, `|` and `|&`.
- **What it refuses, so the list is plain:** cd, pushd and popd; every wrapper (env, timeout, nice, stdbuf, xvfb-run,
  nohup, time, command, builtin, exec, noglob); every shell keyword (if, then, else, elif, fi, do, done, while, until,
  case, for, select, function); any program given by a path or in quotes; nodejs; and every other program, which
  routineProgram refuses today anyway.
- **Refusal text** (a reason of its own, `progReason`, not plainReason): `routine (<role>): start each command with
  its program by name (<what was found>): git, gh, npm, node, python, python3 or date, after only TZ=, LC_ALL=, LANG=,
  LANGUAGE= or NO_COLOR=; redirects after the program; no cd, wrappers or shell keywords`.
  - `<what was found>` is one of:
    - `<the word as typed> is not a routine program`;
    - `a command with no program`;
    - `a redirect before the program`;
    - `<NAME>= may not be set`.
- **Why this is enough:** plainForm already guarantees that bash splits these commands and words as parseShell does
  (C-PLAIN). With R-PROG, the word bash runs is one of seven fixed names, looked up in a PATH the line cannot set. No
  alias exists in a non-interactive bash. A function needs `(` or `{`, and `hash` or `enable` would be the program
  word. So unwrapRole has nothing left to strip but the allowed settings. Its wrapper branches become dead code for
  routines: leave them, with a one-line comment saying so.
- **The S3 second layer:** in bash(), move the role loops over `c.redirects` and `c.reads` above `if
  (!argv.length) continue;`. This affects routines only; the day path is unchanged.

#### 3.3 Rows and mutants (RB = real bash or real git on a scratch repository; the verifier runs them)
- **RB, unchanged probes:** the auditor's sap4-wt.mjs, sap4-wrap.mjs, sap4-wrap2.mjs, sap4-empty.mjs, sap4-xvfb.mjs
  and sap4-sweep.mjs, and my adv4-cd.mjs, run as they are against the fix head.
  - Every line that did harm on 60ae124 is refused: W1-W5, T1, Q1, the four sap4-empty routine lines, the four
    sap4-xvfb lines, both adv4-cd `;` lines, and the sap4-wt Writes, commit and push.
  - The other lines keep their decisions, by name: the night switch to cs/x, C1-C3, the `docs/x` control, T1c, T2,
    the main-folder commit deny, and the sap4-sweep rows.
  - The output is saved, and every line whose decision moved is named.
- **In-suite rows (PASS4_ROUTINE_CASES, night-shift and pr-advisor unless noted):**
  - P4-S1: `docs/do git status`, `./docs/timeout 5 git status`, `docs/env git status`, `docs/then git status`,
    `.gate/w/docs/do git status`, `/usr/bin/git status` and `"git" status`. Deny, reason R-PROG.
  - P4-S2: `if "TZ=./x" date; then date; fi`, `while "TZ=x" date; do date; done`, `'do' git status` and `if git
    status; then date; fi`. Deny, R-PROG.
  - P4-S3:
    - Deny, R-PROG: `>CLAUDE.md`, `TZ=x >CLAUDE.md`, `git status; >.claude/settings.json`, `TZ=x
      >.claude/settings.json`, `TZ=x` alone, `>docs/x.md` (night), `2>&1 git status` and `>docs/x.md git status`.
    - Control null: `git status > docs/x.md` (night).
  - P4-S5: `xvfb-run -a -f .claude/settings.json git status`, `xvfb-run -a -e CLAUDE.md git status` and `xvfb-run -a
    git status`. Deny, R-PROG.
  - Wrappers and keywords: `timeout 5 git status`, `nice git status`, `env TZ=x date`, `stdbuf -oL git log -1`,
    `nohup git status`, `time git status`, `command git status`, `exec git status`, `noglob git status`, `nodejs x.js`
    and `PATH=x git status`. Deny: R-PROG, or unwrapRole's ROLE_ENV_OK text for PATH. Name which one answers.
  - A4-1: `cd .gate/nonexist; git status > .claude/settings.json`, `cd docs && git status` and `pushd docs`. Deny,
    R-PROG.
  - Nulls, unchanged decisions:
    - pr-advisor's step-0 line;
    - NIGHT_NPM;
    - the checkpoint save and watch lines with quoted `-m` and `--trailer`;
    - `TZ=Europe/Amsterdam date +%H:%M`, `TZ="Europe/Amsterdam" date`, `LC_ALL=C git log -1 --oneline`;
    - `git status 2>&1`, `git log -1 >&2`;
    - `git status; git log -1`, the same two commands on two lines, and `git status;` with a trailing `;`;
    - `git commit -q -m "fix #16: it's done"` (night).
- **P4-S4 rows:**
  - Real git, in-suite: a worktree `.gate/w` on `cs/wt` from origin/main. Then:
    - Write and Edit on `.gate/w/.claude/settings.json` and `.gate/w/.github/workflows/ci.yml`: deny (the writePath
      text). The redirect `git status > .gate/w/.claude/settings.json`: deny (the writeTarget text).
    - With those files changed outside the guard (as a checkout would) and staged in .gate/w, `git commit -q -m
      "notes"` from .gate/w: deny, "the commit would change .claude/settings.json".
    - With a commit made outside the guard, `git push -q origin cs/wt` and checkpoint save from .gate/w: deny, "the
      branch changes .claude/settings.json"; origin has no cs/wt afterwards.
    - Control: a docs-only change in .gate/w commits and pushes (null; origin has it).
    - Control: the main-folder rows unchanged.
  - A scratchpad worktree: Write on `<scratchpad>/w/.claude/settings.json`, where w is a worktree of this repository:
    deny.
- **C6 nuance row:** night `git status > 'a*b'`: deny with writeTarget's text ("no variable, glob, brace or ~ in a
  write path"). This gives the move below a live killer with R-PLAIN on.
- **Mutants (kind and reason, killed by the rows above):**
  - R-PROG removed: S1, S2 and S5 rows pass.
  - The "no program" end check removed: the S3 rows pass.
  - The "redirect before the program" check removed: `>docs/x.md git status` and `2>&1 git status` change reason or
    pass.
  - `cd` added to the list: A4-1 passes.
  - A quoted program word allowed: `"git" status` passes.
  - The settings name check removed: PATH answers by unwrapRole's text (a reason move).
  - The commit-check tree-path half removed: the .gate/w commit row passes. Real mutant.
  - protectedHit's tree-path half removed: the .gate/w push and checkpoint save rows pass. Real mutant.
  - kitPath removed from writePath: the Write rows pass.
  - kitPath removed from writeTarget: the redirect row passes.
  - The quoted-glob rule removed: the C6 nuance row passes.
  - In PLAIN_BEHIND_MUTANTS (R-PLAIN off), the S3 second layer back in its old order: `>CLAUDE.md` passes.
- **C-PLAIN:**
  - The corpus keeps every line R-PLAIN accepts after R-PROG. Lines it now refuses leave the corpus by name.
  - The generator draws program words from the seven and settings from ROLE_ENV_OK.
  - The verifier's token-soup differential reruns on the new form with UNSAFE 0.
  - The teeth stay: each parser mutant still fails C-PLAIN.
- **Main's test file:** rows of main's mutation table that R-PROG makes survivors are moved into
  PLAIN_BEHIND_MUTANTS, byte for byte, under the C6 conditions in section 4. The env, command, exec, timeout and
  routine-cd rules are the likely ones. Every failing case of main's file is named, with its message.

### 4. C6: the four rows moved into PLAIN_BEHIND_MUTANTS
**Accepted, as a named C6 edit.** The verifier showed:
- the four rows are byte-identical in main 12d7961, at 01b158c and in head's PLAIN_BEHIND_MUTANTS;
- they are deleted from head's copy of main's table only, with comment lines naming the move;
- with R-PLAIN on, no CASES row can kill them (R-PLAIN refuses first, and CASES is kind-only);
- with R-PLAIN off, five night CASES rows kill them, and plainForm refuses each of those five.
The move keeps each rule tested and changes no rule. Conditions:
- the C6 nuance row above (a quoted glob target) goes into A3's own table, so 'no expansion in a redirect target'
  also has a killer with R-PLAIN on;
- the report names the four rows and the reason;
- the same treatment, under the same proof, is allowed for the rows R-PROG adds.

### 5. Texts, word for word (fix pass 4 and the final docs pass)
**STATUS's R-PLAIN line (replaces it whole; this also closes P4-S7 and P4-V2):**
- **R-PLAIN, routines' plain shell (fix pass 3, narrowed in fix pass 4):** before the guard reads a routine's Bash
  line, the line must be in a plain form that the guard splits exactly as bash does, or it is refused. Outside quotes
  it allows letters, digits, spaces, tabs, newlines, the characters `_ - . / : = @ % + , ^`, a `~` inside a word, and
  the operators `&&`, `||`, `;`, `|`, `|&`, `&`, `>`, `>>`, `>|`, `<`, `&>`, `&>>`, and `>&` followed by a number or
  `-` that ends the word. It refuses `<&`, `;;`, `;&` and a quoted word right before `<` or `>`. Inside double quotes
  it allows no `$`, backtick or backslash. Every quote must be closed. Every command starts with its program,
  unquoted and by its bare name: git, gh, npm, node, python, python3 or date, after only `TZ=`, `LC_ALL=`, `LANG=`,
  `LANGUAGE=` or `NO_COLOR=` settings, and its redirects come after the program (R-PROG, fix pass 4). So routines use
  no comments, expansions, here-documents, braces, brackets or parentheses; no cd, no wrapper (env, timeout, nice,
  stdbuf, xvfb-run, nohup, time, command, exec), no shell keyword (if, then, do, while and the rest), no program given
  by a path, and no command without a program. A corpus test against real bash checks the claim.

**STATUS, the fix-pass-4 summary (in A3's row):** "Fix pass 4 (pass 4's one Critical and four High findings, all on
the routine side, plus one High the Advisor found; five of the six were on main too): R-PROG narrows R-PLAIN so every routine
command starts with git, gh, npm, node, python, python3 or date by name, which closes wrappers and keywords given by a
path, a quoted program after a keyword, a command with no program, xvfb-run's file options and a failed cd followed by
`;`. And a routine's kit-path checks also read a path by its place in its own worktree: the night commit and push
checks (and checkpoint save and watch) test the name git prints, the path the commit or push publishes, and a
routine's Write, Edit and redirect checks use the worktree-aware kit-path test, so a worktree under `.gate/` or the
scratchpad cannot carry a rules or CI change into a commit or a push. Checked by the verifier alone (the review
budget)."

**NIGHT_SHIFT.md, step 7, add after "…run git in the foreground.":** "Start every command with its program by name
(git, gh, npm, node, python, python3 or date): no cd, env, timeout, xvfb-run or other wrapper, and no if, while or
other shell keyword."

**The report:** under "changes to his words", fix pass 4 is listed with this reason: "pass 4 found one Critical and
five High routine gaps (P4-S1 to P4-S5, and A4-1); his review budget allows a further round only for those, so they
were fixed by narrowing R-PLAIN (R-PROG) and by one worktree path rule, and checked by the verifier alone; nothing
below High was built." P4-S1, P4-S3, P4-S4, P4-S5 and A4-1 are named as pre-existing on main 12d7961 and fixed here.
The movers list carries P4-V1's three PASS2 rows and every row R-PROG moves.

### 6. Follow-ups, word for word (STATUS's follow-ups)
All four wait for the kit freeze to lift (A4-FREEZE (a): after the netcode proof test is under way). None is built in
A3. The final docs pass writes only these lines, the texts in section 5, and P4-V3's line fix.
1. **FU-P4a, here-document bodies in D-ASK** (pass 4, P4-S6, Medium, day): D-ASK does not look inside a
   here-document's body. So in a day session, a body with an unquoted delimiter can run a command unseen: a `$(...)`
   or a backtick in the body, or a backslash-newline that moves the end line so that the next lines run as commands.
   Fix: such a body (unquoted delimiter, and the body holds a `\`, a `$` or a backtick) asks too, and an agent's is
   refused; FU-P3a's corpus covers it. After the kit freeze.
2. **FU-P4b, D-ASK's prompt cost** (pass 4, P4-S8 and P4-V4, Low, day): D-ASK asks on every day line that holds a
   comment, a `\` line continuation, `$'…'`, an unusual here-document delimiter or `>&file`, and refuses an agent's,
   even far from kit files (`npm test # run` asks; an agent's `ls docs # list` is refused). This is the cost of
   Mustafa's rule ("any line the guard can't fully read is an ask"). Watch it in day sessions. If it is noisy, a
   later kit change-set may let a comment that holds no quote or backslash pass (it hides nothing from the reader),
   or tell agents in their agent files to write shell lines without these forms. Mustafa's call. After the kit
   freeze.
3. **FU-P4c, the day clone folder after unknown options** (automated review 6, finding 14, Medium, day): when `git
   clone` carries an option the guard does not know, the guard checks the folder git makes from the URL only at the
   second positional word. So two unknown options that take values, or a short bundle with a value letter such as
   `-qb`, can shift the URL further, and a clone into a kit folder named by the URL's last part does not ask. Fix:
   with an unknown option present, check the folder made from every positional word (pass 2's shape), and read a
   short bundle that holds a value letter (b, o, u, c, j) as taking the next word. After the kit freeze.
4. **The A3 docs lines (now, in the final docs pass):** the new R-PLAIN line (section 5) replaces the old one (P4-S7,
   P4-V2), and STATUS.md:157 reads `NIGHT_SHIFT.md:27-30` (P4-V3), re-verified at the final head.
5. **The A4 and B.1 words:** N/queue/mustafa-2026-11-05-a4-b1.md goes into STATUS's "Queued, in Mustafa's words" in
   this docs pass, as that file says.
6. **Night-window "drift": not a drift; no edit.**
   - Main (CLAUDE.md:116) and A3 (CLAUDE.md:121) both say 07:30.
   - NIGHT_SHIFT.md:3's "00:10 to 07:10" is the cron's start times: the last run starts at 07:10 and only finishes
     work (STATUS:617; WINDOW-0730 (a), STATUS:543).
   - NIGHT_SHIFT.md:17, NIGHT_SHIFT.md:66 and REPO_MAP.md:75 say the window ends at 07:30.
   - The 08:30 the docs-keeper saw is in this session's own checkout, /home/user/Shamble-Frontlines, on branch
     claude/sleepy-tesla-ofsxsr. That checkout's CLAUDE.md is from kit-v3-0 (3f07223). The session should know its
     loaded CLAUDE.md is older than main's, and should read main's copies before it relies on the loaded text.

### 7. The check after the fixes (under his review budget)
- **Who:** the verifier alone (Opus 5.5, effort high), on the fix-pass-4 diff only (60ae124 to the fix head), plus the
  automated security review that runs on the fix commit by itself. No security-auditor pass and no skeptics. This
  follows my pass-3 items 5.1 (R-PROG narrows R-PLAIN) and 5.4 (P4-S4 is one by-construction rule).
- **What the verifier checks:**
  - The suites.
  - Section 3.3's rows: the RB probes run unchanged with their output saved, and every moved line named.
  - Every mutant killed.
  - C-PLAIN and the token-soup differential with UNSAFE 0.
  - No routine line looser than at 60ae124: role decisions, with every newly stricter routine row named, and each one
    checked against the routine texts (NIGHT_SHIFT.md, CHECKPOINTS.md, main's advisor.md: their commands all pass
    R-PLAIN with R-PROG).
  - Main's test file, with every failure named.
  - The C6 moves, byte for byte.
  - The gate, with smoke PASS.
  - Section 5's texts, word for word, and the line references at the head.
- **What makes a further finding a follow-up, not another pass:**
  1. Anything below Critical or High, from the verifier or the automated review, is a STATUS follow-up with its
     words. It is not built in A3 (Mustafa's budget).
  2. A Critical or High finding where the fix-pass-4 code does not close its own rows, or loosens a routine line, is
     fixed by the builder within section 3's shapes (a narrowing, never a new parse rule). The verifier rechecks only
     that row, its mutant and the no-looser diff. No new pass.
  3. A Critical or High finding of a new kind comes to me. My default: if one more narrowing of R-PLAIN or one
     by-construction refusal closes it, it goes into A3 with the verifier's check. If not, A3 goes ahead as it is (it
     is stricter than main on every routine line), and the finding becomes a named urgent follow-up, raised with
     Mustafa under "Decisions needed".
- **Then:** the docs-keeper's final pass (the texts in sections 5 and 6 only), the report with the checkpoint folded
  in, my review of the final head (I run the gate myself), and then Mustafa's reconfirm. A3 stays a draft until
  then.

### Decisions needed
None. Where to fix, the fix shapes and the check are mine to decide inside his scope stop and review budget: the
fixes narrow R-PLAIN and add no shell syntax; they answer only Critical and High findings; everything below High
becomes a follow-up. A4 waits (A4-FREEZE (a)).

### For Mustafa
Nothing now. A3 (PR #38) stays a draft. After fix pass 4, the verifier's PASS and my final-head review, it comes to
you to reconfirm the merge (it changes `.claude/`, CLAUDE.md and the night shift's rules). It now also closes five
routine gaps that are on main today (and one in its own earlier code), among them one Critical: a worktree under `.gate/` could carry a rules or CI
change into a night push.

DONE
