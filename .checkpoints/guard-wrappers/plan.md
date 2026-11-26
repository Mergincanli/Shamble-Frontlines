# Plan: guard-wrappers (SAME-GAP) · risk tier 2 · for the advisor agent's review

- **Asked by:** Mustafa's message of 2026-11-25 17:05 Amsterdam, item 3 (SAME-GAP (a), now, from the new main).
- **Branch:** `claude/confident-gates-tbnq38` (fast-forwarded to main c1da1bd after #69's merge; stands in for
  `cs/guard-wrappers`). **Base:** main c1da1bd.
- **This file names no wrapper and no command form** (his item 3e: the open forms stay out of docs, notes and pull
  request text until the fix is merged). The full table, the forms and the test rows are in the session's scratchpad:
  `/tmp/claude-0/-home-user-Shamble-Frontlines/016237bc-e7ac-5a74-a70f-32a985385e94/scratchpad/guard-wrappers-plan-detail.md`.

## 1. Goal (his item 3a, a rule, not a list of forms)
Nothing in front of git or gh gets through the day guard unread. Three parts, all in `.claude/hooks/routine-guard.mjs`:
- **T, one table for every name `unwrap()` strips:** every wrapper and keyword it strips today (each name in the
  `WRAPPERS` set, `command`, and the four option-taking wrappers it handles by hand) reads its options from
  `WRAP_OPTS` (flags, options with a value, options it reads but cannot follow, and a count of positional words before
  the command). A name with no options gets an empty entry: any option word after it is unknown. An option word
  outside the table makes the command unreadable: an ask in a session, a refusal for an agent (as #69 did for three
  wrappers). A program word still starting with `-` after a table name is unreadable too (#69's line, now reached by
  every stripped name). The hand-written branches for `command` and the other wrappers go; `WRAPPERS` stays as a set
  (`dayFolder` and `unwrapRole` read it) and points `unwrap()` to an empty entry for its keywords.
  Mapping, so "every" can be checked: each `WRAPPERS` name and `command` → the table (an entry with options where the
  program has any, an empty one otherwise); the hand-handled wrappers → the table; nothing stays outside it.
- **C, the turn limit (SA-R1-3, his 3b):** `unwrap()` counts its turns; a line that reaches the limit with words left
  is unreadable (asks) instead of passing.
- **U, unknown wrappers (his 3a, last sentence):** a program the guard does not strip, with a later word whose base
  name is `git` or `gh`, is unreadable, unless the program only reads words (a short list: echo, printf, which, type,
  whereis, man, grep, egrep, fgrep, rg, ls, cat, head, tail, wc, test, `[`). In `bash()` after the cd skip; day
  sessions only (routines: R-PROG already refuses every program outside their short list).

## 2. Symbols
- `WRAP_OPTS`: new entries (with options where the program has any) and a `pos` field; `o.value` read with `?.` (an
  empty entry has none). `unwrap()`: the `command` line and three hand-written branches removed; the `WRAPPERS` line
  becomes a lookup of an empty entry; rule C. New `NON_RUNNERS` (a Set) and rule U in `bash()`.
- Unchanged: `parseShell`, `unwrapRole` (routines), `dayFolder`, `dayShellWrites`, every message of #69.

## 3. Tests (pushed together with the fix, never before: his 3e)
Every known option form of each table entry with a force push behind it (deny) and with git status (stays allowed);
every unknown form asks and refuses an agent; rule U's ask and agent refusal, and its read-only programs staying
silent; rule C's line at the limit (ask); the env -u row the advisor agent asked for in #69 (L2: env takes `git` as
the value and runs another program, so no check: allowed); a mutant for each rule.
Existing rows, checked before the review by reading every command string in the test file with the real
`parseShell()`/`unwrap()` (758 strings; `.gate/guard-wrappers/scan-rows.mjs`, nothing run):
- **One row changes:** #69's "a program named constructor is no wrapper (no throw)" (`constructor -x git status`,
  expected allowed) would ask under rule U, which is right (an unknown program with git behind it). Its command
  becomes `constructor -x status`: still allowed, and the plain-object mutant it exists for still throws on `-x`.
- **One mutant row is replaced:** "R-c unwrap strips command's options" anchors on the `command` line that moves into
  the table; it is replaced by a mutant on the table's `command` entry, caught by a known-form row.
