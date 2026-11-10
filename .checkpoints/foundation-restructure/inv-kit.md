TASK: foundation 2.1 inventory, kit area (hits-b-kit.txt 24 hits + full read of CLAUDE.md, .claude/agents, rules, skills, CHECKPOINTS, NIGHT_SHIFT, security-guidance, security-patterns.json). Read-only.

## Plan (blocks)
B1 read facts + hits (done at start)
B2 classify 24 hits
B3 CLAUDE.md full read
B4 .claude/agents/* full read
B5 .claude/rules/* + skills/*/SKILL.md
B6 CHECKPOINTS.md, NIGHT_SHIFT.md, security-guidance, security-patterns.json
B7 STATUS lines 14-15, 745 (rule text Q2)
B8 design answers 1-4
DONE marker at end

## Notes
- B1 done. next B3/B4 reads.
- B3 CLAUDE.md done. findings:
  - CLAUDE.md:19 "It moves to `legacy/` in phase 1." STALE after move (future tense). REQUIRED-ish: becomes false; also Q: is foundation 2.1 "phase 1"?
  - CLAUDE.md:21-22 "No engine code: throwaway spikes only, and only after Mustafa and the Advisor have approved the drafts; until then, stop." Q for Mustafa (2.1 is a move, 2.2 a toolchain).
  - CLAUDE.md:40-42 Layout lists the five at root beside docs/ tools/: STALE. REQUIRED (line 42 + 41).
  - CLAUDE.md:46 hard constraint "double-clicking index.html (file://)": basename still true (NAME), but only an ADR may change the constraint text; Q for Mustafa whether to add legacy/.
  - CLAUDE.md:52 "Phase 0 writes no v2 code" Q for 2.2.
  - CLAUDE.md:71,73 tools/preview.js eval, tools/sim.js: no folder; NAME true under (a).
  - CLAUDE.md:98 tier table "game code, pages": NAME.
  - CLAUDE.md:149-159 model rule vs Mustafa 2026-11-10 decision (Fable advisor tool): Q; not needed for move.
  - CLAUDE.md:214-218 commands take <dir>: still true; OPTIONAL hint "(legacy build: legacy)".
  - CLAUDE.md:219 tools/village.py: line true; village.py default (tools/village.py:10) BREAKS (not my area).
  - CLAUDE.md:211 setup unaffected. :43 layering NAME.
- B4a advisor.md done. findings:
  - advisor.md:67,91,156 `shamble-*.js`/`index.html`/`editor.html` basenames/globs: NAME, still true (legacy/shamble-core.js matches shamble-*.js as a name). No edit.
  - advisor.md:98 "No engine code. Throwaway spikes only, and only after Mustafa and you have approved the decision drafts." Q for 2.2 (and 2.1?).
  - advisor.md:99 legacy frozen: 2.1 moves bytes unchanged; Q whether a move counts as "critical fixes only".
  - Q4 lines: :39-42 cat 3 gate files; :55 highest tier; :63-64 moved file from outside tier 0 not tier 0; :67-70 tier 1; :71-77 tier 2 always needs-mustafa, kit/CI/gate only from day session, read --no-renames list; :91-92 step 2 trigger shamble-*.js/pages -> needs-mustafa verify in day session; :153-158 routine green doesn't stand when gate files change; :111 plan names tier + security-auditor; :227-234 tier-2 checklist (before/after rule table, kit tests same counts, his go); :182-184 Brave checklist format; :313 hard-constraint change needs accepted ADR on main first.
