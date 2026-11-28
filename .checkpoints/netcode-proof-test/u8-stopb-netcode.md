# U8 Stop B round: netcode-architect seat (NC-SB), xhigh, C9

Read: u8-brief.md rev 1.1 (sha256 3d102be9, checked) 0.1, :19-23, 7; advisor-u8-brief.md (595521d9, checked) C1-C10,
L1-L5; advisor-u6c-brief.md section 5 items 1-8; u8-build.md (cf07e435, checked) F1-F28, block log, fix pass; the
patch (60205020, checked). Built tree read only: W = .claude/worktrees/agent-ac59cce3e5d002441/spikes/netcode-proof.
Merged code read on main's checkout (M = spikes/netcode-proof): server/src/main.rs, tick_loop.rs, proc_read.rs,
super/src/main.rs, log/src/lib.rs, judge/src/{lib,eval}.rs, room/src/room.rs, lab/src/{main,probe}.rs,
marks/marks.json, marks/README.md, marks/SERIES.md, results/u3c-kernel-check/. Judged against the re-plan
(u8-brief.md:19-23 with L3, R1.md:256-262), not the stale plan.md lines.

## VERDICT: REVISE
4 High, 8 Medium, 7 Low. The package is careful in many places (C5's two halves, F19's " 0 received", the S14 rig's
like-for-like delay and recovery rules, the settings read once for both logs), but four defects would make R1's judged
blocks read wrong: npt-judge never sees R1's logs (NC-SB-1); the headers run's measured window outlives the load, so
-2 can FAIL at R1 and a FAIL is kept (NC-SB-2); the probe's six legs share one run id, which makes -3's L2 parts
INVALID (NC-SB-3); and S14's kernel sample is too small for a p99 within 10 %, so a MISS (netem wins) can come from
sampling alone (NC-SB-4). Each fix is small and inside U8's files or its interface list; none changes a pass bar.

## Findings (most severe first; file:line in W unless marked M)

### NC-SB-1 (High) npt-judge reads only `*.jsonl`; every R1 npt-log is named `*.log`
- What fails: M judge/src/lib.rs:18-33 collects only files whose extension is `jsonl` ("Every `*.jsonl` file under a
  folder"). R1 names the room logs `room-NN<sfx>.log` (ops/runs/r1.sh:160), npt-super's `super-NN<sfx>.log`
  (ops/sbin/npt-lib.sh:214, :217 require `.log$`), the probe's legs `room-02-<leg>.log` (advisor-u6c-brief.md C1 (b),
  R1.md:175), LG's npt-log `load-NN.npt.log` (r1.sh:230) and the browser legs `.npt.log` (ops/browser/lib.mjs:167).
