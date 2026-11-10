# Checkpoint: foundation-toolchain
- **Goal:** plan foundation step 2.2, the toolchain (ADR 0002 part 2, migration plan 2.2), as a change-set plan that
  delivers Mustafa's four asks (what 2.2 pins now; the empty slots until the boundary spike, each by its open question;
  the CI job that runs the Rust checks; its own plan review by the advisor agent). Plan only: no build, no code.
  Plan: `.checkpoints/foundation-toolchain/plan.md` (revision 4; the advisor's APPROVE of revision 3, bf1737f).
- **Branch:** `claude/laughing-pasteur-fwya24` (this cloud session's branch, standing in for `cs/foundation-toolchain`)
  · **Base:** main 2d22b69 (PR #46 merged)
- **Last update:** 2026-11-10 19:52 Amsterdam

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
4. Plan revision 1 written (`foundation-toolchain/plan.md`) · f535752 · the advisor tool is not available in this
   session (recorded in Part 9).
5. The advisor agent's review of revision 1 at effort xhigh · notes `advisor-plan-rev1.md` (DONE), `advisor-marks.mjs`
   · APPROVE f5357520329a1ce76b7bf3ec3a873ea86f378cbc with conditions 1-6; "No new plan review is needed if you fold
   conditions 1 to 5 in as written."
6. Plan revision 2: conditions 1-5 folded as written, the Lows of 6 folded, the verdict recorded in Part 9 · b6c20be.

7. Fold check of revision 2: workflow wf_93f9ff1a-f0b, 4 of 4 returned (notes `fold-check-a.md`, `-b.md`, `-c.md`,
   `-critic.md`, all DONE) · Medium: step 5's detached worktree stops the saver; E11 fails on the workspace links;
   Part 9 overclaimed the advisor's checks. Lows: wording, the probe's DNS loophole, push-by-hand for planted
   failures, E2's interface rule, the plan's ending order, unsaved advisor verdict (saved:
   `advisor-plan-rev1-verdict.md`), his xhigh sentence added to `mustafa-2026-11-10-session.md`.
8. Plan revision 3: every fold-check finding fixed · bf1737f.
9. The advisor agent's check of revision 3 at effort xhigh · notes `advisor-plan-rev3.md` (DONE), returned verdict
   `advisor-plan-rev3-verdict.md` · APPROVE bf1737fe9ffcdb594c0ade078af445e58d1415ff, six Low conditions L-a to L-f
   ("fold each in during the build, or when FDN-NOCODE is sent. No new plan review is needed.").
10. Plan revision 4: L-a to L-f folded as written (FDN-HEADING added per L-f) · this commit. The advisor tool was not
    consulted before the plan was committed or before it was called done: it is not among this session's tools.

## Current step
Done for this session (plan only). Waiting on Mustafa: FDN-NOCODE and FDN-HEADING (before the build's step 5), and his
start of the build.

## Next steps
1. The build session (when Mustafa starts it): read plan.md revision 4, Part 8's steps 1-7, in order; its PR stays a
   draft until 2.1b has merged; the kit lines (K2a-K4) only after his FDN answers and condition 5's check.
2. This checkpoint and the plan are folded into the build's report and deleted before its PR is marked ready
   (CHECKPOINTS §9).

## Open findings
- The advisor tool (Fable 5.2) is not among this session's tools (ToolSearch "advisor": no match). Mustafa's words: it
  "is an extra. The recorded plan review by the advisor agent stays required". The advisor agent review goes ahead.
- STATUS on main still calls kit-v3-5 in flight; PR #46 merged as 2d22b69 (docs drift for docs-keeper, not 2.2's).
- The local `unshare` rehearsal was refused by the session's permission check; not retried (plan Appendix B).
- Kit follow-up for STATUS (the advisor, outside this plan): CHECKPOINTS.md §10's `git -C <path> status` is refused by
  the guard (`routine-guard.mjs:1046`); waits for the kit to unfreeze; docs-keeper records it in the build's docs pass.

## Key numbers
- The guard's marks on 2.2's files (2d22b69, `marks.mjs`): protected: the new workflow, CLAUDE.md, advisor.md, the
  rules, `.npmrc`; not protected: every root pin, `crates/**`, `packages/**`, `.gitignore`; the PR Advisor may merge
  only the docs among them (REPO_MAP, STATUS, ROADMAP, SPIKE_PLAN, reports), not the register or the ADRs.
- npm registry 2026-11-10: typescript 6.0 line 6.0.2, 6.0.3 (latest 7.0.2); vitest 5.1.0; @playwright/test 1.65.0.
- Runner image 20261101.360.1: Rustup 1.29.2, Rust 1.99.1 (= the pin).

## Decisions taken
- The advisor agent, 2026-11-10: Q2.2-a (a) final (G0's exit-2 mark to 2.3); Q2.2-c (a) Node 22 final, with a dated
  follow-up (tools/ and v2 to Node 26 together, after 2.3, merged before 2027-03-31); the sketch's three changes
  approved; planted-failure commits on the change-set's branch approved with condition 3.
- Mustafa, 2026-11-10 (this session's prompt): FREEZE-FDN covers the toolchain's CI job (each such edit asks him and
  merges on his go); guard checks for the new pins are not covered and wait for the gate step (2.3); 2.2 merges after
  2.1b.

## Running work
- saver: background task b04hgafwt · output `/tmp/claude-0/-home-user-Shamble-Frontlines/29a42dc7-ecd4-506e-b90a-121dd5e50988/tasks/b04hgafwt.output`
  · started 18:35 Amsterdam · stopped by the session at 19:52 after the last save (the task's work in this session
  is done).
- workflow run wf_93f9ff1a-f0b (task wt8ab4d43) · script as launched `foundation-toolchain/verify-fold-run1.js` · no
  args · notes `.checkpoints/foundation-toolchain/fold-check-*.md` · launched 19:10 Amsterdam · done 19:24, 4 of 4.
