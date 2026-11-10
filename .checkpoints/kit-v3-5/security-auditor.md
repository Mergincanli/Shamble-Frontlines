# security-auditor, kit-v3-5 (F1), PR #46, head f9728c9 vs main 44446a5 (one round, risk tier 2)

## Plan
1. mergeAllowed + pr-advisor merge block: bypass hunt (nested .claude, rules names, gate names deeper in tools/,
   shrinkwrap, .npmrc, node_modules, symlinks, case twins, @tsv escapes, empty/odd lists, renames/copies, removed,
   counts, still-running). Run the real function in scratch (sa5/), no repo writes.
2. npm docs: shrinkwrap precedence, npm ci, --ignore-scripts.
3. Part A: protectedRel / CANON / GATE_FILES, writes and day asks; legit paths not broken.
4. Known limits in STATUS: complete, honest, severities for write-side limits; Windows-alias lines vs his words.
5. rule-diff.md 17 rows vs the diff; overstating rules text.

## Findings (a line each, as found)
Method: read the diff (guard, tests, advisor.md, CLAUDE.md, STATUS limits), the plan rev3, rule-diff, his words of
2026-11-10; ran the real `mergeAllowed` on 49 edge names (scratch `sa5/probe.mjs`) and the real `decide` merge check
with a faked gh on 7 lists (`sa5/merge-probe.mjs`). Nothing written in the repo but this file.

- SA5-1 Low. npmCheck's "install only main's lockfile" compares tools/package.json and package-lock.json only
  (routine-guard.mjs:561); npm prefers npm-shrinkwrap.json (docs below). Reach is tiny: routines cannot write it
  (GATE_FILES now), the night switch refuses a branch that carries it (protectedRel via kitName), and a routine starts
  from a fresh clone. Fix: add 'tools/npm-shrinkwrap.json' to that diff list and refuse when the file exists untracked.
- SA5-2 Low. The merge check accepts an empty list with changed_files 0, and a blank line counts as a file but checks
  no name (routine-guard.mjs:167-171; probe: both "merge goes on"). Only malformed or empty GitHub answers reach it;
  an empty PR changes no tree. Fix: refuse when cf < 1 or any line's first field is empty.
- SA5-3 Low. Windows reserved names pass (docs/aux.md, docs/CON, tools/nul.js; probe ALLOW). Git for Windows refuses to
  check them out (compat/mingw.c is_valid_win32_path, active when core.protectNTFS is on), so Mustafa's Windows pull
  would fail; no bypass. Not in STATUS's limits. Fix: one known-limit line (no code, by his words).
- SA5-4 Low (wording). advisor.md:44-46 (tier-2 bracket): (a) "never a ... gate ... name in any letter case" is true
  only under tools/ (docs/package.json merges, test row :238); (b) it names only Mustafa's words as what the guard
  cannot judge, but symlinks, case twins, gitignored and reserved names inside the places also pass, and a symlink does
  not show in a --name-only list (mode 120000 shows in --raw / --summary). Fix: "gate name under tools/"; name those
  and read `git diff --raw --no-renames` (or --summary) yourself.
- SA5-5 note, no change. docs/ENGINE_OVERVIEW.md is a decision draft (CLAUDE.md:20-22) but in neither DECISION_DOCS
  nor advisor.md's decision-doc list; by SA5-RIDE (a) "the fixed list ... and nothing more" it stays out; the advisor's
  own reading covers it (tier 1 rule: decision docs need his go).
- PX-1 pre-existing, outside the diff, NOT tested. The merge check matches check runs by name only
  (routine-guard.mjs:1480-1488; no app, workflow or check-suite binding). GitHub: check runs are created by GitHub Apps
  (REST checks docs), GITHUB_TOKEN is one, and [skip ci] in a PR's head commit skips push and pull_request workflows
  (Actions docs). So a same-repo branch's workflow with checks: write could post green "gate" and "smoke" on a head
  whose real CI never ran. If confirmed, by the scale it gets around the merge check (Critical), though the advisor's
  review still stands. Fix (own change-set): require gate.yml's workflow run on that head SHA (actions/runs?head_sha=)
  with jobs gate and smoke successful, and the check runs' app to be github-actions.