- Scenario: at 12:57 `r1.sh collect` (r1.sh:534-540) runs npt-judge over /var/lib/npt/results/r1, finds no log, and
  writes verdicts with every mark NOT-RUN. R1.md:228 has the session read only these summaries, PROMPT-R1.md:99-100
  pushes only them, and R1.md:229 deletes the raw tarball at the close: unless someone renames files by hand, R1's
  judged blocks (-16, PM-0006-11, -3's L2 legs, the bites) are never judged. No test runs the judge's file rule on the
  names (u8-build.md fix pass item 9 checks read_log's content only).
- Smallest fix: name every npt-log `*.jsonl` (r1.sh:160's `log`, npt-lib.sh:214/:217's pattern and the super- name,
  keep/starts globs r1.sh:255-256 and :458, probe_presence r1.sh:408, LG's log_path r1.sh:230, lib.mjs:167), keep
  LG's stdout logs (load-NN.log) and S14's JSON as they are; add "the leg before the extension" to R1.md:243-249's
  interface list for U6c's per-leg names; a test 7 case that every npt-log path r1.sh writes matches judge
  lib.rs:27's rule.

### NC-SB-2 (High) The LG rooms run with duration_ms 0, so their measured windows outlive the load
- What fails: every room gets `duration_ms 0` (r1.sh:160), so it ends only when room_down closes stdin. U6a marks
  every tick after warmup_ticks `measured` until the stop (M server/src/tick_loop.rs:548-552; no `after` phase for
  ticks). In headers, LG runs 300 s from its own start (r1.sh:363), then load_wait polls until LG is stopped and only
  then room_down runs (r1.sh:364-365): the tail ticks are measured with the 48 clients gone. A closed link frees the
  slot (M room/src/room.rs:889-894) and snapshot_clients counts present slots only (M tick_loop.rs:266-273).
- Scenario: -2's "a connected snapshot-receiving client on every soldier" (min >= 48 over measured ticks, README:145
  default window) reads below 48 on the tail (and "300 live projectiles", a minimum, falls once the bots stop
  firing if those are bot-fired). README:242 makes one failed valid run fail the part whatever `min`, so -2 is FAIL
  at R1, and ADR 0001:1582 keeps a failing run ("never re-run until green"). ds: the ws_split pool runs from the mark
  to the run's end (u6b-brief.md:46-58), so condition 10's window is not "the 5 minutes after it" but also takes the
  48 WebSocket closes (r1.sh:470, :475-476).
- Smallest fix: give the LG rooms a duration that ends while LG still runs: headers duration_ms 300,000 with LG 330 s;
  ds duration_ms 420,000 (mark 120 s + 300 s) with LG 450 s; the block waits for npt-super's room_exit (as the probe
  block does, r1.sh:382-386), then LG's end. R1.md:158-160 and the hours stay. Calibrate carries no mark and may stay.

### NC-SB-3 (High) The probe's six leg logs share one run id, so -3's L2 parts read INVALID
- What fails: room_settings gives the probe one `run`, R1-probe-02 (r1.sh:156); U6c writes one log per leg with that
  leg's profile in its header (u6c-brief.md:192-193) and nothing in U6c's brief or R1.md:243-249 asks for a per-leg
  run. The judge groups logs by `run` (M judge/src/eval.rs:37-75), makes a run whose logs disagree on `profile`
  INVALID (eval.rs:52-61, :542-546), and a check without `from` reads every log of the run (README:162), so -3's
  P1 checks would pool all six legs anyway. -3's notes make the L2 probe "the start of every L2 block" before any L2
  mark counts.
- Scenario: U6c copies the settings' run into each leg's header (the natural reading of C1 (b)); at R1 every L2 part
  of -3 (P0 to P4, B1) is INVALID; R2's probe block repeats it.
- Smallest fix: R1.md:243-249 gains the interface line "each leg's header: run `<settings run>-<leg>`, profile = the
  leg, process `probe`, layer `L2`; npt-super's log keeps the settings' run"; probe_presence (r1.sh:401-433) also
  reads each leg log's header and footer (run ends `-<leg>` and differs per leg, profile = leg, process probe, layer
  L2, footer present), a miss making that leg INVALID; fixture cases in ops/tests/r1sh.test.mjs.

