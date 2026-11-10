# Checkpoint: foundation (plan for foundation steps 2.1 and 2.2; this session plans only)
- **Goal:** plan step 2.1, the restructure (the five legacy files into `legacy/`), as its own change-set, listing every
  file that names a legacy path and marking which are kit or gate files; sketch step 2.2, the toolchain, on one page;
  name what in 2.3 to 2.6 waits and for which result; say when 2.1 can build without colliding with PR #46 and what
  PR #40 must do after 2.1 merges. Plan: `.checkpoints/foundation/plan.md` (being written).
- **Source of the task:** Mustafa's prompt of 2026-11-10 in this session, with his decision of 2026-11-10 (word for
  word in `.checkpoints/foundation/mustafa-2026-11-10.md`).
- **Branch:** `claude/advisor-fable-4hxq72` (this cloud session may push only there; it stands in for the plan's
  branch; no code, no file moves) · **Base:** main 44446a5 (PR #44's merge)
- **Account:** Mustafa's personal account (one account per branch). The university account runs PR #40 (netcode
  proof test, `claude/netcode-proof-test-plan-hcky5g`) and PR #46 (kit-v3-5, `claude/sleepy-tesla-ofsxsr`): never
  push there, never touch `spikes/`.
- **Last update:** 2026-11-10 09:55 Amsterdam

## Steps done
0. Read CLAUDE.md, CHECKPOINTS.md, REPO_MAP, ADR 0002's migration plan, SPIKE_PLAN's foundation entry, STATUS
   Phase / In flight / Next. The first turn stopped on the model rule; resolved by Mustafa (2026-11-10), who said to
   carry on. Nothing written before that.

1. Facts gathered (no commit of their own; recorded here): ADR 0002 parts 1-3, OQ-0002-1..9, the boundary spike's
   items 1-10, PM-0002-35..69; R5-LAYOUT-REC1 and REC6; the tools' path handling; the guard's gate-file list; PR #46
   and #40 files. Raw grep: 62 tracked files (outside `.checkpoints/`) name a legacy file, 692 hits.

2. Advisor tool consulted (go; notes under Open findings). Inventory workflow launched 07:31 (Running work).

