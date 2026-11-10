# Checkpoint: foundation-toolchain
- **Goal:** plan foundation step 2.2, the toolchain (ADR 0002 part 2, migration plan 2.2), as a change-set plan that
  delivers Mustafa's four asks (what 2.2 pins now; the empty slots until the boundary spike, each by its open question;
  the CI job that runs the Rust checks; its own plan review by the advisor agent). Plan only: no build, no code.
  Plan: `.checkpoints/foundation-toolchain/plan.md` (not written yet).
- **Branch:** `claude/laughing-pasteur-fwya24` (this cloud session's branch, standing in for `cs/foundation-toolchain`)
  · **Base:** main 2d22b69 (PR #46 merged)
- **Last update:** 2026-11-10 18:34 Amsterdam

## Steps done
1. Model check: `get_session` shows `session_context.model` claude-opus-5-5 (Opus 5.5) · no commit · go.
2. Read: CLAUDE.md, CHECKPOINTS.md, REPO_MAP, STATUS Phase / In flight / Next; ADR 0002 parts 1-4 and 7-9, OQ-0002-1,
   -3, -4, -7, spike items 4 and 8, PM-0002-37 to -39, migration plan 2.2; SPIKE_PLAN's foundation entry; at 828a3e0
   (`git show`, main folder not switched): the restructure plan's Part 2, Part 5 and Appendix B,
   `advisor-plan-rev2.md`, `mustafa-2026-11-10.md`, `mustafa-2026-11-10-answers.md`; PR #40's head 411eeee: the
   spike pins Rust 1.99.1 (`spikes/netcode-proof/rust-toolchain.toml`), and its `check.mjs` holds
   `referenceProblems` (:1250) and `cargoConfigFiles` (:1226) · no commit.

## Current step
Research: how a GitHub-hosted runner can run steps with the network cut off (before promising PM-0002-37).

## Next steps
1. Research the network cut-off with sources (GitHub docs: passwordless sudo, container `--network` unsupported;
   util-linux `unshare`/`setpriv`; runner image; npm `--offline`; cargo `--offline`); rehearse locally in this
   container if `unshare` works here (labelled as this container, not the runner).
2. Re-find every cited line on main 2d22b69; run the guard's `protectedRel` and `mergeAllowed` on 2.2's file list.
3. Write `plan.md`; consult the advisor tool (not in this session's tool list: recorded) and the advisor agent at
   effort xhigh; record its verdict against the plan's commit SHA.

## Open findings
- The advisor tool (Fable 5.2) is not among this session's tools (ToolSearch "advisor": no match). Mustafa's words: it
  "is an extra. The recorded plan review by the advisor agent stays required". The advisor agent review goes ahead.
- STATUS on main still calls kit-v3-5 in flight; PR #46 merged as 2d22b69 (docs drift for docs-keeper, not 2.2's).

## Key numbers
- (none yet)

## Decisions taken
- Mustafa, 2026-11-10 (this session's prompt): FREEZE-FDN covers the toolchain's CI job (each such edit asks him and
  merges on his go); guard checks for the new pins are not covered and wait for the gate step (2.3); 2.2 merges after
  2.1b.

## Running work
- (none)
