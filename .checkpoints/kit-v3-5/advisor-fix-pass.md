# Advisor: kit-v3-5 fix-pass ruling (round 1 on head f9728c9)

Ruled 2026-11-10 from main's `.claude/agents/advisor.md` (Review budget) and his rule of 2026-11-10 ("Review budget
as in A4: two rounds at most, and Lows never trigger a push"). Sources: `security-auditor.md`, `verifier.md` and
`test-engineer.md` in `.checkpoints/kit-v3-5/` (commits 624af9c and 7db0087), and advisor.md's tier-2 bracket at
f9728c9 (:80-83).

## Summary
- **Round 1 found nothing Critical or High in the diff.** It found four Mediums (all in the test file), Lows, three
  write-side limits and one finding outside the diff (PX-1).
- **I accept the session's proposal, with the changes below.**
  - The test file carries the Mediums, and the Lows ride with them.
  - The guard and advisor.md each take one small write, in the same push. That push is needed anyway for the Mediums,
    so no Low triggers a push of its own.
  - The docs Lows ride the report's docs push.
- **No second round.** The verifier re-checks what changed (outside the budget). A Critical or High found there opens
  round 2, run by the reviewer who owns that finding (security-auditor for the guard, test-engineer for the rows), on
  the changed hunks only.
- **PX-1's route goes to Mustafa (PX1-ROUTE).** The PR stays needs-mustafa, as it is anyway at tier 2. PX-1 and W1-W3
  are open security findings.

## Fix now
**One write of `.claude/hooks/routine-guard.test.mjs`** (one ask):
- **TE1 (Medium):** the fake `gh` checks its arguments. The files read must carry `--paginate` and exactly the
  `--jq '.[] | [.filename, (.previous_filename // "")] | @tsv'` filter, or the fake answers code 1. Add a mutant that
  puts back `.[].filename`; some row must catch it.
- **TE2 (Medium):** rows at the places' edges, all refused: `docsx/a.md`, `tools.mjs`, `README.md.js`, `readme.md`.
  The three mutants that survive today must be caught.
- **TE3 (Medium):** a refused row for `docs/ADR/0001-x.md` and for each of the 10 decision-doc names (one row each is
  fine). The two surviving mutants must be caught.
- **TE4 (Medium):** the case row becomes `tools/Node_Modules/acorn/dist/acorn.js`, which only the `node_modules` rule
  refuses.
- **V1 (Low, rides):** a two-file row in the `@tsv` shape, with one docs name and an empty second field, so that
  removing `.filter(Boolean)` (routine-guard.mjs:1500) fails a row.
- **V2 (Low, rides):** a refused merge row for `.claude/agent-memory-local/advisor/MEMORY.md`.
- **TE5 (Low, rides):** the short-name row becomes `docs/CLAUDE~1.MD`.
- **TE7 (Low, rides):** the default fake file list prints the `@tsv` shape (`docs/a.md<TAB>`, then a newline).

**One write of `.claude/hooks/routine-guard.mjs`** (one ask; Lows riding the same push):
- **SA5-2 = TE6:** refuse a pr-advisor merge when `changed_files` is below 1, or when any line's first field (the new
  name) is empty. Add refused rows (0 files; a blank line inside the list) and one mutant per check.
- **SA5-1:** the night install check (:561) also compares `tools/npm-shrinkwrap.json` with main's and refuses it when
  it exists but is untracked. Add a night row and a mutant.
- Nothing else in the guard changes in this pass.

**One write of `.claude/agents/advisor.md`** (one ask). SA5-4 (Low, rides), tier-2 bracket (:80-83). Replace the
bracket with:

