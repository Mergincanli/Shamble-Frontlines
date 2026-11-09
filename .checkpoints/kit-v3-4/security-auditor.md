# security-auditor: kit-v3-4 (A4, the review policy), review of ad1f9bd (2026-11-08)

Role read from origin/main (`git show origin/main:.claude/agents/security-auditor.md`, `CLAUDE.md`). Read-only in the
worktree. Base: origin/main 626df67 (merge-base). Head: ad1f9bd.

## Plan
1. Threat model: the assets are main's integrity (branch protection, the merge check: expected head, the advisor's
   APPROVE before his go), Mustafa's go on kit/CI/guard/decision docs, the night shift's merge ban, the "Never" list.
   Actors: a day session, the night shift routine, the PR Advisor routine, a subagent, a hostile PR text (data).
2. Read the full diff of the five kit files against main; list every changed rule sentence; compare with
   kit-draft.md §8 (E2) for completeness and honesty.
3. Tier 0 merge path: can a kit/CI/guard/decision-doc change be classed tier 0 and merged without his go?
4. Revert exception: abuse paths (non-revert change, kit revert without his go); the exact-undo check.
5. Budget and severity: can Critical/High end as a follow-up; can a security finding stay open and merge?
6. CHECKPOINTS.md §10 vs the guard per seat (run guard.mjs per seat where possible).
7. E3/E6: diff --stat for hooks/settings/CI/gate; the three kit test files on head and main, counts compared.

## Findings
(appended as confirmed)

Resumed 2026-11-09 after a container restart (the first run wrote only the plan). Head now c2ac5a6: the two
commits after ad1f9bd add only `.checkpoints/` notes (infra-engineer, verifier, this file), so the kit files reviewed
are ad1f9bd's. Scratch: `.gate/sa-a4/` (a throwaway git repo `ren/`, `merge-rename.mjs`).

### Evidence runs
- E3 spot-check on head: `node --test .claude/scripts/checkpoint.test.mjs` 27/27, `.claude/hooks/settings.test.mjs`
  127/127 (main's counts, as the verifier found; routine-guard's 1034 not re-run). E6: `git diff --name-only
  origin/main...HEAD` holds no `.claude/hooks|scripts|settings`, `.github/`, `tools/`, `.npmrc`, `.mcp.json`, game file
  or page; `git diff --quiet origin/main HEAD -- .claude/hooks .claude/scripts .claude/settings.json .github tools`: equal.
- §10 and Rollback per seat, through `.claude/hooks/guard.mjs` (PreToolUse JSON): day: `git worktree remove`, `cargo
  clean --target-dir`, `git revert -m 1` no hook decision, `--force` deny; night-shift: worktree remove no decision,
  `--force`, cargo, revert deny; pr-advisor: all four deny. Matches CHECKPOINTS.md:214-215 and CLAUDE.md:112-115.
- Rename visibility (scratch repo `.gate/sa-a4/ren`): after `git mv kit/rules.json docs/rules-notes.md`,
  `git diff --name-only main...HEAD` prints only `docs/rules-notes.md`; with `--no-renames` it prints both paths.
- The PR Advisor's merge check (`.gate/sa-a4/merge-rename.mjs`: the real `decide()` with a fake io that answers
  `pulls/<n>/files` the way `--jq '.[].filename'` reads GitHub's entries): edit or delete `.claude/settings.json`:
  deny; rename `.claude/settings.json` -> `docs/settings-notes.md`: no decision (allowed); rename
  `.github/workflows/gate.yml` -> `docs/gate.yml`: allowed. GitHub's REST docs ("List pull requests files", Diff Entry):
  `filename` required, `status` enum includes `renamed`, `previous_filename` optional; "a maximum of 3000 files".
- Claude Code's memory docs (code.claude.com/docs/en/memory): `CLAUDE.local.md` "loads alongside CLAUDE.md and is
  treated the same way"; CLAUDE.md and CLAUDE.local.md in subdirectories load on demand when Claude reads a file there.
  `protectedRel` (`routine-guard.mjs:32-43`) matches only a last part equal to `CLAUDE.md`, `.npmrc`, `.mcp.json`;
  `.gitignore:11` ignores `CLAUDE.local.md` (a commit needs `git add -f`, so it does not stop a deliberate PR).
- Main's branch protection (STATUS:981): a PR with 0 approvals and the `gate` and `smoke` checks; neither reads
  `.claude/`. So for a pr-advisor merge, the guard's protected-file refusal is the only mechanical wall.

