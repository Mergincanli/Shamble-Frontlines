# Advisor: kit-v3-5 plan revision 1 (F1, the guard's rename gap, agent memory included)

Reviewed 2026-11-10 from main's `.claude/agents/advisor.md` and `CLAUDE.md` (origin/main 44446a5), on
`.checkpoints/kit-v3-5/plan-rev1.md` (f6be38e), `facts.md`, his words (`mustafa-2026-11-09-answers.md`,
`mustafa-2026-11-10-queue.md`), STATUS "From kit-v3-4" (:560-585), SA-1/SA-5/SA-8 at d96e215, and the code
(`routine-guard.mjs` :31-43, :65-73, :1389-1467; `routine-guard.test.mjs` :35-92, :276-281, :476-567).

## Verdict: REVISE

The plan is the right shape: right scope (F1 plus his "agent memory included"), right tier (2), right reviewers,
red-then-green evidence with mutants, and the MERGE-ONLY split is correct. But read as an attacker who can open a PR
the routine would merge, it leaves two ways around the merge check open (R1, R2), and two of its own evidence claims
would fail as written (R3, R4). The fixes are additive and precise, so revision 2 needs only a delta check from me
(plan reviews are not review rounds). Fold in the conditions below; then build.

Risk tier: 2 (the guard, `.claude/`, `CLAUDE.md`). needs-mustafa: his go on the final head, from a day session.

## What I checked myself (read-only)
- The merge check reads `.filename` only and matches it with `protectedRel`, case-sensitively
  (`routine-guard.mjs:1461-1463`); `protectedRel` exempts `.claude/agent-memory-local` (:38) and checks `.claude` as
  the first segment only (:37). `CANON` (:70-71), used only by `protectedFold` on win32/darwin (:65-73), has no entry
  for `CLAUDE.local.md` or `AGENTS.md`.
- The check-runs read is one page (`per_page=100`, :1453) and never compares with the response's `total_count`.
- Mutants are caught only by `CASES` rows, and each target text must occur exactly once (test :545-563).
- Live, read-only, through this session's `gh` (2.89.0):
  - PR #1: 54 files, 51 `renamed`; the plan's `--jq '.[] | [.filename, (.previous_filename // "")] | @tsv'` prints
    `new<TAB>old` per line (e.g. `.claude/agents/art-director.md<TAB>shamble-frontlines-repo/.claude/agents/art-director.md`).
    So E5 has a real rename PR: #1.
  - PR #44 (15), #28 (14), #26 (31): `changed_files` equals the number of `@tsv` lines.
  - PR #40 (426 files): `gh api --paginate .../pulls/40/files?per_page=100` prints page 1 (100 lines), then fails with
    exit 1: GitHub's `Link` header names `repositories/<numeric id>/...`, which this environment's proxy refuses (HTTP
    403). An explicit `...&page=2` on the `repos/...` path answers 100. So in this environment any PR over 100 files
    already fails closed in the merge check (:1462), and "`--jq` per page under `--paginate`" (plan risk 3) cannot be
    tested live past page 1 here.
  - `gh api rate_limit --jq '["a\tb\nc\\d", ""] | @tsv'` prints `a\tb\nc\\d` then a real tab then a newline: gh's jq
    escapes tab, newline and backslash inside a field, and an empty second field leaves a trailing tab.
- `.gitignore` (main): `node_modules/`, `.gate/`, `dist/`, `.env`, `.env.*`, `*.log`, `.claude/settings.local.json`,
  `.claude/agent-memory-local/`, `.claude/claude-security-guidance.local.md`, `CLAUDE.local.md`. No `AGENTS.md`,
  `CLAUDE.local.md` or `node_modules` file is tracked today.
- `tools/gate.mjs:63` and `:291` `require` `acorn`, `acorn-walk`, `eslint-scope` and `three` from `tools/node_modules`.

