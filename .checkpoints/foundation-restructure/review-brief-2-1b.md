# Review brief: foundation step 2.1b, the move (PR #51, round 1, on the final diff)

Repository: `/home/user/Shamble-Frontlines`, branch `claude/clever-fermi-oudsfq` (stands in for
`cs/foundation-restructure`), base main **76b4f7a** (2.1a, PR #50, merged). The head to review is the branch's HEAD
when you start (`git rev-parse HEAD`; name it in your answer). The final diff: `git diff 76b4f7a HEAD -- . ':!.checkpoints'`.
Risk tier 2 (the four kit lines). You review; you never edit a file outside your notes; you never commit or push.

## How to work (the guard's rules for agents today)
- Write only your notes file, `.checkpoints/foundation-restructure/<your-agent-name>-2-1b.md` (plan first, a line per
  finding as you go), with the Write or Edit tool.
- In the shell: one plain command with a literal path, run from the repo root. Never `git -C`, `git -c` or
  `--git-dir`: the guard refused that form twice today. A refused command cannot be asked: note it and go on.
- Scratch runs go under `.gate/` (gitignored). Linux needs `xvfb-run -a` before `tools/preview.js` and
  `tools/pageshot.js --headed`.
- Severity, one scale (CLAUDE.md): Critical, High, Medium, Low. Every finding with file:line and the evidence (a
  command and its output, or the code lines). One round, one fix pass: below High a finding becomes a STATUS follow-up.

## What changed (commits on top of 76b4f7a; the `checkpoint(...)` commits touch only `.checkpoints/`)
- c090843, the move: `git mv` of `shamble-core.js`, `shamble-game.js`, `shamble-editor.js`, `index.html`,
  `editor.html` into `legacy/`, bytes unchanged, nothing else in the commit.
- 1e6f84e, the commands in text: `tools/README.md` (every `.` folder argument → `legacy`, the smoke row's "outside the
  repo" → "outside `legacy/`", and the paragraph "**The build folder.**" describing the code's rule), the header
  comments of preview.js, pageshot.js, sim.js, reach.js, perfbots.js, tuning-dump.js, aabbs.js, `tools/village.py`:2,
  the root `README.md`.
- b6b3b81, the four kit lines, on Mustafa's chat yes (KIT-2.1B (a), 2026-11-11): CLAUDE.md :19, :43, :44 and
  `.claude/agents/verifier.md:28`.
- 2a37121, by the session's scripts: `docs/DECISIONS.md` (his six messages as entries, his render-budget note under
  "Queued"), ADR 0002's Status line (his SCOPE (a) quoted whole), the report's Appendix A.
- The docs pass by docs-keeper (REPO_MAP, START_HERE, MODULE_MAP, TECHNICAL, STATUS, STATUS_ARCHIVE, ROADMAP, ADR 0002's
  OQ-0002-2 answer and :7, the register `docs/adr/README.md`).
- Untouched by design: every gate file (`tools/gate.mjs`, `smoke.mjs`, `battery.js`, `gate-globals.json`, the package
  files), `.github/`, settings, hooks, `spikes/`, the tuning oracle files, PM rows.

## The evidence (in `.checkpoints/foundation-restructure/`; scripts copied beside their results)
- E1 `e1-e2-result.json`: `git diff --find-renames=100% --name-status c090843^ c090843` = five R100 rows, nothing else;
  the five blobs equal at 76b4f7a, 44446a5 and `legacy/`. E2 (same file): 44446a5's untouched gate files, run from a
  throwaway worktree's `legacy/tools/`, pass on the moved bytes (+0 -0, smoke 8/8 and 5/5).
- E3 `e3-results-2-1b.json` (31 entries) and `e3-bases-2-1b-result.json`: each mutation planted in OLD (44446a5, files at
  the root, its own gate) and in the head (files in `legacy/`, main's gate): equal non-empty failing sets, 31/31; the
  head's gate against five bases (44446a5, 8e58937, 76b4f7a, c090843, b6b3b81) PASS.
- E4 `e4-result.json`: the tuning dumps at A (`.`) and at the head (`legacy`) byte-identical, both maps, twice; R7's
  builder reproduces `docs/legacy-tuning.json` and `docs/LEGACY_TUNING.md` byte for byte.
- E5 `e5-result.json`: saves under `file://` in the pinned headless Chromium 1194: SHARED across folders. Run 4: both
  console lines (`saves-console-lines.md`) on the real `legacy/index.html` and `legacy/editor.html`, own profile, 10/10.
- E6 `e6-commands-result.json`: the 14 changed command lines run as written, all exit 0 with their files.
- E8 `e8-result.md`: main + `git merge --no-ff` of the PR head (M) then `git revert -m 1` (R) in a throwaway worktree:
  the tree equals 76b4f7a; `node tools/gate.mjs M` and `--smoke-only` PASS.
- `kit-check-result.json`: the kit commit equals the approved before/after text, 14/14. `case-test-result.json`:
  findBuild's letter case on Linux. `prompt-tally.json`: what reached Mustafa as a prompt today.
- CI: push #674 / pull request #675 on c090843 (gate headers "(files at the root) | head = working tree (files in
  legacy/)"), #676-#679 on 1e6f84e and b6b3b81, #681/#682 on f7ceba8: all green.

## Focus by reviewer
- **security-auditor** (tier 2): the four kit lines (exactly as approved; nothing loosened); the move against the guard
  (`.claude/hooks/guard.mjs`, `routine-guard.mjs`: protected paths, GATE_FILES; nothing names a legacy path) and against
  the gate's move check (tools/gate.mjs:242-249: a stray or decoy page left at the root; letter case); the README's
  build-folder paragraph against the code (smoke.mjs `findBuild` and `LEGACY_JS`, gate.mjs `BASE_PREFIXES`,
  `basePrefix`, the move check): every sentence true, Windows and macOS stated "by reading the code, not run"; **both
  console lines** in `saves-console-lines.md`, read first: what they read and write, that they make no network request
  and delete nothing, and the restore line's handling of the pasted backup text (it is pasted as a JavaScript
  expression); the docs' DECISIONS.md entries are byte-checked copies (`decisions-entries.mjs`).
- **test-engineer** (is it proven): E1 to E6 and E8 above, each against plan 1.5's definitions; E3's controls and its
  two wording notes (M19, M20: the report's "5b + 5d" grouping); E5's limits (Linux headless, not Brave on Windows);
  the README paragraph's claims and the tests behind them (`case-test-result.json`; 2.1a's rows-test-f1 row
  `j_letter_case`); what is NOT proven.
- **infra-engineer** (the move against CI and the gate): `.github/workflows/gate.yml`'s base logic (push: `BEFORE`; pull
  request: `origin/$BASE_REF`) on the move and on its revert (E8); the runs' headers; the Playwright cache; that a pull
  request cut before the move still gates; the merge order with PR #40 (four shared docs files) and #48 (none); the
  Windows step (`node tools/gate.mjs origin/main` on this branch).
