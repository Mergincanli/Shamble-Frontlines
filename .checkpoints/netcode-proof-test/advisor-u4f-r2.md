# The advisor's read of U4f's 5.3 record run (the 17 pre-runs), 2026-11-19

Seat: a day session's advisor (general agent on main's `.claude/agents/advisor.md`, read whole with `git show
origin/main:`). Read only. This file is my one write. I ran no build, no scenario, no judge and no commit. Every figure
below comes from r2.txt or from the run's own logs and verdict files, read with `python3 -I` inline (no file written).

## 1. Verdict
**The part-clean, then the sets -47, -20 and -22, may go ahead, in that order, one per call. Three -21 rows (p4, p13,
p19) did not read as expected. They come to me under 5.3's last paragraph, and they hold -21's set, which waits for
U4f2 anyway. Nothing goes to Mustafa.**
- **Every -47 and -20 row reads as expected** (p5, p6, p10, p11, p12, p21), with two Low notes on p12.
- **Eight of the eleven -21 rows read as expected** (p2, p3, p8, p9, p15, p16, p18, p20), with Low notes on p18
  and p20.
- **Not as expected: p4, p13 and p19, all three the lag switch under option A at P1.**
  - Honest slot 24 walks into building area 3 at warm-up tick 205 and stays inside to the run's last tick (926).
  - That fails PM-0001-2's "everyone in view on at least 99 % of each client's snapshots": the lowest share is 0.365
    (slot 24) on p4 and p13, and 0.37 on p19. All 48 slots are below 0.99.
  - The -21 lag-switch option A part is therefore INVALID by `pre:PM-0001-2`, and its own checks are never read.
  - p19 also misses its reported figures: the catch-up cap refused 7 commands of the honest 400 ms holds (0 expected).
- **The run is tied to the merge.**
  - The run came from d502f54. Its spike tree f7ed90ad… equals 25d316e's (`git rev-parse`), and nothing under spikes/
    changed after d502f54.
  - The run log shows the lock held exclusively, an empty diff hash, 17 rows and exit 0 at 19:42:17 UTC.
  - r2.txt's five sha256s match the files: marks.json and SERIES.md at d502f54, and the three binaries in bin/.
  - D10 holds: marks.json and crates/judge are unchanged since 5b29296b.
- **How I read the rows.** I read each row by name from its `<key>-verdict.json` (TE3-3). For bite runs I read the
  `bites[].runs[].checks` entries, because the judge prints only each mark's first reason. The reported figures come
  from the run's lab.jsonl and room.jsonl.

## 2. The rows
"PM-0001-2 all pass" means all 11 of its checks on the run, the shot band included (contract (e) "in").

