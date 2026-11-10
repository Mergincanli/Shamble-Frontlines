# Plan: the foundation, steps 2.1 and 2.2 (revision 0, draft; the inventory sections are being filled)

Status: draft, not yet reviewed by the advisor agent. This session plans only: no code, no file moves.
Source: Mustafa's decision of 2026-11-10 and his task list, word for word in `mustafa-2026-11-10.md` (this folder).
Base: main 44446a5. Branch of this plan: `claude/advisor-fable-4hxq72` (Mustafa's personal account; one account per
branch). Everything below is from reading at 44446a5 (grep and read only); nothing was moved or run after a move.

## Part 1. Step 2.1, the restructure, as its own change-set

### 1.1 Goal, scope, change type
- **Goal:** the five legacy files (`shamble-core.js`, `shamble-game.js`, `shamble-editor.js`, `index.html`,
  `editor.html`) move with `git mv` into `legacy/`, bytes unchanged, so `legacy/index.html` still runs by double-click
  (ADR 0002 part 1); today's gate, the legacy oracle and every command that names them follow; nothing else changes.
- **Not in 2.1:** no v2 file (no `Cargo.toml`, `package.json`, `crates/`, `packages/`: that is 2.2), no game-code
  change, no edit inside `spikes/`, no guard or settings or CI edit, no rewrite of reports, research or ROADMAP history.
