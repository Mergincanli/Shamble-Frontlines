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

## Current step
1. Facts: ADR 0002 parts (layout, pins, open questions, the boundary spike's items, PM-0002-35 to -69), R5's layout
   recommendations, how the tools resolve paths, PR #46's and PR #40's files.

## Next steps
2. Inventory of every file naming a legacy path (raw `git grep`, then a workflow of at most 3 agents to classify it).
3. Plan draft; consult the advisor tool; the `advisor` agent's plan review; fix; advisor tool before done.

## Open findings
- PR #46 (kit-v3-5, draft, step 0 on 2026-11-09): will edit `.claude/hooks/routine-guard.mjs` and its tests,
  advisor.md's tier-2 bracket, STATUS, ROADMAP, the kit-v3-4 report.
- PR #40 (netcode proof test): changes `docs/REPO_MAP.md`, `docs/STATUS.md`, `docs/design/*`, `spikes/netcode-proof/`
  (157 files), `.checkpoints/netcode-proof-test*`; nothing in tools/, .claude/, .github/, CLAUDE.md. Its
  `spikes/netcode-proof/check.mjs` `referenceProblems` fails if any tracked non-Markdown file outside `spikes/`,
  `docs/`, `.checkpoints/` contains the text `spikes/` (matters for 2.2's root manifests). Its Cargo.toml has its own
  `[workspace]`; its toolchain pin is Rust 1.99.1, "ADR 0002 part 2's pin for the foundation is the same 1.99.1".

## Key numbers
- (none yet)

## Decisions taken
- Plan home: `.checkpoints/foundation/plan.md` on this branch (precedent: `.checkpoints/netcode-proof-test/plan.md`).

## Running work
- (none)
