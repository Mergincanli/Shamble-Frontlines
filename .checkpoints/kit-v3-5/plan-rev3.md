# kit-v3-5 (F1) plan, revision 3 (2026-11-10): an allow-list for the PR Advisor's merges

His answers of 2026-11-10 (`mustafa-2026-11-10-answers.md`) give three directions:
- SA5-RIDE (a), "as the fixed list in your plan and nothing more";
- switch to an allow-list if it is smaller than clauses C0-C5;
- do not build for Windows short names, `::$DATA`, or trailing dots and spaces. Each is a known limit.

The session's five-line answer, given in chat: yes, the allow-list is smaller. This revision replaces revision 2 §1
(the clause table C0-C5 and the `git check-ignore` call). Everything else in revisions 1 and 2 stands unless changed
here. The review budget is as in A4: two rounds at most, and Lows never trigger a push.

## 1. The merge test, as built: `mergeAllowed(rel)`
Only the pr-advisor merge check uses it, on both names of every changed file. A name is allowed only if all hold:

| # | Rule | Refused row that needs it alone | Mutant |
|---|---|---|---|
| L1 | plain: only `A-Z a-z 0-9 _ - . /` | `docs/a b.md` | drop the test |
| L2 | no empty segment and no segment that starts with `.` | `docs/.claude/skills/x/SKILL.md` | drop the test |
| L3 | no `node_modules` segment, any letter case | `tools/node_modules/acorn/dist/acorn.js` | drop the test |
| L4 | in an allowed place: `docs/`, `tools/`, `README.md`, `FIRST_PROMPTS.md` (exact case) | `dist/a.js`; `shamble-game.js` | drop the test |
| L5 | not a rules name in any letter case: `CLAUDE.md`, `CLAUDE.local.md`, `AGENTS.md` | `docs/Claude.md` | drop the test |
| L6 | not a gate file in any letter case (`GATE_FILES`) | `tools/Gate.mjs` | drop the test |
| L7 | not a decision doc in any letter case: `docs/adr/**` and SA5's fixed list | `docs/Vision_V2.md`; `docs/adr/0001-x.md` | drop the test |

- **Case.** L5-L7 compare in lower case on every platform, because the merged tree is checked out on Windows and macOS.
  L4's places are exact, so `Docs/x.md` is refused as not a place. Case-folding them would only let more through.
- **Rows covered more than once** (kept as plain rows): `.claude/settings.json`, `.Claude/rules/x.md`, `.env`,
  `.gate/x.mjs` and `.github/workflows/x.yml` (L2, L4); `tools/Node_Modules/x/package.json` (L3); `:(exclude)x.md`,
  `CLAUDE~1.MD` and `a::$DATA` (L1, L4); `CLAUDE.local.md` and `AGENTS.md` at the root (L4, L5); `docs/DECISIONS.md`,
  `docs/adr/README.md` (L7).
- **Passing rows:**
  - `docs/a.md`, `tools/x.mjs`, `README.md` and `FIRST_PROMPTS.md`;
  - a docs rename;
  - 2999 matching lines;
  - `docs/a-b_c.1.md`.
- **Both names (C5), kept.** The `@tsv` list and its parse are as in revision 2 §2. Row: `docs/settings.json` renamed
  from `.claude/settings.json`; its mutant reads the new name only.
- **The lists and counts, kept from revision 2 §2:** the 3000 cap (`>=`, with 3000 matching lines), the whole list
  (`changed_files` against the line count), the check runs' `total_count`, and the still-running row (the stale case).
- **What goes, from revision 2:**
  - C0-C3 and `mergeProtected`;
  - the `git check-ignore` call, `realIo`'s `literal` option, the `:` refusal and its exit-code rows;
  - E9's git part. No new git call is added, so the advisor's D1, D2 and D6 fall away.
- **Kept from revision 1, for writes and day-session asks:** part A (`protectedRel` gains `CLAUDE.local.md` and
  `AGENTS.md`, `CANON` gains both, and their night and day rows and mutants).

## 2. Known limits, one line each (in STATUS and the report)
- Windows 8.3 short names (`CLAUDE~1.MD`): not handled; L1 happens to refuse `~`.
- NTFS streams (`::$DATA`): not handled; L1 happens to refuse `:` and `$`.
- Trailing dots and spaces: not handled. L1 refuses spaces; a trailing dot passes, so `docs/CLAUDE.md.` would load as
  `docs/CLAUDE.md` on Windows. Forks are skipped, so only someone with push access could open such a PR.
- A future `.gitignore` entry inside an allowed place (such as `tools/dist/`) is not refused; a `.gitignore` change
  needs a look at the list.
- The foundation's restructure: `legacy/` and the v2 folders are not allowed places. Their PRs wait for his go until
  the foundation's gate step adds them, which is one list edit, made when KIT-FREEZE-2 lifts.
- Here a PR of more than 100 files never merges through the PR Advisor: the proxy refuses the next-page links that gh
  follows (PR #40, 2026-11-10), and the merge check fails closed.
- The write-side limits (revision 2 §4, condition 10): unchanged and recorded.

## 3. Rules text: changes to revision 2 §3
- **The advisor.md tier-2 bracket:**
  > (the guard lets the PR Advisor merge only plain paths in `docs/`, `tools/`, `README.md` and `FIRST_PROMPTS.md`,
  > read with both names of a renamed or copied file and with whole lists, and never a rules, gate or decision-doc name
  > in any letter case; everything else merges from a day session on his go. It cannot judge a word or decision of
  > Mustafa's, so still read the `--no-renames` list yourself)
- **Unchanged:** advisor.md tier 0 (:63-64) and tier 2 (:71), CLAUDE.md:64, F4's row 0, the line under the table, and
  D5's tier-1 sentence and row-1 cell.
- **The guard's comments** describe the allow-list.

## 4. Evidence and reviews: changes
- **E1 (red, then green)** and **E2 (mutants)**: as before, on the new rows.
- **E5, live:** the `@tsv` reads of PRs #1, #40 and #44 stand.
- **E9 becomes:** the new guard's merge check, through `decide` with gh faked, on PR #1's and PR #44's real lists
  (refused) and on a docs-only list (merges).
- **Reviews:** one round, as planned. security-auditor rules on the allow-list against an attacker who can open a
  same-repository PR. SA5-RIDE rides that round: it is one list in the same check, so it needs no round of its own.

## 5. Build order
1. The scratch build is redone, red then green, and the advisor checks this revision (not a review round).
2. Then four kit writes, one ask each:
   - the test file, then a red run in the worktree;
   - the guard, then green;
   - advisor.md;
   - CLAUDE.md.
3. Then the docs, the gate, the review round and the report.
