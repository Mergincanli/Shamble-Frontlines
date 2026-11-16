# security-auditor: PR #54 (claude/night-shift-stints), round 1

Head checked: 20f9d59918eea1e3431ae830f72510f83f6424c7 on main 53272a11024dcec8349e67bd8c70a8db337e85bb.
Change: .claude/NIGHT_SHIFT.md +3 -0, after step 6's last line (lines 45-47). Read only; no build, no test.
(No Edit tool in this agent: the file is rewritten whole with Write at each block.)

## Plan
1. Diff and stat: confirm only that file, only those three lines (+3 -0), no guard/hook/settings/CI/script.
2. Mustafa's words (history.md 2177-2199): NS-STATUS (a) and his yes to NS-LINES (a); sha256 of the three lines.
3. Read NIGHT_SHIFT.md whole on the head (handoff, stop, branch rules).
4. Routine guard on main (.claude/hooks/guard.mjs): does it allow `node .claude/scripts/checkpoint.mjs watch <slug>
   --every 10 --for 25` with run_in_background; what it says about background processes, `--every`/`--for`.
5. checkpoint.mjs watch: what it does (commits/pushes? which branch? lock?), bounded lifetime, `--for` semantics.
6. CHECKPOINTS.md §5 (saver rules), handoff step 4 (45-minute stale status), two-runs-one-branch risk.
7. Threat model, findings, verdict.

## Progress

### 1. Scope (done)
- `git diff --stat 53272a1 20f9d59`: .claude/NIGHT_SHIFT.md only, 3 insertions. merge-base with origin/main = 53272a1
  = origin/main (head is on current main).
- `git diff --quiet 53272a1 20f9d59 -- .claude/hooks .claude/settings.json .claude/scripts .github tools/gate.mjs`:
  exit 0, so no guard, hook, settings, script, CI or gate change.
- GitHub REST (gh api repos/.../pulls/54 and /files): head 20f9d59918eea1e3431ae830f72510f83f6424c7, base main
  53272a1, draft, 1 file .claude/NIGHT_SHIFT.md +3 -0. The PR is what was reviewed.

### 2. His words and the hash (done)
- history.md 2177-2187 NS-STATUS (a), 2189-2197 NS-LINES (a) "yes", sha256 4e3d1565...3639223.
- `sed -n 45,47p` of the head file without the final newline: sha256 4e3d15658c0afaf40d922cdce9559a8dc2d1079ea04a8fc96e327798b3639223
  (matches). With the newline: c4797a52..., the same as history.md's copy with its newline: the head lines equal his
  lines byte for byte. Placement: right after step 6's last line (44), before step 7: matches "after step 6's last line".
- His scope bullets: only that file (yes), only the lines it needs (+3 -0), no guard/hook/settings/script change (yes).

### 3. NIGHT_SHIFT.md on the head (read whole, 78 lines) (done)
- Step 3 busy: status "running" and Last update < 45 min -> stop silently. Step 4 handoff: > 45 min -> switch to the
  checkpoint's cs/ branch (guard refuses if rules/CI/gate differ from main) and resume.
- Step 6 status at least every 30 min. Step 7 saver during long agent runs (CHECKPOINTS §5). Step 8: stop the saver with
  TaskStop (not pkill) before marking ready; then push nothing but a reviewed fix.