### NC-SB-4 (High) S14's kernel sample cannot hold a p99 within 10 %
- What fails (F17's "3 runs x 170 s" with stalls over the first 59 s only, compare.py:14-15, :45, :163-173): each run
  has 1,771 steady messages; at 2 % loss about 35 episodes, about 105 pooled. The kernel p99 is then rank 105 of
  about 106, the second largest. At a 100 ms RTT the RACK repair sits at about gap + RTT (133 ms) and the next tier
  (a lost retransmission, or two losses in a row) at about 166-266 ms, with a share of a few per cent: U3c's own
  loopback counts were 6 of 172 kernel episodes and a 4.2 % model share outside the RACK path
  (M results/u3c-kernel-check/out-20261108T004723Z/verdict.json). With 2-4 expected tail episodes in 106, whether
  the kernel's second largest lands in the 133 ms cluster or the tail is close to a coin toss; the model's p99 over
  100 seeds (about 3,500 episodes) lands in the tail. My estimate: a 30-50 % chance of a p99 MISS by sampling alone
  (reasoned, medium confidence; not measured at 100 ms). The burst half's p99 is the maximum of 30 recoveries against
  the model's 99th percentile of 1,000.
- Scenario: a MISS means "netem wins" (R1.md:151-155; plan.md:1256): -18's and -31's WebSocket parts move to VM runs
  with netem in R3, decided by noise; a lucky PASS keeps the model where its tail was never tested.
- Smallest fix (inside Mustafa's bar): about 1,000 steady episodes on the kernel side, e.g. a 10-minute steady window
  per run (probe.py:28-32, compare.py:45-47 and the knob schedule together, before any run), or several namespace
  pairs run at once in npt-s14; R1.md states the kernel's n and each p99's rank. If R1's hours cannot take it, the
  advisor takes it to Mustafa: the bar's power at R1's n is his to accept, never changed here.

### NC-SB-5 (Medium) The probe lists PM-0001-4 and -6, read from the brief's "C-U8-2/-4/-6"
- r1.sh:137 writes marks PM-0001-3,PM-0001-4,PM-0001-6 and R1.md:33 lists "-3, C-U8-2, -4, -6"; u8-brief.md:392
  means the conditions C-U8-4 and C-U8-6. -4 and -6 are R2's S30 marks (marks.json: where R2, parts params.soak S30).
  No verdict changes (no part matches), but every probe log's header and the plan he OKs claim the tick budget and
  snapshot encoding run at R1. Fix: marks=PM-0001-3; R1.md:33 "-3; conditions C-U8-2, C-U8-4, C-U8-6".

### NC-SB-6 (Medium) -16 is judged on the 20 lightest starts only
- -16's notes: "Every room start in every run" (marks.json PM-0001-16); R1.md:186-189 says -16's event is written at
  every start, but only the `start` rooms list the mark (r1.sh:135; clients 1), so the alone, headers (48 tokens),
  calibrate, ds and ten browser-room starts are never judged against "under 2 s". -16 can PASS at R1 while one of R1's
  starts took longer. Fix: PM-0001-16 in every room kind but probe and bite (r1.sh:134-140), R1.md's table to match.

### NC-SB-7 (Medium) LG's npt-logs never reach the results
- npt-load writes its npt-log to load-NN.npt.log (r1.sh:230), but npt-load-ctl's `log` returns only the stdout file
  logs/load-NN.log (ops/sbin/npt-load-ctl:73), collect judges RS's tree only (r1.sh:537), and teardown deletes LG first
  (R1.md:96). -1's LG part (u6c-brief.md:230 puts it in R1's headers block) and calibration's per-second record are
  lost. Fix: one fixed npt-load-ctl verb for that file (security reads it), saved by each LG block; and an interface
  line: npt-load's header carries the room's role, layer, profile, scene and tick_hz, which load_settings does not pass
  today (r1.sh:228-233), else the run's logs disagree and the judge makes the run INVALID (eval.rs:52-61).

### NC-SB-8 (Medium) Bring-up reads `alone` from a buffered log, and C1's INVALID never reaches the verdicts
- r1.sh:305-310 reads room-00.log 2 s after OPEN, before room_down; npt-server's log is a BufWriter (M server/src/
  main.rs:477) flushed only at the footer (M tick_loop.rs:941); F28's fix covers npt-super's log only. The read
  works only if about 8 KiB of tick records spilled the affinity event first; "unread" makes every timing block
  INVALID (r1.sh:315-323). Separately, timing_invalid writes a text file (r1.sh:82-88) that npt-judge does not read
  (r1.sh:537): with alone 0, verdicts.json shows -1 FAIL on the headers run where C1 rules the block INVALID.
  Fix: read both affinity events after room_down (min, as -1 does); collect judges the timing blocks apart when
  bringup/TIMING-INVALID exists and writes the C1 line at the top of the pushed summaries.

### NC-SB-9 (Medium) The probe block does not check C-U8-4's rate or P1's slack and policy edge reads
- u8-brief.md:363-364 gives C-U8-4's run conditions, the rate among them, to the probe block, and U6c relies on it
  ("the rate check is U8's probe block's", u6c-brief.md:208-209). probe_presence (r1.sh:401-433) checks only the
  /proc/net/udp and /proc/stat edge reads and P1's drops; R1.md:167-172 lists "the rate within 1 %" and "the slack
  and policy reads at both edges" as INVALID conditions nobody computes. Fix: per leg, the window's datagrams per
  direction over the window's seconds, 10,000/s within 1 %; P1's slack and policy reads present at both edges; names
  from U6c's merge (interface list); fixture cases.