Not checked: whether Claude Code runs on Mustafa's Windows machine (STATUS:188 leaves it open); whether the routine's
environment has the same proxy (very likely, not seen); git-for-Windows' handling of trailing dots, spaces and 8.3
short names; the git-merge manual's `--overwrite-ignore` sentence (from memory, not fetched in this review: the
security-auditor confirms it). I did not run the test suites (plan stage; facts §3 records 1034 / 127 pass).

## Findings (CLAUDE.md severity scale)

**R1 (High; Critical if a case-insensitive checkout runs Claude Code) Letter case.** The merge check compares names
case-sensitively, so `.Claude/rules/x.md`, `.Claude/settings.local.json`, `docs/Claude.md`, `agents.md` or
`Tools/gate.mjs` pass it. On Windows and default macOS volumes these land in, or open as, the protected files. The kit
itself treats those platforms as case-folding (:65-67, S-3). A new name with no lower-case twin (for example
`.Claude/rules/zz.md` or `.Claude/settings.local.json`) lands inside `.claude/` on such a checkout, and a settings file
there can add hooks, which run code. This is a way around the merge check wherever such a checkout exists.

**R2 (Critical) Agent memory is one case of a wider class: any gitignored path.** No legitimate PR carries a file that
main's `.gitignore` ignores, yet the plan refuses only `.claude/agent-memory-local/`. The worst case is code: a PR that
force-adds `tools/node_modules/acorn/dist/acorn.js` (or any package the gate requires, gate.mjs:63, :291) passes the
merge check. CI stays green, because `npm ci` deletes `node_modules` first. But on every machine that already ran
`npm ci`, the next `git pull` silently overwrites that ignored file (git's default, `--overwrite-ignore`; the
security-auditor confirms the source), and the next `node tools/gate.mjs`, which every session runs, executes it. His
"agent memory included" is the instance; the class is "gitignored", and it costs one rule.

**R3 (Medium) A mutant that must survive.** `mergeProtected` as written has three clauses, and the second
(`.claude/agent-memory-local/`) is fully covered by the third (any segment `.claude`). The mutant that removes the
second clause therefore survives, so E2 ("no survivors") fails. Write each clause so that some row needs it alone.

**R4 (Medium) `CANON` misses the new names.** Part A protects `CLAUDE.local.md` and `AGENTS.md`, but `protectedFold`
maps only the names in `CANON` (:70-71). On win32 and darwin, a day-session edit of `claude.local.md` or `Agents.md`
would not ask. Part A's "stricter for day-session edits" holds only if both names join `CANON`.

**R5 (Medium) More than 100 check runs.** :1453 reads one page. A failing `gate` run beyond page 1 is never seen. The
fix is one line in the same function (deny when `total_count` is not a whole number or exceeds the runs read), and the
freeze follows this pass, so it rides here, not as a follow-up.

