# Facts for the foundation plan's inventory agents (the session's, 2026-11-10; trust these, do not re-derive)

## The move (ADR 0002 part 1; migration plan 2.1)
- The five legacy files `shamble-core.js`, `shamble-game.js`, `shamble-editor.js`, `index.html`, `editor.html` move
  together into `legacy/` with `git mv`, bytes unchanged, so their relative `<script src>` paths still work.
- Assumed layout for the inventory: OQ-0002-2 option (a): the legacy gate and tools STAY in `tools/` under their current
  names and learn the `legacy/` path. Note per file what would differ under (b) (legacy tools inside `legacy/`) only
  where it matters.
- Done-when: today's gate passes on `legacy/`; the four Phase 0 mutation proofs still fail on the right checks
  (`docs/reports/2026-10-13-phase-0.md:25-27`: deleted `refreshRect` → [3] DROPPED; `bootEditor();` in `bootGame` → [4]
  FLAGGED; a syntax error → [1]; `wrType` copied into core → [3] duplicate); PM-0002-36 (the tuning oracle);
  Mustafa starts a match from `legacy/index.html` by double-click in Brave.

## How the tools find the files (read at main 44446a5)
- `tools/gate.mjs:33-35`: ROOT = the repo root; FILES at the root; `baseFile` (:94-96) tries `<f>` then
  `shamble-frontlines-repo/<f>` (CS-1's earlier move: the precedent for a fallback chain); `headFile` (:97-100) reads
  ROOT/<f>; `classicParts` (:150) resolves `<script src>` against ROOT; `runSmoke({ root: ROOT })` (:190).
- `tools/smoke.mjs:17-24, 118, 130`: ROOT = the repo root, `runSmoke({ root = ROOT })`.
- `tools/preview.js:17, 93-94`, `tools/pageshot.js:19, 36`, `tools/battery.js:65-67, 211`: take the build folder as an
  argument (callers pass `.` today; after the move, `legacy`).
- `tools/village.py:10`: defaults to `<repo>/shamble-core.js`.
- `tools/tuning-dump.js:3`, `sim.js`, `reach.js`, `perfbots.js`, `aabbs.js`: comments give `node tools/preview.js . eval ...`.

## The guard (kit) and the gate files
- A file is a kit or gate file when the guard's exported `protectedRel` says so (`.claude/hooks/routine-guard.mjs:32`).
  Run on the hit list (2026-11-10): PROTECTED = `.claude/agents/advisor.md`, `netcode-architect.md`, `verifier.md`,
  `.claude/hooks/routine-guard.test.mjs`, `.claude/hooks/settings.test.mjs`, `.claude/rules/engine-core.md`,
  `engine-editor.md`, `engine-game.md`, `pages.md`, `.claude/skills/gate/SKILL.md`, `.claude/skills/render-ab/SKILL.md`,
  `CLAUDE.md`, `tools/gate.mjs`, `tools/smoke.mjs`, `tools/battery.js`. Not protected: everything else on the list,
  including `tools/README.md`, `tools/preview.js`, `tools/pageshot.js`, `tools/village.py`, every doc, the five files.
  Each edit to a PROTECTED file asks Mustafa in a day session.
- `GATE_FILES` (routine-guard.mjs:26-27) = tools/gate.mjs, smoke.mjs, battery.js, gate-globals.json, package.json,
  package-lock.json; `DENIED_SCRIPTS` (:437) = tools/preview.js, tools/pageshot.js (routines may not run them at all).
- `.claude/rules/*.md` path globs are `**/shamble-core.js`, `**/index.html`, `**/tools/**` etc.: they already match
  `legacy/...`.
- `.claude/settings.json` allow rules name `node tools/gate.mjs *`, `node tools/preview.js *` etc. (unchanged under (a)).
- `.github/workflows/gate.yml` runs `npm ci --prefix tools`, `node tools/gate.mjs "$BASE" --no-smoke`,
  `node tools/gate.mjs --smoke-only` (unchanged under (a)).

## The tuning oracle (PM-0002-36)
- `docs/legacy-tuning.json` is not the dump's raw output: R7's builder `.checkpoints/r7-legacy-inventory/build-tuning.py`
  (readable at a715be2) builds it and `docs/LEGACY_TUNING.md` from the two raw dumps (`tuning/cabana-1.json`,
  `tuning/tri-1.json`, also at a715be2); its `where` fields read `git show 48ffb51:shamble-{core,game}.js` (a fixed old
  commit). The JSON's `about.produced_by` records `node tools/preview.js . eval ...`: 2.1 must not edit
  `docs/legacy-tuning.json` or `docs/LEGACY_TUNING.md`, or the byte-for-byte check loses its reference.

## Other sessions (never push there; never touch `spikes/`)
- PR #46 (kit-v3-5, draft, no plan yet on 2026-11-09): will edit `.claude/hooks/routine-guard.mjs` and its tests,
  advisor.md's tier-2 bracket, STATUS, ROADMAP, the kit-v3-4 report. Its file list is not final.
- PR #40 (netcode proof test): changes `docs/REPO_MAP.md`, `docs/STATUS.md`, `docs/design/*`, `spikes/netcode-proof/`,
  `.checkpoints/netcode-proof-test*`. Its `spikes/netcode-proof/check.mjs` `referenceProblems` fails when a tracked
  non-Markdown file outside `spikes/`, `docs/`, `.checkpoints/` contains the text `spikes/`; its marks.json hashes
  PM rows of ADR 0001, 0004 and 0006.

## Classes to use (one per hit or group of hits)
- BREAKS: code or config that fails or reads the wrong file after the move (must change in 2.1).
- COMMAND: a command in text that would fail if run as written after the move (e.g. `preview.js . eval`).
- STALE: current-truth prose that becomes false (e.g. "double-click index.html", a layout listing root files).
- NAME: a bare file name that stays true after the move (a basename, `shamble-*.js`): no change.
- CITATION: `file:line` evidence recorded at a commit (git mv keeps line numbers): no change, at most a one-line note.
- HISTORY: reports, ROADMAP entries, FIRST_PROMPTS, research records of what was true then: no change.
- TESTDATA: a test fixture that uses the names as data and does not read the real files: no change.
