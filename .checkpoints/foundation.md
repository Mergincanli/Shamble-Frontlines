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
- **Last update:** 2026-11-10 01:10 Amsterdam

## Steps done
0. Read CLAUDE.md, CHECKPOINTS.md, REPO_MAP, ADR 0002's migration plan, SPIKE_PLAN's foundation entry, STATUS
   Phase / In flight / Next. First turn stopped on the model rule (main model was not Opus 5.5); Mustafa switched it to
   Opus 5.5 and said to carry on (2026-11-10). Nothing written before that.

1. Facts gathered (no commit of their own; recorded here): ADR 0002 parts 1-3, OQ-0002-1..9, the boundary spike's
   items 1-10, PM-0002-35..69; R5-LAYOUT-REC1 and REC6; the tools' path handling; the guard's gate-file list; PR #46
   and #40 files. Raw grep: 62 tracked files (outside `.checkpoints/`) name a legacy file, 692 hits.

## Current step
2. Consult the advisor tool on the approach, then the inventory workflow (3 classifier agents, then 1 critic).

## Next steps
3. Plan draft `.checkpoints/foundation/plan.md`; the `advisor` agent's plan review; fix; advisor tool before done.

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
- PM-0002-36 as written cannot hold: `docs/legacy-tuning.json` is curated (keys about, tables{where, group,
  look_brief_pending, values}, boot_changes, skipped), not the dump's raw output; and the mark's command lacks
  `--pre=tools/tuning-dump.js`. Needs a reading (dump at base `.` vs head `legacy` byte for byte, plus values equal).
- The four Phase 0 mutation proofs: `docs/reports/2026-10-13-phase-0.md:25-27` (deleted `refreshRect` → [3] DROPPED;
  `bootEditor();` in `bootGame` → [4] FLAGGED; syntax error → [1]; `wrType` copied into core → [3] duplicate);
  check-5's 22 mutations: `docs/reports/2026-10-14-gate-check-5.md:30-41`.
- KVStore: localStorage on file:// (shamble-core.js:31-41); whether Brave keeps saves across the folder change is to be
  checked (Chromium file:// storage), not assumed.
- SPIKE-SCOPE (a) (STATUS:745): "once the drafts are approved, the legacy constraints apply to `legacy/` only and
  `spikes/` is throwaway".

## Key numbers
- 62 files / 692 hits (raw `git grep` of the five names, outside `.checkpoints/`).

## Decisions taken
- Plan home: `.checkpoints/foundation/plan.md` on this branch (precedent: `.checkpoints/netcode-proof-test/plan.md`).

## Running work
- (none)
