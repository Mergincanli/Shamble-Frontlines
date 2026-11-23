# U8 Stop B round: netcode-architect seat (NC-SB)

Plan: (1) read brief u8-brief.md 0.1, 7, :19-23; advisor-u8-brief C1-C10, L1-L5; advisor-u6c-brief s5; u8-build.md F1-F28 + fix pass.
(2) R1.md judged blocks vs marks.json + plan.md. (3) S14 rig probe.py/compare.py, F17, F9. (4) r1.sh probe block, probe_presence, F19, alone.
(5) fix item 8 Settings values vs main.rs; marks per room; F20 bites; F21; probe 732 s vs 0.20 h; F28 waits. (6) outage / re-run. (7) verdict.

Status: parts 1, 2 (most), 4 (most), 5 (most) read. Next: S14 rig (part 3), npt-ctl/npt-lib/npt-s14, then write findings.

## Raw notes (one line each; checked against W or merged code)
- Hashes match: brief 3d102be9, ruling 595521d9, notes cf07e435, patch 60205020.
- r1.sh:186-189 room_up waits for room_start.spawn_to_open_ns in super-NN.log; super/src/main.rs:301-310 names match (room_start/spawn_to_open_ns, room_exit/nonzero). BUT super start() on deadline (10 s default) writes room_start with open=false (main.rs:270-278, :325-327): room_up then returns "open" for a killed room. Low.
- probe exit wait r1.sh:382-386: super hold() returns when the child exits (main.rs:329-337) then finish flushes: works even before F28's fix; room_up needs F28.
- HIGH candidate A: headers/ds/calibrate rooms duration_ms 0 (r1.sh:160); measured ticks = every tick after warmup (tick_loop.rs:548-552, no After for ticks); LG ends first (r1.sh:363-365, :470-476); Room::leave frees the slot (room.rs:889-894) so snapshot_clients drops (tick_loop.rs:266-273) -> -2 "connected snapshot-receiving client on every soldier" min>=48 over measured window (README 2.3 default measured) FAILS; README s3 step 5: a failed valid run fails the part whatever min -> -2 FAIL at R1, kept. ds: ws_split pool = mark .. run end (u6b-brief.md:46-58) -> includes 48 WS closes.
- HIGH candidate B: probe's six leg logs share settings run R1-probe-02 (r1.sh:156); leg logs carry their own profile (u6c-brief.md:192-193); judge groups by run (eval.rs:37-75), always_rules INVALID on profile disagreement (eval.rs:542-546); checks without `from` pool every log of the run (README:162-163) -> -3's L2 parts INVALID at R1. Not in R1.md:243-249 alignment list; probe_presence reads no header.
- MED: R1.md:33 and r1.sh:137 read the brief's "C-U8-2/-4/-6" as marks PM-0001-4, -6 (u8-brief.md:392 means conditions C-U8-4, C-U8-6). No verdict change (parts need S30), wrong record + wrong claim in the plan he OKs.
- MED: -16 notes "every room start in every run" (marks.json -16); U8 lists -16 only on the 20 clients=1 starts rooms (r1.sh:135); headers/probe/ds/browser starts not judged.
- MED: npt-load's npt-log (log_path load-NN.npt.log r1.sh:230) never fetched: npt-load-ctl `log` reads logs/load-NN.log (stdout) only (npt-load-ctl:73); collect judges RS only (r1.sh:537); LG deleted first -> -1's LG part record (u6c-brief.md:230) lost.
- MED: bring-up reads alone 2 s after OPEN while the room's log is a BufWriter (server main.rs:477) flushed only at finish (tick_loop.rs:941): works only by the 8 KiB spill; read after room_down instead. And TIMING-INVALID is a text file npt-judge does not read (r1.sh:82-88, :537): verdicts.json would show -1 FAIL on the headers run where C1 rules INVALID.
- r1.sh:238-246 load_ready counts iterations (ssh + 1 s), not seconds.
- B1 packets: lab treats packets as outside bursts (lab/src/probe.rs:124-128), SERIES.md:472 1,010,000 at B1: U8's 1,000,000 right if U6c copies L1; name it in the alignment list. Low.
- io_threads = |housekeeping| (2 or 3 on CCX23) matches ADR 0001:1420 placement; -11 (R2) caps CPU not threads. Sound.
- warmup 1,800: LG READY within 30 s (u6c-brief.md:228) inside 60 s. Sound; the end is the problem (A).
- first_cpu_model: r1.sh:277 strips one leading space; log lib.rs read_local trims both ends: normalisation differs. Low.
- PM-0006-11 only on headers: correct (unpinned rooms and the probe write no affinity -> empty sibling check would FAIL).
- F20: none of -2 fire-stops (needs >10 min S30, R2), -3's five (four L1 at U2/C; not-net-of-base is MP's), -14 unsplit-group (S30, R2), -36 read-once and -52 predicted-damage (ML, R8) needed at R1.
- Probe hours: 6x(10+100+12)=732 s; chain sum ~0.69 h of 1.00 h slot. 0.20 h vs ~0.21 h fine.