- B4b other agents done. findings:
  - verifier.md:28 `node tools/preview.js . eval tools/sim.js` COMMAND: after move `.` holds no game files -> REQUIRED edit `.` -> `legacy` (on a pre-move base worktree `.` stays right: note for the 2.1 PR's own base runs).
  - verifier.md:18-21 gate/battery fallback, no folder given: NAME, true. :21 "gate v2 arrives in phase 1": future, no edit.
  - netcode-architect.md:21 "fenced SIM block in shamble-core.js": NAME. perf-auditor.md:34 "tools/preview.js eval" no folder: NAME. test-engineer.md:16 tools/README.md, tools/gate.mjs: true under (a). art-director/engine-architect/game-designer/graphics-engineer: legacy as a word only, no paths.
- B5 rules + skills done. findings:
  - rules engine-core/editor/game.md:3 and pages.md:3-4 globs `**/<name>` match legacy/<name>: NAME, no edit (rule loading keeps working). tools.md:3 `**/tools/**` true under (a) and (b). netcode.md: no legacy paths.
  - rules prose (engine-core.md:5, engine-editor.md:5-6, engine-game.md:5, pages.md:7-9) basenames: NAME.
  - tools.md:6 "resolve dependencies relative to tools/": true under (a); under (b) still true inside legacy/tools.
  - gate/SKILL.md:6 `node tools/gate.mjs $ARGUMENTS` true under (a) (gate learns legacy/). :8-14 manual fallback basenames + `<dir>`: NAME. :18 "Never fix a failure by editing the gate tools": 2.1 edits gate tools to follow the move, not to fix a failure: note for plan (say so explicitly).
  - render-ab/SKILL.md:10-11 `<dir>` placeholders: true. Edge: for any PR whose base predates the move (2.1 itself, and branches cut before it), base `<dir>`=`.` and head `legacy`, so "identical commands" (:9) cannot hold literally: OPTIONAL note, not required.
  - advisor-report, changeset SKILL.md: no legacy paths.
- B6 done (CHECKPOINTS.md grep+headings: no legacy paths; security-guidance, security-patterns.json: none). findings:
  - NIGHT_SHIFT.md:27-29 handoff switch refused when branch's rules/CI/gate files differ from main; :60-61 night never changes .claude/ or CLAUDE.md -> 2.1 is never Night-queue work.
  - NIGHT_SHIFT.md:54-55 "No v2 spike runs until Mustafa and the Advisor have approved the decision drafts (V2_PHASE0_BRIEF §4)." Q (2.2).
  - tools/preview.js:93 reads `dir + '/shamble-core.js'`: confirms verifier.md:28 with `.` fails ENOENT after move (read, not run).
- B7 done. STATUS:14-15 "signed off as the basis for the spikes, not yet accepted (Status stays Proposed...)"; STATUS:745-746 SPIKE-SCOPE (a) "once the drafts are approved, the legacy constraints apply to `legacy/` only and `spikes/` is throwaway." ADR 0002:38-40 same + ":38 the netcode proof test runs before the foundation" (SLICE-ORDER) vs Mustafa 2026-11-10 "may plan and build foundation steps 2.1 and 2.2 now"; PR #40 (proof test) still open. V2_PHASE0_BRIEF.md:10,22: restructure is Phase 1.
  - advisor.md:150-152 day-session advisor "If the PR changes a tool you reproduce with, use main's copy": main's gate.mjs reads ROOT/<f> (facts gate.mjs:97-100) so it cannot check 2.1's head; the PR's own gate must be used -> plan must say how the advisor verifies (finding for advisor).
  - gate/SKILL.md:18 "Never 'fix' a failure by editing the gate tools." + tools.md:11-12 "After changing any gate tool, prove it still bites": 2.1 edits gate tools to follow the move; the four mutation proofs are that proof.
  - advisor.md:63-64 bracket "a file renamed or moved from a path outside tier 0, and any CLAUDE.md ... (risk tier 2)": unclear whether a moved game file alone is tier 2; #46 edits "advisor.md's tier-2 bracket". Moot for 2.1's tier (gate files make it 2).
- B8 design (from reading only; nothing run after a move):
 1. Minimal kit edits (layout (a)), 2 files, 3 places:
   R1 verifier.md:28 `node tools/preview.js . eval tools/sim.js` -> `node tools/preview.js legacy eval tools/sim.js` (BREAKS as written: preview.js:93 reads <dir>/shamble-core.js). Alternative with 0 kit edits: preview.js (not protected) falls back to legacy/ when <dir>/shamble-core.js is missing (gate's fallback-chain precedent) - plan choice.
   R2 CLAUDE.md:41-42 Layout lists the five as root siblings of docs/ tools/: STALE. :41 prepend "`legacy/` (frozen game): "; :42 `index.html` (player build) · `editor.html` -> `legacy/index.html` (player build) · `legacy/editor.html`.
   R3 CLAUDE.md:19 "It moves to `legacy/` in phase 1." -> "It lives in `legacy/` (foundation step 2.1)." STALE (future tense). Wording depends on Q-PHASE.
   Leave alone (NAME true): advisor.md:67,91,156; netcode-architect.md:21; verifier.md:18-21; rules engine-core/editor/game/pages globs+prose; gate SKILL :6-14; render-ab SKILL :10-11 (<dir>); CLAUDE.md:43,71,73,98,214-219; perf-auditor:34; test-engineer:16; tools.md. CLAUDE.md:46 left for Mustafa (Q-DBL).
   Optional: render-ab SKILL note for pre-move base (base `.`, head `legacy`); CLAUDE.md:214-218 hint "(the legacy build: `legacy`)".
 2. Mustafa Qs: Q-DBL (CLAUDE.md:46 + advisor.md:313), Q-PHASE (CLAUDE.md:19-22,52; advisor.md:98-99; NIGHT_SHIFT:54-55; V2_PHASE0_BRIEF:22 vs his 2026-11-10 go), Q-PROOF (ADR 0002:38 proof test before foundation; PR #40 open), Q-SCOPE (STATUS:745 vs :14-15), Q-MODEL (CLAUDE.md:149-159 vs 2026-11-10 decision), Q-BRACKET (advisor.md:63-64).
 3. #46 overlap: R1-R3 touch CLAUDE.md, verifier.md: not in #46's list. No overlap unless 2.1 edits routine-guard tests (hits-a) or advisor.md. STATUS (docs-keeper) overlaps #46 and #40 (not kit).
 4. Tier: 2 (gate files + CLAUDE.md + verifier.md; advisor.md:71-77), needs-mustafa, merge on his go from a day session only, advisor + security-auditor, tier-2 checklist :227-234, CI green does not stand (:153-156), :91-92 pages -> verify in day session, Brave check (his done-when; :182-184 format). Without kit edits still tier 2 (gate files); without gate edits tier 1 + needs-mustafa (:91-92).
- B9 (advisor's gaps) full read of infra-engineer.md, security-auditor.md, docs-keeper.md, test-engineer.md: done.
  - infra-engineer.md:17 gate.yml as a source: true. security-auditor.md: no paths. test-engineer.md:16 tools/README.md, tools/gate.mjs: true under (a).
  - docs-keeper.md:21-23 REPO_MAP + `node .claude/scripts/repo-map-check.mjs`: script (.claude/scripts/repo-map-check.mjs:22-37) requires every tracked top-level path in REPO_MAP and flags map rows whose path no longer exists -> after the move it exits 1 (missing `legacy/`; stale root rows for the five) until docs/REPO_MAP.md changes in the same PR. Not run by CI or gate (only .claude/settings.json:43 allow rule). Script itself needs no edit. REPO_MAP.md is docs (not protected), not my area.
  - Correction: CLAUDE.md's protected-files line is :64 (not :85-86); true under (a).
  - Grep-only (not read line by line): CHECKPOINTS.md (headings + pattern grep), art-director, engine-architect, game-designer, graphics-engineer, perf-auditor, rust-reviewer, netcode-architect beyond :14-25 (pattern grep for names, tools, legacy).
- B10 under (b) (legacy tools into legacy/): kit lines naming tool paths that would change: CLAUDE.md:38,64,71,73,212,214-219; advisor.md:40-42,59,150-152,155; verifier.md:18,19,28; perf-auditor.md:34; test-engineer.md:16; gate/SKILL.md:6,8,12; render-ab/SKILL.md:3,10,11. Plus outside my files: guard GATE_FILES (routine-guard.mjs:26-27) and DENIED_SCRIPTS (:437) = PR #46's file; .claude/settings.json allow rules; .github/workflows/gate.yml. (a) keeps kit edits to 2 files.
- Design revisions: R3 -> "It lives in `legacy/`." (true under every option). R2 exact lines below. verifier.md hits stays 1 (sum 24); :28 in lines + indirect. Point 4 adds advisor.md:129-131. None of R1-R3 changes a rule's meaning (for the :230-232 before/after table).
  R2a CLAUDE.md:41 old: "`shamble-core.js` (engine) ← `shamble-game.js` (gameplay + dispatch + `bootGame`) ← `shamble-editor.js` (authoring, editor.html only)" new: "`legacy/`: `shamble-core.js` (engine) ← `shamble-game.js` (gameplay + dispatch + `bootGame`) ← `shamble-editor.js` (authoring, editor.html only)"
  R2b CLAUDE.md:42 old: "`index.html` (player build) · `editor.html` (authoring build) · `docs/` ·" new: "`legacy/index.html` (player build) · `legacy/editor.html` (authoring build) · `docs/` ·"
DONE