### Threat-model answers
1. Loosening beyond his words: none found in the sentences I checked myself (CLAUDE.md:92-115, :117-126, :266;
   advisor.md:43-44, :50-92, :164-170, :198-199, :205-276, :291-298; SKILL.md:21-33; security-auditor.md:3, :52-54).
   Every needs-mustafa trigger stays (advisor.md:81-92 lists step 0, 2, 4 and categories 1-4 whole; "a tier lifts no
   trigger or Never line" CLAUDE.md:93, advisor.md:55-57). The merge check sentence is unchanged apart from the
   revert parenthesis (CLAUDE.md:117-126); "the night shift never merges" stays (:124) and the guard enforces it
   (`routine-guard.mjs:1449`). The one widened Never line (CLAUDE.md:266, "a pure revert needs none") is his
   REVERT-EXCEPTION (a) and is scoped to Rollback (day sessions; the guard refuses `git revert` in both routines).
   Tier 0 is narrower than today's "ready covers docs" (his TIER0-SCOPE (a)); "auto-merge" is read as the PR
   Advisor's pinned merge, and the guard refuses GitHub auto-merge for every session (`routine-guard.mjs:1393`).
   Agree with the verifier (no loosening beyond his words; its L3 on CLAUDE.md's short table).
2. Tier 0's merge path: the text routes kit, CI, gate and decision docs away from tier 0, but the method it names and
   the guard it relies on both miss renames, and the guard misses `CLAUDE.local.md`: SA-1, SA-2.
3. Revert exception: no merge path without his go for a kit or game revert (today's classes; the guard refuses the
   PR Advisor's merge of protected files, rename gap aside: SA-1). A non-revert cannot merge through it if the
   exact-inverse check is run; but the check never confirms the named SHA or the trigger: SA-3. Agree with IE-1 (High)
   and IE-2 (Medium); see the note under SA-3 for IE-1's CI follow-up.
4. Budget and severity: a Critical or High cannot end as a follow-up (CLAUDE.md:102-106, advisor.md:247-248). A
   security finding left open keeps needs-mustafa (advisor.md:92, :167, :251-252; security-auditor.md:54), but "left
   open" does not say a deferred follow-up counts: SA-4.
5. §10 vs the guard: matches per seat (above). Agree with IE-8 (ignored files and detached-HEAD commits are lost) and
   IE-9; nothing security-specific to add.
6. E3/E6: hold (above).

## Findings (CLAUDE.md's severity scale)

### SA-1 (Critical; pre-existing in the guard, outside this diff, which relies on it) The PR Advisor's merge check misses protected files renamed away, and `CLAUDE.local.md`
- Where: `.claude/hooks/routine-guard.mjs:1460-1463` (`--jq '.[].filename'`: a rename's old path is in
  `previous_filename`, never read); `:32-43` (`protectedRel`: `CLAUDE.local.md` is not protected, though the comment
  at :31 says "agent instructions anywhere"). The diff states the wall as fact: advisor.md:73-74 ("the guard refuses
  the PR Advisor's merge of those"), CLAUDE.md:99.
- Sketch: an injected or mistaken day session opens a PR that only `git mv .claude/settings.json docs/settings-notes.md`
  (or adds `docs/CLAUDE.local.md` with `git add -f`); CI is green (the gate reads no `.claude/`), the merge check sees
  only `docs/...`, the PR Advisor merges at once; main then runs every session with no hooks and no deny list (or with
  extra instructions loaded whenever a session reads a file under `docs/`). Gets around the guard and the merge check.
- Evidence: `.gate/sa-a4/merge-rename.mjs` results above; GitHub Diff Entry schema; Claude Code memory docs.
- Fix: its own risk-tier-2 change-set (the guard is out of A4's scope, E6): read both names
  (`--jq '.[] | .filename, (.previous_filename // empty)'`), refuse when the PR's `changed_files` is 3000 or more (the
  API's cap: SA-8), and protect `CLAUDE.local.md` (and `AGENTS.md`, which loads when no CLAUDE.md is above it) in
  `protectedRel`, with test rows. Until it lands, SA-2's text fix closes the honest-misread path; the deliberate path
  stays open on main today with or without A4. Under the new budget this cannot be a silent follow-up: Mustafa decides
  (Decisions needed).

### SA-2 (High; in this diff) The tier rule's file list is rename-blind, and "every changed file is under docs/" admits instruction files
- Where: advisor.md:54 (`git diff --name-only origin/main...<headSHA>`, new), copying step 0's command at advisor.md:127
  (unchanged line); advisor.md:58-59 (tier 0: "every changed file is under `docs/`, or is a Markdown file outside the
  kit").
- Sketch: the PR Advisor, following the text exactly, sets risk tier 0 for the rename PR of SA-1 (its file list shows
  only `docs/settings-notes.md`) and for a PR adding `docs/CLAUDE.local.md`; step 0's kit check misses the same rename.
  The text calls a merge the guard does not refuse a refused one (evidence that does not hold).
- Fix (two text edits, this change-set's fix pass): in advisor.md:54 and :127 use `git diff --name-only --no-renames
  origin/main...<headSHA>` (the guard's own listing already uses `--no-renames`, `routine-guard.test.mjs:31`); add to
  tier 0's "Not tier 0, whatever the file": "a file renamed or moved from a path outside tier 0, and any `CLAUDE.md`,
  `CLAUDE.local.md` or `AGENTS.md` anywhere (risk tier 2)". Optionally soften advisor.md:73-74 to "the guard refuses
  the PR Advisor's merge of those (renames: SA-1's follow-up)".

### SA-3 (Medium) The revert check never confirms the named merge or that main broke
- Where: advisor.md:269-272 (the PR states "the reason ... and the reverted merge's full SHA"; the check compares the
  diff with `git diff <merge-sha> <merge-sha>^1`), CLAUDE.md:112-115.
- Sketch: a session names any old commit (for example one that added a protection to a game file or a doc) as "the
  merge that broke main" and opens its exact inverse with no plan; the check passes. It still merges only by today's
  classes, so the reach is the plan step skipped and, for a docs-only original, the PR Advisor's merge at once.
- Fix: add to "Your check of a revert PR": "`<merge-sha>` is a first-parent commit of main
  (`git merge-base --is-ancestor <merge-sha> origin/main` and in `git rev-list --first-parent origin/main`), and the
  stated trigger holds: the red `gate` or `smoke` run on main names that commit and its re-run is red too, or the
  report is Mustafa's own words; otherwise it is a normal change-set, plan first." Note for IE-1's CI follow-up (a
  gate that derives allowed drops from the "This reverts commit <sha>" line): it must make the same first-parent
  check and allow exactly that merge's added symbols, or the line becomes a free pass to drop symbols.

### SA-4 (Medium) "A security finding left open" does not say a deferred follow-up counts as open
- Where: advisor.md:92, :167, :251-252; security-auditor.md:54; against CLAUDE.md:103-104 and advisor.md:249 ("below the
  bar a finding becomes a STATUS follow-up").
- Sketch: a tier-1 PR carries a Medium or Low security finding (say `innerHTML` on a player name); the implementer
  defers it to STATUS as the budget tells it to, and the PR Advisor reads "deferred in writing" as addressed and
  classes the PR ready: a security finding ships without his go. The budget makes this the normal path, and "a Low
  never triggers a push" guarantees a Low security finding is deferred.
- Fix: one clause in advisor.md:251-252 and security-auditor.md:54: "a security finding not fixed in the PR (a STATUS
  follow-up included) counts as left open".

### SA-5 (Low) The decision docs' protection is judgment only
- Where: advisor.md:60-65 (ADRs, the register and nine named docs: always needs-mustafa), against `protectedRel`
  (`routine-guard.mjs:32-43`: `docs/` is never protected).
- Sketch: a PR Advisor misreading merges an ADR edit as tier 0; nothing mechanical stops it (true on main today too).
- Fix (follow-up, with SA-1's guard change-set if Mustafa wants it): refuse pr-advisor merges of `docs/adr/**` and the
  named design and look docs. The "records his words" half stays judgment.

### SA-6 (Low) Who may change a finding's severity is not stated
- Where: CLAUDE.md:107-111; advisor.md:246-250.
- Sketch: an implementer relabels a reviewer's High as Medium to fall below the third-round bar.
- Fix: "the severity is the reviewer's label; only the advisor changes it, in writing, with the reason".

### SA-7 (Low) A kit revert's `git revert` is not seen by the guard (L4); the steps do not say who runs it
- Where: advisor.md:265-266 (step 2 asks Mustafa first); STATUS:865 (L4: `git revert` not read).
- Sketch: a subagent told to "build the revert" runs `git revert` on kit files in a worktree; the guard's subagent
  refusal for kit writes never fires.
- Fix: "the main session runs `git revert`, never a subagent". The PR review and his go stay the wall.

### SA-8 (Low; pre-existing) `pulls/files` returns at most 3000 files
- Where: `routine-guard.mjs:1461`.
- Sketch: a PR with more than 3000 changed files hides a protected file past the cap from the merge check.
- Fix: in SA-1's change-set, refuse a pr-advisor merge when the PR's `changed_files` is 3000 or more.

### Where I agree or differ with the other reviews
- infra-engineer: agree IE-1 (High) and its fix; add SA-3's note to its CI follow-up. Agree IE-2, IE-5, IE-8, IE-9.
  IE-4's "no other PR merges until the revert lands" is right; it overlaps nothing here. No disagreement.
- verifier: agree (his words byte for byte, no loosening beyond them, E3/E6). Its E2 table does not cover the gap in
  SA-2 because the gap is in a command the table rates "same" (step 0's line), not in a changed rule's meaning.
- Not duplicated: rollback mechanics, §10's git and cargo behaviour, line-number drift.

### Anti-cheat coverage
Not applicable: no game, server, network or storage code in the diff.

### Not verified
The live ruleset and its bypass actors (STATUS:981 read, not the API); routine-guard.test.mjs's 1034 (verifier's run);
GitHub's real `pulls/files` answer for a rename on this repository (schema from the docs, behaviour by fake io).

## Verdict: PASS WITH NOTES (the diff), with one Critical outside it for Mustafa
The diff loosens no protection beyond his A4 words and his three answers; tiers add and never remove; Critical and
High cannot become follow-ups; the revert exception is day-only and enforced per seat; E3 and E6 hold. Fix SA-2 (High)
in the fix pass (two text edits) and, if the pass runs anyway, SA-3 and SA-4 (one sentence each). SA-1 (Critical) is a
pre-existing guard defect the diff relies on: its own risk-tier-2 change-set, and Mustafa decides whether A4 waits for
it. SA-5 to SA-8 are Low follow-ups. A security finding is open, so the PR stays needs-mustafa (it is tier 2 anyway).
