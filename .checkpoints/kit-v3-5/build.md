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
