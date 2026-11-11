# Checkpoint: foundation-toolchain
- **Goal:** build foundation step 2.2, the toolchain (ADR 0002 part 2, migration plan 2.2), from its approved plan, in
  this session on this branch (Mustafa's change of plan, 2026-11-10, `mustafa-2026-11-10-build-here.md`): steps 1-4
  now; step 5 after 2.1b and LANE-RULES merge; the network-cut rule; each kit edit asks him; the PR a draft until 2.1b
  has merged. (The planning part of this task is done: steps 1-11 below.)
  Plan: `.checkpoints/foundation-toolchain/plan.md` (revision 5: revision 4 plus Mustafa's answers; the advisor's
  APPROVE of revision 3, bf1737f).
- **Branch:** `claude/laughing-pasteur-fwya24` (this cloud session's branch, standing in for `cs/foundation-toolchain`)
  · **Base:** main 2d22b69 (PR #46 merged)
- **Last update:** 2026-11-11 10:00 Amsterdam

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

11. Mustafa's answers of 2026-11-10 recorded (`mustafa-2026-11-10-answers-2-2.md`, word for word): FDN-NOCODE (a),
    FDN-HEADING (a), the build started in a fresh session (steps 1-4 before 2.1b merges; step 5 waits), his rule for
    the network cut (one fix round, then the offline flags, PM-0002-37 partly proven, tell him), his go on the ADR
    index entry in the PR · plan revision 5 · this commit.

12. His change of plan recorded (`mustafa-2026-11-10-build-here.md`): the build runs here, on this branch; no CLAUDE.md
    rule requires a fresh session (protocol step 3); K4 needs Accept edits or Manual mode (CLAUDE.md:67-68); after K4
    the advisor runs as a general agent reading main's advisor.md (CLAUDE.md:59-61).

13. Build step 1, the root pins · d3a1e9c · `rust-toolchain.toml` (1.99.1), `Cargo.toml`, `package.json` (TypeScript
    6.0.3, Vitest 5.1.0, @playwright/test 1.65.0, Node 22), `package-lock.json` (39 packages), `.gitignore` (+2);
    Cargo.lock moves to step 2 (cargo cannot lock a workspace with no members); E4a none, E6 and E7 hold; E4c: the
    toolkit installs from its own lockfile with the root `package.json` present (159 packages); gate `--no-smoke`
    PASS (1-4, 5b-5d); 5a cannot start here (the toolkit's Playwright 1.56.1 wants Chromium 1194, the container has
    1243, and a second Playwright install is forbidden): 5a runs in CI's smoke job.

14. Build step 2, the crates and packages · 566363f · E1 (cargo test 6/6; wasm32 build of the five Wasm crates;
    release overflow tests pass, and fail "did not panic as expected" with sim's override removed: the control); L1
    (an unsafe block: "usage of an `unsafe` block"); E7 (6 package.json files exact; packages private with their own
    type); E11 (69 lock entries = 64 registry packages, all npmjs with sha512 integrity, + 5 workspace links; install
    scripts: fsevents only, optional, macOS); E4a none; gate `--no-smoke` PASS, symbols +0 -0; local build folder
    cleaned (48.5 MiB). The session's permission check refused running npm scripts here twice (`npm test`, also in a
    scratch workspace): not retried; E1's npm half runs in CI (E2), L2 rests on npm's own docs (B3 quote), and L3
    becomes planted failure M4 in CI.

15. Build step 3, K1 · fb17451 · `.github/workflows/toolchain.yml` (the write went through the session's ask path; the
    YAML parses, the three scripts pass `bash -n`, the same action SHAs as gate.yml, gate.yml unchanged); pushed; draft
    PR #48 (https://github.com/Mergincanli/Shamble-Frontlines/pull/48). E2 on fb17451: push run 42891450472 (job
    130093350804) and pull_request run 42891534489, both success. The log: rustup installed 1.99.1 "because:
    overridden by '.../rust-toolchain.toml'"; `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, "rustc equals the pin 1.99.1";
    control "connected to 140.82.114.4:443 before the cut"; inside: `uid=1001(runner) gid=65534(nogroup)
    groups=65534(nogroup)`, only `lo` (UP), rustc 1.99.1, "getent: no answer (printed, not judged)", `curl: (7)`,
    "the network is cut: ... (curl exit 7)"; cargo test 6/6, the wasm32 build, `npm ci --offline` (46 packages from
    the cache), `npm test --workspaces` 5/5 (Playwright's test ran with no browser installed: A2), tsc 6.0.3. Job
    23 s (the cut step 4 s). A1, A2, A3 proven; his fallback rule not needed.

16. Build step 4, the planted failures, each pushed alone by hand and red at the expected step before its revert:
    - M1a 1bee16c, run 42891819768 (job 130094623250): red at `cargo test` inside the cut, `planted_network_use`
      panicked "failed to lookup address information: Temporary failure in name resolution"; revert 2ee2bff.
    - M1b 363c455, run 42892038296 (job 130095416867): red at `npm test --workspaces` inside the cut, @shamble/net
      "1 failed | 1 passed", "Lifecycle script `test` failed"; revert 5c64404.
    - M2 0d3ec36, run 42892226674 (job 130098974545): with no toolchain file `rustup toolchain install` fell back to
      the image's default ("using existing install for stable-x86_64-unknown-linux-gnu ... it's the default
      toolchain", 1.99.1 today); the pin check failed closed, "sed: can't read rust-toolchain.toml", exit 2; the fetch
      and cut steps skipped, no cargo step ran; revert b60a996.
    - M3 af65262, run 42893284193 (job 130099681197): red at `cargo fetch --locked`, "cannot update the lock file ...
      because --locked was passed to prevent this", exit 101 (the pin check had passed); revert 00cee0b.
    - M4 2d9766f (L3 moved to CI), run 42893510607 (job 130100473389): red at `npm test --workspaces`, "workspace
      @shamble/types@0.0.0 ... Missing script: "test"" (N3 proven); revert 66d9888.
    Each revert exact (`git diff HEAD~2 HEAD` empty); the tree at 66d9888 equals fb17451 outside `.checkpoints/`.
    Final head 66d9888 all green: toolchain push 42893708734 and pull_request 42893718519; gate workflow push
    42893708791 and pull_request 42893718283 (gate and smoke jobs, 6 check runs, all success).

## Current step
Steps 1-4 done. Waiting (his word, 2026-11-11): step 5 starts when he says 2.1b and LANE-RULES (PR #49) are in; PR #48
stays a draft. Plan revision 6 records KIT-ASK-SEEN (b).

## Next steps
1. Step 5 (main merged in by the worktree path, condition 5's check, then K2a, K2b, K3, K4: each asked in chat with its
   exact text and written only after his yes, then the guard's ask at the write) after 2.1b and LANE-RULES
   have merged (K4 in Accept edits or Manual mode, CLAUDE.md:67-68; after K4 the advisor runs as a general agent
   reading main's advisor.md, CLAUDE.md:59-61); steps 6-7 after (docs incl. REPO_MAP and the register's G0 reading;
   the review round 3 at a time; the verifier; the report; the checkpoint folded in and deleted; ready; the
   advisor's review of the head; his go).
2. This checkpoint and the plan are folded into the build's report and deleted before its PR is marked ready
   (CHECKPOINTS §9).

## Open findings
- The advisor tool (Fable 5.2) is not among this session's tools (ToolSearch "advisor": no match). Mustafa's words: it
  "is an extra. The recorded plan review by the advisor agent stays required". The advisor agent review goes ahead.
- STATUS on main still calls kit-v3-5 in flight; PR #46 merged as 2d22b69 (docs drift for docs-keeper, not 2.2's).
- The local `unshare` rehearsal was refused by the session's permission check; not retried (plan Appendix B).
- K1's ask (the session's event log, list_events): the guard's PreToolUse ask fired for the workflow write at
  2026-11-10 20:02:42 UTC ("kit and gate files ask first in a day session: .github/workflows/toolchain.yml", a
  can_use_tool request) and was answered "allow" at 20:02:48 UTC; the log does not say who allowed it. Mustafa,
  KIT-ASK-SEEN (b), 2026-11-11: treated as not seen. Recorded as a kit finding for 2.3, "auto mode can pass the
  guard's ask" (plan Appendix B; step 6 carries it into STATUS and the report); no guard change now (kit frozen).
- Kit follow-up for STATUS (the advisor, outside this plan): CHECKPOINTS.md §10's `git -C <path> status` is refused by
  the guard (`routine-guard.mjs:1046`); waits for the kit to unfreeze; docs-keeper records it in the build's docs pass.

## Key numbers
- The guard's marks on 2.2's files (2d22b69, `marks.mjs`): protected: the new workflow, CLAUDE.md, advisor.md, the
  rules, `.npmrc`; not protected: every root pin, `crates/**`, `packages/**`, `.gitignore`; the PR Advisor may merge
  only the docs among them (REPO_MAP, STATUS, ROADMAP, SPIKE_PLAN, reports), not the register or the ADRs.
- npm registry 2026-11-10: typescript 6.0 line 6.0.2, 6.0.3 (latest 7.0.2); vitest 5.1.0; @playwright/test 1.65.0.
- Runner image 20261101.360.1: Rustup 1.29.2, Rust 1.99.1 (= the pin).
- Local: rustup 1.28.2; `rustup toolchain install` installed 1.99.1 with clippy, rustfmt, wasm32 from the file;
  `rustup show active-toolchain` prints `1.99.1-x86_64-unknown-linux-gnu (overridden by '<repo>/rust-toolchain.toml')`.

## Decisions taken
- Mustafa, 2026-11-11 (KIT-ASK-SEEN (b), `mustafa-2026-11-11-kit-ask-seen.md`): before each step-5 kit edit, ask him in
  chat and write it only after his yes; "auto mode can pass the guard's ask" is a kit finding for the gate step (2.3), no
  guard change now. LANE-RULES comes with PR #49 (his go in the morning); he says when 2.1b and LANE-RULES are in; PR #48
  stays a draft.
- The session, 2026-11-10 (method, not design): the npm test runs move from local (E1's npm half, L3) to CI (E2, a
  fifth planted failure M4: a package without a test script), because the session's permission check refused npm
  script runs; L2 rests on npm's documented behaviour. To fold into the report.
- Mustafa, 2026-11-10: FDN-NOCODE (a) ("One clause per line naming my 2026-11-10 decision as the exception; each edit
  asks me and merges on my go."); FDN-HEADING (a) ("The edit asks me and merges on my go."); the build starts now in a
  fresh session, steps 1-4 before 2.1b merges, step 5 waits; the network cut: one fix round, then cargo's and npm's
  own offline flags, PM-0002-37 recorded as partly proven, and he is told; his go on the ADR index entry in the PR.
- The advisor agent, 2026-11-10: Q2.2-a (a) final (G0's exit-2 mark to 2.3); Q2.2-c (a) Node 22 final, with a dated
  follow-up (tools/ and v2 to Node 26 together, after 2.3, merged before 2027-03-31); the sketch's three changes
  approved; planted-failure commits on the change-set's branch approved with condition 3.
- Mustafa, 2026-11-10 (this session's prompt): FREEZE-FDN covers the toolchain's CI job (each such edit asks him and
  merges on his go); guard checks for the new pins are not covered and wait for the gate step (2.3); 2.2 merges after
  2.1b.

## Running work
- saver: background task b04hgafwt · output `/tmp/claude-0/-home-user-Shamble-Frontlines/29a42dc7-ecd4-506e-b90a-121dd5e50988/tasks/b04hgafwt.output`
  · started 18:35 Amsterdam · stopped by the session at 19:41 after the last save.
- saver (restarted for the build, his word): background task bz6ds367a · started 20:55 Amsterdam · ended 22:45
  Amsterdam at its time limit ("watch stopped: time is up", last tick "nothing to save") · not restarted while the
  build waits for 2.1b and LANE-RULES (no agents, no workflow); restart it when step 5 begins.
- workflow run wf_93f9ff1a-f0b (task wt8ab4d43) · script as launched `foundation-toolchain/verify-fold-run1.js` · no
  args · notes `.checkpoints/foundation-toolchain/fold-check-*.md` · launched 19:10 Amsterdam · done 19:24, 4 of 4.
