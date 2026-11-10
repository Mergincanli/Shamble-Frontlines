# Checkpoint: foundation-toolchain
- **Goal:** plan foundation step 2.2, the toolchain (ADR 0002 part 2, migration plan 2.2), as a change-set plan that
  delivers Mustafa's four asks (what 2.2 pins now; the empty slots until the boundary spike, each by its open question;
  the CI job that runs the Rust checks; its own plan review by the advisor agent). Plan only: no build, no code.
  Plan: `.checkpoints/foundation-toolchain/plan.md` (revision 1).
- **Branch:** `claude/laughing-pasteur-fwya24` (this cloud session's branch, standing in for `cs/foundation-toolchain`)
  · **Base:** main 2d22b69 (PR #46 merged)
- **Last update:** 2026-11-10 18:53 Amsterdam

## Steps done
1. Model check: `get_session` shows `session_context.model` claude-opus-5-5 (Opus 5.5) · no commit · go.
2. Read: CLAUDE.md, CHECKPOINTS.md, REPO_MAP, STATUS Phase / In flight / Next; ADR 0002 parts 1-4 and 7-9, OQ-0002-1,
   -3, -4, -7, spike items 4 and 8, PM-0002-37 to -39, migration plan 2.2; SPIKE_PLAN's foundation entry; at 828a3e0
   (`git show`, main folder not switched): the restructure plan's Part 2, Part 5 and Appendix B,
   `advisor-plan-rev2.md`, `mustafa-2026-11-10.md`, `mustafa-2026-11-10-answers.md`; PR #40's head 411eeee: the
   spike pins Rust 1.99.1 (`spikes/netcode-proof/rust-toolchain.toml`), and its `check.mjs` holds
   `referenceProblems` (:1250) and `cargoConfigFiles` (:1226) · no commit.

3. Research, four web-fetch agents (3 at a time), notes `foundation-toolchain/research-network.md` B1-B5 · autosave
   4eb3e65 · GitHub docs: passwordless sudo on Linux VMs, job containers refuse `--network`; util-linux unshare and
   setpriv, the kernel's no_new_privs; npm `--offline`, cargo `--offline`/`fetch`, rustup auto-install (on by default
   since 1.28.1); Node 26 is Active LTS, 22 ends 2027-04-30. PM-0002-37 can be promised.
4. Plan revision 1 written (`foundation-toolchain/plan.md`) · this commit · the advisor tool is not available in this
   session (recorded in Part 9).

## Current step
The advisor agent's plan review of revision 1 at effort xhigh (notes `foundation-toolchain/advisor-plan-rev1.md`).

## Next steps
1. Run the advisor agent on plan revision 1 (effort xhigh); record its verdict and the SHA in plan.md Part 9 and here.
2. On REVISE: revision 2 with the exact changes, then the advisor's check of it. On ESCALATE: Mustafa's decision.
3. The final report: the plan's path and SHA, a one-screen summary, the verdict, Decisions needed (FDN-NOCODE), For
   Mustafa.

## Open findings
- The advisor tool (Fable 5.2) is not among this session's tools (ToolSearch "advisor": no match). Mustafa's words: it
  "is an extra. The recorded plan review by the advisor agent stays required". The advisor agent review goes ahead.
- STATUS on main still calls kit-v3-5 in flight; PR #46 merged as 2d22b69 (docs drift for docs-keeper, not 2.2's).
- The local `unshare` rehearsal was refused by the session's permission check; not retried (plan Appendix B).

## Key numbers
- The guard's marks on 2.2's files (2d22b69, `marks.mjs`): protected: the new workflow, CLAUDE.md, advisor.md, the
  rules, `.npmrc`; not protected: every root pin, `crates/**`, `packages/**`, `.gitignore`; the PR Advisor may merge
  only the docs among them (REPO_MAP, STATUS, ROADMAP, SPIKE_PLAN, reports), not the register or the ADRs.
- npm registry 2026-11-10: typescript 6.0 line 6.0.2, 6.0.3 (latest 7.0.2); vitest 5.1.0; @playwright/test 1.65.0.
- Runner image 20261101.360.1: Rustup 1.29.2, Rust 1.99.1 (= the pin).

## Decisions taken
- Mustafa, 2026-11-10 (this session's prompt): FREEZE-FDN covers the toolchain's CI job (each such edit asks him and
  merges on his go); guard checks for the new pins are not covered and wait for the gate step (2.3); 2.2 merges after
  2.1b.

## Running work
- saver: background task b04hgafwt · output `/tmp/claude-0/-home-user-Shamble-Frontlines/29a42dc7-ecd4-506e-b90a-121dd5e50988/tasks/b04hgafwt.output`
  · started 18:35 Amsterdam · stops by 20:25 · last check 18:52: ok (saved and pushed 4eb3e65).
