# Advisor ruling: U8 Stop B round 2's open findings, 2026-11-23

Plan: read the three round-2 notes (u8-r2-netcode.md, u8-r2-security.md, u8-r2-infra.md), check the new snapshot's
sha256 and spot-check each open High in it and in the merged code (log/src/lib.rs, sudo's list output, Ubuntu's
/etc/issue.d); then rule part by part (1 every round-2 finding, 2 NC-R2-5's call, 3 pass 3's list with proofs, 4 its
size against the 6,000 stop, 5 the last round's scope). This file is rewritten after each part; the last line is DONE.

Status: all parts done (1-5).

**Inputs checked:** wip/u8-build.patch sha256 bca52b2c...e6eb7 (matches); the three notes, each ending DONE; my
ruling advisor-u8-stopb.md. **What I checked myself (read only):**
- NC-R2-1: W's probe block runs LG for PROBE_TOTAL_S + 60 s, then after the probe's exit sends `lg "stop 02"` (TERM,
  KILL after 10 s: npt-load-ctl stop) and saves `lg "npt-log 02"` straight into load-02.jsonl, `|| true` (r1.sh:516-531);
  the same unchecked save follows every LG block (r1.sh:504, :603, :632). read_log refuses a half-written line ("not
  one JSON record", log/src/lib.rs:658-659) and an empty file ("empty log: no header", :722-726), and the judge never
  skips a log (judge/src/lib.rs:36-46). So one such file fails collect's whole-tree call. Holds.
- SEC-R2-1 = INF-R2-1: rs.yaml's firstboot writes `> /etc/issue.d/50-npt-hostkeys.issue` as its first step under
  `set -euo pipefail` and the ERR trap (rs.yaml:109-119); in this Ubuntu 24.04.5 container `/etc/issue.d` does not
  exist and `dpkg -S` names no owner. Holds.
- SEC-R2-2: this container's sudo is 1.9.15p5. `sudo -n -l` through a pipe printed its 126-character Defaults line
  whole; the same command under a terminal with no width set wrapped it at 80 columns with an 8-space indent. A
  regular file takes the same no-pipe path (the seat's display.c read). r1.sh:424 writes the list to a regular file,
  and sudo_exact (r1.sh:158-177) counts every continuation as an extra line. Holds.
- The judge's complete rule: a log with no footer makes its run INVALID unless `params.killed` (eval.rs:575-582).
- U6c's probe runs on the housekeeping vCPUs, "no tick thread here" (u6c-brief.md:203-206).

## Verdict: REVISE, one pass 3 and a last round on the three open Highs alone
Round 2 closed all six round-1 Highs and every round-1 Medium. Three new Highs come from the fix pass's own lines:
NC-R2-1, SEC-R2-1 (= INF-R2-1) and SEC-R2-2. Each has a small fix. No pass mark, judge rule, marks.json number,
money, run day or sitting changes.

## 1. Every round-2 finding
| Id | Sev | Ruling | Pass 3 item |
|---|---|---|---|
| NC-R2-1 | High | Fix, wider than the seat's form. (a) The probe block waits for LG's own end (`load_wait 02 120`: LG ends at its duration, 60 s after the probe), and `lg stop` stays the abort path only. (b) One helper saves every npt-log copied into the results, room logs (keep) and LG copies alike. A complete file is saved as `.jsonl`. A file cut at a line end is saved as it is, since the judge reads it as cut short. A file cut inside a line keeps its lines up to the last full one as `.jsonl` and the whole file as `.jsonl.raw`. An empty file or one with no header line is saved as `.jsonl.empty` and named in the block's results. Why not the seat's `.cut` for every file without a footer: that hides the cut from the judge, whose complete rule makes such a run INVALID (eval.rs:575-582). The prefix keeps that rule's verdict, and the whole-tree call (bites, pooled checks) no longer fails on any cut line, whoever wrote it | 1 |
| SEC-R2-1 = INF-R2-1 | High | Fix: `install -d -o root -g root -m 0755 /etc/issue.d` as the step's first command, in rs.yaml and lg.yaml | 2 |
| SEC-R2-2 | High | Fix: `sudo -n -l \| cat > "$d/sudo-l.txt"` (a pipe, so sudo does not wrap; pipefail keeps its failure) | 3 |
| NC-R2-2 | Medium | Fix: on the READY deadline, room_down first, then `lg stop`; each READY poll bounded by the time left (`timeout`) | 4 |
| NC-R2-3 | Medium | Fix: calibrate's room outlives its LG again (duration_ms 0; the block: LG's end, then room_down). My ruling's "calibrate may take headers' values" was wrong for it: calibrate carries no mark, and its summary reads inputs over LG's whole run | 5 |
| SEC-R2-3 | Medium | Fix as the seat writes it: the PowerShell forms become `git -c *hookspath*` and `git * -c *hookspath*`; Bash keeps `git -c *` | 6 |
| NC-R2-4 | Low | Into U6c's message now (the main session): which records carry P1's two reads, per plan.md:1256's pair (the header's and the measured window's start edge). probe_presence reads exactly those at U6c's merge: one alignment, no U8 line now | U6c's merge |
| NC-R2-5 | Low | My call: narrow the skip (section 2) | 7 |
| NC-R2-6 | Low | Fix: the stdout copy named `load-NN$ATT.log`; the EXIT path fetches LG's npt-log through item 1's helper before any re-run | 8 |
| SEC-R2-4 | Low | Fix: name the listen read (`Start-Process <driver> -ArgumentList <driverArgs> -PassThru`, `Get-NetTCPConnection -State Listen -OwningProcess <Id>`, loopback only, `Stop-Process`), and repeat it for any driver start check 6 replaces | 8 |
| INF-R2-2 | Low | Fix (two stale lines) | 8 |
| INF-R2-3 | Low | Fix ("a fourth start") | 8 |
| INF-R2-4 | Low | Fix: curl adds `--speed-limit 1000 --speed-time 60` | 8 |
| INF-R2-5 | Low | Fix only the line "press Enter on the console to redraw the lines" (PROMPT-R1.md section 2 step 2). Drop the move of the package installs into firstboot: apt's update on Hetzner's mirror takes well under the 15-minute rule's margin, and the move changes cloud-init's structure for a minute's gain | 8 |
| INF-R2-6 | Low | Fix: `ssh -F <ssh> npt-lg grep btime /proc/stat` | 8 |

The builder's choices that the seats read sound stand as built: the null certificate pair, the 15-minute LG fallback,
npt-load-ctl's log delete, the slack and policy names, the 20-minute margin, npt-host's start limit and sandbox subset.

## 2. NC-R2-5: what C1 skips when bring-up reads alone 0
**Ruled: the chain skips headers, ds-selftest and the bites; it runs probe, calibrate, starts and s14.** This is my
C1's form (inside the plan, no mark moves, undoable), so it is mine; it goes in the day's message.
- `alone` says only that some task may still run on the tick vCPU. A block is spoiled by that only if what it judges
  runs there.
- **Skipped:**
  - headers: -1 reads alone itself, and PM-0006-11's sibling check rests on the same CPU setup;
  - the bites: PM-0001-1/shared-vcpu would fail for the stray task, not for the bite;
  - ds-selftest: DelayStream's microsecond lateness bar, with the pump's vCPU not yet known from U6b's merged code.
    It can join the run list at U6b's merge, if the pump runs on the io threads (one line in R1.md then).
- **Run:**
  - probe: U6c's probe has no tick thread and runs on the housekeeping vCPUs, which `npt-ctl cpus` keeps apart from
    the tick vCPU (u6c-brief.md:203-206);
  - starts: -16 reads spawn to Open against 2 s, and a co-runner on one vCPU costs milliseconds;
  - calibrate and s14 carry no affinity read.
- **What this keeps under alone 0:** -3's L2 legs, ImpairSocket's self-test and -16.
- **Text:** TIMING_BLOCKS reads "headers ds-selftest bites". PROMPT-R1.md section 2 and R1.md say "not run (C1)",
  replacing "recorded INVALID".

## 3. Pass 3: the list in order, each with its proof
Highs first, then the Mediums, then the Lows. The same builder runs it (opus, xhigh, its own worktree).
1. **NC-R2-1.**
   - Files: r1.sh (the probe block's wait; one save helper used by keep and every LG fetch); R1.md, one line on how a
     cut log is kept.
   - Proof: test 7 fixtures. A complete copy is saved `.jsonl`. A copy cut inside a line is saved as a prefix that
     ends at a newline, every line parsing, beside the `.raw`. An empty copy goes to `.empty`. The probe block waits
     for LG's end before any `lg stop`.
   - Proof the builder cannot give: the main session runs the merged npt-judge once over that fixture tree (no new
     code). The whole-tree call passes, and the run holding the cut copy reads INVALID by the complete rule.
   - Plant: the probe block's LG fetch bypassing the helper fails test 7.
2. **SEC-R2-1 = INF-R2-1.**
   - Files: rs.yaml, lg.yaml.
   - Proof: test 1 asserts `install -d ... /etc/issue.d` comes before the write in both files.
   - Plant: drop it, and test 1 fails.
3. **SEC-R2-2.**
   - Files: r1.sh's bringup.
   - Proof: test 7 asserts the pipe form. A fixture with the peers and rooms lines wrapped at 80, the way lbuf.c
     wraps them, must stop sudo_exact.
   - Recorded in the builder's notes: on this container's sudo 1.9.15p5, `sudo -n -l` into a regular file under .gate/
     wraps a line over 80 columns, and the pipe form does not.
   - Plant: the plain redirect back fails test 7.
4. **NC-R2-2.**
   - Files: r1.sh's lg_block and load_ready.
   - Proof: the "2.3" trace test reads room_down before `lg stop`, and a poll's bound shrinks to the time left.
5. **NC-R2-3.**
   - Files: r1.sh's room_settings and calibrate block.
   - Proof: the "2.3" test's calibrate row: the room has no duration, and LG ends before room_down.
6. **SEC-R2-3.**
   - Files: settings.json; R1.md's Stop B line (start check 4 run once in each shell must prompt, not deny).
   - Proof: the operator test checks that no deny matches PROMPT-R1.md's own git commands, compared case-insensitively.
   - Plant: the old PowerShell `git -c *` back fails it.
7. **NC-R2-5.**
   - Files: r1.sh's TIMING_BLOCKS; PROMPT-R1.md and R1.md lines.
   - Proof: the C1 fixture in test 7 skips exactly headers, ds-selftest and bites, and runs the rest.
8. **The Lows:** NC-R2-6, SEC-R2-4, INF-R2-2, -3, -4, the line from INF-R2-5, and INF-R2-6.
   - Files: r1.sh, npt-host.service, the cloud-init curl lines, R1.md, PROMPT-R1.md, rules.md.
   - Proof: test 1 for curl's options; test 7 for `$ATT` on the stdout copy.
- **NC-R2-4 has no pass-3 line:** U6c's message now; probe_presence at U6c's merge.
- **At the end:** every `node --test 'ops/**/*.test.mjs'` and `bash ops/tests/wrappers.test.sh` case passes; the
  redact step reports 0 findings; the package files' hashes are unchanged; a new snapshot with its sha256.

## 4. Size against SIZE-CHECK
By my count, about 85 added lines: item 1 about 45 (the helper and its fixtures), items 2-7 about 35, the Lows about
8. Likely 85-125, so U8 lands at about 5,960-6,000. The stop is live, most likely in item 8 if it trips at all.
- The builder writes the running total after each item.
- Past 6,000, it stops at the end of that item, and the main session sends Mustafa the three lines
  (history.md:2048-2051, as written).
- Nothing is moved or split to get under the stop. INF-R2-5's package move is dropped on its merits (section 1), not
  for size.
- My lean for those three lines: go on. What would be left is a handful of Low lines, U8 is off the cargo lane, and
  no date moves.

## 5. The last round's scope
The budget allows it on the open Highs alone: CLAUDE.md's third round, the last.
- **Two seats, xhigh, at once, one round.** Each reads only pass 3's lines for these Highs, their tests and plants:
  - netcode-architect: NC-R2-1, meaning the probe block's wait, the helper in keep and every LG fetch, its fixtures,
    and the main session's npt-judge run.
  - security-auditor: SEC-R2-1 = INF-R2-1 in both cloud-init files, and SEC-R2-2 (the pipe form, its fixture, the
    container record).
- infra-engineer is not needed, since INF-R2-1 is SEC-R2-1's fix.
- **Not re-read by a seat:** items 4-8 (Mediums and Lows). They rest on their tests and plants and on my read of the
  pushed head, which is not a round. SEC-R2-3's text is the security seat's own; NC-R2-2, -3 and -5 are small
  reorderings with their trace tests.
- **After it:** a Critical or High still open stops U8 for Mustafa, under Decisions needed. Nothing below High
  triggers a push; it becomes a STATUS follow-up named in U8's report.

## What I could not check
No VM: the Hetzner image's /etc/issue.d, sudo's output over ssh on RS, and npt-load's log buffering and TERM behaviour
are read from this container, sudo's source and the briefs. U6b and U6c are not merged. I ran no test and no
npt-judge. The size is an estimate.

## Decisions needed
None now. Conditional: **SIZE-STOP** (pass 3 passes SIZE-CHECK's 6,000 added lines). Why it matters: the build stops
until he answers. Options: go on and finish pass 3, or stop U8 where it is. Lean: go on.

## For Mustafa
Nothing now. These go in the day's message, each his to reverse:
1. Under alone 0 the chain skips only headers, ds-selftest and the bites (NC-R2-5).
2. A log cut inside a line is kept whole beside its readable lines. The judge reads it as cut short, so its run is
   INVALID by the judge's own rule, and the judge itself does not change.
3. The PowerShell deny on `git -c` narrows to the hooksPath key (SEC-R2-3), in the settings he approves at Stop B.

DONE