| Row | Build, suite, option | What the verdict reads, by name | As expected? | May its set run? |
|---|---|---|---|---|
| p2 | lab, clamp, A, P0 | PM-0001-2 all pass (shots 700.8); "never refused or counted" 0, pass; "1,000 points" 897, fail by length; twin check Gap; `clamp_kind.rerun_reports_count` 0 | **yes** | -21: after U4f2 |
| p8 | bites, refuse-not-clamp | bite shown; "never refused or counted" 897, **fail** (passes on p2); "1,000 points" 897, fail by length; Gap | **yes** | -21: after U4f2 |
| p15 | lab, clamp, B | as p2: all pass; 0, pass; 895, fail by length; Gap; reruns 0 | **yes** | -21: after U4f2 |
| p4 | lab, lag switch, A, P1 | PM-0001-2 **FAIL**: everyone in view 0.365 (needs 0.99); the -21 part INVALID, so neither "no tick moves further" nor "100 bursts" is read. From the logs, the catch-up figure's max is -238,678 µm (it would pass) | **no** (F1) | -21: not until F1 is fixed (it waits for U4f2 anyway) |
| p13 | bites, no-catchup-cap, A, P1 | the bite run INVALID by the same precondition (0.365). From the logs: catch-up figure max -238,678 (it would pass); `lagswitch_cmds_count` max 7 (at most 7, D13: holds); every tick hash and the fate log equal p4's (the bite changes nothing at 200 ms: TE-7) | **no** (F1) | as p4 |
| p18 | lab, lag switch, B, P1 | PM-0001-2 all pass (in view 1); "no tick moves further" -1,400,000, pass; "100 bursts" **5**, fail by length (the expect says 3: F5) | **yes** (note F5) | -21: after U4f2 |
| p19 | lab, lag switch, A, P1, 400 ms (unjudged) | PM-0001-2 **FAIL** as p4 (0.37); reported: late by W 2, 2, 2, 0, 0 over 5 holds (6 in all; about 3 or 4 each expected); `input_limited` 25 (about 5 a hold; about 4 expected); **kind-10 refusals 7 (0 expected)**; `lagswitch_cmds_count` max 8 (at most 8 holds; "about 5" does not); `seq_outside_window` 48 (not predicted) | **no** (F1, F2) | as p4 |
| p3 | lab, fast clock, A, P0, 70 s | PM-0001-2 all pass (691.5); "at most the server's ticks plus the lead bound" -732, pass; "moves no further" -146,400,000, pass; "10 runs" 1, fail by length; episode: lead-bound 720, not-newer 899, `left_out` 0; `ping_limited` 0 (S1, by design) | **yes** | -21: after U4f2 |
| p9 | bites, not-older-than | bite shown; excess **167, fail** (-732 on p3); movement 33,400,000, fail (allowed); "10 runs" 1, fail by length; not-newer 0 | **yes** | -21: after U4f2 |
| p16 | lab, fast clock, B | as p3: all pass (696.8); excess -12, pass; movement -2,400,000, pass; "10 runs" 1, fail by length; lead-bound 720, not-newer 899; kind 10 (B's `Full`) 22 on the fast client | **yes** | -21: after U4f2 |
| p20 | lab, fast clock, A, 130 s, 60 s idle (unjudged) | PM-0001-2 all pass (686.8); two episodes: excess -1,811 and -733, movement -362,200,000 and -146,600,000 (both at most 0); late **0**; lead-bound 0 then 720; "10 runs" 2, fail by length | **yes** (note F3) | -21: after U4f2 |
| p5 | lab, -20, A, P3 | PM-0001-2 all pass (699.75); on time 0.999792 (needs 0.999), pass; missing 0.000208 (needs at most 0.001), pass; presses 14,402, 0 lost, 0 doubled; reorder down 101 ‰ (above 0: D3) | **yes** | -20: **yes** |
| p6 | lab, -20, B, P3 | as p5: all pass (699.6); 0.999792 and 0.000208, pass; presses 14,400, 0, 0; reorder 101 ‰; late 13 of 44,187 commands (0.03 %) | **yes** (note F6) | -20: **yes** |
| p12 | bites, redundancy-1, B, P3 | bite valid (PM-0001-2 passed) and shown; on time **0.894861, fail** (passes on p6); missing 0.105, fail (reached); "0 presses lost" **119, fail** (F5: reached); presses 14,398; 0 doubled | **yes** (notes F5) | -20: **yes** |
| p10 | bites, newest-sample, clock, P2 | bite shown; offset p99 **9.25 ms, fail**; "1,000 connects" 48, fail by length; contract (e) FAIL (shots 0: F4) | **yes** (note F4) | -47: **yes** |
| p11 | bites, tick-quantised | bite shown; offset p99 **12.81 ms, fail** (D4: about 11.9); 48, fail by length; contract (e) FAIL (F4) | **yes** (note F4) | -47: **yes** |
| p21 | lab, clock, P2 | offset p99 2.60 ms (needs at most 5), pass; "1,000 connects" 48, fail by length; contract (e) FAIL (F4) | **yes** (note F4) | -47: **yes** |

**Read across every row:**
- **Overflow resyncs:** 0 on all 17 rows, 48 clients each (816 sessions).
- **C62H-BOUND tally:** "neither 0" on every row. Measured incomplete equals link-lost wherever client hashes are
  filed. The clock rows file none, by design.
- **Honest clients' forged-kind refusals:** 0 on every row. The one honest counter that moved is
  `seq_outside_window`, 212 on p12: the bite's own gaps, a counter that never counts as forged.
- **Stamps:** 0 skipped everywhere; the fast clock's `left_out` is 0 everywhere.

## 3. Findings (CLAUDE.md's scale)
- **ADV-U4FR2-1 (F1), Medium: the honest clients' open-loop `steady` script has no leash. On the lag switch at P1
  under option A, one soldier walks into a building in 7 s and stays there, which voids PM-0001-2.**
  - **The evidence:**
    - p4, p13 and p19's `tick.s.in_building_slot_area` reads [24, 3] on every tick from 205 (warm-up) to 926 (the
      run's end). The summary line says "in a building: 600 slot-ticks (slots 24)".
    - Every slot's in-view share is 0.365 to 0.488, so all 48 are below 0.99. Carry 49's "holding every very large
      unit" falls to 0.9868.
    - None of this happens under option B (p18), at P0 under option A (p2, p3, p20) or in the brawl (p5, p6, p12): 0
      slot-ticks on each.
    - Slot 24 itself is clean: no missing input, never late, no refusal, the same 922 commands on p4 and p18.
  - **The cause, as far as I can tell:**
    - `steady` is open loop: the walk comes from `MOVES` by command count (bots/src/lib.rs:651). Only `Kind::Brawl`
      takes BRAWL-1's leash (:653).
    - -21's honest clients run `steady` by U4F-Q11 (ii) (a) (c-u4f-r2.mjs:48, :50, :52).
    - My reading: the sim moves one tick per command against the doors and vehicles as they stand at that tick
      (sim/src/world.rs:8, :1246). Option A runs a command when it arrives, so a different arrival pattern meets a
      different door state, and an open walk can end up inside and stay. This is not verified: no positions are
      logged.
  - **Reach:**
    - -21 only. -20 and -22 brawl with the leash (the default, lab/src/scenario.rs:2094; c-u4f-runs.mjs:51-52 pass no `--behaviour`), and
      -47's part needs no precondition.
    - Carry 86 (u4f-brief.md :1806-1809) says "a suite that fails comes to the advisor first": the lag-switch
      option A suite has failed its showing, so it cannot count until a pre-run passes.
    - Over -21's 10-minute runs every suite on `steady` is at risk: U4r's P0 seed 1 had a building entry at 458 s
      before the leash (brawl1-review-r1-full.md:30).
  - **Route, as a carry by number in U4f2's brief** (the main session takes the next free number beside C-U4F2-1 to
    -4):
    - before -21's set, every -21 suite's honest clients stay out of buildings for 610 s at P0 and P1 under both
      options;
    - a sim-only test with a plant shows it, on BRAWL-1's model (brawl1-review-r1-full.md:30-32);
    - U4f2's record run of all 21 rows reads "in a building: 0" and PM-0001-2 all passing on every -21 row.
  - **The choice of fix is U4f2's brief's, and comes to me.** For example: the leash on `steady`; a walk kept clear
    of the doors; or another honest script that keeps the twins' hashes deterministic.
  - **His involvement:** it changes no pass mark, judge or marks.json text, so it is inside the plan (TEST-FIRST rule
    2). It counts in U4f2's size line. If U4f2's estimate passes about 1,500 lines, that goes to him before the build
    (advisor-u4f-merge-head.md, U4f2's stop).
- **ADV-U4FR2-2 (F2), Medium: p19 contradicts U4F-Q15 (c)'s arithmetic, my ruling's "the cap refuses nothing there"
  (advisor-u4f-brief.md:293-295; u4f-brief.md :1037, :1621-1624).**
  - **What the run shows:** over the five 400 ms holds, the bucket and W did not hold the burst under the cap.
    - Option A's per-tick cap (room/src/input.rs:323) refused **7** of the holder's commands under kind 10, a forged
      kind (`buffer_full_count` 7, slot 0).
    - `lagswitch_cmds_count` reached 8, the cap.
    - W dropped 2, 2, 2, 0 and 0 (6 in all, not 3 or 4 each). The bucket limited 25 datagrams (about 5 a hold, not
      about 4). 48 sequences were dropped outside the kept window, which nothing predicted.
  - **What it does not touch:** p19 is unjudged, and no -21 check reads refusals. So no bar moves and no verdict
    changes.
  - **Why Medium:** a recorded ruling's premise does not hold, and it bears on OQ-0001-6. Under option A, an honest
    400 ms stall gets commands refused under a forged kind.
  - **Route:**
    - U4f2's brief restates p19's reading with these figures and corrects Q15 (c)'s sentence. A carry by number.
    - -20's set report names it in the OQ-0001-6 reading (NC-U4F-5) as an early, unjudged read. No B-side 400 ms run
      exists to compare against.
- **ADV-U4FR2-3 (F3), Low: p20 does not show the past window W acting.**
  - Late is 0, and the episode's refusals equal p3's exactly (lead-bound 720, not-newer 899). So F1's every-10th stamp
    (bots/src/lib.rs:216-223) closed the idled bank at once, not "within 10 commands" with up to 9 dropped late.
  - My reading: the first command after the idle was a 10th. Per-command stamps are not logged, so this is not
    verified.
  - The expect line holds: both episodes' excess and movement are at most 0, and the figures are reported.
  - u4f-brief.md 5.3's rev 3.2 sentence for p20 ("every command of the episode is dropped `late`") predates F1 and does
    not describe the run.
  - Route: U4f2's record of p20 says this. f38 stays W's proof.
- **ADV-U4FR2-4 (F4), Low: the contract's shot band fails on the clock suite by construction.**
  - p10, p11 and p21 read contract FAIL, (e) out, with shots 0, because the clock suite never fires (U4F-Q4 (b)). The
    band applies by client count alone (lab/src/contract.rs:227).
  - No verdict reads the contract, and c-u4f-runs.mjs runs none, so -47's set is unaffected.
  - Route: -47's set report names it. The fix, "not applied" for a never-firing suite, rides U4f2's lane, where the
    lab opens.
  - The same contract's (c) one-admit rule would not hold for a suite that reconnects either (u4f-build.md:469-470).
- **ADV-U4FR2-5 (F5), Low: slips in the expect lines and in 5.3's list of checks a bite reaches.**
  - **(a) The burst count.** p4's "(3 bursts in 20 s)": the check reads every phase ("window": "all",
    marks.json:5051-5061), so a 30 s run counts 5 (p18).
    - At the judged 600 s it reads 101, 100 of them measured, so the verdict is the same either way.
    - This is his marks.json text. I note it only and ask no change.
  - **(b) p12's figure.** p12's "about 95 %" (A3) read 89.5 %. My reading: under the controller a step can give two
    commands, and redundancy-1 sends only the newest, so the older one is never sent. That matches the honest gaps,
    `seq_outside_window` 212.
  - **(c) p12's lost presses.** p12 also fails "0 presses lost" (119).
    - **Ruled: reached by the bite.** A command lost with its only datagram loses that tick's press, the same as the
      missing check.
    - 5.3's list (":4724-4737, the missing check") gains it.
  - Route: U4f2 rewords c-u4f-r2.mjs's lines when it adds the forged rows (C-U4F2-4 opens the file).