### 4. Routine guard on main (guard.mjs = launcher; rules in routine-guard.mjs, 1540 lines) (done)
- Bash, routines: R-PLAIN first (plainForm, 592+): the saver line `node .claude/scripts/checkpoint.mjs watch <real slug>
  --every 10 --for 25` uses only letters, digits, space, `.`, `/`, `-`: plain. R-PROG: program `node` in ROUTINE_PROGS (584).
  With CHECKPOINTS §5's `--trailer "Co-Authored-By: ... <noreply@anthropic.com>"` it stays plain (PLAIN_DQ 587 bars only
  $ ` \ inside double quotes).
- Background: routine-guard.mjs 688-689, 753-756: only git is refused with a background & or run_in_background; node is not.
- interpreter() 989-1013: node flags only --test/--check (none used); script tracked (1003), equal to origin/main's copy
  (1006, after originMainCheck 1004); for checkpoint.mjs save|watch, protectedInBranch(vcwd) (1165-1172, three-dot
  origin/main...HEAD) must find nothing (1008-1010). checkpoint.mjs is unchanged by this PR, so on main it is main's copy.
  Each stint start re-runs these checks (every ~25 min, vs once per 110 min before): a small gain.
- kill/pkill/killall refused with "use TaskStop" (980). Tools other than Bash/MCP/edit/read/web return null for
  routines (decide 396-428): TaskStop allowed. No approval prompt in a routine: PermissionRequest is denied (404-407), an
  ask is printed as a deny (guard.mjs 17-18, 44): a refusal is never a wait, so no stall.
- Status refresh: mcp update_issue_comment, night-shift: allowed only on #16 or its own PRs (1452). No new form.
- protectedRel (35-47): .claude/NIGHT_SHIFT.md is protected: a night run can't edit it; after the merge, a night branch
  cut from main before the merge differs from main in it, so its handoff switch is refused (switchTargetCheck
  1184-1221, two-dot main vs target): fail-closed, the usual effect of any kit merge.

### 5. The saver (.claude/scripts/checkpoint.mjs, unchanged) (done)
- parse 60-80: --every/--for in minutes; --for > 0 and <= 115 (default 110). `--every 10 --for 25` is valid.
- watch 189-211: end = start + 25 min; sleeps min(every, remaining): saves at 10 and 20 if .checkpoints/ changed, at 25
  `last('time is up')` does a final save then process.exit(0). Lifetime: 25 min plus one save (push retries 2+4+8+16 s).
  Also stops on a branch change, the checkpoint file gone, SIGINT/SIGTERM.
- save(background=true) 180-183: never pushes when the branch holds unpushed commits outside .checkpoints/ (no code
  published); push 127-141 never forced, a refused push left alone; only cs/ or claude/ branches; secret scan on notes.
- The saver makes no GitHub API call (git only): it can never refresh the status comment itself. Only a live model does.
- Under the Bash tool's default background timeout (30 min, per the tool's own description), so the harness would
  stop it anyway at 30 min; CHECKPOINTS §5 says set 7200000; either fits a 25-min stint.

### 6. CHECKPOINTS §5 and last night's evidence (done)
- §5 (131-166): saver with `--trailer`, one saver per task, check at every update, restart on `time is up`, TaskStop
  before ready. Stints fit: one at a time, restart on `time is up` is exactly "start the next stint".
- Night run's checkpoint git show 34a2e55:.checkpoints/npt-u4f-brief.md 94-106 (branch origin/cs/npt-u4f-brief):
  stints `--every 10 --for 25`; first saver stopped by TaskStop; stint 1 00:13-00:38 "time is up"; restarted 00:39 while
  netcode-architect ran to 00:48: the stint end did wake the run while a background agent ran.
- Its stint commits 1e97978, 1279cab, 459e436: "checkpoint(npt-u4f-brief): autosave [skip ci]" with Co-Authored-By and
  Claude-Session trailers: the run took `--trailer` from §5 along with the step-6 form. Not a finding (attribution, and
  NIGHT_SHIFT.md line 2 says follow CHECKPOINTS.md for every item).

## Threat model (durable; memory not written: this task limits writes to this notes file)
Actor: a night run (routine, SHAMBLE_ROLE=night-shift) under main's routine guard; hostile inputs are queue comments
by others (already ignored, step 2) and tool results. The lines are instructions to that run, not code.
- Q1 refused command / stall: no. The form is plain node on main's tracked, unchanged script; background is refused
  only for git; a routine never waits on a prompt (deny, not ask). At worst protectedInBranch denies -> item stops
  under Decisions needed (lines 6-7), which is the existing fail-closed path.
- Q2 long-lived background process: no new kind. The saver already runs in the background (§5, step 7, up to 110 min);
  a stint is shorter (25 min), self-ending, one at a time, and the harness's default background cap (30 min) is above
  it. Step 8's TaskStop-before-ready still applies to the stint running then.
- Q3 loosened stop or handoff: no. The 45-min busy/handoff thresholds and the 06:45/07:20/07:30 FINISH times are
  untouched. The liveness signal (Last update) is still written only by a live model through update_issue_comment;
  the saver can't touch it, so a cut-off run's status still goes stale and its handoff still happens. Trade-off: a run
  waiting on a hung agent now keeps the busy lock until it stops (the FINISH times still bind it); before, a later run
  would take over its branch while it was still alive.
- Q4 two runs on one branch: reduced. The hazard it fixes: a live run waiting > 45 min looked cut off and a later run
  handed off onto its branch. Residuals: SA-NS-1 (orphan stint after a cut-off, bounded) and SA-NS-2 (foreground
  waits are not woken). Both bounded by the saver never forcing and never publishing code.
- Q5 secrets, guard, hooks, settings, CI: untouched (git diff --quiet above). The lines name no secret, token or URL.

## Findings
- SA-NS-1 Low. The first stint is not tied to a fresh status. Order in the lines: stint end -> refresh -> next stint,
  so a stint orphaned by a cut-off after a refresh ends <= 25 min after it, 20 min inside the 45-min handoff. The
  first stint may start when the status is already up to 30 min old (step 6's floor): if the run is cut off then and
  the cut-off session's background process outlives it (not known for cloud routines), the orphan can save and push
  checkpoint commits up to ~10 min after a handoff run has taken the branch. Effect: one side's push is refused
  (never forced, left alone, CHECKPOINTS §5 "push refused: fetch and look"); no data loss, no code published. Before
  the change the same orphan could live 110 min. Fix: accept (recommended). To close it, "refresh the status, then
  start the first stint" would have to go into his step-6 text: his decision (changes_his_lines true).
- SA-NS-2 Low. The wake-up works only while the run waits idle on background agents. A run blocked inside a foreground
  Agent or workflow call is likely not woken by a stint end (background-task ends are reported between turns; not
  verified in a routine), so its status can still pass 45 min and a later run can hand off onto a branch a live run
  holds. Pre-existing (step 6's 30-min rule can't be met then either); the change neither creates nor widens it;
  last night's run shows the background case works (34a2e55, 00:38 wake while the agent ran to 00:48). Fix: accept now;
  a later, separate kit line ("while the saver runs in stints, launch agents in the background") would be his decision
  under KIT-FREEZE-2; it does not reword his three lines (changes_his_lines false).

## Verdict
APPROVE WITH FINDINGS: two Lows, both residual races that the change narrows, recommended as accepted or STATUS
follow-ups; neither triggers a push. His three lines are exact (sha256 match) and are not challenged.
