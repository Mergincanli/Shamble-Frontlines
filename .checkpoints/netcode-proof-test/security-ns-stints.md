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

### 2. His words and the hash (done)
- history.md 2177-2187 NS-STATUS (a), 2189-2197 NS-LINES (a) "yes", sha256 4e3d1565...3639223.
- `sed -n 45,47p` of the head file without the final newline: sha256 4e3d15658c0afaf40d922cdce9559a8dc2d1079ea04a8fc96e327798b3639223
  (matches). With the newline: c4797a52..., the same as history.md's copy with its newline: the head lines equal his
  lines byte for byte. Placement: right after step 6's last line (44), before step 7: matches "after step 6's last line".

### 3. NIGHT_SHIFT.md on the head (read whole, 78 lines) (done)
- Step 3 busy: status "running" and Last update < 45 min -> stop silently. Step 4 handoff: > 45 min -> switch to the
  checkpoint's cs/ branch (guard refuses if rules/CI/gate differ from main) and resume.
- Step 6 status at least every 30 min. Step 7 saver during long agent runs (CHECKPOINTS §5). Step 8: stop the saver with
  TaskStop (not pkill) before marking ready; then push nothing but a reviewed fix.

### 4. Routine guard on main (guard.mjs = launcher; rules in routine-guard.mjs, 1540 lines) (done)
- Bash, routines: R-PLAIN first (plainForm, 592+): the saver line `node .claude/scripts/checkpoint.mjs watch <real slug>
  --every 10 --for 25` uses only letters, digits, space, `.`, `/`, `-`: plain. R-PROG: program `node` in ROUTINE_PROGS (584).
- Background: routine-guard.mjs 688-689, 753-756: only git is refused with a background & or run_in_background; node is not.
- interpreter() 989-1013: node flags only --test/--check (none used); script tracked (1003), equal to origin/main's copy
  (1006, after originMainCheck 1004); for checkpoint.mjs save|watch, protectedInBranch(vcwd) must find nothing (1008-1010).
  checkpoint.mjs is unchanged by this PR, so on main it is main's copy.
- kill/pkill/killall refused with "use TaskStop" (980). No approval prompt in a routine: a refusal is a deny, never a
  wait (NIGHT_SHIFT.md 5-7; guard.mjs 17-18 "an ask never reaches a routine").

### 5. The saver (.claude/scripts/checkpoint.mjs, unchanged) (done)
- parse 60-80: --every/--for in minutes; --for > 0 and <= 115 (default 110). `--every 10 --for 25` is valid.
- watch 189-211: end = start + 25 min; sleeps min(every, remaining): saves at 10 and 20 if .checkpoints/ changed, at 25
  `last('time is up')` does a final save then process.exit(0). Lifetime: 25 min plus one save (push retries 2+4+8+16 s).
  Also stops on a branch change, the checkpoint file gone, SIGINT/SIGTERM.
- save(background=true) 180-183: never pushes when the branch holds unpushed commits outside .checkpoints/ (no code
  published); push 127-141 never forced, a refused push left alone; only cs/ or claude/ branches; secret scan on notes.
- Under the Bash tool's default background timeout (30 min, per the tool's own description), so the harness would
  stop it anyway at 30 min.

### 6. Guard tool dispatch, protectedInBranch, switch check, gh comment edit (in progress)