- Rows whose outcome stays the same: `time -p cd /p && git switch` from a worktree (ask, now through the main-folder
  check instead of the unknown folder); `env X=1 -i git push --force` (ask); the generated rows of #69 (their asks
  keep #69's reason, which is set first). All other existing rows: no hit.

## 4. SA-R2-2 (his 3c: find out, change nothing)
Claude Code's hooks reference (https://code.claude.com/docs/en/hooks, read through the claude-code-guide agent): a
command hook's default timeout is 600 s; on PreToolUse a timed-out hook "doesn't block the tool call, except for
hooks whose failure blocks the action ... To block the call when the hook times out, set `onFailure: "block"`"; a
timeout counts as a failure; with "block" a failure "does what exit code 2 does on that event, except on
PermissionRequest, where it denies the request"; onFailure needs Claude Code 2.1.295 or later. Our three guard hooks
already carry `"onFailure": "block"` (`.claude/settings.json:273, 283, 292`), and this container runs 2.1.332. So a
very long line that runs the guard past 600 s is blocked, not let through. Nothing changes. Caveats: the quotes came
through a model-extracted page fetch (to be rechecked on the page before any decision doc pins them); a session on
an older Claude Code (before 2.1.295) would let a timed-out call through (`claude --version` shows it).

## 5. Size, tier, evidence, order
- **Size (estimate):** guard about +25/−8, tests about +55: about 80 added lines of code and tests (stop line 200, CAP (a)).
- **Risk tier 2.** Change type: logic in a kit file. Nothing a player sees: no Brave check.
- **Evidence:** the three kit test files (main: 1247, 127, 33, measured on c1da1bd before the edit), plus the new rows
  and mutants; `node tools/gate.mjs --no-smoke` (smoke cannot start in this container; CI runs it); `unwrap()` called
  alone on the open forms before and after (output kept in the scratchpad until the merge).
- **Build:** after APPROVE, in a copy under `.gate/guard-wrappers/` (gitignored), with the full suite run there; then
  the guard and its tests applied to the real files and committed as ONE commit (the checkpoint saver pushes any code
  commit, so a guard-only commit would go up without its tests), and the hook never runs a half-edited guard. Commit
  messages, docs, notes and the PR text name no open form; the reviewers keep their probe scripts in `.gate/` or the
  scratchpad, never in `.checkpoints/`.
- **Order:** this plan, the advisor agent, the edits (each asks Mustafa), the kit tests and the gate, security-auditor
  on the final diff (at most two rounds), the verifier, docs-keeper, the report, the merge on his go after his
  Advisor's check. Checkpoint `.checkpoints/guard-wrappers.md`, pushed after every step.

## 6. Risks
- Rule U asks on a command that only names git or gh as data (an install line, a file called git): an ask, never a
  silent pass; agents are refused, so they keep such words out of command lines.
- A form a session uses today starts to ask: the existing suite must stay green unchanged; the known-forms rows cover
  the forms in use.
- Until the merge: git and gh always start their own command (his item 4); nothing is tried live; the new rows go up
  only with the fix.

## 7. The advisor agent's verdict: APPROVE with 9 conditions (its notes: advisor-plan.md); what the copy build showed
Conditions 1-3 and its ruling (1) concern open forms and stay out of this file until the merge; they are met in the
copy (`.gate/guard-wrappers/hooks/`). Condition 4: every mutant anchor present and unique (the suites assert it).
5: the constructor row is kept with the rule-U outcome (ask) and a new row keeps the no-throw case (allowed).
6: new rows only in DAY_CASES, CASES and the mutant lists (decided by the hook, never run in bash). 7: red then
green in the copy: 92 failures on the unchanged guard, then 1437/1437 (1247 old, 190 new). 8 and 9: at the apply
step and in the report.
Counts corrected from §3: existing test lines changed: 3 (the constructor row, now two rows; two mutant rows
re-anchored: "R-c unwrap strips command's options" onto the table's `command` entry, and "a Map, not a plain object"
onto the new lookup line); one new row in CASES (keywords before git status go on), which catches the older "shell
keywords" mutant now that rule U reads on. Rule U leaves out a program word that starts with `-` (the wrapper step
already asks on it), so #69's leftover-option row keeps its ask.
Size: guard +33/−14, tests +54/−3 = 87 added lines of code and tests (stop line 200).

