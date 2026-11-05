# Task: advisor ruling on A3 (kit-v3-3, PR #38) build questions 1-4 after steps 3-5 (f22c4df)
## Plan (blocks)
B1 read mustafa-words, advisor-plan-rev1 (C6), advisor-e10, builder-step3/4/5
B2 read plan-rev2 relevant sections (day ask, merge rule, 4.4 settings)
B3 check code at worktree: routine-guard.mjs + test :110 :130, settings.json
B4 run tests
B5 write ruling
## Notes
- B1 done: inputs read. C6 = "existing tests pass unedited (additions only; any edit named+justified in report)" -> C6 itself allows a named, justified edit; builder read it stricter. Step4 builder skipped dayEdit + A1-A6,A8,A25,A36,S2/S3 (Edit forms). Merge: builder allowed plain `git merge origin/main`. Step3: no allow rule for git rev-list. Next B2 plan sections.
- B2 done: plan 4.1 dayEdit wired at decide() `if (!role) return dayEdit(...)`; WT-UPDATE (a) free forms: switch -c <new> origin/main, merge --ff-only origin/main, pull --ff-only origin main, switch <ref> with no protected diff; 4.4 "Nothing else (ask stays []; no allow for node .checkpoints/*)" -> "nothing else" was about ask list + node allow, rev-list allow not considered. Next B3 code.
- B3 findings (code at d3d73a6, worktree clean):
  - test :129 comment "day sessions keep their freedom (no role: only git and gh are checked)" + :130 both from kit-v3-1 (3f43a7a/ee2a384, 2026-10-27): they encode the pre-item-1 policy Mustafa changed. C6's own text allows a named+justified edit; C6 is about ROUTINE paths, :130 is a DAY case.
  - routine-guard.mjs:766-768 merge allows '--ff-only origin/main' or 'origin/main'; :770 pull ff-only only (asymmetric). :649 routine reason "a day session merges main in". fakeIo (test :30-72) answers `diff --name-only origin/main...HEAD` with branchDiff default docs/a.md -> protectedHit null -> a guarded plain-merge rule keeps :110 unedited.
  - dayShellWrites :565-592: $/glob target -> ask even with agent_id (:581-585); kit/dir target with agent -> deny (:589). kitDir :106-113 ('.', .claude, .github, tools, folder holding CLAUDE.md).
  - isWin :64 `(io.platform ?? process.platform)`: production falls back to process.platform; A36s test :628-629 both platforms. OK.
  - mainFolderMove :741-779: `--` after ref at :750, :759; `git switch -` -> ask (:747). agent -> deny (:778).
  - settings.json allow :15-69 has git diff/log/show/merge-base/rev-parse, no rev-list; deny :178 rev-list --output. settings.test.mjs:35-45 NEEDED (routine commands must match an allow) lacks rev-list.
Next B4: real-git probe of `--` fix, then run tests.
- B4 done: real git 2.43 probe (throwaway repo in scratchpad): `git diff --name-only origin/main docs` (docs a folder, no such ref) exit 0, empty = fail open; with `--` after: "fatal: bad revision 'docs'" exit 128 -> the step-4 `--` fix is real. Tests at worktree: routine-guard 317/317, combined with settings 441/441 (matches builder-step5). decide day return at routine-guard.mjs:336; KIT_AGENT :514; DAY_CASES comment :590.
Next B5: write ruling.

## Ruling (block B5)
**Verdict: PROCEED, with changes R1-R4.** Nothing here is Mustafa's to decide: each point is a technical choice inside
his item 1 and item 4, and the PR is already needs-mustafa for his merge reconfirmation (escalation category 3). Every
change below is named in the report under "changes to the plan" so his reconfirmation covers it.