> (the guard lets the PR Advisor merge only plain paths in `docs/`, `tools/`, `README.md` and `FIRST_PROMPTS.md`, read
> with both names of a renamed or copied file and with whole lists, never a rules or decision-doc name in any letter
> case, and never a gate file's name under `tools/`; everything else merges from a day session on his go. It cannot
> judge a word or decision of Mustafa's, and inside those places it lets through symlinks, case-only twins, gitignored
> names and Windows reserved names, so still read `git diff --raw --no-renames origin/main...<headSHA>` yourself: it
> shows each file's mode, so a symlink (120000) or a submodule (160000) stands out)

The rule-diff table gains this row, marked "same or stricter": the guard is unchanged by it, and the text now states
the guard's reach exactly.

## The docs push (with the report)
- **V3:** STATUS:145's stale register line numbers are corrected.
- **V4:**
  - ROADMAP:110 is wrapped.
  - STATUS:710 holds a quote of his. Wrap it only at a space, never inside a word. E7 is re-run with whitespace
    collapsed, so his characters stay the same. If that is in doubt, leave it at 121 characters: a Low that touches
    his words is not worth the risk.
- **V5:** rule-diff row 15 gains its note: the narrower first clause is governed by advisor.md tier 0.
- **V6:** STATUS:1010 takes my E4 wording in full.
- **SA5-3:** a known-limit line: "Windows reserved names (`CON`, `NUL`, `AUX`, `COM1` and the like) inside the places
  are not refused (his direction of 2026-11-10: no building for Windows name aliases)."
- **W1-W3:** STATUS's limits take the security-auditor's severities: W1 nested `.claude/` writes, Medium; W2 gitignored
  writes, Medium; W3 `kitDir`, Low. All are deferred under KIT-FREEZE-2.
- **SA5-5:** no change. Under his "the fixed list ... and nothing more", `docs/ENGINE_OVERVIEW.md` stays off the
  guard's list. One STATUS line records that the advisor still treats it as a decision draft (CLAUDE.md:20-22) that
  needs his go.
- **PX-1:** a STATUS follow-up, with its severity as below and his route once he answers.
- **TE8:** the report's counts become 44 merge rows and 52 new rows in all, plus the rows this fix pass adds.

## Re-check (not a round)
- **The verifier, on the new head, checks only what changed:**
  - the test-file rows and mutants above;
  - the guard hunk (SA5-1, SA5-2);
  - the advisor.md bracket and its rule-diff row;
  - the docs lines.
- **It runs:**
  - the kit suites (routine-guard with the new count and no surviving mutant; settings 127; checkpoint 27);
  - `node tools/gate.mjs` (symbols +0 −0);
  - `repo-map-check.mjs`;
  - E7 on his words.
- After that I review the new head, then he gives his go on it.

## PX-1: my reading, for the decision
- PX-1 says the merge check matches the `gate` and `smoke` check runs by name only (routine-guard.mjs:1480-1488). A
  workflow pushed on another same-repository branch could post green runs with those names on a head whose real CI
  never ran (`[skip ci]`).
- Unverified. On main today, before this merge, it would get around the merge check: Critical by the scale, if
  confirmed.
- After kit-v3-5, its reach on the PR Advisor is small. The allow-list admits only `docs/` and non-gate `tools/` paths,
  and the gate reads neither (security-auditor: "CI reads only gate files from tools/"). So a faked green on an allowed
  PR hides no result that a real run would have shown, unless main itself is red.
- Day sessions run the gate themselves (advisor.md, Never: "Approve without running the gate yourself").
- What stays exposed is GitHub's own required-checks protection, and any merge that trusts those names.
- I rate it **Medium after this merge, if confirmed**.
- A proof needs a workflow pushed on a branch: a `.github/` change, so risk tier 2 work in its own change-set, not
  now.

## Decisions needed
**PX1-ROUTE.** When should the guard stop trusting check runs by their name alone?
- Why it matters: the check could be faked by anyone with push access, through a workflow on another branch. After
  this merge, little can slip through the PR Advisor that way, but GitHub's own required checks trust the same names.
- (a) A STATUS follow-up. Its own kit change-set after the foundation's gate step (KIT-FREEZE-2 holds): first a proof
  test, then bind `gate` and `smoke` to the gate workflow's run on that head SHA and to the GitHub Actions app.
- (b) An exception to the freeze: the same change-set right after kit-v3-5 merges, before the foundation's work.
- Recommendation: (a). This pass's allow-list already leaves the PR Advisor nothing that a faked green would hide, and
  the fix is a new guard check, which the freeze exists to hold back.

## For Mustafa
1. Answer PX1-ROUTE: (a) or (b).
2. Approve the three kit writes as they ask: the test file, the guard, advisor.md.
3. After the verifier's PASS and my review of the new head: your go on that head ("merge #46").
4. After the merge: re-enable the PR Advisor routine yourself.
