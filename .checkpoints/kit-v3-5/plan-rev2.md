# kit-v3-5 (F1) plan, revision 2 (2026-11-10): revision 1 with the advisor's 14 conditions

Revision 1 (`plan-rev1.md`) stands except where this file changes it. The advisor's verdict: `advisor-plan-rev1.md`
(REVISE; R1-R14; conditions 1-14). Each condition is listed below, with the change it makes.

## 1. The merge test, as built (conditions 1, 2, 3, 4 and SA5-RIDE)
`mergeProtected(rel)` is used only by the pr-advisor merge check. Its fold runs on every platform: the merged tree is
checked out on Windows and macOS, not only on the routine's Linux machine.

**Fold.** Each segment that matches a `CANON` name in another case is read as that name. `CANON` gains
`CLAUDE.local.md` and `AGENTS.md` (condition 4), so `protectedFold` also asks on win32 for `docs/claude.local.md`.
`.claude` matches in any case.

**Clauses.** The gitignore clause runs once over all names. Each clause has a row that only it refuses, and a mutant
(condition 3):

| # | Clause | Row that needs it alone (refused) | Mutant |
|---|---|---|---|
| C0 | the fold itself | `docs/Claude.md` | drop the fold (identity) |
| C1 | `protectedRel` of the folded path (with part A's `CLAUDE.local.md`, `AGENTS.md`) | `Tools/gate.mjs`; `agents.md` | `if (false)` on C1 |
| C2 | a `.claude` segment at any depth, any case | `docs/.claude/skills/x/SKILL.md` | `if (false)` on C2 |
| C3 | a path main's `.gitignore` ignores, compared case-insensitively | `tools/node_modules/acorn/dist/acorn.js` | `if (false)` on C3 |
| C4 | the decision docs (only if SA5-RIDE (a)) | `docs/VISION_V2.md`; `docs/adr/0001-x.md`; `docs/DECISIONS.md` | `if (false)` on C4 |
| C5 | both names of a renamed or copied file | rename of `.claude/settings.json` into `docs/settings.json` | read `.filename` only |

**Rows covered by more than one clause** (kept as plain rows):
- `.Claude/rules/x.md` and `.Claude/settings.local.json` (C0, C1, C2);
- `.claude/agent-memory-local/advisor/MEMORY.md` (C2, C3);
- `CLAUDE.local.md` (C1, C3);
- `tools/Node_Modules/acorn/package.json` (C3);
- `.gate/x.mjs`, `.env` and `dist/a.js` (C3).

**Passing rows:**
- `.env.example` (C3's `!` line);
- a docs rename `docs/a.md` to `docs/b.md`;
- a docs PR with matching `changed_files`;
- 2999 matching lines.

**C3's build, choice (a): the guard asks git.**
- **The call:** `git -c core.ignoreCase=true check-ignore --no-index -z -- <every name read>`, run in the routine's
  checkout, which its procedure keeps detached at `origin/main` (the PR's own `.gitignore` never takes part). The
  guard's `io.git` takes no stdin, so the names go after `--` as arguments (with `GIT_LITERAL_PATHSPECS=1`, as for
  every guard git call).
- **Reading the answer:**
  - exit 0: refuse, naming the NUL-separated names git printed;
  - exit 1: none ignored;
  - any other code or an empty answer with exit 0: refuse (fail closed).
- **Why (a) over a fixed list (b):** (a) follows main's own `.gitignore` exactly, including `!` lines, nested files and
  `*.log`. It needs no guard change when `.gitignore` changes, which matters because KIT-FREEZE-2 follows this pass.
- **Its test fake:** `fakeIo`'s git answers `check-ignore` with a configured list.
- **Proof against real git, E9 (new):** the exact command, run in this worktree on the rows above, lists exactly the
  refused names and not `.env.example`.

**C4 (only if SA5-RIDE (a)).** A fixed list: `docs/adr/**`, `docs/VISION_V2.md`, `docs/LOOP_V2.md`, `docs/LOOK_V2.md`,
`docs/EDITOR_V2.md`, `docs/V2_PHASE0_BRIEF.md`, `docs/DESIGN_NOTES.md`, `docs/ART_DIRECTION.md`, `docs/GAME_DESIGN.md`,
`docs/UNITS.md`, `docs/DECISIONS.md`. Folded like the rest.

**Other Windows name aliases (condition 1):** trailing dots and spaces, 8.3 short names such as `CLAUDE~1.MD`, and
`::$DATA`. The security-auditor rules on these in writing: close the cheap ones (for example, strip trailing dots and
spaces before the fold) or record each as a known limit with a severity.

## 2. The lists and counts (conditions 5, 6, 7, 8)
**Check runs (condition 5), for every session.** Deny when `total_count` is not a whole number, or is larger than the
number of runs read.
- `fakeIo` answers `total_count` = the number of runs.
- Rows, refused: 101 reported against 100 read; a missing `total_count`.
- A mutant drops the check.
- Revision 1's risk 4 goes.

**`changed_files` (condition 6).** The order is the cap, then the list read, then the count. Rows:
- refused: missing; 2.5; 3000; more lines than `changed_files`; fewer lines than `changed_files`;
- merged: 2999 matching lines;
- a mutant turns `>=` into `>` on the cap, and the 3000 row catches it.

**Pagination (condition 7).** Keep `--paginate` (it fails closed, and it is the smaller change). Known limit, in STATUS
and the report: "Here a PR of more than 100 files never merges through the PR Advisor: the proxy refuses the next-page
links that gh follows (PR #40, 2026-11-10); the merge check fails closed." E5 becomes:
- the new command on PR #1, where its renames print both names;
- the new command on PR #40: exit 1, refused;
- `changed_files` equals the line count on #1 and #44.

The count check stays for environments where pagination works.

**The `@tsv` parse (condition 8):**
- split each line on the real tab, and never unescape: no protected name holds a tab, newline or backslash, and `/` is
  not escaped;
- count lines after dropping only the final empty line;
- a row with an escaped tab in a name: one file, one line, and the count matches.

## 3. Rules text (condition 9; E6's table carries each line)
- **advisor.md tier 0 (:63-64),** the end of the "Not tier 0" sentence: "... and any `CLAUDE.md`, `CLAUDE.local.md` or
  `AGENTS.md` anywhere, any `.claude/` folder at any depth, and any path `.gitignore` ignores, agent memory included
  (risk tier 2)."
- **advisor.md tier 2 (:71):** "anything in a `.claude/` folder at any depth (agent memory included: it is gitignored,
  so a PR that carries it is tier 2)".
- **advisor.md tier-2 bracket (:76-77):**
  > (the guard refuses the PR Advisor's merge of those it can see: both names of a renamed or copied file, in any letter
  > case, `CLAUDE.local.md` and `AGENTS.md` anywhere, a `.claude/` folder at any depth, any path `.gitignore` ignores,
  > and a file or check list it cannot read whole. It cannot judge a word or decision of Mustafa's, so still read the
  > `--no-renames` list yourself)

  With SA5-RIDE (a), "the decision docs" joins the list.
- **CLAUDE.md:64:** `CLAUDE.local.md` and `AGENTS.md` join the protected files. Nested `.claude/` and gitignored paths
  are not named there: day-session asks do not cover them (MERGE-ONLY).
- **F4's row 0 (CLAUDE.md:97):**
  > docs that add or change no word or decision of Mustafa's; not ADRs, the register, design or look docs; not a
  > `CLAUDE.md`, `CLAUDE.local.md`, `AGENTS.md`, `.claude/` folder, agent memory or other gitignored path, or a file
  > renamed or moved from outside tier 0

  Then one sentence under the table: "Steps 4 and 5 apply at every tier; decision docs always need his go."
- **The guard's header comment (:5-6) and the comment at :31** describe the new checks.

## 4. The docs part (conditions 10, 11, 12, 13)
- **Write-side limits (condition 10)**, in STATUS's limits and the report, each with the security-auditor's severity,
  deferred under KIT-FREEZE-2:
  - a day subagent or a routine may write a nested `<dir>/.claude/` file, or a gitignored path other than the
    protected names, without an ask;
  - `kitDir` (:120) checks a folder for `CLAUDE.md` only.
- **F5 (condition 11)** is not in this pass. Its STATUS line ends: "Waits for the foundation's gate step (KIT-FREEZE-2,
  2026-11-10)." F2 gets the same ending. F1 is carried out; F4 rides (§3); F3 is docs-keeper's and rides this docs
  part.
- **The kit-v3-4 report correction (condition 12)** is appended at the end as "Correction, 2026-11-10, after the
  merge": the advisor's two Lows and its agent-memory finding. E7 shows the report's diff adds lines only, so his words
  in its appendix stay byte for byte.
- **Small text (condition 13):**
  - one ask per kit-file edit call;
  - the reviewers run Opus 5.5 at high;
  - STATUS:793's `enabled: true` becomes "paused by Mustafa, 2026-11-09 18:22 UTC, until kit-v3-5 merges;
    re-enabling is his";
  - his one-line conflict answer was given in chat on 2026-11-10: LANE-RULES' Fable line amends CLAUDE.md's "No other
    model". The report records it.

## 5. Build order (condition 14)
1. SA5-RIDE goes to him now.
2. Everything is prepared in a scratch folder: the test file (rows and mutants, red against today's guard), the guard
   file, and the rules lines.
3. After his answer, the writes, one per kit file, each an ask:
   - the test file, then a red run recorded (E1);
   - the guard file, then green, with mutants and E2;
   - advisor.md;
   - CLAUDE.md.
4. Then the docs, the gate, the review round and the report.

## 6. Evidence and reviews: changes to revision 1
- **E5:** as in §2 (PRs #1, #40 and #44).
- **E9 (new):** the real `git check-ignore` call on the C3 rows (§1).
- **Reviews:** unchanged (security-auditor, test-engineer and the verifier; Opus 5.5 at high). Every security-auditor
  finding is addressed or deferred in writing (R2's merge-overwrite claim to be confirmed from git's docs). The
  security-auditor also rules on the Windows aliases (§1).
- **Rulings recorded:**
  - F4-RIDE: fold it in, with §3's text.
  - MERGE-ONLY: approved, with the write-side limits recorded (§4).
  - SA5-RIDE: his question, in the advisor's rewritten text.