What I checked (worktree at d3d73a6 = f22c4df + notes, tree clean): routine-guard.mjs :64 (isWin), :94-113 (kitPath,
kitDir), :314-315 (ask), :336 (decide's day return), :565-592 (dayShellWrites), :709-720 (protectedHit,
protectedInBranch), :741-779 (mainFolderMove); routine-guard.test.mjs :30-72 (fakeIo), :110, :129-130, :555-600
(dayIo, DAY_CASES), :610-658; settings.json :15-69 (allow), :178 (deny); settings.test.mjs :34-55 (NEEDED, NEVER);
git blame of :110 and :129-130 (kit-v3-1, 3f43a7a and ee2a384, 2026-10-27). Ran: routine-guard 317/317, with settings
441/441. Real git 2.43 in a throwaway repo: `git diff --name-only origin/main docs` (docs a folder, no such ref) exits 0
with an empty list; with `--` after the ref it fails ("bad revision 'docs'", exit 128).
Not checked: anything in a live Claude Code session (E11's job); the builders' mutation numbers (scratch copies) I took
from their notes, the in-suite mutation tests I ran.

### R1. Test :130 (the day Edit of CLAUDE.md): change it to `true`, and build dayEdit now
C6 does not forbid this; the builder's brief read it more strictly than I wrote it. C6 is about routine paths ("Routine
paths: every existing routine-guard and settings test passes unedited ... any edit to an existing case named and
justified in the report"). :130 is a day case, and it asserts the exact policy Mustafa's item 1 reverses. Keeping it
would mean not building item 1 for the file tools, which are the main way a session edits a file. No better way exists:
removing the case loses coverage; gaming the fake io would make the test lie. Flip it, and fix the comment above it,
which has become false.
- File: `.claude/hooks/routine-guard.test.mjs`.
  - :129 old `  // day sessions keep their freedom (no role: only git and gh are checked)`
    new `  // day sessions keep their freedom outside kit and gate files (kit-v3-3 item 1: those ask; the shapes are DAY_CASES)`
  - :130 old `  ['day: edit CLAUDE.md', DAY, ev('Edit', { file_path: `${P}/CLAUDE.md` }), false],`
    new `  ['day: edit CLAUDE.md asks (kit-v3-3 item 1)', DAY, ev('Edit', { file_path: `${P}/CLAUDE.md` }), true],`
  - add right after it (keeps the case's old intent, a day file-tool edit of a game file passes):
    `  ['day: edit a game file', DAY, ev('Edit', { file_path: `${P}/shamble-game.js` }), false],`
  - :590's comment: drop "(the Edit-tool forms ... wait: see the report)".
- Then build what step 4 left out, as plan §4.1 has it: `dayEdit(tool, ti, cwd, io, input)` (Edit, Write,
  NotebookEdit, MultiEdit; kitPath; agent_id -> deny with KIT_AGENT, else ask) and routine-guard.mjs:336
  `if (!role) return null;` -> `if (!role) return dayEdit(tool, ti, cwd, io, input);`. Tests A1-A6, A8, A9-A11, A22,
  A22b, A25, A36 in their Edit forms (A36 through `io.platform`, R4c), S2/S3 in settings.test.mjs in their Edit form,
  and the "day ask off" mutation row extended to dayEdit (it must fail A1/A2). A26 (a role never gets an ask) must run
  over the Edit forms too.
- The report carries: "One existing test changed, on purpose: `routine-guard.test.mjs:130` asserted that a day session's
  Edit of CLAUDE.md passes with no decision (kit-v3-1, 2026-10-27). Mustafa's item 1 ('ask before kit and gate edits
  in day sessions') reverses exactly that, so the case now expects the ask (the case table counts an ask as refused;
  A2 checks its shape). Its comment at :129 changed with it, and a new case keeps the old intent for a game file. No
  routine case changed."

### R2. Test :110 (`git merge origin/main` in the main folder): keep :110 unedited; keep the plain form, but guard it
Neither form as it stands. The plan's strict rule (ff-only only) asks on the most common honest update: bringing
reviewed main into the branch the folder is already on (a cloud session's main folder sits on its own claude/... branch;
routine-guard.mjs:649's own reason says "a day session merges main in"). That is not a switch "to another branch, a
PR's head or an old commit". But the builder's form lets a plain merge through even when the branch has kit edits of
its own; then the merged kit files are a mix of the branch's and main's that nobody reviewed, and a conflict can leave
markers in `.claude/settings.json` or `guard.mjs` under the running session. A fast-forward can never do either.
The fix uses the existing helper: when the branch has changed no protected file since its merge base with origin/main,
a three-way merge takes main's kit files exactly, with no conflict possible in them.
- File: `.claude/hooks/routine-guard.mjs`, mainFolderMove, :766-768.
  old:
  `    const a = args.join(' ');`
  `    if (a !== '--ff-only origin/main' && a !== 'origin/main') why = 'only origin/main merges into the main folder';`
  new:
  `    const a = args.join(' ');`
  `    if (a !== '--ff-only origin/main' && a !== 'origin/main') why = 'only origin/main merges into the main folder';`
  `    else if (a === 'origin/main') {`
  `      // a plain merge takes main's kit files exactly only when the branch changed none since the merge base`
  `      const h = protectedHit(['origin/main...HEAD'], vcwd, io);`
  `      if (h) why = h.failed ? "the guard can't compare the branch with origin/main"`
  `        : `the branch's own ${h.hit} would mix with main's; use git merge --ff-only origin/main or a worktree`;`
  `    }`
  (builder rewraps to 120 columns). `git pull` stays ff-only only, as planned.
- :110 stays as it is: fakeIo answers `diff --name-only origin/main...HEAD` with docs/a.md (test :57), so it still
  passes. T5b stays null (dayIo's fallback answers an empty list).
- New tests (additions): T5c `git merge origin/main` where the three-dot diff names `.claude/settings.json` -> ask,
  reason matches /mix with main/; T5d the same with the diff failing -> ask; T5e T5c with agent_id -> deny. dayIo gets
  an option for the three-dot answer (new scaffold, not an existing case). Mutation row: "plain merge unguarded" must
  fail T5c.
- The report carries: "WT-UPDATE (a) refined (advisor ruling, build step 4): besides `git merge --ff-only origin/main`,
  a plain `git merge origin/main` stays free in the main folder when the branch has changed no rules, CI or gate file
  since it left main (then the merge takes main's reviewed files exactly, with no conflict possible in them); otherwise
  it asks. Why: bringing reviewed main into the folder's own branch is not a switch, and a fast-forward is impossible
  once the branch has commits. Existing case :110 unchanged."

### R3. `git rev-list` allow rule: add it
Plan §4.4's "Nothing else" was about the ask list and the `node .checkpoints/*` allow (NODE-ALLOW); nobody weighed a
rev-list allow, and without it step 3 does not do its job: the routines need rev-list (STATUS:577, :624;
advisor.md:80) and a permission prompt in a routine is refused by the guard's PermissionRequest branch. Its
neighbours (`git diff *`, `git log *`, `git show *`, `git merge-base *`, `git rev-parse *`) are all allowed already
(settings.json:17-21, :29, :57). The deny `Bash(git rev-list *--output*)` (:178) stays and wins over the allow; the
guard refuses `--output` and its abbreviations in every session (GIT_LONG_DANGER).
- File: `.claude/settings.json`, allow list: after :29 `"Bash(git merge-base *)",` add `"Bash(git rev-list *)",`.
- File: `.claude/hooks/settings.test.mjs`: add (no existing line changed)
  `NEEDED.push('git rev-list --count origin/main..HEAD'); // kit-v3-3: the PR Advisor's read (TEST-RUN B2)`
  after the NEEDED array, so the "needed, matches an allow" and "no deny" tests cover it. The NEVER entry stays.
- Plan §4.4 and the report carry: "deny `Bash(git rev-list *--output*)`, and one allow, `Bash(git rev-list *)` (advisor
  ruling, build step 4): without it a routine's rev-list meets a permission prompt, which the guard refuses in
  routines; deny beats allow, so `--output` stays refused."

### R4. Step 4's choices, and the two fixes
(a) An agent's folder target (`.`, `.claude`, `.github`, `tools`, a folder holding CLAUDE.md) refused like a kit target
(routine-guard.mjs:588-589): **confirmed.** A folder target is a write that can reach kit files; AF-AGENT's reason
(an agent's ask stalls an unwatched run) applies unchanged, and agents write only notes, scratch and their named docs.
Consequence to state in the report: after A3, agents in isolated worktrees cannot build kit or gate files either;
that is AF-AGENT (b) as decided (the main session edits them, and asks).
(b) An unresolvable target (`$`, substitution, glob) for an agent: **change it to a refusal.** The builder's ask
(:581-585, `continue` before the agent check) is what the plan text literally says, but AF-AGENT is the rule for
agents, and its reason fits this case best of all: agents write to `$N/...` paths all the time (every notes file), so
each such write would wait for a person, all night tonight. A refusal costs nothing: the agent spells the path out and
the guard then resolves it (null for notes, deny for kit). The main session keeps the ask.
- File: `.claude/hooks/routine-guard.mjs`, dayShellWrites :581-585.
  old:
  `    if (/[$`*?[{]/.test(raw)) {`
  `      first ??= ask(`kit and gate files ask first in a day session: ${what} writes ${raw}, which the guard cannot ` +`
  `        'resolve (a variable, substitution or glob)');`
  `      continue;`
  `    }`
  new:
  `    if (/[$`*?[{]/.test(raw)) {`
  `      if (agent) return deny(`${KIT_AGENT}; an agent spells its write targets out (${what} writes ${raw}, a ` +`
  `        'variable, substitution or glob the guard cannot resolve)');`
  `      first ??= ask(`kit and gate files ask first in a day session: ${what} writes ${raw}, which the guard cannot ` +`
  `        'resolve (a variable, substitution or glob)');`
  `      continue;`
  `    }`
- Tests (additions): A20a `echo x > $F` with agent_id -> deny /spells its write targets out/; A20b `tee .gate/*.md`
  with agent_id -> deny; A20c the literal `echo x > .checkpoints/x.md` with agent_id -> null. Mutation row: "agent ask on
  an unresolvable target" must fail A20a.
- The report and STATUS's stated limits carry: "An agent's write to a target the guard cannot resolve ($, substitution,
  glob) is refused, not asked (AF-AGENT: an ask stalls an unwatched agent); it writes the literal path instead. A main
  day session's ask waits for a person: when a day session runs unattended, its own kit writes and unresolved targets
  wait until someone answers."
(c) A36 on every OS through an injected `io.platform`: **confirmed, better than the plan's "win32 only, skipped
elsewhere".** CI is Linux, so a skipped test would never run. The seam is safe: routine-guard.mjs:64 falls back to
`process.platform` when io carries none, and realIo sets none, so production reads the real OS. Plan §5 A36's text
changes to "both platforms, through io.platform"; the report says so.
Fix 1, the `--` after the ref (routine-guard.mjs:750, :759): **confirmed, and now shown with real git**: without it,
`git diff --name-only origin/main docs` reads `docs` as a path when no such ref exists, exits 0 and lists nothing, so a
switch would pass unchecked; with `--` git fails ("bad revision", exit 128) and the guard asks. Add one real-git test
(beside step 3's, an addition): in a throwaway repo with a `docs` folder and no `docs` ref, the no-`--` form exits 0
with empty output and the `--` form exits non-zero. The routine's `protectedInBranch` keeps its range unchanged (C6).
Fix 2, `git switch -` asks (:747): **confirmed.** `-` is the previous branch, which the guard cannot read from the
words; fail closed is right. T26 stands.
Step 3's `--outp` finding: **accepted; E4 is corrected, the guard's refusal stays.** Shown: `--output=<f>` writes a
file for rev-list, log, show and diff (git 2.43). Not shown and withdrawn: "`--outp` writes"; git 2.43 rejects it
(rev-list and diff exit 129, ambiguous with `--output-indicator-*`; log and show exit 128). The guard keeps refusing
`--outp` because abbreviation rules change between git versions and the check costs nothing. The real-git test asserts
only the guard's refusal for `--outp`, as built. The report's E4 line says exactly this.

### Order and evidence
Build R1-R4 as one more commit on the branch (step 4b), before step 6. Gate and both test files after; mutation rows as
listed; the report's list of changes to the plan gets R1-R4's four lines. Agents: Opus 5.5 at high, at most 3 at a
time; checkpoint saved after the step. Edits stay sequential on routine-guard.mjs.

### Decisions needed
- None new from these questions. Still open from before, and still Mustafa's (C2): **WT-NIGHT**, whether the night
  shift may switch its folder to an item's branch when that branch's rules, CI and gate files equal main's (a), or
  must work in a separate worktree, which needs its own change-set first, and until then stops the item (b).
  Recommendation: (a). Its build waits for his answer.

### For Mustafa
1. In the morning: answer WT-NIGHT (above).
2. Nothing else from this ruling; the PR's merge reconfirmation comes with the report.
- B5 done: ruling written. DONE
DONE