3. Plan revision 1 written (`.checkpoints/foundation/plan.md`): the one-screen summary, Part 1 (2.1: goal, the
   inventory of 63 files, the edits, OQ-0002-2, evidence E1-E7, steps, reviewers, Brave check, risks), Part 2 (2.2
   sketch), Part 3 (2.3-2.6 waits), Part 4 (#46, #40), Part 5 (questions). inv-code, inv-kit, inv-docs DONE.

4. The workflow finished (4 of 4 agents, 0 errors; 729k agent tokens, 27 min). The critic: 5 missing, 2 wrong, 21 of
   22 spot-checks upheld, 6 design issues; all folded into plan revision 1: F1 (a pure revert of a one-PR 2.1 turns
   CI red) → 2.1a (tools learn both layouts) then 2.1b (the move), E8 rehearses the revert, Q-A6; F2 (no `"type":
   "module"` at the root, 2.2); F3 (PM-0002-36 cannot pass as written, Q-A1 widened); the deferred gate-globals
   wording (STATUS:1009) folded into 2.1a; the rules' globs vs a v2 `index.html` (2.2 note); the wrong saves claim
   (CS-1 never tested it) and battery.js (no edit) corrected.

5. Advisor tool before committing to the plan (go, with six fixes): judge pinned to OLD = 44446a5; E8 rehearses
   `git revert -m 1` of a local merge, with the gate and `--smoke-only`; his 2026-11-10 decision recorded on main by
   2.1a (STATUS, pointers in ADR 0002 step 1, SPIKE_PLAN order and Depends on, STATUS Next); the hand-over (branch,
   plan location across 2.1a and 2.1b); SPLIT as his decision; HC-PATH and SCOPE with advisor.md:313's consequence
   (HC-PATH now recommends leaving :46 untouched; SCOPE needs ADR 0002 part 1 accepted, before 2.2). Also Appendix A
   (all 63 paths, script-generated), Appendix B (2.2's coexistence rules). Plan revision 2 at 8bbebfa.

6. The advisor agent's plan review of revision 2 (8bbebfa): APPROVE WITH CONDITIONS, for 2.1 only (notes
   `.checkpoints/foundation/advisor-plan-rev2.md`). Its H1-H3 checked against the source files on PR #46's branch
   (d7526f5): KIT-FREEZE-2 and LANE-RULES are his words of 2026-11-10 06:33 and 07:08 UTC; SPIKE-SCOPE (a) was taken
   by the Advisor on 2026-10-25 (STATUS:739-746). Conditions C1-C8 applied: plan revision 3.

7. Advisor tool before calling the plan done: six consistency fixes (E3 in 2.1b, the 1.2 rows, two stale risks, #40
   after each PR, the docs count, models named by CLAUDE.md's section) and one full read of the plan; applied.

## Current step
Done, for this session: plan revision 3 at `.checkpoints/foundation/plan.md` (this branch). The checkpoint folder stays:
it is the plan's home until 2.1a's build moves it (plan 1.1, Hand-over).

## Next steps
1. Mustafa answers FREEZE-FDN, SPLIT and SCOPE (plan Part 5), and says whether a session on this account starts
   building 2.1a now (allowed by the advisor's approval: the dry run, the tool edits with their asks, E3; not its merge).
2. 2.1a's step 0: re-read the open PRs' file lists (#46, STATUS-DIET, LANE-RULES, #40), record OLD's SHA, move the plan
   to `.checkpoints/foundation-restructure/plan.md`.
3. 2.2: its own plan and plan review, after SCOPE is answered (Part 2 is the sketch).

## Open findings
- PR #46 (kit-v3-5, draft, step 0 on 2026-11-09): will edit `.claude/hooks/routine-guard.mjs` and its tests,
  advisor.md's tier-2 bracket, STATUS, ROADMAP, the kit-v3-4 report.
- PR #40 (netcode proof test): changes `docs/REPO_MAP.md`, `docs/STATUS.md`, `docs/design/*`, `spikes/netcode-proof/`
  (157 files), `.checkpoints/netcode-proof-test*`; nothing in tools/, .claude/, .github/, CLAUDE.md. Its
  `spikes/netcode-proof/check.mjs` `referenceProblems` fails if any tracked non-Markdown file outside `spikes/`,
  `docs/`, `.checkpoints/` contains the text `spikes/` (matters for 2.2's root manifests). Its Cargo.toml has its own
  `[workspace]`; its toolchain pin is Rust 1.99.1, "ADR 0002 part 2's pin for the foundation is the same 1.99.1".

- Gate: `tools/gate.mjs:33-35` hardcodes ROOT = repo root and FILES at the root; `baseFile` already falls back to
  `OLD_PREFIX` (`shamble-frontlines-repo/`, CS-1's earlier move) — the precedent for a `legacy/` → root → old-prefix
  chain. `runSmoke({ root: ROOT })` at :190; smoke.mjs defaults root to the repo root (:18, :118).
- `preview.js`, `pageshot.js`, `battery.js` take the build folder as an argument: callers pass `legacy` instead of `.`.
  `village.py` defaults to `<repo>/shamble-core.js` (:10).
- Guard: `GATE_FILES` (routine-guard.mjs:26-27) names `tools/gate.mjs`, `smoke.mjs`, `battery.js`, `gate-globals.json`,
  `package.json`, `package-lock.json`; `DENIED_SCRIPTS` (:437) `tools/preview.js`, `tools/pageshot.js`; :534 diffs
  `tools/package*.json`. If the legacy tools stay in `tools/` (OQ-0002-2 (a)) the guard needs no edit; if they move
  into `legacy/` (b), the guard, settings.json (17 `tools/` allow rules) and gate.yml all change.
- `.claude/rules/*` globs are `**/shamble-core.js`, `**/index.html` etc.: they already match `legacy/…` (no edit).
- PM-0002-36: `docs/legacy-tuning.json` is not the dump's raw output; R7's builder
  `.checkpoints/r7-legacy-inventory/build-tuning.py` (at a715be2) builds it from two raw dumps (also at a715be2), its
  `where` fields from `git show 48ffb51:shamble-*.js`; the mark's command lacks `--pre=tools/tuning-dump.js`. So the
  mark is reachable through R7's builder (dumps at `legacy` → builder → `cmp`); the reading goes to the advisor
  agent. 2.1 must not edit `docs/legacy-tuning.json` or `docs/LEGACY_TUNING.md`.
- Advisor tool (2026-11-10, before the workflow): go; keep tools in `tools/`; pass `model: 'opus'` and effort on every
  agent; grep without the prefix guard (done: 63 files, 728 hits); kit column from `protectedRel` (done); OQ-0002-2's
  three parts; evidence that does not lean on the edited gate; rule conflicts go to Mustafa as questions; #46 file list
  not final; #40's marks.json rows; 2.2 vs `check.mjs`; G0 belongs to 2.3; saves unknown; label "grep only".
- The four Phase 0 mutation proofs: `docs/reports/2026-10-13-phase-0.md:25-27` (deleted `refreshRect` → [3] DROPPED;
  `bootEditor();` in `bootGame` → [4] FLAGGED; syntax error → [1]; `wrType` copied into core → [3] duplicate);
  check-5's 22 mutations: `docs/reports/2026-10-14-gate-check-5.md:30-41`.
- KVStore: localStorage on file:// (shamble-core.js:31-41); whether Brave keeps saves across the folder change is to be
  checked (Chromium file:// storage), not assumed.
- SPIKE-SCOPE (a) (STATUS:745): "once the drafts are approved, the legacy constraints apply to `legacy/` only and
  `spikes/` is throwaway".

## Key numbers
- 63 files / 728 hits (`git grep` of the five names with no prefix guard, outside `.checkpoints/`; the first,
  guarded grep gave 62 / 692). Area lists: `.checkpoints/foundation/hits-{a-code,b-kit,c-docs}.txt` (101 / 24 / 603).
- 15 of the 63 files are PROTECTED by the guard's `protectedRel` (12 under `.claude/` or CLAUDE.md, 3 gate files).

## Decisions taken
- Plan home: `.checkpoints/foundation/plan.md` on this branch (precedent: `.checkpoints/netcode-proof-test/plan.md`).
- OQ-0002-2 (decider: this plan, with docs-keeper): tools stay in `tools/`, legacy docs stay in `docs/`, the gate v2's
  path deferred to 2.3 · proposed here, approved by the advisor agent (Q-A2) · 2026-11-10.
- Q-A1 E4 accepted as PM-0002-36's reading (register in 2.1b; PM row fixed at ADR 0002's next revision); Q-A3 G0 to
  2.3 (provisional); Q-A4 follows FREEZE-FDN, no preview.js fallback; Q-A5 one prefix per base; Q-A6 the split's
  reasoning holds (his choice) · the advisor agent · 2026-11-10.

## Running work
- Workflow wf_92d477b2-b78 (task wbpbds2b5) · script copy `.checkpoints/foundation/inventory-run1.js` · no args ·
  notes `.checkpoints/foundation/inv-{code,kit,docs,critic}.md` · launched 07:31 Amsterdam · done (4 of 4 returned).
- Saver: background task bsmtuh7qm, started 07:31, stopped 09:55 by TaskStop after the last agent finished (its output ends
  "[killed]": the final save was made by hand); ok throughout (saved and pushed at 07:41, 07:51, 08:11).
- The advisor agent's plan review of revision 2 (8bbebfa), launched 08:46 Amsterdam: done (APPROVE WITH CONDITIONS).
