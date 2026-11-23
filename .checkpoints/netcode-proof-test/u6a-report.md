# U6a's report: the server core (2026-11-23)
Brief u6a-brief.md rev 1.1 (c109691a…; the advisor's conditions 1-8 and SEC-U6AD-1 to -6, section 0.1). Base aa5c62b
(measured 709, bites 806). One round (NPT-TRIM), netcode and test seats, then a second on what changed (the High).

## What was built, and its size
- TickLoop (schedule, catch-up bound and dropped time, bounded channels, decoding off the tick thread, register 96's
  RTT hand-off, kick/leave/replace and their closing order, the panic path), the npt-server binary (settings, the 0600
  token file, OPEN within 5 s, exit codes), S4's reads (steal, RSS, schedstat, affinity, taskset pinning, the header's
  machine and carry 60's params), the -2 room log, npt-super's start timer and hold form, C-U6-17's cap, C-U6-18's order.
- **Size: 5,875 added, 27 removed, 19 files** (wip/u6a-build.patch 33c9b5dd…, measured by `git apply --stat`); the
  stop is past 6,000 (SIZE-CHECK). The build came in at 5,460; the fix pass added 383, FP-13 32.
- **Block 0** (before the build): the url 2.5.9 bump and the manifest edits; cargo-deny "advisories ok, bans ok, sources
  ok" (exit 0, five duplicate warnings), cargo-audit 0 vulnerabilities with one allowed warning RUSTSEC-2026-0441;
  dependency-check.md's U6a section; evidence u6a-block0.md. The build itself touched no manifest or lock.

## Evidence
- **Counts:** the builder's check4 on the fix-pass tree, all 11 steps PASS: measured 749 (+40), bites 849 (+43),
  against section 3's bars of +33 and +36; the test step 793.8 s (0.74 of base). FP-13 added one measured test.
  **M5 in the main folder** (wip/u6a-build.patch 33c9b5dd… applied, 19 of 19 files byte-equal to the builder's tree;
  main-run.sh, .gate/u6a-merge/m5-check.log, 03:39:38-03:55:44 UTC): all 11 steps PASS; measured 750 passed (709 +41),
  bites 850 (806 +44), 0 failed, 0 skipped; test step 854.9 s; self 78 of 78; redact 351 items, 0 findings. The gate
  (`node tools/gate.mjs --no-smoke`): PASS.
- **Plants** (each failed its named test for its planted reason, restored to its clean sha256): P1-P11 and P7b on the
  build; after the fix pass P7/P7b (the cap removed: fails the carry test's B leg with 6 refusals), P8, P9 (also FP-4's
  and FP-13's tests), and the new P12 (npt-super's header ignoring its flags), P13 (the SMT-sibling refusal removed),
  P14 (`alone` 1 on an empty walk). Logs .gate/u6a/plant-p*.log, fp*.log, fp-restored.log.
- **Bites** (each fails its mark's check by name): PM-0001-16/double-load on npt-super's own log (after FP-1),
  PM-0006-11/no-smt-map, PM-0001-1/shared-vcpu (its direct read; the log half cannot bite in this container).
- **Readings, never judged:** 20 starts, spawn to OPEN p50 634 ms, max 979 ms (debug build, this container).

## The round
- Round 1: both seats REVISE on one shared High (NR-U6A-1 = TR-U6A-1: npt-super's header lacked the run's bite,
  tick_hz and scene, so its log could not sit in the room's run); Mediums NR-U6A-2, -3, TR-U6A-2, -3, -4. Fix pass
  FP-1 to FP-12 (u6a-fix-pass.md): every Medium and up, and six Lows.
- Round 2 (u6a-r2-netcode.md, u6a-r2-test.md): both APPROVE WITH FINDINGS; the High and the Mediums closed; one new
  Medium (TR2-U6A-1, a flake FP-4 opened) fixed as FP-13 with no third round (NPT-TRIM); Lows below.

## Rows written (SERIES.md; no row edited in this unit)
Four per-second reads (steal, rss, proc, io) and `second.s.out_queue_count`; the loop's five series; the events
affinity (alone, sibling_free), cpu_read.changed, tick_panic.server_ns, room_start.spawn_to_open_ns (npt-super),
room_exit.nonzero (npt-super); the 15 -2 names; carry 60's seven params; stand_ins, burn and top-up (0: F4). The -2
rows' writer cell reads npt-room and carry 60's npt-lab while npt-server writes them (register 98's deferral).

## New pub items
npt-server `TickLoop` (start, connect, event, ticks, finished, clock_resolution_ns, stop, io_on_a_tick_sibling),
`LoopSettings`, `PhaseTimers`; npt-proto `channel_of`; npt-room `ClientSlot::relevant`; npt-log `Machine::read_local`;
npt-bots `Behaviour::host_step`. `StealReader` stays pub(crate).

## Carries, by number
- **Met:** C-U6-17 (option B proven by the carry test and P7; option A holds by its own rule, since its refusal counts
  arrivals, not stamps: TR2-U6A-2), C-U6-18, C-U6-4's npt-server half, carry 60, register 96's TickLoop half,
  C-U6-16 (c), C-U6-14's first clause, C-U6-19's gate line, conditions 3-5, SEC-U6AD-1 to -6.
- **Out, with their takers:** NR-U6A-5 (a per-session share of the inbound queue) U6d; NR-U6A-6 (the -2 count's cost
  in a timed phase) U6e; NR-U6A-7 (TickLoop's runtime and `stop` on a plain thread) and NR2-U6A-2 (exited I/O tids out
  of sibling_free) U6b; O1 and TR-U6A-5 (the kicked session's close, untested) U6d; O2 (one home for the protocol
  version, content hash and mode hash) U6b and U7 (advisor-u7-brief.md C2); F4 (S1's CPU stand-in burn) U6e; F1 (the
  room's half of register 96) open; NR-U6A-1 and NR2-U6A-1 (npt-super's log takes the room's run id and the same eight
  header values from one source, bite runs included; npt-super now needs --scene and --tick-hz; it ends by its stdin
  closing) U8's fix pass.

## Not verified
No RS, no load, no transport; the tick vCPU "alone" only as far as this container's processes go; the shared-vcpu
bite's log half; the kicked session's close; FP-2's and FP-3's paths in the binary and in affinity() (their pure parts
are tested); Machine::read_local on a two-thread SMT map; no judge run over any U6a log (each bite's check is an inline
copy); the 20-start reading used the measured debug binary only, and its recorded command lacks the flags npt-super
now needs; the fix-pass plants ran without `--locked`; the double-load bite shares the 5 s cap with its own 2 s.
