SUBS = [
("| resetting and replaying the local soldier over 6 ticks (200 ms at 30 Hz) at most 1 ms p99; 12 ticks for the 60 Hz form (PM-0001-40) |",
 "| resetting and replaying the local soldier over 6 ticks (200 ms at 30 Hz: R1's mark and the budgets row as written) at most 1 ms p99, and over 7 ticks, the deepest replay part 4's arithmetic gives at P2 ((100 + 100 + 33.3) ÷ 33.3 = 7, with a 3-tick buffer), at most 1 ms p99 too (proposed by the netcode review: R1's 200 ms covers a buffer of at most 2 ticks); 12 ticks for the 60 Hz form (PM-0001-40); reported: the real replay depth per profile P0 to P4 (median and max, from PM-0001-41's runs; part 4 gives 6.5–8.5 ticks at P3 and 8–10 at P4) and the replay time forced at P3's and P4's measured maximum depths |"),
("the replay forced on every snapshot with exactly 6 ticks (proposed: with the skip on, too few and too easy replays are timed);",
 "the replay forced on every snapshot with exactly 6 ticks, then in separate runs with exactly 7 (proposed: with the skip on, too few and too easy replays are timed);"),
("| 3 runs, each passes (18,000 replays each; p99 = the 17,820th).",
 "| 3 runs per depth, each passes (18,000 replays each; p99 = the 17,820th)."),
("r3 R3-MEASURE-F7 (lines 641–646) |",
 "r3 R3-MEASURE-F7 (lines 641–646); the draft, part 4 (the real depth, 5 to 7 ticks at P2); the netcode review of 2026-11-02, finding 10 |"),
]
