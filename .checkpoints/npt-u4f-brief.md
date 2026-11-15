# Checkpoint: npt-u4f-brief
- **Goal:** draft U4f's build brief (the netcode proof test's clock and commands): netcode-architect drafts;
  test-engineer and security-auditor check in parallel; one revision (rev 2). Notes only, no build, no test run, no
  pull request. · plan: issue #16 item 7 →
  `git show 781212774986118183a7c6521c34cca31b5fe2a0:.checkpoints/netcode-proof-test/night-2026-11-16.md`
  (after `git fetch origin claude/netcode-proof-test-plan-hcky5g`)
- **Branch:** cs/npt-u4f-brief · **Base:** origin/main 53272a1
- **Spike worktree (read only):** `.gate/npt-spike-1116` at 11fd190e9536702f0dcfc72d584b637ebc6184be
- **Last update:** 2026-11-16 00:51 Amsterdam

## Steps done
1. Start checks (role night-shift, window 00:11, no other run tonight), status comment 6902964789 posted; spike
   worktree added at 11fd190; branch created from origin/main 53272a1 · (this commit) · ok

2. netcode-architect drafted rev 1 (`u4f-brief-draft.md`, 837 lines, sections 0-12 present; notes end DONE) · the
   828e2f4 (the saver's autosave holds the finished draft and notes) · 11 findings (1 High, 7 Medium, 3 Low),
   12 U4F-Q, 10 D

## Current step
3. test-engineer and security-auditor check rev 1 in parallel, xhigh.

## Next steps
3. test-engineer (`test-engineer-u4f-check.md`) and security-auditor (`security-auditor-u4f-check.md`) check rev 1 in
   parallel, xhigh (plan item 7, Agents 2).
4. netcode-architect revises once for both checks: rev 2 with a change list at its top (plan item 7, Agents 3).
5. Commit and push everything; night report paragraph (branch, files, the checks' top findings); tick item 7.

## Open findings
- Drafter's findings (rev 1, table at the draft's end; notes `netcode-architect-u4f.md`): NA-4 High (Pong carries
  only the next tick: ClockSync's server time known to one tick, -47's p99 ≤ 5 ms near 6 ms at P2; U4F-Q6);
  NA-3, NA-5, NA-6, NA-7, NA-8, NA-9, NA-12 Medium; NA-1, NA-2, NA-10, NA-11 Low · open (for the checks and rev 2)
- SC-1 (the night session's own spot-check of NA-4, by reading): the code basis holds: the Pong carries `id`,
  `echo_ns`, `tick` and `hold_us` only (spike server/src/pump.rs:461-466; `tick` = last_tick + 1, lab/src/scenario.rs:459).
  My own arithmetic for D4's random-phase model also lands near 6 ms (tick floor: std 33.3/√12 = 9.6 ms, /√20 = 2.15
  ms; one-way asymmetry U[-10,10] each way: std 4.08 ms, /√20 = 0.91 ms; combined 2.34 ms; p99 ≈ 2.576 × 2.34 ≈ 6.0
  ms). But D4's "uniform phase" may not hold: TICK_HZ is 30 (sim/src/ticks.rs:12), so the draft's 10 pings a second is
  exactly 3 ticks, and on L1's simulated clock the samples' tick phase moves only with the delay jitter (±10 ms of a
  33.3 ms window); the floor error then averages less and leaves a phase-dependent bias, so NA-4 may be understated.
  For rev 2: D4 should also model the phase-locked case (ping period a whole number of ticks), or the brief should say
  the ping period must not be a whole number of ticks · open (passed to the reviser)

## Key numbers
- rev 1: 837 lines; 9 field tables; tests ≥ +60 measured / ≥ +74 bites build; 44 negative cases; 17 bites; 65 judged
  runs (36 measured, 12 twins, 17 bite runs); 14 carry placeholders; 15 @@WORDS@@; 29 recheck rows; 15 open items
  (12 routed, 3 left); 12 U4F-Q; 10 needs-a-run (D1-D10) (the drafter's counts)

## Decisions taken
- Branch name cs/npt-u4f-brief, as the plan's rule 2 names it · the plan · 2026-11-15

## Running work
- netcode-architect (draft rev 1), xhigh, launched 00:13 Amsterdam, done 00:48 (35 min, 117 tool uses); notes
  `.checkpoints/npt-u4f-brief/netcode-architect-u4f.md`, draft `.checkpoints/npt-u4f-brief/u4f-brief-draft.md`
- saver: run in 25-minute stints (`--every 10 --for 25`), so each stint's end wakes the run to refresh the status
  comment (a later hourly run reads a status older than 45 minutes as a cut-off). First saver b0yfndb40 stopped by
  TaskStop at 00:13 for this. Stint 1 bmctdoqdt 00:13-00:38: ok (saved 1e97978, 1279cab, 459e436; time is up). Now:
  background task bmv14vtwd · output
  `/tmp/claude-0/-home-user-Shamble-Frontlines/2cc6b1c7-405a-556f-81c6-a85b5a29465f/tasks/bmv14vtwd.output` · started
  00:39 · stops by 01:04 · last check 00:39: restarted
- test-engineer (check, `test-engineer-u4f-check.md`) and security-auditor (check, `security-auditor-u4f-check.md`),
  both xhigh, launched 00:50 Amsterdam in parallel, running (2 agents at a time)
