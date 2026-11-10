# Plan: the foundation, steps 2.1 and 2.2 (revision 3)

Status: revision 3. The advisor agent reviewed revision 2 (8bbebfa): APPROVE WITH CONDITIONS, for 2.1 only;
conditions C1-C8 are applied here (Part 5 records its rulings). Revision 3 itself is not reviewed: the advisor checks
C1-C8 at 2.1a's PR head. 2.2 is a sketch and needs its own plan and plan review. This session plans only: no code, no
file moves.
Source: Mustafa's decision of 2026-11-10 and his task list, word for word in `mustafa-2026-11-10.md` (this folder).
Base: main 44446a5. Branch of this plan: `claude/advisor-fable-4hxq72` (Mustafa's personal account; one account per
branch). Everything below is from reading at 44446a5 (grep and read only); nothing was moved or run after a move,
except one control: R7's tuning builder rerun on its own dumps (E4).

## In one screen
- **2.1, the restructure:** the five legacy files move into `legacy/` with `git mv`, bytes unchanged. 63 files name
  them (728 hits); after reading each, 2.1 edits 2 gate files (`tools/gate.mjs`, `tools/smoke.mjs`), 2 kit files
  (`CLAUDE.md` 3 lines, `.claude/agents/verifier.md` 1 line), `tools/village.py`, `tools/README.md`, 7 tool header
  comments, the root `README.md`, and 9 docs (REPO_MAP, START_HERE, MODULE_MAP, TECHNICAL, STATUS, ROADMAP, the
  register, ADR 0002 and SPIKE_PLAN; the last two only with pointers to his decision and OQ-0002-2's answer);
  everything else stays true as a name, a pinned citation or history. No guard, settings, CI or `spikes/` file
  changes.
- **Two pull requests:** 2.1a teaches the gate both layouts (nothing moves), 2.1b moves the files. One PR would make
  its own revert fail CI (the rollback rule needs a pure revert with CI green).
- **OQ-0002-2** (this plan decides it, the ADR says): the legacy tools stay in `tools/`; the legacy docs stay in
  `docs/`; the gate v2's path is left to 2.3. Each choice costs the fewest path changes.
- **Proof that does not trust the edited gate:** same blobs before and after (E1); main's untouched gate passes on
  the moved bytes (E2); the edited gate fails the same 26 mutations as main's (E3); the tuning oracle rebuilt byte for
  byte (E4); a saves test (E5); a rehearsed revert stays green (E8); Mustafa plays `legacy/index.html` in Brave.
- **2.2, the toolchain:** pin now Rust 1.99.1 (the same as the netcode spike), the Cargo and npm workspaces, TypeScript
  6.0.x, Node's major, the test and lint tool versions; empty stub crates and packages. The boundary spike's answers
  (wasm-bindgen, one Wasm instance or two, isolation, generated types, build sizes) stay empty slots.
- **2.3-2.6** wait for the netcode proof test (L1, the protocol messages), the sim core spike (replay and hash), the
  boundary spike (types, the boundary layer, the Worker layout) and the preview hosting check; 2.3 also waits for #46.
- **PR #46 and his queue (STATUS-DIET, LANE-RULES):** 2.1a shares no file with them and can be built now; 2.1b shares
  CLAUDE.md, STATUS, REPO_MAP and ROADMAP; both merge after them, on his go.
  **PR #40:** after each of 2.1a and 2.1b merges, it merges main and keeps both sides of STATUS (and, after 2.1b,
  REPO_MAP); nothing else changes for it.
- **Approval:** the advisor agent approved 2.1 with conditions (applied); 2.1a may be built now, but its merge and
  2.1b's kit sitting wait for FREEZE-FDN and SPLIT. 2.2 needs its own plan and review.
- **For Mustafa:** FREEZE-FDN (does KIT-FREEZE-2 cover the foundation's own edits?), SPLIT, SCOPE (before 2.2's merge)
  (Part 5).

## Part 1. Step 2.1, the restructure, as its own change-set

### 1.1 Goal, scope, change type
- **Goal:** the five legacy files (`shamble-core.js`, `shamble-game.js`, `shamble-editor.js`, `index.html`,
  `editor.html`) move with `git mv` into `legacy/`, bytes unchanged, so `legacy/index.html` still runs by double-click
  (ADR 0002 part 1); today's gate, the legacy oracle and every command that names them follow; nothing else changes.
- **Not in 2.1:** no v2 file (no `Cargo.toml`, `package.json`, `crates/`, `packages/`: that is 2.2), no game-code
  change, no edit inside `spikes/`, no guard or settings or CI edit, no rewrite of reports, research or ROADMAP history.