- **Change type:** a move plus tool and docs edits; risk tier 2 (CLAUDE.md, `.claude/` files, gate files, and "a file
  renamed or moved from a path outside tier 0", advisor.md:63-64). Merge route: `needs-mustafa`, on his go in a day
  session, after the advisor's APPROVE of the final head, with his Brave check first (he plays the moved game).
- **Branch:** a `cs/foundation-restructure` stand-in on this account (one account per branch), from main once PR #46
  has merged, or from main now if 2.1 builds beside #46 (Part 4); the plan's slug for its checkpoint:
  `foundation-restructure`.

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
| `CLAUDE.md` | 5 (+ :19, :21-22, :52 without a name) | STALE :19 ("It moves to `legacy/` in phase 1"), :41-42 (Layout lists the five at the root); NAME :43, :71, :73, :98, :214-219 (`<dir>` placeholders); rule text :46, :21-22, :52, :149-159 (Part 5) | **edit** :19 → "It lives in `legacy/`."; :41 → "`legacy/`: `shamble-core.js` (engine) ← …"; :42 → "`legacy/index.html` (player build) · `legacy/editor.html` (authoring build) · …"; :46 and the scope line only as Mustafa answers HC-PATH and SCOPE |
| `.claude/agents/verifier.md` | 1 (+ :28 with the folder `.`) | COMMAND :28 `node tools/preview.js . eval tools/sim.js` (preview.js:93 reads `<dir>/shamble-core.js`: fails after the move); NAME :18-21 | **edit** :28 `.` → `legacy` (or Q-A4's fallback) |
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
| `tools/smoke.mjs` | yes (gate) | 4 | BREAKS :18, :118, :201 (default root = the repo root; the CLI runs `runSmoke()` with it); NAME the page names | **edit**: default root = `<repo>/legacy` |
| `tools/battery.js` | yes (gate) | 5 | NAME; `readBuild(dir)` :64-68 and its CLI :210-213 take a folder; the gate passes strings | none |
| `tools/preview.js` | no | 4 | COMMAND :2 (`preview.js . <cmd>`); code takes the folder (:17, :93-94) | edit the comment |
| `tools/pageshot.js` | no | 3 | COMMAND :8 (`pageshot.js . index.html`); code takes the folder (:19, :36) | edit the comment |
| `tools/village.py` | no | 2 | BREAKS :10 (default `<repo>/shamble-core.js`); COMMAND :2 | **edit** :10 → `os.path.join(_REPO, 'legacy', 'shamble-core.js')`; :2 |
| `tools/README.md` | no | 8 (+ :22, :24-27, :58 without a name) | COMMAND :22 (`battery.js .`), :23-28, :56, :58 (`preview.js .`, `pageshot.js .`); NAME the rest | **edit** `.` → `legacy` |
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

<!-- PART-2.1-INVENTORY-DOCS -->

### 1.7 Reviewers (the final diff; one round, one fix pass; 3 at a time)
The advisor (plan, then every pushed head) and security-auditor (every risk-tier-2 change) always; test-engineer (is
the move proven: E1-E6); infra-engineer (CI and the gate's behaviour on old bases); then the verifier until PASS.
No art-director: nothing a player sees changes (bytes unchanged), and Mustafa's Brave check covers it.

### 1.8 What Mustafa checks in Brave (ADR 0002's foundation checklist, step 1)
1. Before: if E5 says saves do not carry over, open the editor in your current copy, Maps, and for each map you want,
   Export, then Copy and save the text (`editor.html:1058`, one map at a time; there is no import button, so your old
   copy keeps working as the way back to them). If E5 says they carry over, skip this.
2. Download the branch as a zip, unzip it to a new folder, double-click `legacy/index.html`. Expected: the menu opens;
   Deploy starts a match; you can move, shoot and pause, as before.
3. Double-click `legacy/editor.html`. Expected: the editor's menu opens; Map editor and Maps open, and your saved maps
   are listed (or, if E5 said they do not carry over, the list is empty and nothing is lost: your old copy still has
   them).
4. Say "done" or "not yet" for each step.

### 1.9 Risks
- **A tool or command missed by the grep** → the dry run (step 1) and E6 run every changed command once; the critic's
  sweep (`inv-critic.md`) looked for names built at run time, globs and relative paths.
- **The gate reads the wrong copy** (a stray root copy beside `legacy/`) → the head reads `legacy/` only; optional:
  the gate fails when a root copy of any of the five reappears (a proposal for the advisor, Q-A5's neighbour).
- **Old bases break the gate** (a PR cut before the move, or 8e58937's CS-1 layout) → the base fallback chain and E3's
  three bases.
- **Saves seem lost in Brave** → E5 before the check; the old folder keeps its storage; nothing is deleted.
- **Collisions** → Part 4: no file of PR #46's current list, no `spikes/` file, no PM row of ADR 0001, 0004 or 0006.
- **The PR Advisor routine is paused** until #46 merges → 2.1 is `needs-mustafa` anyway and merges on his go.

### 1.5 Evidence (it must not lean on the edited gate)
advisor.md:90-91 and :153-156: CI's green does not stand when a gate file changes; advisor.md:150-152: a day-session
advisor reproduces with main's copy of a changed tool, and main's `gate.mjs` reads the five files at the root, so it
cannot read 2.1's head. So the proof comes in three independent parts, each a command whose output goes into the report:
- **E1, the bytes did not change.** `git diff --find-renames=100% --name-status <base> HEAD` lists exactly five `R100`
  rows for the five files, and `git rev-parse <base>:<f>` equals `git rev-parse HEAD:legacy/<f>` for each (same blob).
- **E2, main's untouched gate passes on those bytes.** In a throwaway worktree of main under `.gate/foundation/`, with
  the five files checked out from 2.1's head (`git show HEAD:legacy/<f> > <f>`; by E1 they are main's own bytes), run
  main's `node tools/gate.mjs origin/main`: PASS, symbols +0 -0. Plus CI's green on main itself for the same blobs.
- **E3, the edited gate still bites.** In throwaway worktrees of 2.1's head: the four Phase 0 mutations
  (`docs/reports/2026-10-13-phase-0.md:25-27`) and check 5's 22 mutations with its 3 controls
  (`docs/reports/2026-10-14-gate-check-5.md:30-41`), each planted in `legacy/`, each failing exactly the checks those
  reports name; the controls `--allow-drop refreshRect` and "a pure move" pass. Also the edited gate against three
  bases: main 44446a5 (root layout), 8e58937 (CS-1's `shamble-frontlines-repo/` layout) and 2.1's own head (`legacy/`).
- **E4, the oracle (PM-0002-36).** The dump with `--pre=tools/tuning-dump.js` for both maps (default and
  `--map=tri`), at base with folder `.` and at head with folder `legacy`, twice each: all four head runs byte-identical
  to the base runs; then R7's builder (`build-tuning.py` at a715be2, run unchanged except its input and output paths,
  into `.gate/foundation/`) on the head dumps gives files `cmp`-equal to `docs/legacy-tuning.json` and
  `docs/LEGACY_TUNING.md`. Whether this is the mark's reading is the advisor agent's ruling (Q-A1 below).
- **E5, the saves.** A headless Chromium test with one persistent profile: a page at `<tmp>/index.html` writes a
  KVStore key, a page at `<tmp>/legacy/index.html` reads it; result recorded as carried over or not. UNKNOWN until run.
- **E6, the smoke and the commands.** The full gate with smoke on the head (5a: both pages); every command the edit list
  changes, run once as written (`preview.js legacy shot`, `pageshot.js legacy index.html`, `village.py` with no
  arguments, `battery.js legacy`); `node .claude/scripts/repo-map-check.mjs` exit 0; the kit tests unchanged and
  passing with main's counts (`node --test .claude/hooks/*.test.mjs .claude/scripts/checkpoint.test.mjs`).
- **E7, Mustafa in Brave** (ADR 0002's foundation checklist, step 1): below, 1.8.

### 1.6 Steps (each one commit, the gate after each; checkpoint `foundation-restructure`)
0. Re-read PR #46's file list; confirm no overlap with 1.3's list (or hold the overlapping file).
1. **Dry run** in a throwaway worktree under `.gate/foundation/`: `git mv` the five files, run today's gate and the
   smoke, list every failure. Nothing is committed from it; it confirms the inventory (grep only so far) or adds to it.
2. **The move and the code that reads it** (one commit, so the gate passes at every commit): `git mv` of the five
   files into `legacy/`; `tools/gate.mjs` and `tools/smoke.mjs` (gate files: each edit asks Mustafa) read the head from
   `legacy/` only; `tools/village.py`'s default path. The gate with smoke PASS. `git diff -M100%` still shows the five
   renames as `R100` beside the tool edits (E1).
3. **The commands in text** (one commit): `tools/README.md` and the tool header comments (`preview.js . ` → `legacy`).
4. **The kit lines** (one commit, each edit asks Mustafa): CLAUDE.md and verifier.md (1.3).
5. **E1 to E6**, outputs into the checkpoint notes.
6. **The docs** (docs-keeper): 1.3's docs list, REPO_MAP, STATUS, ROADMAP; the register's OQ-0002-2 row and ADR 0002's
   OQ-0002-2 entry ("answered by the restructure plan").
7. **Reviews** on the final diff (1.7), one fix pass, the report, the checkpoint deleted in the last commit, PR ready.

## Part 2. Step 2.2, the toolchain: a one-page sketch

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
| `.gitignore` | adds `target/`, `pkg/`, `dist/`, `coverage/`, `playwright-report/`, `test-results/`, proptest's and insta's pending files (no line naming the spikes folder: see the rule below) | `.gitignore` | part 1, R5-LAYOUT-REC6 |
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
  (b) 2.2 ships a one-step setup check that 2.3's G0 absorbs. Proposed: (a), for the advisor to rule.
- **Q2.2-b, the guard does not protect the new pins.** `protectedRel` (run 2026-11-10) returns false for root
  `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `package.json`, `package-lock.json`, `deny.toml`, `.cargo/
  config.toml`. Whether they join the guard's gate files is a guard edit, after PR #46 merges (it rewrites the guard),
  most naturally with 2.3, when the gate v2 starts reading them.
- **Q2.2-c, which Node.** Part 2 says "the way the legacy toolkit pins 22". Node's release table
  (github.com/nodejs/Release, read 2026-11-10; "Dates are subject to change") lists 22.x as "Maintenance LTS",
  end-of-life "2027-04-30", and 24.x as "Maintenance LTS" since "2026-10-20", end-of-life "2028-04-30"; the line now in
  active LTS was not read. Options: (a) 22, one Node for legacy and v2, with a bump change-set before April 2027;
  (b) a newer LTS for v2 (read the active line first), legacy stays on 22. Read as "pin the major the way TL17 does",
  part 2 allows either. A toolchain choice for the advisor; Mustafa only if it escalates.
- **Q2.2-d, the offline proof.** PM-0002-37's network cut-off tool is proposed, not researched (a network namespace).
  2.2's plan reads how a GitHub runner can run a step with no network before it promises the mark.

**Rules 2.2 must keep because PR #40's spike is in the tree** (from `spikes/netcode-proof/check.mjs` on PR #40):
- No tracked non-Markdown file outside `spikes/`, `docs/`, `.checkpoints/` may contain the text `spikes/`
  (`referenceProblems`): so the root `Cargo.toml` takes `members = ["crates/*"]` and no `exclude` line, the root
  `package.json` takes `packages/*`, and `.gitignore`, ESLint and Prettier configs name what they cover, never what
  they skip.
- No root `.cargo/config.toml`: cargo reads config files in every folder above the one it runs in, and the spike's
  check reads them too (`cargoConfigFiles`, `cargoConfigProblems`): a root `rustflags`, alias, `runner` or `include` key
  would change or fail the spike's builds. Put the target folder and flags on the CI command line instead.
- Must be checked in 2.2's build, not assumed: `cargo metadata` from `spikes/netcode-proof/` still names the spike's
  own workspace once a root workspace exists (the spike's `Cargo.toml` has its own `[workspace]`), and
  `npm ci --prefix tools` still installs the legacy toolkit from `tools/package-lock.json` once a root `package.json`
  with workspaces exists.

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

### PR #46 (kit-v3-5, the guard's rename gap; university account)
- **Where it stands** (its branch `claude/sleepy-tesla-ofsxsr`, read 2026-11-10 at cb6636d): a draft at step 0, his
  words saved, no plan yet; it will edit `.claude/hooks/routine-guard.mjs` and its tests, advisor.md's tier-2 bracket,
  STATUS, ROADMAP and the kit-v3-4 report. Its file list is not final until its plan exists.
- **His words make it first:** "the guard fix, agent memory included, is the very next change-set. Until it merges,
  I'm pausing the PR Advisor routine" (GUARD-RENAME (c), 2026-11-09, in its checkpoint).
- **Can 2.1 build beside it?** Yes, on files #46 does not name. With the legacy tools left in `tools/` (OQ-0002-2 (a),
  Part 1), 2.1 edits no guard file, no `.claude/settings.json`, no `.github/` file and no guard test, and leaves
  advisor.md alone (its `shamble-*.js`, `index.html`, `editor.html` stay true as names). The only shared files are
  STATUS and ROADMAP, which 2.1's docs pass touches last.
- **When 2.1 merges:** after #46. Three reasons: his words make #46 the next change-set; 2.1 is mostly renames and #46
  rewrites how the guard's merge check reads a renamed file ("reads both names of a renamed file", its F1), so the fixed
  check should be the one that judges 2.1; and the PR Advisor routine stays paused until #46 merges anyway (2.1 is
  `needs-mustafa` in any case: kit edits and a Brave check).
- **Before 2.1 builds:** re-read #46's file list once its plan exists; if it names any file in 2.1's edit list (Part 1),
  that file's edit waits until #46 merges. Before 2.1's docs pass: merge main (with #46) into 2.1's branch, in a
  worktree, or in the main folder with the guard's ask (CLAUDE.md, Never: only `--ff-only` merges of `origin/main` run
  there without one).

### PR #40 (the netcode proof test; university account)
What 2.1 changes for it, and what #40 must do once 2.1 is on main:
1. **Merge main into its branch** before its own merge (in a worktree, as its notes already do). Expect conflicts in
   `docs/REPO_MAP.md` and `docs/STATUS.md` only: #40 adds the `spikes/` rows and its status; 2.1 adds the `legacy/` rows
   and moves the five files' rows. Keep both, then run `node .claude/scripts/repo-map-check.mjs` and the gate.
2. **Nothing in `spikes/` changes.** Its `check.mjs` `referenceProblems` reads the moved files under `legacy/` (non-
   Markdown, outside `spikes/`): they contain no `spikes/` text, so the check still passes; `check.test.mjs`'s fixtures
   use the old names as data only (lines 1263-1283 on its branch).
3. **Its gate command stays** `node tools/gate.mjs --no-smoke` (the tools stay in `tools/`); after the merge the gate
   reads `legacy/` and its base is the post-move main. Its notes cite `tools/gate.mjs:34` (FILES): that line moves in
   2.1; notes are records and need no change, but a new citation should read the new line.
4. **Its marks file is safe if 2.1 leaves the PM rows alone:** `marks.json` hashes PM rows of ADR 0001, 0004 and 0006
   by their text. 2.1's docs pass edits no `| PM-` row in those three ADRs (Part 1 checks each hit).
5. **If #40 merges first:** 2.1 merges main and resolves the same two files; nothing else changes.

## Part 5. Questions

### For Mustafa (rule text only he can settle; each asked, never read into)
- **HC-PATH.** CLAUDE.md:46's hard constraint reads "the game runs by double-clicking index.html (file://)"; after 2.1
  the page is `legacy/index.html`. Hard constraints change "ONLY through an accepted ADR", and ADR 0002 is Proposed.
  Options: (a) 2.1 changes the line to `legacy/index.html` (the same rule at its new path), covered by your go of
  2026-11-10 for 2.1; (b) leave the line (the name stays true) until ADR 0002 is accepted; (c) 2.1 waits for ADR 0002's
  acceptance. Recommended: (a).
- **SCOPE.** Your SPIKE-SCOPE (a) reads "once the drafts are approved, the legacy constraints apply to `legacy/` only"
  (STATUS:745), while STATUS:14-15 says the drafts are "signed off as the basis for the spikes, not yet accepted". It
  matters for 2.2: its Rust and TypeScript modules break the constraints as written if they apply to the whole
  repository ("classic scripts sharing globals", "Three.js r128", "No addons"), and CLAUDE.md:52 ("Phase 0 writes no v2
  code"), CLAUDE.md:21-22, advisor.md:98 ("No engine code") and NIGHT_SHIFT.md:54-55 still forbid it. Options: (a) your
  go of 2026-11-10 means SPIKE-SCOPE (a) has fired: 2.1's CLAUDE.md edit scopes the hard constraints to `legacy/`, and
  2.2's own kit edit adds "foundation steps 2.1 and 2.2 (Mustafa, 2026-11-10)" as the exception to those four lines;
  (b) the constraints stay repository-wide until ADR 0002 is accepted, and 2.2 waits. Recommended: (a).
- **MODEL-RULE.** Your decision overrides CLAUDE.md's model rule on this account "until that rule is updated". Options:
  (a) 2.1's CLAUDE.md sitting adds your words to the rule, in their own commit; (b) its own small kit change-set after
  #46; (c) leave it until a later kit pass. Recommended: (a), since 2.1 already opens CLAUDE.md in a kit sitting.

### For the advisor agent (readings and choices it may make)
- **Q-A1, PM-0002-36's reading.** E4 (dumps at `.` and `legacy` byte-identical, then R7's builder reproduces
  `docs/legacy-tuning.json` and `docs/LEGACY_TUNING.md` byte for byte) against the mark's words ("reproduces
  `docs/legacy-tuning.json` byte for byte", with a command that lacks `--pre`).
- **Q-A2, OQ-0002-2's three parts** (its decider: "the phase 1 restructure plan (the main session with docs-keeper)"):
  the legacy tools: (a), stay in `tools/`; the legacy docs (row KT21): 1.4; the gate v2's script: deferred to 2.3's
  plan, which must also teach the guard the gate v2's files.
- **Q-A3, G0.** Q2.2-a: the "fails G0 with exit 2" mark moves to 2.3.
- **Q-A4, verifier.md:28.** Edit the kit line (proposed), or make `preview.js` fall back to `legacy/` when the folder
  holds no game files (no kit edit, but a silent fallback in a repro tool).
- **Q-A5, the gate's base fallback.** One prefix per base where all five files exist (proposed), not a per-file mix.
