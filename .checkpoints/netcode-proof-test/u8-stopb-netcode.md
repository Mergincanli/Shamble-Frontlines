# U8 Stop B round: netcode-architect seat (NC-SB)

Plan: (1) read brief u8-brief.md 0.1, 7, :19-23; advisor-u8-brief C1-C10, L1-L5; advisor-u6c-brief s5; u8-build.md F1-F28 + fix pass.
(2) R1.md judged blocks vs marks.json + plan.md. (3) S14 rig probe.py/compare.py, F17, F9. (4) r1.sh probe block, probe_presence, F19, alone.
(5) fix item 8 Settings values vs main.rs; marks per room; F20 bites; F21; probe 732 s vs 0.20 h; F28 waits. (6) outage / re-run. (7) verdict.

Status: part 1 done (hashes of brief 3d102be9, ruling 595521d9, notes cf07e435, patch 60205020 all match). Reading R1.md, r1.sh done.

## Raw notes (one line each; checked against W unless marked "to check")
- r1.sh:186-189 room_up waits for super-NN.log room_start.spawn_to_open_ns (F28 assumed fixed); to check field names in super/src/main.rs.
- r1.sh:382-386 probe waits for room_exit.nonzero in super-02.log; to check field name.
- r1.sh:82-88 timing_invalid writes a text file only; collect (r1.sh:537) runs npt-judge over the logs, which does not read it: to check whether each judged mark's own INVALID rule catches alone 0.
- r1.sh:238-246 load_ready counts iterations (ssh + 1 s each), not seconds: "READY within 60 s" can pass at ~70-90 s.
- r1.sh:470 ds: LG 420 s from its own start; window OPEN+120..OPEN+420; LG end only delta after window end.
- r1.sh:363 headers: LG 300 s, warmup_ticks 1800 (60 s); to check -1/-2/PM-0006-11 window needs.
- r1.sh:451-457 starts: 20 starts; to check -16's count.
