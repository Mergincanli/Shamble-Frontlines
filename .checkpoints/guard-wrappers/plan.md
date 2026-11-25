# Plan: guard-wrappers (SAME-GAP) · risk tier 2 · for the advisor agent's review

- **Asked by:** Mustafa's message of 2026-11-25 17:05 Amsterdam, item 3 (SAME-GAP (a), now, from the new main).
- **Branch:** `claude/confident-gates-tbnq38` (fast-forwarded to main c1da1bd after #69's merge; stands in for
  `cs/guard-wrappers`). **Base:** main c1da1bd.
- **This file names no wrapper and no command form** (his item 3e: the open forms stay out of docs, notes and pull
  request text until the fix is merged). The full table, the forms and the test rows are in the session's scratchpad:
  `/tmp/claude-0/-home-user-Shamble-Frontlines/016237bc-e7ac-5a74-a70f-32a985385e94/scratchpad/guard-wrappers-plan-detail.md`.

## 1. Goal (his item 3a, a rule, not a list of forms)
Nothing in front of git or gh gets through the day guard unread. Four parts, all in `.claude/hooks/routine-guard.mjs`:
- **T, tables:** every option-taking wrapper that `unwrap()` strips reads its options from `WRAP_OPTS` (flags, options
  with a value, options it reads but cannot follow, and a count of positional words before the command). An option
  word outside the table makes the command unreadable: an ask in a session, a refusal for an agent (as #69 did for
  three wrappers). The other wrappers' old hand-written branches in `unwrap()` go.
- **L, leftover:** a program word that still starts with `-` after `unwrap()` is unreadable. This covers the keywords
  and the option-less wrappers without a table entry for each.
- **C, the turn limit (SA-R1-3, his 3b):** `unwrap()` counts its turns; a line that reaches the limit with words left
  is unreadable (asks) instead of passing.
- **U, unknown wrappers (his 3a, last sentence):** a program the guard does not strip, with a later word whose base
  name is `git` or `gh`, is unreadable, unless the program only reads words (a short list: echo, printf, which, type,
  whereis, man, grep, egrep, fgrep, rg, ls, cat, head, tail, wc, test, `[`). In `bash()` after the cd skip; day
  sessions only (routines: R-PROG already refuses every program outside their short list).

## 2. Symbols
- `WRAP_OPTS`: six new entries and a `pos` field. `unwrap()`: the table branch also serves the stripped keywords that
  take options (the `WRAPPERS` line skips names the table holds; the set itself is unchanged, since `dayFolder` and
  `unwrapRole` read it); rules L and C; three old branches removed. New `NON_RUNNERS` (a Set) and rule U in `bash()`.
- Unchanged: `parseShell`, `unwrapRole` (routines), `dayFolder`, `dayShellWrites`, the `command` line (an existing
  mutant anchors on it), every message of #69.

## 3. Tests (pushed together with the fix, never before: his 3e)
Every known option form of each table wrapper with a force push behind it (deny) and with git status (stays allowed);
every unknown form asks and refuses an agent; rule U's ask and agent refusal, and its read-only programs staying
silent; rule C's line at the limit (ask); the env -u row the advisor agent asked for in #69 (L2: env takes `git` as
the value and runs another program, so no check: allowed); a mutant for each rule. All 1247 existing tests unchanged.

## 4. SA-R2-2 (his 3c: find out, change nothing)
Claude Code's hooks reference (https://code.claude.com/docs/en/hooks, read through the claude-code-guide agent): a
command hook's default timeout is 600 s; on PreToolUse a timed-out hook "doesn't block the tool call, except for
hooks whose failure blocks the action ... To block the call when the hook times out, set `onFailure: "block"`"; a
timeout counts as a failure; with "block" a failure "does what exit code 2 does on that event, except on
PermissionRequest, where it denies the request"; onFailure needs Claude Code 2.1.295 or later. Our three guard hooks
already carry `"onFailure": "block"` (`.claude/settings.json:273, 283, 292`), and this container runs 2.1.332. So a
very long line that runs the guard past 600 s is blocked, not let through. Nothing changes.

## 5. Size, tier, evidence, order
- **Size (estimate):** guard about +25/−8, tests about +55: about 80 added lines of code and tests (stop line 200, CAP (a)).
- **Risk tier 2.** Change type: logic in a kit file. Nothing a player sees: no Brave check.
- **Evidence:** the three kit test files (main: 1247, 127, 33, measured on c1da1bd before the edit), plus the new rows
  and mutants; `node tools/gate.mjs --no-smoke` (smoke cannot start in this container; CI runs it); `unwrap()` called
  alone on the open forms before and after (output kept in the scratchpad until the merge).
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