### NC-SB-10 (Medium) A block that dies leaves its room running on the tick vCPU, and no block can be re-run
- Under `set -e`, a failed `lg` call (load_up, r1.sh:236, in headers :363) exits the block before room_down; the
  holder keeps npt-super's stdin open, so the room keeps its tick thread on vCPU 3 until teardown. Every later pinned
  room (starts, ds, bites: tick defaults to 3, r1.sh:131) shares that vCPU and the housekeeping vCPUs with it: the ds
  self-test's E and -16's starts are measured beside a stray room. A re-run of the block fails at once: the run id and
  log paths are fixed (r1.sh:156, :160) and both logs are create_new (M server/src/main.rs:466-476, super/src/
  main.rs:214-217). Fix: an EXIT trap per block that brings its rooms down; chain_run (r1.sh:542-559) refuses to start
  the next block while any npt-super runs (recording why); a re-run carries an attempt suffix in run id and log names.

### NC-SB-11 (Medium) The certificate-outage path R1.md promises is not built
- R1.md:110-112: if Let's Encrypt or Cloudflare fails, "the server chain may run on the room's own certificate". But
  every room names production.chain.pem and transport `both` (r1.sh:131, :163), `chain` checks only bring-up
  (r1.sh:581-583), and ds's WebSocket pin needs production.txt's ledger line (r1.sh:222-226). If U6b's room refuses a
  missing chain at start (check at its merge), every server block stops at room_up. Fix: r1.sh records whether the
  production chain exists; without it the server rooms run transport `wt` with no chain named, ds-selftest is recorded
  NOT-RUN, and R1.md:110-112 says exactly that.

### NC-SB-12 (Medium) S14's noise guard counts the bursts' retransmissions, and no VOID or INVALID run can be re-run
- compare.py:118-122 compares the steady window's episodes with tcpi_total_retrans over the whole run; the ten bursts
  add tens of retransmissions, so the advisor's noise guard (U3c run.sh: episodes beyond retransmissions are receiver
  noise) can no longer fire, and noise episodes just above half a gap would drag the stall p50 down. compare.py:232-233
  promises "one rerun inside R1's S14 block", but the s14 block (r1.sh:482-488) never computes a run's standing on RS.
  Fix: the client reads total_retrans at 59 s too (probe.py:116-125) and the guard uses it; the block runs
  `compare.py summary` on RS after npt-s14 and, with time left, one more npt-s14 run into a new folder, both kept.

### NC-SB-13 (Low) room_up takes a killed or failed start for an open room
- npt-super writes room_start with the elapsed time when the room misses its 10 s deadline or exits before OPEN
  (M super/src/main.rs:262-278), and room_exit right after; room_up (r1.sh:186-189) returns on room_start alone, so a
  block then drives LG at a dead room. Fix: room_up also requires no room_exit within about 1 s.

### NC-SB-14 (Low) load_ready counts iterations, not seconds
- r1.sh:238-246: 60 tries of ssh plus 1 s can take 80-120 s. Harmless today because npt-load exits 1 without READY
  after 30 s (u6c-brief.md:228); a deadline by `date +%s` would make R1.md:164's "60 s" literal.

### NC-SB-15 (Low) first_cpu_model is normalised differently from npt-server's read
- r1.sh:277 strips one leading space; M log/src/lib.rs read_local trims both ends. A model name with trailing spaces
  would write `changed` 1 at every start (PM-0006-11's report). Fix: trim both ends, or take the alone room's header
  `machine.cpu`.

### NC-SB-16 (Low) B1's packet count is an unstated interface
- r1.sh:168-169 passes probe_packets 1,000,000 for every leg; SERIES.md:472 wants 1,010,000 sent at B1, and L1 reads
  `packets` as "outside the bursts" (M lab/src/probe.rs:124-128). Right if U6c copies L1; name it in R1.md:243-249.
  PROBE_TOTAL_S (r1.sh:41) leaves out B1's extra second (inside its 12 s).

### NC-SB-17 (Low) S14's burst switch is recorded as one time for two directions
- probe.py:140-147 changes the client side, then the server side, and records the end of both (probe.py:161): the data
  direction switches a call's length earlier than recorded and the start runs late by two `tc` calls (about 5-15 ms
  each pair), close to the 20 ms slack (compare.py:50, :115-117). Bias at most about 1-2 % of a recovery; record each
  direction's switch time and start the "on" calls early by the measured call time.

### NC-SB-18 (Low) The day's results live only on RS until the last collect
- collect runs once, at 12:57 (R1.md:95). An RS loss (provider fault) after the server chain loses all of it. A
  collect after `chain-status` reads done (PROMPT-R1.md section 4) costs a minute.

