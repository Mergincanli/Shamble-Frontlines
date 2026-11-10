# kit-v3-5 E6: the rule-diff table (every changed rule, main 44446a5 against kit commit 9bacf47)

Verdicts: looser, stricter or same. Nothing is looser.

The guard's rules (`.claude/hooks/routine-guard.mjs`):

| # | Rule | Before (main) | After | Verdict | Why |
|---|---|---|---|---|---|
| 1 | The PR Advisor's merge test | refuses a path `protectedRel` protects (a deny-list; agent memory and `.claude/worktrees/<x>/` content exempt) | `mergeAllowed`: only plain paths with no dot or empty segment and no `node_modules`, in `docs/`, `tools/`, `README.md`, `FIRST_PROMPTS.md`, and no rules, gate or decision-doc name in any letter case | stricter | His allow-list direction (2026-11-10) and SA5-RIDE (a). Every path `protectedRel` refused is refused (L2, L5 and L6), and so are the exempt ones (L2) |
| 2 | The file list | `.filename` only; the count unchecked | both names of a renamed or copied file (`@tsv`); refused at 3000 or more, or when the line count is not `changed_files` | stricter | F1 (SA-1, SA-8); revision 2 §2 |
| 3 | Every merge's check list | the first 100 check runs, count unchecked | refused when `total_count` is not a whole number or exceeds the runs read | stricter | The advisor's R5 |
| 4 | `protectedRel` | `CLAUDE.md`, `.npmrc`, `.mcp.json` by name | also `CLAUDE.local.md` and `AGENTS.md` by name | stricter | Routines may not write them; day sessions ask (part A) |
| 5 | `CANON` | no `CLAUDE.local.md`, `AGENTS.md` | both, so win32 and darwin fold them | stricter | The advisor's R4 |
| 6 | `GATE_FILES` | six files | adds `tools/npm-shrinkwrap.json` | stricter | The advisor's E1 (npm reads it before `package-lock.json`) |

The rules text:

| # | File and place | Before | After | Verdict | Why |
|---|---|---|---|---|---|
| 7 | advisor.md, escalation category 3's gate-file list | ends with `tools/package-lock.json` | adds `tools/npm-shrinkwrap.json` | stricter | E1 |
| 8 | advisor.md, tier 0 "Not tier 0" | "... and any `CLAUDE.md`, `CLAUDE.local.md` or `AGENTS.md` anywhere (risk tier 2)" | adds "any `.claude/` folder at any depth, and any path `.gitignore` ignores, agent memory included" | stricter | Revision 2, condition 9 |
| 9 | advisor.md, tier 1 | (no merge-path sentence) | "Only plain paths in `docs/`, `tools/`, `README.md` and `FIRST_PROMPTS.md`, with no rules, gate or decision-doc name, can merge through the PR Advisor (the guard's allow-list); every other PR merges from a day session on his go." | stricter | E3; his allow-list and SA5-RIDE (a) |
| 10 | advisor.md, tier 2's file list | "anything in `.claude/` (agent memory aside)" | "anything in a `.claude/` folder at any depth (agent memory included: it is gitignored, so a PR that carries it is tier 2)" | stricter | Revision 2, condition 9 |
| 11 | advisor.md, tier 2's bracket | "the guard refuses the PR Advisor's merge of those it can see; it does not yet see a renamed-away path, `CLAUDE.local.md` or `AGENTS.md`, so read the `--no-renames` list yourself" | the guard's allow-list described as built; "It cannot judge a word or decision of Mustafa's, so still read the `--no-renames` list yourself" | same | Describes row 1; the advisor's own reading stays |
| 12 | advisor.md, step 2's gate-file list | `package*.json`, any `.npmrc` | adds `npm-shrinkwrap.json` | stricter | E1 |
| 13 | advisor.md, step 6 (APPROVE + needs-mustafa) | "merge the PR if every check is green" | adds "(if the guard's allow-list admits it; otherwise say it merges from a day session)" | same | Describes the guard's refusal; gives no seat a new power |
| 14 | CLAUDE.md:64, the guard's protected files | "`.claude/` except agent memory, every `CLAUDE.md`, `.github/`, `tools/gate.mjs` and the other gate files" | adds "`CLAUDE.local.md` and `AGENTS.md`" | stricter | Part A (row 4) |
| 15 | CLAUDE.md, risk-tier row 0 | "docs with no word or decision of Mustafa's; not ADRs, the register, design or look docs" | "docs that add or change no word or decision of Mustafa's; ...; not a `CLAUDE.md`, `CLAUDE.local.md`, `AGENTS.md`, `.claude/` folder, agent memory or other gitignored path, or a file renamed or moved from outside tier 0" | stricter | F4-RIDE (the advisor), revision 2's text |
| 16 | CLAUDE.md, risk-tier row 1, Merges | "`ready`: the PR Advisor; else his go" | adds "(outside the guard's allow-list: from a day session)" | stricter | E3 |
| 17 | CLAUDE.md, under the tier table | (none) | "Steps 4 and 5 apply at every tier; decision docs always need his go." | same | Restates advisor.md's steps 4 and 5 and its decision-doc rule (F4) |