## 8. Round 1 (security-auditor on 5df545c: REVISE) and the plugin review: the fix pass, built in the copy
Findings (ids only here; their forms stay out until the merge): SA-d Critical, older than this diff (git under its
own helper names): the half behind another program is fixed (rule U's word test and read-on know the helper names);
the bare half (no program in front) is outside his literal words and asked of him (SA-D-DIRECT). SA-a Medium fixed
(a wrapper run by the input-fed wrapper with no program word after it asks). SA-g Medium fixed: rule U's read-on
keeps git's deny only when the word after git or gh is not an option, so a data word asks instead of being refused
(this narrows the advisor agent's condition 2 for that one shape; still stricter than main; agents refused). SA-h,
SA-b, SA-f Low fixed (the last path part; read-only programs by bare name; rows). SA-c, SA-e and shell function
definitions recorded, not fixed. The plugin review (High): one long option of the input-fed wrapper takes its value
only after `=`; it was read as taking the next word, a silent pass this diff opened: fixed (it now asks), checked
against findutils' manual; the other tables' values are required (coreutils: "Mandatory arguments..."; the
display-server wrapper's getopt line uses single colons).
Open by design (stated as L1's class): words the input-fed wrapper reads from its input reach a program the guard does
not strip unread.
Copy: red 18 failures on 5df545c's guard, green 1485/1485; the auditor's exhaustive pass against main (9804 lines):
no deny or ask became a pass (121 deny→ask, each where the real program is a dash word: SA-e).
Size against main: guard +38/−14, tests +77/−3 = 115 added lines of code and tests (stop line 200).

## 9. Round 4 (Mustafa, 2026-11-25 23:59: R3-WAY (a); one last round, on what changed since round 3 only)
His words: "xargs asks me in a session and refuses a subagent, unless a read-only program follows it." Round 4 is the
last round; a Critical or High left open means two options only (merge what is approved, or drop the change).
- Built in the copy `.gate/guard-wrappers/hooks-r4b/` (not yet applied). It closes R3-1 and R3-2 as his line says.
- The pre-built hooks-r4 had dropped R2-1's check (R2-1 reopened there); found before applying, closed in hooks-r4b.
- Hunk order: one atomic Edit of the changed function, so the guard never names an undeclared variable between asks.
- Tests: rows R3 and R3b (asks, agent refusals, two silent controls) and five mutants; one mutant anchor re-pointed.
- Copy: green 1532/1532; red 29 on 9ad6bc9's guard, red 15 on hooks-r4.
- Size against main: guard +46/−14, tests +103/−3 = 149 added lines of code and tests (stop line 200).
- Boundary, for the advisor agent's ruling: other programs outside the guard's wrapper table are proposed as a named
  limit (INPUT-RUN, the class of stated limits L1-L4), a follow-up for the next kit change-set; detail in the scratchpad.
- Out of this round (his item 4): R2-2, R3-3, R3-4..R3-6, R2-3.

### 9b. The advisor agent on §9 (2026-11-26; notes advisor-plan.md, "Round 4 plan change")
- hooks-r4b APPROVE, applied only together with ADV-R4-1 (Critical, older than this change-set: a spelling of xargs
  the guard left silent; his line (a) decides it). Built in `.gate/guard-wrappers/hooks-r4c/`: ask-only, agents
  refused. Like xargs at the front, it stays silent only when a read-only program follows (his line, not stricter).
- INPUT-RUN: a named limit (the class of stated limits L1-L4), a follow-up for the next kit change-set; listed in the
  report as a security finding left open. Lows ADV-R4-2 and ADV-R4-4: follow-ups; ADV-R4-3 (a comment) rides.
- Copy: green 1551/1551; red 17 on hooks-r4b, red 45 on 9ad6bc9's guard. Size against main: guard +55/−14,
  tests +111/−3 = 166 added lines (stop line 200). The wide sweep against hooks-r4b and main: running.
- Next: the advisor agent reviews hooks-r4c (a plan-change review, not a round), then the Edits asked of Mustafa.
- Bringing in main (#71): in one separate detached worktree after round 4 (merge, docs-keeper, report, checkpoint
  deletion, push by name); the main folder does not move.