### NC-SB-19 (Low, for R8) A browser leg's run differs from its room's run
- config.mjs's legHeader names run R1-<suite>-<browser>-<transport>-aNN; the room's log is R1-browsers-NN (r1.sh:156).
  R1's legs are rough, so nothing is judged; R8's ML parts of -14 read `from: process room` inside the browser's run
  (marks.json PM-0001-14) and would read nothing.

## The asked items, one by one
- **F17's S14 choices** (compare.py:9-34):
  - Stalls over the steady window only: sound; it keeps the 2 % repairs apart from the bursts and both sides use the
    same index cut (compare.py:71-72, :145). The sample it leaves is the problem (NC-SB-4).
  - BASE over every message: sound; the median stays on the undelayed delay while under half the messages are delayed
    (about a fifth here), and the model's p50_ns is also over every message (M net/src/tcp_model/tests.rs:1439).
  - Bursts on both directions at send time: sound (netem decides loss at enqueue, so packets already in its delay
    queue still arrive); MOVED's rider must model it the same way, as the header says. Recording: NC-SB-17.
  - 3 runs x 170 s: unsound for the p99 halves (NC-SB-4).
  - Burst slack 20 ms: sound as a validity limit (it moves a recovery by at most about 1.4 %), thin margin (NC-SB-17).
  - The model's fallback to BASE: sound; `late` lists every message more than 1 ms above p50 (tests.rs:1440-1443), so a
    message absent from it is within 1 ms of BASE, and a recovery message is always in it.
  - What a mark judges: none of these changes -18's or -31's bars; S14 decides only which model the WebSocket rows use.
- **F9** (tcpi_rtt INVALID outside 99-120 ms): sound as a "netem in place" check. Expected about 100.1 ms: U3c's same
  server read 0.125 ms on loopback (verdict.json rtt_ns 124,667), so ACKs go out on read and no delayed-ACK gap enters
  srtt. It is a validity condition the plan does not write (C12 (f) only "reads ... against" 100 ms): the advisor
  confirms it; it is no pass bar.
- **The probe block** (r1.sh:370-395, :401-433; fix item 4): the empty token, LG's 792 s, the wait on npt-super's
  room_exit and the per-leg edge reads are right; INVALID is never "unreadable counts as present". Gaps: NC-SB-3 (run
  per leg), NC-SB-9 (rate, slack and policy), NC-SB-5 (marks).
- **F19** (C5's RS half): sound. LG's own nftables accepts ICMP echo from any source (ops/cloud-init/lg.yaml:38), so
  only fw-lg can drop RS's echo, and " 0 received" (r1.sh:298) tells a silent drop from a ping that never ran.
- **`alone`**: the logic is right (U6a's not_alone exempts only the tick thread and kernel threads bound to that vCPU
  alone, M server/src/proc_read.rs:144-151); the read's timing and its INVALID label are NC-SB-8.
- **Fix item 8's values** (against M server/src/main.rs:62-103):
  - io_threads = the housekeeping vCPUs (2 with SMT siblings, else 3): sound; it matches ADR 0001:1420's placement,
    and -11 (R2) caps the runtime's CPU, not its thread count.
  - warmup_ticks 1,800 where LG joins, else 0: sound at the start (READY within 30 s, u6c-brief.md:228, inside 60 s);
    0 is right where no mark reads a measured window (alone, starts, bites, browsers; -1, -16 and PM-0006-11 read
    `all`). The window's end is NC-SB-2 (duration_ms 0 is also U8's value).
  - first_cpu_model from bring-up, empty at the first start: sound in meaning ("the run's first read" is the VM day's);
    normalisation NC-SB-15.
- **Marks per room** (r1.sh:134-144 against R1.md:27-43): alone, calibrate, ds and browsers none: right (ds is the
  advisor's self-test, not a mark); PM-0006-11 on headers only: right (the unpinned rooms and the probe write no
  affinity event, so its sibling check would read empty); headers -1, -2, PM-0006-11: right; probe: NC-SB-5; start:
  right but too narrow (NC-SB-6); bites: right.
- **F20**: none of the other bites is needed before Thu 3 Dec. -2/fire-stops needs a run past 10 minutes (an S30, R2);
  -3's clock-seed, rtt-each-way, loss-per-round-trip and normal-jitter are L1 bites (U2, in C) and not-net-of-base
  fails MP's part (R7); -14/unsplit-group belongs to the S30 runs (R2); -36/read-once and -52/predicted-damage are
  browser-side and their judged parts are ML's (R8). Bites count from any serving run in the judged set (eval.rs:1386-
  1398), so R1's report must judge R1 with C's -3 bite logs, and read part verdicts where a mark stays NOT-RUN.