- **Change type:** a move plus tool and docs edits; risk tier 2 (CLAUDE.md, `.claude/` files, gate files, and "a file
  renamed or moved from a path outside tier 0", advisor.md:63-64). Merge route for each PR: `needs-mustafa`, on his
  go in a day session after the advisor's APPROVE of its final head, as FREEZE-FDN allows; 2.1b with his Brave check
  first (he plays the moved game).
- **Two pull requests, one after the other** (Q-A6): a revert must be "the exact inverse of the merge" with "CI green
  on its head" (advisor.md:285-290). If one PR held both the gate edit and the move, its revert would bring back
  today's gate, whose base reader knows only the root and `shamble-frontlines-repo/` (gate.mjs:92-95), while CI's base
  (`origin/main`, gate.yml:42-49) would hold the files in `legacy/`: the gate dies with exit 2, on the revert PR and on
  main after it (the critic's F1). So:
  - **2.1a, the tools learn both layouts** (the two gate files, `village.py`, the gate-globals wording, and his
    2026-11-10 decision recorded on main; nothing moves): merged first. A revert of it is plain (the files are still
    at the root).
  - **2.1b, the move** (the five files, the kit lines, the commands in text, the docs): merged second. A revert of it
    leaves 2.1a's gate, which reads a `legacy/` base and a root head, so CI stays green (rehearsed: E8).
- **Hand-over (who builds, on which branch, where the plan lives):** this account builds both (his decision; one
  account per branch). A cloud session pushes only to its own `claude/…` branch, which stands in for
  `cs/foundation-restructure`. This plan lives at `.checkpoints/foundation/plan.md` on `claude/advisor-fable-4hxq72`,
  whose commits touch only `.checkpoints/`. 2.1a builds either on this branch, if this session continues (its first
  build commit moves the plan to `.checkpoints/foundation-restructure/plan.md` and the planning checkpoint
  `.checkpoints/foundation.md` becomes the build checkpoint), or on a new session's branch cut from main, whose first
  commit copies the plan from this branch's last commit (its SHA is in the final report). 2.1a's last commit deletes
  the checkpoint folder (CHECKPOINTS §9), so 2.1a's report records the plan's last SHA; 2.1b then starts the same branch
  again from main once 2.1a has merged (by merge commit), by a fast-forward to main, never a force-push (the cloud
  session's own instructions say a merged PR's branch restarts from main; no repo rule says so), and copies the plan
  back from that SHA as its step 0.

### 1.2 Every file that names a legacy path
Method: `git grep -E 'shamble-(core|game|editor)\.js|shamble-\*\.js|shamble-\*|shamble-\{|index\.html|editor\.html'`
over the tracked tree outside `.checkpoints/` at 44446a5: **63 files, 728 hits**; plus a full read of tools/, CI, the
hooks, scripts and settings, the kit files and the five files themselves for references a name grep misses (folder
arguments such as `preview.js .`, defaults, globs). **Kit or gate file** = the guard's own `protectedRel`
(`.claude/hooks/routine-guard.mjs:32`), run on the list: 15 of the 63 are protected, and each edit to one asks
Mustafa in a day session. Every row is from reading (grep and read only; nothing was moved or run after a move):
step 1 of 1.6, a dry-run move, confirms it. Classes: BREAKS (fails or reads the wrong file), COMMAND (a command in
text that fails as written), STALE (current prose that becomes false), NAME (a basename that stays true), CITATION
(`file:line` at a commit; `git mv` keeps line numbers), HISTORY (a record of what was true then), TESTDATA (names used
as data, the real files never read). Details per hit: `inv-code.md`, `inv-kit.md`, `inv-docs.md`, `inv-critic.md`
in this folder.

**Kit files (all protected: an edit asks Mustafa)**
| File | Hits | Class | 2.1 does |
|---|---|---|---|
| `CLAUDE.md` | 5 (+ :19, :21-22, :52 without a name) | STALE :19 ("It moves to `legacy/` in phase 1"), :41-42 (Layout lists the five at the root); NAME :43, :71, :73, :98, :214-219 (`<dir>` placeholders); rule text :46, :20-22, :52-53 (Part 5, SCOPE); :149-159 the model rule (LANE-RULES') | **edit** :19 → "It lives in `legacy/`."; :41 → "`legacy/`: `shamble-core.js` (engine) ← …"; :42 → "`legacy/index.html` (player build) · `legacy/editor.html` (authoring build) · …"; :46 stays untouched (SCOPE); the scope line is 2.2's |
| `.claude/agents/verifier.md` | 1 (+ :28 with the folder `.`) | COMMAND :28 `node tools/preview.js . eval tools/sim.js` (preview.js:93 reads `<dir>/shamble-core.js`: fails after the move); NAME :18-21 | **edit** :28 `.` → `legacy` if FREEZE-FDN allows kit lines; else a known-stale note in STATUS (Q-A4) |
| `.claude/agents/advisor.md` | 3 | NAME :67, :91, :156 (`shamble-*.js`, `index.html`, `editor.html` still name the files) | none (and #46 edits this file) |
| `.claude/agents/netcode-architect.md` | 1 | NAME :21 | none |
| `.claude/rules/engine-core.md`, `engine-game.md`, `engine-editor.md`, `pages.md` | 2, 2, 3, 4 | NAME: their `paths:` globs are `**/<name>` and already match `legacy/<name>`; prose names | none |
| `.claude/skills/gate/SKILL.md` | 2 | NAME (`node tools/gate.mjs` stays; the gate learns `legacy/`) | none; note: 2.1 edits the gate tools to follow the move, not to fix a failure (:18), and E3 is the proof tools.md:11-12 asks for |
| `.claude/skills/render-ab/SKILL.md` | 1 | NAME (`<dir>` placeholders) | optional note: for a base from before the move, base `<dir>` is `.` and head `legacy` |
| `.claude/hooks/routine-guard.test.mjs` | 15 | TESTDATA (fake paths, throwaway repos) | none (and #46 edits this file) |
| `.claude/hooks/settings.test.mjs` | 1 (+ :41 with `.`) | TESTDATA (:232 a non-kit path; :41 a command string matched against allow rules, still allowed) | none |
Checked by full read, nothing to change: `.claude/CHECKPOINTS.md`, `NIGHT_SHIFT.md` (but :54-55 is rule text, Part 5),
`claude-security-guidance.md`, `security-patterns.json`, the other agent files, `rules/tools.md`, `rules/netcode.md`,
`skills/advisor-report`, `skills/changeset`, `.claude/settings.json` (its `tools/` allow rules stay true under (a)),
`hooks/guard.mjs`, `hooks/routine-guard.mjs` (`protectedRel` never matches `legacy/`), `hooks/syntax-gate.mjs`
(path-free), `scripts/*` (`repo-map-check.mjs` needs no edit; REPO_MAP does, 1.3).

**Code, CI, hooks and root files**
| File | Protected | Hits | Class | 2.1 does |
|---|---|---|---|---|
| `tools/gate.mjs` | yes (gate) | 24 | BREAKS :33-35 (ROOT and FILES at the root), :96-100 (`headFile` reads ROOT/<f>), :150 (`classicParts` resolves `<script src>` against ROOT), :190 (`runSmoke({ root: ROOT })`); NAME the rest (check labels, 5d's `WANT` names relative to the page) | **edit** (1.3) |
| `tools/smoke.mjs` | yes (gate) | 4 | BREAKS :18, :118, :201 (default root = the repo root; the CLI runs `runSmoke()` with it); NAME the page names | **edit** (2.1a): default root = the gate's `BUILD`, one shared rule (1.3) |
| `tools/battery.js` | yes (gate) | 5 | NAME; `readBuild(dir)` :64-68 and its CLI :210-213 take a folder; the gate passes strings | none |
| `tools/preview.js` | no | 4 | COMMAND :2 (`preview.js . <cmd>`); code takes the folder (:17, :93-94) | edit the comment |
| `tools/pageshot.js` | no | 3 | COMMAND :8 (`pageshot.js . index.html`); code takes the folder (:19, :36) | edit the comment |
| `tools/village.py` | no | 2 | BREAKS :10 (default `<repo>/shamble-core.js`); COMMAND :2 | **edit** :10 (2.1a): `legacy/shamble-core.js` when it exists, else the root one; :2 doc (2.1b) |
| `tools/README.md` | no | 8 (+ :22, :24-27, :58 without a name) | COMMAND :22 (`battery.js .`), :23-28, :56, :58 (`preview.js .`, `pageshot.js .`); STALE :35 (gate-globals wording); NAME the rest | **edit**: :35 in 2.1a; `.` → `legacy` in 2.1b |
| `tools/sim.js`, `reach.js`, `perfbots.js`, `tuning-dump.js`, `aabbs.js` | no | 0 (a folder `.` each) | COMMAND in the header comment (`preview.js . eval …`) | edit the comment |
| `README.md` | no | 6 | STALE :12 ("Double-click `index.html`"), :13, :19-22 (the layout table) | **edit** → `legacy/…` |
| `index.html`, `editor.html`, the three `.js` files | no | 4, 7, 3, 4, 6 | NAME (headers, comments); `<script src>` relative (index.html:1256-1257, editor.html:1257-1259); three.js from its CDN URL; no link, fetch or asset path between files | **move only** (bytes unchanged) |
| `FIRST_PROMPTS.md` | no | 5 | HISTORY | none |
| `.claude/hooks/routine-guard.test.mjs`, `settings.test.mjs` | yes | 15, 1 | TESTDATA (above) | none |
Checked by full read, nothing to change: `.github/workflows/gate.yml` (no paths; `node tools/gate.mjs "$BASE"` stays,
and on 2.1's own PR its base is the root layout, so the base fallback must read the root), `tools/package.json`,
`tools/gate-globals.json`, `.gitignore`, `.gitattributes` (`* text=auto eol=lf` covers `legacy/` too),
`docs/ui_theme_mockup.html` (self-contained), `.claude/settings.json`, `.claude/hooks/*.mjs`, `.claude/scripts/*.mjs`.
Found beyond the grep: the night shift runs only tracked scripts equal to main's (routine-guard.mjs:959-984), so after
2.1 merges a night item on a branch cut before 2.1 can no longer run its own `tools/gate.mjs`: such branches are
re-cut from main (2.1's report checks the Night queue, issue #16, for such items; the legacy items there are paused).

**Docs (none protected; ADRs, the register, design and look docs keep his go, tier 1)**
| File | Hits | Class | 2.1 does |
|---|---|---|---|
| `docs/LEGACY_INVENTORY.md` | 238 | CITATION 226 (pinned to 48ffb51's lines, :25-26), NAME 12; :389-390 a recorded run | none |
| `docs/LEGACY_TUNING.md`, `docs/legacy-tuning.json` | 60, 60 | CITATION (the `where` fields, from 48ffb51); :9 and `about.produced_by` hold `preview.js .` as produced | none: frozen (the oracle's reference) |
| `docs/EDITOR_V1_INVENTORY.md` | 39 | CITATION 37 (pinned to 67cf55b), NAME 2 | none |
| `docs/TECHNICAL.md` | 34 | NAME; CITATION :96; STALE by omission §9 :238-239 | edit §9 (1.4) |
| `docs/research/r1`, `r2`, `r4`, `r5`, `UNREAD_SOURCES.md` | 26, 15, 6, 8, 3 | 31 are URLs ending in `/index.html` (NAME); CITATION 25; HISTORY 2 | none (research records) |
| `docs/MODULE_MAP.md` | 20 | NAME | one line (1.4) |
| `docs/START_HERE.md` | 16 (+ :104, :114 with `.`) | STALE :12, :21-22; COMMAND :104, :114-116 | edit (1.4) |
| `docs/REPO_MAP.md` | 8 | STALE :14-18 (the check fails on them); NAME :100-102 | edit (1.4) |
| `docs/adr/0002-…md` | 8 | already in after-move terms (:138-139, :955, :1405, :1431); quotes (:56, :951) | a pointer to his decision at step 1 (2.1a); OQ-0002-2's answer (2.1b) |
| `docs/adr/0003-rendering.md` | 7 | CITATION (line numbers kept), one inside PM-0003-47 (not hashed by #40) | none |
| `docs/STATUS.md` | 7 | HISTORY, CITATION; the risk at :987 is wrong under `git mv` | his decision and :1009 closed (2.1a); 2.1's row, :987 corrected (2.1b) |
| `docs/adr/0001`, `0004`, `0006` | 1 each | CITATION: quotes of CLAUDE.md:46, outside every PM row | none |
| `docs/adr/0005-…md` | 2 | CITATION | none |
| `docs/ART_DIRECTION.md`, `docs/EDITOR_V2.md`, `docs/ROADMAP.md` | 2, 1, 1 | NAME (EDITOR_V2:94 is the v2 editor's rule; ROADMAP:3 the banner) | none (ROADMAP gets 2.1's entry) |
| `docs/reports/*` (14 files) | 39 | HISTORY | none |
Total 603 docs hits, every one read in context by the docs agent (`inv-docs.md`).

### 1.3 The edits, file by file (layout: OQ-0002-2 (a))
**Moved, bytes unchanged:** `shamble-core.js`, `shamble-game.js`, `shamble-editor.js`, `index.html`, `editor.html` →
`legacy/`. Game symbols +0 −0 (the gate's check 3 lists a pure move as "moved").

**Gate files (each edit asks Mustafa):**
2.1a (the tools learn both layouts; nothing moves):
- `tools/gate.mjs`: add one top-level name, `BUILD`: the folder that holds all five files, `legacy/` or the root; if
  both hold them, or neither, the gate stops with exit 2 and says which (two copies must never be judged by guess).
  `headFile` reads `BUILD/<f>`; `baseFile` tries `legacy/<f>`, then `<f>` (a base from before the move), then
  `shamble-frontlines-repo/<f>` (CS-1's era), `legacy/` first so a later root file of the same name is never read as
  legacy; `classicParts` resolves `<script src>` against `BUILD` and keeps the containment check under it; the smoke
  runs with `root: BUILD`; the base line and the header comment name the folder. Unchanged: `ROOT` (git and `TOOLS`),
  `FILES`, `OLD_PREFIX`, `toBuild`, checks 5c and 5d (their names are relative to the page). Removed names: none.
  One prefix per base (Q-A5, ruled): try `legacy/`, the root, then `shamble-frontlines-repo/`, take the first holding
  all five files, never mix within one base; none: exit 2 naming the base and the prefixes tried; the chosen prefix is
  printed on the base line and in `--json` (adds `BASE_PREFIXES` and `basePrefix`).
- `tools/smoke.mjs`: `runSmoke`'s default root is the gate's `BUILD`, by one shared rule (the folder that holds all
  five files; both or neither: exit 2), not a separate "both pages" test (the advisor's L6); its CLI follows; `ROOT`,
  `ROOT_URL` and `rel` stay; the header's "outside the repo" → "outside the build folder" (a page may read only files
  under its own folder: stricter under `legacy/`, and the pages read no sibling file).
- `tools/village.py`: its default core path is `legacy/shamble-core.js` when that exists, else the root one.
- The deferred Low of kit-v2-1 (STATUS:1009, "fix the wording when gate.mjs is next touched"): gate.mjs:333 and
  tools/README.md:35 call `gate-globals.json` "reviewed data, not a gate-tool change", against kit-v2-1's category 3
  (it is a gate file: his go). 2.1a fixes both lines (e.g. "a gate file: a change needs Mustafa's go"), and
  docs-keeper closes the STATUS line.
- `tools/battery.js`: none (it takes the folder as an argument).
- Symbols: the gate gains `BUILD`, `BASE_PREFIXES` and `basePrefix`; the five game files are untouched (+0 −0).

2.1b (the move): the five files with `git mv`; the edits below. The gate files are not touched again.

**Not protected (2.1b):** `tools/README.md` (every `.` folder argument → `legacy`; :21 "outside the repo" →
"outside `legacy/`"; one line saying the legacy build lives in `legacy/`), `tools/village.py`'s doc line :2, the header
comments of `preview.js`, `pageshot.js`, `sim.js`, `reach.js`, `perfbots.js`, `tuning-dump.js`, `aabbs.js`, and the
root `README.md` (:12, :13, :19-22).

**Kit lines (2.1b; each asks Mustafa):** `CLAUDE.md` :19, :41, :42 as in 1.2 (:46 stays untouched);
`.claude/agents/verifier.md:28` (`.` → `legacy`); all of them only as FREEZE-FDN allows. The model rule is LANE-RULES'
and the scope line is 2.2's (SCOPE).

**Docs:** 1.4.

**Never in 2.1:** the text `spikes/` in any of these edits (PR #40's `referenceProblems`); `docs/legacy-tuning.json`
and `docs/LEGACY_TUNING.md` (the oracle's reference); a PM row of ADR 0001, 0004 or 0006.

### 1.4 OQ-0002-2's three parts, and the docs
ADR 0002:998-1007: "Where do the legacy gate and tools live after the move, and where does the gate v2's script
live?", decided by "the phase 1 restructure plan (the main session with docs-keeper)", by "which layout keeps
R5-LAYOUT-REC6's marks ... with the fewest path changes, and where the legacy docs go (row KT21)". The marks hold in
either layout (0002:1358), so the path counts decide:
| Part | Proposed | Cost of the other option |
|---|---|---|
| The legacy gate and tools | **(a) stay in `tools/`**, learning `legacy/`: 2 gate files and 2 kit files edited (1.3) | (b) inside `legacy/`: the guard's `GATE_FILES`, `DENIED_SCRIPTS`, `CANON` and its npm check (routine-guard.mjs:26-28, :71, :437, :534: PR #46's file), 17 `tools/` allow rules in `.claude/settings.json`, `gate.yml`'s `--prefix tools` and both gate steps, and kit lines in about 8 more files (inv-kit B10) |
| The legacy docs (KT21: START_HERE, MODULE_MAP, TECHNICAL, GAME_DESIGN, ART_DIRECTION, ANIMATION_PLAN, AUDIT_BUGS_PERF, `ui_theme_mockup.html`) | **stay in `docs/`**, marked legacy (REPO_MAP already labels most; one banner line where 2.1 edits anyway): 0 path changes | to `legacy/docs/`: 27 lines in 12 files, 8 of them protected, 2 relative links broken (GAME_DESIGN.md:69, :90), and about 25 references in history and ADRs left dangling. KT21's mark is "redesign": START_HERE gets rewritten for v2, not archived. The R7 files (LEGACY_INVENTORY, LEGACY_TUNING, legacy-tuning.json, EDITOR_V1_INVENTORY) stay in `docs/` either way: ADRs cite them and PM-0002-36 names `docs/legacy-tuning.json` |
| The gate v2's script | **deferred to 2.3's plan**, which edits the guard anyway to teach it the gate v2's files: either the gate v2 gets its own name beside `tools/gate.mjs`, or it takes `tools/gate.mjs` and the legacy gate moves then. Either way after PR #46 | deciding now would edit the guard in 2.1 |

**The docs edits** (not protected; docs-keeper; 2.1a's step 4 records his decision, 2.1b's step 10 does the rest):
- `docs/REPO_MAP.md`: :14-18 (the five root rows) → one `legacy/` row, which also says that `file:line` citations keep
  their numbers after the move (`shamble-game.js:N` means `legacy/shamble-game.js:N`); :6's date. Without this,
  `repo-map-check.mjs` exits 1 (stale rows, `legacy/` missing).
- `docs/START_HERE.md`: :12 (the double-click rule names `legacy/index.html`), :21-22 ("The five code files sit at the
  repo root" → "in `legacy/`"), the commands at :104, :114, :115, :116 (`.` → `legacy`).
- `docs/MODULE_MAP.md`: one line above the file table (all five live in `legacy/`).
- `docs/TECHNICAL.md`: §9's file inventory (:238-239) says `legacy/`; :10 optional.
- `docs/STATUS.md`: in 2.1a, his 2026-11-10 decision (step 4) and the deferred gate-globals Low at :1009 closed; in
  2.1b, 2.1's in-flight row, and the risk at :987 ("The inventory's line numbers go stale once phase 1 moves the legacy
  files") corrected: `git mv` keeps bytes and line numbers. (In `docs/DECISIONS.md` and the split files if STATUS-DIET
  lands first.)
- `docs/ROADMAP.md`: 2.1's entry; its banner at :3 optional.
- `docs/adr/README.md` (the register) and ADR 0002's OQ-0002-2 entry: the answer above, and the register records
  PM-0002-36's reading (Q-A1) (decision docs: his go).
- `docs/SPIKE_PLAN.md` and ADR 0002's migration step 1: only 2.1a's dated pointers to his decision (step 4).
- Not edited: `docs/legacy-tuning.json` and `docs/LEGACY_TUNING.md` (their `produced_by` and :9 commands keep `.`, as
  produced at 48ffb51), every report, ROADMAP's history, the research files, LEGACY_INVENTORY and EDITOR_V1_INVENTORY
  (226 and 37 citations, pinned to their commits), the ADRs' quotes of CLAUDE.md:46 (ADR 0001:1067, 0002:951,
  0004:840, 0006:806; none in a PM row), ADR 0002's own text (it already speaks of `legacy/index.html`), EDITOR_V2.md:94
  (the v2 editor's rule, OQ-0002-1), ART_DIRECTION and GAME_DESIGN (names that stay true).
- PM rows: of ADR 0001's 63, 0004's 75 and 0006's 32, none contains a legacy path, so PR #40's hashed rows stay
  byte-identical.

### 1.5 Evidence (it must not lean on the edited gate)
advisor.md:90-91 and :153-156: CI's green does not stand when a gate file changes. A day-session advisor runs the PR
head's own gate in a worktree (advisor.md:149-150), but for a PR that edits the gate that run is not independent: the
judge is the thing under change. So the proof comes in parts that do not trust the edited gate, each a command whose
output goes into the report.

The independent judge is pinned by SHA: **OLD = 44446a5** here, or the main that 2.1a branches from if a later merge
(PR #46, PR #40) lands first; 2.1a's checkpoint records the exact SHA, and its gate files must equal 44446a5's (no
open PR touches them). After 2.1a merges, `origin/main` holds the edited gate, so "main's gate" is no longer
independent. In 2.1a the base is OLD; in 2.1b it is the main that holds 2.1a (call it A), and the judge is still
OLD's gate.

- **E1, the bytes did not change** (2.1b). `git diff --find-renames=100% --name-status A HEAD` lists exactly five
  `R100` rows for the five files, and `git rev-parse A:<f>` equals `git rev-parse HEAD:legacy/<f>` for each (same blob;
  A's five blobs equal OLD's, since 2.1a moves nothing).
- **E2, OLD's untouched gate passes on the moved bytes** (2.1b). In a throwaway worktree of 2.1b's head under
  `.gate/`, OLD's gate files are written into `legacy/tools/` (`git show 44446a5:tools/<x> > …/legacy/tools/<x>` for
  gate.mjs, smoke.mjs, battery.js, gate-globals.json, package.json, each named; `node_modules` linked, which prompts
  once: the allow rule at settings.json:74 covers one folder depth only). That gate takes its root as the folder above
  its own (gate.mjs:32-33, smoke.mjs:17-18), so it reads the moved bytes as its head and OLD's root paths as its base:
  `node …/legacy/tools/gate.mjs 44446a5` must PASS with symbols +0 -0 and the smoke passing.
- **E3, the edited gate bites exactly like OLD's.** Two worktrees: OLD with its own gate (files at the root), and the
  PR's head with the edited gate. One script in the notes plants each mutation in both (an exact string edit, asserted
  to hit once), runs `node tools/gate.mjs 44446a5 --json`, records the failing check ids (not their message text: the
  smoke's messages name `legacy/…` on one side), and restores the file with `git show HEAD:<path> > <path>` (`git
  checkout` and `git restore` are denied). The mutations: the four Phase 0 ones (`docs/reports/2026-10-13-phase-0.md:
  25-27`) and check 5's 22 (`docs/reports/2026-10-14-gate-check-5.md:30-41`); the controls: `--allow-drop refreshRect`,
  a pure move, and check 5's 3. Pass: for every mutation the two failing sets are equal and not empty; every control
  passes in both. (The report's grouping is loose, e.g. its "5b + 5d" pairs two mutations; a difference from that
  wording is a note, not a fail.) Run in 2.1a with the files at the root on both sides, and again in 2.1b with them in
  `legacy/` on the head side. Also the edited gate against three bases: OLD (root layout), 8e58937 (CS-1's
  `shamble-frontlines-repo/` layout) and its own head.
- **E4, the oracle (PM-0002-36; the advisor accepted this reading, Q-A1).** The dump with `--pre=tools/tuning-dump.js`
  for both maps (default and `--map=tri`), at base with folder `.` and at head with folder `legacy`, twice each, with
  A's copies of `preview.js` and `tuning-dump.js` for both runs (2.1b edits their header comments): all four head runs
  byte-identical to the base runs; then R7's builder (`build-tuning.py` at a715be2, run unchanged except its input and
  output paths, into `.gate/foundation/`) on the head dumps gives files `cmp`-equal to `docs/legacy-tuning.json` and
  `docs/LEGACY_TUNING.md`. Control already run in this session (inv-code B7b): the builder on R7's own dumps of
  a715be2 reproduces both files today, byte for byte.
- **E5, the saves.** A headless Chromium test with one persistent profile (Playwright from `tools/node_modules`): a
  page at `<site>/index.html` writes a KVStore key (asserting `KVStore.backend === 'localStorage'`); after a relaunch,
  the same page reads it back (the control: else the test proves nothing); then `<site>/editor.html` (same folder),
  `<site>/legacy/index.html`, `<site>/legacy/editor.html` and an unrelated folder read it. Recorded: shared, per folder,
  or per file. UNKNOWN until run (CS-1's earlier move left the same question untested: `docs/reports/
  2026-10-13-repo-root.md:55`); headless Chromium on Linux is not Brave on Windows, so Mustafa's backup (1.8) stays
  whatever it says.
- **E6, the smoke and the commands.** The full gate with smoke on the head (5a: both pages); every command the edit list
  changes, run once as written (`preview.js legacy shot`, `pageshot.js legacy index.html`, `village.py` with no
  arguments, `battery.js legacy`); `node .claude/scripts/repo-map-check.mjs` exit 0; the kit tests unchanged and
  passing with main's counts (`node --test .claude/hooks/*.test.mjs .claude/scripts/checkpoint.test.mjs`).
- **E8, the rollback works the way CI would see it** (2.1b). In a throwaway worktree of A: `git merge --no-ff` of 2.1b's
  head (a local merge commit M, as GitHub would make it), then `git revert -m 1 --no-edit M` (the whole merge, as
  advisor.md:271-274 does it); on that tree, `node tools/gate.mjs M` (CI's base on the revert PR is the main that holds
  M) and `node tools/gate.mjs --smoke-only` must both PASS. No commit leaves the worktree. The merge and the revert
  write CLAUDE.md and verifier.md in that worktree, and the guard does not read `git revert` (STATUS, limit L4): the
  main session runs E8 itself, never a subagent, after one ask to Mustafa (advisor.md, Revert, step 2).
- **C7, every command under the guard of the day:** each evidence command is re-checked against the guard on main when
  it runs (PR #46 rewrites it first); a refusal is a finding for the checkpoint, never something to work around.
- **E7, Mustafa in Brave** (ADR 0002's foundation checklist, step 1): below, 1.8.

### 1.6 Steps (each one commit, the gate after each; checkpoint `foundation-restructure`)
**2.1a**
0. Re-read the open PRs' file lists (#46, STATUS-DIET, LANE-RULES, #40); a file one of them edits waits until it
   merges. Record OLD's exact SHA.
1. **Dry run** in a throwaway worktree under `.gate/`: `git mv` the five files, run today's gate and the smoke, list
   every failure (expected: exit 2, "shamble-core.js is missing from the working tree", gate.mjs:98). Nothing is
   committed from it; it confirms the inventory (grep only so far) or adds to it.
2. **The tools** (one commit; gate files: each edit asks Mustafa): `tools/gate.mjs`, `tools/smoke.mjs`,
   `tools/village.py`, and the gate-globals wording in gate.mjs:333 and tools/README.md:35 (1.3). The gate with smoke
   PASS on the root layout.
3. **Its evidence:** E3 on the root layout (the edited gate fails each of the 26 mutations exactly as OLD's gate
   does); the edited gate on the dry-run worktree's `legacy/` layout PASS against OLD; the both-copies case stops with
   exit 2; the three bases of E3.
4. **His decision on main** (docs-keeper; decision docs, so his go): his words of 2026-11-10, verbatim, in STATUS's
   "Decisions pending", and a dated pointer to them in each line they overtake: ADR 0002's migration plan step 1
   ("The spikes before the foundation", 0002:1422-1425), SLICE-ORDER's condition 3 (STATUS:654: "The netcode proof
   test ... still runs before the foundation"), SPIKE_PLAN's order paragraph (:86-91, :115) and the foundation entry's
   "Depends on" (:429-432), STATUS's "Next" (in `docs/DECISIONS.md` instead if STATUS-DIET lands first). Until this
   lands, the only record is this plan's folder on a branch; and a revert of 2.1a would remove it again (its report
   says so).
5. **Reviews** (security-auditor, test-engineer, infra-engineer for the gate on old bases; the advisor), the report, PR
   ready, `needs-mustafa` (gate files and decision docs; no Brave check: nothing a player sees changes). Merged after
   PR #46 and his queue's items, on his go, once FREEZE-FDN allows it.

**2.1b** (from main with 2.1a merged)
6. **The move** (one commit, nothing else): `git mv` of the five files into `legacy/`. The gate with smoke PASS.
7. **The commands in text** (one commit): `tools/README.md` and the tool header comments (`.` → `legacy`), the root
   `README.md`.
8. **The kit lines** (one commit, each edit asks Mustafa): CLAUDE.md and verifier.md (1.3).
9. **E1 to E6, E8** (E3 again, now with the files in `legacy/` on the head side: PM-0002-35 is judged after the
   move), outputs into the checkpoint notes.
10. **The docs** (docs-keeper): 1.4's list; the register's OQ-0002-2 row and ADR 0002's OQ-0002-2 entry ("answered by
   the restructure plan").
11. **Reviews** on the final diff (1.7), one fix pass, the report, the checkpoint deleted in the last commit, PR ready;
    Mustafa's backup and Brave check (1.8), then his go.

### 1.7 Reviewers, models and effort (the final diff of each PR; one round, one fix pass; 3 agents at a time)
- **2.1a:** security-auditor (every risk-tier-2 change), test-engineer (is it proven: E3, the three bases, the
  both-copies stop), infra-engineer (CI and the gate on old bases); then the verifier until PASS; the advisor on every
  pushed head.
- **2.1b:** security-auditor, test-engineer (E1 to E6, E8), infra-engineer (the move against CI and the gate);
  docs-keeper for the docs; the verifier until PASS; the advisor on every pushed head. No art-director: nothing a player
  sees changes (bytes unchanged), and Mustafa's Brave check covers it.
- **Models and effort:** as CLAUDE.md's "Models and effort" section and his LANE-RULES (Part 5) say: the reviewer
  agents, the verifier and docs-keeper run from their agent files (`model: opus`, their stated effort); docs-keeper's
  ADR and register edits get `xhigh`; built-in and workflow agents get `model: 'opus'` and `effort: 'high'` passed
  explicitly; the advisor tool is an extra, never an agent. At most 3 agents at a time, the saver running while they
  do.

### 1.8 What Mustafa checks in Brave (ADR 0002's foundation checklist, step 1; 2.1b only)
0. Confirm you have this version: download the PR's branch as a zip from its GitHub page (Code, Download ZIP) and unzip
   it to a new folder. On screen, the folder holds `legacy/` with the game in it, and no `index.html` at its top.
1. Before (whatever E5 says, and before you take the merged main, whose top no longer has the game): back up your
   saves. In Brave, open your current `index.html`, press F12, Console (type `allow pasting` if asked), paste the line
   the session gives you (it copies every saved key as one text), and paste that into a text file outside the
   repository; do the same on your current `editor.html` (maps, prefabs, Character Lab). Expected: each text file
   starts with `{` and holds your keys (`shamble_prog`, `vmap:…` for each map). The editor's Export shows one map at a
   time and has no import (`editor.html:1058`; `:1248` is read-only), so the session also gives a restore line that
   puts back only missing keys. Your old copy keeps its saves either way: nothing is deleted. (Both console lines are
   in 2.1b's report, for security-auditor to read first.)
2. Double-click `legacy/index.html` in the new folder. Expected: the menu opens; Deploy starts a match; you can move,
   shoot and pause, as before.
3. Double-click `legacy/editor.html`. Expected: the editor's menu opens; Map editor and Maps open, and your saved maps
   are listed (or, if E5 said they do not carry over, the list is empty and nothing is lost: your old copy has them,
   and the restore line puts them back).
4. Say "done" or "not yet" for each step. If every step passes, say "merge #<n>" in a session's chat.

### 1.9 Risks
- **A tool or command missed by the grep** → the dry run (step 1) and E6 run every changed command once; the critic's
  sweep (`inv-critic.md`) looked for names built at run time, globs and relative paths.
- **The gate reads the wrong copy** (a stray root copy beside `legacy/`) → `BUILD` is the folder that holds all five;
  if both the root and `legacy/` hold them, the gate stops with exit 2 (1.3), tested in 2.1a's step 3.
- **Old bases break the gate** (a PR cut before the move, or 8e58937's CS-1 layout) → the base fallback chain and E3's
  three bases.
- **A revert of the move turns CI red** → 2.1a lands the both-layout gate first, so 2.1b's revert is pure and green
  (E8). A revert of 2.1a after 2.1b has merged would break the gate: revert 2.1b first (2.1b's report says so).
- **Saves seem lost in Brave** → E5 before the check; the old folder keeps its storage; nothing is deleted.
- **Collisions** → Part 4: 2.1a shares no file with the open PRs; 2.1b shares CLAUDE.md (with #46 and LANE-RULES),
  STATUS and REPO_MAP (with STATUS-DIET and #40) and ROADMAP, and merges after them; no `spikes/` file; no PM row of
  ADR 0001, 0004 or 0006.
- **The PR Advisor routine is paused** until #46 merges → 2.1 is `needs-mustafa` anyway and merges on his go.

## Part 2. Step 2.2, the toolchain: a one-page sketch

**Status:** a sketch only. The advisor agent's approval covers 2.1, not 2.2: 2.2 gets its own plan and plan review,
and its merge also waits for SCOPE and FREEZE-FDN.

**What 2.2 is** (ADR 0002 migration plan 2.2, part 2, part 3): the two workspaces and the pins; the first crates
`sim`, `protocol`, `room`, `client-endpoint`, `sim-wasm`, `devtools` and packages `client`, `sim-worker`, `net`,
`types`, `e2e`, each an empty stub with one trivial test. Done when (PM-0002-37, -38, and part of -39): from a fresh
clone on the CI runner, `cargo test --workspace --locked` and `npm ci && npm test --workspaces` pass with no network
after the install steps; `rustc --version` in CI equals the pin; a pull request that edits `Cargo.toml` without
`Cargo.lock` fails at the first cargo step; deleting the toolchain file fails G0 with exit 2.

**What it can pin now** (each a fact in the drafts, no spike needed):
| Pin | Value | Where | Source |
|---|---|---|---|
| Rust | one exact stable: 1.99.1, `profile = "minimal"`, `clippy`, `rustfmt`, target `wasm32-unknown-unknown` | root `rust-toolchain.toml` | ADR 0002 part 2; the netcode spike pins the same 1.99.1 (`spikes/netcode-proof/rust-toolchain.toml` on PR #40), so native and Wasm come from one compiler across both |
| Cargo | virtual workspace, `members = ["crates/*"]`, `resolver = "3"`, `[workspace.package] edition = "2024"`, `rust-version = "1.99.1"`, empty `[workspace.dependencies]`, `[workspace.lints]` (part 7: `unsafe_code = "deny"`, clippy `all` warn), `Cargo.lock` committed, `--locked` everywhere | root `Cargo.toml` | part 2, part 7 |
| Profiles | overflow checks on for `sim` and `protocol` in every profile | root `Cargo.toml` | part 4 |
| npm | workspaces `packages/*`, one root `package-lock.json`, exact versions (no `^`) | root `package.json` | part 2 |
| Node | major 22, the way the legacy toolkit pins it (`engines.node: "22.x"`, CI `node-version: 22`) | root `package.json`, CI | part 2 (TL17); see Q2.2-c |
| TypeScript | 6.0.x, one exact patch | root `package.json` | part 2 |
| Test tools | Vitest (Node) for the packages; Playwright Test for `e2e` (an empty spec) | root `package.json` | part 8 |
| Lint tools | ESLint 10, typescript-eslint, Prettier: versions pinned now; configs and enforcement are gate stages G1-G3 (2.3) | root `package.json` | part 7, part 9 |
| `.gitignore` | adds `target/`, `pkg/`, `dist/`, `coverage/`, `playwright-report/`, `test-results/`, proptest's and insta's pending files (no line naming the spikes folder: Appendix B) | `.gitignore` | part 1, R5-LAYOUT-REC6 |
| CI | one new workflow job: install the pinned toolchain with `rustup toolchain install` before any cargo call, then the two test commands with the network cut off | `.github/workflows/` | part 2; PM-0002-37's offline tool is "proposed, not researched" |

**What waits for the boundary spike** (each open question named; 2.2 leaves its slot empty):
- **OQ-0002-7 and the wasm-bindgen pin:** which wasm-bindgen version `sim-wasm` uses, the CLI-equals-lock check, and
  whether its new home is healthy (a by-day reading, not a spike). `sim-wasm` stays an empty crate without bindings.
- **OQ-0002-3, one Wasm instance or two:** decides `sim-wasm`'s exports and `sim-worker`'s shape; both stay stubs.
- **OQ-0002-4, isolation:** the isolation flag and the Vite headers; nothing in 2.2.
- **Spike item 4, generated types:** the three generators of `packages/types`; the package stays empty (2.4).
- **Spike item 8, size and tools:** `opt-level` "s" or "z", wasm-opt, `skipLibCheck` (R5-TSLINT-OQ6), the three.js
  type casts (R5-TSLINT-OQ4), Playwright's clock in the Worker (R5-TEST-OQ1): no build profile or tsconfig flag that
  these decide is set in 2.2.
- **OQ-0002-1, the editor from `file://`:** no `editor` package in 2.2 (it enters at slice 5).
- Not the boundary spike's, but also left empty: the sim's maths and geometry dependencies (the sim core spike, ADR
  0004), the protocol codec and the transport crates (the netcode proof test, ADR 0001; `server` enters at slice 3),
  the three.js version (ADR 0003; `client` gets it at 2.5).

**Open questions for 2.2's own plan** (not decided here):
- **Q2.2-a, G0 belongs to 2.3.** 2.2's done-when says "deleting the toolchain file fails G0 with exit 2", but G0 is a
  stage of the gate v2 (2.3, part 9). Options: (a) that mark moves to 2.3, where `--prove` runs it (PM-0002-39);
  (b) 2.2 ships a one-step setup check that 2.3's G0 absorbs. The advisor agent ruled (a) provisionally (Q-A3); final
  in 2.2's plan review.
- **Q2.2-b, the guard does not protect the new pins.** `protectedRel` (run 2026-11-10) returns false for root
  `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `package.json`, `package-lock.json`, `deny.toml`, `.cargo/
  config.toml`. Whether they join the guard's gate files is a guard edit, after PR #46 merges (it rewrites the guard)
  and inside KIT-FREEZE-2's window, so most naturally with 2.3, the gate step, when the gate v2 starts reading them.
- **Q2.2-c, which Node.** Part 2 says "the way the legacy toolkit pins 22". Node's release table
  (github.com/nodejs/Release, read 2026-11-10; "Dates are subject to change") lists 22.x as "Maintenance LTS",
  end-of-life "2027-04-30", and 24.x as "Maintenance LTS" since "2026-10-20", end-of-life "2028-04-30"; the line now in
  active LTS was not read. Options: (a) 22, one Node for legacy and v2, with a bump change-set before April 2027;
  (b) a newer LTS for v2 (read the active line first), legacy stays on 22. Read as "pin the major the way TL17 does",
  part 2 allows either. A toolchain choice for the advisor; Mustafa only if it escalates.
- **Q2.2-d, the offline proof.** PM-0002-37's network cut-off tool is proposed, not researched (a network namespace).
  2.2's plan reads how a GitHub runner can run a step with no network before it promises the mark.

**Rules 2.2 must keep while PR #40's spike is in the tree:** Appendix B (no text `spikes/` in root files, no root
`.cargo/config.toml`, no `"type": "module"` at the root, the rules' globs, two coexistence checks to run).

**Risk tier and route:** tier 2 (a new `.github/` workflow; root pins), so the advisor plus security-auditor,
infra-engineer, test-engineer, rust-reviewer and engine-architect on the final diff (3 at a time), and Mustafa's go in
a day session. If the boundary spike later fails and ADR 0002 is revised, 2.2's stubs are cheap to change: that is the
reason to keep them empty.

## Part 3. Steps 2.3 to 2.6: what each waits for

Mustafa (2026-11-10): "Steps 2.3 to 2.6 keep their order and wait for their results." Beyond the results, SPIKE_PLAN's
foundation entry also lists "the drafts accepted" (ADR 0002 becomes Accepted only after its spikes report, STATUS
Phase); his decision lifted that for 2.1 and 2.2 only.
| Step | Waits for | Why (the draft's words) |
|---|---|---|
| 2.3 gate v2 and CI | 2.2 merged; the **netcode proof test**'s L1 (G11 re-proves R5-TEST-REC4's L1 as a gate check); the **sim core spike**'s replay runner and state hash (G10 re-proves R5-TEST-REC2's tools; the sim's dependencies set G5's feature comparison); the **boundary spike** (G4 generated types, G9 the boundary layer, and part 9's WebGPU-in-CI route: "a CI job if the boundary spike finds one"); PR #46 merged (the guard must learn the gate v2's files); the gate v2's path (OQ-0002-2's third part, decided in 2.3's plan); Mustafa's branch-protection setting for the one required check | migration plan 2.3; SPIKE_PLAN foundation entry, "re-proves as gate checks marks first proven in the sim core spike ... and the netcode proof test" |
| 2.4 generated types | 2.3 (G4 runs in CI); the **boundary spike's item 4** (the three generators, the layout hash) | "Done when: the marks of the spike's item 4 hold in CI" |
| 2.5 client + local server skeleton | 2.4; the **netcode proof test** (ADR 0001's milestone: `Join`, `JoinAccepted`, `Input`, `Snapshot` over the loopback backend with L1); the **sim core spike** (ADR 0004's milestone: per-tick hashes natively twice and in Wasm); the **boundary spike** (OQ-0002-3 one instance or two; part 5's block B2 or B3, PM-0002-11; the pacing W2 or W3, OQ-0002-4's second trigger; PORTAL-EMBED's non-isolated fallback); ADR 0003's backend naming (WebGPU or the WebGL 2 fallback) | migration plan 2.5 |
| 2.6 previews | 2.5; the **preview hosting check** (spike 6, which "must end before the foundation's preview step"; PREV-DELETE); B9's interim position in place and OQ-0006-13 answered before the first client-host token; Mustafa's deploy token in the `preview` environment; the boundary spike's isolation results (previews isolated only with the flag, PM-0002-68) | migration plan 2.6; ADR 0006 part 13 |

## Part 4. The other two pull requests

### PR #46 (kit-v3-5, the guard's rename gap; university account), and the queue behind it
- **Where it stands** (its branch `claude/sleepy-tesla-ofsxsr`, read 2026-11-10 at d7526f5): plan revision 3, an
  allow-list for the PR Advisor's merges; its build order writes the guard's test file, the guard, `advisor.md` and
  `CLAUDE.md` (plan-rev3.md §5), then STATUS, ROADMAP and the kit-v3-4 report. Its rev 3 already counts the
  foundation: "`legacy/` and the v2 folders are not allowed places. Their PRs wait for his go until the foundation's
  gate step adds them" (plan-rev3.md:54-55; that lane's reading, not his words).
- **His words make it first:** "the guard fix, agent memory included, is the very next change-set. Until it merges,
  I'm pausing the PR Advisor routine" (GUARD-RENAME (c), 2026-11-09).
- **His queue of 2026-11-10 behind it** (word for word in `.checkpoints/kit-v3-5/mustafa-2026-11-10-queue.md` and
  `mustafa-2026-11-10-answers.md` on that branch, d7526f5): KIT-FREEZE-2 ("After kit-v3-5 merges, the kit freezes
  again until the foundation's gate step. Exceptions: a broken main, and items 2 and 3 below."), STATUS-DIET (STATUS
  split into STATUS, DECISIONS and STATUS_ARCHIVE by a line-range script; REPO_MAP and CLAUDE.md's read list follow)
  and LANE-RULES (three CLAUDE.md edits: lanes, models, the read list), and ORDER (b): "STATUS-DIET and LANE-RULES
  first, then the unit walk". KIT-FREEZE-2 bears on 2.1's merges: FREEZE-FDN, Part 5.
- **Can 2.1 build beside them?** 2.1a, yes: its files (`tools/gate.mjs`, `tools/smoke.mjs`, `tools/village.py`,
  `tools/README.md`) are in none of those lists. 2.1b shares `CLAUDE.md` (with #46 and LANE-RULES), STATUS and REPO_MAP
  (with STATUS-DIET and #40) and ROADMAP; it touches neither `advisor.md` nor the guard.
- **Order of merges:** #46, then STATUS-DIET and LANE-RULES (his ORDER (b)), then 2.1a and 2.1b, each on his go
  (FREEZE-FDN decides whether they may merge before the gate step). Whichever of two PRs that share a file merges
  second brings main in and resolves. Three reasons for 2.1 after #46: his words make #46 next; 2.1 is mostly renames
  and #46 rewrites how the guard reads a renamed file, so the fixed check should judge 2.1; and the PR Advisor routine
  stays paused until #46 merges (2.1 is `needs-mustafa` either way).
- **If STATUS-DIET lands first:** 2.1a's record of his decision goes to `docs/DECISIONS.md`, not STATUS; the plan's
  STATUS line numbers (:654, :739-746, :987, :1009) are found again in the split files; and no 2.1 edit to STATUS lands
  while STATUS-DIET's line-range script is in flight.
- **Before each build step:** re-read the open PRs' file lists; a file another open PR edits waits until that PR
  merges. Bringing main into 2.1's branch: in a worktree, or in the main folder with the guard's ask (CLAUDE.md, Never:
  only `--ff-only` merges of `origin/main` run there without one).

### PR #40 (the netcode proof test; university account)
What 2.1 changes for it, and what #40 must do once each PR is on main. #40's own files outside `spikes/` are
`docs/REPO_MAP.md`, `docs/STATUS.md` and `docs/design/*`; its `check.mjs` reads only its marks' rows in ADR 0001, 0004
and 0006 (`marksProblems`, `expectedMarks`) and the non-Markdown files outside `spikes/`, `docs/`, `.checkpoints/`
(`referenceProblems`), so 2.1a's pointers in ADR 0002 and SPIKE_PLAN and 2.1b's OQ-0002-2 entry are not read by it.
1. **After 2.1a merges:** merge main into its branch before its own merge (in a worktree, as its notes already do);
   expect a conflict in `docs/STATUS.md` at most (2.1a's record of his decision, or `docs/DECISIONS.md` if STATUS-DIET
   came first). Its gate run then uses 2.1a's gate, which reads the root layout as before.
2. **After 2.1b merges:** merge main again; expect conflicts in `docs/REPO_MAP.md` and `docs/STATUS.md`: #40 adds the
   `spikes/` rows and its status, 2.1b adds the `legacy/` row and drops the five root rows. Keep both, then run
   `node .claude/scripts/repo-map-check.mjs` and the gate.
3. **Nothing in `spikes/` changes.** Its `check.mjs` `referenceProblems` reads the moved files under `legacy/` (non-
   Markdown, outside `spikes/`): they contain no `spikes/` text, so the check still passes; `check.test.mjs`'s fixtures
   use the old names as data only (lines 1263-1283 on its branch).
4. **Its gate command stays** `node tools/gate.mjs --no-smoke` (the tools stay in `tools/`); after the merge the gate
   reads `legacy/` and its base is the post-move main. Its notes cite `tools/gate.mjs:34` (FILES): that line moves in
   2.1; notes are records and need no change, but a new citation should read the new line.
5. **Its marks file is safe if 2.1 leaves the PM rows alone:** `marks.json` hashes PM rows of ADR 0001, 0004 and 0006
   by their text. 2.1's docs pass edits no `| PM-` row in those three ADRs (Part 1 checks each hit).
6. **If #40 merges first:** 2.1a and 2.1b merge main and resolve the same files; nothing else changes.

## Part 5. Questions

### For Mustafa (rule text and choices only he can settle; each asked, never read into)
Two rules frame SCOPE: CLAUDE.md's hard constraints change "ONLY through an accepted ADR signed off by Mustafa" (and
its Never list: "change a hard constraint without an accepted ADR"), and the advisor's Never list says "Merge a
hard-constraint change before its accepted ADR is on main" (advisor.md:313). ADR 0002 is Proposed. The model rule is
not asked: his LANE-RULES of 2026-11-10 settles it ("on my personal account, Fable 5.2 may be the session's advisor
through Claude Code's advisor tool. The main model and every agent stay Opus 5.5", with his addition "it is an extra.
The recorded plan review by the advisor agent stays required"); LANE-RULES makes that CLAUDE.md edit, not 2.1.
- **FREEZE-FDN.** Your KIT-FREEZE-2 (2026-11-10 06:33 UTC, after your foundation decision of the same day) reads "After
  kit-v3-5 merges, the kit freezes again until the foundation's gate step. Exceptions: a broken main, and items 2 and 3
  below." Does it also cover the foundation's own kit and gate edits? 2.1 cannot move the files without editing the
  gate, and 2.1 and 2.2 would merge inside the freeze. Options: (a) the foundation's minimum edits are exempt, each
  still asked and merged on your go: for 2.1, `tools/gate.mjs`, `tools/smoke.mjs`, CLAUDE.md :19 and :41-42,
  `verifier.md:28`; for 2.2, its CI job, the constraint-scope lines and the rules' globs; (b) gate files are exempt,
  but `.claude/`, CLAUDE.md and `.github/` stay frozen: 2.1b leaves those 3 kit lines stale until 2.3, and 2.2's CI
  marks move to 2.3; (c) everything is frozen: 2.1 then waits for 2.3, which needs 2.1 (a deadlock). Recommended: (a).
  Until you answer, 2.1a may be built but not merged, and 2.1b's kit sitting waits.
- **SPLIT.** You asked for 2.1 "as its own change-set". The rollback rule needs a pure revert with CI green
  (advisor.md:285-289), and a one-PR 2.1 cannot give that (1.1). Options: (a) two PRs under this one plan, still one
  change-set: 2.1a (the tools learn both layouts) then 2.1b (the move); two gos, two kit sittings (2.1a's gate files,
  2.1b's CLAUDE.md and verifier.md lines), and only 2.1b needs your Brave check; (b) one PR, accepting that if it breaks
  main, the fix is a planned fix-forward PR instead of a revert. Recommended: (a) (the advisor agent: "the reasoning
  holds").
- **SCOPE (HC-PATH folded in; needed before 2.2's merge, not for 2.1).** SPIKE-SCOPE (a), taken by the Advisor on
  2026-10-25 (STATUS:739-746), reads "once the drafts are approved, the legacy constraints apply to `legacy/` only";
  STATUS:14-15 says the drafts are "signed off as the basis for the spikes, not yet accepted". 2.2's Rust and
  TypeScript modules break the constraints if they apply to the whole repository ("classic scripts sharing globals",
  "Three.js r128", "No addons"), and CLAUDE.md:20-22 and :52-54, advisor.md:98 ("No engine code"),
  V2_PHASE0_BRIEF.md:20-21 and :144 (the Advisor's document: no session edits it) and NIGHT_SHIFT.md:54-55 still forbid
  v2 code. Scoping the constraints to `legacy/` is a hard-constraint change, so it needs an accepted ADR on main before
  it merges. Options: (a) you accept ADR 0002's part 1 alone now (the repository after the restructure, with "the
  legacy hard constraints then apply to `legacy/` only"), recorded in its Status line: the move then has an accepted
  ADR too, and 2.2's kit edit scopes the constraints and adds your 2026-11-10 decision as the exception to the "no v2
  code" lines; (b) 2.2 waits until ADR 0002 is accepted after the boundary spike; (c) you change the Never lines
  yourself for this one case. Recommended: (a). Either way 2.1b leaves CLAUDE.md:46 ("the game runs by
  double-clicking index.html (file://)", quoted word for word by ADRs 0001:1067, 0002:951, 0004:840, 0006:806)
  untouched: its words stay true by name and the Layout lines give the path, so advisor.md:313 cannot bite 2.1b.

### The advisor agent's rulings on this plan (revision 2, 8bbebfa: APPROVE WITH CONDITIONS, for 2.1 only)
Its notes: `advisor-plan-rev2.md` (this folder). 2.1a's build may start (the dry run, the three tool edits with their
asks, E3); both merges and 2.1b's kit sitting wait for FREEZE-FDN and SPLIT; 2.2 needs its own plan and plan review.
- **Q-A1, PM-0002-36:** E4 accepted as the mark's reading for 2.1b (dumps byte-identical over two runs each, then R7's
  builder at a715be2 reproduces both files with `cmp`; A's copies of `preview.js` and `tuning-dump.js` for both runs).
  The reading is recorded in the register (`docs/adr/README.md`) in 2.1b's docs pass (his go); the PM row's wording is
  fixed at ADR 0002's next revision, not in 2.1b (r5:1245 has the same command).
- **Q-A2, OQ-0002-2:** approved as proposed: tools stay in `tools/` (the fewest kit edits, which matters more under
  KIT-FREEZE-2), docs stay in `docs/`, the gate v2's path deferred to 2.3's plan.
- **Q-A3, G0:** provisionally (a): the "fails G0 with exit 2" mark moves to 2.3 (PM-0002-39 puts it under `--prove`);
  2.2 keeps "rustc --version in CI equals the pin". Final in 2.2's plan review, the done-when change in the register.
- **Q-A4, verifier.md:28:** follows FREEZE-FDN: edit the line if kit lines are allowed; if not, leave it and record it
  in STATUS as a known stale command (it fails loudly, ENOENT). Never a fallback in `preview.js`.
- **Q-A5, the base prefix:** one prefix per base: try `legacy/`, then the root, then `shamble-frontlines-repo/`, take the
  first that holds all five; if none, exit 2 naming the base and the prefixes tried; print the chosen prefix in the
  base line and in `--json`; never mix prefixes within one base.
- **Q-A6, two PRs:** the reasoning holds; recommend SPLIT (a); his choice. In its reading, two PRs under one plan are
  still one change-set (ADR 0002:1426: "one change-set per step").

## Appendix A. The 63 files that name a legacy file, by path
From `git grep -c` at 44446a5 (outside `.checkpoints/`); "Kit or gate" from the guard's `protectedRel`; the class of
each "none" row is in 1.2. Not in this list because no line names a file, yet edited in 2.1b: the header comments of
`tools/sim.js`, `reach.js`, `perfbots.js`, `tuning-dump.js`, `aabbs.js` (`preview.js .`), and register
`docs/adr/README.md`'s OQ-0002-2 row; `docs/SPIKE_PLAN.md` gets 2.1a's pointer to his decision.

| # | Path | Hits | Kit or gate | 2.1 does |
|---|---|---|---|---|
| 1 | `.claude/agents/advisor.md` | 3 | yes | none (NAME) |
| 2 | `.claude/agents/netcode-architect.md` | 1 | yes | none (NAME) |
| 3 | `.claude/agents/verifier.md` | 1 | yes | edit (2.1b, as FREEZE-FDN allows): :28 `.` → `legacy` |
| 4 | `.claude/hooks/routine-guard.test.mjs` | 15 | yes | none (TESTDATA) |
| 5 | `.claude/hooks/settings.test.mjs` | 1 | yes | none (TESTDATA) |
| 6 | `.claude/rules/engine-core.md` | 2 | yes | none (NAME) |
| 7 | `.claude/rules/engine-editor.md` | 3 | yes | none (NAME) |
| 8 | `.claude/rules/engine-game.md` | 2 | yes | none (NAME) |
| 9 | `.claude/rules/pages.md` | 4 | yes | none (NAME) |
| 10 | `.claude/skills/gate/SKILL.md` | 2 | yes | none (NAME) |
| 11 | `.claude/skills/render-ab/SKILL.md` | 1 | yes | none (NAME) |
| 12 | `CLAUDE.md` | 5 | yes | edit (2.1b, as FREEZE-FDN allows): :19, :41-42; :46 untouched |
| 13 | `FIRST_PROMPTS.md` | 5 | no | none (HISTORY) |
| 14 | `README.md` | 6 | no | edit (2.1b): :12-13, :19-22 |
| 15 | `docs/ART_DIRECTION.md` | 2 | no | none (NAME) |
| 16 | `docs/EDITOR_V1_INVENTORY.md` | 39 | no | none (CITATION) |
| 17 | `docs/EDITOR_V2.md` | 1 | no | none (NAME) |
| 18 | `docs/LEGACY_INVENTORY.md` | 238 | no | none (CITATION) |
| 19 | `docs/LEGACY_TUNING.md` | 60 | no | none: frozen (oracle reference) |
| 20 | `docs/MODULE_MAP.md` | 20 | no | edit (2.1b): one line above the table |
| 21 | `docs/REPO_MAP.md` | 8 | no | edit (2.1b): :14-18, :6 |
| 22 | `docs/ROADMAP.md` | 1 | no | edit: entries (history untouched) |
| 23 | `docs/START_HERE.md` | 16 | no | edit (2.1b): :12, :21-22, :104, :114-116 |
| 24 | `docs/STATUS.md` | 7 | no | edit: 2.1a his decision; 2.1b row, :987 |
| 25 | `docs/TECHNICAL.md` | 34 | no | edit (2.1b): §9 :238-239 |
| 26 | `docs/adr/0001-multiplayer-netcode.md` | 1 | no | none (CITATION) |
| 27 | `docs/adr/0002-code-structure-build-toolchain.md` | 8 | no | edit: OQ-0002-2 answer (2.1b), step-1 pointer (2.1a); his go |
| 28 | `docs/adr/0003-rendering.md` | 7 | no | none (CITATION) |
| 29 | `docs/adr/0004-simulation-core.md` | 1 | no | none (CITATION) |
| 30 | `docs/adr/0005-content-abilities-assets.md` | 2 | no | none (CITATION) |
| 31 | `docs/adr/0006-infrastructure-operations.md` | 1 | no | none (CITATION) |
| 32 | `docs/legacy-tuning.json` | 60 | no | none: frozen (oracle reference) |
| 33 | `docs/reports/2026-10-13-phase-0.md` | 1 | no | none (HISTORY) |
| 34 | `docs/reports/2026-10-13-repo-root.md` | 3 | no | none (HISTORY) |
| 35 | `docs/reports/2026-10-14-bot-poplar-wedge.md` | 1 | no | none (HISTORY) |
| 36 | `docs/reports/2026-10-14-editor-prefab-list.md` | 4 | no | none (HISTORY) |
| 37 | `docs/reports/2026-10-14-editor-undo-ground.md` | 3 | no | none (HISTORY) |
| 38 | `docs/reports/2026-10-14-gate-check-5.md` | 5 | no | none (HISTORY) |
| 39 | `docs/reports/2026-10-14-map-dedupe-on-load.md` | 4 | no | none (HISTORY) |
| 40 | `docs/reports/2026-10-15-editor-v2-steps-0-1.md` | 1 | no | none (HISTORY) |
| 41 | `docs/reports/2026-10-15-team-prep-fixes.md` | 4 | no | none (HISTORY) |
| 42 | `docs/reports/2026-10-16-editor-hotfix.md` | 5 | no | none (HISTORY) |
| 43 | `docs/reports/2026-10-16-three-teams-core-steps-1-6.md` | 2 | no | none (HISTORY) |
| 44 | `docs/reports/2026-10-16-three-teams-core.md` | 4 | no | none (HISTORY) |
| 45 | `docs/reports/2026-10-17-kit-v1-4.md` | 1 | no | none (HISTORY) |
| 46 | `docs/reports/2026-10-24-kit-v3-0.md` | 1 | no | none (HISTORY) |
| 47 | `docs/research/UNREAD_SOURCES.md` | 3 | no | none (CITATION/URL/HISTORY) |
| 48 | `docs/research/r1-simulation-core.md` | 26 | no | none (CITATION/URL/HISTORY) |
| 49 | `docs/research/r2-netcode.md` | 15 | no | none (CITATION/URL/HISTORY) |
| 50 | `docs/research/r4-physics-navigation.md` | 6 | no | none (CITATION/URL/HISTORY) |
| 51 | `docs/research/r5-toolchain-quality.md` | 8 | no | none (CITATION/URL/HISTORY) |
| 52 | `editor.html` | 7 | no | move (2.1b), bytes unchanged |
| 53 | `index.html` | 4 | no | move (2.1b), bytes unchanged |
| 54 | `shamble-core.js` | 3 | no | move (2.1b), bytes unchanged |
| 55 | `shamble-editor.js` | 6 | no | move (2.1b), bytes unchanged |
| 56 | `shamble-game.js` | 4 | no | move (2.1b), bytes unchanged |
| 57 | `tools/README.md` | 8 | no | edit (2.1a :35 wording; 2.1b commands) |
| 58 | `tools/battery.js` | 5 | yes | none (NAME) |
| 59 | `tools/gate.mjs` | 24 | yes | edit (2.1a): both layouts; :333 wording |
| 60 | `tools/pageshot.js` | 3 | no | edit (2.1b): header comment |
| 61 | `tools/preview.js` | 4 | no | edit (2.1b): header comment |
| 62 | `tools/smoke.mjs` | 4 | yes | edit (2.1a): default root |
| 63 | `tools/village.py` | 2 | no | edit (2.1a :10 default; 2.1b :2 doc) |

## Appendix B. Rules 2.2 must keep while PR #40's spike is in the tree
- No tracked non-Markdown file outside `spikes/`, `docs/`, `.checkpoints/` may contain the text `spikes/`
  (`referenceProblems`): so the root `Cargo.toml` takes `members = ["crates/*"]` and no `exclude` line, the root
  `package.json` takes `packages/*`, and `.gitignore`, ESLint and Prettier configs name what they cover, never what
  they skip.
- No root `.cargo/config.toml`: cargo reads config files in every folder above the one it runs in, and the spike's
  check reads them too (`cargoConfigFiles`, `cargoConfigProblems`): a root `rustflags`, alias, `runner` or `include` key
  would change or fail the spike's builds. Put the target folder and flags on the CI command line instead.
- No `"type": "module"` in the root `package.json` (the critic's F2): the syntax hook runs `node --check` on the file
  in place (`.claude/hooks/syntax-gate.mjs:3, :21-23`), and Node takes a `.js` file's module type from the nearest
  `package.json`, so the legacy scripts would be checked as modules. Each package sets its own type (the gate itself is
  safe: it checks `.cjs` temp copies, gate.mjs:115).
- The legacy rules' globs (`.claude/rules/pages.md:3-4` `**/index.html`, `**/editor.html`; `engine-*.md:3`) will also
  match a v2 `index.html` once one exists, and load "edit both pages identically" on v2 files; CLAUDE.md:46's bare
  `index.html` becomes ambiguous then too. 2.2 (or 2.5, which brings the first v2 page) narrows them to `legacy/…`:
  a kit edit, his ask.
- Must be checked in 2.2's build, not assumed: `cargo metadata` from `spikes/netcode-proof/` still names the spike's
  own workspace once a root workspace exists (the spike's `Cargo.toml` has its own `[workspace]`), and
  `npm ci --prefix tools` still installs the legacy toolkit from `tools/package-lock.json` once a root `package.json`
  with workspaces exists.
