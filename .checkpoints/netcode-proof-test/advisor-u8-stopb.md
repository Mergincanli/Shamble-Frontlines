# Advisor ruling: U8 Stop B (U8's one review round), 2026-11-23

Plan: read advisor.md (main), the three seats' notes, u8-brief rev 1.1 section 0.1 and 7, my earlier ruling
advisor-u8-brief.md (section 6, C9), advisor-u6c-brief.md:195-197, u8-build.md (F1-F28, Fix pass), NPT-TRIM and
TEST-FIRST in history.md; spot-check the Highs in W and the merged code; then rule part by part (1 findings, 2 builder
items, 3 Q1, 4 S14's sample, 5 overlaps, 6 now or later, 7 fix list, size, round 2). This file is rewritten after each
part; the last line is DONE.

Status: all parts done (1-7).

**Inputs checked:** advisor.md read from origin/main; wip/u8-build.patch sha256 60205020...c47d662 (matches); W read
only; the three seats' notes, each ending DONE; u8-build.md F1-F28 and its fix pass.
**What I checked myself (read only):** judge/src/lib.rs:18-33 reads only `*.jsonl` and W's r1.sh names every room log
`room-NN<sfx>.log` (NC-SB-1 holds); r1.sh's room_settings writes `duration_ms 0` for every room and headers runs LG
300 s then room_down (NC-SB-2 holds); judge/src/eval.rs:37-75 groups logs by `run` and :540-546 makes a run whose logs
disagree on `profile` INVALID, while the probe has one run id (NC-SB-3 holds); probe.py and compare.py fix 60 s steady,
10 bursts, 3 runs (NC-SB-4's counts hold); u6b-brief.md:209-212 plans the room's own certificate "for runs with no
browser" (INF-SB-1's fix is U6b's design); server/src/tick_loop.rs:1118-1150 writes every -2 series, everyone_in_view
included; replan-marks.md:125 and :152 make U4k's hit rule the ES one (carry 93); marks/README.md:150-153 (an empty
series is a gap) and :236-245 (verdict steps); log/src/lib.rs:659 and judge/src/lib.rs:36-46 (one cut line fails the
whole folder's read); W's rs.yaml firstboot (the fingerprint only at cloud-init's end; the build paths); u7-brief.md:61-62
(the remap) and advisor-u7-brief.md:62-65, :235 (my C13: the dist compared in R1's report, "stops nothing").

## Verdict: REVISE
Six Highs are open (NC-SB-1 to -4, SEC-SB-1, INF-SB-1). Each is fixable inside U8's files and its interface lists. No
pass mark, judge rule, marks.json number, money, run day or sitting changes. One fix pass now (section 7); then round 2,
the same three seats, on what changed only. Two additions of mine: ADV-SB-1 (Medium, below) and a term on SEC-SB-5
(section 5).

## 1. Every finding: fix now (U8's next fix pass; item numbers are section 7's) or defer with a taker
| Id | Sev | Ruling | Item / taker |
|---|---|---|---|
| NC-SB-1 | High | Fix now. Every npt-log is named `*.jsonl`; the judge's rule stays (changing it would be a judge change: his) | 1 |
| NC-SB-2 | High | Fix now, the seat's durations (headers 300,000 ms with LG 330 s; ds 420,000 ms with LG 450 s; calibrate may take headers' values), the block waiting for npt-super's room_exit, then LG's end. Plus the READY guard (section 3) and U6c's term: npt-load writes its footer on every end, the server's close included, and never rejoins | 3; U6c's message |
| NC-SB-3 | High | Fix now on U8's side (interface line, probe_presence reads each leg's header and footer); U6c's message takes the line | 4; U6c's message |
| NC-SB-4 | High | Fix now, in the form of section 4 | 5 |
| NC-SB-5 | Medium | Fix now: the probe lists PM-0001-3 only; R1.md:33 reads "-3; conditions C-U8-2, C-U8-4, C-U8-6" | 15 |
| NC-SB-6 | Medium | Fix now: PM-0001-16 in every room kind but probe and bite. It carries out -16's own notes ("Every room start in every run": marks.json), so it changes no mark | 15 |
| NC-SB-7 | Medium | Fix now: one fixed npt-load-ctl verb for load-NN's npt-log, saved by each LG block; load_settings passes role, layer, profile, scene and tick_hz from the room's settings. npt-load's header and footer go to U6c's message | 12; U6c's message |
| NC-SB-8 | Medium | Fix now, one change of form: `alone` read after room_down (both affinity events, the min); when bring-up writes TIMING-INVALID, the chain does not run the timing blocks (each recorded "not run: alone 0 (C1)" beside the task list; calibrate and s14 still run), and collect's summaries open with the C1 line. This replaces "judge apart": no log is made that could FAIL a mark under a state C1 already rules invalid | 10 |
| NC-SB-9 | Medium | Fix now: logic and fixtures; the record names at U6c's merge | 13 |
| NC-SB-10 | Medium | Fix now: an EXIT trap per block; the chain refuses a block while any npt-super or another chain runs (recorded why); each re-run carries `-aN` in its run id and log names; a block marked done is refused a re-run unless its folder holds an INVALID file (ADR 0001:1582: a failing run is never re-run). Every attempt is kept and judged | 11 |
| NC-SB-11 | Medium | Fix now by INF-SB-1's form (section 5): the server chain then names no production file, so R1.md:110-112's promise holds with no NOT-RUN branch | 2 |
| NC-SB-12 | Medium | Fix now: the noise guard reads the steady window's retransmissions; the s14 block runs `compare.py summary` on RS; a VOID or INVALID S14 re-runs the same day only if it ends 5 minutes before the bites' start, else on R2's RS (a new folder, R1's kept). A MISS is never re-run | 14 |
| NC-SB-13 | Low | Fix (rides r1.sh); proven live at U6b's merge (F28's flush) | 20 |
| NC-SB-14 | Low | Fix (two lines) | 20 |
| NC-SB-15 | Low | Fix (trim both ends) | 20 |
| NC-SB-16 | Low | Fix: the interface line now, the value checked at U6c's merge | 20 |
| NC-SB-17 | Low | Fix the recording: each direction's switch time, the validity rule on the data direction's window. Drop the early-start compensation: the recorded windows show any bias, which is under 2 % of a recovery inside a 10 % band | 20 |
| NC-SB-18 | Low | Fix: a collect as soon as `chain-status` reads the server chain done (one PROMPT-R1.md line) | 16 |
| NC-SB-19 | Low | Carry: R8's run package (STATUS follow-up named in U8's report); R1's legs are rough | R8 |
| SEC-SB-1 | High | Fix now, one fix with SEC-SB-4 (section 5) | 6 |
| SEC-SB-2 | Medium | Fix now; test 1's assert covers both cloud-init files. It matters more now: F12's term (section 2) lets npt log in during the build | 7 |
| SEC-SB-3 | Medium | Fix now, the setting verified against the current docs first (section 5) | 8 |
| SEC-SB-4 | Medium | Fix now, with SEC-SB-1 | 6 |
| SEC-SB-5 | Medium | Fix now, in the form of section 5 (revises my C13 for U7) | 9 |
| SEC-SB-6 | Low | Fix (rides the cloud-init edits) | 7 |
| SEC-SB-7 | Low | Fix (rides the operator edits): the hook's own ref check is the part that counts | 8 |
| SEC-SB-8 | Low | Fix (RS's exact sudo list; LG's three checks in PROMPT-R1.md section 2) | 16, 19 |
| SEC-SB-9 | Low | Split. Now: the options a read-only server cannot trip (ProtectHome=yes, PrivateTmp=yes, CapabilityBoundingSet= empty, RestrictNamespaces=yes, LockPersonality=yes, SystemCallArchitectures=native; NoNewPrivileges kept). At U6c's merge, from its host's file and system-call needs: ProtectSystem=strict, PrivateDevices, MemoryDenyWriteExecute, SystemCallFilter, RestrictAddressFamilies; any not added then is carried to R2's package with R1's `systemd-analyze security npt-host` reading. Why: nothing here runs systemd, and a unit that will not start loses the browser legs (`r1.sh host` refuses). What stays open is a security finding left open: U8's report names it | 17; U6c's merge; R2 |
| SEC-SB-10 | Low | Fix (two nft lines, `nft -c` in test 1) | 7 |
| SEC-SB-11 | Low | Fix (two lines) | 8 |
| INF-SB-1 | High | Fix now: the logic now; U6b's own-certificate field form and the hash file's WebSocket pin at U6b's merge | 2; U6b's merge |
| INF-SB-2 | Medium | Fix now: bounded fetches (cloud-init) and R1.md's delete rule and provider list; with F12's term | 7, 16 |
| INF-SB-3 | Medium | Fix now: a Stop B step before FIXED-IP; nbg1 is the fallback at the same prices (mine: no money moves); the location a fill | 16 |
| INF-SB-4 | Medium | Fix now: "exactly the list's /32s" at Stop B and 09:10, source counts read aloud | 16 |
| INF-SB-5 | Medium | Fix now: C7's new text confirmed (section 5) | 16 |
| INF-SB-6 | Low | Fix: StartLimitIntervalSec=300, StartLimitBurst=3, host_active refuses a failed npt-host.path; the chain written under a temporary name then renamed goes to U6c's message | 17; U6c's message |
| INF-SB-7 | Low | Fix (slack per VM against its delete-by) | 16 |
| INF-SB-8 | Low | Fix (`command -v dig tcpdump` before the build) | 7 |
| INF-SB-9 | Low | Fix: read PID 1's Cpus_allowed_list after set-property; still holding the tick vCPU, run the fallback too, record both. Low by chance, but alone 0 loses every timing block | 18 |
| INF-SB-10 | Low | Fix (one line each) | 16 |
| INF-SB-11 | Low | Fix (10 minutes, then to him) | 16 |
| INF-SB-12 | Low | Fix, with NC-SB-10: a 20-minute margin; LG blocks also against LG's delete-by | 11 |
| INF-SB-13 | Low | Fix (one sentence: the dates now rest on advisor-u6c-brief.md's cadence) | 16 |
| ADV-SB-1 | Medium (mine) | Fix now. collect makes one npt-judge call over the whole tree, and one cut line in any log fails the whole read (log/src/lib.rs:659; judge/src/lib.rs:36-46: "a judge never skips one"). A room killed at npt-super's deadline, or LG stopped by `lg stop`, can leave one; R1's verdicts are then lost again (the laptop reads only summaries: R1.md section 7). On a whole-tree error, collect judges each block folder apart and records the file the error names; the whole-tree call stays first (pooled checks and bites need it) | 1 |

Budget: this is the round. Lows ride the push the Highs need; none would trigger one alone. A Medium or Low is
deferred only where named above (NC-SB-19, part of SEC-SB-9), each with its taker.

## 2. The builder's items left to me
- **F9 (S14's kernel run INVALID outside 99-120 ms): confirmed as a validity rule, not a pass bar.** It tests that
  netem's 100 ms round trip is in place (my C12 (f) read "against" 100 ms; the netcode seat expects about 100.1 ms, U3c's
  same server reading 0.125 ms on loopback, so no delayed-ACK gap enters srtt). A run outside it did not test the model:
  INVALID, never a MISS. It moves no bar of his: TCP-FREEZE's 10 % and NETEM-BAR (a) stand. The model side uses the
  measured tcpi_rtt (compare.py env), so the comparison stays self-consistent inside the window. R1.md section 5 says
  this in one line; an INVALID S14 reads its value and comes to me; a re-run goes per NC-SB-12.
- **F12 (the first SSH by the console fingerprint; plan.md:287 stale): confirmed, with one term.** Under DNS-01 by hand
  the first SSH must come before the certificate, so the console fingerprint anchors it (him present), and the
  HTTPS-published line after the production certificate is the cross-check before the server chain and every browser
  room (the security seat: the order holds). plan.md:287 is already on R1.md's superseded list. **The term:** the
  fingerprint reaches the console today only through cloud-init's keys_to_console, which runs after runcmd's whole
  build (INF-SB-2 (a)), and nothing here shows that the Hetzner console displays cloud-init's /dev/console output (the
  infra seat could not check it). So firstboot's first step, before any fetch, writes the SHA-256 fingerprints of the
  host keys (made earlier, in cloud-init's init stage) to the screen the console shows (tty1), on RS and on LG; the fix
  pass picks the means (a write to /dev/tty1, or /etc/issue.d with an agetty reload) and records it; keys_to_console
  stays as the second copy. PROMPT-R1.md section 2 step 2 names where to look. If no fingerprint shows 15 minutes after
  a VM's creation, no SSH is made and INF-SB-2's teardown rule applies: never trust on first use (over a first-use
  SSH the session would print a TXT value he puts into Cloudflare, so an impostor could get a trusted certificate for
  his name). Test 1 asserts the step comes before the first fetch in both files.
- **F17 (S14's choices): confirmed,** as the netcode seat reads them (steady-window stalls, BASE over every message,
  bursts on both directions at send time, the 20 ms slack, the model's BASE fallback), except the sample (NC-SB-4,
  section 4), the noise guard (NC-SB-12) and the switch recording (NC-SB-17).
- **F20 (the other R1 bites not placed): confirmed, none is needed before Thu 3 Dec.** R1.md section 8 lists each with
  the run or unit that judges it (the netcode seat's list, read against replan-marks.md's table) and says R1's report
  reads part verdicts where a mark stays NOT-RUN for a bite R1 does not run (bites count from any serving run in the
  judged set: eval.rs:1386-1398; collect on RS judges R1's logs only).

## 3. Q1: R1's headers run and -2
**Ruled: it stays a measured -2 run,** as plan.md:819 and my Q-U6A-2 ruling (advisor-u6a-brief.md:71-73) place it.
That changes nothing about which runs count, so it is not his to decide; it is named in the day's message so he can
reverse it.
- **The premise is weaker than it reads.** U6a writes every -2 series on R1's code, everyone_in_view included
  (tick_loop.rs:1118-1150; Q-U6A-2 put them in U6a so that R1 counts). U4k's hit rule is the ES one (carry 93:
  replan-marks.md:125, :152), and R1 runs SS only. SS passed at L1 on BRAWL-1 and U4f's sets (replan-marks.md:19).
  U6d's interest writers are not what -2 reads at R1.
- **What could fail -2 at R1 for a reason that is not -2's own is bring-up, and two guards close it.** (1) NC-SB-2's
  windows end inside the load. (2) The READY guard: when LG has printed no READY 10 s before the room's warm-up ends
  (warmup_ticks 1,800 = 60 s), the block stops LG and brings the room down before its first measured tick. Its -2
  series are then empty, which is a gap, so the run reads INVALID, never FAIL (README:150-153, step 4 at :239-241).
  A FAIL that is left after both guards means the load is not the stress scene on the real rig: what -2 exists to say.
- **What he should know (one line in the day's message):** a FAIL there is kept for the whole test (ADR 0001:1582, no
  retries), and an INVALID one keeps -2's SS part INVALID beside later passes (README step 5, :242-244), which would
  come to me and then to him. If he would rather have R1's -2 as a rough reading, judged from R2's runs on, that
  changes which runs count (his), and it is a one-line change to the headers room's marks until Stop B. **My lean:
  keep it measured.**

## 4. NC-SB-4 and Q3: S14's sample
**The form: the main session's 10-minute steady window, without moving any block.** S14 keeps its place as the
server chain's last block, with a longer schedule, and runs on into the browser sitting.
- **Schedule per run** (probe.py, compare.py, the s14 tests and R1.md change together, before any run): 600 s steady
  (stalls over messages written before 599 s), then 34 one-second bursts, one every 10 s (600 to 930 s), then the
  20 s tail: 950 s and 28,500 messages a run; three runs, about 48 minutes. MOVED's rider reads the longer burst list
  in the same (start_ns, len_ns) form; the model side takes NPT_KC_MESSAGES from `compare.py env` as now.
- **The sample:** about 1,000 steady episodes pooled (17,970 steady messages a run at 2 % loss, three runs, a little
  fewer where two losses fall in one repair), so the stall p99 is about the 11th largest, not the 2nd; and 102 burst
  recoveries, so the burst p99 is the 2nd largest, not the maximum of 30. That meets the netcode seat's "about 1,000"
  for the stall half and gives the burst half the same fix, which the seat named but left open. R1.md section 5
  states each half's n and the rank its p99 reads.
- **The day:** the chain's other blocks about 10:39-11:14 (NC-SB-2 adds about 90 s), S14 about 11:14-12:02; the
  browser legs stay 11:39-12:57, so they overlap S14 by about 20-25 minutes. Hours billed (4 per VM), planned EUR 0.62,
  the cap EUR 0.80, both delete-bys, his sittings (161 minutes) and the slot table (133) do not move. Nothing here is
  his: no money, no bar, no run day; it sits inside the plan he OKs at Stop B.
- **Why the overlap is sound** (my reading; the netcode seat reads it in round 2): S14's traffic runs on a veth pair
  inside its own two namespaces with their own netem qdiscs (npt-s14:39-48), never on eth0's path. The browser rooms
  beside it are light (one room at a time, one browser attempt at a time, -36's three browsers for ten minutes,
  unpinned on the housekeeping vCPUs: R1.md section 4), while S14's effects are tens of milliseconds (the episode
  threshold is half a gap, 16.7 ms; 10 % of a RACK repair is about 13 ms). And any contamination that could matter is
  caught, never silent: send lateness p99 over 1 ms makes a run INVALID, steady episodes beyond the steady window's
  retransmissions make it VOID (NC-SB-12's fix), tcpi_rtt outside 99-120 ms INVALID. r1.sh's browsers and
  browsers-down write a UTC stamp line, and the s14 block copies them beside each run, so the overlap is on record.
- **Text that follows:** PROMPT-R1.md section 5 starts the browser legs at about 11:39 while chain-status shows s14
  running (every other server block must read done or stopped); R1.md's table shows S14 at about 0.80 h beside the
  browser legs (0 added hours) and the timeline; the bites chain refuses to start while the server chain runs.
- **Why not the others:** (a) thirty runs pass the cap as placed and, re-ordered, need his cap of EUR 1.00; (b) as
  the infra seat placed it pushes the browser sitting to 12:06, makes the bites refused and uses the spare hour; parallel
  namespace pairs change the root wrapper npt-s14 (a new security read) and make the bursts' tc calls contend across
  pairs inside NC-SB-17's 20 ms slack. None of the three fixes the burst half's n.
- **Nothing for him:** no cap change, and no p99 power to accept at a smaller n.

## 5. The overlaps and the setting
- **INF-SB-1 with NC-SB-11: one fix, INF-SB-1's form.** room_settings names the production pair only for the browser
  rooms; every other kind (alone, headers, probe, calibrate, start, ds, bite) takes U6b's own-certificate form, which
  u6b-brief.md:209-212 already plans "for runs with no browser"; load_settings' WebSocket pin reads the room's hash file
  whenever the room serves its own certificate. Bring-up then opens room 00 before any certificate exists, and the
  server chain depends on no production file, so NC-SB-11's branch (transport `wt`, ds NOT-RUN) is not needed and
  R1.md:110-112's outage path holds by construction. The exact field form waits for U6b's merge (section 6).
- **SEC-SB-1 with SEC-SB-4: one fix, every driver started by lib.mjs and bound to loopback.** lib.mjs starts all three
  drivers itself, from the recorded binary after its SHA-256 check: chromedriver and msedgedriver with `--port=<p>` and
  no `--allowed-ips`; geckodriver with `--host=127.0.0.1 --port=<p>` and its WebSocket port left to the system;
  remote() connects to 127.0.0.1:<p>, so WebdriverIO starts no driver (the security seat's utils build/index.js:766-770).
  This supersedes u8-brief Block 9's `allowedIps ["127.0.0.1"]` and SEC-R1D-2's "keep geckodriver's localhost default"
  (the seat corrected its own premise). C8's start read adds the per-program inbound allow rules (none enabled for a
  driver path or node.exe, else stop) and the line "answer Cancel to any Windows Firewall dialog; one appearing is a
  stop". Evidence: test 4 and two plants (section 7), and, because the container cannot run Windows drivers, one dry
  start of each driver on his laptop at Stop B, read back with `Get-NetTCPConnection -State Listen -OwningProcess
  <pid>` (read only, one prompt each): loopback only, else stop.
- **INF-SB-5: confirmed.** C7 now reads: "The Limits tab is read for the largest run, as plan.md:871-872 asks: RS plus
  five CAX21s, and six Primary IPv4s (the two kept, four made with the extra servers), so any increase asked at Stop B
  is in place by Mon 14 Dec." Every server's public IPv4 is a Primary IP (the seat's Hetzner page), and the test runs
  on IPv4.
- **SEC-SB-3's setting, `blockReadsOutsideWorkingDirectories`: never relied on from the seat's notes alone.** Before
  the operator edit, the fix pass, or the main session for it (code.claude.com is not on settings.json's documentation
  list, so that read is a day-session fetch through the prompt or the classifier), reads Claude Code's permissions and
  settings pages that day and records: the setting's exact name, where it goes, the version that has it, and what it
  covers (the read-only Bash set and the file tools). Stop B then checks the laptop's Claude Code version against it.
  If the pages do not name it so, or it does not cover the read-only Bash commands, the fix falls back to deny rules
  for each read-only command naming `npt_operator` or `.ssh`, in both shells, and says so in rules.md. The passphrase,
  with the key held in Windows' ssh-agent from Stop B, stands either way: it makes a leaked file ciphertext.
- **SEC-SB-5, my term (it revises my C13 for U7, advisor-u7-brief.md:235, "a mismatch is recorded and stops
  nothing"):** the seat is right that a compare after the run protects nothing on the day. The expected list is
  committed at X (ops/runs/r1-dist.sha256, filled at Stop B), and bring-up compares RS's dist.sha256 with it. On a
  difference: `r1.sh host` and every `browsers` call refuse, so no page is served; the differing file names come to
  him and me; the server chain still runs, its binaries' SHA-256s kept for R1's report as C13 has it. Not "stop before
  any room": the exposure is the page his browsers run, while a room faces only his /32 and LG, with no egress. The
  browser legs then run only on his typed word that day, after he sees which files differ (a security call, his).
  Because U7's remap makes equality likely but unverified until RS (advisor-u7-brief.md:65), the main session builds
  U7's dist twice in this container before Stop B's fill, at two different clone paths and CARGO_HOMEs; equal lists
  become the expected list; a difference comes to me before Stop B.

## 6. What can be fixed now, and what waits for U6b's, U6c's or U7's merge
- **Now:** every U8-side change in section 7, items 1-20, written against the merged U6a code and the briefs' names.
- **At U6b's merge** (built now from 21765e3; its certificate choice, settings names and npt-super's flush are U6b's):
  INF-SB-1's field form (how the room is told to serve its own certificate) and which hash file the WebSocket pin
  reads; U6b's settings names in room_settings (test 7 reads the merged struct, as it does for U6a's); F28's flush,
  which room_up's live read, the probe's exit wait and NC-SB-13's room_exit check need: written now against U6a's
  event names, proven live at the merge.
- **At U6c's merge; into U6c's build message now** (its build has not started; the main session adds these lines,
  inside the plan; U6c's own round's netcode seat reads them against U6c's code):
  1. NC-SB-1: the leg logs are `room-02-<leg>.jsonl` (the leg before the extension); npt-load writes its log at the
     path load_settings gives (`.jsonl`).
  2. NC-SB-3: each leg's header: run `<settings run>-<leg>`, profile = the leg, process `probe`, layer `L2`;
     npt-super's log keeps the settings' run.
  3. NC-SB-7 and NC-SB-2: npt-load's header carries the room's role, layer, profile, scene and tick_hz beside run,
     set, marks, date, commit and seed, all from its settings; it writes its footer on every end, the server's close
     included; it never rejoins once the server has closed.
  4. NC-SB-9: the names of the per-leg rate counts and of P1's slack and policy reads, to U8's interface list.
  5. NC-SB-16: `probe_packets` counts the packets outside B1's bursts, as L1 reads it (lab/src/probe.rs:124-128).
  6. INF-SB-6: the production chain is written under a temporary name, then renamed into place (the path unit fires
     on its first appearance).
  Then at the merge: SEC-SB-9's rest from the host's needs; the probe block's hours against U6c's one-leg reading
  (already in R1.md section 8).
- **At U7's merge, before Stop B's fill:** SEC-SB-5's two-path dist build.
- **Proof of the alignments:** each is a one-line edit, proven by test 7 reading the merged code and by the fixtures;
  I read each pushed head (not a round).

## 7. The fix pass: the list in order, the size, and round 2
Highs first, then the security Mediums, then the rest; each item names its findings, its files, and the test or
plant that proves it. One builder (opus, xhigh, its own worktree), as the last pass.
1. **NC-SB-1, ADV-SB-1.** r1.sh (room_settings' log, keep, the starts count, probe_presence's leg names, load_settings'
   log_path, collect), npt-lib.sh:214/:217 (the super log's pattern and name), lib.mjs:167, R1.md's interface list.
   Proof: a test 7 case that reads the judge's extension rule from crates/judge/src/lib.rs at test time and matches
   every npt-log path r1.sh, npt-lib.sh and lib.mjs write; a fixture tree with one cut log makes collect write
   per-block verdicts and name the file; plant: one `.log` back in room_settings fails test 7.
2. **INF-SB-1, NC-SB-11.** r1.sh room_settings and load_settings, R1.md section 3. Proof: test 7: no non-browser kind
   names `production.*`; ds's WebSocket pin comes from the room's hash file; plant: the production pair back on
   `alone` fails it.
3. **NC-SB-2 and Q1's READY guard.** r1.sh (a duration per kind; the headers, ds and calibrate blocks), R1.md section 5.
   Proof: test 7: each LG-joined room ends at least 30 s before its LG; headers' READY deadline sits 10 s before the
   warm-up's end, and without READY the room is down before any measured tick (fixture); plant: `duration_ms 0` back
   fails it.
4. **NC-SB-3.** r1.sh probe_presence, R1.md's interface list. Proof: r1sh.test.mjs fixtures: a leg carrying the
   settings run, two legs sharing a run, a wrong profile, a missing footer: each makes that leg INVALID; six good legs
   read present; plant: the run check removed fails a fixture.
5. **NC-SB-4 (section 4).** probe.py, compare.py, the s14 test, R1.md sections 1, 2 and 5, PROMPT-R1.md sections 4-5,
   r1.sh (browsers stamps, the bites chain's refusal). Proof: the s14 test: one schedule in both files, at least 1,000
   expected steady episodes and at least 101 recoveries pooled; test 5 recomputes hours and timeline; plant:
   STEADY_S 60 fails the s14 test.
6. **SEC-SB-1, SEC-SB-4 (section 5).** lib.mjs, config.mjs (driver arguments; :4-5 reworded), rules.md:52-53,
   PROMPT-R1.md and R1.md (C8's allow-rule read, the dialog line, the Stop B dry start). Proof: test 4: every driver's
   argv bound to loopback (no `0.0.0.0`, no `--allowed-ips`, gecko `--host=127.0.0.1`), and the options name a remote
   driver for all three; plants: gecko's `--host` dropped, and `--allowed-ips` put back on chromedriver, each fail
   test 4; the Stop B listen read on his laptop, recorded.
7. **SEC-SB-2, SEC-SB-6, SEC-SB-10, INF-SB-2's bounds, INF-SB-8, F12's term.** rs.yaml, lg.yaml, test 1. Proof: test 1:
   no root install, chown or chmod on a path under a user's home; the build tree read as its owner; the uid 2001 DNS
   drop before the loopback accept (`nft -c`); every curl with `--max-time`, every git fetch with a low-speed limit;
   `command -v dig tcpdump`; the fingerprint step before the first fetch; both files under 32 KiB; plant: rs.yaml:163's
   old line back fails test 1.
8. **SEC-SB-3, SEC-SB-7, SEC-SB-11.** settings.json, pre-push, rules.md, PROMPT-R1.md, operator test. Proof: the
   setting as the docs record it (section 5) or the fallback denies; `Read(~/.ssh/**)` denied; the push denies; the
   hook on a throwaway repo refuses any remote ref but `refs/heads/claude/npt-results`; plant: a push to another ref
   that passes the hook fails the test.
9. **SEC-SB-5 (section 5).** ops/runs/r1-dist.sha256 (its fill at Stop B), r1.sh bringup, host and browsers, test 5.
   Proof: fixtures: a differing list makes host and browsers refuse while the server chain may start; an empty list
   is refused.
10. **NC-SB-8.** r1.sh bringup, chain_run, collect. Proof: test 7 fixtures: alone read from the closed log; with
    TIMING-INVALID the chain records the five timing blocks not run and runs calibrate and s14.
11. **NC-SB-10, INF-SB-12.** r1.sh. Proof: test 7: a trap per block; the chain refuses while a room's holder or another
    chain runs; `-aN` in run id and log names; a done block without an INVALID file is refused a re-run.
12. **NC-SB-7.** npt-load-ctl (one fixed verb), r1.sh's LG blocks and load_settings, the wrappers test. Proof: test 3
    (fixed path, NN refusals, plant P7 still fails); test 7 (the header fields equal the room's).
13. **NC-SB-9.** r1.sh probe_presence. Proof: fixtures: a rate 2 % off, and a missing slack or policy read at one edge,
    each INVALID.
14. **NC-SB-12.** probe.py, compare.py, r1.sh's s14 block. Proof: the s14 test: a fixture whose steady episodes exceed
    the steady window's retransmissions reads VOID, though the bursts' retransmissions would have hidden it.
15. **NC-SB-5, NC-SB-6.** r1.sh room_settings, R1.md's tables. Proof: test 7 (the probe lists PM-0001-3 only; -16 in
    every kind but probe and bite).
16. **INF-SB-2's rule and provider list, INF-SB-3, -4, -5, -7, -10, -11, -13, F9, F20, NC-SB-18, SEC-SB-8's LG
    checks.** R1.md, PROMPT-R1.md; test 5 where it computes (hours, slack, the six Primary IPv4s).
17. **INF-SB-6, SEC-SB-9's subset.** npt-host.service, r1.sh host_active, test 1.
18. **INF-SB-9.** npt-ctl cpus, test 3 (a fixture where set-property succeeds and PID 1 still holds the tick vCPU).
19. **SEC-SB-8's RS half.** r1.sh bringup. Proof: a sudo list with one line beyond sudoers.d/npt's nine stops.
20. **NC-SB-13, -14, -15, -16, -17.** r1.sh, probe.py, compare.py, their tests.
At the end: `node --test 'ops/**/*.test.mjs'` and `bash ops/tests/wrappers.test.sh` all pass, the redact step 0
findings, the package files' hashes unchanged (DEP-HOLD), a new snapshot with its sha256.

**Size against SIZE-CHECK.** My per-item count is about 850 added lines (tests most of it; the renames add none),
likely 850-1,250: U8 at about 5,800-6,200 against the 6,000 stop, so the stop is live. The builder writes the running
total after each item; past 6,000 it stops at the end of that item, and the main session sends Mustafa the three
lines (history.md:2048-2051, as written); nothing is moved or split to get under it. The order above leaves the Highs
and the security Mediums done if it stops. My lean for those lines: go on, since U8 is off the cargo lane (no date
moves) and what is left is R1's evidence or security.

**Round 2's scope** (allowed: six Highs open; the same three seats at xhigh, at once, one round). Each reads only the
diff from 60205020's tree to the fix pass's snapshot, with this ruling:
- **netcode-architect:** items 1-5, 10-15 and 20, and item 16's F9, F20 and NC-SB-18 lines: the four NC Highs closed
  (the judge's rule against every npt-log name; -2's and condition 10's windows inside the load and the READY guard;
  six consistent leg runs; S14's n, ranks and the overlap's guards), and NC-SB-8's changed form.
- **security-auditor:** items 6-9, 12's new npt-load-ctl verb, 17, 18 and 19, and item 16's SEC-SB-8 lines: SEC-SB-1
  closed with the Stop B read written; the setting's record; GTFOBins on the changed root lines (npt-ctl cpus,
  npt-load-ctl, firstboot); the dist compare's refusals.
- **infra-engineer:** items 2, 3 and 5 for hours and timeline, 7's bounds and fingerprint step, 11, 16 and 17:
  INF-SB-1 closed.
No new finding outside the changed lines unless it is Critical. A Critical or High still open after round 2 comes to
me; one more round on it alone is the budget's last, and one still open after that stops U8 for Mustafa.

## What I could not check
No VM, no Windows, no web reading here: the Claude Code setting, Hetzner's console and stock, and the drivers' flags
are taken from the seats' cited sources and are re-read as sections 2 and 5 say. I ran no test and no npt-judge.
U6b and U6c are not merged, so their forms are read from their briefs. The S14 overlap is reasoned, not measured; the
episode count and the size are estimates.

## Decisions needed
None now. Conditional: **SIZE-STOP** (U8's fix pass passing SIZE-CHECK's 6,000 added lines). Why it matters: the build
stops until he answers. Options: go on and finish the pass, or stop U8 where it is. Lean: go on (off the lane, no date
moves).

## For Mustafa
Nothing now. In the day's message, each his to reverse: (1) R1's headers run stays a measured -2 run (a FAIL there is
kept; lean keep); (2) S14 takes a 10-minute steady window and runs on beside the browser legs (no hours or money move);
(3) Stop B grows by about 15 minutes: confirm fsn1 offers CCX23 and CAX21, firewall sources exactly the /32s, a
passphrase on the operator key held in Windows' ssh-agent, one dry start of each browser driver with its listen read,
and Claude Code's version; (4) on the run day, a dist that differs from the expected list holds the browser legs for
his word; (5) part of SEC-SB-9 (the page host's sandbox) stays open until U6c's merge, named in U8's report.

DONE
