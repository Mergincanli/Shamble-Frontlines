# kit-v3-5 build notes (the main session), 2026-11-10

Prepared in the session scratchpad (`kb5/`), outside the repo; nothing applied to a kit file yet (each write asks
Mustafa; the guard edit waits for his SA5-RIDE answer). The scratch guard carries C4 (SA5-RIDE (a), recommended); if he
answers (b), C4's lines, rows and mutant come out before the writes.

## The scratch files
- `kb5/hooks/routine-guard.mjs`: part A (CLAUDE.local.md, AGENTS.md in protectedRel, two lines), CANON's two names,
  `mergeProtected` (C0 fold on every platform, C1 protectedRel, C2 `.claude` at any depth, C4 decision docs), the check
  list's `total_count`, the pr-advisor block (cap, `@tsv` list, count, `:` names, C5 both names, C3 `git check-ignore`
  with literal pathspecs off for that one call), `realIo`'s `{ literal }` option, the header comments.
- `kb5/hooks/routine-guard.test.mjs`: `lineCount`, `docsLines`, fakeIo's `totalCount`/`noTotal`,
  `changedFiles`/`noChanged`, `ignored`/`ignoreCode`/`ignoreOut` (check-ignore answers exit 1 by default); 37 merge rows,
  2 night-shift write rows, 5 day rows; 13 routine mutants and 1 day mutant.

## E1, red then green
- Red (the new tests against main's guard, a copy that lists every failing row): 38 rows fail, 34 routine and 4 day:
  every new refused row. Rows that pass on main too: `.env.example`, a docs rename, 2999 files, the escaped tab, and
  `gate still running` (main's :1457 already refuses it; the row pins that line, which had no row or mutant).
- First green run: one mutant survived, "a check still running", because a run with no conclusion is also "not
  green". Fixed: the row now carries the stale case GitHub showed on #44's head (in progress, conclusion success).
- Green (the new tests against the new guard): routine-guard 1078/1078 (main: 1034), no surviving mutant.
- settings.test.mjs 127/127 and checkpoint.test.mjs 27/27, run in a plain `git archive` copy of the tree with the new
  hooks (settings.test.mjs reads `../settings.json` and the root, so it needs the layout).

## E5, live (read-only gh, 2026-11-10)
- PR #1: the new filter prints 54 lines, 51 renamed (new, a tab, old); `changed_files` 54.
- PR #43: 7 lines, `changed_files` 7. PR #44: 15 lines, `changed_files` 15.
- PR #40: exit 1 after 100 lines (`changed_files` 431): "Numeric-ID repository paths (repositories/{id}/...) are not
  supported through this proxy" (HTTP 403). The merge check throws on it and fails closed.

## E9, the guard's own merge check with real git (`kb5/e9/e9.mjs`, CLAUDE_PROJECT_DIR = a5, main's .gitignore)
- docs only: merges; a name with `*`: merges; `.env.example`: merges.
- refused, "which main's .gitignore ignores": `tools/node_modules/acorn/dist/acorn.js`, `tools/Node_Modules/acorn/
  package.json`, `.gate/x.mjs`, `.env`, `dist/a.js`, `notes.log`.
- PR #1's real list: refused (`.claude/agents/art-director.md`); PR #44's: refused (`.claude/CHECKPOINTS.md`); PR #43's:
  refused (`docs/EDITOR_V2.md`, a decision doc, C4).

## Revision 3 build (the allow-list; the scratchpad's `kb6/`, 2026-11-10)
- `kb6/hooks/routine-guard.mjs`: part A and CANON as before; `mergeAllowed` (L1 plain names, L2 no dot or empty
  segment, L3 no `node_modules`, L4 places `docs/`, `tools/`, `README.md`, `FIRST_PROMPTS.md` in exact case, L5-L7 no
  rules, gate or decision-doc name in lower case); the check list's `total_count`; the pr-advisor block (cap, `@tsv`
  list, count, C5 both names, the allow-list). No git call added; `realIo` unchanged.
- `kb6/hooks/routine-guard.test.mjs`: 41 merge rows (L1-L7 each with a row only it refuses; dot, case, Windows-name
  and stream rows; passing rows), 2 night rows, 5 day rows; 14 routine mutants, 1 day mutant.
- Red (the new tests on main's guard): 39 rows fail, 35 routine and 4 day.
- Green: routine-guard 1081/1081 (main 1034), no surviving mutant, on the first run; settings 127/127 and checkpoint
  27/27 in a `git archive` copy of the tree with the new hooks.
- E9 (`kb6/e9/e9.mjs`, decide() with gh faked on live lists): docs only merges; refused: `docs/a*.md`, `.env.example`,
  `tools/node_modules/...`, `tools/Node_Modules/...`, `.gate/x.mjs`, `.env`, `dist/a.js`, `notes.log`, and the real
  lists of PRs #1 (`.claude/agents/art-director.md`), #43 (`docs/EDITOR_V2.md`) and #44 (`.claude/CHECKPOINTS.md`).
- SHA-256 of the two files: `kb6/SHA256SUMS`.

## Revision 3 with the advisor's E1-E5 (the scratchpad's `kb7/`, 2026-11-10): the build that is applied
- The advisor's revision 3 check (appended to `advisor-plan-rev1.md`): APPROVE with E1-E5. E1: `GATE_FILES` gains
  `tools/npm-shrinkwrap.json` (npm reads it before `package-lock.json`; the security-auditor confirms the precedence),
  and so do advisor.md's two gate-file lists. E2: a gate file's name is refused anywhere under `tools/` in a merge, in
  any letter case (this replaces L6's exact-path test, which E2 would otherwise leave a surviving mutant). E3: the
  widened day-session text (advisor.md tier 1 and step 6, CLAUDE.md row 1). E4: the known-limit lines as the advisor
  worded them. E5: STATUS records both 2026-11-10 messages word for word; the report says SA5-RIDE (a) moves most
  docs-keeper PRs (they touch the register) to a day session.
- Red (the new tests on main's guard): 43 rows fail, 39 routine and 4 day; the four new ones among them (a routine may
  write `tools/npm-shrinkwrap.json` today).
- Green: routine-guard 1086/1086, no surviving mutant; settings 127/127 and checkpoint 27/27 in a `git archive` copy.
- The rules drafts: `kb7/role-next.md` (advisor.md: category 3's list, tier 0, tier 1, tier 2 and its bracket, step 2's
  list, step 6; 20 added lines, none over 120 characters) and `kb7/rules-next.md` (CLAUDE.md :64, rows 0 and 1, the line
  under the table; 7 added lines).
- SHA-256 of the four files to apply: `kb7/SHA256SUMS` (guard 6507f795..., tests 240ea071..., advisor.md 9eb2b3ba...,
  CLAUDE.md a7c1f741...).