- **F21** (browser rooms unpinned, one at a time): sound. They still start under taskset on the housekeeping vCPUs,
  write no affinity event, serve no mark, and their rough readings (-14, -29, -36, PM-0006-25, -56's item 2) read
  no server timing. Their starts should still count for -16 (NC-SB-6).
- **The probe's 732 s against R1.md's 0.20 h**: the arithmetic holds (6 x 122 s; about 0.21 h with start and stop,
  R1.md:240-242). The server chain comes to about 0.69 h of its 1.00 h slot by my count (headers about 0.09, probe
  0.21, calibrate 0.09, starts 0.02, ds 0.13, s14 0.15), so it fits; NC-SB-4's fix would use that slack.
- **F28's waits**: with npt-super flushing after each event, room_up and the probe's exit wait work (the exit wait
  works even without it: hold returns when the room exits and finish flushes, M super/src/main.rs:325-341, :379-385).
  Two more are needed: the alone read (NC-SB-8: npt-server's log, not npt-super's) and room_up's open test
  (NC-SB-13).

## What a provider outage does to a judged block
- Hetzner fsn1 refuses the VMs: nothing judged runs; nothing is judged wrongly; R1 waits.
- GitHub or crates.io down at bring-up: no build, teardown; nothing judged.
- Let's Encrypt or Cloudflare fails the order: as built, every server room names the missing production chain, so the
  whole server chain may stop at room_up (NC-SB-11); S14 alone still runs (namespaces, no room).
- A network fault between RS and LG mid-chain: the LG blocks stop at their first failed `lg` call and leave their room
  pinned on the tick vCPU (NC-SB-10), which then contaminates the ds self-test and the starts; S14 is unaffected.
- RS lost after the server chain: the day's raw results are gone, since collect runs only at the end (NC-SB-18).

## Does a judged block's result survive a re-run
- Room blocks: not on the same VM day as built (fixed run ids and log names, create_new logs: NC-SB-10); on a new day
  the same run ids return, and logs of two days judged together would merge into one run that disagrees on `date`
  (eval.rs:52-61). A re-run needs an attempt suffix in run id and log names. ADR 0001:1582 forbids re-running a
  failing run; only an INVALID or stopped one may run again.
- S14: yes. Each npt-s14 run writes a new folder (ops/sbin/npt-s14:28-31) and the model side is seeded, so a re-run on
  R2's RS adds evidence without touching R1's; but the in-block re-run compare.py promises is not built (NC-SB-12).
- The judge's verdict on fixed logs is deterministic; the C1 INVALID label is not carried into it (NC-SB-8).

## For the advisor (questions, not findings)
- Q1: R1's headers run makes -2 a measured run on code before U6d (interest writers) and U4k (the stress-scene hit
  rule). Any -2 check that R1's code cannot meet FAILs -2 for the whole test (README:242; ADR 0001:1582). Is R1's
  headers run meant as a measured -2 run (plan.md:819), or as a role outside the parts? Not U8's to decide.
- Q2: F9's 99-120 ms INVALID window for S14 (above): confirm it as a validity condition.
- Q3: if NC-SB-4's larger kernel sample does not fit R1, the p99 halves' power at R1's n goes to Mustafa.

## What I could not check
- No VM: netem's behaviour on `tc qdisc change` and RACK's numbers at 100 ms are reasoned from the kernel's rules and
  U3c's loopback counts, not measured; NC-SB-4's 30-50 % is an estimate.
- U6b and U6c are not merged: whether the room refuses a missing chain (NC-SB-11), how npt-load ends its sessions
  (NC-SB-2's snapshot-client half), and the probe's header fields (NC-SB-3) are read from their briefs.
- I ran no test and no npt-judge; the judge's file rule and run grouping are read from the code.

DONE