Checked and holding (no finding): nested .claude and any dot segment (L2); rules names any case, any depth (L5); gate
names at any depth under tools/, shrinkwrap included (L6); tools/.npmrc (L2); node_modules any case (L3); case twins of
places (L4 exact); ADR dir any case; @tsv escapes (backslash refused by L1; \n \r refused); both names of a rename;
removed files (name still listed); cap >= 3000; changed_files missing, non-integer or mismatched; total_count missing
or larger; still running; gh failure -> throw -> deny. CI reads only gate files from tools/ (gate.mjs requires
./battery.js, ./smoke.mjs, gate-globals.json; npm ci reads package.json, the lockfile or shrinkwrap, .npmrc). No hook
or script imports a tools/ or docs/ file; no @-imports from CLAUDE.md or agents. Writes follow symlinks (realOf), so a
merged symlink in docs/ cannot open a protected write. Race: head pinned by expectedHeadSha.

npm (Q2): package-lock-json page: "If both package-lock.json and npm-shrinkwrap.json are present in the root of a
project, npm-shrinkwrap.json will take precedence and package-lock.json will be ignored." npm-ci page: "The project
must have an existing package-lock.json or npm-shrinkwrap.json." --ignore-scripts "does not run scripts specified in
package.json files": it stops install-time code, not the risk: the shrinkwrap picks what is installed ("resolved" may
be "a complete URL" off the registry), and that code runs when the gate require()s it. So the shrinkwrap stays a gate
file with --ignore-scripts. (docs.npmjs.com/cli/v10/configuring-npm/package-lock-json, .../npm-shrinkwrap-json,
.../commands/npm-ci; read 2026-11-10.)

Part A (Q3): protectedRel + CANON + GATE_FILES: night writes/commits/switch and day asks cover CLAUDE.local.md,
AGENTS.md (any depth, folded on win32/darwin) and the shrinkwrap; agent memory and worktree recursion unchanged; no
tracked file in the repo has those names, so nothing legitimate newly asks or is refused. kitDir sees tracked
AGENTS.md / CLAUDE.local.md through ls-files, but an untracked (gitignored) CLAUDE.local.md in a subfolder only by the
CLAUDE.md exists-check (W3 below).

Known limits (Q4): honest; the three Windows lines are one line each, "not handled", nothing built: they match his
words. The trailing-dot line's "not verified" can now be answered: Git for Windows refuses a segment ending in a dot
or space when core.protectNTFS is on (compat/mingw.c, "cannot end in ` ` or `.`"), so docs/CLAUDE.md. fails checkout
there rather than loading as CLAUDE.md. Missing: reserved names (SA5-3), the empty list (SA5-2), PX-1.
Write-side severities: W1 nested <dir>/.claude/ writes without an ask: Medium (Claude Code can load nested skills;
small reach: needs a misled writer, and the PR Advisor now refuses dot segments). W2 gitignored paths beyond the
protected names (tools/node_modules, .gate/, .env*, dist/, *.log): Medium (tools/node_modules is code the local gate
runs, so a local gate result can be falsified; CI reinstalls, merges refuse it). W3 kitDir checks a folder for
CLAUDE.md only (routine-guard.mjs:147): Low (an untracked CLAUDE.local.md / AGENTS.md in a subfolder can be moved or
removed with its folder).

Nothing loosened (Q5): rule-diff's 17 rows match the diff (rows 1-6 guard, 7-13 advisor.md, 14-17 CLAUDE.md); row 1's
"every path protectedRel refused is refused" holds (base names via L2/L5, .github/.git/.claude via L2, GATE_FILES via
L6). Only overstatement: SA5-4.

Verdict: PASS WITH NOTES (no Critical or High in the diff; PX-1 pre-existing, unverified).