**R6 (Medium) Evidence E5 and plan risk 3 need the live facts above.** Pagination beyond page 1 fails closed here (PR
#40), and the `@tsv` behaviour is now seen. Record both; use PR #1 for the rename read.

**R7 (Medium) Part D's rows are incomplete.** E lists rows for `changed_files` = 3000 and for a list shorter than
`changed_files`. Part D also refuses a missing or non-integer `changed_files`, and "not equal" works both ways; the
`>=` boundary needs a passing row at 2999, or a `>=` to `>` mutant survives.

**R8 (Medium) The advisor.md replacement text drops an instruction.** Plan F turns the tier-2 bracket into "the guard
reads both names ... and refuses ...", which deletes "so read the `--no-renames` list yourself". The guard still
cannot judge his words or the decision docs (SA-5's judgment half), so the instruction stays. The tier-0 and tier-2
lines must also name what the guard now refuses (nested `.claude/`, gitignored paths), or the rules and the guard
disagree again (facts Tension 1).

**R9 (Low) F4's row 0 says "or a rename".** advisor.md:63 says "a file renamed or moved from a path outside tier 0".
"A rename" alone would put a docs-to-docs rename outside tier 0, while plan E's own row merges that rename. Match
advisor.md's wording.

**R10 (Low) "F4 and F5 by the ruling" (plan G).** The plan rules F4 only. F5 is not about the guard.

**R11 (Low) Ask count.** "Four asks" understates it: every Edit or Write call on a kit file asks him. Say "one ask per
kit-file edit call", and batch each file's changes into as few calls as possible.

**R12 (Low) Effort is not stated.** The reviewers run Opus 5.5 at `high` (their frontmatter: security-auditor.md:6,
test-engineer.md:6, verifier.md:6). Say so.

**R13 (Low) STATUS:793 also says `enabled: true`** ("Mustafa un-paused it"), which is now false (paused by him
2026-11-09 18:22 UTC, checkpoint). Plan G names only the "no MCP connections" half.

**R14 (Low) His queue asks for a one-line answer** ("Tell me in one line if any of this conflicts with a ruling I
already gave."). Neither the checkpoint nor the plan records one. A candidate conflict: LANE-RULES' Fable 5.2 line
against CLAUDE.md "Models and effort" ("No other model"). His new words would win, but he asked to be told.

## Conditions for revision 2 (each is a change to the plan)

1. **Fold case in the merge check (R1).** `mergeProtected` matches every segment case-insensitively, on every
   platform: the merged tree is checked out on Windows and macOS. Use `CANON` (with the new names) for the fold. Add
   refused rows: `.Claude/rules/x.md`, `.Claude/settings.local.json`, `docs/Claude.md`, `agents.md`, `Tools/gate.mjs`.
   Add a mutant that drops the fold. Ask the security-auditor to rule, in writing, on the other Windows name aliases
   (trailing dots and spaces, 8.3 short names such as `CLAUDE~1.MD`, `::$DATA`): close the cheap ones, or record
   them as known limits with a severity.
2. **Refuse every gitignored path in a merge (R2).** Replace the agent-memory clause with "any path main's
   `.gitignore` ignores, compared case-insensitively". Two acceptable builds; the plan names its choice and why:
   - (a) the guard asks git, with main's `.gitignore` from the routine's `origin/main` checkout:
     `git -c core.ignoreCase=true check-ignore --no-index -z ...`, reading the listed names (not the exit code alone);
     any error refuses;
   - (b) a fixed list in code, plus a test that fails when a `.gitignore` line has no matching rule (no drift).
   Refused rows at least: `.claude/agent-memory-local/advisor/MEMORY.md`, `tools/node_modules/acorn/dist/acorn.js`,
   `tools/Node_Modules/acorn/package.json`, `.gate/x.mjs`, `.env`, `dist/a.js`. Passing row: `.env.example`. Writes
   stay as they are (MERGE-ONLY).
3. **Every clause needed alone (R3).** After 1 and 2, list `mergeProtected`'s clauses, the row that needs each one
   alone, and its mutant. E2 stays "no survivors".
4. **`CANON` gains `CLAUDE.local.md` and `AGENTS.md` (R4),** with a win32 day-session row: `docs/claude.local.md`
   asks. Keep the existing fold rows passing.
5. **Check runs (R5).** Deny, for every session (the read is for every session today), when `total_count` is not a
   whole number or is larger than the number of runs read. `fakeIo` answers `total_count` = the number of runs.
   Rows: 101 runs reported against 100 read (refused); a missing `total_count` (refused). A mutant that drops it.
   Remove plan risk 4.
6. **Part D's rows (R7):** `changed_files` missing (refused), `changed_files` = 2.5 (refused), more lines than
   `changed_files` (refused), and 2999 matching lines (merged). Mutant `>=` to `>` on the cap.
7. **Pagination, written down (R6).** Keep `--paginate` (fail-closed and the smaller change). Record as a known limit
   in STATUS and the report: "Here a PR of more than 100 files never merges through the PR Advisor: the proxy refuses
   the next-page links that gh follows (PR #40, 2026-11-10); the merge check fails closed." E5 becomes: the new
   command on PR #1 (renames, both names printed) and on PR #40 (exit 1, refused), plus `changed_files` = lines on #1
   and #44. Keep the count check (part D) for environments where pagination works: it also catches a filter that
   runs other than per page.
8. **The `@tsv` parse, stated.** Split each line on the real tab; never unescape (no protected name contains a tab,
   newline or backslash, and `/` is not escaped); count lines after dropping only the final empty line. Test a name
   with an escaped tab: one file, one line, count matches.
9. **Rules text (R8, R9).** Use these, adjusted only to the final checks, and put each in E6's table:
   - advisor.md tier 0 (:63-64), the end of the "Not tier 0" sentence: "... and any `CLAUDE.md`, `CLAUDE.local.md`
     or `AGENTS.md` anywhere, any `.claude/` folder at any depth, and any path `.gitignore` ignores, agent memory
     included (risk tier 2)."
   - advisor.md tier 2 (:71): "anything in a `.claude/` folder at any depth (agent memory included: it is
     gitignored, so a PR that carries it is tier 2)".
   - advisor.md tier 2 bracket (:76-77): "(the guard refuses the PR Advisor's merge of those it can see: both names of
     a renamed or copied file, in any letter case, `CLAUDE.local.md` and `AGENTS.md` anywhere, a `.claude/` folder at
     any depth, any path `.gitignore` ignores, and a file or check list it cannot read whole. It cannot judge a word or
     decision of Mustafa's, so still read the `--no-renames` list yourself)". If he answers SA5-RIDE (a), add "the
     decision docs" to the list.
   - CLAUDE.md:64: as the plan says (`CLAUDE.local.md` and `AGENTS.md` join the protected files). Do not name nested
     `.claude/` or gitignored paths there: day-session asks do not cover them (MERGE-ONLY).
   - F4's row 0 (CLAUDE.md:97): "docs that add or change no word or decision of Mustafa's; not ADRs, the register,
     design or look docs; not a `CLAUDE.md`, `CLAUDE.local.md`, `AGENTS.md`, `.claude/` folder, agent memory or other
     gitignored path, or a file renamed or moved from outside tier 0". Then one sentence under the table: "Steps 4 and
     5 apply at every tier; decision docs always need his go."
   - The guard's header comment on the merge check (:5-6), and the comment at :31, describe the new checks.
10. **The write-side limits MERGE-ONLY leaves (MERGE-ONLY, below).** STATUS's limits and the report name them, each
    with the security-auditor's severity: a day subagent or a routine may write a nested `<dir>/.claude/` file or a
    gitignored path (other than the protected names) without an ask; `kitDir` (:120) checks a folder for `CLAUDE.md`
    only. Under KIT-FREEZE-2 they wait for the foundation's gate step.
11. **F5 (R10):** not in this pass. STATUS's F5 line ends: "Waits for the foundation's gate step (KIT-FREEZE-2,
    2026-11-10)." The same for any other F-item left out.
12. **The kit-v3-4 report correction** is a dated section appended at the end ("Correction, 2026-11-10, after the
    merge"): the two Lows and the agent-memory finding. His words in its appendix stay byte for byte; E7 shows the
    report's diff is additions only.
13. **Small text fixes (R11-R14):** one ask per kit-file edit call; the reviewers' model and effort (Opus 5.5, high);
    STATUS:793's `enabled: true` becomes "paused by Mustafa, 2026-11-09 18:22 UTC, until kit-v3-5 merges; re-enabling
    is his"; and the report (or the next chat update) gives his one-line conflict answer, naming the LANE-RULES model
    line if the session agrees it conflicts.
14. **Build order:** ask SA5-RIDE now. The tests (red) are written meanwhile; the guard edit waits for his answer, so
    that one guard edit carries everything.

## Answers to the questions put to me

**1. The plan as a whole:** REVISE, with conditions 1-14. Revision 2 gets a delta check only.

**2. Do parts A-E close the five gaps, with nothing new around them?** Gaps 1-5 of plan §1: yes, as written, once the
rows land. Missing: letter case (R1), gitignored paths beyond agent memory (R2), more than 100 check runs (R5). The
items you named:
- nested `.claude/agents/`, `commands/`, `settings.json`: covered by "any segment `.claude`" in merges, whether or not
  Claude Code loads them. That is the right rule (refuse unknowns).
- `--jq` per page under `--paginate`: moot here (it fails closed after page 1, live on #40). Part D's count check
  covers other environments.
- `@tsv`: safe (seen live). Escaping touches only tab, newline, CR and backslash, none of which occur in a protected
  name; each file stays one line, so the count holds (condition 8).
- `removed`: `.filename` is the deleted path, already checked today. `copied`: `previous_filename` is the source;
  refusing a copy of a protected file is strict and harmless. `changed`/`unchanged`: `.filename`.
- Also seen, no condition: the file list is computed by GitHub for the PR's current head, not for the SHA passed. A
  push in between makes the merge fail on `expectedHeadSha`, so only a stale list on GitHub's side could matter.
  security-auditor: say whether it needs a limit line.

**3. Decisions.**
- **F4-RIDE (mine): fold it in,** with condition 9's text. The freeze follows, CLAUDE.md is written in this pass
  anyway, every change is on the strict side, and he reconfirms the head (tier 2).
- **MERGE-ONLY (mine): approved.** Refusing agent memory or nested `.claude/` for writes would break the eleven
  `memory: local` agents and the worktree memory paths (rows :171, :681, :757, :1756-1757; facts §2), and it would
  widen this pass under a freeze. The write-side gaps become recorded limits (condition 10).
- **SA5-RIDE (his): I agree with (a),** but the question as written leaves out what "yes" changes. Today a decision-doc
  PR is needs-mustafa, and the routine merges it after his go in the routine's own chat. Under (a), the guard refuses
  that merge, so decision-doc PRs merge only from a day session on his go, like kit files. That is a change to a merge
  path, and it is why the question is his. Rewrite it (text below). advisor.md tier 1 then gains: "decision-doc PRs
  merge from a day session on his go (the guard refuses the PR Advisor's merge of them)". The list is matched
  case-insensitively (condition 1), and naming a new decision doc later is a kit change, frozen until the gate step.

**4. Evidence and reviewers:** E1-E8 fit a tier-2 logic change, with E5 replaced (condition 7) and E2 fixed (condition
3). One round of security-auditor, test-engineer and the verifier (3 agents, one batch) is enough and in budget; the
verifier's re-runs are outside it. Give the security-auditor R1, R2, the Windows aliases and the stale-list question;
give test-engineer `fakeIo`'s fidelity (`@tsv`, `total_count`, `changed_files`).

**5. Docs part G:** right, plus conditions 11-13. STATUS F1's text (:562-568) must be rewritten as carried out, not
left with "Whether A4 waits for F1 is his open GUARD-RENAME": it is answered (c). STATUS F4 must be marked done here.
The queue entries go in word for word (E7).

**6. What a tier-2 plan still lacks:** the effort (condition 13). The rest is present: tier named, security-auditor
named, checkpoint kept (`.checkpoints/kit-v3-5.md`), 3 agents at most, his go on the final head, the rule-diff table
(E6), kit test counts before and after (E3). The tier-2 checklist also wants each security-auditor finding addressed
or deferred in writing; the plan should say so in §6.

## Decisions needed (for Mustafa)

**SA5-RIDE, rewritten.** Should the guard also stop the PR Advisor from merging the decision docs?
- Why it matters: today only the advisor's judgment keeps the PR Advisor from merging an ADR or a design or look doc
  without your go. The guard is being changed now, and after this pass the kit freezes until the foundation's gate
  step.
- (a) Yes. The guard refuses the PR Advisor's merge of `docs/adr/**`, `VISION_V2`, `LOOP_V2`, `LOOK_V2`, `EDITOR_V2`,
  `V2_PHASE0_BRIEF`, `DESIGN_NOTES`, `ART_DIRECTION`, `GAME_DESIGN`, `UNITS`, and the coming `docs/DECISIONS.md`.
  Those PRs then merge only from a day session on your go, never from the routine, even after your go in its chat. A
  quote of you in any other doc stays the advisor's judgment. Naming a new decision doc later is a kit change.
- (b) No. Judgment only, as now.
- Recommendation: (a). It is strictly safer, it costs one list and a few test rows, and it only moves where those
  merges happen.

## For Mustafa
1. Answer SA5-RIDE: (a) or (b).
2. Nothing else now. Later in this change-set: approve each kit-file edit as it asks; give your go on the final head;
   then re-enable the PR Advisor routine yourself.

---

# Revision 2 delta check (2026-11-10, on `plan-rev2.md` at 890ed2d)

## Verdict: APPROVE, with conditions D1-D6

Revision 2 meets conditions 1-14 in substance. But C3's git call as written fails on every run (exit 128, three
separate ways), so the PR Advisor routine would refuse every merge. That fails closed, so it is safe, but it is
broken. The test fake would hide it. The fixes are mechanical and are checked at the final-head review and by E9.
No revision 3 is needed.

## What I ran (read-only, in this worktree, git 2.43.0; `tools/node_modules` absent, as in the routine's checkout)
- The plan's exact form, `git -c core.ignoreCase=true check-ignore --no-index -z -- <names>`, prints
  `fatal: -z only makes sense with --stdin`, exit 128.
- Without `-z`, but with `GIT_LITERAL_PATHSPECS=1`, which `realIo()` sets on every guard git call
  (`routine-guard.mjs`, `run()` in `realIo`), it prints `fatal: <name>: pathspec magic not supported by this command:
  'literal'`, exit 128. The same happens through `--stdin -z`.
- An empty name, which is what a non-rename line's empty second field gives, prints
  `fatal: empty string is not a valid pathspec`, exit 128.
- With `GIT_LITERAL_PATHSPECS=0` and no `-z`, all the rows behave right:
  - Exit 0, listing: `.claude/agent-memory-local/advisor/MEMORY.md`, `tools/node_modules/acorn/dist/acorn.js`,
    `tools/Node_Modules/acorn/package.json`, `.gate/x.mjs`, `.env`, `dist/a.js`, `CLAUDE.local.md`,
    `docs/CLAUDE.local.md`, `.Claude/settings.local.json`, `Claude.local.md` and
    `.Claude/agent-memory-local/a/MEMORY.md`. So directory patterns match even when the folder does not exist, and
    `core.ignoreCase=true` folds case.
  - `docs/a.md` plus `.env.example` gives exit 1.
  - Without `core.ignoreCase`, `tools/Node_Modules/x.js` gives exit 1, so the `-c` flag is needed.
  - Output quotes unusual names (`"docs/a\\\\tb.log"`), so use it for the message only.
  - Pathspec-magic names: `:(exclude)...` and `:!...` exit 128; `:(top)x` is read as magic (exit 1).
- C4's list matches advisor.md's decision docs (:60-63) exactly. All nine files exist on main, and `docs/adr/` holds
  the register (`README.md`), the template and 0001-0006. `docs/DECISIONS.md` does not exist yet, which is fine: it is
  listed for STATUS-DIET.

## Conditions 1-14
| # | Met? | Note |
|---|---|---|
| 1 letter case | met | rows, fold mutant and the Windows aliases go to the security-auditor; C4's fold: D5 |
| 2 gitignored paths | met in design; the build fails | D1, D2 |
| 3 each clause alone | met, with D3 and D4 | the table holds: see below |
| 4 CANON | met | |
| 5 check runs | met | |
| 6 D rows | met, with D3 | |
| 7 pagination | met | |
| 8 @tsv parse | met, with D1 (empty second field) | |
| 9 rules text | met; D5 adds the tier-1 line for SA5 (a) | |
| 10 write-side limits | met | |
| 11 F5 (and F2) | met | |
| 12 report correction | met | |
| 13 small text | met | the one-line conflict answer is recorded as given |
| 14 build order | met | |

**The clause table (§1), checked row by row against the other clauses:**
- `docs/Claude.md`: only C0 refuses it (C3: `CLAUDE.md` is not gitignored).
- `Tools/gate.mjs` and `agents.md`: only C1 (not gitignored, no `.claude`).
- `docs/.claude/skills/x/SKILL.md`: only C2.
- `tools/node_modules/...`: only C3.
- The decision docs: only C4.
- The `.claude/settings.json` to `docs/settings.json` rename: only C5.
- Root `.claude/agent-memory-local/...` is caught by both C2 and C3. That is fine, because neither clause depends on
  that row.
- C1 versus C3 on `CLAUDE.local.md`: in merges, C3 also catches it, so part A's new `CLAUDE.local.md` test is pinned
  only by write rows. See D4.

**Order (§2): cap, then list, then count.** Right, as long as every refusal comes before `return null` and the name
clauses run on the whole list after the count. A consequence for the rows is in D3.

## Conditions D1-D6
- **D1, C3's call.**
  - Run this one call without literal pathspecs: `io.git` gains an option (an env override) used only here, and that
    `realIo` change goes in §3's symbols.
  - Pass the names as arguments, without `-z`. Drop empty names first. Refuse outright, before the call, any name that
    starts with `:` (pathspec magic; no real file needs it).
  - Decide by exit code: 0 refuses, 1 passes, anything else refuses. Exit 0 with empty output also refuses. Use git's
    output only for the message, cut short.
- **D2, E9 and the fake.**
  - E9 runs the guard's own merge check through `realIo()`'s git, the real code path, against real git. Its file list
    is a faked `gh` answer: PR #1's real `@tsv` output plus the C3 rows. A shell line typed by hand would miss the
    literal-pathspec failure.
  - `fakeIo`'s git answers `check-ignore` with exit 1 by default (its catch-all answers exit 0 with nothing printed,
    which D1 refuses).
  - Add rows: `check-ignore` exits 128 (refused), exit 0 with empty output (refused), and a name starting with `:`
    (refused). Add a mutant on the "other exit code refuses" branch.
- **D3, the cap rows.**
  - The 3000 row carries 3000 matching docs lines, so that only the cap refuses it. Otherwise the count check also
    refuses it, and the `>=` to `>` mutant survives.
  - The missing and `2.5` rows are also refused by the strict count check. Give the whole-number test no mutant (or
    drop the test).
- **D4, pin `CLAUDE.local.md` in `protectedRel`.**
  - Use night-shift write rows (a routine write to `CLAUDE.local.md` and to `docs/AGENTS.md` refused) and a
    day-session ask row.
  - A pr-advisor write row proves nothing: the notes-only allow-list (:417) refuses it anyway.
- **D5, C4 (only if he answers SA5-RIDE (a)).**
  - Compare the lower-cased path against a lower-cased list, or add the names to `CANON`.
  - Refused rows: `Docs/Vision_V2.md` and `docs/adr/README.md`.
  - Text, carried in E6's table:
    - advisor.md tier 1 gains "Decision-doc PRs merge from a day session on his go: the guard refuses the PR
      Advisor's merge of them."
    - CLAUDE.md's tier table, row 1's "Merges" cell, gains "(decision docs: a day session)".
- **D6, scope.** §3's file and symbol list adds the `realIo`/`io.git` change (D1). E8 confirms it.

## Decisions needed
- SA5-RIDE is with Mustafa, in the rewritten text. Nothing new.

## For Mustafa
- Nothing new beyond SA5-RIDE.