- **ADV-U4FR2-6 (F6), Low: the late count, reconciled.**
  - On p6 the session counters say late 13, while the room ticks' `tick.v.late_count` sums to 12. One late command was
    counted outside a tick record.
  - Every other row agrees.
  - Route: -20's set report says which figure carry 74's B reading uses: the counters, against the commands sent.
- **Note, no finding: the fast-clock rows' late share.**
  - On p20, option A's all-slots "not on time" reads 1.41 % (p3, p9, p16: 0.83 %). That is the fast-clock cheat
    alone: slot 0 is 67.7 % not on time, refused under kinds 1 and 2. The honest clients read 0.000 %.
  - Carry 74's 1 % is the honest clients' measure, so no row trips it. The highest honest share is 0.03 % (p6, B
    reading).
  - The redundancy-1 bite (p12) is its own named effect.

## 4. Readiness: the part-clean, then -47, -20, -22
- **The part-clean (u4f-brief.md 5.5 item 2) is ready and needed.**
  - At 19:54 UTC: 5,205 MiB free (`df -BM`), 1,109 above the 4,096 floor. No cargo, npt-* or runner process was
    running (`pgrep`).
  - The shared target holds 6,348 MiB; the main folder's cache (.gate/netcode-proof-test/target) 776 MiB.
  - The pre-runs took 79 MiB of logs against the 195 MiB projected, plus 7 MiB of binaries.
  - The sets need about 940 MiB of logs: -47 about 47 (2.2 MiB per 30 s clock run, scaled to 130 s), -20 about 470
    to 560 (3.3 MiB per 30 s P3 run, so about 67 per 610 s run, against the runner's 84 MB), -22 about 320. On top
    come the release builds in the main cache.
  - Without the part-clean that would fall under the floor. Run it as approved (W1-TARGET item 3 as amended, P27),
    with no lane running, and take D2 before and after. KL-U4F should not be needed. If D2 shows the room short, it
    goes to him (5.5 item 3).
- **The sets, one per call, in R2's order:**
  - In the main folder: `flock -x .gate/shared-target/.lane.lock node spikes/netcode-proof/ops/runs/c-u4f-runs.mjs
    <set> --date <day>`, with CARGO_TARGET_DIR unset (c-u4f-runs.mjs:3-8).
  - Order: 47, then 20, then 22.
  - Restart the saver buovbuxl2 inside its window (about 21:32 UTC); -20 alone runs about 39 minutes.
- **What the sets must watch, from these rows:**
  - **-47:**
    - p21's p99 is 2.60 ms on one round of 48 connects, against 5 ms. The set's 1,000 connects at three seeds are
      the real read.
    - Both bites show clearly: 9.25 and 12.81 ms.
    - The contract does not apply to these runs (F4). If the report quotes one, it says so.
  - **-20:**
    - The event queue's high-water at P3 in 20 s: 488 (A), 689 (B), 625 (bite, B). That is at least 2.3 times
      below the cap of 1,600. B3's 60 s figure was 1,045.
    - Overflow resyncs were 0 in every pre-run.
    - The 10-minute runs are the read (R2): an honest client with any overflow resync comes to me before -22's set
      starts. The report states the 10-minute high-water and its margin; a margin under 1.5 is named for U4g.
    - On time is 0.999792 under both options, against 0.999 (0.021 % not on time against a 0.1 % allowance).
      Missing is 0.000208. Reorder is 101 ‰ (above 0).
    - The bite should read shown (89.5 %; F5).
    - The report carries NC-U4F-5 and C-OQ6-SEC-1, F2's early read, and F6's figure choice.
  - **-22:** no pre-run, by design (5.2).
    - The pre-runs show an honest high-water of at most 39 events at P0.
    - Its load report states each run's `overflow_resyncs_count` and N1, as already ruled.

## 5. Decisions needed
None. F1 to F6 are inside the plan (TEST-FIRST rule 2). None changes a pass mark, the judge or marks.json, costs
money or touches main. F1's fix is U4f2's brief's to propose and mine to check. F5 (a) asks for no change to his
marks.json.

## For Mustafa
Nothing. The next day's message names, among the choices taken without him:
- F1's route: -21's honest script must be shown to stay out of buildings before -21's set;
- F2: the correction to the Q15 (c) reasoning;
- F5 (c)'s ruling: the redundancy-1 bite reaches "0 presses lost".

## 6. What I read, ran and did not check
**Read:**
- main's advisor.md, whole;
- r2.txt, whole (330 lines), and r2.log's head, tail and every exit line;
- all 17 `<key>-verdict.json`, every part, run and bite check by name;
- c-u4f-r2.mjs, whole, with RUNS and every expect;
- c-u4f-runs.mjs :1-140;
- u4f-brief.md 5.1-5.5 (:959-1126), U4F-Q15 (:1611-1625), :1803-1809 (carry 86), the p19 and fast-clock rows
  (:526-537);
- advisor-u4f-stop.md :1-140 (R1, R2), advisor-u4f-r1.md :1-120 and :320-365, advisor-u4f-merge-head.md, whole;
- advisor-u4f-brief.md :290-296;
- u4f-report.md "What comes next" (:523-553), with greps for p19, p20 and S1;
- the card's "Now: U4f" (:53-108) and :160-191;
- u4f-build.md :460-475;
- brawl1-review-r1-full.md :30-39 (the leash and building record);
- in the tree at d502f54:
  - bots/src/lib.rs :85-135, :198-223, :333-373, :425-505, :520-560 and :630-670;
  - room/src/input.rs :1-40, :60-72, :140-160 and :270-330; room/src/room.rs :120-170;
  - proto/src/input.rs :80-100 and :145-170;
  - sim/src/world.rs :1-30 and :1240-1300;
  - lab/src/contract.rs :10-18 and :208-245;
  - lab/src/scenario.rs :1236-1275, :1578-1600 and :2075-2100;
  - l1/src/lib.rs :182-212;
  - marks.json's -20 and -21 checks and parts, and :336-345; SERIES.md :273, :370.

**Ran (read only):**
- `git show origin/main:.claude/agents/advisor.md`, `git rev-parse` of both spike trees, `git log` and `git diff
  --stat` (D10, spikes/ since d502f54), `git status`;
- `sha256sum` of marks.json, SERIES.md and the three binaries;
- `df -BM`, `du`, `pgrep`, greps;
- inline `python3 -I` reads of the 17 rows' room.jsonl and lab.jsonl:
  - the in-view shares per slot;
  - building occupancy by tick;
  - per-tick lag-switch commands and late drops;
  - the session counters by slot;
  - the event-queue high-water marks and reorder events;
  - both readings of the late share;
  - p4 against p13's tick hashes.

**Not checked:**
- I ran nothing that builds or simulates.
- The cause of slot 24's entry under option A (F1) is my reading of the code, with no positions in the logs.
- p20's first post-idle stamp index (F3) is inferred.
- p19's kind-10 refusals cannot be split by hold: the counter is a session total.
- I did not check the lane lock's state directly (only that no process holds work), nor the builders' worktrees.
